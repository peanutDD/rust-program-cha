// borrowing_analysis.rs
// Rust 引用与借用深度分析
// 基于 https://course.rs/basic/ownership/borrowing.html 的全面总结

/*
引用与借用 (References and Borrowing) 完整指南

本文件包含了 Rust 中引用与借用的所有核心概念，包括：
1. 引用的基本概念
2. 借用规则
3. 可变引用与不可变引用
4. 悬垂引用
5. 引用作为函数参数
6. 字符串切片
7. 其他类型的切片
8. 生命周期基础
9. 常见错误和最佳实践
*/

// ============================================================================
// 1. 引用的基本概念 (Basic Concepts of References)
// ============================================================================

/// 引用允许你使用值但不获取其所有权
/// 引用就像一个指针，因为它是一个地址，我们可以由此访问储存于该地址的属于其他变量的数据
/// 与指针不同，引用在其生命周期内保证指向某个特定类型的有效值
fn basic_reference_example() {
  let s1 = String::from("hello");

  // &s1 语法让我们创建一个指向值 s1 的引用，但是并不拥有它
  let len = calculate_length(&s1);

  // 因为并不拥有这个值，所以当引用停止使用时，它所指向的值也不会被丢弃
  println!("The length of '{}' is {}.", s1, len);

  // s1 在这里仍然有效，因为我们没有转移它的所有权
}

/// 计算字符串长度的函数，接受一个 String 的引用作为参数
/// 这个函数签名中的 &String 表示参数 s 是一个引用
fn calculate_length(s: &String) -> usize {
  s.len()
  // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，所以什么也不会发生
}

// ============================================================================
// 2. 借用 (Borrowing)
// ============================================================================

/// 我们将创建一个引用的行为称为借用（borrowing）
/// 正如现实生活中，如果一个人拥有某样东西，你可以从他那里借来
/// 当使用完毕后，也必须要物归原主
fn borrowing_concept() {
  let s = String::from("hello");

  // 这里我们借用了 s 的值
  let len = calculate_length(&s);

  println!("Length: {}", len);

  // s 在这里仍然有效，因为我们只是借用了它，没有获取所有权
  println!("Original string: {}", s);
}

// ============================================================================
// 3. 可变引用 (Mutable References)
// ============================================================================

/// 如果我们想要修改借用的值，我们需要使用可变引用
fn mutable_reference_example() {
  let mut s = String::from("hello");

  // 使用 &mut 创建可变引用
  change(&mut s);

  println!("Modified string: {}", s);
}

/// 接受可变引用的函数
fn change(some_string: &mut String) {
  some_string.push_str(", world");
}

// ============================================================================
// 4. 借用规则 (Borrowing Rules)
// ============================================================================

/// Rust 的借用规则确保内存安全：
/// 1. 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用
/// 2. 引用必须总是有效的
fn borrowing_rules_demonstration() {
  let mut s = String::from("hello");

  // 规则1：多个不可变引用是允许的
  {
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 变量 r1 和 r2 不会再被使用
  }

  // 规则1：只能有一个可变引用
  {
    let r1 = &mut s; // 没问题
    // let r2 = &mut s; // 错误！不能同时有两个可变引用
    println!("{}", r1);
  }

  // 规则1：不能同时有可变和不可变引用
  {
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    // let r3 = &mut s; // 错误！不能在有不可变引用的同时有可变引用
    println!("{} and {}", r1, r2);
  }
}

/// 演示引用的作用域
fn reference_scope_example() {
  let mut s = String::from("hello");

  let r1 = &s; // 没问题
  let r2 = &s; // 没问题
  println!("{} and {}", r1, r2);
  // 此位置之后 r1 和 r2 不再使用

  let r3 = &mut s; // 没问题
  println!("{}", r3);
}

// ============================================================================
// 5. 悬垂引用 (Dangling References)
// ============================================================================

/// 在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个悬垂指针
/// 在 Rust 中编译器确保引用永远也不会变成悬垂状态

// 这个函数会产生编译错误
/*
fn dangle() -> &String { // dangle 返回一个字符串的引用
    let s = String::from("hello"); // s 是一个新字符串
    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。危险！
*/

/// 正确的做法是直接返回 String
fn no_dangle() -> String {
  let s = String::from("hello");
  s // 返回 s，所有权被移动出去，没有值被释放
}

