//! # Rust 流程控制学习项目
//!
//! 基于 https://course.rs/basic/flow-control.html 的全面流程控制教程和实践
//!
//! 本项目包含：
//! - 全面的流程控制概念讲解
//! - 递进式练习题
//! - 实际应用案例
//! - 性能优化技巧

mod exercises;
mod flow_control_comprehensive;

use std::io::{self, Write};

fn main() {
    print_welcome();

    loop {
        print_menu();

        let choice = get_user_input("请选择学习模式 (1-4): ");

        match choice.trim() {
            "1" => {
                println!("\n🎯 开始流程控制教程学习...");
                flow_control_comprehensive::run_all_examples();
            }
            "2" => {
                println!("\n🎯 开始流程控制练习...");
                exercises::run_all_exercises();
            }
            "3" => {
                println!("\n🎯 综合学习模式...");
                flow_control_comprehensive::run_all_examples();
                println!("\n{}", "=".repeat(60));
                exercises::run_all_exercises();
            }
            "4" => {
                println!("\n👋 感谢使用 Rust 流程控制学习项目！");
                print_learning_summary();
                break;
            }
            _ => {
                println!("\n❌ 无效选择，请重新输入！");
            }
        }

        println!("\n按 Enter 键继续...");
        let _ = io::stdin().read_line(&mut String::new());
    }
}

fn print_welcome() {
    println!("{}", "=".repeat(60));
    println!("🦀 Rust 流程控制学习项目");
    println!("{}", "=".repeat(60));
    println!("📚 基于 course.rs 官方教程的全面流程控制学习资源");
    println!("🎯 涵盖条件语句、循环、模式匹配等核心概念");
    println!("💡 包含丰富的实例和递进式练习");
    println!("{}", "=".repeat(60));
}

