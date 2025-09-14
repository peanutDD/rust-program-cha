// main.rs
// Rust 字符串与切片学习项目主入口
// 基于 https://course.rs/basic/compound-type/string-slice.html

mod exercises;
mod string_slice_comprehensive;

fn main() {
    println!("🦀 欢迎来到 Rust 字符串与切片完整学习指南！");
    println!("📚 本项目基于 course.rs 的字符串切片教程");
    println!("🎯 涵盖所有核心概念，包含详细案例和最佳实践\n");

    // 获取命令行参数
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "exercises" | "ex" => {
                println!("🎯 运行练习模式\n");
                exercises::run_all_exercises();
            }
            "challenge" | "ch" => {
                println!("🏆 运行挑战模式\n");
                exercises::challenge_text_analyzer();
            }
            "help" | "h" => {
                print_help();
            }
            _ => {
                println!("❌ 未知参数: {}", args[1]);
                print_help();
            }
        }
    } else {
        // 默认运行完整的学习示例
        println!("📖 运行完整学习指南\n");
        string_slice_comprehensive::run_comprehensive_examples();

        println!("\n💡 提示: 使用以下命令探索更多内容:");
        println!("   cargo run exercises  # 运行练习题");
        println!("   cargo run challenge  # 运行挑战练习");
        println!("   cargo run help       # 显示帮助信息");
    }
}

fn print_help() {
    println!("🦀 Rust 字符串与切片学习指南");
    println!("\n📋 可用命令:");
    println!("   cargo run              # 运行完整学习指南（默认）");
    println!("   cargo run exercises    # 运行练习题");
    println!("   cargo run challenge    # 运行挑战练习");
    println!("   cargo run help         # 显示此帮助信息");
    println!("\n📚 学习内容:");
    println!("   • 切片基本概念和语法");
    println!("   • 字符串切片 (&str) 详解");
    println!("   • String 类型和操作方法");
    println!("   • UTF-8 编码和 Unicode 处理");
    println!("   • 字符串转换和解析");
    println!("   • 性能优化和最佳实践");
    println!("   • 实用函数和高级操作");
    println!("\n🎯 建议学习路径:");
    println!("   1. 先运行默认模式学习基础概念");
    println!("   2. 通过练习题巩固知识");
    println!("   3. 挑战高级文本分析器项目");
    println!("\n📖 参考资源: https://course.rs/basic/compound-type/string-slice.html");
}
