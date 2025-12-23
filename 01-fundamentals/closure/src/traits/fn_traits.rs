//! # Fn、FnMut、FnOnce Trait 详解

/// 演示 Fn trait
/// 
/// Fn 是最严格的闭包 trait，可以多次调用且不修改环境
pub fn demo_fn_trait() {
    println!("\n=== Fn Trait 示例 ===");

    let message = String::from("Hello, Rust!");

    let print_message = || {
        println!("消息: {}", message);
        message.len()
    };

    println!("消息长度: {}", print_message());
    println!("消息长度: {}", print_message());
    println!("原始消息仍可用: {}", message);

    // 使用 Fn 作为参数
    demo_fn_as_parameter();
}

fn demo_fn_as_parameter() {
    fn apply_twice<F>(f: F, x: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(f(x))
    }

    let double = |x| x * 2;
    let result = apply_twice(double, 5);
    println!("应用两次 double(5): {}", result);
}

/// 演示 FnMut trait
///
/// FnMut 可以多次调用，但可以修改捕获的变量
pub fn demo_fn_mut_trait() {
    println!("\n=== FnMut Trait 示例 ===");

    let mut count = 0;

    let mut increment = || {
        count += 1;
        println!("当前计数: {}", count);
        count
    };

    increment();
    increment();
    increment();

    // 使用 FnMut 作为参数
    demo_fn_mut_as_parameter();
}

fn demo_fn_mut_as_parameter() {
    fn call_multiple_times<F>(mut f: F)
    where
        F: FnMut(i32) -> i32,
    {
        for i in 1..=3 {
            let result = f(i);
            println!("调用 {} 次，结果: {}", i, result);
        }
    }

    let mut accumulator = 0;
    let accumulate = |x| {
        accumulator += x;
        accumulator
    };

    call_multiple_times(accumulate);
}

/// 演示 FnOnce trait
///
/// FnOnce 只能调用一次，因为它会消费捕获的变量
pub fn demo_fn_once_trait() {
    println!("\n=== FnOnce Trait 示例 ===");

    let s = String::from("hello");

    let consume_string = || {
        println!("消费字符串: {}", s);
        drop(s);
    };

    consume_string();
    // consume_string(); // 编译错误：已经被消费

    // 使用 FnOnce 作为参数
    demo_fn_once_as_parameter();
}

fn demo_fn_once_as_parameter() {
    fn call_once<F>(f: F) -> String
    where
        F: FnOnce() -> String,
    {
        f()
    }

    let message = String::from("Hello, World!");
    let closure = || message;

    let result = call_once(closure);
    println!("FnOnce 调用结果: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_trait() {
        let x = 10;
        let closure = || x + 1;
        assert_eq!(closure(), 11);
        assert_eq!(closure(), 11);
    }

    #[test]
    fn test_fn_mut_trait() {
        let mut count = 0;
        let mut increment = || {
            count += 1;
            count
        };
        assert_eq!(increment(), 1);
        assert_eq!(increment(), 2);
    }

    #[test]
    fn test_fn_once_trait() {
        let data = vec![1, 2, 3];
        let consume = || data;
        let result = consume();
        assert_eq!(result, vec![1, 2, 3]);
    }
}

