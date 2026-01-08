//! # Rust 所有权系统全面指南模块
//!
//! 本模块提供了 Rust 所有权系统的全面解析，包括：
//! - 所有权的核心概念和规则
//! - 移动语义和复制语义
//! - 引用与借用机制
//! - 生命周期管理
//! - 闭包的详细解释
//! - 实际应用案例和最佳实践

pub mod fundamentals;
pub mod move_semantics;
pub mod references_borrowing;
pub mod lifetime_management;
pub mod closures;
pub mod smart_pointers;
pub mod practical_examples;
pub mod best_practices;

// 重新导出主要函数
pub use fundamentals::ownership_fundamentals;
pub use move_semantics::move_semantics_deep_dive;
pub use references_borrowing::references_and_borrowing;
pub use lifetime_management::lifetime_management;
pub use closures::closure_comprehensive_explanation;
pub use smart_pointers::smart_pointers_and_ownership;
pub use practical_examples::practical_ownership_examples;
pub use best_practices::common_mistakes_and_best_practices;

/// # Rust 所有权系统全面学习指南
///
/// 所有权是 Rust 最独特和核心的特性，它使得 Rust 能够在不使用垃圾回收器的情况下保证内存安全。
pub fn comprehensive_ownership_guide() {
  println!("🦀 Rust 所有权系统全面学习指南");
  println!("{}", "=".repeat(80));

  // 第一部分：所有权基础理论
  ownership_fundamentals();

  // 第二部分：移动语义深入解析
  move_semantics_deep_dive();

  // 第三部分：引用与借用机制
  references_and_borrowing();

  // 第四部分：生命周期管理
  lifetime_management();

  // 第五部分：闭包详细解释
  closure_comprehensive_explanation();

  // 第六部分：智能指针与所有权
  smart_pointers_and_ownership();

  // 第七部分：实际应用案例
  practical_ownership_examples();

  // 第八部分：常见错误与最佳实践
  common_mistakes_and_best_practices();

  println!("\n{}", "=".repeat(80));
  println!("✅ 所有权系统学习完成！掌握了 Rust 的核心特性。");
}

