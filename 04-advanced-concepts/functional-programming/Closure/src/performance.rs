//! # 闭包的性能考虑
//!
//! 本模块详细分析闭包的性能特征，包括：
//! - 闭包与函数的性能对比
//! - 内联优化
//! - 捕获开销分析
//! - 内存使用模式
//! - 性能优化技巧

use std::hint::black_box;
use std::time::{Duration, Instant};

/// 演示闭包的性能考虑
pub fn demonstrate() {
    println!("\n⚡ 7. 闭包的性能考虑");
    println!("{}", "-".repeat(40));

    performance_comparison();
    inline_optimization();
    capture_overhead_analysis();
    memory_usage_patterns();
    optimization_techniques();
}

/// 性能对比测试
fn performance_comparison() {
    println!("\n📊 性能对比测试:");

    const ITERATIONS: usize = 10_000_000;

    // 普通函数
    fn regular_function(x: i64) -> i64 {
        x * 2 + 1
    }

    // 函数指针
    let function_pointer: fn(i64) -> i64 = |x| x * 2 + 1;

    // 闭包（无捕获）
    let closure_no_capture = |x: i64| x * 2 + 1;

    // 闭包（有捕获）
    let multiplier = 2;
    let addend = 1;
    let closure_with_capture = |x: i64| x * multiplier + addend;

    println!("执行 {} 次迭代的性能测试:", ITERATIONS);

    // 测试普通函数
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..ITERATIONS {
        sum = sum.wrapping_add(regular_function(i as i64));
    }
    let duration = start.elapsed();
    println!("普通函数: {:?}, 结果: {}", duration, sum);

    // 测试函数指针
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..ITERATIONS {
        sum = sum.wrapping_add(function_pointer(i as i64));
    }
    let duration = start.elapsed();
    println!("函数指针: {:?}, 结果: {}", duration, sum);

    // 测试无捕获闭包
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..ITERATIONS {
        sum = sum.wrapping_add(closure_no_capture(i as i64));
    }
    let duration = start.elapsed();
    println!("无捕获闭包: {:?}, 结果: {}", duration, sum);

    // 测试有捕获闭包
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..ITERATIONS {
        sum = sum.wrapping_add(closure_with_capture(i as i64));
    }
    let duration = start.elapsed();
    println!("有捕获闭包: {:?}, 结果: {}", duration, sum);

    println!("\n💡 性能分析:");
    println!("- 在优化编译下，简单闭包通常会被内联，性能接近普通函数");
    println!("- 函数指针由于间接调用，可能略慢于直接函数调用");
    println!("- 捕获变量的闭包可能有额外的内存访问开销");
}

/// 内联优化演示
fn inline_optimization() {
    println!("\n🚀 内联优化演示:");

    const ITERATIONS: usize = 1_000_000;

    // 简单计算闭包（容易内联）
    let simple_closure = |x: f64| x * x + 2.0 * x + 1.0;

    // 复杂计算闭包（可能不会内联）
    let complex_closure = |x: f64| {
        let mut result = x;
        for _ in 0..10 {
            result = result.sin().cos().tan().abs();
        }
        result
    };

    println!("测试内联优化效果 ({} 次迭代):", ITERATIONS);

    // 测试简单闭包
    let start = Instant::now();
    let mut sum = 0.0;
    for i in 0..ITERATIONS {
        sum += simple_closure(i as f64);
    }
    let simple_duration = start.elapsed();
    println!("简单闭包: {:?}, 结果: {:.2}", simple_duration, sum);

    // 测试复杂闭包
    let start = Instant::now();
    let mut sum = 0.0;
    for i in 0..ITERATIONS {
        sum += complex_closure(i as f64 * 0.001); // 缩小输入避免数值问题
    }
    let complex_duration = start.elapsed();
    println!("复杂闭包: {:?}, 结果: {:.2}", complex_duration, sum);

    // 高阶函数内联测试
    demonstrate_higher_order_inlining();
}

