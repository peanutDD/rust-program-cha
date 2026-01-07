//! # Rust 所有权系统全面指南
//!
//! 本模块提供了 Rust 所有权系统的全面解析，包括：
//! - 所有权的核心概念和规则
//! - 移动语义和复制语义
//! - 引用与借用机制
//! - 生命周期管理
//! - 闭包的详细解释
//! - 实际应用案例和最佳实践

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// # Rust 所有权系统全面学习指南
///
/// 所有权是 Rust 最独特和核心的特性，它使得 Rust 能够在不使用垃圾回收器的情况下保证内存安全。
pub fn comprehensive_ownership_guide() {
  println!("🦀 Rust 所有权系统全面学习指南");
  println!("{}", "=".repeat(80));

  // 第一部分：所有权基础理论
  ownership_fundamentals();

  // 第二部分：移动语义深入解析
  move_semantics_deep_dive();

  // 第三部分：引用与借用机制
  references_and_borrowing();

  // 第四部分：生命周期管理
  lifetime_management();

  // 第五部分：闭包详细解释
  closure_comprehensive_explanation();

  // 第六部分：智能指针与所有权
  smart_pointers_and_ownership();

  // 第七部分：实际应用案例
  practical_ownership_examples();

  // 第八部分：常见错误与最佳实践
  common_mistakes_and_best_practices();

  println!("\n{}", "=".repeat(80));
  println!("✅ 所有权系统学习完成！掌握了 Rust 的核心特性。");
}

/// ## 第一部分：所有权基础理论
///
/// 所有权系统是 Rust 内存管理的核心，基于三个基本规则。
pub fn ownership_fundamentals() {
  println!("\n=== 第一部分：所有权基础理论 ===");

  ownership_rules_explanation();
  ownership_scope_demonstration();
  string_ownership_examples();
  heap_vs_stack_analysis();
}

/// ### 1.1 所有权三大规则
///
/// Rust 的所有权系统基于三个基本规则，这些规则在编译时被强制执行。
fn ownership_rules_explanation() {
  println!("\n--- 1.1 所有权三大规则 ---");

  println!("\n📋 所有权规则：");
  println!("1. Rust 中的每一个值都有一个被称为其所有者（owner）的变量");
  println!("2. 值在任一时刻有且只有一个所有者");
  println!("3. 当所有者（变量）离开作用域，这个值将被丢弃");

  println!("\n🔍 规则详解：");

  // 规则 1 演示：每个值都有所有者
  {
    let s = String::from("hello"); // s 是字符串值的所有者
    println!("规则1演示: 变量 s 拥有字符串 '{}'", s);
  } // s 离开作用域，字符串被释放

  // 规则 2 演示：同一时刻只有一个所有者
  {
    let s1 = String::from("world");
    let s2 = s1; // 所有权从 s1 移动到 s2
    // println!("{}", s1);  // 编译错误！s1 不再有效
    println!("规则2演示: 所有权已从 s1 移动到 s2: {}", s2);
  }

  // 规则 3 演示：离开作用域时自动释放
  {
    let s = String::from("auto drop");
    println!("规则3演示: {} 将在作用域结束时自动释放", s);
  } // 这里 s 离开作用域，内存被自动释放

  println!("✓ 所有权规则确保了内存安全，无需手动管理内存");
}

/// ### 1.2 作用域与所有权
///
/// 作用域决定了变量的生命周期，是所有权系统的基础。
fn ownership_scope_demonstration() {
  println!("\n--- 1.2 作用域与所有权演示 ---");

  println!("\n🔍 作用域层次演示：");

  // 外层作用域
  let outer_var = "外层变量";
  println!("外层作用域: {}", outer_var);

  {
    // 内层作用域
    let inner_var = String::from("内层变量");
    println!("内层作用域: {}", inner_var);
    println!("内层可以访问外层: {}", outer_var);

    {
      // 更深层作用域
      let deep_var = "深层变量";
      println!("深层作用域: {}", deep_var);
      println!("深层可以访问外层: {}, {}", outer_var, inner_var);
    } // deep_var 在这里被释放

    println!("回到内层作用域: {}", inner_var);
    // println!("{}", deep_var);  // 编译错误！deep_var 已超出作用域
  } // inner_var 在这里被释放

  println!("回到外层作用域: {}", outer_var);
  // println!("{}", inner_var);  // 编译错误！inner_var 已超出作用域

  println!("✓ 变量在离开其定义的作用域时自动释放");
}

/// ### 1.3 字符串所有权详解
///
/// 通过字符串类型深入理解所有权的工作机制。
fn string_ownership_examples() {
  println!("\n--- 1.3 字符串所有权详解 ---");

  // 字符串字面量 vs String 类型
  println!("\n🔍 字符串类型对比：");

  // 字符串字面量：存储在程序二进制文件中，不可变
  let literal = "字符串字面量"; // &str 类型，存储在栈上
  println!("字符串字面量: {} (类型: &str)", literal);

  // String 类型：可变，存储在堆上
  let mut owned_string = String::from("可变字符串"); // String 类型，数据在堆上
  println!("String 类型: {} (类型: String)", owned_string);

  // String 的所有权转移
  println!("\n🔄 所有权转移演示：");

  let s1 = String::from("hello");
  println!("s1 创建: {}", s1);

  let s2 = s1; // 所有权从 s1 移动到 s2
  println!("s1 移动到 s2: {}", s2);
  // println!("{}", s1);  // 编译错误！s1 已失效

  // 克隆创建深拷贝
  let s3 = s2.clone(); // 创建 s2 的深拷贝
  println!("s2 克隆到 s3: s2={}, s3={}", s2, s3);

  // 字符串修改
  owned_string.push_str(" - 已修改");
  println!("修改后的字符串: {}", owned_string);

  println!("✓ String 类型涉及堆内存分配，所有权转移避免了双重释放");
}

/// ### 1.4 堆与栈的内存分析
///
/// 理解不同数据类型在内存中的存储方式。
fn heap_vs_stack_analysis() {
  println!("\n--- 1.4 堆与栈的内存分析 ---");

  println!("\n📊 内存布局分析：");

  // 栈上数据：Copy trait
  let x = 5; // 存储在栈上
  let y = x; // 复制值，不是移动
  println!("栈上数据 - x: {}, y: {} (两者都有效)", x, y);

  // 堆上数据：Move 语义
  let s1 = String::from("堆上数据"); // 字符串数据在堆上
  let s2 = s1; // 移动所有权，不是复制
  // println!("{}", s1);  // 编译错误！s1 已失效
  println!("堆上数据 - s2: {} (s1 已失效)", s2);

  // 复合类型的内存布局
  demonstrate_compound_types_memory();

  println!("\n💡 关键区别：");
  println!("• 栈上数据：固定大小，快速访问，自动复制");
  println!("• 堆上数据：动态大小，需要指针访问，移动所有权");
  println!("• Copy trait：允许简单的位复制");
  println!("• Drop trait：自定义清理逻辑");
}

fn demonstrate_compound_types_memory() {
  println!("\n🔍 复合类型内存演示：");

  // 元组：如果所有元素都实现 Copy，则元组也实现 Copy
  let tuple1 = (1, 2, 3); // 所有元素都是 i32，实现了 Copy
  let tuple2 = tuple1; // 复制，不是移动
  println!("Copy 元组 - tuple1: {:?}, tuple2: {:?}", tuple1, tuple2);

  // 包含 String 的元组：不实现 Copy
  let tuple3 = (String::from("hello"), 42);
  let tuple4 = tuple3; // 移动所有权
  // println!("{:?}", tuple3);  // 编译错误！
  println!("Move 元组 - tuple4: {:?}", tuple4);

  // 数组：如果元素类型实现 Copy，则数组也实现 Copy
  let arr1 = [1, 2, 3, 4, 5];
  let arr2 = arr1; // 复制整个数组
  println!("Copy 数组 - arr1: {:?}, arr2: {:?}", arr1, arr2);
}

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

