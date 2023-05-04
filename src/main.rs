use LinuxMonitor::pgconn::run_collector;
use aes::{Aes128, cipher::{KeyInit, generic_array::GenericArray}};
use tokio_postgres::Error;


#[tokio::main]
async fn main() -> Result<(), Error> {
    run_collector().await;
    // benchmarks_insert();
    // benchmarks_select();
    Ok(())
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
fn show_aes() {
    let mut key: [u8; 32] = [0; 32];
    std::io::Read::read(&mut "key".as_bytes(), &mut key);
    let key = GenericArray::from();
    let cipher = Aes128::new(key)
}