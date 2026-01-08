//! Rust 字符、布尔、单元类型全面教程
//! 基于 https://course.rs/basic/base-type/char-bool.html 的详细讲解
//! 包含所有知识点和具体案例

#![allow(unused_variables, dead_code)]

fn main() {
  println!("=== Rust 字符、布尔、单元类型全面教程 ===");

  // 1. 字符类型基础
  char_type_basics();

  // 2. 字符的内存表示和大小
  char_memory_representation();

  // 3. 字符字面量和转义序列
  char_literals_and_escapes();

  // 4. Unicode 和字符编码
  unicode_and_encoding();

  // 5. 字符操作和方法
  char_operations_and_methods();

  // 6. 布尔类型基础
  bool_type_basics();

  // 7. 布尔运算和逻辑操作
  bool_operations();

  // 8. 布尔类型在控制流中的应用
  bool_in_control_flow();

  // 9. 单元类型基础
  unit_type_basics();

  // 10. 单元类型的应用场景
  unit_type_applications();

  // 11. 类型转换
  type_conversions();

  // 12. 实际应用案例
  practical_examples();

  // 13. 性能和内存考虑
  performance_and_memory();

  // 14. 常见错误和最佳实践
  common_mistakes_and_best_practices();
}

/// 1. 字符类型基础
fn char_type_basics() {
  println!("\n=== 1. 字符类型基础 ===");

  // 字符类型 char 表示单个 Unicode 标量值 <mcreference link="https://course.rs/basic/base-type/char-bool.html" index="2">2</mcreference>
  let c1 = 'a'; // ASCII 字符
  let c2 = '中'; // 中文字符
  let c3 = '🦀'; // Emoji 表情符号
  let c4 = '\u{1F980}'; // Unicode 转义序列（螃蟹 emoji）

  println!("ASCII 字符: '{}'", c1);
  println!("中文字符: '{}'", c2);
  println!("Emoji: '{}'", c3);
  println!("Unicode 转义: '{}'", c4);

  // 字符类型的特点 <mcreference link="https://course.rs/basic/base-type/char-bool.html" index="2">2</mcreference>
  println!("\n字符类型特点:");
  println!("- 使用单引号定义");
  println!("- 占用 4 个字节（32 位）");
  println!("- 表示 Unicode 标量值");
  println!("- 范围: U+0000 到 U+D7FF 和 U+E000 到 U+10FFFF");

  // 显式类型注解
  let explicit_char: char = 'R';
  println!("显式类型注解: {}", explicit_char);
}

/// 2. 字符的内存表示和大小
fn char_memory_representation() {
  println!("\n=== 2. 字符的内存表示和大小 ===");

  use std::mem::size_of_val;

  // 不同字符的内存大小 <mcreference link="https://practice-zh.course.rs/basic-types/char-bool-unit.html" index="1">1</mcreference>
  let ascii_char = 'a';
  let chinese_char = '中';
  let emoji_char = '🦀';

  println!("字符内存大小:");
  println!(
    "ASCII 字符 '{}': {} 字节",
    ascii_char,
    size_of_val(&ascii_char)
  );
  println!(
    "中文字符 '{}': {} 字节",
    chinese_char,
    size_of_val(&chinese_char)
  );
  println!("Emoji '{}': {} 字节", emoji_char, size_of_val(&emoji_char));

  // 所有 char 类型都占用 4 字节，无论实际字符需要多少字节
  println!("\n注意: 所有 char 类型都固定占用 4 字节");

  // 字符的数值表示
  println!("\n字符的数值表示:");
  println!(
    "'a' 的 Unicode 码点: U+{:04X} ({})",
    ascii_char as u32, ascii_char as u32
  );
  println!(
    "'中' 的 Unicode 码点: U+{:04X} ({})",
    chinese_char as u32, chinese_char as u32
  );
  println!(
    "'🦀' 的 Unicode 码点: U+{:04X} ({})",
    emoji_char as u32, emoji_char as u32
  );
}

