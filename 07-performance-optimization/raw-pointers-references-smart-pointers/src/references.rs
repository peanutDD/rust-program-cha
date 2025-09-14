//! # 引用 (References) 深度解析
//!
//! 引用是 Rust 中最常用的指针类型，提供了安全的内存访问机制。
//! 本模块深入探讨引用的借用规则、生命周期和各种使用模式。
//!
//! ## 引用类型
//!
//! - `&T`: 不可变引用（共享引用）
//! - `&mut T`: 可变引用（独占引用）
//!
//! ## 核心特性
//!
//! 1. **借用检查**: 编译时确保内存安全
//! 2. **生命周期**: 确保引用的有效性
//! 3. **借用规则**: 防止数据竞争和悬垂引用
//! 4. **零成本抽象**: 运行时无额外开销
//! 5. **自动解引用**: 方法调用时自动解引用
//! 6. **不能为空**: 引用总是指向有效内存

use std::collections::HashMap;
use std::fmt::Display;

/// 运行所有引用示例
pub fn run_reference_examples() {
    println!("🔗 引用深度解析");
    println!("{}", "=".repeat(50));

    basic_reference_usage();
    borrowing_rules();
    lifetime_basics();
    lifetime_annotations();
    lifetime_elision();
    reference_patterns();
    method_syntax_and_deref();
    reference_coercion();
    interior_mutability_preview();
    advanced_lifetime_patterns();
}

/// 基础引用使用
fn basic_reference_usage() {
    println!("\n📌 1. 基础引用使用");
    println!("{}", "-".repeat(30));

    // 不可变引用
    let x = 42;
    let x_ref = &x;
    println!("原始值: x = {}", x);
    println!("引用值: *x_ref = {}", *x_ref);
    println!("引用地址: x_ref = {:p}", x_ref);
    println!("原始地址: &x = {:p}", &x);

    // 可变引用
    let mut y = 100;
    let y_ref = &mut y;
    println!("\n可变引用:");
    println!("修改前: *y_ref = {}", *y_ref);
    *y_ref = 200;
    println!("修改后: *y_ref = {}", *y_ref);
    // 可变引用使用完毕后，可以再次访问原变量
    println!("原变量 y = {}", y);

    // 引用的引用
    let z = 42;
    let z_ref = &z;
    let z_ref_ref = &z_ref;
    println!("\n引用的引用:");
    println!("z = {}", z);
    println!("*z_ref = {}", *z_ref);
    println!("**z_ref_ref = {}", **z_ref_ref);
    println!("地址链: z({:p}) -> z_ref({:p}) -> z_ref_ref({:p})", &z, z_ref, z_ref_ref);

    // 引用与所有权
    demonstrate_reference_ownership();
}

/// 演示引用与所有权的关系
fn demonstrate_reference_ownership() {
    println!("\n🏠 引用与所有权:");
    
    let s1 = String::from("Hello");
    let s1_ref = &s1;  // 借用，不转移所有权
    
    println!("原始字符串: {}", s1);     // s1 仍然有效
    println!("引用字符串: {}", s1_ref); // 通过引用访问
    println!("再次访问: {}", s1);       // s1 仍然有效
    
    // 函数调用中的借用
    fn print_string(s: &String) {
        println!("函数中的字符串: {}", s);
    }
    
    print_string(&s1);  // 传递引用
    println!("函数调用后: {}", s1);  // s1 仍然有效
    
    // 对比：如果传递所有权
    fn take_ownership(s: String) {
        println!("获得所有权: {}", s);
    } // s 在这里被销毁
    
    let s2 = String::from("World");
    take_ownership(s2);  // 转移所有权
    // println!("所有权转移后: {}", s2);  // 错误！s2 已被移动
    println!("✅ 引用允许在不转移所有权的情况下使用值");
}

