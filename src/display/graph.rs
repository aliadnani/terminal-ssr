use yansi::{Color, Paint};

use crate::telemetry::sys::SystemInfo;

fn progress_bar(progress: &f32) -> String {
    let shaded_blocks: usize = ((progress / (100 as f32)) * (50 as f32)).round() as usize;
    "█".repeat(shaded_blocks) + &"▒".repeat(50 - shaded_blocks)
}

pub fn render(info: SystemInfo) -> String {
    let header = format!(
        "{}",
        Paint::cyan(format!("Device: {}@{}", info.host_name, info.os)),
    );
    let uptime = format!(
        "{}",
        Paint::yellow(format!("Uptime:  {}s", &info.uptime.to_string())),
    );
    let cpu_stats = format!(
        "{} {}",
        Paint::white(format!("CPU:    {}%", &info.cpu_usage.to_string()[0..4])).bg(Color::Magenta),
        progress_bar(&info.cpu_usage)
    );
    let memory_stats = format!(
        "{} {}",
        Paint::white(format!("Memory: {}%", &info.memory_usage.to_string()[0..4])).bg(Color::Red),
        progress_bar(&info.memory_usage)
    );

    let graph = vec![header, uptime, cpu_stats, memory_stats];

    graph.join("\n")
}
