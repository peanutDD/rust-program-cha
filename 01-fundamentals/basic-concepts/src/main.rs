//! # Rust 变量绑定与解构示例代码集合
//!
//! 本文件包含了技术文档中的核心示例代码，展示了 Rust 的基础概念。
//! 所有示例都经过性能优化，使用引用避免不必要的克隆和移动。
//!
//! ## 性能优化要点
//! - 使用 `&str` 而不是 `String` 用于字符串字面量
//! - 使用引用传递避免所有权转移
//! - 使用引用迭代保留原始数据可用性
//! - 使用 `&'static str` 用于错误消息避免内存分配


fn main() {
  println!("=== Rust变量绑定与解构示例 ===\n");

  // 1. 基本变量绑定示例
  basic_variable_binding();

  // 2. 所有权与移动语义示例
  ownership_examples();

  // 3. 借用机制示例
  borrowing_examples();

  // 4. 元组解构示例
  tuple_destructuring();

  // 5. 结构体解构示例
  struct_destructuring();

  // 6. 枚举与模式匹配示例
  enum_pattern_matching();

  // 7. 高级解构示例
  advanced_destructuring();

  // 8. 实际应用示例
  practical_examples();
}

// 1. 基本变量绑定
fn basic_variable_binding() {
  println!("=== 基本变量绑定 ===");

  let x = 5; // 不可变绑定
  let mut y = 10; // 可变绑定
  let z = "hello"; // 字符串字面量（&str 类型，存储在栈上）

  println!("x: {}, y: {}, z: {}", x, y, z);

  y = 20; // 修改可变变量
  println!("修改后的y: {}", y);

  println!();
}

// 2. 所有权与移动语义
fn ownership_examples() {
  println!("=== 所有权与移动语义 ===");

  // 移动语义示例
  let s1 = String::from("hello"); // 堆分配字符串
  let s2 = s1; // s1的所有权移动到s2
               // println!("{}", s1);  // 编译错误：s1 已被移动
  println!("s2: {}", s2);

  // 复制语义示例
  let x = 5;
  let y = x; // 复制语义，x仍然有效
  println!("x: {}, y: {}", x, y);

  println!();
}

// 3. 借用机制
fn borrowing_examples() {
  println!("=== 借用机制 ===");

  let s = String::from("hello");
  let r1 = &s; // 不可变借用（借用，不移动）
  let r2 = &s; // 可以有多个不可变借用
  println!("r1: {}, r2: {}", r1, r2);
  // s 仍然可用，因为只是借用

  let mut s2 = String::from("world");
  {
    let r3 = &mut s2; // 可变借用（独占借用）
    r3.push_str("!");
    println!("r3: {}", r3);
  } // r3 作用域结束，可变借用结束
  println!("s2 现在可以访问: {}", s2);

  println!();
}

// 4. 元组解构
fn tuple_destructuring() {
  println!("=== 元组解构 ===");

  let coordinates = (3.14, 2.71, 1.41);

  // 基本解构
  let (x, y, z) = coordinates;
  println!("x: {}, y: {}, z: {}", x, y, z);

  // 忽略某些元素
  let (first, _, third) = coordinates;
  println!("first: {}, third: {}", first, third);

  // 使用剩余模式
  let (head, ..) = coordinates;
  println!("head: {}", head);

  println!();
}

// 5. 结构体解构
#[derive(Debug)]
struct Point {
  x: f64,
  y: f64,
}

#[derive(Debug)]
struct Rectangle {
  top_left: Point,
  bottom_right: Point,
}

fn struct_destructuring() {
  println!("=== 结构体解构 ===");

  let rect = Rectangle {
    top_left: Point { x: 0.0, y: 1.0 },
    bottom_right: Point { x: 3.0, y: 0.0 },
  };

  // 基本结构体解构
  let Rectangle {
    top_left,
    bottom_right,
  } = &rect;
  println!("Top left: {:?}, Bottom right: {:?}", top_left, bottom_right);

  // 嵌套解构与重命名
  let Rectangle {
    top_left: Point { x: x1, y: y1 },
    bottom_right: Point { x: x2, y: y2 },
  } = rect;
  println!("Coordinates: ({}, {}) to ({}, {})", x1, y1, x2, y2);

  println!();
}

// 6. 枚举与模式匹配
#[derive(Debug)]
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn process(&self) {
    match self {
      Message::Quit => {
        println!("Quitting application");
      }
      Message::Move { x, y } => {
        println!("Moving to coordinates ({}, {})", x, y);
      }
      Message::Write(text) => {
        println!("Writing text: {}", text);
      }
      Message::ChangeColor(r, g, b) => {
        println!("Changing color to RGB({}, {}, {})", r, g, b);
      }
    }
  }
}

