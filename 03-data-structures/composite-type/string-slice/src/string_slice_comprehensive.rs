// string_slice_comprehensive.rs
// Rust 字符串与切片完整学习指南
// 基于 https://course.rs/basic/compound-type/string-slice.html

/// Rust 字符串与切片完整学习指南
///
/// 本模块涵盖了 Rust 中字符串和切片的所有核心概念：
/// 1. 切片的基本概念
/// 2. 字符串切片 &str
/// 3. String 类型详解
/// 4. 字符串字面量
/// 5. 字符串索引和 UTF-8 编码
/// 6. 字符串操作方法
/// 7. 字符串转换
/// 8. 其他类型的切片
/// 9. 性能考虑和最佳实践

pub fn run_all_examples() {
    println!("=== Rust 字符串与切片完整指南 ===");

    // 1. 切片基本概念
    println!("\n1. 切片基本概念:");
    demonstrate_slice_concept();

    // 2. 字符串切片 &str
    println!("\n2. 字符串切片 &str:");
    demonstrate_string_slice();

    // 3. String 类型详解
    println!("\n3. String 类型详解:");
    demonstrate_string_type();

    // 4. 字符串字面量
    println!("\n4. 字符串字面量:");
    demonstrate_string_literals();

    // 5. 字符串索引和 UTF-8 编码
    println!("\n5. 字符串索引和 UTF-8 编码:");
    demonstrate_string_indexing();

    // 6. 字符串操作方法
    println!("\n6. 字符串操作方法:");
    demonstrate_string_operations();

    // 7. 字符串转换
    println!("\n7. 字符串转换:");
    demonstrate_string_conversion();

    // 8. 其他类型的切片
    println!("\n8. 其他类型的切片:");
    demonstrate_other_slices();

    // 9. 性能考虑和最佳实践
    println!("\n9. 性能考虑和最佳实践:");
    demonstrate_performance_best_practices();

    println!("\n=== 字符串与切片指南完成 ===");
}

/// 1. 切片基本概念演示
/// 切片是对集合中部分连续元素的引用，不拥有数据的所有权
fn demonstrate_slice_concept() {
    // 什么是切片？
    // 切片允许你引用集合中部分连续的元素序列，而不是引用整个集合

    let s = String::from("hello world");

    // 字符串切片的语法：&s[start..end]
    let hello = &s[0..5]; // "hello"
    let world = &s[6..11]; // "world"

    println!("原字符串: {}", s);
    println!("切片 hello: {}", hello);
    println!("切片 world: {}", world);

    // 切片语法的简化形式
    let s = "hello world";

    let slice1 = &s[0..2]; // "he"
    let slice2 = &s[..2]; // 等同于 &s[0..2]
    let slice3 = &s[3..]; // "lo world"
    let slice4 = &s[..]; // "hello world" - 整个字符串的切片

    println!("slice1: {}", slice1);
    println!("slice2: {}", slice2);
    println!("slice3: {}", slice3);
    println!("slice4: {}", slice4);
}

/// 2. 字符串切片 &str 详解
/// &str 是字符串切片类型，是对字符串数据的不可变引用
fn demonstrate_string_slice() {
    // &str 类型的特点：
    // 1. 不可变引用
    // 2. 不拥有数据所有权
    // 3. 指向内存中的字符串数据
    // 4. 包含指针和长度信息

    // 字符串字面量就是 &str 类型
    let s1: &str = "Hello, Rust!";
    println!("字符串字面量: {}", s1);

    // 从 String 创建字符串切片
    let s2 = String::from("Hello, World!");
    let slice = &s2[0..5]; // &str 类型
    println!("从 String 创建的切片: {}", slice);

    // 字符串切片作为函数参数
    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }

    greet("Alice"); // 字符串字面量
    greet(&s2); // String 的引用
    greet(&s2[0..5]); // 字符串切片

    // 字符串切片的内存布局
    // &str 包含两个部分：
    // 1. 指向字符串数据的指针
    // 2. 字符串的长度
    let s = "hello";
    println!("字符串 '{}' 的长度: {} 字节", s, s.len());
}

