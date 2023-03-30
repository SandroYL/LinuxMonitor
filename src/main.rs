use std::thread::sleep;

use chrono::{Duration, Utc};
use rand::Rng;
use sysinfo::{System, SystemExt};
use tokio_postgres::{Client, Error, NoTls};

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
    let mut system = System::new_all();
    //一些测试数据

    let environment_temp = String::from("env_temp");
    let disk = vec![String::from("C"), String::from("D"), String::from("E"), String::from("F")];
    let fan_speed = String::from("fans");
    let models = vec![String::from("TCN"), String::from("LSTM"), 
                                    String::from("ARIMA"), String::from("BAYES"), String::from("MARKOV")];


    let interval = Duration::milliseconds(990);
    loop {
        //十秒一次
        let timestamp = Utc::now();
        let next_timestamp = timestamp + interval;
        system.refresh_all();
        let Collectors = TimeCollector::Collectors::new();
        let cpu_infos = Collectors.time_datas.cpu_time_info();
        let cpu_usage = Collectors.time_datas.cpu_usage_info(&mut system);

        for ctimes in cpu_infos.into_iter().zip(cpu_usage) {
            let query = format!(
                "insert into cpu_time values ('{}', '{}', {})",
                timestamp, ctimes.0.device, ctimes.1
            );
            client.query(&query, &[]).await?;
        }

        let temperature_info = Collectors.time_datas.temperature_info();
        for temp in temperature_info.into_iter() {
            let query = format!(
                "insert into temperature values ('{}', '{}' , {})",
                timestamp,
                temp.get_name(),
                temp.temperature
            );
            client.query(&query, &[]).await?;
        }

        let net_info = Collectors.time_datas.net_info().await;

        let query = format!(
            "insert into internet values ('{}', '{}', {})",
            timestamp,
            net_info.device,
            net_info.get_speed(),
        );
        client.query(&query, &[]).await?;
        let mut rng = rand::thread_rng();

        let query = format!(
            "insert into env_temp values ('{}', '{}', {})",
            timestamp,
            environment_temp,
            rng.gen_range(0..=50000)
        );
        client.query(&query, &[]).await?;

        for i in 0..disk.len() {
            let query = format!(
                "insert into disk values ('{}', '{}', {})",
                timestamp,
                disk[i],
                rng.gen_range(0..=50000)
            );
            client.query(&query, &[]).await?;
        }

        let query = format!(
            "insert into fan_speed values ('{}', '{}', {})",
            timestamp,
            fan_speed,
            rng.gen_range(0..=300)
        );
        client.query(&query, &[]).await?;
        
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
