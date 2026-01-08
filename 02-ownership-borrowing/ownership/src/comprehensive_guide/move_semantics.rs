//! # 第二部分：移动语义深入解析
//!
//! 移动语义是 Rust 所有权系统的核心机制，避免了昂贵的深拷贝操作。

use std::collections::HashMap;

/// ## 第二部分：移动语义深入解析
///
/// 移动语义是 Rust 所有权系统的核心机制，避免了昂贵的深拷贝操作。
pub fn move_semantics_deep_dive() {
  println!("\n=== 第二部分：移动语义深入解析 ===");

  move_vs_copy_analysis();
  function_ownership_transfer();
  return_value_ownership();
  partial_moves_explanation();
}

/// ### 2.1 移动 vs 复制语义
///
/// 深入理解何时发生移动，何时发生复制。
fn move_vs_copy_analysis() {
  println!("\n--- 2.1 移动 vs 复制语义 ---");

  println!("\n🔍 Copy trait 类型（复制语义）：");

  // 基本类型都实现了 Copy trait
  let a = 5;
  let b = a; // 复制值
  println!("整数复制: a={}, b={} (都有效)", a, b);

  let c = 3.14;
  let d = c; // 复制值
  println!("浮点复制: c={}, d={} (都有效)", c, d);

  let e = true;
  let f = e; // 复制值
  println!("布尔复制: e={}, f={} (都有效)", e, f);

  let g = 'A';
  let h = g; // 复制值
  println!("字符复制: g={}, h={} (都有效)", g, h);

  println!("\n🔄 Move 语义类型（移动语义）：");

  // String 类型不实现 Copy，使用移动语义
  let s1 = String::from("移动语义");
  let s2 = s1; // 移动所有权
  // println!("{}", s1);  // 编译错误！
  println!("String 移动: s2={} (s1 已失效)", s2);

  // Vec 类型也使用移动语义
  let v1 = vec![1, 2, 3, 4, 5];
  let v2 = v1; // 移动所有权
  // println!("{:?}", v1);  // 编译错误！
  println!("Vec 移动: v2={:?} (v1 已失效)", v2);

  // HashMap 也使用移动语义
  let mut map1 = HashMap::new();
  map1.insert("key", "value");
  let map2 = map1; // 移动所有权
  // println!("{:?}", map1);  // 编译错误！
  println!("HashMap 移动: map2={:?} (map1 已失效)", map2);

  println!("\n💡 判断规则：");
  println!("• 实现 Copy trait：按位复制，原变量仍有效");
  println!("• 未实现 Copy trait：移动所有权，原变量失效");
  println!("• 包含堆数据的类型通常不实现 Copy");
}

/// ### 2.2 函数调用中的所有权转移
///
/// 函数参数和返回值如何影响所有权。
fn function_ownership_transfer() {
  println!("\n--- 2.2 函数调用中的所有权转移 ---");

  println!("\n🔍 函数参数所有权转移：");

  // 获取所有权的函数
  fn takes_ownership(some_string: String) {
    println!("函数内部: {}", some_string);
  } // some_string 在这里离开作用域并被释放

  // 复制参数的函数
  fn makes_copy(some_integer: i32) {
    println!("函数内部: {}", some_integer);
  } // some_integer 离开作用域，但因为是 Copy 类型，没有特殊处理

  let s = String::from("hello");
  takes_ownership(s); // s 的所有权移动到函数中
  // println!("{}", s);  // 编译错误！s 已失效

  let x = 5;
  makes_copy(x); // x 被复制到函数中
  println!("x 仍然有效: {}", x); // x 仍然有效

  println!("\n🔄 避免所有权转移的方法：");

  // 使用引用避免所有权转移
  fn calculate_length(s: &String) -> usize {
    s.len()
  }

  let s1 = String::from("hello world");
  let len = calculate_length(&s1); // 传递引用，不转移所有权
  println!("字符串 '{}' 的长度是 {}", s1, len); // s1 仍然有效

  // 使用克隆创建副本
  fn process_string(s: String) -> String {
    format!("{} - 已处理", s)
  }

  let original = String::from("原始数据");
  let processed = process_string(original.clone()); // 克隆避免移动
  println!("原始: {}, 处理后: {}", original, processed);
}

/// ### 2.3 返回值与所有权
///
/// 函数返回值如何转移所有权。
fn return_value_ownership() {
  println!("\n--- 2.3 返回值与所有权 ---");

  println!("\n🔍 返回值所有权转移：");

  // 返回所有权的函数
  fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // 返回 some_string，所有权移动到调用者
  }

  // 获取所有权并返回的函数
  fn takes_and_gives_back(a_string: String) -> String {
    a_string // 返回 a_string，所有权移动到调用者
  }

  let s1 = gives_ownership(); // gives_ownership 将返回值的所有权移动给 s1
  println!("获得所有权: {}", s1);

  let s2 = String::from("hello");
  let s3 = takes_and_gives_back(s2); // s2 移动到函数中，返回值移动给 s3
  // println!("{}", s2);  // 编译错误！s2 已失效
  println!("转移所有权: {}", s3);

  println!("\n🔄 返回多个值：");

  // 返回元组来返回多个值
  fn calculate_length_and_return(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // 返回字符串和长度
  }

  let s4 = String::from("hello world");
  let (s5, len) = calculate_length_and_return(s4);
  // println!("{}", s4);  // 编译错误！s4 已失效
  println!("返回的字符串: {}, 长度: {}", s5, len);

  println!("\n💡 最佳实践：");
  println!("• 使用引用避免不必要的所有权转移");
  println!("• 返回值可以转移所有权给调用者");
  println!("• 考虑使用元组返回多个值");
}

/// ### 2.4 部分移动详解
///
/// 结构体和枚举中的部分移动机制。
fn partial_moves_explanation() {
  println!("\n--- 2.4 部分移动详解 ---");

  #[derive(Debug)]
  struct Person {
    name: String,
    age: u32,
    email: String,
  }

  println!("\n🔍 结构体部分移动：");

  let person = Person {
    name: String::from("张三"),
    age: 30,
    email: String::from("zhangsan@example.com"),
  };

  // 移动部分字段
  let name = person.name; // 移动 name 字段
  let age = person.age; // 复制 age 字段（u32 实现了 Copy）

  println!("移动的姓名: {}", name);
  println!("复制的年龄: {}", age);

  // person.name 已被移动，但 person.email 仍可访问
  println!("剩余的邮箱: {}", person.email);

  // println!("{}", person.name);  // 编译错误！name 已被移动
  // println!("{:?}", person);     // 编译错误！整个结构体不完整

  println!("\n🔍 枚举中的移动：");

  #[derive(Debug)]
  #[allow(dead_code)]
  enum Message {
    Text(String),
    Number(i32),
    Coordinate { x: i32, y: i32 },
  }

  let msg = Message::Text(String::from("Hello"));

  match msg {
    Message::Text(text) => {
      println!("提取的文本: {}", text);
      // text 的所有权被移动到这里
    }
    Message::Number(num) => {
      println!("数字: {}", num);
    }
    Message::Coordinate { x, y } => {
      println!("坐标: ({}, {})", x, y);
    }
  }

  // println!("{:?}", msg);  // 编译错误！msg 的内容已被移动

  println!("\n💡 部分移动规则：");
  println!("• 可以移动结构体的部分字段");
  println!("• 移动后，原结构体不能作为整体使用");
  println!("• 未移动的字段仍可单独访问");
  println!("• 模式匹配可能导致值的移动");
}