/// 3. String 类型详解
/// String 是可增长、可变、拥有所有权的 UTF-8 编码字符串
fn demonstrate_string_type() {
    // String 类型的特点：
    // 1. 可变且可增长
    // 2. 拥有数据所有权
    // 3. 存储在堆上
    // 4. UTF-8 编码

    // 创建 String 的方法

    // 方法1: String::new() - 创建空字符串
    let mut s1 = String::new();
    s1.push_str("Hello");
    println!("方法1 - String::new(): {}", s1);

    // 方法2: String::from() - 从字符串字面量创建
    let s2 = String::from("Hello, World!");
    println!("方法2 - String::from(): {}", s2);

    // 方法3: to_string() 方法
    let s3 = "Hello, Rust!".to_string();
    println!("方法3 - to_string(): {}", s3);

    // 方法4: String::with_capacity() - 预分配容量
    let mut s4 = String::with_capacity(10);
    s4.push_str("Hello");
    println!("方法4 - with_capacity(): {}, 容量: {}", s4, s4.capacity());

    // String 的内存布局
    // String 包含三个部分：
    // 1. 指向堆上数据的指针
    // 2. 长度 (len)
    // 3. 容量 (capacity)
    let s = String::from("hello");
    println!("String '{}' - 长度: {}, 容量: {}", s, s.len(), s.capacity());
}

/// 4. 字符串字面量详解
/// 字符串字面量是硬编码到程序中的字符串值
fn demonstrate_string_literals() {
    // 字符串字面量的特点：
    // 1. 类型是 &str
    // 2. 存储在程序的二进制文件中
    // 3. 不可变
    // 4. 在程序运行期间一直有效

    // 基本字符串字面量
    let s1 = "Hello, World!";
    println!("基本字符串字面量: {}", s1);

    // 原始字符串字面量 (raw string literals)
    let s2 = r"Hello\nWorld";
    println!("原始字符串字面量: {}", s2);

    // 多行原始字符串
    let s3 = r#"
    这是一个
    多行字符串
    "包含引号"
  "#;
    println!("多行原始字符串: {}", s3);

    // 字节字符串字面量
    let s4 = b"Hello";
    println!("字节字符串字面量: {:?}", s4);

    // 字符串字面量的生命周期是 'static
    let s5: &'static str = "This string lives for the entire program duration";
    println!("静态生命周期字符串: {}", s5);
}

/// 5. 字符串索引和 UTF-8 编码
/// Rust 字符串是 UTF-8 编码，不支持直接索引访问
fn demonstrate_string_indexing() {
    // 为什么 Rust 不支持字符串索引？
    // 1. UTF-8 编码中，字符可能占用 1-4 个字节
    // 2. 索引操作可能不是 O(1) 时间复杂度
    // 3. 可能导致无效的字符边界访问

    let s = String::from("hello");
    // let h = s[0]; // 这会编译错误！

    // 正确的访问方式：

    // 1. 使用切片（需要确保边界正确）
    let hello = "hello";
    let h = &hello[0..1];
    println!("使用切片访问: {}", h);

    // 2. 使用 chars() 迭代器
    let s = "hello";
    for (i, c) in s.chars().enumerate() {
        println!("字符 {}: {}", i, c);
    }

    // 3. 使用 bytes() 迭代器
    let s = "hello";
    for (i, b) in s.bytes().enumerate() {
        println!("字节 {}: {}", i, b);
    }

    // UTF-8 编码示例
    let s = String::from("中国人");
    println!("字符串 '{}' 的字节长度: {}", s, s.len());
    println!("字符串 '{}' 的字符数量: {}", s, s.chars().count());

    // 遍历中文字符
    for c in s.chars() {
        println!("字符: {}", c);
    }

    // 危险的切片操作示例（会 panic）
    // let s = String::from("中国人");
    // let invalid = &s[0..2]; // 这会 panic！因为切在了字符中间

    // 安全的切片操作
    let s = "中国人";
    let safe_slice = &s[0..3]; // 一个中文字符占 3 个字节
    println!("安全的切片: {}", safe_slice);
}

