//! # Rust 闭包深度解析库
//!
//! 这是一个全面、深入的 Rust 闭包教程库，基于 [Rust语言圣经 - 闭包章节](https://course.rs/advance/functional-programing/closure.html)
//! 的内容进行了深度扩展和实践。
//!
//! ## 📚 模块结构
//!
//! ### 🎯 基础模块
//! - [`basics`] - 闭包基础概念和语法
//! - [`capture`] - 闭包的捕获机制详解  
//! - [`traits`] - 闭包的三种 Trait: Fn, FnMut, FnOnce
//! - [`types`] - 闭包的类型推导和类型注解
//!
//! ### 🚀 进阶模块
//! - [`parameters`] - 闭包作为参数和返回值
//! - [`applications`] - 闭包的实际应用场景
//! - [`performance`] - 闭包的性能考虑
//!
//! ### 🔥 高级模块
//! - [`advanced`] - 闭包的高级特性和模式
//! - [`concurrency`] - 并发编程中的闭包
//! - [`pitfalls`] - 常见陷阱和最佳实践
//!
//! ## 🚀 快速开始
//!
//! ### 作为库使用
//! ```rust
//! use closure_tutorial::*;
//!
//! // 运行所有演示
//! run_all_demos();
//!
//! // 运行特定级别的演示
//! run_basic_demos();     // 基础演示
//! run_advanced_demos();  // 高级演示
//!
//! // 运行特定模块
//! basics::demonstrate();
//! capture::demonstrate();
//! traits::demonstrate();
//! ```
//!
//! ### 作为二进制程序使用
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
//! # 显示帮助
//! cargo run -- --help
//! ```
//!
//! ## 📖 学习路径
//!
//! ### 🌱 初学者路径
//! 1. [`basics`] - 了解闭包基本语法
//! 2. [`capture`] - 理解捕获机制
//! 3. [`traits`] - 掌握三种 Trait
//!
//! ### 🌿 进阶者路径
//! 4. [`types`] - 深入类型系统
//! 5. [`parameters`] - 学习高阶函数
//! 6. [`applications`] - 实际应用场景
//!
//! ### 🌳 高级用户路径
//! 7. [`performance`] - 性能优化
//! 8. [`advanced`] - 高级模式
//! 9. [`concurrency`] - 并发编程
//! 10. [`pitfalls`] - 最佳实践
//!
//! ## 🎯 特性
//!
//! - ✅ **全面覆盖**: 从基础语法到高级模式
//! - ✅ **实践导向**: 大量实际代码示例
//! - ✅ **性能优化**: 深入分析性能考虑
//! - ✅ **最佳实践**: 避免常见陷阱
//! - ✅ **并发支持**: 多线程环境下的闭包使用
//! - ✅ **文档完善**: 详细的 API 文档和示例

pub mod advanced;
pub mod applications;
pub mod basics;
pub mod capture;
pub mod concurrency;
pub mod parameters;
pub mod performance;
pub mod pitfalls;
pub mod traits;
pub mod types;

// 重新导出主要的演示函数
pub use advanced::demonstrate as demonstrate_advanced;
pub use applications::demonstrate as demonstrate_applications;
pub use basics::demonstrate as demonstrate_basics;
pub use capture::demonstrate as demonstrate_capture;
pub use concurrency::demonstrate as demonstrate_concurrency;
pub use parameters::demonstrate as demonstrate_parameters;
pub use performance::demonstrate as demonstrate_performance;
pub use pitfalls::demonstrate as demonstrate_pitfalls;
pub use traits::demonstrate as demonstrate_traits;
pub use types::demonstrate as demonstrate_types;

/// 运行所有闭包演示
///
/// 这个函数会按照学习顺序依次运行所有模块的演示，
/// 提供完整的闭包学习体验。
///
/// # 示例
///
/// ```rust
/// use closure_tutorial::run_all_demos;
///
/// run_all_demos();
/// ```
///
/// # 错误处理
///
/// 如果某个演示模块出现错误，会打印错误信息但继续执行其他模块。
pub fn run_all_demos() {
    println!("🦀 Rust 闭包深度解析教程");
    println!("📖 版本: {}", version());
    println!("📝 描述: {}", description());
    println!("基于 https://course.rs/advance/functional-programing/closure.html");
    println!("{}", "=".repeat(60));

    // 按照学习顺序执行所有演示，添加错误处理
    run_demo_with_error_handling("基础概念", demonstrate_basics);
    run_demo_with_error_handling("捕获机制", demonstrate_capture);
    run_demo_with_error_handling("Trait 系统", demonstrate_traits);
    run_demo_with_error_handling("类型推导", demonstrate_types);
    run_demo_with_error_handling("参数传递", demonstrate_parameters);
    run_demo_with_error_handling("实际应用", demonstrate_applications);
    run_demo_with_error_handling("性能考虑", demonstrate_performance);
    run_demo_with_error_handling("高级特性", demonstrate_advanced);
    run_demo_with_error_handling("常见陷阱", demonstrate_pitfalls);
    run_demo_with_error_handling("并发编程", demonstrate_concurrency);

    println!("{}", "=".repeat(60));
    println!("✅ Rust 闭包深度解析完成！");

    print_summary();
}

