//! Rust 枚举全面教程
//!
//! 本模块涵盖了 Rust 枚举的所有核心概念和高级特性
//! 包括：定义、模式匹配、Option、Result、自定义枚举等
//! 基于 https://course.rs/basic/compound-type/enum.html 的内容

use std::fmt;

// ============================================================================
// 1. 枚举的基本定义
// ============================================================================

/// 基础枚举示例：IP 地址类型
/// 枚举值只可能是其中一个成员
#[derive(Debug, Clone, PartialEq)]
pub enum IpAddrKind {
    V4,
    V6,
}

/// 带数据的枚举：每个成员可以存储不同类型的数据
#[derive(Debug, Clone, PartialEq)]
pub enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

/// 复杂枚举：不同成员存储不同类型和数量的数据
#[derive(Debug, Clone)]
pub enum Message {
    Quit,                       // 无数据
    Move { x: i32, y: i32 },    // 命名字段
    Write(String),              // 单个字符串
    ChangeColor(i32, i32, i32), // 三个整数
}

// ============================================================================
// 2. 枚举方法实现
// ============================================================================

impl Message {
    /// 为枚举实现方法
    pub fn call(&self) {
        match self {
            Message::Quit => println!("退出程序"),
            Message::Move { x, y } => println!("移动到坐标 ({}, {})", x, y),
            Message::Write(text) => println!("写入文本: {}", text),
            Message::ChangeColor(r, g, b) => println!("改变颜色为 RGB({}, {}, {})", r, g, b),
        }
    }

    /// 获取消息类型名称
    pub fn message_type(&self) -> &'static str {
        match self {
            Message::Quit => "Quit",
            Message::Move { .. } => "Move",
            Message::Write(_) => "Write",
            Message::ChangeColor(_, _, _) => "ChangeColor",
        }
    }
}

// ============================================================================
// 3. Option<T> 枚举详解
// ============================================================================

/// Option<T> 是 Rust 标准库中最重要的枚举之一
/// 用于表示一个值可能存在（Some(T)）或不存在（None）的情况
pub fn option_examples() {
    println!("\n=== Option<T> 枚举示例 ===");

    // 创建 Option 值
    let some_number = Some(5);
    let some_string = Some("hello");
    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);

    // 使用 match 处理 Option
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    println!(
        "x 的值: {}",
        match x {
            Some(i) => i.to_string(),
            None => "无值".to_string(),
        }
    );

    println!(
        "y 的值: {}",
        match y {
            Some(i) => i.to_string(),
            None => "无值".to_string(),
        }
    );

    // Option 的常用方法
    demonstrate_option_methods();
}

fn demonstrate_option_methods() {
    println!("\n--- Option 常用方法 ---");

    let x = Some(2);
    let y: Option<i32> = None;

    // is_some() 和 is_none()
    println!("x.is_some(): {}", x.is_some());
    println!("y.is_none(): {}", y.is_none());

    // unwrap() - 获取值，如果是 None 会 panic
    println!("x.unwrap(): {}", x.unwrap());
    // println!("y.unwrap(): {}", y.unwrap()); // 这会 panic!

    // unwrap_or() - 提供默认值
    println!("y.unwrap_or(0): {}", y.unwrap_or(0));

    // map() - 对 Some 中的值进行转换
    let doubled = x.map(|val| val * 2);
    println!("x.map(|val| val * 2): {:?}", doubled);

    // and_then() - 链式操作
    let result = x.and_then(|val| if val > 1 { Some(val * 2) } else { None });
    println!("x.and_then(...): {:?}", result);
}

// ============================================================================
// 4. Result<T, E> 枚举详解
// ============================================================================

/// Result<T, E> 用于错误处理
/// Ok(T) 表示成功，Err(E) 表示错误
pub fn result_examples() {
    println!("\n=== Result<T, E> 枚举示例 ===");

    // 字符串解析示例
    let good_result: Result<i32, _> = "42".parse();
    let bad_result: Result<i32, _> = "abc".parse();

    println!("解析 '42': {:?}", good_result);
    println!("解析 'abc': {:?}", bad_result);

    // 使用 match 处理 Result
    match good_result {
        Ok(num) => println!("成功解析数字: {}", num),
        Err(e) => println!("解析失败: {}", e),
    }

    match bad_result {
        Ok(num) => println!("成功解析数字: {}", num),
        Err(e) => println!("解析失败: {}", e),
    }

    // Result 的常用方法
    demonstrate_result_methods();
}

