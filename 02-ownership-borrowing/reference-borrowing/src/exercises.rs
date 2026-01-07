//! # Rust 引用与借用练习题
//!
//! 这些练习帮助你巩固引用与借用的概念。
//! 基于 https://course.rs/basic/ownership/borrowing.html

/*
练习说明：
1. 每个练习都有详细的说明和预期结果
2. 有些练习故意包含错误，需要你修复
3. 运行 `cargo test exercises` 来检查你的答案
4. 所有练习都基于 https://course.rs/basic/ownership/borrowing.html 的内容
*/

// ============================================================================
// 练习 1: 基本引用
// ============================================================================

/// 练习1：修复这个函数，使其能够计算字符串长度而不获取所有权
/// 提示：使用引用
#[allow(dead_code)] // 练习题，供学习者实现
fn exercise1_calculate_length(_s: String) -> usize {
  // TODO: 修改函数签名和实现
  0 // 占位符
}

/// 练习1的正确实现（在测试中使用）
#[cfg(test)]
fn exercise1_calculate_length_solution(s: &String) -> usize {
  s.len()
}

// ============================================================================
// 练习 2: 可变引用
// ============================================================================

/// 练习2：修复这个函数，使其能够修改传入的字符串
/// 提示：使用可变引用
#[allow(dead_code)] // 练习题，供学习者实现
fn exercise2_append_world(_s: &String) {
  // TODO: 修改函数签名，使其能够修改字符串
  // s.push_str(", world!");
}

/// 练习2的正确实现（在测试中使用）
#[cfg(test)]
fn exercise2_append_world_solution(s: &mut String) {
  s.push_str(", world!");
}

// ============================================================================
// 练习 3: 借用规则
// ============================================================================

/// 练习3：修复这个函数中的借用错误
/// 提示：注意可变引用和不可变引用的规则
#[allow(dead_code)] // 练习题，供学习者实现
fn exercise3_borrowing_rules() {
  let _s = String::from("hello");

  // TODO: 修复下面的代码，使其能够编译
  /*
  let r1 = &s;
  let r2 = &s;
  let r3 = &mut s;

  println!("{}, {}, and {}", r1, r2, r3);
  */
}

/// 练习3的正确实现（在测试中使用）
#[cfg(test)]
fn exercise3_borrowing_rules_solution() {
  let mut s = String::from("hello");

  let r1 = &s;
  let r2 = &s;
  println!("{} and {}", r1, r2);
  // r1 和 r2 不再使用

  let r3 = &mut s;
  println!("{}", r3);
}

// ============================================================================
// 练习 4: 字符串切片
// ============================================================================

/// 练习4：实现一个函数，返回字符串的最后一个单词
/// 提示：使用字符串切片和迭代器
#[allow(dead_code)] // 练习题，供学习者实现
fn exercise4_last_word(_s: &String) -> &str {
  // TODO: 实现这个函数
  // 提示：可以使用 split_whitespace() 和 last()
  ""
}

/// 练习4的正确实现（在测试中使用）
#[cfg(test)]
fn exercise4_last_word_solution(s: &str) -> &str {
  s.split_whitespace().last().unwrap_or("")
}

// ============================================================================
// 练习 5: 数组切片
// ============================================================================

/// 练习5：实现一个函数，返回数组中最大值的索引
/// 提示：使用切片和迭代器
#[allow(dead_code)] // 练习题，供学习者实现
fn exercise5_max_index(_arr: &[i32]) -> Option<usize> {
  // TODO: 实现这个函数
  None
}

/// 练习5的正确实现（在测试中使用）
#[cfg(test)]
fn exercise5_max_index_solution(arr: &[i32]) -> Option<usize> {
  if arr.is_empty() {
    return None;
  }

  let mut max_idx = 0;
  for (i, &value) in arr.iter().enumerate() {
    if value > arr[max_idx] {
      max_idx = i;
    }
  }
  Some(max_idx)
}

// ============================================================================
// 练习 6: 生命周期
// ============================================================================

/// 练习6：为这个函数添加正确的生命周期标注
/// 提示：返回的引用的生命周期与输入参数相关
#[allow(dead_code)] // 练习题，供学习者实现
fn exercise6_longer_string<'a>(_x: &'a str, _y: &'a str) -> &'a str {
  // TODO: 添加生命周期标注
  ""
}

