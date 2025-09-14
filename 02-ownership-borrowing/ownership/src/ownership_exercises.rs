//! # Rust 所有权系统练习题
//!
//! 基于 https://course.rs/basic/ownership/ownership.html 和 https://practice.rs/ownership/ownership.html
//! 提供循序渐进的练习题，帮助理解和掌握所有权概念

/// 练习1：基础所有权转移
///
/// 修复下面的代码，使其能够编译通过
pub fn exercise_1_basic_ownership() {
  println!("\n=== 练习1：基础所有权转移 ===");

  // TODO: 修复这段代码
  // 提示：考虑如何在不失去所有权的情况下使用字符串

  let s1 = String::from("hello");
  let s2 = s1.clone(); // 修复：使用 clone() 而不是直接移动

  println!("s1: {}", s1);
  println!("s2: {}", s2);

  println!("✅ 练习1完成：通过克隆避免所有权转移");
}

/// 练习2：函数参数的所有权
///
/// 实现一个函数，接收字符串所有权并返回处理后的字符串
pub fn exercise_2_function_ownership() {
  println!("\n=== 练习2：函数参数的所有权 ===");

  fn take_and_give_back(s: String) -> String {
    let mut result = s;
    result.push_str(", world!");
    result // 返回所有权给调用者
  }

  let s1 = String::from("hello");
  let s2 = take_and_give_back(s1);

  // println!("{}", s1); // 这行会编译错误，因为 s1 的所有权已转移
  println!("处理后的字符串: {}", s2);

  println!("✅ 练习2完成：理解函数中的所有权转移");
}

/// 练习3：借用与引用
///
/// 使用引用来避免所有权转移
pub fn exercise_3_borrowing() {
  println!("\n=== 练习3：借用与引用 ===");

  fn calculate_length(s: &String) -> usize {
    s.len() // 借用，不获取所有权
  }

  fn add_suffix(s: &mut String) {
    s.push_str(" - modified");
  }

  let s = String::from("hello");
  let len = calculate_length(&s); // 借用 s
  println!("字符串 '{}' 的长度是: {}", s, len); // s 仍然有效

  let mut s2 = String::from("hello");
  add_suffix(&mut s2); // 可变借用
  println!("修改后的字符串: {}", s2);

  println!("✅ 练习3完成：掌握借用机制");
}

/// 练习4：借用规则
///
/// 理解借用的限制规则
pub fn exercise_4_borrowing_rules() {
  println!("\n=== 练习4：借用规则 ===");

  let mut s = String::from("hello");

  // 规则1：可以有多个不可变引用
  let r1 = &s;
  let r2 = &s;
  println!("r1: {}, r2: {}", r1, r2);
  // r1 和 r2 的作用域在这里结束

  // 规则2：同一时间只能有一个可变引用
  let r3 = &mut s;
  r3.push_str(", world");
  println!("r3: {}", r3);
  // r3 的作用域在这里结束

  // 现在可以再次创建引用
  let r4 = &s;
  println!("r4: {}", r4);

  println!("✅ 练习4完成：理解借用规则的限制");
}

/// 练习5：悬垂引用预防
///
/// 理解 Rust 如何防止悬垂引用
pub fn exercise_5_dangling_references() {
  println!("\n=== 练习5：悬垂引用预防 ===");

  // 这个函数会导致编译错误 - 悬垂引用
  /*
  fn dangle() -> &String {
    let s = String::from("hello");
    &s // 错误：返回对局部变量的引用
  } // s 在这里被丢弃，返回的引用变成悬垂引用
  */

  // 正确的做法：返回拥有所有权的值
  fn no_dangle() -> String {
    let s = String::from("hello");
    s // 移动所有权给调用者
  }

  let result = no_dangle();
  println!("安全的字符串: {}", result);

  println!("✅ 练习5完成：理解悬垂引用的预防机制");
}

/// 练习6：字符串切片
///
/// 理解字符串切片的所有权特性
pub fn exercise_6_string_slices() {
  println!("\n=== 练习6：字符串切片 ===");

  fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
        return &s[0..i];
      }
    }

    &s[..] // 返回整个字符串的切片
  }

  let s = String::from("hello world");
  let word = first_word(&s);
  println!("第一个单词: {}", word);

  // 字符串字面量就是切片
  let s_literal = "hello world";
  let word2 = first_word(s_literal);
  println!("字面量的第一个单词: {}", word2);

  println!("✅ 练习6完成：掌握字符串切片的使用");
}