/// 演示高阶函数内联
fn demonstrate_higher_order_inlining() {
    println!("\n📝 高阶函数内联测试:");

    const SIZE: usize = 1_000_000;
    let data: Vec<i32> = (0..SIZE as i32).collect();

    // 使用迭代器链（容易内联）
    let start = Instant::now();
    let sum1: i64 = data
        .iter()
        .map(|&x| x as i64)
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .sum();
    let iterator_duration = start.elapsed();

    // 使用传统循环
    let start = Instant::now();
    let mut sum2 = 0i64;
    for &x in &data {
        let x = x as i64;
        if x % 2 == 0 {
            sum2 += x * x;
        }
    }
    let loop_duration = start.elapsed();

    println!("迭代器链: {:?}, 结果: {}", iterator_duration, sum1);
    println!("传统循环: {:?}, 结果: {}", loop_duration, sum2);
    println!("结果一致: {}", sum1 == sum2);

    println!("\n💡 内联分析:");
    println!("- 编译器通常能很好地内联简单的迭代器操作");
    println!("- 复杂的闭包可能阻止内联优化");
    println!("- 使用 #[inline] 属性可以提示编译器进行内联");
}

/// 捕获开销分析
fn capture_overhead_analysis() {
    println!("\n🔍 捕获开销分析:");

    const ITERATIONS: usize = 5_000_000;

    // 测试不同捕获方式的开销
    let value1 = 42i64;
    let value2 = 24i64;
    let mut mutable_value = 0i64;

    // 无捕获
    let no_capture = |x: i64| x + 1;

    // 不可变借用捕获
    let immutable_capture = |x: i64| x + value1;

    // 可变借用捕获
    let mut mutable_capture = |x: i64| {
        mutable_value += 1;
        x + mutable_value
    };

    // 所有权捕获
    let owned_values = vec![value1, value2];
    let move_capture = move |x: i64| x + owned_values[0] + owned_values[1];

    println!("捕获开销测试 ({} 次迭代):", ITERATIONS);

    // 测试无捕获
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..ITERATIONS {
        sum = sum.wrapping_add(no_capture(i as i64));
    }
    let no_capture_duration = start.elapsed();
    println!("无捕获: {:?}", no_capture_duration);

    // 测试不可变借用捕获
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..ITERATIONS {
        sum = sum.wrapping_add(immutable_capture(i as i64));
    }
    let immutable_duration = start.elapsed();
    println!("不可变借用: {:?}", immutable_duration);

    // 重置可变值 - 注释掉以避免借用冲突
    // mutable_value = 0;

    // 测试可变借用捕获
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..ITERATIONS {
        sum = sum.wrapping_add(mutable_capture(i as i64));
    }
    let mutable_duration = start.elapsed();
    println!("可变借用: {:?}", mutable_duration);

    // 测试所有权捕获
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..ITERATIONS {
        sum = sum.wrapping_add(move_capture(i as i64));
    }
    let move_duration = start.elapsed();
    println!("所有权捕获: {:?}", move_duration);

    analyze_capture_costs();
}

/// 分析捕获成本
fn analyze_capture_costs() {
    println!("\n📊 捕获成本分析:");

    // 大型数据结构捕获测试
    let large_vec: Vec<i32> = (0..10000).collect();
    let large_string = "A".repeat(10000);

    println!("大型数据结构捕获测试:");

    // 借用捕获（零成本）
    let borrow_closure = || large_vec.len() + large_string.len();

    // 克隆捕获（高成本）
    let clone_vec = large_vec.clone();
    let clone_string = large_string.clone();
    let clone_closure = move || clone_vec.len() + clone_string.len();

    // 测试借用捕获
    let start = Instant::now();
    let result1 = borrow_closure();
    let borrow_time = start.elapsed();

    // 测试移动捕获
    let start = Instant::now();
    let result2 = clone_closure();
    let move_time = start.elapsed();

    println!("借用捕获: {:?}, 结果: {}", borrow_time, result1);
    println!("移动捕获: {:?}, 结果: {}", move_time, result2);

    println!("\n💡 捕获成本总结:");
    println!("- 不可变借用: 零运行时成本，但有生命周期限制");
    println!("- 可变借用: 可能有同步开销，特别是在多线程环境");
    println!("- 所有权转移: 可能涉及内存复制，但没有生命周期限制");
    println!("- 大型数据结构应优先考虑借用而非移动");
}