/// ## 第三部分：引用与借用机制
///
/// 引用允许使用值而不获取其所有权，是 Rust 中重要的内存管理机制。
pub fn references_and_borrowing() {
  println!("\n=== 第三部分：引用与借用机制 ===");

  immutable_references();
  mutable_references();
  borrowing_rules_demonstration();
  dangling_references_prevention();
}

/// ### 3.1 不可变引用
///
/// 不可变引用允许读取值但不能修改。
fn immutable_references() {
  println!("\n--- 3.1 不可变引用 ---");

  println!("\n🔍 基本引用操作：");

  let s1 = String::from("hello");
  let len = calculate_length(&s1); // 传递 s1 的引用
  println!("字符串 '{}' 的长度是 {}", s1, len); // s1 仍然有效

  fn calculate_length(s: &String) -> usize {
    // s 是 String 的引用
    s.len()
  } // s 离开作用域，但因为它不拥有引用的值，所以什么也不会发生

  println!("\n🔍 多个不可变引用：");

  let s2 = String::from("world");
  let r1 = &s2; // 第一个不可变引用
  let r2 = &s2; // 第二个不可变引用
  let r3 = &s2; // 第三个不可变引用

  println!("原始值: {}", s2);
  println!("引用1: {}", r1);
  println!("引用2: {}", r2);
  println!("引用3: {}", r3);

  println!("\n🔍 引用的引用：");

  let x = 5;
  let y = &x; // y 是 x 的引用
  let z = &y; // z 是 y 的引用（引用的引用）

  println!("x = {}", x);
  println!("y = {} (引用 x)", y);
  println!("z = {} (引用 y，需要解引用)", *z);

  // 自动解引用
  println!("比较: x == *y = {}", x == *y);
  println!("比较: x == **z = {}", x == **z);

  println!("\n💡 不可变引用特点：");
  println!("• 可以有多个不可变引用");
  println!("• 不能通过引用修改值");
  println!("• 引用必须总是有效的");
  println!("• 使用 & 创建引用，* 解引用");
}

/// ### 3.2 可变引用
///
/// 可变引用允许修改借用的值。
fn mutable_references() {
  println!("\n--- 3.2 可变引用 ---");

  println!("\n🔍 基本可变引用：");

  let mut s = String::from("hello");
  change(&mut s); // 传递可变引用
  println!("修改后的字符串: {}", s);

  fn change(some_string: &mut String) {
    some_string.push_str(", world");
  }

  println!("\n🔍 可变引用的限制：");

  let mut s1 = String::from("hello");

  {
    let r1 = &mut s1; // 第一个可变引用
    // let r2 = &mut s1;  // 编译错误！不能有两个可变引用
    println!("可变引用: {}", r1);
  } // r1 离开作用域

  let r2 = &mut s1; // 现在可以创建新的可变引用
  println!("新的可变引用: {}", r2);

  println!("\n🔍 可变引用与不可变引用的冲突：");

  let mut s2 = String::from("hello");

  let r1 = &s2; // 不可变引用
  let r2 = &s2; // 另一个不可变引用
  println!("不可变引用: {}, {}", r1, r2);
  // r1 和 r2 在这里不再使用

  let r3 = &mut s2; // 可变引用
  println!("可变引用: {}", r3);

  // 演示引用作用域
  demonstrate_reference_scope();

  println!("\n💡 可变引用规则：");
  println!("• 同一时间只能有一个可变引用");
  println!("• 可变引用与不可变引用不能同时存在");
  println!("• 引用的作用域从创建开始到最后一次使用结束");
}

fn demonstrate_reference_scope() {
  println!("\n🔍 引用作用域演示：");

  let mut s = String::from("hello");

  let r1 = &s; // 不可变引用开始
  let r2 = &s; // 另一个不可变引用
  println!("不可变引用: {}, {}", r1, r2);
  // r1 和 r2 的作用域在这里结束，因为它们不再被使用

  let r3 = &mut s; // 可变引用开始
  r3.push_str(", world");
  println!("可变引用: {}", r3);
  // r3 的作用域在这里结束

  // 现在可以再次创建引用
  let r4 = &s;
  println!("新的不可变引用: {}", r4);
}

/// ### 3.3 借用规则演示
///
/// 详细演示 Rust 的借用检查器规则。
fn borrowing_rules_demonstration() {
  println!("\n--- 3.3 借用规则演示 ---");

  println!("\n📋 借用规则总结：");
  println!("1. 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用");
  println!("2. 引用必须总是有效的");

  println!("\n🔍 规则1演示 - 引用互斥：");

  let mut data = vec![1, 2, 3, 4, 5];

  // 场景1：多个不可变引用
  {
    let r1 = &data;
    let r2 = &data;
    let r3 = &data;
    println!("多个不可变引用: {:?}, {:?}, {:?}", r1, r2, r3);
  }

  // 场景2：单个可变引用
  {
    let r1 = &mut data;
    r1.push(6);
    println!("单个可变引用: {:?}", r1);
  }

  // 场景3：引用作用域不重叠
  {
    let r1 = &data;
    println!("不可变引用: {:?}", r1);
  } // r1 作用域结束

  {
    let r2 = &mut data;
    r2.push(7);
    println!("可变引用: {:?}", r2);
  } // r2 作用域结束

  println!("\n🔍 规则2演示 - 引用有效性：");

  // 正确的引用使用
  let valid_reference = {
    let s = String::from("valid");
    // return &s;  // 编译错误！返回悬垂引用
    s // 返回所有权
  };
  println!("有效的值: {}", valid_reference);

  // 函数中的借用
  demonstrate_function_borrowing();

  println!("\n💡 借用检查器的作用：");
  println!("• 在编译时防止数据竞争");
  println!("• 确保内存安全");
  println!("• 避免悬垂指针");
  println!("• 防止迭代器失效");
}

fn demonstrate_function_borrowing() {
  println!("\n🔍 函数借用演示：");

  // 借用参数的函数
  fn process_data(data: &Vec<i32>) -> i32 {
    data.iter().sum()
  }

  // 可变借用参数的函数
  fn modify_data(data: &mut Vec<i32>) {
    data.push(100);
    data.sort();
  }

  let mut numbers = vec![3, 1, 4, 1, 5, 9];

  // 不可变借用
  let sum = process_data(&numbers);
  println!("数据和: {}", sum);

  // 可变借用
  modify_data(&mut numbers);
  println!("修改后的数据: {:?}", numbers);

  // 原始数据仍然有效
  println!("原始变量仍有效: {:?}", numbers);
}

/// ### 3.4 悬垂引用防护
///
/// Rust 如何在编译时防止悬垂引用。
fn dangling_references_prevention() {
  println!("\n--- 3.4 悬垂引用防护 ---");

  println!("\n🚫 悬垂引用示例（编译错误）：");

  // 以下代码会导致编译错误
  /*
  fn dangle() -> &String {  // 返回字符串的引用
      let s = String::from("hello");  // s 是新字符串
      &s  // 返回字符串 s 的引用
  }  // s 离开作用域并被丢弃，其内存被释放
     // 引用指向的内存已被释放！
  */

  println!("上述代码无法编译，因为：");
  println!("• s 在函数结束时被释放");
  println!("• 返回的引用指向已释放的内存");
  println!("• Rust 编译器检测到悬垂引用");

  println!("\n✅ 正确的解决方案：");

  // 解决方案1：返回所有权
  fn no_dangle() -> String {
    let s = String::from("hello");
    s // 返回 s，所有权移动到调用者
  }

  let result1 = no_dangle();
  println!("返回所有权: {}", result1);

  // 解决方案2：使用生命周期参数
  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
  }

  let string1 = String::from("long string");
  let string2 = "short";
  let result2 = longest(&string1, string2);
  println!("最长的字符串: {}", result2);

  // 解决方案3：使用静态生命周期
  fn get_static_str() -> &'static str {
    "这是静态字符串" // 字符串字面量有 'static 生命周期
  }

  let result3 = get_static_str();
  println!("静态字符串: {}", result3);

  println!("\n💡 防护机制：");
  println!("• 编译时生命周期检查");
  println!("• 借用检查器验证引用有效性");
  println!("• 强制明确生命周期关系");
  println!("• 零运行时开销的安全保证");
}

