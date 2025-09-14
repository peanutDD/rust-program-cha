//! # 生命周期省略规则（Lifetime Elision Rules）
//!
//! 本模块深入探讨 Rust 的生命周期省略规则，包括：
//! - 三大生命周期省略规则的详细分析
//! - 各种应用场景和实例
//! - 编译器如何推断生命周期
//! - 何时需要显式标注生命周期

use std::collections::HashMap;
use std::fmt::Display;

/// # 生命周期省略规则概述
///
/// Rust 编译器使用三个规则来推断生命周期，这些规则被称为生命周期省略规则。
/// 如果编译器应用这三个规则后仍然无法确定引用的生命周期，就会要求显式标注。
pub mod elision_rules_overview {
    /// 演示生命周期省略规则的概述
    pub fn demonstrate_elision_overview() {
        println!("=== 生命周期省略规则概述 ===");
        println!("Rust 编译器使用以下三个规则来推断生命周期：");
        println!("1. 每个引用参数都有自己的生命周期参数");
        println!("2. 如果只有一个输入生命周期参数，该生命周期被赋予所有输出生命周期参数");
        println!(
            "3. 如果有多个输入生命周期参数，但其中一个是 &self 或 &mut self，self 的生命周期被赋予所有输出生命周期参数"
        );
        println!();

        // 演示编译器如何应用这些规则
        demonstrate_compiler_inference();
    }

    /// 演示编译器如何推断生命周期
    fn demonstrate_compiler_inference() {
        println!("--- 编译器生命周期推断过程 ---");

        // 示例1：单个参数的函数
        let s = "hello world";
        let first = first_word(s);
        println!("第一个单词: {}", first);

        // 示例2：多个参数的函数（需要显式标注）
        let s1 = "hello";
        let s2 = "world";
        let longer = longest_explicit(s1, s2);
        println!("较长的字符串: {}", longer);
    }

    /// 规则2的应用：单个输入参数
    /// 编译器推断：fn first_word<'a>(s: &'a str) -> &'a str
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    /// 需要显式标注的情况：多个输入参数且没有 self
    fn longest_explicit<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
}

/// # 规则一：每个引用参数都有自己的生命周期参数
///
/// 编译器为每个引用参数分配一个独立的生命周期参数
pub mod rule_one {
    /// 演示规则一的应用
    pub fn demonstrate_rule_one() {
        println!("\n=== 规则一：每个引用参数都有自己的生命周期参数 ===");

        // 演示不同数量参数的函数
        demonstrate_parameter_variations();

        // 演示编译器的推断过程
        demonstrate_inference_process();
    }

    /// 演示不同数量参数的函数
    fn demonstrate_parameter_variations() {
        println!("\n--- 不同参数数量的函数 ---");

        let s = "hello";
        let result1 = single_param(s);
        println!("单参数函数结果: {}", result1);

        let s1 = "hello";
        let s2 = "world";
        let result2 = double_param(s1, s2);
        println!("双参数函数结果: {}", result2);

        let s3 = "rust";
        let result3 = triple_param(s1, s2, s3);
        println!("三参数函数结果: {}", result3);
    }

    /// 单个引用参数
    /// 编译器推断：fn single_param<'a>(s: &'a str) -> &'a str
    fn single_param(s: &str) -> &str {
        s
    }

    /// 两个引用参数
    /// 编译器推断：fn double_param<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str
    fn double_param<'a>(s1: &'a str, _s2: &str) -> &'a str {
        s1 // 只返回第一个参数，所以返回值生命周期与 s1 相同
    }

    /// 三个引用参数
    /// 编译器推断：fn triple_param<'a, 'b, 'c>(s1: &'a str, s2: &'b str, s3: &'c str) -> &'a str
    fn triple_param<'a>(s1: &'a str, _s2: &str, _s3: &str) -> &'a str {
        s1 // 只返回第一个参数
    }

    /// 演示编译器的推断过程
    fn demonstrate_inference_process() {
        println!("\n--- 编译器推断过程演示 ---");

        println!("原始函数签名: fn example(s1: &str, s2: &str) -> &str");
        println!("应用规则一后: fn example<'a, 'b>(s1: &'a str, s2: &'b str) -> &str");
        println!("由于无法确定返回值的生命周期，需要显式标注");

        // 演示需要显式标注的情况
        let s1 = "hello";
        let s2 = "world";
        let result = example_explicit(s1, s2);
        println!("显式标注的函数结果: {}", result);
    }

    /// 需要显式标注生命周期的函数
    fn example_explicit<'a>(s1: &'a str, _s2: &str) -> &'a str {
        s1
    }
}

