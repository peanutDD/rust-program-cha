//! # Rust 闭包深度解析与全面学习指南
//!
//! 本指南基于 https://course.rs/advance/functional-programing/closure.html
//! 提供了 Rust 闭包的深度解析和全面学习资源，从基础概念到编译器实现原理。
//!
//! ## 核心内容体系
//!
//! ### 第一部分：基础理论
//! - 闭包的本质和内存模型
//! - 编译器如何实现闭包
//! - 闭包与函数指针的区别
//! - 类型系统和生命周期
//!
//! ### 第二部分：捕获机制深度解析
//! - 三种捕获方式的内存布局
//! - 编译器的捕获推导算法
//! - move 语义的底层实现
//! - 捕获变量的性能影响
//!
//! ### 第三部分：Trait 系统深入
//! - Fn/FnMut/FnOnce 的实现原理
//! - 闭包的动态分发与静态分发
//! - 闭包的单态化过程
//! - 高阶类型和泛型约束
//!
//! ### 第四部分：高级应用与模式
//! - 函数式编程范式
//! - 异步编程中的闭包
//! - 并发编程模式
//! - 设计模式实现
//!
//! ### 第五部分：性能优化与最佳实践
//! - 零成本抽象的实现
//! - 内联优化策略
//! - 内存使用优化
//! - 编译时优化技巧
//!
//! ## 什么是闭包？
//!
//! 闭包（Closure）是一种可以捕获其环境中变量的匿名函数。
//! 在计算机科学中，闭包是引用了自由变量的函数，这个被引用的自由变量
//! 将和这个函数一同存在，即使已经离开了创造它的环境也不例外。
//!
//! ### 闭包的内存模型
//!
//! 在 Rust 中，每个闭包都有一个唯一的匿名类型，编译器会为每个闭包
//! 生成一个结构体，该结构体包含所有被捕获的变量。这种设计使得闭包
//! 既保持了类型安全，又实现了零成本抽象。

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

/// # 第一部分：闭包基础理论与实现原理

/// ## 1.1 闭包的本质和编译器实现
///
/// 在 Rust 中，每个闭包都有一个独特的匿名类型。编译器会为每个闭包
/// 生成一个结构体，该结构体实现了相应的 Fn trait。
///
/// ### 编译器转换示例：
/// ```rust
/// let x = 10;
/// let closure = |y| x + y;
/// ```
///
/// 编译器大致会生成类似这样的代码：
/// ```rust
/// struct ClosureType {
///     x: i32,  // 捕获的变量
/// }
///
/// impl Fn<(i32,)> for ClosureType {
///     type Output = i32;
///     extern "rust-call" fn call(&self, args: (i32,)) -> i32 {
///         self.x + args.0
///     }
/// }
/// ```
pub fn closure_compiler_implementation() {
  println!("=== 1.1 闭包的编译器实现原理 ===");

  let x = 10;
  let y = 20;

  // 这个闭包会被编译器转换为一个包含 x 和 y 的结构体
  let closure = |z| {
    println!("闭包内部访问: x={}, y={}, z={}", x, y, z);
    x + y + z
  };

  println!("闭包调用结果: {}", closure(5));

  // 展示闭包的大小（包含捕获的变量）
  println!("闭包的内存大小: {} bytes", std::mem::size_of_val(&closure));

  // 对比：不捕获变量的闭包
  let no_capture = |a: i32| a * 2;
  println!(
    "无捕获闭包的大小: {} bytes",
    std::mem::size_of_val(&no_capture)
  );
}

/// ## 1.2 闭包与函数指针的深度对比
///
/// 理解闭包和函数指针的区别对于掌握 Rust 的函数式编程至关重要。
pub fn closure_vs_function_pointer() {
  println!("\n=== 1.2 闭包 vs 函数指针 ===");

  // 函数指针：固定大小，无状态
  fn regular_function(x: i32) -> i32 {
    x * 2
  }

  let func_ptr: fn(i32) -> i32 = regular_function;
  println!("函数指针大小: {} bytes", std::mem::size_of_val(&func_ptr));

  // 闭包：可变大小，有状态
  let multiplier = 3;
  let closure = |x: i32| x * multiplier;
  println!("闭包大小: {} bytes", std::mem::size_of_val(&closure));

  // 函数指针可以转换为闭包，但反之不行
  let closure_from_fn: Box<dyn Fn(i32) -> i32> = Box::new(func_ptr);
  println!("从函数指针创建的闭包结果: {}", closure_from_fn(5));

  // 展示类型系统的差异
  demonstrate_type_differences();
}

fn demonstrate_type_differences() {
  println!("\n--- 类型系统差异演示 ---");

  // 每个闭包都有唯一的类型
  let closure1 = |x: i32| x + 1;
  let closure2 = |x: i32| x + 1; // 即使逻辑相同，类型也不同

  // 这会编译失败：
  // let same_type = if true { closure1 } else { closure2 };

  // 但可以通过 trait object 统一类型
  let boxed1: Box<dyn Fn(i32) -> i32> = Box::new(closure1);
  let boxed2: Box<dyn Fn(i32) -> i32> = Box::new(closure2);

  let dynamic_closure = if true { boxed1 } else { boxed2 };
  println!("动态分发闭包结果: {}", dynamic_closure(10));
}

/// ## 1.3 闭包基础语法深入
///
/// 闭包的语法看似简单，但背后有很多细节值得深入理解。
pub fn basic_closure_syntax() {
  println!("=== 1. 闭包的基本语法 ===");

  // 最简单的闭包：无参数
  let simple_closure = || println!("这是一个简单的闭包");
  simple_closure();

  // 带参数的闭包
  let add = |x, y| x + y;
  println!("5 + 3 = {}", add(5, 3));

  // 带类型注解的闭包
  let multiply: fn(i32, i32) -> i32 = |x, y| x * y;
  println!("4 * 6 = {}", multiply(4, 6));

  // 多行闭包
  let complex_closure = |x: i32| {
    let doubled = x * 2;
    let result = doubled + 10;
    println!("处理 {} 的结果是 {}", x, result);
    result
  };

  let result = complex_closure(5);
  println!("复杂闭包返回值: {}", result);
}

/// # 2. 闭包与函数的区别
///
/// 主要区别：闭包可以捕获环境中的变量，而函数不能
pub fn closure_vs_function() {
  println!("\n=== 2. 闭包与函数的区别 ===");

  let x = 10;

  // 普通函数无法访问环境变量
  fn normal_function(y: i32) -> i32 {
    // x // 这里无法访问外部变量 x，会编译错误
    y * 2
  }

  // 闭包可以捕获环境变量
  let closure_with_capture = |y| x + y; // 捕获了变量 x

  println!("普通函数结果: {}", normal_function(5));
  println!("闭包捕获变量结果: {}", closure_with_capture(5));

  // 闭包的类型推导
  let inferred_closure = |a| a + 1;
  println!("类型推导闭包: {}", inferred_closure(10));

  // 一旦确定类型，就不能改变
  // inferred_closure("hello"); // 这会编译错误，因为已经推导为整数类型
}

/// # 第二部分：捕获机制深度解析