/// ## 第四部分：生命周期管理
///
/// 生命周期确保引用在需要时始终有效。
pub fn lifetime_management() {
  println!("\n=== 第四部分：生命周期管理 ===");

  lifetime_basics();
  lifetime_annotations();
  lifetime_in_structs();
  static_lifetime_explanation();
}

/// ### 4.1 生命周期基础
///
/// 理解生命周期的概念和必要性。
fn lifetime_basics() {
  println!("\n--- 4.1 生命周期基础 ---");

  println!("\n🔍 生命周期概念：");
  println!("• 生命周期是引用保持有效的作用域");
  println!("• 每个引用都有一个生命周期");
  println!("• 大部分时候生命周期是隐式的");
  println!("• 当编译器无法推断时需要显式标注");

  println!("\n🔍 生命周期示例：");

  {
    let _r: &str; // 声明引用 r
    {
      let _x = 5; // x 的生命周期开始
      // r = &x;        // 编译错误！x 的生命周期比 r 短
    } // x 的生命周期结束
    // println!("{}", r); // r 引用的值已失效
  }

  // 正确的生命周期关系
  {
    let x = 5; // x 的生命周期开始
    let r = &x; // r 引用 x
    println!("r: {}", r); // 使用引用
  } // x 和 r 的生命周期都结束

  println!("\n🔍 函数中的生命周期：");

  // 简单情况：编译器可以推断
  fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
        return &s[0..i];
      }
    }
    &s[..]
  }

  let sentence = "hello world";
  let word = first_word(sentence);
  println!("第一个单词: {}", word);

  println!("\n💡 生命周期规则：");
  println!("• 引用的生命周期不能超过其引用的值");
  println!("• 函数返回的引用必须来自参数或静态值");
  println!("• 编译器使用生命周期省略规则推断");
}

/// ### 4.2 生命周期标注
///
/// 显式生命周期标注的语法和使用。
fn lifetime_annotations() {
  println!("\n--- 4.2 生命周期标注 ---");

  println!("\n🔍 生命周期标注语法：");
  println!("• 以撇号开头：'a, 'b, 'static");
  println!("• 通常使用短名称：'a, 'b, 'c");
  println!("• 放在 & 之后，类型之前：&'a str");

  println!("\n🔍 函数生命周期标注：");

  // 需要生命周期标注的函数
  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
  }

  let string1 = String::from("abcd");
  let string2 = "xyz";
  let result = longest(&string1, string2);
  println!("最长的字符串: {}", result);

  // 不同生命周期的示例
  fn different_lifetimes<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x // 只能返回 x，因为返回类型绑定到 'a
  }

  let s1 = "hello";
  let s2 = "world";
  let result2 = different_lifetimes(s1, s2);
  println!("返回第一个参数: {}", result2);

  println!("\n🔍 生命周期省略规则：");

  // 规则1：每个引用参数都有自己的生命周期
  fn rule1_example(s: &str) -> &str {
    // 等价于 fn rule1_example<'a>(s: &'a str) -> &'a str
    s
  }

  // 规则2：如果只有一个输入生命周期，它被赋予所有输出生命周期
  fn rule2_example(s: &str) -> (&str, &str) {
    // 等价于 <'a>(s: &'a str) -> (&'a str, &'a str)
    (s, s)
  }

  // 规则3：如果有 &self 或 &mut self，self 的生命周期被赋予所有输出

  let test_str = "test";
  let r1 = rule1_example(test_str);
  let (r2, r3) = rule2_example(test_str);
  println!("省略规则示例: {}, {}, {}", r1, r2, r3);

  println!("\n💡 生命周期标注原则：");
  println!("• 生命周期标注不改变引用的实际生命周期");
  println!("• 它们描述了多个引用生命周期之间的关系");
  println!("• 帮助编译器验证引用的有效性");
  println!("• 只在编译时存在，运行时无开销");
}

/// ### 4.3 结构体中的生命周期
///
/// 结构体包含引用时的生命周期处理。
fn lifetime_in_structs() {
  println!("\n--- 4.3 结构体中的生命周期 ---");

  println!("\n🔍 包含引用的结构体：");

  // 结构体中的生命周期标注
  #[derive(Debug)]
  struct ImportantExcerpt<'a> {
    part: &'a str,
  }

  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find a '.'");
  let i: ImportantExcerpt<'_> = ImportantExcerpt {
    part: first_sentence,
  };
  println!("重要摘录: {:?}", i);

  println!("\n🔍 结构体方法中的生命周期：");

  impl<'a> ImportantExcerpt<'a> {
    // 方法的生命周期省略
    fn level(&self) -> i32 {
      3
    }

    // 返回引用的方法
    fn announce_and_return_part(&self, announcement: &str) -> &str {
      println!("注意！{}", announcement);
      self.part // 返回 self.part，生命周期与 self 相同
    }

    // 多个生命周期参数
    fn compare_parts<'b>(&self, other: &'b str) -> &str {
      if self.part.len() > other.len() {
        self.part
      } else {
        // other  // 编译错误！不能返回 'b 生命周期的引用
        self.part // 只能返回 'a 生命周期的引用
      }
    }
  }

  let level = i.level();
  println!("级别: {}", level);

  let announcement = "今天是个好日子";
  let part = i.announce_and_return_part(announcement);
  println!("返回的部分: {}", part);

  let other_text = "short";
  let comparison = i.compare_parts(other_text);
  println!("比较结果: {}", comparison);

  println!("\n🔍 多个引用字段：");

  #[derive(Debug)]
  #[allow(dead_code)]
  struct TwoRefs<'a, 'b> {
    first: &'a str,
    second: &'b str,
  }

  let s1 = "first string";
  let s2 = "second string";
  let two_refs = TwoRefs {
    first: s1,
    second: s2,
  };
  println!("两个引用: {:?}", two_refs);

  println!("\n💡 结构体生命周期要点：");
  println!("• 结构体包含引用时必须标注生命周期");
  println!("• 结构体实例的生命周期不能超过其引用的数据");
  println!("• 方法中的生命周期遵循省略规则");
  println!("• 可以有多个不同的生命周期参数");
}

/// ### 4.4 静态生命周期
///
/// 'static 生命周期的特殊性质。
fn static_lifetime_explanation() {
  println!("\n--- 4.4 静态生命周期 ---");

  println!("\n🔍 'static 生命周期：");
  println!("• 'static 表示引用在整个程序运行期间都有效");
  println!("• 字符串字面量默认具有 'static 生命周期");
  println!("• 存储在程序的二进制文件中");

  // 字符串字面量
  let s: &'static str = "我有静态生命周期";
  println!("静态字符串: {}", s);

  // 静态变量
  static HELLO_WORLD: &str = "Hello, world!";
  println!("静态变量: {}", HELLO_WORLD);

  println!("\n🔍 'static 的使用场景：");

  // 函数返回静态引用
  fn get_static_string() -> &'static str {
    "这是静态字符串"
  }

  let static_str = get_static_string();
  println!("函数返回的静态字符串: {}", static_str);

  // 泛型约束中的 'static
  fn print_it<T: std::fmt::Display + 'static>(input: T) {
    println!("静态约束: {}", input);
  }

  print_it("字符串字面量");
  print_it(42);
  print_it(true);

  println!("\n🔍 'static 与生命周期参数的区别：");

  // 生命周期参数：引用的生命周期由调用者决定
  fn with_lifetime<'a>(x: &'a str) -> &'a str {
    x
  }

  // 静态生命周期：引用必须在整个程序期间有效
  fn with_static(x: &'static str) -> &'static str {
    x
  }

  let local_string = String::from("local");
  let lifetime_result = with_lifetime(&local_string); // 可以传递局部字符串的引用
  println!("生命周期参数: {}", lifetime_result);

  let static_result = with_static("static"); // 只能传递静态字符串
  println!("静态生命周期: {}", static_result);

  // with_static(&local_string);  // 编译错误！局部字符串没有静态生命周期

  println!("\n💡 'static 使用建议：");
  println!("• 不要过度使用 'static 约束");
  println!("• 优先考虑生命周期参数");
  println!("• 'static 主要用于全局数据和字符串字面量");
  println!("• 在泛型约束中谨慎使用");
}

