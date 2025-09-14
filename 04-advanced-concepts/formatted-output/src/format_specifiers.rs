//! # Rust 格式说明符详解
//!
//! 本模块深入介绍 Rust 格式化系统中的格式说明符，包括占位符语法、
//! 位置参数、命名参数等高级用法。
//!
//! ## 格式说明符语法
//!
//! Rust 的格式说明符遵循以下语法结构：
//! ```text
//! {[argument][:[fill]align][sign][#][0][width][.precision][type]}
//! ```
//!
//! ### 语法组件说明
//!
//! - `argument` - 参数标识符（位置索引或命名参数）
//! - `fill` - 填充字符（默认为空格）
//! - `align` - 对齐方式（`<` 左对齐，`>` 右对齐，`^` 居中）
//! - `sign` - 符号显示（`+` 总是显示符号，`-` 仅负数显示符号）
//! - `#` - 替代格式标志
//! - `0` - 零填充标志
//! - `width` - 最小宽度
//! - `precision` - 精度控制
//! - `type` - 格式类型（如 `x` 十六进制，`b` 二进制等）
//!
//! ## 设计哲学
//!
//! Rust 的格式说明符设计体现了以下原则：
//! 1. **明确性** - 每个格式选项都有明确的语义
//! 2. **可组合性** - 不同格式选项可以灵活组合
//! 3. **类型安全** - 编译时检查格式与类型的兼容性
//! 4. **性能优先** - 格式化代码经过编译时优化

use std::collections::HashMap;

/// 演示格式说明符的基础用法
///
/// 展示 Rust 中各种格式说明符的语法和使用方法。
///
/// # 占位符类型
///
/// | 占位符 | 说明 | 示例 | 输出 |
/// |--------|------|------|------|
/// | `{}` | 默认格式 | `format!("{}", 42)` | `"42"` |
/// | `{:?}` | Debug 格式 | `format!("{:?}", vec![1,2])` | `"[1, 2]"` |
/// | `{:#?}` | 美化 Debug | `format!("{:#?}", vec![1,2])` | 多行格式 |
/// | `{:x}` | 小写十六进制 | `format!("{:x}", 255)` | `"ff"` |
/// | `{:X}` | 大写十六进制 | `format!("{:X}", 255)` | `"FF"` |
/// | `{:b}` | 二进制 | `format!("{:b}", 8)` | `"1000"` |
/// | `{:o}` | 八进制 | `format!("{:o}", 8)` | `"10"` |
///
/// # Examples
///
/// ```rust
/// # use formatted_output::format_specifiers::demonstrate_basic_specifiers;
/// demonstrate_basic_specifiers();
/// ```
pub fn demonstrate_basic_specifiers() {
    println!("\n=== 格式说明符基础用法演示 ===");

    // 1. 基本占位符
    println!("\n--- 基本占位符 ---");
    println!("默认格式: {}", 42);
    println!("字符串: {}", "Hello, Rust!");
    println!("浮点数: {}", 3.14159);
    println!("布尔值: {}", true);

    // 2. Debug 格式
    println!("\n--- Debug 格式 ---");
    let vec = vec![1, 2, 3, 4, 5];
    println!("Debug 格式: {:?}", vec);
    println!("美化 Debug: {:#?}", vec);

    let map = {
        let mut m = HashMap::new();
        m.insert("name", "Rust");
        m.insert("type", "Language");
        m
    };
    println!("HashMap Debug: {:#?}", map);

    // 3. 数值格式
    println!("\n--- 数值格式 ---");
    let number = 255;
    println!("十进制: {}", number);
    println!("二进制: {:b}", number);
    println!("八进制: {:o}", number);
    println!("小写十六进制: {:x}", number);
    println!("大写十六进制: {:X}", number);

    // 4. 浮点数格式
    println!("\n--- 浮点数格式 ---");
    let pi = std::f64::consts::PI;
    println!("默认: {}", pi);
    println!("科学计数法: {:e}", pi);
    println!("大写科学计数法: {:E}", pi);

    // 5. 特殊值处理
    println!("\n--- 特殊值处理 ---");
    println!("无穷大: {}", f64::INFINITY);
    println!("负无穷大: {}", f64::NEG_INFINITY);
    println!("非数值: {}", f64::NAN);
}

