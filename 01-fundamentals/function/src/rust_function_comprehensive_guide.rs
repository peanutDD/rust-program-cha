//! # Rust 函数详解指南
//!
//! 本文件基于 https://course.rs/basic/base-type/function.html 的内容，
//! 提供了关于 Rust 函数的全面、详细的讲解和示例。
//!
//! ## 目录
//! 1. 函数基础概念
//! 2. 函数定义与调用
//! 3. 函数参数详解
//! 4. 函数返回值
//! 5. 语句与表达式在函数中的应用
//! 6. 函数作用域与生命周期
//! 7. 高级函数特性
//! 8. 实际应用示例
//! 9. 常见错误与最佳实践
//! 10. 性能优化技巧

use std::fmt::Debug;

/// 运行所有函数演示
pub fn run_all_function_demos() {
  println!("🦀 === Rust 函数详解指南 ===");
  println!();

  basic_function_concepts();
  function_definition_and_calling();
  function_parameters_demo();
  function_return_values_demo();
  statements_expressions_in_functions();
  function_scope_and_lifetime();
  advanced_function_features();
  practical_function_examples();
  common_mistakes_and_best_practices();
  performance_optimization_tips();

  println!("\n🎯 === 总结 ===");
  println!("1. 函数是 Rust 程序的基本构建块");
  println!("2. 函数名使用 snake_case 命名规范");
  println!("3. 参数必须明确指定类型");
  println!("4. 返回值可以通过 return 或最后一个表达式指定");
  println!("5. 理解语句与表达式的区别对函数设计至关重要");
  println!("6. 合理使用函数可以提高代码的可读性和可维护性");
}

/// 1. 函数基础概念
pub fn basic_function_concepts() {
  println!("=== 1. 函数基础概念 ===");

  // 函数是什么？
  println!("📚 函数定义：");
  println!("- 函数是一组执行特定任务的语句块");
  println!("- 函数是可读、可维护、可重用的代码单元");
  println!("- 每个 Rust 程序至少有一个函数：main() 函数");
  println!();

  // 函数的作用
  println!("🎯 函数的作用：");
  println!("- 代码复用：避免重复编写相同的代码");
  println!("- 模块化：将复杂问题分解为小的、可管理的部分");
  println!("- 抽象：隐藏实现细节，提供清晰的接口");
  println!("- 测试：便于单独测试各个功能模块");
  println!();

  // 函数的组成部分
  println!("🔧 函数的组成部分：");
  println!("- fn 关键字：声明函数");
  println!("- 函数名：标识函数的名称");
  println!("- 参数列表：函数接收的输入");
  println!("- 返回类型：函数输出的数据类型");
  println!("- 函数体：函数的具体实现");
  println!();

  // 基本语法格式
  println!("📝 基本语法格式：");
  println!("fn function_name(param1: Type1, param2: Type2) -> ReturnType {{");
  println!("    // 函数体");
  println!("    // 返回值");
  println!("}}");
  println!();
}

/// 2. 函数定义与调用
pub fn function_definition_and_calling() {
  println!("=== 2. 函数定义与调用 ===");

  // 最简单的函数定义
  println!("🔹 最简单的函数定义：");
  simple_hello();
  println!();

  // 函数调用顺序
  println!("🔹 函数调用顺序演示：");
  println!("Rust 不关心函数定义的位置，只要在调用时可见即可");
  call_order_demo();
  println!();

  // 嵌套函数调用
  println!("🔹 嵌套函数调用：");
  nested_function_calls();
  println!();

  // 函数作为值
  println!("🔹 函数作为值：");
  function_as_value_demo();
  println!();
}

// 简单的 hello 函数
fn simple_hello() {
  println!("Hello from simple_hello function!");
}

// 演示函数调用顺序
fn call_order_demo() {
  println!("调用 first_function");
  first_function();

  println!("调用 second_function");
  second_function();
}

fn first_function() {
  println!("  这是第一个函数");
}

fn second_function() {
  println!("  这是第二个函数");
}

// 嵌套函数调用
fn nested_function_calls() {
  println!("开始嵌套调用");
  level_one();
}