/// 3. 字符字面量和转义序列
fn char_literals_and_escapes() {
  println!("\n=== 3. 字符字面量和转义序列 ===");

  // 基本字符字面量
  let letter = 'A';
  let digit = '9';
  let symbol = '@';

  println!("基本字符: {}, {}, {}", letter, digit, symbol);

  // 转义序列
  let newline = '\n'; // 换行符
  let tab = '\t'; // 制表符
  let carriage_return = '\r'; // 回车符
  let backslash = '\\'; // 反斜杠
  let single_quote = '\''; // 单引号
  let null_char = '\0'; // 空字符

  println!("\n转义序列:");
  println!("换行符: {:?}", newline);
  println!("制表符: {:?}", tab);
  println!("回车符: {:?}", carriage_return);
  println!("反斜杠: {:?}", backslash);
  println!("单引号: {:?}", single_quote);
  println!("空字符: {:?}", null_char);

  // Unicode 转义序列
  let unicode_a = '\u{0041}'; // 'A' 的 Unicode 表示
  let unicode_heart = '\u{2764}'; // ❤️ 心形符号
  let unicode_chinese = '\u{4E2D}'; // '中' 字

  println!("\nUnicode 转义序列:");
  println!("\\u{{0041}}: {}", unicode_a);
  println!("\\u{{2764}}: {}", unicode_heart);
  println!("\\u{{4E2D}}: {}", unicode_chinese);

  // 十六进制转义序列（仅限 ASCII）
  let hex_a = '\x41'; // 'A' 的十六进制表示
  println!("\n十六进制转义 \\x41: {}", hex_a);
}

/// 4. Unicode 和字符编码
fn unicode_and_encoding() {
  println!("\n=== 4. Unicode 和字符编码 ===");

  // Unicode 范围说明 <mcreference link="https://course.rs/basic/base-type/char-bool.html" index="2">2</mcreference>
  println!("Unicode 标量值范围:");
  println!("- 有效范围1: U+0000 到 U+D7FF");
  println!("- 有效范围2: U+E000 到 U+10FFFF");
  println!("- 代理对范围 U+D800 到 U+DFFF 不是有效的 char 值");

  // 不同语言的字符
  let english = 'H';
  let chinese = '你';
  let japanese = 'あ';
  let korean = '안';
  let arabic = 'ع';
  let russian = 'Ж';
  let greek = 'Ω';

  println!("\n多语言字符支持:");
  println!("英文: {} (U+{:04X})", english, english as u32);
  println!("中文: {} (U+{:04X})", chinese, chinese as u32);
  println!("日文: {} (U+{:04X})", japanese, japanese as u32);
  println!("韩文: {} (U+{:04X})", korean, korean as u32);
  println!("阿拉伯文: {} (U+{:04X})", arabic, arabic as u32);
  println!("俄文: {} (U+{:04X})", russian, russian as u32);
  println!("希腊文: {} (U+{:04X})", greek, greek as u32);

  // 特殊符号和 Emoji
  let symbols = ['©', '®', '™', '€', '£', '¥', '§'];
  let emojis = ['😀', '😍', '🚀', '🌟', '🎉', '💻', '🦀'];

  println!("\n特殊符号:");
  for symbol in symbols {
    println!("{} (U+{:04X})", symbol, symbol as u32);
  }

  println!("\nEmoji 字符:");
  for emoji in emojis {
    println!("{} (U+{:04X})", emoji, emoji as u32);
  }
}

