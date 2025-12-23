//! # 高级模式示例

use closure::patterns::*;
use closure::traits::*;

fn main() {
    println!("🦀 闭包高级模式示例\n");
    println!("{}", "=".repeat(60));
    
    demo_trait_hierarchy();
    demo_higher_order_functions();
    demo_combinators();
    
    println!("\n{}", "=".repeat(60));
    println!("✅ 高级模式示例完成！");
}

