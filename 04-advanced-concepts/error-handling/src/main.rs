//! Rust 错误处理深度教程
//! 
//! 本教程全面覆盖 Rust 错误处理的所有核心概念：
//! 1. 错误处理基础和设计哲学
//! 2. panic! 机制和不可恢复错误
//! 3. Result<T, E> 类型和可恢复错误
//! 4. Option<T> 类型和空值处理
//! 5. 错误传播和 ? 操作符
//! 6. 自定义错误类型
//! 7. 错误处理模式和最佳实践
//! 8. 高级错误处理技巧

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read, Write};
use std::num::ParseIntError;
use std::panic;
use std::thread;
use std::time::Duration;

// ============================================================================
// 1. 错误处理基础和设计哲学
// ============================================================================

/// Rust 错误处理的核心原则演示
fn error_handling_philosophy() {
    println!("\n=== Rust 错误处理设计哲学 ===");
    
    // Rust 将错误分为两大类：
    // 1. 不可恢复错误 (panic!) - 程序无法继续执行
    // 2. 可恢复错误 (Result<T, E>) - 程序可以处理并继续执行
    
    println!("1. 显式错误处理 - 错误必须被明确处理");
    println!("2. 零成本抽象 - 错误处理不会带来运行时开销");
    println!("3. 类型安全 - 编译时保证错误处理的正确性");
    println!("4. 组合性 - 错误可以被组合和转换");
}

// ============================================================================
// 2. panic! 机制和不可恢复错误
// ============================================================================

/// 演示 panic! 的各种触发方式
fn demonstrate_panic_scenarios() {
    println!("\n=== panic! 机制演示 ===");
    
    // 2.1 显式 panic!
    println!("\n2.1 显式 panic! 调用:");
    // panic!("这是一个显式的 panic!"); // 取消注释会导致程序崩溃
    
    // 2.2 数组越界访问
    println!("2.2 数组越界访问会触发 panic!:");
    let _arr = [1, 2, 3];
    // let invalid = _arr[10]; // 取消注释会导致 panic!
    
    // 2.3 整数溢出 (debug 模式下)
    println!("2.3 整数溢出 (debug 模式下会 panic!):");
    // let max = i32::MAX;
    // let overflow = max + 1; // debug 模式下会 panic!
    
    // 2.4 unwrap() 调用失败
    println!("2.4 unwrap() 调用失败:");
    let _result: Result<i32, &str> = Err("错误信息");
    // _result.unwrap(); // 取消注释会导致 panic!
    
    println!("panic! 演示完成 (实际的 panic! 调用已注释)");
}

/// 演示如何捕获 panic!
fn demonstrate_panic_catching() {
    println!("\n=== 捕获 panic! 演示 ===");
    
    // 使用 std::panic::catch_unwind 捕获 panic!
    let result = panic::catch_unwind(|| {
        println!("即将触发 panic!");
        panic!("这是一个被捕获的 panic!");
    });
    
    match result {
        Ok(_) => println!("代码正常执行"),
        Err(_) => println!("捕获到 panic! - 程序继续运行"),
    }
    
    // 设置自定义 panic hook
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(|panic_info| {
        println!("自定义 panic 处理器: {:?}", panic_info);
    }));
    
    // 恢复原始 hook
    panic::set_hook(original_hook);
}

// ============================================================================
// 3. Result<T, E> 类型和可恢复错误
// ============================================================================

/// 演示 Result 类型的基本使用
fn demonstrate_result_basics() {
    println!("\n=== Result<T, E> 基础演示 ===");
    
    // 3.1 创建 Result 值
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("出现错误");
    
    println!("成功值: {:?}", success);
    println!("错误值: {:?}", failure);
    
    // 3.2 模式匹配处理 Result
    match success {
        Ok(value) => println!("成功获取值: {}", value),
        Err(error) => println!("错误: {}", error),
    }
    
    // 3.3 使用 if let 简化处理
    if let Ok(value) = success {
        println!("使用 if let 获取值: {}", value);
    }
    
    if let Err(error) = failure {
        println!("使用 if let 获取错误: {}", error);
    }
}

