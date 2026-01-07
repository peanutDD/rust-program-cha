//! # Rust 文档测试深度分析
//!
//! 本模块全面分析 Rust 的文档测试系统，这是 Rust 独有的强大功能，
//! 能够确保文档中的代码示例始终保持正确和最新。
//!
//! ## 文档测试核心概念
//!
//! 文档测试是 Rust 的一个独特功能，它可以：
//! - 运行文档注释中的代码示例
//! - 验证示例代码的正确性

// 示例代码模块，允许未定义的 cfg 条件（用于演示条件编译）
#![allow(unexpected_cfgs)]
//! - 确保文档与代码同步
//! - 提供额外的测试覆盖
//!
//! ## 测试类型分类
//!
//! ### 基本测试
//! ```rust
//! let x = 2 + 2;
//! assert_eq!(x, 4);
//! ```
//!
//! ### 隐藏行测试
//! ```rust
//! # use std::collections::HashMap;
//! # fn main() {
//! let mut map = HashMap::new();
//! map.insert("key", "value");
//! # }
//! ```
//!
//! ### Should Panic 测试
//! ```should_panic
//! panic!("这个测试应该 panic");
//! ```
//!
//! ### 编译失败测试
//! ```compile_fail
//! let x: i32 = "not a number"; // 应该编译失败
//! ```
//!
//! ### 忽略测试
//! ```ignore
//! some_undefined_function(); // 完全忽略
//! ```
//!
//! ### 不运行测试
//! ```no_run
//! std::process::exit(0); // 编译但不执行
//! ```
//!
//! ## 高级特性
//!
//! - 自定义测试属性
//! - 条件编译测试
//! - 跨平台测试
//! - 性能基准测试
//! - 集成测试支持

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::thread;
use std::time::Duration;

/// 演示文档测试功能的主函数
///
/// 展示 Rust 文档测试系统的各种功能和用法。
pub fn demonstrate_doc_tests() {
    println!("\n=== Rust 文档测试系统演示 ===");

    demonstrate_basic_doc_tests();
    demonstrate_hidden_lines();
    demonstrate_test_attributes();
    demonstrate_error_handling_tests();
    demonstrate_advanced_testing();
    demonstrate_test_organization();
    demonstrate_performance_tests();
    demonstrate_integration_tests();
}

/// 演示基本文档测试
///
/// 基本的文档测试是最常用的形式，直接在文档注释中编写可执行的代码。
///
/// # 基本语法
///
/// ```rust
/// // 这是一个基本的文档测试
/// let result = 2 + 2;
/// assert_eq!(result, 4);
/// ```
///
/// # 多行测试
///
/// ```rust
/// let mut vec = Vec::new();
/// vec.push(1);
/// vec.push(2);
/// vec.push(3);
///
/// assert_eq!(vec.len(), 3);
/// assert_eq!(vec[0], 1);
/// assert_eq!(vec[2], 3);
/// ```
///
/// # 使用外部 crate
///
/// ```rust
/// # use std::collections::HashMap;
/// let mut scores = HashMap::new();
/// scores.insert("Blue", 10);
/// scores.insert("Yellow", 50);
///
/// assert_eq!(scores.get("Blue"), Some(&10));
/// ```
fn demonstrate_basic_doc_tests() {
    println!("\n--- 基本文档测试演示 ---");

    let calculator = BasicCalculator::new();
    calculator.demonstrate_usage();

    println!("基本文档测试特点:");
    println!("  - 直接在文档中编写代码");
    println!("  - 自动编译和运行");
    println!("  - 验证代码正确性");
    println!("  - 提供使用示例");
}

/// 基本计算器
///
/// 演示基本文档测试的计算器实现。
///
/// # Examples
///
/// ## 基本运算
///
/// ```rust
/// # use comments_documentation::doc_tests::BasicCalculator;
/// let calc = BasicCalculator::new();
///
/// assert_eq!(calc.add(2, 3), 5);
/// assert_eq!(calc.subtract(10, 4), 6);
/// assert_eq!(calc.multiply(3, 4), 12);
/// assert_eq!(calc.divide(15, 3), Ok(5));
/// ```
///
/// ## 错误处理
///
/// ```rust
/// # use comments_documentation::doc_tests::BasicCalculator;
/// let calc = BasicCalculator::new();
///
/// // 除零错误
/// assert!(calc.divide(10, 0).is_err());
/// ```
///
/// ## 链式操作
///
/// ```rust
/// # use comments_documentation::doc_tests::BasicCalculator;
/// let calc = BasicCalculator::new();
///
/// let result = calc.add(calc.multiply(2, 3), calc.subtract(10, 5));
/// assert_eq!(result, 11); // (2 * 3) + (10 - 5) = 6 + 5 = 11
/// ```
#[derive(Debug, Clone)]
pub struct BasicCalculator {
    /// 计算历史
    history: Vec<String>,
}

