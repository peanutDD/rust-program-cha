//! # Rust 函数语法深度解析
//!
//! 本模块详细解析 Rust 函数语法的各个组成部分，帮助深入理解函数定义的每个元素。

/// # Rust 函数语法完整解析
///
/// 基于函数定义图解：`fn add(i: i32, j: i32) -> i32 { ... }`
///
/// 让我们逐一分析每个组成部分的含义、作用和最佳实践。
pub fn comprehensive_function_syntax_analysis() {
  println!("🦀 Rust 函数语法深度解析");
  println!("{}", "=".repeat(60));

  // 1. 函数关键字分析
  function_keyword_analysis();

  // 2. 标识符命名规则
  identifier_naming_analysis();

  // 3. 参数系统详解
  parameter_system_analysis();

  // 4. 返回类型系统
  return_type_analysis();

  // 5. 函数体结构
  function_body_analysis();

  // 6. 完整语法示例
  complete_syntax_examples();

  println!("\n{}", "=".repeat(60));
  println!("✅ 函数语法解析完成！");
}

/// ## 1. 函数关键字 `fn` 深度解析
///
/// `fn` 关键字是 Rust 中定义函数的唯一方式，它告诉编译器接下来是一个函数定义。
fn function_keyword_analysis() {
  println!("\n=== 1. 函数关键字 `fn` 分析 ===");

  println!("\n--- 1.1 关键字的作用 ---");
  println!("• `fn` 是 Rust 的保留关键字，专门用于函数定义");
  println!("• 它标志着函数定义的开始");
  println!("• 编译器通过 `fn` 识别函数声明");
  println!("• 必须小写，不能是 `Fn` 或 `FN`");

  println!("\n--- 1.2 语法位置 ---");
  println!("• 必须位于函数定义的最开始");
  println!("• 可以有可见性修饰符前缀：`pub fn`");
  println!("• 可以有属性标注：`#[inline] fn`");

  // 示例：不同的函数定义方式
  demonstrate_fn_keyword_usage();
}

fn demonstrate_fn_keyword_usage() {
  println!("\n--- 1.3 实际使用示例 ---");

  // 基本函数
  fn basic_function() {
    println!("这是一个基本函数");
  }

  // 公共函数
  // pub fn public_function() { ... }

  // 带属性的函数
  #[inline]
  fn inline_function() -> i32 {
    42
  }

  basic_function();
  let result = inline_function();
  println!("内联函数结果: {}", result);

  println!("✓ 所有函数都以 `fn` 关键字开始");
}

/// ## 2. 标识符（函数名）深度解析
///
/// 函数名是函数的唯一标识符，遵循 Rust 的命名规范。
fn identifier_naming_analysis() {
  println!("\n=== 2. 标识符（函数名）分析 ===");

  println!("\n--- 2.1 命名规则 ---");
  println!("• 必须以字母或下划线开头");
  println!("• 可以包含字母、数字、下划线");
  println!("• 使用 snake_case 命名风格（Rust 约定）");
  println!("• 不能使用 Rust 保留关键字");
  println!("• 区分大小写");

  println!("\n--- 2.2 命名最佳实践 ---");
  println!("• 使用动词描述函数的行为");
  println!("• 名称应该清晰表达函数的目的");
  println!("• 避免缩写，除非是广泛认知的");
  println!("• 保持一致的命名风格");

  demonstrate_naming_conventions();
}

fn demonstrate_naming_conventions() {
  println!("\n--- 2.3 命名示例对比 ---");

  // 好的命名示例
  fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
  }

  fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
  }

  fn process_user_data(data: &str) -> String {
    data.trim().to_lowercase()
  }

  // 使用示例
  let area = calculate_area(10.0, 5.0);
  let email_valid = is_valid_email("user@example.com");
  let processed = process_user_data("  HELLO WORLD  ");

  println!("✓ 好的命名：calculate_area -> 面积: {}", area);
  println!("✓ 好的命名：is_valid_email -> 有效: {}", email_valid);
  println!("✓ 好的命名：process_user_data -> 处理后: '{}'", processed);

  println!("\n--- 2.4 避免的命名方式 ---");
  println!("❌ 避免：calc, proc, chk（过度缩写）");
  println!("❌ 避免：function1, func2（无意义命名）");
  println!("❌ 避免：doStuff（非 snake_case）");
}

