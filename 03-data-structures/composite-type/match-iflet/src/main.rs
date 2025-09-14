//! Rust match 和 if let 模式匹配学习项目
//!
//! 本项目提供全面的 Rust 模式匹配教程和实践练习
//! 基于 https://course.rs/basic/match-pattern/match-if-let.html

mod exercises;
mod match_iflet_comprehensive;

use std::io::{self, Write};

/// 主函数 - 程序入口点
fn main() {
    print_welcome();

    loop {
        print_menu();

        let choice = get_user_input("请选择学习模式 (输入数字): ");

        match choice.trim() {
            "1" => {
                println!("\n🎯 开始学习 match 和 if let 基础教程...");
                match_iflet_comprehensive::demonstrate_all();
            }
            "2" => {
                println!("\n💪 开始递进式练习...");
                run_exercises();
            }
            "3" => {
                println!("\n🧪 运行所有测试用例...");
                run_tests();
            }
            "4" => {
                println!("\n📚 显示学习总结...");
                show_learning_summary();
            }
            "5" => {
                println!("\n👋 感谢使用 Rust 模式匹配学习项目！");
                break;
            }
            _ => {
                println!("\n❌ 无效选择，请输入 1-5 之间的数字。");
            }
        }

        println!("\n{}", "=".repeat(60));
    }
}

/// 打印欢迎信息
fn print_welcome() {
    println!("{}", "=".repeat(60));
    println!("🦀 欢迎使用 Rust match 和 if let 模式匹配学习项目！");
    println!("{}", "=".repeat(60));
    println!("📖 本项目基于 Rust Course 官方教程");
    println!("🔗 参考: https://course.rs/basic/match-pattern/match-if-let.html");
    println!("{}", "=".repeat(60));
}

/// 打印主菜单
fn print_menu() {
    println!("\n📋 学习模式选择:");
    println!("  1️⃣  基础教程 - match 和 if let 全面讲解");
    println!("  2️⃣  实践练习 - 递进式编程练习");
    println!("  3️⃣  测试验证 - 运行所有测试用例");
    println!("  4️⃣  学习总结 - 知识点回顾");
    println!("  5️⃣  退出程序");
    println!("{}", "-".repeat(40));
}

/// 获取用户输入
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input
}

/// 运行所有练习
fn run_exercises() {
    println!("\n🚀 开始运行递进式练习...");

    // 练习1：基础 match 表达式
    exercises::exercise1_basic_match::simulate_traffic_light();

    // 练习2：带数据的枚举和解构
    exercises::exercise2_enum_with_data::geometry_calculations();

    // 练习3：if let 和 Option 处理
    exercises::exercise3_if_let_option::student_management_demo();

    // 练习4：while let 和迭代器处理
    exercises::exercise4_while_let_iterator::task_processing_demo();

    // 练习5：matches! 宏和复杂模式匹配
    exercises::exercise5_matches_macro::log_analysis_demo();

    // 练习6：综合应用 - 配置文件解析器
    exercises::exercise6_config_parser::config_parsing_demo();

    println!("\n✅ 所有练习运行完成！");
}

/// 运行测试用例
fn run_tests() {
    println!("\n🧪 运行测试用例...");
    println!("💡 提示: 请在终端中运行 'cargo test' 来执行所有测试");

    // 这里我们可以手动运行一些简单的测试
    test_basic_functionality();
}

/// 基础功能测试
fn test_basic_functionality() {
    println!("\n🔍 运行基础功能测试:");

    // 测试 Option 处理
    let some_value = Some(42);
    let none_value: Option<i32> = None;

    match some_value {
        Some(x) => println!("✅ Option::Some 测试通过: {}", x),
        None => println!("❌ Option::Some 测试失败"),
    }

    if let None = none_value {
        println!("✅ Option::None 测试通过");
    } else {
        println!("❌ Option::None 测试失败");
    }

    // 测试 Result 处理
    let ok_result: Result<i32, &str> = Ok(100);
    let err_result: Result<i32, &str> = Err("错误信息");

    match ok_result {
        Ok(value) => println!("✅ Result::Ok 测试通过: {}", value),
        Err(e) => println!("❌ Result::Ok 测试失败: {}", e),
    }

    if let Err(error) = err_result {
        println!("✅ Result::Err 测试通过: {}", error);
    } else {
        println!("❌ Result::Err 测试失败");
    }

    // 测试 matches! 宏
    let number = 42;
    if matches!(number, 40..=50) {
        println!("✅ matches! 宏测试通过: {} 在范围 40-50 内", number);
    } else {
        println!("❌ matches! 宏测试失败");
    }

    println!("\n🎉 基础功能测试完成！");
}

