
# 项目信息

## 项目概述
**项目名称**: dumbo_log  
**项目类型**: 日志写入库  
**开发语言**: Rust  
**项目目标**: 提供一个结构化的日志输出库，支持按照固定格式输出日志，便于后续分析和解析

## 设计目标
在现有日志初始化模块基础上，增加日志输出API，按照指定格式输出日志，便于后续分析和解析。

## 核心功能

### 1. 日志初始化
- 支持文件日志输出
- 支持控制台日志输出（可选）
- 支持通过环境变量配置日志级别
- 基于 log4rs 实现

### 2. 日志输出配置
- 支持从JSON文件加载配置
- 全局单例存储配置
- 配置包含：collect_id、collect_id_length、targets

### 3. 日志输出API
- `log_as_string(id: &str, data: &str)`: 输出字符串类型日志
- `log_as_number<T>(id: &str, data: T)`: 输出数字类型日志（泛型支持）
- `log_as_date(id: &str, data: NaiveDate)`: 输出日期类型日志
- `log_as_datetime(id: &str, data: NaiveDateTime)`: 输出日期时间类型日志

### 4. 日志格式化
- 固定格式：`[log4rs前缀][collect_id][id.name][timestamp][log_data]`
- 时间戳：16位毫秒时间戳，左补零
- UTC时间，确保跨时区一致性

### 5. 错误处理
- ID存在性验证
- ID长度验证
- 配置初始化检查
- 错误日志使用常规方式输出

## 技术架构

### 模块结构
```
src/
├── lib.rs           # 主入口，导出公共API
├── config.rs        # 配置加载和管理
├── formatter.rs     # 日志格式化（纯函数）
└── output.rs        # 日志输出API（副作用函数）
```

### 职责划分
- **config.rs**: 配置文件解析、配置存储（全局单例）、配置查询
- **formatter.rs**: 日志格式化逻辑（纯函数，易于测试）
- **output.rs**: 日志输出API，调用formatter并使用log crate输出

## API设计

### 初始化API
```rust
// 初始化日志系统
pub fn init_log(log_path: &Path, prefix: Option<&str>) -> Result<()>
pub fn init_log_with_console(log_path: &Path, prefix: Option<&str>, enable_console: bool) -> Result<()>

// 初始化日志输出配置
pub fn init_log_config(config_path: &Path) -> Result<()>
```

### 日志输出API
```rust
// 字符串类型
pub fn log_as_string(id: &str, data: &str) -> Result<()>

// 数字类型（泛型）
pub fn log_as_number<T>(id: &str, data: T) -> Result<()>
where
    T: Num + Display + Copy + 'static

// 日期类型
pub fn log_as_date(id: &str, data: NaiveDate) -> Result<()>

// 日期时间类型
pub fn log_as_datetime(id: &str, data: NaiveDateTime) -> Result<()>
```

## 配置格式

### 配置文件结构
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

### 配置字段说明
- `collect_id`: 采集标识符，用于标识日志来源
- `collect_id_length`: collect_id的固定长度
- `targets`: 目标配置数组
  - `id.name`: 目标标识名称
  - `id.length`: id.name的固定长度

## 日志格式

### 完整格式
```
[log4rs前缀][collect_id(collect_id_length字符)][id.name(id.length字符)][timestamp(16字符)][log_data]
```

### 格式说明
- **log4rs前缀**: 由log4rs自动生成，包含时间、日志级别等（reading_skip部分）
- **collect_id**: 从配置中读取，固定长度为 `collect_id_length`
- **id.name**: 由API参数传入，固定长度为 `id.length`
- **timestamp**: 16位数字，毫秒时间戳，左补零到16位
- **log_data**: 实际的日志数据

### 示例
```
2024-01-01 10:30:00 INFO RESUME-AGENTuser_login0001704067200001{"user_id":123,"status":"success"}
```

## 依赖项

### 现有依赖
```toml
[dependencies]
anyhow = "1.0"
log = "0.4"
log4rs = "1.0"
```

### 新增依赖
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = "0.4"
num-traits = "0.2"
once_cell = "1.0"
```

## 开发规范

### 1. 纯函数设计
- formatter模块中的格式化函数设计为纯函数
- 为纯函数编写单元测试
- output模块中的API函数调用纯函数，产生副作用

### 2. 错误处理
- 使用 `anyhow::Result` 处理错误
- 提供清晰的错误信息
- 错误发生时使用常规方式输出错误日志

### 3. 全局单例
- 使用 `once_cell::sync::OnceCell` 实现全局单例
- 配置在应用启动时初始化，退出时释放
- 使用 `RefCell<LogConfig>` 存储配置，支持内部可变性
- 最小化借用生命周期

### 4. 时间处理
- 所有日期时间都使用UTC时间
- 时间戳格式：16位毫秒时间戳，左补零
- 使用 `chrono::Utc::now().timestamp_millis()` 获取时间戳

### 5. ID验证
- 验证 `id` 是否在配置的 `targets` 中存在
- 验证 `id` 长度是否与配置匹配
- 不匹配时返回错误，并输出错误日志

## 使用示例

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

## 测试策略

### 单元测试
- formatter模块的纯函数测试
- 配置解析测试
- ID验证测试（存在性、长度）
- 时间戳生成测试（16位，左补零）

### 集成测试
- 完整的日志输出流程测试
- 错误处理测试（ID不存在、长度不匹配）
- 多线程安全性测试（如果需要）

## 注意事项

1. **UTC时间**：所有日期时间都使用UTC时间，确保跨时区一致性
2. **ID验证**：严格验证ID存在性和长度，不匹配时报错
3. **全局单例**：配置只初始化一次，避免重复初始化
4. **错误日志**：错误发生时使用常规方式输出，避免递归错误
5. **时间戳格式**：16位毫秒时间戳，左补零，固定长度
6. **性能考虑**：直接获取时间戳，高效且固定长度

## 扩展方向

### 1. 日志解析
- 实现日志解析模块，按照固定格式解析日志
- 支持批量解析和实时解析

### 2. 数据存储
- 实现日志存储模块，支持多种存储后端
- 支持分区策略和数据保留策略

### 3. 数据展示
- 实现数据展示模块，支持多种可视化方式
- 支持表格、折线图、柱状图、单值等展示方式

### 4. 性能优化
- 支持异步日志输出
- 支持批量写入
- 支持日志压缩

## 相关文档
- [设计文档](./DESIGN.md)
- [日志分析定义方案](./log_analysis_definition_v1.md)
