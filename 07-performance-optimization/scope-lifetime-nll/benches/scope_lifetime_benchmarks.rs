//! 作用域、生命周期和 NLL 性能基准测试
//! 
//! 本文件包含用于测试不同实现方式性能差异的基准测试。

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use scope_lifetime_nll::*;
use std::time::Duration;

/// 测试不同借用模式的性能
fn benchmark_borrowing_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("borrowing_patterns");
    
    // 测试数据
    let data: Vec<String> = (0..1000)
        .map(|i| format!("item_{}", i))
        .collect();
    
    // 基准测试：使用借用
    group.bench_function("with_borrowing", |b| {
        b.iter(|| {
            let result: Vec<&str> = black_box(&data)
                .iter()
                .map(|s| s.as_str())
                .collect();
            black_box(result)
        })
    });
    
    // 基准测试：使用克隆
    group.bench_function("with_cloning", |b| {
        b.iter(|| {
            let result: Vec<String> = black_box(&data)
                .iter()
                .cloned()
                .collect();
            black_box(result)
        })
    });
    
    // 基准测试：使用移动
    group.bench_function("with_moving", |b| {
        b.iter(|| {
            let data_copy = data.clone();
            let result: Vec<String> = black_box(data_copy)
                .into_iter()
                .collect();
            black_box(result)
        })
    });
    
    group.finish();
}

/// 测试不同作用域模式的性能
fn benchmark_scope_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("scope_patterns");
    
    // 基准测试：嵌套作用域
    group.bench_function("nested_scopes", |b| {
        b.iter(|| {
            let mut result = 0;
            for i in 0..1000 {
                {
                    let temp = i * 2;
                    {
                        let inner_temp = temp + 1;
                        result += inner_temp;
                    }
                }
            }
            black_box(result)
        })
    });
    
    // 基准测试：平坦作用域
    group.bench_function("flat_scope", |b| {
        b.iter(|| {
            let mut result = 0;
            for i in 0..1000 {
                let temp = i * 2;
                let inner_temp = temp + 1;
                result += inner_temp;
            }
            black_box(result)
        })
    });
    
    group.finish();
}

/// 测试生命周期相关的性能
fn benchmark_lifetime_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("lifetime_patterns");
    
    let data = vec!["hello", "world", "rust", "programming"];
    
    // 基准测试：返回引用
    group.bench_function("return_reference", |b| {
        b.iter(|| {
            fn find_longest<'a>(strings: &'a [&str]) -> Option<&'a str> {
                strings.iter().max_by_key(|s| s.len()).copied()
            }
            
            let result = find_longest(black_box(&data));
            black_box(result)
        })
    });
    
    // 基准测试：返回拥有的值
    group.bench_function("return_owned", |b| {
        b.iter(|| {
            fn find_longest_owned(strings: &[&str]) -> Option<String> {
                strings.iter()
                    .max_by_key(|s| s.len())
                    .map(|s| s.to_string())
            }
            
            let result = find_longest_owned(black_box(&data));
            black_box(result)
        })
    });
    
    group.finish();
}

/// 测试 NLL 改进的性能影响
fn benchmark_nll_improvements(c: &mut Criterion) {
    let mut group = c.benchmark_group("nll_improvements");
    
    // 基准测试：NLL 允许的模式
    group.bench_function("nll_pattern", |b| {
        b.iter(|| {
            let mut data = vec![1, 2, 3, 4, 5];
            
            // NLL 允许这种模式
            let first = &data[0];
            if *first > 0 {
                data.push(6); // 在 NLL 之前这里会报错
            }
            
            black_box(data)
        })
    });
    
    // 基准测试：传统模式（需要重构）
    group.bench_function("traditional_pattern", |b| {
        b.iter(|| {
            let mut data = vec![1, 2, 3, 4, 5];
            
            // 传统模式需要重构
            let should_push = {
                let first = &data[0];
                *first > 0
            };
            
            if should_push {
                data.push(6);
            }
            
            black_box(data)
        })
    });
    
    group.finish();
}