/// 6. 字符串操作方法
/// String 提供了丰富的操作方法
fn demonstrate_string_operations() {
    // 字符串追加操作

    // push_str() - 追加字符串切片
    let mut s1 = String::from("Hello");
    s1.push_str(", World!");
    println!("push_str 结果: {}", s1);

    // push() - 追加单个字符
    let mut s2 = String::from("Hello");
    s2.push('!');
    println!("push 结果: {}", s2);

    // + 操作符连接字符串
    let s3 = String::from("Hello");
    let s4 = String::from(", World!");
    let s5 = s3 + &s4; // 注意：s3 的所有权被移动了
    println!("+ 操作符结果: {}", s5);
    // println!("{}", s3); // 这会编译错误，因为 s3 已被移动

    // format! 宏连接字符串
    let s6 = String::from("Hello");
    let s7 = String::from("World");
    let s8 = format!("{}, {}!", s6, s7);
    println!("format! 结果: {}", s8);
    println!("s6 仍然有效: {}", s6); // s6 和 s7 仍然有效

    // 字符串替换操作
    let s = String::from("Hello, World!");
    let new_s = s.replace("World", "Rust");
    println!("replace 结果: {}", new_s);

    // 字符串删除操作
    let mut s = String::from("Hello, World!");

    // pop() - 删除并返回最后一个字符
    if let Some(c) = s.pop() {
        println!("pop 删除的字符: {}", c);
    }
    println!("pop 后的字符串: {}", s);

    // remove() - 删除指定位置的字符
    let mut s = String::from("Hello!");
    let c = s.remove(5); // 删除 '!'
    println!("remove 删除的字符: {}", c);
    println!("remove 后的字符串: {}", s);

    // truncate() - 截断字符串
    let mut s = String::from("Hello, World!");
    s.truncate(5);
    println!("truncate 后的字符串: {}", s);

    // clear() - 清空字符串
    let mut s = String::from("Hello, World!");
    s.clear();
    println!("clear 后的字符串长度: {}", s.len());

    // 字符串查找操作
    let s = "Hello, World!";

    // contains() - 检查是否包含子字符串
    println!("包含 'World': {}", s.contains("World"));

    // starts_with() 和 ends_with()
    println!("以 'Hello' 开头: {}", s.starts_with("Hello"));
    println!("以 '!' 结尾: {}", s.ends_with("!"));

    // find() - 查找子字符串位置
    if let Some(pos) = s.find("World") {
        println!("'World' 的位置: {}", pos);
    }
}

/// 7. 字符串转换
/// 不同字符串类型之间的转换
fn demonstrate_string_conversion() {
    // &str 转 String
    let str_slice = "Hello, World!";

    // 方法1: to_string()
    let string1 = str_slice.to_string();
    println!("to_string(): {}", string1);

    // 方法2: String::from()
    let string2 = String::from(str_slice);
    println!("String::from(): {}", string2);

    // 方法3: to_owned()
    let string3 = str_slice.to_owned();
    println!("to_owned(): {}", string3);

    // String 转 &str
    let string = String::from("Hello, World!");

    // 方法1: 借用 (最常用)
    let str_slice1: &str = &string;
    println!("借用转换: {}", str_slice1);

    // 方法2: as_str()
    let str_slice2: &str = string.as_str();
    println!("as_str(): {}", str_slice2);

    // 字符串与其他类型的转换

    // 数字转字符串
    let num = 42;
    let num_str = num.to_string();
    println!("数字转字符串: {}", num_str);

    // 字符串转数字
    let str_num = "42";
    let parsed_num: i32 = str_num.parse().expect("解析失败");
    println!("字符串转数字: {}", parsed_num);

    // 使用 parse() 的安全版本
    let str_num = "not_a_number";
    match str_num.parse::<i32>() {
        Ok(num) => println!("解析成功: {}", num),
        Err(e) => println!("解析失败: {}", e),
    }

    // 字符串与字节的转换
    let s = String::from("Hello");

    // String 转 Vec<u8>
    let bytes = s.into_bytes();
    println!("字符串转字节: {:?}", bytes);

    // Vec<u8> 转 String
    let bytes = vec![72, 101, 108, 108, 111]; // "Hello" 的字节
    let s = String::from_utf8(bytes).expect("无效的 UTF-8");
    println!("字节转字符串: {}", s);

    // 不安全的转换（不检查 UTF-8 有效性）
    let bytes = vec![72, 101, 108, 108, 111];
    let s = unsafe { String::from_utf8_unchecked(bytes) };
    println!("不安全转换: {}", s);
}

