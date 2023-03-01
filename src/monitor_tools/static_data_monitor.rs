use std::{fs::File, io::{BufReader, BufRead}};


fn get_device_name() -> String {
    let file = File::open("/proc/sys/kernel/hostname");
    let reader = BufReader::new(file.unwrap());
    let file_name =  reader.lines().nth(0).unwrap().unwrap();
    file_name    
}

/// 保存在/proc/sys/kernel/ostype和/proc/sys/kernel/osrelease内
/// 
/// 预期格式为Linux-5.4.0-144-generic
fn get_sys_core_type() -> String {
    let file = File::open("/proc/sys/kernel/ostype");
    let reader = match file {
        Ok(fp) => BufReader::new(fp),
        Err(_) => panic!("NOTFOUND_SYSTYPE")
    };
    let os_type = reader.lines().next().unwrap().unwrap();

    let file = File::open("/proc/sys/kernel/osrelease");
    let reader = match file {
        Ok(fp) => BufReader::new(fp),
        Err(_) => panic!("NOTFOUND_SYSRELEASE")
    };
    let os_release = reader.lines().next().unwrap().unwrap();

    format!("{}-{}", os_type, os_release)
}

