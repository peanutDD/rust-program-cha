//! # Rust 深入生命周期分析 - 主程序入口
//!
//! 这是 Rust 深入生命周期分析项目的主入口程序。
//! 提供交互式菜单，让用户可以选择运行不同的生命周期示例。

use advanced_lifetime::*;
use std::io::{self, Write};

fn main() {
    display_welcome();

    loop {
        display_menu();

        let choice = get_user_input("请选择要运行的示例 (输入数字): ");

        match choice.trim() {
            "1" => {
                println!("\n🚀 运行所有示例...");
                run_all_examples();
            }
            "2" => {
                println!("\n🌱 运行基础示例...");
                run_basic_examples();
            }
            "3" => {
                println!("\n🚀 运行高级示例...");
                run_advanced_examples();
            }
            "4" => {
                println!("\n📚 运行生命周期基础概念示例...");
                lifetime_basics::run_all_lifetime_basics_examples();
            }
            "5" => {
                println!("\n📝 运行生命周期省略规则示例...");
                lifetime_elision::run_all_elision_examples();
            }
            "6" => {
                println!("\n🔒 运行静态生命周期示例...");
                static_lifetime::run_all_static_lifetime_examples();
            }
            "7" => {
                println!("\n🚀 运行高级生命周期模式示例...");
                advanced_patterns::run_all_advanced_patterns_examples();
            }
            "8" => {
                println!("\n⛓️ 运行生命周期约束示例...");
                lifetime_bounds::run_all_lifetime_bounds_examples();
            }
            "9" => {
                println!("\n🏗️ 运行实际应用案例示例...");
                practical_examples::run_all_practical_examples();
            }
            "10" => {
                println!("\n🔧 运行常见问题和解决方案示例...");
                common_issues::run_all_common_issues_examples();
            }
            "11" => {
                display_library_info();
            }
            "12" => {
                display_learning_guide();
            }
            "0" | "q" | "quit" | "exit" => {
                println!("\n👋 感谢使用 Rust 深入生命周期分析！");
                println!("💡 记住：实践是掌握生命周期的最好方法！");
                break;
            }
            _ => {
                println!("\n❌ 无效选择，请输入 0-12 之间的数字。");
            }
        }

        if choice.trim() != "11" && choice.trim() != "12" {
            wait_for_continue();
        }
    }
}

/// 显示欢迎信息
fn display_welcome() {
    println!("\n{}", "=".repeat(60));
    println!("🦀 欢迎使用 Rust 深入生命周期分析 🦀");
    println!("{}", "=".repeat(60));
    println!("📖 基于 Rust Course 的深度扩展学习库");
    println!("🔗 参考：https://course.rs/advance/lifetime/advance.html");
    println!("{}", "=".repeat(60));
}

/// 显示主菜单
fn display_menu() {
    println!("\n📋 请选择要运行的示例:");
    println!("{}", "-".repeat(40));
    println!("  1️⃣  运行所有示例 (完整学习体验)");
    println!("  2️⃣  运行基础示例 (适合初学者)");
    println!("  3️⃣  运行高级示例 (适合进阶者)");
    println!("{}", "-".repeat(40));
    println!("  4️⃣  生命周期基础概念");
    println!("  5️⃣  生命周期省略规则");
    println!("  6️⃣  静态生命周期");
    println!("  7️⃣  高级生命周期模式");
    println!("  8️⃣  生命周期约束");
    println!("  9️⃣  实际应用案例");
    println!("  🔟 常见问题和解决方案");
    println!("{}", "-".repeat(40));
    println!("  1️⃣1️⃣ 显示库信息");
    println!("  1️⃣2️⃣ 显示学习指南");
    println!("  0️⃣  退出程序");
    println!("{}", "-".repeat(40));
}

/// 获取用户输入
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

/// 等待用户按回车继续
fn wait_for_continue() {
    println!("\n⏸️  按回车键继续...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

/// 显示库信息
fn display_library_info() {
    println!("\n{}", "=".repeat(50));
    println!("📚 库信息");
    println!("{}", "=".repeat(50));
    println!("{}", get_library_info());
    println!("\n🎯 特色功能:");
    println!("  • 全面覆盖 Rust 生命周期的所有重要概念");
    println!("  • 深度分析每个概念的理论和实践");
    println!("  • 提供大量实际应用场景的代码示例");
    println!("  • 包含性能优化和最佳实践指导");
    println!("  • 详细分析常见编译错误和解决方案");
    println!("  • 完整的单元测试覆盖");
    println!("{}", "=".repeat(50));
}

/// 显示学习指南
fn display_learning_guide() {
    println!("\n{}", "=".repeat(50));
    println!("📖 学习指南");
    println!("{}", "=".repeat(50));
    println!("\n🎯 推荐学习路径:");
    println!("  1️⃣  基础阶段：生命周期基础概念");
    println!("     理解悬垂引用、借用检查器、生命周期注解");
    println!("\n  2️⃣  规则阶段：生命周期省略规则");
    println!("     掌握三大省略规则，了解何时需要显式标注");
    println!("\n  3️⃣  特殊阶段：静态生命周期");
    println!("     深入理解 'static 的特殊性和使用场景");
    println!("\n  4️⃣  进阶阶段：高级生命周期模式");
    println!("     探索子类型、高阶生命周期等高级概念");
    println!("\n  5️⃣  约束阶段：生命周期约束");
    println!("     掌握复杂的生命周期约束场景");
    println!("\n  6️⃣  实践阶段：实际应用案例");
    println!("     通过真实案例学习生命周期的实际应用");
    println!("\n  7️⃣  问题解决：常见问题和解决方案");
    println!("     学会解决常见编译错误和性能优化");
    println!("\n💡 学习建议:");
    println!("  • 按顺序学习，每个阶段都要充分理解");
    println!("  • 多动手实践，修改示例代码");
    println!("  • 遇到编译错误时仔细阅读错误信息");
    println!("  • 在实际项目中应用学到的概念");
    println!("{}", "=".repeat(50));
}
