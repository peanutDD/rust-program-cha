//! # Rust 对齐和填充详解
//!
//! 本模块深入介绍 Rust 格式化系统中的对齐和填充功能，包括宽度控制、
//! 对齐方式、填充字符等高级格式化特性。
//!
//! ## 对齐和填充语法
//!
//! Rust 的对齐和填充遵循以下语法结构：
//! ```text
//! {[argument]:[fill]align[width]}
//! ```
//!
//! ### 语法组件详解
//!
//! - `fill` - 填充字符（默认为空格，可以是任意单个字符）
//! - `align` - 对齐方式：
//!   - `<` - 左对齐（默认）
//!   - `>` - 右对齐
//!   - `^` - 居中对齐
//! - `width` - 最小宽度（可以是数字或参数）
//!
//! ## 设计原理
//!
//! Rust 的对齐和填充系统设计体现了以下特点：
//! 1. **灵活性** - 支持多种对齐方式和填充字符
//! 2. **一致性** - 所有格式化类型都支持对齐和填充
//! 3. **性能** - 编译时优化，运行时高效
//! 4. **可读性** - 语法简洁，易于理解和使用

use std::collections::HashMap;

/// 演示基本对齐功能
///
/// 展示左对齐、右对齐、居中对齐的基本用法和效果。
///
/// # 对齐方式说明
///
/// | 符号 | 对齐方式 | 说明 | 示例 |
/// |------|----------|------|------|
/// | `<` | 左对齐 | 内容靠左，右侧填充 | `"hello     "` |
/// | `>` | 右对齐 | 内容靠右，左侧填充 | `"     hello"` |
/// | `^` | 居中对齐 | 内容居中，两侧填充 | `"  hello   "` |
///
/// # Examples
///
/// ```rust
/// # use formatted_output::alignment_padding::demonstrate_basic_alignment;
/// demonstrate_basic_alignment();
/// ```
pub fn demonstrate_basic_alignment() {
    println!("\n=== 基本对齐功能演示 ===");

    let text = "Hello";
    let width = 15;

    // 1. 左对齐（默认）
    println!("\n--- 左对齐 ---");
    println!("默认对齐: '{}'", format!("{:<width$}", text, width = width));
    println!("显式左对齐: '{}'", format!("{:<15}", text));

    // 2. 右对齐
    println!("\n--- 右对齐 ---");
    println!("右对齐: '{}'", format!("{:>15}", text));
    println!("数字右对齐: '{}'", format!("{:>10}", 42));

    // 3. 居中对齐
    println!("\n--- 居中对齐 ---");
    println!("居中对齐: '{}'", format!("{:^15}", text));
    println!("奇数宽度: '{}'", format!("{:^14}", text));

    // 4. 不同数据类型的对齐
    println!("\n--- 不同数据类型对齐 ---");
    println!("字符串左对齐: '{}'", format!("{:<12}", "Rust"));
    println!("数字右对齐: '{}'", format!("{:>12}", 12345));
    println!("浮点数居中: '{}'", format!("{:^12}", 3.14159));
    println!("布尔值左对齐: '{}'", format!("{:<12}", true));

    // 5. 动态宽度对齐
    println!("\n--- 动态宽度对齐 ---");
    let items = vec!["短", "中等长度", "这是一个很长的文本内容"];
    let max_width = items.iter().map(|s| s.chars().count()).max().unwrap_or(0);

    for item in items {
        println!(
            "左对齐: '{}'",
            format!("{:<width$}", item, width = max_width)
        );
        println!(
            "右对齐: '{}'",
            format!("{:>width$}", item, width = max_width)
        );
        println!("居中: '{}'", format!("{:^width$}", item, width = max_width));
        println!("{:-<width$}", "", width = max_width + 10); // 分隔线
    }
}

