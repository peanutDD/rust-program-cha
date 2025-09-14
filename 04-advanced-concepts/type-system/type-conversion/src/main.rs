//! Rust 类型转换全面教程
//!
//! 本教程基于 https://course.rs/advance/into-types/converse.html
//! 全面深入地讲解 Rust 中的类型转换机制
//!
//! 涵盖内容：
//! 1. as 关键字转换
//! 2. From 和 Into trait
//! 3. TryFrom 和 TryInto trait
//! 4. transmute 和 unsafe 转换
//! 5. 实际应用场景和最佳实践

use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::mem;

fn main() {
    println!("=== Rust 类型转换全面教程 ===");

    // 1. as 关键字转换
    demonstrate_as_conversion();

    // 2. From 和 Into trait
    demonstrate_from_into();

    // 3. TryFrom 和 TryInto trait
    demonstrate_try_conversion();

    // 4. transmute 和 unsafe 转换
    demonstrate_unsafe_conversion();

    // 5. 实际应用场景
    demonstrate_real_world_examples();

    // 6. 性能对比和最佳实践
    demonstrate_performance_and_best_practices();
}

/// 1. as 关键字转换 - 最基础的类型转换方式
///
/// as 转换的特点：
/// - 编译时检查，运行时执行
/// - 主要用于数值类型之间的转换
/// - 可能会发生截断或精度丢失
/// - 不能用于复杂类型的转换
fn demonstrate_as_conversion() {
    println!("\n=== 1. as 关键字转换 ===");

    // 1.1 基础数值转换
    println!("\n1.1 基础数值转换:");
    let a: i32 = 42;
    let b: i64 = a as i64; // 小类型到大类型，安全
    let c: i16 = a as i16; // 大类型到小类型，可能截断
    println!("i32 {} -> i64 {}, i16 {}", a, b, c);

    // 1.2 浮点数转换
    println!("\n1.2 浮点数转换:");
    let f1: f32 = 3.14159;
    let f2: f64 = f1 as f64; // f32 到 f64，精度提升
    let f3: f32 = f2 as f32; // f64 到 f32，可能精度丢失
    println!("f32 {} -> f64 {} -> f32 {}", f1, f2, f3);

    // 1.3 整数和浮点数互转
    println!("\n1.3 整数和浮点数互转:");
    let int_val = 42i32;
    let float_val = int_val as f64;
    let back_to_int = float_val as i32;
    println!(
        "i32 {} -> f64 {} -> i32 {}",
        int_val, float_val, back_to_int
    );

    // 1.4 截断示例
    println!("\n1.4 截断示例:");
    let large_num: i64 = 300;
    let small_num: i8 = large_num as i8; // 300 % 256 = 44
    println!("i64 {} -> i8 {} (发生截断)", large_num, small_num);

    // 1.5 指针转换
    println!("\n1.5 指针转换:");
    let x = 42i32;
    let ptr = &x as *const i32;
    let addr = ptr as usize;
    println!("指针地址: 0x{:x}", addr);

    // 1.6 字符转换
    println!("\n1.6 字符转换:");
    let ch = 'A';
    let ascii_val = ch as u8;
    let unicode_val = ch as u32;
    println!(
        "字符 '{}' -> ASCII {} -> Unicode {}",
        ch, ascii_val, unicode_val
    );
}

/// 2. From 和 Into trait - 更安全和表达性更强的转换
///
/// From/Into 的特点：
/// - 类型安全，不会丢失数据
/// - 可以为自定义类型实现
/// - Into 是 From 的反向实现
/// - 编译器会自动实现 Into 如果实现了 From
fn demonstrate_from_into() {
    println!("\n=== 2. From 和 Into trait ===");

    // 2.1 标准库提供的 From 实现
    println!("\n2.1 标准库提供的 From 实现:");
    let s1 = String::from("hello");
    let s2: String = "world".into(); // &str -> String
    println!("String::from: {}, into(): {}", s1, s2);

    // 2.2 数值类型的 From 转换
    println!("\n2.2 数值类型的 From 转换:");
    let small: i16 = 42;
    let large: i64 = i64::from(small); // 小类型到大类型总是安全的
    println!("i16 {} -> i64 {}", small, large);

    // 2.3 自定义类型的 From 实现
    println!("\n2.3 自定义类型的 From 实现:");

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    // 从元组创建 Person
    impl From<(String, u32)> for Person {
        fn from(tuple: (String, u32)) -> Self {
            Person {
                name: tuple.0,
                age: tuple.1,
            }
        }
    }

    // 从字符串创建 Person（简化版）
    impl From<&str> for Person {
        fn from(name: &str) -> Self {
            Person {
                name: name.to_string(),
                age: 0,
            }
        }
    }

    let person1 = Person::from(("Alice".to_string(), 30));
    let person2: Person = "Bob".into(); // 使用 Into
    println!("Person1: {:?}", person1);
    println!("Person2: {:?}", person2);

    // 2.4 链式转换
    println!("\n2.4 链式转换:");
    // 注意：i32 不能直接转换为 String，标准库没有提供 From<i32> for String
    // 实际使用中应该用 format! 或 to_string()
    let result = format!("{}", 42i32);
    println!("数字转字符串: {}", result);

    // 演示实际可用的链式转换
    let char_to_string: String = 'A'.into(); // char -> String
    println!("字符转字符串: {}", char_to_string);
}