/// 练习6的正确实现（在测试中使用）
#[cfg(test)]
fn exercise6_longer_string_solution<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() { x } else { y }
}

// ============================================================================
// 练习 7: 结构体中的引用
// ============================================================================

/// 练习7：修复这个结构体定义，使其能够存储字符串引用
/// 提示：需要生命周期参数
// struct Exercise7Book {
//     title: &str,
//     author: &str,
// }
// TODO: 添加生命周期参数

/// 练习7的正确实现
#[allow(dead_code)] // 演示代码，在测试中使用
struct Exercise7BookSolution<'a> {
  title: &'a str,
  author: &'a str,
}

// ============================================================================
// 练习 8: 复杂的借用场景
// ============================================================================

/// 练习8：实现一个函数，交换两个可变引用指向的值
/// 提示：使用 std::mem::swap
#[allow(dead_code)] // 练习题，供学习者实现
fn exercise8_swap_values(_x: &i32, _y: &i32) {
  // TODO: 修改函数签名和实现
  // std::mem::swap(x, y);
}

/// 练习8的正确实现（在测试中使用）
#[cfg(test)]
fn exercise8_swap_values_solution(x: &mut i32, y: &mut i32) {
  std::mem::swap(x, y);
}

// ============================================================================
// 练习 9: 字符串处理
// ============================================================================

/// 练习9：实现一个函数，移除字符串中的所有空格
/// 要求：不能获取字符串的所有权，返回新的 String
#[allow(dead_code)] // 练习题，供学习者实现
fn exercise9_remove_spaces(_s: &String) -> String {
  // TODO: 实现这个函数
  String::new()
}

/// 练习9的正确实现（在测试中使用）
#[cfg(test)]
fn exercise9_remove_spaces_solution(s: &str) -> String {
  s.chars().filter(|c| !c.is_whitespace()).collect()
}

// ============================================================================
// 练习 10: 高级切片操作
// ============================================================================

/// 练习10：实现一个函数，将数组分成两半
/// 返回两个切片的元组
#[allow(dead_code)] // 练习题，供学习者实现
fn exercise10_split_array(_arr: &[i32]) -> (&[i32], &[i32]) {
  // TODO: 实现这个函数
  (&[], &[])
}

/// 练习10的正确实现（在测试中使用）
#[cfg(test)]
fn exercise10_split_array_solution(arr: &[i32]) -> (&[i32], &[i32]) {
  let mid = arr.len() / 2;
  arr.split_at(mid)
}

// ============================================================================
// 测试用例
// ============================================================================

#[cfg(test)]
mod exercise_tests {
  use super::*;

  #[test]
  fn test_exercise1_solution() {
    let s = String::from("hello");
    let len = exercise1_calculate_length_solution(&s);
    assert_eq!(len, 5);
    // s 仍然有效
    assert_eq!(s, "hello");
  }

  #[test]
  fn test_exercise2_solution() {
    let mut s = String::from("hello");
    exercise2_append_world_solution(&mut s);
    assert_eq!(s, "hello, world!");
  }

  #[test]
  fn test_exercise3_solution() {
    // 这个测试确保解决方案能够编译
    exercise3_borrowing_rules_solution();
  }

  #[test]
  fn test_exercise4_solution() {
    let s = "hello world rust";
    assert_eq!(exercise4_last_word_solution(s), "rust");

    let s2 = "single";
    assert_eq!(exercise4_last_word_solution(s2), "single");

    let s3 = "";
    assert_eq!(exercise4_last_word_solution(s3), "");
  }

  #[test]
  fn test_exercise5_solution() {
    let arr = [1, 5, 3, 9, 2];
    assert_eq!(exercise5_max_index_solution(&arr), Some(3));

    let arr2 = [10];
    assert_eq!(exercise5_max_index_solution(&arr2), Some(0));

    let arr3: [i32; 0] = [];
    assert_eq!(exercise5_max_index_solution(&arr3), None);
  }

  #[test]
  fn test_exercise6_solution() {
    let s1 = "hello";
    let s2 = "world!";
    assert_eq!(exercise6_longer_string_solution(s1, s2), "world!");

    let s3 = "rust";
    let s4 = "go";
    assert_eq!(exercise6_longer_string_solution(s3, s4), "rust");
  }

