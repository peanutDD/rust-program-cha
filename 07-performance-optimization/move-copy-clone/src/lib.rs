//! # Move, Copy, and Clone 深度解析
//!
//! 本库提供了对Rust中三个核心概念的全面分析：
//! - **Move**: 移动语义和所有权转移
//! - **Copy**: Copy trait 和栈复制机制
//! - **Clone**: Clone trait 和深拷贝机制
//!
//! ## 核心概念
//!
//! ### Move (移动)
//! - 所有权转移，原变量失效
//! - 默认行为，适用于所有类型
//! - 零成本抽象，编译时优化
//!
//! ### Copy (复制)
//! - 栈上按位复制，原变量仍有效
//! - 需要实现 Copy trait
//! - 只适用于简单类型（如基本数据类型）
//!
//! ### Clone (克隆)
//! - 深拷贝，可能涉及堆内存分配
//! - 需要实现 Clone trait
//! - 适用于复杂类型，成本较高
//!
//! ## 模块结构
//!
//! - `move_semantics`: 移动语义深度解析
//! - `copy_trait`: Copy trait 机制分析
//! - `clone_trait`: Clone trait 机制分析
//! - `comparison`: 三者详细对比
//! - `practical_examples`: 实际应用案例
//! - `performance`: 性能分析和基准测试
//! - `edge_cases`: 边界情况和陷阱

pub mod move_semantics;
pub mod copy_trait;
pub mod clone_trait;
pub mod comparison;
pub mod practical_examples;
pub mod performance;
pub mod edge_cases;

/// 库的版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 库的描述信息
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// 打印库的基本信息
pub fn print_library_info() {
    println!("=== Move, Copy, and Clone 深度解析 ===");
    println!("版本: {}", VERSION);
    println!("描述: {}", DESCRIPTION);
    println!("\n本库将帮助您深入理解Rust中的三个核心概念：");
    println!("1. Move - 移动语义和所有权转移");
    println!("2. Copy - Copy trait 和栈复制机制");
    println!("3. Clone - Clone trait 和深拷贝机制");
    println!("\n通过实际案例和性能分析，您将掌握这些概念的区别和最佳使用场景。");
    println!("================================================\n");
}

/// 运行所有示例的主函数
pub fn run_all_examples() {
    print_library_info();
    
    println!("🚀 1. Move 语义示例");
    move_semantics::run_move_examples();
    
    println!("\n📋 2. Copy trait 示例");
    copy_trait::run_copy_examples();
    
    println!("\n🔄 3. Clone trait 示例");
    clone_trait::run_clone_examples();
    
    println!("\n⚖️ 4. 三者对比分析");
    comparison::run_comparison_analysis();
    
    println!("\n💡 5. 实际应用案例");
    practical_examples::practical_examples();
    
    println!("\n⚡ 6. 性能分析");
    performance::comprehensive_performance_comparison();
    
    println!("\n⚠️ 7. 边界情况和陷阱");
    edge_cases::run_all_edge_cases();
}

/// 快速演示三者区别的函数
pub fn quick_demo() {
    println!("=== Move, Copy, Clone 快速演示 ===");
    
    // Move 示例
    println!("\n1. Move 示例:");
    let s1 = String::from("Hello");
    let s2 = s1; // s1 被移动到 s2
    // println!("{}", s1); // 这行会编译错误，因为 s1 已被移动
    println!("s2: {}", s2);
    
    // Copy 示例
    println!("\n2. Copy 示例:");
    let x = 42;
    let y = x; // x 被复制到 y，x 仍然有效
    println!("x: {}, y: {}", x, y);
    
    // Clone 示例
    println!("\n3. Clone 示例:");
    let v1 = vec![1, 2, 3];
    let v2 = v1.clone(); // 显式克隆 v1
    println!("v1: {:?}, v2: {:?}", v1, v2);
    
    println!("\n=== 演示完成 ===\n");
}