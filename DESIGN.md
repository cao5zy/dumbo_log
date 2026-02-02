
# 日志输出模块概要设计 v2.3（最终版）

## 1. 设计目标
在现有日志初始化模块基础上，增加日志输出API，按照指定格式输出日志，便于后续分析和解析。

## 2. 配置设计

### 2.1 配置文件结构
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
    }
  ]
}
```

### 2.2 配置字段说明
- `collect_id`: 采集标识符，用于标识日志来源
- `collect_id_length`: collect_id的固定长度
- `targets`: 目标配置数组
  - `id.name`: 目标标识名称
  - `id.length`: id.name的固定长度

### 2.3 配置加载与存储
- 支持从JSON文件加载配置
- 使用全局单例存储配置（`once_cell`）
- 提供配置初始化函数 `init_log_config(config_path: &Path) -> Result<()>`
- 配置在应用程序退出时释放

## 3. 日志输出API设计

### 3.1 API函数列表

#### 3.1.1 字符串类型
```rust
pub fn log_as_string(id: &str, data: &str) -> Result<()>
```

#### 3.1.2 数字类型（泛型实现）
使用泛型 trait 统一处理所有数字类型：
- 整数：`i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- 浮点：`f32`, `f64`

```rust
pub fn log_as_number<T>(id: &str, data: T) -> Result<()>
where
    T: Num + Display + Copy + 'static
```

#### 3.1.3 日期类型
使用 `chrono::NaiveDate` 表示日期（UTC时间，无时区信息）

```rust
pub fn log_as_date(id: &str, data: NaiveDate) -> Result<()>
```

#### 3.1.4 日期时间类型
使用 `chrono::NaiveDateTime` 表示日期时间（UTC时间，无时区信息）

```rust
pub fn log_as_datetime(id: &str, data: NaiveDateTime) -> Result<()>
```

### 3.2 参数说明
- `id`: 目标标识名称，对应配置中的 `id.name`
- `data`: 要记录的日志数据

### 3.3 错误处理
- 如果传入的 `id` 长度与配置中的 `id.length` 不匹配，返回错误并输出错误日志
- 如果传入的 `id` 在配置的 `targets` 中不存在，返回错误并输出错误日志
- 如果配置未初始化，返回错误并输出错误日志
- 错误日志使用常规方式输出（不按照特殊格式）

## 4. 日志格式设计

### 4.1 完整格式
```
[log4rs前缀][collect_id(collect_id_length字符)][id.name(id.length字符)][timestamp(16字符)][log_data]
```

### 4.2 格式说明
- **log4rs前缀**: 由log4rs自动生成，包含时间、日志级别等（reading_skip部分）
- **collect_id**: 从配置中读取，固定长度为 `collect_id_length`
- **id.name**: 由API参数传入，固定长度为 `id.length`
- **timestamp**: 16位数字，毫秒时间戳，左补零到16位
- **log_data**: 实际的日志数据

### 4.3 时间戳生成方案

**方案：毫秒时间戳 + 左补零到16位**

- 获取方式：`chrono::Utc::now().timestamp_millis()`
- 格式：16位数字，左补零
- 示例：
  - 原始毫秒时间戳：`1704115800001`（13位）
  - 左补零到16位：`0001704115800001`

**时间戳长度分析：**

| 年份 | 毫秒时间戳 | 原始长度 | 补零后长度 |
|------|-----------|---------|-----------|
| 1970 | 0 | 1位 | 0000000000000000 |
| 2000 | 946684800000 | 12位 | 0000946684800000 |
| 2024 | 1704067200000 | 13位 | 0001704067200000 |
| 2100 | 4102444800000 | 13位 | 0004102444800000 |
| 3000 | 32503680000000 | 14位 | 0032503680000000 |
| 9999 | 253402300799000 | 15位 | 0253402300799000 |

**优势：**
1. 高效：直接获取时间戳，O(1)时间复杂度
2. 固定长度：始终为16位，便于固定长度解析
3. 足够空间：16位可以支持到公元 292,278,994 年（i64最大值）

