//! # Rust 裸指针、引用和智能指针深度解析
//!
//! 本库提供了对 Rust 中三种主要指针类型的全面分析：
//! - 裸指针 (Raw Pointers): `*const T` 和 `*mut T`
//! - 引用 (References): `&T` 和 `&mut T`
//! - 智能指针 (Smart Pointers): `Box<T>`, `Rc<T>`, `Arc<T>`, `RefCell<T>` 等
//!
//! ## 模块结构
//!
//! - [`raw_pointers`]: 裸指针的使用、安全性和应用场景
//! - [`references`]: 引用的借用规则、生命周期和使用模式
//! - [`smart_pointers`]: 各种智能指针类型和使用场景
//! - [`comparison`]: 三者的详细对比分析和选择指南
//! - [`practical_examples`]: 实际应用案例和最佳实践
//! - [`safety_analysis`]: 安全性分析和常见陷阱
//! - [`performance`]: 性能分析和基准测试
//!
//! ## 快速开始
//!
//! ```rust
//! use raw_pointers_references_smart_pointers::*;
//!
//! // 运行所有示例
//! fn main() {
//!     println!("=== 裸指针示例 ===");
//!     raw_pointers::run_raw_pointer_examples();
//!
//!     println!("\n=== 引用示例 ===");
//!     references::run_reference_examples();
//!
//!     println!("\n=== 智能指针示例 ===");
//!     smart_pointers::run_smart_pointer_examples();
//!
//!     println!("\n=== 对比分析 ===");
//!     comparison::run_comparison_analysis();
//! }
//! ```

pub mod raw_pointers;
pub mod references;
pub mod smart_pointers;
pub mod comparison;
pub mod practical_examples;
pub mod safety_analysis;
pub mod performance;

// 重新导出主要的公共接口
pub use raw_pointers::*;
pub use references::*;
pub use smart_pointers::*;
pub use comparison::*;
// pub use practical_examples::*; // 避免命名冲突
pub use safety_analysis::*;
pub use performance::*;

/// 运行所有示例的便捷函数
pub fn run_all_examples() {
    println!("🚀 Rust 裸指针、引用和智能指针深度解析");
    println!("{}\n", "=".repeat(50));

    println!("📍 1. 裸指针深度解析");
    println!("{}", "-".repeat(30));
    raw_pointers::run_raw_pointer_examples();

    println!("\n📍 2. 引用深度解析");
    println!("{}", "-".repeat(30));
    references::run_reference_examples();

    println!("\n📍 3. 智能指针深度解析");
    println!("{}", "-".repeat(30));
    smart_pointers::run_smart_pointer_examples();

    println!("\n📍 4. 三者对比分析");
    println!("{}", "-".repeat(30));
    comparison::run_all_comparisons();

    println!("\n📍 5. 实际应用案例");
    println!("{}", "-".repeat(30));
    practical_examples::run_all_examples();

    println!("\n📍 6. 安全性分析");
    println!("{}", "-".repeat(30));
    safety_analysis::run_all_safety_analysis();

    println!("\n📍 7. 性能分析");
    println!("{}", "-".repeat(30));
    performance::run_all_performance_tests();

    println!("\n🎉 所有示例运行完成！");
}

/// 快速演示三者区别的函数
pub fn quick_demo() {
    println!("🎯 快速演示三者区别...");
    println!("{}", "=".repeat(40));
    println!("=== 裸指针、引用、智能指针快速演示 ===");

    // 1. 裸指针示例
    println!("\n1. 裸指针示例:");
    let x = 42;
    let raw_ptr: *const i32 = &x;
    unsafe {
        println!("   通过裸指针访问: {}", *raw_ptr);
    }

    // 2. 引用示例
    println!("\n2. 引用示例:");
    let y = 42;
    let reference: &i32 = &y;
    println!("   通过引用访问: {}", *reference);

    // 3. 智能指针示例
    println!("\n3. 智能指针示例:");
    let smart_ptr = Box::new(42);
    println!("   通过智能指针访问: {}", *smart_ptr);

    println!("\n=== 演示完成 ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_demo() {
        quick_demo();
    }

    #[test]
    fn test_all_modules_exist() {
        // 确保所有模块都能正常编译
        assert!(true);
    }
}