/// 8. 其他类型的切片
/// 除了字符串切片，Rust 还支持其他类型的切片
fn demonstrate_other_slices() {
    // 数组切片
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4]; // [2, 3, 4]
    println!("数组切片: {:?}", slice);

    // Vector 切片
    let vec = vec![1, 2, 3, 4, 5];
    let slice = &vec[1..4];
    println!("Vector 切片: {:?}", slice);

    // 切片作为函数参数
    fn print_slice(slice: &[i32]) {
        println!("切片内容: {:?}", slice);
    }

    print_slice(&arr[..]); // 传递数组切片
    print_slice(&vec[..]); // 传递 Vector 切片
    print_slice(slice); // 传递已有切片

    // 可变切片
    let mut arr = [1, 2, 3, 4, 5];
    let slice = &mut arr[1..4];
    slice[0] = 10; // 修改切片中的元素
    println!("修改后的数组: {:?}", arr);

    // 切片的方法
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[..];

    println!("切片长度: {}", slice.len());
    println!("切片是否为空: {}", slice.is_empty());
    println!("第一个元素: {:?}", slice.first());
    println!("最后一个元素: {:?}", slice.last());

    // 切片迭代
    for (i, &value) in slice.iter().enumerate() {
        println!("索引 {}: 值 {}", i, value);
    }

    // 切片分割
    let slice = &[1, 2, 3, 4, 5][..];
    let (left, right) = slice.split_at(2);
    println!("左半部分: {:?}", left);
    println!("右半部分: {:?}", right);
}

/// 9. 性能考虑和最佳实践
/// 字符串和切片的性能优化建议
fn demonstrate_performance_best_practices() {
    println!("=== 性能考虑和最佳实践 ===");

    // 1. 选择合适的字符串类型
    println!("\n1. 字符串类型选择:");

    // 函数参数优先使用 &str
    fn process_string_good(s: &str) {
        println!("处理字符串: {}", s);
    }

    // 避免不必要的 String 参数
    fn process_string_bad(s: String) {
        println!("处理字符串: {}", s);
    }

    let s = String::from("Hello");
    process_string_good(&s); // 好：可以传递 String 的引用
    // process_string_bad(s);   // 坏：会转移所有权

    // 2. 预分配容量
    println!("\n2. 预分配容量:");

    // 好：预分配足够的容量
    let mut s = String::with_capacity(100);
    for i in 0..10 {
        s.push_str(&format!("Item {} ", i));
    }
    println!("预分配容量的字符串: {}", s);

    // 3. 避免不必要的分配
    println!("\n3. 避免不必要的分配:");

    // 好：使用 format! 一次性创建
    let name = "Alice";
    let age = 30;
    let message = format!("Hello, {}! You are {} years old.", name, age);
    println!("一次性创建: {}", message);

    // 坏：多次分配和连接
    // let mut message = String::from("Hello, ");
    // message.push_str(name);
    // message.push_str("! You are ");
    // message.push_str(&age.to_string());
    // message.push_str(" years old.");

    // 4. 字符串比较的性能
    println!("\n4. 字符串比较:");

    let s1 = "hello";
    let s2 = String::from("hello");

    // 字符串切片比较（快）
    println!("&str 比较: {}", s1 == "hello");

    // String 与 &str 比较（快）
    println!("String 与 &str 比较: {}", s2 == "hello");

    // 5. 迭代的性能
    println!("\n5. 字符串迭代:");

    let s = "Hello, 世界!";

    // 按字符迭代（处理 Unicode）
    print!("按字符: ");
    for c in s.chars() {
        print!("{} ", c);
    }
    println!();

    // 按字节迭代（最快，但不处理 Unicode）
    print!("按字节: ");
    for b in s.bytes() {
        print!("{} ", b);
    }
    println!();

    // 6. 内存使用优化
    println!("\n6. 内存使用:");

    let mut s = String::with_capacity(10);
    s.push_str("hello");
    println!("字符串: '{}', 长度: {}, 容量: {}", s, s.len(), s.capacity());

    // 收缩容量以节省内存
    s.shrink_to_fit();
    println!("收缩后: '{}', 长度: {}, 容量: {}", s, s.len(), s.capacity());

    // 7. 最佳实践总结
    println!("\n=== 最佳实践总结 ===");
    println!("• 函数参数优先使用 &str 而不是 String");
    println!("• 需要修改字符串时才使用 String");
    println!("• 大量字符串操作时预分配容量");
    println!("• 使用 format! 宏进行复杂的字符串格式化");
    println!("• 注意 UTF-8 编码，避免无效的切片操作");
    println!("• 选择合适的迭代方式（chars vs bytes）");
    println!("• 适时使用 shrink_to_fit() 节省内存");
}