/// 5. 字符操作和方法
fn char_operations_and_methods() {
  println!("\n=== 5. 字符操作和方法 ===");

  let ch = 'A';
  let digit_ch = '5';
  let space_ch = ' ';
  let chinese_ch = '中';

  // 字符分类方法
  println!("字符分类方法:");
  println!("'{}' 是字母: {}", ch, ch.is_alphabetic());
  println!("'{}' 是数字: {}", digit_ch, digit_ch.is_numeric());
  println!("'{}' 是字母或数字: {}", ch, ch.is_alphanumeric());
  println!("'{}' 是空白字符: {}", space_ch, space_ch.is_whitespace());
  println!("'{}' 是大写: {}", ch, ch.is_uppercase());
  println!("'{}' 是小写: {}", ch, ch.is_lowercase());
  println!("'{}' 是 ASCII: {}", ch, ch.is_ascii());
  println!("'{}' 是 ASCII: {}", chinese_ch, chinese_ch.is_ascii());

  // 大小写转换
  println!("\n大小写转换:");
  let lower_ch = 'a';
  let upper_ch = 'Z';

  println!(
    "'{}' 转大写: {:?}",
    lower_ch,
    lower_ch.to_uppercase().collect::<String>()
  );
  println!(
    "'{}' 转小写: {:?}",
    upper_ch,
    lower_ch.to_lowercase().collect::<String>()
  );

  // ASCII 相关方法
  println!("\nASCII 相关方法:");
  let ascii_ch = 'B';
  println!(
    "'{}' 是 ASCII 字母: {}",
    ascii_ch,
    ascii_ch.is_ascii_alphabetic()
  );
  println!(
    "'{}' 是 ASCII 数字: {}",
    digit_ch,
    digit_ch.is_ascii_digit()
  );
  println!(
    "'{}' 是 ASCII 大写: {}",
    ascii_ch,
    ascii_ch.is_ascii_uppercase()
  );
  println!(
    "'{}' 是 ASCII 小写: {}",
    ascii_ch,
    ascii_ch.is_ascii_lowercase()
  );

  // 数值转换
  println!("\n数值转换:");
  let digit = '7';
  if let Some(num) = digit.to_digit(10) {
    println!("字符 '{}' 转为数字: {}", digit, num);
  }

  // 十六进制数字
  let hex_digit = 'F';
  if let Some(hex_num) = hex_digit.to_digit(16) {
    println!("十六进制字符 '{}' 转为数字: {}", hex_digit, hex_num);
  }

  // 字符编码长度
  println!("\n字符编码长度:");
  let chars = ['a', '中', '🦀'];
  for ch in chars {
    println!("'{}' UTF-8 编码长度: {} 字节", ch, ch.len_utf8());
    println!("'{}' UTF-16 编码长度: {} 个16位单元", ch, ch.len_utf16());
  }
}

/// 6. 布尔类型基础
fn bool_type_basics() {
  println!("\n=== 6. 布尔类型基础 ===");

  // 布尔类型的两个值 <mcreference link="https://course.rs/basic/base-type/char-bool.html" index="2">2</mcreference>
  let t = true;
  let f: bool = false; // 显式类型注解

  println!("布尔值: true = {}, false = {}", t, f);

  // 布尔类型的内存大小 <mcreference link="https://course.rs/basic/base-type/char-bool.html" index="2">2</mcreference>
  use std::mem::size_of_val;
  println!("布尔类型内存大小: {} 字节", size_of_val(&t));

  // 布尔值的数值表示
  println!("\n布尔值的数值表示:");
  println!("true as u8 = {}", true as u8);
  println!("false as u8 = {}", false as u8);

  // 从其他类型创建布尔值
  println!("\n从其他类型创建布尔值:");
  let zero = 0;
  let non_zero = 42;

  // 注意：Rust 不会自动将数字转换为布尔值
  // 需要显式比较
  println!("{} != 0 = {}", zero, zero != 0);
  println!("{} != 0 = {}", non_zero, non_zero != 0);
}