/// 3. TryFrom 和 TryInto trait - 可失败的类型转换
///
/// TryFrom/TryInto 的特点：
/// - 转换可能失败，返回 Result
/// - 用于可能丢失数据或无效的转换
/// - 提供错误处理机制
/// - 更安全的转换方式
fn demonstrate_try_conversion() {
    println!("\n=== 3. TryFrom 和 TryInto trait ===");

    // 3.1 标准库的 TryFrom 实现
    println!("\n3.1 标准库的 TryFrom 实现:");

    // 大类型到小类型的转换
    let large_num: i64 = 300;
    match i8::try_from(large_num) {
        Ok(small) => println!("转换成功: i64 {} -> i8 {}", large_num, small),
        Err(e) => println!("转换失败: {}", e),
    }

    let valid_num: i64 = 100;
    match i8::try_from(valid_num) {
        Ok(small) => println!("转换成功: i64 {} -> i8 {}", valid_num, small),
        Err(e) => println!("转换失败: {}", e),
    }

    // 3.2 使用 TryInto
    println!("\n3.2 使用 TryInto:");
    let result: Result<i8, _> = large_num.try_into();
    match result {
        Ok(val) => println!("TryInto 成功: {}", val),
        Err(e) => println!("TryInto 失败: {}", e),
    }

    // 3.3 自定义 TryFrom 实现
    println!("\n3.3 自定义 TryFrom 实现:");

    #[derive(Debug)]
    struct PositiveNumber(u32);

    #[derive(Debug)]
    enum ConversionError {
        Negative,
        TooLarge,
    }

    impl fmt::Display for ConversionError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ConversionError::Negative => write!(f, "数字不能为负数"),
                ConversionError::TooLarge => write!(f, "数字太大"),
            }
        }
    }

    impl TryFrom<i32> for PositiveNumber {
        type Error = ConversionError;

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value < 0 {
                Err(ConversionError::Negative)
            } else if value > 1000 {
                Err(ConversionError::TooLarge)
            } else {
                Ok(PositiveNumber(value as u32))
            }
        }
    }

    // 测试自定义 TryFrom
    let test_values = [-5, 42, 1500];
    for &val in &test_values {
        match PositiveNumber::try_from(val) {
            Ok(pos_num) => println!("转换成功: {} -> {:?}", val, pos_num),
            Err(e) => println!("转换失败 {}: {}", val, e),
        }
    }

    // 3.4 字符串解析示例
    println!("\n3.4 字符串解析示例:");
    let strings = ["42", "hello", "3.14"];
    for s in &strings {
        match s.parse::<i32>() {
            Ok(num) => println!("解析成功: '{}' -> {}", s, num),
            Err(e) => println!("解析失败 '{}': {}", s, e),
        }
    }
}