fn demonstrate_result_methods() {
    println!("\n--- Result 常用方法 ---");

    let good: Result<i32, &str> = Ok(42);
    let bad: Result<i32, &str> = Err("错误信息");

    // is_ok() 和 is_err()
    println!("good.is_ok(): {}", good.is_ok());
    println!("bad.is_err(): {}", bad.is_err());

    // unwrap_or() - 提供默认值
    println!("bad.unwrap_or(0): {}", bad.unwrap_or(0));

    // map() - 对 Ok 中的值进行转换
    let doubled = good.map(|val| val * 2);
    println!("good.map(|val| val * 2): {:?}", doubled);

    // map_err() - 对 Err 中的错误进行转换
    let mapped_err = bad.map_err(|e| format!("转换后的错误: {}", e));
    println!("bad.map_err(...): {:?}", mapped_err);
}

// ============================================================================
// 5. match 模式匹配详解
// ============================================================================

/// 硬币枚举示例
#[derive(Debug)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
pub enum UsState {
    Alabama,
    Alaska,
    California,
    Texas,
}

/// 计算硬币价值
pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("幸运便士!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("来自 {:?} 州的25美分硬币!", state);
            25
        }
    }
}

/// match 必须穷尽所有可能性
pub fn match_examples() {
    println!("\n=== match 模式匹配示例 ===");

    let coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(coin);
    println!("硬币价值: {} 美分", value);

    // 匹配 Option
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five: {:?}", five);
    println!("six: {:?}", six);
    println!("none: {:?}", none);

    // 使用 _ 通配符
    let some_u8_value = 3u8;
    match some_u8_value {
        1 => println!("一"),
        3 => println!("三"),
        5 => println!("五"),
        7 => println!("七"),
        _ => println!("其他数字"),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// ============================================================================
// 6. if let 简洁控制流
// ============================================================================

pub fn if_let_examples() {
    println!("\n=== if let 简洁控制流示例 ===");

    let config_max = Some(3u8);

    // 使用 match 的冗长写法
    match config_max {
        Some(max) => println!("最大值配置为 {}", max),
        _ => (),
    }

    // 使用 if let 的简洁写法
    if let Some(max) = config_max {
        println!("使用 if let: 最大值配置为 {}", max);
    }

    // if let 与 else 结合
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("使用你最喜欢的颜色 {} 作为背景", color);
    } else if is_tuesday {
        println!("星期二是绿色的日子!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("使用紫色作为背景颜色");
        } else {
            println!("使用橙色作为背景颜色");
        }
    } else {
        println!("使用蓝色作为背景颜色");
    }

    // while let 循环
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("\n--- while let 循环示例 ---");
    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }
}

// ============================================================================
// 7. 自定义枚举应用案例
// ============================================================================

/// 网络请求状态枚举
#[derive(Debug, Clone)]
pub enum RequestStatus {
    Pending,
    Success(String),
    Error { code: u16, message: String },
    Timeout,
}

impl RequestStatus {
    pub fn is_success(&self) -> bool {
        matches!(self, RequestStatus::Success(_))
    }

    pub fn get_message(&self) -> String {
        match self {
            RequestStatus::Pending => "请求处理中...".to_string(),
            RequestStatus::Success(data) => format!("请求成功: {}", data),
            RequestStatus::Error { code, message } => {
                format!("请求失败 [{}]: {}", code, message)
            }
            RequestStatus::Timeout => "请求超时".to_string(),
        }
    }
}

/// 用户权限枚举
#[derive(Debug, Clone, PartialEq)]
pub enum Permission {
    Read,
    Write,
    Execute,
    Admin,
}

/// 用户角色枚举
#[derive(Debug, Clone)]
pub enum UserRole {
    Guest,
    User(Vec<Permission>),
    Moderator(Vec<Permission>),
    Admin,
}