impl BasicCalculator {
    /// 创建新的计算器实例
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::BasicCalculator;
    /// let calc = BasicCalculator::new();
    /// assert_eq!(calc.get_history().len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    /// 加法运算
    ///
    /// # Arguments
    ///
    /// * `a` - 第一个操作数
    /// * `b` - 第二个操作数
    ///
    /// # Returns
    ///
    /// 返回两数之和
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::BasicCalculator;
    /// let calc = BasicCalculator::new();
    /// assert_eq!(calc.add(5, 3), 8);
    /// assert_eq!(calc.add(-2, 7), 5);
    /// assert_eq!(calc.add(0, 0), 0);
    /// ```
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    /// 减法运算
    ///
    /// # Arguments
    ///
    /// * `a` - 被减数
    /// * `b` - 减数
    ///
    /// # Returns
    ///
    /// 返回两数之差
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::BasicCalculator;
    /// let calc = BasicCalculator::new();
    /// assert_eq!(calc.subtract(10, 3), 7);
    /// assert_eq!(calc.subtract(5, 8), -3);
    /// assert_eq!(calc.subtract(0, 0), 0);
    /// ```
    pub fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }

    /// 乘法运算
    ///
    /// # Arguments
    ///
    /// * `a` - 第一个因数
    /// * `b` - 第二个因数
    ///
    /// # Returns
    ///
    /// 返回两数之积
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::BasicCalculator;
    /// let calc = BasicCalculator::new();
    /// assert_eq!(calc.multiply(4, 5), 20);
    /// assert_eq!(calc.multiply(-3, 2), -6);
    /// assert_eq!(calc.multiply(0, 100), 0);
    /// ```
    pub fn multiply(&self, a: i32, b: i32) -> i32 {
        a * b
    }

    /// 除法运算
    ///
    /// # Arguments
    ///
    /// * `a` - 被除数
    /// * `b` - 除数
    ///
    /// # Returns
    ///
    /// 成功时返回商，除零时返回错误
    ///
    /// # Errors
    ///
    /// 当除数为零时返回 `DivisionByZeroError`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::BasicCalculator;
    /// let calc = BasicCalculator::new();
    ///
    /// // 正常除法
    /// assert_eq!(calc.divide(15, 3), Ok(5));
    /// assert_eq!(calc.divide(7, 2), Ok(3)); // 整数除法
    ///
    /// // 除零错误
    /// assert!(calc.divide(10, 0).is_err());
    /// ```
    pub fn divide(&self, a: i32, b: i32) -> Result<i32, DivisionByZeroError> {
        if b == 0 {
            Err(DivisionByZeroError)
        } else {
            Ok(a / b)
        }
    }

    /// 获取计算历史
    ///
    /// # Returns
    ///
    /// 返回计算历史记录的切片
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::BasicCalculator;
    /// let calc = BasicCalculator::new();
    /// assert_eq!(calc.get_history().len(), 0);
    /// ```
    pub fn get_history(&self) -> &[String] {
        &self.history
    }

    /// 演示计算器用法
    pub fn demonstrate_usage(&self) {
        println!("基本计算器演示:");
        println!("  - 加法: 2 + 3 = {}", self.add(2, 3));
        println!("  - 减法: 10 - 4 = {}", self.subtract(10, 4));
        println!("  - 乘法: 3 * 4 = {}", self.multiply(3, 4));

        match self.divide(15, 3) {
            Ok(result) => println!("  - 除法: 15 / 3 = {}", result),
            Err(e) => println!("  - 除法错误: {}", e),
        }

        match self.divide(10, 0) {
            Ok(result) => println!("  - 除法: 10 / 0 = {}", result),
            Err(e) => println!("  - 除法错误: {}", e),
        }
    }
}

/// 除零错误
///
/// 当尝试除以零时抛出的错误。
///
/// # Examples
///
/// ```rust
/// # use comments_documentation::doc_tests::{BasicCalculator, DivisionByZeroError};
/// let calc = BasicCalculator::new();
///
/// match calc.divide(10, 0) {
///     Ok(_) => panic!("应该返回错误"),
///     Err(DivisionByZeroError) => println!("正确捕获除零错误"),
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DivisionByZeroError;

impl Display for DivisionByZeroError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "不能除以零")
    }
}

impl Error for DivisionByZeroError {}

/// 演示隐藏行功能
///
/// 隐藏行允许在文档测试中包含设置代码，而不在文档中显示。
///
/// # 隐藏行语法
///
/// 以 `#` 开头的行在文档中不显示，但会在测试中执行：
///
/// ```rust
/// # use std::collections::HashMap;
/// # fn setup_data() -> HashMap<String, i32> {
/// #     let mut map = HashMap::new();
/// #     map.insert("apple".to_string(), 5);
/// #     map.insert("banana".to_string(), 3);
/// #     map
/// # }
/// let data = setup_data();
/// assert_eq!(data.get("apple"), Some(&5));
/// ```
///
/// # 隐藏导入
///
/// ```rust
/// # use std::collections::HashSet;
/// # use std::thread;
/// # use std::time::Duration;
/// let mut set = HashSet::new();
/// set.insert("item1");
/// set.insert("item2");
///
/// assert_eq!(set.len(), 2);
/// assert!(set.contains("item1"));
/// ```
fn demonstrate_hidden_lines() {
    println!("\n--- 隐藏行功能演示 ---");

    let manager = HiddenLineManager::new();
    manager.demonstrate_hidden_functionality();

    println!("隐藏行功能特点:");
    println!("  - 隐藏设置代码");
    println!("  - 简化文档显示");
    println!("  - 保持测试完整性");
    println!("  - 支持复杂初始化");
}

/// 隐藏行管理器
///
/// 演示如何使用隐藏行来简化文档测试的显示。
///
/// # Examples
///
/// ## 基本用法
///
/// ```rust
/// # use comments_documentation::doc_tests::HiddenLineManager;
/// # let manager = HiddenLineManager::new();
/// let result = manager.process_data("test data");
/// assert!(result.contains("processed"));
/// ```
///
/// ## 复杂初始化
///
/// ```rust
/// # use comments_documentation::doc_tests::HiddenLineManager;
/// # use std::collections::HashMap;
/// #
/// # fn create_test_data() -> HashMap<String, String> {
/// #     let mut data = HashMap::new();
/// #     data.insert("key1".to_string(), "value1".to_string());
/// #     data.insert("key2".to_string(), "value2".to_string());
/// #     data.insert("key3".to_string(), "value3".to_string());
/// #     data
/// # }
/// #
/// # let manager = HiddenLineManager::new();
/// let test_data = create_test_data();
/// let results = manager.batch_process(&test_data);
///
/// assert_eq!(results.len(), 3);
/// assert!(results.contains_key("key1"));
/// ```
#[derive(Debug)]
pub struct HiddenLineManager {
    /// 内部状态
    state: HashMap<String, String>,
}

impl HiddenLineManager {
    /// 创建新的管理器
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::HiddenLineManager;
    /// let manager = HiddenLineManager::new();
    /// // 管理器已准备就绪
    /// ```
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    /// 处理单个数据项
    ///
    /// # Arguments
    ///
    /// * `data` - 要处理的数据
    ///
    /// # Returns
    ///
    /// 返回处理后的数据字符串
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::HiddenLineManager;
    /// let manager = HiddenLineManager::new();
    /// let result = manager.process_data("input");
    /// assert!(result.starts_with("processed:"));
    /// ```
    pub fn process_data(&self, data: &str) -> String {
        format!("processed: {}", data)
    }

