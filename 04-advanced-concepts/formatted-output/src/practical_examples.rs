//! # Rust 格式化输出实际应用案例
//!
//! 本模块展示 Rust 格式化输出在实际项目中的应用，包括日志系统、
//! 数据展示、调试信息、报表生成、配置文件输出等各种实际场景。
//!
//! ## 应用场景
//!
//! 1. **日志系统** - 结构化日志输出、日志级别、时间戳格式化
//! 2. **数据展示** - 表格输出、图表绘制、统计报告
//! 3. **调试信息** - 错误诊断、性能分析、状态监控
//! 4. **用户界面** - 进度条、菜单、交互提示
//! 5. **文件输出** - 配置文件、报表、数据导出
//! 6. **网络通信** - API 响应、协议格式化、消息序列化
//!
//! ## 设计原则
//!
//! 1. **可读性** - 输出内容清晰易懂
//! 2. **一致性** - 格式化风格统一
//! 3. **性能** - 高效的格式化操作
//! 4. **可维护性** - 易于修改和扩展

use std::collections::{BTreeMap, HashMap};
use std::fmt::{self, Display, Write};
use std::fs::File;
use std::io::{self, Write as IoWrite};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
// use chrono::{DateTime, Local, Utc}; // 注释掉，使用标准库的时间

/// 日志级别枚举
///
/// 定义不同的日志级别，用于日志系统。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Fatal = 5,
}

impl LogLevel {
    /// 获取日志级别的字符串表示
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
        }
    }

    /// 获取日志级别的颜色代码
    pub fn color_code(&self) -> &'static str {
        match self {
            LogLevel::Trace => "\x1b[37m", // 白色
            LogLevel::Debug => "\x1b[36m", // 青色
            LogLevel::Info => "\x1b[32m",  // 绿色
            LogLevel::Warn => "\x1b[33m",  // 黄色
            LogLevel::Error => "\x1b[31m", // 红色
            LogLevel::Fatal => "\x1b[35m", // 紫色
        }
    }

    /// 获取日志级别的图标
    pub fn icon(&self) -> &'static str {
        match self {
            LogLevel::Trace => "🔍",
            LogLevel::Debug => "🐛",
            LogLevel::Info => "ℹ️",
            LogLevel::Warn => "⚠️",
            LogLevel::Error => "❌",
            LogLevel::Fatal => "💀",
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // 使用 {:#} 格式时显示带颜色和图标的版本
            write!(
                f,
                "{}{} {}{}",
                self.color_code(),
                self.icon(),
                self.as_str(),
                "\x1b[0m"
            )
        } else {
            write!(f, "{}", self.as_str())
        }
    }
}

/// 日志记录结构体
///
/// 包含完整的日志信息，支持多种格式化输出。
#[derive(Debug, Clone)]
pub struct LogRecord {
    pub timestamp: SystemTime,
    pub level: LogLevel,
    pub target: String,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub thread_id: String,
    pub metadata: HashMap<String, String>,
}

impl LogRecord {
    /// 创建新的日志记录
    pub fn new(level: LogLevel, target: &str, message: &str) -> Self {
        Self {
            timestamp: SystemTime::now(),
            level,
            target: target.to_string(),
            message: message.to_string(),
            file: None,
            line: None,
            thread_id: format!("{:?}", thread::current().id()),
            metadata: HashMap::new(),
        }
    }

    /// 设置文件和行号信息
    pub fn with_location(mut self, file: &str, line: u32) -> Self {
        self.file = Some(file.to_string());
        self.line = Some(line);
        self
    }

    /// 添加元数据
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    /// 格式化时间戳
    pub fn format_timestamp(&self, format: &str) -> String {
        let duration = self
            .timestamp
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let secs = duration.as_secs();
        let nanos = duration.subsec_nanos();

        // 简单的时间格式化，不依赖 chrono
        match format {
            "simple" => format!("{}", secs),
            "compact" => format!("{}", secs),
            "time_only" => format!("{}.{:03}", secs, nanos / 1_000_000),
            _ => format!("{}.{:09}", secs, nanos),
        }
    }
}

/// 为 LogRecord 实现 Display trait
impl Display for LogRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 标准日志格式
        write!(
            f,
            "[{}] {:5} [{}] {}",
            self.format_timestamp("simple"),
            self.level,
            self.target,
            self.message
        )?;

        // 添加位置信息
        if let (Some(file), Some(line)) = (&self.file, &self.line) {
            write!(f, " ({}:{})", file, line)?;
        }

        // 添加元数据
        if !self.metadata.is_empty() {
            write!(f, " {{");
            let mut first = true;
            for (key, value) in &self.metadata {
                if !first {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", key, value)?;
                first = false;
            }
            write!(f, "}}");
        }

        Ok(())
    }
}

