// Rust变量绑定与解构示例代码集合 - 优化版本
// 本文件展示了更深入的Rust概念和最佳实践
// 包含性能优化、错误处理、生命周期管理等高级特性

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::sync::Arc;

// 自定义错误类型
#[derive(Debug)]
enum AppError {
  DivisionByZero,
  InvalidInput(String),
  #[allow(dead_code)]
  NetworkError {
    code: u16,
    message: String,
  },
}

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      AppError::DivisionByZero => write!(f, "除零错误"),
      AppError::InvalidInput(msg) => write!(f, "无效输入: {}", msg),
      AppError::NetworkError { code, message } => {
        write!(f, "网络错误 {}: {}", code, message)
      }
    }
  }
}

impl Error for AppError {}

type Result<T> = std::result::Result<T, AppError>;

fn main() -> Result<()> {
  println!("=== Rust变量绑定与解构示例 - 优化版本 ===\n");

  // 使用Result处理可能的错误
  basic_variable_binding()?;
  ownership_and_smart_pointers()?;
  advanced_borrowing_patterns()?;
  tuple_and_array_destructuring()?;
  struct_destructuring_advanced()?;
  enum_pattern_matching_enhanced()?;
  advanced_destructuring_patterns()?;
  practical_examples_optimized()?;
  lifetime_and_generic_examples()?;

  Ok(())
}

// 1. 基本变量绑定 - 增强版
fn basic_variable_binding() -> Result<()> {
  println!("=== 基本变量绑定与类型推断 ===");

  // 类型推断与显式类型标注
  let x: i32 = 5; // 显式类型标注
  let mut y = 10_u64; // 类型后缀
  let z = String::from("hello"); // 自动类型推断

  // 常量与静态变量
  const MAX_SIZE: usize = 1000;
  static GLOBAL_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

  println!("x: {}, y: {}, z: {}", x, y, z);
  println!("常量 MAX_SIZE: {}", MAX_SIZE);
  println!(
    "全局计数器: {}",
    GLOBAL_COUNTER.load(std::sync::atomic::Ordering::Relaxed)
  );

  // 变量遮蔽(shadowing)
  let x = x as f64; // 类型转换
  let x = x * 2.5; // 重新绑定
  println!("遮蔽后的x: {}", x);

  y += 10;
  println!("修改后的y: {}", y);

  // 解构赋值
  let (a, b, c) = (1, 2, 3);
  println!("解构赋值: a={}, b={}, c={}", a, b, c);

  println!();
  Ok(())
}

// 2. 所有权与智能指针
fn ownership_and_smart_pointers() -> Result<()> {
  println!("=== 所有权与智能指针 ===");

  // 基本所有权
  let s1 = String::from("hello");
  let s2 = s1.clone(); // 显式克隆
  println!("s1: {}, s2: {}", s1, s2);

  // Rc - 引用计数智能指针
  let data = Rc::new(String::from("共享数据"));
  let data1 = Rc::clone(&data);
  let data2 = Rc::clone(&data);
  println!("Rc引用计数: {}", Rc::strong_count(&data));
  println!("data1: {}, data2: {}", data1, data2);

  // Arc - 原子引用计数(线程安全)
  let shared_data = Arc::new(vec![1, 2, 3, 4, 5]);
  let shared_clone = Arc::clone(&shared_data);
  println!("Arc共享数据: {:?}", shared_clone);

  // Box - 堆分配
  let boxed_value = Box::new(42);
  println!("Box值: {}", boxed_value);

  println!();
  Ok(())
}

// 3. 高级借用模式
fn advanced_borrowing_patterns() -> Result<()> {
  println!("=== 高级借用模式 ===");

  let mut data = vec![1, 2, 3, 4, 5];

  // 分割借用
  let (left, right) = data.split_at_mut(2);
  left[0] = 10;
  right[0] = 20;
  println!("分割借用后: {:?}", data);

  // 内部可变性 - Cell和RefCell
  use std::cell::RefCell;
  let cell_data = RefCell::new(vec![1, 2, 3]);
  {
    let mut borrowed = cell_data.borrow_mut();
    borrowed.push(4);
  } // 借用在此处结束
  println!("RefCell数据: {:?}", cell_data.borrow());

  // 生命周期省略规则示例
  fn get_first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
  }

  let sentence = "Hello world";
  let first = get_first_word(sentence);
  println!("第一个单词: {}", first);

  println!();
  Ok(())
}

// 4. 元组和数组解构增强
fn tuple_and_array_destructuring() -> Result<()> {
  println!("=== 元组和数组解构增强 ===");

  // 嵌套元组解构
  let nested = ((1, 2), (3, 4), (5, 6));
  let ((a, b), (c, d), (e, f)) = nested;
  println!(
    "嵌套解构: a={}, b={}, c={}, d={}, e={}, f={}",
    a, b, c, d, e, f
  );

  // 数组模式匹配
  let array = [1, 2, 3, 4, 5];
  match array {
    [first, .., last] => println!("首尾元素: {} 和 {}", first, last),
  }

  // 切片模式匹配
  let slice = &array[1..4];
  match slice {
    [x, y, z] => println!("切片三元素: {}, {}, {}", x, y, z),
    _ => println!("切片长度不是3"),
  }

  println!();
  Ok(())
}

