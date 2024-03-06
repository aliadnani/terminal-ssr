use std::{sync::Arc, time::Duration};

use sysinfo::System;
use tokio::{sync::Mutex, task::JoinHandle, time::sleep};

#[derive(Clone)]
pub struct SystemInfo {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub uptime: i32,
    pub os: String,
    pub kernel: String,
    pub host_name: String,
}

pub trait SystemInfoService {
    fn get_info(&self) -> SystemInfo;
}

pub struct CachingSystemInfoService {
    current_system_info: SystemInfo,
    system: Arc<Mutex<System>>,
}

impl CachingSystemInfoService {
    pub async fn new(lock_system: Arc<Mutex<System>>) -> CachingSystemInfoService {
        for _ in 0..5 {
            // Refresh several times as documentation says first readings won't be accurate
            lock_system.lock().await.refresh_all();
            sleep(Duration::from_millis(200)).await;
        }

        let current_system_info = CachingSystemInfoService::refresh_info(&lock_system).await;
        CachingSystemInfoService {
            current_system_info: current_system_info,
            system: lock_system.clone(),
        }
    }

    pub fn set_system_info(&mut self, system_info: SystemInfo) -> () {
        self.current_system_info = system_info;
    }

    pub fn schedule_refresh(this: Arc<Mutex<Self>>, refresh_interval: Duration) -> JoinHandle<()> {
        let handle = tokio::spawn(async move {
            loop {
                let this = &mut this.lock().await;
                sleep(refresh_interval).await;

                let new_system_info = CachingSystemInfoService::refresh_info(&this.system).await;

                this.set_system_info(new_system_info);
            }
        });

        handle
    }

    pub async fn refresh_info(lock_system: &Arc<Mutex<System>>) -> SystemInfo {
        let system = &mut lock_system.lock().await;

        system.refresh_all();
        SystemInfo {
            cpu_usage: system.global_cpu_info().cpu_usage(),
            // I don't think this is accurate
            memory_usage: (system.used_memory() as f32) / (system.total_memory() as f32) * 100_f32,
            os: sysinfo::System::long_os_version().unwrap(),
            kernel: sysinfo::System::kernel_version().unwrap(),
            host_name: sysinfo::System::host_name().unwrap(),
            uptime: sysinfo::System::uptime() as i32,
        }
    }
}

impl<'a> SystemInfoService for CachingSystemInfoService {
    fn get_info(&self) -> SystemInfo {
        self.current_system_info.clone()
    }
}
