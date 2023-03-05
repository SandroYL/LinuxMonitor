use core::panic;
use std::{fs::{File, self}, io::{BufReader, BufRead}, thread::sleep, time::Duration, collections::HashMap};

use chrono::{Local, Datelike, Timelike};
use sysinfo::{System, SystemExt, CpuExt, NetworkExt};

use super::structs::{CPUTimes, MemoryInfo, ACPIInfo, DeviceTemperature, DeviceVoltage, NetInfo};


pub struct TimeSeriesMonitor {}

impl TimeSeriesMonitor {

    pub fn new() ->TimeSeriesMonitor {
        TimeSeriesMonitor {  }
    }

    fn get_current_time_parse() -> String {
        let cur_time = Local::now();
    
        let mut ymd_hms:String = Default::default();
        let year = cur_time.year();
        let month = cur_time.month();
        let day = cur_time.day();
        let hour = cur_time.hour();
        let minu = cur_time.minute();
        let sec = cur_time.second();
        ymd_hms = format!("{}-{}-{}-{}-{}-{}",year,month,day,hour,minu,sec);
        
        ymd_hms
    }
    
    fn get_uptime_parse() -> String {
        let file = match File::open("/proc/uptime") {
            Ok(fp) => fp,
            Err(_) => {
                panic!("ERROR_UPTIME");
            }  
        };
        let line = BufReader::new(file).lines().nth(0);
        let ret = match line.unwrap() {
            Ok(two_times) => {
                let mut strs = two_times.split(' ');
                strs.nth(0).unwrap().to_string()
            },
            Err(_) => {
                "NotFound!".to_string()
            }
        };
        ret
    }
    
    fn cpu_time_info() -> Vec<CPUTimes> {
        let file = match File::open("/proc/stat") {
            Ok(fp) => fp,
            Err(_) => {
                panic!("ERROR_CPUINFO");
            }  
        };
        let lines = BufReader::new(file).lines();
        let mut cpu_time_info_vecs:Vec<CPUTimes> = Vec::new();
        for line in lines.take(33) {
            let total_info = line.unwrap().replace("  ", " ");
            let mut infos = total_info.split(' ');
            let name       = infos.next().unwrap().to_string();
            let user        : u128 = infos.next().unwrap().parse().unwrap();
            let guest_user  : u128 = infos.next().unwrap().parse().unwrap();
            let system      : u128 = infos.next().unwrap().parse().unwrap();
            let idle        : u128 = infos.next().unwrap().parse().unwrap();
            let iowait      : u128 = infos.next().unwrap().parse().unwrap();
            let irq         : u128 = infos.next().unwrap().parse().unwrap();
            let softirq     : u128 = infos.next().unwrap().parse().unwrap();
            cpu_time_info_vecs.push(CPUTimes::new(
                name, 
                user, 
                guest_user, 
                system, 
                idle, 
                iowait, 
                irq, 
                softirq,
            ));
        }
        cpu_time_info_vecs
    }
    
    /// 内存模块部分重要信息
    /// 
    /// 详细请见 Instrucment 内的部分说明
    fn sys_mem_info() -> MemoryInfo {
        let file = match File::open("/proc/meminfo") {
            Ok(fp) => fp,
            Err(_) => {
                panic!("ERROR_MEMINFO");
            }  
        };
        let lines = BufReader::new(file).lines();
        let (mut mem_total, mut mem_free, mut mem_available, mut buffers, mut cached, mut swap_cached) = (0, 0, 0, 0, 0, 0);
        for line in lines.take(6) {
            let total_info = line.unwrap().replace(" ", "");
            let mut infos = total_info.split(':');
            match infos.next() {
                Some("MemTotal") => mem_total = infos.next()
                                                    .unwrap()
                                                    .chars()
                                                    .filter(|ch| ch.is_numeric())
                                                    .collect::<String>()
                                                    .parse::<u128>()
                                                    .unwrap(),
                Some("MemFree") => mem_free = infos.next()
                                                    .unwrap()
                                                    .chars()
                                                    .filter(|ch| ch.is_numeric())
                                                    .collect::<String>()
                                                    .parse::<u128>()
                                                    .unwrap(),
                Some("MemAvailable") => mem_available = infos.next()
                                                .unwrap()
                                                .chars()
                                                .filter(|ch| ch.is_numeric())
                                                .collect::<String>()
                                                .parse::<u128>()
                                                .unwrap(),
                Some("Buffers") => buffers = infos.next()
                                                .unwrap()
                                                .chars()
                                                .filter(|ch| ch.is_numeric())
                                                .collect::<String>()
                                                .parse::<u128>()
                                                .unwrap(),
                Some("Cached") => cached = infos.next()
                                                .unwrap()
                                                .chars()
                                                .filter(|ch| ch.is_numeric())
                                                .collect::<String>()
                                                .parse::<u128>()
                                                .unwrap(),
                Some("SwapCached") => swap_cached = infos.next()
                                                    .unwrap()
                                                    .chars()
                                                    .filter(|ch| ch.is_numeric())
                                                    .collect::<String>()
                                                    .parse::<u128>()
                                                    .unwrap(),
                Some(_) => panic!("Memory parse Error line 141"),
                None => panic!("Memory parse Error line 142"),
            }
        }
        MemoryInfo::new(
            mem_total, 
            mem_free, 
            mem_available, 
            buffers, 
            cached, 
            swap_cached,
        )
    }
    
