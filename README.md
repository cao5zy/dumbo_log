
# dumbo_log
a log helper functions for dumbo series

# install
```
cargo add dumbo_log
```

# 基础日志功能

## 初始化日志系统

```
use std::path::Path;
use dumbo_log::init_log; 
use anyhow::Result;

fn main() -> Result<()> {
    let log_path = Path::new("./app.log");
    
    // 使用默认前缀初始化日志，通过环境变量DUMBO_LOG_LEVEL来设置LevelFilter
    init_log(log_path, None)?;
    
    // 或者使用自定义前缀初始化日志，通过环境变量MY_APP_DUMBO_LOG_LEVEL来设置LevelFilter
    // init_log(log_path, Some("MY_APP"))?;
    
    log::info!("日志系统初始化成功");
    log::warn!("这是一条警告日志");
    log::error!("这是一条错误日志");
    
    Ok(())
}
```

By default, the levelFilter is Info.
Available levelFilter:
- Debug
- Info
- Warn
- Error

## 控制台输出功能

### 使用 init_log_with_console 函数

`init_log_with_console` 函数扩展了基础的日志功能，支持同时输出到文件和控制台：

```
use std::path::Path;
use dumbo_log::init_log_with_console; 
use anyhow::Result;

fn main() -> Result<()> {
    let log_path = Path::new("./app.log");
    
    // 启用控制台输出的日志初始化
    // 第三个参数为 true 表示启用控制台输出
    init_log_with_console(log_path, None, true)?;
    
    log::info!("这条日志会同时输出到文件和控制台");
    log::warn!("警告信息也会显示在控制台");
    log::error!("错误信息在控制台高亮显示");
    
    Ok(())
}
```

### 参数说明

- `log_path`: 日志文件路径（与 `init_log` 相同）
- `prefix`: 可选前缀，用于构造环境变量名（与 `init_log` 相同）
- `enable_console`: 
  - `true`: 启用控制台输出，日志会同时写入文件和控制台
  - `false`: 仅写入文件，功能等同于 `init_log` 函数

### 使用场景

1. **开发调试**: 在开发阶段启用控制台输出，方便实时查看日志
2. **生产环境**: 在生产环境中可以禁用控制台输出，仅保留文件日志
3. **动态切换**: 可以通过配置参数在不同环境中灵活切换输出方式

注意：启用控制台输出时，日志格式与文件日志保持一致，便于统一分析。

# 结构化日志输出功能

## 功能概述

dumbo_log 提供了结构化日志输出API，按照固定格式输出日志，便于后续分析和解析。日志格式为：

```
[log4rs前缀][collect_id][id.name][timestamp][log_data]
```

## 配置文件

在使用结构化日志输出功能之前，需要创建配置文件 `log_config.json`：

```json
{
  "collect_id": "RESUME-AGENT",
  "collect_id_length": 12,
  "targets": [
    {
      "id": {
        "name": "user_login",
        "length": 10
      }
    },
    {
      "id": {
        "name": "cpu_usage",
        "length": 9
      }
    },
    {
      "id": {
        "name": "report_date",
        "length": 11
      }
    },
    {
      "id": {
        "name": "event_time",
        "length": 10
      }
    }
  ]
}
```

### 配置字段说明

- `collect_id`: 采集标识符，用于标识日志来源
- `collect_id_length`: collect_id的固定长度
- `targets`: 目标配置数组
  - `id.name`: 目标标识名称
  - `id.length`: id.name的固定长度

## 使用示例

### 完整示例

```
use dumbo_log::{init_log, init_log_config, log_as_string, log_as_number, log_as_date, log_as_datetime};
use std::path::Path;
use anyhow::Result;
use chrono::{NaiveDate, Utc};

fn main() -> Result<()> {
    // 1. 初始化日志系统
    init_log(Path::new("app.log"), None)?;
    
    // 2. 初始化日志输出配置
    init_log_config(Path::new("log_config.json"))?;
    
    // 3. 使用结构化日志输出API
    
    // 输出字符串类型日志
    log_as_string("user_login", r#"{"user_id":123,"status":"success"}"#)?;
    
    // 输出数字类型日志（支持所有数字类型）
    log_as_number("cpu_usage", 75.5)?;
    log_as_number("cpu_usage", 80i32)?;
    
    // 输出日期类型日志
    let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    log_as_date("report_date", date)?;
    
    // 输出日期时间类型日志
    let datetime = Utc::now().naive_utc();
    log_as_datetime("event_time", datetime)?;
    
    Ok(())
}
```

