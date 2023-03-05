//用来收集所有的数据，并永久化

use std::{fs::File, io::Write};

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
        match File::create("/home/guest/src/inside") {
            Ok(fp) => {file = fp;}
            Err(error) => {return Err(error);}
        };
        file.write(b"123")?;
        file.write_all(device_name.as_bytes())?;
        file.write_all(sys_core_type.as_bytes())?;
        file.write(b"\n")?;
        for info in cpu_infos.into_iter() {
            file.write(format!("{:?}", info).as_bytes())?;
            file.write(b"\n")?;
        }
        Ok(())
    }
}