/// 7. 布尔运算和逻辑操作
fn bool_operations() {
  println!("\n=== 7. 布尔运算和逻辑操作 ===");

  let a = true;
  let b = false;

  // 基本逻辑运算
  println!("基本逻辑运算:");
  println!("{} && {} = {}", a, b, a && b); // 逻辑与
  println!("{} || {} = {}", a, b, a || b); // 逻辑或
  println!("!{} = {}", a, !a); // 逻辑非

  // 真值表
  println!("\n完整真值表:");
  let values = [true, false];

  println!("A     | B     | A && B | A || B | !A");
  println!("------|-------|--------|--------|----");
  for &a in &values {
    for &b in &values {
      println!(
        "{:<5} | {:<5} | {:<6} | {:<6} | {}",
        a,
        b,
        a && b,
        a || b,
        !a
      );
    }
  }

  // 短路求值
  println!("\n短路求值演示:");

  fn expensive_true() -> bool {
    println!("  执行了 expensive_true()");
    true
  }

  fn expensive_false() -> bool {
    println!("  执行了 expensive_false()");
    false
  }

  println!("true || expensive_false():");
  let result1 = true || expensive_false(); // expensive_false() 不会被调用
  println!("结果: {}", result1);

  println!("\nfalse && expensive_true():");
  let result2 = false && expensive_true(); // expensive_true() 不会被调用
  println!("结果: {}", result2);

  // 位运算（对布尔值也适用）
  println!("\n位运算:");
  println!("{} & {} = {}", a, b, a & b); // 位与
  println!("{} | {} = {}", a, b, a | b); // 位或
  println!("{} ^ {} = {}", a, b, a ^ b); // 位异或
}

/// 8. 布尔类型在控制流中的应用
fn bool_in_control_flow() {
  println!("\n=== 8. 布尔类型在控制流中的应用 ===");

  let condition = true;
  let number = 42;

  // if 表达式 <mcreference link="https://course.rs/basic/base-type/char-bool.html" index="2">2</mcreference>
  println!("if 表达式:");
  if condition {
    println!("条件为真");
  } else {
    println!("条件为假");
  }

  // 复合条件
  println!("\n复合条件:");
  let age = 25;
  let has_license = true;

  if age >= 18 && has_license {
    println!("可以开车");
  } else if age >= 18 {
    println!("年龄够了，但需要驾照");
  } else {
    println!("年龄不够");
  }

  // 三元运算符的替代
  println!("\n条件表达式:");
  let status = if number > 0 { "正数" } else { "非正数" };
  println!("数字 {} 是 {}", number, status);

  // while 循环
  println!("\nwhile 循环:");
  let mut count = 0;
  while count < 3 {
    println!("计数: {}", count);
    count += 1;
  }

  // loop 循环与 break 条件
  println!("\nloop 循环与条件退出:");
  let mut i = 0;
  loop {
    if i >= 2 {
      break;
    }
    println!("循环 {}", i);
    i += 1;
  }

  // match 表达式中的布尔值
  println!("\nmatch 表达式:");
  let is_weekend = false;
  match is_weekend {
    true => println!("周末，休息！"),
    false => println!("工作日，加油！"),
  }
}

/// 9. 单元类型基础
fn unit_type_basics() {
  println!("\n=== 9. 单元类型基础 ===");

  // 单元类型 () <mcreference link="https://course.rs/basic/base-type/char-bool.html" index="2">2</mcreference>
  let unit_value = ();
  println!("单元值: {:?}", unit_value);

  // 单元类型的内存大小
  use std::mem::size_of_val;
  println!("单元类型内存大小: {} 字节", size_of_val(&unit_value));

  // 显式返回单元类型
  fn explicit_unit() -> () {
    println!("显式返回单元类型");
    () // 显式返回
  }

  // 隐式返回单元类型
  fn implicit_unit() {
    println!("隐式返回单元类型");
    // 没有返回值，隐式返回 ()
  }

  let result1 = explicit_unit();
  let result2 = implicit_unit();

  println!("显式返回结果: {:?}", result1);
  println!("隐式返回结果: {:?}", result2);

  // 单元类型的比较
  println!("\n单元类型比较:");
  println!("() == (): {}", () == ());
  println!("unit_value == (): {}", unit_value == ());
}

