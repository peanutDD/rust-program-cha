//! # 静态分发与动态分发

/// 演示静态分发 vs 动态分发
pub fn demo_static_vs_dynamic_dispatch() {
    println!("\n=== 静态分发 vs 动态分发 ===");

    // 静态分发示例
    demo_static_dispatch();

    // 动态分发示例
    demo_dynamic_dispatch();
}

fn demo_static_dispatch() {
    println!("\n--- 静态分发 (编译时确定) ---");

    let closure1 = |x: i32| x + 1;
    let closure2 = |x: i32| x * 2;

    let result1 = static_call(closure1, 5);
    let result2 = static_call(closure2, 5);

    println!("静态分发结果1: {}", result1);
    println!("静态分发结果2: {}", result2);
}

fn static_call<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x) // 编译时确定，可能被内联
}

fn demo_dynamic_dispatch() {
    println!("\n--- 动态分发 (运行时确定) ---");

    let closures: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 2),
        Box::new(|x| x * x),
    ];

    for (i, closure) in closures.iter().enumerate() {
        let result = dynamic_call(closure.as_ref(), 5);
        println!("动态分发闭包 {}: {}", i, result);
    }
}

fn dynamic_call(f: &dyn Fn(i32) -> i32, x: i32) -> i32 {
    f(x) // 运行时通过虚表调用
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_dispatch() {
        let closure = |x| x + 1;
        assert_eq!(static_call(closure, 5), 6);
    }

    #[test]
    fn test_dynamic_dispatch() {
        let closure: Box<dyn Fn(i32) -> i32> = Box::new(|x| x * 2);
        assert_eq!(dynamic_call(closure.as_ref(), 5), 10);
    }
}

