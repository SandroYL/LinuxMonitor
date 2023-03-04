use std::{fs::File, io::{BufReader, BufRead}};
use super::structs::CPUInfos;

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

fn get_cpu_infos() -> Vec<CPUInfos> {
    let file = File::open("/proc/cpuinfo");
    let reader = match file {
        Ok(fp) => BufReader::new(fp),
        Err(_) => panic!("NOTFOUND_CPUINFOS")
    };
    let lines = reader.lines();
    let mut cpu_infos:Vec<CPUInfos> = Vec::new();
    let (mut processor, mut vendor_id, mut cpu_family, mut model, mut model_name, mut stepping, mut microcode, mut cpu_mhz, mut cache_size_kb):
    (usize, String, usize, usize, String, usize, u128, f32, u128) 
    = (0, Default::default(), 0, 0, Default::default(), 0, 0, 0.0, 0);
    for line in lines {
        match line {
            Ok(info) => {
                let mut info_split = info.split(':');
                let t = info_split.next().unwrap().trim_end();
                match t {
                    "processor" => {processor = info_split.next().unwrap().trim().parse::<usize>().unwrap();},
                    "vendor_id" => {vendor_id = info_split.next().unwrap().trim().to_string();},
                    "cpu_family" => {cpu_family = info_split.next().unwrap().trim().parse::<usize>().unwrap();},
                    "model" => {model = info_split.next().unwrap().trim().parse::<usize>().unwrap();},
                    "model name" => {model_name = info_split.next().unwrap().trim().to_string();},
                    "stepping" => {stepping = info_split.next().unwrap().trim().parse::<usize>().unwrap();},
                    "microcode" => {microcode = u128::from_str_radix(info_split.next().unwrap().trim().trim_start_matches("0x"), 16).unwrap();},
                    "cpu Mhz" => {cpu_mhz = info_split.next().unwrap().trim().parse::<f32>().unwrap();},
                    "cache size" => {cache_size_kb = info_split.next().unwrap().trim().trim_end_matches(" KB").parse::<u128>().unwrap();},
                    "" => {cpu_infos.push(CPUInfos::new(processor, vendor_id.clone(), cpu_family, model, model_name.clone(), stepping, microcode, cpu_mhz, cache_size_kb))},
                    _ => {},

                }
            },
            Err(_) => panic!("ERROR CPUINFOS PARSING"),
        }
    }
    cpu_infos
}