/// # 规则二：单个输入生命周期参数传播到输出
///
/// 如果只有一个输入生命周期参数，该生命周期被赋予所有输出生命周期参数
pub mod rule_two {
    use std::collections::HashMap;

    /// 演示规则二的应用
    pub fn demonstrate_rule_two() {
        println!("\n=== 规则二：单个输入生命周期参数传播到输出 ===");

        // 演示基本应用
        demonstrate_basic_application();

        // 演示复杂数据结构
        demonstrate_complex_structures();

        // 演示多个输出的情况
        demonstrate_multiple_outputs();
    }

    /// 演示基本应用
    fn demonstrate_basic_application() {
        println!("\n--- 规则二基本应用 ---");

        let text = "hello world rust programming";

        // 各种单参数函数的应用
        let first = get_first_word(text);
        println!("第一个单词: {}", first);

        let last = get_last_word(text);
        println!("最后一个单词: {}", last);

        let slice = get_slice(text, 6, 11);
        println!("切片: {}", slice);

        let trimmed = trim_whitespace("  hello world  ");
        println!("去除空白: '{}'", trimmed);
    }

    /// 获取第一个单词
    /// 编译器推断：fn get_first_word<'a>(text: &'a str) -> &'a str
    pub fn get_first_word(text: &str) -> &str {
        text.split_whitespace().next().unwrap_or("")
    }

    /// 获取最后一个单词
    /// 编译器推断：fn get_last_word<'a>(text: &'a str) -> &'a str
    fn get_last_word(text: &str) -> &str {
        text.split_whitespace().last().unwrap_or("")
    }

    /// 获取字符串切片
    /// 编译器推断：fn get_slice<'a>(text: &'a str, start: usize, end: usize) -> &'a str
    fn get_slice(text: &str, start: usize, end: usize) -> &str {
        &text[start..end]
    }

    /// 去除首尾空白
    /// 编译器推断：fn trim_whitespace<'a>(text: &'a str) -> &'a str
    fn trim_whitespace(text: &str) -> &str {
        text.trim()
    }

    /// 演示复杂数据结构
    fn demonstrate_complex_structures() {
        println!("\n--- 复杂数据结构中的规则二 ---");

        let data = vec![1, 2, 3, 4, 5];

        // 处理切片
        let first_half = get_first_half(&data);
        println!("前半部分: {:?}", first_half);

        let max_element = find_max(&data);
        println!("最大元素: {:?}", max_element);

        // 处理哈希表
        let mut map = HashMap::new();
        map.insert("key1", "value1");
        map.insert("key2", "value2");

        if let Some(value) = get_value(&map, "key1") {
            println!("找到的值: {}", value);
        }
    }

    /// 获取切片的前半部分
    /// 编译器推断：fn get_first_half<'a, T>(slice: &'a [T]) -> &'a [T]
    fn get_first_half<T>(slice: &[T]) -> &[T] {
        let mid = slice.len() / 2;
        &slice[..mid]
    }

    /// 查找最大元素
    /// 编译器推断：fn find_max<'a, T>(slice: &'a [T]) -> Option<&'a T>
    fn find_max<T: Ord>(slice: &[T]) -> Option<&T> {
        slice.iter().max()
    }

