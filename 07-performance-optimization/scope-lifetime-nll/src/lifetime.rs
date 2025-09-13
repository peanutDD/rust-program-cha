//! # 生命周期 (Lifetime) 深度分析
//!
//! 生命周期是Rust中用于确保引用有效性的机制。它描述了引用在程序中保持有效的时间范围。
//! 生命周期与作用域密切相关，但更加精确和灵活。
//!
//! ## 生命周期的核心概念
//!
//! 1. **生命周期参数**: 用于标注引用的生命周期
//! 2. **生命周期省略规则**: 编译器自动推断生命周期的规则
//! 3. **静态生命周期**: 整个程序运行期间都有效的生命周期
//! 4. **生命周期子类型**: 生命周期之间的关系
//! 5. **高阶生命周期**: 更复杂的生命周期场景

use std::collections::HashMap;
use std::fmt::Display;

// 全局静态数据
static GLOBAL_DATA: &str = "全局静态数据";

/// 运行所有生命周期示例
pub fn run_lifetime_examples() {
    println!("\n=== 生命周期 (Lifetime) 深度分析 ===");
    
    basic_lifetime_concepts();
    lifetime_parameters_examples();
    lifetime_elision_rules();
    static_lifetime_examples();
    lifetime_subtyping_examples();
    struct_lifetime_examples();
    method_lifetime_examples();
    higher_ranked_lifetimes();
    lifetime_bounds_examples();
}

/// 1. 基本生命周期概念
fn basic_lifetime_concepts() {
    println!("\n⏰ 1. 基本生命周期概念");
    println!("生命周期确保引用在其指向的数据有效期间保持有效。");
    
    // 基本的生命周期关系
    {
        let _r: &i32;         // 声明引用r
        {
            let _x = 5;       // x的生命周期开始
            // r = &x;        // 错误！x的生命周期比r短
        }                     // x的生命周期结束
        // println!("{}", r); // 错误！r引用的数据已经无效
    }
    
    // 正确的生命周期关系
    {
        let x = 5;            // x的生命周期开始
        let r = &x;           // r借用x，r的生命周期不能超过x
        println!("有效的引用: {}", r);
    }                         // x和r的生命周期都在这里结束
    
    // 生命周期与作用域的关系
    demonstrate_lifetime_scope_relationship();
}

/// 演示生命周期与作用域的关系
fn demonstrate_lifetime_scope_relationship() {
    println!("\n🔗 生命周期与作用域的关系:");
    
    let string1 = String::from("长字符串");
    
    {
        let string2 = String::from("短字符串");
        let result = longest(&string1, &string2);
        println!("最长的字符串: {}", result);
    } // string2在这里被销毁，但result在使用时是安全的
    
    println!("string1仍然有效: {}", string1);
    
    println!("✅ 生命周期确保引用在使用时指向的数据仍然有效");
}

/// 比较两个字符串切片的长度，返回较长的那个
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// 2. 生命周期参数示例
fn lifetime_parameters_examples() {
    println!("\n🏷️  2. 生命周期参数");
    println!("生命周期参数用于明确指定引用之间的生命周期关系。");
    
    // 单个生命周期参数
    fn first_word<'a>(s: &'a str) -> &'a str {
        s.split_whitespace().next().unwrap_or("")
    }
    
    let sentence = "Hello Rust World";
    let word = first_word(&sentence);
    println!("第一个单词: {}", word);
    
    // 多个生命周期参数
    fn compare_and_return<'a, 'b>(x: &'a str, _y: &'b str, return_first: bool) -> &'a str 
    where 'b: 'a  // 'b的生命周期至少和'a一样长
    {
        if return_first {
            x
        } else {
            // 这里需要确保y的生命周期至少和返回值一样长
            x // 为了简化，总是返回x
        }
    }
    
    let str1 = "第一个字符串";
    let str2 = "第二个字符串";
    let result = compare_and_return(&str1, &str2, true);
    println!("比较结果: {}", result);
    
    // 复杂的生命周期关系
    demonstrate_complex_lifetime_relationships();
}

