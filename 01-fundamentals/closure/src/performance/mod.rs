//! # 性能优化模块

pub mod optimization;
pub mod benchmarking;
pub mod best_practices;

pub use optimization::*;
pub use benchmarking::*;
pub use best_practices::*;

/// 运行所有性能相关示例
pub fn run_all_performance() {
    println!("\n=== 第五部分：性能优化与最佳实践 ===\n");
    
    optimization::demo_optimization_strategies();
    benchmarking::demo_performance_comparison();
    best_practices::demo_best_practices();
}

