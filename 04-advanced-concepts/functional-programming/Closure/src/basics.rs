//! # 闭包基础概念和语法
//!
//! 本模块介绍 Rust 闭包的基础概念，包括：
//! - 闭包的定义和基本语法
//! - 闭包与函数的区别
//! - 闭包的类型推导
//! - 闭包的基本使用方式

/// 演示闭包基础概念
pub fn demonstrate() {
    println!("\n🔍 1. 闭包基础概念");
    println!("{}", "-".repeat(40));

    basic_syntax();
    closure_with_parameters();
    closure_with_type_annotations();
    multiline_closures();
    closure_vs_function();
}

/// 演示闭包的基本语法
fn basic_syntax() {
    println!("\n📝 基本语法:");

    // 最简单的闭包
    let simple_closure = || println!("Hello from closure!");
    simple_closure();

    // 带返回值的闭包
    let add_one = |x| x + 1;
    println!("add_one(5) = {}", add_one(5));

    // 闭包可以赋值给变量
    let multiply = |a, b| a * b;
    println!("multiply(3, 4) = {}", multiply(3, 4));

    // 闭包可以直接调用
    let result = (|x: i32| x * x)(6);
    println!("直接调用闭包: (|x| x * x)(6) = {}", result);
}

/// 演示带参数的闭包
fn closure_with_parameters() {
    println!("\n📝 带参数的闭包:");

    // 单个参数
    let square = |x| {
        println!("计算 {} 的平方", x);
        x * x
    };
    println!("square(4) = {}", square(4));

    // 多个参数
    let calculate = |a, b, operation: &str| match operation {
        "add" => a + b,
        "sub" => a - b,
        "mul" => a * b,
        "div" => a / b,
        _ => 0,
    };

    println!("calculate(10, 3, \"add\") = {}", calculate(10, 3, "add"));
    println!("calculate(10, 3, \"mul\") = {}", calculate(10, 3, "mul"));

    // 无参数闭包
    let get_random = || {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        std::ptr::addr_of!(hasher).hash(&mut hasher);
        hasher.finish() % 100
    };
    println!("随机数: {}", get_random());
}

/// 演示带类型注解的闭包
fn closure_with_type_annotations() {
    println!("\n📝 带类型注解的闭包:");

    // 显式指定参数和返回值类型
    let typed_closure = |x: i32, y: i32| -> i32 {
        println!("计算 {} + {} = {}", x, y, x + y);
        x + y
    };
    println!("typed_closure(5, 3) = {}", typed_closure(5, 3));

    // 复杂类型的闭包
    let string_processor =
        |s: &str| -> String { format!("处理后的字符串: [{}]", s.to_uppercase()) };
    println!("{}", string_processor("hello world"));

    // 返回元组的闭包
    let tuple_creator = |a: i32, b: i32| -> (i32, i32, i32) { (a, b, a + b) };
    let (x, y, sum) = tuple_creator(7, 8);
    println!("元组结果: ({}, {}, {})", x, y, sum);
}

/// 演示多行闭包
fn multiline_closures() {
    println!("\n📝 多行闭包:");

    let complex_calculation = |n: i32| -> i32 {
        println!("开始复杂计算，输入: {}", n);

        let mut result = n;

        // 第一步：平方
        result = result * result;
        println!("步骤1 - 平方: {}", result);

        // 第二步：加10
        result = result + 10;
        println!("步骤2 - 加10: {}", result);

        // 第三步：除以2
        result = result / 2;
        println!("步骤3 - 除以2: {}", result);

        println!("计算完成，最终结果: {}", result);
        result
    };

    let final_result = complex_calculation(5);
    println!("complex_calculation(5) = {}", final_result);

    // 条件逻辑的闭包
    let conditional_closure = |x: i32| -> String {
        if x > 0 {
            format!("正数: {}", x)
        } else if x < 0 {
            format!("负数: {}", x)
        } else {
            "零".to_string()
        }
    };

    println!("{}", conditional_closure(10));
    println!("{}", conditional_closure(-5));
    println!("{}", conditional_closure(0));
}

/// 演示闭包与函数的区别
fn closure_vs_function() {
    println!("\n📝 闭包 vs 函数:");

    // 普通函数
    fn regular_function(x: i32) -> i32 {
        x * 2
    }

    // 等价的闭包
    let closure_equivalent = |x: i32| -> i32 { x * 2 };

    println!("函数调用: regular_function(5) = {}", regular_function(5));
    println!(
        "闭包调用: closure_equivalent(5) = {}",
        closure_equivalent(5)
    );

    // 闭包可以捕获环境变量（这是主要区别）
    let multiplier = 3;
    let closure_with_capture = |x| x * multiplier; // 捕获了 multiplier
    println!(
        "捕获环境变量的闭包: closure_with_capture(4) = {}",
        closure_with_capture(4)
    );

    // 函数不能直接访问环境变量
    // fn function_cannot_capture(x: i32) -> i32 {
    //     x * multiplier  // 编译错误！函数不能捕获环境变量
    // }

    // 闭包的类型推导
    let inferred_closure = |x| x + 1; // 类型会根据使用情况推导
    println!("类型推导的闭包: {}", inferred_closure(10));

    // 闭包可以存储在变量中并传递
    let operations = vec![|x: i32| x + 1, |x: i32| x * 2, |x: i32| x - 3];

    println!("\n闭包数组的使用:");
    for (i, op) in operations.iter().enumerate() {
        println!("操作{}: op(5) = {}", i + 1, op(5));
    }

    demonstrate_closure_flexibility();
}

/// 演示闭包的灵活性
fn demonstrate_closure_flexibility() {
    println!("\n📝 闭包的灵活性:");

    // 闭包可以作为参数传递
    fn apply_operation<F>(x: i32, y: i32, op: F) -> i32
    where
        F: Fn(i32, i32) -> i32,
    {
        op(x, y)
    }

    let add = |a, b| a + b;
    let multiply = |a, b| a * b;
    let power = |a: i32, b| a.pow(b as u32);

    println!(
        "apply_operation(5, 3, add) = {}",
        apply_operation(5, 3, add)
    );
    println!(
        "apply_operation(5, 3, multiply) = {}",
        apply_operation(5, 3, multiply)
    );
    println!(
        "apply_operation(2, 3, power) = {}",
        apply_operation(2, 3, power)
    );

    // 闭包可以返回闭包
    fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
        move |x| x * factor
    }

    let double = create_multiplier(2);
    let triple = create_multiplier(3);

    println!("double(7) = {}", double(7));
    println!("triple(7) = {}", triple(7));

    // 闭包链式调用
    let result = (0..5)
        .map(|x| x * 2) // 每个元素乘以2
        .filter(|&x| x > 2) // 过滤大于2的元素
        .fold(0, |acc, x| acc + x); // 求和

    println!("链式闭包操作结果: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_closure() {
        let add_one = |x| x + 1;
        assert_eq!(add_one(5), 6);
    }

    #[test]
    fn test_closure_with_capture() {
        let multiplier = 3;
        let multiply_by_three = |x| x * multiplier;
        assert_eq!(multiply_by_three(4), 12);
    }

    #[test]
    fn test_multiline_closure() {
        let complex = |x: i32| -> i32 {
            let step1 = x * x;
            let step2 = step1 + 10;
            step2 / 2
        };
        assert_eq!(complex(4), 13); // (4*4 + 10) / 2 = 13
    }
}