/// 显示学习总结
fn show_learning_summary() {
    println!("\n📚 Rust match 和 if let 模式匹配学习总结");
    println!("{}", "=".repeat(50));

    println!("\n🎯 核心知识点:");
    println!("  📌 match 表达式");
    println!("     • 穷尽性匹配 - 必须覆盖所有可能的值");
    println!("     • 通配符 _ - 匹配任何值但不绑定");
    println!("     • 守卫 (Guard) - 在模式中添加额外条件");
    println!("     • @ 绑定 - 在匹配的同时绑定值");
    println!("     • 多模式匹配 - 使用 | 分隔多个模式");

    println!("\n  📌 if let 表达式");
    println!("     • 简化的模式匹配 - 只关心一种情况");
    println!("     • 可选的 else 分支");
    println!("     • 适用于 Option 和 Result 类型");

    println!("\n  📌 while let 循环");
    println!("     • 条件循环 - 当模式匹配成功时继续");
    println!("     • 常用于迭代器处理");

    println!("\n  📌 matches! 宏");
    println!("     • 返回布尔值的模式匹配");
    println!("     • 适用于条件判断");
    println!("     • 支持复杂模式和范围");

    println!("\n  📌 解构模式");
    println!("     • 元组解构 - (a, b, c)");
    println!("     • 结构体解构 - Point {{ x, y }}");
    println!("     • 数组/切片解构 - [first, rest @ ..]");
    println!("     • 枚举解构 - Some(value), Ok(data)");

    println!("\n🚀 实际应用场景:");
    println!("  🔧 错误处理 - Result<T, E> 类型");
    println!("  🔧 可选值处理 - Option<T> 类型");
    println!("  🔧 状态机实现 - 枚举状态转换");
    println!("  🔧 配置解析 - 复杂数据结构处理");
    println!("  🔧 HTTP 请求处理 - 路由和状态码");
    println!("  🔧 JSON 数据处理 - 动态类型解析");

    println!("\n💡 最佳实践:");
    println!("  ✨ 优先使用 match 进行穷尽性检查");
    println!("  ✨ 使用 if let 简化单一模式匹配");
    println!("  ✨ 利用 matches! 宏进行条件判断");
    println!("  ✨ 合理使用通配符 _ 避免未使用变量警告");
    println!("  ✨ 使用守卫添加复杂条件逻辑");
    println!("  ✨ 通过 @ 绑定在匹配时保留原值");

    println!("\n🎓 学习建议:");
    println!("  📖 从简单的 Option 和 Result 匹配开始");
    println!("  📖 逐步学习复杂的解构模式");
    println!("  📖 多练习实际项目中的应用场景");
    println!("  📖 理解编译器的穷尽性检查机制");
    println!("  📖 掌握性能优化技巧");

    println!("\n🔗 相关资源:");
    println!("  📚 Rust Book: https://doc.rust-lang.org/book/ch06-02-match.html");
    println!("  📚 Rust Course: https://course.rs/basic/match-pattern/match-if-let.html");
    println!(
        "  📚 Rust Reference: https://doc.rust-lang.org/reference/expressions/match-expr.html"
    );

    println!("{}", "=".repeat(50));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_matching() {
        let some_value = Some(42);
        let result = match some_value {
            Some(x) if x > 40 => "大于40",
            Some(_) => "小于等于40",
            None => "空值",
        };
        assert_eq!(result, "大于40");
    }

    #[test]
    fn test_if_let_syntax() {
        let config = Some("debug");
        let mut result = String::new();

        if let Some(mode) = config {
            result = format!("模式: {}", mode);
        }

        assert_eq!(result, "模式: debug");
    }

    #[test]
    fn test_matches_macro() {
        let number = 42;
        assert!(matches!(number, 40..=50));
        assert!(!matches!(number, 0..=10));

        let option = Some("test");
        assert!(matches!(option, Some(_)));
    }

    #[test]
    fn test_while_let_loop() {
        let mut stack = vec![1, 2, 3];
        let mut sum = 0;

        while let Some(value) = stack.pop() {
            sum += value;
        }

        assert_eq!(sum, 6);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_destructuring_patterns() {
        let tuple = (1, 2, 3);
        let (a, b, c) = tuple;
        assert_eq!(a + b + c, 6);

        let array = [1, 2, 3, 4, 5];
        if let [first, second, ..] = array {
            assert_eq!(first + second, 3);
        }
    }

    #[test]
    fn test_guard_patterns() {
        let number = 42;
        let result = match number {
            x if x < 0 => "负数",
            x if x == 0 => "零",
            x if x < 50 => "小正数",
            _ => "大正数",
        };
        assert_eq!(result, "小正数");
    }

    #[test]
    fn test_at_binding() {
        let number = 42;
        let result = match number {
            x @ 40..=50 => format!("范围内的值: {}", x),
            _ => "范围外的值".to_string(),
        };
        assert_eq!(result, "范围内的值: 42");
    }

    #[test]
    fn test_multiple_patterns() {
        let character = 'a';
        let result = match character {
            'a' | 'e' | 'i' | 'o' | 'u' => "元音",
            _ => "辅音",
        };
        assert_eq!(result, "元音");
    }
}