/// ## 3. 参数系统深度解析
///
/// 参数是函数接收外部数据的机制，包括参数名、类型和传递方式。
fn parameter_system_analysis() {
  println!("\n=== 3. 参数系统分析 ===");

  println!("\n--- 3.1 参数语法结构 ---");
  println!("• 格式：`参数名: 参数类型`");
  println!("• 多个参数用逗号分隔");
  println!("• 参数名遵循 snake_case 命名");
  println!("• 类型必须明确指定（无类型推导）");

  println!("\n--- 3.2 参数类型分类 ---");
  demonstrate_parameter_types();

  println!("\n--- 3.3 参数传递方式 ---");
  demonstrate_parameter_passing();

  println!("\n--- 3.4 特殊参数模式 ---");
  demonstrate_special_parameters();
}

fn demonstrate_parameter_types() {
  println!("\n基本类型参数:");

  // 基本类型参数
  fn handle_numbers(integer: i32, float: f64, boolean: bool) {
    println!("整数: {}, 浮点: {}, 布尔: {}", integer, float, boolean);
  }

  // 字符串参数
  fn handle_strings(owned: String, borrowed: &str) {
    println!("拥有的字符串: {}, 借用的字符串: {}", owned, borrowed);
  }

  // 集合类型参数
  fn handle_collections(vec: Vec<i32>, slice: &[i32]) {
    println!("向量长度: {}, 切片长度: {}", vec.len(), slice.len());
  }

  // 使用示例
  handle_numbers(42, 3.14, true);
  handle_strings("拥有".to_string(), "借用");
  handle_collections(vec![1, 2, 3], &[4, 5, 6]);
}

fn demonstrate_parameter_passing() {
  println!("\n参数传递方式:");

  // 按值传递（移动语义）
  fn take_ownership(data: String) {
    println!("获得所有权: {}", data);
    // data 在函数结束时被销毁
  }

  // 不可变借用
  fn borrow_immutable(data: &String) {
    println!("不可变借用: {}", data);
    // 不能修改 data
  }

  // 可变借用
  fn borrow_mutable(data: &mut String) {
    data.push_str(" - 已修改");
    println!("可变借用并修改: {}", data);
  }

  // 使用示例
  let original = String::from("原始数据");
  borrow_immutable(&original);

  let mut mutable_data = String::from("可变数据");
  borrow_mutable(&mut mutable_data);
  println!("修改后的数据: {}", mutable_data);

  // 最后移动所有权
  take_ownership(original);
  // println!("{}", original); // 编译错误：值已被移动
}

fn demonstrate_special_parameters() {
  println!("\n特殊参数模式:");

  // 元组参数
  fn handle_tuple(point: (i32, i32)) {
    println!("坐标点: ({}, {})", point.0, point.1);
  }

  // 结构体参数
  #[derive(Debug)]
  struct Person {
    name: String,
    age: u32,
  }

  fn handle_struct(person: &Person) {
    println!("人员信息: {:?}", person);
  }

  // 泛型参数
  fn handle_generic<T: std::fmt::Display>(value: T) {
    println!("泛型值: {}", value);
  }

  // 使用示例
  handle_tuple((10, 20));

  let person = Person {
    name: "张三".to_string(),
    age: 25,
  };
  handle_struct(&person);

  handle_generic(42);
  handle_generic("字符串");
}

/// ## 4. 返回类型系统深度解析
///
/// 返回类型定义了函数执行后返回给调用者的数据类型。
fn return_type_analysis() {
  println!("\n=== 4. 返回类型系统分析 ===");

  println!("\n--- 4.1 返回类型语法 ---");
  println!("• 使用 `->` 箭头符号指示返回类型");
  println!("• 箭头后跟具体的类型名称");
  println!("• 无返回值时可省略（默认为单元类型 `()`）");
  println!("• 返回类型必须与函数体最后的表达式类型匹配");

  demonstrate_return_types();

  println!("\n--- 4.2 返回值的实现方式 ---");
  demonstrate_return_mechanisms();

  println!("\n--- 4.3 复杂返回类型 ---");
  demonstrate_complex_return_types();
}

