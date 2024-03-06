use yansi::Paint;

use crate::telemetry::sys::SystemInfo;

fn progress_bar(progress: &f32) -> String {
    let shaded_blocks: usize = ((progress / 100_f32) * 50_f32).round() as usize;
    "█".repeat(shaded_blocks) + &"▒".repeat(50 - shaded_blocks)
}

fn clear_line_string(num_lines: u16) -> String {
    format!(
        "{}{}",
        ansi_escapes::EraseLine,
        ansi_escapes::EraseLines(num_lines)
    )
}

/// Generate the system info string to be printed to stdout
/// Also clears out the previous system info using ANSI escape codes
pub fn render(info: SystemInfo) -> String {
    // This doesn't work within docker :(
    let header = format!("{}", format!("Device: {} @ {}", info.host_name, info.os));
    let header = header.cyan();

    let uptime = format!(
        "{}",
        format!(
            "Uptime: {}s ≈ {:.5}hr ≈ {:.5}d",
            &info.uptime.to_string(),
            ((info.uptime as f32) / 3600.0).to_string(),
            ((info.uptime as f32) / (3600.0 * 24.0)).to_string()
        ),
    );
    let uptime = uptime.yellow();

    let cpu_text = format!("{}", format!("CPU: {:.5}%", &info.cpu_usage.to_string()),);
    let cpu_text = cpu_text.magenta();

    let cpu_bar = progress_bar(&info.cpu_usage);
    let cpu_bar = cpu_bar.white();

    let memory_text = format!(
        "{}",
        format!("Memory: {:.5}%", &info.memory_usage.to_string())
    );
    let memory_text = memory_text.red();

    let memory_bar = progress_bar(&info.memory_usage);
    let memory_bar = memory_bar.white();

    let graph = vec![header, uptime, cpu_text, cpu_bar, memory_text, memory_bar];

    let mut graph = graph
        .iter()
        .map(|line| {
            format!(
                "{}{}{}",
                ansi_escapes::EraseLine,
                ansi_escapes::CursorLeft,
                line
            )
        })
        .collect::<Vec<String>>();

    graph[0] = format!(
        "{}{}",
        // Clear the extra 2 lines cURL puts between SSE data
        clear_line_string((graph.len() as u16) + 2),
        graph[0]
    );

    graph.join("\n")
}
