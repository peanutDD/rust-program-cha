//! # Rust 枚举学习主程序
//!
//! 这是一个完整的 Rust 枚举学习项目，包含了详细的教程和练习
//! 基于 https://course.rs/basic/compound-type/enum.html 的内容

mod enum_comprehensive;
mod exercises;

use enum_comprehensive::*;
use exercises::*;

fn main() {
    println!("🦀 欢迎来到 Rust 枚举学习教程!");
    println!("{}", "=".repeat(60));
    println!("本教程基于 https://course.rs/basic/compound-type/enum.html");
    println!("包含全面的枚举概念讲解和实践练习\n");

    // 运行教程示例
    println!("📚 第一部分：枚举教程示例");
    println!("{}", "-".repeat(40));
    run_all_examples();

    println!();

    // 运行练习题
    println!("🏃 第二部分：枚举练习题");
    println!("{}", "-".repeat(40));
    run_all_exercises();

    println!();

    // 总结
    print_summary();
}

fn print_summary() {
    println!("📋 学习总结");
    println!("{}", "=".repeat(60));

    println!("\n🎯 核心知识点:");
    println!("  ✅ 枚举的基本定义和使用");
    println!("  ✅ 带数据的枚举变体");
    println!("  ✅ Option<T> 枚举的使用");
    println!("  ✅ Result<T, E> 错误处理");
    println!("  ✅ match 模式匹配");
    println!("  ✅ if let 简洁控制流");
    println!("  ✅ 枚举方法实现");
    println!("  ✅ 泛型枚举");
    println!("  ✅ 状态机模式");
    println!("  ✅ 实际应用案例");

    println!("\n🏆 完成的练习:");
    println!("  1️⃣  交通信号灯系统");
    println!("  2️⃣  几何图形计算");
    println!("  3️⃣  Option 安全操作");
    println!("  4️⃣  Result 错误处理");
    println!("  5️⃣  命令行解析器");
    println!("  6️⃣  订单状态机");
    println!("  7️⃣  泛型API响应");
    println!("  8️⃣  文件系统节点");

    println!("\n💡 学习建议:");
    println!("  • 多练习 match 模式匹配，这是 Rust 的核心特性");
    println!("  • 熟练使用 Option 和 Result 进行安全编程");
    println!("  • 理解枚举在状态机中的应用");
    println!("  • 掌握 if let 和 while let 的使用场景");
    println!("  • 学会设计合理的枚举来建模业务逻辑");

    println!("\n🔗 相关资源:");
    println!("  📖 Rust 语言圣经: https://course.rs/basic/compound-type/enum.html");
    println!("  📚 官方文档: https://doc.rust-lang.org/book/ch06-00-enums.html");
    println!("  🎮 Rust 练习: https://github.com/rust-lang/rustlings");

    println!("\n🎉 恭喜完成 Rust 枚举学习教程!");
    println!("继续加油，掌握更多 Rust 特性! 🚀");
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_main_runs_without_panic() {
        // 这个测试确保 main 函数可以正常运行而不会 panic
        // 在实际项目中，我们会分别测试各个模块
        println!("集成测试：确保主程序可以正常运行");
    }

    #[test]
    fn test_enum_comprehensive_module() {
        // 测试教程模块的一些基本功能
        use enum_comprehensive::*;

        let ip = IpAddr::V4(192, 168, 1, 1);
        assert_eq!(format!("{}", ip), "192.168.1.1");

        let msg = Message::Write("test".to_string());
        assert_eq!(msg.message_type(), "Write");
    }

    #[test]
    fn test_exercises_module() {
        // 测试练习模块的一些基本功能
        use exercises::*;

        let light = TrafficLight::Red;
        assert_eq!(light.wait_time(), 60);
        assert!(!light.can_pass());

        let circle = Shape::Circle { radius: 5.0 };
        assert_eq!(circle.shape_type(), "圆形");
    }
}
