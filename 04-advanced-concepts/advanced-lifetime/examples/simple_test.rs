//! 简单的生命周期测试示例

use advanced_lifetime::{lifetime_basics, lifetime_elision, static_lifetime};

fn main() {
    println!("🦀 Rust 生命周期学习项目 - 简单测试 🦀");
    println!("========================================");

    // 测试基础生命周期概念
    println!("\n--- 测试生命周期基础 ---");
    lifetime_basics::run_all_demonstrations();

    println!("\n--- 测试生命周期省略 ---");
    lifetime_elision::run_all_demonstrations();

    println!("\n--- 测试静态生命周期 ---");
    static_lifetime::run_all_demonstrations();

    println!("\n========================================");
    println!("✅ 简单测试完成！");
}
