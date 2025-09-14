//! # Rust 所有权系统全面学习指南
//!
//! 基于 https://course.rs/basic/ownership/ownership.html 的内容
//! 提供全面、深入的所有权、借用和闭包学习材料

mod advanced_ownership_examples;
mod ownership_comprehensive_guide;
mod ownership_exercises;

use advanced_ownership_examples::{common_ownership_solutions, demonstrate_real_world_ownership};
use ownership_comprehensive_guide::comprehensive_ownership_guide;
use ownership_exercises::{challenge_exercises, run_all_exercises};

fn main() {
  println!("🦀 欢迎来到 Rust 所有权系统全面学习指南！");
  println!("📚 基于 https://course.rs/basic/ownership/ownership.html 深度分析");
  println!("{}", "=".repeat(80));

  // 运行完整的所有权学习指南
  comprehensive_ownership_guide();

  // 运行实际应用场景示例
  demonstrate_real_world_ownership();

  // 运行常见问题解决方案
  common_ownership_solutions();

  // 运行练习题
  run_all_exercises();
  challenge_exercises();

  println!("\n{}", "=".repeat(80));
  println!("🎉 恭喜！您已完成 Rust 所有权系统的全面学习！");
  println!("📋 学习内容包括：");
  println!("   • 所有权基础理论与三大铁律");
  println!("   • 移动语义与借用机制详解");
  println!("   • 生命周期管理与注解");
  println!("   • 智能指针与内存管理");
  println!("   • 实际开发中的应用场景");
  println!("   • 常见错误与解决方案");
  println!("\n💡 建议：");
  println!("   • 多练习代码示例，加深理解");
  println!("   • 阅读 OWNERSHIP_DEEP_ANALYSIS.md 获取更多理论知识");
  println!("   • 运行 cargo test 验证理解");
  println!("📖 继续学习：探索更多 Rust 高级特性");
}

/// 快速概览函数
///
/// 提供所有权系统的快速概览和要点总结
#[allow(dead_code)]
fn quick_overview() {
  println!("\n⚡ Rust 所有权系统快速概览");
  println!("{}", "-".repeat(50));

  println!("\n🔑 核心概念：");
  println!("• 所有权 (Ownership) - 每个值都有一个所有者");
  println!("• 借用 (Borrowing) - 临时访问而不获取所有权");
  println!("• 生命周期 (Lifetimes) - 引用的有效范围");
  println!("• 移动语义 (Move Semantics) - 所有权转移");

  println!("\n📏 借用规则：");
  println!("• 同一时间只能有一个可变引用");
  println!("• 可以有多个不可变引用");
  println!("• 引用必须总是有效的");

  println!("\n🎯 设计目标：");
  println!("• 内存安全 - 防止悬垂指针、双重释放");
  println!("• 零成本抽象 - 编译时检查，运行时无开销");
  println!("• 并发安全 - 防止数据竞争");

  println!("\n💡 学习建议：");
  println!("• 从简单示例开始，逐步理解概念");
  println!("• 多写代码，让编译器帮助学习");
  println!("• 理解错误信息，它们是最好的老师");
  println!("• 实践中应用，加深理解");
}
