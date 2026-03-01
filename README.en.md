# dumbo_log - A Simple and Efficient Rust Logging Library

[![Crates.io](https://img.shields.io/crates/v/dumbo_log)](https://crates.io/crates/dumbo_log)
[![Docs.rs](https://docs.rs/dumbo_log/badge.svg)](https://docs.rs/dumbo_log)

[Home-https://www.craftaidhub.com](https://www.craftaidhub.com)

dumbo_log is a lightweight Rust logging library designed specifically for the dumbo series of projects, providing simple and easy-to-use logging functionality along with powerful structured logging capabilities.

## 🌟 Key Features

- **Simple to Use**: Initialize the logging system with just one line of code
- **Flexible Configuration**: Dynamically control log levels via environment variables
- **Dual Output Support**: Supports both file and console output simultaneously
- **Structured Logging**: Provides standardized structured log format for easier analysis
- **Type Safety**: Supports logging multiple data types (strings, numbers, dates, etc.)
- **Zero-Cost Abstraction**: Minimal performance overhead when not in use

## 🚀 Quick Start

### 1. Installation

```bash
cargo add dumbo_log
```

### 2. Basic Usage

```rust
use std::path::Path;
use dumbo_log::init_log;
use anyhow::Result;

fn main() -> Result<()> {
    // Initialize the logging system
    let log_path = Path::new("./app.log");
    init_log(log_path, None)?;
    
    // Start logging
    log::info!("Application started successfully");
    log::warn!("This is a warning message");
    log::error!("An error occurred");
    
    Ok(())
}
```

## 📚 Complete Documentation

### Basic Logging Features

#### Initializing the Logging System

The `init_log` function initializes the logging system and supports dynamic log level configuration via environment variables.

```rust
use std::path::Path;
use dumbo_log::init_log; 
use anyhow::Result;

fn main() -> Result<()> {
    let log_path = Path::new("./app.log");
    
    // Initialize logging with default prefix, using DUMBO_LOG_LEVEL environment variable to set LevelFilter
    init_log(log_path, None)?;
    
    // Or initialize with custom prefix, using MY_APP_DUMBO_LOG_LEVEL environment variable to set LevelFilter
    // init_log(log_path, Some("MY_APP"))?;
    
    log::info!("Logging system initialized successfully");
    log::warn!("This is a warning log");
    log::error!("This is an error log");
    
    Ok(())
}
```

**Log Level Information**:
- Default level: `Info`
- Available levels: `Debug`, `Info`, `Warn`, `Error`

#### Console Output Feature

Use the `init_log_with_console` function to output logs to both file and console:

```rust
use std::path::Path;
use dumbo_log::init_log_with_console; 
use anyhow::Result;

fn main() -> Result<()> {
    let log_path = Path::new("./app.log");
    
    // Initialize logging with console output enabled
    init_log_with_console(log_path, None, true)?;
    
    log::info!("This log will be output to both file and console");
    
    Ok(())
}
```

**Parameter Description**:
- `log_path`: Log file path
- `prefix`: Optional prefix for constructing environment variable name
- `enable_console`: 
  - `true`: Enable console output
  - `false`: Write to file only

**Usage Scenarios**:
- **Development & Debugging**: Enable console output for real-time monitoring
- **Production Environment**: Disable console output, keep file logs only
- **Dynamic Switching**: Flexibly switch output methods via configuration parameters

### Structured Logging Features

#### Feature Overview

dumbo_log provides structured logging APIs that output logs in a fixed format, making them easier to analyze and parse.

**Log Format**:
```
[log4rs prefix][collect_id][id.name][timestamp][log_data]
```

#### Configuration File

Create a `log_config.json` configuration file:

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
    }
  ]
}
```

#### Complete Usage Example

```rust
use dumbo_log::{init_log, init_log_config, log_as_string, log_as_number, log_as_date, log_as_datetime};
use std::path::Path;
use anyhow::Result;
use chrono::{NaiveDate, Utc};

fn main() -> Result<()> {
    // 1. Initialize the logging system
    init_log(Path::new("app.log"), None)?;
    
    // 2. Initialize log configuration
    init_log_config(Path::new("log_config.json"))?;
    
    // 3. Use structured logging
    log_as_string("user_login", r#"{"user_id":123,"status":"success"}"#)?;
    log_as_number("cpu_usage", 75.5)?;
    log_as_number("cpu_usage", 80i32)?;
    
    let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    log_as_date("report_date", date)?;
    
    let datetime = Utc::now().naive_utc();
    log_as_datetime("event_time", datetime)?;
    
    Ok(())
}
```

#### API Reference

| Function | Description | Example |
|------|------|------|
| `init_log_config` | Load log configuration from JSON file | `init_log_config(Path::new("log_config.json"))?` |
| `log_as_string` | Output string-type logs | `log_as_string("user_login", r#"{"user_id":123}"#)?` |
| `log_as_number` | Output number-type logs | `log_as_number("cpu_usage", 75.5)?` |
| `log_as_date` | Output date-type logs | `log_as_date("report_date", date)?` |
| `log_as_datetime` | Output datetime-type logs | `log_as_datetime("event_time", datetime)?` |

#### Log Format Details

**Complete Format**:
```
[log4rs prefix][collect_id(collect_id_length chars)][id.name(id.length chars)][timestamp(16 chars)][log_data]
```

**Example Output**:
```
2024-01-01 10:30:00 INFO RESUME-AGENTuser_login0001704067200001{"user_id":123,"status":"success"}
```

#### Error Handling

The API performs the following validations:
1. **Configuration Not Initialized**: Returns an error if `init_log_config` hasn't been called
2. **ID Not Found**: Returns an error if the provided `id` doesn't exist in the configuration
3. **ID Length Mismatch**: Returns an error if the provided `id` length doesn't match the configuration

#### Important Notes

- **UTC Time**: All datetime values use UTC time to ensure cross-timezone consistency
- **ID Validation**: Strictly validates ID existence and length, returns errors on mismatch
- **Global Singleton**: Configuration is initialized only once to prevent duplicate initialization
- **Timestamp Format**: 16-digit millisecond timestamp, left-padded with zeros, fixed length
- **Thread Safety**: Configuration uses `OnceCell` for thread-safe access

## 🤝 Contribution Guide

Issues and Pull Requests are welcome! Please ensure:

1. Code follows Rust best practices
2. Add corresponding test cases
3. Update relevant documentation

## 📄 License

This project is licensed under the [MIT License](LICENSE).

---

More content available at my [personal website-https://www.craftaidhub.com](https://www.craftaidhub.com) for deeper collaboration and communication
