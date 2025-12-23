//! # 闭包高级专题模块
//!
//! 本模块包含闭包的深度内容，帮助真正理解闭包的本质

pub mod lifetime_deep_dive;
pub mod common_pitfalls;
pub mod closure_with_generics;
pub mod type_system_analysis;
pub mod real_world_cases;

pub use lifetime_deep_dive::*;
pub use common_pitfalls::*;
pub use closure_with_generics::*;
pub use type_system_analysis::*;
pub use real_world_cases::*;

/// 运行所有高级专题
pub fn run_all_advanced() {
    println!("\n=== 第六部分：闭包高级专题 ===\n");
    
    lifetime_deep_dive::demo_lifetime_issues();
    common_pitfalls::demo_common_pitfalls();
    closure_with_generics::demo_generics_interaction();
    type_system_analysis::demo_type_system();
    real_world_cases::demo_real_world_cases();
}

