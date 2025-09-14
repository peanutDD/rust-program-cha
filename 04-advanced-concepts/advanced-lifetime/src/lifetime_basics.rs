//! # 生命周期基础概念
//!
//! 本模块深入探讨 Rust 生命周期的核心概念，包括：
//! - 悬垂引用的产生和防范
//! - 借用检查器的工作原理
//! - 生命周期注解的语法和使用

use std::fmt::Display;

/// # 1. 悬垂引用（Dangling References）
///
/// 悬垂引用是指引用指向已经被释放的内存地址，这是内存安全的重大威胁。
/// Rust 通过生命周期系统在编译期防止悬垂引用的产生。
pub mod dangling_references {
    /// 演示悬垂引用的产生场景
    ///
    /// 以下代码会产生编译错误，因为 r 试图引用一个已经离开作用域的值
    pub fn demonstrate_dangling_reference() {
        println!("=== 悬垂引用演示 ===");

        // 这段代码无法编译，用于说明悬垂引用的问题
        /*
        let r;
        {
            let x = 5;
            r = &x;  // 错误：x 在这个作用域结束后会被释放
        }
        println!("r: {}", r);  // r 引用了已释放的内存
        */

        // 正确的做法：确保引用的生命周期不超过被引用值的生命周期
        let x = 5;
        let r = &x;
        println!("正确的引用: r = {}", r);

        // 演示更复杂的悬垂引用场景
        demonstrate_function_return_dangling();
    }

    /// 演示函数返回悬垂引用的场景
    fn demonstrate_function_return_dangling() {
        println!("\n--- 函数返回悬垂引用演示 ---");

        // 这个函数无法编译，因为返回了局部变量的引用
        /*
        fn dangle() -> &str {
            let s = String::from("hello");
            &s  // 错误：返回了局部变量的引用
        }
        */

        // 正确的做法：返回拥有所有权的值
        fn no_dangle() -> String {
            let s = String::from("hello");
            s // 返回所有权，避免悬垂引用
        }

        let result = no_dangle();
        println!("正确返回的字符串: {}", result);
    }
}

/// # 2. 借用检查器（Borrow Checker）
///
/// 借用检查器是 Rust 编译器的核心组件，负责验证引用的有效性和内存安全。
pub mod borrow_checker {
    /// 演示借用检查器的工作原理
    pub fn demonstrate_borrow_checker() {
        println!("\n=== 借用检查器演示 ===");

        // 1. 基本借用规则
        demonstrate_basic_borrowing_rules();

        // 2. 可变借用和不可变借用的冲突
        demonstrate_mutable_immutable_conflict();

        // 3. 借用作用域分析
        demonstrate_borrow_scope_analysis();
    }

    /// 演示基本借用规则
    fn demonstrate_basic_borrowing_rules() {
        println!("\n--- 基本借用规则 ---");

        let mut s = String::from("hello");

        // 规则1：可以有多个不可变借用
        let r1 = &s;
        let r2 = &s;
        println!("多个不可变借用: r1={}, r2={}", r1, r2);

        // 规则2：可变借用和不可变借用不能同时存在
        // let r3 = &s;      // 不可变借用
        // let r4 = &mut s;  // 错误：不能在不可变借用存在时创建可变借用

        // 规则3：同一时间只能有一个可变借用
        let r3 = &mut s;
        // let r4 = &mut s;  // 错误：不能有多个可变借用
        r3.push_str(", world");
        println!("可变借用修改后: {}", r3);
    }

    /// 演示可变借用和不可变借用的冲突
    fn demonstrate_mutable_immutable_conflict() {
        println!("\n--- 可变借用和不可变借用冲突 ---");

        let mut s = String::from("hello");

        // 这段代码展示了 NLL (Non-Lexical Lifetimes) 的改进
        let r1 = &s; // 不可变借用
        let r2 = &s; // 不可变借用
        println!("不可变借用: {}, {}", r1, r2);
        // r1 和 r2 在这里不再被使用，所以它们的生命周期结束

        let r3 = &mut s; // 可变借用，这里是安全的
        r3.push_str(", world");
        println!("可变借用: {}", r3);
    }

    /// 演示借用作用域分析
    fn demonstrate_borrow_scope_analysis() {
        println!("\n--- 借用作用域分析 ---");

        let mut s = String::from("hello");

        {
            let r1 = &mut s; // 可变借用开始
            r1.push_str(", world");
            println!("内部作用域的可变借用: {}", r1);
        } // r1 的生命周期结束

        // 现在可以创建新的借用
        let r2 = &s;
        println!("外部作用域的不可变借用: {}", r2);

        // 演示复杂的借用场景
        demonstrate_complex_borrowing(&mut s);
    }

