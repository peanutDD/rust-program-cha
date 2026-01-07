// 示例代码库，允许未使用的导入和代码（用于教学演示）
#![allow(unused_imports)]
#![allow(dead_code)]

//! # Rust 深入生命周期分析
//!
//! 这是一个全面深入分析 Rust 生命周期概念的库，涵盖了从基础概念到高级应用的所有重要知识点。
//! 本库基于 Rust Course (https://course.rs/advance/lifetime/advance.html) 的内容进行深度扩展和实践。
//!
//! ## 模块结构
//!
//! - [`lifetime_basics`] - 生命周期基础概念：悬垂引用、借用检查器、生命周期注解语法
//! - [`lifetime_elision`] - 生命周期省略规则：三大省略规则的详细分析和应用场景
//! - [`static_lifetime`] - 静态生命周期：'static的含义、使用场景和注意事项
//! - [`advanced_patterns`] - 高级生命周期模式：子类型、高阶生命周期、生命周期子类型化
//! - [`lifetime_bounds`] - 生命周期约束：T: 'a、where子句、复杂约束场景
//! - [`practical_examples`] - 实际应用案例：结构体生命周期、方法中的生命周期、复杂数据结构
//! - [`common_issues`] - 常见问题和解决方案：编译错误分析、最佳实践、性能考虑
//!
//! ## 快速开始
//!
//! ```rust
//! use advanced_lifetime::*;
//!
//! // 运行所有示例
//! fn main() {
//!     // 基础概念演示
//!     lifetime_basics::run_all_lifetime_basics_examples();
//!     
//!     // 省略规则演示
//!     lifetime_elision::run_all_elision_examples();
//!     
//!     // 静态生命周期演示
//!     static_lifetime::run_all_static_lifetime_examples();
//!     
//!     // 高级模式演示
//!     advanced_patterns::run_all_advanced_patterns_examples();
//!     
//!     // 生命周期约束演示
//!     lifetime_bounds::run_all_lifetime_bounds_examples();
//!     
//!     // 实际应用案例演示
//!     practical_examples::run_all_practical_examples();
//!     
//!     // 常见问题和解决方案演示
//!     common_issues::run_all_common_issues_examples();
//! }
//! ```
//!
//! ## 学习路径建议
//!
//! 1. **基础阶段**：从 `lifetime_basics` 开始，理解生命周期的基本概念
//! 2. **规则阶段**：学习 `lifetime_elision` 中的省略规则，掌握何时需要显式标注
//! 3. **特殊阶段**：深入 `static_lifetime`，理解静态生命周期的特殊性
//! 4. **进阶阶段**：探索 `advanced_patterns` 中的高级概念和模式
//! 5. **约束阶段**：掌握 `lifetime_bounds` 中的复杂约束场景
//! 6. **实践阶段**：通过 `practical_examples` 学习实际应用
//! 7. **问题解决**：参考 `common_issues` 解决常见问题和优化性能
//!
//! ## 特色功能
//!
//! - 🎯 **全面覆盖**：涵盖 Rust 生命周期的所有重要概念
//! - 📚 **深度分析**：每个概念都有详细的理论解释和代码示例
//! - 🔧 **实用案例**：提供大量实际应用场景的代码示例
//! - 🚀 **性能优化**：包含性能考虑和最佳实践指导
//! - 🐛 **问题解决**：详细分析常见编译错误和解决方案
//! - ✅ **测试完备**：每个模块都包含完整的单元测试

// 导出所有公共模块
pub mod advanced_patterns;
pub mod common_issues;
pub mod lifetime_basics;
pub mod lifetime_bounds;
pub mod lifetime_elision;
pub mod practical_examples;
pub mod static_lifetime;

// 重新导出主要的运行函数
pub use advanced_patterns::run_all_advanced_patterns_examples;
pub use common_issues::run_all_common_issues_examples;
pub use lifetime_basics::run_all_lifetime_basics_examples;
pub use lifetime_bounds::run_all_lifetime_bounds_examples;
pub use lifetime_elision::run_all_elision_examples;
pub use practical_examples::run_all_practical_examples;
pub use static_lifetime::run_all_static_lifetime_examples;

