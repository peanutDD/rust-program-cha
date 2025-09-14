//! Rust 格式化参数详解
//!
//! 本模块深入探讨 Rust 中的格式化参数系统，包括 format_args! 宏和 write! 系列宏的使用。
//! 这些工具提供了零成本抽象的格式化能力，是 Rust 格式化系统的核心。

use std::fmt::Write;
use std::io::Write as IoWrite;

/// 自定义缓冲区写入器
/// 演示如何实现自定义的写入目标
pub struct BufferWriter {
    buffer: String,
}

impl BufferWriter {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.buffer
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

impl Write for BufferWriter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buffer.push_str(s);
        Ok(())
    }
}

/// 演示 format_args! 宏的基本用法
pub fn demonstrate_format_args_basics() {
    println!("\n=== format_args! 基础用法 ===");

    // format_args! 的基本概念
    println!("\n--- 基本概念 ---");
    println!("format_args! 宏创建一个 Arguments 结构体，用于延迟格式化");
    println!("这提供了零成本抽象，避免了不必要的字符串分配");

    // 基本使用示例
    println!("\n--- 基本用法 ---");

    // 直接格式化
    let name = "Alice";
    let age = 30;
    println!("直接格式化: Hello, {}! You are {} years old.", name, age);

    // 使用 format! 宏
    let formatted = format!("Hello, {}! You are {} years old.", name, age);
    println!("format! 结果: {}", formatted);

    // 演示不同的格式化方式
    println!("\n--- 不同格式化方式对比 ---");
    let value = 42;

    // println! 直接输出
    print!("println!: ");
    println!("Value is {}", value);

    // format! 创建字符串
    let s1 = format!("Value is {}", value);
    println!("format!: {}", s1);

    // 使用自定义写入器
    let mut buffer = BufferWriter::new();
    write!(buffer, "Value is {}", value).unwrap();
    println!("自定义写入器: {}", buffer.content());
}

/// 演示 write! 系列宏的使用
pub fn demonstrate_write_macros() {
    println!("\n=== write! 系列宏演示 ===");

    let mut buffer = BufferWriter::new();

    // write! 宏 - 不添加换行符
    println!("\n--- write! 宏 ---");
    write!(buffer, "Hello, ").unwrap();
    write!(buffer, "World!").unwrap();
    println!("write! 结果: '{}'", buffer.content());

    // writeln! 宏 - 添加换行符
    println!("\n--- writeln! 宏 ---");
    buffer.clear();
    writeln!(buffer, "第一行").unwrap();
    writeln!(buffer, "第二行").unwrap();
    println!("writeln! 结果:\n{}", buffer.content());

    // 格式化参数演示
    println!("\n--- 格式化参数 ---");
    buffer.clear();
    let name = "Rust";
    let version = "1.70";
    writeln!(buffer, "欢迎使用 {} 版本 {}!", name, version).unwrap();
    println!("格式化参数: {}", buffer.content());

    // 数值格式化
    println!("\n--- 数值格式化 ---");
    buffer.clear();
    let pi = std::f64::consts::PI;
    writeln!(buffer, "π = {:.2}", pi).unwrap();
    writeln!(buffer, "π = {:.6}", pi).unwrap();
    writeln!(buffer, "π = {:e}", pi).unwrap();
    println!("数值格式化:\n{}", buffer.content());
}

/// 演示高级 format_args 用法
pub fn demonstrate_advanced_format_args() {
    println!("\n=== 高级 format_args 用法 ===");

    let mut buffer = BufferWriter::new();

    // 位置参数
    println!("\n--- 位置参数 ---");
    buffer.clear();
    writeln!(buffer, "{1} {0} {2}", "world", "hello", "!").unwrap();
    println!("位置参数: {}", buffer.content());

    // 命名参数
    println!("\n--- 命名参数 ---");
    buffer.clear();
    writeln!(
        buffer,
        "{greeting} {name}!",
        greeting = "Hello",
        name = "Alice"
    )
    .unwrap();
    println!("命名参数: {}", buffer.content());

    // 混合参数
    println!("\n--- 混合参数 ---");
    buffer.clear();
    writeln!(buffer, "{0} {name} {1}", "Hello", "!", name = "World").unwrap();
    println!("混合参数: {}", buffer.content());

    // 动态宽度和精度
    println!("\n--- 动态宽度和精度 ---");
    buffer.clear();
    let width = 10;
    let precision = 2;
    let value = 3.14159;
    writeln!(
        buffer,
        "值: {:width$.precision$}",
        value,
        width = width,
        precision = precision
    )
    .unwrap();
    println!("动态格式化: {}", buffer.content());

    // 条件格式化示例
    println!("\n--- 条件格式化 ---");
    buffer.clear();
    let temperature = 25.5;
    let status = if temperature > 30.0 {
        "热"
    } else if temperature < 10.0 {
        "冷"
    } else {
        "适中"
    };
    writeln!(buffer, "温度: {:.1}°C ({})", temperature, status).unwrap();
    println!("条件格式化: {}", buffer.content());
}