impl UserRole {
    pub fn has_permission(&self, permission: &Permission) -> bool {
        match self {
            UserRole::Guest => permission == &Permission::Read,
            UserRole::User(perms) | UserRole::Moderator(perms) => perms.contains(permission),
            UserRole::Admin => true,
        }
    }

    pub fn get_permissions(&self) -> Vec<Permission> {
        match self {
            UserRole::Guest => vec![Permission::Read],
            UserRole::User(perms) | UserRole::Moderator(perms) => perms.clone(),
            UserRole::Admin => vec![
                Permission::Read,
                Permission::Write,
                Permission::Execute,
                Permission::Admin,
            ],
        }
    }
}

// ============================================================================
// 8. 枚举与泛型结合
// ============================================================================

/// 泛型枚举：表示操作结果
#[derive(Debug, Clone)]
pub enum OperationResult<T, E> {
    Success(T),
    Failure(E),
    Pending,
}

impl<T, E> OperationResult<T, E> {
    pub fn is_success(&self) -> bool {
        matches!(self, OperationResult::Success(_))
    }

    pub fn is_failure(&self) -> bool {
        matches!(self, OperationResult::Failure(_))
    }

    pub fn is_pending(&self) -> bool {
        matches!(self, OperationResult::Pending)
    }

    pub fn map<U, F>(self, f: F) -> OperationResult<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            OperationResult::Success(value) => OperationResult::Success(f(value)),
            OperationResult::Failure(error) => OperationResult::Failure(error),
            OperationResult::Pending => OperationResult::Pending,
        }
    }
}

// ============================================================================
// 9. 实际应用案例：状态机
// ============================================================================

/// 状态机示例：文档编辑器状态
#[derive(Debug, Clone)]
pub enum DocumentState {
    Draft {
        content: String,
        last_modified: String,
    },
    UnderReview {
        content: String,
        reviewer: String,
        submitted_at: String,
    },
    Published {
        content: String,
        published_at: String,
        version: u32,
    },
    Archived {
        reason: String,
        archived_at: String,
    },
}

impl DocumentState {
    pub fn new_draft(content: String) -> Self {
        DocumentState::Draft {
            content,
            last_modified: "now".to_string(),
        }
    }

    pub fn submit_for_review(self, reviewer: String) -> Result<Self, String> {
        match self {
            DocumentState::Draft { content, .. } => Ok(DocumentState::UnderReview {
                content,
                reviewer,
                submitted_at: "now".to_string(),
            }),
            _ => Err("只有草稿状态的文档可以提交审核".to_string()),
        }
    }

    pub fn publish(self, version: u32) -> Result<Self, String> {
        match self {
            DocumentState::UnderReview { content, .. } => Ok(DocumentState::Published {
                content,
                published_at: "now".to_string(),
                version,
            }),
            _ => Err("只有审核中的文档可以发布".to_string()),
        }
    }

    pub fn archive(self, reason: String) -> Self {
        DocumentState::Archived {
            reason,
            archived_at: "now".to_string(),
        }
    }

    pub fn get_status(&self) -> &'static str {
        match self {
            DocumentState::Draft { .. } => "草稿",
            DocumentState::UnderReview { .. } => "审核中",
            DocumentState::Published { .. } => "已发布",
            DocumentState::Archived { .. } => "已归档",
        }
    }
}

// ============================================================================
// 10. 枚举的 Display 实现
// ============================================================================

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpAddr::V4(a, b, c, d) => write!(f, "{}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(addr) => write!(f, "{}", addr),
        }
    }
}

impl fmt::Display for RequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_message())
    }
}

// ============================================================================
// 11. 综合应用案例：简单计算器
// ============================================================================

/// 计算器操作枚举
#[derive(Debug, Clone)]
pub enum CalculatorOp {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
    Power(f64, f64),
    SquareRoot(f64),
}

/// 计算器结果枚举
#[derive(Debug, Clone)]
pub enum CalculatorResult {
    Success(f64),
    Error(String),
}