/// 测试内存分配模式的性能
fn benchmark_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");
    
    // 基准测试：栈分配
    group.bench_function("stack_allocation", |b| {
        b.iter(|| {
            let data = [1, 2, 3, 4, 5]; // 栈分配
            let sum: i32 = data.iter().sum();
            black_box(sum)
        })
    });
    
    // 基准测试：堆分配
    group.bench_function("heap_allocation", |b| {
        b.iter(|| {
            let data = vec![1, 2, 3, 4, 5]; // 堆分配
            let sum: i32 = data.iter().sum();
            black_box(sum)
        })
    });
    
    // 基准测试：Box 分配
    group.bench_function("box_allocation", |b| {
        b.iter(|| {
            let data = Box::new([1, 2, 3, 4, 5]); // Box 分配
            let sum: i32 = data.iter().sum();
            black_box(sum)
        })
    });
    
    group.finish();
}

/// 测试智能指针的性能
fn benchmark_smart_pointers(c: &mut Criterion) {
    use std::rc::Rc;
    use std::sync::Arc;
    
    let mut group = c.benchmark_group("smart_pointers");
    
    let data = vec![1, 2, 3, 4, 5];
    
    // 基准测试：原始引用
    group.bench_function("raw_reference", |b| {
        b.iter(|| {
            let reference = &data;
            let sum: i32 = reference.iter().sum();
            black_box(sum)
        })
    });
    
    // 基准测试：Rc
    group.bench_function("rc_pointer", |b| {
        let rc_data = Rc::new(data.clone());
        b.iter(|| {
            let cloned = rc_data.clone();
            let sum: i32 = cloned.iter().sum();
            black_box(sum)
        })
    });
    
    // 基准测试：Arc
    group.bench_function("arc_pointer", |b| {
        let arc_data = Arc::new(data.clone());
        b.iter(|| {
            let cloned = arc_data.clone();
            let sum: i32 = cloned.iter().sum();
            black_box(sum)
        })
    });
    
    group.finish();
}

/// 测试错误处理模式的性能
fn benchmark_error_handling(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_handling");
    
    // 基准测试：Result 模式
    group.bench_function("result_pattern", |b| {
        b.iter(|| {
            fn safe_divide(a: f64, b: f64) -> Result<f64, &'static str> {
                if b == 0.0 {
                    Err("Division by zero")
                } else {
                    Ok(a / b)
                }
            }
            
            let results: Vec<_> = (0..1000)
                .map(|i| safe_divide(i as f64, (i % 10 + 1) as f64))
                .collect();
            black_box(results)
        })
    });
    
    // 基准测试：Option 模式
    group.bench_function("option_pattern", |b| {
        b.iter(|| {
            fn safe_divide_opt(a: f64, b: f64) -> Option<f64> {
                if b == 0.0 {
                    None
                } else {
                    Some(a / b)
                }
            }
            
            let results: Vec<_> = (0..1000)
                .map(|i| safe_divide_opt(i as f64, (i % 10 + 1) as f64))
                .collect();
            black_box(results)
        })
    });
    
    group.finish();
}

// 配置基准测试
criterion_group!(
    benches,
    benchmark_borrowing_patterns,
    benchmark_scope_patterns,
    benchmark_lifetime_patterns,
    benchmark_nll_improvements,
    benchmark_memory_patterns,
    benchmark_smart_pointers,
    benchmark_error_handling
);

criterion_main!(benches);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_setup() {
        // 确保基准测试设置正确
        let data: Vec<String> = (0..10)
            .map(|i| format!("item_{}", i))
            .collect();
        
        assert_eq!(data.len(), 10);
        assert_eq!(data[0], "item_0");
    }

    #[test]
    fn test_performance_patterns() {
        // 测试性能模式是否正确工作
        
        // 测试借用模式
        let data = vec!["hello".to_string(), "world".to_string()];
        let borrowed: Vec<&str> = data.iter().map(|s| s.as_str()).collect();
        assert_eq!(borrowed, vec!["hello", "world"]);
        
        // 测试克隆模式
        let cloned: Vec<String> = data.iter().cloned().collect();
        assert_eq!(cloned, data);
    }
}