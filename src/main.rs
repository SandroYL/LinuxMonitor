use std::{fs::File, io::{BufReader, BufRead}};

pub fn main() {
    let file = File::open("/proc/sys/kernel/hostname");
    let reader = BufReader::new(file.unwrap());
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
    println!("Hello World!");
}