fn demonstrate_return_types() {
  println!("\n--- 4.1 基本返回类型示例 ---");

  // 无返回值（单元类型）
  fn no_return() {
    println!("无返回值函数");
    // 隐式返回 ()
  }

  // 显式单元类型返回
  fn explicit_unit() -> () {
    println!("显式单元类型返回");
  }

  // 基本类型返回
  fn return_integer() -> i32 {
    42 // 表达式，无分号
  }

  fn return_string() -> String {
    "Hello, Rust!".to_string()
  }

  fn return_boolean() -> bool {
    true
  }

  // 使用示例
  no_return();
  explicit_unit();
  println!("返回的整数: {}", return_integer());
  println!("返回的字符串: {}", return_string());
  println!("返回的布尔值: {}", return_boolean());
}

fn demonstrate_return_mechanisms() {
  println!("\n--- 4.2 返回值实现方式 ---");

  // 表达式返回（推荐）
  fn expression_return(x: i32) -> i32 {
    x * 2 // 最后一行是表达式，无分号
  }

  // 显式 return 语句
  fn explicit_return(x: i32) -> i32 {
    return x * 2; // 使用 return 关键字
  }

  // 条件返回
  fn conditional_return(x: i32) -> &'static str {
    if x > 0 {
      "正数"
    } else if x < 0 {
      "负数"
    } else {
      "零"
    }
  }

  // 提前返回
  fn early_return(x: i32) -> i32 {
    if x < 0 {
      return 0; // 提前返回
    }

    x * x // 正常返回
  }

  // 使用示例
  println!("表达式返回: {}", expression_return(5));
  println!("显式返回: {}", explicit_return(5));
  println!("条件返回: {}", conditional_return(-3));
  println!("提前返回: {}", early_return(-5));
}

fn demonstrate_complex_return_types() {
  println!("\n--- 4.3 复杂返回类型示例 ---");

  // 元组返回
  fn return_tuple() -> (i32, String, bool) {
    (42, "元组".to_string(), true)
  }

  // Option 返回
  fn return_option(x: i32) -> Option<i32> {
    if x >= 0 { Some(x) } else { None }
  }

  // Result 返回
  fn return_result(x: i32) -> Result<i32, String> {
    if x != 0 {
      Ok(100 / x)
    } else {
      Err("除零错误".to_string())
    }
  }

  // 向量返回
  fn return_vector() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
  }

  // 使用示例
  let (num, text, flag) = return_tuple();
  println!("元组返回: {}, {}, {}", num, text, flag);

  match return_option(10) {
    Some(value) => println!("Option 返回: {}", value),
    None => println!("Option 返回: None"),
  }

  match return_result(5) {
    Ok(value) => println!("Result 成功: {}", value),
    Err(error) => println!("Result 错误: {}", error),
  }

  let numbers = return_vector();
  println!("向量返回: {:?}", numbers);
}

/// ## 5. 函数体结构深度解析
///
/// 函数体是函数的执行逻辑所在，由大括号包围的代码块组成。
fn function_body_analysis() {
  println!("\n=== 5. 函数体结构分析 ===");

  println!("\n--- 5.1 函数体语法 ---");
  println!("• 使用大括号 `{{}}` 包围函数体");
  println!("• 左大括号通常与函数签名在同一行");
  println!("• 函数体包含语句和表达式");
  println!("• 最后一个表达式（无分号）作为返回值");

  demonstrate_function_body_structure();

  println!("\n--- 5.2 语句与表达式 ---");
  demonstrate_statements_vs_expressions();

  println!("\n--- 5.3 作用域和生命周期 ---");
  demonstrate_scope_and_lifetime();
}

fn demonstrate_function_body_structure() {
  println!("\n--- 5.1 函数体结构示例 ---");

  // 简单函数体
  fn simple_body() -> i32 {
    42
  }

  // 复杂函数体
  fn complex_body(x: i32, y: i32) -> i32 {
    // 局部变量声明（语句）
    let sum = x + y;
    let product = x * y;

    // 条件逻辑
    if sum > product { sum } else { product }
  }

  // 带有多个代码块的函数体
  fn multiple_blocks(numbers: &[i32]) -> (i32, i32) {
    let sum = {
      let mut total = 0;
      for &num in numbers {
        total += num;
      }
      total // 代码块的返回值
    };

    let max = {
      let mut maximum = numbers[0];
      for &num in numbers {
        if num > maximum {
          maximum = num;
        }
      }
      maximum
    };

    (sum, max) // 函数的返回值
  }

  // 使用示例
  println!("简单函数体: {}", simple_body());
  println!("复杂函数体: {}", complex_body(10, 5));

  let numbers = [1, 5, 3, 9, 2];
  let (sum, max) = multiple_blocks(&numbers);
  println!("多代码块函数: 和={}, 最大值={}", sum, max);
}

