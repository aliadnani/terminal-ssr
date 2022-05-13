use sysinfo::{ProcessorExt, System, SystemExt};

pub struct SystemInfo {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub uptime: i32,
    pub os: String,
    pub kernel: String,
    pub host_name: String,
}

pub fn get_info(system: &mut System) -> SystemInfo {
    system.refresh_all();
    SystemInfo {
        cpu_usage: system.global_processor_info().cpu_usage(),
        // I don't think this is accurate
        memory_usage: (system.used_memory() as f32) / (system.total_memory() as f32) * 100_f32,
        os: system.long_os_version().unwrap(),
        kernel: system.kernel_version().unwrap(),
        host_name: system.host_name().unwrap(),
        uptime: system.uptime() as i32,
    }
}
