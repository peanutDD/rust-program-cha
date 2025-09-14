//! # Rust 语句与表达式详解
//!
//! 本文件详细讲解 Rust 中语句(Statement)与表达式(Expression)的概念、区别和使用方法
//! 基于 https://course.rs/basic/base-type/statement-expression.html 的内容进行扩展

/// # 1. 基本概念
///
/// ## 1.1 语句 (Statement)
/// 语句是执行一些操作但不返回值的指令
/// Rust 中有两种语句：
/// - 声明语句 (Declaration Statement)
/// - 表达式语句 (Expression Statement)
///
/// ## 1.2 表达式 (Expression)
/// 表达式会计算并产生一个值
/// 表达式是函数式编程的重要特征
pub fn basic_concepts_demo() {
  println!("=== 基本概念演示 ===");

  // 语句示例：let 是声明语句，不返回值
  let x = 5; // 这是一个语句

  // 表达式示例：5 是一个表达式，返回值 5
  let y = {
    let inner = 10;
    inner * 2 // 这是表达式，没有分号，返回值 20
  };

  println!("x = {}, y = {}", x, y);

  // 错误示例：不能将语句赋值给变量
  // let z = (let a = 8); // 编译错误！let 是语句，不返回值
}

/// # 2. 声明语句详解
///
/// ## 2.1 变量声明语句
/// 使用 let 关键字进行变量绑定
pub fn declaration_statements_demo() {
  println!("\n=== 声明语句演示 ===");

  // 基本变量声明
  let a = 8;
  let b: Vec<f64> = Vec::new();
  let (c, d) = ("hello", true);

  println!("a = {}, c = {}, d = {}", a, c, d);

  // 可变变量声明
  let mut counter = 0;
  counter += 1;
  println!("counter = {}", counter);

  // 类型注解
  let explicit_type: i32 = 42;
  let inferred_type = 42; // 编译器推断为 i32

  println!("explicit: {}, inferred: {}", explicit_type, inferred_type);
}

/// ## 2.2 Item 声明语句
/// 函数、结构体、类型别名、静态变量、特质、实现或模块的声明
pub fn item_declarations_demo() {
  println!("\n=== Item 声明演示 ===");

  // 函数声明
  fn inner_function(x: i32) -> i32 {
    x * 2
  }

  // 结构体声明
  struct Point {
    x: f64,
    y: f64,
  }

  // 类型别名声明
  type UserId = u64;

  // 使用声明的项
  let result = inner_function(5);
  let point = Point { x: 1.0, y: 2.0 };
  let user_id: UserId = 12345;

  println!("inner_function(5) = {}", result);
  println!("point = ({}, {})", point.x, point.y);
  println!("user_id = {}", user_id);
}

/// # 3. 表达式详解
///
/// ## 3.1 字面量表达式
pub fn literal_expressions_demo() {
  println!("\n=== 字面量表达式演示 ===");

  // 各种字面量表达式
  let unit = (); // 单元类型
  let string = "hello"; // 字符串字面量
  let character = '1'; // 字符字面量
  let integer = 15; // 整数字面量
  let float = 3.14; // 浮点数字面量
  let boolean = true; // 布尔字面量

  println!("unit: {:?}", unit);
  println!("string: {}", string);
  println!("character: {}", character);
  println!("integer: {}", integer);
  println!("float: {}", float);
  println!("boolean: {}", boolean);
}

/// ## 3.2 元组表达式
pub fn tuple_expressions_demo() {
  println!("\n=== 元组表达式演示 ===");

  // 多元素元组
  let tuple1 = (0.0, 4.5);
  let tuple2 = ("a", 4usize, true);

  // 单元素元组（需要逗号）
  let single_element = (42,); // 单元素元组
  let not_tuple = (42); // 这只是括号中的数字，不是元组

  println!("tuple1: {:?}", tuple1);
  println!("tuple2: {:?}", tuple2);
  println!("single_element: {:?}", single_element);
  println!("not_tuple: {:?}", not_tuple);

  // 元组解构
  let (x, y) = tuple1;
  println!("解构后: x = {}, y = {}", x, y);
}

/// ## 3.3 结构体表达式
pub fn struct_expressions_demo() {
  println!("\n=== 结构体表达式演示 ===");

  // 定义结构体
  #[derive(Debug)]
  struct Point {
    x: f64,
    y: f64,
  }

  #[derive(Debug)]
  struct TuplePoint(f64, f64);

  // 结构体表达式
  let point1 = Point { x: 10.0, y: 20.0 };
  let tuple_point = TuplePoint(10.0, 20.0);

  // 基于现有结构体创建新结构体
  let point2 = Point { x: 5.0, ..point1 };

  println!("point1: {:?}", point1);
  println!("tuple_point: {:?}", tuple_point);
  println!("point2: {:?}", point2);
}

