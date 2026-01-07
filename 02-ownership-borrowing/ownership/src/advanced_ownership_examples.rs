//! # 高级所有权示例和常见陷阱
//!
//! 基于 https://course.rs/basic/ownership/ownership.html 的深入分析
//! 包含更多实际场景中的所有权使用案例和常见错误解决方案

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// 演示所有权在实际开发中的应用场景
pub fn demonstrate_real_world_ownership() {
  println!("\n🌟 实际开发中的所有权应用场景");
  println!("{}", "=".repeat(60));

  // 1. 字符串处理中的所有权
  string_processing_ownership();

  // 2. 集合类型的所有权管理
  collection_ownership_patterns();

  // 3. 错误处理中的所有权
  error_handling_ownership();

  // 4. 异步编程中的所有权
  async_ownership_patterns();

  // 5. 性能优化中的所有权策略
  performance_ownership_strategies();
}

/// 字符串处理中的所有权模式
fn string_processing_ownership() {
  println!("\n--- 字符串处理中的所有权 ---");

  // 问题：不必要的字符串克隆
  #[allow(dead_code)]
  fn inefficient_string_concat(strings: Vec<String>) -> String {
    let mut result = String::new();
    for s in strings {
      result.push_str(&s.clone()); // 不必要的克隆
    }
    result
  }

  // 解决方案1：直接使用引用
  fn efficient_string_concat_v1(strings: &[String]) -> String {
    let mut result = String::new();
    for s in strings {
      result.push_str(s); // 直接使用引用
    }
    result
  }

  // 解决方案2：使用迭代器和 collect
  fn efficient_string_concat_v2(strings: &[String]) -> String {
    strings
      .iter()
      .map(|s| s.as_str())
      .collect::<Vec<&str>>()
      .join("")
  }

  // 解决方案3：使用 join 方法
  fn efficient_string_concat_v3(strings: &[String]) -> String {
    strings.join("")
  }

  let test_strings = vec![
    String::from("Hello"),
    String::from(" "),
    String::from("World"),
    String::from("!"),
  ];

  println!("原始数据: {:?}", test_strings);
  println!("拼接结果: {}", efficient_string_concat_v1(&test_strings));
  println!("使用迭代器: {}", efficient_string_concat_v2(&test_strings));
  println!("使用 join: {}", efficient_string_concat_v3(&test_strings));

  // 演示字符串切片的所有权
  demonstrate_string_slices();
}

/// 演示字符串切片的所有权特性
fn demonstrate_string_slices() {
  println!("\n🔍 字符串切片所有权分析:");

  let s = String::from("hello world");

  // 字符串切片不拥有数据
  let hello = &s[0..5]; // 借用 s 的一部分
  let world = &s[6..11]; // 借用 s 的另一部分

  println!("原字符串: {}", s);
  println!("切片1: {}", hello);
  println!("切片2: {}", world);

  // 字符串字面量的特殊性
  let literal: &str = "hello"; // 存储在程序的二进制文件中
  println!("字符串字面量: {}", literal);

  // 演示悬垂引用的预防
  // let dangling = create_dangling_reference(); // 这会编译失败
  let safe_string = create_safe_string();
  println!("安全的字符串创建: {}", safe_string);
}

// 这个函数会导致编译错误 - 悬垂引用
/*
fn create_dangling_reference() -> &str {
  let s = String::from("hello");
  &s // 错误：返回对局部变量的引用
} // s 在这里被丢弃，返回的引用变成悬垂引用
*/

// 正确的做法：返回拥有所有权的值
fn create_safe_string() -> String {
  let s = String::from("hello");
  s // 移动所有权给调用者
}

/// 集合类型的所有权管理模式
fn collection_ownership_patterns() {
  println!("\n--- 集合类型的所有权管理 ---");

  // Vec 的所有权转移
  demonstrate_vec_ownership();

  // HashMap 的所有权管理
  demonstrate_hashmap_ownership();

  // 嵌套集合的所有权
  demonstrate_nested_collections();
}

