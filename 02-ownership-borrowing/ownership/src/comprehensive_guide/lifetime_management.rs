//! # 第四部分：生命周期管理
//!
//! 生命周期确保引用在需要时始终有效。

/// ## 第四部分：生命周期管理
///
/// 生命周期确保引用在需要时始终有效。
pub fn lifetime_management() {
  println!("\n=== 第四部分：生命周期管理 ===");

  lifetime_basics();
  lifetime_annotations();
  lifetime_in_structs();
  static_lifetime_explanation();
}

/// ### 4.1 生命周期基础
///
/// 理解生命周期的概念和必要性。
fn lifetime_basics() {
  println!("\n--- 4.1 生命周期基础 ---");

  println!("\n🔍 生命周期概念：");
  println!("• 生命周期是引用保持有效的作用域");
  println!("• 每个引用都有一个生命周期");
  println!("• 大部分时候生命周期是隐式的");
  println!("• 当编译器无法推断时需要显式标注");

  println!("\n🔍 生命周期示例：");

  {
    let _r: &str; // 声明引用 r
    {
      let _x = 5; // x 的生命周期开始
      // r = &x;        // 编译错误！x 的生命周期比 r 短
    } // x 的生命周期结束
    // println!("{}", r); // r 引用的值已失效
  }

  // 正确的生命周期关系
  {
    let x = 5; // x 的生命周期开始
    let r = &x; // r 引用 x
    println!("r: {}", r); // 使用引用
  } // x 和 r 的生命周期都结束

  println!("\n🔍 函数中的生命周期：");

  // 简单情况：编译器可以推断
  fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
        return &s[0..i];
      }
    }
    &s[..]
  }

  let sentence = "hello world";
  let word = first_word(sentence);
  println!("第一个单词: {}", word);

  println!("\n💡 生命周期规则：");
  println!("• 引用的生命周期不能超过其引用的值");
  println!("• 函数返回的引用必须来自参数或静态值");
  println!("• 编译器使用生命周期省略规则推断");
}

/// ### 4.2 生命周期标注
///
/// 显式生命周期标注的语法和使用。
fn lifetime_annotations() {
  println!("\n--- 4.2 生命周期标注 ---");

  println!("\n🔍 生命周期标注语法：");
  println!("• 以撇号开头：'a, 'b, 'static");
  println!("• 通常使用短名称：'a, 'b, 'c");
  println!("• 放在 & 之后，类型之前：&'a str");

  println!("\n🔍 函数生命周期标注：");

  // 需要生命周期标注的函数
  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
  }

  let string1 = String::from("abcd");
  let string2 = "xyz";
  let result = longest(&string1, string2);
  println!("最长的字符串: {}", result);

  // 不同生命周期的示例
  fn different_lifetimes<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x // 只能返回 x，因为返回类型绑定到 'a
  }

  let s1 = "hello";
  let s2 = "world";
  let result2 = different_lifetimes(s1, s2);
  println!("返回第一个参数: {}", result2);

  println!("\n🔍 生命周期省略规则：");

  // 规则1：每个引用参数都有自己的生命周期
  fn rule1_example(s: &str) -> &str {
    // 等价于 fn rule1_example<'a>(s: &'a str) -> &'a str
    s
  }

  // 规则2：如果只有一个输入生命周期，它被赋予所有输出生命周期
  fn rule2_example(s: &str) -> (&str, &str) {
    // 等价于 <'a>(s: &'a str) -> (&'a str, &'a str)
    (s, s)
  }

  // 规则3：如果有 &self 或 &mut self，self 的生命周期被赋予所有输出

  let test_str = "test";
  let r1 = rule1_example(test_str);
  let (r2, r3) = rule2_example(test_str);
  println!("省略规则示例: {}, {}, {}", r1, r2, r3);

  println!("\n💡 生命周期标注原则：");
  println!("• 生命周期标注不改变引用的实际生命周期");
  println!("• 它们描述了多个引用生命周期之间的关系");
  println!("• 帮助编译器验证引用的有效性");
  println!("• 只在编译时存在，运行时无开销");
}

