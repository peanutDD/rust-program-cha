//! Rust 生命周期深度分析文档
//! 基于 https://course.rs/basic/lifetime.html 的全面知识点分析
//! 包含生命周期的所有核心概念、语法规则、高级特性和实际应用案例

use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Rust 生命周期深度分析文档 ===");
    println!();

    // 1. 生命周期基础概念
    lifetime_basics_demo();

    // 2. 生命周期语法
    lifetime_syntax_demo();

    // 3. 生命周期规则
    lifetime_rules_demo();

    // 4. 结构体中的生命周期
    lifetime_structs_demo();

    // 5. 高级生命周期特性
    lifetime_advanced_demo();

    // 6. 常见生命周期模式
    lifetime_patterns_demo();

    // 7. 生命周期错误处理
    lifetime_errors_demo();

    // 8. 实际应用案例
    lifetime_real_world_demo();

    println!("\n=== 生命周期分析完成 ===");
}

/// 1. 生命周期基础概念演示
fn lifetime_basics_demo() {
    println!("\n1. 生命周期基础概念");
    println!("===================");

    // 1.1 什么是生命周期
    println!("\n1.1 什么是生命周期？");
    println!("生命周期是引用保持有效的作用域。它确保引用在被使用时始终指向有效的数据。");

    // 基本生命周期示例
    {
        let r: Option<&i32> = None; // 声明引用 r，但未初始化
        {
            let x = 5; // x 的生命周期开始
            // r = Some(&x);  // 错误！x 的生命周期比 r 短
        } // x 的生命周期结束
        // println!("r: {:?}", r); // 错误！r 引用了已销毁的 x
    }

    // 正确的生命周期示例
    {
        let x = 5; // x 的生命周期开始
        let r = &x; // r 引用 x，生命周期合法
        println!("正确的引用: r = {}", r);
    } // x 和 r 的生命周期都结束

    // 1.2 为什么需要生命周期
    println!("\n1.2 为什么需要生命周期？");
    println!("- 防止悬垂引用（dangling references）");
    println!("- 确保内存安全");
    println!("- 在编译时检查引用的有效性");
    println!("- 避免运行时的内存访问错误");

    // 悬垂引用示例（编译错误）
    // let reference_to_nothing = dangle(); // 这会导致编译错误

    // 1.3 生命周期与所有权的关系
    println!("\n1.3 生命周期与所有权的关系");
    println!("- 所有权确保每个值都有一个所有者");
    println!("- 生命周期确保引用在其引用的值被销毁之前保持有效");
    println!("- 借用检查器使用生命周期来验证所有借用都是有效的");

    ownership_and_lifetime_demo();
}

/// 所有权和生命周期关系演示
fn ownership_and_lifetime_demo() {
    println!("\n所有权和生命周期关系演示:");

    let string1 = String::from("hello");
    let string2 = String::from("world");

    // 函数调用，涉及生命周期
    let result = longest(&string1, &string2);
    println!("最长的字符串: {}", result);

    // 展示不同作用域的生命周期
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        println!("在内部作用域中，最长的字符串: {}", result);
    }
    // result 在这里仍然有效，因为它引用的是 string1
    // 但如果 result 引用的是 string2，这里就会出错
}

/// 2. 生命周期语法演示
fn lifetime_syntax_demo() {
    println!("\n2. 生命周期语法");
    println!("===============");

    // 2.1 生命周期参数
    println!("\n2.1 生命周期参数语法");
    println!("生命周期参数以单引号开头，通常使用短小的名称如 'a, 'b, 'static");

    // 基本语法示例
    let string1 = "hello";
    let string2 = "world";
    let result = longest_with_explicit_lifetime(&string1, &string2);
    println!("显式生命周期标注结果: {}", result);

    // 2.2 函数签名中的生命周期
    println!("\n2.2 函数签名中的生命周期");
    syntax_examples();

    // 2.3 多个生命周期参数
    println!("\n2.3 多个生命周期参数");
    multiple_lifetime_demo();
}

/// 显式生命周期标注的 longest 函数
fn longest_with_explicit_lifetime<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

