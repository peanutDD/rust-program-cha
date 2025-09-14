//! Rust Result 错误处理深度分析
//!
//! 本文档全面分析 Rust 中的 Result<T, E> 类型，涵盖基础概念、语法操作、
//! 错误传播、自定义错误类型、最佳实践和实际应用案例。

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::num::ParseIntError;
use std::time::{Duration, Instant};

fn main() {
    println!("=== Rust Result 错误处理深度分析 ===");

    // 1. Result 基础概念
    demonstrate_result_basics();

    // 2. Result 语法和基本操作
    demonstrate_result_syntax();

    // 3. 错误传播机制
    demonstrate_error_propagation();

    // 4. Result 组合器
    demonstrate_result_combinators();

    // 5. 自定义错误类型
    demonstrate_custom_errors();

    // 6. 错误处理模式
    demonstrate_error_handling_patterns();

    // 7. Result 最佳实践
    demonstrate_result_best_practices();

    // 8. 实际应用案例
    demonstrate_real_world_applications();

    println!("\n=== Result 分析完成 ===");
}

// ============================================================================
// 1. Result 基础概念
// ============================================================================

fn demonstrate_result_basics() {
    println!("\n1. Result 基础概念");
    println!("==================");

    // 1.1 Result<T, E> 类型定义
    println!("\n1.1 Result<T, E> 类型定义:");

    // Result 是一个枚举类型，有两个变体：Ok(T) 和 Err(E)
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("Something went wrong");

    println!("成功结果: {:?}", success);
    println!("失败结果: {:?}", failure);

    // 1.2 Result 与 Option 的区别和联系
    println!("\n1.2 Result 与 Option 的区别:");

    // Option 表示值的存在或不存在
    let some_value: Option<i32> = Some(42);
    let no_value: Option<i32> = None;

    // Result 表示操作的成功或失败，失败时包含错误信息
    let parse_success: Result<i32, ParseIntError> = "42".parse();
    let parse_failure: Result<i32, ParseIntError> = "abc".parse();

    println!("Option Some: {:?}, None: {:?}", some_value, no_value);
    println!("Result Ok: {:?}", parse_success);
    println!("Result Err: {:?}", parse_failure);

    // 1.3 Result 的内存布局和性能特性
    println!("\n1.3 Result 的内存特性:");
    println!(
        "Result<i32, String> 大小: {} bytes",
        std::mem::size_of::<Result<i32, String>>()
    );
    println!(
        "Option<i32> 大小: {} bytes",
        std::mem::size_of::<Option<i32>>()
    );

    // 1.4 Result 的零成本抽象特性
    demonstrate_zero_cost_abstraction();
}

fn demonstrate_zero_cost_abstraction() {
    println!("\n1.4 零成本抽象演示:");

    let start = Instant::now();

    // 使用 Result 进行大量操作
    let mut sum: i64 = 0;
    for i in 0..100000 {
        let result: Result<i32, &str> = if i % 2 == 0 { Ok(i) } else { Err("odd") };
        if let Ok(value) = result {
            sum += value as i64;
        }
    }

    let duration = start.elapsed();
    println!("Result 操作耗时: {:?}, 结果: {}", duration, sum);
}

// ============================================================================
// 2. Result 语法和基本操作
// ============================================================================

fn demonstrate_result_syntax() {
    println!("\n2. Result 语法和基本操作");
    println!("========================");

    // 2.1 创建 Result
    println!("\n2.1 创建 Result:");

    // 直接创建
    let ok_result: Result<i32, &str> = Ok(100);
    let err_result: Result<i32, &str> = Err("错误信息");

    // 通过函数返回
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("除零错误".to_string())
        } else {
            Ok(a / b)
        }
    }

    let division_result = divide(10.0, 2.0);
    println!("除法结果: {:?}", division_result);

    // 2.2 模式匹配
    println!("\n2.2 模式匹配:");

    match divide(10.0, 0.0) {
        Ok(result) => println!("计算成功: {}", result),
        Err(error) => println!("计算失败: {}", error),
    }

    // 使用 if let 进行简化匹配
    if let Ok(value) = divide(15.0, 3.0) {
        println!("简化匹配成功: {}", value);
    }

    // 2.3 unwrap 系列方法
    println!("\n2.3 unwrap 系列方法:");

    let safe_result: Result<i32, &str> = Ok(42);
    println!("unwrap 成功: {}", safe_result.unwrap());

    // unwrap_or: 提供默认值
    let unsafe_result: Result<i32, &str> = Err("错误");
    println!("unwrap_or 默认值: {}", unsafe_result.unwrap_or(0));

    // unwrap_or_else: 通过闭包计算默认值
    println!(
        "unwrap_or_else 计算值: {}",
        unsafe_result.unwrap_or_else(|_| 42)
    );

    // unwrap_or_default: 使用类型的默认值
    println!("unwrap_or_default: {}", unsafe_result.unwrap_or_default());

    // 2.4 expect 方法
    println!("\n2.4 expect 方法:");

    let result: Result<&str, &str> = Ok("成功值");
    println!("expect 成功: {}", result.expect("这不应该失败"));

    // expect 提供更好的错误信息
    demonstrate_expect_usage();
}

