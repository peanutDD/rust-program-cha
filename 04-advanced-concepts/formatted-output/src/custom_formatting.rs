//! # Rust 自定义格式化详解
//!
//! 本模块深入介绍如何为自定义类型实现格式化特征，包括 Display、Debug、
//! 以及其他格式化 trait 的实现，让自定义类型能够无缝集成到 Rust 的
//! 格式化系统中。
//!
//! ## 格式化 Trait 体系
//!
//! Rust 的格式化系统基于以下核心 trait：
//!
//! - `Display` - 用户友好的格式化输出（`{}`）
//! - `Debug` - 调试信息输出（`{:?}`）
//! - `Binary` - 二进制格式输出（`{:b}`）
//! - `Octal` - 八进制格式输出（`{:o}`）
//! - `LowerHex` - 小写十六进制输出（`{:x}`）
//! - `UpperHex` - 大写十六进制输出（`{:X}`）
//! - `LowerExp` - 小写科学计数法（`{:e}`）
//! - `UpperExp` - 大写科学计数法（`{:E}`）
//!
//! ## 设计原则
//!
//! 1. **用户友好性** - Display 应该提供清晰、易读的输出
//! 2. **调试完整性** - Debug 应该包含所有必要的调试信息
//! 3. **一致性** - 格式化输出应该与类型的语义保持一致
//! 4. **性能考虑** - 避免不必要的内存分配和计算

use std::collections::HashMap;
use std::fmt::{self, Binary, Debug, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex};

/// 用户信息结构体
///
/// 演示如何为复杂结构体实现多种格式化 trait。
///
/// # Examples
///
/// ```rust
/// # use formatted_output::custom_formatting::User;
/// let user = User::new("张三", 25, "zhangsan@example.com");
/// println!("{}", user);     // Display 格式
/// println!("{:?}", user);   // Debug 格式
/// ```
#[derive(Clone)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub email: String,
    pub created_at: String,
    pub is_active: bool,
}

impl User {
    /// 创建新用户
    ///
    /// # Arguments
    ///
    /// * `name` - 用户姓名
    /// * `age` - 用户年龄
    /// * `email` - 用户邮箱
    ///
    /// # Returns
    ///
    /// 返回新创建的用户实例
    pub fn new(name: &str, age: u32, email: &str) -> Self {
        Self {
            name: name.to_string(),
            age,
            email: email.to_string(),
            created_at: "2024-01-15 10:30:00".to_string(),
            is_active: true,
        }
    }

    /// 设置用户状态
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
}

/// 为 User 实现 Display trait
///
/// Display 应该提供用户友好的、简洁的输出格式，
/// 通常用于向最终用户展示信息。
impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.is_active {
            "活跃"
        } else {
            "非活跃"
        };
        write!(
            f,
            "{} ({}岁) - {} [{}]",
            self.name, self.age, self.email, status
        )
    }
}

/// 为 User 实现 Debug trait
///
/// Debug 应该提供完整的、详细的调试信息，
/// 包含所有字段的值，便于开发者调试。
impl Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("name", &self.name)
            .field("age", &self.age)
            .field("email", &self.email)
            .field("created_at", &self.created_at)
            .field("is_active", &self.is_active)
            .finish()
    }
}

/// 复数结构体
///
/// 演示如何为数学类型实现多种格式化 trait，
/// 包括标准格式和科学计数法格式。
///
/// # Examples
///
/// ```rust
/// # use formatted_output::custom_formatting::Complex;
/// let c = Complex::new(3.0, 4.0);
/// println!("{}", c);      // Display: 3 + 4i
/// println!("{:?}", c);    // Debug: Complex { real: 3.0, imag: 4.0 }
/// println!("{:e}", c);    // Scientific: 3.000e0 + 4.000e0i
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    /// 创建新的复数
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    /// 计算复数的模长
    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    /// 计算复数的幅角
    pub fn phase(&self) -> f64 {
        self.imag.atan2(self.real)
    }
}