/// 10. 单元类型的应用场景
fn unit_type_applications() {
  println!("\n=== 10. 单元类型的应用场景 ===");

  // 1. 无返回值的函数
  fn print_message(msg: &str) {
    println!("消息: {}", msg);
    // 隐式返回 ()
  }

  print_message("Hello, Rust!");

  // 2. 单元结构体
  struct Unit;

  impl Unit {
    fn new() -> Self {
      Unit
    }

    fn do_something(&self) {
      println!("单元结构体执行操作");
    }
  }

  let unit_struct = Unit::new();
  unit_struct.do_something();

  // 3. 泛型中的占位符
  use std::marker::PhantomData;

  struct Container<T> {
    _phantom: PhantomData<T>,
    data: i32,
  }

  impl<T> Container<T> {
    fn new(data: i32) -> Self {
      Container {
        _phantom: PhantomData,
        data,
      }
    }
  }

  let _container: Container<()> = Container::new(42);

  // 4. Result 类型中的成功值
  fn operation_that_might_fail(succeed: bool) -> Result<(), &'static str> {
    if succeed {
      Ok(()) // 成功，但没有有意义的返回值
    } else {
      Err("操作失败")
    }
  }

  match operation_that_might_fail(true) {
    Ok(()) => println!("操作成功"),
    Err(e) => println!("错误: {}", e),
  }

  // 5. 集合中的占位符
  use std::collections::HashSet;

  let mut set: HashSet<()> = HashSet::new();
  set.insert(());
  println!("集合大小: {}", set.len());

  // 6. 函数指针
  fn callback() {
    println!("回调函数被调用");
  }

  fn execute_callback(f: fn()) {
    f();
  }

  execute_callback(callback);
}

/// 11. 类型转换
fn type_conversions() {
  println!("\n=== 11. 类型转换 ===");

  // 字符转换
  println!("字符类型转换:");
  let ch = 'A';
  let ascii_value = ch as u8; // 字符转 ASCII 值
  let unicode_value = ch as u32; // 字符转 Unicode 码点

  println!("'{}' 的 ASCII 值: {}", ch, ascii_value);
  println!("'{}' 的 Unicode 码点: {}", ch, unicode_value);

  // 数字转字符
  let ascii_num = 65u8;
  let unicode_num = 0x4E2D_u32; // '中' 的 Unicode 码点

  if let Some(char_from_ascii) = char::from_u32(ascii_num as u32) {
    println!("ASCII {} 转字符: '{}'", ascii_num, char_from_ascii);
  }

  if let Some(char_from_unicode) = char::from_u32(unicode_num) {
    println!("Unicode {} 转字符: '{}'", unicode_num, char_from_unicode);
  }

  // 布尔转换
  println!("\n布尔类型转换:");
  let bool_val = true;
  let bool_as_int = bool_val as i32;
  let bool_as_byte = bool_val as u8;

  println!("true as i32: {}", bool_as_int);
  println!("true as u8: {}", bool_as_byte);

  // 数字转布尔（需要比较）
  let numbers = [0, 1, -1, 42];
  for num in numbers {
    println!("{} != 0: {}", num, num != 0);
  }

  // 字符串转字符
  println!("\n字符串转字符:");
  let string = "Hello";
  if let Some(first_char) = string.chars().next() {
    println!("字符串 \"{}\" 的第一个字符: '{}'", string, first_char);
  }

  // 字符转字符串
  let ch = '世';
  // ✅ 优化：使用 format! 或直接使用字符（根据需求）
  let char_to_string = ch.to_string(); // 需要 String 时使用
  // 如果只需要显示，可以直接使用字符：println!("{}", ch);
  println!("字符 '{}' 转字符串: \"{}\"", ch, char_to_string);

  // From/Into trait 示例 <mcreference link="https://practice-zh.course.rs/type-conversions/from-into.html" index="5">5</mcreference>
  println!("\nFrom/Into trait 转换:");
  let bool_to_int: i32 = false.into();
  let bool_from_int = i32::from(true);
  println!("false.into(): {}", bool_to_int);
  println!("i32::from(true): {}", bool_from_int);
}

