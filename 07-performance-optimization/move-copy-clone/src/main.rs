//! Move, Copy, Clone 深度解析 - 主程序入口
//! 
//! 这个程序提供了一个交互式的演示界面，让用户可以选择运行不同的示例和分析。

use move_copy_clone::*;
use std::io::{self, Write};

fn main() {
    println!("🦀 欢迎使用 Rust Move, Copy, Clone 深度解析工具！");
    println!("{}", "=".repeat(60));
    
    // 显示项目信息
    print_library_info();
    
    loop {
        display_menu();
        
        print!("请选择一个选项 (输入数字): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => {
                println!("\n🚀 运行 Move 语义示例...");
                println!("{}", "=".repeat(40));
                move_semantics::run_move_examples();
            }
            "2" => {
                println!("\n📋 运行 Copy Trait 示例...");
                println!("{}", "=".repeat(40));
                copy_trait::run_copy_examples();
            }
            "3" => {
                println!("\n🔄 运行 Clone Trait 示例...");
                println!("{}", "=".repeat(40));
                clone_trait::run_clone_examples();
            }
            "4" => {
                println!("\n⚖️ 运行对比分析...");
                println!("{}", "=".repeat(40));
                comparison::summary_comparison();
            }
            "5" => {
                println!("\n🎯 运行实际应用案例...");
                println!("{}", "=".repeat(40));
                practical_examples::run_all_examples();
            }
            "6" => {
                println!("\n⚠️ 运行边界情况分析...");
                println!("{}", "=".repeat(40));
                edge_cases::run_all_edge_cases();
            }
            "7" => {
                println!("\n📊 运行性能分析...");
                println!("{}", "=".repeat(40));
                performance::run_all_performance_analysis();
            }
            "8" => {
                println!("\n🎯 快速演示三者区别...");
                println!("{}", "=".repeat(40));
                quick_demo();
            }
            "9" => {
                println!("\n🚀 运行所有示例...");
                println!("{}", "=".repeat(40));
                run_all_examples();
            }
            "10" => {
                println!("\n📚 显示学习指南...");
                println!("{}", "=".repeat(40));
                show_learning_guide();
            }
            "11" => {
                println!("\n🔧 显示最佳实践...");
                println!("{}", "=".repeat(40));
                show_best_practices();
            }
            "0" | "q" | "quit" | "exit" => {
                println!("\n👋 感谢使用！祝您 Rust 学习愉快！");
                break;
            }
            _ => {
                println!("\n❌ 无效选项，请重新选择。");
            }
        }
        
        println!("\n{}", "=".repeat(60));
        println!("按 Enter 键继续...");
        let mut _input = String::new();
        io::stdin().read_line(&mut _input).unwrap();
    }
}

/// 显示主菜单
fn display_menu() {
    println!("\n📋 请选择要运行的示例:");
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│  1. 🚀 Move 语义深度解析                                │");
    println!("│  2. 📋 Copy Trait 详细分析                              │");
    println!("│  3. 🔄 Clone Trait 完整指南                             │");
    println!("│  4. ⚖️  三者对比分析                                    │");
    println!("│  5. 🎯 实际应用案例                                     │");
    println!("│  6. ⚠️  边界情况和陷阱                                  │");
    println!("│  7. 📊 性能分析和基准测试                               │");
    println!("│  8. 🎯 快速演示三者区别                                 │");
    println!("│  9. 🚀 运行所有示例                                     │");
    println!("│ 10. 📚 学习指南                                         │");
    println!("│ 11. 🔧 最佳实践                                         │");
    println!("│  0. 👋 退出程序                                         │");
    println!("└─────────────────────────────────────────────────────────┘");
}

