//! # Rust 格式化输出完整教程
//!
//! 本项目提供了 Rust 格式化输出的全面教程，涵盖从基础到高级的所有知识点。
//! 基于 https://course.rs/basic/formatted-output.html 的内容进行深度扩展。
//!
//! ## 模块结构
//!
//! - `basic_formatting` - 基础格式化宏的使用
//! - `format_specifiers` - 格式说明符详解
//! - `alignment_padding` - 对齐和填充技巧
//! - `advanced_formatting` - 高级格式化特性
//! - `custom_formatting` - 自定义格式化实现
//! - `format_args` - 格式化参数和性能优化
//! - `practical_examples` - 实际应用案例
//!
//! ## 使用方法
//!
//! 运行 `cargo run` 查看完整的演示，或者运行 `cargo test` 执行所有测试用例。

mod advanced_formatting;
mod alignment_padding;
mod basic_formatting;
mod custom_formatting;
mod format_args;
mod format_specifiers;
mod practical_examples;

use std::io::{self, Write};

fn main() {
    println!("🦀 Rust 格式化输出完整教程");
    println!("═══════════════════════════════════════════════════════════════");
    println!("本教程将全面展示 Rust 格式化输出的各个方面，从基础到高级应用。");
    println!("基于 https://course.rs/basic/formatted-output.html 深度扩展。");
    println!();

    // 显示菜单
    show_menu();

    // 交互式演示
    interactive_demo();
}

/// 显示教程菜单
fn show_menu() {
    println!("📚 教程内容目录:");
    println!("  1️⃣  基础格式化 - println!、print!、format! 等宏的使用");
    println!("  2️⃣  格式说明符 - 占位符、参数、格式化选项");
    println!("  3️⃣  对齐填充 - 文本对齐、填充字符、宽度控制");
    println!("  4️⃣  高级格式化 - 精度、数值格式、科学计数法");
    println!("  5️⃣  自定义格式化 - 实现 Display 和 Debug trait");
    println!("  6️⃣  格式化参数 - format_args! 和性能优化");
    println!("  7️⃣  实际应用 - 日志系统、数据展示、调试输出");
    println!("  0️⃣  全部演示 - 运行所有示例");
    println!();
}

/// 交互式演示
fn interactive_demo() {
    loop {
        print!("请选择要演示的内容 (0-7, q退出): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("输入错误，请重试。");
            continue;
        }

        let choice = input.trim();

        match choice {
            "0" => {
                println!("\n🚀 开始全部演示...");
                run_all_demos();
                break;
            }
            "1" => {
                println!("\n📖 基础格式化演示");
                basic_formatting::demonstrate_basic_formatting();
                basic_formatting::demonstrate_performance_comparison();
                basic_formatting::demonstrate_practical_usage();
                basic_formatting::demonstrate_edge_cases();
                basic_formatting::demonstrate_best_practices();
            }
            "2" => {
                println!("\n🔧 格式说明符演示");
                format_specifiers::demonstrate_basic_specifiers();
                format_specifiers::demonstrate_positional_arguments();
                format_specifiers::demonstrate_named_arguments();
                format_specifiers::demonstrate_mixed_arguments();
                format_specifiers::demonstrate_advanced_features();
                format_specifiers::demonstrate_best_practices();
            }
            "3" => {
                println!("\n📐 对齐和填充演示");
                alignment_padding::demonstrate_basic_alignment();
                alignment_padding::demonstrate_fill_characters();
                alignment_padding::demonstrate_width_control();
                alignment_padding::demonstrate_advanced_techniques();
                alignment_padding::demonstrate_best_practices();
            }
            "4" => {
                println!("\n🔬 高级格式化演示");
                advanced_formatting::demonstrate_precision_control();
                advanced_formatting::demonstrate_number_formatting();
                advanced_formatting::demonstrate_scientific_notation();
                advanced_formatting::demonstrate_base_conversion();
                advanced_formatting::demonstrate_practical_applications();
            }
            "5" => {
                println!("\n🎨 自定义格式化演示");
                custom_formatting::demonstrate_basic_custom_formatting();
                custom_formatting::demonstrate_color_formatting();
                custom_formatting::demonstrate_matrix_formatting();
                custom_formatting::demonstrate_log_formatting();
                custom_formatting::demonstrate_advanced_custom_formatting();
                custom_formatting::demonstrate_best_practices();
            }
            "6" => {
                println!("\n⚡ 格式化参数演示");
                format_args::demonstrate_format_args_basics();
                format_args::demonstrate_write_macros();
                format_args::demonstrate_advanced_format_args();
                format_args::demonstrate_best_practices();
            }
            "7" => {
                println!("\n🏗️ 实际应用演示");
                practical_examples::demonstrate_logging_system();
                practical_examples::demonstrate_data_presentation();
                practical_examples::demonstrate_debugging_output();
                practical_examples::demonstrate_config_generation();
                practical_examples::demonstrate_best_practices();
            }
            "q" | "Q" => {
                println!("\n👋 感谢使用 Rust 格式化输出教程！");
                break;
            }
            _ => {
                println!("❌ 无效选择，请输入 0-7 或 q");
            }
        }

        if choice != "0" {
            println!("\n按回车键继续...");
            let mut _input = String::new();
            let _ = io::stdin().read_line(&mut _input);
        }
    }
}