/// 内存使用模式分析
fn memory_usage_patterns() {
    println!("\n💾 内存使用模式分析:");

    // 闭包大小分析
    analyze_closure_sizes();

    // 堆分配模式
    analyze_heap_allocation_patterns();

    // 内存布局优化
    demonstrate_memory_layout_optimization();
}

/// 分析闭包大小
fn analyze_closure_sizes() {
    println!("\n📏 闭包大小分析:");

    // 无捕获闭包
    let no_capture = || 42;

    // 捕获单个值
    let value = 42i32;
    let single_capture = move || value;

    // 捕获多个值
    let a = 1i32;
    let b = 2i64;
    let c = 3.0f64;
    let multiple_capture = move || a + b as i32 + c as i32;

    // 捕获大型结构
    let large_array = [0u8; 1024];
    let large_capture = move || large_array[0];

    println!("闭包内存大小:");
    println!("无捕获闭包: {} 字节", std::mem::size_of_val(&no_capture));
    println!("单值捕获: {} 字节", std::mem::size_of_val(&single_capture));
    println!(
        "多值捕获: {} 字节",
        std::mem::size_of_val(&multiple_capture)
    );
    println!("大型捕获: {} 字节", std::mem::size_of_val(&large_capture));

    println!("\n💡 大小分析:");
    println!("- 无捕获闭包通常为零大小类型 (ZST)");
    println!("- 闭包大小等于所有捕获变量的大小之和");
    println!("- 大型捕获会显著增加闭包的内存占用");
}

/// 分析堆分配模式
fn analyze_heap_allocation_patterns() {
    println!("\n🏗️ 堆分配模式分析:");

    // 栈上闭包（推荐）
    fn use_stack_closure<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(42)
    }

    // 堆上闭包（避免过度使用）
    fn use_boxed_closure(f: Box<dyn Fn(i32) -> i32>) -> i32 {
        f(42)
    }

    let multiplier = 2;

    // 栈上使用
    let result1 = use_stack_closure(|x| x * multiplier);
    println!("栈上闭包结果: {}", result1);

    // 堆上使用
    let result2 = use_boxed_closure(Box::new(move |x| x * multiplier));
    println!("堆上闭包结果: {}", result2);

    println!("\n💡 分配模式建议:");
    println!("- 优先使用泛型参数而非 Box<dyn Fn>");
    println!("- 避免不必要的堆分配");
    println!("- 考虑使用 impl Trait 语法");
}

/// 演示内存布局优化
fn demonstrate_memory_layout_optimization() {
    println!("\n🎯 内存布局优化:");

    // 未优化的结构
    #[derive(Clone)]
    struct UnoptimizedData {
        flag: bool,         // 1 字节
        value: i64,         // 8 字节
        small: i16,         // 2 字节
        another_flag: bool, // 1 字节
    }

    // 优化的结构
    #[derive(Clone)]
    struct OptimizedData {
        value: i64,         // 8 字节
        small: i16,         // 2 字节
        flag: bool,         // 1 字节
        another_flag: bool, // 1 字节
    }

    let unopt_data = UnoptimizedData {
        flag: true,
        value: 42,
        small: 10,
        another_flag: false,
    };

    let opt_data = OptimizedData {
        value: 42,
        small: 10,
        flag: true,
        another_flag: false,
    };

    // 创建捕获这些结构的闭包
    let unopt_closure = move || unopt_data.value + unopt_data.small as i64;
    let opt_closure = move || opt_data.value + opt_data.small as i64;

    println!("结构体大小对比:");
    println!(
        "未优化结构: {} 字节",
        std::mem::size_of::<UnoptimizedData>()
    );
    println!("优化结构: {} 字节", std::mem::size_of::<OptimizedData>());
    println!("未优化闭包: {} 字节", std::mem::size_of_val(&unopt_closure));
    println!("优化闭包: {} 字节", std::mem::size_of_val(&opt_closure));

    println!("\n💡 布局优化建议:");
    println!("- 按大小降序排列结构体字段");
    println!("- 避免捕获不必要的大型数据");
    println!("- 考虑使用引用而非所有权转移");
}