/// ## 2.1 三种捕获方式的内存布局分析
///
/// Rust 闭包的捕获机制不仅影响语义，更直接影响内存布局和性能。
/// 理解这些机制的底层实现对于编写高效代码至关重要。
pub fn capture_mechanism_deep_dive() {
  println!("\n=== 第二部分：捕获机制深度解析 ===");

  // 演示不同捕获方式的内存布局
  demonstrate_memory_layout();

  // 分析编译器的捕获推导
  analyze_capture_inference();

  // 展示捕获对性能的影响
  capture_performance_impact();
}

fn demonstrate_memory_layout() {
  println!("\n--- 2.1.1 内存布局分析 ---");

  let small_var = 42u8; // 1 byte
  let medium_var = 1000u32; // 4 bytes
  let large_var = [0u64; 100]; // 800 bytes

  // 只捕获小变量
  let small_closure = |x: u8| small_var + x;
  println!(
    "只捕获 u8 的闭包大小: {} bytes",
    std::mem::size_of_val(&small_closure)
  );

  // 捕获中等变量
  let medium_closure = |x: u32| medium_var + x;
  println!(
    "只捕获 u32 的闭包大小: {} bytes",
    std::mem::size_of_val(&medium_closure)
  );

  // 捕获大变量（通过引用）
  let large_closure = |x: usize| large_var[x];
  println!(
    "捕获大数组引用的闭包大小: {} bytes",
    std::mem::size_of_val(&large_closure)
  );

  // 捕获大变量（通过 move）
  let moved_closure = move |x: usize| large_var[x];
  println!(
    "move 大数组的闭包大小: {} bytes",
    std::mem::size_of_val(&moved_closure)
  );
}

fn analyze_capture_inference() {
  println!("\n--- 2.1.2 编译器捕获推导算法 ---");

  let mut counter = 0;
  let immutable_data = vec![1, 2, 3, 4, 5];

  // 编译器推导：只读访问 -> 不可变借用
  let read_only = || {
    println!("只读访问: {:?}", immutable_data);
    immutable_data.len()
  };

  // 编译器推导：修改访问 -> 可变借用
  let mut modify_access = || {
    counter += 1;
    println!("修改计数器: {}", counter);
  };

  println!("只读闭包结果: {}", read_only());
  modify_access();

  // 编译器推导：所有权转移 -> move 语义（在使用后创建）
  let ownership_transfer = move || {
    let local_data = immutable_data; // 获取所有权
    local_data.into_iter().sum::<i32>()
  };

  println!("所有权转移闭包结果: {}", ownership_transfer());

  // 展示推导规则
  demonstrate_inference_rules();
}

fn demonstrate_inference_rules() {
  println!("\n--- 编译器推导规则演示 ---");

  let data = vec![1, 2, 3];
  let mut mutable_data = vec![4, 5, 6];

  // 规则1：最小权限原则
  let minimal_borrow = || data.len(); // 只需要不可变借用

  // 规则2：根据使用方式推导
  let mut mutable_borrow = || {
    mutable_data.push(7); // 需要可变借用
  };

  println!("最小权限闭包: {}", minimal_borrow());
  mutable_borrow();
  println!("可变数据: {:?}", mutable_data);

  // 规则3：显式 move 覆盖推导（在使用后创建）
  let explicit_move = move || {
    println!("显式移动: {:?}", data); // 强制获取所有权
  };

  explicit_move();
  // println!("{:?}", data);  // 编译错误：data 已被移动
}

fn capture_performance_impact() {
  println!("\n--- 2.1.3 捕获对性能的影响 ---");

  use std::time::Instant;

  let large_vec: Vec<i32> = (0..1000000).collect();

  // 测试引用捕获的性能
  let start = Instant::now();
  let ref_closure = || large_vec.len();
  let ref_result = ref_closure();
  let ref_time = start.elapsed();

  // 测试移动捕获的性能（创建时间）
  let start = Instant::now();
  let move_closure = move || large_vec.len();
  let move_creation_time = start.elapsed();

  let start = Instant::now();
  let move_result = move_closure();
  let move_call_time = start.elapsed();

  println!("引用捕获结果: {}, 耗时: {:?}", ref_result, ref_time);
  println!("移动捕获创建耗时: {:?}", move_creation_time);
  println!(
    "移动捕获调用结果: {}, 耗时: {:?}",
    move_result, move_call_time
  );
}

/// ## 2.2 传统的三种捕获方式详解
///
/// 在理解了底层机制后，我们来详细看看三种基本的捕获方式。
pub fn closure_capture_modes() {
  println!("\n=== 3. 闭包的捕获方式 ===");

  // 3.1 不可变借用捕获
  {
    println!("\n--- 3.1 不可变借用捕获 ---");
    let name = String::from("Alice");

    let print_name = || println!("姓名: {}", name); // 不可变借用

    print_name();
    print_name(); // 可以多次调用

    // name 仍然可以使用
    println!("原始变量仍可用: {}", name);
  }

  // 3.2 可变借用捕获
  {
    println!("\n--- 3.2 可变借用捕获 ---");
    let mut counter = 0;

    let mut increment = || {
      counter += 1; // 可变借用
      println!("计数器: {}", counter);
    };

    increment();
    increment();
    increment();

    // 注意：在闭包存在期间，counter 不能被其他方式访问
    println!("最终计数器值: {}", counter);
  }

  // 3.3 获取所有权捕获
  {
    println!("\n--- 3.3 获取所有权捕获 ---");
    let data = vec![1, 2, 3, 4, 5];

    let consume_data = || {
      println!("消费数据: {:?}", data); // 获取所有权
      data.len() // 返回长度
    };

    let length = consume_data();
    println!("数据长度: {}", length);

    // data 已经被移动，无法再使用
    // println!("{:?}", data); // 这会编译错误
  }
}

/// # 4. move 关键字
///
/// move 关键字强制闭包获取被捕获变量的所有权
pub fn move_keyword_examples() {
  println!("\n=== 4. move 关键字 ===");

  // 4.1 不使用 move
  {
    println!("\n--- 4.1 不使用 move ---");
    let x = vec![1, 2, 3];

    let closure = || println!("向量: {:?}", x); // 借用

    closure();
    println!("原始向量仍可用: {:?}", x);
  }

  // 4.2 使用 move
  {
    println!("\n--- 4.2 使用 move ---");
    let x = vec![1, 2, 3];

    let closure = move || println!("向量: {:?}", x); // 移动所有权

    closure();
    // println!("{:?}", x); // 编译错误：x 已被移动
  }

  // 4.3 move 与 Copy 类型
  {
    println!("\n--- 4.3 move 与 Copy 类型 ---");
    let x = 42; // i32 实现了 Copy

    let closure = move || println!("数字: {}", x); // 复制而非移动

    closure();
    println!("原始数字仍可用: {}", x); // Copy 类型不会被移动
  }

  // 4.4 move 在多线程中的应用
  {
    println!("\n--- 4.4 move 在多线程中的应用 ---");
    let data = vec![1, 2, 3, 4, 5];

    let handle = thread::spawn(move || {
      println!("线程中的数据: {:?}", data);
      data.iter().sum::<i32>()
    });

    let sum = handle.join().unwrap();
    println!("线程计算结果: {}", sum);

    // data 已被移动到线程中，这里无法使用
    // println!("{:?}", data); // 编译错误
  }
}

/// # 第三部分：Trait 系统深入解析