/// 演示填充字符功能
///
/// 展示如何使用不同的填充字符来美化输出格式。
///
/// # 填充字符特性
///
/// 1. **单字符限制** - 填充字符必须是单个字符
/// 2. **Unicode 支持** - 支持任意 Unicode 字符作为填充
/// 3. **组合使用** - 可以与对齐方式组合使用
/// 4. **视觉效果** - 用于创建表格、分隔线等视觉元素
///
/// # Examples
///
/// ```rust
/// # use formatted_output::alignment_padding::demonstrate_fill_characters;
/// demonstrate_fill_characters();
/// ```
pub fn demonstrate_fill_characters() {
    println!("\n=== 填充字符功能演示 ===");

    let text = "Rust";

    // 1. 基本填充字符
    println!("\n--- 基本填充字符 ---");
    println!("星号填充: '{}'", format!("{:*^20}", text));
    println!("等号填充: '{}'", format!("{:=^20}", text));
    println!("减号填充: '{}'", format!("{:-^20}", text));
    println!("点号填充: '{}'", format!("{:.^20}", text));
    println!("下划线填充: '{}'", format!("{:_^20}", text));

    // 2. 特殊字符填充
    println!("\n--- 特殊字符填充 ---");
    println!("波浪线填充: '{}'", format!("{:~^20}", text));
    println!("井号填充: '{}'", format!("{:#^20}", text));
    println!("加号填充: '{}'", format!("{:+^20}", text));
    println!("空格填充: '{}'", format!("{: ^20}", text));

    // 3. Unicode 字符填充
    println!("\n--- Unicode 字符填充 ---");
    println!("心形填充: '{}'", format!("{:♥^20}", text));
    println!("星星填充: '{}'", format!("{:★^20}", text));
    println!("箭头填充: '{}'", format!("{:→^20}", text));
    println!("圆点填充: '{}'", format!("{:●^20}", text));

    // 4. 数字格式的填充
    println!("\n--- 数字格式填充 ---");
    let number = 42;
    println!("零填充: '{}'", format!("{:0>8}", number));
    println!("空格填充: '{}'", format!("{: >8}", number));
    println!("星号填充: '{}'", format!("{:*>8}", number));

    // 5. 创建装饰性边框
    println!("\n--- 装饰性边框 ---");
    create_decorative_box("重要通知", "系统将在今晚进行维护");
    create_decorative_box("用户信息", "张三 | 软件工程师 | 北京");

    // 6. 表格样式填充
    println!("\n--- 表格样式填充 ---");
    create_table_with_fill();
}

/// 创建装饰性文本框
///
/// # Arguments
///
/// * `title` - 标题文本
/// * `content` - 内容文本
fn create_decorative_box(title: &str, content: &str) {
    let width = 50;

    // 顶部边框
    println!("{:═^width$}", "", width = width);

    // 标题行
    println!("║{:^width$}║", title, width = width - 2);

    // 分隔线
    println!("{:─^width$}", "", width = width);

    // 内容行
    println!("║{:^width$}║", content, width = width - 2);

    // 底部边框
    println!("{:═^width$}", "", width = width);
    println!();
}

/// 创建带填充的表格
fn create_table_with_fill() {
    let data = vec![
        ("ID", "姓名", "部门", "薪资"),
        ("001", "张三", "技术部", "15000"),
        ("002", "李四", "市场部", "12000"),
        ("003", "王五", "人事部", "10000"),
    ];

    // 表格头部
    println!("{:─^60}", " 员工信息表 ");

    for (i, (id, name, dept, salary)) in data.iter().enumerate() {
        if i == 0 {
            // 表头
            println!("│{:^8}│{:^12}│{:^12}│{:^12}│", id, name, dept, salary);
            println!("{:─^60}", "");
        } else {
            // 数据行
            println!("│{:^8}│{:^12}│{:^12}│{:>12}│", id, name, dept, salary);
        }
    }

    // 表格底部
    println!("{:─^60}", "");
}