fn level_one() {
  println!("  第一层调用");
  level_two();
}

fn level_two() {
  println!("    第二层调用");
  level_three();
}

fn level_three() {
  println!("      第三层调用 - 最深层");
}

// 函数作为值的演示
fn function_as_value_demo() {
  // 函数指针
  let func_ptr: fn() = simple_hello;
  println!("通过函数指针调用：");
  func_ptr();

  // 函数作为参数传递
  execute_function(simple_hello);
}

fn execute_function(f: fn()) {
  println!("执行传入的函数：");
  f();
}

/// 3. 函数参数详解
pub fn function_parameters_demo() {
  println!("=== 3. 函数参数详解 ===");

  // 单个参数
  println!("🔹 单个参数：");
  greet_person("Alice");
  println!();

  // 多个参数
  println!("🔹 多个参数：");
  let sum = add_numbers(10, 20);
  println!("10 + 20 = {}", sum);
  println!();

  // 不同类型的参数
  println!("🔹 不同类型的参数：");
  display_info("Bob", 25, true);
  println!();

  // 引用参数
  println!("🔹 引用参数：");
  let name = String::from("Charlie");
  greet_with_reference(&name);
  println!("原始字符串仍然可用: {}", name);
  println!();

  // 可变引用参数
  println!("🔹 可变引用参数：");
  let mut counter = 0;
  increment_counter(&mut counter);
  println!("计数器值: {}", counter);
  println!();

  // 切片参数
  println!("🔹 切片参数：");
  let numbers = [1, 2, 3, 4, 5];
  let total = sum_slice(&numbers);
  println!("数组 {:?} 的和为: {}", numbers, total);
  println!();

  // 元组参数
  println!("🔹 元组参数：");
  let point = (3, 4);
  let distance = calculate_distance(point);
  println!("点 {:?} 到原点的距离: {:.2}", point, distance);
  println!();

  // 结构体参数
  println!("🔹 结构体参数：");
  let person = Person {
    name: String::from("David"),
    age: 30,
    email: String::from("david@example.com"),
  };
  display_person_info(&person);
  println!();
}

// 单个参数函数
fn greet_person(name: &str) {
  println!("Hello, {}!", name);
}

// 多个参数函数
fn add_numbers(a: i32, b: i32) -> i32 {
  a + b
}

// 不同类型参数
fn display_info(name: &str, age: u32, is_student: bool) {
  println!("姓名: {}, 年龄: {}, 是学生: {}", name, age, is_student);
}

// 引用参数
fn greet_with_reference(name: &String) {
  println!("Hello, {}! (通过引用)", name);
}

// 可变引用参数
fn increment_counter(counter: &mut i32) {
  *counter += 1;
  println!("计数器递增到: {}", counter);
}

// 切片参数
fn sum_slice(numbers: &[i32]) -> i32 {
  let mut total = 0;
  for &num in numbers {
    total += num;
  }
  total
}

// 元组参数
fn calculate_distance(point: (i32, i32)) -> f64 {
  let (x, y) = point;
  ((x * x + y * y) as f64).sqrt()
}

// 结构体定义
#[derive(Debug)]
struct Person {
  name: String,
  age: u32,
  email: String,
}

// 结构体参数
fn display_person_info(person: &Person) {
  // 明确访问字段以演示结构体参数的使用
  println!(
    "人员信息: 姓名={}, 年龄={}, 邮箱={}",
    person.name, person.age, person.email
  );
}

