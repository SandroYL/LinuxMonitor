// 定义所有要用到的结构

#[derive(Debug)]
pub struct CPUTimes {
    device          : String,
    user            : u128,
    guest_user      : u128,
    system          : u128,
    idle            : u128,
    iowait          : u128,
    irq             : u128,
    softirq         : u128,
}

#[derive(Debug)]
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
    device          : String,
    temperature     : i64,
}

pub struct DeviceVoltage {
    device          : String,
    voltage         : i64,
}

pub struct NetInfo {
    interface_name: String,
    d_receive: f64,
    d_transmit: f64,
}

impl NetInfo {
    pub fn new(interface_name: String, d_receive: f64, d_transmit: f64) -> NetInfo {
        NetInfo { 
            interface_name, 
            d_receive, 
            d_transmit, 
        }
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
}

impl CPUTimes{
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
}

impl DeviceVoltage {
    pub fn new (device: String, voltage: i64) -> DeviceVoltage {
        DeviceVoltage { 
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