/// 文件操作的 Result 示例
fn demonstrate_file_operations() -> Result<String, io::Error> {
    println!("\n=== 文件操作 Result 示例 ===");
    
    // 尝试读取文件
    let mut file = File::open("example.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    Ok(contents)
}

/// 数字解析的 Result 示例
fn demonstrate_parsing() {
    println!("\n=== 数字解析 Result 示例 ===");
    
    let numbers = vec!["42", "abc", "123", "xyz"];
    
    for num_str in numbers {
        match num_str.parse::<i32>() {
            Ok(num) => println!("解析成功: {} -> {}", num_str, num),
            Err(e) => println!("解析失败: {} -> {}", num_str, e),
        }
    }
}

// ============================================================================
// 4. Option<T> 类型和空值处理
// ============================================================================

/// 演示 Option 类型的基本使用
fn demonstrate_option_basics() {
    println!("\n=== Option<T> 基础演示 ===");
    
    // 4.1 创建 Option 值
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;
    
    println!("Some 值: {:?}", some_value);
    println!("None 值: {:?}", none_value);
    
    // 4.2 模式匹配处理 Option
    match some_value {
        Some(value) => println!("找到值: {}", value),
        None => println!("没有值"),
    }
    
    // 4.3 使用 if let 简化处理
    if let Some(value) = some_value {
        println!("使用 if let 获取值: {}", value);
    }
    
    // 4.4 Option 的常用方法
    println!("\nOption 常用方法演示:");
    println!("is_some(): {}", some_value.is_some());
    println!("is_none(): {}", none_value.is_none());
    println!("unwrap_or(0): {}", none_value.unwrap_or(0));
    println!("unwrap_or_else(|| 100): {}", none_value.unwrap_or_else(|| 100));
}

/// HashMap 查找的 Option 示例
fn demonstrate_hashmap_lookup() {
    println!("\n=== HashMap 查找 Option 示例 ===");
    
    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    
    let names = vec!["Alice", "Bob", "Charlie"];
    
    for name in names {
        match scores.get(name) {
            Some(score) => println!("{} 的分数: {}", name, score),
            None => println!("{} 没有分数记录", name),
        }
    }
}

/// 字符串查找的 Option 示例
fn demonstrate_string_operations() {
    println!("\n=== 字符串操作 Option 示例 ===");
    
    let text = "Hello, Rust World!";
    
    // 查找子字符串
    match text.find("Rust") {
        Some(index) => println!("找到 'Rust' 在位置: {}", index),
        None => println!("未找到 'Rust'"),
    }
    
    // 获取字符
    match text.chars().nth(7) {
        Some(ch) => println!("第8个字符是: {}", ch),
        None => println!("没有第8个字符"),
    }
    
    // 分割字符串
    let parts: Vec<&str> = text.split(", ").collect();
    println!("分割结果: {:?}", parts);
}

// ============================================================================
// 5. 错误传播和 ? 操作符
// ============================================================================

/// 演示错误传播的传统方式
fn read_file_traditional(filename: &str) -> Result<String, io::Error> {
    let file_result = File::open(filename);
    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    
    let mut contents = String::new();
    let read_result = file.read_to_string(&mut contents);
    match read_result {
        Ok(_) => Ok(contents),
        Err(error) => Err(error),
    }
}

/// 演示使用 ? 操作符简化错误传播
fn read_file_with_question_mark(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// 演示链式调用中的错误传播
fn read_and_parse_number(filename: &str) -> Result<i32, Box<dyn Error>> {
    let contents = std::fs::read_to_string(filename)?;
    let number = contents.trim().parse::<i32>()?;
    Ok(number)
}

/// 演示 ? 操作符在 Option 中的使用
fn get_first_word_length(text: &str) -> Option<usize> {
    let first_word = text.split_whitespace().next()?;
    Some(first_word.len())
}

/// 错误传播演示函数
fn demonstrate_error_propagation() {
    println!("\n=== 错误传播演示 ===");
    
    // 创建测试文件
    let test_content = "42";
    if let Ok(mut file) = File::create("test_number.txt") {
        let _ = file.write_all(test_content.as_bytes());
    }
    
    // 测试文件读取和解析
    match read_and_parse_number("test_number.txt") {
        Ok(number) => println!("成功解析数字: {}", number),
        Err(error) => println!("解析失败: {}", error),
    }
    
    // 测试 Option 中的 ? 操作符
    let text = "Hello Rust World";
    match get_first_word_length(text) {
        Some(length) => println!("第一个单词长度: {}", length),
        None => println!("没有找到单词"),
    }
    
    // 清理测试文件
    let _ = std::fs::remove_file("test_number.txt");
}

// ============================================================================
// 6. 自定义错误类型
// ============================================================================

/// 自定义错误枚举
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    InvalidInput(String),
}

/// 为自定义错误实现 Display trait
impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "除零错误"),
            MathError::NegativeSquareRoot => write!(f, "负数开平方根错误"),
            MathError::InvalidInput(msg) => write!(f, "无效输入: {}", msg),
        }
    }
}

/// 为自定义错误实现 Error trait
impl Error for MathError {}

/// 实现从其他错误类型的转换
impl From<ParseIntError> for MathError {
    fn from(error: ParseIntError) -> Self {
        MathError::InvalidInput(format!("解析错误: {}", error))
    }
}

/// 使用自定义错误的数学运算
fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn safe_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