    /// 批量处理数据
    ///
    /// # Arguments
    ///
    /// * `data` - 要处理的数据映射
    ///
    /// # Returns
    ///
    /// 返回处理结果的映射
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::HiddenLineManager;
    /// # use std::collections::HashMap;
    /// #
    /// # let manager = HiddenLineManager::new();
    /// # let mut input = HashMap::new();
    /// # input.insert("test".to_string(), "data".to_string());
    /// let results = manager.batch_process(&input);
    /// assert_eq!(results.len(), 1);
    /// ```
    pub fn batch_process(&self, data: &HashMap<String, String>) -> HashMap<String, String> {
        data.iter()
            .map(|(k, v)| (k.clone(), self.process_data(v)))
            .collect()
    }

    /// 演示隐藏功能
    pub fn demonstrate_hidden_functionality(&self) {
        println!("隐藏行管理器演示:");
        println!("  - 处理数据: {}", self.process_data("sample"));

        let mut test_data = HashMap::new();
        test_data.insert("item1".to_string(), "value1".to_string());
        test_data.insert("item2".to_string(), "value2".to_string());

        let results = self.batch_process(&test_data);
        println!("  - 批量处理: {} 项结果", results.len());
    }
}

/// 演示测试属性
///
/// Rust 文档测试支持多种特殊属性来控制测试行为。
fn demonstrate_test_attributes() {
    println!("\n--- 测试属性演示 ---");

    let tester = AttributeTester::new();
    tester.demonstrate_attributes();

    println!("测试属性类型:");
    println!("  - should_panic: 期望 panic 的测试");
    println!("  - ignore: 忽略的测试");
    println!("  - no_run: 编译但不运行");
    println!("  - compile_fail: 编译失败测试");
    println!("  - edition: 指定 Rust 版本");
}

/// 属性测试器
///
/// 演示各种文档测试属性的使用。
///
/// # Should Panic 测试
///
/// 用于测试应该 panic 的代码：
///
/// ```should_panic
/// # use comments_documentation::doc_tests::AttributeTester;
/// let tester = AttributeTester::new();
/// tester.panic_method(); // 这会 panic
/// ```
///
/// # 忽略测试
///
/// 某些测试可能需要特殊环境或很耗时：
///
/// ```ignore
/// # use comments_documentation::doc_tests::AttributeTester;
/// let tester = AttributeTester::new();
/// tester.expensive_operation(); // 这个测试被忽略
/// ```
///
/// # 不运行测试
///
/// 编译检查但不实际执行：
///
/// ```no_run
/// # use comments_documentation::doc_tests::AttributeTester;
/// let tester = AttributeTester::new();
/// tester.system_exit(); // 编译但不运行
/// ```
///
/// # 编译失败测试
///
/// 验证某些代码应该编译失败：
///
/// ```compile_fail
/// # use comments_documentation::doc_tests::AttributeTester;
/// let tester = AttributeTester::new();
/// let invalid: i32 = "not a number"; // 应该编译失败
/// ```
#[derive(Debug)]
pub struct AttributeTester {
    /// 测试数据
    data: Vec<i32>,
}

impl AttributeTester {
    /// 创建新的测试器
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::AttributeTester;
    /// let tester = AttributeTester::new();
    /// assert_eq!(tester.get_data().len(), 0);
    /// ```
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// 会 panic 的方法
    ///
    /// # Panics
    ///
    /// 这个方法总是会 panic
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// # use comments_documentation::doc_tests::AttributeTester;
    /// let tester = AttributeTester::new();
    /// tester.panic_method(); // 这会 panic
    /// ```
    pub fn panic_method(&self) {
        panic!("这是一个故意的 panic");
    }

    /// 昂贵的操作
    ///
    /// 这个方法模拟一个耗时的操作。
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use comments_documentation::doc_tests::AttributeTester;
    /// let tester = AttributeTester::new();
    /// tester.expensive_operation(); // 这个测试被忽略
    /// ```
    pub fn expensive_operation(&self) {
        // 模拟耗时操作
        thread::sleep(Duration::from_secs(10));
    }

    /// 系统退出方法
    ///
    /// 这个方法会退出程序。
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use comments_documentation::doc_tests::AttributeTester;
    /// let tester = AttributeTester::new();
    /// tester.system_exit(); // 编译但不运行
    /// ```
    pub fn system_exit(&self) {
        std::process::exit(0);
    }

    /// 获取测试数据
    ///
    /// # Returns
    ///
    /// 返回内部数据的切片
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::AttributeTester;
    /// let tester = AttributeTester::new();
    /// assert_eq!(tester.get_data().len(), 0);
    /// ```
    pub fn get_data(&self) -> &[i32] {
        &self.data
    }

    /// 演示属性功能
    pub fn demonstrate_attributes(&self) {
        println!("属性测试器演示:");
        println!("  - 正常方法调用成功");
        println!("  - Panic 方法: 在测试中会被捕获");
        println!("  - 昂贵操作: 在实际测试中被忽略");
        println!("  - 系统退出: 只编译不运行");
    }
}

/// 演示错误处理测试
///
/// 文档测试中的错误处理是确保 API 健壮性的重要部分。
fn demonstrate_error_handling_tests() {
    println!("\n--- 错误处理测试演示 ---");

    let handler = ErrorHandler::new();
    handler.demonstrate_error_handling();

    println!("错误处理测试特点:");
    println!("  - 验证错误情况");
    println!("  - 测试错误类型");
    println!("  - 确保错误消息");
    println!("  - 验证恢复机制");
}

