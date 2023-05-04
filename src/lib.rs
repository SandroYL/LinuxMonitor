use chrono::{DateTime, Utc};


pub mod monitor_tools;
pub mod waiting_list;
pub mod pgconn;

#[derive(Clone, Copy, Debug)]
pub enum data_type {
    cpu,
    ram,
    disk_write,
    disk_read,
    voltage,
    fanspeed,
    temperature,
    net_transmit,
    net_receive,
    sysload,
}

#[derive(Debug)]
pub enum ConnectMessage {
    SensorMessage(DateTime<Utc>, u32, f32, data_type),
    RackMessage(u32, u32, String, String),
    BackUp,
}