fn parse_and_calculate(input: &str) -> Result<f64, MathError> {
    let number: f64 = input.parse().map_err(|e| {
        MathError::InvalidInput(format!("无法解析 '{}': {}", input, e))
    })?;
    safe_sqrt(number)
}

/// 演示自定义错误类型
fn demonstrate_custom_errors() {
    println!("\n=== 自定义错误类型演示 ===");
    
    // 测试除法
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(error) => println!("除法错误: {}", error),
    }
    
    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(error) => println!("除法错误: {}", error),
    }
    
    // 测试平方根
    match safe_sqrt(16.0) {
        Ok(result) => println!("sqrt(16) = {}", result),
        Err(error) => println!("平方根错误: {}", error),
    }
    
    match safe_sqrt(-4.0) {
        Ok(result) => println!("sqrt(-4) = {}", result),
        Err(error) => println!("平方根错误: {}", error),
    }
    
    // 测试解析和计算
    let inputs = vec!["16", "-4", "abc"];
    for input in inputs {
        match parse_and_calculate(input) {
            Ok(result) => println!("sqrt({}) = {}", input, result),
            Err(error) => println!("计算错误: {}", error),
        }
    }
}

// ============================================================================
// 7. 错误处理模式和最佳实践
// ============================================================================

/// 演示各种错误处理模式
fn demonstrate_error_patterns() {
    println!("\n=== 错误处理模式演示 ===");
    
    let result: Result<i32, &str> = Ok(42);
    let error_result: Result<i32, &str> = Err("错误信息");
    
    // 7.1 unwrap() - 确信不会出错时使用
    println!("\n7.1 unwrap() 模式:");
    let value = result.unwrap();
    println!("unwrap 获取值: {}", value);
    
    // 7.2 expect() - 提供更好的错误信息
    println!("\n7.2 expect() 模式:");
    let value = result.expect("这里不应该出错");
    println!("expect 获取值: {}", value);
    
    // 7.3 unwrap_or() - 提供默认值
    println!("\n7.3 unwrap_or() 模式:");
    let value = error_result.unwrap_or(0);
    println!("unwrap_or 获取值: {}", value);
    
    // 7.4 unwrap_or_else() - 使用闭包计算默认值
    println!("\n7.4 unwrap_or_else() 模式:");
    let value = error_result.unwrap_or_else(|_| {
        println!("计算默认值");
        -1
    });
    println!("unwrap_or_else 获取值: {}", value);
    
    // 7.5 map() - 转换成功值
    println!("\n7.5 map() 模式:");
    let doubled = result.map(|x| x * 2);
    println!("map 转换结果: {:?}", doubled);
    
    // 7.6 map_err() - 转换错误值
    println!("\n7.6 map_err() 模式:");
    let mapped_error = error_result.map_err(|e| format!("映射错误: {}", e));
    println!("map_err 转换结果: {:?}", mapped_error);
    
    // 7.7 and_then() - 链式操作
    println!("\n7.7 and_then() 模式:");
    let chained = result.and_then(|x| {
        if x > 0 {
            Ok(x * 2)
        } else {
            Err("值必须为正数")
        }
    });
    println!("and_then 链式结果: {:?}", chained);
}

/// 演示组合器方法的实际应用
fn demonstrate_combinators() {
    println!("\n=== 组合器方法实际应用 ===");
    
    // 模拟用户输入处理
    let user_inputs = vec!["42", "0", "-5", "abc"];
    
    for input in user_inputs {
        let result = input
            .parse::<i32>()
            .map_err(|_| "解析失败")
            .and_then(|num| {
                if num > 0 {
                    Ok(num)
                } else {
                    Err("数字必须为正数")
                }
            })
            .map(|num| num * num);
        
        match result {
            Ok(square) => println!("输入 '{}' 的平方: {}", input, square),
            Err(error) => println!("处理 '{}' 失败: {}", input, error),
        }
    }
}

// ============================================================================
// 8. 高级错误处理技巧
// ============================================================================

/// 演示错误上下文和错误链
fn demonstrate_error_context() {
    println!("\n=== 错误上下文演示 ===");
    
    // 模拟多层函数调用中的错误传播
    fn level3() -> Result<i32, Box<dyn Error>> {
        Err("底层错误".into())
    }
    
    fn level2() -> Result<i32, Box<dyn Error>> {
        level3().map_err(|e| format!("level2 中的错误: {}", e).into())
    }
    
    fn level1() -> Result<i32, Box<dyn Error>> {
        level2().map_err(|e| format!("level1 中的错误: {}", e).into())
    }
    
    match level1() {
        Ok(value) => println!("成功: {}", value),
        Err(error) => {
            println!("错误链: {}", error);
            
            // 遍历错误链
            let mut source = error.source();
            let mut level = 1;
            while let Some(err) = source {
                println!("  原因 {}: {}", level, err);
                source = err.source();
                level += 1;
            }
        }
    }
}