/// 借用规则详解
fn borrowing_rules() {
    println!("\n📌 2. 借用规则详解");
    println!("{}", "-".repeat(30));

    // 规则1: 多个不可变引用
    demonstrate_multiple_immutable_references();
    
    // 规则2: 可变引用的独占性
    demonstrate_mutable_reference_exclusivity();
    
    // 规则3: 不可变和可变引用不能共存
    demonstrate_mixed_reference_rules();
    
    // 规则4: 引用的作用域
    demonstrate_reference_scope();
}

/// 演示多个不可变引用
fn demonstrate_multiple_immutable_references() {
    println!("\n📖 规则1: 多个不可变引用");
    
    let data = vec![1, 2, 3, 4, 5];
    let ref1 = &data;
    let ref2 = &data;
    let ref3 = &data;
    
    println!("原始数据: {:?}", data);
    println!("引用1: {:?}", ref1);
    println!("引用2: {:?}", ref2);
    println!("引用3: {:?}", ref3);
    println!("✅ 可以同时存在多个不可变引用");
}

/// 演示可变引用的独占性
fn demonstrate_mutable_reference_exclusivity() {
    println!("\n✏️ 规则2: 可变引用的独占性");
    
    let mut data = vec![1, 2, 3];
    
    {
        let mut_ref = &mut data;
        mut_ref.push(4);
        println!("通过可变引用修改: {:?}", mut_ref);
        
        // 在可变引用存在期间，不能创建其他引用
        // let another_ref = &data;     // 错误！
        // let another_mut = &mut data; // 错误！
    } // mut_ref 在这里结束
    
    println!("可变引用结束后: {:?}", data);
    println!("✅ 可变引用具有独占性");
}

/// 演示混合引用规则
fn demonstrate_mixed_reference_rules() {
    println!("\n🔄 规则3: 不可变和可变引用不能共存");
    
    let mut data = String::from("Hello");
    
    // 正确的使用模式
    {
        let immut_ref1 = &data;
        let immut_ref2 = &data;
        println!("不可变引用: {}, {}", immut_ref1, immut_ref2);
    } // 不可变引用在这里结束
    
    {
        let mut_ref = &mut data;
        mut_ref.push_str(", World!");
        println!("可变引用: {}", mut_ref);
    } // 可变引用在这里结束
    
    println!("最终结果: {}", data);
    println!("✅ 不可变和可变引用分开使用");
}

/// 演示引用作用域
fn demonstrate_reference_scope() {
    println!("\n🎯 规则4: 引用的作用域");
    
    let mut data = vec![1, 2, 3];
    
    let immut_ref = &data;
    println!("不可变引用: {:?}", immut_ref);
    
    // 从 Rust 2018 开始，引用的生命周期更智能
    // 不可变引用在最后一次使用后就结束了
    
    let mut_ref = &mut data;  // 现在可以创建可变引用
    mut_ref.push(4);
    println!("可变引用: {:?}", mut_ref);
    
    // 演示非词法生命周期 (NLL)
    demonstrate_nll();
}

/// 演示非词法生命周期 (Non-Lexical Lifetimes)
fn demonstrate_nll() {
    println!("\n🧠 非词法生命周期 (NLL):");
    
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    
    // 在旧版本的 Rust 中，这会编译失败
    // 但在新版本中，由于 NLL，这是可以的
    let value = map.get("key1").unwrap_or(&"default");
    println!("获取的值: {}", value);
    
    // value 的使用结束后，可以获取可变引用
    map.insert("key2", "value2");
    println!("插入后的 map: {:?}", map);
    
    println!("✅ NLL 使借用检查更智能");
}

/// 生命周期基础
fn lifetime_basics() {
    println!("\n📌 3. 生命周期基础");
    println!("{}", "-".repeat(30));

    // 生命周期的概念
    demonstrate_lifetime_concept();
    
    // 悬垂引用预防
    demonstrate_dangling_reference_prevention();
    
    // 生命周期与作用域
    demonstrate_lifetime_vs_scope();
}