    /// 从哈希表获取值
    /// 编译器推断：fn get_value<'a>(map: &'a HashMap<&str, &str>, key: &str) -> Option<&'a str>
    fn get_value<'a>(map: &'a HashMap<&str, &str>, key: &str) -> Option<&'a str> {
        map.get(key).copied()
    }

    /// 演示多个输出的情况
    fn demonstrate_multiple_outputs() {
        println!("\n--- 多个输出的情况 ---");

        let text = "hello,world,rust";
        let (first, rest) = split_first(text);
        println!("第一部分: {}, 其余部分: {}", first, rest);

        let numbers = vec![1, 2, 3, 4, 5];
        let (min, max) = find_min_max(&numbers);
        println!("最小值: {:?}, 最大值: {:?}", min, max);
    }

    /// 分割字符串，返回第一部分和其余部分
    /// 编译器推断：fn split_first<'a>(text: &'a str) -> (&'a str, &'a str)
    fn split_first(text: &str) -> (&str, &str) {
        if let Some(pos) = text.find(',') {
            (&text[..pos], &text[pos + 1..])
        } else {
            (text, "")
        }
    }

    /// 查找最小值和最大值
    /// 编译器推断：fn find_min_max<'a, T>(slice: &'a [T]) -> (Option<&'a T>, Option<&'a T>)
    fn find_min_max<T: Ord>(slice: &[T]) -> (Option<&T>, Option<&T>) {
        (slice.iter().min(), slice.iter().max())
    }
}

/// # 规则三：self 的生命周期传播到输出
///
/// 如果有多个输入生命周期参数，但其中一个是 &self 或 &mut self，
/// self 的生命周期被赋予所有输出生命周期参数
pub mod rule_three {
    use std::collections::HashMap;

    /// 演示规则三的应用
    pub fn demonstrate_rule_three() {
        println!("\n=== 规则三：self 的生命周期传播到输出 ===");

        // 演示基本结构体方法
        demonstrate_basic_struct_methods();

        // 演示复杂的方法签名
        demonstrate_complex_method_signatures();

        // 演示可变借用的情况
        demonstrate_mutable_borrowing();
    }

    /// 演示基本结构体方法
    fn demonstrate_basic_struct_methods() {
        println!("\n--- 基本结构体方法 ---");

        let text = String::from("Hello, Rust programming!");
        let parser = TextParser::new(&text);

        // 各种方法的应用
        let first_word = parser.get_first_word();
        println!("第一个单词: {}", first_word);

        let word_at = parser.get_word_at(2);
        println!("第三个单词: {:?}", word_at);

        let contains = parser.contains_word("Rust");
        println!("包含 'Rust': {}", contains);

        let prefix = parser.get_prefix(5);
        println!("前5个字符: {}", prefix);
    }