/// 语法示例
fn syntax_examples() {
    println!("函数签名语法示例:");
    println!("fn function<'a>(param: &'a Type) -> &'a ReturnType");
    println!("fn function<'a, 'b>(param1: &'a Type1, param2: &'b Type2) -> &'a ReturnType");

    // 实际示例
    let text = "Hello, Rust!";
    let first_word = get_first_word(&text);
    println!("第一个单词: {}", first_word);

    let numbers = vec![1, 2, 3, 4, 5];
    let first_element = get_first_element(&numbers);
    match first_element {
        Some(value) => println!("第一个元素: {}", value),
        None => println!("向量为空"),
    }
}

/// 获取第一个单词
fn get_first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

/// 获取第一个元素
fn get_first_element<'a, T>(slice: &'a [T]) -> Option<&'a T> {
    slice.first()
}

/// 多个生命周期参数演示
fn multiple_lifetime_demo() {
    let string1 = "hello";
    let string2 = "world";
    let announcement = "This is an announcement!";

    let result = longest_with_announcement(&string1, &string2, announcement);
    println!("带公告的最长字符串: {}", result);

    // 不同生命周期的复杂示例
    complex_lifetime_example();
}

/// 带公告的最长字符串函数
fn longest_with_announcement<'a, 'b>(x: &'a str, y: &'a str, ann: &'b str) -> &'a str {
    println!("公告: {}", ann);
    if x.len() > y.len() { x } else { y }
}

/// 复杂生命周期示例
fn complex_lifetime_example() {
    println!("\n复杂生命周期示例:");

    let string1 = String::from("long string is long");
    let string2 = String::from("short");
    let string3 = String::from("medium length");

    let result = find_longest_of_three(&string1, &string2, &string3);
    println!("三个字符串中最长的: {}", result);
}

/// 找到三个字符串中最长的
fn find_longest_of_three<'a>(x: &'a str, y: &'a str, z: &'a str) -> &'a str {
    let longest_xy = if x.len() > y.len() { x } else { y };
    if longest_xy.len() > z.len() {
        longest_xy
    } else {
        z
    }
}

/// 3. 生命周期规则演示
fn lifetime_rules_demo() {
    println!("\n3. 生命周期规则");
    println!("===============");

    // 3.1 生命周期省略规则
    println!("\n3.1 生命周期省略规则");
    elision_rules_demo();

    // 3.2 输入输出生命周期
    println!("\n3.2 输入输出生命周期");
    input_output_lifetime_demo();

    // 3.3 生命周期子类型
    println!("\n3.3 生命周期子类型");
    lifetime_subtyping_demo();
}

/// 生命周期省略规则演示
fn elision_rules_demo() {
    println!("生命周期省略规则:");
    println!("1. 每个引用参数都有自己的生命周期参数");
    println!("2. 如果只有一个输入生命周期参数，该生命周期被赋予所有输出生命周期参数");
    println!(
        "3. 如果有多个输入生命周期参数，但其中一个是 &self 或 &mut self，则 self 的生命周期被赋予所有输出生命周期参数"
    );

    // 规则1示例：每个参数都有自己的生命周期
    let text = "hello world";
    let first_char = get_first_char(&text);
    println!("第一个字符: {:?}", first_char);

    // 规则2示例：单个输入参数
    let numbers = vec![1, 2, 3];
    let last = get_last(&numbers);
    println!("最后一个元素: {:?}", last);

    // 展示编译器如何应用省略规则
    demonstrate_elision_rules();
}

/// 获取第一个字符（省略生命周期）
fn get_first_char(s: &str) -> Option<char> {
    s.chars().next()
}

/// 获取最后一个元素（省略生命周期）
fn get_last<T>(slice: &[T]) -> Option<&T> {
    slice.last()
}

/// 演示省略规则
fn demonstrate_elision_rules() {
    println!("\n省略规则演示:");

    // 这些函数签名是等价的：
    // fn first_word(s: &str) -> &str
    // fn first_word<'a>(s: &'a str) -> &'a str

    let sentence = "Hello Rust programming";
    let word1 = first_word_implicit(&sentence);
    let word2 = first_word_explicit(&sentence);

    println!("隐式生命周期: {}", word1);
    println!("显式生命周期: {}", word2);
}