/// 4. 函数返回值详解
pub fn function_return_values_demo() {
  println!("=== 4. 函数返回值详解 ===");

  // 无返回值函数（返回单元类型）
  println!("🔹 无返回值函数：");
  print_message();
  println!();

  // 显式 return 语句
  println!("🔹 显式 return 语句：");
  let result1 = multiply_explicit(6, 7);
  println!("6 * 7 = {} (显式 return)", result1);
  println!();

  // 隐式返回（表达式）
  println!("🔹 隐式返回（表达式）：");
  let result2 = multiply_implicit(8, 9);
  println!("8 * 9 = {} (隐式返回)", result2);
  println!();

  // 提前返回
  println!("🔹 提前返回：");
  println!("检查正数: {}", check_positive(5));
  println!("检查负数: {}", check_positive(-3));
  println!();

  // 返回元组（多个值）
  println!("🔹 返回元组（多个值）：");
  let (quotient, remainder) = divide_with_remainder(17, 5);
  println!("17 ÷ 5 = {} 余 {}", quotient, remainder);
  println!();

  // 返回结构体
  println!("🔹 返回结构体：");
  let rect = create_rectangle(10, 20);
  println!("创建的矩形: {:?}", rect);
  println!("矩形面积: {}", rect.area());
  println!();

  // 返回 Option
  println!("🔹 返回 Option：");
  match find_first_even(&[1, 3, 4, 7, 8]) {
    Some(num) => println!("找到第一个偶数: {}", num),
    None => println!("没有找到偶数"),
  }
  println!();

  // 返回 Result
  println!("🔹 返回 Result：");
  match safe_divide(10.0, 2.0) {
    Ok(result) => println!("10.0 / 2.0 = {}", result),
    Err(e) => println!("错误: {}", e),
  }

  match safe_divide(10.0, 0.0) {
    Ok(result) => println!("10.0 / 0.0 = {}", result),
    Err(e) => println!("错误: {}", e),
  }
  println!();
}

// 无返回值函数
fn print_message() {
  println!("这是一个无返回值的函数");
  // 隐式返回 ()
}

// 显式 return
fn multiply_explicit(a: i32, b: i32) -> i32 {
  return a * b;
}

// 隐式返回
fn multiply_implicit(a: i32, b: i32) -> i32 {
  a * b // 注意：没有分号
}

// 提前返回
fn check_positive(num: i32) -> &'static str {
  if num <= 0 {
    return "不是正数"; // 提前返回
  }
  "是正数"
}

// 返回元组
fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
  (dividend / divisor, dividend % divisor)
}

// 矩形结构体
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
}

// 返回结构体
fn create_rectangle(width: u32, height: u32) -> Rectangle {
  Rectangle { width, height }
}

// 返回 Option
fn find_first_even(numbers: &[i32]) -> Option<i32> {
  for &num in numbers {
    if num % 2 == 0 {
      return Some(num);
    }
  }
  None
}

// 返回 Result
fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
  if b == 0.0 {
    Err("除数不能为零".to_string())
  } else {
    Ok(a / b)
  }
}

/// 5. 语句与表达式在函数中的应用
pub fn statements_expressions_in_functions() {
  println!("=== 5. 语句与表达式在函数中的应用 ===");

  // 语句 vs 表达式
  println!("🔹 语句 vs 表达式：");
  let result1 = statement_vs_expression_demo();
  println!("函数返回值: {}", result1);
  println!();

  // 块表达式
  println!("🔹 块表达式：");
  let result2 = block_expression_demo();
  println!("块表达式结果: {}", result2);
  println!();

  // 条件表达式
  println!("🔹 条件表达式：");
  let result3 = conditional_expression_demo(15);
  println!("条件表达式结果: {}", result3);
  println!();

  // 复杂表达式
  println!("🔹 复杂表达式：");
  let result4 = complex_expression_demo(5);
  println!("复杂表达式结果: {}", result4);
  println!();
}

// 语句与表达式演示
fn statement_vs_expression_demo() -> i32 {
  let x = 5; // 语句
  let y = 10; // 语句

  // 块表达式作为返回值
  {
    let sum = x + y; // 语句
    sum * 2 // 表达式（没有分号）
  }
}

// 块表达式演示
fn block_expression_demo() -> i32 {
  let a = {
    let inner = 10;
    inner + 5 // 表达式
  };

  let b = {
    let inner = 20;
    inner * 2 // 表达式
  };

  a + b // 表达式
}

// 条件表达式
fn conditional_expression_demo(x: i32) -> &'static str {
  if x > 10 {
    "大于10"
  } else if x > 5 {
    "大于5但小于等于10"
  } else {
    "小于等于5"
  }
}

