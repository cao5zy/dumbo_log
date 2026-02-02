
use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime};
use log::info;
use num_traits::Num;
use serde::Serialize;
use std::fmt::Display;

use crate::config::{get_collect_id, validate_id};
use crate::formatter::{format_date, format_datetime, format_log, format_number, format_string, generate_timestamp};

/// 转义字符串中的换行符，确保日志数据在一行上
///
/// # 参数
/// - `data`: 原始字符串数据
///
/// # 返回
/// - `String`: 转义后的字符串，其中 `\n` 被替换为 `\\n`，`\r` 被替换为 `\\r`
///
/// # 示例
/// ```
/// let input = "hello\nworld";
/// let output = escape_newlines(input);
/// assert_eq!(output, "hello\\nworld");
/// ```
pub fn escape_newlines(data: &str) -> String {
    data.replace('\n', "\\n")
        .replace('\r', "\\r")
}

/// 将可序列化的数据转换为 JSON 字符串（纯函数）
///
/// # 参数
/// - `data`: 实现了 `Serialize` trait 的数据
///
/// # 返回
/// - `Result<String>`: 成功时返回 JSON 字符串，失败时返回序列化错误
///
/// # 示例
/// ```
/// use serde::Serialize;
/// #[derive(Serialize)]
/// struct User {
///     name: String,
///     age: u32,
/// }
/// let user = User { name: "Alice".to_string(), age: 30 };
/// let json = serialize_to_json(&user).unwrap();
/// assert!(json.contains("Alice"));
/// ```
pub fn serialize_to_json<T>(data: &T) -> Result<String>
where
    T: Serialize,
{
    let result = serde_json::to_string(data)?;
    Ok(result)
}

/// 输出字符串类型日志
///
/// # 参数
/// - `id`: 目标标识名称
/// - `data`: 要记录的日志数据
///
/// # 返回
/// - `Result<()>`: 成功时返回`Ok(())`，失败时返回错误信息
pub fn log_as_string(id: &str, data: &str) -> Result<()> {
    // 验证ID
    if let Err(e) = validate_id(id) {
        log_error(&format!("ID validation failed: {}", e));
        return Err(e);
    }

    // 获取配置
    let collect_id = get_collect_id()?;

    // 生成时间戳
    let timestamp = generate_timestamp();

    // 转义换行符，确保日志数据在一行上
    let escaped_data = escape_newlines(data);

    // 格式化数据
    let log_data = format_string(&escaped_data);

    // 组合日志
    let formatted_log = format_log(&collect_id, id, &timestamp, &log_data);

    // 输出日志
    info!("{}", formatted_log);

    Ok(())
}

/// 输出数字类型日志（泛型实现）
///
/// # 参数
/// - `id`: 目标标识名称
/// - `data`: 要记录的日志数据
///
/// # 返回
/// - `Result<()>`: 成功时返回`Ok(())`，失败时返回错误信息
pub fn log_as_number<T>(id: &str, data: T) -> Result<()>
where
    T: Num + Display + Copy + 'static,
{
    // 验证ID
    if let Err(e) = validate_id(id) {
        log_error(&format!("ID validation failed: {}", e));
        return Err(e);
    }

    // 获取配置
    let collect_id = get_collect_id()?;

    // 生成时间戳
    let timestamp = generate_timestamp();

    // 格式化数据
    let log_data = format_number(data);

    // 组合日志
    let formatted_log = format_log(&collect_id, id, &timestamp, &log_data);

    // 输出日志
    info!("{}", formatted_log);

    Ok(())
}

/// 输出日期类型日志
///
/// # 参数
/// - `id`: 目标标识名称
/// - `data`: 要记录的日志数据
///
/// # 返回
/// - `Result<()>`: 成功时返回`Ok(())`，失败时返回错误信息
pub fn log_as_date(id: &str, data: NaiveDate) -> Result<()> {
    // 验证ID
    if let Err(e) = validate_id(id) {
        log_error(&format!("ID validation failed: {}", e));
        return Err(e);
    }

    // 获取配置
    let collect_id = get_collect_id()?;

    // 生成时间戳
    let timestamp = generate_timestamp();

    // 格式化数据
    let log_data = format_date(data);

    // 组合日志
    let formatted_log = format_log(&collect_id, id, &timestamp, &log_data);

    // 输出日志
    info!("{}", formatted_log);

    Ok(())
}