/// 演示多线程环境下的错误处理
fn demonstrate_threaded_errors() {
    println!("\n=== 多线程错误处理演示 ===");
    
    let handles: Vec<_> = (0..3)
        .map(|i| {
            thread::spawn(move || {
                if i == 1 {
                    Err(format!("线程 {} 发生错误", i))
                } else {
                    Ok(format!("线程 {} 成功", i))
                }
            })
        })
        .collect();
    
    for (i, handle) in handles.into_iter().enumerate() {
        match handle.join() {
            Ok(result) => match result {
                Ok(msg) => println!("线程 {}: {}", i, msg),
                Err(error) => println!("线程 {} 错误: {}", i, error),
            },
            Err(_) => println!("线程 {} panic!", i),
        }
    }
}

/// 演示超时和重试机制
fn demonstrate_retry_mechanism() {
    println!("\n=== 重试机制演示 ===");
    
    fn unreliable_operation(attempt: u32) -> Result<String, &'static str> {
        if attempt < 3 {
            Err("操作失败")
        } else {
            Ok("操作成功".to_string())
        }
    }
    
    fn retry_operation<F, T, E>(mut operation: F, max_attempts: u32) -> Result<T, E>
    where
        F: FnMut(u32) -> Result<T, E>,
    {
        for attempt in 1..=max_attempts {
            match operation(attempt) {
                Ok(result) => return Ok(result),
                Err(error) => {
                    if attempt == max_attempts {
                        return Err(error);
                    }
                    println!("尝试 {} 失败，重试中...", attempt);
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }
        unreachable!()
    }
    
    match retry_operation(unreliable_operation, 5) {
        Ok(result) => println!("重试成功: {}", result),
        Err(error) => println!("重试失败: {}", error),
    }
}

// ============================================================================
// 主函数 - 运行所有演示
// ============================================================================

fn main() {
    println!("🦀 Rust 错误处理深度教程");
    println!("{}", "=".repeat(50));
    
    // 1. 错误处理基础
    error_handling_philosophy();
    
    // 2. panic! 机制
    demonstrate_panic_scenarios();
    demonstrate_panic_catching();
    
    // 3. Result 类型
    demonstrate_result_basics();
    demonstrate_parsing();
    
    // 测试文件操作（可能失败）
    match demonstrate_file_operations() {
        Ok(contents) => println!("文件内容: {}", contents),
        Err(error) => println!("文件操作失败: {} (这是正常的，因为文件不存在)", error),
    }
    
    // 4. Option 类型
    demonstrate_option_basics();
    demonstrate_hashmap_lookup();
    demonstrate_string_operations();
    
    // 5. 错误传播
    demonstrate_error_propagation();
    
    // 6. 自定义错误
    demonstrate_custom_errors();
    
    // 7. 错误处理模式
    demonstrate_error_patterns();
    demonstrate_combinators();
    
    // 8. 高级技巧
    demonstrate_error_context();
    demonstrate_threaded_errors();
    demonstrate_retry_mechanism();
    
    println!("\n{}", "=".repeat(50));
    println!("🎉 Rust 错误处理教程完成！");
    println!("\n核心要点总结:");
    println!("• 使用 Result<T, E> 处理可恢复错误");
    println!("• 使用 Option<T> 处理可能为空的值");
    println!("• 使用 ? 操作符简化错误传播");
    println!("• 创建自定义错误类型提供更好的错误信息");
    println!("• 使用组合器方法优雅地处理错误");
    println!("• 在适当的时候使用 panic! 处理不可恢复错误");
    println!("• 构建错误上下文和错误链提供更好的调试信息");
}

// ============================================================================
// 测试模块
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10.0, 2.0).unwrap(), 5.0);
        assert!(safe_divide(10.0, 0.0).is_err());
    }
    
    #[test]
    fn test_safe_sqrt() {
        assert_eq!(safe_sqrt(16.0).unwrap(), 4.0);
        assert!(safe_sqrt(-4.0).is_err());
    }
    
    #[test]
    fn test_get_first_word_length() {
        assert_eq!(get_first_word_length("Hello World"), Some(5));
        assert_eq!(get_first_word_length(""), None);
    }
    
    #[test]
    fn test_error_conversion() {
        let parse_error: ParseIntError = "abc".parse::<i32>().unwrap_err();
        let math_error: MathError = parse_error.into();
        assert!(matches!(math_error, MathError::InvalidInput(_)));
    }
    
    #[test]
    #[should_panic(expected = "测试 panic!")]
    fn test_panic() {
        panic!("测试 panic!");
    }
}