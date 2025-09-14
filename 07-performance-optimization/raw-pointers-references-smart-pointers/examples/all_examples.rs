//! # 所有示例的综合演示
//!
//! 这个示例程序展示了项目中所有模块的核心功能，
//! 提供了一个完整的学习和演示入口。

use raw_pointers_references_smart_pointers::*;

fn main() {
    println!("🦀 Rust 指针类型深度解析 - 综合示例");
    println!("{}", "=".repeat(80));
    
    // 1. 快速演示
    println!("\n🚀 1. 快速演示");
    quick_demo();
    
    // 2. 引用详解
    println!("\n\n📚 2. 引用和借用规则");
    references::run_reference_examples();
    
    // 3. 智能指针详解
    println!("\n\n🧠 3. 智能指针详解");
    smart_pointers::run_smart_pointer_examples();
    
    // 4. 裸指针详解
    println!("\n\n⚠️ 4. 裸指针详解");
    raw_pointers::run_raw_pointer_examples();
    
    // 5. 对比分析
    println!("\n\n🔍 5. 三者对比分析");
    comparison::run_all_comparisons();
    
    // 6. 实际应用案例
    println!("\n\n🛠️ 6. 实际应用案例");
    practical_examples::run_all_examples();
    
    // 7. 性能分析
    println!("\n\n⚡ 7. 性能分析");
    performance::run_all_performance_tests();
    
    // 8. 安全性分析
    println!("\n\n🔒 8. 安全性分析");
    safety_analysis::run_all_safety_analysis();
    
    // 总结
    println!("\n\n🎉 学习总结");
    print_learning_summary();
}

/// 打印学习总结
fn print_learning_summary() {
    println!("{}", "=".repeat(80));
    println!("\n🎯 核心要点总结:");
    
    println!("\n📚 引用 (References):");
    println!("  • 编译时安全，零运行时开销");
    println!("  • 借用检查器确保内存安全");
    println!("  • 生命周期注解表达引用关系");
    println!("  • 适用于函数参数和临时借用");
    
    println!("\n🧠 智能指针 (Smart Pointers):");
    println!("  • 自动内存管理，运行时安全检查");
    println!("  • Box<T>: 堆分配，递归数据结构");
    println!("  • Rc<T>/Arc<T>: 引用计数，共享所有权");
    println!("  • RefCell<T>/Mutex<T>: 内部可变性");
    println!("  • 适用于复杂所有权模式");
    
    println!("\n⚠️ 裸指针 (Raw Pointers):");
    println!("  • 最大灵活性，需要手动确保安全");
    println!("  • 支持指针算术和类型转换");
    println!("  • 适用于 FFI、系统编程、性能关键代码");
    println!("  • 必须在 unsafe 块中解引用");
    
    println!("\n🎯 选择指南:");
    println!("  1. 优先使用引用 - 安全、高效、简单");
    println!("  2. 需要复杂所有权时使用智能指针");
    println!("  3. 仅在必要时使用裸指针");
    println!("  4. 始终考虑安全性和可维护性");
    
    println!("\n🛡️ 安全编程原则:");
    println!("  • 利用类型系统防止错误");
    println!("  • 最小化 unsafe 代码范围");
    println!("  • 使用工具检测内存错误");
    println!("  • 编写全面的测试");
    
    println!("\n📈 性能考虑:");
    println!("  • 引用 ≈ 裸指针 > 智能指针（访问速度）");
    println!("  • 编译器优化可以消除大部分开销");
    println!("  • 选择合适的数据结构比微优化更重要");
    
    println!("\n🚀 下一步学习建议:");
    println!("  1. 深入学习 Rust 异步编程");
    println!("  2. 探索 Rust 宏系统");
    println!("  3. 学习 Rust 的 trait 系统");
    println!("  4. 实践大型项目开发");
    println!("  5. 贡献开源 Rust 项目");
    
    println!("\n{}", "=".repeat(80));
    println!("🎉 恭喜完成 Rust 指针类型深度学习！");
    println!("💡 记住：安全性和性能可以兼得，这就是 Rust 的魅力！");
    println!("{}", "=".repeat(80));
}