/// 错误处理器
///
/// 演示如何在文档测试中处理各种错误情况。
///
/// # Error Handling Examples
///
/// ## 基本错误处理
///
/// ```rust
/// # use comments_documentation::doc_tests::ErrorHandler;
/// let handler = ErrorHandler::new();
///
/// match handler.risky_operation("valid") {
///     Ok(result) => println!("成功: {}", result),
///     Err(e) => println!("错误: {}", e),
/// }
/// ```
///
/// ## 特定错误类型
///
/// ```rust
/// # use comments_documentation::doc_tests::{ErrorHandler, ProcessingError};
/// let handler = ErrorHandler::new();
///
/// match handler.risky_operation("") {
///     Ok(_) => panic!("应该返回错误"),
///     Err(ProcessingError::InvalidInput) => println!("正确捕获输入错误"),
///     Err(e) => panic!("意外的错误类型: {:?}", e),
/// }
/// ```
///
/// ## 错误链
///
/// ```rust
/// # use comments_documentation::doc_tests::ErrorHandler;
/// let handler = ErrorHandler::new();
///
/// if let Err(e) = handler.complex_operation("invalid") {
///     println!("主错误: {}", e);
///     
///     let mut source = e.source();
///     while let Some(err) = source {
///         println!("原因: {}", err);
///         source = err.source();
///     }
/// }
/// ```
#[derive(Debug)]
pub struct ErrorHandler {
    /// 处理状态
    state: HashMap<String, bool>,
}

impl ErrorHandler {
    /// 创建新的错误处理器
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::ErrorHandler;
    /// let handler = ErrorHandler::new();
    /// // 处理器已准备就绪
    /// ```
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    /// 风险操作
    ///
    /// 可能失败的操作，用于演示错误处理。
    ///
    /// # Arguments
    ///
    /// * `input` - 输入数据
    ///
    /// # Returns
    ///
    /// 成功时返回处理结果，失败时返回错误
    ///
    /// # Errors
    ///
    /// - `InvalidInput`: 输入为空或无效
    /// - `ProcessingFailed`: 处理过程中出错
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::ErrorHandler;
    /// let handler = ErrorHandler::new();
    ///
    /// // 成功情况
    /// assert!(handler.risky_operation("valid input").is_ok());
    ///
    /// // 失败情况
    /// assert!(handler.risky_operation("").is_err());
    /// ```
    pub fn risky_operation(&self, input: &str) -> Result<String, ProcessingError> {
        if input.is_empty() {
            return Err(ProcessingError::InvalidInput);
        }

        if input.contains("invalid") {
            return Err(ProcessingError::ProcessingFailed(
                "包含无效字符".to_string(),
            ));
        }

        Ok(format!("处理结果: {}", input))
    }

    /// 复杂操作
    ///
    /// 可能产生错误链的复杂操作。
    ///
    /// # Arguments
    ///
    /// * `input` - 输入数据
    ///
    /// # Returns
    ///
    /// 成功时返回处理结果，失败时返回错误
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::ErrorHandler;
    /// let handler = ErrorHandler::new();
    ///
    /// match handler.complex_operation("test") {
    ///     Ok(result) => println!("成功: {}", result),
    ///     Err(e) => {
    ///         println!("错误: {}", e);
    ///         // 可以检查错误源
    ///     }
    /// }
    /// ```
    pub fn complex_operation(&self, input: &str) -> Result<String, ProcessingError> {
        // 首先进行基本验证
        self.risky_operation(input)?;

        // 然后进行复杂处理
        if input.len() > 100 {
            return Err(ProcessingError::ProcessingFailed("输入过长".to_string()));
        }

        Ok(format!("复杂处理结果: {}", input))
    }

    /// 演示错误处理
    pub fn demonstrate_error_handling(&self) {
        println!("错误处理器演示:");

        // 成功情况
        match self.risky_operation("valid input") {
            Ok(result) => println!("  - 成功: {}", result),
            Err(e) => println!("  - 错误: {}", e),
        }

        // 失败情况
        match self.risky_operation("") {
            Ok(_) => println!("  - 意外成功"),
            Err(e) => println!("  - 预期错误: {}", e),
        }

        // 复杂操作
        match self.complex_operation("test") {
            Ok(result) => println!("  - 复杂操作成功: {}", result),
            Err(e) => println!("  - 复杂操作错误: {}", e),
        }
    }
}

/// 处理错误类型
///
/// 演示自定义错误类型在文档测试中的使用。
///
/// # Examples
///
/// ```rust
/// # use comments_documentation::doc_tests::ProcessingError;
/// let error = ProcessingError::InvalidInput;
/// assert_eq!(error.to_string(), "输入无效");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessingError {
    /// 输入无效
    InvalidInput,
    /// 处理失败
    ProcessingFailed(String),
    /// 网络错误
    NetworkError,
    /// 超时错误
    Timeout,
}

impl Display for ProcessingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ProcessingError::InvalidInput => write!(f, "输入无效"),
            ProcessingError::ProcessingFailed(msg) => write!(f, "处理失败: {}", msg),
            ProcessingError::NetworkError => write!(f, "网络错误"),
            ProcessingError::Timeout => write!(f, "操作超时"),
        }
    }
}

impl Error for ProcessingError {}

/// 演示高级测试功能
///
/// 包括条件编译、平台特定测试等高级功能。
fn demonstrate_advanced_testing() {
    println!("\n--- 高级测试功能演示 ---");

    let advanced = AdvancedTester::new();
    advanced.demonstrate_advanced_features();

    println!("高级测试功能:");
    println!("  - 条件编译测试");
    println!("  - 平台特定测试");
    println!("  - 功能标志测试");
    println!("  - 版本特定测试");
}

/// 高级测试器
///
/// 演示高级文档测试功能。
///
/// # 条件编译测试
///
/// ```rust
/// # #[cfg(feature = "advanced")]
/// # use comments_documentation::doc_tests::AdvancedTester;
/// #
/// # #[cfg(feature = "advanced")]
/// # {
/// let tester = AdvancedTester::new();
/// let result = tester.advanced_feature();
/// assert!(result.contains("advanced"));
/// # }
/// ```
///
/// # 平台特定测试
///
/// ```rust
/// # use comments_documentation::doc_tests::AdvancedTester;
/// let tester = AdvancedTester::new();
///
/// #[cfg(target_os = "linux")]
/// {
///     let linux_result = tester.linux_specific();
///     assert!(linux_result.contains("linux"));
/// }
///
/// #[cfg(target_os = "windows")]
/// {
///     let windows_result = tester.windows_specific();
///     assert!(windows_result.contains("windows"));
/// }
/// ```
#[derive(Debug)]
pub struct AdvancedTester {
    /// 配置选项
    config: HashMap<String, bool>,
}

