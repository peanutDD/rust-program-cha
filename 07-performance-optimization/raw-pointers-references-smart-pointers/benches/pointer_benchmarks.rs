//! # 指针类型性能基准测试
//!
//! 使用 Criterion 进行详细的性能基准测试

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use raw_pointers_references_smart_pointers::performance::*;
use std::rc::Rc;
use std::sync::Arc;

/// 基础访问性能基准测试
fn basic_access_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic_access");
    
    let data = vec![1, 2, 3, 4, 5];
    let reference = &data;
    let boxed = Box::new(data.clone());
    let rc = Rc::new(data.clone());
    let arc = Arc::new(data.clone());
    let raw_ptr = data.as_ptr();
    
    group.bench_function("reference_access", |b| {
        b.iter(|| {
            for i in 0..reference.len() {
                black_box(reference[i]);
            }
        })
    });
    
    group.bench_function("box_access", |b| {
        b.iter(|| {
            for i in 0..boxed.len() {
                black_box(boxed[i]);
            }
        })
    });
    
    group.bench_function("rc_access", |b| {
        b.iter(|| {
            for i in 0..rc.len() {
                black_box(rc[i]);
            }
        })
    });
    
    group.bench_function("arc_access", |b| {
        b.iter(|| {
            for i in 0..arc.len() {
                black_box(arc[i]);
            }
        })
    });
    
    group.bench_function("raw_pointer_access", |b| {
        b.iter(|| unsafe {
            for i in 0..data.len() {
                black_box(*raw_ptr.add(i));
            }
        })
    });
    
    group.finish();
}

/// 内存分配性能基准测试
fn allocation_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocation");
    
    group.bench_function("box_allocation", |b| {
        b.iter(|| {
            let boxed = Box::new(black_box(42));
            black_box(boxed);
        })
    });
    
    group.bench_function("rc_allocation", |b| {
        b.iter(|| {
            let rc = Rc::new(black_box(42));
            black_box(rc);
        })
    });
    
    group.bench_function("arc_allocation", |b| {
        b.iter(|| {
            let arc = Arc::new(black_box(42));
            black_box(arc);
        })
    });
    
    group.finish();
}

/// 引用计数性能基准测试
fn reference_counting_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("reference_counting");
    
    let rc = Rc::new(vec![1; 1000]);
    let arc = Arc::new(vec![1; 1000]);
    
    group.bench_function("rc_clone", |b| {
        b.iter(|| {
            let cloned = Rc::clone(&rc);
            black_box(cloned);
        })
    });
    
    group.bench_function("arc_clone", |b| {
        b.iter(|| {
            let cloned = Arc::clone(&arc);
            black_box(cloned);
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    basic_access_benchmarks,
    allocation_benchmarks,
    reference_counting_benchmarks
);
criterion_main!(benches);