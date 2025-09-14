//! # Rust 基础格式化输出
//!
//! 本模块详细介绍 Rust 中的基础格式化输出功能，包括 `println!`、`print!`、`format!` 等宏的使用。
//!
//! ## 核心概念
//!
//! Rust 的格式化系统基于以下几个核心宏：
//! - `println!` - 打印到标准输出并添加换行符
//! - `print!` - 打印到标准输出但不添加换行符
//! - `format!` - 格式化字符串但不输出，返回 String
//! - `eprint!` / `eprintln!` - 打印到标准错误输出
//!
//! ## 设计原理
//!
//! Rust 的格式化系统设计具有以下特点：
//! 1. **类型安全** - 编译时检查格式字符串和参数类型匹配
//! 2. **零成本抽象** - 格式化代码在编译时优化
//! 3. **可扩展性** - 支持自定义类型的格式化
//! 4. **国际化友好** - 支持 Unicode 和本地化

use std::fmt;
use std::io::{self, Write};

/// 演示基础格式化输出功能
///
/// 这个函数展示了 Rust 中各种基础格式化宏的使用方法和区别。
///
/// # 格式化宏对比
///
/// | 宏名 | 输出目标 | 换行 | 返回值 | 使用场景 |
/// |------|----------|------|--------|-----------|
/// | `println!` | 标准输出 | 是 | `()` | 一般输出 |
/// | `print!` | 标准输出 | 否 | `()` | 连续输出 |
/// | `format!` | 无输出 | 否 | `String` | 字符串构建 |
/// | `eprintln!` | 标准错误 | 是 | `()` | 错误信息 |
/// | `eprint!` | 标准错误 | 否 | `()` | 错误信息 |
///
/// # Examples
///
/// ```rust
/// # use formatted_output::basic_formatting::demonstrate_basic_formatting;
/// demonstrate_basic_formatting();
/// ```
pub fn demonstrate_basic_formatting() {
    println!("\n=== Rust 基础格式化输出演示 ===");

    // 1. println! 宏 - 最常用的输出宏
    println!("\n--- println! 宏演示 ---");
    println!("Hello, World!"); // 基本输出
    println!("数字: {}", 42); // 单个参数
    println!("多个参数: {} 和 {}", "Rust", 2024); // 多个参数

    // 2. print! 宏 - 不换行输出
    println!("\n--- print! 宏演示 ---");
    print!("这是第一部分 ");
    print!("这是第二部分 ");
    print!("这是第三部分\n"); // 手动添加换行

    // 3. format! 宏 - 构建字符串
    println!("\n--- format! 宏演示 ---");
    let formatted_string = format!("用户: {}, 年龄: {}", "张三", 25);
    println!("构建的字符串: {}", formatted_string);

    let complex_format = format!(
        "复杂格式化: 姓名={name}, 年龄={age}, 城市={city}",
        name = "李四",
        age = 30,
        city = "北京"
    );
    println!("{}", complex_format);

    // 4. 错误输出宏
    println!("\n--- 错误输出宏演示 ---");
    eprintln!("这是一个错误信息"); // 输出到 stderr
    eprint!("警告: ");
    eprintln!("操作可能失败");

    // 5. 格式化宏的性能对比
    demonstrate_performance_comparison();

    // 6. 实际应用场景
    demonstrate_practical_usage();
}