fn demonstrate_expect_usage() {
    fn parse_config_value(input: &str) -> Result<i32, ParseIntError> {
        input.parse()
    }

    let config_value = "42";
    let parsed = parse_config_value(config_value).expect("配置值应该是有效的整数");

    println!("解析的配置值: {}", parsed);
}

// ============================================================================
// 3. 错误传播机制
// ============================================================================

fn demonstrate_error_propagation() {
    println!("\n3. 错误传播机制");
    println!("================");

    // 3.1 ? 操作符详解
    println!("\n3.1 ? 操作符:");

    fn read_and_parse_file(path: &str) -> Result<i32, Box<dyn Error>> {
        let content = fs::read_to_string(path)?; // 错误传播
        let number: i32 = content.trim().parse()?; // 错误传播
        Ok(number)
    }

    // 创建测试文件
    let _ = fs::write("/tmp/test_number.txt", "42");

    match read_and_parse_file("/tmp/test_number.txt") {
        Ok(num) => println!("读取并解析成功: {}", num),
        Err(e) => println!("操作失败: {}", e),
    }

    // 3.2 错误链式传播
    println!("\n3.2 错误链式传播:");

    fn complex_operation() -> Result<String, Box<dyn Error>> {
        let data = fetch_data()?;
        let processed = process_data(&data)?;
        let result = format_result(&processed)?;
        Ok(result)
    }

    fn fetch_data() -> Result<String, &'static str> {
        Ok("raw data".to_string())
    }

    fn process_data(data: &str) -> Result<String, &'static str> {
        if data.is_empty() {
            Err("数据为空")
        } else {
            Ok(data.to_uppercase())
        }
    }

    fn format_result(data: &str) -> Result<String, &'static str> {
        Ok(format!("处理结果: {}", data))
    }

    match complex_operation() {
        Ok(result) => println!("复杂操作成功: {}", result),
        Err(e) => println!("复杂操作失败: {}", e),
    }

    // 3.3 早期返回模式
    println!("\n3.3 早期返回模式:");

    fn validate_and_process(input: &str) -> Result<i32, String> {
        // 验证输入不为空
        if input.is_empty() {
            return Err("输入不能为空".to_string());
        }

        // 验证输入长度
        if input.len() > 10 {
            return Err("输入过长".to_string());
        }

        // 解析数字
        let number: i32 = input.parse().map_err(|_| "无法解析为数字".to_string())?;

        // 验证范围
        if number < 0 || number > 100 {
            return Err("数字超出范围 [0, 100]".to_string());
        }

        Ok(number * 2)
    }

    let test_cases = ["42", "", "abc", "150", "very_long_input_string"];
    for case in &test_cases {
        match validate_and_process(case) {
            Ok(result) => println!("输入 '{}' 处理成功: {}", case, result),
            Err(e) => println!("输入 '{}' 处理失败: {}", case, e),
        }
    }
}

// ============================================================================
// 4. Result 组合器
// ============================================================================

