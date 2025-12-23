//! # 闭包 Trait 系统模块
//!
//! 本模块深入解析 Fn、FnMut、FnOnce 三种 trait

pub mod fn_traits;
pub mod trait_hierarchy;
pub mod dispatch;

pub use fn_traits::*;
pub use trait_hierarchy::*;
pub use dispatch::*;

/// 运行所有 Trait 相关示例
pub fn run_all_traits() {
    println!("\n=== 第二部分：Trait 系统深入解析 ===\n");
    
    trait_hierarchy::demo_trait_hierarchy();
    fn_traits::demo_fn_trait();
    fn_traits::demo_fn_mut_trait();
    fn_traits::demo_fn_once_trait();
    dispatch::demo_static_vs_dynamic_dispatch();
}