/// 隐式生命周期版本
fn first_word_implicit(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

/// 显式生命周期版本
fn first_word_explicit<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

/// 输入输出生命周期演示
fn input_output_lifetime_demo() {
    println!("输入输出生命周期关系:");

    let data = vec![1, 2, 3, 4, 5];
    let slice = &data[1..4];
    let processed = process_slice(slice);
    println!("处理后的切片: {:?}", processed);

    // 多个输入，一个输出
    let str1 = "hello";
    let str2 = "world";
    let combined = combine_strings(str1, str2);
    println!("组合字符串: {}", combined);
}

/// 处理切片（输入输出生命周期相同）
fn process_slice<'a>(slice: &'a [i32]) -> &'a [i32] {
    // 返回原切片的一部分
    if slice.len() > 2 {
        &slice[1..slice.len() - 1]
    } else {
        slice
    }
}

/// 组合字符串（需要显式生命周期）
fn combine_strings<'a>(s1: &'a str, s2: &str) -> &'a str {
    // 这里只能返回 s1，因为返回类型的生命周期是 'a
    if s1.len() > s2.len() {
        s1
    } else {
        s1 // 不能返回 s2，因为它的生命周期可能不是 'a
    }
}

/// 生命周期子类型演示
fn lifetime_subtyping_demo() {
    println!("生命周期子类型关系:");
    println!("如果 'a: 'b，则 'a 是 'b 的子类型，'a 至少和 'b 一样长");

    subtyping_example();
}

/// 子类型示例
fn subtyping_example() {
    let string1 = String::from("long string");
    {
        let string2 = String::from("short");
        let result = choose_str(&string1, &string2, true);
        println!("选择的字符串: {}", result);
    }
    // string1 仍然有效
    println!("string1 仍然有效: {}", string1);
}

/// 选择字符串（演示生命周期约束）
fn choose_str<'a, 'b: 'a>(s1: &'a str, s2: &'b str, choose_first: bool) -> &'a str {
    if choose_first {
        s1
    } else {
        s2 // 这里 'b: 'a 约束确保 s2 的生命周期至少和 'a 一样长
    }
}

/// 4. 结构体中的生命周期演示
fn lifetime_structs_demo() {
    println!("\n4. 结构体中的生命周期");
    println!("=====================");

    // 4.1 结构体生命周期参数
    println!("\n4.1 结构体生命周期参数");
    struct_lifetime_demo();

    // 4.2 方法中的生命周期
    println!("\n4.2 方法中的生命周期");
    method_lifetime_demo();

    // 4.3 impl块生命周期
    println!("\n4.3 impl块生命周期");
    impl_lifetime_demo();
}

/// 包含引用的结构体
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

/// 包含多个引用的结构体
#[derive(Debug)]
struct DoubleRef<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

/// 结构体生命周期演示
fn struct_lifetime_demo() {
    println!("结构体生命周期参数演示:");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("重要摘录: {:?}", excerpt);

    // 多个生命周期参数
    let text1 = "Hello";
    let text2 = "World";

    let double_ref = DoubleRef {
        first: text1,
        second: text2,
    };

    println!("双重引用: {:?}", double_ref);
}

/// 方法中的生命周期演示
fn method_lifetime_demo() {
    println!("方法中的生命周期:");

    let text = "Hello, Rust programming!";
    let excerpt = ImportantExcerpt { part: text };

    let level = excerpt.announce_and_return_part("Today is a great day!");
    println!("返回的部分: {}", level);

    // 使用生命周期方法
    let first_word = excerpt.get_first_word();
    println!("第一个单词: {}", first_word);
}

impl<'a> ImportantExcerpt<'a> {
    /// 公告并返回部分内容
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注意！{}", announcement);
        self.part
    }

    /// 获取第一个单词
    fn get_first_word(&self) -> &str {
        self.part.split_whitespace().next().unwrap_or("")
    }

    /// 获取长度
    fn len(&self) -> usize {
        self.part.len()
    }
}

/// impl块生命周期演示
fn impl_lifetime_demo() {
    println!("impl块生命周期演示:");

    let content = "This is some important content for analysis.";
    let analyzer = TextAnalyzer::new(content);

    println!("文本长度: {}", analyzer.length());
    println!("单词数量: {}", analyzer.word_count());
    println!("第一个单词: {}", analyzer.first_word());
    println!("最后一个单词: {}", analyzer.last_word());
}

