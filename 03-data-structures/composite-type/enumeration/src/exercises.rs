//! Rust 枚举练习题
//!
//! 本模块包含了从基础到高级的枚举练习题
//! 涵盖定义、模式匹配、Option、Result、自定义枚举等各个方面

use std::collections::HashMap;
use std::fmt;

// ============================================================================
// 练习 1: 基础枚举定义和使用
// ============================================================================

/// 练习1：定义一个表示交通信号灯的枚举
/// 要求：包含红、黄、绿三种状态，并实现获取等待时间的方法
#[derive(Debug, Clone, PartialEq)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    /// 获取信号灯的等待时间（秒）
    pub fn wait_time(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 45,
        }
    }

    /// 获取下一个信号灯状态
    pub fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }

    /// 判断是否可以通行
    pub fn can_pass(&self) -> bool {
        matches!(self, TrafficLight::Green)
    }
}

// ============================================================================
// 练习 2: 带数据的枚举
// ============================================================================

/// 练习2：定义一个表示几何图形的枚举
/// 要求：包含圆形、矩形、三角形，每种图形存储相应的参数
#[derive(Debug, Clone)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    /// 计算图形面积
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }

    /// 计算图形周长
    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle { radius } => 2.0 * std::f64::consts::PI * radius,
            Shape::Rectangle { width, height } => 2.0 * (width + height),
            Shape::Triangle { base, height } => {
                // 假设是等腰三角形，计算斜边
                let side = (height * height + (base / 2.0) * (base / 2.0)).sqrt();
                base + 2.0 * side
            }
        }
    }

    /// 获取图形类型名称
    pub fn shape_type(&self) -> &'static str {
        match self {
            Shape::Circle { .. } => "圆形",
            Shape::Rectangle { .. } => "矩形",
            Shape::Triangle { .. } => "三角形",
        }
    }
}

// ============================================================================
// 练习 3: Option<T> 实际应用
// ============================================================================

/// 练习3：实现一个安全的除法函数和数组查找函数
pub struct MathUtils;

impl MathUtils {
    /// 安全除法，返回 Option<f64>
    pub fn safe_divide(dividend: f64, divisor: f64) -> Option<f64> {
        if divisor == 0.0 {
            None
        } else {
            Some(dividend / divisor)
        }
    }

    /// 查找数组中的最大值
    pub fn find_max(numbers: &[i32]) -> Option<i32> {
        if numbers.is_empty() {
            None
        } else {
            Some(*numbers.iter().max().unwrap())
        }
    }

    /// 查找数组中指定索引的元素
    pub fn get_element(arr: &[i32], index: usize) -> Option<i32> {
        arr.get(index).copied()
    }

    /// 链式操作示例：计算数组平均值
    pub fn calculate_average(numbers: &[f64]) -> Option<f64> {
        if numbers.is_empty() {
            None
        } else {
            let sum: f64 = numbers.iter().sum();
            Some(sum / numbers.len() as f64)
        }
    }
}

// ============================================================================
// 练习 4: Result<T, E> 错误处理
// ============================================================================

/// 自定义错误类型
#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidFormat,
    OutOfRange,
    EmptyInput,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidFormat => write!(f, "无效的格式"),
            ParseError::OutOfRange => write!(f, "数值超出范围"),
            ParseError::EmptyInput => write!(f, "输入为空"),
        }
    }
}

/// 练习4：实现字符串解析功能
pub struct StringParser;

impl StringParser {
    /// 解析字符串为正整数
    pub fn parse_positive_int(s: &str) -> Result<u32, ParseError> {
        if s.trim().is_empty() {
            return Err(ParseError::EmptyInput);
        }

        match s.trim().parse::<i32>() {
            Ok(num) => {
                if num < 0 {
                    Err(ParseError::OutOfRange)
                } else {
                    Ok(num as u32)
                }
            }
            Err(_) => Err(ParseError::InvalidFormat),
        }
    }