    /// 演示复杂的借用场景
    fn demonstrate_complex_borrowing(s: &mut String) {
        println!("\n--- 复杂借用场景 ---");

        // 函数参数本身就是一个可变借用
        s.push_str("!");

        // 在函数内部创建不可变借用
        let len = s.len();
        println!("字符串长度: {}", len);

        // 再次使用可变借用
        s.push_str(" Rust");
        println!("最终字符串: {}", s);
    }
}

/// # 3. 生命周期注解语法
///
/// 生命周期注解用于明确指定引用之间的生命周期关系，帮助编译器进行借用检查。
pub mod lifetime_annotation_syntax {
    use std::fmt::Display;

    /// 演示生命周期注解的基本语法
    pub fn demonstrate_lifetime_annotation() {
        println!("\n=== 生命周期注解语法演示 ===");

        // 1. 基本生命周期注解
        demonstrate_basic_annotation();

        // 2. 函数中的生命周期注解
        demonstrate_function_annotation();

        // 3. 结构体中的生命周期注解
        demonstrate_struct_annotation();

        // 4. 方法中的生命周期注解
        demonstrate_method_annotation();
    }

    /// 演示基本生命周期注解
    fn demonstrate_basic_annotation() {
        println!("\n--- 基本生命周期注解 ---");

        let string1 = String::from("long string is long");
        let string2 = "xyz";

        let result = longest(&string1, string2);
        println!("最长的字符串是: {}", result);

        // 演示生命周期约束
        {
            let string3 = String::from("short");
            let result2 = longest(&string1, &string3);
            println!("比较结果: {}", result2);
        } // string3 在这里被释放，但 result2 不能在这个作用域外使用
    }

    /// 需要生命周期注解的函数
    ///
    /// 这个函数接受两个字符串切片，返回较长的那个
    /// 由于返回值可能来自任一参数，编译器无法推断生命周期，需要显式注解
    pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    /// 演示函数中的生命周期注解
    fn demonstrate_function_annotation() {
        println!("\n--- 函数生命周期注解 ---");

        let s1 = "hello";
        let s2 = "world";

        // 不同的生命周期注解模式
        let result1 = first_word(s1);
        println!("第一个单词: {}", result1);

        let result2 = longest_with_announcement(s1, s2, "比较两个字符串");
        println!("带公告的最长字符串: {}", result2);
    }

    /// 只有一个生命周期参数的函数
    /// 返回值的生命周期与输入参数相同
    fn first_word<'a>(s: &'a str) -> &'a str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    /// 多个生命周期参数的函数
    /// 演示泛型、生命周期和 trait bound 的结合使用
    fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("公告: {}", ann);
        if x.len() > y.len() { x } else { y }
    }

    /// 演示结构体中的生命周期注解
    fn demonstrate_struct_annotation() {
        println!("\n--- 结构体生命周期注解 ---");

        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };

        println!("重要摘录: {}", i.part);

        // 演示生命周期约束
        let announcement = "今天我们学习生命周期";
        let level = i.announce_and_return_part(announcement);
        println!("返回的部分: {}", level);
    }

    /// 包含引用的结构体必须有生命周期注解
    #[derive(Debug)]
    pub struct ImportantExcerpt<'a> {
        pub part: &'a str,
    }

    /// 演示方法中的生命周期注解
    fn demonstrate_method_annotation() {
        println!("\n--- 方法生命周期注解 ---");

        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let excerpt = ImportantExcerpt {
            part: first_sentence,
        };

        // 调用不同的方法来演示生命周期规则
        let level = excerpt.level();
        println!("级别: {}", level);

        let announcement = "特别公告";
        let returned_part = excerpt.announce_and_return_part(announcement);
        println!("公告后返回: {}", returned_part);
    }

    impl<'a> ImportantExcerpt<'a> {
        /// 方法的生命周期省略规则
        /// 返回值的生命周期与 self 相同
        fn level(&self) -> i32 {
            3
        }

        /// 多个生命周期参数的方法
        /// 返回值的生命周期与 self 相同，而不是与 announcement 相同
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("请注意: {}", announcement);
            self.part
        }

        /// 演示不同生命周期参数的方法
        fn compare_and_return<'b>(&self, other: &'b str) -> &str
        where
            'a: 'b, // 'a 的生命周期至少与 'b 一样长
        {
            if self.part.len() > other.len() {
                self.part
            } else {
                // 这里不能返回 other，因为它的生命周期可能比返回值要求的短
                self.part
            }
        }
    }
}

