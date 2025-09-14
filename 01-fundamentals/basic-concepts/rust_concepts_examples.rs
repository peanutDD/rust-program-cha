// Rust 核心概念详细案例教程
// 本文件包含了 Rust 变量绑定、解构、所有权、智能指针等核心概念的详细示例

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// ============================================================================
// 第一部分：变量绑定与可变性
// ============================================================================

/// 演示基本的变量绑定和可变性
fn variable_binding_examples() {
  println!("=== 变量绑定与可变性示例 ===");

  // 1. 默认不可变绑定
  let x = 5;
  println!("不可变变量 x: {}", x);
  // x = 6; // 编译错误：cannot assign twice to immutable variable

  // 2. 显式可变绑定
  let mut y = 10;
  println!("可变变量 y 初始值: {}", y);
  y = 15;
  println!("可变变量 y 修改后: {}", y);

  // 3. 变量遮蔽（Shadowing）
  let spaces = "   ";
  println!("字符串 spaces: '{}'", spaces);
  let spaces = spaces.len(); // 类型从 &str 变为 usize
  println!("数字 spaces: {}", spaces);

  // 4. 遮蔽 vs 可变性的区别
  let mut _guess = "42";
  _guess = "43"; // 可以修改值，但不能改变类型
                // guess = guess.len(); // 编译错误：类型不匹配

  let guess = "42";
  let guess = guess.len(); // 遮蔽可以改变类型
  println!("遮蔽后的 guess: {}", guess);
}

// ============================================================================
// 第二部分：解构赋值详细示例
// ============================================================================