// 复杂表达式
fn complex_expression_demo(n: i32) -> i32 {
  match n {
    1..=5 => {
      let base = n * 2;
      base + 10
    }
    6..=10 => {
      let base = n * 3;
      base - 5
    }
    _ => {
      let base = n * 4;
      base / 2
    }
  }
}

/// 6. 函数作用域与生命周期
pub fn function_scope_and_lifetime() {
  println!("=== 6. 函数作用域与生命周期 ===");

  // 局部变量作用域
  println!("🔹 局部变量作用域：");
  scope_demo();
  println!();

  // 参数作用域
  println!("🔹 参数作用域：");
  parameter_scope_demo("参数值");
  println!();

  // 变量遮蔽
  println!("🔹 变量遮蔽：");
  variable_shadowing_demo();
  println!();

  // 生命周期基础
  println!("🔹 生命周期基础：");
  lifetime_basic_demo();
  println!();
}

// 作用域演示
fn scope_demo() {
  let outer = "外部变量";
  println!("外部作用域: {}", outer);

  {
    let inner = "内部变量";
    println!("内部作用域: {}, {}", outer, inner);

    {
      let deep_inner = "深层内部变量";
      println!("深层作用域: {}, {}, {}", outer, inner, deep_inner);
    }
    // deep_inner 在这里不可访问
  }
  // inner 在这里不可访问
  println!("回到外部作用域: {}", outer);
}

// 参数作用域
fn parameter_scope_demo(param: &str) {
  println!("参数在函数内部可访问: {}", param);

  let local = "局部变量";
  println!("局部变量: {}", local);

  // 参数和局部变量在函数结束时销毁
}

// 变量遮蔽
fn variable_shadowing_demo() {
  let x = 5;
  println!("外层 x: {}", x);

  {
    let x = "字符串"; // 遮蔽外层的 x
    println!("内层 x: {}", x);

    {
      let x = true; // 再次遮蔽
      println!("深层 x: {}", x);
    }

    println!("回到内层 x: {}", x);
  }

  println!("回到外层 x: {}", x);
}

// 生命周期基础
fn lifetime_basic_demo() {
  let string1 = String::from("长字符串");
  let string2 = "短字符串";

  let result = longest(&string1, string2);
  println!("最长的字符串: {}", result);
}

// 生命周期注解示例
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() { x } else { y }
}

/// 7. 高级函数特性
pub fn advanced_function_features() {
  println!("=== 7. 高级函数特性 ===");

  // 泛型函数
  println!("🔹 泛型函数：");
  generic_function_demo();
  println!();

  // 函数重载（通过泛型实现）
  println!("🔹 函数重载（通过泛型实现）：");
  function_overloading_demo();
  println!();

  // 高阶函数
  println!("🔹 高阶函数：");
  higher_order_function_demo();
  println!();

  // 闭包
  println!("🔹 闭包：");
  closure_demo();
  println!();

  // 函数指针
  println!("🔹 函数指针：");
  function_pointer_demo();
  println!();
}

// 泛型函数演示
fn generic_function_demo() {
  let int_result = generic_max(10, 20);
  println!("整数最大值: {}", int_result);

  let float_result = generic_max(3.14, 2.71);
  println!("浮点数最大值: {}", float_result);

  let char_result = generic_max('a', 'z');
  println!("字符最大值: {}", char_result);
}

// 泛型函数
fn generic_max<T: PartialOrd>(a: T, b: T) -> T {
  if a > b { a } else { b }
}

// 函数重载演示
fn function_overloading_demo() {
  // 通过泛型实现类似重载的效果
  println!("打印整数: {}", format_value(42));
  println!("打印浮点数: {}", format_value(3.14));
  println!("打印字符串: {}", format_value("Hello"));
}

// 泛型格式化函数
fn format_value<T: std::fmt::Display>(value: T) -> String {
  format!("值: {}", value)
}

// 高阶函数演示
fn higher_order_function_demo() {
  let numbers = vec![1, 2, 3, 4, 5];

  // 使用函数作为参数
  let doubled = apply_operation(&numbers, double);
  println!("原数组: {:?}", numbers);
  println!("翻倍后: {:?}", doubled);

  let squared = apply_operation(&numbers, square);
  println!("平方后: {:?}", squared);
}