  #[test]
  fn test_exercise7_solution() {
    let title = "The Rust Programming Language";
    let author = "Steve Klabnik and Carol Nichols";

    let book = Exercise7BookSolution { title, author };

    assert_eq!(book.title, "The Rust Programming Language");
    assert_eq!(book.author, "Steve Klabnik and Carol Nichols");
  }

  #[test]
  fn test_exercise8_solution() {
    let mut x = 5;
    let mut y = 10;

    exercise8_swap_values_solution(&mut x, &mut y);

    assert_eq!(x, 10);
    assert_eq!(y, 5);
  }

  #[test]
  fn test_exercise9_solution() {
    let s = "hello world rust";
    assert_eq!(exercise9_remove_spaces_solution(s), "helloworldrust");

    let s2 = "  a  b  c  ";
    assert_eq!(exercise9_remove_spaces_solution(s2), "abc");
  }

  #[test]
  fn test_exercise10_solution() {
    let arr = [1, 2, 3, 4, 5, 6];
    let (left, right) = exercise10_split_array_solution(&arr);
    assert_eq!(left, &[1, 2, 3]);
    assert_eq!(right, &[4, 5, 6]);

    let arr2 = [1, 2, 3, 4, 5];
    let (left2, right2) = exercise10_split_array_solution(&arr2);
    assert_eq!(left2, &[1, 2]);
    assert_eq!(right2, &[3, 4, 5]);
  }
}

// ============================================================================
// 挑战练习
// ============================================================================

/// 挑战练习1：实现一个安全的字符串分割函数
/// 要求：返回一个包含所有单词的 Vec，但每个单词都是原字符串的切片
#[allow(dead_code)] // 挑战题，供学习者实现
fn challenge1_split_words(s: &str) -> Vec<&str> {
  s.split_whitespace().collect()
}

/// 挑战练习2：实现一个函数，找到两个字符串的公共前缀
#[allow(dead_code)] // 挑战题，供学习者实现
fn challenge2_common_prefix<'a>(s1: &'a str, s2: &'a str) -> &'a str {
  let bytes1 = s1.as_bytes();
  let bytes2 = s2.as_bytes();

  let mut i = 0;
  while i < bytes1.len() && i < bytes2.len() && bytes1[i] == bytes2[i] {
    i += 1;
  }

  &s1[..i]
}

/// 挑战练习3：实现一个函数，安全地获取数组的第n个元素
#[allow(dead_code)] // 挑战题，供学习者实现
fn challenge3_safe_get<T>(arr: &[T], index: usize) -> Option<&T> {
  arr.get(index)
}

#[cfg(test)]
mod challenge_tests {
  use super::*;

  #[test]
  fn test_challenge1() {
    let s = "hello world rust programming";
    let words = challenge1_split_words(s);
    assert_eq!(words, vec!["hello", "world", "rust", "programming"]);
  }

  #[test]
  fn test_challenge2() {
    assert_eq!(challenge2_common_prefix("hello", "help"), "hel");
    assert_eq!(challenge2_common_prefix("rust", "ruby"), "ru");
    assert_eq!(challenge2_common_prefix("abc", "xyz"), "");
  }

  #[test]
  fn test_challenge3() {
    let arr = [1, 2, 3, 4, 5];
    assert_eq!(challenge3_safe_get(&arr, 2), Some(&3));
    assert_eq!(challenge3_safe_get(&arr, 10), None);
  }
}

// ============================================================================
// 练习指导
// ============================================================================

/*
练习完成指南：

1. 从练习1开始，逐个完成每个练习
2. 每完成一个练习，运行对应的测试来验证
3. 如果遇到编译错误，仔细阅读错误信息
4. 参考解决方案，但尽量先自己尝试
5. 完成基础练习后，挑战高级练习

常见错误类型：
- E0382: borrow of moved value（值被移动后的借用）
- E0499: cannot borrow as mutable more than once（多次可变借用）
- E0502: cannot borrow as mutable because it is also borrowed as immutable（同时存在可变和不可变借用）
- E0106: missing lifetime specifier（缺少生命周期标注）

学习建议：
1. 理解所有权、借用和生命周期的关系
2. 多练习不同场景下的引用使用
3. 学会阅读和理解编译器错误信息
4. 掌握切片的使用技巧
5. 了解何时使用引用，何时转移所有权
*/