/// 为 Complex 实现 Display trait
///
/// 提供数学上常见的复数表示形式
impl Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.real, self.imag) {
            (r, i) if i == 0.0 => write!(f, "{}", r),
            (r, i) if r == 0.0 => {
                if i == 1.0 {
                    write!(f, "i")
                } else if i == -1.0 {
                    write!(f, "-i")
                } else {
                    write!(f, "{}i", i)
                }
            }
            (r, i) if i > 0.0 => {
                if i == 1.0 {
                    write!(f, "{} + i", r)
                } else {
                    write!(f, "{} + {}i", r, i)
                }
            }
            (r, i) => {
                if i == -1.0 {
                    write!(f, "{} - i", r)
                } else {
                    write!(f, "{} - {}i", r, -i)
                }
            }
        }
    }
}

/// 为 Complex 实现 LowerExp trait（科学计数法）
impl LowerExp for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.real, self.imag) {
            (r, i) if i == 0.0 => write!(f, "{:e}", r),
            (r, i) if r == 0.0 => write!(f, "{:e}i", i),
            (r, i) if i > 0.0 => write!(f, "{:e} + {:e}i", r, i),
            (r, i) => write!(f, "{:e} - {:e}i", r, -i),
        }
    }
}

/// 为 Complex 实现 UpperExp trait（大写科学计数法）
impl UpperExp for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.real, self.imag) {
            (r, i) if i == 0.0 => write!(f, "{:E}", r),
            (r, i) if r == 0.0 => write!(f, "{:E}i", i),
            (r, i) if i > 0.0 => write!(f, "{:E} + {:E}i", r, i),
            (r, i) => write!(f, "{:E} - {:E}i", r, -i),
        }
    }
}

/// 颜色结构体
///
/// 演示如何为颜色类型实现多种进制格式化输出。
///
/// # Examples
///
/// ```rust
/// # use formatted_output::custom_formatting::Color;
/// let red = Color::new(255, 0, 0);
/// println!("{}", red);      // Display: RGB(255, 0, 0)
/// println!("{:x}", red);    // Hex: ff0000
/// println!("{:X}", red);    // Upper Hex: FF0000
/// println!("{:b}", red);    // Binary: 111111110000000000000000
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// 创建新颜色
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// 转换为 RGB 值
    pub fn to_rgb(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    /// 从 RGB 值创建颜色
    pub fn from_rgb(rgb: u32) -> Self {
        Self {
            r: ((rgb >> 16) & 0xFF) as u8,
            g: ((rgb >> 8) & 0xFF) as u8,
            b: (rgb & 0xFF) as u8,
        }
    }

    /// 获取颜色亮度
    pub fn brightness(&self) -> f64 {
        (0.299 * self.r as f64 + 0.587 * self.g as f64 + 0.114 * self.b as f64) / 255.0
    }
}

/// 为 Color 实现 Display trait
impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RGB({}, {}, {})", self.r, self.g, self.b)
    }
}

/// 为 Color 实现 LowerHex trait
impl LowerHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rgb = self.to_rgb();
        if f.alternate() {
            write!(f, "#{:06x}", rgb)
        } else {
            write!(f, "{:06x}", rgb)
        }
    }
}

/// 为 Color 实现 UpperHex trait
impl UpperHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rgb = self.to_rgb();
        if f.alternate() {
            write!(f, "#{:06X}", rgb)
        } else {
            write!(f, "{:06X}", rgb)
        }
    }
}

/// 为 Color 实现 Binary trait
impl Binary for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rgb = self.to_rgb();
        if f.alternate() {
            write!(f, "0b{:024b}", rgb)
        } else {
            write!(f, "{:024b}", rgb)
        }
    }
}

/// 为 Color 实现 Octal trait
impl Octal for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rgb = self.to_rgb();
        if f.alternate() {
            write!(f, "0o{:08o}", rgb)
        } else {
            write!(f, "{:08o}", rgb)
        }
    }
}

/// 矩阵结构体
///
/// 演示如何为复杂数据结构实现美观的格式化输出。
///
/// # Examples
///
/// ```rust
/// # use formatted_output::custom_formatting::Matrix;
/// let matrix = Matrix::new(vec![
///     vec![1.0, 2.0, 3.0],
///     vec![4.0, 5.0, 6.0],
/// ]);
/// println!("{}", matrix);
/// ```
#[derive(Debug, Clone)]
pub struct Matrix {
    pub data: Vec<Vec<f64>>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    /// 创建新矩阵
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };
        Self { data, rows, cols }
    }

    /// 创建零矩阵
    pub fn zeros(rows: usize, cols: usize) -> Self {
        let data = vec![vec![0.0; cols]; rows];
        Self { data, rows, cols }
    }

    /// 创建单位矩阵
    pub fn identity(size: usize) -> Self {
        let mut data = vec![vec![0.0; size]; size];
        for i in 0..size {
            data[i][i] = 1.0;
        }
        Self {
            data,
            rows: size,
            cols: size,
        }
    }

    /// 获取矩阵元素
    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        self.data.get(row)?.get(col).copied()
    }

    /// 设置矩阵元素
    pub fn set(&mut self, row: usize, col: usize, value: f64) -> Result<(), &'static str> {
        if row >= self.rows || col >= self.cols {
            return Err("索引超出范围");
        }
        self.data[row][col] = value;
        Ok(())
    }
}