/// 演示生命周期概念
fn demonstrate_lifetime_concept() {
    println!("\n⏰ 生命周期概念:");
    
    // 生命周期确保引用的有效性
    let string1 = String::from("Hello");
    
    {
        let string2 = String::from("World");
        let result = longest_string(&string1, &string2);
        println!("较长的字符串: {}", result);
    } // string2 在这里被销毁
    
    // result 不能在这里使用，因为它可能引用已销毁的 string2
    println!("string1 仍然有效: {}", string1);
}

/// 返回较长字符串的引用
fn longest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

/// 演示悬垂引用预防
fn demonstrate_dangling_reference_prevention() {
    println!("\n🚫 悬垂引用预防:");
    
    // 这个函数会导致编译错误
    /*
    fn create_dangling_reference() -> &str {
        let s = String::from("Hello");
        &s  // 错误！返回对局部变量的引用
    } // s 在这里被销毁
    */
    
    // 正确的做法：返回拥有所有权的值
    fn create_owned_string() -> String {
        String::from("Hello")
    }
    
    let owned = create_owned_string();
    println!("拥有所有权的字符串: {}", owned);
    println!("✅ Rust 防止悬垂引用的产生");
}

/// 演示生命周期与作用域的关系
fn demonstrate_lifetime_vs_scope() {
    println!("\n🎭 生命周期 vs 作用域:");
    
    let outer_var = String::from("外部变量");
    
    {
        let inner_var = String::from("内部变量");
        let outer_ref = &outer_var;  // 引用外部变量
        let inner_ref = &inner_var;  // 引用内部变量
        
        println!("内部作用域: {} | {}", outer_ref, inner_ref);
        
        // outer_ref 的生命周期可以延续到外部作用域
        // inner_ref 的生命周期仅限于内部作用域
    }
    
    // inner_ref 在这里不可用
    // outer_ref 也不在这个作用域中定义
    println!("外部作用域: {}", outer_var);
    println!("✅ 生命周期确保引用在有效期内使用");
}

/// 生命周期注解
fn lifetime_annotations() {
    println!("\n📌 4. 生命周期注解");
    println!("{}", "-".repeat(30));

    // 函数中的生命周期注解
    demonstrate_function_lifetime_annotations();
    
    // 结构体中的生命周期注解
    demonstrate_struct_lifetime_annotations();
    
    // 方法中的生命周期注解
    demonstrate_method_lifetime_annotations();
}

/// 演示函数中的生命周期注解
fn demonstrate_function_lifetime_annotations() {
    println!("\n🔧 函数生命周期注解:");
    
    // 简单的生命周期注解
    fn first_word<'a>(s: &'a str) -> &'a str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
    
    let sentence = "Hello world from Rust";
    let word = first_word(sentence);
    println!("句子: {}", sentence);
    println!("第一个单词: {}", word);
    
    // 多个生命周期参数
    fn compare_and_return<'a, 'b>(s1: &'a str, s2: &'b str, flag: bool) -> &'a str {
        if flag {
            s1
        } else {
            // s2  // 这会导致编译错误，因为返回类型是 'a
            s1  // 必须返回与 'a 生命周期相同的引用
        }
    }
    
    let str1 = "First string";
    let str2 = "Second string";
    let result = compare_and_return(str1, str2, true);
    println!("比较结果: {}", result);
}

/// 演示结构体中的生命周期注解
fn demonstrate_struct_lifetime_annotations() {
    println!("\n🏗️ 结构体生命周期注解:");
    
    // 包含引用的结构体必须有生命周期注解
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'")
    ;
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("小说: {}", novel);
    println!("摘录: {:?}", excerpt);
    
    // 多个引用字段
    #[derive(Debug)]
    struct DoubleRef<'a, 'b> {
        first: &'a str,
        second: &'b str,
    }
    
    let s1 = "First";
    let s2 = "Second";
    let double = DoubleRef {
        first: s1,
        second: s2,
    };
    
    println!("双引用结构体: {:?}", double);
}