/// 演示 Vec 的所有权行为
fn demonstrate_vec_ownership() {
  println!("\n📦 Vec 所有权演示:");

  let vec1 = vec![1, 2, 3, 4, 5];
  println!("原始 vec: {:?}", vec1);

  // 移动整个 vec
  let vec2 = vec1;
  // println!("{:?}", vec1); // 编译错误：vec1 已被移动
  println!("移动后的 vec: {:?}", vec2);

  // 部分移动：从 vec 中取出元素
  let mut vec3 = vec![String::from("a"), String::from("b"), String::from("c")];
  let first = vec3.remove(0); // 移动第一个元素
  println!("移动的元素: {}", first);
  println!("剩余元素: {:?}", vec3);

  // 使用引用避免移动
  let vec4 = vec![10, 20, 30];
  let sum: i32 = vec4.iter().sum(); // 使用迭代器引用
  println!("求和结果: {}, 原 vec 仍可用: {:?}", sum, vec4);

  // 克隆 vs 移动的性能考虑
  let large_vec: Vec<i32> = (0..1000).collect();
  let _cloned = large_vec.clone(); // 昂贵的克隆操作
  let _moved = large_vec; // 廉价的移动操作
  // println!("{:?}", large_vec); // 编译错误：已被移动
}

/// 演示 HashMap 的所有权管理
fn demonstrate_hashmap_ownership() {
  println!("\n🗂️ HashMap 所有权演示:");

  let mut map = HashMap::new();

  // 插入拥有所有权的值
  let key1 = String::from("name");
  let value1 = String::from("Alice");

  map.insert(key1, value1);
  // println!("{}, {}", key1, value1); // 编译错误：已被移动到 map 中

  // 使用引用查询，避免移动
  let name_key = String::from("name");
  if let Some(name) = map.get(&name_key) {
    println!("找到姓名: {}", name);
  }
  println!("查询键仍可用: {}", name_key);

  // 演示 entry API 的所有权模式 - 使用计数器
  let mut counter_map: HashMap<String, i32> = HashMap::new();
  let counter_key = String::from("count");
  let counter = counter_map.entry(counter_key.clone()).or_insert(0);
  *counter += 1;
  println!("计数器: {}", counter);

  println!("最终 map: {:?}", map);
  println!("计数器 map: {:?}", counter_map);
}

/// 演示嵌套集合的所有权
fn demonstrate_nested_collections() {
  println!("\n🎯 嵌套集合所有权演示:");

  // Vec<Vec<T>> 的所有权
  let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

  // 移动内部 Vec
  let first_row = matrix.remove(0);
  println!("移动的行: {:?}", first_row);
  println!("剩余矩阵: {:?}", matrix);

  // 借用内部元素
  if let Some(row) = matrix.get(0) {
    if let Some(&element) = row.get(1) {
      println!("借用的元素: {}", element);
    }
  }

  // HashMap<String, Vec<String>> 的复杂所有权
  let mut groups: HashMap<String, Vec<String>> = HashMap::new();

  groups.insert(
    String::from("fruits"),
    vec![String::from("apple"), String::from("banana")],
  );

  groups.insert(
    String::from("colors"),
    vec![String::from("red"), String::from("blue")],
  );

  // 安全地修改嵌套集合
  if let Some(fruits) = groups.get_mut("fruits") {
    fruits.push(String::from("orange"));
  }

  println!("分组数据: {:?}", groups);
}

/// 错误处理中的所有权模式
fn error_handling_ownership() {
  println!("\n--- 错误处理中的所有权 ---");

  // Result<T, E> 的所有权
  demonstrate_result_ownership();

  // Option<T> 的所有权
  demonstrate_option_ownership();

  // 自定义错误类型的所有权
  demonstrate_custom_error_ownership();
}

/// 演示 Result 的所有权行为
fn demonstrate_result_ownership() {
  println!("\n✅ Result 所有权演示:");

  fn parse_number(s: String) -> Result<i32, String> {
    match s.parse() {
      Ok(n) => Ok(n),
      Err(_) => Err(format!("无法解析: {}", s)), // s 被移动到错误信息中
    }
  }

  let input = String::from("42");
  match parse_number(input) {
    Ok(n) => println!("解析成功: {}", n),
    Err(e) => println!("解析失败: {}", e),
  }
  // println!("{}", input); // 编译错误：input 已被移动

  // 使用引用避免移动
  fn parse_number_ref(s: &str) -> Result<i32, String> {
    match s.parse() {
      Ok(n) => Ok(n),
      Err(_) => Err(format!("无法解析: {}", s)),
    }
  }

  let input2 = String::from("not_a_number");
  match parse_number_ref(&input2) {
    Ok(n) => println!("解析成功: {}", n),
    Err(e) => println!("解析失败: {}", e),
  }
  println!("输入仍可用: {}", input2); // input2 仍然有效
}

