//! # 闭包性能优化示例
//!
//! 这个示例展示了如何优化闭包的性能，包括内联、避免不必要的分配等技巧。

use closure_tutorial::performance;
use std::hint::black_box;
use std::time::{Duration, Instant};

fn main() {
    println!("⚡ 闭包性能优化示例");
    println!("{}", "=".repeat(50));

    // 1. 闭包 vs 函数性能对比
    demonstrate_closure_vs_function();

    // 2. 内联优化
    demonstrate_inlining();

    // 3. 捕获开销分析
    demonstrate_capture_overhead();

    // 4. 内存分配优化
    demonstrate_allocation_optimization();

    // 5. 迭代器性能
    demonstrate_iterator_performance();

    // 6. 运行库中的性能演示
    println!("\n📚 库演示");
    performance::demonstrate();

    println!("\n✅ 性能优化示例完成！");
}

/// 演示闭包与函数的性能对比
fn demonstrate_closure_vs_function() {
    println!("\n🏃 1. 闭包 vs 函数性能对比");

    const ITERATIONS: usize = 10_000_000;

    // 普通函数
    fn add_function(a: i32, b: i32) -> i32 {
        a + b
    }

    // 闭包
    let add_closure = |a: i32, b: i32| a + b;

    // 测试函数性能
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..ITERATIONS {
        sum = sum.wrapping_add(add_function(i as i32, 1) as i64);
    }
    let function_time = start.elapsed();
    black_box(sum); // 防止编译器优化掉计算

    // 测试闭包性能
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..ITERATIONS {
        sum = sum.wrapping_add(add_closure(i as i32, 1) as i64);
    }
    let closure_time = start.elapsed();
    black_box(sum);

    println!("函数调用时间: {:?}", function_time);
    println!("闭包调用时间: {:?}", closure_time);
    println!(
        "性能差异: {:.2}%",
        (closure_time.as_nanos() as f64 / function_time.as_nanos() as f64 - 1.0) * 100.0
    );
}

/// 演示内联优化
fn demonstrate_inlining() {
    println!("\n🎯 2. 内联优化");

    const ITERATIONS: usize = 1_000_000;

    // 简单闭包（容易内联）
    let simple_closure = |x: i32| x * 2;

    // 复杂闭包（不容易内联）
    let complex_closure = |x: i32| {
        let mut result = x;
        for _ in 0..10 {
            result = result * 2 + 1;
            result = result / 2;
        }
        result
    };

    // 测试简单闭包
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..ITERATIONS {
        sum = sum.wrapping_add(simple_closure(i as i32) as i64);
    }
    let simple_time = start.elapsed();
    black_box(sum);

    // 测试复杂闭包
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..ITERATIONS {
        sum = sum.wrapping_add(complex_closure(i as i32) as i64);
    }
    let complex_time = start.elapsed();
    black_box(sum);

    println!("简单闭包时间: {:?}", simple_time);
    println!("复杂闭包时间: {:?}", complex_time);
    println!(
        "复杂度影响: {:.2}x",
        complex_time.as_nanos() as f64 / simple_time.as_nanos() as f64
    );
}

/// 演示捕获开销分析
fn demonstrate_capture_overhead() {
    println!("\n📦 3. 捕获开销分析");

    const ITERATIONS: usize = 1_000_000;

    // 无捕获闭包
    let no_capture = |x: i32| x + 1;

    // 按值捕获
    let value = 42;
    let capture_by_value = move |x: i32| x + value;

    // 按引用捕获
    let reference_value = 42;
    let capture_by_ref = |x: i32| x + reference_value;

    // 捕获大型数据结构
    let large_data = vec![1; 1000];
    let capture_large = move |x: i32| x + large_data[0];

    // 测试无捕获
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..ITERATIONS {
        sum = sum.wrapping_add(no_capture(i as i32) as i64);
    }
    let no_capture_time = start.elapsed();
    black_box(sum);

    // 测试按值捕获
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..ITERATIONS {
        sum = sum.wrapping_add(capture_by_value(i as i32) as i64);
    }
    let value_capture_time = start.elapsed();
    black_box(sum);

    // 测试按引用捕获
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..ITERATIONS {
        sum = sum.wrapping_add(capture_by_ref(i as i32) as i64);
    }
    let ref_capture_time = start.elapsed();
    black_box(sum);

    // 测试大型数据捕获
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..ITERATIONS {
        sum = sum.wrapping_add(capture_large(i as i32) as i64);
    }
    let large_capture_time = start.elapsed();
    black_box(sum);

    println!("无捕获时间: {:?}", no_capture_time);
    println!("按值捕获时间: {:?}", value_capture_time);
    println!("按引用捕获时间: {:?}", ref_capture_time);
    println!("大型数据捕获时间: {:?}", large_capture_time);
}

