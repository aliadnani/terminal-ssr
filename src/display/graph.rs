use yansi::Paint;

use crate::telemetry::sys::SystemInfo;

fn progress_bar(progress: &f32) -> String {
    let shaded_blocks: usize = ((progress / 100_f32) * 50_f32).round() as usize;
    "█".repeat(shaded_blocks) + &"▒".repeat(50 - shaded_blocks)
}

pub fn render(info: SystemInfo) -> String {
    // `\x1B[2K` clears the line - removes 'data:'
    // `\x1B[1000D` sets cursor backwards
    // `\x1B[8A` sets the cursor up 8 lines up
    let clear_screen = format!("\x1B[2K\x1B[1000D\x1B[8A");

    let header = format!(
        "{}",
        Paint::cyan(format!(
            "\x1B[1000DDevice: {} @ {}",
            info.host_name, info.os
        )),
    );
    let uptime = format!(
        "{}",
        Paint::yellow(format!("\x1B[1000DUptime:  {}s", &info.uptime.to_string())),
    );
    let cpu_stats = format!(
        "{}\n\x1B[1000D{}",
        Paint::magenta(format!("\x1B[1000DCPU: {}%", &info.cpu_usage.to_string())),
        progress_bar(&info.cpu_usage)
    );
    let memory_stats = format!(
        "{}\n\x1B[1000D{}",
        Paint::red(format!(
            "\x1B[1000DMemory: {}%",
            &info.memory_usage.to_string()
        )),
        progress_bar(&info.memory_usage)
    );

    let graph = vec![clear_screen, header, uptime, cpu_stats, memory_stats];

    graph.join("\n")
}
