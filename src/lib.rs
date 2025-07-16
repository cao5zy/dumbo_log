use anyhow::{anyhow, Result};
use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config as LogConfig, Root},
    encode::pattern::PatternEncoder,
};
use std::env;
use std::path::Path;

/// 初始化日志系统
///
/// # 参数
/// - `log_path`: 日志文件路径
/// - `prefix`: 可选前缀，用于构造环境变量名。如果为None，使用"DUMBO_LOG_LEVEL"；
///             如果为Some("PREFIX")，使用"PREFIX_DUMBO_LOG_LEVEL"
///
/// # 返回
/// - `Result<()>`: 成功时返回`Ok(())`，失败时返回错误信息
pub fn init_log(log_path: &Path, prefix: Option<&str>) -> Result<()> {
    let env_var = match prefix {
        Some(p) => format!("{}_DUMBO_LOG_LEVEL", p),
        None => "DUMBO_LOG_LEVEL".to_string(),
    };

    let level = env::var(&env_var)
        .unwrap_or_else(|_| "Info".to_string())
        .parse::<LevelFilter>()
        .unwrap_or(LevelFilter::Info);

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {t} - {m}{n}")))
        .build(log_path)
        .map_err(|e| anyhow!("创建日志文件失败: {}", e))?;

    let log_config = LogConfig::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(level))
        .map_err(|e| anyhow!("构建日志配置失败: {}", e))?;

    log4rs::init_config(log_config).map_err(|e| anyhow!("初始化日志系统失败: {}", e))?;

    Ok(())
}