impl AdvancedTester {
    /// 创建新的高级测试器
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::AdvancedTester;
    /// let tester = AdvancedTester::new();
    /// // 测试器已准备就绪
    /// ```
    pub fn new() -> Self {
        let mut config = HashMap::new();
        config.insert("advanced_mode".to_string(), true);
        config.insert("debug_mode".to_string(), false);

        Self { config }
    }

    /// 高级功能
    ///
    /// 只在启用高级功能时可用。
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::AdvancedTester;
    /// let tester = AdvancedTester::new();
    /// let result = tester.advanced_feature();
    /// assert!(result.contains("advanced"));
    /// ```
    #[cfg(feature = "advanced")]
    pub fn advanced_feature(&self) -> String {
        "advanced functionality enabled".to_string()
    }

    /// 高级功能（默认实现）
    ///
    /// 当高级功能未启用时的默认实现。
    #[cfg(not(feature = "advanced"))]
    pub fn advanced_feature(&self) -> String {
        "advanced functionality disabled".to_string()
    }

    /// Linux 特定功能
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::AdvancedTester;
    /// let tester = AdvancedTester::new();
    ///
    /// #[cfg(target_os = "linux")]
    /// {
    ///     let result = tester.linux_specific();
    ///     assert!(result.contains("linux"));
    /// }
    /// ```
    #[cfg(target_os = "linux")]
    pub fn linux_specific(&self) -> String {
        "linux specific functionality".to_string()
    }

    /// Windows 特定功能
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::AdvancedTester;
    /// let tester = AdvancedTester::new();
    ///
    /// #[cfg(target_os = "windows")]
    /// {
    ///     let result = tester.windows_specific();
    ///     assert!(result.contains("windows"));
    /// }
    /// ```
    #[cfg(target_os = "windows")]
    pub fn windows_specific(&self) -> String {
        "windows specific functionality".to_string()
    }

    /// 通用平台功能
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    pub fn platform_specific(&self) -> String {
        "generic platform functionality".to_string()
    }

    /// 演示高级功能
    pub fn demonstrate_advanced_features(&self) {
        println!("高级测试器演示:");
        println!("  - 高级功能: {}", self.advanced_feature());

        #[cfg(target_os = "linux")]
        println!("  - Linux 功能: {}", self.linux_specific());

        #[cfg(target_os = "windows")]
        println!("  - Windows 功能: {}", self.windows_specific());

        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        println!("  - 通用功能: {}", self.platform_specific());
    }
}

/// 演示测试组织
///
/// 如何组织和结构化文档测试。
fn demonstrate_test_organization() {
    println!("\n--- 测试组织演示 ---");

    let organizer = TestOrganizer::new();
    organizer.demonstrate_organization();

    println!("测试组织方法:");
    println!("  - 按功能分组");
    println!("  - 按复杂度分层");
    println!("  - 按场景分类");
    println!("  - 集成测试支持");
}

/// 测试组织器
///
/// 演示如何有效组织文档测试。
///
/// # 基础测试组
///
/// ## 创建和初始化
///
/// ```rust
/// # use comments_documentation::doc_tests::TestOrganizer;
/// let organizer = TestOrganizer::new();
/// assert_eq!(organizer.get_test_count(), 0);
/// ```
///
/// ## 添加测试
///
/// ```rust
/// # use comments_documentation::doc_tests::TestOrganizer;
/// let mut organizer = TestOrganizer::new();
/// organizer.add_test("test1");
/// organizer.add_test("test2");
/// assert_eq!(organizer.get_test_count(), 2);
/// ```
///
/// # 中级测试组
///
/// ## 批量操作
///
/// ```rust
/// # use comments_documentation::doc_tests::TestOrganizer;
/// let mut organizer = TestOrganizer::new();
/// let tests = vec!["test1", "test2", "test3"];
/// organizer.add_batch(&tests);
/// assert_eq!(organizer.get_test_count(), 3);
/// ```
///
/// ## 测试过滤
///
/// ```rust
/// # use comments_documentation::doc_tests::TestOrganizer;
/// let mut organizer = TestOrganizer::new();
/// organizer.add_test("unit_test_1");
/// organizer.add_test("integration_test_1");
/// organizer.add_test("unit_test_2");
///
/// let unit_tests = organizer.filter_tests("unit");
/// assert_eq!(unit_tests.len(), 2);
/// ```
///
/// # 高级测试组
///
/// ## 复杂场景测试
///
/// ```rust
/// # use comments_documentation::doc_tests::TestOrganizer;
/// # use std::collections::HashMap;
/// let mut organizer = TestOrganizer::new();
///
/// // 设置复杂测试环境
/// let mut test_data = HashMap::new();
/// test_data.insert("config".to_string(), "test_config".to_string());
/// test_data.insert("data".to_string(), "test_data".to_string());
///
/// organizer.setup_environment(&test_data);
/// organizer.add_test("complex_scenario_test");
///
/// let results = organizer.run_tests();
/// assert!(results.contains_key("complex_scenario_test"));
/// ```
#[derive(Debug)]
pub struct TestOrganizer {
    /// 测试列表
    tests: Vec<String>,
    /// 测试环境
    environment: HashMap<String, String>,
    /// 测试结果
    results: HashMap<String, bool>,
}

