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
    let test_1 = vec![
        String::from("fan1"),
        String::from("fan2"),
        String::from("fan3"),
        String::from("fan4"),
    ];
    let test_2 = vec![
        String::from("impl1"),
        String::from("impl2"),
        String::from("impl3"),
        String::from("impl4"),
        String::from("impl5"),
        String::from("impl6"),
        String::from("impl7"),
        String::from("impl8"),
    ];
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
        for i in 0..4 {
            let query = format!(
                "insert into test1 values ('{}', '{}', {})",
                timestamp,
                test_1[i],
                rng.gen_range(0..=100)
            );
            client.query(&query, &[]).await?;
        }
        for i in 0..8 {
            let query = format!(
                "insert into test2 values ('{}', '{}', {})",
                timestamp,
                test_2[i],
                rng.gen_range(20..=80)
            );
            client.query(&query, &[]).await?;
        }

        let sleep_duration = next_timestamp.signed_duration_since(Utc::now());
        sleep(sleep_duration.to_std().unwrap());
    }
    Ok(())
}
