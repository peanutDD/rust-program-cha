//! # 第八部分：常见错误与最佳实践
//!
//! 总结所有权系统的常见陷阱和最佳实践。

use std::borrow::Cow;

/// ## 第八部分：常见错误与最佳实践
///
/// 总结所有权系统的常见陷阱和最佳实践。
pub fn common_mistakes_and_best_practices() {
  println!("\n=== 第八部分：常见错误与最佳实践 ===");

  common_ownership_mistakes();
  borrowing_best_practices();
  lifetime_guidelines();
  performance_best_practices();
}

/// ### 8.1 常见所有权错误
///
/// 新手常犯的所有权相关错误。
fn common_ownership_mistakes() {
  println!("\n--- 8.1 常见所有权错误 ---");

  println!("\n🚫 错误1：使用已移动的值");
  println!("// 错误代码示例：");
  println!("// let s1 = String::from(\"hello\");");
  println!("// let s2 = s1;  // s1 被移动");
  println!("// println!(\"{{}}\", s1);  // 编译错误！");

  println!("\n✅ 正确做法：");
  let s1 = String::from("hello");
  let s2 = s1.clone(); // 克隆而不是移动
  println!("s1: {}, s2: {}", s1, s2);

  println!("\n🚫 错误2：悬垂引用");
  println!("// 错误代码示例：");
  println!("// fn dangle() -> &String {{");
  println!("//     let s = String::from(\"hello\");");
  println!("//     &s  // 返回局部变量的引用");
  println!("// }}  // s 被释放，引用变为悬垂");

  println!("\n✅ 正确做法：");
  fn no_dangle() -> String {
    let s = String::from("hello");
    s // 返回所有权
  }
  let result = no_dangle();
  println!("正确返回: {}", result);

  println!("\n🚫 错误3：借用检查冲突");
  println!("// 错误代码示例：");
  println!("// let mut v = vec![1, 2, 3];");
  println!("// let r = &v[0];  // 不可变借用");
  println!("// v.push(4);      // 可变借用，冲突！");
  println!("// println!(\"{{}}\", r);");

  println!("\n✅ 正确做法：");
  let mut v = vec![1, 2, 3];
  {
    let r = &v[0]; // 不可变借用
    println!("第一个元素: {}", r);
  } // 借用结束
  v.push(4); // 现在可以可变借用
  println!("修改后的向量: {:?}", v);

  println!("\n🚫 错误4：过度克隆");
  println!("// 低效代码示例：");
  println!("// fn process_string(s: String) -> String {{");
  println!("//     s.to_uppercase()");
  println!("// }}");
  println!("// let original = String::from(\"hello\");");
  println!("// let result = process_string(original.clone());  // 不必要的克隆");

  println!("\n✅ 正确做法：");
  fn process_string_ref(s: &str) -> String {
    s.to_uppercase()
  }
  let original = String::from("hello");
  let result = process_string_ref(&original); // 使用引用
  println!("原始: {}, 处理后: {}", original, result);

  println!("\n💡 避免错误的要点：");
  println!("• 理解移动语义和借用规则");
  println!("• 优先使用引用而不是克隆");
  println!("• 注意引用的生命周期");
  println!("• 使用编译器错误信息学习");
}

/// ### 8.2 借用最佳实践
///
/// 高效使用借用机制的指导原则。
fn borrowing_best_practices() {
  println!("\n--- 8.2 借用最佳实践 ---");

  println!("\n✅ 实践1：优先使用不可变引用");

  fn analyze_data(data: &[i32]) -> (i32, i32, f64) {
    let sum: i32 = data.iter().sum();
    let max = *data.iter().max().unwrap_or(&0);
    let avg = sum as f64 / data.len() as f64;
    (sum, max, avg)
  }

  let numbers = vec![1, 5, 3, 9, 2, 7];
  let (sum, max, avg) = analyze_data(&numbers);
  println!("分析结果 - 和: {}, 最大: {}, 平均: {:.2}", sum, max, avg);
  println!("原始数据仍可用: {:?}", numbers);

  println!("\n✅ 实践2：最小化可变借用的作用域");

  let mut data = vec![1, 2, 3, 4, 5];

  // 将可变借用限制在最小作用域内
  {
    let last = data.last_mut().unwrap();
    *last *= 10;
  } // 可变借用结束

  // 现在可以进行其他操作
  let sum: i32 = data.iter().sum();
  println!("修改后的数据: {:?}, 和: {}", data, sum);

  println!("\n✅ 实践3：使用方法链避免中间变量");

  let text = "hello world rust programming";
  let result: Vec<String> = text
    .split_whitespace()
    .filter(|word| word.len() > 4)
    .map(|word| word.to_uppercase())
    .collect();

  println!("处理结果: {:?}", result);

  println!("\n✅ 实践4：合理使用 as_ref() 和 as_mut()");

  let mut option_string = Some(String::from("hello"));

  // 使用 as_ref() 避免移动 Option 中的值
  if let Some(ref s) = option_string {
    println!("字符串长度: {}", s.len());
  }

  // 使用 as_mut() 修改 Option 中的值
  if let Some(ref mut s) = option_string {
    s.push_str(", world");
  }

  println!("修改后: {:?}", option_string);

  println!("\n💡 借用最佳实践总结：");
  println!("• 默认使用不可变引用");
  println!("• 最小化可变借用作用域");
  println!("• 利用方法链减少中间变量");
  println!("• 合理使用 Option 的引用方法");
}