fn enum_pattern_matching() {
  println!("=== 枚举与模式匹配 ===");

  let messages = vec![
    Message::Quit,
    Message::Move { x: 10, y: 20 },
    Message::Write("Hello".to_string()), // 使用 to_string() 而不是 String::from()
    Message::ChangeColor(255, 0, 0),
  ];

  // 使用引用迭代，避免移动 messages
  for msg in &messages {
    msg.process();
  }
  
  // messages 仍然可用
  println!("消息数量: {}", messages.len());

  println!();
}

// 7. 高级解构示例
fn advanced_destructuring() {
  println!("=== 高级解构示例 ===");

  // 数组解构
  let array = [1, 2, 3, 4, 5];
  let [first, second, ..] = array;
  println!("Array - First: {}, Second: {}", first, second);

  // 匹配守卫
  let number = 15;
  match number {
    n if n < 0 => println!("Negative: {}", n),
    0 => println!("Zero"),
    n if n <= 10 => println!("Small positive: {}", n),
    n => println!("Large positive: {}", n),
  }

  // @绑定模式
  let value = 7;
  match value {
    n @ 1..=5 => println!("Small number: {}", n),
    n @ 6..=10 => println!("Medium number: {}", n),
    _ => println!("Large number"),
  }

  println!();
}

// 8. 实际应用示例
#[derive(Debug)]
enum HttpResponse {
  Ok { data: String, status: u16 },
  NotFound { message: String },
  ServerError { error: String, code: u16 },
}

/// 处理 HTTP 响应，使用引用避免移动
fn handle_response(response: &HttpResponse) {
  match response {
    HttpResponse::Ok { data, status } => {
      println!("✅ Success ({}): {}", status, data);
    }
    HttpResponse::NotFound { message } => {
      println!("❌ Not Found: {}", message);
    }
    HttpResponse::ServerError { error, code } => {
      println!("🔥 Server Error ({}): {}", code, error);
    }
  }
}

fn practical_examples() {
  println!("=== 实际应用示例 ===");

  let responses = vec![
    HttpResponse::Ok {
      data: "User data loaded".to_string(),
      status: 200,
    },
    HttpResponse::NotFound {
      message: "User not found".to_string(),
    },
    HttpResponse::ServerError {
      error: "Database connection failed".to_string(),
      code: 500,
    },
  ];

  // 使用引用迭代，避免移动
  for response in &responses {
    handle_response(response);
  }

  // Option和Result的实际应用
  let data = vec![1, 2, 3, 4, 5];

  match data.get(2) {
    Some(value) => println!("Found value at index 2: {}", value),
    None => println!("Index 2 is out of bounds"),
  }

  // 安全的除法操作 - 使用 &str 避免不必要的 String 分配
  fn safe_divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
      Err("Division by zero")
    } else {
      Ok(a / b)
    }
  }

  match safe_divide(10.0, 3.0) {
    Ok(result) => println!("Division result: {:.2}", result),
    Err(error) => println!("Error: {}", error),
  }

  println!();
}

// 额外的辅助函数示例

// 处理复杂嵌套数据结构
#[derive(Debug)]
struct Config {
  database: DatabaseConfig,
  server: ServerConfig,
}

#[derive(Debug)]
struct DatabaseConfig {
  host: String,
  port: u16,
  credentials: Option<Credentials>,
}

#[derive(Debug)]
struct ServerConfig {
  #[allow(dead_code)]
  bind_address: String,
  worker_count: usize,
}

#[derive(Debug)]
struct Credentials {
  username: String,
  #[allow(dead_code)]
  password: String,
}

#[allow(dead_code)]
fn process_config(config: Config) {
  match config {
    Config {
      database:
        DatabaseConfig {
          host,
          port,
          credentials: Some(Credentials { username, .. }),
        },
      server: ServerConfig { worker_count, .. },
    } => {
      println!("✅ Connecting to {}:{} as {}", host, port, username);
      println!("🚀 Starting {} workers", worker_count);
    }
    Config {
      database: DatabaseConfig {
        credentials: None, ..
      },
      ..
    } => {
      println!("❌ No database credentials provided");
    }
  }
}

// 函数式编程风格的解构应用 - 优化版本
#[allow(dead_code)]
fn functional_style_example() {
  let numbers = vec![1, -2, 3, -4, 5];

  // 使用引用迭代，避免移动
  let processed: Vec<String> = numbers
    .iter()
    .filter_map(|&n| match n {
      x if x > 0 => Some(format!("positive: {}", x)),
      0 => Some("zero".to_string()),
      _ => None,
    })
    .collect();

  println!("Processed numbers: {:?}", processed);
  // numbers 仍然可用
  println!("Original numbers: {:?}", numbers);
}

// 性能优化示例 - 避免不必要的移动
#[allow(dead_code)]
fn efficient_pattern_matching(data: &[String]) {
  for item in data {
    match item.as_str() {
      // 使用as_str()避免移动
      "start" => println!("🟢 Starting process"),
      "stop" => println!("🔴 Stopping process"),
      _ => println!("➡️ Processing: {}", item),
    }
  }
}