/// 演示 Option 的所有权行为
fn demonstrate_option_ownership() {
  println!("\n🎲 Option 所有权演示:");

  let mut maybe_string = Some(String::from("hello"));

  // take() 方法移动值，留下 None
  let taken = maybe_string.take();
  println!("取出的值: {:?}", taken);
  println!("原 Option: {:?}", maybe_string); // 现在是 None

  // 使用 as_ref() 获取引用而不移动
  let maybe_string2 = Some(String::from("world"));
  if let Some(ref s) = maybe_string2 {
    println!("借用的值: {}", s);
  }
  println!("原 Option 仍有值: {:?}", maybe_string2);

  // map 操作的所有权转换
  let numbers = vec![Some(1), None, Some(3), Some(4)];
  let doubled: Vec<Option<i32>> = numbers
    .into_iter() // 移动 numbers
    .map(|opt| opt.map(|n| n * 2))
    .collect();
  println!("翻倍后: {:?}", doubled);
  // println!("{:?}", numbers); // 编译错误：numbers 已被移动
}

/// 演示自定义错误类型的所有权
fn demonstrate_custom_error_ownership() {
  println!("\n🚨 自定义错误类型所有权演示:");

  #[derive(Debug)]
#[allow(dead_code)]
enum CustomError {
  InvalidInput(String),
  NetworkError { message: String, code: u32 },
  InternalError,
}

  fn process_data(data: String) -> Result<String, CustomError> {
    if data.is_empty() {
      return Err(CustomError::InvalidInput(data)); // 移动 data 到错误中
    }

    if data.len() > 100 {
      return Err(CustomError::NetworkError {
        message: format!("数据过长: {} 字符", data.len()),
        code: 413,
      });
    }

    Ok(format!("处理完成: {}", data))
  }

  let test_data = String::from("");
  match process_data(test_data) {
    Ok(result) => println!("成功: {}", result),
    Err(e) => println!("错误: {:?}", e),
  }
  // println!("{}", test_data); // 编译错误：test_data 已被移动
}

/// 异步编程中的所有权模式
fn async_ownership_patterns() {
  println!("\n--- 异步编程中的所有权 ---");

  // 注意：这里只演示概念，实际异步代码需要 async runtime
  demonstrate_async_concepts();
}

/// 演示异步编程中的所有权概念
fn demonstrate_async_concepts() {
  println!("\n⚡ 异步所有权概念演示:");

  // 模拟异步函数的所有权需求
  struct AsyncData {
    content: String,
  }

  impl AsyncData {
    fn new(content: String) -> Self {
      AsyncData { content }
    }

    // 模拟异步处理，需要拥有数据
    fn process_owned(self) -> String {
      format!("异步处理: {}", self.content)
    }

    // 模拟异步处理，只需要借用
    fn process_borrowed(&self) -> String {
      format!("异步借用处理: {}", self.content)
    }
  }

  let data = AsyncData::new(String::from("重要数据"));

  // 在实际异步代码中，这些会是 async fn
  let result1 = data.process_borrowed();
  println!("{}", result1);

  let result2 = data.process_owned(); // 移动数据
  println!("{}", result2);
  // println!("{:?}", data.content); // 编译错误：data 已被移动

  println!("💡 在异步编程中，需要特别注意数据的生命周期和所有权转移");
}

/// 性能优化中的所有权策略
fn performance_ownership_strategies() {
  println!("\n--- 性能优化中的所有权策略 ---");

  // 零拷贝字符串处理
  demonstrate_zero_copy_strings();

  // 引用计数优化
  demonstrate_rc_optimization();

  // 内存池模式
  demonstrate_memory_pool_pattern();
}

/// 演示零拷贝字符串处理
fn demonstrate_zero_copy_strings() {
  println!("\n🚀 零拷贝字符串处理:");

  use std::borrow::Cow;

  // 使用生命周期省略规则，编译器会自动推断
  fn process_text(input: &str) -> Cow<'_, str> {
    if input.contains("bad") {
      // 需要修改，创建新字符串
      Cow::Owned(input.replace("bad", "good"))
    } else {
      // 不需要修改，直接借用
      Cow::Borrowed(input)
    }
  }

  let text1 = "这是一个好的文本";
  let text2 = "这是一个bad文本";

  let result1 = process_text(text1);
  let result2 = process_text(text2);

  println!("文本1 (借用): {}", result1);
  println!("文本2 (拥有): {}", result2);

  // 检查是否发生了拷贝
  match result1 {
    Cow::Borrowed(_) => println!("文本1: 零拷贝 ✅"),
    Cow::Owned(_) => println!("文本1: 发生拷贝"),
  }

  match result2 {
    Cow::Borrowed(_) => println!("文本2: 零拷贝"),
    Cow::Owned(_) => println!("文本2: 发生拷贝 ✅"),
  }
}