#[derive(Debug)]
struct Point {
  x: i32,
  y: i32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Person {
  name: String,
  age: u32,
  email: String,
}

/// 演示各种解构模式
fn destructuring_examples() {
  println!("\n=== 解构赋值示例 ===");

  // 1. 元组解构
  let tuple = (1, 2, 3, 4, 5);
  let (first, second, .., last) = tuple;
  println!(
    "元组解构 - 第一个: {}, 第二个: {}, 最后一个: {}",
    first, second, last
  );

  let (a, b, c) = (10, 20, 30);
  println!("简单元组解构: a={}, b={}, c={}", a, b, c);

  // 2. 数组解构
  let array = [1, 2, 3, 4, 5];
  let [first, .., last] = array;
  println!("数组解构 - 第一个: {}, 最后一个: {}", first, last);

  let [x, y, z, ..] = array;
  println!("数组前三个元素: x={}, y={}, z={}", x, y, z);

  // 3. 结构体解构
  let point = Point { x: 10, y: 20 };
  let Point { x, y } = point; // 字段名简化
  println!("结构体解构: x={}, y={}", x, y);

  let Point { x: px, y: py } = Point { x: 5, y: 15 }; // 重命名字段
  println!("结构体解构重命名: px={}, py={}", px, py);

  // 4. 嵌套解构
  let person = Person {
    name: String::from("Alice"),
    age: 30,
    email: String::from("alice@example.com"),
  };

  let Person { name, age, .. } = person; // 忽略部分字段
  println!("嵌套解构: name={}, age={}", name, age);

  // 5. 引用解构
  let point_ref = &Point { x: 100, y: 200 };
  let Point { x, y } = point_ref;
  println!("引用解构: x={}, y={}", x, y);
}

// ============================================================================
// 第三部分：所有权系统详细示例
// ============================================================================

/// 演示所有权转移
fn ownership_examples() {
  println!("\n=== 所有权系统示例 ===");

  // 1. 所有权转移（Move）
  let s1 = String::from("hello");
  let s2 = s1; // s1 的所有权转移给 s2
               // println!("{}", s1); // 编译错误：value borrowed here after move
  println!("所有权转移后 s2: {}", s2);

  // 2. 克隆（Clone）
  let s3 = String::from("world");
  let s4 = s3.clone(); // 深拷贝
  println!("克隆后 s3: {}, s4: {}", s3, s4);

  // 3. Copy 类型
  let x = 5;
  let y = x; // i32 实现了 Copy trait，所以是复制而非移动
  println!("Copy 类型 x: {}, y: {}", x, y);

  // 4. 函数参数的所有权
  fn take_ownership(s: String) {
    println!("函数内部: {}", s);
  } // s 在这里被丢弃

  let s = String::from("function");
  take_ownership(s);
  // println!("{}", s); // 编译错误：所有权已转移

  // 5. 返回值的所有权
  fn give_ownership() -> String {
    String::from("yours")
  }

  let s = give_ownership();
  println!("返回的所有权: {}", s);
}

/// 演示借用和引用
fn borrowing_examples() {
  println!("\n=== 借用系统示例 ===");

  // 1. 不可变借用
  let s = String::from("hello");
  let len = calculate_length(&s); // 借用而不获取所有权
  println!("字符串 '{}' 的长度是 {}", s, len);

  fn calculate_length(s: &String) -> usize {
    s.len()
  }

  // 2. 可变借用
  let mut s = String::from("hello");
  change(&mut s);
  println!("修改后的字符串: {}", s);

  fn change(s: &mut String) {
    s.push_str(", world");
  }

  // 3. 借用规则演示
  let mut s = String::from("hello");

  let r1 = &s; // 不可变借用
  let r2 = &s; // 可以有多个不可变借用
  println!("r1: {}, r2: {}", r1, r2);
  // r1 和 r2 在这里不再使用

  let r3 = &mut s; // 可变借用
  println!("r3: {}", r3);
  // 不能同时有可变和不可变借用
}

// ============================================================================
// 第四部分：智能指针详细示例
// ============================================================================

/// 演示 Box<T> 智能指针
fn box_examples() {
  println!("\n=== Box<T> 智能指针示例 ===");

  // 1. 基本 Box 使用
  let b = Box::new(5);
  println!("Box 中的值: {}", b);

  // 2. 递归类型定义
  #[derive(Debug)]
  #[allow(dead_code)]
  enum List {
    Cons(i32, Box<List>),
    Nil,
  }

  use List::{Cons, Nil};

  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  println!("递归链表: {:?}", list);

  // 3. 大型数据结构
  #[allow(dead_code)]
  struct LargeStruct {
    data: [u8; 1000],
  }

  let large_data = Box::new(LargeStruct { data: [0; 1000] });
  println!(
    "大型结构体已分配到堆上，大小: {} 字节",
    std::mem::size_of_val(&*large_data)
  );
}

/// 演示 Rc<T> 引用计数智能指针
fn rc_examples() {
  println!("\n=== Rc<T> 引用计数示例 ===");

  #[derive(Debug)]
  #[allow(dead_code)]
  enum List {
    Cons(i32, Rc<List>),
    Nil,
  }

  use List::{Cons, Nil};

  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  println!("创建 a 后引用计数: {}", Rc::strong_count(&a));

  let b = Cons(3, Rc::clone(&a));
  println!("创建 b 后引用计数: {}", Rc::strong_count(&a));

  {
    let c = Cons(4, Rc::clone(&a));
    println!("创建 c 后引用计数: {}", Rc::strong_count(&a));
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
  }

  println!("c 离开作用域后引用计数: {}", Rc::strong_count(&a));
}

/// 演示 RefCell<T> 内部可变性
fn refcell_examples() {
  println!("\n=== RefCell<T> 内部可变性示例 ===");

  #[derive(Debug)]
  struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
  }

  impl MockMessenger {
    fn new() -> MockMessenger {
      MockMessenger {
        sent_messages: RefCell::new(vec![]),
      }
    }

    fn send(&self, message: &str) {
      // 即使 self 是不可变的，我们也能修改内部数据
      self.sent_messages.borrow_mut().push(String::from(message));
    }