/// 日志格式化器
///
/// 提供多种日志输出格式。
pub struct LogFormatter {
    pub use_colors: bool,
    pub show_thread: bool,
    pub show_location: bool,
    pub timestamp_format: String,
}

impl LogFormatter {
    /// 创建默认格式化器
    pub fn new() -> Self {
        Self {
            use_colors: true,
            show_thread: false,
            show_location: true,
            timestamp_format: "simple".to_string(),
        }
    }

    /// 创建紧凑格式化器
    pub fn compact() -> Self {
        Self {
            use_colors: false,
            show_thread: false,
            show_location: false,
            timestamp_format: "time_only".to_string(),
        }
    }

    /// 创建详细格式化器
    pub fn verbose() -> Self {
        Self {
            use_colors: true,
            show_thread: true,
            show_location: true,
            timestamp_format: "rfc3339".to_string(),
        }
    }

    /// 格式化日志记录
    pub fn format(&self, record: &LogRecord) -> String {
        let mut output = String::new();

        // 时间戳
        write!(
            output,
            "[{}]",
            record.format_timestamp(&self.timestamp_format)
        )
        .unwrap();

        // 日志级别
        if self.use_colors {
            write!(output, " {:#}", record.level).unwrap();
        } else {
            write!(output, " {:5}", record.level).unwrap();
        }

        // 线程信息
        if self.show_thread {
            write!(output, " [{}]", record.thread_id).unwrap();
        }

        // 目标模块
        write!(output, " [{}]", record.target).unwrap();

        // 消息
        write!(output, " {}", record.message).unwrap();

        // 位置信息
        if self.show_location {
            if let (Some(file), Some(line)) = (&record.file, &record.line) {
                write!(output, " ({}:{})", file, line).unwrap();
            }
        }

        // 元数据
        if !record.metadata.is_empty() {
            write!(output, " {{").unwrap();
            let mut first = true;
            for (key, value) in &record.metadata {
                if !first {
                    write!(output, ", ").unwrap();
                }
                write!(output, "{}: {}", key, value).unwrap();
                first = false;
            }
            write!(output, "}}").unwrap();
        }

        output
    }
}

/// 数据表格结构体
///
/// 用于格式化表格数据的显示。
#[derive(Debug, Clone)]
pub struct DataTable {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub title: Option<String>,
    pub show_index: bool,
    pub alignment: Vec<Alignment>,
}

/// 对齐方式枚举
#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

impl DataTable {
    /// 创建新的数据表格
    pub fn new(headers: Vec<String>) -> Self {
        let alignment = vec![Alignment::Left; headers.len()];
        Self {
            headers,
            rows: Vec::new(),
            title: None,
            show_index: false,
            alignment,
        }
    }

    /// 设置标题
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// 显示行索引
    pub fn with_index(mut self) -> Self {
        self.show_index = true;
        self
    }

    /// 设置列对齐方式
    pub fn with_alignment(mut self, alignment: Vec<Alignment>) -> Self {
        self.alignment = alignment;
        self
    }

    /// 添加数据行
    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    /// 计算列宽度
    fn calculate_column_widths(&self) -> Vec<usize> {
        let mut widths = self.headers.iter().map(|h| h.len()).collect::<Vec<_>>();

        // 考虑索引列
        if self.show_index {
            widths.insert(0, format!("{}", self.rows.len()).len().max(3));
        }

        // 检查数据行
        for row in &self.rows {
            let start_idx = if self.show_index { 1 } else { 0 };
            for (i, cell) in row.iter().enumerate() {
                if start_idx + i < widths.len() {
                    widths[start_idx + i] = widths[start_idx + i].max(cell.len());
                }
            }
        }

        widths
    }

    /// 格式化单元格
    fn format_cell(&self, content: &str, width: usize, alignment: Alignment) -> String {
        match alignment {
            Alignment::Left => format!("{:<width$}", content, width = width),
            Alignment::Right => format!("{:>width$}", content, width = width),
            Alignment::Center => format!("{:^width$}", content, width = width),
        }
    }
}

