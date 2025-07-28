mod cpu_metrics;
mod memory_metrics;
mod additional_info;
mod disk_metrics;

use std::fs::File;
use std::net::Ipv4Addr;
use tokio::time::{sleep, Duration};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use warp::Filter;

#[derive(Debug, Deserialize)]
struct Config {
    ip: Ipv4Addr,
    port: u16,
    endpoint: String,
}

#[tokio::main]
async fn main() {
    // Load config from YAML
    let config_file = File::open("Config.yaml").expect("Failed to open config file");
    let config: Config = serde_yaml::from_reader(config_file).expect("Failed to parse config");

    let ip = config.ip;
    let port = config.port;
    let endpoint = config.endpoint.trim_start_matches('/').to_string();
    
    // Create a WebSocket filter for the "hypertrace" path
    let ws_route = warp::path(endpoint)
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(handle_ws)
        });

    // Start the server on port 3030
    println!("Listening on ws://127.0.0.1:3030/hypertrace");
    warp::serve(ws_route)
        .run((ip, port))
        .await;
}

// Handle WebSocket connections
async fn handle_ws(ws: warp::ws::WebSocket) {
    let (mut tx, mut rx) = ws.split();

    // Process incoming messages and respond accordingly
    while let Some(result) = rx.next().await {
        match result {
            Ok(msg) => {
                if msg.is_text() {
                    let text = msg.to_str().unwrap_or_default();
                    match text.trim() {
                        "get_metrics" => {
                            // Update previous stats for the next iteration
                            let prev_stat = cpu_metrics::CpuStat::read_from_file()
                                .expect("Failed to read CPU stats");

                            // Wait for 1 second to calculate CPU load
                            sleep(Duration::from_secs(1)).await;

                            // Read current CPU stats
                            let curr_stat = cpu_metrics::CpuStat::read_from_file()
                                .expect("Failed to read CPU stats");

                            let cpu_load = cpu_metrics::calculate_cpu_load(&prev_stat, &curr_stat);

                            // Fetch memory metrics
                            let memory_metrics = memory_metrics::get_memory_info();

                            // Collect disk stats
                            let disk_metrics = disk_metrics::get_disk_usage().unwrap_or_else(|_| serde_json::json!({"error": "Failed to collect disk metrics"}));

                            let uptime = additional_info::get_uptime().unwrap();

                            // Combine CPU load and memory metrics into a single JSON object
                            let combined_metrics = serde_json::json!({
                                "cpu_load": cpu_load,
                                "memory": memory_metrics.unwrap_or_else(|_| serde_json::json!({"error": "Failed to collect memory metrics"})),
                                "uptime": uptime,
                                "disk_metrics": disk_metrics
                            });

                            // Send the combined metrics to the client
                            if let Err(e) = tx.send(warp::ws::Message::text(combined_metrics.to_string())).await {
                                eprintln!("Error sending message: {}", e);
                                break;
                            }
                        }
                        "get_cpu_info" => {
                            // Fetch CPU info
                            if let Some(cpu_info) = cpu_metrics::CpuInfo::new() {
                                let cpu_info_json = serde_json::to_string(&cpu_info).unwrap();
                                if let Err(e) = tx.send(warp::ws::Message::text(cpu_info_json)).await {
                                    eprintln!("Error sending CPU info: {}", e);
                                    break;
                                }
                            } else {
                                eprintln!("Failed to retrieve CPU info.");
                            }
                        }
                        _ => {
                            eprintln!("Received unexpected text message: {}", text);
                        }
                    }
                } else {
                    eprintln!("Received non-text message, ignoring.");
                }
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }
}