/// 练习7：结构体中的所有权
///
/// 理解结构体字段的所有权
pub fn exercise_7_struct_ownership() {
  println!("\n=== 练习7：结构体中的所有权 ===");

  #[derive(Debug)]
  struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
  }

  let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };

  // 部分移动：只移动某些字段
  let user2 = User {
    email: String::from("another@example.com"),
    username: user1.username,           // 移动 username
    active: user1.active,               // Copy trait，复制
    sign_in_count: user1.sign_in_count, // Copy trait，复制
  };

  // user1.username 不再可用，但其他字段仍可用
  // println!("{}", user1.username); // 编译错误
  println!("user1.email: {}", user1.email); // 仍然可用
  println!("user2: {:?}", user2);

  println!("✅ 练习7完成：理解结构体字段的部分移动");
}

/// 练习8：枚举中的所有权
///
/// 理解枚举变体的所有权
pub fn exercise_8_enum_ownership() {
  println!("\n=== 练习8：枚举中的所有权 ===");

  #[derive(Debug)]
  #[allow(dead_code)]
  enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
  }

  let msg1 = Message::Write(String::from("hello"));
  let msg2 = msg1; // 移动整个枚举值

  // println!("{:?}", msg1); // 编译错误：msg1 已被移动
  println!("移动后的消息: {:?}", msg2);

  // 使用 match 解构枚举
  match msg2 {
    Message::Write(text) => {
      println!("文本消息: {}", text); // text 拥有字符串的所有权
    }
    _ => println!("其他类型的消息"),
  }
  // msg2 在 match 中被消费，不再可用

  println!("✅ 练习8完成：理解枚举的所有权转移");
}

/// 练习9：向量的所有权
///
/// 理解向量元素的所有权
pub fn exercise_9_vector_ownership() {
  println!("\n=== 练习9：向量的所有权 ===");

  let mut v = vec![String::from("hello"), String::from("world")];

  // 移动向量中的元素
  let first = v.remove(0); // 移动第一个元素
  println!("移动的元素: {}", first);
  println!("剩余向量: {:?}", v);

  // 借用向量元素
  let v2 = vec![String::from("foo"), String::from("bar")];
  for item in &v2 {
    // 借用每个元素
    println!("借用的元素: {}", item);
  }
  println!("向量仍可用: {:?}", v2);

  // 移动向量中的所有元素
  for item in v2 {
    // 移动每个元素
    println!("移动的元素: {}", item);
  }
  // println!("{:?}", v2); // 编译错误：v2 已被移动

  println!("✅ 练习9完成：理解向量元素的所有权管理");
}

/// 练习10：闭包中的所有权
///
/// 理解闭包如何捕获变量
pub fn exercise_10_closure_ownership() {
  println!("\n=== 练习10：闭包中的所有权 ===");

  let s = String::from("hello");

  // 闭包借用变量
  let closure1 = || {
    println!("闭包中借用: {}", s);
  };
  closure1();
  println!("s 仍可用: {}", s);

  // 闭包移动变量
  let s2 = String::from("world");
  let closure2 = move || {
    println!("闭包中移动: {}", s2);
  };
  closure2();
  // println!("{}", s2); // 编译错误：s2 已被移动到闭包中

  // 闭包修改变量
  let mut count = 0;
  let mut closure3 = || {
    count += 1;
    println!("计数: {}", count);
  };
  closure3();
  closure3();
  println!("最终计数: {}", count);

  println!("✅ 练习10完成：理解闭包的变量捕获机制");
}

/// 综合练习：实现一个简单的文本处理器
///
/// 综合运用所有权、借用、生命周期等概念
pub fn comprehensive_exercise() {
  println!("\n=== 综合练习：文本处理器 ===");

  struct TextProcessor {
    content: String,
  }

  impl TextProcessor {
    fn new(content: String) -> Self {
      TextProcessor { content }
    }

    // 借用内容进行只读操作
    fn word_count(&self) -> usize {
      self.content.split_whitespace().count()
    }

    // 可变借用进行修改操作
    fn add_prefix(&mut self, prefix: &str) {
      self.content = format!("{}: {}", prefix, self.content);
    }

    // 消费 self，返回处理后的内容
    fn into_uppercase(self) -> String {
      self.content.to_uppercase()
    }

    // 返回内容的引用
    fn get_content(&self) -> &str {
      &self.content
    }
  }

  let mut processor = TextProcessor::new(String::from("hello world"));

  // 使用借用进行只读操作
  println!("单词数量: {}", processor.word_count());
  println!("原始内容: {}", processor.get_content());

  // 使用可变借用进行修改
  processor.add_prefix("Greeting");
  println!("添加前缀后: {}", processor.get_content());

  // 消费对象，获取最终结果
  let final_content = processor.into_uppercase();
  println!("最终结果: {}", final_content);

  // processor 不再可用
  // println!("{}", processor.get_content()); // 编译错误

  println!("✅ 综合练习完成：成功实现文本处理器");
}