/// 演示宽度控制功能
///
/// 展示如何精确控制输出宽度，包括固定宽度和动态宽度。
///
/// # 宽度控制特性
///
/// 1. **固定宽度** - 使用数字指定固定宽度
/// 2. **动态宽度** - 使用参数指定运行时宽度
/// 3. **最小宽度** - 内容超出宽度时不会被截断
/// 4. **字符计数** - 按字符数而非字节数计算宽度
///
/// # Examples
///
/// ```rust
/// # use formatted_output::alignment_padding::demonstrate_width_control;
/// demonstrate_width_control();
/// ```
pub fn demonstrate_width_control() {
    println!("\n=== 宽度控制功能演示 ===");

    // 1. 固定宽度控制
    println!("\n--- 固定宽度控制 ---");
    let items = vec!["短", "中等长度", "这是一个很长的文本"];

    for item in &items {
        println!("宽度5: '{}'", format!("{:<5}", item));
        println!("宽度10: '{}'", format!("{:<10}", item));
        println!("宽度20: '{}'", format!("{:<20}", item));
        println!("{:-<25}", "");
    }

    // 2. 动态宽度控制
    println!("\n--- 动态宽度控制 ---");
    let widths = vec![8, 12, 16, 20];
    let text = "Dynamic";

    for width in widths {
        println!(
            "宽度{}: '{}'",
            width,
            format!("{:^width$}", text, width = width)
        );
    }

    // 3. 内容超出宽度的处理
    println!("\n--- 内容超出宽度处理 ---");
    let long_text = "这是一个非常长的文本内容，超出了指定的宽度";

    println!("原始文本: '{}'", long_text);
    println!("宽度10: '{}'", format!("{:<10}", long_text));
    println!("宽度20: '{}'", format!("{:<20}", long_text));
    println!("宽度30: '{}'", format!("{:<30}", long_text));

    // 4. 数字宽度控制
    println!("\n--- 数字宽度控制 ---");
    let numbers = vec![1, 42, 999, 1234, 99999];

    for num in numbers {
        println!(
            "数字{}: 左对齐'{}' 右对齐'{}' 零填充'{}'",
            num,
            format!("{:<8}", num),
            format!("{:>8}", num),
            format!("{:08}", num)
        );
    }

    // 5. 响应式宽度布局
    println!("\n--- 响应式宽度布局 ---");
    create_responsive_layout();

    // 6. 列对齐示例
    println!("\n--- 列对齐示例 ---");
    create_column_alignment_demo();
}

/// 创建响应式宽度布局
fn create_responsive_layout() {
    let data = vec![
        ("用户名", "邮箱", "注册日期"),
        ("张三", "zhangsan@example.com", "2024-01-01"),
        ("李四", "lisi@company.org", "2024-01-02"),
        ("王五", "wangwu@domain.net", "2024-01-03"),
    ];

    // 计算每列的最大宽度
    let mut col_widths = vec![0; 3];
    for row in &data {
        col_widths[0] = col_widths[0].max(row.0.chars().count());
        col_widths[1] = col_widths[1].max(row.1.chars().count());
        col_widths[2] = col_widths[2].max(row.2.chars().count());
    }

    // 添加一些额外空间
    for width in &mut col_widths {
        *width += 2;
    }

    println!("响应式表格布局:");
    for (i, (col1, col2, col3)) in data.iter().enumerate() {
        if i == 0 {
            // 表头
            println!(
                "┌{:─<width1$}┬{:─<width2$}┬{:─<width3$}┐",
                "",
                "",
                "",
                width1 = col_widths[0],
                width2 = col_widths[1],
                width3 = col_widths[2]
            );
            println!(
                "│{:^width1$}│{:^width2$}│{:^width3$}│",
                col1,
                col2,
                col3,
                width1 = col_widths[0],
                width2 = col_widths[1],
                width3 = col_widths[2]
            );
            println!(
                "├{:─<width1$}┼{:─<width2$}┼{:─<width3$}┤",
                "",
                "",
                "",
                width1 = col_widths[0],
                width2 = col_widths[1],
                width3 = col_widths[2]
            );
        } else {
            // 数据行
            println!(
                "│{:<width1$}│{:<width2$}│{:^width3$}│",
                col1,
                col2,
                col3,
                width1 = col_widths[0],
                width2 = col_widths[1],
                width3 = col_widths[2]
            );
        }
    }
    println!(
        "└{:─<width1$}┴{:─<width2$}┴{:─<width3$}┘",
        "",
        "",
        "",
        width1 = col_widths[0],
        width2 = col_widths[1],
        width3 = col_widths[2]
    );
}