/// # 4. 生命周期的高级概念
///
/// 探讨生命周期的一些高级概念和使用场景
pub mod advanced_lifetime_concepts {
    /// 演示生命周期的高级概念
    pub fn demonstrate_advanced_concepts() {
        println!("\n=== 生命周期高级概念演示 ===");

        // 1. 生命周期子类型化
        demonstrate_lifetime_subtyping();

        // 2. 生命周期约束
        demonstrate_lifetime_bounds();

        // 3. 高阶生命周期
        demonstrate_higher_ranked_lifetimes();
    }

    /// 演示生命周期子类型化
    fn demonstrate_lifetime_subtyping() {
        println!("\n--- 生命周期子类型化 ---");

        // 较长的生命周期可以被强制转换为较短的生命周期
        let long_lived = String::from("long lived string");

        {
            let short_lived = String::from("short");

            // 这里演示了生命周期的协变性
            let result = choose_str(&long_lived, &short_lived);
            println!("选择的字符串: {}", result);
        }
    }

    /// 选择第一个字符串
    /// 演示生命周期参数的协变性
    fn choose_str<'a, 'b>(first: &'a str, _second: &'b str) -> &'a str {
        first
    }

    /// 演示生命周期约束
    fn demonstrate_lifetime_bounds() {
        println!("\n--- 生命周期约束 ---");

        let string1 = String::from("hello");
        let string2 = String::from("world");

        let container = Container {
            data: &string1,
            reference: &string2,
        };

        container.print_data();
    }

    /// 演示复杂的生命周期约束
    struct Container<'a, 'b> {
        data: &'a str,
        reference: &'b str,
    }

    impl<'a, 'b> Container<'a, 'b> {
        /// 方法中的生命周期约束
        fn print_data(&self) {
            println!("数据: {}, 引用: {}", self.data, self.reference);
        }

        /// 演示生命周期约束的传递
        fn get_longer(&self) -> &str {
            if self.data.len() > self.reference.len() {
                self.data
            } else {
                self.reference
            }
        }
    }

    /// 演示高阶生命周期（Higher-Ranked Trait Bounds）
    fn demonstrate_higher_ranked_lifetimes() {
        println!("\n--- 高阶生命周期 ---");

        let string = String::from("hello world");
        let closure = |s: &str| s.len();

        let result = apply_to_string(&string, closure);
        println!("应用闭包的结果: {}", result);
    }

    /// 使用高阶生命周期约束的函数
    /// for<'a> 表示对于任何生命周期 'a
    fn apply_to_string<F>(s: &str, f: F) -> usize
    where
        F: for<'a> Fn(&'a str) -> usize,
    {
        f(s)
    }
}

/// 运行所有生命周期基础概念的演示
pub fn run_all_demonstrations() {
    run_all_lifetime_basics_examples();
}

/// 运行所有生命周期基础概念的演示（别名函数）
pub fn run_all_lifetime_basics_examples() {
    println!("🦀 Rust 深入生命周期 - 基础概念篇 🦀");
    println!("========================================");

    // 1. 悬垂引用演示
    dangling_references::demonstrate_dangling_reference();

    // 2. 借用检查器演示
    borrow_checker::demonstrate_borrow_checker();

    // 3. 生命周期注解语法演示
    lifetime_annotation_syntax::demonstrate_lifetime_annotation();

    // 4. 高级概念演示
    advanced_lifetime_concepts::demonstrate_advanced_concepts();

    println!("\n========================================");
    println!("✅ 生命周期基础概念演示完成！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifetime_basics() {
        // 测试基本的生命周期功能
        let s1 = "hello";
        let s2 = "world";

        let result = lifetime_annotation_syntax::longest(s1, s2);
        assert_eq!(result, "world"); // "world" 比 "hello" 长
    }

    #[test]
    fn test_struct_with_lifetime() {
        let novel = String::from("Call me Ishmael.");
        let first_sentence = novel.split('.').next().unwrap();

        let excerpt = lifetime_annotation_syntax::ImportantExcerpt {
            part: first_sentence,
        };

        assert_eq!(excerpt.part, "Call me Ishmael");
    }
}
