//用来收集所有的数据，并永久化

use std::{fs::File, io::Write, time::Duration};

use LinuxMonitor::monitor_tools::{static_data_monitor::StaticDataMonitor, time_serise_monitor::TimeSeriesMonitor};
use sysinfo::{System, SystemExt};

pub struct Collectors {
    system: System,
    static_datas: StaticDataMonitor,
    time_datas: TimeSeriesMonitor,
}

impl Collectors {
    pub fn new() -> Collectors {
        Collectors { 
            system: System::new_all(),
            static_datas: StaticDataMonitor::new(),
            time_datas: TimeSeriesMonitor::new(),
        }
    }

    pub fn get_static_data(&self) -> std::io::Result<()> {
        let device_name = self.static_datas.get_device_name();
        let sys_core_type = self.static_datas.get_sys_core_type();
        let cpu_infos = self.static_datas.get_cpu_infos();
        let mut file:File; 
        match File::create("/home/guest/src/static") {
            Ok(fp) => {file = fp;}
            Err(error) => {return Err(error);}
        };
        file.write_all(device_name.as_bytes())?;
        file.write(b"\n")?;
        file.write_all(sys_core_type.as_bytes())?;
        file.write(b"\n")?;
        for info in cpu_infos.into_iter() {
            file.write(format!("{:?}", info).as_bytes())?;
            file.write(b"\n")?;
        }
        Ok(())
    }
    pub fn get_timeseries_data(&mut self, update_time: u32, refresh_times: usize) -> std::io::Result<()> {
        let mut file:File; 
        match File::create("/home/guest/src/timeseries") {
            Ok(fp) => {file = fp;}
            Err(error) => {return Err(error);}
        };

        for i in 0..refresh_times {
            let current_time = self.time_datas.get_current_time_parse();
            let uptime = self.time_datas.get_uptime_parse();
            let cputime_info = self.time_datas.cpu_time_info();
            let sys_mem = self.time_datas.sys_mem_info();
            let acpi_info = self.time_datas.acpi_info();
            let temperature_info = self.time_datas.temperature_info();
            let voltage_info = self.time_datas.voltage_info();
            let cpu_usage_info = self.time_datas.cpu_usage_info(&mut self.system);
            let net_info = self.time_datas.net_info(&mut self.system);

            file.write_all(format!("现在时间: {}\n", current_time).as_bytes())?;
            file.write_all(format!("已运行时间: {}s\n", uptime).as_bytes())?;
            file.write_all(format!("相关内存信息: {:?}\n", sys_mem).as_bytes())?;
            for ctimes in cputime_info.into_iter().zip(cpu_usage_info) {
                file.write_all(format!("CPUTime | {:?} | CPU利用率: {}%\n", ctimes.0, ctimes.1).as_bytes())?;
            }
            file.write_all(b"\n")?;
            for acpi in acpi_info {
                file.write_all(format!("ACPI | {:?}\n", acpi).as_bytes())?;
            }
            file.write_all(b"\n")?;
            for temperature in temperature_info.into_iter() {
                file.write_all(format!("Temperature | {:?}\n", temperature).as_bytes())?;
            }
            file.write_all(b"\n")?;
            for voltage in voltage_info.into_iter() {
                file.write_all(format!("Voltage | {:?}\n", voltage).as_bytes())?;
            }
            file.write_all(b"\n")?;
            for nets in net_info.into_iter() {
                file.write_all(format!("网络状况 | 接口\"{}\" 接收:{}B, 发送:{}B\n", nets.get_name(), nets.get_receive(), nets.get_transmit()).as_bytes())?;
            }
            file.write_all(format!("\n第 {} 次写入完成\n\n", i + 1).as_bytes())?;
            println!("第 {} 次写入完成", i + 1);
            std::thread::sleep(Duration::from_secs(update_time.into()));
        }
        Ok(())
    }
}