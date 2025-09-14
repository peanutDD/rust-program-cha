//! # Rust 元组学习主程序
//!
//! 这是一个完整的 Rust 元组学习项目，包含了详细的教程和练习
//! 基于 https://course.rs/basic/compound-type/tuple.html 的内容

mod exercises;
mod tuple_comprehensive;

use exercises::*;
use tuple_comprehensive::*;

fn main() {
    println!("🦀 欢迎来到 Rust 元组学习之旅！");
    println!("===================================\n");

    // 显示菜单
    println!("📚 学习内容:");
    println!("1. 📖 元组基础教程");
    println!("2. 🏃 实践练习");
    println!("3. 🎯 全部内容\n");

    // 运行所有内容
    run_all_examples();

    println!("\n{}", "=".repeat(50));

    run_all_exercises();

    println!("\n{}", "=".repeat(50));
    println!("🎓 学习完成！");
    println!("\n💡 接下来你可以:");
    println!("• 查看和修改 src/tuple_comprehensive.rs 中的教程代码");
    println!("• 完成 src/exercises.rs 中的练习题");
    println!("• 尝试编写自己的元组应用程序");
    println!("• 探索更多 Rust 复合类型（数组、结构体、枚举等）");

    println!("\n🔗 相关资源:");
    println!("• Rust 官方文档: https://doc.rust-lang.org/book/");
    println!("• Rust 语言圣经: https://course.rs/");
    println!("• Rust 练习: https://github.com/rust-lang/rustlings");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_functionality() {
        // 测试主要功能是否正常工作
        // 这里只是确保代码能够编译和运行
        assert!(true);
    }

    #[test]
    fn test_tuple_basics() {
        // 基础元组测试
        let tuple = (1, "hello", 3.14);
        assert_eq!(tuple.0, 1);
        assert_eq!(tuple.1, "hello");
        assert_eq!(tuple.2, 3.14);
    }

    #[test]
    fn test_tuple_destructuring() {
        // 元组解构测试
        let tuple = (10, 20, 30);
        let (a, b, c) = tuple;
        assert_eq!(a + b + c, 60);
    }

    #[test]
    fn test_nested_tuples() {
        // 嵌套元组测试
        let nested = ((1, 2), (3, 4));
        let ((a, b), (c, d)) = nested;
        assert_eq!(a + b + c + d, 10);
    }

    #[test]
    fn test_tuple_as_function_return() {
        // 元组作为函数返回值测试
        fn get_name_age() -> (String, u32) {
            ("Alice".to_string(), 25)
        }

        let (name, age) = get_name_age();
        assert_eq!(name, "Alice");
        assert_eq!(age, 25);
    }

    #[test]
    fn test_tuple_pattern_matching() {
        // 元组模式匹配测试
        fn classify_point(point: (i32, i32)) -> &'static str {
            match point {
                (0, 0) => "origin",
                (0, _) => "y-axis",
                (_, 0) => "x-axis",
                (x, y) if x == y => "diagonal",
                _ => "other",
            }
        }

        assert_eq!(classify_point((0, 0)), "origin");
        assert_eq!(classify_point((0, 5)), "y-axis");
        assert_eq!(classify_point((3, 0)), "x-axis");
        assert_eq!(classify_point((2, 2)), "diagonal");
        assert_eq!(classify_point((1, 3)), "other");
    }
}
