//! # 闭包性能基准测试
//!
//! 使用 Criterion 进行性能基准测试
//!
//! ## 运行方式
//!
//! ```bash
//! # 运行所有基准测试
//! cargo bench
//!
//! # 运行特定测试
//! cargo bench closure_vs_function
//!
//! # 生成详细报告
//! cargo bench -- --verbose
//! ```

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

/// 测试 1: 闭包调用 vs 函数调用
fn closure_vs_function(c: &mut Criterion) {
    let mut group = c.benchmark_group("closure_vs_function");

    // 测试普通函数
    group.bench_function("function", |b| {
        fn function(x: i32) -> i32 {
            x * 2 + 1
        }
        b.iter(|| {
            for i in 0..1000 {
                black_box(function(black_box(i)));
            }
        });
    });

    // 测试闭包
    group.bench_function("closure", |b| {
        let closure = |x: i32| x * 2 + 1;
        b.iter(|| {
            for i in 0..1000 {
                black_box(closure(black_box(i)));
            }
        });
    });

    group.finish();
}

/// 测试 2: 不同捕获方式的性能
fn capture_modes(c: &mut Criterion) {
    let mut group = c.benchmark_group("capture_modes");

    // 无捕获
    group.bench_function("no_capture", |b| {
        let closure = |x: i32| x * 2 + 1;
        b.iter(|| {
            for i in 0..1000 {
                black_box(closure(black_box(i)));
            }
        });
    });

    // 捕获一个变量
    group.bench_function("capture_one", |b| {
        let multiplier = 2;
        let closure = |x: i32| x * multiplier + 1;
        b.iter(|| {
            for i in 0..1000 {
                black_box(closure(black_box(i)));
            }
        });
    });

    // 捕获多个变量
    group.bench_function("capture_multiple", |b| {
        let multiplier = 2;
        let adder = 1;
        let closure = |x: i32| x * multiplier + adder;
        b.iter(|| {
            for i in 0..1000 {
                black_box(closure(black_box(i)));
            }
        });
    });

    group.finish();
}

/// 测试 3: 静态分发 vs 动态分发
fn static_vs_dynamic_dispatch(c: &mut Criterion) {
    let mut group = c.benchmark_group("dispatch");

    // 静态分发
    group.bench_function("static", |b| {
        let closure = |x: i32| x * 2 + 1;
        b.iter(|| {
            for i in 0..1000 {
                black_box(call_static(&closure, black_box(i)));
            }
        });
    });

    // 动态分发
    group.bench_function("dynamic", |b| {
        let closure: Box<dyn Fn(i32) -> i32> = Box::new(|x| x * 2 + 1);
        b.iter(|| {
            for i in 0..1000 {
                black_box(call_dynamic(closure.as_ref(), black_box(i)));
            }
        });
    });

    group.finish();
}

fn call_static<F>(f: &F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

fn call_dynamic(f: &dyn Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

/// 测试 4: 迭代器与闭包
fn iterator_with_closure(c: &mut Criterion) {
    let mut group = c.benchmark_group("iterator");

    let numbers: Vec<i32> = (0..1000).collect();

    // 使用迭代器 + 闭包
    group.bench_function("iterator_closure", |b| {
        b.iter(|| {
            let result: i32 = numbers
                .iter()
                .filter(|&&x| x % 2 == 0)
                .map(|&x| x * 2)
                .sum();
            black_box(result)
        });
    });

    // 手写循环
    group.bench_function("manual_loop", |b| {
        b.iter(|| {
            let mut result = 0;
            for &x in &numbers {
                if x % 2 == 0 {
                    result += x * 2;
                }
            }
            black_box(result)
        });
    });

    group.finish();
}

/// 测试 5: 不同大小的闭包
fn closure_size_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("closure_size");

    // 小闭包（不捕获）
    group.bench_function("size_0", |b| {
        let closure = |x: i32| x * 2;
        b.iter(|| black_box(closure(black_box(42))));
    });

    // 捕获一个 i32
    group.bench_function("size_i32", |b| {
        let value = 42;
        let closure = move |x: i32| x * value;
        b.iter(|| black_box(closure(black_box(42))));
    });

    // 捕获一个小数组
    group.bench_function("size_small_array", |b| {
        let arr = [1, 2, 3, 4];
        let closure = move |x: usize| arr[x];
        b.iter(|| black_box(closure(black_box(0))));
    });

    // 捕获一个大数组（通过引用）
    group.bench_function("size_large_ref", |b| {
        let arr = [0; 1000];
        let closure = || arr[0];
        b.iter(|| black_box(closure()));
    });

    group.finish();
}

/// 测试 6: 闭包组合
fn closure_composition(c: &mut Criterion) {
    let mut group = c.benchmark_group("composition");

    let add_one = |x: i32| x + 1;
    let double = |x: i32| x * 2;
    let square = |x: i32| x * x;

    // 单个闭包
    group.bench_function("single", |b| {
        b.iter(|| black_box(add_one(black_box(42))));
    });

    // 组合两个闭包
    group.bench_function("compose_two", |b| {
        b.iter(|| black_box(double(add_one(black_box(42)))));
    });

    // 组合三个闭包
    group.bench_function("compose_three", |b| {
        b.iter(|| black_box(square(double(add_one(black_box(42))))));
    });

    group.finish();
}

/// 测试 7: 不同输入大小的性能
fn input_size_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("input_size");

    for size in [10, 100, 1000, 10000].iter() {
        let numbers: Vec<i32> = (0..*size).collect();

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let result: i32 = numbers
                    .iter()
                    .filter(|&&x| x % 2 == 0)
                    .map(|&x| x * 2)
                    .sum();
                black_box(result)
            });
        });
    }

    group.finish();
}

/// 测试 8: FnOnce vs FnMut vs Fn 的性能
fn trait_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("trait_types");

    // Fn trait
    group.bench_function("Fn", |b| {
        let value = 42;
        let closure = || value * 2;
        b.iter(|| {
            for _ in 0..100 {
                black_box(call_fn(&closure));
            }
        });
    });

    // FnMut trait
    group.bench_function("FnMut", |b| {
        let mut count = 0;
        let mut closure = || {
            count += 1;
            count
        };
        b.iter(|| {
            for _ in 0..100 {
                black_box(call_fn_mut(&mut closure));
            }
        });
    });

    // FnOnce trait (需要每次重新创建)
    group.bench_function("FnOnce", |b| {
        b.iter(|| {
            for _ in 0..100 {
                let value = 42;
                let closure = || value;
                black_box(call_fn_once(closure));
            }
        });
    });

    group.finish();
}

fn call_fn<F>(f: &F) -> i32
where
    F: Fn() -> i32,
{
    f()
}

fn call_fn_mut<F>(f: &mut F) -> i32
where
    F: FnMut() -> i32,
{
    f()
}

fn call_fn_once<F>(f: F) -> i32
where
    F: FnOnce() -> i32,
{
    f()
}

// 配置基准测试组
criterion_group!(
    benches,
    closure_vs_function,
    capture_modes,
    static_vs_dynamic_dispatch,
    iterator_with_closure,
    closure_size_impact,
    closure_composition,
    input_size_scaling,
    trait_performance
);

criterion_main!(benches);