// 高阶函数
fn apply_operation<F>(numbers: &[i32], op: F) -> Vec<i32>
where
  F: Fn(i32) -> i32,
{
  numbers.iter().map(|&x| op(x)).collect()
}

fn double(x: i32) -> i32 {
  x * 2
}

fn square(x: i32) -> i32 {
  x * x
}

// 闭包演示
fn closure_demo() {
  let multiplier = 3;

  // 闭包捕获外部变量
  let multiply_by_three = |x| x * multiplier;

  let numbers = vec![1, 2, 3, 4, 5];
  let results: Vec<i32> = numbers.iter().map(|&x| multiply_by_three(x)).collect();

  println!("原数组: {:?}", numbers);
  println!("乘以3后: {:?}", results);

  // 闭包作为参数
  let sum = calculate_with_closure(10, 20, |a, b| a + b);
  println!("闭包计算和: {}", sum);

  let product = calculate_with_closure(10, 20, |a, b| a * b);
  println!("闭包计算积: {}", product);
}

fn calculate_with_closure<F>(a: i32, b: i32, operation: F) -> i32
where
  F: Fn(i32, i32) -> i32,
{
  operation(a, b)
}

// 函数指针演示
fn function_pointer_demo() {
  // 函数指针数组
  let operations: [fn(i32, i32) -> i32; 4] = [add, subtract, multiply, divide];
  let operation_names = ["加法", "减法", "乘法", "除法"];

  let a = 20;
  let b = 4;

  for (i, &op) in operations.iter().enumerate() {
    let result = op(a, b);
    println!("{}: {} 和 {} = {}", operation_names[i], a, b, result);
  }
}

fn add(a: i32, b: i32) -> i32 {
  a + b
}
fn subtract(a: i32, b: i32) -> i32 {
  a - b
}
fn multiply(a: i32, b: i32) -> i32 {
  a * b
}
fn divide(a: i32, b: i32) -> i32 {
  a / b
}

/// 8. 实际应用示例
pub fn practical_function_examples() {
  println!("=== 8. 实际应用示例 ===");

  // 数据处理管道
  println!("🔹 数据处理管道：");
  data_processing_pipeline();
  println!();

  // 配置管理
  println!("🔹 配置管理：");
  configuration_management_demo();
  println!();

  // 错误处理
  println!("🔹 错误处理：");
  error_handling_demo();
  println!();

  // 算法实现
  println!("🔹 算法实现：");
  algorithm_implementation_demo();
  println!();
}

// 数据处理管道
fn data_processing_pipeline() {
  let raw_data = vec!["  hello  ", "  WORLD  ", "  rust  ", "  programming  "];

  let processed_data = raw_data
    .iter()
    .map(|s| clean_string(s))
    .map(|s| capitalize_first(&s))
    .filter(|s| is_valid_word(s))
    .collect::<Vec<String>>();

  println!("原始数据: {:?}", raw_data);
  println!("处理后数据: {:?}", processed_data);
}

fn clean_string(s: &str) -> String {
  s.trim().to_lowercase()
}

fn capitalize_first(s: &str) -> String {
  let mut chars = s.chars();
  match chars.next() {
    None => String::new(),
    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
  }
}

fn is_valid_word(s: &str) -> bool {
  s.len() > 3
}

// 配置管理
#[derive(Debug)]
struct AppConfig {
  database_url: String,
  port: u16,
  debug_mode: bool,
  max_connections: u32,
}

fn configuration_management_demo() {
  let config = load_config();
  println!("应用配置: {:?}", config);

  if validate_config(&config) {
    println!("配置验证通过");
    apply_config(&config);
  } else {
    println!("配置验证失败");
  }
}

fn load_config() -> AppConfig {
  AppConfig {
    database_url: "postgresql://localhost:5432/mydb".to_string(),
    port: 8080,
    debug_mode: true,
    max_connections: 100,
  }
}

