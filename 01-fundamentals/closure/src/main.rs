//! # Rust 闭包学习项目主程序
//!
//! 这个项目提供了全面的 Rust 闭包学习指南和实践案例
//!
//! ## 运行方式
//!
//! ```bash
//! # 运行所有示例
//! cargo run
//!
//! # 运行特定示例
//! cargo run --example basic_usage
//! cargo run --example advanced_patterns
//!
//! # 运行练习
//! cargo run --bin basics_exercise
//!
//! # 运行测试
//! cargo test
//!
//! # 运行基准测试（需要 nightly）
//! cargo +nightly bench
//! ```

use closure::*;

fn main() {
    print_banner();
    
    // 运行所有教程示例
    run_all_examples();
    
    print_next_steps();
}

fn print_banner() {
    println!("\n{}", "=".repeat(80));
    println!("{:^80}", "🦀 Rust 闭包深度学习指南 🦀");
    println!("{}", "=".repeat(80));
    println!();
}

fn print_next_steps() {
    println!("\n{}", "=".repeat(80));
    println!("\n💡 接下来可以做什么？\n");
    
    println!("📚 学习路径:");
    println!("   1. 完成 exercises/ 目录下的练习");
    println!("   2. 运行 examples/ 目录下的示例程序");
    println!("   3. 阅读 docs/ 目录下的详细文档");
    println!("   4. 运行性能基准测试（需要 nightly）");
    
    println!("\n🔧 可用命令:");
    println!("   cargo run                      # 运行完整教程");
    println!("   cargo run --example basic_usage    # 基础用法示例");
    println!("   cargo run --example advanced_patterns  # 高级模式示例");
    println!("   cargo test                     # 运行所有测试");
    println!("   cargo +nightly bench           # 性能基准测试");
    
    println!("\n📖 推荐学习资源:");
    println!("   - The Rust Book: https://doc.rust-lang.org/book/ch13-01-closures.html");
    println!("   - Rust 语言圣经: https://course.rs/advance/functional-programing/closure.html");
    println!("   - Rust By Example: https://doc.rust-lang.org/rust-by-example/fn/closures.html");
    
    println!("\n{}", "=".repeat(80));
    println!("\n✨ Happy Coding! ✨\n");
}