/// 12. 实际应用案例
fn practical_examples() {
  println!("\n=== 12. 实际应用案例 ===");

  // 案例1: 字符分类和处理
  fn analyze_text(text: &str) {
    let mut letter_count = 0;
    let mut digit_count = 0;
    let mut space_count = 0;
    let mut other_count = 0;

    for ch in text.chars() {
      if ch.is_alphabetic() {
        letter_count += 1;
      } else if ch.is_numeric() {
        digit_count += 1;
      } else if ch.is_whitespace() {
        space_count += 1;
      } else {
        other_count += 1;
      }
    }

    println!(
      "文本分析 \"{}\": 字母{}, 数字{}, 空格{}, 其他{}",
      text, letter_count, digit_count, space_count, other_count
    );
  }

  analyze_text("Hello 123 世界!");

  // 案例2: 密码强度检查
  fn check_password_strength(password: &str) -> bool {
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;
    let mut has_special = false;

    for ch in password.chars() {
      if ch.is_ascii_uppercase() {
        has_upper = true;
      } else if ch.is_ascii_lowercase() {
        has_lower = true;
      } else if ch.is_ascii_digit() {
        has_digit = true;
      } else if !ch.is_ascii_alphanumeric() {
        has_special = true;
      }
    }

    let is_strong = has_upper && has_lower && has_digit && has_special && password.len() >= 8;

    println!(
      "密码 \"{}\": 大写{}, 小写{}, 数字{}, 特殊字符{}, 长度{}, 强度{}",
      password,
      has_upper,
      has_lower,
      has_digit,
      has_special,
      password.len() >= 8,
      if is_strong { "强" } else { "弱" }
    );

    is_strong
  }

  check_password_strength("Password123!");
  check_password_strength("weak");

  // 案例3: 简单的状态机
  #[derive(Debug, PartialEq)]
  enum State {
    Idle,
    Running,
    Stopped,
  }

  struct Machine {
    state: State,
    is_enabled: bool,
  }

  impl Machine {
    fn new() -> Self {
      Machine {
        state: State::Idle,
        is_enabled: true,
      }
    }

    fn start(&mut self) -> Result<(), &'static str> {
      if !self.is_enabled {
        return Err("机器未启用");
      }

      match self.state {
        State::Idle => {
          self.state = State::Running;
          Ok(())
        }
        State::Running => Err("机器已在运行"),
        State::Stopped => Err("机器已停止，需要重置"),
      }
    }

    fn stop(&mut self) -> Result<(), &'static str> {
      match self.state {
        State::Running => {
          self.state = State::Stopped;
          Ok(())
        }
        _ => Err("机器未在运行"),
      }
    }

    fn reset(&mut self) {
      self.state = State::Idle;
    }

    fn enable(&mut self, enabled: bool) {
      self.is_enabled = enabled;
    }
  }

  let mut machine = Machine::new();
  println!("\n状态机演示:");
  println!(
    "初始状态: {:?}, 启用: {}",
    machine.state, machine.is_enabled
  );

  match machine.start() {
    Ok(()) => println!("启动成功，当前状态: {:?}", machine.state),
    Err(e) => println!("启动失败: {}", e),
  }

  match machine.stop() {
    Ok(()) => println!("停止成功，当前状态: {:?}", machine.state),
    Err(e) => println!("停止失败: {}", e),
  }

  machine.reset();
  println!("重置后状态: {:?}", machine.state);

  // 案例4: 字符编码转换
  fn char_encoding_info(ch: char) {
    println!("\n字符 '{}' 编码信息:", ch);
    println!("  Unicode 码点: U+{:04X}", ch as u32);
    println!("  UTF-8 长度: {} 字节", ch.len_utf8());
    println!("  UTF-16 长度: {} 个16位单元", ch.len_utf16());

    // UTF-8 编码
    let mut utf8_bytes = [0u8; 4];
    let utf8_str = ch.encode_utf8(&mut utf8_bytes);
    print!("  UTF-8 字节: ");
    for &byte in utf8_str.as_bytes() {
      print!("{:02X} ", byte);
    }
    println!();

    // UTF-16 编码
    let mut utf16_units = [0u16; 2];
    let utf16_slice = ch.encode_utf16(&mut utf16_units);
    print!("  UTF-16 单元: ");
    for unit in utf16_slice {
      print!("{:04X} ", unit);
    }
    println!();
  }

  char_encoding_info('A');
  char_encoding_info('中');
  char_encoding_info('🦀');
}

