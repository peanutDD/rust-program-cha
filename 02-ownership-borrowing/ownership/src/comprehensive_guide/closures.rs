//! # 第五部分：闭包详细解释
//!
//! 闭包是可以捕获其环境的匿名函数，与所有权系统紧密相关。

/// ## 第五部分：闭包详细解释
///
/// 闭包是可以捕获其环境的匿名函数，与所有权系统紧密相关。
pub fn closure_comprehensive_explanation() {
  println!("\n=== 第五部分：闭包详细解释 ===");

  closure_basics_and_syntax();
  closure_capture_modes();
  closure_traits_explanation();
  closure_ownership_interaction();
  closure_practical_examples();
}

/// ### 5.1 闭包基础与语法
///
/// 闭包的定义、语法和基本使用。
fn closure_basics_and_syntax() {
  println!("\n--- 5.1 闭包基础与语法 ---");

  println!("\n🔍 什么是闭包：");
  println!("• 闭包是可以捕获其环境的匿名函数");
  println!("• 可以保存在变量中或作为参数传递");
  println!("• 可以在一个地方创建，在另一个地方调用");
  println!("• 可以从定义它们的作用域中捕获值");

  println!("\n🔍 闭包语法：");

  // 基本闭包语法
  let simple_closure = |x| x + 1;
  println!("简单闭包: 5 + 1 = {}", simple_closure(5));

  // 带类型标注的闭包
  let typed_closure = |x: i32| -> i32 { x * 2 };
  println!("类型标注闭包: 5 * 2 = {}", typed_closure(5));

  // 多参数闭包
  let multi_param = |x, y| x + y;
  println!("多参数闭包: 3 + 4 = {}", multi_param(3, 4));

  // 无参数闭包
  let no_param = || {
    println!("无参数闭包被调用");
    42
  };
  println!("无参数闭包返回: {}", no_param());

  println!("\n🔍 闭包 vs 函数：");

  // 函数定义
  fn function_add(x: i32, y: i32) -> i32 {
    x + y
  }

  // 等价的闭包
  let closure_add = |x: i32, y: i32| -> i32 { x + y };

  println!("函数调用: {}", function_add(2, 3));
  println!("闭包调用: {}", closure_add(2, 3));

  println!("\n🔍 闭包类型推断：");

  // 编译器可以推断闭包的类型
  let inferred_closure = |x| x + 1;
  let result1 = inferred_closure(5); // 推断为 i32
  // let result2 = inferred_closure(5.0); // 编译错误！类型已确定
  println!("类型推断结果: {}", result1);

  // 显式类型标注
  let explicit_closure: fn(i32) -> i32 = |x| x + 1;
  println!("显式类型闭包: {}", explicit_closure(10));

  println!("\n💡 闭包特点：");
  println!("• 语法简洁，使用 |参数| 表达式");
  println!("• 可以捕获环境中的变量");
  println!("• 类型可以被推断");
  println!("• 每个闭包都有唯一的类型");
}

/// ### 5.2 闭包捕获模式
///
/// 闭包如何捕获环境中的变量。
fn closure_capture_modes() {
  println!("\n--- 5.2 闭包捕获模式 ---");

  println!("\n🔍 三种捕获模式：");
  println!("1. 不可变借用 (Fn)");
  println!("2. 可变借用 (FnMut)");
  println!("3. 获取所有权 (FnOnce)");

  println!("\n🔍 不可变借用捕获：");

  let x = 4;
  let equal_to_x = |z| z == x; // 闭包不可变借用 x

  println!("x = {}", x); // x 仍然可用
  let y = 4;
  println!("equal_to_x(y) = {}", equal_to_x(y));
  println!("x 仍然可用: {}", x);

  println!("\n🔍 可变借用捕获：");

  let mut list = vec![1, 2, 3];
  println!("调用闭包前: {:?}", list);

  let mut borrows_mutably = || list.push(7); // 闭包可变借用 list
  // println!("{:?}", list);  // 编译错误！list 被可变借用

  borrows_mutably();
  println!("调用闭包后: {:?}", list);

  println!("\n🔍 获取所有权捕获：");

  let list2 = vec![1, 2, 3];
  println!("move 前: {:?}", list2);

  let takes_ownership = move || {
    println!("闭包内部: {:?}", list2);
    list2 // 返回 list2，转移所有权
  };

  // println!("{:?}", list2);  // 编译错误！list2 已被移动
  let moved_list = takes_ownership();
  println!("move 后: {:?}", moved_list);

  println!("\n🔍 强制移动捕获：");

  let x = vec![1, 2, 3];
  let y = vec![4, 5, 6];

  // 使用 move 关键字强制移动
  let move_closure = move || {
    println!("x: {:?}", x);
    println!("y: {:?}", y);
  };

  // println!("{:?}", x);  // 编译错误！x 已被移动
  // println!("{:?}", y);  // 编译错误！y 已被移动

  move_closure();

  println!("\n🔍 捕获模式选择：");

  let data = vec![1, 2, 3, 4, 5];

  // 编译器自动选择最小权限的捕获模式
  let read_only = || {
    println!("只读访问: {:?}", data); // 不可变借用
  };

  read_only();

  println!("\n💡 捕获模式总结：");
  println!("• 编译器自动选择最小权限的捕获模式");
  println!("• move 关键字强制获取所有权");
  println!("• 捕获模式影响闭包的 trait 实现");
  println!("• 理解捕获模式有助于避免所有权问题");
}