/// 4. transmute 和 unsafe 转换 - 底层内存转换
///
/// transmute 的特点：
/// - 直接重新解释内存位模式
/// - 极其危险，需要 unsafe 块
/// - 要求源类型和目标类型大小相同
/// - 绕过所有类型安全检查
fn demonstrate_unsafe_conversion() {
    println!("\n=== 4. transmute 和 unsafe 转换 ===");

    // 4.1 基础 transmute 示例
    println!("\n4.1 基础 transmute 示例:");

    // 将 f32 的位模式重新解释为 u32
    let f: f32 = 3.14159;
    let bits: u32 = unsafe { mem::transmute(f) };
    println!("f32 {} 的位模式: 0x{:08x}", f, bits);

    // 将 u32 重新解释为 f32
    let back_to_float: f32 = unsafe { mem::transmute(bits) };
    println!("位模式 0x{:08x} 作为 f32: {}", bits, back_to_float);

    // 4.2 数组和元组的转换
    println!("\n4.2 数组和元组的转换:");

    let array: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
    let as_u32: u32 = unsafe { mem::transmute(array) };
    println!("数组 {:?} 作为 u32: 0x{:08x}", array, as_u32);

    // 4.3 函数指针转换
    println!("\n4.3 函数指针转换:");

    fn example_function() -> i32 {
        42
    }

    let fn_ptr = example_function as *const ();
    let addr = fn_ptr as usize;
    println!("函数地址: 0x{:x}", addr);

    // 4.4 更安全的替代方案
    println!("\n4.4 更安全的替代方案:");

    // 使用 to_ne_bytes 和 from_ne_bytes
    let num: u32 = 0x12345678;
    let bytes = num.to_ne_bytes();
    let back_to_num = u32::from_ne_bytes(bytes);
    println!("u32 {} -> bytes {:?} -> u32 {}", num, bytes, back_to_num);

    // 使用联合体（Union）
    #[repr(C)]
    union FloatOrInt {
        f: f32,
        i: u32,
    }

    let mut converter = FloatOrInt { f: 3.14159 };
    let float_bits = unsafe { converter.i };
    println!(
        "使用联合体转换: f32 {} -> u32 0x{:08x}",
        unsafe { converter.f },
        float_bits
    );

    // 4.5 危险示例和注意事项
    println!("\n4.5 危险示例和注意事项:");

    // 错误示例：大小不匹配（编译时会报错）
    // let wrong: u64 = unsafe { mem::transmute(42u32) }; // 编译错误

    // 错误示例：违反类型不变量
    // let invalid_bool: bool = unsafe { mem::transmute(2u8) }; // 未定义行为

    println!("transmute 使用注意事项:");
    println!("1. 确保源类型和目标类型大小相同");
    println!("2. 确保转换后的值对目标类型是有效的");
    println!("3. 考虑字节序问题");
    println!("4. 尽量使用更安全的替代方案");
}

/// 5. 实际应用场景 - 真实世界中的类型转换
fn demonstrate_real_world_examples() {
    println!("\n=== 5. 实际应用场景 ===");

    // 5.1 网络编程中的字节序转换
    println!("\n5.1 网络编程中的字节序转换:");

    let host_value: u32 = 0x12345678;
    let network_value = host_value.to_be(); // 转换为大端序
    let back_to_host = u32::from_be(network_value);

    println!("主机字节序: 0x{:08x}", host_value);
    println!("网络字节序: 0x{:08x}", network_value);
    println!("转换回主机序: 0x{:08x}", back_to_host);

    // 5.2 JSON 序列化中的类型转换
    println!("\n5.2 配置解析中的类型转换:");

    #[derive(Debug)]
    struct Config {
        port: u16,
        host: String,
        debug: bool,
    }

    impl TryFrom<&str> for Config {
        type Error = String;

        fn try_from(config_str: &str) -> Result<Self, Self::Error> {
            // 简化的配置解析
            let lines: Vec<&str> = config_str.lines().collect();
            if lines.len() != 3 {
                return Err("配置格式错误".to_string());
            }

            let port = lines[0]
                .parse::<u16>()
                .map_err(|_| "端口解析错误".to_string())?;
            let host = lines[1].to_string();
            let debug = lines[2]
                .parse::<bool>()
                .map_err(|_| "调试标志解析错误".to_string())?;

            Ok(Config { port, host, debug })
        }
    }

    let config_text = "8080\nlocalhost\ntrue";
    match Config::try_from(config_text) {
        Ok(config) => println!("配置解析成功: {:?}", config),
        Err(e) => println!("配置解析失败: {}", e),
    }

    // 5.3 数据库 ORM 中的类型转换
    println!("\n5.3 数据库 ORM 中的类型转换:");

    #[derive(Debug)]
    struct User {
        id: i64,
        name: String,
        email: Option<String>,
    }

    // 模拟数据库行
    type DbRow = (i64, String, Option<String>);

    impl From<DbRow> for User {
        fn from(row: DbRow) -> Self {
            User {
                id: row.0,
                name: row.1,
                email: row.2,
            }
        }
    }

    let db_row: DbRow = (
        1,
        "Alice".to_string(),
        Some("alice@example.com".to_string()),
    );
    let user: User = db_row.into();
    println!("数据库行转用户: {:?}", user);

    // 5.4 错误处理中的类型转换
    println!("\n5.4 错误处理中的类型转换:");

    #[derive(Debug)]
    enum AppError {
        Io(std::io::Error),
        Parse(std::num::ParseIntError),
        Custom(String),
    }

    impl From<std::io::Error> for AppError {
        fn from(error: std::io::Error) -> Self {
            AppError::Io(error)
        }
    }

    impl From<std::num::ParseIntError> for AppError {
        fn from(error: std::num::ParseIntError) -> Self {
            AppError::Parse(error)
        }
    }

    fn parse_number(s: &str) -> Result<i32, AppError> {
        let num = s.parse::<i32>()?; // 自动转换 ParseIntError -> AppError
        Ok(num)
    }

    match parse_number("42") {
        Ok(num) => println!("解析成功: {}", num),
        Err(e) => println!("解析失败: {:?}", e),
    }

    match parse_number("invalid") {
        Ok(num) => println!("解析成功: {}", num),
        Err(e) => println!("解析失败: {:?}", e),
    }
}

