use log::LevelFilter;
use anyhow::{anyhow, Result};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config as LogConfig, Root},
    encode::pattern::PatternEncoder,
};
use std::path::Path;

/// 初始化日志系统
///
/// # 参数
/// - `log_path`: 日志文件路径
///
/// # 返回
/// - `Result<()>`: 成功时返回`Ok(())`，失败时返回错误信息
pub fn init_log(log_path: &Path) -> Result<()> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {t} - {m}{n}")))
        .build(log_path)
        .map_err(|e| anyhow!("创建日志文件失败: {}", e))?;

    let log_config = LogConfig::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .map_err(|e| anyhow!("构建日志配置失败: {}", e))?;

    log4rs::init_config(log_config)
        .map_err(|e| anyhow!("初始化日志系统失败: {}", e))?;
    
    Ok(())
}