/// ## 3.4 块表达式
pub fn block_expressions_demo() {
  println!("\n=== 块表达式演示 ===");

  // 块表达式返回最后一个表达式的值
  let x: i32 = {
    println!("计算中...");
    let a = 5;
    let b = 10;
    a + b // 没有分号，这是表达式，返回 15
  };

  // 块表达式以语句结尾，返回 ()
  let y: () = {
    println!("执行一些操作");
    let _temp = 42;
    // 没有返回表达式，返回 ()
  };

  // 复杂的块表达式
  let result = {
    let base = 10;
    let multiplier = 2;
    let intermediate = base * multiplier;

    if intermediate > 15 {
      intermediate + 5
    } else {
      intermediate - 5
    }
  };

  println!("x = {}", x);
  println!("y = {:?}", y);
  println!("result = {}", result);
}

/// ## 3.5 范围表达式
pub fn range_expressions_demo() {
  println!("\n=== 范围表达式演示 ===");

  // 各种范围表达式
  let range1 = 1..5; // 1, 2, 3, 4 (不包含 5)
  let range2 = 1..=5; // 1, 2, 3, 4, 5 (包含 5)
  let range3 = 3..; // 从 3 开始的无限范围
  let range4 = ..4; // 到 4 结束的范围
  let range5 = ..; // 全范围

  println!("range1 包含的元素:");
  for i in range1 {
    print!("{} ", i);
  }
  println!();

  println!("range2 包含的元素:");
  for i in range2 {
    print!("{} ", i);
  }
  println!();

  // 使用范围进行切片
  let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  let slice1 = &arr[2..5]; // [3, 4, 5]
  let slice2 = &arr[..3]; // [1, 2, 3]
  let slice3 = &arr[7..]; // [8, 9, 10]

  println!("slice1: {:?}", slice1);
  println!("slice2: {:?}", slice2);
  println!("slice3: {:?}", slice3);
}

/// ## 3.6 if 表达式
pub fn if_expressions_demo() {
  println!("\n=== if 表达式演示 ===");

  let number = 6;

  // if 作为表达式
  let description = if number % 2 == 0 { "偶数" } else { "奇数" };

  println!("{} 是 {}", number, description);

  // 复杂的 if 表达式
  let category = if number < 0 {
    "负数"
  } else if number == 0 {
    "零"
  } else if number < 10 {
    "个位数"
  } else {
    "多位数"
  };

  println!("{} 属于: {}", number, category);

  // if let 表达式
  let optional_value = Some(42);
  let result = if let Some(value) = optional_value {
    value * 2
  } else {
    0
  };

  println!("if let 结果: {}", result);
}

/// ## 3.7 match 表达式
pub fn match_expressions_demo() {
  println!("\n=== match 表达式演示 ===");

  let number = 3;

  // 基本 match 表达式
  let description = match number {
    1 => "一",
    2 => "二",
    3 => "三",
    4 | 5 => "四或五",
    6..=10 => "六到十",
    _ => "其他",
  };

  println!("{} 对应: {}", number, description);

  // match 与 Option
  let optional = Some("hello");
  let result = match optional {
    Some(value) => format!("值是: {}", value),
    None => "没有值".to_string(),
  };

  println!("Option 匹配结果: {}", result);

  // match 与结构体
  #[derive(Debug)]
  enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
  }

  let msg = Message::Move { x: 10, y: 20 };

  let response = match msg {
    Message::Quit => "退出".to_string(),
    Message::Move { x, y } => format!("移动到 ({}, {})", x, y),
    Message::Write(text) => format!("写入: {}", text),
    Message::ChangeColor(r, g, b) => format!("改变颜色为 RGB({}, {}, {})", r, g, b),
  };

  println!("消息处理: {}", response);
}

/// ## 3.8 数组表达式
pub fn array_expressions_demo() {
  println!("\n=== 数组表达式演示 ===");

  // 数组字面量
  let arr1 = [1, 2, 3, 4, 5];
  let arr2 = ["a", "b", "c"];

  // 重复元素的数组
  let arr3 = [0; 5]; // [0, 0, 0, 0, 0]
  let arr4 = ["default"; 3]; // ["default", "default", "default"]

  println!("arr1: {:?}", arr1);
  println!("arr2: {:?}", arr2);
  println!("arr3: {:?}", arr3);
  println!("arr4: {:?}", arr4);

  // 数组索引表达式
  let first = arr1[0];
  let last = arr1[arr1.len() - 1];

  println!("第一个元素: {}, 最后一个元素: {}", first, last);
}