fn demonstrate_result_combinators() {
    println!("\n4. Result 组合器");
    println!("================");

    // 4.1 map 和 map_err
    println!("\n4.1 map 和 map_err:");

    let result: Result<i32, &str> = Ok(5);

    // map: 转换成功值
    let doubled: Result<i32, &str> = result.map(|x| x * 2);
    println!("map 结果: {:?}", doubled);

    let error_result: Result<i32, &str> = Err("原始错误");

    // map_err: 转换错误值
    let mapped_error = error_result.map_err(|e| format!("包装错误: {}", e));
    println!("map_err 结果: {:?}", mapped_error);

    // 4.2 and_then 和 or_else
    println!("\n4.2 and_then 和 or_else:");

    fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("除零错误".to_string())
        } else {
            Ok(a / b)
        }
    }

    fn safe_sqrt(x: f64) -> Result<f64, String> {
        if x < 0.0 {
            Err("负数开方错误".to_string())
        } else {
            Ok(x.sqrt())
        }
    }

    // and_then: 链式操作，只有前一个成功才执行下一个
    let chained_result = safe_divide(16.0, 4.0).and_then(|x| safe_sqrt(x));
    println!("链式操作结果: {:?}", chained_result);

    // or_else: 错误恢复
    let recovery_result: Result<f64, String> = safe_divide(10.0, 0.0).or_else(|_| Ok(0.0)); // 除零时返回 0
    println!("错误恢复结果: {:?}", recovery_result);

    // 4.3 复杂组合器链
    println!("\n4.3 复杂组合器链:");

    fn process_number_chain(input: &str) -> Result<String, String> {
        input
            .parse::<f64>()
            .map_err(|_| "解析失败".to_string())
            .and_then(|x| {
                if x >= 0.0 {
                    Ok(x)
                } else {
                    Err("负数不支持".to_string())
                }
            })
            .map(|x| x.sqrt())
            .map(|x| x * 2.0)
            .map(|x| format!("最终结果: {:.2}", x))
    }

    let test_inputs = ["16", "-4", "abc", "25"];
    for input in &test_inputs {
        match process_number_chain(input) {
            Ok(result) => println!("输入 '{}': {}", input, result),
            Err(e) => println!("输入 '{}' 失败: {}", input, e),
        }
    }

    // 4.4 组合多个 Result
    println!("\n4.4 组合多个 Result:");

    fn combine_results() -> Result<i32, String> {
        let a = "10".parse::<i32>().map_err(|_| "解析 a 失败".to_string())?;
        let b = "20".parse::<i32>().map_err(|_| "解析 b 失败".to_string())?;
        let c = "30".parse::<i32>().map_err(|_| "解析 c 失败".to_string())?;

        Ok(a + b + c)
    }

    match combine_results() {
        Ok(sum) => println!("组合结果: {}", sum),
        Err(e) => println!("组合失败: {}", e),
    }
}

// ============================================================================
// 5. 自定义错误类型
// ============================================================================