/// ## 第五部分：闭包详细解释
///
/// 闭包是可以捕获其环境的匿名函数，与所有权系统紧密相关。
pub fn closure_comprehensive_explanation() {
  println!("\n=== 第五部分：闭包详细解释 ===");

  closure_basics_and_syntax();
  closure_capture_modes();
  closure_traits_explanation();
  closure_ownership_interaction();
  closure_practical_examples();
}

/// ### 5.1 闭包基础与语法
///
/// 闭包的定义、语法和基本使用。
fn closure_basics_and_syntax() {
  println!("\n--- 5.1 闭包基础与语法 ---");

  println!("\n🔍 什么是闭包：");
  println!("• 闭包是可以捕获其环境的匿名函数");
  println!("• 可以保存在变量中或作为参数传递");
  println!("• 可以在一个地方创建，在另一个地方调用");
  println!("• 可以从定义它们的作用域中捕获值");

  println!("\n🔍 闭包语法：");

  // 基本闭包语法
  let simple_closure = |x| x + 1;
  println!("简单闭包: 5 + 1 = {}", simple_closure(5));

  // 带类型标注的闭包
  let typed_closure = |x: i32| -> i32 { x * 2 };
  println!("类型标注闭包: 5 * 2 = {}", typed_closure(5));

  // 多参数闭包
  let multi_param = |x, y| x + y;
  println!("多参数闭包: 3 + 4 = {}", multi_param(3, 4));

  // 无参数闭包
  let no_param = || {
    println!("无参数闭包被调用");
    42
  };
  println!("无参数闭包返回: {}", no_param());

  println!("\n🔍 闭包 vs 函数：");

  // 函数定义
  fn function_add(x: i32, y: i32) -> i32 {
    x + y
  }

  // 等价的闭包
  let closure_add = |x: i32, y: i32| -> i32 { x + y };

  println!("函数调用: {}", function_add(2, 3));
  println!("闭包调用: {}", closure_add(2, 3));

  println!("\n🔍 闭包类型推断：");

  // 编译器可以推断闭包的类型
  let inferred_closure = |x| x + 1;
  let result1 = inferred_closure(5); // 推断为 i32
  // let result2 = inferred_closure(5.0); // 编译错误！类型已确定
  println!("类型推断结果: {}", result1);

  // 显式类型标注
  let explicit_closure: fn(i32) -> i32 = |x| x + 1;
  println!("显式类型闭包: {}", explicit_closure(10));

  println!("\n💡 闭包特点：");
  println!("• 语法简洁，使用 |参数| 表达式");
  println!("• 可以捕获环境中的变量");
  println!("• 类型可以被推断");
  println!("• 每个闭包都有唯一的类型");
}

/// ### 5.2 闭包捕获模式
///
/// 闭包如何捕获环境中的变量。
fn closure_capture_modes() {
  println!("\n--- 5.2 闭包捕获模式 ---");

  println!("\n🔍 三种捕获模式：");
  println!("1. 不可变借用 (Fn)");
  println!("2. 可变借用 (FnMut)");
  println!("3. 获取所有权 (FnOnce)");

  println!("\n🔍 不可变借用捕获：");

  let x = 4;
  let equal_to_x = |z| z == x; // 闭包不可变借用 x

  println!("x = {}", x); // x 仍然可用
  let y = 4;
  println!("equal_to_x(y) = {}", equal_to_x(y));
  println!("x 仍然可用: {}", x);

  println!("\n🔍 可变借用捕获：");

  let mut list = vec![1, 2, 3];
  println!("调用闭包前: {:?}", list);

  let mut borrows_mutably = || list.push(7); // 闭包可变借用 list
  // println!("{:?}", list);  // 编译错误！list 被可变借用

  borrows_mutably();
  println!("调用闭包后: {:?}", list);

  println!("\n🔍 获取所有权捕获：");

  let list2 = vec![1, 2, 3];
  println!("move 前: {:?}", list2);

  let takes_ownership = move || {
    println!("闭包内部: {:?}", list2);
    list2 // 返回 list2，转移所有权
  };

  // println!("{:?}", list2);  // 编译错误！list2 已被移动
  let moved_list = takes_ownership();
  println!("move 后: {:?}", moved_list);

  println!("\n🔍 强制移动捕获：");

  let x = vec![1, 2, 3];
  let y = vec![4, 5, 6];

  // 使用 move 关键字强制移动
  let move_closure = move || {
    println!("x: {:?}", x);
    println!("y: {:?}", y);
  };

  // println!("{:?}", x);  // 编译错误！x 已被移动
  // println!("{:?}", y);  // 编译错误！y 已被移动

  move_closure();

  println!("\n🔍 捕获模式选择：");

  let data = vec![1, 2, 3, 4, 5];

  // 编译器自动选择最小权限的捕获模式
  let read_only = || {
    println!("只读访问: {:?}", data); // 不可变借用
  };

  read_only();

  println!("\n💡 捕获模式总结：");
  println!("• 编译器自动选择最小权限的捕获模式");
  println!("• move 关键字强制获取所有权");
  println!("• 捕获模式影响闭包的 trait 实现");
  println!("• 理解捕获模式有助于避免所有权问题");
}

/// ### 5.3 闭包 Trait 详解
///
/// Fn、FnMut、FnOnce 三个 trait 的详细解释。
fn closure_traits_explanation() {
  println!("\n--- 5.3 闭包 Trait 详解 ---");

  println!("\n🔍 三个闭包 Trait：");
  println!("• FnOnce: 只能调用一次的闭包");
  println!("• FnMut: 可以多次调用，可以修改捕获的变量");
  println!("• Fn: 可以多次调用，只能不可变访问捕获的变量");

  println!("\n🔍 FnOnce 示例：");

  let consume_closure = || {
    let data = vec![1, 2, 3];
    data // 返回 data，消费它
  };

  let result = consume_closure(); // 第一次调用
  println!("FnOnce 结果: {:?}", result);
  // consume_closure();  // 编译错误！不能再次调用

  println!("\n🔍 FnMut 示例：");

  let mut counter = 0;
  let mut increment = || {
    counter += 1;
    counter
  };

  println!("第一次调用: {}", increment());
  println!("第二次调用: {}", increment());
  println!("第三次调用: {}", increment());

  println!("\n🔍 Fn 示例：");

  let multiplier = 2;
  let multiply = |x| x * multiplier;

  println!("Fn 调用1: {}", multiply(5));
  println!("Fn 调用2: {}", multiply(10));
  println!("Fn 调用3: {}", multiply(15));

  println!("\n🔍 Trait 层次关系：");
  println!("• Fn: FnMut + FnOnce (最严格)");
  println!("• FnMut: FnOnce (中等)");
  println!("• FnOnce (最宽松)");

  // 演示 trait 约束
  demonstrate_closure_traits();
}

fn demonstrate_closure_traits() {
  println!("\n🔍 闭包 Trait 约束演示：");

  // 接受 FnOnce 的函数
  fn call_once<F>(f: F) -> i32
  where
    F: FnOnce() -> i32,
  {
    f()
  }

  // 接受 FnMut 的函数
  fn call_mut<F>(mut f: F) -> i32
  where
    F: FnMut() -> i32,
  {
    f() + f() // 调用两次
  }

  // 接受 Fn 的函数
  fn call_fn<F>(f: F) -> i32
  where
    F: Fn() -> i32,
  {
    f() + f() + f() // 调用三次
  }

  let value = 10;

  // Fn 闭包可以传递给所有函数
  let fn_closure = || value;
  println!("FnOnce 调用: {}", call_once(fn_closure));

  let fn_closure2 = || value;
  println!("FnMut 调用: {}", call_mut(fn_closure2));

  let fn_closure3 = || value;
  println!("Fn 调用: {}", call_fn(fn_closure3));

  // FnMut 闭包
  let mut counter = 0;
  let mut fnmut_closure = || {
    counter += 1;
    counter
  };

  // fnmut_closure 只能传递给 FnOnce 和 FnMut
  // println!("Fn 调用: {}", call_fn(fnmut_closure));  // 编译错误！
  println!("FnMut 调用: {}", call_mut(&mut fnmut_closure));
}