/// 演示方法中的生命周期注解
fn demonstrate_method_lifetime_annotations() {
    println!("\n🔨 方法生命周期注解:");
    
    struct StringHolder<'a> {
        content: &'a str,
    }
    
    impl<'a> StringHolder<'a> {
        // 方法的生命周期注解
        fn get_content(&self) -> &str {
            self.content
        }
        
        // 带有额外生命周期参数的方法
        fn compare_with<'b>(&self, other: &'b str) -> &str {
            if self.content.len() > other.len() {
                self.content
            } else {
                // other  // 这会导致编译错误
                self.content  // 返回 self 的引用
            }
        }
        
        // 静态方法
        fn create_from_static() -> StringHolder<'static> {
            StringHolder {
                content: "Static string",
            }
        }
    }
    
    let content = "Hello, lifetime!";
    let holder = StringHolder { content };
    
    println!("持有者内容: {}", holder.get_content());
    
    let other = "Short";
    let comparison = holder.compare_with(other);
    println!("比较结果: {}", comparison);
    
    let static_holder = StringHolder::create_from_static();
    println!("静态持有者: {}", static_holder.get_content());
}

/// 生命周期省略规则
fn lifetime_elision() {
    println!("\n📌 5. 生命周期省略规则");
    println!("{}", "-".repeat(30));

    // 规则1: 每个引用参数都有自己的生命周期
    demonstrate_elision_rule_1();
    
    // 规则2: 如果只有一个输入生命周期，它被赋给所有输出生命周期
    demonstrate_elision_rule_2();
    
    // 规则3: 如果有 &self 或 &mut self，self 的生命周期被赋给所有输出生命周期
    demonstrate_elision_rule_3();
}

/// 演示省略规则1
fn demonstrate_elision_rule_1() {
    println!("\n1️⃣ 规则1: 每个引用参数都有自己的生命周期");
    
    // 显式写法
    fn explicit_lifetimes<'a, 'b>(s1: &'a str, s2: &'b str) {
        println!("参数1: {}, 参数2: {}", s1, s2);
    }
    
    // 省略写法（编译器自动推断）
    fn elided_lifetimes(s1: &str, s2: &str) {
        println!("参数1: {}, 参数2: {}", s1, s2);
    }
    
    let str1 = "Hello";
    let str2 = "World";
    
    explicit_lifetimes(str1, str2);
    elided_lifetimes(str1, str2);
    println!("✅ 编译器自动为每个引用参数分配生命周期");
}

/// 演示省略规则2
fn demonstrate_elision_rule_2() {
    println!("\n2️⃣ 规则2: 单一输入生命周期传播到输出");
    
    // 显式写法
    fn explicit_single_lifetime<'a>(s: &'a str) -> &'a str {
        s
    }
    
    // 省略写法
    fn elided_single_lifetime(s: &str) -> &str {
        s
    }
    
    let input = "Single lifetime";
    let result1 = explicit_single_lifetime(input);
    let result2 = elided_single_lifetime(input);
    
    println!("显式结果: {}", result1);
    println!("省略结果: {}", result2);
    println!("✅ 单一输入生命周期自动传播到输出");
}

/// 演示省略规则3
fn demonstrate_elision_rule_3() {
    println!("\n3️⃣ 规则3: self 的生命周期传播到输出");
    
    struct TextProcessor<'a> {
        text: &'a str,
    }
    
    impl<'a> TextProcessor<'a> {
        // 显式写法
        fn get_text_explicit(&self) -> &'a str {
            self.text
        }
        
        // 省略写法
        fn get_text_elided(&self) -> &str {
            self.text
        }
        
        // 带有其他参数的方法
        fn combine_with(&self, other: &str) -> &str {
            // 返回类型的生命周期来自 &self
            self.text
        }
    }
    
    let text = "Processor text";
    let processor = TextProcessor { text };
    
    println!("显式方法: {}", processor.get_text_explicit());
    println!("省略方法: {}", processor.get_text_elided());
    
    let other = "Other text";
    println!("组合方法: {}", processor.combine_with(other));
    println!("✅ self 的生命周期自动传播到输出");
}