fn demonstrate_custom_errors() {
    println!("\n5. 自定义错误类型");
    println!("==================");

    // 5.1 简单自定义错误
    println!("\n5.1 简单自定义错误:");

    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NegativeSquareRoot,
        Overflow,
    }

    impl fmt::Display for MathError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                MathError::DivisionByZero => write!(f, "除零错误"),
                MathError::NegativeSquareRoot => write!(f, "负数开方错误"),
                MathError::Overflow => write!(f, "数值溢出错误"),
            }
        }
    }

    impl Error for MathError {}

    fn safe_math_operation(a: f64, b: f64) -> Result<f64, MathError> {
        if b == 0.0 {
            return Err(MathError::DivisionByZero);
        }

        let result = a / b;

        if result < 0.0 {
            return Err(MathError::NegativeSquareRoot);
        }

        if result > f64::MAX / 2.0 {
            return Err(MathError::Overflow);
        }

        Ok(result.sqrt())
    }

    match safe_math_operation(16.0, 4.0) {
        Ok(result) => println!("数学操作成功: {}", result),
        Err(e) => println!("数学操作失败: {}", e),
    }

    // 5.2 复杂自定义错误类型
    println!("\n5.2 复杂自定义错误类型:");

    #[derive(Debug)]
    struct ValidationError {
        field: String,
        message: String,
        code: u32,
    }

    impl fmt::Display for ValidationError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "验证错误 [{}]: {} (代码: {})",
                self.field, self.message, self.code
            )
        }
    }

    impl Error for ValidationError {}

    impl ValidationError {
        fn new(field: &str, message: &str, code: u32) -> Self {
            ValidationError {
                field: field.to_string(),
                message: message.to_string(),
                code,
            }
        }
    }

    fn validate_user_input(name: &str, age: i32, email: &str) -> Result<(), ValidationError> {
        if name.is_empty() {
            return Err(ValidationError::new("name", "姓名不能为空", 1001));
        }

        if age < 0 || age > 150 {
            return Err(ValidationError::new("age", "年龄必须在 0-150 之间", 1002));
        }

        if !email.contains('@') {
            return Err(ValidationError::new("email", "邮箱格式无效", 1003));
        }

        Ok(())
    }

    let test_cases = [
        ("", 25, "test@example.com"),
        ("张三", -5, "test@example.com"),
        ("李四", 30, "invalid-email"),
        ("王五", 25, "wang@example.com"),
    ];

    for (name, age, email) in &test_cases {
        match validate_user_input(name, *age, email) {
            Ok(()) => println!("用户 {} 验证通过", name),
            Err(e) => println!("用户验证失败: {}", e),
        }
    }

    // 5.3 错误转换和包装
    println!("\n5.3 错误转换和包装:");

    #[derive(Debug)]
    enum AppError {
        Io(io::Error),
        Parse(ParseIntError),
        Validation(ValidationError),
        Custom(String),
    }

    impl fmt::Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                AppError::Io(e) => write!(f, "IO 错误: {}", e),
                AppError::Parse(e) => write!(f, "解析错误: {}", e),
                AppError::Validation(e) => write!(f, "验证错误: {}", e),
                AppError::Custom(msg) => write!(f, "自定义错误: {}", msg),
            }
        }
    }

    impl Error for AppError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                AppError::Io(e) => Some(e),
                AppError::Parse(e) => Some(e),
                AppError::Validation(e) => Some(e),
                AppError::Custom(_) => None,
            }
        }
    }

    impl From<io::Error> for AppError {
        fn from(error: io::Error) -> Self {
            AppError::Io(error)
        }
    }

    impl From<ParseIntError> for AppError {
        fn from(error: ParseIntError) -> Self {
            AppError::Parse(error)
        }
    }

    impl From<ValidationError> for AppError {
        fn from(error: ValidationError) -> Self {
            AppError::Validation(error)
        }
    }

    fn complex_app_operation(input: &str) -> Result<i32, AppError> {
        // 这里会自动进行错误转换
        let number: i32 = input.parse()?; // ParseIntError -> AppError

        if number < 0 {
            return Err(AppError::Custom("数字不能为负".to_string()));
        }

        Ok(number * 2)
    }

    match complex_app_operation("42") {
        Ok(result) => println!("应用操作成功: {}", result),
        Err(e) => {
            println!("应用操作失败: {}", e);
            if let Some(source) = e.source() {
                println!("根本原因: {}", source);
            }
        }
    }
}

// ============================================================================
// 6. 错误处理模式
// ============================================================================