    fn acpi_info() -> Vec<ACPIInfo> {
        let file = match File::open("/proc/acpi/wakeup") {
            Ok(fp) => fp,
            Err(_) => {
                panic!("ERROR_ACPI");
            }  
        };
        let mut lines = BufReader::new(file).lines();
        let mut acpi_info_vec: Vec<ACPIInfo> = Vec::new();
        lines.next();
        for line in lines {
            let line = line.unwrap();
            let line = line.split_ascii_whitespace();
            let (mut device, mut s_state, mut status): (String, String, String) = (Default::default(), Default::default(), Default::default());
            let mut count = 0;
            for info in line {
                match count {
                    0 => {
                        count += 1;
                        device = info.to_string();
                    },
                    1 => {
                        count += 1;
                        s_state = info.to_string();
                    }
                    2 => {
                        count = 3;
                        status = info.to_string();
                    }
                    _ => {}
                }
            }
            acpi_info_vec.push(ACPIInfo::new (
                device,
                s_state,
                status,
            ));
        }
        acpi_info_vec
    } 
    
    /// 可以挖掘的温度检测模块
    /// 
    /// 以vec中device_name,volt形式返回
    fn temperature_info() -> Vec<DeviceTemperature> {
        let mut temperatures: Vec<DeviceTemperature> = Vec::new();
    
        //for core
        for i in 1..=64 {
            let device = format!("core{}", i);
            let file_path = format!("/sys/class/hwmon/hwmon0/temp{}_input", i);
            let data = match fs::read_to_string(file_path) {
                Ok(infos) => infos,
                Err(_) => break,
            };
            temperatures.push(DeviceTemperature::new(
                device,
                data.trim_end().parse::<i64>().unwrap()
            ));
        }
        temperatures
    }
    
    /// 可以挖掘的电压检测模块
    fn voltage_info() -> Vec<DeviceVoltage> {
        let mut voltages: Vec<DeviceVoltage> = Vec::new();
        // for fans
        for i in 1..=32 {
            let device = format!("fans{}", i);
            let file_path = format!("/sys/class/hwmon/hwmon1/fan{}_input", i);
            let data = match fs::read_to_string(file_path) {
                Ok(infos) => infos,
                Err(_) => break,
            };
            voltages.push(DeviceVoltage::new(
                device,
                data.trim_end().parse::<i64>().unwrap()
            ));
        }
        voltages
    }
    
    /// cpu使用率
    fn cpu_usage_info(system:&mut System) -> Vec<f32>  {
        system.refresh_cpu();
        let mut ret:Vec<f32> = Vec::new();
        for cpu in system.cpus() {
            ret.push(cpu.cpu_usage());
        }
        ret
    }
    
    /// 网络流量
    fn net_info(system:&mut System) -> Vec<NetInfo> {
        system.refresh_networks_list();
        let mut name_rxptx: HashMap<String,(u64,u64)> = HashMap::new();
        for (interface_name, data) in system.networks() {
            name_rxptx.entry(interface_name.to_string()).or_insert((data.received(), data.transmitted()));
        }
    
        sleep(Duration::from_secs(1));
        system.refresh_networks_list();
        for (interface_name, data) in system.networks() {
            name_rxptx.entry(interface_name.to_string()).and_modify(|rx_tx| {
                rx_tx.0 = data.received() - rx_tx.0;
                rx_tx.1 = data.transmitted() - rx_tx.1;
            });
        }
        let mut NetInfos: Vec<NetInfo> = Vec::new();
        for (interface_name, (rx, tx)) in name_rxptx.into_iter() {
            NetInfos.push(NetInfo::new(interface_name, rx as f64 / 1024.0, tx as f64 / 1024.0));
        }
        NetInfos
    }
}

