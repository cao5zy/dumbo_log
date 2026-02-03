use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::path::Path;

/// ID配置
#[derive(Debug, Clone, Deserialize)]
pub struct IdConfig {
    /// 目标标识名称
    pub name: String,
    /// id.name的固定长度
    pub length: usize,
}

/// 目标配置
#[derive(Debug, Clone, Deserialize)]
pub struct TargetConfig {
    pub id: IdConfig,
}

/// 日志配置
#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
    /// 采集标识符
    pub collect_id: String,
    /// collect_id的固定长度
    pub collect_id_length: usize,
    /// 目标配置数组
    pub targets: Vec<TargetConfig>,
}

impl LogConfig {
    /// 从JSON文件加载配置
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .map_err(|e| anyhow!("读取配置文件失败: {}", e))?;

        let config: LogConfig =
            serde_json::from_str(&content).map_err(|e| anyhow!("解析配置文件失败: {}", e))?;

        // 验证配置
        config.validate()?;

        Ok(config)
    }

    /// 验证配置的有效性
    fn validate(&self) -> Result<()> {
        // 验证 collect_id 长度
        if self.collect_id.len() != self.collect_id_length {
            return Err(anyhow!(
                "collect_id 长度不匹配: 期望 {}, 实际 {}",
                self.collect_id_length,
                self.collect_id.len()
            ));
        }

        // 验证 targets 中没有重复的 name
        let mut names = std::collections::HashSet::new();
        for target in &self.targets {
            if !names.insert(&target.id.name) {
                return Err(anyhow!("targets 中存在重复的 name: {}", target.id.name));
            }
        }

        Ok(())
    }

    /// 获取指定名称的ID配置
    pub fn get_id_config(&self, name: &str) -> Option<&IdConfig> {
        self.targets
            .iter()
            .find(|t| t.id.name == name)
            .map(|t| &t.id)
    }
}

/// 全局配置单例
/// 使用 OnceCell 而不是 RwLock，因为配置在初始化后是只读的
static GLOBAL_CONFIG: OnceCell<LogConfig> = OnceCell::new();

/// 初始化日志配置
///
/// # 参数
/// - `config_path`: 配置文件路径
///
/// # 返回
/// - `Result<()>`: 成功时返回`Ok(())`，失败时返回错误信息
pub fn init_log_config(config_path: &Path) -> Result<()> {
    let config = LogConfig::from_file(config_path)?;

    GLOBAL_CONFIG
        .set(config)
        .map_err(|_| anyhow!("日志配置已经初始化，不能重复初始化"))?;

    Ok(())
}

/// 获取全局配置
///
/// # 返回
/// - `Result<&'static LogConfig>`: 成功时返回配置的引用，失败时返回错误信息
pub fn get_config() -> Result<&'static LogConfig> {
    GLOBAL_CONFIG
        .get()
        .ok_or_else(|| anyhow!("日志配置未初始化，请先调用 init_log_config"))
}

/// 验证ID是否存在且长度匹配
///
/// # 参数
/// - `id`: 目标标识名称
///
/// # 返回
/// - `Result<()>`: 成功时返回`Ok(())`，失败时返回错误信息
pub fn validate_id(id: &str) -> Result<()> {
    let config = get_config()?;

    let id_config = config
        .get_id_config(id)
        .ok_or_else(|| anyhow!("ID '{}' 在配置中不存在", id))?;

    if id.len() != id_config.length {
        return Err(anyhow!(
            "ID '{}' 长度不匹配: 期望 {}, 实际 {}",
            id,
            id_config.length,
            id.len()
        ));
    }

    Ok(())
}

/// 获取 collect_id
///
/// # 返回
/// - `Result<String>`: 成功时返回 collect_id，失败时返回错误信息
pub fn get_collect_id() -> Result<String> {
    let config = get_config()?;
    Ok(config.collect_id.clone())
}

/// 获取 collect_id_length
///
/// # 返回
/// - `Result<usize>`: 成功时返回 collect_id_length，失败时返回错误信息
pub fn get_collect_id_length() -> Result<usize> {
    let config = get_config()?;
    Ok(config.collect_id_length)
}

