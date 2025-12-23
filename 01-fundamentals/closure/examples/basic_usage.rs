//! # 基础用法示例

use closure::basics::*;

fn main() {
    println!("🦀 闭包基础用法示例\n");
    println!("{}",  "=".repeat(60));
    
    demo_basic_syntax();
    demo_closure_vs_function();
    demo_capture_modes();
    demo_move_keyword();
    
    println!("\n{}", "=".repeat(60));
    println!("✅ 基础示例完成！");
}

