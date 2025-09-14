// main.rs
// Rust 引用与借用学习项目主入口

mod borrowing_analysis;
mod exercises;

fn main() {
  println!("🦀 欢迎来到 Rust 引用与借用学习项目！");
  println!("📚 本项目基于 https://course.rs/basic/ownership/borrowing.html");
  println!("🎯 涵盖了引用与借用的所有核心概念和实践案例\n");

  // 运行所有引用与借用的示例
  borrowing_analysis::run_all_examples();

  println!("\n✅ 恭喜！你已经完成了 Rust 引用与借用的学习！");
  println!("💡 关键要点回顾：");
  println!("   • 引用允许使用值但不获取所有权");
  println!("   • 借用规则确保内存安全");
  println!("   • 可变引用和不可变引用不能同时存在");
  println!("   • 切片提供了安全访问集合部分数据的方式");
  println!("   • 生命周期确保引用的有效性");
  println!("\n🚀 继续探索 Rust 的其他特性吧！");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_reference() {
    let s = String::from("hello");
    let len = s.len();
    let r = &s;
    assert_eq!(r.len(), len);
    // s 仍然有效
    assert_eq!(s, "hello");
  }

  #[test]
  fn test_mutable_reference() {
    let mut s = String::from("hello");
    {
      let r = &mut s;
      r.push_str(", world");
    }
    assert_eq!(s, "hello, world");
  }

  #[test]
  fn test_string_slice() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    assert_eq!(hello, "hello");
    assert_eq!(world, "world");
  }

  #[test]
  fn test_array_slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
  }

  #[test]
  fn test_first_word() {
    fn first_word(s: &str) -> &str {
      let bytes = s.as_bytes();
      for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
          return &s[0..i];
        }
      }
      &s[..]
    }

    let s = String::from("hello world");
    assert_eq!(first_word(&s), "hello");

    let s_literal = "rust programming";
    assert_eq!(first_word(s_literal), "rust");
  }

  #[test]
  fn test_lifetime_function() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
      if x.len() > y.len() { x } else { y }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    assert_eq!(result, "abcd");
  }
}
