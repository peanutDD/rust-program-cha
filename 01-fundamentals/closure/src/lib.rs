// 示例代码库，允许未使用的导入和代码（用于教学演示）
#![allow(unused_imports)]
#![allow(dead_code)]

//! # Rust 闭包深度学习库
//!
//! 这是一个全面的 Rust 闭包学习库，提供了从基础到高级的完整教程。
//!
//! ## 模块组织
//!
//! - `basics`: 闭包基础概念
//! - `traits`: Trait 系统深入
//! - `patterns`: 高级编程模式
//! - `examples`: 实际应用示例
//! - `performance`: 性能优化指南
//!
//! ## 快速开始
//!
//! ```rust,no_run
//! use closure::basics::*;
//! use closure::traits::*;
//!
//! // 创建一个简单的闭包
//! let add_one = |x: i32| x + 1;
//! println!("Result: {}", add_one(5));
//! ```

// 基础概念模块
pub mod basics;

// Trait 系统模块
pub mod traits;

// 高级模式模块
pub mod patterns;

// 实际应用示例
pub mod examples;

// 性能优化
pub mod performance;

// 高级专题模块（深度内容）
pub mod advanced;

/// 运行所有示例和教程
pub fn run_all_examples() {
    println!("🦀 Rust 闭包全面学习指南 🦀");
    println!("{}", "=".repeat(60));

    basics::run_all_basics();
    traits::run_all_traits();
    patterns::run_all_patterns();
    examples::run_all_examples();
    performance::run_all_performance();
    advanced::run_all_advanced();

    println!("\n{}", "=".repeat(60));
    println!("🎉 闭包学习指南完成！");
    print_key_takeaways();
}

/// 运行深度学习内容
pub fn run_advanced_topics() {
    println!("🎓 Rust 闭包深度专题");
    println!("{}", "=".repeat(60));
    
    advanced::run_all_advanced();
    
    println!("\n{}", "=".repeat(60));
    println!("🎓 深度专题完成！");
}

/// 打印关键要点总结
fn print_key_takeaways() {
    println!("\n关键要点总结:");
    println!("1. 闭包是可以捕获环境变量的匿名函数");
    println!("2. 三种 trait：Fn > FnMut > FnOnce");
    println!("3. move 关键字强制获取所有权");
    println!("4. 闭包是零成本抽象");
    println!("5. 广泛应用于迭代器、错误处理、异步编程等场景");
    println!("\n📚 继续学习:");
    println!("- 阅读 docs/ 目录下的详细文档");
    println!("- 完成 exercises/ 目录下的练习");
    println!("- 运行 examples/ 目录下的示例程序");
    println!("- 查看 benches/ 目录下的性能测试");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_closure() {
        let add = |x, y| x + y;
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_closure_capture() {
        let x = 10;
        let add_x = |y| x + y;
        assert_eq!(add_x(5), 15);
    }

    #[test]
    fn test_move_closure() {
        let data = vec![1, 2, 3];
        let len = data.len();
        let get_len = move || data.len();
        assert_eq!(get_len(), len);
    }
}