/// ### 5.3 闭包 Trait 详解
///
/// Fn、FnMut、FnOnce 三个 trait 的详细解释。
fn closure_traits_explanation() {
  println!("\n--- 5.3 闭包 Trait 详解 ---");

  println!("\n🔍 三个闭包 Trait：");
  println!("• FnOnce: 只能调用一次的闭包");
  println!("• FnMut: 可以多次调用，可以修改捕获的变量");
  println!("• Fn: 可以多次调用，只能不可变访问捕获的变量");

  println!("\n🔍 FnOnce 示例：");

  let consume_closure = || {
    let data = vec![1, 2, 3];
    data // 返回 data，消费它
  };

  let result = consume_closure(); // 第一次调用
  println!("FnOnce 结果: {:?}", result);
  // consume_closure();  // 编译错误！不能再次调用

  println!("\n🔍 FnMut 示例：");

  let mut counter = 0;
  let mut increment = || {
    counter += 1;
    counter
  };

  println!("第一次调用: {}", increment());
  println!("第二次调用: {}", increment());
  println!("第三次调用: {}", increment());

  println!("\n🔍 Fn 示例：");

  let multiplier = 2;
  let multiply = |x| x * multiplier;

  println!("Fn 调用1: {}", multiply(5));
  println!("Fn 调用2: {}", multiply(10));
  println!("Fn 调用3: {}", multiply(15));

  println!("\n🔍 Trait 层次关系：");
  println!("• Fn: FnMut + FnOnce (最严格)");
  println!("• FnMut: FnOnce (中等)");
  println!("• FnOnce (最宽松)");

  // 演示 trait 约束
  demonstrate_closure_traits();
}

fn demonstrate_closure_traits() {
  println!("\n🔍 闭包 Trait 约束演示：");

  // 接受 FnOnce 的函数
  fn call_once<F>(f: F) -> i32
  where
    F: FnOnce() -> i32,
  {
    f()
  }

  // 接受 FnMut 的函数
  fn call_mut<F>(mut f: F) -> i32
  where
    F: FnMut() -> i32,
  {
    f() + f() // 调用两次
  }

  // 接受 Fn 的函数
  fn call_fn<F>(f: F) -> i32
  where
    F: Fn() -> i32,
  {
    f() + f() + f() // 调用三次
  }

  let value = 10;

  // Fn 闭包可以传递给所有函数
  let fn_closure = || value;
  println!("FnOnce 调用: {}", call_once(fn_closure));

  let fn_closure2 = || value;
  println!("FnMut 调用: {}", call_mut(fn_closure2));

  let fn_closure3 = || value;
  println!("Fn 调用: {}", call_fn(fn_closure3));

  // FnMut 闭包
  let mut counter = 0;
  let mut fnmut_closure = || {
    counter += 1;
    counter
  };

  // fnmut_closure 只能传递给 FnOnce 和 FnMut
  // println!("Fn 调用: {}", call_fn(fnmut_closure));  // 编译错误！
  println!("FnMut 调用: {}", call_mut(&mut fnmut_closure));
}

