pub mod encrypt;

use std::collections::{VecDeque, HashSet};

use LinuxMonitor::pgconn::run_collector;
use aes::{Aes128, cipher::{generic_array::GenericArray, BlockDecrypt}, Aes256};
use block_modes::{Cbc, block_padding::Pkcs7, BlockMode};
use tokio_postgres::Error;

use crate::encrypt::u8_base64;


#[tokio::main]
async fn main() -> Result<(), Error> {
    run_collector().await;
    // benchmarks_insert();
    // benchmarks_select();
    Ok(())
}

#[test]
fn show_aes() {
    use std::io::Read;
    type Aes128Cbc = Cbc<Aes128, Pkcs7>;
    let mut key: [u8; 16] = [0; 16];
    let mut iv: [u8; 16] = [0; 16];
    let number = format!("{:0>32b}", 0.9739077_f32.to_bits()) ;
    // let number = "00000000000000000000000000000000".to_string();
    let mut number = number.as_bytes();
    let mut plaintext1: [u8; 16] = [0; 16];
    let mut plaintext2: [u8; 16] = [0; 16];
    let mut plaintext3 = [16; 16];
    let mut number1 = &number[..16];
    let mut number2 = &number[16..32];
    Read::read(&mut number1, &mut plaintext1).unwrap();
    Read::read(&mut number2, &mut plaintext2).unwrap();
    Read::read(&mut "key".as_bytes(), &mut key).unwrap();
    Read::read(&mut "key".as_bytes(), &mut iv).unwrap();
    let mut plaintext1 = GenericArray::from(plaintext1);
    let mut plaintext2 = GenericArray::from(plaintext2);
    let mut plaintext3 = GenericArray::from(plaintext3);
    let mut cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let mut end = [plaintext1, plaintext2, plaintext3];
    println!("{:?}", end);
    cipher.encrypt_blocks(&mut end);
    println!("{:?}", end);
    let mut chs: Vec<u8> = Vec::new();
    for unum in [end[0],end[1], end[2]].concat() {
        chs.push(unum.into());
    }
    println!("{}", u8_base64(&chs).chars().take(64).collect::<String>());
}


// #[test]
// fn benchmarks_insert() {
//     let manager =
//         PostgresConnectionManager::new("host=localhost user=postgres".parse().unwrap(), NoTls);
//     let pool = r2d2::Pool::builder()
//         .max_size(10)
//         .connection_timeout(Duration::from_secs(30))
//         .build(manager)
//         .unwrap();
//     let mut fp = File::create("insert_result").unwrap();
//     let name = Arc::new(RwLock::new(vec![
//         String::from("device1"),
//         String::from("device2"),
//         String::from("device3"),
//         String::from("device4"),
//         String::from("device5"),
//         String::from("device6"),
//         String::from("device7"),
//         String::from("device8"),
//         String::from("device9"),
//         String::from("device10"),
//     ]));
//     for _ in 0..1000 {
//         let mut nhandles = vec![];
//         let mut yhandles = vec![];
//         for i in 0..10 {
//             let pool = pool.clone();
//             let name = Arc::clone(&name);
//             let handle = thread::spawn(move || {
//                 let mut client = pool.get().unwrap();
//                 let cur_time = Utc::now();
//                 let query = format!(
//                     "insert into nontime values ('{}', '{}', {})",
//                     cur_time,
//                     name.read().unwrap()[i % 10],
//                     i
//                 );
//                 client.execute(&query, &[]).unwrap();
//             });
//             nhandles.push(handle);
//         }
//         for i in 0..10 {
//             let pool = pool.clone();
//             let name = Arc::clone(&name);
//             let handle = thread::spawn(move || {
//                 let mut client = pool.get().unwrap();
//                 let cur_time = Utc::now();
//                 let query = format!(
//                     "insert into yestime values ('{}', '{}', {})",
//                     cur_time,
//                     name.read().unwrap()[i % 10],
//                     i
//                 );
//                 client.execute(&query, &[]).unwrap();
//             });
//             yhandles.push(handle);
//         }
//         // println!("Testing insert! {} times testing..", j);
//         let (mut m1, mut m2) = (0, 0);
//         let startTime = Utc::now();
//         for handle in nhandles {
//             handle.join().unwrap();
//         }
//         let end_time = Utc::now();
//         // println!("non Timescaledb: {:?}", (end_time - startTime).num_nanoseconds().unwrap());
//         m1 = (end_time - startTime).num_nanoseconds().unwrap();
//         let startTime = Utc::now();
//         for handle in yhandles {
//             handle.join().unwrap();
//         }
//         let end_time = Utc::now();
//         m2 = (end_time - startTime).num_nanoseconds().unwrap();
//         // println!("use Timescaledb: {:?}", (end_time - startTime).num_nanoseconds().unwrap());
//         // println!("{} {}", m1, m2);
//         let s = format!("{} {}\n", m1, m2);
//         fp.write_all(s.as_bytes());
//     }
// }