/// 演示不同格式化宏的性能特点
///
/// 展示各种格式化宏在不同场景下的性能表现和适用性。
///
/// # 性能分析
///
/// - `println!` 和 `print!` 直接输出，适合调试和日志
/// - `format!` 分配内存创建字符串，适合字符串处理
/// - 编译时优化使得简单格式化几乎零成本
///
/// # Examples
///
/// ```rust
/// # use formatted_output::basic_formatting::demonstrate_performance_comparison;
/// demonstrate_performance_comparison();
/// ```
pub fn demonstrate_performance_comparison() {
    println!("\n--- 性能对比演示 ---");

    // 1. 直接输出 vs 字符串构建
    println!("直接输出: 立即显示，无内存分配");

    let built_string = format!("字符串构建: 需要内存分配，返回 String");
    println!("{}", built_string);

    // 2. 简单格式化 vs 复杂格式化
    let simple = "Hello";
    println!("简单: {}", simple); // 编译时优化

    let complex = format!(
        "复杂: {prefix}-{middle}-{suffix}",
        prefix = "start",
        middle = 42,
        suffix = "end"
    );
    println!("{}", complex);

    // 3. 批量输出场景
    println!("\n批量输出场景:");
    for i in 1..=3 {
        print!("项目{} ", i); // 连续输出
    }
    println!(); // 最后换行

    // 4. 条件格式化
    let debug_mode = true;
    if debug_mode {
        println!("调试模式: 详细信息输出");
    } else {
        println!("正常模式: 简洁输出");
    }
}

/// 演示格式化宏的实际应用场景
///
/// 展示在真实项目中如何有效使用各种格式化宏。
///
/// # 应用场景
///
/// 1. **日志记录** - 使用 `println!` 记录程序运行状态
/// 2. **错误处理** - 使用 `eprintln!` 输出错误信息
/// 3. **数据展示** - 使用 `format!` 构建展示字符串
/// 4. **调试输出** - 使用 `print!` 进行连续调试输出
///
/// # Examples
///
/// ```rust
/// # use formatted_output::basic_formatting::demonstrate_practical_usage;
/// demonstrate_practical_usage();
/// ```
pub fn demonstrate_practical_usage() {
    println!("\n--- 实际应用场景演示 ---");

    // 1. 日志记录场景
    println!("\n1. 日志记录:");
    log_info("系统启动");
    log_warning("内存使用率较高");
    log_error("数据库连接失败");

    // 2. 数据展示场景
    println!("\n2. 数据展示:");
    let user = User {
        name: "王五".to_string(),
        age: 28,
        email: "wangwu@example.com".to_string(),
    };
    display_user_info(&user);

    // 3. 进度显示场景
    println!("\n3. 进度显示:");
    show_progress(0, 100);
    show_progress(25, 100);
    show_progress(50, 100);
    show_progress(100, 100);

    // 4. 表格输出场景
    println!("\n4. 表格输出:");
    print_table_header();
    print_table_row("张三", 25, "北京");
    print_table_row("李四", 30, "上海");
    print_table_row("王五", 28, "广州");

    // 5. 交互式输出场景
    println!("\n5. 交互式输出:");
    interactive_menu();
}

/// 用户信息结构体
///
/// 用于演示数据展示场景的示例结构体。
#[derive(Debug, Clone)]
struct User {
    /// 用户姓名
    name: String,
    /// 用户年龄
    age: u32,
    /// 用户邮箱
    email: String,
}

/// 记录信息级别日志
///
/// # Arguments
///
/// * `message` - 日志消息内容
///
/// # Examples
///
/// ```rust
/// # use formatted_output::basic_formatting::log_info;
/// log_info("系统正常运行");
/// ```
pub fn log_info(message: &str) {
    println!("[INFO] {}", message);
}

/// 记录警告级别日志
///
/// # Arguments
///
/// * `message` - 警告消息内容
///
/// # Examples
///
/// ```rust
/// # use formatted_output::basic_formatting::log_warning;
/// log_warning("磁盘空间不足");
/// ```
pub fn log_warning(message: &str) {
    println!("[WARN] {}", message);
}

/// 记录错误级别日志
///
/// # Arguments
///
/// * `message` - 错误消息内容
///
/// # Examples
///
/// ```rust
/// # use formatted_output::basic_formatting::log_error;
/// log_error("网络连接超时");
/// ```
pub fn log_error(message: &str) {
    eprintln!("[ERROR] {}", message);
}