/// 演示位置参数的使用
///
/// 展示如何使用数字索引来指定参数位置，实现参数重用和重排序。
///
/// # 位置参数特性
///
/// 1. **索引从 0 开始** - 第一个参数索引为 0
/// 2. **可以重复使用** - 同一个参数可以多次引用
/// 3. **可以重新排序** - 参数顺序可以与占位符顺序不同
/// 4. **混合使用** - 可以与自动索引混合使用（但不推荐）
///
/// # Examples
///
/// ```rust
/// # use formatted_output::format_specifiers::demonstrate_positional_arguments;
/// demonstrate_positional_arguments();
/// ```
pub fn demonstrate_positional_arguments() {
    println!("\n=== 位置参数演示 ===");

    // 1. 基本位置参数
    println!("\n--- 基本位置参数 ---");
    println!("{0} {1} {2}", "第一个", "第二个", "第三个");

    // 2. 参数重用
    println!("\n--- 参数重用 ---");
    println!("{0} 和 {0} 是相同的", "Rust");
    println!("{0} + {1} = {2}, {1} + {0} = {2}", 3, 5, 8);

    // 3. 参数重排序
    println!("\n--- 参数重排序 ---");
    println!("正常顺序: {0} {1} {2}", "A", "B", "C");
    println!("反向顺序: {2} {1} {0}", "A", "B", "C");
    println!("自定义顺序: {1} {0} {2} {1}", "X", "Y", "Z");

    // 4. 复杂位置参数示例
    println!("\n--- 复杂位置参数 ---");
    let name = "张三";
    let age = 25;
    let city = "北京";
    let job = "工程师";

    println!(
        "{0} 是一名 {3}，今年 {1} 岁，住在 {2}。{0} 在 {2} 工作。",
        name, age, city, job
    );

    // 5. 位置参数与格式说明符结合
    println!("\n--- 位置参数与格式说明符 ---");
    let number = 42;
    println!(
        "数字 {0} 的不同表示: 十进制={0}, 二进制={0:b}, 十六进制={0:x}",
        number
    );

    // 6. 多类型位置参数
    println!("\n--- 多类型位置参数 ---");
    println!(
        "用户 {0} (ID: {1}) 的余额是 {2:.2} 元，状态: {3}",
        "李四", 12345, 1234.567, true
    );
}

/// 演示命名参数的使用
///
/// 展示如何使用命名参数提高代码可读性和维护性。
///
/// # 命名参数优势
///
/// 1. **可读性强** - 参数名称直接表达含义
/// 2. **维护性好** - 修改参数顺序不影响格式字符串
/// 3. **错误减少** - 减少参数位置错误的可能性
/// 4. **自文档化** - 格式字符串本身就是文档
///
/// # Examples
///
/// ```rust
/// # use formatted_output::format_specifiers::demonstrate_named_arguments;
/// demonstrate_named_arguments();
/// ```
pub fn demonstrate_named_arguments() {
    println!("\n=== 命名参数演示 ===");

    // 1. 基本命名参数
    println!("\n--- 基本命名参数 ---");
    println!(
        "Hello, {name}! You are {age} years old.",
        name = "Alice",
        age = 30
    );

    // 2. 复杂命名参数
    println!("\n--- 复杂命名参数 ---");
    println!(
        "{product} 的价格是 {price:.2} 元，库存 {stock} 件，折扣 {discount}%",
        product = "笔记本电脑",
        price = 5999.99,
        stock = 15,
        discount = 10
    );

    // 3. 命名参数重用
    println!("\n--- 命名参数重用 ---");
    println!(
        "{language} 是一门系统编程语言。学习 {language} 需要时间，但 {language} 值得学习。",
        language = "Rust"
    );

    // 4. 命名参数与格式说明符
    println!("\n--- 命名参数与格式说明符 ---");
    println!(
        "十六进制颜色: #{red:02X}{green:02X}{blue:02X}",
        red = 255,
        green = 128,
        blue = 0
    );

    // 5. 结构化数据展示
    println!("\n--- 结构化数据展示 ---");
    let user_info = format!(
        "用户档案:\n\
         姓名: {name}\n\
         年龄: {age}\n\
         邮箱: {email}\n\
         电话: {phone}\n\
         地址: {address}",
        name = "王五",
        age = 28,
        email = "wangwu@example.com",
        phone = "138-0000-0000",
        address = "北京市朝阳区"
    );
    println!("{}", user_info);

    // 6. 日志格式化
    println!("\n--- 日志格式化 ---");
    log_with_named_params("INFO", "用户登录", "auth", "2024-01-15 10:30:00", 12345);

    log_with_named_params(
        "ERROR",
        "数据库连接失败",
        "database",
        "2024-01-15 10:31:00",
        0,
    );
}

