//! # 闭包的类型推导和类型注解
//!
//! 本模块详细介绍 Rust 闭包的类型系统，包括：
//! - 闭包的类型推导机制
//! - 显式类型注解
//! - 闭包类型的唯一性
//! - 类型推导的限制和规则
//! - 实际应用中的类型处理

/// 演示闭包的类型推导和类型注解
pub fn demo_type_system() {
    println!("\n🔍 闭包的类型推导和类型注解");
    println!("{}", "-".repeat(40));

    type_inference_basics();
    explicit_type_annotations();
    closure_type_uniqueness();
    type_inference_limitations();
    practical_type_handling();
}

/// 演示类型推导基础
fn type_inference_basics() {
    println!("\n📝 类型推导基础:");

    // 1. 基本类型推导
    let simple_closure = |x| x + 1;

    // 第一次使用确定类型
    let result1 = simple_closure(5i32); // 推导为 |i32| -> i32
    println!("simple_closure(5i32) = {}", result1);

    // 后续使用必须保持一致
    let result2 = simple_closure(10i32);
    println!("simple_closure(10i32) = {}", result2);

    // 以下代码会编译错误，因为类型已经确定为 i32
    // let result3 = simple_closure(3.14f64);  // 编译错误！

    // 2. 从上下文推导类型
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("从上下文推导: {:?} -> {:?}", numbers, doubled);

    // 3. 返回值类型推导
    let calculate = |a, b| {
        if a > b {
            a - b
        } else {
            b - a
        }
    };

    let diff = calculate(10, 7); // 推导为 i32
    println!("calculate(10, 7) = {}", diff);

    demonstrate_inference_with_generics();
}

/// 演示泛型中的类型推导
fn demonstrate_inference_with_generics() {
    println!("\n📝 泛型中的类型推导:");

    // 泛型函数中的闭包类型推导
    fn process_with_closure<T, F>(value: T, processor: F) -> T
    where
        F: Fn(T) -> T,
        T: std::fmt::Display + Copy,
    {
        println!("处理前: {}", value);
        let result = processor(value);
        println!("处理后: {}", result);
        result
    }

    // 整数处理
    let int_result = process_with_closure(42, |x| x * 2);
    println!("整数处理结果: {}", int_result);

    // 浮点数处理
    let float_result = process_with_closure(3.14, |x| x * x);
    println!("浮点数处理结果: {}", float_result);
}

/// 演示显式类型注解
fn explicit_type_annotations() {
    println!("\n📝 显式类型注解:");

    // 1. 参数类型注解
    let typed_closure =
        |x: i32, y: f64| -> String { format!("整数: {}, 浮点数: {:.2}", x, y) };

    let result = typed_closure(42, 3.14159);
    println!("显式类型注解结果: {}", result);

    // 2. 复杂类型注解
    let complex_closure: fn(i32) -> i32 = |x| x * 2;
    println!("复杂类型注解: {}", complex_closure(21));
}

/// 演示闭包类型的唯一性
fn closure_type_uniqueness() {
    println!("\n📝 闭包类型的唯一性:");

    // 每个闭包都有唯一的类型，即使功能相同
    let closure1 = |x| x + 1;
    let closure2 = |x| x + 1;

    // closure1 和 closure2 的类型不同，即使功能相同
    println!("每个闭包都有唯一的类型，即使功能相同");
    println!("closure1(5) = {}", closure1(5));
    println!("closure2(5) = {}", closure2(5));
}

/// 演示类型推导的限制
fn type_inference_limitations() {
    println!("\n📝 类型推导的限制:");

    // 1. 无法推导的情况需要显式注解
    let numbers = vec![1, 2, 3];
    let _doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    // 如果没有类型注解，collect() 无法确定目标类型

    println!("某些情况下需要显式类型注解");
}

/// 演示实际应用中的类型处理
fn practical_type_handling() {
    println!("\n📝 实际应用中的类型处理:");

    // 使用 trait 对象处理不同类型的闭包
    let closures: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 2),
        Box::new(|x| x * x),
    ];

    for (i, closure) in closures.iter().enumerate() {
        println!("闭包 {}: closure(5) = {}", i + 1, closure(5));
    }
}