    /// 文本解析器结构体
    pub struct TextParser<'a> {
        text: &'a str,
    }

    impl<'a> TextParser<'a> {
        /// 创建新的解析器
        pub fn new(text: &'a str) -> Self {
            TextParser { text }
        }

        /// 获取第一个单词
        /// 编译器推断：fn get_first_word(&'a self) -> &'a str
        pub fn get_first_word(&self) -> &str {
            self.text.split_whitespace().next().unwrap_or("")
        }

        /// 获取指定位置的单词
        /// 编译器推断：fn get_word_at(&'a self, index: usize) -> Option<&'a str>
        fn get_word_at(&self, index: usize) -> Option<&str> {
            self.text.split_whitespace().nth(index)
        }

        /// 检查是否包含指定单词
        /// 编译器推断：fn contains_word(&'a self, word: &str) -> bool
        /// 注意：这里 word 参数的生命周期与返回值无关
        fn contains_word(&self, word: &str) -> bool {
            self.text.contains(word)
        }

        /// 获取前缀
        /// 编译器推断：fn get_prefix(&'a self, len: usize) -> &'a str
        fn get_prefix(&self, len: usize) -> &str {
            if len >= self.text.len() {
                self.text
            } else {
                &self.text[..len]
            }
        }

        /// 比较并返回较长的字符串
        /// 编译器推断：fn compare_and_return(&'a self, other: &str) -> &'a str
        /// 注意：返回值的生命周期与 self 相同，而不是与 other 相同
        fn compare_and_return(&self, other: &str) -> &str {
            if self.text.len() > other.len() {
                self.text
            } else {
                // 这里不能返回 other，因为它的生命周期可能比 self 短
                self.text
            }
        }
    }

    /// 演示复杂的方法签名
    fn demonstrate_complex_method_signatures() {
        println!("\n--- 复杂方法签名 ---");

        let data = vec!["apple", "banana", "cherry", "date"];
        let container = DataContainer::new(&data);

        // 演示各种复杂方法
        let filtered = container.filter_by_length(5);
        println!("长度大于5的项: {:?}", filtered);

        let found = container.find_by_prefix("ch");
        println!("以'ch'开头的项: {:?}", found);

        let combined = container.combine_with_separator(", ");
        println!("组合结果: {}", combined);
    }

    /// 数据容器结构体
    struct DataContainer<'a> {
        data: &'a [&'a str],
    }

    impl<'a> DataContainer<'a> {
        /// 创建新容器
        fn new(data: &'a [&'a str]) -> Self {
            DataContainer { data }
        }

        /// 按长度过滤
        /// 编译器推断：fn filter_by_length(&'a self, min_len: usize) -> Vec<&'a str>
        fn filter_by_length(&self, min_len: usize) -> Vec<&str> {
            self.data
                .iter()
                .filter(|&&item| item.len() > min_len)
                .copied()
                .collect()
        }

        /// 按前缀查找
        /// 编译器推断：fn find_by_prefix(&'a self, prefix: &str) -> Option<&'a str>
        fn find_by_prefix(&self, prefix: &str) -> Option<&str> {
            self.data
                .iter()
                .find(|&&item| item.starts_with(prefix))
                .copied()
        }

        /// 使用分隔符组合
        /// 编译器推断：fn combine_with_separator(&'a self, sep: &str) -> String
        /// 注意：返回 String（拥有所有权），所以不涉及生命周期
        fn combine_with_separator(&self, sep: &str) -> String {
            self.data.join(sep)
        }
    }

    /// 演示可变借用的情况
    fn demonstrate_mutable_borrowing() {
        println!("\n--- 可变借用情况 ---");

        let mut text = String::from("hello world");
        let mut editor = TextEditor::new(&mut text);

        // 演示可变方法
        editor.append(" rust");
        editor.prepend("Welcome: ");
        editor.replace_word("world", "Rust");

        let result = editor.get_content();
        println!("编辑后的内容: {}", result);
    }

    /// 文本编辑器结构体
    struct TextEditor<'a> {
        content: &'a mut String,
    }

    impl<'a> TextEditor<'a> {
        /// 创建新编辑器
        fn new(content: &'a mut String) -> Self {
            TextEditor { content }
        }

        /// 追加文本
        /// 编译器推断：fn append(&'a mut self, text: &str)
        fn append(&mut self, text: &str) {
            self.content.push_str(text);
        }

        /// 前置文本
        /// 编译器推断：fn prepend(&'a mut self, text: &str)
        fn prepend(&mut self, text: &str) {
            *self.content = format!("{}{}", text, self.content);
        }

        /// 替换单词
        /// 编译器推断：fn replace_word(&'a mut self, from: &str, to: &str)
        fn replace_word(&mut self, from: &str, to: &str) {
            *self.content = self.content.replace(from, to);
        }

        /// 获取内容
        /// 编译器推断：fn get_content(&'a self) -> &'a str
        fn get_content(&self) -> &str {
            self.content
        }
    }
}

/// # 省略规则的边界情况
///
/// 探讨生命周期省略规则无法应用的情况，需要显式标注的场景
pub mod elision_boundaries {
    /// 演示省略规则的边界情况
    pub fn demonstrate_elision_boundaries() {
        println!("\n=== 省略规则的边界情况 ===");

        // 演示需要显式标注的情况
        demonstrate_explicit_annotation_needed();

        // 演示复杂的生命周期关系
        demonstrate_complex_lifetime_relationships();

        // 演示高阶函数的情况
        demonstrate_higher_order_functions();
    }

    /// 演示需要显式标注的情况
    fn demonstrate_explicit_annotation_needed() {
        println!("\n--- 需要显式标注的情况 ---");

        let s1 = "hello";
        let s2 = "world";

        // 多个输入参数，没有 self，需要显式标注
        let longer = longest(s1, s2);
        println!("较长的字符串: {}", longer);

        // 复杂的返回类型
        let (first, second) = split_and_return(s1, s2);
        println!("分割结果: {}, {}", first, second);

        // 条件返回不同参数
        let result = conditional_return(s1, s2, true);
        println!("条件返回: {}", result);
    }