/// 文本分析器结构体
#[derive(Debug)]
struct TextAnalyzer<'a> {
    text: &'a str,
}

impl<'a> TextAnalyzer<'a> {
    fn new(text: &'a str) -> Self {
        TextAnalyzer { text }
    }

    fn length(&self) -> usize {
        self.text.len()
    }

    fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }

    fn first_word(&self) -> &str {
        self.text.split_whitespace().next().unwrap_or("")
    }

    fn last_word(&self) -> &str {
        self.text.split_whitespace().last().unwrap_or("")
    }

    fn contains(&self, pattern: &str) -> bool {
        self.text.contains(pattern)
    }
}

/// 5. 高级生命周期特性演示
fn lifetime_advanced_demo() {
    println!("\n5. 高级生命周期特性");
    println!("==================");

    // 5.1 高阶生命周期
    println!("\n5.1 高阶生命周期 (HRTB - Higher-Ranked Trait Bounds)");
    higher_ranked_demo();

    // 5.2 生命周期边界
    println!("\n5.2 生命周期边界");
    lifetime_bounds_demo();

    // 5.3 'static生命周期
    println!("\n5.3 'static生命周期");
    static_lifetime_demo();
}

/// 高阶生命周期演示
fn higher_ranked_demo() {
    println!("高阶生命周期演示:");

    // 使用高阶生命周期的函数
    let closure = |x: &str| x.len();
    let result = apply_to_string(&closure, "Hello, HRTB!");
    println!("字符串长度: {}", result);

    // 更复杂的高阶生命周期示例
    hrtb_complex_example();
}

/// 应用到字符串的高阶函数
fn apply_to_string<F>(f: &F, s: &str) -> usize
where
    F: for<'a> Fn(&'a str) -> usize,
{
    f(s)
}

/// 复杂的HRTB示例
fn hrtb_complex_example() {
    println!("\n复杂的HRTB示例:");

    let strings = vec!["hello", "world", "rust", "programming"];
    let max_len = find_max_length(&strings);
    println!("最大长度: {}", max_len);
}

/// 找到最大长度
fn find_max_length(strings: &[&str]) -> usize {
    let len_fn: fn(&str) -> usize = |s: &str| s.len();
    strings.iter().map(|&s| len_fn(s)).max().unwrap_or(0)
}

/// 生命周期边界演示
fn lifetime_bounds_demo() {
    println!("生命周期边界演示:");

    let data = vec![1, 2, 3, 4, 5];
    let container = Container::new(&data);

    let processed = container.process_with_context("processing");
    println!("处理结果: {:?}", processed);

    // 生命周期边界约束
    lifetime_bound_constraint_demo();
}

/// 容器结构体
struct Container<'a, T> {
    data: &'a [T],
}

impl<'a, T> Container<'a, T>
where
    T: Clone + std::fmt::Debug,
{
    fn new(data: &'a [T]) -> Self {
        Container { data }
    }

    fn process_with_context<'b>(&self, context: &'b str) -> Vec<T>
    where
        'a: 'b, // 生命周期边界：'a 必须比 'b 长
    {
        println!("处理上下文: {}", context);
        self.data.to_vec()
    }
}

/// 生命周期边界约束演示
fn lifetime_bound_constraint_demo() {
    println!("\n生命周期边界约束:");

    let text = String::from("Hello, lifetime bounds!");
    let processor = TextProcessor::new(&text);

    let result = processor.process_with_prefix("PREFIX: ");
    println!("处理结果: {}", result);
}

/// 文本处理器
struct TextProcessor<'a> {
    text: &'a str,
}

impl<'a> TextProcessor<'a> {
    fn new(text: &'a str) -> Self {
        TextProcessor { text }
    }

    fn process_with_prefix<'b>(&self, prefix: &'b str) -> String
    where
        'a: 'b,
    {
        format!("{}{}", prefix, self.text)
    }
}

/// 'static生命周期演示
fn static_lifetime_demo() {
    println!("'static生命周期演示:");

    // 字符串字面量具有 'static 生命周期
    let static_str: &'static str = "This string lives for the entire program duration";
    println!("静态字符串: {}", static_str);

    // 使用 'static 生命周期的函数
    let result = get_static_str();
    println!("静态函数返回: {}", result);

    // 'static 约束
    static_constraint_demo();

    // 泄漏内存以获得 'static 引用
    leak_for_static_demo();
}

