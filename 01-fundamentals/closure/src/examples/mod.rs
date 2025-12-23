//! # 实际应用示例模块

pub mod iterators;
pub mod error_handling;
pub mod practical_uses;

pub use iterators::*;
pub use error_handling::*;
pub use practical_uses::*;

/// 运行所有应用示例
pub fn run_all_examples() {
    println!("\n=== 第四部分：实际应用示例 ===\n");
    
    iterators::demo_iterator_patterns();
    error_handling::demo_error_handling();
    practical_uses::demo_practical_uses();
}