/// 为 DataTable 实现 Display trait
impl Display for DataTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let widths = self.calculate_column_widths();
        let total_width = widths.iter().sum::<usize>() + widths.len() * 3 + 1;

        // 标题
        if let Some(title) = &self.title {
            writeln!(
                f,
                "{:═^width$}",
                format!(" {} ", title),
                width = total_width
            )?;
        }

        // 表头
        write!(f, "│")?;
        if self.show_index {
            write!(f, " {:^width$} │", "#", width = widths[0])?;
        }

        let header_start = if self.show_index { 1 } else { 0 };
        for (i, header) in self.headers.iter().enumerate() {
            let width = widths[header_start + i];
            let alignment = self.alignment.get(i).copied().unwrap_or(Alignment::Left);
            write!(f, " {} │", self.format_cell(header, width, alignment))?;
        }
        writeln!(f)?;

        // 分隔线
        write!(f, "├")?;
        for (i, &width) in widths.iter().enumerate() {
            if i > 0 {
                write!(f, "┼")?;
            }
            write!(f, "{:─<width$}", "", width = width + 2)?;
        }
        writeln!(f, "┤")?;

        // 数据行
        for (row_idx, row) in self.rows.iter().enumerate() {
            write!(f, "│")?;

            if self.show_index {
                write!(f, " {:>width$} │", row_idx + 1, width = widths[0])?;
            }

            let data_start = if self.show_index { 1 } else { 0 };
            for (i, cell) in row.iter().enumerate() {
                let width = widths[data_start + i];
                let alignment = self.alignment.get(i).copied().unwrap_or(Alignment::Left);
                write!(f, " {} │", self.format_cell(cell, width, alignment))?;
            }
            writeln!(f)?;
        }

        // 底部边框
        write!(f, "└")?;
        for (i, &width) in widths.iter().enumerate() {
            if i > 0 {
                write!(f, "┴")?;
            }
            write!(f, "{:─<width$}", "", width = width + 2)?;
        }
        writeln!(f, "┘")?;

        Ok(())
    }
}

/// 进度条结构体
///
/// 用于显示操作进度。
#[derive(Debug, Clone)]
pub struct ProgressBar {
    pub current: u64,
    pub total: u64,
    pub width: usize,
    pub show_percentage: bool,
    pub show_numbers: bool,
    pub show_eta: bool,
    pub start_time: Instant,
    pub message: String,
}

impl ProgressBar {
    /// 创建新的进度条
    pub fn new(total: u64) -> Self {
        Self {
            current: 0,
            total,
            width: 40,
            show_percentage: true,
            show_numbers: true,
            show_eta: true,
            start_time: Instant::now(),
            message: String::new(),
        }
    }

    /// 设置进度条宽度
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// 设置消息
    pub fn with_message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    /// 更新进度
    pub fn set_progress(&mut self, current: u64) {
        self.current = current.min(self.total);
    }

    /// 增加进度
    pub fn increment(&mut self, delta: u64) {
        self.current = (self.current + delta).min(self.total);
    }

    /// 计算百分比
    pub fn percentage(&self) -> f64 {
        if self.total == 0 {
            100.0
        } else {
            (self.current as f64 / self.total as f64) * 100.0
        }
    }

    /// 估算剩余时间
    pub fn eta(&self) -> Option<Duration> {
        if self.current == 0 {
            return None;
        }

        let elapsed = self.start_time.elapsed();
        let rate = self.current as f64 / elapsed.as_secs_f64();

        if rate > 0.0 {
            let remaining_items = self.total - self.current;
            let remaining_secs = remaining_items as f64 / rate;
            Some(Duration::from_secs_f64(remaining_secs))
        } else {
            None
        }
    }

    /// 格式化时间
    fn format_duration(duration: Duration) -> String {
        let secs = duration.as_secs();
        if secs < 60 {
            format!("{}s", secs)
        } else if secs < 3600 {
            format!("{}m{}s", secs / 60, secs % 60)
        } else {
            format!("{}h{}m", secs / 3600, (secs % 3600) / 60)
        }
    }
}

/// 为 ProgressBar 实现 Display trait
impl Display for ProgressBar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let percentage = self.percentage();
        let filled = ((percentage / 100.0) * self.width as f64) as usize;
        let empty = self.width - filled;

        // 消息
        if !self.message.is_empty() {
            write!(f, "{}: ", self.message)?;
        }

        // 进度条
        write!(f, "[")?;
        write!(f, "{}", "█".repeat(filled))?;
        write!(f, "{}", "░".repeat(empty))?;
        write!(f, "]")?;

        // 百分比
        if self.show_percentage {
            write!(f, " {:6.2}%", percentage)?;
        }

        // 数字
        if self.show_numbers {
            write!(f, " ({}/{})", self.current, self.total)?;
        }

        // 预计剩余时间
        if self.show_eta {
            if let Some(eta) = self.eta() {
                write!(f, " ETA: {}", Self::format_duration(eta))?;
            }
        }

        Ok(())
    }
}

/// 系统监控数据结构体
///
/// 用于显示系统性能监控信息。
#[derive(Debug, Clone)]
pub struct SystemMonitor {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_in: u64,
    pub network_out: u64,
    pub uptime: Duration,
    pub processes: u32,
    pub load_average: (f64, f64, f64),
}