/// ### 5.4 闭包与所有权交互
///
/// 闭包如何与 Rust 的所有权系统交互。
fn closure_ownership_interaction() {
  println!("\n--- 5.4 闭包与所有权交互 ---");

  println!("\n🔍 闭包中的所有权转移：");

  let data = vec![1, 2, 3, 4, 5];

  // 闭包获取所有权
  let take_ownership = move || {
    println!("闭包拥有数据: {:?}", data);
    data.len() // 返回长度
  };

  // data 已被移动，不能再使用
  // println!("{:?}", data);  // 编译错误！

  let length = take_ownership();
  println!("数据长度: {}", length);

  println!("\n🔍 闭包返回引用：");

  let text = String::from("hello world");

  // 返回引用的闭包
  let get_reference = || -> &str {
    &text // 返回 text 的引用
  };

  let reference = get_reference();
  println!("引用内容: {}", reference);
  println!("原始数据仍可用: {}", text);

  println!("\n🔍 闭包与生命周期：");

  // 生命周期约束的闭包
  fn create_closure<'a>(s: &'a str) -> impl Fn() -> &'a str {
    move || s
  }

  let string = "static string";
  let closure = create_closure(string);
  println!("闭包返回: {}", closure());

  println!("\n🔍 闭包作为返回值：");

  // 返回闭包的函数
  fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
  }

  let add_5 = make_adder(5);
  println!("add_5(10) = {}", add_5(10));
  println!("add_5(20) = {}", add_5(20));

  // 返回不同类型闭包的函数
  fn make_closure(condition: bool) -> Box<dyn Fn(i32) -> i32> {
    if condition {
      Box::new(|x| x * 2)
    } else {
      Box::new(|x| x + 1)
    }
  }

  let closure1 = make_closure(true);
  let closure2 = make_closure(false);

  println!("条件闭包1: {}", closure1(5));
  println!("条件闭包2: {}", closure2(5));

  println!("\n💡 所有权交互要点：");
  println!("• move 关键字强制闭包获取所有权");
  println!("• 闭包可以返回引用，但需要满足生命周期约束");
  println!("• impl Trait 语法简化闭包返回类型");
  println!("• Box<dyn Trait> 用于返回不同类型的闭包");
}

/// ### 5.5 闭包实际应用示例
///
/// 闭包在实际编程中的常见应用场景。
fn closure_practical_examples() {
  println!("\n--- 5.5 闭包实际应用示例 ---");

  println!("\n🔍 迭代器与闭包：");

  let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

  // 过滤偶数
  let evens: Vec<i32> = numbers.iter().filter(|&x| x % 2 == 0).cloned().collect();
  println!("偶数: {:?}", evens);

  // 映射操作
  let squares: Vec<i32> = numbers.iter().map(|x| x * x).collect();
  println!("平方: {:?}", squares);

  // 链式操作
  let result: Vec<i32> = numbers.iter().filter(|&x| *x > 5).map(|x| x * 2).collect();
  println!("大于5的数乘以2: {:?}", result);

  println!("\n🔍 错误处理与闭包：");

  let strings = vec!["1", "2", "not_a_number", "4", "5"];

  // 使用 filter_map 处理错误
  let parsed_numbers: Vec<i32> = strings.iter().filter_map(|s| s.parse().ok()).collect();
  println!("解析成功的数字: {:?}", parsed_numbers);

  // 使用 map 和 unwrap_or
  let with_defaults: Vec<i32> = strings.iter().map(|s| s.parse().unwrap_or(0)).collect();
  println!("带默认值的解析: {:?}", with_defaults);

  println!("\n🔍 自定义迭代器适配器：");

  // 创建自定义的迭代器适配器
  trait IteratorExt: Iterator {
    fn my_filter<P>(self, predicate: P) -> std::iter::Filter<Self, P>
    where
      Self: Sized,
      P: FnMut(&Self::Item) -> bool,
    {
      self.filter(predicate)
    }
  }

  impl<I: Iterator> IteratorExt for I {}

  let custom_result: Vec<i32> = (1..=10).my_filter(|&x| x % 3 == 0).collect();
  println!("自定义过滤器结果: {:?}", custom_result);

  println!("\n🔍 闭包作为配置：");

  // 使用闭包配置行为
  struct Processor<F>
  where
    F: Fn(i32) -> i32,
  {
    transform: F,
  }

  impl<F> Processor<F>
  where
    F: Fn(i32) -> i32,
  {
    fn new(transform: F) -> Self {
      Processor { transform }
    }

    fn process(&self, data: Vec<i32>) -> Vec<i32> {
      data.into_iter().map(&self.transform).collect()
    }
  }

  let doubler = Processor::new(|x| x * 2);
  let squared = Processor::new(|x| x * x);

  let data = vec![1, 2, 3, 4, 5];
  println!("加倍处理: {:?}", doubler.process(data.clone()));
  println!("平方处理: {:?}", squared.process(data));

  println!("\n💡 实际应用总结：");
  println!("• 迭代器方法大量使用闭包");
  println!("• 闭包简化错误处理逻辑");
  println!("• 可以创建灵活的配置系统");
  println!("• 函数式编程风格的核心工具");
}

/// ## 第六部分：智能指针与所有权
///
/// 智能指针提供了额外的功能和保证，扩展了所有权系统。
pub fn smart_pointers_and_ownership() {
  println!("\n=== 第六部分：智能指针与所有权 ===");

  box_pointer_explanation();
  rc_pointer_explanation();
  refcell_interior_mutability();
  smart_pointer_combinations();
}

/// ### 6.1 Box<T> 智能指针
///
/// Box 提供堆上数据的所有权。
fn box_pointer_explanation() {
  println!("\n--- 6.1 Box<T> 智能指针 ---");

  println!("\n🔍 Box 的基本用法：");

  // 在堆上存储数据
  let b = Box::new(5);
  println!("Box 中的值: {}", b);

  // Box 实现了 Deref，可以像引用一样使用
  let x = 5;
  let y = Box::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y); // 解引用 Box
  println!("Box 解引用: x={}, *y={}", x, *y);

  println!("\n🔍 递归类型与 Box：");

  // 使用 Box 创建递归类型
  #[derive(Debug)]
  #[allow(dead_code)]
  enum List {
    Cons(i32, Box<List>),
    Nil,
  }

  use List::{Cons, Nil};

  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  println!("递归列表: {:?}", list);

  println!("\n🔍 Box 的所有权转移：");

  let boxed_value = Box::new(String::from("boxed string"));
  let moved_box = boxed_value; // Box 的所有权转移
  // println!("{}", boxed_value);  // 编译错误！
  println!("移动后的 Box: {}", moved_box);

  println!("\n💡 Box 特点：");
  println!("• 在堆上分配数据");
  println!("• 编译时已知大小");
  println!("• 实现 Deref 和 Drop trait");
  println!("• 用于递归类型和大型数据");
}

/// ### 6.2 Rc<T> 引用计数指针
///
/// Rc 允许多个所有者共享数据。
fn rc_pointer_explanation() {
  println!("\n--- 6.2 Rc<T> 引用计数指针 ---");

  println!("\n🔍 Rc 的基本用法：");

  let data = Rc::new(String::from("shared data"));
  println!("创建 Rc，引用计数: {}", Rc::strong_count(&data));

  let data2 = Rc::clone(&data); // 增加引用计数
  println!("克隆后，引用计数: {}", Rc::strong_count(&data));

  {
    let data3 = Rc::clone(&data); // 再次增加引用计数
    println!("内层作用域，引用计数: {}", Rc::strong_count(&data));
    println!("所有引用的值: {}, {}, {}", data, data2, data3);
  } // data3 离开作用域，引用计数减少

  println!("离开内层作用域，引用计数: {}", Rc::strong_count(&data));

  println!("\n🔍 Rc 与链表：");

  #[derive(Debug)]
  #[allow(dead_code)]
  enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
  }

  use RcList::{Cons as RcCons, Nil as RcNil};

  let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
  println!("a 的引用计数: {}", Rc::strong_count(&a));

  let b = RcCons(3, Rc::clone(&a));
  println!("创建 b 后，a 的引用计数: {}", Rc::strong_count(&a));

  let c = RcCons(4, Rc::clone(&a));
  println!("创建 c 后，a 的引用计数: {}", Rc::strong_count(&a));

  println!("列表 a: {:?}", a);
  println!("列表 b: {:?}", b);
  println!("列表 c: {:?}", c);

  println!("\n💡 Rc 特点：");
  println!("• 允许多个所有者");
  println!("• 只能用于单线程");
  println!("• 数据是不可变的");
  println!("• 运行时引用计数");
}