fn validate_config(config: &AppConfig) -> bool {
  !config.database_url.is_empty() && config.port > 0 && config.max_connections > 0
}

fn apply_config(config: &AppConfig) {
  println!("应用配置:");
  println!("  数据库URL: {}", config.database_url);
  println!("  端口: {}", config.port);
  println!("  调试模式: {}", config.debug_mode);
  println!("  最大连接数: {}", config.max_connections);
}

// 错误处理
fn error_handling_demo() {
  let test_cases = vec![("123", "456"), ("abc", "123"), ("123", "0"), ("100", "25")];

  for (a_str, b_str) in test_cases {
    match safe_calculation(a_str, b_str) {
      Ok(result) => println!("计算 {} / {} = {}", a_str, b_str, result),
      Err(e) => println!("计算 {} / {} 失败: {}", a_str, b_str, e),
    }
  }
}

fn safe_calculation(a_str: &str, b_str: &str) -> Result<f64, String> {
  let a = parse_number(a_str)?;
  let b = parse_number(b_str)?;
  divide_safe(a, b)
}

fn parse_number(s: &str) -> Result<f64, String> {
  s.parse::<f64>().map_err(|_| format!("无法解析数字: {}", s))
}

fn divide_safe(a: f64, b: f64) -> Result<f64, String> {
  if b == 0.0 {
    Err("除数不能为零".to_string())
  } else {
    Ok(a / b)
  }
}

// 算法实现
fn algorithm_implementation_demo() {
  // 斐波那契数列
  println!("斐波那契数列前10项:");
  for i in 0..10 {
    print!("{} ", fibonacci(i));
  }
  println!();

  // 快速排序
  let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
  println!("排序前: {:?}", numbers);
  quick_sort(&mut numbers);
  println!("排序后: {:?}", numbers);

  // 二分查找
  let target = 25;
  match binary_search(&numbers, target) {
    Some(index) => println!("在索引 {} 找到 {}", index, target),
    None => println!("未找到 {}", target),
  }
}

// 斐波那契数列（递归实现）
fn fibonacci(n: u32) -> u64 {
  match n {
    0 => 0,
    1 => 1,
    _ => fibonacci(n - 1) + fibonacci(n - 2),
  }
}

// 快速排序
fn quick_sort(arr: &mut [i32]) {
  if arr.len() <= 1 {
    return;
  }

  let pivot_index = partition(arr);
  let (left, right) = arr.split_at_mut(pivot_index);

  quick_sort(left);
  quick_sort(&mut right[1..]);
}

fn partition(arr: &mut [i32]) -> usize {
  let pivot = arr[arr.len() - 1];
  let mut i = 0;

  for j in 0..arr.len() - 1 {
    if arr[j] <= pivot {
      arr.swap(i, j);
      i += 1;
    }
  }

  arr.swap(i, arr.len() - 1);
  i
}

// 二分查找
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
  let mut left = 0;
  let mut right = arr.len();

  while left < right {
    let mid = left + (right - left) / 2;

    match arr[mid].cmp(&target) {
      std::cmp::Ordering::Equal => return Some(mid),
      std::cmp::Ordering::Less => left = mid + 1,
      std::cmp::Ordering::Greater => right = mid,
    }
  }

  None
}

/// 9. 常见错误与最佳实践
pub fn common_mistakes_and_best_practices() {
  println!("=== 9. 常见错误与最佳实践 ===");

  // 常见错误
  println!("🔹 常见错误示例：");
  common_mistakes_demo();
  println!();

  // 最佳实践
  println!("🔹 最佳实践：");
  best_practices_demo();
  println!();

  // 代码风格
  println!("🔹 代码风格：");
  code_style_demo();
  println!();
}