/// ## 3.1 Fn Trait 族的实现原理
///
/// Rust 的闭包 trait 系统是一个精心设计的层次结构，理解其实现原理
/// 对于掌握闭包的高级用法至关重要。
pub fn fn_traits_deep_dive() {
  println!("\n=== 第三部分：Trait 系统深入解析 ===");

  // 展示 trait 的层次关系
  demonstrate_trait_hierarchy();

  // 分析单态化过程
  analyze_monomorphization();

  // 展示动态分发 vs 静态分发
  compare_dispatch_methods();
}

fn demonstrate_trait_hierarchy() {
  println!("\n--- 3.1.1 Trait 层次关系演示 ---");

  // Fn: 最严格，可以多次调用且不修改环境
  let immutable_env = 42;
  let fn_closure = || immutable_env * 2;

  // FnMut: 中等严格，可以多次调用但可能修改环境
  let mut mutable_env = 0;
  let mut fn_mut_closure = || {
    mutable_env += 1;
    mutable_env
  };

  // FnOnce: 最宽松，可能消费环境变量
  let owned_data = vec![1, 2, 3];
  let fn_once_closure = move || owned_data.into_iter().sum::<i32>();

  // 展示 trait 的包含关系：Fn ⊆ FnMut ⊆ FnOnce
  call_fn_trait(&fn_closure);
  call_fn_mut_trait(&mut fn_mut_closure);
  call_fn_once_trait(fn_once_closure);

  // Fn 可以当作 FnMut 和 FnOnce 使用
  call_fn_mut_trait(&mut || immutable_env * 3);
  call_fn_once_trait(|| immutable_env * 4);
}

fn call_fn_trait<F>(f: &F) -> i32
where
  F: Fn() -> i32,
{
  let result = f();
  println!("调用 Fn trait: {}", result);
  result
}

fn call_fn_mut_trait<F>(f: &mut F) -> i32
where
  F: FnMut() -> i32,
{
  let result = f();
  println!("调用 FnMut trait: {}", result);
  result
}

fn call_fn_once_trait<F>(f: F) -> i32
where
  F: FnOnce() -> i32,
{
  let result = f();
  println!("调用 FnOnce trait: {}", result);
  result
}

fn analyze_monomorphization() {
  println!("\n--- 3.1.2 单态化过程分析 ---");

  // 编译器会为每个具体的闭包类型生成专门的代码
  let closure1 = |x: i32| x + 1;
  let closure2 = |x: i32| x * 2;

  // 即使签名相同，这两个闭包也有不同的类型
  process_closure_generic(closure1, 10);
  process_closure_generic(closure2, 10);

  // 展示泛型特化
  demonstrate_generic_specialization();
}

fn process_closure_generic<F>(f: F, input: i32) -> i32
where
  F: Fn(i32) -> i32,
{
  println!("处理闭包，输入: {}, 输出: {}", input, f(input));
  f(input)
}

fn demonstrate_generic_specialization() {
  println!("\n--- 泛型特化演示 ---");

  // 编译器会为每种闭包类型生成特化版本
  let add_closure = |x| x + 5;
  let mul_closure = |x| x * 3;
  let complex_closure = |x| {
    let temp = x * 2;
    temp + 1
  };

  // 每个调用都会被单态化为具体的函数
  apply_operation(add_closure, 10);
  apply_operation(mul_closure, 10);
  apply_operation(complex_closure, 10);
}

fn apply_operation<F>(op: F, value: i32) -> i32
where
  F: Fn(i32) -> i32,
{
  op(value)
}

fn compare_dispatch_methods() {
  println!("\n--- 3.1.3 静态分发 vs 动态分发 ---");

  // 静态分发：编译时确定具体类型
  static_dispatch_example();

  // 动态分发：运行时通过虚表调用
  dynamic_dispatch_example();
}

fn static_dispatch_example() {
  println!("\n静态分发示例:");

  let closure1 = |x: i32| x + 1;
  let closure2 = |x: i32| x * 2;

  // 编译器知道确切的类型，可以内联优化
  let result1 = static_call(closure1, 5);
  let result2 = static_call(closure2, 5);

  println!("静态分发结果: {}, {}", result1, result2);
}

fn static_call<F>(f: F, x: i32) -> i32
where
  F: Fn(i32) -> i32,
{
  f(x) // 编译时确定，可能被内联
}

fn dynamic_dispatch_example() {
  println!("\n动态分发示例:");

  let closures: Vec<Box<dyn Fn(i32) -> i32>> = vec![
    Box::new(|x| x + 1),
    Box::new(|x| x * 2),
    Box::new(|x| x * x),
  ];

  // 运行时通过虚表调用
  for (i, closure) in closures.iter().enumerate() {
    let result = dynamic_call(closure.as_ref(), 5);
    println!("动态分发闭包 {}: {}", i, result);
  }
}

fn dynamic_call(f: &dyn Fn(i32) -> i32, x: i32) -> i32 {
  f(x) // 运行时通过虚表调用
}

/// ## 3.2 传统的 Trait 详解
///
/// 在理解了实现原理后，我们来详细看看三个基本 trait。
pub fn closure_traits_explanation() {
  println!("\n=== 5. 闭包的三种 Trait ===");

  println!("\n闭包根据如何使用捕获的变量，自动实现以下 trait 之一：");
  println!("- FnOnce: 只能调用一次，会消费捕获的变量");
  println!("- FnMut: 可以多次调用，可以修改捕获的变量");
  println!("- Fn: 可以多次调用，只能不可变地访问捕获的变量");

  println!("\n继承关系: Fn: FnMut: FnOnce");
  println!("即：实现 Fn 的闭包也实现 FnMut 和 FnOnce");
}

/// # 5.1 FnOnce 示例
///
/// FnOnce 闭包只能被调用一次，因为它会消费捕获的变量
pub fn fn_once_examples() {
  println!("\n=== 5.1 FnOnce 示例 ===");

  // 示例1：消费 String
  {
    println!("\n--- 示例1：消费 String ---");
    let s = String::from("hello");

    let consume_string = || {
      println!("消费字符串: {}", s);
      drop(s); // 显式消费
    };

    consume_string(); // 只能调用一次
                      // consume_string(); // 编译错误：已经被消费
  }

  // 示例2：返回捕获的值
  {
    println!("\n--- 示例2：返回捕获的值 ---");
    let data = vec![1, 2, 3];

    let return_data = || data; // 返回 data，转移所有权

    let result = return_data();
    println!("返回的数据: {:?}", result);
    // return_data(); // 编译错误：只能调用一次
  }

  // 示例3：使用 FnOnce 作为参数
  fn call_once<F>(f: F) -> String
  where
    F: FnOnce() -> String,
  {
    f()
  }

  let message = String::from("Hello, World!");
  let closure = || message;

  let result = call_once(closure);
  println!("FnOnce 调用结果: {}", result);
}