/// 性能优化技巧
fn optimization_techniques() {
    println!("\n🚀 性能优化技巧:");

    // 1. 避免不必要的分配
    demonstrate_allocation_avoidance();

    // 2. 使用适当的捕获方式
    demonstrate_capture_optimization();

    // 3. 编译器优化提示
    demonstrate_compiler_hints();

    // 4. 批量处理优化
    demonstrate_batch_processing();
}

/// 演示避免分配
fn demonstrate_allocation_avoidance() {
    println!("\n💡 避免不必要的分配:");

    const SIZE: usize = 100000;
    let data: Vec<i32> = (0..SIZE as i32).collect();

    // 低效：每次都创建新的 Vec
    let start = Instant::now();
    let _results: Vec<Vec<i32>> = (0..100)
        .map(|offset| data.iter().skip(offset).take(1000).cloned().collect())
        .collect();
    let inefficient_time = start.elapsed();

    // 高效：重用缓冲区
    let start = Instant::now();
    let mut buffer = Vec::with_capacity(1000);
    let mut _results: Vec<i32> = Vec::new();
    for offset in 0..100 {
        buffer.clear();
        buffer.extend(data.iter().skip(offset).take(1000).cloned());
        // 在实际应用中，这里会处理 buffer
    }
    let efficient_time = start.elapsed();

    println!("低效方式: {:?}", inefficient_time);
    println!("高效方式: {:?}", efficient_time);
    println!(
        "性能提升: {:.2}x",
        inefficient_time.as_nanos() as f64 / efficient_time.as_nanos() as f64
    );
}

/// 演示捕获优化
fn demonstrate_capture_optimization() {
    println!("\n🎯 捕获方式优化:");

    #[derive(Clone)]
    struct LargeStruct {
        data: Vec<i32>,
        metadata: String,
        flags: [bool; 100],
    }

    impl LargeStruct {
        fn new() -> Self {
            LargeStruct {
                data: (0..10000).collect(),
                metadata: "Large metadata string".repeat(100),
                flags: [false; 100],
            }
        }

        fn get_sum(&self) -> i32 {
            self.data.iter().sum()
        }
    }

    let large_struct = LargeStruct::new();

    // 低效：捕获整个结构
    let inefficient_closure = {
        let large_struct = large_struct.clone(); // 昂贵的克隆
        move || large_struct.get_sum()
    };

    // 高效：只捕获需要的部分
    let sum = large_struct.get_sum();
    let efficient_closure = move || sum;

    println!(
        "低效闭包大小: {} 字节",
        std::mem::size_of_val(&inefficient_closure)
    );
    println!(
        "高效闭包大小: {} 字节",
        std::mem::size_of_val(&efficient_closure)
    );

    // 性能测试
    let start = Instant::now();
    for _ in 0..10000 {
        black_box(inefficient_closure());
    }
    let inefficient_time = start.elapsed();

    let start = Instant::now();
    for _ in 0..10000 {
        black_box(efficient_closure());
    }
    let efficient_time = start.elapsed();

    println!("低效方式: {:?}", inefficient_time);
    println!("高效方式: {:?}", efficient_time);
}