/// 返回静态字符串
fn get_static_str() -> &'static str {
    "This is a static string"
}

/// 'static约束演示
fn static_constraint_demo() {
    println!("\n'static约束演示:");

    let static_data = "static data";
    store_static_ref(static_data);

    // 使用Box::leak创建'static引用
    let boxed_string = Box::new(String::from("leaked string"));
    let static_ref: &'static str = Box::leak(boxed_string);
    store_static_ref(static_ref);
}

/// 存储静态引用
fn store_static_ref(data: &'static str) {
    println!("存储的静态数据: {}", data);
    // 这里可以将 data 存储在全局变量中，因为它有 'static 生命周期
}

/// 泄漏内存获得'static引用演示
fn leak_for_static_demo() {
    println!("\n泄漏内存获得'static引用:");

    let dynamic_string = String::from("This will be leaked");
    let static_ref: &'static str = Box::leak(dynamic_string.into_boxed_str());

    println!("泄漏的字符串: {}", static_ref);

    // 注意：这种方式会导致内存泄漏，应谨慎使用
    println!("警告：Box::leak 会导致内存泄漏，仅在特殊情况下使用");
}

/// 6. 常见生命周期模式演示
fn lifetime_patterns_demo() {
    println!("\n6. 常见生命周期模式");
    println!("==================");

    // 6.1 借用检查器工作原理
    println!("\n6.1 借用检查器工作原理");
    borrow_checker_demo();

    // 6.2 NLL非词法生命周期
    println!("\n6.2 NLL (Non-Lexical Lifetimes) 非词法生命周期");
    nll_demo();

    // 6.3 生命周期协变逆变
    println!("\n6.3 生命周期协变逆变");
    variance_demo();
}

/// 借用检查器工作原理演示
fn borrow_checker_demo() {
    println!("借用检查器工作原理:");
    println!("1. 分析每个引用的生命周期");
    println!("2. 确保引用不会超出其引用数据的生命周期");
    println!("3. 检查可变借用和不可变借用的规则");

    // 正确的借用模式
    {
        let mut data = vec![1, 2, 3, 4, 5];
        let slice = &data[1..4];
        println!("切片: {:?}", slice);
        // slice 的生命周期在这里结束

        data.push(6); // 现在可以修改 data
        println!("修改后的数据: {:?}", data);
    }

    // 借用检查器的生命周期分析
    borrow_checker_analysis();
}

/// 借用检查器分析示例
fn borrow_checker_analysis() {
    println!("\n借用检查器分析示例:");

    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);

    // 借用检查器分析这些操作的生命周期
    {
        let alice_score = scores.get("Alice"); // 不可变借用开始
        match alice_score {
            Some(score) => println!("Alice的分数: {}", score),
            None => println!("未找到Alice的分数"),
        }
        // 不可变借用在这里结束
    }

    scores.insert("Charlie", 92); // 现在可以修改
    println!("所有分数: {:?}", scores);
}

/// NLL非词法生命周期演示
fn nll_demo() {
    println!("NLL非词法生命周期演示:");
    println!("NLL允许引用的生命周期在最后一次使用后结束，而不是在作用域结束时");

    // NLL之前的问题
    let mut data = vec![1, 2, 3];

    // 在NLL之前，这段代码可能无法编译
    // 因为 r 的生命周期会持续到作用域结束
    let r = &data[0];
    println!("第一个元素: {}", r);
    // r 在这里实际上不再被使用

    data.push(4); // NLL使这成为可能
    println!("修改后的数据: {:?}", data);

    // 更复杂的NLL示例
    nll_complex_example();
}

/// 复杂的NLL示例
fn nll_complex_example() {
    println!("\n复杂的NLL示例:");

    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    // NLL允许这种模式
    let value = map.get("key1");
    if let Some(v) = value {
        println!("找到值: {}", v);
    }
    // value 的生命周期在这里结束

    map.insert("key3", "value3"); // 现在可以修改map
    println!("更新后的map: {:?}", map);
}