impl CalculatorOp {
    pub fn execute(&self) -> CalculatorResult {
        match self {
            CalculatorOp::Add(a, b) => CalculatorResult::Success(a + b),
            CalculatorOp::Subtract(a, b) => CalculatorResult::Success(a - b),
            CalculatorOp::Multiply(a, b) => CalculatorResult::Success(a * b),
            CalculatorOp::Divide(a, b) => {
                if *b == 0.0 {
                    CalculatorResult::Error("除零错误".to_string())
                } else {
                    CalculatorResult::Success(a / b)
                }
            }
            CalculatorOp::Power(base, exp) => CalculatorResult::Success(base.powf(*exp)),
            CalculatorOp::SquareRoot(n) => {
                if *n < 0.0 {
                    CalculatorResult::Error("负数不能开平方根".to_string())
                } else {
                    CalculatorResult::Success(n.sqrt())
                }
            }
        }
    }
}

pub struct Calculator {
    history: Vec<(CalculatorOp, CalculatorResult)>,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            history: Vec::new(),
        }
    }

    pub fn calculate(&mut self, op: CalculatorOp) -> &CalculatorResult {
        let result = op.execute();
        self.history.push((op, result));
        &self.history.last().unwrap().1
    }

    pub fn get_history(&self) -> &[(CalculatorOp, CalculatorResult)] {
        &self.history
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

// ============================================================================
// 12. 运行所有示例的函数
// ============================================================================

pub fn run_all_examples() {
    println!("🦀 Rust 枚举全面教程示例");
    println!("{}", "=".repeat(50));

    // 1. 基础枚举示例
    basic_enum_examples();

    // 2. Option 示例
    option_examples();

    // 3. Result 示例
    result_examples();

    // 4. match 示例
    match_examples();

    // 5. if let 示例
    if_let_examples();

    // 6. 自定义枚举应用
    custom_enum_applications();

    // 7. 状态机示例
    state_machine_example();

    // 8. 计算器示例
    calculator_example();

    println!("\n🎉 所有枚举示例运行完成!");
}

fn basic_enum_examples() {
    println!("\n=== 基础枚举示例 ===");

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6("::1".to_string());

    println!("IPv4 地址: {}", home);
    println!("IPv6 地址: {}", loopback);

    // 枚举方法调用
    let msg1 = Message::Write("Hello, Rust!".to_string());
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::ChangeColor(255, 0, 0);

    msg1.call();
    msg2.call();
    msg3.call();

    println!("消息类型: {}", msg1.message_type());
}

fn custom_enum_applications() {
    println!("\n=== 自定义枚举应用示例 ===");

    // 网络请求状态
    let statuses = vec![
        RequestStatus::Pending,
        RequestStatus::Success("数据获取成功".to_string()),
        RequestStatus::Error {
            code: 404,
            message: "页面未找到".to_string(),
        },
        RequestStatus::Timeout,
    ];

    for status in statuses {
        println!("请求状态: {}", status);
        println!("是否成功: {}", status.is_success());
    }

    // 用户权限示例
    let user_role = UserRole::User(vec![Permission::Read, Permission::Write]);
    let admin_role = UserRole::Admin;

    println!("\n--- 用户权限检查 ---");
    println!(
        "普通用户有写权限: {}",
        user_role.has_permission(&Permission::Write)
    );
    println!(
        "普通用户有管理权限: {}",
        user_role.has_permission(&Permission::Admin)
    );
    println!(
        "管理员有管理权限: {}",
        admin_role.has_permission(&Permission::Admin)
    );
}

fn state_machine_example() {
    println!("\n=== 状态机示例：文档编辑器 ===");

    let mut doc = DocumentState::new_draft("这是一篇文档的内容".to_string());
    println!("文档状态: {}", doc.get_status());

    // 提交审核
    doc = doc.submit_for_review("张三".to_string()).unwrap();
    println!("文档状态: {}", doc.get_status());

    // 发布
    doc = doc.publish(1).unwrap();
    println!("文档状态: {}", doc.get_status());

    // 归档
    doc = doc.archive("内容过时".to_string());
    println!("文档状态: {}", doc.get_status());
}

fn calculator_example() {
    println!("\n=== 计算器示例 ===");

    let mut calc = Calculator::new();

    let operations = vec![
        CalculatorOp::Add(10.0, 5.0),
        CalculatorOp::Subtract(20.0, 8.0),
        CalculatorOp::Multiply(6.0, 7.0),
        CalculatorOp::Divide(15.0, 3.0),
        CalculatorOp::Divide(10.0, 0.0), // 除零错误
        CalculatorOp::Power(2.0, 3.0),
        CalculatorOp::SquareRoot(16.0),
        CalculatorOp::SquareRoot(-4.0), // 负数开方错误
    ];

    for op in operations {
        let result = calc.calculate(op.clone());
        match result {
            CalculatorResult::Success(value) => {
                println!("{:?} = {}", op, value);
            }
            CalculatorResult::Error(msg) => {
                println!("{:?} -> 错误: {}", op, msg);
            }
        }
    }

    println!("\n计算历史记录数量: {}", calc.get_history().len());
}

// ============================================================================
// 13. 测试模块
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_addr() {
        let v4 = IpAddr::V4(192, 168, 1, 1);
        let v6 = IpAddr::V6("2001:db8::1".to_string());

        assert_eq!(format!("{}", v4), "192.168.1.1");
        assert_eq!(format!("{}", v6), "2001:db8::1");
    }

    #[test]
    fn test_message_methods() {
        let msg = Message::Write("test".to_string());
        assert_eq!(msg.message_type(), "Write");

        let move_msg = Message::Move { x: 1, y: 2 };
        assert_eq!(move_msg.message_type(), "Move");
    }

    #[test]
    fn test_option_methods() {
        let some_val = Some(5);
        let none_val: Option<i32> = None;

        assert!(some_val.is_some());
        assert!(none_val.is_none());
        assert_eq!(some_val.unwrap_or(0), 5);
        assert_eq!(none_val.unwrap_or(0), 0);
    }

    #[test]
    fn test_result_methods() {
        let ok_val: Result<i32, &str> = Ok(42);
        let err_val: Result<i32, &str> = Err("error");

        assert!(ok_val.is_ok());
        assert!(err_val.is_err());
        assert_eq!(ok_val.unwrap_or(0), 42);
        assert_eq!(err_val.unwrap_or(0), 0);
    }

    #[test]
    fn test_user_permissions() {
        let user = UserRole::User(vec![Permission::Read, Permission::Write]);
        let admin = UserRole::Admin;
        let guest = UserRole::Guest;

        assert!(user.has_permission(&Permission::Read));
        assert!(user.has_permission(&Permission::Write));
        assert!(!user.has_permission(&Permission::Admin));

        assert!(admin.has_permission(&Permission::Admin));

        assert!(guest.has_permission(&Permission::Read));
        assert!(!guest.has_permission(&Permission::Write));
    }

    #[test]
    fn test_document_state_machine() {
        let doc = DocumentState::new_draft("content".to_string());
        assert_eq!(doc.get_status(), "草稿");

        let doc = doc.submit_for_review("reviewer".to_string()).unwrap();
        assert_eq!(doc.get_status(), "审核中");

        let doc = doc.publish(1).unwrap();
        assert_eq!(doc.get_status(), "已发布");

        let doc = doc.archive("reason".to_string());
        assert_eq!(doc.get_status(), "已归档");
    }

    #[test]
    fn test_calculator() {
        let mut calc = Calculator::new();

        let result = calc.calculate(CalculatorOp::Add(2.0, 3.0));
        match result {
            CalculatorResult::Success(val) => assert_eq!(*val, 5.0),
            _ => panic!("Expected success"),
        }

        let result = calc.calculate(CalculatorOp::Divide(10.0, 0.0));
        match result {
            CalculatorResult::Error(_) => (), // Expected
            _ => panic!("Expected error"),
        }

        assert_eq!(calc.get_history().len(), 2);
    }

    #[test]
    fn test_operation_result() {
        let success: OperationResult<i32, String> = OperationResult::Success(42);
        let failure: OperationResult<i32, String> = OperationResult::Failure("error".to_string());
        let pending: OperationResult<i32, String> = OperationResult::Pending;

        assert!(success.is_success());
        assert!(failure.is_failure());
        assert!(pending.is_pending());

        let mapped = success.map(|x| x * 2);
        match mapped {
            OperationResult::Success(val) => assert_eq!(val, 84),
            _ => panic!("Expected success"),
        }
    }
}
