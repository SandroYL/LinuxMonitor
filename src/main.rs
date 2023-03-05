use crate::TimeCollector::Collectors;

pub mod TimeCollector;

pub fn main() {
    let collector:Collectors = Collectors::new();
    match collector.get_static_data() {
        Ok(_) => {println!("OK");}
        Err(err) => {println!("{:?}", err);}
    }
    println!("Hello World!");
}