/// 演示复杂的生命周期关系
fn demonstrate_complex_lifetime_relationships() {
    println!("\n🔄 复杂的生命周期关系:");
    
    // 函数接受多个引用参数，返回其中一个
    fn choose_string<'a>(first: &'a str, second: &'a str, choose_first: bool) -> &'a str {
        if choose_first {
            first
        } else {
            second
        }
    }
    
    let string_a = String::from("字符串A");
    {
        let string_b = String::from("字符串B");
        let chosen = choose_string(&string_a, &string_b, false);
        println!("选择的字符串: {}", chosen);
    } // string_b在这里被销毁，但chosen的使用是安全的
    
    // 生命周期参数的约束
    fn process_data<'a, 'b>(data: &'a str, processor: &'b str) -> (&'a str, usize) 
    where 'a: 'b  // 'a的生命周期至少和'b一样长
    {
        (data, processor.len())
    }
    
    let data = "要处理的数据";
    let processor = "处理器";
    let (processed, len) = process_data(&data, &processor);
    println!("处理结果: {}, 处理器长度: {}", processed, len);
    
    println!("✅ 生命周期参数提供了精确的引用关系控制");
}

/// 3. 生命周期省略规则
fn lifetime_elision_rules() {
    println!("\n✂️  3. 生命周期省略规则");
    println!("编译器可以根据特定规则自动推断生命周期，无需显式标注。");
    
    // 规则1: 每个引用参数都有自己的生命周期参数
    fn rule1_example(s: &str) -> usize {
        // 等价于: fn rule1_example<'a>(s: &'a str) -> usize
        s.len()
    }
    
    let text = "测试文本";
    println!("规则1示例 - 文本长度: {}", rule1_example(&text));
    
    // 规则2: 如果只有一个输入生命周期参数，它被赋给所有输出生命周期参数
    fn rule2_example(s: &str) -> &str {
        // 等价于: fn rule2_example<'a>(s: &'a str) -> &'a str
        s.chars().next().map(|c| &s[..c.len_utf8()]).unwrap_or("")
    }
    
    let sample = "示例";
    println!("规则2示例 - 第一个字符: {}", rule2_example(&sample));
    
    // 规则3: 如果有多个输入生命周期参数，但其中一个是&self或&mut self，
    // 那么self的生命周期被赋给所有输出生命周期参数
    demonstrate_method_lifetime_elision();
    
    // 需要显式生命周期的情况
    demonstrate_explicit_lifetime_needed();
}

/// 演示方法中的生命周期省略
fn demonstrate_method_lifetime_elision() {
    println!("\n🔧 方法中的生命周期省略:");
    
    struct TextProcessor {
        prefix: String,
    }
    
    impl TextProcessor {
        // 规则3: self的生命周期被赋给返回值
        fn add_prefix(&self, text: &str) -> String {
            // 等价于: fn add_prefix<'a, 'b>(&'a self, text: &'b str) -> String
            format!("{}: {}", self.prefix, text)
        }
        
        // 返回引用的情况
        fn get_prefix(&self) -> &str {
            // 等价于: fn get_prefix<'a>(&'a self) -> &'a str
            &self.prefix
        }
    }
    
    let processor = TextProcessor {
        prefix: String::from("前缀"),
    };
    
    let result = processor.add_prefix("内容");
    println!("添加前缀后: {}", result);
    
    let prefix_ref = processor.get_prefix();
    println!("获取前缀: {}", prefix_ref);
    
    println!("✅ 方法中的生命周期省略简化了代码编写");
}

/// 演示需要显式生命周期的情况
fn demonstrate_explicit_lifetime_needed() {
    println!("\n❗ 需要显式生命周期的情况:");
    
    // 多个引用参数，返回引用，无法自动推断
    fn longest_explicit<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
    
    // 如果没有显式生命周期，以下代码不会编译：
    // fn longest_implicit(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() { x } else { y }
    // }
    
    let str1 = "短";
    let str2 = "更长的字符串";
    let result = longest_explicit(&str1, &str2);
    println!("最长字符串: {}", result);
    
    println!("✅ 复杂情况下需要显式指定生命周期参数");
}

/// 4. 静态生命周期示例
fn static_lifetime_examples() {
    println!("\n🌍 4. 静态生命周期 ('static)");
    println!("静态生命周期表示引用在整个程序运行期间都有效。");
    
    // 字符串字面量具有静态生命周期
    let static_str: &'static str = "这是一个静态字符串";
    println!("静态字符串: {}", static_str);
    
    // 使用全局静态变量
    println!("全局静态数据: {}", GLOBAL_DATA);
    
    // 常量也具有静态生命周期
    const CONSTANT_DATA: &str = "常量数据";
    println!("常量数据: {}", CONSTANT_DATA);
    
    // 函数返回静态引用
    fn get_static_str() -> &'static str {
        "返回的静态字符串"
    }
    
    let static_result = get_static_str();
    println!("函数返回的静态字符串: {}", static_result);
    
    // 静态生命周期的约束
    demonstrate_static_lifetime_constraints();
}

