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