/// 13. 性能和内存考虑
fn performance_and_memory() {
  println!("\n=== 13. 性能和内存考虑 ===");

  use std::mem;

  // 类型大小比较
  println!("类型大小比较:");
  println!("char: {} 字节", mem::size_of::<char>());
  println!("bool: {} 字节", mem::size_of::<bool>());
  println!("(): {} 字节", mem::size_of::<()>());
  println!("u8: {} 字节", mem::size_of::<u8>());
  println!("u32: {} 字节", mem::size_of::<u32>());

  // 数组大小比较
  println!("\n数组大小比较:");
  let char_array: [char; 100] = ['a'; 100];
  let bool_array: [bool; 100] = [true; 100];
  let unit_array: [(); 100] = [(); 100];

  println!("[char; 100]: {} 字节", mem::size_of_val(&char_array));
  println!("[bool; 100]: {} 字节", mem::size_of_val(&bool_array));
  println!("[(); 100]: {} 字节", mem::size_of_val(&unit_array));

  // 字符串 vs 字符数组的性能
  println!("\n字符处理性能考虑:");
  let text = "Hello, 世界! 🦀";

  // 字符迭代（推荐）
  let start = std::time::Instant::now();
  let char_count = text.chars().count();
  let char_duration = start.elapsed();

  // 字节迭代（不推荐用于 Unicode 文本）
  let start = std::time::Instant::now();
  let byte_count = text.bytes().count();
  let byte_duration = start.elapsed();

  println!("文本: \"{}\"", text);
  println!("字符数: {} (耗时: {:?})", char_count, char_duration);
  println!("字节数: {} (耗时: {:?})", byte_count, byte_duration);

  // 布尔值优化
  println!("\n布尔值优化建议:");
  println!("- 使用位字段存储多个布尔值");
  println!("- 考虑使用 Option<T> 而不是 (bool, T)");
  println!("- 在性能关键代码中避免不必要的布尔转换");
}

