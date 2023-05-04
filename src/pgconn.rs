use std::{thread, time::Duration};

use chrono::Utc;
use futures::FutureExt;
use r2d2::PooledConnection;
use r2d2_postgres::PostgresConnectionManager;
use rand::Rng;
use rand_distr::Normal;
use tokio::{runtime::Runtime, sync::mpsc, time::interval};
use tokio_postgres::{Client, Error, NoTls};

use crate::{ConnectMessage, data_type};

async fn main_connnect(
    client: Client,
    rx: &mut mpsc::Receiver<ConnectMessage>,
) -> Result<(), Error> {
    loop {
        let mut cnt = 0;
        let mut msgs = Vec::new();
        while let Some(msg) = rx.recv().now_or_never() {
            msgs.push(msg);
            cnt += 1;
        }
        let mut query =
            format!("insert into sensor_vals (time, sensor_id, val, type, predict) values");
        if !msgs.is_empty() {
            for (i, msg) in msgs.into_iter().enumerate() {
                match msg {
                    Some (ConnectMessage) => {
                        match ConnectMessage {
                            ConnectMessage::SensorMessage(time, uid, vals, data_type) => {
                                let sub_vals =
                                    format!("('{time}', {uid}, {vals}, '{:?}', false),", data_type);
                                query = format!("{} {}", query, sub_vals);
                                
                            }
                            ConnectMessage::RackMessage(rid, uid, rname, uname) => {
                                let add_rack = format!("insert into rack (rid, uid, name_rack, name_unit) values ({rid}, {uid}, '{rname}', '{uname}')");
                                client.execute(&add_rack, &[]).await?;
                            }
                            ConnectMessage::BackUp => {
                                let cur = Utc::now();
                                let back_up = format!("copy (select * from sensor_vals) TO '/usr/lib/postgresql/15/backup/{:?}.csv' WITH (FORMAT CSV);", cur);
                                client.execute(&back_up, &[]).await?;
                                client.execute("truncate sensor_vals", &[]).await?;
                            }
                        }
                    }
                    None => {}
                }
            }
            if query.ends_with(",") {
                //change end ',' to ';'
                query.pop();
                query.push(';');
                println!("{cnt}");
                client.simple_query(&query).await?;
            }
        }
    }
}


fn generate(sensor_type: data_type,) -> f32 {
    let rng = rand::thread_rng();
    let normal = match sensor_type {
        data_type::cpu => Normal::new(50.0, 20.0).unwrap(),
        data_type::ram => Normal::new(50.0, 20.0).unwrap(),
        data_type::disk_write => Normal::new(3000.0, 1500.0).unwrap(),
        data_type::disk_read => Normal::new(1000.0, 800.0).unwrap(),
        data_type::voltage => Normal::new(220.0, 2.0).unwrap(),
        data_type::fanspeed => Normal::new(750.0, 30.0).unwrap(),
        data_type::temperature => Normal::new(80.0, 5.0).unwrap(),
        data_type::net_transmit => Normal::new(5000.0, 2000.0).unwrap(),
        data_type::net_receive => Normal::new(5000.0, 2000.0).unwrap(),
        data_type::sysload => Normal::new(1.0, 0.2).unwrap(),
    };
    rng.clone().sample_iter(normal).filter(|&x| x > 0.0).take(1).next().unwrap()
}

async fn generate_data(
    sensor_id: u32,
    sensor_type: data_type,
    delta_time: Duration,
    tx: mpsc::Sender<ConnectMessage>,
) {
    loop {
        let cur = Utc::now();
        let val = generate(sensor_type);
        let msg = ConnectMessage::SensorMessage(cur, sensor_id, val, sensor_type);
        tx.send(msg).await.unwrap();
        thread::sleep(delta_time);
    }
}

async fn generate_backup(
    delta_time: Duration,
    tx: mpsc::Sender<ConnectMessage>,
) {
    loop {
        thread::sleep(delta_time);
        tx.send(ConnectMessage::BackUp).await.unwrap();
    }
}

pub async fn run_collector() {
    // create_racks(&mut pool.get().unwrap(), rx).unwrap();
    use tokio_postgres::Config;
    let mut config = Config::new();
    config.host("localhost");
    config.port(5432);
    config.user("postgres");
    config.password("123456");
    config.dbname("postgres");
    let (client, connect) = config.connect(NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connect.await {
            eprintln!("connnection error!: {}", e);
        }
    });
    let rt = Runtime::new().unwrap();
    let (tx, mut rx) = mpsc::channel::<ConnectMessage>(100);
    let mut handles = vec![];
    handles.push(rt.spawn(async move {
        main_connnect(client, &mut rx).await;
    }));
    for i in 0..100 {
        let tx2 = tx.clone();
        handles.push(rt.spawn(async move {
            match i % 10 {
            0 => generate_data(i, data_type::cpu, Duration::from_millis(1000), tx2).await,
            1 => generate_data(i, data_type::ram, Duration::from_millis(1000), tx2).await,
            2 => generate_data(i, data_type::disk_write, Duration::from_millis(1000), tx2).await,
            3 => generate_data(i, data_type::disk_read, Duration::from_millis(1000), tx2).await,
            4 => generate_data(i, data_type::voltage, Duration::from_millis(1000), tx2).await,
            5 => generate_data(i, data_type::fanspeed, Duration::from_millis(1000), tx2).await,
            6 => generate_data(i, data_type::temperature, Duration::from_millis(1000), tx2).await,
            7 => generate_data(i, data_type::net_transmit, Duration::from_millis(1000), tx2).await,
            8 => generate_data(i, data_type::net_receive, Duration::from_millis(1000), tx2).await,
            9 => generate_data(i, data_type::sysload, Duration::from_millis(1000), tx2).await,
            _ => {
                panic!("wrong")
            }}
            }
        ));
    }
    let tx2 = tx.clone();
    handles.push(rt.spawn(async move {
        generate_backup(Duration::from_secs(518400), tx2).await;
    }));
    for handle in handles {
        handle.await.unwrap();
    }
}

fn create_racks(
    client: &mut PooledConnection<PostgresConnectionManager<NoTls>>,
    rx: mpsc::Receiver<ConnectMessage>,
) -> Result<(), Error> {
    //every racks -> 10 units\\ every unit -> 100 sensors

    for i in 0..10 {
        for j in 0..10 {
            let rname = format!("rname{i}");
            let uname = format!("uname{}", i * 10 + j);
            let query = format!(
                "insert into rack values ({i}, {}, '{rname}', '{uname}');",
                i * 10 + j
            );
            client.simple_query(&query)?;
            for k in 0..100 {
                let query = format!(
                    "insert into sensors values ({}, {})",
                    i * 10 + j,
                    1000 * i + 100 * j + k
                );
                client.simple_query(&query)?;
            }
        }
    }
    Ok(())
}

