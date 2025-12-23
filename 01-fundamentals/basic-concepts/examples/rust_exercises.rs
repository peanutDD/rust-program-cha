// Rust 核心概念练习题
// 本文件包含了各种练习题，帮助巩固 Rust 核心概念
// 请尝试完成每个练习，并运行代码验证结果

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

// ============================================================================
// 练习 1：变量绑定与可变性
// ============================================================================

/// 练习 1.1：修复编译错误
fn exercise_1_1() {
  println!("=== 练习 1.1：变量绑定 ===");

  // TODO: 修复下面的代码，使其能够编译通过
  let x = 5;
  println!("x 的值是: {}", x);

  // 取消注释并修复
  // x = 10;
  // println!("x 的新值是: {}", x);

  // 提示：考虑使用 mut 关键字或变量遮蔽
}

/// 练习 1.2：变量遮蔽实践
fn exercise_1_2() {
  println!("\n=== 练习 1.2：变量遮蔽 ===");

  // TODO: 使用变量遮蔽将字符串转换为数字
  let input = "42";
  println!("输入字符串: {}", input);

  // 在这里添加代码，将 input 遮蔽为数字类型
  // let input = ...

  // println!("转换后的数字: {}", input);
}

// ============================================================================
// 练习 2：解构赋值
// ============================================================================

#[derive(Debug)]
struct Student {
  name: String,
  age: u32,
  grade: char,
  subjects: Vec<String>,
}

/// 练习 2.1：结构体解构
fn exercise_2_1() {
  println!("\n=== 练习 2.1：结构体解构 ===");

  let student = Student {
    name: String::from("Alice"),
    age: 20,
    grade: 'A',
    subjects: vec![String::from("Math"), String::from("Physics")],
  };

  // TODO: 使用解构提取学生的姓名和年龄，忽略其他字段
  // let Student { ... } = student;

  // println!("学生姓名: {}, 年龄: {}", name, age);
}

/// 练习 2.2：复杂解构
fn exercise_2_2() {
  println!("\n=== 练习 2.2：复杂解构 ===");

  let data = ((1, 2), (3, 4), (5, 6));

  // TODO: 使用解构提取第一个和最后一个元组的第一个元素
  // let ((...), _, (...)) = data;

  // println!("第一个: {}, 最后一个: {}", first, last);
}

// ============================================================================
// 练习 3：所有权与借用
// ============================================================================

/// 练习 3.1：修复所有权问题
fn exercise_3_1() {
  println!("\n=== 练习 3.1：所有权问题 ===");

  let s1 = String::from("hello");
  let s2 = s1; // s1 的所有权转移给 s2

  // TODO: 修复下面的代码，使其能够编译通过
  // println!("s1: {}, s2: {}", s1, s2);

  // 提示：考虑使用克隆或借用
}

/// 练习 3.2：实现一个函数，计算字符串长度但不获取所有权
fn exercise_3_2() {
  println!("\n=== 练习 3.2：借用实践 ===");

  fn string_length(/* TODO: 添加参数 */) -> usize {
    // TODO: 实现函数体
    0
  }

  let text = String::from("Hello, Rust!");
  let len = string_length(/* TODO: 传递参数 */);

  println!("字符串 '{}' 的长度是: {}", text, len);
}

/// 练习 3.3：可变借用
fn exercise_3_3() {
  println!("\n=== 练习 3.3：可变借用 ===");

  fn append_exclamation(/* TODO: 添加参数 */) {
    // TODO: 在字符串末尾添加感叹号
  }

  let mut message = String::from("Hello, World");
  append_exclamation(/* TODO: 传递参数 */);

  println!("修改后的消息: {}", message);
}

// ============================================================================
// 练习 4：智能指针
// ============================================================================

/// 练习 4.1：使用 Rc 实现共享所有权
fn exercise_4_1() {
  println!("\n=== 练习 4.1：Rc 共享所有权 ===");

  #[derive(Debug)]
  enum List {
    Cons(i32, Rc<List>),
    Nil,
  }

  use List::{Cons, Nil};

  // TODO: 创建一个共享的链表尾部
  // let tail = Rc::new(...);

  // TODO: 创建两个不同的链表，它们共享相同的尾部
  // let list1 = Cons(1, Rc::clone(&tail));
  // let list2 = Cons(2, Rc::clone(&tail));

  // println!("List1: {:?}", list1);
  // println!("List2: {:?}", list2);
  // println!("引用计数: {}", Rc::strong_count(&tail));
}