/// 创建列对齐演示
fn create_column_alignment_demo() {
    let financial_data = vec![
        ("项目", "收入", "支出", "利润"),
        ("产品A", "50000.00", "30000.00", "20000.00"),
        ("产品B", "75000.50", "45000.25", "30000.25"),
        ("产品C", "120000.75", "80000.50", "40000.25"),
        ("总计", "245000.25", "155000.75", "90000.50"),
    ];

    println!("财务报表 - 列对齐演示:");
    println!("{:═^60}", "");

    for (i, (project, income, expense, profit)) in financial_data.iter().enumerate() {
        if i == 0 {
            // 表头 - 居中对齐
            println!(
                "║{:^12}║{:^12}║{:^12}║{:^12}║",
                project, income, expense, profit
            );
            println!("{:═^60}", "");
        } else if i == financial_data.len() - 1 {
            // 总计行 - 特殊格式
            println!("{:─^60}", "");
            println!(
                "║{:^12}║{:>12}║{:>12}║{:>12}║",
                project, income, expense, profit
            );
        } else {
            // 数据行 - 项目左对齐，数字右对齐
            println!(
                "║{:<12}║{:>12}║{:>12}║{:>12}║",
                project, income, expense, profit
            );
        }
    }
    println!("{:═^60}", "");
}

/// 演示高级对齐和填充技巧
///
/// 展示复杂场景下的对齐和填充应用，包括多级对齐、条件格式化等。
///
/// # 高级技巧
///
/// 1. **多级对齐** - 在不同层级应用不同的对齐规则
/// 2. **条件格式化** - 根据内容动态调整格式
/// 3. **组合效果** - 多种格式化技巧的组合使用
/// 4. **性能优化** - 高效的格式化实现方式
///
/// # Examples
///
/// ```rust
/// # use formatted_output::alignment_padding::demonstrate_advanced_techniques;
/// demonstrate_advanced_techniques();
/// ```
pub fn demonstrate_advanced_techniques() {
    println!("\n=== 高级对齐和填充技巧 ===");

    // 1. 多级对齐
    println!("\n--- 多级对齐 ---");
    create_hierarchical_menu();

    // 2. 条件格式化
    println!("\n--- 条件格式化 ---");
    create_conditional_formatting();

    // 3. 进度条效果
    println!("\n--- 进度条效果 ---");
    create_progress_bars();

    // 4. 数据对比表
    println!("\n--- 数据对比表 ---");
    create_comparison_table();

    // 5. 日志格式化
    println!("\n--- 日志格式化 ---");
    create_formatted_logs();
}

/// 创建分层菜单
fn create_hierarchical_menu() {
    let menu_items = vec![
        (0, "主菜单"),
        (1, "文件操作"),
        (2, "新建文件"),
        (2, "打开文件"),
        (2, "保存文件"),
        (1, "编辑操作"),
        (2, "复制"),
        (2, "粘贴"),
        (2, "撤销"),
        (1, "帮助"),
        (2, "关于"),
    ];

    println!("分层菜单结构:");
    for (level, item) in menu_items {
        let indent = "  ".repeat(level);
        let prefix = match level {
            0 => "═══",
            1 => "├──",
            2 => "│  └─",
            _ => "   ",
        };

        println!("{}{} {}", indent, prefix, item);
    }
}