// #[test]
// fn benchmarks_select() {
//     let manager =
//         PostgresConnectionManager::new("host=localhost user=postgres".parse().unwrap(), NoTls);
//     let pool = r2d2::Pool::builder()
//         .max_size(10)
//         .connection_timeout(Duration::from_secs(30))
//         .build(manager)
//         .unwrap();
//     let mut fp = File::create("select_result").unwrap();
    
//     let name = Arc::new(RwLock::new(vec![
//         String::from("device1"),
//         String::from("device2"),
//         String::from("device3"),
//         String::from("device4"),
//         String::from("device5"),
//         String::from("device6"),
//         String::from("device7"),
//         String::from("device8"),
//         String::from("device9"),
//         String::from("device10"),
//     ]));
//     for j in 0..1000 {
//         let mut nhandles = vec![];
//         let mut yhandles = vec![];
//         for i in 0..10 {
//             let pool = pool.clone();
//             let name = Arc::clone(&name);
//             let handle = thread::spawn(move || {
//                 let mut client = pool.get().unwrap();
//                 let cur_time = Utc::now();
//                 let query = format!("select * from nontime where value = {}", i,);
//                 client.execute(&query, &[]).unwrap();
//             });
//             nhandles.push(handle);
//         }
//         for i in 0..10 {
//             let pool = pool.clone();
//             let name = Arc::clone(&name);
//             let handle = thread::spawn(move || {
//                 let mut client = pool.get().unwrap();
//                 let cur_time = Utc::now();
//                 let query = format!("select * from yestime where value = {}", i,);
//                 client.execute(&query, &[]).unwrap();
//             });
//             yhandles.push(handle);
//         }
//         // println!("Testing insert! {} times testing..", j);
//         let (mut m1, mut m2) = (0, 0);
//         let startTime = Utc::now();
//         for handle in nhandles {
//             handle.join().unwrap();
//         }
//         let end_time = Utc::now();
//         // println!("non Timescaledb: {:?}", (end_time - startTime).num_nanoseconds().unwrap());
//         m1 = (end_time - startTime).num_nanoseconds().unwrap();
//         let startTime = Utc::now();
//         for handle in yhandles {
//             handle.join().unwrap();
//         }
//         let end_time = Utc::now();
//         m2 = (end_time - startTime).num_nanoseconds().unwrap();
//         // println!("use Timescaledb: {:?}", (end_time - startTime).num_nanoseconds().unwrap());
//         // println!("{} {}", m1, m2);
//         let s = format!("{} {}\n", m1, m2);
//         fp.write_all(s.as_bytes());
//     }
// }



// #[tokio::test]
// async fn backup_table() {
//     let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=123456 dbname=postgres", NoTls).await.unwrap();
//     tokio::spawn(async move {
//         if let Err(e) = connection.await {
//             eprintln!("connection error: {}", e);
//         }
//     });
//     // 构建一个查询语句，用来选择表中的所有数据
//     client.batch_execute("copy (select * from cpu_time) TO '/usr/lib/postgresql/15/backup/test.csv' WITH (FORMAT CSV);").await.unwrap();
// }





#[test]
fn show() {
    println!("{}", "nejKXaTPqZQxoP6uPwVpDhi/3kctbAptAbz1MYGeod8Z4Rnr+7C+jbnIXE0mBO1m".len());
}