/// 练习 4.2：使用 RefCell 实现内部可变性
fn exercise_4_2() {
  println!("\n=== 练习 4.2：RefCell 内部可变性 ===");

  use std::cell::RefCell;

  #[derive(Debug)]
  struct Counter {
    value: RefCell<i32>,
  }

  impl Counter {
    fn new() -> Counter {
      Counter {
        value: RefCell::new(0),
      }
    }

    // TODO: 实现 increment 方法
    fn increment(&self) {
      // 提示：使用 borrow_mut()
    }

    // TODO: 实现 get_value 方法
    fn get_value(&self) -> i32 {
      // 提示：使用 borrow()
      0
    }
  }

  let counter = Counter::new();
  println!("初始值: {}", counter.get_value());

  counter.increment();
  counter.increment();
  println!("增加后的值: {}", counter.get_value());
}

// ============================================================================
// 练习 5：并发编程
// ============================================================================

/// 练习 5.1：多线程计数器
fn exercise_5_1() {
  println!("\n=== 练习 5.1：多线程计数器 ===");

  // TODO: 创建一个线程安全的计数器
  // let counter = Arc::new(Mutex::new(0));

  let mut handles = vec![];

  for i in 0..5 {
    // TODO: 为每个线程克隆计数器
    // let counter = Arc::clone(&counter);

    let handle = thread::spawn(move || {
      // TODO: 在每个线程中增加计数器
      println!("线程 {} 正在运行", i);
    });

    handles.push(handle);
  }

  // 等待所有线程完成
  for handle in handles {
    handle.join().unwrap();
  }

  // TODO: 打印最终的计数器值
  // println!("最终计数: {}", *counter.lock().unwrap());
}

/// 练习 5.2：消息传递
fn exercise_5_2() {
  println!("\n=== 练习 5.2：消息传递 ===");

  use std::sync::mpsc;
  use std::time::Duration;

  // TODO: 创建一个通道
  // let (tx, rx) = mpsc::channel();

  // TODO: 在新线程中发送消息
  thread::spawn(move || {
    let messages = vec!["Hello", "from", "another", "thread"];

    for msg in messages {
      // TODO: 发送消息
      // tx.send(msg).unwrap();
      thread::sleep(Duration::from_millis(500));
    }
  });

  // TODO: 在主线程中接收消息
  // for received in rx {
  //     println!("收到消息: {}", received);
  // }
}

// ============================================================================
// 练习 6：错误处理
// ============================================================================

/// 练习 6.1：实现安全的除法函数
fn exercise_6_1() {
  println!("\n=== 练习 6.1：安全除法 ===");

  // TODO: 实现一个安全的除法函数，返回 Result
  fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    // 提示：检查除数是否为零
    Ok(0.0) // 占位符
  }

  // 测试函数
  match safe_divide(10.0, 2.0) {
    Ok(result) => println!("10.0 / 2.0 = {}", result),
    Err(e) => println!("错误: {}", e),
  }

  match safe_divide(10.0, 0.0) {
    Ok(result) => println!("10.0 / 0.0 = {}", result),
    Err(e) => println!("错误: {}", e),
  }
}

/// 练习 6.2：链式错误处理
fn exercise_6_2() {
  println!("\n=== 练习 6.2：链式错误处理 ===");

  // TODO: 实现一个函数，解析字符串为数字并加倍
  fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    // 提示：使用 ? 操作符
    Ok(0) // 占位符
  }

  let inputs = ["42", "abc", "100"];

  for input in &inputs {
    match parse_and_double(input) {
      Ok(result) => println!("'{}' -> {}", input, result),
      Err(e) => println!("解析 '{}' 失败: {}", input, e),
    }
  }
}

// ============================================================================
// 练习 7：生命周期
// ============================================================================

/// 练习 7.1：实现一个比较函数
fn exercise_7_1() {
  println!("\n=== 练习 7.1：生命周期参数 ===");

  // TODO: 实现一个函数，返回两个字符串切片中较长的一个
  // fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
  //     if x.len() > y.len() { x } else { y }
  // }

  let string1 = "short";
  let string2 = "this is a longer string";

  // let result = longer(string1, string2);
  // println!("较长的字符串是: '{}'", result);
}

/// 练习 7.2：带生命周期的结构体
fn exercise_7_2() {
  println!("\n=== 练习 7.2：结构体生命周期 ===");

  // TODO: 定义一个包含字符串切片的结构体
  // struct TextHolder<'a> {
  //     text: &'a str,
  // }

  // impl<'a> TextHolder<'a> {
  //     fn new(text: &'a str) -> Self {
  //         TextHolder { text }
  //     }
  //
  //     fn get_text(&self) -> &str {
  //         self.text
  //     }
  // }

  let content = "Hello, lifetime!";
  // let holder = TextHolder::new(content);
  // println!("持有的文本: {}", holder.get_text());
}