/// 生命周期协变逆变演示
fn variance_demo() {
    println!("生命周期协变逆变演示:");
    println!("协变：如果 'a: 'b，则 &'a T 可以转换为 &'b T");
    println!("逆变：如果 'a: 'b，则 fn(&'b T) 可以转换为 fn(&'a T)");
    println!("不变：类型参数既不协变也不逆变");

    // 协变示例
    covariance_example();

    // 逆变示例（函数指针）
    contravariance_example();
}

/// 协变示例
fn covariance_example() {
    println!("\n协变示例:");

    let string = String::from("Hello, variance!");
    {
        let string_ref: &str = &string;
        // 这里 string_ref 的生命周期可以"缩短"到更小的作用域
        use_str_ref(string_ref);
    }

    println!("原始字符串仍然有效: {}", string);
}

fn use_str_ref(s: &str) {
    println!("使用字符串引用: {}", s);
}

/// 逆变示例
fn contravariance_example() {
    println!("\n逆变示例（函数指针）:");

    // 函数指针在参数上是逆变的
    let longer_lived = String::from("longer lived string");

    {
        let shorter_lived = String::from("shorter");

        // 这个函数可以接受更长生命周期的参数
        let func: fn(&str) = print_string;
        func(&longer_lived);
        func(&shorter_lived);
    }
}

fn print_string(s: &str) {
    println!("打印字符串: {}", s);
}

/// 7. 生命周期错误处理演示
fn lifetime_errors_demo() {
    println!("\n7. 生命周期错误处理");
    println!("==================");

    // 7.1 常见错误类型
    println!("\n7.1 常见生命周期错误类型");
    common_errors_demo();

    // 7.2 错误诊断
    println!("\n7.2 生命周期错误诊断");
    error_diagnosis_demo();

    // 7.3 解决方案和最佳实践
    println!("\n7.3 解决方案和最佳实践");
    solutions_and_best_practices();
}

/// 常见错误演示
fn common_errors_demo() {
    println!("常见生命周期错误:");
    println!("1. 悬垂引用 (Dangling References)");
    println!("2. 借用检查失败");
    println!("3. 生命周期不匹配");
    println!("4. 返回局部变量的引用");

    // 展示正确的解决方案
    demonstrate_correct_patterns();
}

/// 演示正确的模式
fn demonstrate_correct_patterns() {
    println!("\n正确的生命周期模式:");

    // 正确：返回拥有的数据而不是引用
    let owned_string = create_owned_string();
    println!("拥有的字符串: {}", owned_string);

    // 正确：使用适当的生命周期参数
    let text1 = "hello";
    let text2 = "world";
    let result = safe_longest(text1, text2);
    println!("安全的最长字符串: {}", result);

    // 正确：使用智能指针
    smart_pointer_solution();
}

/// 创建拥有的字符串
fn create_owned_string() -> String {
    String::from("This is owned, not borrowed")
}

/// 安全的最长字符串函数
fn safe_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

/// 智能指针解决方案
fn smart_pointer_solution() {
    println!("\n智能指针解决生命周期问题:");

    // 使用Rc来共享所有权
    let shared_data = Rc::new(String::from("Shared data"));
    let clone1 = Rc::clone(&shared_data);
    let clone2 = Rc::clone(&shared_data);

    println!("原始数据: {}", shared_data);
    println!("克隆1: {}", clone1);
    println!("克隆2: {}", clone2);
    println!("引用计数: {}", Rc::strong_count(&shared_data));
}

/// 错误诊断演示
fn error_diagnosis_demo() {
    println!("生命周期错误诊断技巧:");
    println!("1. 仔细阅读编译器错误信息");
    println!("2. 使用 rustc --explain 获取详细解释");
    println!("3. 绘制生命周期图表");
    println!("4. 逐步简化代码找出问题");

    // 演示诊断过程
    diagnostic_process_demo();
}

/// 诊断过程演示
fn diagnostic_process_demo() {
    println!("\n诊断过程演示:");

    // 步骤1：识别问题
    println!("步骤1：识别涉及的引用和它们的生命周期");

    let data = vec![1, 2, 3, 4, 5];
    let slice = &data[1..4];

    // 步骤2：分析生命周期关系
    println!("步骤2：分析 data 和 slice 的生命周期关系");
    println!("data 的生命周期: 从创建到函数结束");
    println!("slice 的生命周期: 从借用到最后使用");

    // 步骤3：验证解决方案
    println!("步骤3：验证当前代码是否正确");
    println!("slice: {:?}", slice);

    println!("诊断完成：代码正确，没有生命周期问题");
}

