//! # Scope, Lifetime, and NLL 深度解析 - 主程序
//!
//! 这是一个全面分析Rust中作用域、生命周期和NLL概念的教学程序。
//! 通过运行此程序，您将看到详细的示例和解释。

use scope_lifetime_nll::*;
use std::io::{self, Write};

fn main() {
    // 清屏并显示欢迎信息
    print!("\x1B[2J\x1B[1;1H"); // ANSI转义序列清屏
    
    println!("🦀 欢迎来到 Rust Scope, Lifetime, and NLL 深度解析！");
    println!("{}", "=".repeat(60));
    
    loop {
        display_menu();
        
        let choice = get_user_input("请选择一个选项 (1-8): ");
        
        match choice.trim() {
            "1" => {
                println!("\n🔍 === 作用域 (Scope) 分析 ===");
                scope::run_scope_examples();
                pause();
            },
            "2" => {
                println!("\n⏰ === 生命周期 (Lifetime) 分析 ===");
                lifetime::run_lifetime_examples();
                pause();
            },
            "3" => {
                println!("\n🚀 === NLL (Non-Lexical Lifetimes) 分析 ===");
                nll::run_nll_examples();
                pause();
            },
            "4" => {
                println!("\n📊 === 三者对比分析 ===");
                comparison::run_comparison_examples();
                pause();
            },
            "5" => {
                println!("\n💡 === 实际应用案例 ===");
                practical_examples::run_practical_examples();
                pause();
            },
            "6" => {
                println!("\n⚠️  === 边界情况和陷阱 ===");
                edge_cases::run_edge_cases_analysis();
                pause();
            },
            "7" => {
                println!("\n✅ === 最佳实践指南 ===");
                best_practices::run_best_practices_analysis();
                pause();
            },
            "8" => {
                println!("\n🎯 === 运行所有示例 ===");
                run_all_examples();
                pause();
            },
            "0" | "q" | "quit" | "exit" => {
                println!("\n👋 感谢使用！希望您对Rust的作用域、生命周期和NLL有了更深入的理解！");
                break;
            },
            _ => {
                println!("\n❌ 无效选项，请重新选择！");
                pause();
            }
        }
        
        // 清屏准备下一轮
        print!("\x1B[2J\x1B[1;1H");
    }
}

/// 显示主菜单
fn display_menu() {
    println!("\n📚 请选择您想要学习的内容：");
    println!("┌─────────────────────────────────────────────────────┐");
    println!("│  1. 🔍 作用域 (Scope) 分析                          │");
    println!("│  2. ⏰ 生命周期 (Lifetime) 分析                     │");
    println!("│  3. 🚀 NLL (Non-Lexical Lifetimes) 分析            │");
    println!("│  4. 📊 三者对比分析                                 │");
    println!("│  5. 💡 实际应用案例                                 │");
    println!("│  6. ⚠️  边界情况和陷阱                              │");
    println!("│  7. ✅ 最佳实践指南                                 │");
    println!("│  8. 🎯 运行所有示例                                 │");
    println!("│  0. 👋 退出程序                                     │");
    println!("└─────────────────────────────────────────────────────┘");
}

/// 获取用户输入
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input
}

/// 暂停程序，等待用户按回车继续
fn pause() {
    println!("\n{}", "─".repeat(60));
    get_user_input("按回车键继续...");
}