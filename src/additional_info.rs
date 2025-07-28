use std::fs;

pub fn get_uptime() -> Result<String, String> {
    // Path to the uptime file in Linux
    const UPTIME_FILE: &str = "/proc/uptime";

    // Read the contents of /proc/uptime
    let uptime_content = fs::read_to_string(UPTIME_FILE)
        .map_err(|e| format!("Failed to read {}: {}", UPTIME_FILE, e))?;

    // Split the content into parts and extract the uptime value
    let uptime_seconds: f64 = uptime_content
        .split_whitespace()
        .next()
        .ok_or_else(|| "Uptime data is missing".to_string())?
        .parse()
        .map_err(|e| format!("Failed to parse uptime: {}", e))?;

    // Convert uptime from seconds to a human-readable format
    let days = (uptime_seconds / 86400.0).floor();
    let hours = ((uptime_seconds % 86400.0) / 3600.0).floor();
    let minutes = ((uptime_seconds % 3600.0) / 60.0).floor();
    let seconds = uptime_seconds % 60.0;

    // Format the uptime as a string
    Ok(format!(
        "{:.0} days, {:.0} hours, {:.0} minutes, {:.0} seconds",
        days, hours, minutes, seconds
    ))
}