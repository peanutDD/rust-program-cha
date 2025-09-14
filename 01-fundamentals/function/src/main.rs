// 引入我们的 Rust 函数详解指南模块
mod function_syntax_analysis;
mod rust_function_comprehensive_guide;

use function_syntax_analysis::*;
use rust_function_comprehensive_guide::*;

fn main() {
  // 运行函数语法深度解析
  comprehensive_function_syntax_analysis();

  // 显示语法要点总结
  syntax_summary();

  println!("\n{}", "=".repeat(80));
  println!("\n🔄 接下来运行完整的函数演示...");
  println!("{}", "=".repeat(80));

  // 运行完整的函数演示
  run_all_function_demos();

  println!("\n\n🎯 想要深入学习特定主题？");
  println!("可以单独调用以下函数：");
  println!("- basic_function_concepts()           // 函数基础概念");
  println!("- function_definition_and_calling()   // 函数定义与调用");
  println!("- function_parameters_demo()          // 函数参数详解");
  println!("- function_return_values_demo()       // 函数返回值");
  println!("- statements_expressions_in_functions() // 语句与表达式");
  println!("- function_scope_and_lifetime()       // 函数作用域与生命周期");
  println!("- advanced_function_features()        // 高级函数特性");
  println!("- practical_function_examples()       // 实际应用示例");
  println!("- common_mistakes_and_best_practices() // 常见错误与最佳实践");
  println!("- performance_optimization_tips()     // 性能优化技巧");

  println!("\n📚 参考资料：");
  println!("- Rust 语言圣经 - 函数: https://course.rs/basic/base-type/function.html");
  println!(
    "- Rust 程序设计语言 - 函数: https://kaisery.github.io/trpl-zh-cn/ch03-03-how-functions-work.html"
  );
  println!(
    "- Rust Reference - 函数: https://vectorworkshopbaoerjie.github.io/book/rustreference/items/functions.html"
  );
}
