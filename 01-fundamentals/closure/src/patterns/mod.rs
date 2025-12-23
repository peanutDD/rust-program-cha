//! # 高级编程模式模块
//!
//! 本模块介绍闭包的高级使用模式

pub mod functional;
pub mod async_patterns;
pub mod combinators;

pub use functional::*;
pub use async_patterns::*;
pub use combinators::*;

/// 运行所有高级模式示例
pub fn run_all_patterns() {
    println!("\n=== 第三部分：高级编程模式 ===\n");
    
    functional::demo_higher_order_functions();
    combinators::demo_combinators();
    async_patterns::demo_callback_patterns();
}