impl SystemMonitor {
    /// 创建示例监控数据
    pub fn sample() -> Self {
        Self {
            cpu_usage: 45.2,
            memory_usage: 68.7,
            disk_usage: 23.1,
            network_in: 1024 * 1024 * 150, // 150 MB
            network_out: 1024 * 1024 * 89, // 89 MB
            uptime: Duration::from_secs(3600 * 24 * 7 + 3600 * 2 + 60 * 30), // 7天2小时30分钟
            processes: 342,
            load_average: (1.23, 1.45, 1.67),
        }
    }

    /// 格式化字节大小
    fn format_bytes(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", bytes, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }

    /// 格式化运行时间
    fn format_uptime(duration: Duration) -> String {
        let total_secs = duration.as_secs();
        let days = total_secs / (24 * 3600);
        let hours = (total_secs % (24 * 3600)) / 3600;
        let minutes = (total_secs % 3600) / 60;

        if days > 0 {
            format!("{}天 {}小时 {}分钟", days, hours, minutes)
        } else if hours > 0 {
            format!("{}小时 {}分钟", hours, minutes)
        } else {
            format!("{}分钟", minutes)
        }
    }

    /// 创建使用率条形图
    fn usage_bar(percentage: f64, width: usize) -> String {
        let filled = ((percentage / 100.0) * width as f64) as usize;
        let empty = width - filled;

        let color = if percentage > 80.0 {
            "\x1b[31m" // 红色
        } else if percentage > 60.0 {
            "\x1b[33m" // 黄色
        } else {
            "\x1b[32m" // 绿色
        };

        format!(
            "{}{}{}{}\x1b[0m",
            color,
            "█".repeat(filled),
            "░".repeat(empty),
            format!(" {:5.1}%", percentage)
        )
    }
}

/// 为 SystemMonitor 实现 Display trait
impl Display for SystemMonitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:═^60}", " 系统监控 ")?;

        // CPU 使用率
        writeln!(
            f,
            "CPU 使用率:    [{}]",
            Self::usage_bar(self.cpu_usage, 20)
        )?;

        // 内存使用率
        writeln!(
            f,
            "内存使用率:    [{}]",
            Self::usage_bar(self.memory_usage, 20)
        )?;

        // 磁盘使用率
        writeln!(
            f,
            "磁盘使用率:    [{}]",
            Self::usage_bar(self.disk_usage, 20)
        )?;

        writeln!(f, "{:─^60}", "")?;

        // 网络流量
        writeln!(
            f,
            "网络流量:      ↓ {}  ↑ {}",
            Self::format_bytes(self.network_in),
            Self::format_bytes(self.network_out)
        )?;

        // 系统信息
        writeln!(f, "运行时间:      {}", Self::format_uptime(self.uptime))?;
        writeln!(f, "进程数量:      {}", self.processes)?;
        writeln!(
            f,
            "负载平均:      {:.2} {:.2} {:.2}",
            self.load_average.0, self.load_average.1, self.load_average.2
        )?;

        writeln!(f, "{:═^60}", "")?;

        Ok(())
    }
}

/// 配置文件生成器
///
/// 用于生成各种格式的配置文件。
#[derive(Debug, Clone)]
pub struct ConfigGenerator {
    pub data: BTreeMap<String, ConfigValue>,
}

/// 配置值枚举
#[derive(Debug, Clone)]
pub enum ConfigValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<ConfigValue>),
    Object(BTreeMap<String, ConfigValue>),
}

impl ConfigGenerator {
    /// 创建新的配置生成器
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    /// 添加配置项
    pub fn add(&mut self, key: &str, value: ConfigValue) {
        self.data.insert(key.to_string(), value);
    }

    /// 生成 TOML 格式
    pub fn to_toml(&self) -> String {
        let mut output = String::new();

        for (key, value) in &self.data {
            writeln!(output, "{} = {}", key, self.format_toml_value(value)).unwrap();
        }

        output
    }

    /// 生成 JSON 格式
    pub fn to_json(&self) -> String {
        let mut output = String::new();
        writeln!(output, "{{").unwrap();

        let mut first = true;
        for (key, value) in &self.data {
            if !first {
                writeln!(output, ",").unwrap();
            }
            write!(output, "  \"{}\": {}", key, self.format_json_value(value)).unwrap();
            first = false;
        }

        writeln!(output).unwrap();
        writeln!(output, "}}").unwrap();

        output
    }