/// 演示最佳实践
pub fn demonstrate_best_practices() {
    println!("\n=== 最佳实践 ===");

    let mut buffer = BufferWriter::new();

    // 1. 避免不必要的字符串分配
    println!("\n--- 性能优化 ---");
    println!("✅ 好的做法: 直接使用 write! 系列宏");
    println!("❌ 避免: 先用 format! 再输出");

    // 2. 错误处理
    println!("\n--- 错误处理 ---");
    buffer.clear();
    match write!(buffer, "安全的写入操作") {
        Ok(_) => println!("写入成功: {}", buffer.content()),
        Err(e) => println!("写入失败: {}", e),
    }

    // 3. 类型安全
    println!("\n--- 类型安全 ---");
    buffer.clear();
    let number: i32 = 42;
    let float: f64 = 3.14;
    writeln!(buffer, "整数: {}, 浮点数: {:.2}", number, float).unwrap();
    println!("类型安全: {}", buffer.content());

    // 4. 可读性
    println!("\n--- 代码可读性 ---");
    buffer.clear();
    let user_name = "Alice";
    let user_age = 30;
    let user_city = "北京";

    // 使用命名参数提高可读性
    writeln!(
        buffer,
        "用户信息: {name} ({age}岁) 来自 {city}",
        name = user_name,
        age = user_age,
        city = user_city
    )
    .unwrap();
    println!("可读性优化: {}", buffer.content());

    // 5. 国际化考虑
    println!("\n--- 国际化支持 ---");
    buffer.clear();
    let items = vec!["苹果", "香蕉", "橙子"];
    writeln!(buffer, "水果列表:").unwrap();
    for (i, item) in items.iter().enumerate() {
        writeln!(buffer, "  {}. {}", i + 1, item).unwrap();
    }
    println!("国际化示例:\n{}", buffer.content());
}

/// 性能对比演示
fn demonstrate_performance_comparison() {
    println!("\n--- 性能对比 ---");

    let data = vec![("Alice", 25), ("Bob", 30), ("Charlie", 35)];

    // 方法1: 使用 format! (会分配字符串)
    println!("方法1 (format!): 会产生临时字符串分配");
    for (name, age) in &data {
        let _formatted = format!("Name: {}, Age: {}", name, age);
        // 在实际应用中，这里可能会存储或传递这个字符串
    }

    // 方法2: 直接使用 write! (零分配)
    println!("方法2 (write!): 零成本抽象，无额外分配");
    let mut buffer = BufferWriter::new();
    for (name, age) in &data {
        write!(buffer, "Name: {}, Age: {}\n", name, age).unwrap();
    }
    println!("结果:\n{}", buffer.content());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_writer() {
        let mut buffer = BufferWriter::new();
        write!(buffer, "Hello, {}", "World").unwrap();
        assert_eq!(buffer.content(), "Hello, World");
    }

    #[test]
    fn test_format_args_basic() {
        let mut buffer = BufferWriter::new();
        writeln!(buffer, "Test: {}", 42).unwrap();
        assert!(buffer.content().contains("Test: 42"));
    }

    #[test]
    fn test_named_arguments() {
        let mut buffer = BufferWriter::new();
        write!(buffer, "{greeting} {name}", greeting = "Hi", name = "Rust").unwrap();
        assert_eq!(buffer.content(), "Hi Rust");
    }

    #[test]
    fn test_precision_formatting() {
        let mut buffer = BufferWriter::new();
        write!(buffer, "{:.2}", 3.14159).unwrap();
        assert_eq!(buffer.content(), "3.14");
    }
}