/// ### 5.4 闭包与所有权交互
///
/// 闭包如何与 Rust 的所有权系统交互。
fn closure_ownership_interaction() {
  println!("\n--- 5.4 闭包与所有权交互 ---");

  println!("\n🔍 闭包中的所有权转移：");

  let data = vec![1, 2, 3, 4, 5];

  // 闭包获取所有权
  let take_ownership = move || {
    println!("闭包拥有数据: {:?}", data);
    data.len() // 返回长度
  };

  // data 已被移动，不能再使用
  // println!("{:?}", data);  // 编译错误！

  let length = take_ownership();
  println!("数据长度: {}", length);

  println!("\n🔍 闭包返回引用：");

  let text = String::from("hello world");

  // 返回引用的闭包
  let get_reference = || -> &str {
    &text // 返回 text 的引用
  };

  let reference = get_reference();
  println!("引用内容: {}", reference);
  println!("原始数据仍可用: {}", text);

  println!("\n🔍 闭包与生命周期：");

  // 生命周期约束的闭包
  fn create_closure<'a>(s: &'a str) -> impl Fn() -> &'a str {
    move || s
  }

  let string = "static string";
  let closure = create_closure(string);
  println!("闭包返回: {}", closure());

  println!("\n🔍 闭包作为返回值：");

  // 返回闭包的函数
  fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
  }

  let add_5 = make_adder(5);
  println!("add_5(10) = {}", add_5(10));
  println!("add_5(20) = {}", add_5(20));

  // 返回不同类型闭包的函数
  fn make_closure(condition: bool) -> Box<dyn Fn(i32) -> i32> {
    if condition {
      Box::new(|x| x * 2)
    } else {
      Box::new(|x| x + 1)
    }
  }

  let closure1 = make_closure(true);
  let closure2 = make_closure(false);

  println!("条件闭包1: {}", closure1(5));
  println!("条件闭包2: {}", closure2(5));

  println!("\n💡 所有权交互要点：");
  println!("• move 关键字强制闭包获取所有权");
  println!("• 闭包可以返回引用，但需要满足生命周期约束");
  println!("• impl Trait 语法简化闭包返回类型");
  println!("• Box<dyn Trait> 用于返回不同类型的闭包");
}

/// ### 5.5 闭包实际应用示例
///
/// 闭包在实际编程中的常见应用场景。
fn closure_practical_examples() {
  println!("\n--- 5.5 闭包实际应用示例 ---");

  println!("\n🔍 迭代器与闭包：");

  let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

  // 过滤偶数
  let evens: Vec<i32> = numbers.iter().filter(|&x| x % 2 == 0).copied().collect();
  println!("偶数: {:?}", evens);

  // 映射操作
  let squares: Vec<i32> = numbers.iter().map(|x| x * x).collect();
  println!("平方: {:?}", squares);

  // 链式操作
  let result: Vec<i32> = numbers.iter().filter(|&x| *x > 5).map(|x| x * 2).collect();
  println!("大于5的数乘以2: {:?}", result);

  println!("\n🔍 错误处理与闭包：");

  let strings = vec!["1", "2", "not_a_number", "4", "5"];

  // 使用 filter_map 处理错误
  let parsed_numbers: Vec<i32> = strings.iter().filter_map(|s| s.parse().ok()).collect();
  println!("解析成功的数字: {:?}", parsed_numbers);

  // 使用 map 和 unwrap_or
  let with_defaults: Vec<i32> = strings.iter().map(|s| s.parse().unwrap_or(0)).collect();
  println!("带默认值的解析: {:?}", with_defaults);

  println!("\n🔍 自定义迭代器适配器：");

  // 创建自定义的迭代器适配器
  trait IteratorExt: Iterator {
    fn my_filter<P>(self, predicate: P) -> std::iter::Filter<Self, P>
    where
      Self: Sized,
      P: FnMut(&Self::Item) -> bool,
    {
      self.filter(predicate)
    }
  }

  impl<I: Iterator> IteratorExt for I {}

  let custom_result: Vec<i32> = (1..=10).my_filter(|&x| x % 3 == 0).collect();
  println!("自定义过滤器结果: {:?}", custom_result);

  println!("\n🔍 闭包作为配置：");

  // 使用闭包配置行为
  struct Processor<F>
  where
    F: Fn(i32) -> i32,
  {
    transform: F,
  }

  impl<F> Processor<F>
  where
    F: Fn(i32) -> i32,
  {
    fn new(transform: F) -> Self {
      Processor { transform }
    }

    // ✅ 优化：如果不需要消费 data，使用引用
    fn process(&self, data: &[i32]) -> Vec<i32> {
      data.iter().map(|&x| (self.transform)(x)).collect()
    }
    
    // 如果需要消费 data（移动语义），保留原版本
    #[allow(dead_code)]
    fn process_owned(&self, data: Vec<i32>) -> Vec<i32> {
      data.into_iter().map(&self.transform).collect()
    }
  }

  let doubler = Processor::new(|x| x * 2);
  let squared = Processor::new(|x| x * x);

  let data = vec![1, 2, 3, 4, 5];
  // ✅ 优化：使用引用，避免克隆
  println!("加倍处理: {:?}", doubler.process(&data));
  println!("平方处理: {:?}", squared.process(&data));
  println!("原始数据仍可用: {:?}", data);

  println!("\n💡 实际应用总结：");
  println!("• 迭代器方法大量使用闭包");
  println!("• 闭包简化错误处理逻辑");
  println!("• 可以创建灵活的配置系统");
  println!("• 函数式编程风格的核心工具");
}