    /// 格式化 TOML 值
    fn format_toml_value(&self, value: &ConfigValue) -> String {
        match value {
            ConfigValue::String(s) => format!("\"{}\"", s),
            ConfigValue::Integer(i) => i.to_string(),
            ConfigValue::Float(f) => f.to_string(),
            ConfigValue::Boolean(b) => b.to_string(),
            ConfigValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| self.format_toml_value(v)).collect();
                format!("[{}]", items.join(", "))
            }
            ConfigValue::Object(_) => "[object]".to_string(), // 简化处理
        }
    }

    /// 格式化 JSON 值
    fn format_json_value(&self, value: &ConfigValue) -> String {
        match value {
            ConfigValue::String(s) => format!("\"{}\"", s),
            ConfigValue::Integer(i) => i.to_string(),
            ConfigValue::Float(f) => f.to_string(),
            ConfigValue::Boolean(b) => b.to_string(),
            ConfigValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| self.format_json_value(v)).collect();
                format!("[{}]", items.join(", "))
            }
            ConfigValue::Object(obj) => {
                let mut items = Vec::new();
                for (k, v) in obj {
                    items.push(format!("\"{}\": {}", k, self.format_json_value(v)));
                }
                format!("{{{}}}", items.join(", "))
            }
        }
    }
}

/// 演示日志系统应用
///
/// 展示如何在实际项目中使用格式化输出构建日志系统。
///
/// # Examples
///
/// ```rust
/// # use formatted_output::practical_examples::demonstrate_logging_system;
/// demonstrate_logging_system();
/// ```
pub fn demonstrate_logging_system() {
    println!("\n=== 日志系统应用演示 ===");

    // 1. 基本日志记录
    println!("\n--- 基本日志记录 ---");

    let logs = vec![
        LogRecord::new(LogLevel::Info, "main", "应用程序启动").with_location("main.rs", 15),
        LogRecord::new(LogLevel::Debug, "auth", "用户认证开始")
            .with_location("auth.rs", 42)
            .with_metadata("user_id", "12345")
            .with_metadata("ip", "192.168.1.100"),
        LogRecord::new(LogLevel::Warn, "database", "连接池接近满载")
            .with_location("db.rs", 128)
            .with_metadata("pool_size", "95")
            .with_metadata("max_size", "100"),
        LogRecord::new(LogLevel::Error, "payment", "支付处理失败")
            .with_location("payment.rs", 67)
            .with_metadata("order_id", "ORD-001")
            .with_metadata("amount", "99.99")
            .with_metadata("error_code", "PAYMENT_DECLINED"),
        LogRecord::new(LogLevel::Fatal, "system", "系统内存不足")
            .with_location("system.rs", 203)
            .with_metadata("available_memory", "128MB")
            .with_metadata("required_memory", "512MB"),
    ];

    for log in &logs {
        println!("{}", log);
    }

    // 2. 不同格式化器
    println!("\n--- 不同日志格式 ---");

    let sample_log = LogRecord::new(LogLevel::Info, "web_server", "HTTP 请求处理完成")
        .with_location("server.rs", 89)
        .with_metadata("method", "GET")
        .with_metadata("path", "/api/users")
        .with_metadata("status", "200")
        .with_metadata("duration", "45ms");

    let formatters = vec![
        ("标准格式", LogFormatter::new()),
        ("紧凑格式", LogFormatter::compact()),
        ("详细格式", LogFormatter::verbose()),
    ];

    for (name, formatter) in formatters {
        println!("{}: {}", name, formatter.format(&sample_log));
    }

    // 3. 结构化日志
    println!("\n--- 结构化日志演示 ---");
    demonstrate_structured_logging();
}

/// 演示结构化日志
fn demonstrate_structured_logging() {
    let events = vec![
        (
            "user_login",
            vec![
                ("user_id", "12345"),
                ("username", "john_doe"),
                ("ip_address", "192.168.1.100"),
                ("user_agent", "Mozilla/5.0"),
                ("success", "true"),
            ],
        ),
        (
            "api_request",
            vec![
                ("method", "POST"),
                ("endpoint", "/api/orders"),
                ("user_id", "12345"),
                ("request_size", "1024"),
                ("response_time", "125ms"),
                ("status_code", "201"),
            ],
        ),
        (
            "database_query",
            vec![
                ("query_type", "SELECT"),
                ("table", "users"),
                ("execution_time", "23ms"),
                ("rows_returned", "1"),
                ("cache_hit", "false"),
            ],
        ),
    ];

    println!("结构化事件日志:");
    println!("{:═^80}", " 事件追踪 ");

    for (event_type, fields) in events {
        let mut log = LogRecord::new(
            LogLevel::Info,
            "event_tracker",
            &format!("事件: {}", event_type),
        );

        for (key, value) in fields {
            log = log.with_metadata(key, value);
        }

        println!("{}", log);
        println!("{:-<80}", "");
    }
}

