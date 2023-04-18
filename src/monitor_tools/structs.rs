// 定义所有要用到的结构

pub struct CPUUsage {
    pub device          : String,
    pub usage           : f32,
    pub freq            : u64,
}

pub struct CPUInfos {
    processor       : usize,
    vendor_id       : String,
    cpu_family      : usize,
    model           : usize,
    model_name      : String,
    stepping        : usize,
    microcode       : u128,
    cpu_mhz         : f32,
    cache_size_kb   : u128,
}
/// 保存在 /proc/meminfo中
/// 
/// 只选取了部分信息, 单位为 kB
pub struct MemoryInfo {
    pub mem_total       : u64,
    pub mem_available   : u64,
    pub mem_used        : u64,
    mem_usage       : f64,
}
pub struct CacheInfo {
    pub swap_total      : u64,
    pub free_total      : u64,
    pub used_total      : u64,
    usage           : f64,
}

pub struct DiskInfo {
    pub disk_name       : String,
    pub space_total     : u64,
    pub space_available : u64,
}

/// 对设备温度监控
/// 
pub struct TemperatureInfo {
    pub device          : String,
    pub temperature     : i64,
}

pub struct FanInfo {
    pub device          : String,
    pub voltage         : i64,
}

pub struct NetInfo {
    pub device: String,
    pub recv: u64,
    pub trans: u64,
    pub speed_recv: f64,
    pub speed_trans: f64,
}

impl DiskInfo {
    pub fn new(disk_name: String, space_total: u64, space_available: u64) -> DiskInfo {
        DiskInfo { 
            disk_name,
            space_total,
            space_available,
        }
    }
}

impl NetInfo {
    pub fn new(device: String, recv: u64, trans: u64, speed_recv: f64, speed_trans: f64) -> NetInfo {
        NetInfo { 
            device, 
            recv,
            trans,
            speed_recv,
            speed_trans,
        }
    }
}

impl MemoryInfo {
    pub fn new (mem_total: u64, mem_available: u64, mem_used: u64, mem_usage: f64,) -> MemoryInfo {
        MemoryInfo { 
            mem_total, 
            mem_available, 
            mem_used, 
            mem_usage,
        }
    }
}

impl CacheInfo {
    pub fn new(swap_total: u64, free_total: u64, 
        used_total: u64, usage: f64) -> CacheInfo {
            CacheInfo { 
                swap_total, 
                free_total,
                used_total, 
                usage, 
            }
    }
}

impl CPUUsage {
    pub fn new(device: String, usage: f32, freq: u64) -> CPUUsage {
        CPUUsage { 
            device, 
            usage,
            freq,
        }
    }   
}




impl TemperatureInfo {
    pub fn new (device: String, temperature: i64) -> TemperatureInfo {
        TemperatureInfo { 
            device, 
            temperature,
        }
    }
    pub fn get_name(&self) -> String {
        self.device.clone()
    }
    pub fn get_temperature(&self) -> i64 {
        self.temperature
    }
}


impl FanInfo {
    pub fn new (device: String, voltage: i64) -> FanInfo {
        FanInfo { 
            device, 
            voltage,
        }
    }
}


impl CPUInfos {
    pub fn new (processor: usize, vendor_id: String, cpu_family: usize, model: usize,
    model_name: String, stepping: usize, microcode: u128, cpu_mhz: f32, cache_size_kb: u128) -> CPUInfos {
        CPUInfos { 
            processor, 
            vendor_id,
            cpu_family, 
            model,
            model_name,
            stepping,
            microcode,
            cpu_mhz,
            cache_size_kb, 
        }
    }
}
