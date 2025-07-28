use std::fs;
use std::process::Command;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuInfo {
    pub cpu_count: u64,
    pub cpu_mhz: f64,
    pub model_name: String,
    pub bogomips: f64,
    pub architecture: String,
    pub cache_info: String,
}

impl CpuInfo {
    pub fn new() -> Option<Self> {
        let cpuinfo = fs::read_to_string("/proc/cpuinfo").ok()?;
        let lscpu_output = Self::run_command("lscpu")?;

        let cpu_count = Self::get_cpu_count(&cpuinfo)?;
        let cpu_mhz = Self::get_cpu_mhz(&cpuinfo)?;
        let model_name = Self::get_model_name(&cpuinfo)?;
        let bogomips = Self::get_bogomips(&cpuinfo)?;
        let architecture = Self::get_architecture(&lscpu_output)?;
        let cache_info = Self::get_cache_info(&lscpu_output)?;

        Some(CpuInfo {
            cpu_count,
            cpu_mhz,
            model_name,
            bogomips,
            architecture,
            cache_info,
        })
    }

    fn run_command(command: &str) -> Option<String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .ok()?;
        String::from_utf8(output.stdout).ok()
    }

    fn get_cpu_count(cpuinfo: &str) -> Option<u64> {
        let count = cpuinfo.lines()
            .filter(|line| line.starts_with("processor"))
            .count();
        Some(count as u64)
    }

    fn get_cpu_mhz(cpuinfo: &str) -> Option<f64> {
        cpuinfo.lines()
            .find(|line| line.starts_with("cpu MHz"))?
            .split(':')
            .nth(1)?
            .trim()
            .parse()
            .ok()
    }

    fn get_model_name(cpuinfo: &str) -> Option<String> {
        cpuinfo.lines()
            .find(|line| line.starts_with("model name"))?
            .split(':')
            .nth(1)?
            .trim()
            .to_string()
            .into()
    }

    fn get_bogomips(cpuinfo: &str) -> Option<f64> {
        cpuinfo.lines()
            .find(|line| line.starts_with("bogomips"))?
            .split(':')
            .nth(1)?
            .trim()
            .parse()
            .ok()
    }

    fn get_architecture(lscpu_output: &str) -> Option<String> {
        lscpu_output.lines()
            .take(4)
            .collect::<Vec<&str>>()
            .join("\n")
            .into()
    }

    fn get_cache_info(lscpu_output: &str) -> Option<String> {
        lscpu_output.lines()
            .filter(|line| line.contains("cache"))
            .collect::<Vec<&str>>()
            .join("\n")
            .into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuStat {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
    pub steal: u64,
}

impl CpuStat {
    pub fn read_from_file() -> Option<CpuStat> {
        let contents = fs::read_to_string("/proc/stat").ok()?;
        let cpu_line = contents.lines().find(|line| line.starts_with("cpu "))?;
        let mut parts = cpu_line.split_whitespace();
        if parts.clone().count() < 9 {
            return None;
        }
        Some(CpuStat {
            user: parts.nth(1)?.parse().ok()?,
            nice: parts.next()?.parse().ok()?,
            system: parts.next()?.parse().ok()?,
            idle: parts.next()?.parse().ok()?,
            iowait: parts.next()?.parse().ok()?,
            irq: parts.next()?.parse().ok()?,
            softirq: parts.next()?.parse().ok()?,
            steal: parts.next()?.parse().ok()?,
        })
    }

    pub fn total(&self) -> u64 {
        self.user + self.nice + self.system + self.idle + self.iowait + self.irq + self.softirq + self.steal
    }

    pub fn busy(&self) -> u64 {
        self.total() - self.idle - self.iowait
    }
}

pub fn calculate_cpu_load(prev: &CpuStat, curr: &CpuStat) -> f64 {
    let prev_total = prev.total();
    let curr_total = curr.total();
    let prev_busy = prev.busy();
    let curr_busy = curr.busy();

    let total_diff = curr_total - prev_total;
    let busy_diff = curr_busy - prev_busy;

    if total_diff == 0 {
        0.0
    } else {
        (busy_diff as f64 / total_diff as f64) * 100.0
    }
}