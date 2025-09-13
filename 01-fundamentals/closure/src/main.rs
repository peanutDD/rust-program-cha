//! # Rust 闭包学习项目主程序
//!
//! 这个项目基于 https://course.rs/advance/functional-programing/closure.html
//! 提供了全面的 Rust 闭包学习指南和实践案例

mod closure_comprehensive_guide;

use closure_comprehensive_guide::*;

fn main() {
  // 运行所有闭包示例和教程
  run_all_examples();

  println!("\n{}", "=".repeat(60));
  println!("🚀 想要深入学习？可以尝试以下操作:");
  println!("1. 运行 `cargo test` 查看测试用例");
  println!("2. 修改示例代码，观察编译器的反应");
  println!("3. 尝试创建自己的闭包示例");
  println!("4. 阅读源码中的详细注释");

  println!("\n📚 推荐进一步学习资源:");
  println!("- Rust 官方文档: https://doc.rust-lang.org/book/ch13-01-closures.html");
  println!("- Rust 语言圣经: https://course.rs/advance/functional-programing/closure.html");
  println!("- Rust By Example: https://doc.rust-lang.org/rust-by-example/fn/closures.html");
}

/// 演示一些交互式的闭包示例
fn interactive_examples() {
  println!("\n🎯 交互式闭包示例");

  // 创建一个简单的计算器
  let mut calculator = {
    let mut history = Vec::new();

    move |operation: &str, a: f64, b: f64| -> f64 {
      let result = match operation {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
          if b != 0.0 {
            a / b
          } else {
            f64::NAN
          }
        }
        _ => f64::NAN,
      };

      history.push(format!("{} {} {} = {}", a, operation, b, result));
      println!("计算: {} {} {} = {}", a, operation, b, result);
      println!("历史记录数量: {}", history.len());

      result
    }
  };

  // 使用计算器
  calculator("+", 10.0, 5.0);
  calculator("*", 3.0, 4.0);
  calculator("/", 15.0, 3.0);
  calculator("-", 20.0, 8.0);
}

#[cfg(test)]
mod integration_tests {
  use super::*;

  #[test]
  fn test_closure_integration() {
    // 测试闭包的集成使用
    let data = vec![1, 2, 3, 4, 5];

    // 链式操作
    let result: Vec<i32> = data
      .iter()
      .filter(|&&x| x % 2 == 0) // 过滤偶数
      .map(|&x| x * x) // 平方
      .collect();

    assert_eq!(result, vec![4, 16]);
  }

  #[test]
  fn test_closure_composition() {
    // 测试闭包组合
    let add_one = |x: i32| x + 1;
    let multiply_two = |x: i32| x * 2;

    let composed = |x| multiply_two(add_one(x));

    assert_eq!(composed(5), 12); // (5 + 1) * 2 = 12
  }

  #[test]
  fn test_closure_with_state() {
    // 测试带状态的闭包
    let mut counter = 0;
    let mut increment = || {
      counter += 1;
      counter
    };

    assert_eq!(increment(), 1);
    assert_eq!(increment(), 2);
    assert_eq!(increment(), 3);
  }
}