/// 输出日期时间类型日志
///
/// # 参数
/// - `id`: 目标标识名称
/// - `data`: 要记录的日志数据
///
/// # 返回
/// - `Result<()>`: 成功时返回`Ok(())`，失败时返回错误信息
pub fn log_as_datetime(id: &str, data: NaiveDateTime) -> Result<()> {
    // 验证ID
    if let Err(e) = validate_id(id) {
        log_error(&format!("ID validation failed: {}", e));
        return Err(e);
    }

    // 获取配置
    let collect_id = get_collect_id()?;

    // 生成时间戳
    let timestamp = generate_timestamp();

    // 格式化数据
    let log_data = format_datetime(data);

    // 组合日志
    let formatted_log = format_log(&collect_id, id, &timestamp, &log_data);

    // 输出日志
    info!("{}", formatted_log);

    Ok(())
}

/// 输出 JSON 类型日志
///
/// # 参数
/// - `id`: 目标标识名称
/// - `data`: 要记录的日志数据，必须实现了 `Serialize` trait
///
/// # 返回
/// - `Result<()>`: 成功时返回`Ok(())`，失败时返回错误信息
///
/// # 示例
/// ```
/// use serde::Serialize;
/// #[derive(Serialize)]
/// struct Event {
///     event_type: String,
///     timestamp: i64,
/// }
/// let event = Event { event_type: "click".to_string(), timestamp: 1234567890 };
/// log_as_json("user_event", &event).unwrap();
/// ```
pub fn log_as_json<T>(id: &str, data: &T) -> Result<()>
where
    T: Serialize,
{
    // 将数据序列化为 JSON 字符串
    let json_string = serialize_to_json(data)?;

    // 调用 log_as_string 写入日志
    let result = log_as_string(id, &json_string);

    // 单一出口点
    result
}