    fn get_messages(&self) -> Vec<String> {
      self.sent_messages.borrow().clone()
    }
  }

  let messenger = MockMessenger::new();
  messenger.send("Hello");
  messenger.send("World");

  println!("发送的消息: {:?}", messenger.get_messages());

  // 演示运行时借用检查
  let data = RefCell::new(5);

  {
    let mut borrowed = data.borrow_mut();
    *borrowed += 1;
    println!("修改后的值: {}", *borrowed);
    // borrowed 在这里被丢弃
  }

  println!("最终值: {}", data.borrow());
}

// ============================================================================
// 第五部分：并发编程示例
// ============================================================================

/// 演示基本线程使用
fn threading_examples() {
  println!("\n=== 线程并发示例 ===");

  // 1. 基本线程创建
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("子线程: {}", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..5 {
    println!("主线程: {}", i);
    thread::sleep(Duration::from_millis(1));
  }

  handle.join().unwrap();

  // 2. 线程间数据共享 - Arc<Mutex<T>>
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();
      *num += 1;
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("计数器最终值: {}", *counter.lock().unwrap());
}

/// 演示消息传递并发
fn message_passing_examples() {
  println!("\n=== 消息传递并发示例 ===");

  use std::sync::mpsc;

  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  for received in rx {
    println!("接收到: {}", received);
  }
}

// ============================================================================
// 第六部分：异步编程示例（需要 tokio）
// ============================================================================

/// 模拟异步函数（简化版，不使用真正的 async/await）
fn async_simulation_examples() {
  println!("\n=== 异步编程概念示例 ===");

  // 这里展示异步编程的概念，实际使用需要 tokio 等运行时
  println!("异步编程允许在等待 I/O 时执行其他任务");
  println!("Future 表示可能尚未完成的计算");
  println!("async/await 语法简化了异步代码的编写");

  // 模拟异步任务的概念
  struct SimpleFuture {
    completed: bool,
  }

  impl SimpleFuture {
    fn new() -> Self {
      SimpleFuture { completed: false }
    }

    fn poll(&mut self) -> bool {
      if !self.completed {
        println!("Future 正在执行...");
        self.completed = true;
        false // 尚未完成
      } else {
        println!("Future 已完成!");
        true // 已完成
      }
    }
  }

  let mut future = SimpleFuture::new();
  while !future.poll() {
    println!("等待 Future 完成...");
  }
}

// ============================================================================
// 第七部分：错误处理示例
// ============================================================================

/// 演示 Result 和 Option 错误处理
fn error_handling_examples() {
  println!("\n=== 错误处理示例 ===");

  // 1. Option 类型处理空值
  fn find_word(text: &str, word: &str) -> Option<usize> {
    text.find(word)
  }

  let text = "Hello, world!";
  match find_word(text, "world") {
    Some(index) => println!("找到 'world' 在位置: {}", index),
    None => println!("未找到 'world'"),
  }

  // 2. Result 类型处理错误
  fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
      Err(String::from("除零错误"))
    } else {
      Ok(a / b)
    }
  }

  match divide(10.0, 2.0) {
    Ok(result) => println!("10.0 / 2.0 = {}", result),
    Err(e) => println!("错误: {}", e),
  }

  match divide(10.0, 0.0) {
    Ok(result) => println!("10.0 / 0.0 = {}", result),
    Err(e) => println!("错误: {}", e),
  }

  // 3. ? 操作符简化错误传播
  fn read_and_parse() -> Result<i32, Box<dyn std::error::Error>> {
    let text = "42";
    let number: i32 = text.parse()?; // 如果解析失败，自动返回错误
    Ok(number * 2)
  }

  match read_and_parse() {
    Ok(result) => println!("解析并计算结果: {}", result),
    Err(e) => println!("解析错误: {}", e),
  }
}

// ============================================================================
// 第八部分：生命周期示例
// ============================================================================

