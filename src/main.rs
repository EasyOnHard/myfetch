use colored::*;
use sysinfo::System;
use whoami::{arch, distro, username};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    // println!("{}", "MyFetch".blue());

    {
        // HOSTNAME
        let hostname = format!(
            "{}@{}",
            username(),
            System::host_name().unwrap_or("Unknown".into())
        );

        println!("{}", hostname.cyan().bold());
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
        // SHELL
        let shell = std::env::var("SHELL").unwrap_or("Unknown".into());
        print_formatted("Shell", &shell);
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
        // MEMORY
        let total_memory = sys.total_memory() as f32;
        let used_memory = sys.used_memory() as f32;
        let percent_used = (used_memory / total_memory) * 100.0; // Get percent used

        // Bytes -> Gigabytes
        let total_memory = total_memory / 1024.0 / 1024.0 / 1024.0;
        let used_memory = used_memory / 1024.0 / 1024.0 / 1024.0;

        // Compile into one String
        let memory_formatted = format!(
            "{:.2} GiB / {:.2} GiB ({:.1}%)",
            used_memory, total_memory, percent_used
        );

        print_formatted("Memory", &memory_formatted);
    }
}

fn print_formatted(name: &str, vals: &String) {
    println!("{}: {}", name.bold(), vals);
}
