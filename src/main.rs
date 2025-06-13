use colored::*;
use sysinfo::System;
use whoami::{arch, distro, username};

fn main() {
    {
        // USERNAME and HOSTNAME
        let hostname = std::fs::read_to_string("/etc/hostname")
            .unwrap_or("Unknown".into())
            .trim()
            .to_string();

        let username_hostname = format!("{}@{}", username(), hostname);

        println!("{}", username_hostname.cyan().bold());
    }

    {
        // KERNEL
        let kernel = System::kernel_version().unwrap_or("Unknown".into());
        print_formatted("Kernel", &kernel);
    }

    {
        // DISTRO and ARCHITECTURE
        let distro: String = format!("{} ({})", distro(), arch());
        print_formatted("OS", &distro);
    }

    {
        // CPU and CORES
        let cpuinfo = std::fs::read_to_string("/proc/cpuinfo").unwrap_or("Unknown".into());

        let cpu = cpuinfo
            .lines()
            .find(|l| l.starts_with("model name"))
            .map(|line| line.split(':').nth(1).unwrap_or("").trim())
            .unwrap_or("Unknown");

        print_formatted("CPU", &cpu.to_string());
    }

    {
        // SHELL
        let shell = std::env::var("SHELL").unwrap_or("Unknown".into());
        print_formatted("Shell", &shell);
    }

    {
        // DE / WM
        let we = std::env::var("DESKTOP_SESSION").unwrap_or("Unknown".into());
        print_formatted("DE/WM", &we);
    }

    {
        // UPTIME
        let uptime: u64 = System::uptime(); // Uptime in Seconds

        let uptime_minutes: u64 = &uptime / 60 % 60; // Seconds/60 are the total minutes, and the modulus takes out the hours.
        let uptime_hours: u64 = &uptime / 3600; // Seconds / # of Seconds in an hour
        let uptime_formatted = format!("{} hours, {} minutes", uptime_hours, uptime_minutes);

        print_formatted("Uptime", &uptime_formatted);
    }

    {
        if let Some((used, total, percent)) = get_memory() {
            let memory_formatted = format!("{:.2} GiB / {:.2} GiB ({:.1}%)", used, total, percent);
            print_formatted("Memory", &memory_formatted);
        } else {
            println!("Memory: Unknown");
        }
    }
}

fn print_formatted(name: &str, vals: &String) {
    // For Easy Reformating/Config
    println!("{}: {}", name.bold(), vals);
}

fn get_memory() -> Option<(f32, f32, f32)> {
    // Reads /proc/meminfo and stores
    let contents = std::fs::read_to_string("/proc/meminfo").ok()?;

    // Variables for Total and Available Memory
    let mut total = 0.0;
    let mut available = 0.0;

    // Store MemTotal and MemAvailable in their Variables
    for line in contents.lines() {
        if line.starts_with("MemTotal:") {
            total = line.split_whitespace().nth(1)?.parse::<f32>().ok()?;
        } else if line.starts_with("MemAvailable:") {
            available = line.split_whitespace().nth(1)?.parse::<f32>().ok()?;
        }

        // Break after both are saved
        if total > 0.0 && available > 0.0 {
            break;
        }
    }

    // Calculate Used and Percent Used
    let used = total - available;
    let percent = (used / total) * 100.0;

    // Return wrapped vars, from KiB to GiB
    Some((used / 1024.0 / 1024.0, total / 1024.0 / 1024.0, percent))
}
