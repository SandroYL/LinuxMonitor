// 定义所有要用到的结构


#[derive(Debug)]
pub(crate) struct CPUTimes {
    name            : String,
    user            : u128,
    guest_user      : u128,
    system          : u128,
    idle            : u128,
    iowait          : u128,
    irq             : u128,
    softirq         : u128,
}

impl CPUTimes{
    pub fn new(name: String, user: u128, guest_user: u128, system: u128, idle: u128, iowait: u128, irq: u128, softirq: u128) -> CPUTimes {
        CPUTimes { 
            name, 
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

/// 保存在 /proc/meminfo中
/// 
/// 只选取了部分信息, 单位为 kB
#[derive(Debug)]
pub struct MemoryInfo {
    mem_total       : u128,
    mem_free        : u128,
    mem_available   : u128,
    buffers         : u128,
    cached          : u128,
    swap_cached     : u128,
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