/// # 5.2 FnMut 示例
///
/// FnMut 闭包可以多次调用，并且可以修改捕获的变量
pub fn fn_mut_examples() {
  println!("\n=== 5.2 FnMut 示例 ===");

  // 示例1：修改捕获的变量
  {
    println!("\n--- 示例1：修改捕获的变量 ---");
    let mut count = 0;

    let mut increment = || {
      count += 1;
      println!("当前计数: {}", count);
      count
    };

    increment();
    increment();
    increment();
  }

  // 示例2：修改 Vec
  {
    println!("\n--- 示例2：修改 Vec ---");
    let mut numbers = vec![1, 2, 3];

    let mut add_number = |n| {
      numbers.push(n);
      println!("添加 {} 后的向量: {:?}", n, numbers);
    };

    add_number(4);
    add_number(5);
    add_number(6);
  }

  // 示例3：使用 FnMut 作为参数
  fn call_multiple_times<F>(mut f: F)
  where
    F: FnMut(i32) -> i32,
  {
    for i in 1..=3 {
      let result = f(i);
      println!("调用 {} 次，结果: {}", i, result);
    }
  }

  let mut accumulator = 0;
  let accumulate = |x| {
    accumulator += x;
    accumulator
  };

  call_multiple_times(accumulate);
}

/// # 5.3 Fn 示例
///
/// Fn 闭包可以多次调用，但只能不可变地访问捕获的变量
pub fn fn_examples() {
  println!("\n=== 5.3 Fn 示例 ===");

  // 示例1：只读访问
  {
    println!("\n--- 示例1：只读访问 ---");
    let message = String::from("Hello, Rust!");

    let print_message = || {
      println!("消息: {}", message);
      message.len()
    };

    println!("消息长度: {}", print_message());
    println!("消息长度: {}", print_message());
    println!("原始消息仍可用: {}", message);
  }

  // 示例2：计算函数
  {
    println!("\n--- 示例2：计算函数 ---");
    let multiplier = 3;

    let multiply_by = |x| x * multiplier;

    println!("5 * {} = {}", multiplier, multiply_by(5));
    println!("10 * {} = {}", multiplier, multiply_by(10));
  }

  // 示例3：使用 Fn 作为参数
  fn apply_twice<F>(f: F, x: i32) -> i32
  where
    F: Fn(i32) -> i32,
  {
    f(f(x))
  }

  let double = |x| x * 2;
  let result = apply_twice(double, 5);
  println!("应用两次 double(5): {}", result); // 5 * 2 * 2 = 20
}

/// # 6. 闭包作为函数参数
///
/// 演示如何将闭包作为参数传递给函数
pub fn closures_as_parameters() {
  println!("\n=== 6. 闭包作为函数参数 ===");

  // 6.1 接受 Fn 的函数
  fn apply_to_vec<F>(vec: &[i32], f: F) -> Vec<i32>
  where
    F: Fn(i32) -> i32,
  {
    vec.iter().map(|&x| f(x)).collect()
  }

  let numbers = vec![1, 2, 3, 4, 5];

  let doubled = apply_to_vec(&numbers, |x| x * 2);
  println!("原数组: {:?}", numbers);
  println!("翻倍后: {:?}", doubled);

  let squared = apply_to_vec(&numbers, |x| x * x);
  println!("平方后: {:?}", squared);

  // 6.2 接受 FnMut 的函数
  fn modify_vec<F>(vec: &mut Vec<i32>, mut f: F)
  where
    F: FnMut(&mut i32),
  {
    for item in vec.iter_mut() {
      f(item);
    }
  }

  let mut numbers = vec![1, 2, 3, 4, 5];
  println!("\n修改前: {:?}", numbers);

  modify_vec(&mut numbers, |x| *x *= 2);
  println!("翻倍后: {:?}", numbers);

  modify_vec(&mut numbers, |x| *x += 10);
  println!("加10后: {:?}", numbers);

  // 6.3 接受 FnOnce 的函数
  fn consume_and_process<F, T>(f: F) -> T
  where
    F: FnOnce() -> T,
  {
    println!("准备执行闭包...");
    f()
  }

  let data = String::from("重要数据");
  let result = consume_and_process(|| {
    println!("处理数据: {}", data);
    data.len()
  });

  println!("处理结果: {}", result);
}

/// # 7. 闭包作为返回值
///
/// 演示如何从函数中返回闭包
pub fn closures_as_return_values() {
  println!("\n=== 7. 闭包作为返回值 ===");

  // 7.1 返回简单闭包
  fn create_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
  }

  let add_5 = create_adder(5);
  println!("add_5(10) = {}", add_5(10));
  println!("add_5(20) = {}", add_5(20));

  // 7.2 返回复杂闭包
  fn create_multiplier_and_formatter(factor: i32) -> impl Fn(i32) -> String {
    move |x| format!("{}×{} = {}", x, factor, x * factor)
  }

  let multiply_by_3 = create_multiplier_and_formatter(3);
  println!("{}", multiply_by_3(7));
  println!("{}", multiply_by_3(12));

  // 7.3 返回带状态的闭包
  fn create_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
      count += 1;
      count
    }
  }

  let mut counter1 = create_counter();
  let mut counter2 = create_counter();

  println!("计数器1: {}", counter1());
  println!("计数器1: {}", counter1());
  println!("计数器2: {}", counter2());
  println!("计数器1: {}", counter1());
}

/// # 第四部分：高级应用与模式

/// ## 4.1 函数式编程模式
///
/// 闭包是函数式编程的核心工具，让我们探索高级的函数式编程模式。
pub fn functional_programming_patterns() {
  println!("\n=== 第四部分：高级应用与模式 ===");

  // 高阶函数模式
  higher_order_functions();

  // 柯里化和偏应用
  currying_and_partial_application();

  // 组合子模式
  combinator_patterns();

  // 惰性求值
  lazy_evaluation_patterns();
}

fn higher_order_functions() {
  println!("\n--- 4.1.1 高阶函数模式 ---");

  // 返回闭包的函数
  let multiplier = create_multiplier(3);
  println!("乘法器结果: {}", multiplier(10));

  // 接受多个闭包的函数
  let result = compose_operations(|x| x * 2, |x| x + 5, |x| x * x, 10);
  println!("组合操作结果: {}", result);

  // 条件执行模式
  conditional_execution_demo();
}

fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
  move |x| x * factor
}

fn compose_operations<F1, F2, F3>(f1: F1, f2: F2, f3: F3, input: i32) -> i32
where
  F1: Fn(i32) -> i32,
  F2: Fn(i32) -> i32,
  F3: Fn(i32) -> i32,
{
  f3(f2(f1(input)))
}

fn conditional_execution_demo() {
  println!("\n条件执行模式:");

  let condition = true;
  let result = if_then_else(
    condition,
    || {
      println!("执行真分支");
      42
    },
    || {
      println!("执行假分支");
      0
    },
  );
  println!("条件执行结果: {}", result);
}

fn if_then_else<T, F1, F2>(condition: bool, then_branch: F1, else_branch: F2) -> T
where
  F1: FnOnce() -> T,
  F2: FnOnce() -> T,
{
  if condition {
    then_branch()
  } else {
    else_branch()
  }
}

fn currying_and_partial_application() {
  println!("\n--- 4.1.2 柯里化和偏应用 ---");

  // 柯里化：将多参数函数转换为单参数函数链
  let add_curried = curry_add();
  let add_5 = add_curried(5);
  println!("柯里化加法: 5 + 3 = {}", add_5(3));

  // 偏应用：固定部分参数
  let multiply_by_2 = partial_multiply(2);
  println!("偏应用乘法: 2 * 7 = {}", multiply_by_2(7));

  // 复杂的柯里化示例
  complex_currying_example();
}

