use std::env;

#[derive(Debug)]
pub struct Config {
    pub server_host: String,
    pub sysinfo_refresh_interval_ms: i32,
    pub sse_refresh_interval: i32,
    pub rust_log: String,
}

impl Config {
    pub fn load_config_from_env_with_defaults() -> Config {
        let server_host = env::var("SERVER_HOST").unwrap_or("0.0.0.0:8910".to_string());
        let sysinfo_refresh_interval_ms = env::var("SYS_INFO_REFRESH_INTERVAL")
            .ok()
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(250);
        let sse_refresh_interval = env::var("SSE_REFRESH_INTERVAL")
            .ok()
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(1000);
        let rust_log = env::var("RUST_LOG")
            .unwrap_or("terminal_ssr=debug,tower_http=debug,axum::rejection=trace".to_string());

        Config {
            server_host,
            sse_refresh_interval,
            sysinfo_refresh_interval_ms,
            rust_log,
        }
    }
}