/// ## 3.9 运算符表达式
pub fn operator_expressions_demo() {
  println!("\n=== 运算符表达式演示 ===");

  let a = 10;
  let b = 3;

  // 算术运算符表达式
  let add = a + b; // 加法
  let sub = a - b; // 减法
  let mul = a * b; // 乘法
  let div = a / b; // 除法
  let rem = a % b; // 取余

  println!("算术运算: {} + {} = {}", a, b, add);
  println!("算术运算: {} - {} = {}", a, b, sub);
  println!("算术运算: {} * {} = {}", a, b, mul);
  println!("算术运算: {} / {} = {}", a, b, div);
  println!("算术运算: {} % {} = {}", a, b, rem);

  // 比较运算符表达式
  let eq = a == b; // 相等
  let ne = a != b; // 不等
  let gt = a > b; // 大于
  let lt = a < b; // 小于
  let ge = a >= b; // 大于等于
  let le = a <= b; // 小于等于

  println!("比较运算: {} == {} = {}", a, b, eq);
  println!("比较运算: {} != {} = {}", a, b, ne);
  println!("比较运算: {} > {} = {}", a, b, gt);
  println!("比较运算: {} < {} = {}", a, b, lt);
  println!("比较运算: {} >= {} = {}", a, b, ge);
  println!("比较运算: {} <= {} = {}", a, b, le);

  // 逻辑运算符表达式
  let x = true;
  let y = false;

  let and = x && y; // 逻辑与
  let or = x || y; // 逻辑或
  let not = !x; // 逻辑非

  println!("逻辑运算: {} && {} = {}", x, y, and);
  println!("逻辑运算: {} || {} = {}", x, y, or);
  println!("逻辑运算: !{} = {}", x, not);

  // 位运算符表达式
  let bit_and = a & b; // 按位与
  let bit_or = a | b; // 按位或
  let bit_xor = a ^ b; // 按位异或
  let bit_not = !a; // 按位取反
  let left_shift = a << 1; // 左移
  let right_shift = a >> 1; // 右移

  println!("位运算: {} & {} = {}", a, b, bit_and);
  println!("位运算: {} | {} = {}", a, b, bit_or);
  println!("位运算: {} ^ {} = {}", a, b, bit_xor);
  println!("位运算: !{} = {}", a, bit_not);
  println!("位运算: {} << 1 = {}", a, left_shift);
  println!("位运算: {} >> 1 = {}", a, right_shift);
}

/// # 4. 表达式语句
/// 表达式后面加分号就变成了表达式语句
pub fn expression_statements_demo() {
  println!("\n=== 表达式语句演示 ===");

  // 表达式变成语句
  5; // 表达式语句，值被丢弃
  "hello"; // 表达式语句，值被丢弃
  true; // 表达式语句，值被丢弃

  // 函数调用表达式语句
  println!("这是一个表达式语句");

  // 赋值表达式语句
  let mut x = 10;
  x += 5; // 赋值表达式语句

  println!("x = {}", x);

  // 块表达式语句
  {
    let temp = 42;
    println!("临时值: {}", temp);
  }; // 注意这里的分号，使块变成语句
}

/// # 5. 语句与表达式的区别和转换
pub fn statement_vs_expression_demo() {
  println!("\n=== 语句与表达式的区别演示 ===");

  // 表达式返回值
  let value_from_expression = {
    let a = 5;
    let b = 10;
    a + b // 表达式，返回 15
  };

  // 语句不返回值（返回单元类型 ()）
  let value_from_statement = {
    let a = 5;
    let b = 10;
    a + b; // 语句（有分号），返回 ()
  };

  println!("表达式的值: {}", value_from_expression);
  println!("语句的值: {:?}", value_from_statement);

  // 函数中的表达式和语句
  fn returns_expression() -> i32 {
    42 // 表达式，作为返回值
  }

  fn returns_unit() {
    42; // 语句，函数返回 ()
  }

  let expr_result = returns_expression();
  let stmt_result = returns_unit();

  println!("表达式函数返回: {}", expr_result);
  println!("语句函数返回: {:?}", stmt_result);
}