// 5. 高级结构体解构
#[derive(Debug, Clone)]
struct Point3D {
  x: f64,
  y: f64,
  z: f64,
}

#[derive(Debug)]
struct Sphere {
  center: Point3D,
  radius: f64,
  material: Material,
}

#[derive(Debug)]
enum Material {
  Metal {
    reflectivity: f64,
  },
  #[allow(dead_code)]
  Glass {
    transparency: f64,
  },
  #[allow(dead_code)]
  Plastic {
    color: (u8, u8, u8),
  },
}

fn struct_destructuring_advanced() -> Result<()> {
  println!("=== 高级结构体解构 ===");

  let sphere = Sphere {
    center: Point3D {
      x: 1.0,
      y: 2.0,
      z: 3.0,
    },
    radius: 5.0,
    material: Material::Metal { reflectivity: 0.8 },
  };

  // 深度嵌套解构
  match sphere {
    Sphere {
      center: Point3D { x, y, z },
      radius,
      material: Material::Metal { reflectivity },
    } => {
      println!(
        "金属球体: 中心({}, {}, {}), 半径: {}, 反射率: {}",
        x, y, z, radius, reflectivity
      );
    }
    Sphere {
      material: Material::Glass { transparency },
      ..
    } => {
      println!("玻璃球体，透明度: {}", transparency);
    }
    Sphere {
      material: Material::Plastic { color: (r, g, b) },
      ..
    } => {
      println!("塑料球体，颜色: RGB({}, {}, {})", r, g, b);
    }
  }

  println!();
  Ok(())
}

// 6. 增强的枚举与模式匹配
#[derive(Debug, Clone)]
enum Command {
  Move { x: i32, y: i32, speed: f32 },
  Rotate { angle: f32, axis: Axis },
  Scale { factor: f32 },
  Batch(Vec<Command>),
}

#[derive(Debug, Clone)]
enum Axis {
  #[allow(dead_code)]
  X,
  #[allow(dead_code)]
  Y,
  Z,
}

impl Command {
  fn execute(&self) -> Result<()> {
    match self {
      Command::Move { x, y, speed } => {
        if *speed <= 0.0 {
          return Err(AppError::InvalidInput("速度必须大于0".to_string()));
        }
        println!("移动到 ({}, {})，速度: {}", x, y, speed);
      }
      Command::Rotate { angle, axis } => {
        println!("绕{:?}轴旋转 {}度", axis, angle);
      }
      Command::Scale { factor } => {
        if *factor <= 0.0 {
          return Err(AppError::InvalidInput("缩放因子必须大于0".to_string()));
        }
        println!("缩放 {}倍", factor);
      }
      Command::Batch(commands) => {
        println!("执行批量命令:");
        for cmd in commands {
          cmd.execute()?;
        }
      }
    }
    Ok(())
  }
}

fn enum_pattern_matching_enhanced() -> Result<()> {
  println!("=== 增强的枚举与模式匹配 ===");

  let commands = vec![
    Command::Move {
      x: 10,
      y: 20,
      speed: 5.0,
    },
    Command::Rotate {
      angle: 90.0,
      axis: Axis::Z,
    },
    Command::Scale { factor: 2.0 },
    Command::Batch(vec![
      Command::Move {
        x: 0,
        y: 0,
        speed: 1.0,
      },
      Command::Scale { factor: 0.5 },
    ]),
  ];

  for cmd in &commands {
    if let Err(e) = cmd.execute() {
      eprintln!("命令执行失败: {}", e);
    }
  }

  println!();
  Ok(())
}

// 7. 高级解构模式
fn advanced_destructuring_patterns() -> Result<()> {
  println!("=== 高级解构模式 ===");

  // 范围模式
  let numbers = vec![1, 5, 10, 15, 25, 50, 100];
  for &num in &numbers {
    match num {
      1..=10 => println!("{} 是小数", num),
      11..=50 => println!("{} 是中数", num),
      51..=100 => println!("{} 是大数", num),
      _ => println!("{} 超出范围", num),
    }
  }

  // 守卫模式与@绑定
  let point = Point3D {
    x: 3.0,
    y: 4.0,
    z: 0.0,
  };
  match point {
    Point3D { x, y, z: 0.0 } if x.powi(2) + y.powi(2) == 25.0 => {
      println!("在XY平面上的单位圆上的点: ({}, {})", x, y);
    }
    Point3D { z, .. } if z > 0.0 => {
      println!("在Z轴正方向的点");
    }
    p @ Point3D { x, y, z } if x == y && y == z => {
      println!("对角线上的点: {:?}", p);
    }
    _ => println!("普通点"),
  }

  println!();
  Ok(())
}

