# dumbo_log
a log helper functions for dumbo series

# install
```
cargo add dumbo_log
```

# use
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

# 控制台输出功能

## 使用 init_log_with_console 函数

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

## 参数说明

- `log_path`: 日志文件路径（与 `init_log` 相同）
- `prefix`: 可选前缀，用于构造环境变量名（与 `init_log` 相同）
- `enable_console`: 
  - `true`: 启用控制台输出，日志会同时写入文件和控制台
  - `false`: 仅写入文件，功能等同于 `init_log` 函数

## 使用场景

1. **开发调试**: 在开发阶段启用控制台输出，方便实时查看日志
2. **生产环境**: 在生产环境中可以禁用控制台输出，仅保留文件日志
3. **动态切换**: 可以通过配置参数在不同环境中灵活切换输出方式

注意：启用控制台输出时，日志格式与文件日志保持一致，便于统一分析。