/// 运行所有生命周期示例的便捷函数
///
/// 这个函数会依次运行所有模块的示例，提供完整的学习体验。
/// 建议在学习时按顺序运行各个模块的示例。
pub fn run_all_examples() {
    println!("🦀 Rust 深入生命周期分析 - 完整示例集 🦀");
    println!("================================================");
    println!("本示例集将带您深入了解 Rust 生命周期的方方面面");
    println!("================================================\n");

    // 1. 基础概念
    println!("📚 第一章：生命周期基础概念");
    lifetime_basics::run_all_lifetime_basics_examples();

    println!("\n{}", "=".repeat(50));
    println!();

    // 2. 省略规则
    println!("📝 第二章：生命周期省略规则");
    lifetime_elision::run_all_elision_examples();

    println!("\n{}", "=".repeat(50));
    println!();

    // 3. 静态生命周期
    println!("🔒 第三章：静态生命周期");
    static_lifetime::run_all_static_lifetime_examples();

    println!("\n{}", "=".repeat(50));
    println!();

    // 4. 高级模式
    println!("🚀 第四章：高级生命周期模式");
    advanced_patterns::run_all_advanced_patterns_examples();

    println!("\n{}", "=".repeat(50));
    println!();

    // 5. 生命周期约束
    println!("⛓️ 第五章：生命周期约束");
    lifetime_bounds::run_all_lifetime_bounds_examples();

    println!("\n{}", "=".repeat(50));
    println!();

    // 6. 实际应用案例
    println!("🏗️ 第六章：实际应用案例");
    practical_examples::run_all_practical_examples();

    println!("\n{}", "=".repeat(50));
    println!();

    // 7. 常见问题和解决方案
    println!("🔧 第七章：常见问题和解决方案");
    common_issues::run_all_common_issues_examples();

    println!("\n{}", "=".repeat(50));
    println!("🎉 恭喜！您已完成 Rust 深入生命周期分析的全部学习内容！");
    println!("💡 建议：多实践、多思考，在实际项目中应用这些概念。");
    println!("📖 参考：https://course.rs/advance/lifetime/advance.html");
    println!("{}", "=".repeat(50));
}

/// 运行基础示例的便捷函数
///
/// 适合初学者，只运行基础概念相关的示例。
pub fn run_basic_examples() {
    println!("🌱 Rust 生命周期基础示例");
    println!("========================\n");

    lifetime_basics::run_all_lifetime_basics_examples();
    lifetime_elision::run_all_elision_examples();
    static_lifetime::run_all_static_lifetime_examples();

    println!("\n========================");
    println!("✅ 基础示例运行完成！");
}

/// 运行高级示例的便捷函数
///
/// 适合有一定基础的开发者，运行高级概念和实际应用示例。
pub fn run_advanced_examples() {
    println!("🚀 Rust 生命周期高级示例");
    println!("========================\n");

    advanced_patterns::run_all_advanced_patterns_examples();
    lifetime_bounds::run_all_lifetime_bounds_examples();
    practical_examples::run_all_practical_examples();
    common_issues::run_all_common_issues_examples();

    println!("\n========================");
    println!("✅ 高级示例运行完成！");
}

/// 库的版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 库的描述信息
pub const DESCRIPTION: &str = "深入分析 Rust 生命周期概念的完整学习库";

/// 获取库信息
pub fn get_library_info() -> String {
    format!("Advanced Lifetime Analysis v{}\n{}", VERSION, DESCRIPTION)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_info() {
        let info = get_library_info();
        assert!(info.contains("Advanced Lifetime Analysis"));
        assert!(info.contains(DESCRIPTION));
    }

    #[test]
    fn test_version_constant() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_description_constant() {
        assert!(!DESCRIPTION.is_empty());
        assert!(DESCRIPTION.contains("Rust"));
        assert!(DESCRIPTION.contains("生命周期"));
    }

    // 集成测试：确保所有模块都能正常导入
    #[test]
    fn test_module_imports() {
        // 测试主要运行函数是否可用
        let _result = lifetime_basics::run_all_lifetime_basics_examples;
        let _result = lifetime_elision::run_all_elision_examples;
        let _result = static_lifetime::run_all_static_lifetime_examples;
        let _result = advanced_patterns::run_all_advanced_patterns_examples;
        let _result = lifetime_bounds::run_all_lifetime_bounds_examples;
        let _result = practical_examples::run_all_practical_examples;
        let _result = common_issues::run_all_common_issues_examples;
    }
}

/// 示例运行器，用于演示特定主题
pub mod examples {
    use super::*;

    /// 运行借用检查器相关示例
    pub fn run_borrow_checker_examples() {
        println!("🔍 借用检查器示例");
        println!("================\n");

        lifetime_basics::run_all_lifetime_basics_examples();
        common_issues::run_all_common_issues_examples();
    }

    /// 运行性能相关示例
    pub fn run_performance_examples() {
        println!("⚡ 性能优化示例");
        println!("==============\n");

        common_issues::run_all_common_issues_examples();
    }

    /// 运行实际应用示例
    pub fn run_real_world_examples() {
        println!("🌍 实际应用示例");
        println!("==============\n");

        practical_examples::run_all_practical_examples();
        static_lifetime::run_all_static_lifetime_examples();
    }

    /// 运行错误处理示例
    pub fn run_error_handling_examples() {
        println!("🚨 错误处理示例");
        println!("==============\n");

        common_issues::run_all_common_issues_examples();
    }
}