fn demonstrate_error_handling_patterns() {
    println!("\n6. 错误处理模式");
    println!("================");

    // 6.1 错误恢复策略
    println!("\n6.1 错误恢复策略:");

    fn resilient_operation(attempts: u32) -> Result<String, String> {
        for attempt in 1..=attempts {
            match risky_operation(attempt) {
                Ok(result) => return Ok(result),
                Err(e) => {
                    println!("尝试 {} 失败: {}", attempt, e);
                    if attempt == attempts {
                        return Err(format!("所有 {} 次尝试都失败了", attempts));
                    }
                    // 简单的退避策略
                    std::thread::sleep(Duration::from_millis(100 * attempt as u64));
                }
            }
        }
        unreachable!()
    }

    fn risky_operation(attempt: u32) -> Result<String, String> {
        if attempt < 3 {
            Err(format!("尝试 {} 失败", attempt))
        } else {
            Ok(format!("尝试 {} 成功", attempt))
        }
    }

    match resilient_operation(5) {
        Ok(result) => println!("弹性操作成功: {}", result),
        Err(e) => println!("弹性操作最终失败: {}", e),
    }

    // 6.2 优雅降级
    println!("\n6.2 优雅降级:");

    fn get_user_data(user_id: u32) -> Result<UserData, String> {
        // 尝试从主数据库获取
        match fetch_from_primary_db(user_id) {
            Ok(data) => {
                println!("从主数据库获取用户数据成功");
                Ok(data)
            }
            Err(_) => {
                println!("主数据库失败，尝试备份数据库");
                // 降级到备份数据库
                match fetch_from_backup_db(user_id) {
                    Ok(data) => {
                        println!("从备份数据库获取用户数据成功");
                        Ok(data)
                    }
                    Err(_) => {
                        println!("备份数据库也失败，使用缓存数据");
                        // 最后降级到缓存
                        fetch_from_cache(user_id)
                    }
                }
            }
        }
    }

    #[derive(Debug)]
    struct UserData {
        id: u32,
        name: String,
        source: String,
    }

    fn fetch_from_primary_db(user_id: u32) -> Result<UserData, String> {
        if user_id == 1 {
            Err("主数据库连接失败".to_string())
        } else {
            Ok(UserData {
                id: user_id,
                name: format!("用户{}", user_id),
                source: "主数据库".to_string(),
            })
        }
    }

    fn fetch_from_backup_db(user_id: u32) -> Result<UserData, String> {
        if user_id == 1 {
            Err("备份数据库连接失败".to_string())
        } else {
            Ok(UserData {
                id: user_id,
                name: format!("用户{}", user_id),
                source: "备份数据库".to_string(),
            })
        }
    }

    fn fetch_from_cache(user_id: u32) -> Result<UserData, String> {
        Ok(UserData {
            id: user_id,
            name: format!("缓存用户{}", user_id),
            source: "缓存".to_string(),
        })
    }

    for user_id in [1, 2] {
        match get_user_data(user_id) {
            Ok(data) => println!("获取用户数据: {:?}", data),
            Err(e) => println!("获取用户数据失败: {}", e),
        }
    }

    // 6.3 错误聚合
    println!("\n6.3 错误聚合:");

    fn validate_multiple_fields(data: &HashMap<String, String>) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 验证姓名
        if let Some(name) = data.get("name") {
            if name.is_empty() {
                errors.push("姓名不能为空".to_string());
            }
        } else {
            errors.push("缺少姓名字段".to_string());
        }

        // 验证邮箱
        if let Some(email) = data.get("email") {
            if !email.contains('@') {
                errors.push("邮箱格式无效".to_string());
            }
        } else {
            errors.push("缺少邮箱字段".to_string());
        }

        // 验证年龄
        if let Some(age_str) = data.get("age") {
            match age_str.parse::<i32>() {
                Ok(age) => {
                    if age < 0 || age > 150 {
                        errors.push("年龄必须在 0-150 之间".to_string());
                    }
                }
                Err(_) => errors.push("年龄必须是有效数字".to_string()),
            }
        } else {
            errors.push("缺少年龄字段".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    let mut test_data = HashMap::new();
    test_data.insert("name".to_string(), "".to_string());
    test_data.insert("email".to_string(), "invalid-email".to_string());
    test_data.insert("age".to_string(), "200".to_string());

    match validate_multiple_fields(&test_data) {
        Ok(()) => println!("所有字段验证通过"),
        Err(errors) => {
            println!("验证失败，错误列表:");
            for (i, error) in errors.iter().enumerate() {
                println!("  {}. {}", i + 1, error);
            }
        }
    }
}

// ============================================================================
// 7. Result 最佳实践
// ============================================================================

fn demonstrate_result_best_practices() {
    println!("\n7. Result 最佳实践");
    println!("==================");

    // 7.1 何时使用 Result vs panic
    println!("\n7.1 Result vs panic 选择:");

    // 使用 Result 的场景：可恢复的错误
    fn parse_user_input(input: &str) -> Result<i32, String> {
        input.parse().map_err(|_| "输入不是有效数字".to_string())
    }

    // 使用 panic 的场景：程序逻辑错误或不可恢复的错误
    fn get_array_element(arr: &[i32], index: usize) -> i32 {
        if index >= arr.len() {
            panic!("数组索引 {} 超出范围 [0, {})", index, arr.len());
        }
        arr[index]
    }

    // 演示 Result 的使用
    match parse_user_input("42") {
        Ok(num) => println!("解析成功: {}", num),
        Err(e) => println!("解析失败: {}", e),
    }

    // 演示合理的 panic 使用（在安全范围内）
    let arr = [1, 2, 3, 4, 5];
    let element = get_array_element(&arr, 2);
    println!("数组元素: {}", element);

    // 7.2 错误信息设计
    println!("\n7.2 错误信息设计:");

    #[derive(Debug)]
    struct DetailedError {
        message: String,
        context: HashMap<String, String>,
        timestamp: Instant,
    }

    impl fmt::Display for DetailedError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.message)?;
            if !self.context.is_empty() {
                write!(f, " (上下文: {:?})", self.context)?;
            }
            Ok(())
        }
    }

    impl Error for DetailedError {}

    impl DetailedError {
        fn new(message: &str) -> Self {
            DetailedError {
                message: message.to_string(),
                context: HashMap::new(),
                timestamp: Instant::now(),
            }
        }

        fn with_context(mut self, key: &str, value: &str) -> Self {
            self.context.insert(key.to_string(), value.to_string());
            self
        }
    }

    fn detailed_operation(input: &str) -> Result<i32, DetailedError> {
        if input.is_empty() {
            return Err(DetailedError::new("输入为空")
                .with_context("operation", "detailed_operation")
                .with_context("input_length", "0"));
        }

        input.parse().map_err(|_| {
            DetailedError::new("数字解析失败")
                .with_context("operation", "parse")
                .with_context("input", input)
                .with_context("expected_type", "i32")
        })
    }

    match detailed_operation("abc") {
        Ok(num) => println!("详细操作成功: {}", num),
        Err(e) => println!("详细操作失败: {}", e),
    }

    // 7.3 性能考虑
    println!("\n7.3 性能考虑:");

    // 避免不必要的错误分配
    fn efficient_validation(input: &str) -> Result<&str, &'static str> {
        if input.is_empty() {
            Err("输入为空")
        } else if input.len() > 100 {
            Err("输入过长")
        } else {
            Ok(input)
        }
    }

    // 使用 Cow 进行零拷贝错误处理
    use std::borrow::Cow;

    fn cow_validation(input: &str) -> Result<Cow<str>, Cow<str>> {
        if input.is_empty() {
            Err(Cow::Borrowed("输入为空"))
        } else if input.contains("bad") {
            Err(Cow::Owned(format!("输入包含禁用词: {}", input)))
        } else {
            Ok(Cow::Borrowed(input))
        }
    }

    match cow_validation("good input") {
        Ok(result) => println!("COW 验证成功: {}", result),
        Err(e) => println!("COW 验证失败: {}", e),
    }

    // 7.4 错误处理的组合模式
    println!("\n7.4 错误处理组合模式:");

    fn combined_validation(input: &str) -> Result<i32, String> {
        efficient_validation(input)
            .map_err(|e| e.to_string())
            .and_then(|s| s.parse().map_err(|_| "解析失败".to_string()))
            .and_then(|n| {
                if n > 0 {
                    Ok(n)
                } else {
                    Err("数字必须为正".to_string())
                }
            })
    }

    let test_inputs = ["", "abc", "-5", "42"];
    for input in &test_inputs {
        match combined_validation(input) {
            Ok(result) => println!("组合验证 '{}' 成功: {}", input, result),
            Err(e) => println!("组合验证 '{}' 失败: {}", input, e),
        }
    }
}