/// 演示引用计数优化
fn demonstrate_rc_optimization() {
  println!("\n🔄 引用计数优化:");

  // 大型数据结构的共享
  #[derive(Debug)]
  struct LargeData {
    data: Vec<u8>,
  }

  impl LargeData {
    fn new(size: usize) -> Self {
      LargeData {
        data: vec![0; size],
      }
    }
  }

  // 使用 Rc 共享大型数据，避免昂贵的克隆
  let large_data = Rc::new(LargeData::new(1000));

  let shared1 = Rc::clone(&large_data);
  let shared2 = Rc::clone(&large_data);
  let _shared3 = Rc::clone(&large_data);

  println!("引用计数: {}", Rc::strong_count(&large_data));
  println!("数据大小: {} 字节", shared1.data.len());

  // 所有共享引用指向同一块内存
  println!(
    "地址相同: {}",
    std::ptr::eq(shared1.as_ref(), shared2.as_ref())
  );

  drop(shared1);
  drop(shared2);
  println!("释放后引用计数: {}", Rc::strong_count(&large_data));
}

/// 演示内存池模式
fn demonstrate_memory_pool_pattern() {
  println!("\n🏊 内存池模式演示:");

  // 简单的对象池实现
  struct ObjectPool<T> {
    objects: RefCell<Vec<T>>,
    factory: Box<dyn Fn() -> T>,
  }

  impl<T> ObjectPool<T> {
    fn new<F>(factory: F) -> Self
    where
      F: Fn() -> T + 'static,
    {
      ObjectPool {
        objects: RefCell::new(Vec::new()),
        factory: Box::new(factory),
      }
    }

    fn get(&self) -> T {
      let mut objects = self.objects.borrow_mut();
      objects.pop().unwrap_or_else(|| (self.factory)())
    }

    fn return_object(&self, obj: T) {
      let mut objects = self.objects.borrow_mut();
      objects.push(obj);
    }
  }

  // 使用对象池
  let pool = ObjectPool::new(|| Vec::<i32>::with_capacity(100));

  // 从池中获取对象
  let mut vec1 = pool.get();
  vec1.extend_from_slice(&[1, 2, 3, 4, 5]);
  println!("使用对象: {:?}", vec1);

  // 清理并归还到池中
  vec1.clear();
  pool.return_object(vec1);

  // 再次获取（可能是同一个对象）
  let vec2 = pool.get();
  println!("重用对象容量: {}", vec2.capacity());

  println!("💡 对象池可以减少内存分配，提高性能");
}

/// 常见所有权错误的解决方案
pub fn common_ownership_solutions() {
  println!("\n🔧 常见所有权错误解决方案");
  println!("{}", "=".repeat(60));

  // 1. 借用检查器错误
  solve_borrow_checker_errors();

  // 2. 生命周期错误
  solve_lifetime_errors();

  // 3. 移动错误
  solve_move_errors();

  // 4. 闭包捕获错误
  solve_closure_capture_errors();
}

/// 解决借用检查器错误
fn solve_borrow_checker_errors() {
  println!("\n--- 借用检查器错误解决方案 ---");

  // 问题：同时存在可变和不可变借用
  fn problematic_code_demo() {
    let mut vec = vec![1, 2, 3, 4, 5];

    // 这种模式会导致借用检查错误
    /*
    let first = &vec[0];  // 不可变借用
    vec.push(6);          // 可变借用 - 错误！
    println!("{}", first);
    */

    // 解决方案1：缩小借用范围
    {
      let first = vec[0]; // 复制值而不是借用
      vec.push(6);
      println!("第一个元素: {}", first);
    }

    // 解决方案2：重新排序操作
    vec.push(7);
    let first = &vec[0];
    println!("第一个元素: {}", first);

    println!("最终向量: {:?}", vec);
  }

  problematic_code_demo();
}

