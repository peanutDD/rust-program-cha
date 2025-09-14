//! # Rust 闭包深度解析教程 - 主程序入口
//!
//! 这是 Rust 闭包教程的主程序入口点。
//! 基于 https://course.rs/advance/functional-programing/closure.html 的全面深入分析。
//!
//! ## 使用方法
//!
//! ```bash
//! # 运行完整教程
//! cargo run
//!
//! # 运行基础教程
//! cargo run -- --basic
//!
//! # 运行高级教程
//! cargo run -- --advanced
//!
//! # 显示帮助信息
//! cargo run -- --help
//! ```

use closure_tutorial::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 解析命令行参数
    match args.get(1).map(|s| s.as_str()) {
        Some("--help") | Some("-h") => {
            print_help();
        }
        Some("--basic") | Some("-b") => {
            println!("🦀 运行 Rust 闭包基础教程");
            run_basic_demos();
        }
        Some("--advanced") | Some("-a") => {
            println!("🦀 运行 Rust 闭包高级教程");
            run_advanced_demos();
        }
        Some("--version") | Some("-v") => {
            println!("Rust 闭包教程 v{}", version());
            println!("{}", description());
        }
        Some(unknown) => {
            eprintln!("❌ 未知参数: {}", unknown);
            eprintln!("使用 --help 查看可用选项");
            std::process::exit(1);
        }
        None => {
            // 默认运行完整教程
            run_all_demos();
        }
    }
}

/// 打印帮助信息
fn print_help() {
    println!("🦀 Rust 闭包深度解析教程");
    println!("基于 https://course.rs/advance/functional-programing/closure.html");
    println!();
    println!("用法:");
    println!("  cargo run [选项]");
    println!();
    println!("选项:");
    println!("  --help, -h      显示此帮助信息");
    println!("  --basic, -b     运行基础教程（适合初学者）");
    println!("  --advanced, -a  运行高级教程（适合有经验的开发者）");
    println!("  --version, -v   显示版本信息");
    println!();
    println!("示例:");
    println!("  cargo run                # 运行完整教程");
    println!("  cargo run -- --basic     # 运行基础教程");
    println!("  cargo run -- --advanced  # 运行高级教程");
    println!();
    println!("学习路径建议:");
    println!("  1. 初学者：先运行 --basic，掌握基础概念");
    println!("  2. 进阶者：运行完整教程，全面了解闭包");
    println!("  3. 高级用户：运行 --advanced，学习高级模式");
}