    /// 解析邮箱地址
    pub fn parse_email(email: &str) -> Result<String, ParseError> {
        if email.trim().is_empty() {
            return Err(ParseError::EmptyInput);
        }

        if email.contains('@') && email.contains('.') {
            Ok(email.trim().to_string())
        } else {
            Err(ParseError::InvalidFormat)
        }
    }

    /// 批量解析数字
    pub fn parse_numbers(input: &str) -> Result<Vec<i32>, ParseError> {
        if input.trim().is_empty() {
            return Err(ParseError::EmptyInput);
        }

        let mut numbers = Vec::new();
        for part in input.split(',') {
            match part.trim().parse::<i32>() {
                Ok(num) => numbers.push(num),
                Err(_) => return Err(ParseError::InvalidFormat),
            }
        }
        Ok(numbers)
    }
}

// ============================================================================
// 练习 5: 复杂枚举应用 - 命令行解析器
// ============================================================================

/// 练习5：实现一个简单的命令行解析器
#[derive(Debug, Clone)]
pub enum Command {
    Help,
    Version,
    List {
        filter: Option<String>,
    },
    Create {
        name: String,
        description: Option<String>,
    },
    Delete {
        id: u32,
        force: bool,
    },
    Update {
        id: u32,
        name: Option<String>,
        description: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub enum CommandError {
    UnknownCommand(String),
    MissingArgument(String),
    InvalidArgument(String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::UnknownCommand(cmd) => write!(f, "未知命令: {}", cmd),
            CommandError::MissingArgument(arg) => write!(f, "缺少参数: {}", arg),
            CommandError::InvalidArgument(arg) => write!(f, "无效参数: {}", arg),
        }
    }
}

pub struct CommandParser;

impl CommandParser {
    /// 解析命令行参数
    pub fn parse(args: &[String]) -> Result<Command, CommandError> {
        if args.is_empty() {
            return Ok(Command::Help);
        }

        match args[0].as_str() {
            "help" => Ok(Command::Help),
            "version" => Ok(Command::Version),
            "list" => {
                let filter = args.get(1).map(|s| s.clone());
                Ok(Command::List { filter })
            }
            "create" => {
                if args.len() < 2 {
                    return Err(CommandError::MissingArgument("name".to_string()));
                }
                let name = args[1].clone();
                let description = args.get(2).map(|s| s.clone());
                Ok(Command::Create { name, description })
            }
            "delete" => {
                if args.len() < 2 {
                    return Err(CommandError::MissingArgument("id".to_string()));
                }
                let id = args[1].parse::<u32>().map_err(|_| {
                    CommandError::InvalidArgument("id must be a number".to_string())
                })?;
                let force = args.get(2).map_or(false, |s| s == "--force");
                Ok(Command::Delete { id, force })
            }
            "update" => {
                if args.len() < 2 {
                    return Err(CommandError::MissingArgument("id".to_string()));
                }
                let id = args[1].parse::<u32>().map_err(|_| {
                    CommandError::InvalidArgument("id must be a number".to_string())
                })?;
                let name = args.get(2).map(|s| s.clone());
                let description = args.get(3).map(|s| s.clone());
                Ok(Command::Update {
                    id,
                    name,
                    description,
                })
            }
            cmd => Err(CommandError::UnknownCommand(cmd.to_string())),
        }
    }