fn curry_add() -> impl Fn(i32) -> Box<dyn Fn(i32) -> i32> {
  |x| Box::new(move |y| x + y)
}

fn partial_multiply(factor: i32) -> impl Fn(i32) -> i32 {
  move |x| x * factor
}

fn complex_currying_example() {
  println!("\n复杂柯里化示例:");

  // 三参数函数的柯里化
  let calculate = |a| move |b| move |c| a * b + c;

  let step1 = calculate(2); // 固定 a = 2
  let step2 = step1(3); // 固定 b = 3
  let result = step2(4); // 计算 2 * 3 + 4

  println!("柯里化计算: 2 * 3 + 4 = {}", result);
}

fn combinator_patterns() {
  println!("\n--- 4.1.3 组合子模式 ---");

  // Option 组合子
  option_combinators();

  // Result 组合子
  result_combinators();

  // 自定义组合子
  custom_combinators();
}

fn option_combinators() {
  println!("\nOption 组合子:");

  let numbers = vec![Some(1), None, Some(3), Some(4), None];

  // 使用 map 和 filter 组合子
  let processed: Vec<i32> = numbers
    .into_iter()
    .filter_map(|opt| opt.map(|x| x * 2))
    .filter(|&x| x > 2)
    .collect();

  println!("处理后的数字: {:?}", processed);
}

fn result_combinators() {
  println!("\nResult 组合子:");

  let operations: Vec<Result<i32, &str>> = vec![Ok(10), Err("错误1"), Ok(20), Ok(30)];

  // 链式操作
  let result: Result<Vec<i32>, &str> = operations.into_iter().map(|r| r.map(|x| x * 2)).collect();

  match result {
    Ok(values) => println!("成功处理: {:?}", values),
    Err(e) => println!("处理失败: {}", e),
  }
}

fn custom_combinators() {
  println!("\n自定义组合子:");

  // 创建一个可能失败的计算链
  let result = safe_divide(10.0, 2.0)
    .and_then(|x| safe_sqrt(x))
    .and_then(|x| safe_log(x));

  match result {
    Ok(value) => println!("计算结果: {:.2}", value),
    Err(e) => println!("计算错误: {}", e),
  }
}

fn safe_divide(a: f64, b: f64) -> Result<f64, &'static str> {
  if b != 0.0 {
    Ok(a / b)
  } else {
    Err("除零错误")
  }
}

fn safe_sqrt(x: f64) -> Result<f64, &'static str> {
  if x >= 0.0 {
    Ok(x.sqrt())
  } else {
    Err("负数开方错误")
  }
}

fn safe_log(x: f64) -> Result<f64, &'static str> {
  if x > 0.0 {
    Ok(x.ln())
  } else {
    Err("非正数取对数错误")
  }
}

fn lazy_evaluation_patterns() {
  println!("\n--- 4.1.4 惰性求值模式 ---");

  // 惰性计算
  lazy_computation_demo();

  // 无限序列
  infinite_sequence_demo();
}

fn lazy_computation_demo() {
  println!("\n惰性计算演示:");

  // 只有在需要时才计算
  let expensive_computation = || {
    println!("执行昂贵的计算...");
    (1..10000).sum::<i32>()
  };

  println!("创建了惰性计算");

  // 条件执行
  let should_compute = true;
  if should_compute {
    let result = expensive_computation();
    println!("计算结果: {}", result);
  }
}

fn infinite_sequence_demo() {
  println!("\n无限序列演示:");

  // 斐波那契数列生成器
  let fib_gen = fibonacci_generator();

  println!("前10个斐波那契数:");
  for (i, num) in fib_gen.take(10).enumerate() {
    println!("fib({}) = {}", i, num);
  }
}

fn fibonacci_generator() -> impl Iterator<Item = u64> {
  let mut a = 0;
  let mut b = 1;

  std::iter::from_fn(move || {
    let next = a;
    let temp = a + b;
    a = b;
    b = temp;
    Some(next)
  })
}

/// ## 4.2 异步编程中的闭包
///
/// 闭包在异步编程中扮演着重要角色，特别是在 Future 和 async/await 中。
pub fn async_programming_with_closures() {
  println!("\n=== 4.2 异步编程中的闭包 ===");

  // 模拟异步操作
  simulate_async_operations();

  // 回调模式
  callback_patterns();

  // 事件处理
  event_handling_patterns();
}

fn simulate_async_operations() {
  println!("\n--- 4.2.1 模拟异步操作 ---");

  // 模拟异步任务
  let async_task = || {
    println!("开始异步任务...");
    thread::sleep(Duration::from_millis(100));
    println!("异步任务完成");
    42
  };

  // 在新线程中执行
  let handle = thread::spawn(async_task);
  let result = handle.join().unwrap();
  println!("异步任务结果: {}", result);
}

fn callback_patterns() {
  println!("\n--- 4.2.2 回调模式 ---");

  // 成功和失败回调
  execute_with_callbacks(
    || {
      // 模拟可能失败的操作
      if true {
        Ok("操作成功")
      } else {
        Err("操作失败")
      }
    },
    |result| println!("成功回调: {}", result),
    |error| println!("失败回调: {}", error),
  );
}

fn execute_with_callbacks<T, E, Op, OnSuccess, OnError>(
  operation: Op,
  on_success: OnSuccess,
  on_error: OnError,
) where
  Op: FnOnce() -> Result<T, E>,
  OnSuccess: FnOnce(T),
  OnError: FnOnce(E),
{
  match operation() {
    Ok(result) => on_success(result),
    Err(error) => on_error(error),
  }
}

fn event_handling_patterns() {
  println!("\n--- 4.2.3 事件处理模式 ---");

  // 模拟事件系统
  let mut event_system = EventSystem::new();

  // 注册事件处理器
  event_system.on_event("user_login", |data| {
    println!("用户登录事件: {}", data);
  });

  event_system.on_event("user_logout", |data| {
    println!("用户登出事件: {}", data);
  });

  // 触发事件
  event_system.emit_event("user_login", "用户Alice");
  event_system.emit_event("user_logout", "用户Alice");
}

struct EventSystem {
  handlers: HashMap<String, Box<dyn Fn(&str)>>,
}

impl EventSystem {
  fn new() -> Self {
    EventSystem {
      handlers: HashMap::new(),
    }
  }

  fn on_event<F>(&mut self, event_type: &str, handler: F)
  where
    F: Fn(&str) + 'static,
  {
    self
      .handlers
      .insert(event_type.to_string(), Box::new(handler));
  }

  fn emit_event(&self, event_type: &str, data: &str) {
    if let Some(handler) = self.handlers.get(event_type) {
      handler(data);
    }
  }
}

