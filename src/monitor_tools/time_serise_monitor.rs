
use std::{fs::{self}, time::Duration, thread};
use sysinfo::{System, SystemExt, CpuExt, DiskExt, NetworkExt};

use super::structs::{CPUUsage, MemoryInfo, TemperatureInfo, FanInfo, NetInfo, CacheInfo, DiskInfo};


pub struct TimeSeriesMonitor {
    system: System,
}

impl TimeSeriesMonitor {

    pub fn new() ->TimeSeriesMonitor {
        let system = System::new_all();
        TimeSeriesMonitor { 
            system
        }
    }
    
    pub fn refresh(&mut self) {
        self.system.refresh_cpu();
        self.system.refresh_disks();
        self.system.refresh_memory();
        self.system.refresh_networks_list();
        self.system.refresh_system();
    }

    ///已启动时间
    pub fn boot_time(&self) -> u64 {
        self.system.boot_time()
    }

    ///cpu利用率，cpu频率
    pub fn cpu_usage(&self) -> CPUUsage {
        let cpu = self.system.global_cpu_info();
        CPUUsage {
            device: String::from(cpu.name()), 
            usage: cpu.cpu_usage(),
            freq: cpu.frequency(),
        }
    }
    
    /// 内存模块部分重要信息
    /// 
    /// 详细请见 Instrucment 内的部分说明
    pub fn mem_info(&self) -> MemoryInfo {
        let (mem_total, mem_available, mem_used) =
            (self.system.total_memory(), self.system.available_memory(),
                self.system.used_memory());
        MemoryInfo::new(
            mem_total,
            mem_available,
            mem_used,
            mem_used as f64 * 100.0 / mem_total as f64,

        )
    }
    
    pub fn cache_info(&self) -> CacheInfo {
        let (swap_total, free_total, used_total) = 
            (self.system.total_swap(), self.system.free_swap(),
                self.system.used_swap());
        CacheInfo::new(
            swap_total,
            free_total,
            used_total,
            used_total as f64 * 100.0 / swap_total as f64,
        )
    }

    
    /// 可以挖掘的温度检测模块
    /// 
    /// 以vec中device_name,volt形式返回
    pub fn temperature_info(&self) -> Vec<TemperatureInfo> {
        let mut temperatures: Vec<TemperatureInfo> = Vec::new();
        //for core
        for i in 1..=4096 {
            let device = format!("core{}", i);
            let file_path = format!("/sys/class/hwmon/hwmon0/temp{}_input", i);
            let data = match fs::read_to_string(file_path) {
                Ok(infos) => infos,
                Err(_) => break,
            };
            temperatures.push(TemperatureInfo::new(
                device,
                data.trim_end().parse::<i64>().unwrap()
            ));
        }
        temperatures
    }
    
    /// 可以挖掘的电压检测模块
    pub fn fan_info(&self) -> Vec<FanInfo> {
        let mut fans: Vec<FanInfo> = Vec::new();
        // for fans
        for i in 1..=4096 {
            let device = format!("fans{}", i);
            let file_path = format!("/sys/class/hwmon/hwmon1/fan{}_input", i);
            let data = match fs::read_to_string(file_path) {
                Ok(infos) => infos,
                Err(_) => break,
            };
            fans.push(FanInfo::new(
                device,
                data.trim_end().parse::<i64>().unwrap()
            ));
        }
        fans
    }
    
    /// 网络流量
    pub fn net_info(&self, duration: Duration) -> NetInfo {
        let nets = self.system.networks();
        let (mut recv, mut trans) = (0, 0);
        for (_, data) in nets {
            recv += data.received();
            trans += data.transmitted();
        }

        NetInfo::new(self.system.host_name().unwrap(), recv, trans, recv as f64 / duration.as_secs_f64(), trans as f64 / duration.as_secs_f64())
    }

    pub fn disk_info(&self) -> Vec<DiskInfo> {
        let disks = self.system.disks();
        disks.into_iter()
        .map(|disk| DiskInfo::new(String::from(disk.name().to_str().unwrap()), disk.total_space(), disk.available_space()))
        .collect()
    }
}