// ============================================================================
// 8. 实际应用案例
// ============================================================================

fn demonstrate_real_world_applications() {
    println!("\n8. 实际应用案例");
    println!("================");

    // 8.1 文件操作错误处理
    println!("\n8.1 文件操作:");

    fn safe_file_operations() -> Result<String, Box<dyn Error>> {
        // 创建测试文件
        let test_file = "/tmp/test_result.txt";
        fs::write(test_file, "Hello, Rust Result!")?;

        // 读取文件
        let content = fs::read_to_string(test_file)?;

        // 处理内容
        let processed = content.to_uppercase();

        // 写回文件
        let output_file = "/tmp/test_result_output.txt";
        fs::write(output_file, &processed)?;

        Ok(processed)
    }

    match safe_file_operations() {
        Ok(content) => println!("文件操作成功: {}", content),
        Err(e) => println!("文件操作失败: {}", e),
    }

    // 8.2 JSON 解析错误处理
    println!("\n8.2 JSON 解析:");

    use std::collections::HashMap;

    fn parse_json_config(json_str: &str) -> Result<HashMap<String, String>, String> {
        // 简化的 JSON 解析（实际应用中应使用 serde_json）
        if !json_str.starts_with('{') || !json_str.ends_with('}') {
            return Err("无效的 JSON 格式".to_string());
        }

        let mut config = HashMap::new();
        let content = &json_str[1..json_str.len() - 1]; // 去掉大括号

        for pair in content.split(',') {
            let parts: Vec<&str> = pair.split(':').collect();
            if parts.len() != 2 {
                return Err(format!("无效的键值对: {}", pair));
            }

            let key = parts[0].trim().trim_matches('"');
            let value = parts[1].trim().trim_matches('"');

            config.insert(key.to_string(), value.to_string());
        }

        Ok(config)
    }

    let json_examples = [
        r#"{"name":"张三","age":"30"}"#,
        r#"{"invalid":json}"#,
        r#"not json at all"#,
    ];

    for json in &json_examples {
        match parse_json_config(json) {
            Ok(config) => println!("JSON 解析成功: {:?}", config),
            Err(e) => println!("JSON 解析失败: {}", e),
        }
    }

    // 8.3 网络请求模拟
    println!("\n8.3 网络请求模拟:");

    #[derive(Debug)]
    enum NetworkError {
        Timeout,
        ConnectionRefused,
        InvalidResponse,
        ServerError(u16),
    }

    impl fmt::Display for NetworkError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                NetworkError::Timeout => write!(f, "请求超时"),
                NetworkError::ConnectionRefused => write!(f, "连接被拒绝"),
                NetworkError::InvalidResponse => write!(f, "无效响应"),
                NetworkError::ServerError(code) => write!(f, "服务器错误: {}", code),
            }
        }
    }

    impl Error for NetworkError {}

    fn simulate_http_request(url: &str) -> Result<String, NetworkError> {
        // 模拟不同的网络情况
        match url {
            "http://timeout.com" => Err(NetworkError::Timeout),
            "http://refused.com" => Err(NetworkError::ConnectionRefused),
            "http://invalid.com" => Err(NetworkError::InvalidResponse),
            "http://error500.com" => Err(NetworkError::ServerError(500)),
            _ => Ok(format!("来自 {} 的响应数据", url)),
        }
    }

    fn http_client_with_retry(url: &str, max_retries: u32) -> Result<String, NetworkError> {
        for attempt in 1..=max_retries {
            match simulate_http_request(url) {
                Ok(response) => return Ok(response),
                Err(NetworkError::Timeout) if attempt < max_retries => {
                    println!("尝试 {} 超时，重试中...", attempt);
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
        Err(NetworkError::Timeout)
    }

    let urls = [
        "http://success.com",
        "http://timeout.com",
        "http://error500.com",
    ];

    for url in &urls {
        match http_client_with_retry(url, 3) {
            Ok(response) => println!("请求 {} 成功: {}", url, response),
            Err(e) => println!("请求 {} 失败: {}", url, e),
        }
    }

    // 8.4 数据库操作模拟
    println!("\n8.4 数据库操作模拟:");

    #[derive(Debug)]
    struct User {
        id: u32,
        name: String,
        email: String,
    }

    #[derive(Debug)]
    enum DatabaseError {
        ConnectionFailed,
        QueryFailed(String),
        UserNotFound(u32),
        DuplicateEntry,
    }

    impl fmt::Display for DatabaseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                DatabaseError::ConnectionFailed => write!(f, "数据库连接失败"),
                DatabaseError::QueryFailed(query) => write!(f, "查询失败: {}", query),
                DatabaseError::UserNotFound(id) => write!(f, "用户 {} 不存在", id),
                DatabaseError::DuplicateEntry => write!(f, "重复条目"),
            }
        }
    }

    impl Error for DatabaseError {}

    struct Database {
        users: HashMap<u32, User>,
    }

    impl Database {
        fn new() -> Self {
            let mut users = HashMap::new();
            users.insert(
                1,
                User {
                    id: 1,
                    name: "张三".to_string(),
                    email: "zhangsan@example.com".to_string(),
                },
            );

            Database { users }
        }

        fn find_user(&self, id: u32) -> Result<&User, DatabaseError> {
            self.users.get(&id).ok_or(DatabaseError::UserNotFound(id))
        }

        fn create_user(&mut self, user: User) -> Result<(), DatabaseError> {
            if self.users.contains_key(&user.id) {
                return Err(DatabaseError::DuplicateEntry);
            }

            self.users.insert(user.id, user);
            Ok(())
        }

        fn update_user(
            &mut self,
            id: u32,
            name: String,
            email: String,
        ) -> Result<(), DatabaseError> {
            match self.users.get_mut(&id) {
                Some(user) => {
                    user.name = name;
                    user.email = email;
                    Ok(())
                }
                None => Err(DatabaseError::UserNotFound(id)),
            }
        }
    }

    let mut db = Database::new();

    // 查找用户
    match db.find_user(1) {
        Ok(user) => println!("找到用户: {:?}", user),
        Err(e) => println!("查找用户失败: {}", e),
    }

    // 创建新用户
    let new_user = User {
        id: 2,
        name: "李四".to_string(),
        email: "lisi@example.com".to_string(),
    };

    match db.create_user(new_user) {
        Ok(()) => println!("用户创建成功"),
        Err(e) => println!("用户创建失败: {}", e),
    }

    // 更新用户
    match db.update_user(
        2,
        "李四更新".to_string(),
        "lisi_updated@example.com".to_string(),
    ) {
        Ok(()) => println!("用户更新成功"),
        Err(e) => println!("用户更新失败: {}", e),
    }

    // 8.5 配置管理系统
    println!("\n8.5 配置管理系统:");

    #[derive(Debug)]
    struct AppConfig {
        database_url: String,
        api_key: String,
        max_connections: u32,
        debug_mode: bool,
    }

    #[derive(Debug)]
    enum ConfigError {
        MissingField(String),
        InvalidValue(String, String),
        ParseError(String),
    }

    impl fmt::Display for ConfigError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ConfigError::MissingField(field) => write!(f, "缺少配置字段: {}", field),
                ConfigError::InvalidValue(field, value) => {
                    write!(f, "配置字段 {} 的值无效: {}", field, value)
                }
                ConfigError::ParseError(msg) => write!(f, "配置解析错误: {}", msg),
            }
        }
    }

    impl Error for ConfigError {}

    impl AppConfig {
        fn from_env() -> Result<Self, ConfigError> {
            // 模拟从环境变量读取配置
            let config_data = HashMap::from([
                (
                    "DATABASE_URL".to_string(),
                    "postgres://localhost/mydb".to_string(),
                ),
                ("API_KEY".to_string(), "secret123".to_string()),
                ("MAX_CONNECTIONS".to_string(), "10".to_string()),
                ("DEBUG_MODE".to_string(), "true".to_string()),
            ]);

            let database_url = config_data
                .get("DATABASE_URL")
                .ok_or_else(|| ConfigError::MissingField("DATABASE_URL".to_string()))?
                .clone();

            let api_key = config_data
                .get("API_KEY")
                .ok_or_else(|| ConfigError::MissingField("API_KEY".to_string()))?
                .clone();

            let max_connections = config_data
                .get("MAX_CONNECTIONS")
                .ok_or_else(|| ConfigError::MissingField("MAX_CONNECTIONS".to_string()))?
                .parse::<u32>()
                .map_err(|_| {
                    ConfigError::InvalidValue(
                        "MAX_CONNECTIONS".to_string(),
                        config_data.get("MAX_CONNECTIONS").unwrap().clone(),
                    )
                })?;

            let debug_mode = config_data
                .get("DEBUG_MODE")
                .ok_or_else(|| ConfigError::MissingField("DEBUG_MODE".to_string()))?
                .parse::<bool>()
                .map_err(|_| {
                    ConfigError::InvalidValue(
                        "DEBUG_MODE".to_string(),
                        config_data.get("DEBUG_MODE").unwrap().clone(),
                    )
                })?;

            Ok(AppConfig {
                database_url,
                api_key,
                max_connections,
                debug_mode,
            })
        }

        fn validate(&self) -> Result<(), ConfigError> {
            if self.database_url.is_empty() {
                return Err(ConfigError::InvalidValue(
                    "DATABASE_URL".to_string(),
                    "不能为空".to_string(),
                ));
            }

            if self.api_key.len() < 8 {
                return Err(ConfigError::InvalidValue(
                    "API_KEY".to_string(),
                    "长度不能少于8个字符".to_string(),
                ));
            }

            if self.max_connections == 0 || self.max_connections > 1000 {
                return Err(ConfigError::InvalidValue(
                    "MAX_CONNECTIONS".to_string(),
                    "必须在1-1000之间".to_string(),
                ));
            }

            Ok(())
        }
    }

    match AppConfig::from_env() {
        Ok(config) => match config.validate() {
            Ok(()) => println!("配置加载和验证成功: {:?}", config),
            Err(e) => println!("配置验证失败: {}", e),
        },
        Err(e) => println!("配置加载失败: {}", e),
    }
}