/// ## 4.3 传统的实际应用场景
///
/// 展示闭包在实际编程中的常见用法
pub fn practical_closure_examples() {
  println!("\n=== 8. 实际应用场景 ===");

  // 8.1 迭代器与闭包
  {
    println!("\n--- 8.1 迭代器与闭包 ---");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 过滤偶数并平方
    let even_squares: Vec<i32> = numbers
      .iter()
      .filter(|&&x| x % 2 == 0)
      .map(|&x| x * x)
      .collect();

    println!("偶数的平方: {:?}", even_squares);

    // 查找第一个大于5的数
    let first_gt_5 = numbers.iter().find(|&&x| x > 5);
    println!("第一个大于5的数: {:?}", first_gt_5);

    // 计算所有数的和
    let sum: i32 = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("所有数的和: {}", sum);
  }

  // 8.2 错误处理与闭包
  {
    println!("\n--- 8.2 错误处理与闭包 ---");

    fn divide_numbers(numbers: Vec<f64>, divisor: f64) -> Vec<Result<f64, String>> {
      numbers
        .into_iter()
        .map(|n| {
          if divisor == 0.0 {
            Err("除数不能为零".to_string())
          } else {
            Ok(n / divisor)
          }
        })
        .collect()
    }

    let numbers = vec![10.0, 20.0, 30.0, 40.0];
    let results = divide_numbers(numbers, 2.0);

    for (i, result) in results.iter().enumerate() {
      match result {
        Ok(value) => println!("结果 {}: {:.2}", i + 1, value),
        Err(e) => println!("错误 {}: {}", i + 1, e),
      }
    }
  }

  // 8.3 配置和策略模式
  {
    println!("\n--- 8.3 配置和策略模式 ---");

    struct Calculator {
      operation: Box<dyn Fn(f64, f64) -> f64>,
    }

    impl Calculator {
      fn new<F>(op: F) -> Self
      where
        F: Fn(f64, f64) -> f64 + 'static,
      {
        Calculator {
          operation: Box::new(op),
        }
      }

      fn calculate(&self, a: f64, b: f64) -> f64 {
        (self.operation)(a, b)
      }
    }

    let adder = Calculator::new(|a, b| a + b);
    let multiplier = Calculator::new(|a, b| a * b);
    let power = Calculator::new(|a, b| a.powf(b));

    println!("5 + 3 = {}", adder.calculate(5.0, 3.0));
    println!("5 * 3 = {}", multiplier.calculate(5.0, 3.0));
    println!("5^3 = {}", power.calculate(5.0, 3.0));
  }

  // 8.4 缓存和记忆化
  {
    println!("\n--- 8.4 缓存和记忆化 ---");

    struct Memoizer<F, Arg, Ret>
    where
      F: Fn(Arg) -> Ret,
      Arg: Clone + std::hash::Hash + Eq,
      Ret: Clone,
    {
      func: F,
      cache: std::cell::RefCell<HashMap<Arg, Ret>>,
    }

    impl<F, Arg, Ret> Memoizer<F, Arg, Ret>
    where
      F: Fn(Arg) -> Ret,
      Arg: Clone + std::hash::Hash + Eq,
      Ret: Clone,
    {
      fn new(func: F) -> Self {
        Memoizer {
          func,
          cache: std::cell::RefCell::new(HashMap::new()),
        }
      }

      fn call(&self, arg: Arg) -> Ret {
        let mut cache = self.cache.borrow_mut();

        if let Some(result) = cache.get(&arg) {
          println!("缓存命中: {}", "value");
          result.clone()
        } else {
          println!("计算中: {}", "value");
          let result = (self.func)(arg.clone());
          cache.insert(arg, result.clone());
          result
        }
      }
    }

    // 斐波那契数列的记忆化版本
    fn fibonacci(n: u32) -> u64 {
      match n {
        0 => 0,
        1 => 1,
        _ => {
          // 这里为了演示简化，实际应该递归调用记忆化版本
          thread::sleep(Duration::from_millis(100)); // 模拟耗时计算
          fibonacci(n - 1) + fibonacci(n - 2)
        }
      }
    }

    let memo_fib = Memoizer::new(|n: u32| {
      if n <= 1 {
        n as u64
      } else {
        n as u64 * 2
      } // 简化版本
    });

    println!("斐波那契(5): {}", memo_fib.call(5));
    println!("斐波那契(5): {}", memo_fib.call(5)); // 应该命中缓存
    println!("斐波那契(10): {}", memo_fib.call(10));
  }
}

/// # 第五部分：性能优化与最佳实践

/// ## 5.1 性能分析与优化
///
/// 深入分析闭包的性能特征，提供优化策略和最佳实践。
pub fn performance_optimization_guide() {
  println!("\n=== 第五部分：性能优化与最佳实践 ===");

  // 内存布局分析
  memory_layout_analysis();

  // 性能基准测试
  performance_benchmarks();

  // 优化策略
  optimization_strategies();

  // 最佳实践
  best_practices_guide();
}

fn memory_layout_analysis() {
  println!("\n--- 5.1.1 内存布局分析 ---");

  // 分析不同闭包的内存占用
  analyze_closure_sizes();

  // 捕获变量的内存影响
  analyze_capture_memory_impact();
}

fn analyze_closure_sizes() {
  println!("\n闭包大小分析:");

  // 无捕获闭包
  let no_capture = || 42;
  println!(
    "无捕获闭包大小: {} bytes",
    std::mem::size_of_val(&no_capture)
  );

  // 捕获单个值
  let x = 10i32;
  let single_capture = move || x;
  println!(
    "单值捕获闭包大小: {} bytes",
    std::mem::size_of_val(&single_capture)
  );

  // 捕获多个值
  let y = 20i64;
  let z = 30u8;
  let multi_capture = move || (x, y, z);
  println!(
    "多值捕获闭包大小: {} bytes",
    std::mem::size_of_val(&multi_capture)
  );

  // 捕获大型数据结构
  let large_vec = vec![1; 1000];
  let large_capture = move || large_vec.len();
  println!(
    "大型数据捕获闭包大小: {} bytes",
    std::mem::size_of_val(&large_capture)
  );
}

fn analyze_capture_memory_impact() {
  println!("\n捕获方式的内存影响:");

  let data = vec![1, 2, 3, 4, 5];

  // 借用捕获
  let borrow_closure = || data.len();
  println!(
    "借用捕获闭包大小: {} bytes",
    std::mem::size_of_val(&borrow_closure)
  );

  // 移动捕获
  let data_clone = data.clone();
  let move_closure = move || data_clone.len();
  println!(
    "移动捕获闭包大小: {} bytes",
    std::mem::size_of_val(&move_closure)
  );

  // 引用捕获
  let data_ref = &data;
  let ref_closure = move || data_ref.len();
  println!(
    "引用捕获闭包大小: {} bytes",
    std::mem::size_of_val(&ref_closure)
  );
}

fn performance_benchmarks() {
  println!("\n--- 5.1.2 性能基准测试 ---");

  // 函数调用 vs 闭包调用
  function_vs_closure_benchmark();

  // 静态分发 vs 动态分发
  dispatch_benchmark();

  // 内联优化效果
  inlining_benchmark();
}

fn function_vs_closure_benchmark() {
  println!("\n函数 vs 闭包性能对比:");

  let iterations = 1_000_000;

  // 普通函数
  let start = std::time::Instant::now();
  for i in 0..iterations {
    let _ = regular_function(i);
  }
  let function_time = start.elapsed();

  // 闭包
  let closure = |x: i32| x * 2 + 1;
  let start = std::time::Instant::now();
  for i in 0..iterations {
    let _ = closure(i);
  }
  let closure_time = start.elapsed();

  println!("函数调用时间: {:?}", function_time);
  println!("闭包调用时间: {:?}", closure_time);
  println!(
    "性能比率: {:.2}",
    closure_time.as_nanos() as f64 / function_time.as_nanos() as f64
  );
}