impl TestOrganizer {
    /// 创建新的测试组织器
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::TestOrganizer;
    /// let organizer = TestOrganizer::new();
    /// assert_eq!(organizer.get_test_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            tests: Vec::new(),
            environment: HashMap::new(),
            results: HashMap::new(),
        }
    }

    /// 添加单个测试
    ///
    /// # Arguments
    ///
    /// * `test_name` - 测试名称
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::TestOrganizer;
    /// let mut organizer = TestOrganizer::new();
    /// organizer.add_test("my_test");
    /// assert_eq!(organizer.get_test_count(), 1);
    /// ```
    pub fn add_test(&mut self, test_name: &str) {
        self.tests.push(test_name.to_string());
    }

    /// 批量添加测试
    ///
    /// # Arguments
    ///
    /// * `test_names` - 测试名称列表
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::TestOrganizer;
    /// let mut organizer = TestOrganizer::new();
    /// let tests = vec!["test1", "test2", "test3"];
    /// organizer.add_batch(&tests);
    /// assert_eq!(organizer.get_test_count(), 3);
    /// ```
    pub fn add_batch(&mut self, test_names: &[&str]) {
        for name in test_names {
            self.add_test(name);
        }
    }

    /// 获取测试数量
    ///
    /// # Returns
    ///
    /// 返回当前测试的数量
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::TestOrganizer;
    /// let organizer = TestOrganizer::new();
    /// assert_eq!(organizer.get_test_count(), 0);
    /// ```
    pub fn get_test_count(&self) -> usize {
        self.tests.len()
    }

    /// 过滤测试
    ///
    /// # Arguments
    ///
    /// * `filter` - 过滤条件
    ///
    /// # Returns
    ///
    /// 返回匹配的测试列表
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::TestOrganizer;
    /// let mut organizer = TestOrganizer::new();
    /// organizer.add_test("unit_test_1");
    /// organizer.add_test("integration_test_1");
    ///
    /// let unit_tests = organizer.filter_tests("unit");
    /// assert_eq!(unit_tests.len(), 1);
    /// ```
    pub fn filter_tests(&self, filter: &str) -> Vec<String> {
        self.tests
            .iter()
            .filter(|test| test.contains(filter))
            .cloned()
            .collect()
    }

    /// 设置测试环境
    ///
    /// # Arguments
    ///
    /// * `env` - 环境配置
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::TestOrganizer;
    /// # use std::collections::HashMap;
    /// let mut organizer = TestOrganizer::new();
    /// let mut env = HashMap::new();
    /// env.insert("debug".to_string(), "true".to_string());
    /// organizer.setup_environment(&env);
    /// ```
    pub fn setup_environment(&mut self, env: &HashMap<String, String>) {
        self.environment = env.clone();
    }

    /// 运行测试
    ///
    /// # Returns
    ///
    /// 返回测试结果映射
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::TestOrganizer;
    /// let mut organizer = TestOrganizer::new();
    /// organizer.add_test("test1");
    /// let results = organizer.run_tests();
    /// assert!(results.contains_key("test1"));
    /// ```
    pub fn run_tests(&mut self) -> &HashMap<String, bool> {
        for test in &self.tests {
            // 模拟测试运行
            self.results.insert(test.clone(), true);
        }
        &self.results
    }

    /// 演示组织功能
    pub fn demonstrate_organization(&self) {
        println!("测试组织器演示:");
        println!("  - 当前测试数量: {}", self.get_test_count());
        println!("  - 环境配置: {} 项", self.environment.len());
        println!("  - 支持测试过滤和批量操作");
    }
}

/// 演示性能测试
///
/// 文档测试也可以用于性能验证和基准测试。
fn demonstrate_performance_tests() {
    println!("\n--- 性能测试演示 ---");

    let perf_tester = PerformanceTester::new();
    perf_tester.demonstrate_performance_testing();

    println!("性能测试特点:");
    println!("  - 基准测试");
    println!("  - 性能回归检测");
    println!("  - 内存使用验证");
    println!("  - 并发性能测试");
}

/// 性能测试器
///
/// 演示如何在文档测试中进行性能验证。
///
/// # 基本性能测试
///
/// ```rust
/// # use comments_documentation::doc_tests::PerformanceTester;
/// # use std::time::Instant;
/// let tester = PerformanceTester::new();
///
/// let start = Instant::now();
/// let result = tester.fast_operation(1000);
/// let duration = start.elapsed();
///
/// assert!(result > 0);
/// // 验证操作在合理时间内完成
/// assert!(duration.as_millis() < 100);
/// ```
///
/// # 内存使用测试
///
/// ```rust
/// # use comments_documentation::doc_tests::PerformanceTester;
/// let tester = PerformanceTester::new();
///
/// // 测试内存效率
/// let data = tester.memory_efficient_operation(10000);
/// assert_eq!(data.len(), 10000);
///
/// // 验证没有内存泄漏（通过 drop 验证）
/// drop(data);
/// ```
///
/// # 并发性能测试
///
/// ```rust
/// # use comments_documentation::doc_tests::PerformanceTester;
/// # use std::thread;
/// # use std::sync::Arc;
/// let tester = Arc::new(PerformanceTester::new());
///
/// let handles: Vec<_> = (0..4)
///     .map(|i| {
///         let tester = Arc::clone(&tester);
///         thread::spawn(move || {
///             tester.thread_safe_operation(i * 1000)
///         })
///     })
///     .collect();
///
/// for handle in handles {
///     let result = handle.join().unwrap();
///     assert!(result > 0);
/// }
/// ```
#[derive(Debug)]
pub struct PerformanceTester {
    /// 缓存数据
    cache: HashMap<usize, Vec<i32>>,
}

impl PerformanceTester {
    /// 创建新的性能测试器
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::PerformanceTester;
    /// let tester = PerformanceTester::new();
    /// // 测试器已准备就绪
    /// ```
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// 快速操作
    ///
    /// 应该在短时间内完成的操作。
    ///
    /// # Arguments
    ///
    /// * `n` - 操作参数
    ///
    /// # Returns
    ///
    /// 返回计算结果
    ///
    /// # Performance
    ///
    /// 时间复杂度: O(1)  
    /// 空间复杂度: O(1)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::PerformanceTester;
    /// # use std::time::Instant;
    /// let tester = PerformanceTester::new();
    ///
    /// let start = Instant::now();
    /// let result = tester.fast_operation(1000);
    /// let duration = start.elapsed();
    ///
    /// assert!(result > 0);
    /// assert!(duration.as_millis() < 10); // 应该很快完成
    /// ```
    pub fn fast_operation(&self, n: usize) -> i32 {
        (n * 2) as i32
    }