/// 演示生命周期参数
fn lifetime_examples() {
  println!("\n=== 生命周期示例 ===");

  // 1. 基本生命周期参数
  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
      x
    } else {
      y
    }
  }

  let string1 = String::from("long string is long");
  let string2 = "xyz";

  let result = longest(string1.as_str(), string2);
  println!("最长的字符串是: {}", result);

  // 2. 结构体中的生命周期
  #[derive(Debug)]
  #[allow(dead_code)]
  struct ImportantExcerpt<'a> {
    part: &'a str,
  }

  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find a '.'");
  let excerpt = ImportantExcerpt {
    part: first_sentence,
  };

  println!("重要摘录: {:?}", excerpt);

  // 3. 生命周期省略规则示例
  fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
        return &s[0..i];
      }
    }

    &s[..]
  }

  let sentence = "Hello world";
  let word = first_word(sentence);
  println!("第一个单词: {}", word);
}

// ============================================================================
// 主函数：运行所有示例
// ============================================================================

fn main() {
  println!("🦀 Rust 核心概念详细案例教程 🦀\n");

  variable_binding_examples();
  destructuring_examples();
  ownership_examples();
  borrowing_examples();
  box_examples();
  rc_examples();
  refcell_examples();
  threading_examples();
  message_passing_examples();
  async_simulation_examples();
  error_handling_examples();
  lifetime_examples();

  println!("\n🎉 所有示例运行完成！");
  println!("\n📚 学习建议：");
  println!("1. 逐个运行每个示例函数，理解其输出");
  println!("2. 尝试修改代码，观察编译器的反应");
  println!("3. 阅读编译器错误信息，它们通常很有帮助");
  println!("4. 实践是掌握 Rust 的最佳方式！");
}

// ============================================================================
// 附加：常见模式和最佳实践
// ============================================================================

/// 常见的 Rust 编程模式
#[allow(dead_code)]
mod common_patterns {
  use std::collections::HashMap;

  /// 建造者模式示例
  #[derive(Debug)]
  pub struct Config {
    pub host: String,
    pub port: u16,
    pub timeout: u64,
  }

  impl Config {
    pub fn builder() -> ConfigBuilder {
      ConfigBuilder::default()
    }
  }

  #[derive(Default)]
  pub struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<u64>,
  }

  impl ConfigBuilder {
    pub fn host(mut self, host: &str) -> Self {
      self.host = Some(host.to_string());
      self
    }

    pub fn port(mut self, port: u16) -> Self {
      self.port = Some(port);
      self
    }

    pub fn timeout(mut self, timeout: u64) -> Self {
      self.timeout = Some(timeout);
      self
    }

    pub fn build(self) -> Result<Config, &'static str> {
      Ok(Config {
        host: self.host.unwrap_or_else(|| "localhost".to_string()),
        port: self.port.unwrap_or(8080),
        timeout: self.timeout.unwrap_or(30),
      })
    }
  }

  /// 状态机模式示例
  pub struct StateMachine<State> {
    state: State,
  }

  pub struct Pending;
  pub struct Running;
  pub struct Completed;

  impl StateMachine<Pending> {
    pub fn new() -> Self {
      StateMachine { state: Pending }
    }

    pub fn start(self) -> StateMachine<Running> {
      StateMachine { state: Running }
    }
  }

  impl StateMachine<Running> {
    pub fn complete(self) -> StateMachine<Completed> {
      StateMachine { state: Completed }
    }
  }

  /// 缓存模式示例
  pub struct Cache<K, V> {
    map: HashMap<K, V>,
  }

  impl<K, V> Cache<K, V>
  where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
  {
    pub fn new() -> Self {
      Cache {
        map: HashMap::new(),
      }
    }

    pub fn get_or_insert_with<F>(&mut self, key: K, f: F) -> V
    where
      F: FnOnce() -> V,
    {
      self.map.entry(key).or_insert_with(f).clone()
    }
  }
}
