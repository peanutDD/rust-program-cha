# 🦀 Rust 核心概念详细案例教程

本教程通过具体的代码示例详细介绍 Rust 的核心概念，包括变量绑定、解构、所有权、智能指针、并发编程等重要主题。

## 📋 目录

1. [变量绑定与可变性](#1-变量绑定与可变性)
2. [解构赋值](#2-解构赋值)
3. [所有权系统](#3-所有权系统)
4. [借用与引用](#4-借用与引用)
5. [智能指针](#5-智能指针)
6. [并发编程](#6-并发编程)
7. [异步编程](#7-异步编程)
8. [错误处理](#8-错误处理)
9. [生命周期](#9-生命周期)
10. [常见模式](#10-常见模式)

## 🚀 快速开始

### 运行完整示例

```bash
# 运行所有示例
cargo run --bin rust_concepts_examples

# 或者直接运行 Rust 文件
rustc rust_concepts_examples.rs && ./rust_concepts_examples
```

### 分步学习

建议按照以下顺序学习：

1. **基础概念**：变量绑定 → 解构 → 所有权
2. **内存管理**：借用 → 生命周期 → 智能指针
3. **高级特性**：并发 → 异步 → 错误处理
4. **实践模式**：常见设计模式

---

## 1. 变量绑定与可变性

### 核心概念

- **默认不可变**：Rust 中变量默认是不可变的
- **显式可变**：使用 `mut` 关键字声明可变变量
- **变量遮蔽**：可以用相同名称声明新变量，甚至改变类型

### 关键示例

```rust
// 不可变绑定
let x = 5;
// x = 6; // ❌ 编译错误

// 可变绑定
let mut y = 10;
y = 15; // ✅ 可以修改

// 变量遮蔽
let spaces = "   ";        // &str 类型
let spaces = spaces.len(); // usize 类型，完全不同的变量
```

### 💡 最佳实践

- 默认使用不可变绑定，只在必要时使用 `mut`
- 利用变量遮蔽进行类型转换
- 使用有意义的变量名

---

## 2. 解构赋值

### 核心概念

- **模式匹配**：Rust 强大的模式匹配系统
- **解构语法**：一次性提取复合数据类型的多个值
- **忽略语法**：使用 `_` 和 `..` 忽略不需要的值

### 关键示例

```rust
// 元组解构
let (a, b, c) = (1, 2, 3);
let (first, .., last) = (1, 2, 3, 4, 5);

// 数组解构
let [x, y, z, ..] = [1, 2, 3, 4, 5];

// 结构体解构
struct Point { x: i32, y: i32 }
let Point { x, y } = Point { x: 10, y: 20 };
let Point { x: px, y: py } = point; // 重命名

// 嵌套解构
let Person { name, age, .. } = person; // 忽略其他字段
```

### 💡 最佳实践

- 使用解构简化代码，提高可读性
- 合理使用 `..` 忽略不需要的字段
- 在函数参数中使用解构

---

## 3. 所有权系统

### 核心概念

- **所有权规则**：每个值都有一个所有者，同时只能有一个所有者
- **移动语义**：赋值或传参时转移所有权
- **克隆**：显式深拷贝数据
- **Copy trait**：简单类型的自动复制

### 关键示例

```rust
// 所有权转移
let s1 = String::from("hello");
let s2 = s1; // s1 不再有效
// println!("{}", s1); // ❌ 编译错误

// 克隆
let s3 = String::from("world");
let s4 = s3.clone(); // 深拷贝
println!("{} {}", s3, s4); // ✅ 都有效

// Copy 类型
let x = 5;
let y = x; // 复制，不是移动
println!("{} {}", x, y); // ✅ 都有效
```

### 💡 最佳实践

- 理解何时发生移动，何时发生复制
- 谨慎使用 `clone()`，考虑性能影响
- 优先使用借用而非所有权转移

---

## 4. 借用与引用

### 核心概念

- **不可变借用**：`&T`，可以有多个
- **可变借用**：`&mut T`，同时只能有一个
- **借用规则**：防止数据竞争和悬垂指针

### 关键示例

```rust
// 不可变借用
let s = String::from("hello");
let len = calculate_length(&s); // 借用，不获取所有权
println!("{} {}", s, len); // s 仍然有效

// 可变借用
let mut s = String::from("hello");
change(&mut s);

// 借用规则
let mut s = String::from("hello");
let r1 = &s;     // ✅ 不可变借用
let r2 = &s;     // ✅ 多个不可变借用
// let r3 = &mut s; // ❌ 不能同时有可变和不可变借用
```

### 💡 最佳实践

- 优先使用借用而非所有权转移
- 理解借用检查器的规则
- 合理设计函数签名

---

## 5. 智能指针

### 核心概念

- **Box<T>**：堆分配，独占所有权
- **Rc<T>**：引用计数，共享所有权（单线程）
- **Arc<T>**：原子引用计数，共享所有权（多线程）
- **RefCell<T>**：内部可变性，运行时借用检查
- **Mutex<T>**：互斥锁，线程安全的内部可变性

### 关键示例

```rust
// Box<T> - 堆分配
let b = Box::new(5);

// Rc<T> - 引用计数
use std::rc::Rc;
let a = Rc::new(5);
let b = Rc::clone(&a); // 增加引用计数

// RefCell<T> - 内部可变性
use std::cell::RefCell;
let data = RefCell::new(5);
*data.borrow_mut() = 10; // 运行时可变借用

// Arc<Mutex<T>> - 线程安全共享
use std::sync::{Arc, Mutex};
let counter = Arc::new(Mutex::new(0));
```

### 💡 最佳实践

- 根据使用场景选择合适的智能指针
- 注意 `RefCell` 的运行时开销
- 避免循环引用，使用 `Weak<T>`

---

## 6. 并发编程

### 核心概念

- **线程创建**：`thread::spawn`
- **消息传递**：`mpsc` 通道
- **共享状态**：`Arc<Mutex<T>>`
- **Send/Sync trait**：线程安全标记

### 关键示例

```rust
// 基本线程
use std::thread;
let handle = thread::spawn(|| {
    println!("Hello from thread!");
});
handle.join().unwrap();

// 共享状态
use std::sync::{Arc, Mutex};
let counter = Arc::new(Mutex::new(0));
let counter_clone = Arc::clone(&counter);

thread::spawn(move || {
    let mut num = counter_clone.lock().unwrap();
    *num += 1;
});

// 消息传递
use std::sync::mpsc;
let (tx, rx) = mpsc::channel();
thread::spawn(move || {
    tx.send("Hello").unwrap();
});
let received = rx.recv().unwrap();
```

### 💡 最佳实践

- 优先使用消息传递而非共享状态
- 理解 Send 和 Sync trait
- 避免死锁和数据竞争

---

## 7. 异步编程

### 核心概念

- **Future trait**：表示异步计算
- **async/await**：异步语法糖
- **异步运行时**：如 Tokio
- **Pin 和 Unpin**：内存固定

### 关键示例

```rust
// 基本异步函数
async fn hello_world() {
    println!("Hello, world!");
}

// 异步 HTTP 请求（需要 reqwest）
async fn fetch_data() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://api.example.com/data").await?;
    let text = response.text().await?;
    Ok(text)
}

// 并发执行
use tokio::join;
async fn concurrent_tasks() {
    let task1 = async { /* 任务1 */ };
    let task2 = async { /* 任务2 */ };
    let (result1, result2) = join!(task1, task2);
}
```

### 💡 最佳实践

- 理解异步和并发的区别
- 合理使用 `join!` 和 `select!`
- 避免阻塞异步运行时

---

## 8. 错误处理

### 核心概念

- **Result<T, E>**：可恢复错误
- **Option<T>**：空值处理
- **panic!**：不可恢复错误
- **? 操作符**：错误传播

### 关键示例

```rust
// Result 类型
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// Option 类型
fn find_word(text: &str, word: &str) -> Option<usize> {
    text.find(word)
}

// ? 操作符
fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.parse()?; // 自动传播错误
    Ok(n * 2)
}

// 组合器方法
let result = some_option
    .map(|x| x * 2)
    .and_then(|x| if x > 10 { Some(x) } else { None })
    .unwrap_or(0);
```

### 💡 最佳实践

- 使用 `Result` 处理可恢复错误
- 使用 `Option` 处理空值
- 合理使用 `?` 操作符
- 创建自定义错误类型

---

## 9. 生命周期

### 核心概念

- **生命周期参数**：`'a`，确保引用有效性
- **生命周期省略**：编译器自动推断
- **静态生命周期**：`'static`
- **结构体生命周期**：包含引用的结构体

### 关键示例

```rust
// 基本生命周期参数
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 结构体中的生命周期
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 生命周期省略
fn first_word(s: &str) -> &str { // 编译器自动推断生命周期
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 静态生命周期
let s: &'static str = "I have a static lifetime.";
```

### 💡 最佳实践

- 理解生命周期的作用
- 依赖编译器的生命周期省略
- 只在必要时显式标注生命周期

---

## 10. 常见模式

### 建造者模式

```rust
struct Config {
    host: String,
    port: u16,
    timeout: u64,
}

impl Config {
    fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<u64>,
}

impl ConfigBuilder {
    fn host(mut self, host: &str) -> Self {
        self.host = Some(host.to_string());
        self
    }
    
    fn build(self) -> Result<Config, &'static str> {
        Ok(Config {
            host: self.host.unwrap_or_else(|| "localhost".to_string()),
            port: self.port.unwrap_or(8080),
            timeout: self.timeout.unwrap_or(30),
        })
    }
}

// 使用
let config = Config::builder()
    .host("example.com")
    .port(443)
    .build()?;
```

### 状态机模式

```rust
struct StateMachine<State> {
    state: State,
}

struct Pending;
struct Running;
struct Completed;

impl StateMachine<Pending> {
    fn new() -> Self {
        StateMachine { state: Pending }
    }
    
    fn start(self) -> StateMachine<Running> {
        StateMachine { state: Running }
    }
}

impl StateMachine<Running> {
    fn complete(self) -> StateMachine<Completed> {
        StateMachine { state: Completed }
    }
}
```

---

## 🎯 学习路径建议

### 初学者（0-2周）
1. 变量绑定和基本语法
2. 所有权和借用基础
3. 基本数据类型和控制流

### 进阶者（2-6周）
1. 生命周期和高级借用
2. 智能指针和内存管理
3. 错误处理和模式匹配

### 高级者（6周+）
1. 并发和异步编程
2. 宏编程和元编程
3. 高级类型系统特性

---

## 🔧 实践建议

1. **动手实践**：运行每个示例，修改代码观察结果
2. **阅读错误**：Rust 编译器错误信息非常有帮助
3. **小项目**：用学到的概念构建小项目
4. **社区参与**：参与 Rust 社区讨论
5. **持续学习**：Rust 生态系统在不断发展

---

## 📚 推荐资源

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Rust 程序设计语言（中文版）](https://kaisery.github.io/trpl-zh-cn/)
- [Course.rs](https://course.rs/)

---

## ⚡ 快速参考

### 常用命令

```bash
# 编译并运行
cargo run --bin rust_concepts_examples

# 检查代码
cargo check

# 格式化代码
cargo fmt

# 运行测试
cargo test

# 生成文档
cargo doc --open
```

### 常用宏

```rust
println!("Hello, {}!", name);     // 打印
dbg!(variable);                   // 调试打印
vec![1, 2, 3];                   // 创建向量
format!("Hello, {}!", name);      // 格式化字符串
panic!("Something went wrong!");  // 恐慌
```

---

**Happy Coding! 🦀**