/// 演示数据展示应用
///
/// 展示如何使用格式化输出进行数据可视化和报表生成。
///
/// # Examples
///
/// ```rust
/// # use formatted_output::practical_examples::demonstrate_data_presentation;
/// demonstrate_data_presentation();
/// ```
pub fn demonstrate_data_presentation() {
    println!("\n=== 数据展示应用演示 ===");

    // 1. 销售数据表格
    println!("\n--- 销售数据表格 ---");

    let mut sales_table = DataTable::new(vec![
        "产品名称".to_string(),
        "销售数量".to_string(),
        "单价".to_string(),
        "总金额".to_string(),
        "增长率".to_string(),
    ])
    .with_title("2024年第一季度销售报告")
    .with_index()
    .with_alignment(vec![
        Alignment::Left,
        Alignment::Right,
        Alignment::Right,
        Alignment::Right,
        Alignment::Center,
    ]);

    let sales_data = vec![
        ("笔记本电脑", 1250, 5999.00, "+15.2%"),
        ("智能手机", 3420, 2999.00, "+8.7%"),
        ("平板电脑", 890, 1999.00, "-3.1%"),
        ("智能手表", 2100, 1299.00, "+22.5%"),
        ("无线耳机", 4560, 299.00, "+45.8%"),
    ];

    for (product, quantity, price, growth) in sales_data {
        let total = quantity as f64 * price;
        sales_table.add_row(vec![
            product.to_string(),
            quantity.to_string(),
            format!("¥{:.2}", price),
            format!("¥{:.2}", total),
            growth.to_string(),
        ]);
    }

    println!("{}", sales_table);

    // 2. 系统监控面板
    println!("\n--- 系统监控面板 ---");
    let monitor = SystemMonitor::sample();
    println!("{}", monitor);

    // 3. 进度条演示
    println!("\n--- 进度条演示 ---");
    demonstrate_progress_bars();

    // 4. 数据统计图表
    println!("\n--- 数据统计图表 ---");
    demonstrate_data_charts();
}

/// 演示进度条
fn demonstrate_progress_bars() {
    let tasks = vec![
        ("文件下载", 85, 100),
        ("数据处理", 342, 500),
        ("备份创建", 7, 10),
        ("索引构建", 1250, 2000),
    ];

    println!("任务进度监控:");

    for (task_name, current, total) in tasks {
        let mut progress = ProgressBar::new(total)
            .with_width(30)
            .with_message(task_name);
        progress.set_progress(current);

        println!("{}", progress);
    }
}

/// 演示数据图表
fn demonstrate_data_charts() {
    // 简单的柱状图
    let data = vec![
        ("一月", 120),
        ("二月", 98),
        ("三月", 145),
        ("四月", 167),
        ("五月", 134),
        ("六月", 189),
    ];

    println!("月度销售趋势图:");
    println!("{:═^50}", " 销售数据 ");

    let max_value = data.iter().map(|(_, v)| *v).max().unwrap_or(1);

    for (month, value) in data {
        let bar_length = (value * 30 / max_value).max(1);
        let bar = "█".repeat(bar_length as usize);

        println!("{:4} │{:<30} {:3}", month, bar, value);
    }

    println!("{:─^50}", "");
}

/// 演示调试信息应用
///
/// 展示如何使用格式化输出进行程序调试和错误诊断。
///
/// # Examples
///
/// ```rust
/// # use formatted_output::practical_examples::demonstrate_debugging_output;
/// demonstrate_debugging_output();
/// ```
pub fn demonstrate_debugging_output() {
    println!("\n=== 调试信息应用演示 ===");

    // 1. 错误诊断
    println!("\n--- 错误诊断信息 ---");
    demonstrate_error_diagnostics();

    // 2. 性能分析
    println!("\n--- 性能分析信息 ---");
    demonstrate_performance_analysis();

    // 3. 状态监控
    println!("\n--- 状态监控信息 ---");
    demonstrate_state_monitoring();

    // 4. 内存使用分析
    println!("\n--- 内存使用分析 ---");
    demonstrate_memory_analysis();
}

/// 演示错误诊断
fn demonstrate_error_diagnostics() {
    // 模拟错误场景
    let errors = vec![
        (
            "数据库连接失败",
            "ConnectionError",
            "无法连接到数据库服务器",
            vec![
                ("host", "localhost:5432"),
                ("database", "myapp"),
                ("timeout", "30s"),
                ("retry_count", "3"),
            ],
        ),
        (
            "JSON 解析错误",
            "ParseError",
            "无效的 JSON 格式",
            vec![
                ("line", "15"),
                ("column", "23"),
                ("expected", "}"),
                ("found", "EOF"),
            ],
        ),
        (
            "权限验证失败",
            "AuthenticationError",
            "用户凭据无效",
            vec![
                ("user_id", "12345"),
                ("token_expired", "true"),
                ("last_login", "2024-01-10 14:30:00"),
            ],
        ),
    ];

    println!("错误诊断报告:");
    println!("{:═^80}", " 系统错误分析 ");

    for (title, error_type, message, details) in errors {
        println!("\n🚨 {}", title);
        println!("   类型: {}", error_type);
        println!("   描述: {}", message);
        println!("   详细信息:");

        for (key, value) in details {
            println!("     • {}: {}", key, value);
        }

        println!("{:-<80}", "");
    }
}