/// 输出错误日志（使用常规方式，不按照特殊格式）
///
/// # 参数
/// - `message`: 错误消息
fn log_error(message: &str) {
    log::error!("[ERROR] {}", message);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{init_log_config, LogConfig, TargetConfig, IdConfig};
    use serde::{Deserialize, Serialize};
    use std::fs;
    use tempfile::NamedTempFile;
    use serial_test::serial;

    // todo: setup_test_config应该只被调用一次，否者会测试报错:
    // thread 'output::tests::test_log_as_string_success' panicked at src/output.rs:387:43:
    // called `Result::unwrap()` on an `Err` value: 日志配置已经初始化，不能重复初始化
    //
    //
    //
    /// 初始化测试配置
    fn setup_test_config() -> NamedTempFile {
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
                },
                {
                    "id": {
                        "name": "user_event",
                        "length": 10
                    }
                }
            ]
        }"#;

        let temp_file = NamedTempFile::new().unwrap();
        fs::write(temp_file.path(), config_content).unwrap();
        temp_file
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
    }

    #[test]
    fn test_serialize_to_json_success() {
        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };
        let result = serialize_to_json(&data);
        assert!(result.is_ok());
        let json = result.unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("42"));
    }

    #[test]
    fn test_serialize_to_json_complex() {
        let data = vec![
            TestData { name: "a".to_string(), value: 1 },
            TestData { name: "b".to_string(), value: 2 },
        ];
        let result = serialize_to_json(&data);
        assert!(result.is_ok());
        let json = result.unwrap();
        assert!(json.contains("a"));
        assert!(json.contains("b"));
    }

    #[test]
    fn test_serialize_to_json_empty_struct() {
        #[derive(Serialize)]
        struct EmptyStruct {}
        let data = EmptyStruct {};
        let result = serialize_to_json(&data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "{}");
    }

    #[test]
    fn test_escape_newlines_simple() {
        let input = "hello\nworld";
        let output = escape_newlines(input);
        assert_eq!(output, "hello\\nworld");
    }

    #[test]
    fn test_escape_newlines_multiple() {
        let input = "line1\nline2\nline3";
        let output = escape_newlines(input);
        assert_eq!(output, "line1\\nline2\\nline3");
    }

    #[test]
    fn test_escape_newlines_carriage_return() {
        let input = "hello\rworld";
        let output = escape_newlines(input);
        assert_eq!(output, "hello\\rworld");
    }

    #[test]
    fn test_escape_newlines_mixed() {
        let input = "hello\r\nworld";
        let output = escape_newlines(input);
        assert_eq!(output, "hello\\r\\nworld");
    }

    #[test]
    fn test_escape_newlines_empty() {
        let input = "";
        let output = escape_newlines(input);
        assert_eq!(output, "");
    }

    #[test]
    fn test_escape_newlines_no_newlines() {
        let input = "hello world";
        let output = escape_newlines(input);
        assert_eq!(output, "hello world");
    }

    #[test]
    fn test_escape_newlines_only_newlines() {
        let input = "\n\n\n";
        let output = escape_newlines(input);
        assert_eq!(output, "\\n\\n\\n");
    }

    #[test]
    #[serial]
    fn test_log_as_string_success() {
        let temp_file = setup_test_config();
        init_log_config(temp_file.path()).unwrap();

        let result = log_as_string("user_login", r#"{"user_id":123,"status":"success"}"#);
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_log_as_string_with_newlines() {
        let temp_file = setup_test_config();
        init_log_config(temp_file.path()).unwrap();

        let data = "line1\nline2\nline3";
        let result = log_as_string("user_login", data);
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_log_as_string_id_not_exist() {
        let temp_file = setup_test_config();
        init_log_config(temp_file.path()).unwrap();

        let result = log_as_string("not_exist", "test data");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("在配置中不存在"));
    }

    #[test]
    #[serial]
    fn test_log_as_string_id_length_mismatch() {
        let temp_file = setup_test_config();
        init_log_config(temp_file.path()).unwrap();

        let result = log_as_string("user_login", "test data");
        // "user_login" 长度为 10，但传入的 id 长度也是 10，所以应该成功
        // 如果传入长度不匹配的 id，比如 "user" (长度 5)
        let result = log_as_string("user", "test data");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("在配置中不存在"));
    }

    #[test]
    #[serial]
    fn test_log_as_number_success() {
        let temp_file = setup_test_config();
        init_log_config(temp_file.path()).unwrap();

        let result = log_as_number("cpu_usage", 75.5f64);
        assert!(result.is_ok());

        let result = log_as_number("cpu_usage", 100i32);
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_log_as_date_success() {
        let temp_file = setup_test_config();
        init_log_config(temp_file.path()).unwrap();

        let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let result = log_as_date("report_date", date);
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_log_as_datetime_success() {
        let temp_file = setup_test_config();
        init_log_config(temp_file.path()).unwrap();

        let datetime = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(10, 30, 0)
            .unwrap();
        let result = log_as_datetime("event_time", datetime);
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_log_as_json_success() {
        let temp_file = setup_test_config();
        init_log_config(temp_file.path()).unwrap();

        let data = TestData {
            name: "test_user".to_string(),
            value: 100,
        };
        let result = log_as_json("user_event", &data);
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_log_as_json_with_nested_struct() {
        let temp_file = setup_test_config();
        init_log_config(temp_file.path()).unwrap();

        #[derive(Serialize)]
        struct NestedData {
            inner: TestData,
            count: u32,
        }

        let data = NestedData {
            inner: TestData { name: "inner".to_string(), value: 50 },
            count: 10,
        };
        let result = log_as_json("user_event", &data);
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_log_as_json_id_not_exist() {
        let temp_file = setup_test_config();
        init_log_config(temp_file.path()).unwrap();

        let data = TestData {
            name: "test".to_string(),
            value: 1,
        };
        let result = log_as_json("not_exist", &data);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("在配置中不存在"));
    }

    #[test]
    #[serial]
    fn test_log_as_json_with_newlines_in_string() {
        let temp_file = setup_test_config();
        init_log_config(temp_file.path()).unwrap();

        let data = TestData {
            name: "line1\nline2".to_string(),
            value: 1,
        };
        let result = log_as_json("user_event", &data);
        assert!(result.is_ok());
    }
}