/// 使用命名参数记录日志
///
/// # Arguments
///
/// * `level` - 日志级别
/// * `message` - 日志消息
/// * `module` - 模块名称
/// * `timestamp` - 时间戳
/// * `user_id` - 用户ID
fn log_with_named_params(level: &str, message: &str, module: &str, timestamp: &str, user_id: u32) {
    println!(
        "[{timestamp}] {level} [{module}] {message} (user_id: {user_id})",
        timestamp = timestamp,
        level = level,
        module = module,
        message = message,
        user_id = user_id
    );
}

/// 演示混合参数使用
///
/// 展示位置参数、命名参数和自动参数的混合使用方式。
///
/// # 混合使用规则
///
/// 1. **位置参数优先** - 位置参数必须在命名参数之前
/// 2. **自动索引限制** - 使用位置参数后不能使用自动索引
/// 3. **命名参数独立** - 命名参数不影响位置参数索引
/// 4. **类型一致性** - 所有参数类型必须与格式说明符兼容
///
/// # Examples
///
/// ```rust
/// # use formatted_output::format_specifiers::demonstrate_mixed_arguments;
/// demonstrate_mixed_arguments();
/// ```
pub fn demonstrate_mixed_arguments() {
    println!("\n=== 混合参数演示 ===");

    // 1. 位置参数 + 命名参数
    println!("\n--- 位置参数 + 命名参数 ---");
    println!(
        "{0} 在 {city} 工作，年龄 {1} 岁，职业是 {job}",
        "张三",
        30,
        city = "上海",
        job = "软件工程师"
    );

    // 2. 复杂混合格式
    println!("\n--- 复杂混合格式 ---");
    let report = format!(
        "报告摘要:\n\
         项目: {0}\n\
         状态: {status}\n\
         进度: {1}%\n\
         负责人: {manager}\n\
         截止日期: {2}",
        "网站重构",
        75,
        "2024-03-01",
        status = "进行中",
        manager = "李经理"
    );
    println!("{}", report);

    // 3. 数据表格格式
    println!("\n--- 数据表格格式 ---");
    print_mixed_table_header();
    print_mixed_table_row(1, "产品A", 99.99, "有库存", 100);
    print_mixed_table_row(2, "产品B", 149.50, "缺货", 0);
    print_mixed_table_row(3, "产品C", 79.99, "有库存", 50);

    // 4. 错误信息格式化
    println!("\n--- 错误信息格式化 ---");
    format_error_message(
        "ValidationError",
        "用户输入验证失败",
        "email",
        "invalid_format",
        400,
    );

    format_error_message(
        "DatabaseError",
        "数据库连接超时",
        "connection",
        "timeout",
        500,
    );
}

/// 打印混合参数表格头部
fn print_mixed_table_header() {
    println!("{:-<60}", "");
    println!(
        "| {id:^4} | {name:^10} | {price:^8} | {status:^8} | {stock:^6} |",
        id = "ID",
        name = "名称",
        price = "价格",
        status = "状态",
        stock = "库存"
    );
    println!("{:-<60}", "");
}

/// 打印混合参数表格行
fn print_mixed_table_row(id: u32, name: &str, price: f64, status: &str, stock: u32) {
    println!(
        "| {0:^4} | {name:^10} | {1:^8.2} | {status:^8} | {2:^6} |",
        id,
        price,
        stock,
        name = name,
        status = status
    );
}

/// 格式化错误信息
fn format_error_message(error_type: &str, message: &str, field: &str, code: &str, status: u16) {
    eprintln!(
        "[{0}] {message} | 字段: {field} | 错误码: {code} | HTTP状态: {1}",
        error_type,
        status,
        message = message,
        field = field,
        code = code
    );
}

