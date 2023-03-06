use crate::TimeCollector::Collectors;


pub mod TimeCollector;

pub fn main() {
    let mut collector = Collectors::new();
    match collector.get_static_data() {
        Ok(_) => {println!("OK");}
        Err(err) => {println!("{:?}", err);}
    }
    match collector.get_timeseries_data(5, 10) {
        Ok(_) => {println!("OK");}
        Err(err) => {println!("{:?}", err);}
    }
    println!("Hello World!");
}