/// 获取指定ID的长度
///
/// # 参数
/// - `id`: 目标标识名称
///
/// # 返回
/// - `Result<usize>`: 成功时返回ID的长度，失败时返回错误信息
pub fn get_id_length(id: &str) -> Result<usize> {
    let config = get_config()?;

    let id_config = config
        .get_id_config(id)
        .ok_or_else(|| anyhow!("ID '{}' 在配置中不存在", id))?;

    Ok(id_config.length)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    #[serial]
    fn test_config_from_file() {
        let config_content = r#"{
            "collect_id": "RESUME-AGENT",
            "collect_id_length": 12,
            "targets": [
                {
                    "id": {
                        "name": "user_login",
                        "length": 10
                    }
                }
            ]
        }"#;

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();
        fs::write(path, config_content).unwrap();

        let config = LogConfig::from_file(path).unwrap();
        assert_eq!(config.collect_id, "RESUME-AGENT");
        assert_eq!(config.collect_id_length, 12);
        assert_eq!(config.targets.len(), 1);
        assert_eq!(config.targets[0].id.name, "user_login");
        assert_eq!(config.targets[0].id.length, 10);
    }

    #[test]
    #[serial]
    fn test_config_validate_collect_id_length() {
        let config_content = r#"{
            "collect_id": "RESUME",
            "collect_id_length": 12,
            "targets": []
        }"#;

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();
        fs::write(path, config_content).unwrap();

        let result = LogConfig::from_file(path);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("collect_id 长度不匹配"));
    }

    #[test]
    #[serial]
    fn test_config_validate_duplicate_names() {
        let config_content = r#"{
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
                        "name": "user_login",
                        "length": 10
                    }
                }
            ]
        }"#;

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();
        fs::write(path, config_content).unwrap();

        let result = LogConfig::from_file(path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("重复的 name"));
    }

    #[test]
    #[serial]
    fn test_get_id_config() {
        let config = LogConfig {
            collect_id: "RESUME-AGENT".to_string(),
            collect_id_length: 12,
            targets: vec![TargetConfig {
                id: IdConfig {
                    name: "user_login".to_string(),
                    length: 10,
                },
            }],
        };

        let id_config = config.get_id_config("user_login").unwrap();
        assert_eq!(id_config.name, "user_login");
        assert_eq!(id_config.length, 10);

        assert!(config.get_id_config("not_exist").is_none());
    }

    #[test]
    #[serial]
    fn test_get_collect_id_length() {
        let config_content = r#"{
            "collect_id": "RESUME-AGENT",
            "collect_id_length": 12,
            "targets": [
                {
                    "id": {
                        "name": "user_login",
                        "length": 10
                    }
                }
            ]
        }"#;

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();
        fs::write(path, config_content).unwrap();

        init_log_config(path).unwrap();
        let length = get_collect_id_length().unwrap();
        assert_eq!(length, 12);
    }

    #[test]
    #[serial]
    fn test_get_id_length() {
        let config_content = r#"{
            "collect_id": "RESUME-AGENT",
            "collect_id_length": 12,
            "targets": [
                {
                    "id": {
                        "name": "user_login",
                        "length": 10
                    }
                }
            ]
        }"#;

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();
        fs::write(path, config_content).unwrap();

        init_log_config(path).unwrap();
        let length = get_id_length("user_login").unwrap();
        assert_eq!(length, 10);
    }

    #[test]
    #[serial]
    fn test_get_id_length_not_exist() {
        let config_content = r#"{
            "collect_id": "RESUME-AGENT",
            "collect_id_length": 12,
            "targets": [
                {
                    "id": {
                        "name": "user_login",
                        "length": 10
                    }
                }
            ]
        }"#;

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();
        fs::write(path, config_content).unwrap();

        init_log_config(path).unwrap();
        let result = get_id_length("not_exist");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("在配置中不存在"));
    }
}