/// ### 6.3 RefCell<T> 内部可变性
///
/// RefCell 提供运行时借用检查的内部可变性。
fn refcell_interior_mutability() {
  println!("\n--- 6.3 RefCell<T> 内部可变性 ---");

  println!("\n🔍 RefCell 基本用法：");

  let data = RefCell::new(5);

  // 不可变借用
  {
    let borrowed = data.borrow();
    println!("不可变借用: {}", *borrowed);
  } // 借用在这里结束

  // 可变借用
  {
    let mut borrowed_mut = data.borrow_mut();
    *borrowed_mut += 10;
    println!("可变借用修改后: {}", *borrowed_mut);
  } // 可变借用在这里结束

  println!("最终值: {}", data.borrow());

  println!("\n🔍 RefCell 与 Rc 结合：");

  let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));
  let data1 = Rc::clone(&shared_data);
  let data2 = Rc::clone(&shared_data);

  // 通过第一个引用修改数据
  data1.borrow_mut().push(4);
  println!("通过 data1 修改后: {:?}", data1.borrow());

  // 通过第二个引用修改数据
  data2.borrow_mut().push(5);
  println!("通过 data2 修改后: {:?}", data2.borrow());

  // 原始引用也能看到变化
  println!("原始 shared_data: {:?}", shared_data.borrow());

  println!("\n🔍 运行时借用检查：");

  let cell = RefCell::new(String::from("hello"));

  // 正确的借用模式
  {
    let borrow1 = cell.borrow();
    let borrow2 = cell.borrow(); // 多个不可变借用是允许的
    println!("多个不可变借用: {}, {}", borrow1, borrow2);
  }

  {
    let mut borrow_mut = cell.borrow_mut();
    borrow_mut.push_str(", world");
    println!("可变借用: {}", borrow_mut);
    // let borrow = cell.borrow();  // 运行时 panic！不能同时有可变和不可变借用
  }

  println!("\n💡 RefCell 特点：");
  println!("• 运行时借用检查");
  println!("• 内部可变性模式");
  println!("• 违反借用规则会导致 panic");
  println!("• 常与 Rc 结合使用");
}

/// ### 6.4 智能指针组合
///
/// 不同智能指针的组合使用模式。
fn smart_pointer_combinations() {
  println!("\n--- 6.4 智能指针组合 ---");

  println!("\n🔍 Rc<RefCell<T>> 模式：");

  #[derive(Debug)]
  #[allow(dead_code)]
  struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
  }

  let leaf = Rc::new(Node {
    value: 3,
    children: RefCell::new(vec![]),
  });

  let branch = Rc::new(Node {
    value: 5,
    children: RefCell::new(vec![Rc::clone(&leaf)]),
  });

  println!("叶子节点: {:?}", leaf);
  println!("分支节点: {:?}", branch);

  // 修改子节点列表
  leaf.children.borrow_mut().push(Rc::clone(&branch));
  println!("修改后的叶子节点引用计数: {}", Rc::strong_count(&leaf));

  println!("\n🔍 Box<dyn Trait> 模式：");

  trait Draw {
    fn draw(&self);
  }

  struct Circle {
    radius: f64,
  }

  struct Rectangle {
    width: f64,
    height: f64,
  }

  impl Draw for Circle {
    fn draw(&self) {
      println!("绘制圆形，半径: {}", self.radius);
    }
  }

  impl Draw for Rectangle {
    fn draw(&self) {
      println!("绘制矩形，宽: {}, 高: {}", self.width, self.height);
    }
  }

  let shapes: Vec<Box<dyn Draw>> = vec![
    Box::new(Circle { radius: 5.0 }),
    Box::new(Rectangle {
      width: 10.0,
      height: 20.0,
    }),
  ];

  for shape in shapes {
    shape.draw();
  }

  println!("\n💡 组合模式总结：");
  println!("• Rc<RefCell<T>>: 多所有者 + 内部可变性");
  println!("• Box<dyn Trait>: 堆分配 + trait 对象");
  println!("• 选择合适的智能指针组合");
  println!("• 注意循环引用问题");
}

/// ## 第七部分：实际应用案例
///
/// 所有权系统在实际项目中的应用示例。
pub fn practical_ownership_examples() {
  println!("\n=== 第七部分：实际应用案例 ===");

  data_structure_examples();
  async_programming_ownership();
  error_handling_patterns();
  performance_optimization_cases();
}

/// ### 7.1 数据结构实现
///
/// 使用所有权系统实现常见数据结构。
fn data_structure_examples() {
  println!("\n--- 7.1 数据结构实现 ---");

  println!("\n🔍 链表实现：");

  #[derive(Debug)]
  struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
  }

  #[derive(Debug)]
  struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
  }

  impl<T> LinkedList<T> {
    fn new() -> Self {
      LinkedList { head: None }
    }

    fn push(&mut self, data: T) {
      let new_node = Box::new(Node {
        data,
        next: self.head.take(),
      });
      self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
      self.head.take().map(|node| {
        self.head = node.next;
        node.data
      })
    }
  }

  let mut list = LinkedList::new();
  list.push(1);
  list.push(2);
  list.push(3);

  println!("链表: {:?}", list);
  println!("弹出: {:?}", list.pop());
  println!("弹出后: {:?}", list);

  println!("\n🔍 二叉树实现：");

  #[derive(Debug)]
  struct BinaryTree<T> {
    root: Option<Box<TreeNode<T>>>,
  }

  #[derive(Debug)]
  struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
  }

  impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
      BinaryTree { root: None }
    }

    fn insert(&mut self, value: T) {
      self.root = Self::insert_node(self.root.take(), value);
    }

    fn insert_node(node: Option<Box<TreeNode<T>>>, value: T) -> Option<Box<TreeNode<T>>> {
      match node {
        None => Some(Box::new(TreeNode {
          value,
          left: None,
          right: None,
        })),
        Some(mut node) => {
          if value <= node.value {
            node.left = Self::insert_node(node.left.take(), value);
          } else {
            node.right = Self::insert_node(node.right.take(), value);
          }
          Some(node)
        }
      }
    }
  }

  let mut tree = BinaryTree::new();
  tree.insert(5);
  tree.insert(3);
  tree.insert(7);
  tree.insert(1);
  tree.insert(9);

  println!("二叉树: {:?}", tree);

  println!("\n💡 数据结构要点：");
  println!("• Box 用于递归数据结构");
  println!("• Option 表示可能为空的节点");
  println!("• take() 方法转移所有权");
  println!("• 所有权确保内存安全");
}

/// ### 7.2 异步编程中的所有权
///
/// 异步编程场景下的所有权处理。
fn async_programming_ownership() {
  println!("\n--- 7.2 异步编程中的所有权 ---");

  println!("\n🔍 数据共享模式：");

  // 模拟异步任务的数据共享
  let shared_counter = Rc::new(RefCell::new(0));

  // 创建多个"任务"（这里用闭包模拟）
  let tasks: Vec<Box<dyn Fn()>> = (0..3)
    .map(|i| {
      let counter = Rc::clone(&shared_counter);
      Box::new(move || {
        let mut count = counter.borrow_mut();
        *count += 1;
        println!("任务 {} 执行，计数器: {}", i, *count);
      }) as Box<dyn Fn()>
    })
    .collect();

  // 执行所有任务
  for task in tasks {
    task();
  }

  println!("最终计数器值: {}", shared_counter.borrow());

  println!("\n🔍 消息传递模式：");

  // 模拟消息传递
  #[derive(Debug, Clone)]
  #[allow(dead_code)]
  struct Message {
    id: u32,
    content: String,
  }

  let messages = vec![
    Message {
      id: 1,
      content: "Hello".to_string(),
    },
    Message {
      id: 2,
      content: "World".to_string(),
    },
    Message {
      id: 3,
      content: "Rust".to_string(),
    },
  ];

  // 处理消息的闭包
  let process_message = |msg: Message| {
    println!("处理消息 {}: {}", msg.id, msg.content);
    msg // 返回消息的所有权
  };

  let processed_messages: Vec<Message> = messages.into_iter().map(process_message).collect();

  println!("处理后的消息: {:?}", processed_messages);

  println!("\n💡 异步编程要点：");
  println!("• Rc<RefCell<T>> 用于单线程异步数据共享");
  println!("• Arc<Mutex<T>> 用于多线程异步数据共享");
  println!("• 消息传递避免共享状态");
  println!("• 所有权转移确保数据安全");
}