fn dangling_reference_example() {
  // let reference_to_nothing = dangle(); // 这会产生编译错误
  let string = no_dangle(); // 这是正确的
  println!("No dangling: {}", string);
}

// ============================================================================
// 6. 字符串切片 (String Slices)
// ============================================================================

/// 字符串切片是对 String 中一部分的引用
/// 切片的类型是 &str
fn string_slice_example() {
  let s = String::from("hello world");

  // 这些都是字符串切片
  let hello = &s[0..5]; // "hello"
  let world = &s[6..11]; // "world"
  let hello2 = &s[..5]; // 等同于 &s[0..5]
  let world2 = &s[6..]; // 从索引6到结尾
  let whole = &s[..]; // 整个字符串的切片

  println!("hello: {}, world: {}", hello, world);
  println!("hello2: {}, world2: {}", hello2, world2);
  println!("whole: {}", whole);
}

/// 使用字符串切片重写 first_word 函数
fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}

/// 更好的 first_word 函数，接受 &str 参数
fn first_word_improved(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}

fn string_slice_usage() {
  let my_string = String::from("hello world");

  // first_word 中传入 `String` 的 slice
  let word = first_word(&my_string);
  println!("First word from slice: {}", word);

  // first_word 中传入 `String` 的引用
  let word = first_word(&my_string);
  println!("First word from string: {}", word);

  let my_string_literal = "hello world";

  // first_word 中传入字符串字面量的 slice
  let word = first_word_improved(&my_string_literal[0..6]);
  println!("First word from literal slice: {}", word);

  // first_word 中传入字符串字面量
  let word = first_word_improved(my_string_literal);
  println!("First word from literal: {}", word);
}

// ============================================================================
// 7. 其他类型的切片 (Other Slices)
// ============================================================================

/// 切片不仅仅适用于字符串，也适用于其他类型的集合
fn other_slices_example() {
  let a = [1, 2, 3, 4, 5];

  // 数组切片的类型是 &[i32]
  let slice = &a[1..3];

  assert_eq!(slice, &[2, 3]);

  println!("Array slice: {:?}", slice);

  // 向量切片
  let v = vec![1, 2, 3, 4, 5];
  let v_slice = &v[2..4];
  println!("Vector slice: {:?}", v_slice);
}

// ============================================================================
// 8. 生命周期基础 (Lifetime Basics)
// ============================================================================

/// 生命周期确保引用有效
/// 大多数时候生命周期是隐含并可以推断的
/// 当引用的生命周期可能以几种不同方式相关时，必须注明生命周期

/// 这个函数需要生命周期标注
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() { x } else { y }
}

fn lifetime_example() {
  let string1 = String::from("abcd");
  let string2 = "xyz";

  let result = longest(string1.as_str(), string2);
  println!("The longest string is {}", result);
}

// ============================================================================
// 9. 常见错误示例和解决方案 (Common Errors and Solutions)
// ============================================================================

/// 演示常见的借用错误
fn common_borrowing_errors() {
  // 错误1：同时有多个可变引用
  /*
  let mut s = String::from("hello");
  let r1 = &mut s;
  let r2 = &mut s; // 错误！
  println!("{}, {}", r1, r2);
  */

  // 正确做法：确保可变引用的作用域不重叠
  let mut s = String::from("hello");
  {
    let r1 = &mut s;
    r1.push_str(", world");
  } // r1 在这里离开作用域

  let r2 = &mut s; // 现在可以创建新的可变引用
  r2.push_str("!");
  println!("Final string: {}", s);
}

/// 演示引用与所有权的交互
fn reference_ownership_interaction() {
  let mut s = String::from("hello");

  // 不可变借用
  let r1 = &s;
  println!("r1: {}", r1);

  // 在 r1 使用完毕后，可以创建可变借用
  let r2 = &mut s;
  r2.push_str(", world");
  println!("r2: {}", r2);

  // 注意：不能在 r1 仍在使用时创建 r2
}

// ============================================================================
// 10. 最佳实践 (Best Practices)
// ============================================================================

/// 最佳实践示例
fn best_practices() {
  // 1. 优先使用不可变引用
  let data = vec![1, 2, 3, 4, 5];
  process_data(&data); // 传递不可变引用

  // 2. 只在需要修改时使用可变引用
  let mut data = vec![1, 2, 3, 4, 5];
  modify_data(&mut data);

  // 3. 使用切片而不是具体类型以提高灵活性
  let s = String::from("hello world");
  let word = get_first_word(&s); // 可以接受 &String

  let s_literal = "hello world";
  let word2 = get_first_word(s_literal); // 也可以接受 &str

  println!("Words: {}, {}", word, word2);
}