/// 创建条件格式化
fn create_conditional_formatting() {
    let scores = vec![
        ("张三", 95),
        ("李四", 87),
        ("王五", 76),
        ("赵六", 92),
        ("钱七", 68),
    ];

    println!("学生成绩单 - 条件格式化:");
    println!("{:═^40}", " 成绩报告 ");

    for (name, score) in scores {
        let (grade, fill_char, align) = match score {
            90..=100 => ("优秀", '★', '^'),
            80..=89 => ("良好", '●', '^'),
            70..=79 => ("及格", '○', '<'),
            _ => ("不及格", '×', '>'),
        };

        match align {
            '^' => println!("{} {:^8} 分数: {:>3} ({})", name, "", score, grade),
            '<' => println!("{} {:<8} 分数: {:>3} ({})", name, "", score, grade),
            '>' => println!("{} {:>8} 分数: {:>3} ({})", name, "", score, grade),
            _ => println!("{} {:8} 分数: {:>3} ({})", name, "", score, grade),
        }
    }
    println!("{:═^40}", "");
}

/// 创建进度条效果
fn create_progress_bars() {
    let tasks = vec![
        ("下载文件", 100),
        ("安装程序", 75),
        ("配置系统", 45),
        ("测试功能", 20),
        ("清理缓存", 0),
    ];

    println!("任务进度条:");
    for (task, progress) in tasks {
        let bar_width = 30;
        let filled = (progress * bar_width) / 100;
        let empty = bar_width - filled;

        let bar = format!(
            "{:█<filled$}{:░<empty$}",
            "",
            "",
            filled = filled,
            empty = empty
        );

        println!("{:<12} [{}] {:>3}%", task, bar, progress);
    }
}

/// 创建数据对比表
fn create_comparison_table() {
    let comparison_data = vec![
        ("指标", "产品A", "产品B", "产品C"),
        ("性能", "★★★★☆", "★★★☆☆", "★★★★★"),
        ("价格", "¥999", "¥1299", "¥1599"),
        ("评分", "4.2", "3.8", "4.7"),
        ("销量", "1.2K", "0.8K", "2.1K"),
    ];

    println!("产品对比表:");
    println!("{:┬^50}", " 产品对比分析 ");

    for (i, (metric, a, b, c)) in comparison_data.iter().enumerate() {
        if i == 0 {
            // 表头
            println!("┌{:─^10}┬{:─^12}┬{:─^12}┬{:─^12}┐", "", "", "", "");
            println!("│{:^10}│{:^12}│{:^12}│{:^12}│", metric, a, b, c);
            println!("├{:─^10}┼{:─^12}┼{:─^12}┼{:─^12}┤", "", "", "", "");
        } else {
            println!("│{:<10}│{:^12}│{:^12}│{:^12}│", metric, a, b, c);
        }
    }
    println!("└{:─^10}┴{:─^12}┴{:─^12}┴{:─^12}┘", "", "", "", "");
}

/// 创建格式化日志
fn create_formatted_logs() {
    let log_entries = vec![
        ("INFO", "2024-01-15 10:30:00", "system", "应用启动成功"),
        ("WARN", "2024-01-15 10:30:15", "auth", "用户登录尝试失败"),
        ("ERROR", "2024-01-15 10:30:30", "database", "连接超时"),
        ("DEBUG", "2024-01-15 10:30:45", "cache", "缓存命中率: 85%"),
        ("INFO", "2024-01-15 10:31:00", "api", "请求处理完成"),
    ];

    println!("系统日志 - 格式化输出:");
    println!("{:═^80}", " 系统运行日志 ");

    for (level, timestamp, module, message) in log_entries {
        let (level_color, level_fill) = match level {
            "INFO" => ("INFO ", ' '),
            "WARN" => ("WARN ", '!'),
            "ERROR" => ("ERROR", '×'),
            "DEBUG" => ("DEBUG", '?'),
            _ => ("UNKNOWN", ' '),
        };

        println!(
            "[{}] {:^7} [{:^10}] {:<40} {}",
            timestamp, "", module, message, level_color
        );
    }
    println!("{:═^80}", "");
}

