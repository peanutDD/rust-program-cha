// 引入我们的语句与表达式指南模块
mod statements_expressions_guide;

use statements_expressions_guide::*;

fn main() {
  // 运行完整的语句与表达式演示
  run_all_demos();

  println!("\n\n🎯 想要深入学习特定主题？");
  println!("可以单独调用以下函数：");
  println!("- basic_concepts_demo()           // 基本概念");
  println!("- declaration_statements_demo()   // 声明语句");
  println!("- block_expressions_demo()        // 块表达式");
  println!("- if_expressions_demo()           // if 表达式");
  println!("- match_expressions_demo()        // match 表达式");
  println!("- practical_examples()            // 实际应用");
  println!("- common_mistakes_demo()          // 常见错误");
  println!("- advanced_usage_demo()           // 高级用法");

  println!("\n📚 参考资料：");
  println!("- Rust 语言圣经: https://course.rs/basic/base-type/statement-expression.html");
  println!("- Rust Reference: https://doc.rust-lang.org/reference/statements-and-expressions.html");
}