/// 引用模式
fn reference_patterns() {
    println!("\n📌 6. 引用模式");
    println!("{}", "-".repeat(30));

    // 模式匹配中的引用
    demonstrate_pattern_matching_references();
    
    // 解构中的引用
    demonstrate_destructuring_references();
    
    // 引用与迭代器
    demonstrate_references_with_iterators();
}

/// 演示模式匹配中的引用
fn demonstrate_pattern_matching_references() {
    println!("\n🎯 模式匹配中的引用:");
    
    let value = Some(42);
    
    // 匹配引用
    match &value {
        Some(n) => println!("匹配到引用: {}", n),
        None => println!("没有值"),
    }
    
    // 在模式中使用 ref
    match value {
        Some(ref n) => println!("使用 ref: {}", n),
        None => println!("没有值"),
    }
    
    // 可变引用模式
    let mut mutable_value = Some(100);
    match &mut mutable_value {
        Some(n) => {
            *n += 50;
            println!("修改后的值: {}", n);
        }
        None => println!("没有值"),
    }
    
    println!("最终值: {:?}", mutable_value);
}

/// 演示解构中的引用
fn demonstrate_destructuring_references() {
    println!("\n🔧 解构中的引用:");
    
    let tuple = (String::from("Hello"), 42, true);
    
    // 解构时借用
    let (ref s, n, b) = tuple;
    println!("借用的字符串: {}", s);
    println!("复制的数字: {}", n);
    println!("复制的布尔值: {}", b);
    println!("原始元组仍可用: {:?}", tuple);
    
    // 结构体解构
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }
    
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    let Person { ref name, age } = person;
    println!("借用的姓名: {}", name);
    println!("复制的年龄: {}", age);
    println!("原始结构体: {:?}", person);
}

/// 演示引用与迭代器
fn demonstrate_references_with_iterators() {
    println!("\n🔄 引用与迭代器:");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // iter() 返回不可变引用
    println!("不可变引用迭代:");
    for item in numbers.iter() {
        println!("  &{} (地址: {:p})", item, item);
    }
    
    // 原始向量仍然可用
    println!("原始向量: {:?}", numbers);
    
    // into_iter() 获取所有权
    let numbers2 = vec![10, 20, 30];
    println!("\n所有权迭代:");
    for item in numbers2.into_iter() {
        println!("  {} (拥有所有权)", item);
    }
    // numbers2 在这里不再可用
    
    // iter_mut() 返回可变引用
    let mut numbers3 = vec![100, 200, 300];
    println!("\n可变引用迭代:");
    for item in numbers3.iter_mut() {
        *item *= 2;
        println!("  修改后: {}", item);
    }
    println!("修改后的向量: {:?}", numbers3);
}

/// 方法语法和自动解引用
fn method_syntax_and_deref() {
    println!("\n📌 7. 方法语法和自动解引用");
    println!("{}", "-".repeat(30));

    // 自动解引用演示
    demonstrate_auto_deref();
    
    // Deref trait
    demonstrate_deref_trait();
    
    // 方法调用的解引用强制转换
    demonstrate_method_deref_coercion();
}

/// 演示自动解引用
fn demonstrate_auto_deref() {
    println!("\n🔄 自动解引用:");
    
    let s = String::from("Hello, world!");
    let s_ref = &s;
    let s_ref_ref = &s_ref;
    
    // 所有这些调用都是等价的
    println!("直接调用: {}", s.len());
    println!("一级引用: {}", s_ref.len());
    println!("二级引用: {}", s_ref_ref.len());
    
    // 手动解引用
    println!("手动解引用: {}", (*s_ref).len());
    println!("双重手动解引用: {}", (**s_ref_ref).len());
    
    println!("✅ Rust 自动处理方法调用的解引用");
}