/// 14. 常见错误和最佳实践
fn common_mistakes_and_best_practices() {
  println!("\n=== 14. 常见错误和最佳实践 ===");

  println!("常见错误:");

  // 错误1: 混淆字符和字符串 <mcreference link="https://practice-zh.course.rs/basic-types/char-bool-unit.html" index="1">1</mcreference>
  println!("\n1. 混淆字符和字符串:");
  // let wrong = "a";  // 这是字符串，不是字符
  let correct = 'a'; // 这是字符
  println!("正确的字符定义: '{}'", correct);

  // 错误2: 假设字符总是 1 字节
  println!("\n2. 字符大小假设错误:");
  println!("所有字符都占用 4 字节，不是 1 字节");

  // 错误3: 直接将数字当作布尔值
  println!("\n3. 数字到布尔值的错误转换:");
  let number = 0;
  // let wrong_bool = number;  // 编译错误
  let correct_bool = number != 0; // 正确的方式
  println!("{} != 0 = {}", number, correct_bool);

  println!("\n最佳实践:");

  // 最佳实践1: 使用适当的字符方法
  println!("\n1. 使用适当的字符方法:");
  let ch = 'A';
  println!("检查是否为字母: {}", ch.is_alphabetic());
  println!("检查是否为 ASCII: {}", ch.is_ascii());

  // 最佳实践2: 明确的布尔表达式
  println!("\n2. 明确的布尔表达式:");
  let age = 20;
  let is_adult = age >= 18; // 明确的布尔表达式
  println!("年龄 {} 是否成年: {}", age, is_adult);

  // 最佳实践3: 合理使用单元类型
  println!("\n3. 合理使用单元类型:");

  // 用于表示"无有意义返回值"的操作
  fn log_message(msg: &str) -> () {
    println!("日志: {}", msg);
  }

  log_message("这是一条日志消息");

  // 最佳实践4: 字符串和字符的转换
  println!("\n4. 字符串和字符的正确转换:");
  let string = "Hello";

  // 获取第一个字符
  if let Some(first) = string.chars().next() {
    println!("第一个字符: '{}'", first);
  }

  // 收集所有字符
  let chars: Vec<char> = string.chars().collect();
  println!("所有字符: {:?}", chars);

  // 最佳实践5: 性能考虑
  println!("\n5. 性能考虑:");
  println!("- 对于 ASCII 文本，优先使用 is_ascii_* 方法");
  println!("- 避免不必要的字符到字符串转换");
  println!("- 在循环中缓存字符分类结果");

  // 最佳实践6: 错误处理
  println!("\n6. 安全的类型转换:");
  let code_point = 0x1F980_u32; // 螃蟹 emoji

  match char::from_u32(code_point) {
    Some(ch) => println!("有效的字符: '{}'", ch),
    None => println!("无效的 Unicode 码点: {}", code_point),
  }
}

// 编译时常量
const MAX_ASCII: u32 = 127;
const UNICODE_MAX: u32 = 0x10FFFF;

#[cfg(test)]
mod tests {
  // 测试模块：使用标准库类型进行测试，无需导入父模块内容

  #[test]
  fn test_char_size() {
    use std::mem::size_of;
    assert_eq!(size_of::<char>(), 4);
  }

  #[test]
  fn test_bool_size() {
    use std::mem::size_of;
    assert_eq!(size_of::<bool>(), 1);
  }

  #[test]
  fn test_unit_size() {
    use std::mem::size_of;
    assert_eq!(size_of::<()>(), 0);
  }

  #[test]
  fn test_char_conversion() {
    let ch = 'A';
    assert_eq!(ch as u32, 65);
    assert_eq!(char::from_u32(65), Some('A'));
  }

  #[test]
  fn test_bool_operations() {
    assert_eq!(true && false, false);
    assert_eq!(true || false, true);
    assert_eq!(!true, false);
  }

  #[test]
  fn test_unit_equality() {
    assert_eq!((), ());

    fn returns_unit() -> () {
      ()
    }

    assert_eq!(returns_unit(), ());
  }

  #[test]
  fn test_char_methods() {
    assert!(('A').is_ascii_uppercase());
    assert!(('5').is_ascii_digit());
    assert!(('中').is_alphabetic());
    assert!(!('中').is_ascii());
  }

  #[test]
  fn test_unicode_ranges() {
    // 测试有效的 Unicode 范围
    assert!(char::from_u32(0x0000).is_some());
    assert!(char::from_u32(0xD7FF).is_some());
    assert!(char::from_u32(0xE000).is_some());
    assert!(char::from_u32(0x10FFFF).is_some());

    // 测试无效的 Unicode 范围（代理对）
    assert!(char::from_u32(0xD800).is_none());
    assert!(char::from_u32(0xDFFF).is_none());

    // 测试超出范围的值
    assert!(char::from_u32(0x110000).is_none());
  }
}