/// 演示静态生命周期的约束
fn demonstrate_static_lifetime_constraints() {
    println!("\n🔒 静态生命周期的约束:");
    
    // 接受静态引用的函数
    fn process_static_data(data: &'static str) -> String {
        format!("处理静态数据: {}", data)
    }
    
    let result1 = process_static_data("字面量字符串");
    println!("{}", result1);
    
    let result2 = process_static_data(GLOBAL_DATA);
    println!("{}", result2);
    
    // 以下代码不会编译，因为局部字符串没有静态生命周期：
    // let local_string = String::from("局部字符串");
    // let result3 = process_static_data(&local_string);
    
    // 泛型中的静态生命周期约束
    fn store_static_ref<T>(data: T) -> T 
    where 
        T: 'static,  // T必须具有静态生命周期
    {
        data
    }
    
    let stored = store_static_ref("静态字符串");
    println!("存储的静态引用: {}", stored);
    
    println!("✅ 静态生命周期提供了全局有效的引用保证");
}

/// 5. 生命周期子类型示例
fn lifetime_subtyping_examples() {
    println!("\n🔄 5. 生命周期子类型");
    println!("生命周期之间存在子类型关系：更长的生命周期是更短生命周期的子类型。");
    
    // 生命周期协变性
    fn demonstrate_lifetime_covariance() {
        let long_lived = String::from("长生命周期");
        
        {
            let short_lived = String::from("短生命周期");
            
            // 函数接受较短的生命周期
            fn use_shorter_lifetime<'a>(s: &'a str) -> &'a str {
                s
            }
            
            // 可以传入更长生命周期的引用
            let result = use_shorter_lifetime(&long_lived);
            println!("使用更长生命周期: {}", result);
            
            let result2 = use_shorter_lifetime(&short_lived);
            println!("使用匹配生命周期: {}", result2);
        }
    }
    
    demonstrate_lifetime_covariance();
    
    // 生命周期约束
    fn demonstrate_lifetime_bounds() {
        fn process_with_bounds<'a, 'b>(x: &'a str, y: &'b str) -> &'a str 
        where 
            'b: 'a,  // 'b必须至少和'a一样长
        {
            println!("处理数据: {} 和 {}", x, y);
            x
        }
        
        let data1 = String::from("数据1");
        let data2 = String::from("数据2");
        
        let result = process_with_bounds(&data1, &data2);
        println!("处理结果: {}", result);
    }
    
    demonstrate_lifetime_bounds();
    
    println!("✅ 生命周期子类型确保了引用关系的安全性");
}

/// 6. 结构体中的生命周期
fn struct_lifetime_examples() {
    println!("\n🏗️  6. 结构体中的生命周期");
    println!("结构体可以包含引用，需要生命周期参数来确保引用的有效性。");
    
    // 包含引用的结构体
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    let novel = String::from("很久很久以前，在一个遥远的星系...");
    let first_sentence = novel.split('.').next().expect("找不到句号");
    
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("重要摘录: {:?}", excerpt);
    
    // 多个生命周期参数的结构体
    #[derive(Debug)]
    struct DoubleRef<'a, 'b> {
        first: &'a str,
        second: &'b str,
    }
    
    let str1 = "第一个字符串";
    let str2 = "第二个字符串";
    
    let double_ref = DoubleRef {
        first: &str1,
        second: &str2,
    };
    
    println!("双重引用: {:?}", double_ref);
    
    // 结构体方法中的生命周期
    demonstrate_struct_methods_with_lifetimes();
}