    /// 内存高效操作
    ///
    /// 创建大量数据但保持内存效率。
    ///
    /// # Arguments
    ///
    /// * `size` - 数据大小
    ///
    /// # Returns
    ///
    /// 返回生成的数据向量
    ///
    /// # Performance
    ///
    /// 时间复杂度: O(n)  
    /// 空间复杂度: O(n)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::PerformanceTester;
    /// let tester = PerformanceTester::new();
    ///
    /// let data = tester.memory_efficient_operation(1000);
    /// assert_eq!(data.len(), 1000);
    ///
    /// // 验证数据正确性
    /// assert_eq!(data[0], 0);
    /// assert_eq!(data[999], 999);
    /// ```
    pub fn memory_efficient_operation(&self, size: usize) -> Vec<i32> {
        (0..size).map(|i| i as i32).collect()
    }

    /// 线程安全操作
    ///
    /// 可以在多线程环境中安全调用的操作。
    ///
    /// # Arguments
    ///
    /// * `value` - 输入值
    ///
    /// # Returns
    ///
    /// 返回处理结果
    ///
    /// # Thread Safety
    ///
    /// 这个方法是线程安全的，可以并发调用。
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::PerformanceTester;
    /// let tester = PerformanceTester::new();
    ///
    /// let result = tester.thread_safe_operation(42);
    /// assert_eq!(result, 84); // 42 * 2
    /// ```
    pub fn thread_safe_operation(&self, value: i32) -> i32 {
        value * 2
    }

    /// 演示性能测试
    pub fn demonstrate_performance_testing(&self) {
        println!("性能测试器演示:");

        let start = std::time::Instant::now();
        let result = self.fast_operation(1000);
        let duration = start.elapsed();

        println!("  - 快速操作: {} (耗时: {:?})", result, duration);

        let data = self.memory_efficient_operation(100);
        println!("  - 内存操作: 生成 {} 项数据", data.len());

        let thread_result = self.thread_safe_operation(42);
        println!("  - 线程安全操作: {}", thread_result);
    }
}

/// 演示集成测试
///
/// 文档测试可以作为集成测试的一部分。
fn demonstrate_integration_tests() {
    println!("\n--- 集成测试演示 ---");

    let integration = IntegrationTester::new();
    integration.demonstrate_integration();

    println!("集成测试特点:");
    println!("  - 端到端测试");
    println!("  - 模块间交互");
    println!("  - 真实场景模拟");
    println!("  - 系统级验证");
}

/// 集成测试器
///
/// 演示如何使用文档测试进行集成测试。
///
/// # 完整工作流测试
///
/// ```rust
/// # use comments_documentation::doc_tests::IntegrationTester;
/// let mut tester = IntegrationTester::new();
///
/// // 完整的工作流程
/// tester.initialize();
/// tester.load_data("test_data");
/// tester.process();
/// let results = tester.get_results();
///
/// assert!(!results.is_empty());
/// assert!(results.contains("processed: test_data"));
/// ```
///
/// # 错误恢复测试
///
/// ```rust
/// # use comments_documentation::doc_tests::IntegrationTester;
/// let mut tester = IntegrationTester::new();
///
/// // 测试错误恢复
/// tester.initialize();
/// tester.load_data("invalid_data");
///
/// match tester.process() {
///     Ok(_) => panic!("应该失败"),
///     Err(_) => {
///         // 错误恢复
///         tester.reset();
///         assert!(tester.is_clean());
///     }
/// }
/// ```
///
/// # 模块交互测试
///
/// ```rust
/// # use comments_documentation::doc_tests::{IntegrationTester, DataProcessor};
/// let mut tester = IntegrationTester::new();
/// let processor = DataProcessor::new();
///
/// // 测试模块间交互
/// tester.initialize();
/// tester.load_data("test_data");
///
/// let raw_data = tester.get_raw_data();
/// let processed = processor.process(raw_data);
///
/// tester.store_results(&processed);
/// let results = tester.get_results();
///
/// assert!(!results.is_empty());
/// ```
#[derive(Debug)]
pub struct IntegrationTester {
    /// 系统状态
    state: SystemState,
    /// 数据存储
    data: Vec<String>,
    /// 处理结果
    results: Vec<String>,
    /// 是否已初始化
    initialized: bool,
}

/// 系统状态枚举
#[derive(Debug, Clone, PartialEq)]
enum SystemState {
    /// 未初始化
    Uninitialized,
    /// 已初始化
    Initialized,
    /// 数据已加载
    DataLoaded,
    /// 处理中
    Processing,
    /// 已完成
    Completed,
    /// 错误状态
    Error(String),
}

impl IntegrationTester {
    /// 创建新的集成测试器
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::IntegrationTester;
    /// let tester = IntegrationTester::new();
    /// assert!(!tester.is_initialized());
    /// ```
    pub fn new() -> Self {
        Self {
            state: SystemState::Uninitialized,
            data: Vec::new(),
            results: Vec::new(),
            initialized: false,
        }
    }

    /// 初始化系统
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::IntegrationTester;
    /// let mut tester = IntegrationTester::new();
    /// tester.initialize();
    /// assert!(tester.is_initialized());
    /// ```
    pub fn initialize(&mut self) {
        self.state = SystemState::Initialized;
        self.initialized = true;
        self.data.clear();
        self.results.clear();
    }

    /// 加载数据
    ///
    /// # Arguments
    ///
    /// * `data` - 要加载的数据
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::IntegrationTester;
    /// let mut tester = IntegrationTester::new();
    /// tester.initialize();
    /// tester.load_data("test_data");
    /// ```
    pub fn load_data(&mut self, data: &str) {
        if self.initialized {
            self.data.push(data.to_string());
            self.state = SystemState::DataLoaded;
        }
    }

