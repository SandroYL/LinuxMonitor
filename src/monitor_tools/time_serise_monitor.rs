use core::panic;
use std::{fs::{File, self}, io::{BufReader, BufRead}, mem::replace};

use chrono::{Local, Datelike, Timelike};

use super::structs::{CPUTimes, MemoryInfo, ACPIInfo, DeviceTempratures};


/// 返回系统时间
/// 
/// 以(y-m-d-h-m-s)的形式
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

/// 保存在/proc/uptime 文件中
/// 
/// 系统运行时间
/// 
/// 返回格式为(second)
/// 
/// 若未得到则返回NOTFOUND
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

/// cpu 部分时间检测
/// 
/// 详情可见 Instrucment 内的部分说明
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

/// acpi模块部分重要信息，
/// 以vec形式返回
/// 
/// 详情可见 Instrucment
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
/// 以vec中device_name,temp形式返回
fn temperature_info() -> Vec<DeviceTempratures> {
    let mut temperatures: Vec<DeviceTempratures> = Vec::new();

    //for core
    for i in 1..=64 {
        let device = format!("core{}", i);
        let file_path = format!("/sys/class/hwmon/hwmon0/temp{}_input", i);
        let data = match fs::read_to_string(file_path) {
            Ok(infos) => infos,
            Err(_) => break,
        };
        temperatures.push(DeviceTempratures::new(
            device,
            data.trim_end().parse::<i64>().unwrap()
        ));
    }
    // for fans
    for i in 1..=32 {
        let device = format!("fans{}", i);
        let file_path = format!("/sys/class/hwmon/hwmon1/fan{}_input", i);
        let data = match fs::read_to_string(file_path) {
            Ok(infos) => infos,
            Err(_) => break,
        };
        temperatures.push(DeviceTempratures::new(
            device,
            data.trim_end().parse::<i64>().unwrap()
        ));
    }
    temperatures
}