/// 演示结构体方法中的生命周期
fn demonstrate_struct_methods_with_lifetimes() {
    println!("\n🔧 结构体方法中的生命周期:");
    
    struct TextHolder<'a> {
        text: &'a str,
    }
    
    impl<'a> TextHolder<'a> {
        // 方法返回结构体中的引用
        fn get_text(&self) -> &str {
            self.text
        }
        
        // 方法接受额外的引用参数
        fn compare_with<'b>(&self, other: &'b str) -> bool {
            self.text == other
        }
        
        // 方法返回两个引用中的一个
        fn choose<'b>(&'a self, _other: &'b str, choose_self: bool) -> &'a str 
        where 
            'b: 'a,
        {
            if choose_self {
                self.text
            } else {
                self.text // 为了满足返回类型，总是返回self.text
            }
        }
    }
    
    let content = "结构体中的文本";
    let holder = TextHolder { text: &content };
    
    println!("获取文本: {}", holder.get_text());
    println!("比较结果: {}", holder.compare_with("其他文本"));
    
    let other_text = "其他文本";
    let chosen = holder.choose(&other_text, true);
    println!("选择的文本: {}", chosen);
    
    println!("✅ 结构体方法可以灵活处理生命周期关系");
}

/// 7. 方法中的生命周期
fn method_lifetime_examples() {
    println!("\n🔧 7. 方法中的生命周期");
    println!("方法中的生命周期遵循特殊的省略规则。");
    
    struct StringProcessor {
        buffer: String,
    }
    
    impl StringProcessor {
        fn new(initial: &str) -> Self {
            StringProcessor {
                buffer: initial.to_string(),
            }
        }
        
        // 返回对内部数据的引用
        fn get_buffer(&self) -> &str {
            &self.buffer
        }
        
        // 修改内部数据并返回引用
        fn append_and_get(&mut self, suffix: &str) -> &str {
            self.buffer.push_str(suffix);
            &self.buffer
        }
        
        // 比较内部数据和外部数据
        fn compare_with(&self, other: &str) -> bool {
            self.buffer == other
        }
        
        // 复杂的方法：接受多个引用，返回引用
        fn process_and_choose<'a>(&'a self, input: &'a str, choose_input: bool) -> &'a str {
            if choose_input {
                input
            } else {
                &self.buffer
            }
        }
    }
    
    let mut processor = StringProcessor::new("初始内容");
    
    println!("初始缓冲区: {}", processor.get_buffer());
    
    let appended = processor.append_and_get(" - 追加内容");
    println!("追加后: {}", appended);
    
    let comparison = processor.compare_with("其他内容");
    println!("比较结果: {}", comparison);
    
    let input = "输入数据";
    let chosen = processor.process_and_choose(&input, false);
    println!("选择的数据: {}", chosen);
    
    println!("✅ 方法中的生命周期提供了灵活的数据访问模式");
}

/// 8. 高阶生命周期
fn higher_ranked_lifetimes() {
    println!("\n🚀 8. 高阶生命周期 (Higher-Ranked Trait Bounds)");
    println!("高阶生命周期允许在泛型中使用更复杂的生命周期约束。");
    
    // 高阶生命周期的函数指针
    fn apply_to_string<F>(f: F, s: &str) -> String 
    where 
        F: for<'a> Fn(&'a str) -> &'a str,
    {
        f(s).to_string()
    }
    
    // 符合高阶生命周期约束的函数
    fn first_char(s: &str) -> &str {
        s.chars().next().map(|c| &s[0..c.len_utf8()]).unwrap_or("")
    }
    
    let text = "测试文本";
    let result = apply_to_string(first_char, &text);
    println!("应用函数结果: {}", result);
    
    // 使用闭包
    let result2 = apply_to_string(|s| s.chars().last().map(|c| &s[s.len()-c.len_utf8()..]).unwrap_or(""), &text);
    println!("闭包结果: {}", result2);
    
    // 高阶生命周期在trait中的应用
    demonstrate_hrtb_in_traits();
}

