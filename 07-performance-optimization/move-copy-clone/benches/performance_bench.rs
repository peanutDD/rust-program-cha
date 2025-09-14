//! Move, Copy, Clone 性能基准测试
//! 
//! 这个文件包含了详细的性能基准测试，用于对比 Move、Copy 和 Clone 的性能特征。

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use move_copy_clone::performance::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

// 测试数据结构
#[derive(Clone)]
struct SmallData {
    value: i32,
}

#[derive(Clone)]
struct MediumData {
    values: [i32; 16],
}

#[derive(Clone)]
struct LargeData {
    values: Vec<i32>,
    metadata: HashMap<String, String>,
}

#[derive(Copy, Clone)]
struct CopyableSmall {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
struct CopyableMedium {
    data: [u8; 64],
}

/// Move 语义基准测试
fn bench_move_semantics(c: &mut Criterion) {
    let mut group = c.benchmark_group("move_semantics");
    
    // 小数据 Move
    group.bench_function("small_data_move", |b| {
        b.iter(|| {
            let data = SmallData { value: black_box(42) };
            let moved_data = black_box(data);
            moved_data
        })
    });
    
    // 中等数据 Move
    group.bench_function("medium_data_move", |b| {
        b.iter(|| {
            let data = MediumData { values: [1; 16] };
            let moved_data = black_box(data);
            moved_data
        })
    });
    
    // 大数据 Move
    group.bench_function("large_data_move", |b| {
        b.iter(|| {
            let mut metadata = HashMap::new();
            metadata.insert("key".to_string(), "value".to_string());
            let data = LargeData {
                values: vec![1; 1000],
                metadata,
            };
            let moved_data = black_box(data);
            moved_data
        })
    });
    
    // Vec Move
    group.bench_function("vec_move", |b| {
        b.iter(|| {
            let vec = vec![1; black_box(1000)];
            let moved_vec = black_box(vec);
            moved_vec
        })
    });
    
    // String Move
    group.bench_function("string_move", |b| {
        b.iter(|| {
            let s = "Hello, World!".repeat(black_box(100));
            let moved_s = black_box(s);
            moved_s
        })
    });
    
    group.finish();
}

/// Copy trait 基准测试
fn bench_copy_trait(c: &mut Criterion) {
    let mut group = c.benchmark_group("copy_trait");
    
    // 基本类型 Copy
    group.bench_function("i32_copy", |b| {
        b.iter(|| {
            let x = black_box(42i32);
            let y = black_box(x);
            y
        })
    });
    
    group.bench_function("f64_copy", |b| {
        b.iter(|| {
            let x = black_box(3.14f64);
            let y = black_box(x);
            y
        })
    });
    
    // 小结构体 Copy
    group.bench_function("small_struct_copy", |b| {
        b.iter(|| {
            let data = CopyableSmall { x: black_box(1), y: black_box(2) };
            let copied = black_box(data);
            copied
        })
    });
    
    // 中等结构体 Copy
    group.bench_function("medium_struct_copy", |b| {
        b.iter(|| {
            let data = CopyableMedium { data: [1; 64] };
            let copied = black_box(data);
            copied
        })
    });
    
    // 数组 Copy
    group.bench_function("array_copy_small", |b| {
        b.iter(|| {
            let arr = [black_box(1); 4];
            let copied = black_box(arr);
            copied
        })
    });
    
    group.bench_function("array_copy_large", |b| {
        b.iter(|| {
            let arr = [black_box(1); 256];
            let copied = black_box(arr);
            copied
        })
    });
    
    group.finish();
}

/// Clone trait 基准测试
fn bench_clone_trait(c: &mut Criterion) {
    let mut group = c.benchmark_group("clone_trait");
    
    // 小数据 Clone
    group.bench_function("small_data_clone", |b| {
        let data = SmallData { value: 42 };
        b.iter(|| {
            let cloned = black_box(data.clone());
            cloned
        })
    });
    
    // Vec Clone
    for size in [10, 100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("vec_clone", size), size, |b, &size| {
            let vec = vec![1; size];
            b.iter(|| {
                let cloned = black_box(vec.clone());
                cloned
            })
        });
    }
    