/// # 6. 实际应用示例
pub fn practical_examples() {
  println!("\n=== 实际应用示例 ===");

  // 示例1：配置初始化
  let config = {
    let debug_mode = true;
    let max_connections = if debug_mode { 10 } else { 100 };
    let timeout = match max_connections {
      1..=10 => 30,
      11..=50 => 60,
      _ => 120,
    };

    (debug_mode, max_connections, timeout)
  };

  println!(
    "配置: debug={}, max_conn={}, timeout={}",
    config.0, config.1, config.2
  );

  // 示例2：数据处理管道
  let numbers = vec![1, 2, 3, 4, 5];
  let processed = {
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    let filtered: Vec<i32> = doubled.into_iter().filter(|&x| x > 5).collect();
    let sum: i32 = filtered.iter().sum();
    sum
  };

  println!("处理结果: {}", processed);

  // 示例3：错误处理
  fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
      Err("除数不能为零".to_string())
    } else {
      Ok(a / b)
    }
  }

  let result = match divide(10.0, 2.0) {
    Ok(value) => format!("结果: {}", value),
    Err(error) => format!("错误: {}", error),
  };

  println!("除法操作: {}", result);
}

/// # 7. 常见错误和注意事项
pub fn common_mistakes_demo() {
  println!("\n=== 常见错误和注意事项 ===");

  // 错误1：试图将语句赋值给变量
  // let x = (let y = 5); // 编译错误！

  // 正确做法：
  let x = {
    let y = 5;
    y // 表达式
  };
  println!("正确的赋值: x = {}", x);

  // 错误2：忘记表达式和语句的区别
  let wrong = {
    let a = 5;
    let b = 10;
    a + b; // 有分号，这是语句，返回 ()
  };

  let correct = {
    let a = 5;
    let b = 10;
    a + b // 没有分号，这是表达式，返回 15
  };

  println!("错误示例返回: {:?}", wrong); // ()
  println!("正确示例返回: {}", correct); // 15

  // 错误3：if 表达式分支返回不同类型
  // let result = if true { 42 } else { "hello" }; // 编译错误！

  // 正确做法：确保所有分支返回相同类型
  let result = if true { "42" } else { "hello" };
  println!("if 表达式结果: {}", result);

  // 注意事项：单元类型的使用
  let unit_value = {
    println!("执行一些副作用操作");
    // 没有返回表达式，返回 ()
  };
  println!("单元类型值: {:?}", unit_value);
}

/// # 8. 高级用法
pub fn advanced_usage_demo() {
  println!("\n=== 高级用法演示 ===");

  // 嵌套块表达式
  let complex_calculation = {
    let base = {
      let x = 5;
      let y = 3;
      x * y
    };

    let modifier = {
      let factor = 2;
      let offset = 1;
      factor + offset
    };

    base + modifier
  };

  println!("复杂计算结果: {}", complex_calculation);

  // 表达式作为函数参数
  fn process_value(value: i32) -> i32 {
    value * 2
  }

  let result = process_value({
    let a = 10;
    let b = 5;
    a - b
  });

  println!("函数参数表达式结果: {}", result);

  // 宏中的表达式
  macro_rules! calculate {
    ($expr:expr) => {{
      println!("计算表达式: {}", stringify!($expr));
      $expr
    }};
  }

  let macro_result = calculate!(3 + 4 * 2);
  println!("宏计算结果: {}", macro_result);
}

/// # 主函数：运行所有演示
pub fn run_all_demos() {
  println!("🦀 Rust 语句与表达式完整指南");
  println!("=================================\n");

  basic_concepts_demo();
  declaration_statements_demo();
  item_declarations_demo();
  literal_expressions_demo();
  tuple_expressions_demo();
  struct_expressions_demo();
  block_expressions_demo();
  range_expressions_demo();
  if_expressions_demo();
  match_expressions_demo();
  array_expressions_demo();
  operator_expressions_demo();
  expression_statements_demo();
  statement_vs_expression_demo();
  practical_examples();
  common_mistakes_demo();
  advanced_usage_demo();

  println!("\n=== 总结 ===");
  println!("1. 语句执行操作但不返回值");
  println!("2. 表达式计算并返回值");
  println!("3. 表达式 + 分号 = 表达式语句");
  println!("4. 块表达式的值是最后一个表达式的值");
  println!("5. Rust 是基于表达式的语言");
  println!("6. 理解语句与表达式的区别对于掌握 Rust 至关重要");
}

// 测试模块
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_block_expression() {
    let result = {
      let a = 5;
      let b = 10;
      a + b
    };
    assert_eq!(result, 15);
  }

  #[test]
  fn test_if_expression() {
    let number = 6;
    let result = if number % 2 == 0 { "even" } else { "odd" };
    assert_eq!(result, "even");
  }

  #[test]
  fn test_match_expression() {
    let value = Some(42);
    let result = match value {
      Some(x) => x * 2,
      None => 0,
    };
    assert_eq!(result, 84);
  }
}