/// 显示学习指南
fn show_learning_guide() {
    println!("📚 Rust Move, Copy, Clone 学习指南");
    println!();
    
    println!("🎯 学习路径建议:");
    println!("1. 📖 理解所有权基础概念");
    println!("   - 所有权规则");
    println!("   - 借用和引用");
    println!("   - 生命周期基础");
    println!();
    
    println!("2. 🚀 深入 Move 语义");
    println!("   - 默认移动语义");
    println!("   - 所有权转移");
    println!("   - 部分移动");
    println!("   - 闭包捕获");
    println!();
    
    println!("3. 📋 掌握 Copy Trait");
    println!("   - Copy 的实现条件");
    println!("   - 栈上复制机制");
    println!("   - 与 Drop 的关系");
    println!("   - 性能考虑");
    println!();
    
    println!("4. 🔄 精通 Clone Trait");
    println!("   - 深拷贝机制");
    println!("   - 自定义实现");
    println!("   - 性能优化");
    println!("   - 智能指针克隆");
    println!();
    
    println!("5. ⚖️ 对比和选择");
    println!("   - 使用场景分析");
    println!("   - 性能对比");
    println!("   - 最佳实践");
    println!("   - 常见陷阱");
    println!();
    
    println!("📖 推荐学习资源:");
    println!("• The Rust Book: https://doc.rust-lang.org/book/");
    println!("• Rust by Example: https://doc.rust-lang.org/rust-by-example/");
    println!("• Rust Nomicon: https://doc.rust-lang.org/nomicon/");
    println!("• Rust Performance Book: https://nnethercote.github.io/perf-book/");
    println!();
    
    println!("🛠️ 实践建议:");
    println!("• 从简单示例开始，逐步增加复杂度");
    println!("• 多写代码，观察编译器错误信息");
    println!("• 使用 cargo expand 查看宏展开");
    println!("• 使用 cargo asm 查看生成的汇编代码");
    println!("• 进行性能基准测试");
}

/// 显示最佳实践
fn show_best_practices() {
    println!("🔧 Move, Copy, Clone 最佳实践");
    println!();
    
    println!("🚀 Move 语义最佳实践:");
    println!("✅ 默认使用 Move 语义");
    println!("✅ 通过引用避免不必要的移动");
    println!("✅ 使用 take() 方法处理 Option<T>");
    println!("✅ 在函数返回时利用移动优化");
    println!("❌ 避免在循环中意外移动");
    println!("❌ 不要为了避免移动而过度使用 Clone");
    println!();
    
    println!("📋 Copy Trait 最佳实践:");
    println!("✅ 只为小型、简单的值类型实现 Copy");
    println!("✅ 确保所有字段都实现 Copy");
    println!("✅ 避免为大型数据结构实现 Copy");
    println!("✅ 使用 #[derive(Copy, Clone)] 自动实现");
    println!("❌ 不要为包含堆数据的类型实现 Copy");
    println!("❌ 不要同时实现 Copy 和 Drop");
    println!();
    
    println!("🔄 Clone Trait 最佳实践:");
    println!("✅ 明确调用 .clone() 表示意图");
    println!("✅ 为复杂类型提供高效的 Clone 实现");
    println!("✅ 考虑使用 Cow<T> 优化克隆");
    println!("✅ 使用 Rc/Arc 共享数据而非克隆");
    println!("❌ 避免在热路径中频繁克隆");
    println!("❌ 不要为了方便而过度使用 Clone");
    println!();
    
    println!("⚖️ 选择指南:");
    println!("🎯 选择 Move 当:");
    println!("  • 需要转移所有权");
    println!("  • 处理大型数据结构");
    println!("  • 追求零成本抽象");
    println!();
    
    println!("📋 选择 Copy 当:");
    println!("  • 处理简单数值类型");
    println!("  • 数据大小很小 (≤ 16 字节)");
    println!("  • 需要频繁复制");
    println!();
    
    println!("🔄 选择 Clone 当:");
    println!("  • 需要深拷贝复杂数据");
    println!("  • 多个所有者需要独立副本");
    println!("  • 跨 API 边界传递数据");
    println!();
    
    println!("🚀 性能优化技巧:");
    println!("• 使用 #[inline] 标记小函数");
    println!("• 考虑内存布局和缓存友好性");
    println!("• 使用 Cow<T> 实现写时复制");
    println!("• 利用编译器的移动优化");
    println!("• 避免不必要的内存分配");
    println!();
    
    println!("🛡️ 安全实践:");
    println!("• 让编译器帮助检查所有权");
    println!("• 使用类型系统表达设计意图");
    println!("• 避免 unsafe 代码中的所有权问题");
    println!("• 使用智能指针管理复杂所有权");
    println!();
    
    println!("🔍 调试技巧:");
    println!("• 使用 cargo expand 查看宏展开");
    println!("• 使用 cargo asm 查看汇编输出");
    println!("• 使用 cargo bench 进行性能测试");
    println!("• 阅读编译器错误信息");
    println!("• 使用 println! 调试所有权转移");
}
