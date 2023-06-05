use aes::{cipher::generic_array::GenericArray, Aes128};
use block_modes::{block_padding::Pkcs7, BlockMode};
use chrono::Utc;
use futures::FutureExt;
use r2d2::PooledConnection;
use r2d2_postgres::PostgresConnectionManager;
use rand::Rng;
use rand_distr::Normal;
use std::{thread, time::Duration};
use tokio::{runtime::Runtime, sync::mpsc};
use tokio_postgres::{Client, Error, NoTls};

use crate::{data_type, ConnectMessage};

static BASE64_CHARS: &'static [u8] =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
type Aes128Ecb = block_modes::Ecb<Aes128, Pkcs7>;

async fn main_connnect(
    client: Client,
    rx: &mut mpsc::Receiver<ConnectMessage>,
    cipher: &mut Aes128Ecb,
) -> Result<(), Error> {
    loop {
        let mut msgs = Vec::new();
        while let Some(msg) = rx.recv().now_or_never() {
            msgs.push(msg);
        }
        let mut query =
            format!("insert into sensor_vals (time, sensor_id, val, type, predict) values");
        if !msgs.is_empty() {
            for msg in msgs.into_iter() {
                match msg {
                    Some(connect_message) => match connect_message {
                        ConnectMessage::SensorMessage(time, uid, vals, data_type) => {
                            let vals = encrypt_sensors(vals, cipher);
                            let sub_vals =
                                format!("('{time}', {uid}, '{vals}', '{:?}', false),", data_type);
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
                    },
                    None => {}
                }
            }
            if query.ends_with(",") {
                query.pop();
                query.push(';');
                client.simple_query(&query).await?;
            }
        }
    }
}

fn generate(sensor_type: data_type) -> f32 {
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
    rng.clone()
        .sample_iter(normal)
        .filter(|&x| x > 0.0)
        .take(1)
        .next()
        .unwrap()
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

async fn generate_backup(delta_time: Duration, tx: mpsc::Sender<ConnectMessage>) {
    loop {
        thread::sleep(delta_time);
        tx.send(ConnectMessage::BackUp).await.unwrap();
    }
}

pub async fn run_collector() {
    use std::io::Read;
    use tokio_postgres::Config;
    // create_racks(&mut pool.get().unwrap(), rx).unwrap();
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
    let mut key: [u8; 16] = [0; 16];
    let mut iv: [u8; 16] = [0; 16];
    Read::read(&mut "key".as_bytes(), &mut key).unwrap();
    Read::read(&mut "key".as_bytes(), &mut iv).unwrap();
    let mut cipher = Aes128Ecb::new_from_slices(&key, &iv).unwrap();
    handles.push(rt.spawn(async move {
        main_connnect(client, &mut rx, &mut cipher).await.unwrap();
    }));
    for i in 0..100 {
        let tx2 = tx.clone();
        handles.push(rt.spawn(async move {
            match i % 10 {
                0 => generate_data(i, data_type::cpu, Duration::from_millis(1000), tx2).await,
                1 => generate_data(i, data_type::ram, Duration::from_millis(1000), tx2).await,
                2 => {
                    generate_data(i, data_type::disk_write, Duration::from_millis(1000), tx2).await
                }
                3 => generate_data(i, data_type::disk_read, Duration::from_millis(1000), tx2).await,
                4 => generate_data(i, data_type::voltage, Duration::from_millis(1000), tx2).await,
                5 => generate_data(i, data_type::fanspeed, Duration::from_millis(1000), tx2).await,
                6 => {
                    generate_data(i, data_type::temperature, Duration::from_millis(1000), tx2).await
                }
                7 => {
                    generate_data(i, data_type::net_transmit, Duration::from_millis(1000), tx2)
                        .await
                }
                8 => {
                    generate_data(i, data_type::net_receive, Duration::from_millis(1000), tx2).await
                }
                9 => generate_data(i, data_type::sysload, Duration::from_millis(1000), tx2).await,
                _ => {
                    panic!("wrong")
                }
            }
        }));
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

fn encrypt_sensors(value: f32, cipher: &mut Aes128Ecb) -> String {
    use std::io::Read;
    let number = format!("{:0>32b}", value.to_bits());
    let number = number.as_bytes();
    let (mut plaintext1, mut plaintext2, plaintext3): ([u8; 16], [u8; 16], [u8; 16]) =
        ([0; 16], [0; 16], [16; 16]);
    Read::read(&mut &number[..16], &mut plaintext1).unwrap();
    Read::read(&mut (&number[16..32]), &mut plaintext2).unwrap();
    let (plaintext1, plaintext2, plaintext3) = (
        GenericArray::from(plaintext1),
        GenericArray::from(plaintext2),
        GenericArray::from(plaintext3),
    );
    let mut blocks = [plaintext1, plaintext2, plaintext3];
    cipher.encrypt_blocks(&mut blocks);
    let slice = [blocks[0], blocks[1], blocks[2]].concat();
    u8_base64(slice.as_slice())
        .chars()
        .take(64)
        .collect::<String>()
}

fn u8_base64(source: &[u8]) -> String {
    let mut ret = String::new();
    let (mut i, mut len) = (0, 0);
    let (mut first_encode, mut second_encode): ([u8; 3], [u8; 4]) = ([0; 3], [0; 4]);
    while len != source.len() {
        first_encode[i] = source[len];
        i += 1;
        len += 1;
        if i == 3 {
            second_encode[0] = (first_encode[0] & 0xfc).wrapping_shr(2);
            second_encode[1] =
                (first_encode[0] & 0x03).wrapping_shl(4) + (first_encode[1] & 0xf0).wrapping_shr(4);
            second_encode[2] =
                (first_encode[1] & 0x0f).wrapping_shl(2) + (first_encode[2] & 0xc0).wrapping_shr(6);
            second_encode[3] = first_encode[2] & 0x3f;
            for k in 0..4 {
                ret.push(BASE64_CHARS[second_encode[k] as usize].into());
            }
            i = 0;
        }
    }
    if i != 0 {
        for k in i..3 {
            first_encode[k] = '\0' as u8;
        }
        second_encode[0] = (first_encode[0] & 0xfc).wrapping_shr(2);
        second_encode[1] =
            (first_encode[0] & 0x03).wrapping_shl(4) + (first_encode[1] & 0xf0).wrapping_shr(4);
        second_encode[2] =
            (first_encode[1] & 0x0f).wrapping_shl(2) + (first_encode[2] & 0xc0).wrapping_shr(6);
        for k in 0..i + 1 {
            ret.push(BASE64_CHARS[second_encode[k] as usize] as char);
        }
    }
    while i != 3 {
        ret.push('=');
        i += 1;
    }
    ret
}