fn regular_function(x: i32) -> i32 {
  x * 2 + 1
}

fn dispatch_benchmark() {
  println!("\n静态分发 vs 动态分发性能对比:");

  let iterations = 100_000;
  let closure = |x: i32| x * 2;

  // 静态分发
  let start = std::time::Instant::now();
  for i in 0..iterations {
    let _ = static_dispatch_call(&closure, i);
  }
  let static_time = start.elapsed();

  // 动态分发
  let dyn_closure: &dyn Fn(i32) -> i32 = &closure;
  let start = std::time::Instant::now();
  for i in 0..iterations {
    let _ = dynamic_dispatch_call(dyn_closure, i);
  }
  let dynamic_time = start.elapsed();

  println!("静态分发时间: {:?}", static_time);
  println!("动态分发时间: {:?}", dynamic_time);
  println!(
    "性能差异: {:.2}x",
    dynamic_time.as_nanos() as f64 / static_time.as_nanos() as f64
  );
}

fn static_dispatch_call<F>(f: F, x: i32) -> i32
where
  F: Fn(i32) -> i32,
{
  f(x)
}

fn dynamic_dispatch_call(f: &dyn Fn(i32) -> i32, x: i32) -> i32 {
  f(x)
}

fn inlining_benchmark() {
  println!("\n内联优化效果演示:");

  // 简单闭包（容易被内联）
  let simple_closure = |x: i32| x + 1;

  // 复杂闭包（不太容易被内联）
  let complex_closure = |x: i32| {
    let mut result = x;
    for _ in 0..10 {
      result = result * 2 + 1;
      result = result % 1000;
    }
    result
  };

  println!("简单闭包适合内联优化");
  println!("复杂闭包可能不会被内联");

  // 实际使用
  let _ = simple_closure(42);
  let _ = complex_closure(42);
}

fn optimization_strategies() {
  println!("\n--- 5.1.3 优化策略 ---");

  // 避免不必要的捕获
  avoid_unnecessary_captures();

  // 选择合适的捕获方式
  choose_optimal_capture_mode();

  // 使用适当的 trait 边界
  use_appropriate_trait_bounds();
}

fn avoid_unnecessary_captures() {
  println!("\n避免不必要的捕获:");

  let large_data = vec![0; 10000];
  let small_value = 42;

  // 不好的做法：捕获整个大型数据结构
  // let bad_closure = move || {
  //     println!("只需要: {}", small_value);
  //     large_data  // 不必要地移动了大型数据
  // };

  // 好的做法：只捕获需要的部分
  let needed_value = small_value;
  let good_closure = move || {
    println!("只需要: {}", needed_value);
  };

  good_closure();
  println!("优化：只捕获真正需要的数据");
}

fn choose_optimal_capture_mode() {
  println!("\n选择最优捕获方式:");

  let data = vec![1, 2, 3, 4, 5];

  // 只读访问：使用借用
  let read_only = || {
    println!("数据长度: {}", data.len());
  };
  read_only();

  // 需要所有权：使用 move
  let data_clone = data.clone();
  let take_ownership = move || data_clone.into_iter().sum::<i32>();
  println!("数据总和: {}", take_ownership());

  println!("根据使用场景选择合适的捕获方式");
}

fn use_appropriate_trait_bounds() {
  println!("\n使用适当的 trait 边界:");

  // 如果只调用一次，使用 FnOnce
  let once_result = call_once(|| {
    println!("这个闭包只会被调用一次");
    42
  });

  // 如果需要多次调用，使用 Fn
  let multi_result = call_multiple(|| {
    println!("这个闭包可以被多次调用");
    24
  });

  println!("FnOnce 结果: {}", once_result);
  println!("Fn 结果: {}", multi_result);
}

fn call_once<F>(f: F) -> i32
where
  F: FnOnce() -> i32,
{
  f()
}

fn call_multiple<F>(f: F) -> i32
where
  F: Fn() -> i32,
{
  f() + f() // 调用两次
}

fn best_practices_guide() {
  println!("\n--- 5.1.4 最佳实践指南 ---");

  // 代码组织最佳实践
  code_organization_practices();

  // 错误处理最佳实践
  error_handling_practices();

  // 测试最佳实践
  testing_practices();
}

fn code_organization_practices() {
  println!("\n代码组织最佳实践:");

  // 1. 保持闭包简洁
  let numbers = vec![1, 2, 3, 4, 5];

  // 好的做法：简洁的闭包
  let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();

  // 2. 复杂逻辑提取为函数
  let processed: Vec<i32> = numbers.iter().map(|&x| complex_processing(x)).collect();

  println!("简洁闭包结果: {:?}", doubled);
  println!("复杂处理结果: {:?}", processed);
}

fn complex_processing(x: i32) -> i32 {
  // 复杂的处理逻辑
  let mut result = x;
  result = result * 3;
  result = result + 10;
  result = result % 100;
  result
}

fn error_handling_practices() {
  println!("\n错误处理最佳实践:");

  let numbers = vec!["1", "2", "invalid", "4"];

  // 使用 filter_map 处理可能的错误
  let parsed: Vec<i32> = numbers.iter().filter_map(|s| s.parse().ok()).collect();

  println!("成功解析的数字: {:?}", parsed);

  // 使用 Result 收集所有错误
  let results: Result<Vec<i32>, _> = numbers.iter().map(|s| s.parse::<i32>()).collect();

  match results {
    Ok(nums) => println!("所有解析成功: {:?}", nums),
    Err(e) => println!("解析失败: {}", e),
  }
}

fn testing_practices() {
  println!("\n测试最佳实践:");

  // 测试闭包行为
  let add_one = |x| x + 1;
  assert_eq!(add_one(5), 6);

  // 测试高阶函数
  let result = apply_twice(|x| x * 2, 5);
  assert_eq!(result, 20);

  println!("所有测试通过");
}

fn apply_twice<F>(f: F, x: i32) -> i32
where
  F: Fn(i32) -> i32,
{
  f(f(x))
}

/// ## 5.2 传统的性能考虑
///
/// 在理解了优化策略后，我们来看看传统的性能考虑。
pub fn closure_performance_considerations() {
  println!("\n=== 9. 闭包的性能考虑 ===");

  println!("\n闭包的性能特点:");
  println!("1. 零成本抽象：编译器会内联简单闭包");
  println!("2. 捕获成本：只捕获实际使用的变量");
  println!("3. 类型大小：闭包的大小取决于捕获的变量");

  // 9.1 零成本抽象示例
  {
    println!("\n--- 9.1 零成本抽象示例 ---");

    let numbers = vec![1, 2, 3, 4, 5];

    // 使用闭包的版本
    let sum_with_closure: i32 = numbers.iter().map(|&x| x * 2).sum();

    // 等价的手写循环版本
    let mut sum_manual = 0;
    for &x in &numbers {
      sum_manual += x * 2;
    }

    println!("闭包版本结果: {}", sum_with_closure);
    println!("手写循环结果: {}", sum_manual);
    println!("编译器会将两者优化为相同的机器码");
  }

  // 9.2 捕获变量的影响
  {
    println!("\n--- 9.2 捕获变量的影响 ---");

    let _large_data = vec![0; 1000000]; // 大型数据
    let small_value = 42;

    // 只捕获需要的变量
    let efficient_closure = |x| x + small_value;

    // 避免不必要的捕获
    // let inefficient_closure = |x| {
    //   let _ = &large_data; // 不必要的捕获
    //   x + small_value
    // };

    println!("高效闭包结果: {}", efficient_closure(10));
    println!("建议：只捕获实际需要的变量");
  }
}