// 8. 优化的实际应用示例
#[derive(Debug)]
struct ApiResponse<T> {
  data: Option<T>,
  status: u16,
  message: String,
}

#[derive(Debug)]
struct User {
  #[allow(dead_code)]
  id: u64,
  name: String,
  email: String,
}

fn practical_examples_optimized() -> Result<()> {
  println!("=== 优化的实际应用示例 ===");

  // 使用泛型的API响应处理
  let responses = vec![
    ApiResponse {
      data: Some(User {
        id: 1,
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
      }),
      status: 200,
      message: "成功".to_string(),
    },
    ApiResponse::<User> {
      data: None,
      status: 404,
      message: "用户未找到".to_string(),
    },
  ];

  for response in responses {
    match response {
      ApiResponse {
        data: Some(user),
        status: 200,
        ..
      } => {
        println!("✅ 用户加载成功: {} ({})", user.name, user.email);
      }
      ApiResponse {
        status: 404,
        message,
        ..
      } => {
        println!("❌ 资源未找到: {}", message);
      }
      ApiResponse {
        status, message, ..
      } => {
        println!("⚠️ 其他状态 {}: {}", status, message);
      }
    }
  }

  // 高效的Option和Result链式操作
  let numbers = vec!["42", "not_a_number", "100"];
  let parsed_numbers: Vec<i32> = numbers.iter().filter_map(|s| s.parse().ok()).collect();
  println!("解析的数字: {:?}", parsed_numbers);

  // 使用?操作符的错误传播
  fn safe_divide(a: f64, b: f64) -> Result<f64> {
    if b == 0.0 {
      Err(AppError::DivisionByZero)
    } else {
      Ok(a / b)
    }
  }

  match safe_divide(10.0, 3.0) {
    Ok(result) => println!("除法结果: {:.2}", result),
    Err(e) => println!("错误: {}", e),
  }

  println!();
  Ok(())
}

// 9. 生命周期与泛型示例
fn lifetime_and_generic_examples() -> Result<()> {
  println!("=== 生命周期与泛型示例 ===");

  // 泛型函数
  fn find_max<T: Ord + Copy>(slice: &[T]) -> Option<T> {
    slice.iter().max().copied()
  }

  let numbers = [1, 5, 3, 9, 2];
  if let Some(max) = find_max(&numbers) {
    println!("最大值: {}", max);
  }

  // 生命周期参数
  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
      x
    } else {
      y
    }
  }

  let str1 = "hello";
  let str2 = "world!";
  let result = longest(str1, str2);
  println!("更长的字符串: {}", result);

  // 使用HashMap进行高效查找
  let mut scores = HashMap::new();
  scores.insert("Alice", 95);
  scores.insert("Bob", 87);
  scores.insert("Charlie", 92);

  // 模式匹配HashMap查找
  match scores.get("Alice") {
    Some(&score) => println!("Alice的分数: {}", score),
    None => println!("未找到Alice的分数"),
  }

  // 使用entry API进行高效更新
  scores.entry("David").or_insert(0);
  *scores.entry("Alice").or_insert(0) += 5;

  println!("最终分数: {:?}", scores);

  println!();
  Ok(())
}

// 性能优化技巧
#[allow(dead_code)]
mod performance_tips {
  // 避免不必要的克隆
  pub fn efficient_string_processing(data: &[String]) -> Vec<&str> {
    data
      .iter()
      .filter(|s| !s.is_empty())
      .map(|s| s.as_str()) // 使用引用而不是克隆
      .collect()
  }

  // 使用迭代器而不是索引
  pub fn sum_even_numbers(numbers: &[i32]) -> i32 {
    numbers.iter().filter(|&&n| n % 2 == 0).sum()
  }

  // 预分配容量
  pub fn create_large_vec(size: usize) -> Vec<i32> {
    let mut vec = Vec::with_capacity(size); // 预分配
    for i in 0..size {
      vec.push(i as i32);
    }
    vec
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_safe_divide() {
    fn safe_divide(a: f64, b: f64) -> Result<f64> {
      if b == 0.0 {
        Err(AppError::DivisionByZero)
      } else {
        Ok(a / b)
      }
    }

    assert!(safe_divide(10.0, 2.0).is_ok());
    assert!(safe_divide(10.0, 0.0).is_err());
  }

  #[test]
  fn test_command_execution() {
    let cmd = Command::Move {
      x: 10,
      y: 20,
      speed: 5.0,
    };
    assert!(cmd.execute().is_ok());

    let invalid_cmd = Command::Move {
      x: 10,
      y: 20,
      speed: -1.0,
    };
    assert!(invalid_cmd.execute().is_err());
  }
}