/// 为 Matrix 实现 Display trait
///
/// 提供美观的矩阵显示格式
impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.rows == 0 || self.cols == 0 {
            return write!(f, "[空矩阵]");
        }

        // 计算每列的最大宽度
        let mut col_widths = vec![0; self.cols];
        for row in &self.data {
            for (col_idx, &value) in row.iter().enumerate() {
                let formatted = format!("{:.3}", value);
                col_widths[col_idx] = col_widths[col_idx].max(formatted.len());
            }
        }

        writeln!(f, "Matrix {}×{}:", self.rows, self.cols)?;

        for (row_idx, row) in self.data.iter().enumerate() {
            write!(f, "│")?;
            for (col_idx, &value) in row.iter().enumerate() {
                write!(f, "{:>width$.3}", value, width = col_widths[col_idx])?;
                if col_idx < self.cols - 1 {
                    write!(f, "  ")?;
                }
            }
            writeln!(f, "│")?;
        }

        Ok(())
    }
}

/// 日志级别枚举
///
/// 演示如何为枚举类型实现格式化 trait。
///
/// # Examples
///
/// ```rust
/// # use formatted_output::custom_formatting::LogLevel;
/// let level = LogLevel::Error;
/// println!("{}", level);    // ERROR
/// println!("{:?}", level);  // Error
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    /// 获取日志级别的数值
    pub fn as_number(&self) -> u8 {
        match self {
            LogLevel::Trace => 0,
            LogLevel::Debug => 1,
            LogLevel::Info => 2,
            LogLevel::Warn => 3,
            LogLevel::Error => 4,
        }
    }

    /// 获取日志级别的颜色代码（ANSI）
    pub fn color_code(&self) -> &'static str {
        match self {
            LogLevel::Trace => "\x1b[37m", // 白色
            LogLevel::Debug => "\x1b[36m", // 青色
            LogLevel::Info => "\x1b[32m",  // 绿色
            LogLevel::Warn => "\x1b[33m",  // 黄色
            LogLevel::Error => "\x1b[31m", // 红色
        }
    }
}

/// 为 LogLevel 实现 Display trait
impl Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level_str = match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        };

        if f.alternate() {
            // 使用 {:#} 格式时显示带颜色的版本
            write!(f, "{}{level_str}\x1b[0m", self.color_code())
        } else {
            write!(f, "{level_str}")
        }
    }
}

/// 日志条目结构体
///
/// 演示复杂结构体的格式化实现，结合多个自定义类型。
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub module: String,
    pub message: String,
    pub metadata: HashMap<String, String>,
}

impl LogEntry {
    /// 创建新的日志条目
    pub fn new(level: LogLevel, module: &str, message: &str) -> Self {
        Self {
            timestamp: "2024-01-15 10:30:00.123".to_string(),
            level,
            module: module.to_string(),
            message: message.to_string(),
            metadata: HashMap::new(),
        }
    }

    /// 添加元数据
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

/// 为 LogEntry 实现 Display trait
impl Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {:5} [{}] {}",
            self.timestamp, self.level, self.module, self.message
        )?;

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

