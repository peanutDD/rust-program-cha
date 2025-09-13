//! # Eq 和 PartialEq 深度解析项目
//!
//! 本项目深入探讨 Rust 中 Eq 和 PartialEq trait 的区别、应用场景和最佳实践。
//!
//! ## 项目结构
//! - `main.rs`: 项目主入口，展示核心概念
//! - `examples.rs`: 实际应用案例
//! - `comparison.rs`: 详细对比分析
//! - `edge_cases.rs`: 边界情况处理
//! - `performance.rs`: 性能分析

mod basic_concepts;
mod trait_definitions;
mod implementation_examples;
mod common_types;
mod custom_types;
mod collections_usage;
mod testing;
mod edge_cases;
mod performance;
mod best_practices;

// 主程序不需要这些导入，各模块会自行导入所需的依赖

fn main() {
    println!("🦀 Eq 和 PartialEq 深度解析");
    println!("{}", "=".repeat(50));
    
    // 1. 基础概念演示
    println!("\n📚 1. 基础概念演示");
    basic_concepts::demonstrate_basic_concepts();
    
    // 2. Trait 定义解析
    println!("\n🔍 2. Trait 定义解析");
    trait_definitions::analyze_trait_definitions();
    
    // 3. 实现示例
    println!("\n💡 3. 实现示例");
    implementation_examples::show_implementation_examples();
    
    // 4. 常见类型的行为
    println!("\n🏗️ 4. 常见类型的行为");
    common_types::demonstrate_common_types();
    
    // 5. 自定义类型实现
    println!("\n🎨 5. 自定义类型实现");
    custom_types::demonstrate_custom_types();
    
    // 6. 集合中的使用
    println!("\n📦 6. 集合中的使用");
    collections_usage::demonstrate_collections_usage();
    
    // 7. 测试和验证
    println!("\n🧪 7. 测试和验证");
    testing::run_comprehensive_tests();
    
    // 8. 边界情况分析
    println!("\n⚠️ 8. 边界情况分析");
    edge_cases::summarize_edge_cases();
    
    // 9. 性能分析
    println!("\n⚡ 9. 性能分析");
    performance::run_performance_analysis();
    
    // 10. 最佳实践指南
    println!("\n🔸 10. 最佳实践指南");
    best_practices::run_best_practices_guide();
    
    println!("\n✅ 演示完成！");
    println!("\n📖 学习资源:");
    println!("   - EQ_PARTIALEQ_COMPREHENSIVE_ANALYSIS.md: 理论深度分析");
    println!("   - 运行 `cargo run --bin examples` 查看更多实际案例");
    println!("   - 运行 `cargo run --bin comparison` 查看详细对比");
    println!("   - 运行 `cargo run --bin edge_cases` 查看边界情况");
    println!("   - 运行 `cargo run --bin performance` 查看性能分析");
    println!("   - 运行 `cargo test` 执行所有测试");
    println!("   - 运行 `cargo bench` 执行性能基准测试");
}

/// 演示基本的相等性比较
fn demonstrate_equality_basics() {
    println!("\n🔸 基本相等性比较:");
    
    // 基本类型
    let a = 42;
    let b = 42;
    println!("  {} == {} : {}", a, b, a == b);
    
    // 字符串
    let s1 = "hello";
    let s2 = "hello";
    println!("  \"{}\" == \"{}\" : {}", s1, s2, s1 == s2);
    
    // 浮点数的特殊情况
    let f1 = 0.1 + 0.2;
    let f2 = 0.3;
    println!("  {} == {} : {} (浮点数精度问题)", f1, f2, f1 == f2);
    
    // NaN 的特殊行为
    let nan1 = f64::NAN;
    let nan2 = f64::NAN;
    println!("  NaN == NaN : {} (NaN 不等于自身)", nan1 == nan2);
}