    /// 执行命令
    pub fn execute(command: &Command) -> String {
        match command {
            Command::Help => "可用命令: help, version, list, create, delete, update".to_string(),
            Command::Version => "版本 1.0.0".to_string(),
            Command::List { filter } => match filter {
                Some(f) => format!("列出所有项目 (过滤器: {})", f),
                None => "列出所有项目".to_string(),
            },
            Command::Create { name, description } => match description {
                Some(desc) => format!("创建项目: {} - {}", name, desc),
                None => format!("创建项目: {}", name),
            },
            Command::Delete { id, force } => {
                if *force {
                    format!("强制删除项目 ID: {}", id)
                } else {
                    format!("删除项目 ID: {}", id)
                }
            }
            Command::Update {
                id,
                name,
                description,
            } => {
                let mut updates = Vec::new();
                if let Some(n) = name {
                    updates.push(format!("名称: {}", n));
                }
                if let Some(d) = description {
                    updates.push(format!("描述: {}", d));
                }
                format!("更新项目 ID: {} ({})", id, updates.join(", "))
            }
        }
    }
}

// ============================================================================
// 练习 6: 状态机实现 - 订单处理系统
// ============================================================================

/// 练习6：实现订单处理状态机
#[derive(Debug, Clone)]
pub enum OrderStatus {
    Pending {
        items: Vec<String>,
        total: f64,
    },
    Confirmed {
        order_id: String,
        items: Vec<String>,
        total: f64,
        estimated_delivery: String,
    },
    Shipped {
        order_id: String,
        tracking_number: String,
        shipped_at: String,
    },
    Delivered {
        order_id: String,
        delivered_at: String,
        signature: Option<String>,
    },
    Cancelled {
        order_id: Option<String>,
        reason: String,
        cancelled_at: String,
    },
}

#[derive(Debug, Clone)]
pub enum OrderError {
    InvalidTransition(String),
    MissingInformation(String),
    OrderNotFound,
}

impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderError::InvalidTransition(msg) => write!(f, "无效的状态转换: {}", msg),
            OrderError::MissingInformation(info) => write!(f, "缺少信息: {}", info),
            OrderError::OrderNotFound => write!(f, "订单未找到"),
        }
    }
}

impl OrderStatus {
    /// 创建新的待处理订单
    pub fn new_pending(items: Vec<String>, total: f64) -> Self {
        OrderStatus::Pending { items, total }
    }

    /// 确认订单
    pub fn confirm(self, order_id: String, estimated_delivery: String) -> Result<Self, OrderError> {
        match self {
            OrderStatus::Pending { items, total } => Ok(OrderStatus::Confirmed {
                order_id,
                items,
                total,
                estimated_delivery,
            }),
            _ => Err(OrderError::InvalidTransition(
                "只有待处理的订单可以确认".to_string(),
            )),
        }
    }

    /// 发货
    pub fn ship(self, tracking_number: String, shipped_at: String) -> Result<Self, OrderError> {
        match self {
            OrderStatus::Confirmed { order_id, .. } => Ok(OrderStatus::Shipped {
                order_id,
                tracking_number,
                shipped_at,
            }),
            _ => Err(OrderError::InvalidTransition(
                "只有已确认的订单可以发货".to_string(),
            )),
        }
    }

    /// 送达
    pub fn deliver(
        self,
        delivered_at: String,
        signature: Option<String>,
    ) -> Result<Self, OrderError> {
        match self {
            OrderStatus::Shipped { order_id, .. } => Ok(OrderStatus::Delivered {
                order_id,
                delivered_at,
                signature,
            }),
            _ => Err(OrderError::InvalidTransition(
                "只有已发货的订单可以标记为送达".to_string(),
            )),
        }
    }

    /// 取消订单
    pub fn cancel(self, reason: String, cancelled_at: String) -> Self {
        let order_id = match &self {
            OrderStatus::Pending { .. } => None,
            OrderStatus::Confirmed { order_id, .. } => Some(order_id.clone()),
            OrderStatus::Shipped { order_id, .. } => Some(order_id.clone()),
            OrderStatus::Delivered { .. } => return self, // 已送达的订单不能取消
            OrderStatus::Cancelled { .. } => return self, // 已取消的订单保持不变
        };

        OrderStatus::Cancelled {
            order_id,
            reason,
            cancelled_at,
        }
    }

