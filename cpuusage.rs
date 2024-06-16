use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

fn get_cpu_times() -> io::Result<(u64, u64)> {
    let path = Path::new("/proc/stat");
    let file = File::open(&path)?;
    let mut lines = io::BufReader::new(file).lines();

    if let Some(Ok(line)) = lines.next() {
        let mut parts = line.split_whitespace();
        if parts.next() == Some("cpu") {
            let user: u64 = parts.next().unwrap_or("0").parse().unwrap_or(0);
            let nice: u64 = parts.next().unwrap_or("0").parse().unwrap_or(0);
            let system: u64 = parts.next().unwrap_or("0").parse().unwrap_or(0);
            let idle: u64 = parts.next().unwrap_or("0").parse().unwrap_or(0);
            let iowait: u64 = parts.next().unwrap_or("0").parse().unwrap_or(0);
            let irq: u64 = parts.next().unwrap_or("0").parse().unwrap_or(0);
            let softirq: u64 = parts.next().unwrap_or("0").parse().unwrap_or(0);
            let steal: u64 = parts.next().unwrap_or("0").parse().unwrap_or(0);

            let total = user + nice + system + idle + iowait + irq + softirq + steal;

            return Ok((idle, total));
        }
    }
    Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to read CPU times"))
}

fn calculate_cpu_usage(prev_idle: u64, prev_total: u64, curr_idle: u64, curr_total: u64) -> f64 {
    let diff_idle = curr_idle - prev_idle;
    let diff_total = curr_total - prev_total;
    1.0 - (diff_idle as f64 / diff_total as f64)
}

fn main() -> io::Result<()> {
    let (prev_idle, prev_total) = get_cpu_times()?;

    sleep(Duration::from_secs(1));

    let (curr_idle, curr_total) = get_cpu_times()?;

    let cpu_usage = calculate_cpu_usage(prev_idle, prev_total, curr_idle, curr_total) * 100.0;
    println!("CPU Usage: {:.2}%", cpu_usage);

    Ok(())
}