/// 运行所有演示
fn run_all_demos() {
    let demos: Vec<(&str, Box<dyn Fn()>)> = vec![
        (
            "基础格式化",
            Box::new(|| {
                basic_formatting::demonstrate_basic_formatting();
                basic_formatting::demonstrate_performance_comparison();
                basic_formatting::demonstrate_practical_usage();
                basic_formatting::demonstrate_edge_cases();
                basic_formatting::demonstrate_best_practices();
            }),
        ),
        (
            "格式说明符",
            Box::new(|| {
                format_specifiers::demonstrate_basic_specifiers();
                format_specifiers::demonstrate_positional_arguments();
                format_specifiers::demonstrate_named_arguments();
                format_specifiers::demonstrate_mixed_arguments();
                format_specifiers::demonstrate_advanced_features();
                format_specifiers::demonstrate_best_practices();
            }),
        ),
        (
            "对齐和填充",
            Box::new(|| {
                alignment_padding::demonstrate_basic_alignment();
                alignment_padding::demonstrate_fill_characters();
                alignment_padding::demonstrate_width_control();
                alignment_padding::demonstrate_advanced_techniques();
                alignment_padding::demonstrate_best_practices();
            }),
        ),
        (
            "高级格式化",
            Box::new(|| {
                advanced_formatting::demonstrate_precision_control();
                advanced_formatting::demonstrate_number_formatting();
                advanced_formatting::demonstrate_scientific_notation();
                advanced_formatting::demonstrate_base_conversion();
                advanced_formatting::demonstrate_practical_applications();
            }),
        ),
        (
            "自定义格式化",
            Box::new(|| {
                custom_formatting::demonstrate_basic_custom_formatting();
                custom_formatting::demonstrate_color_formatting();
                custom_formatting::demonstrate_matrix_formatting();
                custom_formatting::demonstrate_log_formatting();
                custom_formatting::demonstrate_advanced_custom_formatting();
                custom_formatting::demonstrate_best_practices();
            }),
        ),
        (
            "格式化参数",
            Box::new(|| {
                format_args::demonstrate_format_args_basics();
                format_args::demonstrate_write_macros();
                format_args::demonstrate_advanced_format_args();
                format_args::demonstrate_best_practices();
            }),
        ),
        (
            "实际应用",
            Box::new(|| {
                practical_examples::demonstrate_logging_system();
                practical_examples::demonstrate_data_presentation();
                practical_examples::demonstrate_debugging_output();
                practical_examples::demonstrate_config_generation();
                practical_examples::demonstrate_best_practices();
            }),
        ),
    ];

    for (name, demo_fn) in demos {
        println!("\n{:═^80}", format!(" {} 演示 ", name));
        demo_fn();

        // 添加分隔和暂停
        println!("\n{:─^80}", format!(" {} 演示完成 ", name));
        println!("按回车键继续下一个演示...");
        let mut _input = String::new();
        let _ = io::stdin().read_line(&mut _input);
    }

    // 总结
    println!("\n{:═^80}", " 🎉 所有演示完成 ");
    println!("\n📋 教程总结:");
    println!("  ✅ 掌握了 Rust 格式化输出的基础语法");
    println!("  ✅ 学会了使用各种格式说明符和参数");
    println!("  ✅ 了解了对齐、填充和宽度控制技巧");
    println!("  ✅ 探索了高级格式化特性和数值处理");
    println!("  ✅ 实现了自定义类型的格式化输出");
    println!("  ✅ 优化了格式化性能和内存使用");
    println!("  ✅ 应用到了实际项目场景中");

    println!("\n🚀 现在你已经全面掌握了 Rust 格式化输出！");
    println!("💡 建议继续练习和探索更多高级用法。");

    println!("\n📚 相关资源:");
    println!("  • Rust 官方文档: https://doc.rust-lang.org/std/fmt/");
    println!("  • 格式化输出教程: https://course.rs/basic/formatted-output.html");
    println!("  • std::fmt 模块文档: https://doc.rust-lang.org/std/fmt/index.html");

    println!("\n{:═^80}", " 教程结束 ");
}