    /// 处理数据
    ///
    /// # Returns
    ///
    /// 成功时返回 Ok(())，失败时返回错误
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::IntegrationTester;
    /// let mut tester = IntegrationTester::new();
    /// tester.initialize();
    /// tester.load_data("valid_data");
    /// assert!(tester.process().is_ok());
    /// ```
    pub fn process(&mut self) -> Result<(), String> {
        match self.state {
            SystemState::DataLoaded => {
                self.state = SystemState::Processing;

                for data in &self.data {
                    if data.contains("invalid") {
                        self.state = SystemState::Error("Invalid data detected".to_string());
                        return Err("Processing failed".to_string());
                    }

                    self.results.push(format!("processed: {}", data));
                }

                self.state = SystemState::Completed;
                Ok(())
            }
            _ => Err("Invalid state for processing".to_string()),
        }
    }

    /// 获取处理结果
    ///
    /// # Returns
    ///
    /// 返回处理结果的切片
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::IntegrationTester;
    /// let mut tester = IntegrationTester::new();
    /// tester.initialize();
    /// tester.load_data("test");
    /// let _ = tester.process();
    /// let results = tester.get_results();
    /// assert!(!results.is_empty());
    /// ```
    pub fn get_results(&self) -> &[String] {
        &self.results
    }

    /// 获取原始数据
    ///
    /// # Returns
    ///
    /// 返回原始数据的切片
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::IntegrationTester;
    /// let mut tester = IntegrationTester::new();
    /// tester.initialize();
    /// tester.load_data("test");
    /// let raw_data = tester.get_raw_data();
    /// assert_eq!(raw_data.len(), 1);
    /// ```
    pub fn get_raw_data(&self) -> &[String] {
        &self.data
    }

    /// 存储处理结果
    ///
    /// # Arguments
    ///
    /// * `results` - 要存储的结果
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::IntegrationTester;
    /// let mut tester = IntegrationTester::new();
    /// tester.initialize();
    /// let results = vec!["result1".to_string(), "result2".to_string()];
    /// tester.store_results(&results);
    /// ```
    pub fn store_results(&mut self, results: &[String]) {
        self.results.extend_from_slice(results);
    }

    /// 重置系统
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::IntegrationTester;
    /// let mut tester = IntegrationTester::new();
    /// tester.initialize();
    /// tester.load_data("test");
    /// tester.reset();
    /// assert!(tester.is_clean());
    /// ```
    pub fn reset(&mut self) {
        self.state = SystemState::Uninitialized;
        self.initialized = false;
        self.data.clear();
        self.results.clear();
    }

    /// 检查是否已初始化
    ///
    /// # Returns
    ///
    /// 如果已初始化返回 true，否则返回 false
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::IntegrationTester;
    /// let mut tester = IntegrationTester::new();
    /// assert!(!tester.is_initialized());
    /// tester.initialize();
    /// assert!(tester.is_initialized());
    /// ```
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// 检查系统是否干净
    ///
    /// # Returns
    ///
    /// 如果系统状态干净返回 true，否则返回 false
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::IntegrationTester;
    /// let tester = IntegrationTester::new();
    /// assert!(tester.is_clean());
    /// ```
    pub fn is_clean(&self) -> bool {
        matches!(self.state, SystemState::Uninitialized)
            && self.data.is_empty()
            && self.results.is_empty()
    }

    /// 演示集成功能
    pub fn demonstrate_integration(&self) {
        println!("集成测试器演示:");
        println!("  - 当前状态: {:?}", self.state);
        println!("  - 数据项数: {}", self.data.len());
        println!("  - 结果项数: {}", self.results.len());
        println!("  - 已初始化: {}", self.initialized);
    }
}

/// 数据处理器
///
/// 用于集成测试中的数据处理。
///
/// # Examples
///
/// ```rust
/// # use comments_documentation::doc_tests::DataProcessor;
/// let processor = DataProcessor::new();
/// let result = processor.process(&["data1".to_string()]);
/// assert_eq!(result.len(), 1);
/// ```
#[derive(Debug)]
pub struct DataProcessor {
    /// 处理配置
    config: HashMap<String, String>,
}

impl DataProcessor {
    /// 创建新的数据处理器
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::DataProcessor;
    /// let processor = DataProcessor::new();
    /// ```
    pub fn new() -> Self {
        let mut config = HashMap::new();
        config.insert("mode".to_string(), "standard".to_string());
        config.insert("encoding".to_string(), "utf8".to_string());

        Self { config }
    }

    /// 处理数据
    ///
    /// # Arguments
    ///
    /// * `data` - 要处理的数据
    ///
    /// # Returns
    ///
    /// 返回处理后的数据
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_tests::DataProcessor;
    /// let processor = DataProcessor::new();
    /// let input = vec!["test".to_string()];
    /// let result = processor.process(&input);
    /// assert_eq!(result[0], "processed: test");
    /// ```
    pub fn process(&self, data: &[String]) -> Vec<String> {
        data.iter()
            .map(|item| format!("processed: {}", item))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_calculator() {
        let calc = BasicCalculator::new();
        assert_eq!(calc.add(2, 3), 5);
        assert_eq!(calc.subtract(10, 4), 6);
        assert_eq!(calc.multiply(3, 4), 12);
        assert_eq!(calc.divide(15, 3), Ok(5));
        assert!(calc.divide(10, 0).is_err());
    }

    #[test]
    fn test_hidden_line_manager() {
        let manager = HiddenLineManager::new();
        let result = manager.process_data("test");
        assert!(result.starts_with("processed:"));
    }

    #[test]
    fn test_error_handler() {
        let handler = ErrorHandler::new();
        assert!(handler.risky_operation("valid").is_ok());
        assert!(handler.risky_operation("").is_err());
    }

    #[test]
    fn test_integration_tester() {
        let mut tester = IntegrationTester::new();
        assert!(!tester.is_initialized());

        tester.initialize();
        assert!(tester.is_initialized());

        tester.load_data("test_data");
        assert!(tester.process().is_ok());

        let results = tester.get_results();
        assert!(!results.is_empty());
    }
}