/// 演示基本的自定义格式化
///
/// 展示如何为简单类型实现 Display 和 Debug trait。
///
/// # Examples
///
/// ```rust
/// # use formatted_output::custom_formatting::demonstrate_basic_custom_formatting;
/// demonstrate_basic_custom_formatting();
/// ```
pub fn demonstrate_basic_custom_formatting() {
    println!("\n=== 基本自定义格式化演示 ===");

    // 1. User 结构体格式化
    println!("\n--- User 结构体格式化 ---");
    let user1 = User::new("张三", 25, "zhangsan@example.com");
    let mut user2 = User::new("李四", 30, "lisi@example.com");
    user2.set_active(false);

    println!("Display 格式:");
    println!("  用户1: {}", user1);
    println!("  用户2: {}", user2);

    println!("\nDebug 格式:");
    println!("  用户1: {:?}", user1);
    println!("  用户2: {:#?}", user2); // 美化 Debug 格式

    // 2. Complex 复数格式化
    println!("\n--- Complex 复数格式化 ---");
    let complex_numbers = vec![
        Complex::new(3.0, 4.0),
        Complex::new(-2.0, 1.0),
        Complex::new(0.0, -5.0),
        Complex::new(7.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(1.0, 1.0),
        Complex::new(-1.0, -1.0),
    ];

    for complex in &complex_numbers {
        println!(
            "复数: {:12} | Debug: {:?} | 模长: {:.3} | 幅角: {:.3}°",
            format!("{}", complex),
            complex,
            complex.magnitude(),
            complex.phase().to_degrees()
        );
    }

    // 3. 科学计数法格式
    println!("\n--- 复数科学计数法 ---");
    let large_complex = Complex::new(1234567.89, -9876543.21);
    let small_complex = Complex::new(0.000123, -0.000456);

    println!("大复数:");
    println!("  Display: {}", large_complex);
    println!("  科学计数法: {:e}", large_complex);
    println!("  大写科学计数法: {:E}", large_complex);

    println!("小复数:");
    println!("  Display: {}", small_complex);
    println!("  科学计数法: {:e}", small_complex);
    println!("  大写科学计数法: {:E}", small_complex);
}

/// 演示颜色格式化
///
/// 展示如何为颜色类型实现多种进制格式化。
///
/// # Examples
///
/// ```rust
/// # use formatted_output::custom_formatting::demonstrate_color_formatting;
/// demonstrate_color_formatting();
/// ```
pub fn demonstrate_color_formatting() {
    println!("\n=== 颜色格式化演示 ===");

    // 1. 基本颜色
    println!("\n--- 基本颜色格式化 ---");
    let colors = vec![
        ("红色", Color::new(255, 0, 0)),
        ("绿色", Color::new(0, 255, 0)),
        ("蓝色", Color::new(0, 0, 255)),
        ("黄色", Color::new(255, 255, 0)),
        ("紫色", Color::new(128, 0, 128)),
        ("橙色", Color::new(255, 165, 0)),
        ("黑色", Color::new(0, 0, 0)),
        ("白色", Color::new(255, 255, 255)),
    ];

    println!("{:─^80}", " 颜色格式化对比表 ");
    println!(
        "│{:^8}│{:^15}│{:^10}│{:^10}│{:^26}│{:^8}│",
        "颜色", "RGB", "十六进制", "大写十六", "二进制", "亮度"
    );
    println!("{:─^80}", "");

    for (name, color) in colors {
        println!(
            "│{:^8}│{:^15}│{:^10}│{:^10}│{:^26}│{:^8.2}│",
            name,
            format!("{}", color),
            format!("{:x}", color),
            format!("{:X}", color),
            format!("{:b}", color),
            color.brightness()
        );
    }
    println!("{:─^80}", "");

    // 2. 带前缀的格式
    println!("\n--- 带前缀的颜色格式 ---");
    let test_color = Color::new(255, 128, 64);

    println!("颜色 {} 的不同格式:", test_color);
    println!("  十六进制: {:x} | 带前缀: {:#x}", test_color, test_color);
    println!(
        "  大写十六进制: {:X} | 带前缀: {:#X}",
        test_color, test_color
    );
    println!("  二进制: {:b}", test_color);
    println!("  带前缀二进制: {:#b}", test_color);
    println!("  八进制: {:o} | 带前缀: {:#o}", test_color, test_color);

    // 3. 颜色渐变演示
    println!("\n--- 颜色渐变演示 ---");
    create_color_gradient();
}

/// 创建颜色渐变演示
fn create_color_gradient() {
    println!("红色到蓝色渐变 (10步):");

    for i in 0..=10 {
        let ratio = i as f64 / 10.0;
        let red = (255.0 * (1.0 - ratio)) as u8;
        let blue = (255.0 * ratio) as u8;
        let color = Color::new(red, 0, blue);

        println!(
            "步骤 {:2}: {} | {:#x} | 亮度: {:.2}",
            i,
            color,
            color,
            color.brightness()
        );
    }
}

/// 演示矩阵格式化
///
/// 展示如何为复杂数据结构实现美观的格式化输出。
///
/// # Examples
///
/// ```rust
/// # use formatted_output::custom_formatting::demonstrate_matrix_formatting;
/// demonstrate_matrix_formatting();
/// ```
pub fn demonstrate_matrix_formatting() {
    println!("\n=== 矩阵格式化演示 ===");

    // 1. 基本矩阵
    println!("\n--- 基本矩阵 ---");
    let matrix1 = Matrix::new(vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ]);

    println!("{}", matrix1);

    // 2. 不规则矩阵
    println!("\n--- 不同大小的矩阵 ---");
    let matrix2 = Matrix::new(vec![
        vec![1.234, -2.567, 3.890],
        vec![-4.123, 5.678, -6.789],
    ]);

    println!("{}", matrix2);

    // 3. 单位矩阵
    println!("\n--- 单位矩阵 ---");
    let identity = Matrix::identity(4);
    println!("{}", identity);

    // 4. 零矩阵
    println!("\n--- 零矩阵 ---");
    let zeros = Matrix::zeros(3, 5);
    println!("{}", zeros);

    // 5. 大数值矩阵
    println!("\n--- 大数值矩阵 ---");
    let large_matrix = Matrix::new(vec![
        vec![1234567.89, -987654.321, 555.555],
        vec![0.000123, 999999.999, -0.001],
        vec![42.0, -3.14159, 2.71828],
    ]);

    println!("{}", large_matrix);

    // 6. 空矩阵
    println!("\n--- 空矩阵 ---");
    let empty_matrix = Matrix::new(vec![]);
    println!("{}", empty_matrix);
}

