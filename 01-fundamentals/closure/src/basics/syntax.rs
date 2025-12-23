//! # 闭包基础语法
//!
//! 本模块介绍闭包的基本语法和使用方法

/// 演示闭包的基本语法
///
/// # 示例
///
/// ```rust
/// // 最简单的闭包
/// let simple = || println!("Hello!");
/// simple();
///
/// // 带参数的闭包
/// let add = |x, y| x + y;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn demo_basic_syntax() {
    println!("=== 1. 闭包的基本语法 ===");

    // 1.1 最简单的闭包：无参数
    let simple_closure = || println!("这是一个简单的闭包");
    simple_closure();

    // 1.2 带参数的闭包
    let add = |x, y| x + y;
    println!("5 + 3 = {}", add(5, 3));

    // 1.3 带类型注解的闭包
    let multiply: fn(i32, i32) -> i32 = |x, y| x * y;
    println!("4 * 6 = {}", multiply(4, 6));

    // 1.4 多行闭包
    let complex_closure = |x: i32| {
        let doubled = x * 2;
        let result = doubled + 10;
        println!("处理 {} 的结果是 {}", x, result);
        result
    };

    let result = complex_closure(5);
    println!("复杂闭包返回值: {}", result);

    // 1.5 类型推导
    demonstrate_type_inference();
}

/// 演示闭包的类型推导
fn demonstrate_type_inference() {
    println!("\n--- 类型推导示例 ---");

    // 编译器自动推导类型
    let inferred_closure = |a| a + 1;
    println!("类型推导闭包: {}", inferred_closure(10));

    // 一旦确定类型，就不能改变
    // inferred_closure("hello"); // 编译错误：类型已确定为整数

    // 显式类型注解
    let explicit_closure = |x: f64| -> f64 { x * 2.0 };
    println!("显式类型闭包: {}", explicit_closure(3.14));
}

/// 演示闭包与函数的区别
///
/// # 主要区别
///
/// - 闭包可以捕获环境中的变量，而函数不能
/// - 闭包的类型由编译器自动推导
/// - 每个闭包都有唯一的类型
pub fn demo_closure_vs_function() {
    println!("\n=== 2. 闭包与函数的区别 ===");

    let x = 10;

    // 普通函数无法访问环境变量
    fn normal_function(y: i32) -> i32 {
        // x  // 错误：无法访问外部变量 x
        y * 2
    }

    // 闭包可以捕获环境变量
    let closure_with_capture = |y| x + y; // 捕获了变量 x

    println!("普通函数结果: {}", normal_function(5));
    println!("闭包捕获变量结果: {}", closure_with_capture(5));

    // 展示闭包与函数指针的区别
    demonstrate_closure_vs_function_pointer();
}

/// 演示闭包与函数指针的区别
fn demonstrate_closure_vs_function_pointer() {
    println!("\n--- 闭包 vs 函数指针 ---");

    // 函数指针：固定大小，无状态
    fn regular_function(x: i32) -> i32 {
        x * 2
    }

    let func_ptr: fn(i32) -> i32 = regular_function;
    println!("函数指针大小: {} bytes", std::mem::size_of_val(&func_ptr));

    // 闭包：可变大小，有状态
    let multiplier = 3;
    let closure = |x: i32| x * multiplier;
    println!("闭包大小: {} bytes", std::mem::size_of_val(&closure));

    // 函数指针可以转换为闭包，但反之不行
    let closure_from_fn: Box<dyn Fn(i32) -> i32> = Box::new(func_ptr);
    println!("从函数指针创建的闭包结果: {}", closure_from_fn(5));

    // 展示类型系统的差异
    demonstrate_type_system_differences();
}

/// 展示类型系统的差异
fn demonstrate_type_system_differences() {
    println!("\n--- 类型系统差异 ---");

    // 每个闭包都有唯一的类型
    let closure1 = |x: i32| x + 1;
    let closure2 = |x: i32| x + 1; // 即使逻辑相同，类型也不同

    // 这会编译失败（类型不匹配）：
    // let same_type = if true { closure1 } else { closure2 };

    // 但可以通过 trait object 统一类型
    let boxed1: Box<dyn Fn(i32) -> i32> = Box::new(closure1);
    let boxed2: Box<dyn Fn(i32) -> i32> = Box::new(closure2);

    let dynamic_closure = if true { boxed1 } else { boxed2 };
    println!("动态分发闭包结果: {}", dynamic_closure(10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_closure() {
        let add = |x, y| x + y;
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_type_inference() {
        let inferred = |x| x + 1;
        assert_eq!(inferred(5), 6);
    }

    #[test]
    fn test_multi_line_closure() {
        let complex = |x: i32| {
            let temp = x * 2;
            temp + 10
        };
        assert_eq!(complex(5), 20);
    }
}

