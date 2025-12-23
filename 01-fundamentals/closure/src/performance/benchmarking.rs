//! # 性能基准测试

use std::time::Instant;

/// 演示性能比较
pub fn demo_performance_comparison() {
    println!("\n=== 性能比较 ===");

    let iterations = 1_000_000;

    // 普通函数
    let start = Instant::now();
    for i in 0..iterations {
        let _ = regular_function(i);
    }
    let function_time = start.elapsed();

    // 闭包
    let closure = |x: i32| x * 2 + 1;
    let start = Instant::now();
    for i in 0..iterations {
        let _ = closure(i);
    }
    let closure_time = start.elapsed();

    println!("函数调用时间: {:?}", function_time);
    println!("闭包调用时间: {:?}", closure_time);
    println!("结论: 闭包的性能与普通函数相当（零成本抽象）");
}

fn regular_function(x: i32) -> i32 {
    x * 2 + 1
}