/// 演示日志格式化
///
/// 展示如何为日志系统实现格式化输出。
///
/// # Examples
///
/// ```rust
/// # use formatted_output::custom_formatting::demonstrate_log_formatting;
/// demonstrate_log_formatting();
/// ```
pub fn demonstrate_log_formatting() {
    println!("\n=== 日志格式化演示 ===");

    // 1. 基本日志级别
    println!("\n--- 日志级别格式化 ---");
    let levels = vec![
        LogLevel::Trace,
        LogLevel::Debug,
        LogLevel::Info,
        LogLevel::Warn,
        LogLevel::Error,
    ];

    for level in levels {
        println!(
            "级别: {:5} | Debug: {:?} | 数值: {} | 颜色版本: {:#}",
            level,
            level,
            level.as_number(),
            level
        );
    }

    // 2. 日志条目格式化
    println!("\n--- 日志条目格式化 ---");
    let log_entries = vec![
        LogEntry::new(LogLevel::Info, "main", "应用程序启动"),
        LogEntry::new(LogLevel::Debug, "auth", "用户认证开始")
            .with_metadata("user_id", "12345")
            .with_metadata("ip", "192.168.1.100"),
        LogEntry::new(LogLevel::Warn, "database", "连接池接近满载")
            .with_metadata("pool_size", "95")
            .with_metadata("max_size", "100"),
        LogEntry::new(LogLevel::Error, "payment", "支付处理失败")
            .with_metadata("order_id", "ORD-001")
            .with_metadata("amount", "99.99")
            .with_metadata("error_code", "PAYMENT_DECLINED"),
        LogEntry::new(LogLevel::Trace, "cache", "缓存命中"),
    ];

    for entry in log_entries {
        println!("{}", entry);
    }

    // 3. 结构化日志输出
    println!("\n--- 结构化日志输出 ---");
    create_structured_log_demo();
}

/// 创建结构化日志演示
fn create_structured_log_demo() {
    let entries = vec![
        (
            LogLevel::Info,
            "server",
            "HTTP服务器启动",
            vec![("port", "8080"), ("workers", "4")],
        ),
        (
            LogLevel::Debug,
            "router",
            "路由注册",
            vec![("path", "/api/users"), ("method", "GET")],
        ),
        (
            LogLevel::Warn,
            "middleware",
            "请求超时",
            vec![("timeout", "30s"), ("path", "/api/slow")],
        ),
        (
            LogLevel::Error,
            "database",
            "查询失败",
            vec![
                ("query", "SELECT * FROM users"),
                ("error", "connection_lost"),
            ],
        ),
    ];

    println!("结构化日志格式:");
    println!("{:═^100}", " 系统日志 ");

    for (level, module, message, metadata) in entries {
        let mut log_entry = LogEntry::new(level, module, message);
        for (key, value) in metadata {
            log_entry = log_entry.with_metadata(key, value);
        }

        println!("{}", log_entry);
        println!("{:-<100}", "");
    }
}

