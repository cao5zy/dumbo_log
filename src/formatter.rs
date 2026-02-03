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

/// 将字符串格式化为固定长度
///
/// # 参数
/// - `s`: 原始字符串
/// - `length`: 目标长度
///
/// # 返回
/// - `String`: 固定长度的字符串
///   - 如果原始字符串长度等于目标长度，直接返回
///   - 如果原始字符串长度小于目标长度，右补空格
///   - 如果原始字符串长度大于目标长度，截断到目标长度
pub fn format_fixed_length(s: &str, length: usize) -> String {
    let s_len = s.len();
    if s_len == length {
        s.to_string()
    } else if s_len < length {
        // 右补空格
        format!("{:<width$}", s, width = length)
    } else {
        // 截断
        s[..length].to_string()
    }
}

/// 组合完整的日志格式
///
/// # 参数
/// - `collect_id`: 采集标识符
/// - `collect_id_length`: collect_id的固定长度
/// - `id`: 目标标识名称
/// - `id_length`: id的固定长度
/// - `timestamp`: 时间戳
/// - `log_data`: 日志数据
///
/// # 返回
/// - `String`: 完整的日志格式字符串
pub fn format_log(
    collect_id: &str,
    collect_id_length: usize,
    id: &str,
    id_length: usize,
    timestamp: &str,
    log_data: &str,
) -> String {
    let formatted_collect_id = format_fixed_length(collect_id, collect_id_length);
    let formatted_id = format_fixed_length(id, id_length);
    format!(
        "{}{}{}{}",
        formatted_collect_id, formatted_id, timestamp, log_data
    )
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
    fn test_format_fixed_length_exact() {
        let result = format_fixed_length("hello", 5);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_format_fixed_length_shorter() {
        let result = format_fixed_length("hi", 5);
        assert_eq!(result, "hi   ");
    }

    #[test]
    fn test_format_fixed_length_longer() {
        let result = format_fixed_length("hello world", 5);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_format_log() {
        let collect_id = "RESUME-AGENT";
        let collect_id_length = 12;
        let id = "user_login";
        let id_length = 10;
        let timestamp = "0001704067200001";
        let log_data = r#"{"user_id":123,"status":"success"}"#;

        let result = format_log(
            collect_id,
            collect_id_length,
            id,
            id_length,
            timestamp,
            log_data,
        );
        assert_eq!(
            result,
            "RESUME-AGENTuser_login0001704067200001{\"user_id\":123,\"status\":\"success\"}"
        );
    }

    #[test]
    fn test_format_log_with_padding() {
        let collect_id = "SHORT";
        let collect_id_length = 12;
        let id = "login";
        let id_length = 10;
        let timestamp = "0001704067200001";
        let log_data = r#"{"user_id":123}"#;

        let result = format_log(
            collect_id,
            collect_id_length,
            id,
            id_length,
            timestamp,
            log_data,
        );
        assert_eq!(
            result,
            "SHORT       login     0001704067200001{\"user_id\":123}"
        );
    }

    #[test]
    fn test_format_log_with_truncation() {
        let collect_id = "VERY-LONG-COLLECT-ID";
        let collect_id_length = 12;
        let id = "very-long-id-name";
        let id_length = 10;
        let timestamp = "0001704067200001";
        let log_data = r#"{"user_id":123}"#;

        let result = format_log(
            collect_id,
            collect_id_length,
            id,
            id_length,
            timestamp,
            log_data,
        );
        assert_eq!(
            result,
            "VERY-LONG-COvery-long-0001704067200001{\"user_id\":123}"
        );
    }
}
