
use chrono::{NaiveDate, NaiveDateTime, Utc};
use num_traits::Num;
use std::fmt::Display;

/// 生成16位毫秒时间戳（左补零）
///
/// # 返回
/// - `String`: 16位毫秒时间戳字符串
pub fn generate_timestamp() -> String {
    let millis = Utc::now().timestamp_millis();
    format!("{:016}", millis)
}

/// 格式化字符串类型日志数据
///
/// # 参数
/// - `data`: 字符串数据
///
/// # 返回
/// - `String`: 格式化后的字符串
pub fn format_string(data: &str) -> String {
    data.to_string()
}

/// 格式化数字类型日志数据
///
/// # 参数
/// - `data`: 数字数据
///
/// # 返回
/// - `String`: 格式化后的字符串
pub fn format_number<T>(data: T) -> String
where
    T: Num + Display + Copy + 'static,
{
    data.to_string()
}

/// 格式化日期类型日志数据
///
/// # 参数
/// - `data`: 日期数据
///
/// # 返回
/// - `String`: 格式化后的字符串 (YYYY-MM-DD)
pub fn format_date(data: NaiveDate) -> String {
    data.format("%Y-%m-%d").to_string()
}

/// 格式化日期时间类型日志数据
///
/// # 参数
/// - `data`: 日期时间数据
///
/// # 返回
/// - `String`: 格式化后的字符串 (YYYY-MM-DD HH:MM:SS)
pub fn format_datetime(data: NaiveDateTime) -> String {
    data.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// 组合完整的日志格式
///
/// # 参数
/// - `collect_id`: 采集标识符
/// - `id`: 目标标识名称
/// - `timestamp`: 时间戳
/// - `log_data`: 日志数据
///
/// # 返回
/// - `String`: 完整的日志格式字符串
pub fn format_log(collect_id: &str, id: &str, timestamp: &str, log_data: &str) -> String {
    format!("{}{}{}{}", collect_id, id, timestamp, log_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_generate_timestamp() {
        let timestamp = generate_timestamp();
        assert_eq!(timestamp.len(), 16);
        // 验证是数字
        assert!(timestamp.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_format_string() {
        let result = format_string("hello world");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_format_number_integer() {
        let result = format_number(123i32);
        assert_eq!(result, "123");
    }

    #[test]
    fn test_format_number_float() {
        let result = format_number(75.5f64);
        assert_eq!(result, "75.5");
    }

    #[test]
    fn test_format_number_unsigned() {
        let result = format_number(42u8);
        assert_eq!(result, "42");
    }

    #[test]
    fn test_format_date() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let result = format_date(date);
        assert_eq!(result, "2024-01-01");
    }

    #[test]
    fn test_format_datetime() {
        let datetime = NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(10, 30, 0)
            .unwrap();
        let result = format_datetime(datetime);
        assert_eq!(result, "2024-01-01 10:30:00");
    }

    #[test]
    fn test_format_log() {
        let collect_id = "RESUME-AGENT";
        let id = "user_login";
        let timestamp = "0001704067200001";
        let log_data = r#"{"user_id":123,"status":"success"}"#;
        
        let result = format_log(collect_id, id, timestamp, log_data);
        assert_eq!(
            result,
            "RESUME-AGENTuser_login0001704067200001{\"user_id\":123,\"status\":\"success\"}"
        );
    }
}
