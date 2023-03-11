
use std::{thread, time::Duration};

use TimeCollector::Collectors;
use chrono::{Utc};
use sysinfo::{System, SystemExt};
use tokio_postgres::{NoTls, Error, Client};
pub mod TimeCollector;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // 连接到数据库中
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    //准备数据
    match write_data(&client).await {
            Ok(_) => {println!("success")},
            Err(e) => {eprintln!("{}", e)},
    }
    Ok(())
}


async fn write_data(client: &Client) -> Result<(), tokio_postgres::Error> {
    let mut system = System::new_all();
        for i in 1..=1000 { //十秒一次
            system.refresh_all();
            let timestamp = Utc::now();
            let Collectors = TimeCollector::Collectors::new();
            let cpu_infos = Collectors.time_datas.cpu_time_info();
            let cpu_usage = Collectors.time_datas.cpu_usage_info(&mut system);
    
            for ctimes in cpu_infos.into_iter().zip(cpu_usage) {
                let query = format!("insert into cpu_time values ('{}', '{}', {}, {}, {} ,{} ,{} ,{}, {}, {})", timestamp, 
                ctimes.0.device, ctimes.0.user, ctimes.0.guest_user, ctimes.0.system, ctimes.0.idle, ctimes.0.iowait, ctimes.0.irq, ctimes.0.softirq, ctimes.1);
                client.query(&query, &[]).await?;
            }
            
            let temperature_info = Collectors.time_datas.temperature_info();
            for temp in temperature_info.into_iter() {
                let query = format!("insert into temperature values ('{}', '{}' , {})",timestamp, temp.get_name(), temp.temperature);
                client.query(&query, &[]).await?;
            }
    
            let net_info = Collectors.time_datas.net_info(&mut system);
            for internet in net_info.into_iter() {
                let query = format!("insert into internet values ('{}', '{}', {}, {})", timestamp, internet.interface_name, internet.d_receive, internet.d_transmit);
                client.query(&query, &[]).await?;
            }
            thread::sleep(Duration::from_secs(0));
        }
    Ok(())
}