    /// 获取订单状态描述
    pub fn status_description(&self) -> String {
        match self {
            OrderStatus::Pending { items, total } => {
                format!("待处理订单 - {} 件商品，总计 ¥{:.2}", items.len(), total)
            }
            OrderStatus::Confirmed {
                order_id,
                estimated_delivery,
                ..
            } => {
                format!("订单 {} 已确认，预计送达: {}", order_id, estimated_delivery)
            }
            OrderStatus::Shipped {
                order_id,
                tracking_number,
                ..
            } => {
                format!("订单 {} 已发货，快递单号: {}", order_id, tracking_number)
            }
            OrderStatus::Delivered {
                order_id,
                delivered_at,
                signature,
            } => match signature {
                Some(sig) => format!(
                    "订单 {} 已送达 ({})，签收人: {}",
                    order_id, delivered_at, sig
                ),
                None => format!("订单 {} 已送达 ({})", order_id, delivered_at),
            },
            OrderStatus::Cancelled {
                order_id,
                reason,
                cancelled_at,
            } => match order_id {
                Some(id) => format!("订单 {} 已取消 ({})，原因: {}", id, cancelled_at, reason),
                None => format!("订单已取消 ({})，原因: {}", cancelled_at, reason),
            },
        }
    }

    /// 检查订单是否可以修改
    pub fn can_modify(&self) -> bool {
        matches!(
            self,
            OrderStatus::Pending { .. } | OrderStatus::Confirmed { .. }
        )
    }

    /// 获取订单ID（如果有）
    pub fn get_order_id(&self) -> Option<&String> {
        match self {
            OrderStatus::Pending { .. } => None,
            OrderStatus::Confirmed { order_id, .. } => Some(order_id),
            OrderStatus::Shipped { order_id, .. } => Some(order_id),
            OrderStatus::Delivered { order_id, .. } => Some(order_id),
            OrderStatus::Cancelled { order_id, .. } => order_id.as_ref(),
        }
    }
}

// ============================================================================
// 练习 7: 泛型枚举实现 - 通用响应类型
// ============================================================================

/// 练习7：实现通用的API响应类型
#[derive(Debug, Clone)]
pub enum ApiResponse<T, E> {
    Success {
        data: T,
        message: String,
        timestamp: String,
    },
    Error {
        error: E,
        message: String,
        timestamp: String,
    },
    Loading,
}

impl<T, E> ApiResponse<T, E> {
    /// 创建成功响应
    pub fn success(data: T, message: String) -> Self {
        ApiResponse::Success {
            data,
            message,
            timestamp: "now".to_string(),
        }
    }

    /// 创建错误响应
    pub fn error(error: E, message: String) -> Self {
        ApiResponse::Error {
            error,
            message,
            timestamp: "now".to_string(),
        }
    }

    /// 创建加载中响应
    pub fn loading() -> Self {
        ApiResponse::Loading
    }

    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        matches!(self, ApiResponse::Success { .. })
    }

    /// 检查是否错误
    pub fn is_error(&self) -> bool {
        matches!(self, ApiResponse::Error { .. })
    }

    /// 检查是否加载中
    pub fn is_loading(&self) -> bool {
        matches!(self, ApiResponse::Loading)
    }

    /// 映射数据类型
    pub fn map<U, F>(self, f: F) -> ApiResponse<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            ApiResponse::Success {
                data,
                message,
                timestamp,
            } => ApiResponse::Success {
                data: f(data),
                message,
                timestamp,
            },
            ApiResponse::Error {
                error,
                message,
                timestamp,
            } => ApiResponse::Error {
                error,
                message,
                timestamp,
            },
            ApiResponse::Loading => ApiResponse::Loading,
        }
    }

    /// 映射错误类型
    pub fn map_err<F, G>(self, f: G) -> ApiResponse<T, F>
    where
        G: FnOnce(E) -> F,
    {
        match self {
            ApiResponse::Success {
                data,
                message,
                timestamp,
            } => ApiResponse::Success {
                data,
                message,
                timestamp,
            },
            ApiResponse::Error {
                error,
                message,
                timestamp,
            } => ApiResponse::Error {
                error: f(error),
                message,
                timestamp,
            },
            ApiResponse::Loading => ApiResponse::Loading,
        }
    }
}

// ============================================================================
// 练习 8: 综合应用 - 文件系统节点
// ============================================================================