/// 演示编译器优化提示
fn demonstrate_compiler_hints() {
    println!("\n🔧 编译器优化提示:");

    // 使用 #[inline] 提示
    #[inline(always)]
    fn always_inline_function(x: i32) -> i32 {
        x * 2 + 1
    }

    #[inline(never)]
    fn never_inline_function(x: i32) -> i32 {
        x * 2 + 1
    }

    // 使用 likely/unlikely 提示（需要 nightly）
    fn with_branch_hints(x: i32) -> i32 {
        if x > 0 {
            // 假设这个分支更可能被执行
            x * 2
        } else {
            // 假设这个分支不太可能被执行
            x * 3
        }
    }

    const ITERATIONS: usize = 1000000;

    // 测试内联函数
    let start = Instant::now();
    let mut _sum = 0i64;
    for i in 0..ITERATIONS {
        _sum = _sum.wrapping_add(always_inline_function(i as i32) as i64);
    }
    let inline_time = start.elapsed();

    // 测试非内联函数
    let start = Instant::now();
    let mut _sum = 0i64;
    for i in 0..ITERATIONS {
        _sum = _sum.wrapping_add(never_inline_function(i as i32) as i64);
    }
    let no_inline_time = start.elapsed();

    println!("强制内联: {:?}", inline_time);
    println!("禁止内联: {:?}", no_inline_time);

    println!("\n💡 优化提示:");
    println!("- 使用 #[inline] 提示编译器内联热点函数");
    println!("- 使用 #[cold] 标记冷路径");
    println!("- 使用 std::hint::black_box 防止过度优化");
    println!("- 在 release 模式下测试性能");
}

/// 演示批量处理优化
fn demonstrate_batch_processing() {
    println!("\n📦 批量处理优化:");

    let data: Vec<f64> = (0..100000_i32).map(|i| i as f64).collect();

    // 单个处理
    let start = Instant::now();
    let mut results = Vec::new();
    for &value in &data {
        results.push(value.sqrt().sin().cos());
    }
    let individual_time = start.elapsed();

    // 批量处理
    let start = Instant::now();
    let batch_results: Vec<f64> = data
        .chunks(1000)
        .flat_map(|chunk| chunk.iter().map(|&x| x.sqrt().sin().cos()))
        .collect();
    let batch_time = start.elapsed();

    // SIMD 友好的处理
    let start = Instant::now();
    let simd_results: Vec<f64> = data.iter().map(|&x| x.sqrt().sin().cos()).collect();
    let simd_time = start.elapsed();

    println!("单个处理: {:?}", individual_time);
    println!("批量处理: {:?}", batch_time);
    println!("SIMD友好: {:?}", simd_time);

    // 验证结果一致性
    let consistent =
        results.len() == batch_results.len() && batch_results.len() == simd_results.len();
    println!("结果一致: {}", consistent);

    println!("\n💡 批量处理建议:");
    println!("- 使用迭代器链进行批量操作");
    println!("- 考虑数据局部性和缓存友好性");
    println!("- 利用 SIMD 指令进行并行计算");
    println!("- 避免频繁的小批量操作");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_sizes() {
        let no_capture = || 42;
        let value = 42i32;
        let with_capture = move || value;

        // 无捕获闭包应该是零大小类型
        assert_eq!(std::mem::size_of_val(&no_capture), 0);

        // 有捕获的闭包大小应该等于捕获变量的大小
        assert_eq!(
            std::mem::size_of_val(&with_capture),
            std::mem::size_of::<i32>()
        );
    }

    #[test]
    fn test_performance_consistency() {
        let f1 = |x: i32| x * 2;
        let f2: fn(i32) -> i32 = |x| x * 2;

        // 功能应该一致
        assert_eq!(f1(5), f2(5));
        assert_eq!(f1(5), 10);
    }

    #[test]
    fn test_capture_optimization() {
        struct Data {
            values: Vec<i32>,
        }

        let data = Data {
            values: vec![1, 2, 3, 4, 5],
        };

        // 只捕获需要的部分
        let sum = data.values.iter().sum::<i32>();
        let optimized_closure = move || sum;

        assert_eq!(optimized_closure(), 15);
    }

    #[test]
    fn test_inline_optimization() {
        #[inline(always)]
        fn inline_fn(x: i32) -> i32 {
            x + 1
        }

        let closure = |x: i32| inline_fn(x);
        assert_eq!(closure(5), 6);
    }

    #[test]
    fn test_memory_layout() {
        #[repr(C)]
        struct TestStruct {
            a: i64,
            b: i32,
            c: i16,
            d: i8,
        }

        let test_data = TestStruct {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
        };

        let closure = move || test_data.a + test_data.b as i64;
        assert_eq!(closure(), 3);
    }
}
