//! # 闭包类型系统深度分析
//!
//! 揭示闭包在 Rust 类型系统中的工作原理

/// 演示类型系统分析
pub fn demo_type_system() {
    println!("\n=== 闭包类型系统深度分析 ===");

    demo_unique_types();
    demo_type_size();
    demo_type_coercion();
    demo_impl_trait_return();
}

/// 每个闭包都是唯一类型
fn demo_unique_types() {
    println!("\n--- 1. 闭包的唯一类型 ---");

    let closure1 = |x: i32| x + 1;
    let closure2 = |x: i32| x + 1;

    println!("两个闭包即使代码相同，类型也不同");
    println!("closure1 和 closure2 是不同的类型");

    // 无法统一类型
    // let closure = if true { closure1 } else { closure2 }; // 错误

    // 解决方案：使用 trait object
    let boxed1: Box<dyn Fn(i32) -> i32> = Box::new(closure1);
    let boxed2: Box<dyn Fn(i32) -> i32> = Box::new(closure2);
    
    let dynamic = if true { boxed1 } else { boxed2 };
    println!("动态分发结果: {}", dynamic(5));
}

/// 闭包的类型大小
fn demo_type_size() {
    println!("\n--- 2. 闭包的类型大小 ---");

    // 不捕获变量的闭包大小为 0
    let no_capture = |x: i32| x * 2;
    println!("无捕获闭包: {} bytes", std::mem::size_of_val(&no_capture));

    // 捕获一个 i32
    let one_i32 = 42;
    let capture_i32 = move |x: i32| x + one_i32;
    println!("捕获i32: {} bytes", std::mem::size_of_val(&capture_i32));

    // 捕获一个引用
    let data = vec![1, 2, 3];
    let capture_ref = || data.len();
    println!("捕获引用: {} bytes", std::mem::size_of_val(&capture_ref));

    // 捕获多个变量
    let a = 1i32;
    let b = 2i64;
    let c = 3u8;
    let capture_multi = move || (a, b, c);
    println!("捕获多个: {} bytes", std::mem::size_of_val(&capture_multi));
}

/// 类型强制转换
fn demo_type_coercion() {
    println!("\n--- 3. 类型强制转换 ---");

    // 闭包可以强制转换为函数指针（如果不捕获变量）
    let no_capture = |x: i32| x * 2;
    let fn_ptr: fn(i32) -> i32 = no_capture;
    println!("函数指针调用: {}", fn_ptr(5));

    // 捕获变量的闭包不能转换为函数指针
    let multiplier = 3;
    let _with_capture = |x: i32| x * multiplier;
    // let fn_ptr: fn(i32) -> i32 = _with_capture; // 错误

    println!("💡 只有不捕获变量的闭包才能转换为函数指针");
}

/// impl Trait 返回类型
fn demo_impl_trait_return() {
    println!("\n--- 4. impl Trait 返回类型 ---");

    fn create_adder(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n
    }

    let add_5 = create_adder(5);
    println!("add_5(10) = {}", add_5(10));

    println!("💡 impl Trait 隐藏了具体的闭包类型");
    println!("调用者只知道它实现了 Fn(i32) -> i32");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_closure_size() {
        let no_capture = |x: i32| x;
        assert_eq!(std::mem::size_of_val(&no_capture), 0);

        let data = 42;
        let with_capture = move || data;
        assert!(std::mem::size_of_val(&with_capture) > 0);
    }
}