fn demonstrate_statements_vs_expressions() {
  println!("\n--- 5.2 语句与表达式对比 ---");

  fn statement_expression_demo() -> i32 {
    // 语句：以分号结尾，不返回值
    let x = 10; // 变量绑定语句
    let y = 20; // 变量绑定语句

    // 表达式：计算并返回值
    let sum = x + y; // 右侧是表达式，整体是语句

    // 代码块表达式
    let result = {
      let temp = sum * 2;
      temp + 5 // 表达式，无分号
    };

    // 条件表达式
    let final_result = if result > 50 { result } else { 50 };

    final_result // 函数返回表达式
  }

  let result = statement_expression_demo();
  println!("语句与表达式示例结果: {}", result);

  println!("\n关键区别:");
  println!("• 语句执行操作但不返回值（以分号结尾）");
  println!("• 表达式计算并返回值（无分号）");
  println!("• 函数体最后的表达式成为返回值");
}

fn demonstrate_scope_and_lifetime() {
  println!("\n--- 5.3 作用域和生命周期示例 ---");

  fn scope_demo(input: &str) -> String {
    // 函数参数的作用域：整个函数体
    println!("输入参数: {}", input);

    // 外层作用域变量
    let outer_var = "外层变量";

    let result = {
      // 内层作用域
      let inner_var = "内层变量";
      println!("内层: {}, {}", outer_var, inner_var);

      // inner_var 只在这个代码块内有效
      format!("{} + {}", outer_var, inner_var)
    };
    // inner_var 在这里已经超出作用域

    // 条件作用域
    if input.len() > 5 {
      let conditional_var = "条件变量";
      println!("条件内: {}", conditional_var);
    }
    // conditional_var 在这里已经超出作用域

    result
  }

  let demo_result = scope_demo("测试输入");
  println!("作用域演示结果: {}", demo_result);

  println!("\n作用域规则:");
  println!("• 变量在声明的代码块内有效");
  println!("• 内层作用域可以访问外层变量");
  println!("• 变量在离开作用域时自动销毁");
  println!("• 函数参数的作用域是整个函数体");
}

/// ## 6. 完整语法示例集合
///
/// 综合展示各种函数语法的完整示例。
fn complete_syntax_examples() {
  println!("\n=== 6. 完整语法示例集合 ===");

  demonstrate_basic_syntax_variations();
  demonstrate_advanced_syntax_patterns();
  demonstrate_real_world_examples();
}

fn demonstrate_basic_syntax_variations() {
  println!("\n--- 6.1 基础语法变体 ---");

  // 最简单的函数
  fn minimal() {
    println!("最简函数");
  }

  // 单参数函数
  fn single_param(x: i32) -> i32 {
    x * 2
  }

  // 多参数函数
  fn multiple_params(a: i32, b: f64, c: &str) -> String {
    format!("整数: {}, 浮点: {}, 字符串: {}", a, b, c)
  }

  // 复杂返回类型
  fn complex_return(flag: bool) -> Result<(i32, String), &'static str> {
    if flag {
      Ok((42, "成功".to_string()))
    } else {
      Err("失败")
    }
  }

  // 使用示例
  minimal();
  println!("单参数: {}", single_param(21));
  println!("多参数: {}", multiple_params(10, 3.14, "测试"));

  match complex_return(true) {
    Ok((num, msg)) => println!("复杂返回成功: {}, {}", num, msg),
    Err(e) => println!("复杂返回失败: {}", e),
  }
}

fn demonstrate_advanced_syntax_patterns() {
  println!("\n--- 6.2 高级语法模式 ---");

  // 泛型函数
  fn generic_function<T: std::fmt::Display + Clone>(value: T) -> T {
    println!("泛型值: {}", value);
    value.clone()
  }

  // 生命周期参数
  fn lifetime_function<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
  }

  // 高阶函数
  fn higher_order_function<F>(f: F, x: i32) -> i32
  where
    F: Fn(i32) -> i32,
  {
    f(x) + f(x * 2)
  }

  // 使用示例
  let cloned = generic_function("泛型测试");
  println!("克隆结果: {}", cloned);

  let longer = lifetime_function("短", "这是更长的字符串");
  println!("更长的字符串: {}", longer);

  let result = higher_order_function(|x| x * x, 3);
  println!("高阶函数结果: {}", result);
}