fn common_mistakes_demo() {
  println!("❌ 常见错误：");
  println!("1. 忘记指定参数类型");
  println!("   // 错误: fn add(a, b) -> i32 {{ a + b }}");
  println!("   // 正确: fn add(a: i32, b: i32) -> i32 {{ a + b }}");
  println!();

  println!("2. 混淆语句和表达式");
  println!("   // 错误: fn get_value() -> i32 {{ let x = 5; x + 1; }}");
  println!("   // 正确: fn get_value() -> i32 {{ let x = 5; x + 1 }}");
  println!();

  println!("3. 返回类型不匹配");
  println!("   // 错误: fn get_string() -> String {{ \"hello\" }}");
  println!("   // 正确: fn get_string() -> String {{ \"hello\".to_string() }}");
  println!();

  // 演示正确的实现
  let result = correct_add(5, 3);
  println!("正确的加法函数结果: {}", result);

  let value = correct_get_value();
  println!("正确的表达式函数结果: {}", value);

  let text = correct_get_string();
  println!("正确的字符串函数结果: {}", text);
}

fn correct_add(a: i32, b: i32) -> i32 {
  a + b
}

fn correct_get_value() -> i32 {
  let x = 5;
  x + 1 // 表达式，没有分号
}

fn correct_get_string() -> String {
  "hello".to_string()
}

fn best_practices_demo() {
  println!("✅ 最佳实践：");
  println!("1. 使用描述性的函数名");
  println!("2. 保持函数简短和专注");
  println!("3. 使用类型注解提高可读性");
  println!("4. 合理使用错误处理");
  println!("5. 编写文档注释");
  println!();

  // 演示最佳实践
  let user_data = UserData {
    name: "Alice".to_string(),
    age: 25,
    email: "alice@example.com".to_string(),
  };

  match validate_and_process_user(&user_data) {
    Ok(message) => println!("处理成功: {}", message),
    Err(e) => println!("处理失败: {}", e),
  }
}

#[derive(Debug)]
struct UserData {
  name: String,
  age: u32,
  email: String,
}

/// 验证并处理用户数据
///
/// # 参数
/// * `user` - 用户数据引用
///
/// # 返回值
/// * `Ok(String)` - 处理成功的消息
/// * `Err(String)` - 错误信息
///
/// # 示例
/// ```
/// let user = UserData { name: "Alice".to_string(), age: 25, email: "alice@example.com".to_string() };
/// let result = validate_and_process_user(&user);
/// ```
fn validate_and_process_user(user: &UserData) -> Result<String, String> {
  // 验证用户数据
  validate_user_name(&user.name)?;
  validate_user_age(user.age)?;
  validate_user_email(&user.email)?;

  // 处理用户数据
  let processed_name = normalize_name(&user.name);

  Ok(format!("用户 {} 处理完成", processed_name))
}

fn validate_user_name(name: &str) -> Result<(), String> {
  if name.trim().is_empty() {
    Err("用户名不能为空".to_string())
  } else if name.len() < 2 {
    Err("用户名至少需要2个字符".to_string())
  } else {
    Ok(())
  }
}

fn validate_user_age(age: u32) -> Result<(), String> {
  if age < 18 {
    Err("年龄必须大于等于18岁".to_string())
  } else if age > 120 {
    Err("年龄不能超过120岁".to_string())
  } else {
    Ok(())
  }
}

fn validate_user_email(email: &str) -> Result<(), String> {
  if !email.contains('@') {
    Err("邮箱格式无效".to_string())
  } else {
    Ok(())
  }
}

fn normalize_name(name: &str) -> String {
  name.trim().to_lowercase()
}

fn code_style_demo() {
  println!("📝 代码风格指南：");
  println!("1. 函数名使用 snake_case");
  println!("2. 常量使用 SCREAMING_SNAKE_CASE");
  println!("3. 类型名使用 PascalCase");
  println!("4. 适当的空格和缩进");
  println!("5. 有意义的变量名");
  println!();

  // 演示良好的代码风格
  const MAX_RETRY_COUNT: u32 = 3;

  let connection_result = establish_database_connection(MAX_RETRY_COUNT);
  // 明确访问枚举字段以演示 Result 类型的使用
  match &connection_result {
    ConnectionResult::Success(msg) => println!("数据库连接成功: {}", msg),
    ConnectionResult::Failed(msg) => println!("数据库连接失败: {}", msg),
  }
}

#[derive(Debug)]
enum ConnectionResult {
  Success(String),
  Failed(String),
}