/// 练习8：实现文件系统节点枚举
#[derive(Debug, Clone)]
pub enum FileSystemNode {
    File {
        name: String,
        size: u64,
        content: String,
        permissions: FilePermissions,
    },
    Directory {
        name: String,
        children: Vec<FileSystemNode>,
        permissions: FilePermissions,
    },
    SymLink {
        name: String,
        target: String,
    },
}

#[derive(Debug, Clone)]
pub struct FilePermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl FilePermissions {
    pub fn new(read: bool, write: bool, execute: bool) -> Self {
        FilePermissions {
            read,
            write,
            execute,
        }
    }

    pub fn read_only() -> Self {
        FilePermissions::new(true, false, false)
    }

    pub fn read_write() -> Self {
        FilePermissions::new(true, true, false)
    }

    pub fn full() -> Self {
        FilePermissions::new(true, true, true)
    }
}

impl FileSystemNode {
    /// 创建文件节点
    pub fn new_file(name: String, content: String, permissions: FilePermissions) -> Self {
        let size = content.len() as u64;
        FileSystemNode::File {
            name,
            size,
            content,
            permissions,
        }
    }

    /// 创建目录节点
    pub fn new_directory(name: String, permissions: FilePermissions) -> Self {
        FileSystemNode::Directory {
            name,
            children: Vec::new(),
            permissions,
        }
    }

    /// 创建符号链接节点
    pub fn new_symlink(name: String, target: String) -> Self {
        FileSystemNode::SymLink { name, target }
    }

    /// 获取节点名称
    pub fn name(&self) -> &String {
        match self {
            FileSystemNode::File { name, .. } => name,
            FileSystemNode::Directory { name, .. } => name,
            FileSystemNode::SymLink { name, .. } => name,
        }
    }

    /// 获取节点类型
    pub fn node_type(&self) -> &'static str {
        match self {
            FileSystemNode::File { .. } => "文件",
            FileSystemNode::Directory { .. } => "目录",
            FileSystemNode::SymLink { .. } => "符号链接",
        }
    }

    /// 计算节点大小
    pub fn size(&self) -> u64 {
        match self {
            FileSystemNode::File { size, .. } => *size,
            FileSystemNode::Directory { children, .. } => {
                children.iter().map(|child| child.size()).sum()
            }
            FileSystemNode::SymLink { .. } => 0,
        }
    }

    /// 添加子节点（仅对目录有效）
    pub fn add_child(&mut self, child: FileSystemNode) -> Result<(), String> {
        match self {
            FileSystemNode::Directory { children, .. } => {
                children.push(child);
                Ok(())
            }
            _ => Err("只能向目录添加子节点".to_string()),
        }
    }

    /// 查找子节点
    pub fn find_child(&self, name: &str) -> Option<&FileSystemNode> {
        match self {
            FileSystemNode::Directory { children, .. } => {
                children.iter().find(|child| child.name() == name)
            }
            _ => None,
        }
    }

    /// 列出所有子节点名称
    pub fn list_children(&self) -> Vec<String> {
        match self {
            FileSystemNode::Directory { children, .. } => {
                children.iter().map(|child| child.name().clone()).collect()
            }
            _ => Vec::new(),
        }
    }

    /// 检查权限
    pub fn has_permission(&self, permission: &str) -> bool {
        let perms = match self {
            FileSystemNode::File { permissions, .. } => permissions,
            FileSystemNode::Directory { permissions, .. } => permissions,
            FileSystemNode::SymLink { .. } => return true, // 符号链接总是可访问的
        };

        match permission {
            "read" => perms.read,
            "write" => perms.write,
            "execute" => perms.execute,
            _ => false,
        }
    }
}

// ============================================================================
// 运行所有练习的函数
// ============================================================================

pub fn run_all_exercises() {
    println!("🦀 Rust 枚举练习题");
    println!("{}", "=".repeat(50));

    exercise_1_traffic_light();
    exercise_2_geometric_shapes();
    exercise_3_option_applications();
    exercise_4_result_error_handling();
    exercise_5_command_parser();
    exercise_6_order_state_machine();
    exercise_7_generic_api_response();
    exercise_8_file_system();

    println!("\n🎉 所有枚举练习完成!");
}