/// 演示对齐和填充的最佳实践
///
/// 提供使用对齐和填充功能的最佳实践指南和性能优化建议。
///
/// # 最佳实践
///
/// 1. **一致性原则** - 在同一应用中保持格式一致
/// 2. **可读性优先** - 选择提高可读性的对齐方式
/// 3. **性能考虑** - 避免过度复杂的格式化
/// 4. **国际化支持** - 考虑不同语言的显示需求
///
/// # Examples
///
/// ```rust
/// # use formatted_output::alignment_padding::demonstrate_best_practices;
/// demonstrate_best_practices();
/// ```
pub fn demonstrate_best_practices() {
    println!("\n=== 对齐和填充最佳实践 ===");

    // 1. 数据表格的最佳实践
    println!("\n--- 数据表格最佳实践 ---");
    create_best_practice_table();

    // 2. 用户界面布局
    println!("\n--- 用户界面布局 ---");
    create_ui_layout_example();

    // 3. 报告格式化
    println!("\n--- 报告格式化 ---");
    create_report_format();

    // 4. 性能优化示例
    println!("\n--- 性能优化示例 ---");
    demonstrate_performance_tips();

    println!("\n对齐和填充最佳实践要点:");
    println!("  ✓ 数字右对齐，文本左对齐");
    println!("  ✓ 表头居中对齐增强视觉效果");
    println!("  ✓ 使用一致的填充字符");
    println!("  ✓ 考虑内容长度动态调整宽度");
    println!("  ✓ 重要信息使用特殊对齐突出显示");
    println!("  ✓ 避免过度装饰影响可读性");
}

/// 创建最佳实践表格
fn create_best_practice_table() {
    let employee_data = vec![
        ("ID", "姓名", "部门", "薪资", "入职日期"),
        ("E001", "张三", "技术部", "15000", "2023-01-15"),
        ("E002", "李四", "市场部", "12000", "2023-03-20"),
        ("E003", "王五", "人事部", "10000", "2023-05-10"),
        ("E004", "赵六", "财务部", "13000", "2023-07-01"),
    ];

    println!("员工信息表 - 最佳实践格式:");

    // 表格边框
    println!("┌{:─^6}┬{:─^8}┬{:─^10}┬{:─^8}┬{:─^12}┐", "", "", "", "", "");

    for (i, (id, name, dept, salary, date)) in employee_data.iter().enumerate() {
        if i == 0 {
            // 表头 - 居中对齐
            println!(
                "│{:^6}│{:^8}│{:^10}│{:^8}│{:^12}│",
                id, name, dept, salary, date
            );
            println!("├{:─^6}┼{:─^8}┼{:─^10}┼{:─^8}┼{:─^12}┤", "", "", "", "", "");
        } else {
            // 数据行 - ID居中，姓名左对齐，部门左对齐，薪资右对齐，日期居中
            println!(
                "│{:^6}│{:<8}│{:<10}│{:>8}│{:^12}│",
                id, name, dept, salary, date
            );
        }
    }

    // 表格底部
    println!("└{:─^6}┴{:─^8}┴{:─^10}┴{:─^8}┴{:─^12}┘", "", "", "", "", "");
}

/// 创建用户界面布局示例
fn create_ui_layout_example() {
    println!("用户界面布局示例:");

    // 标题栏
    println!("{:═^60}", " 系统控制面板 ");

    // 状态信息
    let status_items = vec![
        ("系统状态", "运行中", "✓"),
        ("CPU使用率", "45%", "○"),
        ("内存使用", "2.1GB/8GB", "○"),
        ("磁盘空间", "156GB/500GB", "○"),
        ("网络连接", "已连接", "✓"),
    ];

    for (item, value, status) in status_items {
        println!("│ {:<12} │ {:>15} │ {:^3} │", item, value, status);
    }

    println!("{:═^60}", "");
}