/// 解决方案和最佳实践
fn solutions_and_best_practices() {
    println!("生命周期最佳实践:");
    println!("1. 优先使用拥有的类型而不是引用");
    println!("2. 保持生命周期参数简单");
    println!("3. 使用生命周期省略规则");
    println!("4. 考虑使用智能指针");
    println!("5. 避免复杂的生命周期关系");

    best_practices_examples();
}

/// 最佳实践示例
fn best_practices_examples() {
    println!("\n最佳实践示例:");

    // 实践1：使用拥有的类型
    let owned_data = create_processed_data();
    println!("处理后的数据: {:?}", owned_data);

    // 实践2：简单的生命周期
    let text = "Hello, best practices!";
    let word_count = count_words(text);
    println!("单词数量: {}", word_count);

    // 实践3：使用智能指针避免生命周期复杂性
    shared_ownership_example();
}

/// 创建处理后的数据
fn create_processed_data() -> Vec<String> {
    vec![
        String::from("processed"),
        String::from("data"),
        String::from("example"),
    ]
}

/// 计算单词数量
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

/// 共享所有权示例
fn shared_ownership_example() {
    println!("\n共享所有权示例:");

    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let data_clone = Arc::clone(&data);

    // 在不同的上下文中使用共享数据
    let handle = thread::spawn(move || {
        println!("线程中的数据: {:?}", data_clone);
        data_clone.len()
    });

    println!("主线程中的数据: {:?}", data);
    let length = handle.join().unwrap();
    println!("数据长度: {}", length);
}

/// 8. 实际应用案例演示
fn lifetime_real_world_demo() {
    println!("\n8. 实际应用案例");
    println!("===============");

    // 8.1 迭代器生命周期
    println!("\n8.1 迭代器生命周期");
    iterator_lifetime_demo();

    // 8.2 闭包生命周期
    println!("\n8.2 闭包生命周期");
    closure_lifetime_demo();

    // 8.3 异步编程中的生命周期
    println!("\n8.3 异步编程中的生命周期");
    async_lifetime_demo();

    // 8.4 智能指针生命周期
    println!("\n8.4 智能指针生命周期");
    smart_pointer_lifetime_demo();
}

/// 迭代器生命周期演示
fn iterator_lifetime_demo() {
    println!("迭代器生命周期应用:");

    let data = vec!["apple", "banana", "cherry", "date"];

    // 迭代器的生命周期与原始数据相关
    let long_names: Vec<&str> = data
        .iter()
        .filter(|&&name| name.len() > 5)
        .map(|&name| name)
        .collect();

    println!("长名称: {:?}", long_names);

    // 自定义迭代器
    let custom_iter = CustomIterator::new(&data);
    for item in custom_iter {
        println!("自定义迭代器项: {}", item);
    }
}

/// 自定义迭代器
struct CustomIterator<'a> {
    data: &'a [&'a str],
    index: usize,
}

impl<'a> CustomIterator<'a> {
    fn new(data: &'a [&'a str]) -> Self {
        CustomIterator { data, index: 0 }
    }
}

impl<'a> Iterator for CustomIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let item = self.data[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

/// 闭包生命周期演示
fn closure_lifetime_demo() {
    println!("闭包生命周期应用:");

    let data = vec![1, 2, 3, 4, 5];
    let multiplier = 2;

    // 闭包捕获环境中的变量
    let multiply_closure = |x: &i32| x * multiplier;

    let results: Vec<i32> = data.iter().map(multiply_closure).collect();
    println!("乘法结果: {:?}", results);

    // 更复杂的闭包生命周期
    complex_closure_example();
}

/// 复杂闭包示例
fn complex_closure_example() {
    println!("\n复杂闭包生命周期:");

    let text = String::from("Hello, closures!");

    // 闭包借用外部变量
    let process_text = |prefix: &str| format!("{}: {}", prefix, text);

    let result1 = process_text("INFO");
    let result2 = process_text("DEBUG");

    println!("结果1: {}", result1);
    println!("结果2: {}", result2);
}