fn exercise_1_traffic_light() {
    println!("\n=== 练习1: 交通信号灯 ===");

    let mut light = TrafficLight::Red;
    for _ in 0..4 {
        println!("当前信号灯: {:?}", light);
        println!("等待时间: {} 秒", light.wait_time());
        println!("可以通行: {}", light.can_pass());
        light = light.next();
        println!();
    }
}

fn exercise_2_geometric_shapes() {
    println!("\n=== 练习2: 几何图形 ===");

    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle {
            width: 10.0,
            height: 6.0,
        },
        Shape::Triangle {
            base: 8.0,
            height: 4.0,
        },
    ];

    for shape in shapes {
        println!("图形: {}", shape.shape_type());
        println!("面积: {:.2}", shape.area());
        println!("周长: {:.2}", shape.perimeter());
        println!();
    }
}

fn exercise_3_option_applications() {
    println!("\n=== 练习3: Option 应用 ===");

    // 安全除法
    let results = vec![
        MathUtils::safe_divide(10.0, 2.0),
        MathUtils::safe_divide(10.0, 0.0),
        MathUtils::safe_divide(-5.0, 2.5),
    ];

    for (i, result) in results.iter().enumerate() {
        match result {
            Some(value) => println!("除法结果 {}: {}", i + 1, value),
            None => println!("除法结果 {}: 除零错误", i + 1),
        }
    }

    // 查找最大值
    let arrays = vec![vec![1, 5, 3, 9, 2], vec![-1, -5, -2], vec![]];

    for (i, arr) in arrays.iter().enumerate() {
        match MathUtils::find_max(arr) {
            Some(max) => println!("数组 {} 的最大值: {}", i + 1, max),
            None => println!("数组 {} 为空", i + 1),
        }
    }

    // 计算平均值
    let number_sets = vec![
        vec![1.0, 2.0, 3.0, 4.0, 5.0],
        vec![10.5, 20.3, 15.7],
        vec![],
    ];

    for (i, numbers) in number_sets.iter().enumerate() {
        match MathUtils::calculate_average(numbers) {
            Some(avg) => println!("数组 {} 的平均值: {:.2}", i + 1, avg),
            None => println!("数组 {} 为空，无法计算平均值", i + 1),
        }
    }
}

fn exercise_4_result_error_handling() {
    println!("\n=== 练习4: Result 错误处理 ===");

    // 解析正整数
    let test_strings = vec!["42", "-5", "abc", "", "  123  "];

    for s in test_strings {
        match StringParser::parse_positive_int(s) {
            Ok(num) => println!("解析 '{}' 成功: {}", s, num),
            Err(e) => println!("解析 '{}' 失败: {}", s, e),
        }
    }

    // 解析邮箱
    let emails = vec!["user@example.com", "invalid-email", "", "test@domain"];

    for email in emails {
        match StringParser::parse_email(email) {
            Ok(valid_email) => println!("有效邮箱: {}", valid_email),
            Err(e) => println!("无效邮箱 '{}': {}", email, e),
        }
    }

    // 批量解析数字
    let number_strings = vec!["1,2,3,4,5", "10, 20, 30", "1,abc,3", ""];

    for s in number_strings {
        match StringParser::parse_numbers(s) {
            Ok(numbers) => println!("解析 '{}' 成功: {:?}", s, numbers),
            Err(e) => println!("解析 '{}' 失败: {}", s, e),
        }
    }
}

fn exercise_5_command_parser() {
    println!("\n=== 练习5: 命令行解析器 ===");

    let test_commands = vec![
        vec![],
        vec!["help".to_string()],
        vec!["version".to_string()],
        vec!["list".to_string()],
        vec!["list".to_string(), "active".to_string()],
        vec!["create".to_string(), "project1".to_string()],
        vec![
            "create".to_string(),
            "project2".to_string(),
            "A test project".to_string(),
        ],
        vec!["delete".to_string(), "123".to_string()],
        vec![
            "delete".to_string(),
            "456".to_string(),
            "--force".to_string(),
        ],
        vec![
            "update".to_string(),
            "789".to_string(),
            "new_name".to_string(),
        ],
        vec!["unknown".to_string()],
    ];

    for args in test_commands {
        match CommandParser::parse(&args) {
            Ok(command) => {
                println!("命令: {:?}", command);
                println!("执行结果: {}", CommandParser::execute(&command));
            }
            Err(e) => println!("解析错误: {}", e),
        }
        println!();
    }
}