/// 创建报告格式
fn create_report_format() {
    println!("月度销售报告:");
    println!("{:═^70}", " 2024年1月销售报告 ");

    // 摘要信息
    let summary = vec![
        ("总销售额", "¥1,234,567.89"),
        ("订单数量", "1,456"),
        ("平均订单价值", "¥847.92"),
        ("客户数量", "892"),
        ("退货率", "2.3%"),
    ];

    println!("\n{:─^70}", " 销售摘要 ");
    for (metric, value) in summary {
        println!("│ {:<20} │ {:>25} │", metric, value);
    }

    // 产品销售排行
    println!("\n{:─^70}", " 产品销售排行 ");
    let products = vec![
        (1, "笔记本电脑", 156, "¥234,567.89"),
        (2, "智能手机", 289, "¥345,678.90"),
        (3, "平板电脑", 134, "¥123,456.78"),
        (4, "智能手表", 267, "¥89,012.34"),
        (5, "耳机", 445, "¥56,789.01"),
    ];

    println!(
        "│{:^4}│{:^12}│{:^8}│{:^15}│",
        "排名", "产品名称", "销量", "销售额"
    );
    println!("{:─^70}", "");

    for (rank, product, quantity, amount) in products {
        println!(
            "│{:^4}│{:<12}│{:>8}│{:>15}│",
            rank, product, quantity, amount
        );
    }

    println!("{:═^70}", "");
}

/// 演示性能优化技巧
fn demonstrate_performance_tips() {
    println!("性能优化技巧:");

    // 1. 预分配字符串容量
    let mut output = String::with_capacity(1000);

    // 2. 批量格式化
    let data = vec![("项目1", 100), ("项目2", 200), ("项目3", 300)];

    for (name, value) in data {
        output.push_str(&format!("{:<10} {:>8}\n", name, value));
    }

    println!("批量格式化结果:");
    print!("{}", output);

    // 3. 重用格式字符串
    const TABLE_ROW_FORMAT: &str = "│{:<12}│{:>10}│{:^8}│";

    println!("重用格式字符串:");
    println!("│{:<12}│{:>10}│{:^8}│", "名称", "数值", "状态");
    println!("│{:<12}│{:>10}│{:^8}│", "测试1", "100", "✓");
    println!("│{:<12}│{:>10}│{:^8}│", "测试2", "200", "✓");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_alignment() {
        assert_eq!(format!("{:<5}", "hi"), "hi   ");
        assert_eq!(format!("{:>5}", "hi"), "   hi");
        assert_eq!(format!("{:^5}", "hi"), " hi  ");
    }

    #[test]
    fn test_fill_characters() {
        assert_eq!(format!("{:*^5}", "hi"), "*hi**");
        assert_eq!(format!("{:0>5}", "42"), "00042");
        assert_eq!(format!("{:-<5}", "x"), "x----");
    }

    #[test]
    fn test_width_control() {
        assert_eq!(format!("{:5}", "hello"), "hello");
        assert_eq!(format!("{:3}", "hello"), "hello"); // 不截断
        assert_eq!(format!("{:width$}", "test", width = 8), "test    ");
    }

    #[test]
    fn test_number_alignment() {
        assert_eq!(format!("{:>8}", 42), "      42");
        assert_eq!(format!("{:08}", 42), "00000042");
        assert_eq!(format!("{:^8}", 42), "   42   ");
    }

    #[test]
    fn test_mixed_alignment() {
        let result = format!(
            "{name:<10} {value:>8} {status:^6}",
            name = "test",
            value = 123,
            status = "ok"
        );
        assert_eq!(result, "test            123   ok  ");
    }

    #[test]
    fn test_unicode_fill() {
        assert_eq!(format!("{:★^7}", "hi"), "★★hi★★★");
        assert_eq!(format!("{:♥<5}", "x"), "x♥♥♥♥");
    }

    #[test]
    fn test_dynamic_width() {
        let width = 10;
        let result = format!("{:^width$}", "test", width = width);
        assert_eq!(result, "   test   ");
    }
}