/// 异步编程生命周期演示
fn async_lifetime_demo() {
    println!("异步编程中的生命周期:");
    println!("在异步编程中，生命周期变得更加复杂，因为异步函数可能在不同时间点执行");

    // 模拟异步操作的同步版本
    simulate_async_operations();
}

/// 模拟异步操作
fn simulate_async_operations() {
    println!("\n模拟异步操作:");

    let data = "async data";

    // 模拟异步处理
    let result = process_async_data(data);
    println!("异步处理结果: {}", result);

    // 异步生命周期的挑战
    println!("\n异步生命周期挑战:");
    println!("1. 跨await点的引用生命周期");
    println!("2. Send和Sync约束");
    println!("3. 'static约束的需求");
}

/// 处理异步数据
fn process_async_data(data: &str) -> String {
    // 在真实的异步函数中，这里会有await
    thread::sleep(Duration::from_millis(10)); // 模拟异步延迟
    format!("Processed: {}", data)
}

/// 智能指针生命周期演示
fn smart_pointer_lifetime_demo() {
    println!("智能指针生命周期应用:");

    // Rc的生命周期管理
    rc_lifetime_example();

    // Arc的生命周期管理
    arc_lifetime_example();

    // Box的生命周期
    box_lifetime_example();
}

/// Rc生命周期示例
fn rc_lifetime_example() {
    println!("\nRc生命周期管理:");

    let data = Rc::new(String::from("Shared string"));
    println!("初始引用计数: {}", Rc::strong_count(&data));

    {
        let clone1 = Rc::clone(&data);
        let clone2 = Rc::clone(&data);
        println!("克隆后引用计数: {}", Rc::strong_count(&data));

        println!("clone1: {}", clone1);
        println!("clone2: {}", clone2);
    } // clone1 和 clone2 在这里被销毁

    println!("作用域结束后引用计数: {}", Rc::strong_count(&data));
}

/// Arc生命周期示例
fn arc_lifetime_example() {
    println!("\nArc生命周期管理（线程安全）:");

    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];

    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("线程 {} 看到的数据: {:?}", i, data_clone);
            data_clone.len()
        });
        handles.push(handle);
    }

    for handle in handles {
        let length = handle.join().unwrap();
        println!("线程返回的长度: {}", length);
    }

    println!("主线程中的数据: {:?}", data);
}

/// Box生命周期示例
fn box_lifetime_example() {
    println!("\nBox生命周期管理:");

    // Box拥有堆上的数据
    let boxed_data = Box::new(String::from("Boxed string"));
    println!("装箱的数据: {}", boxed_data);

    // Box可以转移所有权
    let moved_box = boxed_data;
    println!("移动后的Box: {}", moved_box);

    // 使用Box存储大型数据结构
    let large_data = create_large_boxed_data();
    println!("大型数据的第一个元素: {}", large_data[0]);
}

/// 创建大型装箱数据
fn create_large_boxed_data() -> Box<Vec<i32>> {
    let large_vec = (0..1000).collect();
    Box::new(large_vec)
}

/// 通用的longest函数（用于多个演示）
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

/// 程序结束时的总结
#[allow(dead_code)]
fn lifetime_summary() {
    println!("\n=== Rust生命周期深度分析总结 ===");
    println!("\n本文档涵盖了以下主要内容:");
    println!("1. 生命周期基础概念 - 理解什么是生命周期及其重要性");
    println!("2. 生命周期语法 - 掌握生命周期参数和标注语法");
    println!("3. 生命周期规则 - 了解省略规则和子类型关系");
    println!("4. 结构体生命周期 - 在结构体和方法中使用生命周期");
    println!("5. 高级特性 - HRTB、生命周期边界、'static生命周期");
    println!("6. 常见模式 - 借用检查器、NLL、协变逆变");
    println!("7. 错误处理 - 识别和解决生命周期错误");
    println!("8. 实际应用 - 迭代器、闭包、异步、智能指针中的生命周期");
    println!("\n通过这些示例和解释，您应该对Rust的生命周期系统有了全面的理解。");
    println!("生命周期是Rust内存安全的核心机制，掌握它对于编写安全高效的Rust代码至关重要。");
}