### 4.4 示例
假设：
- log4rs前缀：`2024-01-01 10:30:00 INFO `
- collect_id：`RESUME-AGENT`（长度12）
- id.name：`user_login`（长度10）
- timestamp：`0001704067200001`（毫秒时间戳，左补零到16位）
- log_data：`{"user_id":123,"status":"success"}`

最终输出：
```
2024-01-01 10:30:00 INFO RESUME-AGENTuser_login0001704067200001{"user_id":123,"status":"success"}
```

## 5. 模块结构

### 5.1 模块划分
```
src/
├── lib.rs           # 主入口，导出公共API
├── config.rs        # 配置加载和管理
├── formatter.rs     # 日志格式化（纯函数）
└── output.rs        # 日志输出API（副作用函数）
```

### 5.2 职责划分
- **config.rs**: 配置文件解析、配置存储（全局单例）、配置查询
- **formatter.rs**: 日志格式化逻辑（纯函数，易于测试）
- **output.rs**: 日志输出API，调用formatter并使用log crate输出

## 6. 依赖项

### 6.1 新增依赖
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = "0.4"
num-traits = "0.2"  # 用于数字类型的泛型trait
once_cell = "1.0"   # 用于全局单例
```

### 6.2 现有依赖
- anyhow: 错误处理
- log: 日志门面
- log4rs: 日志实现

## 7. 实现策略

### 7.1 纯函数设计
- formatter模块中的格式化函数设计为纯函数
- 为纯函数编写单元测试
- output模块中的API函数调用纯函数，产生副作用（实际输出日志）

### 7.2 错误处理
- 使用 `anyhow::Result` 处理错误
- 提供清晰的错误信息
- 配置缺失、ID长度不匹配、ID不存在等情况需要明确报错
- 错误发生时，使用常规方式输出错误日志（不按照特殊格式）

### 7.3 全局单例
- 使用 `once_cell::sync::OnceCell` 实现全局单例
- 配置在应用启动时初始化，退出时释放
- 使用 `RefCell<LogConfig>` 存储配置，支持内部可变性
- 最小化借用生命周期

### 7.4 时间戳生成
- 使用 `chrono::Utc::now().timestamp_millis()` 获取毫秒时间戳
- 左补零到16位
- 高效且固定长度

### 7.5 ID验证
- 验证 `id` 是否在配置的 `targets` 中存在
- 验证 `id` 长度是否与配置匹配
- 不匹配时返回错误，并输出错误日志
- 错误日志格式：`[ERROR] ID validation failed: {reason}`

## 8. 使用示例

### 8.1 初始化配置
```rust
use dumbo_log::{init_log, init_log_config};
use std::path::Path;

fn main() -> Result<()> {
    // 初始化日志系统
    init_log(Path::new("app.log"), None)?;
    
    // 初始化日志输出配置
    init_log_config(Path::new("log_config.json"))?;
    
    // 使用日志输出API
    dumbo_log::log_as_string("user_login", "{\"user_id\":123,\"status\":\"success\"}")?;
    dumbo_log::log_as_number("cpu_usage", 75.5)?;
    dumbo_log::log_as_date("report_date", chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())?;
    dumbo_log::log_as_datetime("event_time", chrono::Utc::now().naive_utc())?;
    
    Ok(())
}
```

### 8.2 配置文件示例
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
    }
  ]
}
```

## 9. 测试策略

### 9.1 单元测试
- formatter模块的纯函数测试
- 配置解析测试
- ID验证测试（存在性、长度）
- 时间戳生成测试（16位，左补零）

### 9.2 集成测试
- 完整的日志输出流程测试
- 错误处理测试（ID不存在、长度不匹配）
- 多线程安全性测试（如果需要）

## 10. 注意事项

1. **UTC时间**：所有日期时间都使用UTC时间，确保跨时区一致性
2. **ID验证**：严格验证ID存在性和长度，不匹配时报错
3. **全局单例**：配置只初始化一次，避免重复初始化
4. **错误日志**：错误发生时使用常规方式输出，避免递归错误
5. **时间戳格式**：16位毫秒时间戳，左补零，固定长度
6. **性能考虑**：直接获取时间戳，高效且固定长度