    /// 需要显式标注：多个输入参数
    pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    /// 需要显式标注：复杂返回类型
    fn split_and_return<'a, 'b>(x: &'a str, y: &'b str) -> (&'a str, &'b str) {
        (x, y)
    }

    /// 需要显式标注：条件返回不同参数
    fn conditional_return<'a>(x: &'a str, y: &'a str, condition: bool) -> &'a str {
        if condition { x } else { y }
    }

    /// 演示复杂的生命周期关系
    fn demonstrate_complex_lifetime_relationships() {
        println!("\n--- 复杂生命周期关系 ---");

        let data1 = String::from("first data");
        let data2 = String::from("second data");

        let processor = DataProcessor::new(&data1, &data2);
        let result = processor.process_data();
        println!("处理结果: {}", result);

        // 演示生命周期约束
        let constrained = processor.get_constrained_data();
        println!("约束数据: {}", constrained);
    }

    /// 复杂生命周期关系的结构体
    struct DataProcessor<'a, 'b> {
        primary: &'a str,
        secondary: &'b str,
    }

    impl<'a, 'b> DataProcessor<'a, 'b> {
        /// 创建处理器
        fn new(primary: &'a str, secondary: &'b str) -> Self {
            DataProcessor { primary, secondary }
        }

        /// 处理数据 - 需要显式标注
        fn process_data(&self) -> &'a str {
            // 总是返回主数据
            self.primary
        }

        /// 获取约束数据 - 复杂的生命周期约束
        fn get_constrained_data(&self) -> &str
        where
            'a: 'b, // 'a 必须比 'b 活得更久
        {
            if self.primary.len() > self.secondary.len() {
                self.primary
            } else {
                self.secondary
            }
        }
    }

    /// 演示高阶函数的情况
    fn demonstrate_higher_order_functions() {
        println!("\n--- 高阶函数情况 ---");

        let data = vec!["apple", "banana", "cherry"];

        // 使用闭包处理数据
        let result = process_with_closure(&data, |s| s.len());
        println!("闭包处理结果: {:?}", result);

        // 使用函数指针
        let result2 = process_with_function(&data, str::len);
        println!("函数指针处理结果: {:?}", result2);
    }

    /// 使用闭包的高阶函数
    fn process_with_closure<F>(data: &[&str], f: F) -> Vec<usize>
    where
        F: Fn(&str) -> usize,
    {
        data.iter().map(|&s| f(s)).collect()
    }

    /// 使用函数指针的高阶函数
    fn process_with_function(data: &[&str], f: fn(&str) -> usize) -> Vec<usize> {
        data.iter().map(|&s| f(s)).collect()
    }
}

/// 运行所有生命周期省略规则的演示
pub fn run_all_demonstrations() {
    run_all_elision_examples();
}

/// 运行所有生命周期省略规则的演示（别名函数）
pub fn run_all_elision_examples() {
    println!("🦀 Rust 深入生命周期 - 省略规则篇 🦀");
    println!("===========================================");

    // 1. 省略规则概述
    elision_rules_overview::demonstrate_elision_overview();

    // 2. 规则一演示
    rule_one::demonstrate_rule_one();

    // 3. 规则二演示
    rule_two::demonstrate_rule_two();

    // 4. 规则三演示
    rule_three::demonstrate_rule_three();

    // 5. 边界情况演示
    elision_boundaries::demonstrate_elision_boundaries();

    println!("\n===========================================");
    println!("✅ 生命周期省略规则演示完成！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_two_application() {
        let text = "hello world";
        let first = rule_two::get_first_word(text);
        assert_eq!(first, "hello");
    }

    #[test]
    fn test_rule_three_application() {
        let text = String::from("test string");
        let parser = rule_three::TextParser::new(&text);
        let first_word = parser.get_first_word();
        assert_eq!(first_word, "test");
    }

    #[test]
    fn test_explicit_annotation() {
        let s1 = "hello";
        let s2 = "world";
        let result = elision_boundaries::longest(s1, s2);
        assert_eq!(result, "world"); // "world" 比 "hello" 长，所以返回 "world"
    }
}