fn demonstrate_real_world_examples() {
  println!("\n--- 6.3 实际应用示例 ---");

  // 数据验证函数
  fn validate_email(email: &str) -> Result<(), String> {
    if email.is_empty() {
      return Err("邮箱不能为空".to_string());
    }

    if !email.contains('@') {
      return Err("邮箱格式无效：缺少@符号".to_string());
    }

    if !email.contains('.') {
      return Err("邮箱格式无效：缺少域名".to_string());
    }

    Ok(())
  }

  // 数据处理函数
  fn process_numbers(numbers: &[i32]) -> (i32, f64, i32, i32) {
    let sum: i32 = numbers.iter().sum();
    let average = sum as f64 / numbers.len() as f64;
    let min = *numbers.iter().min().unwrap_or(&0);
    let max = *numbers.iter().max().unwrap_or(&0);

    (sum, average, min, max)
  }

  // 配置解析函数
  fn parse_config(config_str: &str) -> std::collections::HashMap<String, String> {
    let mut config = std::collections::HashMap::new();

    for line in config_str.lines() {
      if let Some((key, value)) = line.split_once('=') {
        config.insert(key.trim().to_string(), value.trim().to_string());
      }
    }

    config
  }

  // 使用示例
  match validate_email("user@example.com") {
    Ok(()) => println!("✓ 邮箱验证通过"),
    Err(e) => println!("✗ 邮箱验证失败: {}", e),
  }

  let numbers = [1, 5, 3, 9, 2, 7];
  let (sum, avg, min, max) = process_numbers(&numbers);
  println!(
    "数据处理: 和={}, 平均={:.2}, 最小={}, 最大={}",
    sum, avg, min, max
  );

  let config_text = "host=localhost\nport=8080\ndebug=true";
  let config = parse_config(config_text);
  println!("配置解析: {:?}", config);
}

/// # 语法要点总结
///
/// 提供函数语法的关键要点和最佳实践总结。
pub fn syntax_summary() {
  println!("\n🎯 Rust 函数语法要点总结");
  println!("{}", "=".repeat(50));

  println!("\n📋 语法结构回顾:");
  println!("```rust");
  println!("fn function_name(param1: Type1, param2: Type2) -> ReturnType {{");
  println!("    // 函数体：语句和表达式");
  println!("    let local_var = param1 + param2;");
  println!("    ");
  println!("    // 返回表达式（无分号）");
  println!("    local_var * 2");
  println!("}}");
  println!("```");

  println!("\n🔑 关键要点:");
  println!("1. **fn 关键字**: 函数定义的标志，必须小写");
  println!("2. **函数名**: 使用 snake_case，清晰表达功能");
  println!("3. **参数列表**: 格式为 `name: type`，多个用逗号分隔");
  println!("4. **返回类型**: 用 `->` 指示，可省略（默认为 `()`）");
  println!("5. **函数体**: 大括号包围，包含语句和表达式");
  println!("6. **返回值**: 最后的表达式（无分号）或 return 语句");

  println!("\n✅ 最佳实践:");
  println!("• 函数名应清晰表达功能目的");
  println!("• 参数类型必须明确指定");
  println!("• 优先使用表达式返回而非 return 语句");
  println!("• 保持函数体简洁，单一职责");
  println!("• 合理使用借用避免不必要的所有权转移");
  println!("• 为复杂函数添加文档注释");

  println!("\n🚀 进阶特性:");
  println!("• 泛型参数: `fn func<T>(param: T) -> T`");
  println!("• 生命周期: `fn func<'a>(param: &'a str) -> &'a str`");
  println!("• trait 约束: `fn func<T: Display>(param: T)`");
  println!("• 高阶函数: `fn func<F: Fn(i32) -> i32>(f: F)`");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_function_syntax_analysis() {
    // 测试基本函数语法
    fn test_add(a: i32, b: i32) -> i32 {
      a + b
    }

    assert_eq!(test_add(2, 3), 5);
  }

  #[test]
  fn test_parameter_types() {
    fn test_params(num: i32, text: &str, flag: bool) -> String {
      format!("{}-{}-{}", num, text, flag)
    }

    let result = test_params(42, "test", true);
    assert_eq!(result, "42-test-true");
  }

  #[test]
  fn test_return_types() {
    fn test_option(x: i32) -> Option<i32> {
      if x > 0 { Some(x) } else { None }
    }

    assert_eq!(test_option(5), Some(5));
    assert_eq!(test_option(-1), None);
  }
}
