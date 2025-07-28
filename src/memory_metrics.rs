use std::fs;
use serde_json::{json, Value};

pub fn get_memory_info() -> Result<Value, String> {
    let meminfo = fs::read_to_string("/proc/meminfo")
        .map_err(|_| "Failed to read /proc/meminfo".to_string())?;

    let mut total_memory = 0;
    let mut free_memory = 0;
    let mut buffers = 0;
    let mut cached = 0;

    for line in meminfo.lines() {
        if line.starts_with("MemTotal:") {
            total_memory = parse_meminfo_line(line);
        } else if line.starts_with("MemFree:") {
            free_memory = parse_meminfo_line(line);
        } else if line.starts_with("Buffers:") {
            buffers = parse_meminfo_line(line);
        } else if line.starts_with("Cached:") {
            cached = parse_meminfo_line(line);
        }
    }

    let used_memory = total_memory - free_memory - buffers - cached;

    // Return memory metrics as JSON
    let memory_info = json!({
        "total": total_memory,
        "used": used_memory,
        "free": free_memory,
        "buffers": buffers,
        "cached": cached
    });

    Ok(memory_info)
}

fn parse_meminfo_line(line: &str) -> u64 {
    line.split_whitespace()
        .nth(1) // Second word is the value
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(0)
}