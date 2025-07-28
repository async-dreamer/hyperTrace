use std::process::Command;
use serde_json::{json, Value};

pub fn get_disk_usage() -> Result<Value, String> {
    // Execute the `df` command to get disk usage information
    let output = Command::new("df")
        .arg("-B1") // Block size of 1 byte
        .arg("/")   // Target the root filesystem
        .output()
        .map_err(|e| format!("Failed to execute df command: {}", e))?;

    // Parse the stdout output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    // Ensure the output has at least two lines (header + data)
    if lines.len() < 2 {
        return Err("Unexpected output from df command: insufficient lines".to_string());
    }

    // Split the second line into parts
    let parts: Vec<&str> = lines[1].split_whitespace().collect();
    if parts.len() < 4 {
        return Err("Unexpected format from df command: insufficient columns".to_string());
    }

    // Parse the disk metrics
    let disk_total = parts[1].parse::<u64>().map_err(|_| "Failed to parse total disk space".to_string())?;
    let disk_used = parts[2].parse::<u64>().map_err(|_| "Failed to parse used disk space".to_string())?;
    let disk_free = parts[3].parse::<u64>().map_err(|_| "Failed to parse free disk space".to_string())?;


    // Return memory metrics as JSON
    let metrics = json!({
        "total": disk_total,
        "free": disk_free,
        "used": disk_used,
    });

    Ok(metrics)
}