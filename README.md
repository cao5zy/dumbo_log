# dumbo_log - 简单高效的 Rust 日志库

[![Crates.io](https://img.shields.io/crates/v/dumbo_log)](https://crates.io/crates/dumbo_log)
[![Docs.rs](https://docs.rs/dumbo_log/badge.svg)](https://docs.rs/dumbo_log)

[Home-https://www.craftaidhub.com](https://www.craftaidhub.com)

dumbo_log 是一个轻量级的 Rust 日志库，专为 dumbo 系列项目设计，提供简单易用的日志记录功能和强大的结构化日志输出能力。

## 🌟 主要特性

- **简单易用**：一行代码即可初始化日志系统
- **灵活配置**：通过环境变量动态控制日志级别
- **双输出支持**：同时支持文件和控制台输出
- **结构化日志**：提供标准化的结构化日志格式，便于后续分析
- **类型安全**：支持多种数据类型的日志输出（字符串、数字、日期等）
- **零成本抽象**：在不使用时几乎没有性能开销

## 🚀 快速开始

### 1. 安装

```bash
cargo add dumbo_log
```

### 2. 基础使用

```rust
use std::path::Path;
use dumbo_log::init_log;
use anyhow::Result;

fn main() -> Result<()> {
    // 初始化日志系统
    let log_path = Path::new("./app.log");
    init_log(log_path, None)?;
    
    // 开始记录日志
    log::info!("应用启动成功");
    log::warn!("这是一条警告");
    log::error!("发生错误");
    
    Ok(())
}
```

## 📚 完整文档

### 基础日志功能

#### 初始化日志系统

`init_log` 函数用于初始化日志系统，支持通过环境变量动态设置日志级别。

```rust
use std::path::Path;
use dumbo_log::init_log; 
use anyhow::Result;

fn main() -> Result<()> {
    let log_path = Path::new("./app.log");
    
    // 使用默认前缀初始化日志，通过环境变量 DUMBO_LOG_LEVEL 来设置 LevelFilter
    init_log(log_path, None)?;
    
    // 或者使用自定义前缀初始化日志，通过环境变量 MY_APP_DUMBO_LOG_LEVEL 来设置 LevelFilter
    // init_log(log_path, Some("MY_APP"))?;
    
    log::info!("日志系统初始化成功");
    log::warn!("这是一条警告日志");
    log::error!("这是一条错误日志");
    
    Ok(())
}
```

**日志级别说明**：
- 默认级别：`Info`
- 可用级别：`Debug`、`Info`、`Warn`、`Error`

#### 控制台输出功能

使用 `init_log_with_console` 函数可以同时输出到文件和控制台：

```rust
use std::path::Path;
use dumbo_log::init_log_with_console; 
use anyhow::Result;

fn main() -> Result<()> {
    let log_path = Path::new("./app.log");
    
    // 启用控制台输出的日志初始化
    init_log_with_console(log_path, None, true)?;
    
    log::info!("这条日志会同时输出到文件和控制台");
    
    Ok(())
}
```

**参数说明**：
- `log_path`: 日志文件路径
- `prefix`: 可选前缀，用于构造环境变量名
- `enable_console`: 
  - `true`: 启用控制台输出
  - `false`: 仅写入文件

**使用场景**：
- **开发调试**: 启用控制台输出，方便实时查看
- **生产环境**: 禁用控制台输出，仅保留文件日志
- **动态切换**: 通过配置参数灵活切换输出方式

### 结构化日志输出功能

#### 功能概述

dumbo_log 提供结构化日志输出 API，按照固定格式输出日志，便于后续分析和解析。

**日志格式**：
```
[log4rs前缀][collect_id][id.name][timestamp][log_data]
```

#### 配置文件

创建 `log_config.json` 配置文件：

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
    }
  ]
}
```

#### 完整使用示例

```rust
use dumbo_log::{init_log, init_log_config, log_as_string, log_as_number, log_as_date, log_as_datetime};
use std::path::Path;
use anyhow::Result;
use chrono::{NaiveDate, Utc};

fn main() -> Result<()> {
    // 1. 初始化日志系统
    init_log(Path::new("app.log"), None)?;
    
    // 2. 初始化日志配置
    init_log_config(Path::new("log_config.json"))?;
    
    // 3. 使用结构化日志输出
    log_as_string("user_login", r#"{"user_id":123,"status":"success"}"#)?;
    log_as_number("cpu_usage", 75.5)?;
    log_as_number("cpu_usage", 80i32)?;
    
    let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    log_as_date("report_date", date)?;
    
    let datetime = Utc::now().naive_utc();
    log_as_datetime("event_time", datetime)?;
    
    Ok(())
}
```

#### API 说明

| 函数 | 描述 | 示例 |
|------|------|------|
| `init_log_config` | 从 JSON 文件加载日志配置 | `init_log_config(Path::new("log_config.json"))?` |
| `log_as_string` | 输出字符串类型日志 | `log_as_string("user_login", r#"{"user_id":123}"#)?` |
| `log_as_number` | 输出数字类型日志 | `log_as_number("cpu_usage", 75.5)?` |
| `log_as_date` | 输出日期类型日志 | `log_as_date("report_date", date)?` |
| `log_as_datetime` | 输出日期时间类型日志 | `log_as_datetime("event_time", datetime)?` |

#### 日志格式详解

**完整格式**：
```
[log4rs前缀][collect_id(collect_id_length字符)][id.name(id.length字符)][timestamp(16字符)][log_data]
```

**示例输出**：
```
2024-01-01 10:30:00 INFO RESUME-AGENTuser_login0001704067200001{"user_id":123,"status":"success"}
```

#### 错误处理

API 会进行以下验证：
1. **配置未初始化**: 如果未调用 `init_log_config`，会返回错误
2. **ID不存在**: 如果传入的 `id` 在配置中不存在，会返回错误  
3. **ID长度不匹配**: 如果传入的 `id` 长度与配置不匹配，会返回错误

#### 注意事项

- **UTC时间**: 所有日期时间都使用 UTC 时间，确保跨时区一致性
- **ID验证**: 严格验证 ID 存在性和长度，不匹配时报错
- **全局单例**: 配置只初始化一次，避免重复初始化
- **时间戳格式**: 16位毫秒时间戳，左补零，固定长度
- **线程安全**: 配置使用 `OnceCell` 实现，支持多线程安全访问

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request！请确保：

1. 代码遵循 Rust 的最佳实践
2. 添加相应的测试用例
3. 更新相关文档

## 📄 许可证

本项目采用 [MIT 许可证](LICENSE)。

---
更多内容，访问我的[个人网站-https://www.craftaidhub.com](https://www.craftaidhub.com)，展开深度合作与交流