/// 演示 Deref trait
fn demonstrate_deref_trait() {
    println!("\n🎭 Deref trait:");
    
    use std::ops::Deref;
    
    // 自定义智能指针
    struct MyBox<T>(T);
    
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    
    impl<T> Deref for MyBox<T> {
        type Target = T;
        
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    
    let boxed_string = MyBox::new(String::from("Boxed string"));
    
    // 自动解引用
    println!("自动解引用: {}", boxed_string.len());
    
    // 手动解引用
    println!("手动解引用: {}", (*boxed_string).len());
    
    // 解引用强制转换
    fn print_str(s: &str) {
        println!("打印字符串: {}", s);
    }
    
    print_str(&boxed_string);  // MyBox<String> -> &String -> &str
    println!("✅ Deref trait 实现了解引用强制转换");
}

/// 演示方法调用的解引用强制转换
fn demonstrate_method_deref_coercion() {
    println!("\n🔧 方法调用的解引用强制转换:");
    
    let s = String::from("Hello");
    let s_ref = &s;
    
    // 先使用引用
    println!("String: {}", s_ref.chars().count());
    
    // 引用使用完毕后，可以移动原值
    let s_box = Box::new(s);
    println!("Box<String>: {}", s_box.chars().count());
    
    // 展示强制转换链
    // Box<String> -> String -> str
    let boxed_string = Box::new(String::from("Boxed"));
    
    fn takes_str_slice(s: &str) {
        println!("接收 &str: {}", s);
    }
    
    takes_str_slice(&boxed_string);  // 自动强制转换
    println!("✅ 多级解引用强制转换");
}

/// 引用强制转换
fn reference_coercion() {
    println!("\n📌 8. 引用强制转换");
    println!("{}", "-".repeat(30));

    // 子类型强制转换
    demonstrate_subtype_coercion();
    
    // 可变性强制转换
    demonstrate_mutability_coercion();
    
    // 生命周期强制转换
    demonstrate_lifetime_coercion();
}

/// 演示子类型强制转换
fn demonstrate_subtype_coercion() {
    println!("\n🔄 子类型强制转换:");
    
    // &String -> &str
    let string = String::from("Hello");
    let string_ref: &String = &string;
    let str_ref: &str = string_ref;  // 自动强制转换
    
    println!("String 引用: {}", string_ref);
    println!("str 引用: {}", str_ref);
    
    // &Vec<T> -> &[T]
    let vec = vec![1, 2, 3, 4, 5];
    let vec_ref: &Vec<i32> = &vec;
    let slice_ref: &[i32] = vec_ref;  // 自动强制转换
    
    println!("Vec 引用: {:?}", vec_ref);
    println!("切片引用: {:?}", slice_ref);
    
    // 函数参数中的强制转换
    fn print_slice(s: &[i32]) {
        println!("切片: {:?}", s);
    }
    
    print_slice(&vec);  // &Vec<i32> -> &[i32]
    println!("✅ 自动子类型强制转换");
}

/// 演示可变性强制转换
fn demonstrate_mutability_coercion() {
    println!("\n🔧 可变性强制转换:");
    
    let mut data = vec![1, 2, 3];
    
    // &mut T -> &T (可变引用可以强制转换为不可变引用)
    let mut_ref: &mut Vec<i32> = &mut data;
    let immut_ref: &Vec<i32> = mut_ref;  // 强制转换
    
    println!("通过不可变引用访问: {:?}", immut_ref);
    
    // 函数参数中的可变性强制转换
    fn read_only(v: &Vec<i32>) {
        println!("只读访问: {:?}", v);
    }
    
    let mut data2 = vec![4, 5, 6];
    read_only(&mut data2);  // &mut Vec<i32> -> &Vec<i32>
    
    println!("✅ 可变引用可以强制转换为不可变引用");
    println!("⚠️ 反向转换（&T -> &mut T）是不允许的");
}

/// 演示生命周期强制转换
fn demonstrate_lifetime_coercion() {
    println!("\n⏰ 生命周期强制转换:");
    
    // 较长生命周期可以强制转换为较短生命周期
    let long_lived = String::from("Long lived string");
    
    {
        let short_lived = String::from("Short lived");
        
        // 函数需要相同的生命周期
        fn compare_strings<'a>(s1: &'a str, s2: &'a str) -> &'a str {
            if s1.len() > s2.len() { s1 } else { s2 }
        }
        
        // long_lived 的生命周期被强制转换为与 short_lived 相同
        let result = compare_strings(&long_lived, &short_lived);
        println!("比较结果: {}", result);
    }
    
    println!("长生命周期变量仍然有效: {}", long_lived);
    println!("✅ 较长生命周期可以强制转换为较短生命周期");
}

/// 内部可变性预览
fn interior_mutability_preview() {
    println!("\n📌 9. 内部可变性预览");
    println!("{}", "-".repeat(30));

    // Cell 类型
    demonstrate_cell_type();
    
    // RefCell 类型
    demonstrate_refcell_type();
}

/// 演示 Cell 类型
fn demonstrate_cell_type() {
    use std::cell::Cell;
    
    println!("\n🔬 Cell 类型:");
    
    let cell = Cell::new(42);
    println!("Cell 初始值: {}", cell.get());
    
    // 即使 cell 是不可变的，也可以修改其内容
    cell.set(100);
    println!("Cell 修改后: {}", cell.get());
    
    // Cell 不提供引用，只能获取值的副本
    let value = cell.get();
    println!("获取的值: {}", value);
    
    println!("✅ Cell 提供了对 Copy 类型的内部可变性");
}

/// 演示 RefCell 类型
fn demonstrate_refcell_type() {
    use std::cell::RefCell;
    
    println!("\n🔬 RefCell 类型:");
    
    let refcell = RefCell::new(vec![1, 2, 3]);
    
    // 借用不可变引用
    {
        let borrowed = refcell.borrow();
        println!("不可变借用: {:?}", *borrowed);
    } // 借用在这里结束
    
    // 借用可变引用
    {
        let mut borrowed_mut = refcell.borrow_mut();
        borrowed_mut.push(4);
        println!("可变借用后: {:?}", *borrowed_mut);
    } // 可变借用在这里结束
    
    // 再次不可变借用
    let final_borrow = refcell.borrow();
    println!("最终状态: {:?}", *final_borrow);
    
    println!("✅ RefCell 提供了运行时借用检查");
    
    // 演示运行时借用检查
    demonstrate_runtime_borrow_checking();
}

/// 演示运行时借用检查
fn demonstrate_runtime_borrow_checking() {
    use std::cell::RefCell;
    
    println!("\n⚠️ 运行时借用检查:");
    
    let refcell = RefCell::new(42);
    
    let _borrow1 = refcell.borrow();
    let _borrow2 = refcell.borrow();  // 多个不可变借用是允许的
    
    println!("多个不可变借用: 成功");
    
    // 释放不可变借用
    drop(_borrow1);
    drop(_borrow2);
    
    let _mut_borrow = refcell.borrow_mut();  // 现在可以可变借用
    println!("可变借用: 成功");
    
    // 以下代码会在运行时 panic
    // let _another_borrow = refcell.borrow();  // panic!
    
    println!("✅ RefCell 在运行时强制执行借用规则");
}

/// 高级生命周期模式
fn advanced_lifetime_patterns() {
    println!("\n📌 10. 高级生命周期模式");
    println!("{}", "-".repeat(30));

    // 静态生命周期
    demonstrate_static_lifetime();
    
    // 生命周期子类型
    demonstrate_lifetime_subtyping();
    
    // 高阶生命周期
    demonstrate_higher_ranked_lifetimes();
}

/// 演示静态生命周期
fn demonstrate_static_lifetime() {
    println!("\n♾️ 静态生命周期:");
    
    // 字符串字面量具有 'static 生命周期
    let static_str: &'static str = "This is a static string";
    println!("静态字符串: {}", static_str);
    
    // 静态变量
    static GLOBAL_STR: &str = "Global static string";
    println!("全局静态字符串: {}", GLOBAL_STR);
    
    // 函数返回静态引用
    fn get_static_str() -> &'static str {
        "Static from function"
    }
    
    let func_static = get_static_str();
    println!("函数返回的静态字符串: {}", func_static);
    
    // 'static 约束
    fn needs_static<T: Display + 'static>(x: T) {
        println!("需要 'static 约束: {}", x);
    }
    
    needs_static(42);  // i32 满足 'static
    needs_static("static string");  // &'static str 满足 'static
    
    println!("✅ 'static 生命周期持续整个程序运行期间");
}