/// 运行所有练习
pub fn run_all_exercises() {
  println!("🎯 Rust 所有权系统练习题");
  println!("{}", "=".repeat(60));

  exercise_1_basic_ownership();
  exercise_2_function_ownership();
  exercise_3_borrowing();
  exercise_4_borrowing_rules();
  exercise_5_dangling_references();
  exercise_6_string_slices();
  exercise_7_struct_ownership();
  exercise_8_enum_ownership();
  exercise_9_vector_ownership();
  exercise_10_closure_ownership();
  comprehensive_exercise();

  println!("\n{}", "=".repeat(60));
  println!("🎉 所有练习完成！您已掌握 Rust 所有权系统的核心概念！");
  println!("💡 建议：重复练习，直到完全理解每个概念");
}

/// 挑战练习：高级所有权场景
pub fn challenge_exercises() {
  println!("\n🔥 挑战练习：高级所有权场景");
  println!("{}", "=".repeat(60));

  challenge_1_complex_borrowing();
  challenge_2_lifetime_annotations();
  challenge_3_smart_pointers();
}

/// 挑战1：复杂借用场景
fn challenge_1_complex_borrowing() {
  println!("\n--- 挑战1：复杂借用场景 ---");

  fn process_data(data: &mut Vec<String>) -> Vec<&str> {
    // 在这个函数中，我们需要返回对 data 中元素的引用
    // 这涉及到复杂的生命周期管理
    data.iter().map(|s| s.as_str()).collect()
  }

  let mut data = vec![
    String::from("apple"),
    String::from("banana"),
    String::from("cherry"),
  ];

  let references = process_data(&mut data);
  println!("引用列表: {:?}", references);

  // 注意：references 的生命周期与 data 绑定
  data.push(String::from("date"));
  // println!("{:?}", references); // 可能的编译错误，取决于具体使用

  println!("✅ 挑战1完成：处理复杂的借用关系");
}

/// 挑战2：生命周期注解
fn challenge_2_lifetime_annotations() {
  println!("\n--- 挑战2：生命周期注解 ---");

  // 函数需要明确的生命周期注解
  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
  }

  let string1 = String::from("long string is long");
  let result;
  {
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
    println!("最长的字符串: {}", result);
  }
  // result 的生命周期受到 string2 的限制

  // 结构体中的生命周期
  struct ImportantExcerpt<'a> {
    part: &'a str,
  }

  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find a '.'");
  let i = ImportantExcerpt {
    part: first_sentence,
  };

  println!("重要摘录: {}", i.part);

  println!("✅ 挑战2完成：掌握生命周期注解");
}

/// 挑战3：智能指针
fn challenge_3_smart_pointers() {
  println!("\n--- 挑战3：智能指针 ---");

  use std::cell::RefCell;
  use std::rc::Rc;

  // Rc<RefCell<T>> 模式：共享可变数据
  let data = Rc::new(RefCell::new(vec![1, 2, 3]));

  let data1 = Rc::clone(&data);
  let data2 = Rc::clone(&data);

  // 通过不同的引用修改共享数据
  data1.borrow_mut().push(4);
  data2.borrow_mut().push(5);

  println!("共享数据: {:?}", data.borrow());
  println!("引用计数: {}", Rc::strong_count(&data));

  // Box<T> 用于递归类型
  #[derive(Debug)]
  #[allow(dead_code)]
  enum List {
    Cons(i32, Box<List>),
    Nil,
  }

  use List::{Cons, Nil};

  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  println!("递归列表: {:?}", list);

  println!("✅ 挑战3完成：掌握智能指针的使用");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_ownership() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    assert_eq!(s1, "hello");
    assert_eq!(s2, "hello");
  }

  #[test]
  fn test_borrowing() {
    fn calculate_length(s: &String) -> usize {
      s.len()
    }

    let s = String::from("hello");
    let len = calculate_length(&s);
    assert_eq!(len, 5);
    assert_eq!(s, "hello"); // s 仍然有效
  }

  #[test]
  fn test_mutable_borrowing() {
    fn change(s: &mut String) {
      s.push_str(", world");
    }

    let mut s = String::from("hello");
    change(&mut s);
    assert_eq!(s, "hello, world");
  }

  #[test]
  fn test_string_slices() {
    fn first_word(s: &str) -> &str {
      let bytes = s.as_bytes();
      for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
          return &s[0..i];
        }
      }
      &s[..]
    }

    let s = "hello world";
    assert_eq!(first_word(s), "hello");
  }

  #[test]
  fn test_struct_ownership() {
    #[derive(Debug, PartialEq, Copy, Clone)]
    struct Point {
      x: i32,
      y: i32,
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = p1; // Point 实现了 Copy，所以这是复制而不是移动

    assert_eq!(p1, Point { x: 1, y: 2 });
    assert_eq!(p2, Point { x: 1, y: 2 });
  }
}