fn print_menu() {
    println!("\n📋 学习模式选择:");
    println!("  1️⃣  流程控制教程 - 全面概念讲解");
    println!("  2️⃣  流程控制练习 - 实践应用");
    println!("  3️⃣  综合学习 - 教程 + 练习");
    println!("  4️⃣  退出程序");
    println!("{}", "-".repeat(40));
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn print_learning_summary() {
    println!("\n📊 学习总结:");
    println!("{}", "=".repeat(50));

    println!("\n🎯 核心知识点:");
    println!("  ✅ 条件语句 (if/else, match)");
    println!("  ✅ 循环语句 (loop, while, for)");
    println!("  ✅ 循环控制 (break, continue, 标签)");
    println!("  ✅ 模式匹配 (解构、守卫、嵌套)");
    println!("  ✅ 流程控制最佳实践");

    println!("\n🏋️ 实践练习:");
    println!("  ✅ 条件语句基础应用");
    println!("  ✅ 循环算法实现");
    println!("  ✅ 循环控制技巧");
    println!("  ✅ 模式匹配进阶");
    println!("  ✅ 综合应用案例");

    println!("\n🚀 实际应用场景:");
    println!("  ✅ 数据处理和分析");
    println!("  ✅ 算法实现");
    println!("  ✅ 状态机设计");
    println!("  ✅ 错误处理");
    println!("  ✅ 系统编程");

    println!("\n💡 学习建议:");
    println!("  📖 理解每种流程控制的适用场景");
    println!("  🔄 多练习复杂的嵌套控制结构");
    println!("  🎯 掌握模式匹配的强大功能");
    println!("  ⚡ 注意性能和内存安全");
    println!("  🛠️  在实际项目中应用所学知识");

    println!("\n🔗 相关资源:");
    println!("  📚 Rust 官方文档: https://doc.rust-lang.org/book/");
    println!("  🎓 Course.rs: https://course.rs/");
    println!("  💻 Rust By Example: https://doc.rust-lang.org/rust-by-example/");
    println!("  🦀 Rustlings: https://github.com/rust-lang/rustlings");

    println!("\n{}", "=".repeat(50));
    println!("🎉 继续你的 Rust 学习之旅！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conditionals() {
        fn classify_number(n: i32) -> &'static str {
            if n > 0 {
                "positive"
            } else if n < 0 {
                "negative"
            } else {
                "zero"
            }
        }

        assert_eq!(classify_number(5), "positive");
        assert_eq!(classify_number(-3), "negative");
        assert_eq!(classify_number(0), "zero");
    }

    #[test]
    fn test_loop_functionality() {
        fn sum_range(start: i32, end: i32) -> i32 {
            let mut sum = 0;
            for i in start..=end {
                sum += i;
            }
            sum
        }

        assert_eq!(sum_range(1, 5), 15); // 1+2+3+4+5 = 15
        assert_eq!(sum_range(0, 0), 0);
        assert_eq!(sum_range(-2, 2), 0); // -2+-1+0+1+2 = 0
    }

    #[test]
    fn test_pattern_matching() {
        fn describe_option(opt: Option<i32>) -> String {
            match opt {
                Some(x) if x > 0 => format!("Positive: {}", x),
                Some(x) if x < 0 => format!("Negative: {}", x),
                Some(0) => "Zero".to_string(),
                Some(_) => "Other".to_string(),
                None => "Nothing".to_string(),
            }
        }

        assert_eq!(describe_option(Some(5)), "Positive: 5");
        assert_eq!(describe_option(Some(-3)), "Negative: -3");
        assert_eq!(describe_option(Some(0)), "Zero");
        assert_eq!(describe_option(None), "Nothing");
    }

    #[test]
    fn test_loop_control() {
        fn find_first_even(numbers: &[i32]) -> Option<i32> {
            for &num in numbers {
                if num % 2 == 0 {
                    return Some(num);
                }
            }
            None
        }

        assert_eq!(find_first_even(&[1, 3, 4, 5, 6]), Some(4));
        assert_eq!(find_first_even(&[1, 3, 5]), None);
        assert_eq!(find_first_even(&[2, 4, 6]), Some(2));
    }

    #[test]
    fn test_nested_loops() {
        fn matrix_sum(matrix: &[[i32; 3]; 3]) -> i32 {
            let mut sum = 0;
            for row in matrix {
                for &element in row {
                    sum += element;
                }
            }
            sum
        }

        let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

        assert_eq!(matrix_sum(&matrix), 45); // 1+2+...+9 = 45
    }

    #[test]
    fn test_while_let() {
        fn process_stack(mut stack: Vec<i32>) -> i32 {
            let mut sum = 0;
            while let Some(value) = stack.pop() {
                if value > 0 {
                    sum += value;
                }
            }
            sum
        }

        let stack = vec![1, -2, 3, -4, 5];
        assert_eq!(process_stack(stack), 9); // 1+3+5 = 9
    }

    #[test]
    fn test_match_guards() {
        fn categorize_age_and_score(age: u32, score: u32) -> &'static str {
            match (age, score) {
                (a, s) if a < 18 && s >= 90 => "Young Genius",
                (a, s) if a < 18 && s >= 70 => "Young Talent",
                (a, s) if a >= 18 && s >= 90 => "Adult Genius",
                (a, s) if a >= 18 && s >= 70 => "Adult Talent",
                (_, s) if s >= 50 => "Average",
                _ => "Needs Improvement",
            }
        }

        assert_eq!(categorize_age_and_score(16, 95), "Young Genius");
        assert_eq!(categorize_age_and_score(25, 85), "Adult Talent");
        assert_eq!(categorize_age_and_score(30, 45), "Needs Improvement");
    }

    #[test]
    fn test_labeled_loops() {
        fn find_in_matrix(matrix: &[[i32; 3]; 3], target: i32) -> Option<(usize, usize)> {
            'outer: for (i, row) in matrix.iter().enumerate() {
                for (j, &element) in row.iter().enumerate() {
                    if element == target {
                        return Some((i, j));
                    }
                }
            }
            None
        }

        let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

        assert_eq!(find_in_matrix(&matrix, 5), Some((1, 1)));
        assert_eq!(find_in_matrix(&matrix, 10), None);
    }
}