/// 演示格式说明符的高级特性
///
/// 展示格式说明符的高级用法，包括条件格式化、动态参数等。
///
/// # 高级特性
///
/// 1. **条件格式化** - 根据条件选择不同的格式
/// 2. **动态宽度** - 运行时确定格式宽度
/// 3. **嵌套格式化** - 格式字符串中包含格式化结果
/// 4. **类型转换** - 自动类型转换和格式适配
///
/// # Examples
///
/// ```rust
/// # use formatted_output::format_specifiers::demonstrate_advanced_features;
/// demonstrate_advanced_features();
/// ```
pub fn demonstrate_advanced_features() {
    println!("\n=== 高级特性演示 ===");

    // 1. 条件格式化
    println!("\n--- 条件格式化 ---");
    let scores = vec![95, 87, 76, 92, 68];
    for (i, score) in scores.iter().enumerate() {
        let grade = if *score >= 90 {
            "优秀"
        } else if *score >= 80 {
            "良好"
        } else if *score >= 70 {
            "及格"
        } else {
            "不及格"
        };

        println!(
            "学生 {student}: {score} 分 ({grade})",
            student = i + 1,
            score = score,
            grade = grade
        );
    }

    // 2. 动态宽度格式化
    println!("\n--- 动态宽度格式化 ---");
    let items = vec![
        ("短名称", 100),
        ("这是一个很长的名称", 200),
        ("中等长度名称", 150),
    ];

    // 计算最大宽度
    let max_width = items.iter().map(|(name, _)| name.len()).max().unwrap_or(0);

    for (name, value) in items {
        println!(
            "{name:<width$} | {value:>8}",
            name = name,
            value = value,
            width = max_width
        );
    }

    // 3. 嵌套格式化
    println!("\n--- 嵌套格式化 ---");
    let user_data = vec![
        ("张三", 25, "北京"),
        ("李四", 30, "上海"),
        ("王五", 28, "广州"),
    ];

    for (name, age, city) in user_data {
        let user_info = format!("{}({}岁)", name, age);
        println!(
            "用户信息: {info:<15} 所在城市: {city}",
            info = user_info,
            city = city
        );
    }

    // 4. 数值范围格式化
    println!("\n--- 数值范围格式化 ---");
    let numbers = vec![1, 42, 999, 1234, 99999];
    for num in numbers {
        let formatted = match num {
            n if n < 10 => format!("{:04}", n),   // 0001
            n if n < 100 => format!("{:04}", n),  // 0042
            n if n < 1000 => format!("{:04}", n), // 0999
            n => format!("{}", n),                // 1234
        };
        println!("数字 {} 格式化为: {}", num, formatted);
    }

    // 5. 百分比和比率格式化
    println!("\n--- 百分比和比率格式化 ---");
    let ratios = vec![0.1, 0.25, 0.5, 0.75, 0.9, 1.0];
    for ratio in ratios {
        println!(
            "比率: {ratio:.2} = {percentage:.1}% = {fraction}",
            ratio = ratio,
            percentage = ratio * 100.0,
            fraction = format!("{}/{}", (ratio * 10.0) as i32, 10)
        );
    }
}

