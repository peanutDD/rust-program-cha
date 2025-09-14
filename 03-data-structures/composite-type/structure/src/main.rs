mod exercises;
mod struct_comprehensive;

use exercises::run_all_exercises;
use struct_comprehensive::run_all_examples;

fn main() {
    println!("🦀 Rust 结构体完整教程");
    println!("========================");

    // 运行教程示例
    println!("\n📚 第一部分：结构体教程");
    run_all_examples();

    // 运行练习题
    println!("\n\n🎯 第二部分：结构体练习");
    run_all_exercises();

    println!("\n🎉 教程和练习全部完成！");
    println!("\n💡 提示：");
    println!("- 运行 `cargo test` 可以执行所有测试");
    println!("- 查看源码了解更多实现细节");
    println!("- 尝试修改代码进行实验");
}