/// ### 8.3 生命周期指导原则
///
/// 生命周期标注和管理的最佳实践。
fn lifetime_guidelines() {
  println!("\n--- 8.3 生命周期指导原则 ---");

  println!("\n✅ 原则1：尽量避免显式生命周期标注");

  // 编译器可以推断的情况
  fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
  }

  let sentence = "hello world";
  let word = first_word(sentence);
  println!("第一个单词: {}", word);

  println!("\n✅ 原则2：必要时使用描述性的生命周期名称");

  // 使用有意义的生命周期名称
  fn find_longest_line<'text>(text: &'text str) -> Option<&'text str> {
    text.lines().max_by_key(|line| line.len())
  }

  let multiline = "short\nthis is a longer line\nshort again";
  if let Some(longest) = find_longest_line(multiline) {
    println!("最长的行: {}", longest);
  }

  println!("\n✅ 原则3：结构体生命周期要谨慎设计");

  #[derive(Debug)]
  struct TextAnalyzer<'a> {
    text: &'a str,
    word_count: usize,
  }

  impl<'a> TextAnalyzer<'a> {
    fn new(text: &'a str) -> Self {
      let word_count = text.split_whitespace().count();
      TextAnalyzer { text, word_count }
    }

    fn get_summary(&self) -> String {
      format!("文本长度: {}, 单词数: {}", self.text.len(), self.word_count)
    }
  }

  let text = "Rust is a systems programming language";
  let analyzer = TextAnalyzer::new(text);
  println!("分析器: {:?}", analyzer);
  println!("摘要: {}", analyzer.get_summary());

  println!("\n✅ 原则4：使用 'static 要谨慎");

  // 只在真正需要静态生命周期时使用
  const GREETING: &'static str = "Hello, Rust!";

  fn get_greeting() -> &'static str {
    GREETING
  }

  println!("静态问候: {}", get_greeting());

  println!("\n💡 生命周期指导原则：");
  println!("• 让编译器尽可能推断生命周期");
  println!("• 使用描述性的生命周期参数名");
  println!("• 结构体生命周期设计要简洁");
  println!("• 谨慎使用 'static 生命周期");
}

/// ### 8.4 性能最佳实践
///
/// 利用所有权系统优化性能的技巧。
fn performance_best_practices() {
  println!("\n--- 8.4 性能最佳实践 ---");

  println!("\n✅ 技巧1：避免不必要的分配");

  // 使用 Cow (Clone on Write) 优化
  // 使用生命周期省略规则，编译器会自动推断
  fn process_text(input: &str) -> Cow<'_, str> {
    if input.contains("bad_word") {
      Cow::Owned(input.replace("bad_word", "***"))
    } else {
      Cow::Borrowed(input)
    }
  }

  let clean_text = "This is clean text";
  let dirty_text = "This contains bad_word";

  println!("处理干净文本: {:?}", process_text(clean_text));
  println!("处理脏文本: {:?}", process_text(dirty_text));

  println!("\n✅ 技巧2：使用迭代器而不是索引");

  let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

  // 高效的迭代器链
  let sum: i32 = numbers
    .iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * x)
    .sum();

  println!("偶数平方和: {}", sum);

  println!("\n✅ 技巧3：合理使用 Vec 容量");

  // 预分配容量避免重复分配
  let mut efficient_vec = Vec::with_capacity(1000);
  for i in 0..1000 {
    efficient_vec.push(i);
  }
  println!(
    "高效向量长度: {}, 容量: {}",
    efficient_vec.len(),
    efficient_vec.capacity()
  );

  println!("\n✅ 技巧4：使用 Box 处理大型数据");

  // 大型结构体使用 Box 避免栈溢出
  #[derive(Debug)]
  #[allow(dead_code)]
  struct LargeData {
    data: [u8; 1024], // 1KB 数据
    id: u32,
  }

  let large_data = Box::new(LargeData {
    data: [0; 1024],
    id: 42,
  });

  println!("大型数据 ID: {}", large_data.id);

  println!("\n💡 性能最佳实践总结：");
  println!("• 使用 Cow 避免不必要的克隆");
  println!("• 优先使用迭代器而不是索引访问");
  println!("• 预分配集合容量");
  println!("• 大型数据使用 Box 存储在堆上");
  println!("• 利用零成本抽象");
}
