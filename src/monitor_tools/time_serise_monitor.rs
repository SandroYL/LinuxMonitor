use std::{fs::File, io::{BufReader, BufRead}, fmt::Error};

use chrono::{Local, Datelike, Timelike, DateTime};


/// 返回系统时间
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
/// 返回格式为(second)
/// 若未得到则返回NOTFOUND
fn get_uptime_parse() -> String {
    let file = match File::open("/proc/uptime") {
        Ok(fp) => fp,
        Err(_) => {
            panic!("ERROR");
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
/// 详情可见 Instrucment 内的部分说明
fn cpu_time_info() {
    
}