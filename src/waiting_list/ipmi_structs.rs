#[allow(non_snake_case)]
struct IPMIItemObEntity {
    /// 监控项id
    itemId: i64, 
    /// 监控设备名称
    monitorId: i64,
    /// 监控项名称
    itemName: String,
    /// 扩展属性
    key: String,
    /// 监控项状态
    status: i32,
    /// 监控项值类型
    valueType: i32,
    /// 监控项值单位
    units: String,
    /// 监控相对应传感器名称
    sensorName: String,
    /// 传感器Id
    sensorId: i32,
    /// 传感器类型       
    sensorType: String,
}

#[allow(non_snake_case)]
struct IPMIDataEntity {
    /// 传感器Id
    sensorId: i32,
    /// 监控项值时间 单位为ms
    clock: u64,
    /// 监控项值 
    value: String,
}

#[allow(non_snake_case)]
struct ItemsDTO {
    /// 监控项Id
    itemId: i64,
    /// 应用集Id
    applicationId: i32,
    /// 监控项值单位
    units: String,
    /// 应用集名称
    aname: String,
    /// 最新时间
    laskclock: i64,
    /// 监控项名称
    Name: String,
    /// 监控项值类型
    valueType: i32,
    /// 最新值
    lastvalue: String,
    /// 当前一个值
    prevvalue: String,
}