fn exercise_6_order_state_machine() {
    println!("\n=== 练习6: 订单状态机 ===");

    let items = vec!["商品A".to_string(), "商品B".to_string()];
    let mut order = OrderStatus::new_pending(items, 199.99);

    println!("1. {}", order.status_description());

    // 确认订单
    order = order
        .confirm("ORD-001".to_string(), "2024-01-15".to_string())
        .unwrap();
    println!("2. {}", order.status_description());

    // 发货
    order = order
        .ship("TRK-123456".to_string(), "2024-01-10".to_string())
        .unwrap();
    println!("3. {}", order.status_description());

    // 送达
    order = order
        .deliver("2024-01-12".to_string(), Some("张三".to_string()))
        .unwrap();
    println!("4. {}", order.status_description());

    println!("订单ID: {:?}", order.get_order_id());
    println!("可以修改: {}", order.can_modify());
}

fn exercise_7_generic_api_response() {
    println!("\n=== 练习7: 泛型API响应 ===");

    // 用户数据响应
    let user_response: ApiResponse<HashMap<String, String>, String> = ApiResponse::success(
        {
            let mut user = HashMap::new();
            user.insert("id".to_string(), "123".to_string());
            user.insert("name".to_string(), "张三".to_string());
            user
        },
        "获取用户信息成功".to_string(),
    );

    println!("用户响应成功: {}", user_response.is_success());

    // 错误响应
    let error_response: ApiResponse<String, u32> =
        ApiResponse::error(404, "资源未找到".to_string());

    println!("错误响应: {}", error_response.is_error());

    // 加载中响应
    let loading_response: ApiResponse<String, String> = ApiResponse::loading();
    println!("加载中: {}", loading_response.is_loading());

    // 映射数据
    let mapped_response = user_response.map(|user_data| format!("用户数量: {}", user_data.len()));

    println!("映射后的响应: {:?}", mapped_response);
}

fn exercise_8_file_system() {
    println!("\n=== 练习8: 文件系统节点 ===");

    // 创建文件系统结构
    let mut root = FileSystemNode::new_directory("root".to_string(), FilePermissions::full());

    let file1 = FileSystemNode::new_file(
        "readme.txt".to_string(),
        "这是一个README文件".to_string(),
        FilePermissions::read_only(),
    );

    let file2 = FileSystemNode::new_file(
        "config.json".to_string(),
        "{\"version\": \"1.0\"}".to_string(),
        FilePermissions::read_write(),
    );

    let symlink =
        FileSystemNode::new_symlink("link_to_readme".to_string(), "readme.txt".to_string());

    let mut subdir = FileSystemNode::new_directory("subdir".to_string(), FilePermissions::full());

    let subfile = FileSystemNode::new_file(
        "data.txt".to_string(),
        "一些数据内容".to_string(),
        FilePermissions::read_write(),
    );

    // 构建文件系统树
    subdir.add_child(subfile).unwrap();
    root.add_child(file1).unwrap();
    root.add_child(file2).unwrap();
    root.add_child(symlink).unwrap();
    root.add_child(subdir).unwrap();

    // 显示文件系统信息
    println!("根目录: {}", root.name());
    println!("类型: {}", root.node_type());
    println!("总大小: {} 字节", root.size());
    println!("子节点: {:?}", root.list_children());

    // 查找特定文件
    if let Some(readme) = root.find_child("readme.txt") {
        println!("\n找到文件: {}", readme.name());
        println!("类型: {}", readme.node_type());
        println!("大小: {} 字节", readme.size());
        println!("可读: {}", readme.has_permission("read"));
        println!("可写: {}", readme.has_permission("write"));
    }
}