fn process_data(data: &[i32]) {
  println!("Processing {} items", data.len());
}

fn modify_data(data: &mut Vec<i32>) {
  data.push(6);
}

fn get_first_word(s: &str) -> &str {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }
  &s[..]
}

// ============================================================================
// 11. 高级借用模式 (Advanced Borrowing Patterns)
// ============================================================================

/// 结构体中的引用
struct ImportantExcerpt<'a> {
  part: &'a str,
}

fn struct_with_references() {
  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find a '.'");
  let i = ImportantExcerpt {
    part: first_sentence,
  };
  println!("Important excerpt: {}", i.part);
}

/// 方法中的生命周期
impl<'a> ImportantExcerpt<'a> {
  /// 获取重要性级别（演示方法中的生命周期）
  #[allow(dead_code)] // 演示代码，实际项目中会使用
  fn level(&self) -> i32 {
    3
  }

  /// 宣布并返回部分内容（演示生命周期参数）
  #[allow(dead_code)] // 演示代码，实际项目中会使用
  fn announce_and_return_part(&self, announcement: &str) -> &str {
    println!("Attention please: {}", announcement);
    self.part
  }
}

// ============================================================================
// 12. 性能考虑 (Performance Considerations)
// ============================================================================

/// 借用 vs 克隆的性能比较
fn performance_comparison() {
  let large_string = "a".repeat(1000000);

  // 高效：使用引用，无额外内存分配
  let len1 = calculate_length_efficient(&large_string);

  // 低效：克隆整个字符串
  let len2 = calculate_length_inefficient(large_string.clone());

  println!("Lengths: {}, {}", len1, len2);
}

fn calculate_length_efficient(s: &String) -> usize {
  s.len()
}

fn calculate_length_inefficient(s: String) -> usize {
  s.len()
}

// ============================================================================
// 主函数：运行所有示例
// ============================================================================

pub fn run_all_examples() {
  println!("=== Rust 引用与借用完整指南 ===");

  println!("\n1. 基本引用概念:");
  basic_reference_example();

  println!("\n2. 借用概念:");
  borrowing_concept();

  println!("\n3. 可变引用:");
  mutable_reference_example();

  println!("\n4. 借用规则演示:");
  borrowing_rules_demonstration();
  reference_scope_example();

  println!("\n5. 悬垂引用:");
  dangling_reference_example();

  println!("\n6. 字符串切片:");
  string_slice_example();
  string_slice_usage();

  println!("\n7. 其他类型切片:");
  other_slices_example();

  println!("\n8. 生命周期:");
  lifetime_example();

  println!("\n9. 常见错误处理:");
  common_borrowing_errors();
  reference_ownership_interaction();

  println!("\n10. 最佳实践:");
  best_practices();

  println!("\n11. 结构体中的引用:");
  struct_with_references();

  println!("\n12. 性能考虑:");
  performance_comparison();

  println!("\n=== 引用与借用指南完成 ===");
}

// ============================================================================
// 总结 (Summary)
// ============================================================================

/*
引用与借用总结：

1. 引用的基本概念：
   - 引用允许使用值但不获取所有权
   - 使用 & 符号创建引用
   - 引用在生命周期内保证指向有效值

2. 借用规则：
   - 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用
   - 引用必须总是有效的

3. 可变引用：
   - 使用 &mut 创建可变引用
   - 可变引用允许修改借用的值
   - 同一时间只能有一个可变引用

4. 悬垂引用：
   - Rust 编译器防止悬垂引用的产生
   - 确保引用的生命周期不超过其指向的数据

5. 切片：
   - 字符串切片 &str 是对字符串一部分的引用
   - 数组切片 &[T] 是对数组一部分的引用
   - 切片提供了安全访问集合部分数据的方式

6. 生命周期：
   - 确保引用有效的机制
   - 大多数情况下可以自动推断
   - 复杂情况下需要显式标注

7. 最佳实践：
   - 优先使用不可变引用
   - 只在需要修改时使用可变引用
   - 使用切片提高函数的灵活性
   - 避免不必要的克隆操作

引用与借用是 Rust 内存安全的核心机制，理解这些概念对于编写高效、安全的 Rust 代码至关重要。
*/