/// 解决生命周期错误
fn solve_lifetime_errors() {
  println!("\n--- 生命周期错误解决方案 ---");

  // 问题：返回局部变量的引用
  /*
  fn bad_function() -> &str {
    let s = String::from("hello");
    &s  // 错误：返回对局部变量的引用
  }
  */

  // 解决方案1：返回拥有所有权的值
  fn good_function_v1() -> String {
    let s = String::from("hello");
    s // 移动所有权
  }

  // 解决方案2：使用静态字符串
  fn good_function_v2() -> &'static str {
    "hello" // 字符串字面量有 'static 生命周期
  }

  // 解决方案3：接受参数并返回其引用
  fn good_function_v3(input: &str) -> &str {
    input // 返回输入的引用
  }

  let result1 = good_function_v1();
  let result2 = good_function_v2();
  let input = String::from("world");
  let result3 = good_function_v3(&input);

  println!("解决方案1: {}", result1);
  println!("解决方案2: {}", result2);
  println!("解决方案3: {}", result3);
}

/// 解决移动错误
fn solve_move_errors() {
  println!("\n--- 移动错误解决方案 ---");

  // 问题：值被移动后继续使用
  fn demonstrate_move_solutions() {
    let original = vec![1, 2, 3, 4, 5];

    // 解决方案1：克隆
    let cloned = original.clone();
    let _moved = original; // 移动原始值
    println!("克隆的向量: {:?}", cloned); // 使用克隆

    // 解决方案2：使用引用
    let another = vec![6, 7, 8, 9, 10];
    process_vector_ref(&another); // 传递引用
    println!("原向量仍可用: {:?}", another);

    // 解决方案3：返回所有权
    let yet_another = vec![11, 12, 13];
    let returned = process_and_return(yet_another);
    println!("返回的向量: {:?}", returned);
  }

  fn process_vector_ref(v: &Vec<i32>) {
    println!("处理向量引用: 长度 = {}", v.len());
  }

  fn process_and_return(mut v: Vec<i32>) -> Vec<i32> {
    v.push(999);
    v // 返回修改后的向量
  }

  demonstrate_move_solutions();
}

/// 解决闭包捕获错误
fn solve_closure_capture_errors() {
  println!("\n--- 闭包捕获错误解决方案 ---");

  // 问题：闭包意外捕获了不需要的变量
  fn demonstrate_closure_solutions() {
    let data = vec![1, 2, 3, 4, 5];
    let multiplier = 2;

    // 解决方案1：明确指定捕获的变量
    let closure1 = {
      let m = multiplier; // 明确复制需要的值
      move |x: i32| x * m
    };

    // 解决方案2：使用 move 关键字
    let data_clone = data.clone();
    let closure2 = move || data_clone.iter().sum::<i32>();

    // 解决方案3：只捕获需要的部分
    let first_element = data[0];
    let closure3 = move || first_element * 10;

    println!("闭包1结果: {}", closure1(5));
    println!("闭包2结果: {}", closure2());
    println!("闭包3结果: {}", closure3());
    println!("原数据仍可用: {:?}", data);
  }

  demonstrate_closure_solutions();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_string_ownership() {
    let s1 = String::from("hello");
    let s2 = s1;
    // assert_eq!(s1, "hello"); // 这会编译失败
    assert_eq!(s2, "hello");
  }

  #[test]
  fn test_reference_borrowing() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    assert_eq!(len, 5);
    assert_eq!(s, "hello"); // s 仍然有效
  }

  fn calculate_length(s: &String) -> usize {
    s.len()
  }

  #[test]
  fn test_mutable_reference() {
    let mut s = String::from("hello");
    change_string(&mut s);
    assert_eq!(s, "hello, world!");
  }

  fn change_string(s: &mut String) {
    s.push_str(", world!");
  }

  #[test]
  fn test_rc_sharing() {
    let data = Rc::new(vec![1, 2, 3]);
    let data1 = Rc::clone(&data);
    let data2 = Rc::clone(&data);

    assert_eq!(Rc::strong_count(&data), 3);
    assert_eq!(*data1, vec![1, 2, 3]);
    assert_eq!(*data2, vec![1, 2, 3]);
  }

  #[test]
  fn test_option_ownership() {
    let mut maybe_string = Some(String::from("test"));
    let taken = maybe_string.take();

    assert_eq!(taken, Some(String::from("test")));
    assert_eq!(maybe_string, None);
  }
}