/// # 10. 常见错误和最佳实践
///
/// 总结使用闭包时的常见错误和最佳实践
pub fn closure_best_practices() {
  println!("\n=== 10. 常见错误和最佳实践 ===");

  println!("\n常见错误:");
  println!("1. 混淆闭包的三种 trait");
  println!("2. 不必要的 move 关键字");
  println!("3. 生命周期问题");
  println!("4. 过度捕获变量");

  println!("\n最佳实践:");
  println!("1. 优先使用 Fn，其次 FnMut，最后 FnOnce");
  println!("2. 只在必要时使用 move");
  println!("3. 保持闭包简洁");
  println!("4. 合理使用类型注解");
  println!("5. 注意闭包的生命周期");

  // 示例：正确的闭包使用模式
  {
    println!("\n--- 正确的闭包使用模式 ---");

    // 好的做法：明确的类型和简洁的逻辑
    fn process_data<F>(data: &[i32], processor: F) -> Vec<i32>
    where
      F: Fn(i32) -> i32,
    {
      data.iter().map(|&x| processor(x)).collect()
    }

    let data = vec![1, 2, 3, 4, 5];

    // 简洁明了的闭包
    let doubled = process_data(&data, |x| x * 2);
    let incremented = process_data(&data, |x| x + 1);

    println!("原数据: {:?}", data);
    println!("翻倍: {:?}", doubled);
    println!("递增: {:?}", incremented);
  }
}

/// # 11. 高级闭包技巧
///
/// 展示一些高级的闭包使用技巧
pub fn advanced_closure_techniques() {
  println!("\n=== 11. 高级闭包技巧 ===");

  // 11.1 闭包组合
  {
    println!("\n--- 11.1 闭包组合 ---");

    fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
    where
      F: Fn(A) -> B,
      G: Fn(B) -> C,
    {
      move |x| g(f(x))
    }

    let add_one = |x: i32| x + 1;
    let double = |x: i32| x * 2;

    let add_one_then_double = compose(add_one, double);

    println!("compose(add_one, double)(5) = {}", add_one_then_double(5));
  }

  // 11.2 柯里化（Currying）
  {
    println!("\n--- 11.2 柯里化 ---");

    fn curry_add(x: i32) -> impl Fn(i32) -> i32 {
      move |y| x + y
    }

    let add_10 = curry_add(10);
    let add_5 = curry_add(5);

    println!("add_10(3) = {}", add_10(3));
    println!("add_5(7) = {}", add_5(7));
  }

  // 11.3 惰性求值
  {
    println!("\n--- 11.3 惰性求值 ---");

    struct LazyValue<F, T>
    where
      F: FnOnce() -> T,
    {
      func: Option<F>,
      value: Option<T>,
    }

    impl<F, T> LazyValue<F, T>
    where
      F: FnOnce() -> T,
    {
      fn new(func: F) -> Self {
        LazyValue {
          func: Some(func),
          value: None,
        }
      }

      fn get(&mut self) -> &T {
        if self.value.is_none() {
          let func = self.func.take().unwrap();
          self.value = Some(func());
        }
        self.value.as_ref().unwrap()
      }
    }

    let mut lazy_expensive_computation = LazyValue::new(|| {
      println!("执行昂贵的计算...");
      thread::sleep(Duration::from_millis(100));
      42
    });

    println!("惰性值创建完成，但尚未计算");
    println!("第一次获取值: {}", lazy_expensive_computation.get());
    println!("第二次获取值: {}", lazy_expensive_computation.get()); // 不会重新计算
  }
}

/// # 12. 测试函数
///
/// 提供测试用例来验证闭包的行为
pub fn run_closure_tests() {
  println!("\n=== 12. 闭包测试 ===");

  // 测试闭包类型推导
  {
    let closure = |x| x + 1;
    assert_eq!(closure(5), 6);
    println!("✓ 闭包类型推导测试通过");
  }

  // 测试闭包捕获
  {
    let multiplier = 3;
    let multiply = |x| x * multiplier;
    assert_eq!(multiply(4), 12);
    println!("✓ 闭包捕获测试通过");
  }

  // 测试 move 语义
  {
    let data = vec![1, 2, 3];
    let len = data.len();
    let closure = move || data.len();
    assert_eq!(closure(), len);
    println!("✓ move 语义测试通过");
  }

  // 测试 FnMut
  {
    let mut counter = 0;
    let mut increment = || {
      counter += 1;
      counter
    };

    assert_eq!(increment(), 1);
    assert_eq!(increment(), 2);
    assert_eq!(increment(), 3);
    println!("✓ FnMut 测试通过");
  }

  println!("\n所有测试通过！✅");
}

/// # 主函数：运行所有示例
pub fn run_all_examples() {
  println!("🦀 Rust 闭包全面学习指南 🦀");
  println!("基于 https://course.rs/advance/functional-programing/closure.html");
  println!("{}", "=".repeat(60));

  closure_compiler_implementation();
  closure_vs_function_pointer();
  basic_closure_syntax();
  closure_vs_function();
  capture_mechanism_deep_dive();
  closure_capture_modes();
  move_keyword_examples();
  fn_traits_deep_dive();
  closure_traits_explanation();
  fn_once_examples();
  fn_mut_examples();
  fn_examples();
  closures_as_parameters();
  closures_as_return_values();
  functional_programming_patterns();
  async_programming_with_closures();
  practical_closure_examples();
  performance_optimization_guide();
  closure_performance_considerations();
  closure_best_practices();
  advanced_closure_techniques();
  run_closure_tests();

  println!("\n{}", "=".repeat(60));
  println!("🎉 闭包学习指南完成！");
  println!("\n关键要点总结:");
  println!("1. 闭包是可以捕获环境变量的匿名函数");
  println!("2. 三种 trait：Fn > FnMut > FnOnce");
  println!("3. move 关键字强制获取所有权");
  println!("4. 闭包是零成本抽象");
  println!("5. 广泛应用于迭代器、错误处理、异步编程等场景");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_closure() {
    let add = |x, y| x + y;
    assert_eq!(add(2, 3), 5);
  }

  #[test]
  fn test_closure_capture() {
    let x = 10;
    let add_x = |y| x + y;
    assert_eq!(add_x(5), 15);
  }

  #[test]
  fn test_move_closure() {
    let data = vec![1, 2, 3];
    let len = data.len();
    let get_len = move || data.len();
    assert_eq!(get_len(), len);
  }

  #[test]
  fn test_fn_mut_closure() {
    let mut count = 0;
    let mut increment = || {
      count += 1;
      count
    };

    assert_eq!(increment(), 1);
    assert_eq!(increment(), 2);
  }

  #[test]
  fn test_closure_as_parameter() {
    fn apply<F>(f: F, x: i32) -> i32
    where
      F: Fn(i32) -> i32,
    {
      f(x)
    }

    let double = |x| x * 2;
    assert_eq!(apply(double, 5), 10);
  }
}
