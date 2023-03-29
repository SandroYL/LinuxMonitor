// 定义所有要用到的结构

use std::fmt;

pub struct CPUTimes {
    pub device          : String,
    pub user            : u128,
    pub guest_user      : u128,
    pub system          : u128,
    pub idle            : u128,
    pub iowait          : u128,
    pub irq             : u128,
    pub softirq         : u128,
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
    mem_total       : u128,
    mem_free        : u128,
    mem_available   : u128,
    buffers         : u128,
    cached          : u128,
    swap_cached     : u128,
}
/// 详情见instrucment.md内说明
pub struct ACPIInfo {
    device          : String,
    s_state         : String,
    status          : String,
}

/// 对设备温度监控
/// 
/// 结果保存在Vec中，0-16为核心温度，17-19为风扇温度
pub struct DeviceTemperature {
    pub device          : String,
    pub temperature     : i64,
}

pub struct DeviceVoltage {
    device          : String,
    voltage         : i64,
}

pub struct NetInfo {
    pub device: String,
    pub iospeed: f64,
}

impl NetInfo {
    pub fn new(device: String, iospeed: f64) -> NetInfo {
        NetInfo { 
            device, 
            iospeed, 
        }
    }
    pub fn get_name(&self) -> String {
        self.device.clone()
    }
    pub fn get_speed(&self) -> f64 {
        self.iospeed
    }

}

impl MemoryInfo {
    pub fn new (mem_total: u128, mem_free: u128, mem_available: u128, buffers: u128, cached: u128, swap_cached: u128) -> MemoryInfo {
        MemoryInfo { 
            mem_total, 
            mem_free, 
            mem_available, 
            buffers, 
            cached, 
            swap_cached,
        }
    }
    //get mem_total
        
}
impl fmt::Debug for MemoryInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "总内存大小: {}GB, 空闲内存大小: {}GB, 可用内存大小: {}GB, buffer缓存: {}MB, cache缓存: {}MB, swap_cached:{}MB",self.mem_total / 1024 / 1024, self.mem_free / 1024 / 1024, self.mem_available / 1024 / 1024, self.buffers / 1024, self.cached / 1024, self.swap_cached / 1024)
    }
}



impl CPUTimes {
    pub fn new(device: String, user: u128, guest_user: u128, system: u128, idle: u128, iowait: u128, irq: u128, softirq: u128) -> CPUTimes {
        CPUTimes { 
            device, 
            user, 
            guest_user, 
            system, 
            idle, 
            iowait, 
            irq, 
            softirq, 
        }
    }   
}
impl fmt::Debug for CPUTimes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CPU编号: {}, 用户态时间: {}s, 用户态低优先级时间: {}s, 系统态时间: {}s, 空闲态时间: {}s, IO等待时间: {}s, 硬中断时间: {}s, 软中断时间: {}s", self.device, self.user, self.guest_user, self.system, self.idle, self.iowait, self.irq, self.softirq)
    }
}


impl ACPIInfo {
    pub fn new (device: String, s_state: String, status: String) -> ACPIInfo {
        ACPIInfo { 
            device, 
            s_state, 
            status, 
        }
    }

}

impl DeviceTemperature {
    pub fn new (device: String, temperature: i64) -> DeviceTemperature {
        DeviceTemperature { 
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

impl fmt::Debug for DeviceTemperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "设备名: {}, 设备温度: {}°C", self.get_name(), self.get_temperature() / 1000)
    }
}

impl DeviceVoltage {
    pub fn new (device: String, voltage: i64) -> DeviceVoltage {
        DeviceVoltage { 
            device, 
            voltage,
        }
    }
}

impl fmt::Debug for DeviceVoltage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "设备名: {}, 设备电压: {}mV", self.device, self.voltage)
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
impl fmt::Debug for CPUInfos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "processor:{},vendor_id:{},cpu_family:{},model:{},model_name:{},stepping:{},microcode:{},cpu_mhz:{},cache_size_kb:{}", self.processor, self.vendor_id, self.cpu_family, self.model, self.model_name, self.stepping, self.microcode, self.cpu_mhz, self.cache_size_kb)
    }
}

impl fmt::Debug for ACPIInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "设备名称: {}, 设备状态: {}, 可用或断电: {}", self.device, self.s_state, self.status)
    }
}