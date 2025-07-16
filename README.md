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
    
    init_log(log_path)?;
    
    log::info!("日志系统初始化成功");
    log::warn!("这是一条警告日志");
    log::error!("这是一条错误日志");
    
    Ok(())
}

```