/// 演示性能分析
fn demonstrate_performance_analysis() {
    let performance_data = vec![
        ("数据库查询", 125.5, 50.0, "ms"),
        ("API 响应时间", 89.2, 100.0, "ms"),
        ("内存使用", 68.7, 80.0, "%"),
        ("CPU 使用率", 45.3, 70.0, "%"),
        ("磁盘 I/O", 234.1, 500.0, "MB/s"),
        ("网络延迟", 12.8, 20.0, "ms"),
    ];

    println!("性能分析报告:");
    println!("{:═^70}", " 系统性能指标 ");

    for (metric, current, threshold, unit) in performance_data {
        let status = if current > threshold {
            "⚠️ 警告"
        } else if current > threshold * 0.8 {
            "⚡ 注意"
        } else {
            "✅ 正常"
        };

        let percentage = (current / threshold * 100.0f64).min(100.0f64);
        let bar_length = (percentage / 100.0 * 20.0) as usize;
        let bar = "█".repeat(bar_length) + &"░".repeat(20 - bar_length);

        println!(
            "{:<12} │{} {:6.1}{} │ {} ({:5.1}%)",
            metric, bar, current, unit, status, percentage
        );
    }

    println!("{:═^70}", "");
}

/// 演示状态监控
fn demonstrate_state_monitoring() {
    let services = vec![
        ("Web 服务器", "运行中", "8080", "正常"),
        ("数据库", "运行中", "5432", "正常"),
        ("Redis 缓存", "运行中", "6379", "正常"),
        ("消息队列", "停止", "5672", "错误"),
        ("文件服务", "运行中", "9000", "警告"),
    ];

    println!("服务状态监控:");
    println!("{:═^60}", " 服务状态面板 ");

    for (service, status, port, health) in services {
        let status_icon = match status {
            "运行中" => "🟢",
            "停止" => "🔴",
            _ => "🟡",
        };

        let health_icon = match health {
            "正常" => "✅",
            "警告" => "⚠️",
            "错误" => "❌",
            _ => "❓",
        };

        println!(
            "{} {:<12} │ 端口: {:>5} │ {} {}",
            status_icon, service, port, health_icon, health
        );
    }

    println!("{:═^60}", "");
}

/// 演示内存使用分析
fn demonstrate_memory_analysis() {
    let memory_regions = vec![
        ("堆内存", 512.5, 1024.0, "MB"),
        ("栈内存", 8.2, 16.0, "MB"),
        ("代码段", 45.8, 100.0, "MB"),
        ("数据段", 23.1, 50.0, "MB"),
        ("共享库", 156.7, 200.0, "MB"),
    ];

    println!("内存使用分析:");
    println!("{:═^65}", " 内存分布图 ");

    let mut total_used = 0.0;
    let mut total_available = 0.0;

    for (region, used, available, unit) in &memory_regions {
        total_used += used;
        total_available += available;

        let usage_percent = (used / available) * 100.0;
        let bar_length = (usage_percent / 100.0 * 25.0) as usize;
        let bar = "█".repeat(bar_length) + &"░".repeat(25 - bar_length);

        println!(
            "{:<8} │{} {:6.1}/{:6.1} {} ({:5.1}%)",
            region, bar, used, available, unit, usage_percent
        );
    }

    println!("{:─^65}", "");
    println!(
        "总计:     {:6.1}/{:6.1} MB ({:5.1}%)",
        total_used,
        total_available,
        (total_used / total_available) * 100.0
    );
    println!("{:═^65}", "");
}

/// 演示配置文件生成应用
///
/// 展示如何使用格式化输出生成各种配置文件。
///
/// # Examples
///
/// ```rust
/// # use formatted_output::practical_examples::demonstrate_config_generation;
/// demonstrate_config_generation();
/// ```
pub fn demonstrate_config_generation() {
    println!("\n=== 配置文件生成应用演示 ===");

    let mut config = ConfigGenerator::new();

    // 添加配置项
    config.add("app_name", ConfigValue::String("MyApp".to_string()));
    config.add("version", ConfigValue::String("1.0.0".to_string()));
    config.add("port", ConfigValue::Integer(8080));
    config.add("debug", ConfigValue::Boolean(true));
    config.add("timeout", ConfigValue::Float(30.5));
    config.add(
        "hosts",
        ConfigValue::Array(vec![
            ConfigValue::String("localhost".to_string()),
            ConfigValue::String("127.0.0.1".to_string()),
        ]),
    );

    // 数据库配置对象
    let mut db_config = BTreeMap::new();
    db_config.insert(
        "host".to_string(),
        ConfigValue::String("localhost".to_string()),
    );
    db_config.insert("port".to_string(), ConfigValue::Integer(5432));
    db_config.insert(
        "database".to_string(),
        ConfigValue::String("myapp".to_string()),
    );
    config.add("database", ConfigValue::Object(db_config));

    // 生成不同格式的配置文件
    println!("\n--- TOML 格式配置 ---");
    println!("{}", config.to_toml());

    println!("\n--- JSON 格式配置 ---");
    println!("{}", config.to_json());

    // 环境变量格式
    println!("\n--- 环境变量格式 ---");
    demonstrate_env_config(&config);
}