/// 演示高阶生命周期在trait中的应用
fn demonstrate_hrtb_in_traits() {
    println!("\n🎯 高阶生命周期在trait中的应用:");
    
    trait StringProcessor {
        fn process(&self, input: &str) -> String;
    }
    
    struct UpperCaseProcessor;
    
    impl StringProcessor for UpperCaseProcessor {
        fn process(&self, input: &str) -> String {
            input.to_uppercase()
        }
    }
    
    // 使用高阶生命周期约束的泛型函数
    fn process_multiple<P>(processor: &P, inputs: &[&str]) -> Vec<String> 
    where 
        P: StringProcessor,
    {
        inputs.iter().map(|&s| processor.process(s)).collect()
    }
    
    let processor = UpperCaseProcessor;
    let inputs = vec!["hello", "world", "rust"];
    let results = process_multiple(&processor, &inputs);
    
    println!("处理结果: {:?}", results);
    
    println!("✅ 高阶生命周期提供了更强大的抽象能力");
}

/// 9. 生命周期约束示例
fn lifetime_bounds_examples() {
    println!("\n🔗 9. 生命周期约束");
    println!("生命周期约束用于指定生命周期之间的关系。");
    
    // 基本生命周期约束
    fn with_lifetime_bound<'a, 'b>(x: &'a str, y: &'b str) -> &'a str 
    where 
        'b: 'a,  // 'b必须至少和'a一样长
    {
        println!("处理: {} 和 {}", x, y);
        x
    }
    
    let long_str = "长生命周期字符串";
    let short_str = "短字符串";
    
    let result = with_lifetime_bound(&short_str, &long_str);
    println!("约束结果: {}", result);
    
    // 结构体中的生命周期约束
    struct BoundedStruct<'a, 'b> 
    where 
        'b: 'a,
    {
        short_ref: &'a str,
        long_ref: &'b str,
    }
    
    let bounded = BoundedStruct {
        short_ref: &short_str,
        long_ref: &long_str,
    };
    
    println!("约束结构体: {} - {}", bounded.short_ref, bounded.long_ref);
    
    // 泛型类型的生命周期约束
    demonstrate_generic_lifetime_bounds();
}

/// 演示泛型类型的生命周期约束
fn demonstrate_generic_lifetime_bounds() {
    println!("\n🎯 泛型类型的生命周期约束:");
    
    // 泛型类型必须满足特定的生命周期约束
    fn process_displayable<T>(item: T) -> String 
    where 
        T: Display + 'static,  // T必须实现Display且具有静态生命周期
    {
        format!("处理: {}", item)
    }
    
    let number = 42;
    let result1 = process_displayable(number);
    println!("{}", result1);
    
    let static_str = "静态字符串";
    let result2 = process_displayable(static_str);
    println!("{}", result2);
    
    // 以下代码不会编译，因为String引用没有静态生命周期：
    // let owned_string = String::from("拥有的字符串");
    // let result3 = process_displayable(&owned_string);
    
    // 更复杂的约束组合
    fn complex_bounds<'a, T>(data: &'a T) -> String 
    where 
        T: Display + Clone + 'a,
    {
        let cloned = data.clone();
        format!("复杂处理: {}", cloned)
    }
    
    let data = "测试数据";
    let result3 = complex_bounds(&data);
    println!("{}", result3);
    
    println!("✅ 生命周期约束确保了类型安全和内存安全");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifetime_examples() {
        // 测试所有生命周期示例是否能正常运行
        run_lifetime_examples();
    }

    #[test]
    fn test_longest_function() {
        let str1 = "short";
        let str2 = "longer string";
        let result = longest(&str1, &str2);
        assert_eq!(result, "longer string");
    }

    #[test]
    fn test_struct_with_lifetime() {
        let text = "test text";
        let excerpt = ImportantExcerpt { part: &text };
        assert_eq!(excerpt.part, "test text");
    }
    
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
}