/// 演示高级自定义格式化技巧
///
/// 展示复杂场景下的格式化实现，包括条件格式化、动态格式等。
///
/// # Examples
///
/// ```rust
/// # use formatted_output::custom_formatting::demonstrate_advanced_custom_formatting;
/// demonstrate_advanced_custom_formatting();
/// ```
pub fn demonstrate_advanced_custom_formatting() {
    println!("\n=== 高级自定义格式化技巧 ===");

    // 1. 条件格式化
    println!("\n--- 条件格式化 ---");
    demonstrate_conditional_formatting();

    // 2. 嵌套结构格式化
    println!("\n--- 嵌套结构格式化 ---");
    demonstrate_nested_formatting();

    // 3. 性能优化技巧
    println!("\n--- 性能优化技巧 ---");
    demonstrate_performance_tips();

    // 4. 错误处理
    println!("\n--- 格式化错误处理 ---");
    demonstrate_error_handling();
}

/// 演示条件格式化
fn demonstrate_conditional_formatting() {
    // 创建一个根据数值大小改变格式的类型
    #[derive(Debug)]
    struct ConditionalNumber(f64);

    impl Display for ConditionalNumber {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.0 {
                x if x > 1000.0 => write!(f, "{:.2e} (大数)", x),
                x if x > 1.0 => write!(f, "{:.2} (正常)", x),
                x if x > 0.0 => write!(f, "{:.6} (小数)", x),
                x if x == 0.0 => write!(f, "零"),
                x => write!(f, "{:.2} (负数)", x),
            }
        }
    }

    let numbers = vec![
        ConditionalNumber(1234567.89),
        ConditionalNumber(42.5),
        ConditionalNumber(0.000123),
        ConditionalNumber(0.0),
        ConditionalNumber(-25.7),
    ];

    for num in numbers {
        println!("数值: {:<20} | Debug: {:?}", format!("{}", num), num);
    }
}

/// 演示嵌套结构格式化
fn demonstrate_nested_formatting() {
    #[derive(Debug)]
    struct Department {
        name: String,
        employees: Vec<User>,
        budget: f64,
    }

    impl Display for Department {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "部门: {} (预算: ¥{:.2})", self.name, self.budget)?;
            writeln!(f, "员工列表:")?;
            for (i, employee) in self.employees.iter().enumerate() {
                writeln!(f, "  {}. {}", i + 1, employee)?;
            }
            Ok(())
        }
    }

    let department = Department {
        name: "技术部".to_string(),
        employees: vec![
            User::new("张三", 25, "zhangsan@company.com"),
            User::new("李四", 30, "lisi@company.com"),
            User::new("王五", 28, "wangwu@company.com"),
        ],
        budget: 500000.0,
    };

    println!("{}", department);
}

/// 演示性能优化技巧
fn demonstrate_performance_tips() {
    // 使用 write! 宏而不是字符串拼接
    #[derive(Debug)]
    struct OptimizedStruct {
        data: Vec<i32>,
    }

    impl Display for OptimizedStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[")?;
            for (i, item) in self.data.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", item)?;
            }
            write!(f, "]")
        }
    }

    let optimized = OptimizedStruct {
        data: vec![1, 2, 3, 4, 5, 10, 20, 30],
    };

    println!("优化的结构: {}", optimized);
    println!("Debug 格式: {:?}", optimized);
}

/// 演示格式化错误处理
fn demonstrate_error_handling() {
    #[derive(Debug)]
    struct FallibleFormatter {
        data: Option<String>,
    }

    impl Display for FallibleFormatter {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self.data {
                Some(data) => write!(f, "数据: {}", data),
                None => write!(f, "[无数据]"),
            }
        }
    }

    let valid = FallibleFormatter {
        data: Some("有效数据".to_string()),
    };

    let invalid = FallibleFormatter { data: None };

    println!("有效实例: {}", valid);
    println!("无效实例: {}", invalid);
}