/// 字符串处理的实用函数示例
pub mod string_utilities {
    /// 安全地获取字符串的前 n 个字符
    pub fn safe_substring(s: &str, start: usize, len: usize) -> Option<&str> {
        let mut char_indices = s.char_indices();

        // 找到起始位置
        let start_byte = char_indices.nth(start)?.0;

        // 找到结束位置
        let end_byte = if len == 0 {
            start_byte
        } else {
            char_indices.nth(len - 1).map(|(i, _)| i).unwrap_or(s.len())
        };

        s.get(start_byte..end_byte)
    }

    /// 统计字符串中的字符数（Unicode 感知）
    pub fn char_count(s: &str) -> usize {
        s.chars().count()
    }

    /// 反转字符串（Unicode 感知）
    pub fn reverse_string(s: &str) -> String {
        s.chars().rev().collect()
    }

    /// 检查字符串是否为回文
    pub fn is_palindrome(s: &str) -> bool {
        let cleaned: String = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_lowercase().next().unwrap())
            .collect();

        let reversed: String = cleaned.chars().rev().collect();
        cleaned == reversed
    }

    /// 单词计数
    pub fn word_count(s: &str) -> usize {
        s.split_whitespace().count()
    }

    /// 首字母大写
    pub fn capitalize_first(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

/// 演示实用函数
fn demonstrate_utilities() {
    use string_utilities::*;

    println!("\n=== 字符串实用函数演示 ===");

    let s = "Hello, 世界! This is a test.";

    // 安全子字符串
    if let Some(sub) = safe_substring(s, 0, 5) {
        println!("前5个字符: {}", sub);
    }

    // 字符计数
    println!("字符数: {}", char_count(s));

    // 字符串反转
    println!("反转: {}", reverse_string("hello"));

    // 回文检查
    println!("'racecar' 是回文: {}", is_palindrome("racecar"));
    println!("'hello' 是回文: {}", is_palindrome("hello"));

    // 单词计数
    println!("单词数: {}", word_count(s));

    // 首字母大写
    println!("首字母大写: {}", capitalize_first("hello world"));
}

/// 常见错误和解决方案
fn demonstrate_common_mistakes() {
    println!("\n=== 常见错误和解决方案 ===");

    // 错误1: 字符串索引
    println!("\n错误1: 尝试索引字符串");
    let s = "hello";
    // let c = s[0]; // 编译错误！

    // 正确做法：
    let c = s.chars().next().unwrap();
    println!("第一个字符: {}", c);

    // 错误2: 在 UTF-8 字符边界切片
    println!("\n错误2: UTF-8 边界问题");
    let s = "中国";
    // let invalid = &s[0..2]; // 运行时 panic！

    // 正确做法：
    let valid = &s[0..3]; // 一个中文字符占3字节
    println!("正确的切片: {}", valid);

    // 错误3: 不必要的 String 分配
    println!("\n错误3: 不必要的分配");

    // 低效的做法：
    fn process_bad(s: String) -> String {
        format!("Processed: {}", s)
    }

    // 高效的做法：
    fn process_good(s: &str) -> String {
        format!("Processed: {}", s)
    }

    let original = String::from("test");
    let result = process_good(&original); // 不转移所有权
    println!("处理结果: {}", result);
    println!("原字符串仍可用: {}", original);
}

/// 主函数，运行所有示例
pub fn run_comprehensive_examples() {
    run_all_examples();
    demonstrate_utilities();
    demonstrate_common_mistakes();

    println!("\n✅ 恭喜！你已经完成了 Rust 字符串与切片的完整学习！");
    println!("💡 关键要点回顾：");
    println!("   • 切片是对集合部分元素的引用，不拥有所有权");
    println!("   • &str 是字符串切片，String 是拥有所有权的字符串");
    println!("   • Rust 字符串是 UTF-8 编码，不支持直接索引");
    println!("   • 选择合适的字符串类型可以提高性能");
    println!("   • 注意字符边界，避免无效的切片操作");
    println!("\n🚀 继续探索 Rust 的其他特性吧！");
}