fn establish_database_connection(max_retries: u32) -> ConnectionResult {
  for attempt in 1..=max_retries {
    println!("尝试连接数据库，第 {} 次", attempt);

    // 模拟连接尝试
    if attempt == max_retries {
      return ConnectionResult::Success("连接成功".to_string());
    }
  }

  ConnectionResult::Failed("连接失败".to_string())
}

/// 10. 性能优化技巧
pub fn performance_optimization_tips() {
  println!("=== 10. 性能优化技巧 ===");

  // 避免不必要的克隆
  println!("🔹 避免不必要的克隆：");
  clone_optimization_demo();
  println!();

  // 使用引用传递
  println!("🔹 使用引用传递：");
  reference_passing_demo();
  println!();

  // 内联函数
  println!("🔹 内联函数：");
  inline_function_demo();
  println!();

  // 迭代器优化
  println!("🔹 迭代器优化：");
  iterator_optimization_demo();
  println!();
}

fn clone_optimization_demo() {
  let large_string = "这是一个很长的字符串".repeat(1000);

  // 低效：不必要的克隆
  let _result1 = process_string_inefficient(large_string.clone());

  // 高效：使用引用
  let _result2 = process_string_efficient(&large_string);

  println!("字符串处理完成（演示克隆优化）");
}

// 低效的实现
fn process_string_inefficient(s: String) -> usize {
  s.len() // 接收所有权，可能导致不必要的移动
}

// 高效的实现
fn process_string_efficient(s: &str) -> usize {
  s.len() // 只借用引用，避免所有权转移
}

fn reference_passing_demo() {
  let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

  // 使用引用传递，避免移动
  let sum = calculate_sum(&numbers);
  let average = calculate_average(&numbers);

  println!("数组: {:?}", numbers);
  println!("和: {}, 平均值: {:.2}", sum, average);
}

fn calculate_sum(numbers: &[i32]) -> i32 {
  numbers.iter().sum()
}

fn calculate_average(numbers: &[i32]) -> f64 {
  if numbers.is_empty() {
    0.0
  } else {
    numbers.iter().sum::<i32>() as f64 / numbers.len() as f64
  }
}

fn inline_function_demo() {
  let numbers = vec![1, 2, 3, 4, 5];
  let doubled: Vec<i32> = numbers.iter().map(|&x| inline_double(x)).collect();

  println!("原数组: {:?}", numbers);
  println!("翻倍后: {:?}", doubled);
}

// 内联函数，减少函数调用开销
#[inline]
fn inline_double(x: i32) -> i32 {
  x * 2
}

fn iterator_optimization_demo() {
  let numbers: Vec<i32> = (1..=1000).collect();

  // 使用迭代器链，延迟计算
  let result: Vec<i32> = numbers
    .iter()
    .filter(|&&x| x % 2 == 0) // 过滤偶数
    .map(|&x| x * x) // 平方
    .take(10) // 只取前10个
    .collect();

  println!("前10个偶数的平方: {:?}", result);

  // 演示零成本抽象
  let sum: i32 = numbers
    .iter()
    .filter(|&&x| x % 3 == 0)
    .map(|&x| x * 2)
    .sum();

  println!("能被3整除的数翻倍后的和: {}", sum);
}

// 测试模块
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add_numbers() {
    assert_eq!(add_numbers(2, 3), 5);
    assert_eq!(add_numbers(-1, 1), 0);
    assert_eq!(add_numbers(0, 0), 0);
  }

  #[test]
  fn test_generic_max() {
    assert_eq!(generic_max(10, 20), 20);
    assert_eq!(generic_max(3.14, 2.71), 3.14);
    assert_eq!(generic_max('a', 'z'), 'z');
  }

  #[test]
  fn test_safe_divide() {
    assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
    assert!(safe_divide(10.0, 0.0).is_err());
  }

  #[test]
  fn test_fibonacci() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(10), 55);
  }

  #[test]
  fn test_validate_user_name() {
    assert!(validate_user_name("Alice").is_ok());
    assert!(validate_user_name("").is_err());
    assert!(validate_user_name("A").is_err());
  }
}