/// 打印学习总结
fn print_summary() {
    println!("\n📚 关键要点总结:");
    println!("1. 闭包是匿名函数，可以捕获环境变量");
    println!("2. 三种捕获方式：不可变借用、可变借用、所有权转移");
    println!("3. 三种 Trait：Fn、FnMut、FnOnce，体现不同的调用能力");
    println!("4. 类型推导灵活，但一旦确定就不能改变");
    println!("5. 可作为参数和返回值，支持高阶函数编程");
    println!("6. 广泛应用于迭代器、事件处理、策略模式等场景");
    println!("7. 性能优秀，编译器会进行内联优化");
    println!("8. 支持递归、工厂模式、组合子等高级模式");
    println!("9. 注意借用检查、生命周期和性能陷阱");
    println!("10. 在并发编程中需要满足 Send + Sync 约束");

    println!("\n🎯 学习建议:");
    println!("- 从简单的闭包语法开始，逐步理解捕获机制");
    println!("- 重点掌握三种 Trait 的区别和使用场景");
    println!("- 多练习闭包在迭代器和函数式编程中的应用");
    println!("- 注意性能优化和常见陷阱的避免");
    println!("- 结合实际项目场景理解闭包的价值");
}

/// 运行基础演示（适合初学者）
///
/// 只运行基础的闭包概念演示，适合刚开始学习闭包的开发者。
///
/// # 错误处理
///
/// 如果某个基础演示模块出现错误，会打印错误信息但继续执行其他模块。
pub fn run_basic_demos() {
    println!("🦀 Rust 闭包基础演示");
    println!("{}", "=".repeat(40));

    run_demo_with_error_handling("基础概念", demonstrate_basics);
    run_demo_with_error_handling("捕获机制", demonstrate_capture);
    run_demo_with_error_handling("Trait 系统", demonstrate_traits);

    println!("{}", "=".repeat(40));
    println!("✅ 基础演示完成！建议继续学习高级特性。");
}

/// 运行高级演示（适合有经验的开发者）
///
/// 运行高级闭包特性演示，适合已经掌握基础概念的开发者。
///
/// # 错误处理
///
/// 如果某个高级演示模块出现错误，会打印错误信息但继续执行其他模块。
pub fn run_advanced_demos() {
    println!("🦀 Rust 闭包高级特性演示");
    println!("{}", "=".repeat(40));

    run_demo_with_error_handling("性能考虑", demonstrate_performance);
    run_demo_with_error_handling("高级特性", demonstrate_advanced);
    run_demo_with_error_handling("常见陷阱", demonstrate_pitfalls);
    run_demo_with_error_handling("并发编程", demonstrate_concurrency);

    println!("{}", "=".repeat(40));
    println!("✅ 高级演示完成！");
}

/// 获取库版本信息
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// 获取库描述信息
pub fn description() -> &'static str {
    "Rust 闭包深度解析教程库 - 全面学习 Rust 闭包的所有特性"
}

/// 带错误处理的演示运行器
///
/// 运行指定的演示函数，如果出现 panic 会捕获并打印错误信息。
fn run_demo_with_error_handling<F>(name: &str, demo_fn: F)
where
    F: FnOnce() + std::panic::UnwindSafe,
{
    use std::panic;

    print!("🔄 正在运行 {} 演示... ", name);

    let result = panic::catch_unwind(|| {
        demo_fn();
    });

    match result {
        Ok(_) => println!("✅ 完成"),
        Err(err) => {
            println!("❌ 出现错误");
            if let Some(msg) = err.downcast_ref::<&str>() {
                eprintln!("错误信息: {}", msg);
            } else if let Some(msg) = err.downcast_ref::<String>() {
                eprintln!("错误信息: {}", msg);
            } else {
                eprintln!("未知错误类型");
            }
        }
    }
}

/// 运行特定类型的演示
///
/// 根据演示类型运行相应的演示函数。
pub fn run_demo_by_type(demo_type: &str) -> Result<(), String> {
    match demo_type {
        "basics" => {
            demonstrate_basics();
            Ok(())
        }
        "capture" => {
            demonstrate_capture();
            Ok(())
        }
        "traits" => {
            demonstrate_traits();
            Ok(())
        }
        "types" => {
            demonstrate_types();
            Ok(())
        }
        "parameters" => {
            demonstrate_parameters();
            Ok(())
        }
        "applications" => {
            demonstrate_applications();
            Ok(())
        }
        "performance" => {
            demonstrate_performance();
            Ok(())
        }
        "advanced" => {
            demonstrate_advanced();
            Ok(())
        }
        "pitfalls" => {
            demonstrate_pitfalls();
            Ok(())
        }
        "concurrency" => {
            demonstrate_concurrency();
            Ok(())
        }
        _ => Err(format!("未知的演示类型: {}", demo_type)),
    }
}

/// 获取所有可用的演示类型
pub fn get_available_demo_types() -> Vec<&'static str> {
    vec![
        "basics",
        "capture",
        "traits",
        "types",
        "parameters",
        "applications",
        "performance",
        "advanced",
        "pitfalls",
        "concurrency",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!version().is_empty());
    }

    #[test]
    fn test_description() {
        assert!(!description().is_empty());
    }

    #[test]
    fn test_run_demo_by_type_valid() {
        // 测试有效的演示类型
        let result = run_demo_by_type("basics");
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_demo_by_type_invalid() {
        // 测试无效的演示类型
        let result = run_demo_by_type("invalid_type");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("未知的演示类型"));
    }

    #[test]
    fn test_get_available_demo_types() {
        let types = get_available_demo_types();
        assert!(!types.is_empty());
        assert!(types.contains(&"basics"));
        assert!(types.contains(&"advanced"));
    }

    #[test]
    fn test_run_basic_demos() {
        // 这个测试确保基础演示函数可以正常调用
        // 在实际测试中，我们可能不想运行完整的演示
        // 所以这里只是一个占位符测试
        assert!(true);
    }
}