/// ### 4.3 结构体中的生命周期
///
/// 结构体包含引用时的生命周期处理。
fn lifetime_in_structs() {
  println!("\n--- 4.3 结构体中的生命周期 ---");

  println!("\n🔍 包含引用的结构体：");

  // 结构体中的生命周期标注
  #[derive(Debug)]
  struct ImportantExcerpt<'a> {
    part: &'a str,
  }

  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find a '.'");
  let i: ImportantExcerpt<'_> = ImportantExcerpt {
    part: first_sentence,
  };
  println!("重要摘录: {:?}", i);

  println!("\n🔍 结构体方法中的生命周期：");

  impl<'a> ImportantExcerpt<'a> {
    // 方法的生命周期省略
    #[allow(dead_code)]
    fn level(&self) -> i32 {
      3
    }

    // 返回引用的方法
    #[allow(dead_code)]
    fn announce_and_return_part(&self, announcement: &str) -> &str {
      println!("注意！{}", announcement);
      self.part // 返回 self.part，生命周期与 self 相同
    }

    // 多个生命周期参数
    fn compare_parts<'b>(&self, other: &'b str) -> &str {
      if self.part.len() > other.len() {
        self.part
      } else {
        // other  // 编译错误！不能返回 'b 生命周期的引用
        self.part // 只能返回 'a 生命周期的引用
      }
    }
  }

  let level = i.level();
  println!("级别: {}", level);

  let announcement = "今天是个好日子";
  let part = i.announce_and_return_part(announcement);
  println!("返回的部分: {}", part);

  let other_text = "short";
  let comparison = i.compare_parts(other_text);
  println!("比较结果: {}", comparison);

  println!("\n🔍 多个引用字段：");

  #[derive(Debug)]
  #[allow(dead_code)]
  struct TwoRefs<'a, 'b> {
    first: &'a str,
    second: &'b str,
  }

  let s1 = "first string";
  let s2 = "second string";
  let two_refs = TwoRefs {
    first: s1,
    second: s2,
  };
  println!("两个引用: {:?}", two_refs);

  println!("\n💡 结构体生命周期要点：");
  println!("• 结构体包含引用时必须标注生命周期");
  println!("• 结构体实例的生命周期不能超过其引用的数据");
  println!("• 方法中的生命周期遵循省略规则");
  println!("• 可以有多个不同的生命周期参数");
}

/// ### 4.4 静态生命周期
///
/// 'static 生命周期的特殊性质。
fn static_lifetime_explanation() {
  println!("\n--- 4.4 静态生命周期 ---");

  println!("\n🔍 'static 生命周期：");
  println!("• 'static 表示引用在整个程序运行期间都有效");
  println!("• 字符串字面量默认具有 'static 生命周期");
  println!("• 存储在程序的二进制文件中");

  // 字符串字面量
  let s: &'static str = "我有静态生命周期";
  println!("静态字符串: {}", s);

  // 静态变量
  static HELLO_WORLD: &str = "Hello, world!";
  println!("静态变量: {}", HELLO_WORLD);

  println!("\n🔍 'static 的使用场景：");

  // 函数返回静态引用
  fn get_static_string() -> &'static str {
    "这是静态字符串"
  }

  let static_str = get_static_string();
  println!("函数返回的静态字符串: {}", static_str);

  // 泛型约束中的 'static
  fn print_it<T: std::fmt::Display + 'static>(input: T) {
    println!("静态约束: {}", input);
  }

  print_it("字符串字面量");
  print_it(42);
  print_it(true);

  println!("\n🔍 'static 与生命周期参数的区别：");

  // 生命周期参数：引用的生命周期由调用者决定
  fn with_lifetime<'a>(x: &'a str) -> &'a str {
    x
  }

  // 静态生命周期：引用必须在整个程序期间有效
  fn with_static(x: &'static str) -> &'static str {
    x
  }

  let local_string = String::from("local");
  let lifetime_result = with_lifetime(&local_string); // 可以传递局部字符串的引用
  println!("生命周期参数: {}", lifetime_result);

  let static_result = with_static("static"); // 只能传递静态字符串
  println!("静态生命周期: {}", static_result);

  // with_static(&local_string);  // 编译错误！局部字符串没有静态生命周期

  println!("\n💡 'static 使用建议：");
  println!("• 不要过度使用 'static 约束");
  println!("• 优先考虑生命周期参数");
  println!("• 'static 主要用于全局数据和字符串字面量");
  println!("• 在泛型约束中谨慎使用");
}