/// 演示自定义格式化的最佳实践
///
/// 提供实现自定义格式化的最佳实践指南。
///
/// # 最佳实践
///
/// 1. **Display vs Debug** - Display 面向用户，Debug 面向开发者
/// 2. **错误处理** - 格式化函数应该处理所有可能的错误情况
/// 3. **性能考虑** - 避免不必要的内存分配
/// 4. **一致性** - 保持格式化输出的一致性
///
/// # Examples
///
/// ```rust
/// # use formatted_output::custom_formatting::demonstrate_best_practices;
/// demonstrate_best_practices();
/// ```
pub fn demonstrate_best_practices() {
    println!("\n=== 自定义格式化最佳实践 ===");

    println!("\n最佳实践指南:");
    println!("  ✓ Display trait 应该提供用户友好的输出");
    println!("  ✓ Debug trait 应该包含完整的调试信息");
    println!("  ✓ 使用 write! 宏而不是字符串拼接提高性能");
    println!("  ✓ 处理所有可能的边界情况和错误状态");
    println!("  ✓ 保持格式化输出的一致性和可读性");
    println!("  ✓ 考虑实现多种格式化 trait 以支持不同用途");
    println!("  ✓ 使用条件格式化提供更好的用户体验");
    println!("  ✓ 为复杂结构提供层次化的格式化输出");

    // 综合示例
    println!("\n--- 综合示例 ---");
    let user = User::new("示例用户", 25, "example@test.com");
    let color = Color::new(128, 64, 192);
    let complex = Complex::new(3.0, -4.0);
    let matrix = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

    println!("用户信息: {}", user);
    println!("颜色: {} (十六进制: {:#x})", color, color);
    println!("复数: {} (科学计数法: {:e})", complex, complex);
    println!("{}", matrix);

    println!("\n格式化系统让 Rust 的类型系统更加强大和灵活！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_display() {
        let user = User::new("测试用户", 30, "test@example.com");
        let display = format!("{}", user);
        assert!(display.contains("测试用户"));
        assert!(display.contains("30岁"));
        assert!(display.contains("test@example.com"));
    }

    #[test]
    fn test_complex_display() {
        assert_eq!(format!("{}", Complex::new(3.0, 4.0)), "3 + 4i");
        assert_eq!(format!("{}", Complex::new(3.0, -4.0)), "3 - 4i");
        assert_eq!(format!("{}", Complex::new(0.0, 4.0)), "4i");
        assert_eq!(format!("{}", Complex::new(3.0, 0.0)), "3");
        assert_eq!(format!("{}", Complex::new(0.0, 1.0)), "i");
        assert_eq!(format!("{}", Complex::new(0.0, -1.0)), "-i");
    }

    #[test]
    fn test_color_formatting() {
        let red = Color::new(255, 0, 0);
        assert_eq!(format!("{}", red), "RGB(255, 0, 0)");
        assert_eq!(format!("{:x}", red), "ff0000");
        assert_eq!(format!("{:#x}", red), "#ff0000");
        assert_eq!(format!("{:X}", red), "FF0000");
        assert_eq!(format!("{:#X}", red), "#FF0000");
    }

    #[test]
    fn test_log_level_display() {
        assert_eq!(format!("{}", LogLevel::Info), "INFO");
        assert_eq!(format!("{}", LogLevel::Error), "ERROR");
        assert_eq!(LogLevel::Error.as_number(), 4);
        assert_eq!(LogLevel::Trace.as_number(), 0);
    }

    #[test]
    fn test_matrix_creation() {
        let matrix = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        assert_eq!(matrix.rows, 2);
        assert_eq!(matrix.cols, 2);
        assert_eq!(matrix.get(0, 0), Some(1.0));
        assert_eq!(matrix.get(1, 1), Some(4.0));
        assert_eq!(matrix.get(2, 0), None);
    }

    #[test]
    fn test_color_brightness() {
        let white = Color::new(255, 255, 255);
        let black = Color::new(0, 0, 0);
        assert!((white.brightness() - 1.0).abs() < 1e-10);
        assert!((black.brightness() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_magnitude() {
        let c = Complex::new(3.0, 4.0);
        assert!((c.magnitude() - 5.0).abs() < 1e-10);
    }
}