### API 说明

#### 1. 初始化配置

```rust
pub fn init_log_config(config_path: &Path) -> Result<()>
```

从JSON文件加载日志配置并初始化全局配置单例。

#### 2. 字符串类型日志

```rust
pub fn log_as_string(id: &str, data: &str) -> Result<()>
```

输出字符串类型的日志数据。

**示例：**
```rust
log_as_string("user_login", r#"{"user_id":123,"status":"success"}"#)?;
```

#### 3. 数字类型日志（泛型）

```rust
pub fn log_as_number<T>(id: &str, data: T) -> Result<()>
where
    T: Num + Display + Copy + 'static
```

输出数字类型的日志数据，支持所有数字类型：
- 整数：`i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- 浮点：`f32`, `f64`

**示例：**
```rust
log_as_number("cpu_usage", 75.5f64)?;
log_as_number("cpu_usage", 80i32)?;
log_as_number("cpu_usage", 100u8)?;
```

#### 4. 日期类型日志

```rust
pub fn log_as_date(id: &str, data: NaiveDate) -> Result<()>
```

输出日期类型的日志数据（UTC时间，无时区信息）。

**示例：**
```rust
use chrono::NaiveDate;

let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
log_as_date("report_date", date)?;
```

#### 5. 日期时间类型日志

```rust
pub fn log_as_datetime(id: &str, data: NaiveDateTime) -> Result<()>
```

输出日期时间类型的日志数据（UTC时间，无时区信息）。

**示例：**
```rust
use chrono::Utc;

let datetime = Utc::now().naive_utc();
log_as_datetime("event_time", datetime)?;
```

## 日志格式说明

### 完整格式

```
[log4rs前缀][collect_id(collect_id_length字符)][id.name(id.length字符)][timestamp(16字符)][log_data]
```

### 格式说明

- **log4rs前缀**: 由log4rs自动生成，包含时间、日志级别等
- **collect_id**: 从配置中读取，固定长度为 `collect_id_length`
- **id.name**: 由API参数传入，固定长度为 `id.length`
- **timestamp**: 16位数字，毫秒时间戳，左补零到16位
- **log_data**: 实际的日志数据

### 输出示例

假设配置为：
- collect_id: `RESUME-AGENT` (长度12)
- id.name: `user_login` (长度10)

输出日志：
```
2024-01-01 10:30:00 INFO RESUME-AGENTuser_login0001704067200001{"user_id":123,"status":"success"}
```

## 错误处理

API会进行以下验证：

1. **配置未初始化**: 如果未调用 `init_log_config`，会返回错误
2. **ID不存在**: 如果传入的 `id` 在配置的 `targets` 中不存在，会返回错误
3. **ID长度不匹配**: 如果传入的 `id` 长度与配置中的 `id.length` 不匹配，会返回错误

错误发生时，会使用常规方式输出错误日志（不按照特殊格式）。

**错误示例：**
```rust
// ID不存在
log_as_string("not_exist", "data")?; // 返回错误: ID 'not_exist' 在配置中不存在

// ID长度不匹配
log_as_string("user_login", "data")?; // 如果 "user_login" 长度不是10，会返回错误
```

## 注意事项

1. **UTC时间**: 所有日期时间都使用UTC时间，确保跨时区一致性
2. **ID验证**: 严格验证ID存在性和长度，不匹配时报错
3. **全局单例**: 配置只初始化一次，避免重复初始化
4. **时间戳格式**: 16位毫秒时间戳，左补零，固定长度
5. **线程安全**: 配置使用 `OnceCell` 实现，支持多线程安全访问