/// 演示格式说明符的最佳实践
///
/// 提供使用格式说明符的最佳实践指南和常见陷阱避免方法。
///
/// # 最佳实践
///
/// 1. **优先使用命名参数** - 提高代码可读性
/// 2. **避免过度复杂** - 保持格式字符串简洁
/// 3. **一致性原则** - 在项目中保持格式风格一致
/// 4. **性能考虑** - 避免在热路径中使用复杂格式
///
/// # 常见陷阱
///
/// 1. **参数数量不匹配** - 占位符与参数数量不一致
/// 2. **类型不兼容** - 格式说明符与参数类型不匹配
/// 3. **索引越界** - 位置参数索引超出范围
/// 4. **混合使用错误** - 不正确的参数混合使用
///
/// # Examples
///
/// ```rust
/// # use formatted_output::format_specifiers::demonstrate_best_practices;
/// demonstrate_best_practices();
/// ```
pub fn demonstrate_best_practices() {
    println!("\n=== 最佳实践指南 ===");

    // 1. 推荐的命名参数使用
    println!("\n--- 推荐做法 ---");

    // ✓ 好的做法：使用命名参数
    println!(
        "订单 {order_id}: {product} x{quantity} = ¥{total:.2}",
        order_id = "ORD-001",
        product = "笔记本电脑",
        quantity = 2,
        total = 11999.98
    );

    // ✓ 好的做法：结构化格式
    let config_info = format!(
        "配置信息:\n\
         - 服务器: {server}\n\
         - 端口: {port}\n\
         - 数据库: {database}\n\
         - 超时: {timeout}ms",
        server = "localhost",
        port = 8080,
        database = "myapp",
        timeout = 5000
    );
    println!("{}", config_info);

    // 2. 避免的做法
    println!("\n--- 应避免的做法 ---");

    // ✗ 不好的做法：过多的位置参数（仅作演示）
    println!(
        "不推荐: {} {} {} {} {} {}",
        "参数1", "参数2", "参数3", "参数4", "参数5", "参数6"
    );

    // ✓ 改进：使用命名参数
    println!(
        "推荐: {first} {second} {third} {fourth} {fifth} {sixth}",
        first = "参数1",
        second = "参数2",
        third = "参数3",
        fourth = "参数4",
        fifth = "参数5",
        sixth = "参数6"
    );

    // 3. 性能优化建议
    println!("\n--- 性能优化建议 ---");

    // ✓ 预分配字符串容量
    let mut result = String::with_capacity(200);
    for i in 1..=5 {
        result.push_str(&format!("项目 {}: 完成\n", i));
    }
    print!("{}", result);

    // ✓ 使用常量格式字符串
    println!(
        "[{timestamp}] {level}: {message}",
        timestamp = "2024-01-15 10:30:00",
        level = "INFO",
        message = "应用启动成功"
    );

    // 4. 错误处理最佳实践
    println!("\n--- 错误处理最佳实践 ---");

    // ✓ 结构化错误信息
    eprintln!(
        "错误详情: {error_type} | 消息: {message} | 位置: {location} | 时间: {timestamp}",
        error_type = "ValidationError",
        message = "用户名不能为空",
        location = "user_service.rs:42",
        timestamp = "2024-01-15 10:30:00"
    );

    // 5. 国际化考虑
    println!("\n--- 国际化考虑 ---");

    // ✓ 参数化消息模板
    println!(
        "{user} 在 {date} 执行了 {action}",
        user = "张三",
        date = "2024年1月15日",
        action = "登录操作"
    );

    // 总结
    println!("\n格式说明符最佳实践要点:");
    println!("  ✓ 优先使用命名参数提高可读性");
    println!("  ✓ 保持格式字符串简洁明了");
    println!("  ✓ 在项目中保持一致的格式风格");
    println!("  ✓ 考虑性能影响，避免过度格式化");
    println!("  ✓ 为错误信息提供结构化格式");
    println!("  ✓ 考虑国际化和本地化需求");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positional_arguments() {
        let result = format!("{0} {1} {0}", "A", "B");
        assert_eq!(result, "A B A");
    }

    #[test]
    fn test_named_arguments() {
        let result = format!("{name} is {age} years old", name = "Alice", age = 30);
        assert_eq!(result, "Alice is 30 years old");
    }

    #[test]
    fn test_mixed_arguments() {
        let result = format!(
            "{0} lives in {city} and is {1} years old",
            "Bob",
            25,
            city = "New York"
        );
        assert_eq!(result, "Bob lives in New York and is 25 years old");
    }

    #[test]
    fn test_number_formats() {
        assert_eq!(format!("{:b}", 8), "1000");
        assert_eq!(format!("{:o}", 8), "10");
        assert_eq!(format!("{:x}", 255), "ff");
        assert_eq!(format!("{:X}", 255), "FF");
    }

    #[test]
    fn test_debug_format() {
        let vec = vec![1, 2, 3];
        let result = format!("{:?}", vec);
        assert_eq!(result, "[1, 2, 3]");
    }

    #[test]
    fn test_escape_braces() {
        let result = format!("{{}} and {}", "value");
        assert_eq!(result, "{} and value");
    }

    #[test]
    fn test_complex_formatting() {
        let result = format!(
            "User {name} (ID: {id:04}) has balance ${balance:.2}",
            name = "John",
            id = 42,
            balance = 1234.567
        );
        assert_eq!(result, "User John (ID: 0042) has balance $1234.57");
    }
}