// ============================================================================
// 测试模块
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_light() {
        let red = TrafficLight::Red;
        assert_eq!(red.wait_time(), 60);
        assert!(!red.can_pass());

        let green = red.next();
        assert_eq!(green, TrafficLight::Green);
        assert!(green.can_pass());
    }

    #[test]
    fn test_shape_calculations() {
        let circle = Shape::Circle { radius: 5.0 };
        let area = circle.area();
        assert!((area - 78.54).abs() < 0.1);

        let rectangle = Shape::Rectangle {
            width: 10.0,
            height: 5.0,
        };
        assert_eq!(rectangle.area(), 50.0);
        assert_eq!(rectangle.perimeter(), 30.0);
    }

    #[test]
    fn test_math_utils() {
        assert_eq!(MathUtils::safe_divide(10.0, 2.0), Some(5.0));
        assert_eq!(MathUtils::safe_divide(10.0, 0.0), None);

        assert_eq!(MathUtils::find_max(&[1, 5, 3]), Some(5));
        assert_eq!(MathUtils::find_max(&[]), None);

        assert_eq!(MathUtils::calculate_average(&[1.0, 2.0, 3.0]), Some(2.0));
        assert_eq!(MathUtils::calculate_average(&[]), None);
    }

    #[test]
    fn test_string_parser() {
        assert!(StringParser::parse_positive_int("42").is_ok());
        assert!(StringParser::parse_positive_int("-5").is_err());
        assert!(StringParser::parse_positive_int("abc").is_err());

        assert!(StringParser::parse_email("user@example.com").is_ok());
        assert!(StringParser::parse_email("invalid").is_err());

        assert_eq!(StringParser::parse_numbers("1,2,3").unwrap(), vec![1, 2, 3]);
        assert!(StringParser::parse_numbers("1,abc,3").is_err());
    }

    #[test]
    fn test_command_parser() {
        let help_cmd = CommandParser::parse(&["help".to_string()]).unwrap();
        assert!(matches!(help_cmd, Command::Help));

        let create_cmd = CommandParser::parse(&["create".to_string(), "test".to_string()]).unwrap();
        assert!(matches!(create_cmd, Command::Create { .. }));

        let unknown_result = CommandParser::parse(&["unknown".to_string()]);
        assert!(unknown_result.is_err());
    }

    #[test]
    fn test_order_state_machine() {
        let order = OrderStatus::new_pending(vec!["item1".to_string()], 100.0);
        let order = order
            .confirm("ORD-001".to_string(), "2024-01-15".to_string())
            .unwrap();
        let order = order
            .ship("TRK-123".to_string(), "2024-01-10".to_string())
            .unwrap();
        let order = order.deliver("2024-01-12".to_string(), None).unwrap();

        assert_eq!(order.get_order_id(), Some(&"ORD-001".to_string()));
        assert!(!order.can_modify());
    }

    #[test]
    fn test_api_response() {
        let success: ApiResponse<i32, String> = ApiResponse::success(42, "OK".to_string());
        assert!(success.is_success());

        let error: ApiResponse<i32, String> =
            ApiResponse::error("Error".to_string(), "Failed".to_string());
        assert!(!success.is_error());

        let loading: ApiResponse<i32, String> = ApiResponse::loading();
        assert!(loading.is_loading());

        let mapped = success.map(|x| x * 2);
        assert!(mapped.is_success());
    }

    #[test]
    fn test_file_system() {
        let mut dir = FileSystemNode::new_directory("test".to_string(), FilePermissions::full());
        let file = FileSystemNode::new_file(
            "test.txt".to_string(),
            "content".to_string(),
            FilePermissions::read_only(),
        );

        assert!(dir.add_child(file).is_ok());
        assert_eq!(dir.list_children(), vec!["test.txt".to_string()]);
        assert!(dir.find_child("test.txt").is_some());
        assert_eq!(dir.size(), 7); // "content" 的长度
    }
}