    // String Clone
    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("string_clone", size), size, |b, &size| {
            let s = "a".repeat(size);
            b.iter(|| {
                let cloned = black_box(s.clone());
                cloned
            })
        });
    }
    
    // HashMap Clone
    group.bench_function("hashmap_clone", |b| {
        let mut map = HashMap::new();
        for i in 0..100 {
            map.insert(format!("key_{}", i), format!("value_{}", i));
        }
        b.iter(|| {
            let cloned = black_box(map.clone());
            cloned
        })
    });
    
    // Rc Clone
    group.bench_function("rc_clone", |b| {
        let data = Rc::new(vec![1; 1000]);
        b.iter(|| {
            let cloned = black_box(data.clone());
            cloned
        })
    });
    
    // Arc Clone
    group.bench_function("arc_clone", |b| {
        let data = Arc::new(vec![1; 1000]);
        b.iter(|| {
            let cloned = black_box(data.clone());
            cloned
        })
    });
    
    group.finish();
}

/// 对比基准测试
fn bench_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("comparison");
    
    // Move vs Copy vs Clone 对比（小数据）
    group.bench_function("small_move", |b| {
        b.iter(|| {
            let data = black_box(42i32);
            data // Move
        })
    });
    
    group.bench_function("small_copy", |b| {
        b.iter(|| {
            let data = black_box(42i32);
            let copied = data; // Copy
            copied
        })
    });
    
    group.bench_function("small_clone", |b| {
        let data = SmallData { value: 42 };
        b.iter(|| {
            let cloned = black_box(data.clone());
            cloned
        })
    });
    
    // 引用 vs Clone 对比
    group.bench_function("reference_access", |b| {
        let data = vec![1; 1000];
        b.iter(|| {
            let referenced = black_box(&data);
            referenced.len()
        })
    });
    
    group.bench_function("clone_access", |b| {
        let data = vec![1; 1000];
        b.iter(|| {
            let cloned = black_box(data.clone());
            cloned.len()
        })
    });
    
    group.finish();
}

/// 循环中的性能测试
fn bench_loop_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("loop_performance");
    
    // 循环中的 Copy
    group.bench_function("loop_copy", |b| {
        let data = 42i32;
        b.iter(|| {
            let mut sum = 0;
            for _ in 0..black_box(1000) {
                let copied = data; // Copy
                sum += copied;
            }
            sum
        })
    });
    
    // 循环中的 Clone
    group.bench_function("loop_clone", |b| {
        let data = SmallData { value: 42 };
        b.iter(|| {
            let mut sum = 0;
            for _ in 0..black_box(1000) {
                let cloned = data.clone(); // Clone
                sum += cloned.value;
            }
            sum
        })
    });
    
    // 循环中的引用
    group.bench_function("loop_reference", |b| {
        let data = SmallData { value: 42 };
        b.iter(|| {
            let mut sum = 0;
            for _ in 0..black_box(1000) {
                let referenced = &data; // Reference
                sum += referenced.value;
            }
            sum
        })
    });
    
    group.finish();
}

/// 内存分配基准测试
fn bench_memory_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocation");
    
    // 栈分配 vs 堆分配
    group.bench_function("stack_allocation", |b| {
        b.iter(|| {
            let data = [black_box(1); 100]; // 栈分配
            data
        })
    });
    
    group.bench_function("heap_allocation", |b| {
        b.iter(|| {
            let data = vec![black_box(1); 100]; // 堆分配
            data
        })
    });
    
    // Box 的 Move
    group.bench_function("box_move", |b| {
        b.iter(|| {
            let data = Box::new([black_box(1); 100]);
            let moved = black_box(data);
            moved
        })
    });
    
    group.finish();
}

/// 智能指针基准测试
fn bench_smart_pointers(c: &mut Criterion) {
    let mut group = c.benchmark_group("smart_pointers");
    
    // Rc 创建和克隆
    group.bench_function("rc_creation", |b| {
        b.iter(|| {
            let data = Rc::new(black_box(vec![1; 100]));
            data
        })
    });
    
    group.bench_function("rc_clone", |b| {
        let data = Rc::new(vec![1; 100]);
        b.iter(|| {
            let cloned = black_box(data.clone());
            cloned
        })
    });
    
    // Arc 创建和克隆
    group.bench_function("arc_creation", |b| {
        b.iter(|| {
            let data = Arc::new(black_box(vec![1; 100]));
            data
        })
    });
    
    group.bench_function("arc_clone", |b| {
        let data = Arc::new(vec![1; 100]);
        b.iter(|| {
            let cloned = black_box(data.clone());
            cloned
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_move_semantics,
    bench_copy_trait,
    bench_clone_trait,
    bench_comparison,
    bench_loop_performance,
    bench_memory_allocation,
    bench_smart_pointers
);
criterion_main!(benches);