/// 显示用户信息
///
/// 使用格式化输出展示用户的详细信息。
///
/// # Arguments
///
/// * `user` - 用户信息引用
///
/// # Examples
///
/// ```rust
/// # use formatted_output::basic_formatting::{User, display_user_info};
/// let user = User {
///     name: "测试用户".to_string(),
///     age: 25,
///     email: "test@example.com".to_string(),
/// };
/// display_user_info(&user);
/// ```
pub fn display_user_info(user: &User) {
    let info = format!(
        "用户信息 - 姓名: {}, 年龄: {}, 邮箱: {}",
        user.name, user.age, user.email
    );
    println!("{}", info);
}

/// 显示进度条
///
/// 使用字符输出模拟进度条效果。
///
/// # Arguments
///
/// * `current` - 当前进度值
/// * `total` - 总进度值
///
/// # Examples
///
/// ```rust
/// # use formatted_output::basic_formatting::show_progress;
/// show_progress(50, 100);  // 显示 50% 进度
/// ```
pub fn show_progress(current: u32, total: u32) {
    let percentage = (current * 100) / total;
    let bar_length = 20;
    let filled = (current * bar_length) / total;

    print!("进度: [");
    for i in 0..bar_length {
        if i < filled {
            print!("█");
        } else {
            print!("░");
        }
    }
    println!("] {}%", percentage);
}

/// 打印表格头部
///
/// 输出格式化的表格头部信息。
///
/// # Examples
///
/// ```rust
/// # use formatted_output::basic_formatting::print_table_header;
/// print_table_header();
/// ```
pub fn print_table_header() {
    println!("{:-<40}", ""); // 分隔线
    println!("| {:<10} | {:<5} | {:<10} |", "姓名", "年龄", "城市");
    println!("{:-<40}", ""); // 分隔线
}

/// 打印表格行
///
/// 输出格式化的表格行数据。
///
/// # Arguments
///
/// * `name` - 姓名
/// * `age` - 年龄
/// * `city` - 城市
///
/// # Examples
///
/// ```rust
/// # use formatted_output::basic_formatting::print_table_row;
/// print_table_row("张三", 25, "北京");
/// ```
pub fn print_table_row(name: &str, age: u32, city: &str) {
    println!("| {:<10} | {:<5} | {:<10} |", name, age, city);
}

/// 显示交互式菜单
///
/// 演示交互式界面的格式化输出。
///
/// # Examples
///
/// ```rust
/// # use formatted_output::basic_formatting::interactive_menu;
/// interactive_menu();
/// ```
pub fn interactive_menu() {
    println!("请选择操作:");
    println!("1. 查看用户信息");
    println!("2. 修改用户信息");
    println!("3. 删除用户");
    println!("0. 退出");
    print!("请输入选项 (0-3): ");
    io::stdout().flush().unwrap(); // 确保提示符立即显示
}

/// 演示格式化宏的边界情况和注意事项
///
/// 展示在使用格式化宏时需要注意的特殊情况和最佳实践。
///
/// # 注意事项
///
/// 1. **参数数量匹配** - 占位符数量必须与参数数量匹配
/// 2. **类型兼容性** - 参数类型必须支持相应的格式化特征
/// 3. **性能考虑** - 避免在循环中频繁使用 `format!`
/// 4. **内存管理** - `format!` 会分配新的字符串内存
///
/// # Examples
///
/// ```rust
/// # use formatted_output::basic_formatting::demonstrate_edge_cases;
/// demonstrate_edge_cases();
/// ```
pub fn demonstrate_edge_cases() {
    println!("\n--- 边界情况和注意事项 ---");

    // 1. 空格式字符串
    println!(""); // 输出空行

    // 2. 只有文本，无占位符
    println!("纯文本输出，无需参数");

    // 3. 转义大括号
    println!("输出大括号: {{}} 和 {{}}"); // 输出: {}

    // 4. 混合转义和占位符
    println!("混合: {{前缀}} {} {{后缀}}", "中间值");

    // 5. 长字符串格式化
    let long_text = format!(
        "这是一个很长的字符串，包含多个参数: {}, {}, {}, {}",
        "参数1", "参数2", "参数3", "参数4"
    );
    println!("{}", long_text);

    // 6. 特殊字符处理
    println!("特殊字符: \n换行 \t制表符 \\反斜杠 \"引号");

    // 7. Unicode 支持
    println!("Unicode 支持: 🦀 Rust 语言 🚀");

    // 8. 数值边界情况
    println!("最大整数: {}", i64::MAX);
    println!("最小整数: {}", i64::MIN);
    println!("浮点数: {}", f64::MAX);
}