/// ### 7.3 错误处理模式
///
/// 所有权系统在错误处理中的应用。
fn error_handling_patterns() {
  println!("\n--- 7.3 错误处理模式 ---");

  println!("\n🔍 Result 类型与所有权：");

  // 自定义错误类型
  #[derive(Debug)]
  enum ParseError {
    InvalidFormat,
    OutOfRange,
  }

  // 解析函数，返回 Result
  fn parse_number(s: String) -> Result<i32, ParseError> {
    if s.is_empty() {
      return Err(ParseError::InvalidFormat);
    }

    match s.parse::<i32>() {
      Ok(n) if n >= 0 && n <= 100 => Ok(n),
      Ok(_) => Err(ParseError::OutOfRange),
      Err(_) => Err(ParseError::InvalidFormat),
    }
  }

  let inputs = vec![
    "42".to_string(),
    "150".to_string(),
    "abc".to_string(),
    "".to_string(),
  ];

  for input in inputs {
    match parse_number(input.clone()) {
      Ok(n) => println!("解析成功: {} -> {}", input, n),
      Err(e) => println!("解析失败: {} -> {:?}", input, e),
    }
  }

  println!("\n🔍 错误传播与所有权：");

  // 链式错误处理
  fn process_data(data: Vec<String>) -> Result<Vec<i32>, ParseError> {
    data.into_iter().map(parse_number).collect()
  }

  let test_data = vec!["1".to_string(), "2".to_string(), "3".to_string()];
  match process_data(test_data) {
    Ok(numbers) => println!("处理成功: {:?}", numbers),
    Err(e) => println!("处理失败: {:?}", e),
  }

  println!("\n💡 错误处理要点：");
  println!("• Result 类型转移错误和成功值的所有权");
  println!("• ? 操作符简化错误传播");
  println!("• 错误类型设计要考虑所有权");
  println!("• 避免不必要的克隆");
}

/// ### 7.4 性能优化案例
///
/// 利用所有权系统进行性能优化。
fn performance_optimization_cases() {
  println!("\n--- 7.4 性能优化案例 ---");

  println!("\n🔍 零拷贝字符串处理：");

  // 避免不必要的字符串分配
  fn process_lines(text: &str) -> Vec<&str> {
    text
      .lines()
      .filter(|line| !line.is_empty())
      .filter(|line| !line.starts_with('#'))
      .collect()
  }

  let input = "line1\n# comment\nline2\n\nline3";
  let processed = process_lines(input);
  println!("处理后的行: {:?}", processed);

  println!("\n🔍 内存池模式：");

  // 简单的对象池
  struct ObjectPool<T> {
    objects: RefCell<Vec<T>>,
    factory: Box<dyn Fn() -> T>,
  }

  impl<T> ObjectPool<T> {
    fn new<F>(factory: F) -> Self
    where
      F: Fn() -> T + 'static,
    {
      ObjectPool {
        objects: RefCell::new(Vec::new()),
        factory: Box::new(factory),
      }
    }

    fn get(&self) -> T {
      self
        .objects
        .borrow_mut()
        .pop()
        .unwrap_or_else(|| (self.factory)())
    }

    fn return_object(&self, obj: T) {
      self.objects.borrow_mut().push(obj);
    }
  }

  let pool = ObjectPool::new(|| Vec::<i32>::with_capacity(100));

  // 使用对象池
  let mut vec1 = pool.get();
  vec1.extend(1..=10);
  println!("使用向量: {:?}", vec1);

  vec1.clear();
  pool.return_object(vec1);

  let vec2 = pool.get(); // 重用之前的向量
  println!("重用向量容量: {}", vec2.capacity());

  println!("\n💡 性能优化要点：");
  println!("• 使用引用避免不必要的拷贝");
  println!("• 对象池减少内存分配");
  println!("• 所有权转移比引用计数更高效");
  println!("• 编译时优化胜过运行时检查");
}

/// ## 第八部分：常见错误与最佳实践
///
/// 总结所有权系统的常见陷阱和最佳实践。
pub fn common_mistakes_and_best_practices() {
  println!("\n=== 第八部分：常见错误与最佳实践 ===");

  common_ownership_mistakes();
  borrowing_best_practices();
  lifetime_guidelines();
  performance_best_practices();
}

/// ### 8.1 常见所有权错误
///
/// 新手常犯的所有权相关错误。
fn common_ownership_mistakes() {
  println!("\n--- 8.1 常见所有权错误 ---");

  println!("\n🚫 错误1：使用已移动的值");
  println!("// 错误代码示例：");
  println!("// let s1 = String::from(\"hello\");");
  println!("// let s2 = s1;  // s1 被移动");
  println!("// println!(\"{{}}\", s1);  // 编译错误！");

  println!("\n✅ 正确做法：");
  let s1 = String::from("hello");
  let s2 = s1.clone(); // 克隆而不是移动
  println!("s1: {}, s2: {}", s1, s2);

  println!("\n🚫 错误2：悬垂引用");
  println!("// 错误代码示例：");
  println!("// fn dangle() -> &String {{");
  println!("//     let s = String::from(\"hello\");");
  println!("//     &s  // 返回局部变量的引用");
  println!("// }}  // s 被释放，引用变为悬垂");

  println!("\n✅ 正确做法：");
  fn no_dangle() -> String {
    let s = String::from("hello");
    s // 返回所有权
  }
  let result = no_dangle();
  println!("正确返回: {}", result);

  println!("\n🚫 错误3：借用检查冲突");
  println!("// 错误代码示例：");
  println!("// let mut v = vec![1, 2, 3];");
  println!("// let r = &v[0];  // 不可变借用");
  println!("// v.push(4);      // 可变借用，冲突！");
  println!("// println!(\"{{}}\", r);");

  println!("\n✅ 正确做法：");
  let mut v = vec![1, 2, 3];
  {
    let r = &v[0]; // 不可变借用
    println!("第一个元素: {}", r);
  } // 借用结束
  v.push(4); // 现在可以可变借用
  println!("修改后的向量: {:?}", v);

  println!("\n🚫 错误4：过度克隆");
  println!("// 低效代码示例：");
  println!("// fn process_string(s: String) -> String {{");
  println!("//     s.to_uppercase()");
  println!("// }}");
  println!("// let original = String::from(\"hello\");");
  println!("// let result = process_string(original.clone());  // 不必要的克隆");

  println!("\n✅ 正确做法：");
  fn process_string_ref(s: &str) -> String {
    s.to_uppercase()
  }
  let original = String::from("hello");
  let result = process_string_ref(&original); // 使用引用
  println!("原始: {}, 处理后: {}", original, result);

  println!("\n💡 避免错误的要点：");
  println!("• 理解移动语义和借用规则");
  println!("• 优先使用引用而不是克隆");
  println!("• 注意引用的生命周期");
  println!("• 使用编译器错误信息学习");
}

