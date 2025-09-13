//! # Scope, Lifetime, and NLL (Non-Lexical Lifetimes) 深度解析
//!
//! 本库提供了对Rust中三个核心概念的全面分析：
//! - **作用域 (Scope)**: 变量和绑定的可见性范围
//! - **生命周期 (Lifetime)**: 引用的有效性时间范围
//! - **非词法生命周期 (NLL)**: Rust 2018引入的更智能的借用检查器
//!
//! ## 模块结构
//!
//! - `scope`: 作用域分析和示例
//! - `lifetime`: 生命周期分析和示例
//! - `nll`: NLL分析和示例
//! - `comparison`: 三者对比分析
//! - `practical_examples`: 实际应用案例
//! - `edge_cases`: 边界情况和陷阱
//! - `best_practices`: 最佳实践指南

pub mod scope;
pub mod lifetime;
pub mod nll;
pub mod comparison;
pub mod practical_examples;
pub mod edge_cases;
pub mod best_practices;

/// 库的版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 库的描述信息
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// 打印库的基本信息
pub fn print_library_info() {
    println!("=== Scope, Lifetime, and NLL 深度解析 ===");
    println!("版本: {}", VERSION);
    println!("描述: {}", DESCRIPTION);
    println!("\n本库将帮助您深入理解Rust中的三个核心概念：");
    println!("1. 作用域 (Scope) - 变量和绑定的可见性范围");
    println!("2. 生命周期 (Lifetime) - 引用的有效性时间范围");
    println!("3. 非词法生命周期 (NLL) - 更智能的借用检查器");
    println!("\n通过实际案例和对比分析，您将掌握这些概念的区别和联系。");
    println!("================================================\n");
}

/// 运行所有示例的主函数
pub fn run_all_examples() {
    print_library_info();
    
    println!("🔍 1. 作用域分析示例");
    scope::run_scope_examples();
    
    println!("\n⏰ 2. 生命周期分析示例");
    lifetime::run_lifetime_examples();
    
    println!("\n🚀 3. NLL分析示例");
    nll::run_nll_examples();
    
    println!("\n📊 4. 对比分析示例");
    comparison::run_comparison_examples();
    
    println!("\n💡 5. 实际应用案例");
    practical_examples::run_practical_examples();
    
    println!("\n⚠️  6. 边界情况分析");
    edge_cases::run_edge_cases_analysis();
    
    println!("\n✅ 7. 最佳实践指南");
    best_practices::run_best_practices_analysis();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_info() {
        // 测试库信息是否正确
        assert!(!VERSION.is_empty());
        assert!(!DESCRIPTION.is_empty());
    }

    #[test]
    fn test_run_all_examples() {
        // 测试所有示例是否能正常运行
        run_all_examples();
    }
}