/// 演示环境变量配置格式
fn demonstrate_env_config(config: &ConfigGenerator) {
    println!("# 环境变量配置");

    for (key, value) in &config.data {
        let env_key = key.to_uppercase().replace('.', "_");
        let env_value = match value {
            ConfigValue::String(s) => s.clone(),
            ConfigValue::Integer(i) => i.to_string(),
            ConfigValue::Float(f) => f.to_string(),
            ConfigValue::Boolean(b) => b.to_string(),
            ConfigValue::Array(_) => "[array]".to_string(),
            ConfigValue::Object(_) => "[object]".to_string(),
        };

        println!("export {}={}", env_key, env_value);
    }
}

/// 演示格式化输出的最佳实践
///
/// 提供在实际项目中使用格式化输出的最佳实践指南。
///
/// # 最佳实践
///
/// 1. **一致性** - 保持格式化风格的一致性
/// 2. **可读性** - 优先考虑输出的可读性
/// 3. **性能** - 在高频输出场景中注意性能
/// 4. **国际化** - 考虑多语言支持
///
/// # Examples
///
/// ```rust
/// # use formatted_output::practical_examples::demonstrate_best_practices;
/// demonstrate_best_practices();
/// ```
pub fn demonstrate_best_practices() {
    println!("\n=== 格式化输出最佳实践 ===");

    println!("\n最佳实践指南:");
    println!("  ✓ 保持格式化风格的一致性");
    println!("  ✓ 优先考虑输出内容的可读性");
    println!("  ✓ 在高频输出场景中注意性能优化");
    println!("  ✓ 使用结构化的日志格式");
    println!("  ✓ 为不同的输出目标选择合适的格式");
    println!("  ✓ 实现可配置的格式化选项");
    println!("  ✓ 考虑国际化和本地化需求");
    println!("  ✓ 提供调试和生产环境的不同输出级别");

    // 实际应用示例总结
    println!("\n--- 应用场景总结 ---");

    let use_cases = vec![
        ("日志系统", "结构化日志、多级别输出、性能监控"),
        ("数据展示", "表格格式、图表绘制、报表生成"),
        ("调试信息", "错误诊断、状态监控、性能分析"),
        ("用户界面", "进度条、菜单、交互提示"),
        ("配置管理", "多格式输出、环境变量、模板生成"),
        ("API 响应", "JSON/XML 格式、错误消息、状态码"),
    ];

    for (scenario, description) in use_cases {
        println!("  📋 {:<12}: {}", scenario, description);
    }

    println!("\n格式化输出是 Rust 程序与用户交互的重要桥梁！");
    println!("通过合理使用格式化功能，可以大大提升程序的可用性和可维护性。");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_display() {
        assert_eq!(format!("{}", LogLevel::Info), "INFO");
        assert_eq!(format!("{}", LogLevel::Error), "ERROR");
    }

    #[test]
    fn test_log_record_creation() {
        let log = LogRecord::new(LogLevel::Info, "test", "测试消息");
        assert_eq!(log.level, LogLevel::Info);
        assert_eq!(log.target, "test");
        assert_eq!(log.message, "测试消息");
    }

    #[test]
    fn test_data_table_creation() {
        let mut table = DataTable::new(vec!["列1".to_string(), "列2".to_string()]);
        table.add_row(vec!["值1".to_string(), "值2".to_string()]);

        assert_eq!(table.headers.len(), 2);
        assert_eq!(table.rows.len(), 1);
    }

    #[test]
    fn test_progress_bar() {
        let mut progress = ProgressBar::new(100);
        progress.set_progress(50);

        assert_eq!(progress.current, 50);
        assert_eq!(progress.total, 100);
        assert_eq!(progress.percentage(), 50.0);
    }

    #[test]
    fn test_config_generator() {
        let mut config = ConfigGenerator::new();
        config.add("test_key", ConfigValue::String("test_value".to_string()));

        assert_eq!(config.data.len(), 1);
        assert!(config.data.contains_key("test_key"));
    }

    #[test]
    fn test_system_monitor_format_bytes() {
        assert_eq!(SystemMonitor::format_bytes(1024), "1.0 KB");
        assert_eq!(SystemMonitor::format_bytes(1024 * 1024), "1.0 MB");
        assert_eq!(SystemMonitor::format_bytes(512), "512 B");
    }
}