/// 格式化宏使用的最佳实践指南
///
/// 提供在实际项目中使用格式化宏的最佳实践建议。
///
/// # 最佳实践
///
/// 1. **选择合适的宏**
///    - 调试输出使用 `println!`
///    - 错误信息使用 `eprintln!`
///    - 字符串构建使用 `format!`
///
/// 2. **性能优化**
///    - 避免在热路径中使用 `format!`
///    - 考虑使用 `write!` 宏写入缓冲区
///    - 预分配字符串容量
///
/// 3. **可读性**
///    - 使用命名参数提高可读性
///    - 适当使用换行和缩进
///    - 添加注释说明复杂格式
///
/// 4. **国际化**
///    - 考虑文本本地化需求
///    - 使用参数化格式支持翻译
///    - 注意文本方向和字符编码
///
/// # Examples
///
/// ```rust
/// # use formatted_output::basic_formatting::demonstrate_best_practices;
/// demonstrate_best_practices();
/// ```
pub fn demonstrate_best_practices() {
    println!("\n--- 最佳实践指南 ---");

    // 1. 使用命名参数提高可读性
    println!(
        "用户 {name} (ID: {id}) 在 {date} 登录",
        name = "张三",
        id = 12345,
        date = "2024-01-15"
    );

    // 2. 结构化日志输出
    let log_entry = format!(
        "[{timestamp}] {level}: {message} (module: {module})",
        timestamp = "2024-01-15 10:30:00",
        level = "INFO",
        message = "用户登录成功",
        module = "auth"
    );
    println!("{}", log_entry);

    // 3. 条件格式化
    let debug = true;
    let user_id = 12345;

    if debug {
        println!("[DEBUG] 处理用户 ID: {}", user_id);
    }

    // 4. 批量格式化优化
    let mut output = String::with_capacity(1000); // 预分配容量
    for i in 1..=5 {
        output.push_str(&format!("项目 {}: 处理完成\n", i));
    }
    print!("{}", output);

    // 5. 错误信息格式化
    let error_code = 404;
    let resource = "用户信息";
    eprintln!(
        "错误 {code}: 无法找到 {resource}",
        code = error_code,
        resource = resource
    );

    println!("\n格式化最佳实践要点:");
    println!("  ✓ 选择合适的输出宏");
    println!("  ✓ 使用命名参数提高可读性");
    println!("  ✓ 考虑性能影响");
    println!("  ✓ 处理特殊字符和 Unicode");
    println!("  ✓ 结构化日志和错误信息");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_macro() {
        let result = format!("Hello, {}!", "World");
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_named_parameters() {
        let result = format!("{name} is {age} years old", name = "Alice", age = 30);
        assert_eq!(result, "Alice is 30 years old");
    }

    #[test]
    fn test_user_display() {
        let user = User {
            name: "Test User".to_string(),
            age: 25,
            email: "test@example.com".to_string(),
        };

        // 测试用户信息格式化
        let info = format!(
            "用户信息 - 姓名: {}, 年龄: {}, 邮箱: {}",
            user.name, user.age, user.email
        );

        assert!(info.contains("Test User"));
        assert!(info.contains("25"));
        assert!(info.contains("test@example.com"));
    }

    #[test]
    fn test_progress_calculation() {
        // 测试进度计算逻辑
        let current = 50;
        let total = 100;
        let percentage = (current * 100) / total;
        assert_eq!(percentage, 50);
    }

    #[test]
    fn test_escape_braces() {
        let result = format!("{{}} and {}", "value");
        assert_eq!(result, "{} and value");
    }
}
