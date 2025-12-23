//! # 闭包基础概念模块
//!
//! 本模块包含 Rust 闭包的基础概念和核心语法

pub mod syntax;
pub mod capture;
pub mod move_keyword;

pub use syntax::*;
pub use capture::*;
pub use move_keyword::*;

/// 运行所有基础示例
pub fn run_all_basics() {
    println!("\n=== 第一部分：闭包基础概念 ===\n");
    
    syntax::demo_basic_syntax();
    syntax::demo_closure_vs_function();
    
    capture::demo_capture_modes();
    capture::demo_compiler_implementation();
    
    move_keyword::demo_move_keyword();
    move_keyword::demo_move_with_threads();
}