/// 6. 性能对比和最佳实践
fn demonstrate_performance_and_best_practices() {
    println!("\n=== 6. 性能对比和最佳实践 ===");

    // 6.1 转换方式的性能对比
    println!("\n6.1 转换方式的性能对比:");

    use std::time::Instant;

    let iterations = 1_000_000;
    let test_value: i64 = 42;

    // as 转换性能测试
    let start = Instant::now();
    for _ in 0..iterations {
        let _: i32 = test_value as i32;
    }
    let as_duration = start.elapsed();

    // TryFrom 转换性能测试
    let start = Instant::now();
    for _ in 0..iterations {
        let _: Result<i32, _> = i32::try_from(test_value);
    }
    let try_from_duration = start.elapsed();

    println!("as 转换耗时: {:?}", as_duration);
    println!("TryFrom 转换耗时: {:?}", try_from_duration);
    println!(
        "性能比率: {:.2}x",
        try_from_duration.as_nanos() as f64 / as_duration.as_nanos() as f64
    );

    // 6.2 最佳实践指南
    println!("\n6.2 最佳实践指南:");

    println!("\n类型转换选择指南:");
    println!("1. 数值类型间的简单转换 -> 使用 as");
    println!("2. 可能失败的转换 -> 使用 TryFrom/TryInto");
    println!("3. 自定义类型转换 -> 实现 From/Into");
    println!("4. 底层内存操作 -> 谨慎使用 transmute");

    println!("\n安全性考虑:");
    println!("1. 优先使用类型安全的转换方法");
    println!("2. 避免数据丢失和溢出");
    println!("3. 处理转换错误");
    println!("4. 文档化转换的前提条件");

    println!("\n性能优化建议:");
    println!("1. 在热路径中使用 as 转换");
    println!("2. 批量转换时考虑 SIMD");
    println!("3. 避免不必要的中间转换");
    println!("4. 使用编译器优化");

    // 6.3 常见陷阱和解决方案
    println!("\n6.3 常见陷阱和解决方案:");

    println!("\n陷阱1: 整数溢出");
    let large: i64 = i64::MAX;
    // 错误做法：let small = large as i32;  // 可能溢出
    // 正确做法：
    match i32::try_from(large) {
        Ok(small) => println!("转换成功: {}", small),
        Err(_) => println!("转换失败：数值太大"),
    }

    println!("\n陷阱2: 浮点数精度丢失");
    let precise: f64 = 1.23456789012345;
    let less_precise = precise as f32 as f64;
    println!("原始值: {}", precise);
    println!("精度丢失后: {}", less_precise);
    println!("差值: {}", (precise - less_precise).abs());

    println!("\n陷阱3: 字符编码问题");
    let unicode_char = '🦀'; // Rust 吉祥物
    let unicode_value = unicode_char as u32;
    println!("Unicode 字符 '{}' 的值: {}", unicode_char, unicode_value);
    // 注意：不能直接转换为 u8，因为会截断

    println!("\n=== 教程总结 ===");
    println!("本教程全面覆盖了 Rust 中的类型转换机制：");
    println!("• as 关键字：快速但可能不安全的转换");
    println!("• From/Into：类型安全的转换 trait");
    println!("• TryFrom/TryInto：可失败的安全转换");
    println!("• transmute：底层内存转换（需谨慎使用）");
    println!("• 实际应用场景和最佳实践");
    println!("\n选择合适的转换方法是编写安全高效 Rust 代码的关键！");
}

// 额外的辅助函数和类型定义

/// 演示泛型转换函数
fn generic_convert<T, U>(value: T) -> U
where
    U: From<T>,
{
    U::from(value)
}

/// 演示条件转换
fn safe_downcast<T, U>(value: T) -> Option<U>
where
    U: TryFrom<T>,
{
    U::try_from(value).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_conversion() {
        let a: i32 = 42;
        let b: i64 = a as i64;
        assert_eq!(b, 42i64);
    }

    #[test]
    fn test_from_conversion() {
        let s = String::from("hello");
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_try_conversion() {
        let large: i64 = 300;
        let result: Result<i8, _> = i8::try_from(large);
        assert!(result.is_err());

        let small: i64 = 100;
        let result: Result<i8, _> = i8::try_from(small);
        assert_eq!(result.unwrap(), 100i8);
    }

    #[test]
    fn test_generic_convert() {
        let result: i64 = generic_convert(42i32);
        assert_eq!(result, 42i64);
    }

    #[test]
    fn test_safe_downcast() {
        let result: Option<i8> = safe_downcast(100i64);
        assert_eq!(result, Some(100i8));

        let result: Option<i8> = safe_downcast(300i64);
        assert_eq!(result, None);
    }
}