/// 演示内存分配优化
fn demonstrate_allocation_optimization() {
    println!("\n💾 4. 内存分配优化");

    const ITERATIONS: usize = 100_000;

    // 避免不必要的分配 - 坏例子
    let bad_closure = |data: &[i32]| {
        let mut result = Vec::new();
        for &item in data {
            result.push(item * 2);
        }
        result
    };

    // 重用缓冲区 - 好例子
    let good_closure = |data: &[i32], buffer: &mut Vec<i32>| {
        buffer.clear();
        buffer.reserve(data.len());
        for &item in data {
            buffer.push(item * 2);
        }
    };

    let test_data = vec![1, 2, 3, 4, 5];

    // 测试坏例子（每次分配新内存）
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let result = bad_closure(&test_data);
        black_box(result);
    }
    let bad_time = start.elapsed();

    // 测试好例子（重用内存）
    let start = Instant::now();
    let mut buffer = Vec::new();
    for _ in 0..ITERATIONS {
        good_closure(&test_data, &mut buffer);
        black_box(&buffer);
    }
    let good_time = start.elapsed();

    println!("频繁分配时间: {:?}", bad_time);
    println!("重用缓冲区时间: {:?}", good_time);
    println!(
        "性能提升: {:.2}x",
        bad_time.as_nanos() as f64 / good_time.as_nanos() as f64
    );
}

/// 演示迭代器性能
fn demonstrate_iterator_performance() {
    println!("\n🔄 5. 迭代器性能");

    let data: Vec<i32> = (0..1_000_000).collect();

    // 传统循环
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..data.len() {
        if data[i] % 2 == 0 {
            sum = sum.wrapping_add((data[i] as i64) * (data[i] as i64));
        }
    }
    let loop_time = start.elapsed();
    let loop_result = sum;

    // 迭代器链式调用
    let start = Instant::now();
    let iterator_result: i64 = data
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| (x as i64) * (x as i64))
        .sum();
    let iterator_time = start.elapsed();

    // 并行迭代器（需要 rayon crate，这里模拟）
    let start = Instant::now();
    let chunks: Vec<_> = data.chunks(data.len() / 4).collect();
    let parallel_result: i64 = chunks
        .iter()
        .map(|chunk| {
            chunk
                .iter()
                .filter(|&&x| x % 2 == 0)
                .map(|&x| (x as i64) * (x as i64))
                .sum::<i64>()
        })
        .sum();
    let parallel_time = start.elapsed();

    println!("传统循环时间: {:?}, 结果: {}", loop_time, loop_result);
    println!("迭代器时间: {:?}, 结果: {}", iterator_time, iterator_result);
    println!(
        "模拟并行时间: {:?}, 结果: {}",
        parallel_time, parallel_result
    );

    // 验证结果一致性
    assert_eq!(loop_result, iterator_result);
    assert_eq!(iterator_result, parallel_result);

    println!(
        "迭代器 vs 循环: {:.2}x",
        loop_time.as_nanos() as f64 / iterator_time.as_nanos() as f64
    );
}

/// 性能测试辅助函数
fn benchmark<F, R>(name: &str, iterations: usize, mut f: F) -> (Duration, R)
where
    F: FnMut() -> R,
{
    let start = Instant::now();
    let mut result = None;

    for _ in 0..iterations {
        result = Some(f());
    }

    let duration = start.elapsed();
    println!("{}: {:?} ({} 次迭代)", name, duration, iterations);

    (duration, result.unwrap())
}

/// 演示不同闭包类型的性能
fn demonstrate_closure_types_performance() {
    println!("\n🎭 闭包类型性能对比");

    const ITERATIONS: usize = 1_000_000;

    // Fn 闭包
    let fn_closure = |x: i32| x + 1;

    // FnMut 闭包
    let mut counter = 0;
    let mut fn_mut_closure = |x: i32| {
        counter += 1;
        x + counter
    };

    // FnOnce 闭包（通过 Box 模拟多次调用）
    let create_fn_once = || {
        let data = vec![1, 2, 3];
        move |x: i32| x + data[0]
    };

    // 测试 Fn
    let (fn_time, _) = benchmark("Fn 闭包", ITERATIONS, || black_box(fn_closure(42)));

    // 测试 FnMut
    let (fn_mut_time, _) = benchmark("FnMut 闭包", ITERATIONS, || black_box(fn_mut_closure(42)));

    // 测试 FnOnce（创建开销）
    let (fn_once_time, _) = benchmark("FnOnce 创建", ITERATIONS / 1000, || {
        let closure = create_fn_once();
        black_box(closure(42))
    });

    println!("性能排序: Fn < FnMut < FnOnce(创建)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_performance() {
        let add_closure = |a: i32, b: i32| a + b;
        assert_eq!(add_closure(2, 3), 5);
    }

    #[test]
    fn test_capture_performance() {
        let value = 42;
        let capture_closure = |x: i32| x + value;
        assert_eq!(capture_closure(8), 50);
    }

    #[test]
    fn test_iterator_performance() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let result: i32 = data.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x).sum();
        assert_eq!(result, 56); // 2² + 4² + 6² = 4 + 16 + 36 = 56
    }

    #[test]
    fn test_benchmark_helper() {
        let (duration, result) = benchmark("测试", 100, || 42);
        assert_eq!(result, 42);
        assert!(duration > Duration::from_nanos(0));
    }
}