/// ### 8.2 借用最佳实践
///
/// 高效使用借用机制的指导原则。
fn borrowing_best_practices() {
  println!("\n--- 8.2 借用最佳实践 ---");

  println!("\n✅ 实践1：优先使用不可变引用");

  fn analyze_data(data: &[i32]) -> (i32, i32, f64) {
    let sum: i32 = data.iter().sum();
    let max = *data.iter().max().unwrap_or(&0);
    let avg = sum as f64 / data.len() as f64;
    (sum, max, avg)
  }

  let numbers = vec![1, 5, 3, 9, 2, 7];
  let (sum, max, avg) = analyze_data(&numbers);
  println!("分析结果 - 和: {}, 最大: {}, 平均: {:.2}", sum, max, avg);
  println!("原始数据仍可用: {:?}", numbers);

  println!("\n✅ 实践2：最小化可变借用的作用域");

  let mut data = vec![1, 2, 3, 4, 5];

  // 将可变借用限制在最小作用域内
  {
    let last = data.last_mut().unwrap();
    *last *= 10;
  } // 可变借用结束

  // 现在可以进行其他操作
  let sum: i32 = data.iter().sum();
  println!("修改后的数据: {:?}, 和: {}", data, sum);

  println!("\n✅ 实践3：使用方法链避免中间变量");

  let text = "hello world rust programming";
  let result: Vec<String> = text
    .split_whitespace()
    .filter(|word| word.len() > 4)
    .map(|word| word.to_uppercase())
    .collect();

  println!("处理结果: {:?}", result);

  println!("\n✅ 实践4：合理使用 as_ref() 和 as_mut()");

  let mut option_string = Some(String::from("hello"));

  // 使用 as_ref() 避免移动 Option 中的值
  if let Some(ref s) = option_string {
    println!("字符串长度: {}", s.len());
  }

  // 使用 as_mut() 修改 Option 中的值
  if let Some(ref mut s) = option_string {
    s.push_str(", world");
  }

  println!("修改后: {:?}", option_string);

  println!("\n💡 借用最佳实践总结：");
  println!("• 默认使用不可变引用");
  println!("• 最小化可变借用作用域");
  println!("• 利用方法链减少中间变量");
  println!("• 合理使用 Option 的引用方法");
}

/// ### 8.3 生命周期指导原则
///
/// 生命周期标注和管理的最佳实践。
fn lifetime_guidelines() {
  println!("\n--- 8.3 生命周期指导原则 ---");

  println!("\n✅ 原则1：尽量避免显式生命周期标注");

  // 编译器可以推断的情况
  fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
  }

  let sentence = "hello world";
  let word = first_word(sentence);
  println!("第一个单词: {}", word);

  println!("\n✅ 原则2：必要时使用描述性的生命周期名称");

  // 使用有意义的生命周期名称
  fn find_longest_line<'text>(text: &'text str) -> Option<&'text str> {
    text.lines().max_by_key(|line| line.len())
  }

  let multiline = "short\nthis is a longer line\nshort again";
  if let Some(longest) = find_longest_line(multiline) {
    println!("最长的行: {}", longest);
  }

  println!("\n✅ 原则3：结构体生命周期要谨慎设计");

  #[derive(Debug)]
  struct TextAnalyzer<'a> {
    text: &'a str,
    word_count: usize,
  }

  impl<'a> TextAnalyzer<'a> {
    fn new(text: &'a str) -> Self {
      let word_count = text.split_whitespace().count();
      TextAnalyzer { text, word_count }
    }

    fn get_summary(&self) -> String {
      format!("文本长度: {}, 单词数: {}", self.text.len(), self.word_count)
    }
  }

  let text = "Rust is a systems programming language";
  let analyzer = TextAnalyzer::new(text);
  println!("分析器: {:?}", analyzer);
  println!("摘要: {}", analyzer.get_summary());

  println!("\n✅ 原则4：使用 'static 要谨慎");

  // 只在真正需要静态生命周期时使用
  const GREETING: &'static str = "Hello, Rust!";

  fn get_greeting() -> &'static str {
    GREETING
  }

  println!("静态问候: {}", get_greeting());

  println!("\n💡 生命周期指导原则：");
  println!("• 让编译器尽可能推断生命周期");
  println!("• 使用描述性的生命周期参数名");
  println!("• 结构体生命周期设计要简洁");
  println!("• 谨慎使用 'static 生命周期");
}

/// ### 8.4 性能最佳实践
///
/// 利用所有权系统优化性能的技巧。
fn performance_best_practices() {
  println!("\n--- 8.4 性能最佳实践 ---");

  println!("\n✅ 技巧1：避免不必要的分配");

  // 使用 Cow (Clone on Write) 优化
  use std::borrow::Cow;

  // 使用生命周期省略规则，编译器会自动推断
  fn process_text(input: &str) -> Cow<'_, str> {
    if input.contains("bad_word") {
      Cow::Owned(input.replace("bad_word", "***"))
    } else {
      Cow::Borrowed(input)
    }
  }

  let clean_text = "This is clean text";
  let dirty_text = "This contains bad_word";

  println!("处理干净文本: {:?}", process_text(clean_text));
  println!("处理脏文本: {:?}", process_text(dirty_text));

  println!("\n✅ 技巧2：使用迭代器而不是索引");

  let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

  // 高效的迭代器链
  let sum: i32 = numbers
    .iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * x)
    .sum();

  println!("偶数平方和: {}", sum);

  println!("\n✅ 技巧3：合理使用 Vec 容量");

  // 预分配容量避免重复分配
  let mut efficient_vec = Vec::with_capacity(1000);
  for i in 0..1000 {
    efficient_vec.push(i);
  }
  println!(
    "高效向量长度: {}, 容量: {}",
    efficient_vec.len(),
    efficient_vec.capacity()
  );

  println!("\n✅ 技巧4：使用 Box 处理大型数据");

  // 大型结构体使用 Box 避免栈溢出
  #[derive(Debug)]
  #[allow(dead_code)]
  struct LargeData {
    data: [u8; 1024], // 1KB 数据
    id: u32,
  }

  let large_data = Box::new(LargeData {
    data: [0; 1024],
    id: 42,
  });

  println!("大型数据 ID: {}", large_data.id);

  println!("\n💡 性能最佳实践总结：");
  println!("• 使用 Cow 避免不必要的克隆");
  println!("• 优先使用迭代器而不是索引访问");
  println!("• 预分配集合容量");
  println!("• 大型数据使用 Box 存储在堆上");
  println!("• 利用零成本抽象");
}

// 测试模块
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ownership_transfer() {
    let s1 = String::from("hello");
    let s2 = s1;
    // s1 不再有效
    assert_eq!(s2, "hello");
  }

  #[test]
  fn test_borrowing() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    assert_eq!(len, 5);
    assert_eq!(s, "hello"); // s 仍然有效

    fn calculate_length(s: &String) -> usize {
      s.len()
    }
  }

  #[test]
  fn test_mutable_reference() {
    let mut s = String::from("hello");
    change(&mut s);
    assert_eq!(s, "hello, world");

    fn change(some_string: &mut String) {
      some_string.push_str(", world");
    }
  }

  #[test]
  fn test_closure_capture() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
  }

  #[test]
  fn test_smart_pointers() {
    let data = Rc::new(RefCell::new(5));
    let data_clone = Rc::clone(&data);

    *data.borrow_mut() += 10;
    assert_eq!(*data_clone.borrow(), 15);
  }

  #[test]
  fn test_box_recursive_type() {
    #[derive(Debug, PartialEq)]
    enum List {
      Cons(i32, Box<List>),
      Nil,
    }

    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // 测试列表结构
    match list {
      Cons(1, ref tail) => match **tail {
        Cons(2, ref tail2) => match **tail2 {
          Cons(3, ref tail3) => {
            assert_eq!(**tail3, Nil);
          }
          _ => panic!("Unexpected structure"),
        },
        _ => panic!("Unexpected structure"),
      },
      _ => panic!("Unexpected structure"),
    }
  }

  #[test]
  fn test_lifetime_function() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
      if x.len() > y.len() { x } else { y }
    }

    let string1 = "abcd";
    let string2 = "xyz";
    let result = longest(string1, string2);
    assert_eq!(result, "abcd");
  }

  #[test]
  fn test_move_closure() {
    let data = vec![1, 2, 3];
    let closure = move || data.len();

    assert_eq!(closure(), 3);
    // data 已被移动，不能再使用
  }
}