/// 演示生命周期子类型
fn demonstrate_lifetime_subtyping() {
    println!("\n🔄 生命周期子类型:");
    
    // 较长的生命周期是较短生命周期的子类型
    let long_string = String::from("Long lived");
    
    {
        let short_string = String::from("Short lived");
        
        // 这个函数接受任何生命周期的字符串引用
        fn process_string<'a>(s: &'a str) -> &'a str {
            println!("处理字符串: {}", s);
            s
        }
        
        // long_string 的生命周期被"缩短"以匹配这个作用域
        let result1 = process_string(&long_string);
        let result2 = process_string(&short_string);
        
        println!("结果1: {}", result1);
        println!("结果2: {}", result2);
    }
    
    println!("长生命周期字符串仍然有效: {}", long_string);
    println!("✅ 生命周期子类型允许灵活的生命周期匹配");
}

/// 演示高阶生命周期
fn demonstrate_higher_ranked_lifetimes() {
    println!("\n🎯 高阶生命周期 (HRTB):");
    
    // for<'a> 语法表示"对于任何生命周期 'a"
    fn apply_to_string<F>(f: F) -> String 
    where
        F: for<'a> Fn(&'a str) -> &'a str,
    {
        let s = "Hello, HRTB!";
        f(s).to_string()
    }
    
    // 这个闭包可以处理任何生命周期的字符串引用
    let result = apply_to_string(|s| {
        if s.len() > 5 {
            &s[0..5]
        } else {
            s
        }
    });
    
    println!("HRTB 结果: {}", result);
    
    // 另一个 HRTB 示例
    fn call_with_different_lifetimes<F>(f: F) 
    where
        F: for<'a, 'b> Fn(&'a str, &'b str) -> bool,
    {
        let s1 = "First";
        {
            let s2 = "Second";
            let result = f(s1, s2);
            println!("比较结果: {}", result);
        }
    }
    
    call_with_different_lifetimes(|a, b| a.len() > b.len());
    
    println!("✅ HRTB 允许函数处理任意生命周期的引用");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_references() {
        let x = 42;
        let x_ref = &x;
        assert_eq!(*x_ref, 42);
        assert_eq!(x, 42);  // x 仍然可用
    }

    #[test]
    fn test_mutable_references() {
        let mut y = 100;
        let y_ref = &mut y;
        *y_ref = 200;
        assert_eq!(y, 200);
    }

    #[test]
    fn test_lifetime_annotations() {
        fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
            if s1.len() > s2.len() { s1 } else { s2 }
        }
        
        let str1 = "Hello";
        let str2 = "World!";
        let result = longest(str1, str2);
        assert_eq!(result, "World!");
    }

    #[test]
    fn test_reference_patterns() {
        let value = Some(42);
        match &value {
            Some(n) => assert_eq!(*n, 42),
            None => panic!("Should not be None"),
        }
        assert_eq!(value, Some(42));  // value 仍然可用
    }

    #[test]
    fn test_deref_coercion() {
        let s = String::from("Hello");
        let s_ref = &s;
        
        fn takes_str_slice(s: &str) -> usize {
            s.len()
        }
        
        // &String -> &str 自动强制转换
        assert_eq!(takes_str_slice(s_ref), 5);
    }
}