// ============================================================================
// 练习 8：综合应用
// ============================================================================

/// 练习 8.1：实现一个简单的缓存
fn exercise_8_1() {
  println!("\n=== 练习 8.1：简单缓存 ===");

  // TODO: 实现一个简单的缓存结构
  struct SimpleCache {
    data: HashMap<String, i32>,
  }

  impl SimpleCache {
    fn new() -> Self {
      SimpleCache {
        data: HashMap::new(),
      }
    }

    // TODO: 实现 get 方法
    fn get(&self, key: &str) -> Option<&i32> {
      None // 占位符
    }

    // TODO: 实现 insert 方法
    fn insert(&mut self, key: String, value: i32) {
      // 实现插入逻辑
    }

    // TODO: 实现 get_or_insert_with 方法
    fn get_or_insert_with<F>(&mut self, key: String, f: F) -> &i32
    where
      F: FnOnce() -> i32,
    {
      // 如果键存在则返回值，否则调用函数计算并插入
      &0 // 占位符
    }
  }

  let mut cache = SimpleCache::new();

  // 测试缓存
  cache.insert("key1".to_string(), 42);
  println!("key1: {:?}", cache.get("key1"));

  let value = cache.get_or_insert_with("key2".to_string(), || {
    println!("计算 key2 的值...");
    100
  });
  println!("key2: {}", value);
}

/// 练习 8.2：实现一个工作队列
fn exercise_8_2() {
  println!("\n=== 练习 8.2：工作队列 ===");

  use std::sync::mpsc;
  use std::time::Duration;

  // TODO: 实现一个简单的工作队列
  struct WorkQueue {
    // 添加必要的字段
  }

  impl WorkQueue {
    fn new() -> Self {
      WorkQueue {
                // 初始化字段
            }
    }

    // TODO: 添加工作到队列
    fn add_work<F>(&self, work: F)
    where
      F: FnOnce() + Send + 'static,
    {
      // 实现添加工作的逻辑
    }

    // TODO: 启动工作线程
    fn start_workers(&self, num_workers: usize) {
      // 实现启动工作线程的逻辑
    }
  }

  // 使用示例（注释掉，因为需要完整实现）
  // let queue = WorkQueue::new();
  // queue.start_workers(3);
  //
  // for i in 0..5 {
  //     queue.add_work(move || {
  //         println!("执行工作 {}", i);
  //         thread::sleep(Duration::from_millis(100));
  //     });
  // }

  println!("工作队列示例（需要完整实现）");
}

// ============================================================================
// 主函数和测试
// ============================================================================

fn main() {
  println!("🎯 Rust 核心概念练习题\n");
  println!("请逐个完成每个练习，取消注释并修复代码\n");

  // 基础练习
  exercise_1_1();
  exercise_1_2();

  // 解构练习
  exercise_2_1();
  exercise_2_2();

  // 所有权练习
  exercise_3_1();
  exercise_3_2();
  exercise_3_3();

  // 智能指针练习
  exercise_4_1();
  exercise_4_2();

  // 并发练习
  exercise_5_1();
  exercise_5_2();

  // 错误处理练习
  exercise_6_1();
  exercise_6_2();

  // 生命周期练习
  exercise_7_1();
  exercise_7_2();

  // 综合应用练习
  exercise_8_1();
  exercise_8_2();

  println!("\n🎉 练习完成！");
  println!("\n💡 提示：");
  println!("1. 逐个取消注释并完成每个 TODO");
  println!("2. 运行代码验证你的实现");
  println!("3. 阅读编译器错误信息，它们会指导你");
  println!("4. 不要害怕实验和犯错误！");
}

// ============================================================================
// 答案提示（可选参考）
// ============================================================================

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_safe_divide() {
    // 这里可以添加测试用例
    assert_eq!(2.0, 1.0 + 1.0);
  }

  // 添加更多测试...
}

// 提示：如果你完成了所有练习，可以尝试以下挑战：
// 1. 实现一个线程安全的 LRU 缓存
// 2. 创建一个简单的 HTTP 客户端
// 3. 实现一个基于 actor 模型的并发系统
// 4. 编写一个简单的解析器
// 5. 实现自定义的智能指针类型
