use std::{thread::sleep, time::Duration};

use LinuxMonitor::monitor_tools::time_serise_monitor::TimeSeriesMonitor;
use chrono::{Utc};
use rand::Rng;
use tokio_postgres::{Client, Error, NoTls};

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
        Ok(_) => {
            println!("success")
        }
        Err(e) => {
            eprintln!("{}", e)
        }
    }
    Ok(())
}

async fn write_data(client: &Client) -> Result<(), tokio_postgres::Error> {
    //一些测试数据
    let mut dataGenerator = TimeSeriesMonitor::new();

    let models = vec![String::from("TCN"), String::from("LSTM"), 
                                    String::from("ARIMA"), String::from("BAYES"), String::from("MARKOV")];


    let interval = Duration::from_millis(990);
    loop {
        //十秒一次
        let timestamp = Utc::now();
        dataGenerator.refresh();
        let next_timestamp = timestamp.checked_add_signed(chrono::Duration::milliseconds(interval.as_millis() as i64)).unwrap();
        
        let cpu_info = dataGenerator.cpu_usage();
        let query = format!(
            "insert into cpu_time values ('{}', '{}', {});"
            , timestamp, cpu_info.device, cpu_info.usage
        );
        client.query(&query, &[]).await?;
        let query = format!(
            "insert into cpu_freq values ('{}', '{}', {});"
            , timestamp, cpu_info.device, cpu_info.freq
        );
        client.query(&query, &[]).await?;

        let temperature_info = dataGenerator.temperature_info();
        for temp in temperature_info.into_iter() {
            let query = format!(
                "insert into temperature values ('{}', '{}' , {});"
                , timestamp, temp.get_name(), temp.temperature
            );
            client.query(&query, &[]).await?;
        }

        let fan_info = dataGenerator.fan_info();
        for fan in fan_info.into_iter() {
            let query = format!(
                "insert into fan_speed values ('{}', '{}' , {});"
                , timestamp, fan.device, fan.voltage,
            );
            client.query(&query, &[]).await?;
        }

        let net_info = dataGenerator.net_info(interval);
        let query = format!(
            "insert into internet values ('{}', '{}', {}, {});"
            , timestamp, net_info.device, net_info.speed_recv, net_info.speed_trans,
        );
        client.query(&query, &[]).await?;

        let disks = dataGenerator.disk_info();
        for disk in disks {
            let query = format!(
                "insert into disk values ('{}', '{}', {}, {});"
                , timestamp, disk.disk_name, disk.space_total, disk.space_available,
            );
            client.query(&query, &[]).await?;
        }

        let mem = dataGenerator.mem_info();
        let query = format!(
            "insert into memory values ('{}', '{}', {}, {});"
            , timestamp, String::from("dells"), mem.mem_total, mem.mem_used,
        );
        client.query(&query, &[]).await?;

        let cache = dataGenerator.cache_info();
        let query = format!(
            "insert into cache values ('{}', '{}', {}, {});"
            , timestamp, String::from("dells"), cache.swap_total, cache.used_total,
        );
        client.query(&query, &[]).await?;
        
        let mut rng = rand::thread_rng();

        for i in 0..models.len() {
            let ratio = rng.gen_range(0.9..=1.1);
            let query = format!(
                "insert into models values ('{}', '{}', {})",
                timestamp,
                models[i],
                ratio
            );
            client.query(&query, &[]).await?;
        }


        let sleep_duration = next_timestamp.signed_duration_since(Utc::now());
        sleep(sleep_duration.to_std().unwrap());
    }
    Ok(())
}
