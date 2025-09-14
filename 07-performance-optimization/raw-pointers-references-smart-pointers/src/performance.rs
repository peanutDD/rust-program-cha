//! # 性能分析模块
//!
//! 本模块提供裸指针、引用和智能指针的性能分析和基准测试。
//! 通过实际的性能测试，帮助理解不同指针类型的性能特征。

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};

/// 运行所有性能测试
pub fn run_all_performance_tests() {
    println!("\n🚀 性能分析和基准测试");
    println!("{}", "=".repeat(60));
    
    // 基础访问性能
    basic_access_performance();
    
    // 内存分配性能
    memory_allocation_performance();
    
    // 引用计数性能
    reference_counting_performance();
    
    // 并发性能
    concurrency_performance();
    
    // 缓存性能
    cache_performance();
    
    // 内存使用分析
    memory_usage_analysis();
    
    // 编译时优化分析
    compile_time_optimization_analysis();
}

/// 基础访问性能测试
fn basic_access_performance() {
    println!("\n⚡ 1. 基础访问性能测试");
    println!("{}", "-".repeat(40));
    
    const ITERATIONS: usize = 10_000_000;
    let data = vec![1i32; 1000];
    
    // 直接访问
    println!("\n📊 直接访问性能:");
    let start = Instant::now();
    let mut sum = 0i64;
    for _ in 0..ITERATIONS {
        sum += data[500] as i64;
    }
    let direct_time = start.elapsed();
    println!("  直接访问: {:?} (结果: {})", direct_time, sum);
    
    // 引用访问
    println!("\n📊 引用访问性能:");
    let data_ref = &data;
    let start = Instant::now();
    let mut sum = 0i64;
    for _ in 0..ITERATIONS {
        sum += data_ref[500] as i64;
    }
    let reference_time = start.elapsed();
    println!("  引用访问: {:?} (结果: {})", reference_time, sum);
    
    // Box 访问
    println!("\n📊 Box 访问性能:");
    let boxed_data = Box::new(data.clone());
    let start = Instant::now();
    let mut sum = 0i64;
    for _ in 0..ITERATIONS {
        sum += boxed_data[500] as i64;
    }
    let box_time = start.elapsed();
    println!("  Box 访问: {:?} (结果: {})", box_time, sum);
    
    // 裸指针访问
    println!("\n📊 裸指针访问性能:");
    let raw_ptr = data.as_ptr();
    let start = Instant::now();
    let mut sum = 0i64;
    unsafe {
        for _ in 0..ITERATIONS {
            sum += *raw_ptr.add(500) as i64;
        }
    }
    let raw_ptr_time = start.elapsed();
    println!("  裸指针访问: {:?} (结果: {})", raw_ptr_time, sum);
    
    // 性能比较
    println!("\n📈 性能比较 (相对于直接访问):");
    println!("  直接访问: 1.00x (基准)");
    println!("  引用访问: {:.2}x", reference_time.as_nanos() as f64 / direct_time.as_nanos() as f64);
    println!("  Box 访问: {:.2}x", box_time.as_nanos() as f64 / direct_time.as_nanos() as f64);
    println!("  裸指针访问: {:.2}x", raw_ptr_time.as_nanos() as f64 / direct_time.as_nanos() as f64);
    
    println!("  ✅ 引用和裸指针通常与直接访问性能相当");
}

/// 内存分配性能测试
fn memory_allocation_performance() {
    println!("\n🏗️ 2. 内存分配性能测试");
    println!("{}", "-".repeat(40));
    
    const ALLOCATIONS: usize = 100_000;
    const DATA_SIZE: usize = 1024;
    
    // 栈分配 (作为对比基准)
    println!("\n📊 栈分配性能 (基准):");
    let start = Instant::now();
    for _ in 0..1000 { // 减少次数避免栈溢出
        let _stack_data = [0u8; 1024];
        // 模拟使用
        std::hint::black_box(_stack_data);
    }
    let stack_time = start.elapsed();
    println!("  栈分配 (1000次): {:?}", stack_time);
    
    // Box 分配
    println!("\n📊 Box 分配性能:");
    let start = Instant::now();
    let mut boxes = Vec::new();
    for _ in 0..ALLOCATIONS {
        let boxed = Box::new(vec![0u8; DATA_SIZE]);
        boxes.push(boxed);
    }
    let box_alloc_time = start.elapsed();
    println!("  Box 分配 ({}次): {:?}", ALLOCATIONS, box_alloc_time);
    
    // 清理 Box
    let start = Instant::now();
    drop(boxes);
    let box_dealloc_time = start.elapsed();
    println!("  Box 释放: {:?}", box_dealloc_time);
    
    // Vec 分配
    println!("\n📊 Vec 分配性能:");
    let start = Instant::now();
    let mut vecs = Vec::new();
    for _ in 0..ALLOCATIONS {
        let vec_data = vec![0u8; DATA_SIZE];
        vecs.push(vec_data);
    }
    let vec_alloc_time = start.elapsed();
    println!("  Vec 分配 ({}次): {:?}", ALLOCATIONS, vec_alloc_time);
    
    // 清理 Vec
    let start = Instant::now();
    drop(vecs);
    let vec_dealloc_time = start.elapsed();
    println!("  Vec 释放: {:?}", vec_dealloc_time);
    
    // 裸指针分配 (使用 libc)
    println!("\n📊 裸指针分配性能:");
    let start = Instant::now();
    let mut raw_ptrs = Vec::new();
    unsafe {
        for _ in 0..ALLOCATIONS {
            let layout = std::alloc::Layout::from_size_align(DATA_SIZE, 8).unwrap();
            let ptr = std::alloc::alloc(layout);
            if !ptr.is_null() {
                raw_ptrs.push((ptr, layout));
            }
        }
    }
    let raw_alloc_time = start.elapsed();
    println!("  裸指针分配 ({}次): {:?}", ALLOCATIONS, raw_alloc_time);
    
    // 清理裸指针
    let start = Instant::now();
    unsafe {
        for (ptr, layout) in raw_ptrs {
            std::alloc::dealloc(ptr, layout);
        }
    }
    let raw_dealloc_time = start.elapsed();
    println!("  裸指针释放: {:?}", raw_dealloc_time);
    
    // 性能比较
    println!("\n📈 分配性能比较:");
    println!("  Box 分配: {:.2}x", box_alloc_time.as_nanos() as f64 / box_alloc_time.as_nanos() as f64);
    println!("  Vec 分配: {:.2}x", vec_alloc_time.as_nanos() as f64 / box_alloc_time.as_nanos() as f64);
    println!("  裸指针分配: {:.2}x", raw_alloc_time.as_nanos() as f64 / box_alloc_time.as_nanos() as f64);
    
    println!("  ✅ 裸指针分配最快，但需要手动管理内存");
}

/// 引用计数性能测试
fn reference_counting_performance() {
    println!("\n🔢 3. 引用计数性能测试");
    println!("{}", "-".repeat(40));
    
    const OPERATIONS: usize = 1_000_000;
    let data = vec![1, 2, 3, 4, 5];
    
    // Rc 克隆性能
    println!("\n📊 Rc 克隆性能:");
    let rc_data = Rc::new(data.clone());
    let start = Instant::now();
    let mut clones = Vec::new();
    for _ in 0..OPERATIONS {
        clones.push(Rc::clone(&rc_data));
    }
    let rc_clone_time = start.elapsed();
    println!("  Rc 克隆 ({}次): {:?}", OPERATIONS, rc_clone_time);
    println!("  最终引用计数: {}", Rc::strong_count(&rc_data));
    
    // 清理 Rc 克隆
    let start = Instant::now();
    drop(clones);
    let rc_drop_time = start.elapsed();
    println!("  Rc 释放: {:?}", rc_drop_time);
    println!("  剩余引用计数: {}", Rc::strong_count(&rc_data));
    
    // Arc 克隆性能
    println!("\n📊 Arc 克隆性能:");
    let arc_data = Arc::new(data.clone());
    let start = Instant::now();
    let mut clones = Vec::new();
    for _ in 0..OPERATIONS {
        clones.push(Arc::clone(&arc_data));
    }
    let arc_clone_time = start.elapsed();
    println!("  Arc 克隆 ({}次): {:?}", OPERATIONS, arc_clone_time);
    println!("  最终引用计数: {}", Arc::strong_count(&arc_data));
    
    // 清理 Arc 克隆
    let start = Instant::now();
    drop(clones);
    let arc_drop_time = start.elapsed();
    println!("  Arc 释放: {:?}", arc_drop_time);
    println!("  剩余引用计数: {}", Arc::strong_count(&arc_data));
    
    // Box 克隆性能 (作为对比)
    println!("\n📊 Box 克隆性能 (深拷贝):");
    let box_data = Box::new(data.clone());
    let start = Instant::now();
    let mut clones = Vec::new();
    for _ in 0..OPERATIONS {
        clones.push(box_data.clone());
    }
    let box_clone_time = start.elapsed();
    println!("  Box 克隆 ({}次): {:?}", OPERATIONS, box_clone_time);
    
    // 性能比较
    println!("\n📈 克隆性能比较:");
    println!("  Rc 克隆: 1.00x (基准)");
    println!("  Arc 克隆: {:.2}x", arc_clone_time.as_nanos() as f64 / rc_clone_time.as_nanos() as f64);
    println!("  Box 克隆: {:.2}x", box_clone_time.as_nanos() as f64 / rc_clone_time.as_nanos() as f64);
    
    println!("  ✅ Rc 最快，Arc 有原子操作开销，Box 需要深拷贝");
}

/// 并发性能测试
fn concurrency_performance() {
    println!("\n🧵 4. 并发性能测试");
    println!("{}", "-".repeat(40));
    
    const THREADS: usize = 4;
    const OPERATIONS_PER_THREAD: usize = 250_000;
    
    // Arc + Mutex 性能
    println!("\n📊 Arc + Mutex 性能:");
    let mutex_data = Arc::new(Mutex::new(0i64));
    let start = Instant::now();
    
    let handles: Vec<_> = (0..THREADS).map(|_| {
        let data = Arc::clone(&mutex_data);
        thread::spawn(move || {
            for _ in 0..OPERATIONS_PER_THREAD {
                let mut guard = data.lock().unwrap();
                *guard += 1;
            }
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let mutex_time = start.elapsed();
    let final_value = *mutex_data.lock().unwrap();
    println!("  Arc + Mutex: {:?} (结果: {})", mutex_time, final_value);
    
    // Arc + RwLock 读性能
    println!("\n📊 Arc + RwLock 读性能:");
    let rwlock_data = Arc::new(RwLock::new(42i64));
    let start = Instant::now();
    
    let handles: Vec<_> = (0..THREADS).map(|_| {
        let data = Arc::clone(&rwlock_data);
        thread::spawn(move || {
            let mut sum = 0i64;
            for _ in 0..OPERATIONS_PER_THREAD {
                let guard = data.read().unwrap();
                sum += *guard;
            }
            sum
        })
    }).collect();
    
    let mut total_sum = 0i64;
    for handle in handles {
        total_sum += handle.join().unwrap();
    }
    
    let rwlock_read_time = start.elapsed();
    println!("  Arc + RwLock 读: {:?} (结果: {})", rwlock_read_time, total_sum);
    
    // Arc + RwLock 写性能
    println!("\n📊 Arc + RwLock 写性能:");
    let rwlock_data = Arc::new(RwLock::new(0i64));
    let start = Instant::now();
    
    let handles: Vec<_> = (0..THREADS).map(|_| {
        let data = Arc::clone(&rwlock_data);
        thread::spawn(move || {
            for _ in 0..OPERATIONS_PER_THREAD {
                let mut guard = data.write().unwrap();
                *guard += 1;
            }
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let rwlock_write_time = start.elapsed();
    let final_value = *rwlock_data.read().unwrap();
    println!("  Arc + RwLock 写: {:?} (结果: {})", rwlock_write_time, final_value);
    
    // 原子操作性能
    println!("\n📊 原子操作性能:");
    let atomic_data = Arc::new(AtomicUsize::new(0));
    let start = Instant::now();
    
    let handles: Vec<_> = (0..THREADS).map(|_| {
        let data = Arc::clone(&atomic_data);
        thread::spawn(move || {
            for _ in 0..OPERATIONS_PER_THREAD {
                data.fetch_add(1, Ordering::Relaxed);
            }
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let atomic_time = start.elapsed();
    let final_value = atomic_data.load(Ordering::Relaxed);
    println!("  原子操作: {:?} (结果: {})", atomic_time, final_value);
    
    // 性能比较
    println!("\n📈 并发性能比较:");
    println!("  原子操作: 1.00x (基准)");
    println!("  Arc + Mutex: {:.2}x", mutex_time.as_nanos() as f64 / atomic_time.as_nanos() as f64);
    println!("  Arc + RwLock 读: {:.2}x", rwlock_read_time.as_nanos() as f64 / atomic_time.as_nanos() as f64);
    println!("  Arc + RwLock 写: {:.2}x", rwlock_write_time.as_nanos() as f64 / atomic_time.as_nanos() as f64);
    
    println!("  ✅ 原子操作最快，RwLock 读并发性好，Mutex 写性能稳定");
}

/// 缓存性能测试
fn cache_performance() {
    println!("\n💾 5. 缓存性能测试");
    println!("{}", "-".repeat(40));
    
    const SIZE: usize = 1024 * 1024; // 1MB
    const ITERATIONS: usize = 10;
    
    // 顺序访问
    println!("\n📊 顺序访问性能:");
    let data = vec![1u8; SIZE];
    let start = Instant::now();
    
    for _ in 0..ITERATIONS {
        let mut sum = 0u64;
        for &byte in &data {
            sum += byte as u64;
        }
        std::hint::black_box(sum);
    }
    
    let sequential_time = start.elapsed();
    println!("  顺序访问: {:?}", sequential_time);
    
    // 随机访问
    println!("\n📊 随机访问性能:");
    let indices: Vec<usize> = (0..SIZE).collect();
    let start = Instant::now();
    
    for _ in 0..ITERATIONS {
        let mut sum = 0u64;
        for &i in &indices {
            let index = (i * 7919) % SIZE; // 伪随机访问
            sum += data[index] as u64;
        }
        std::hint::black_box(sum);
    }
    
    let random_time = start.elapsed();
    println!("  随机访问: {:?}", random_time);
    
    // 指针跳跃访问
    println!("\n📊 指针跳跃访问性能:");
    let boxed_data = Box::new(data.clone());
    let start = Instant::now();
    
    for _ in 0..ITERATIONS {
        let mut sum = 0u64;
        let ptr = boxed_data.as_ptr();
        unsafe {
            for i in 0..SIZE {
                let index = (i * 7919) % SIZE;
                sum += *ptr.add(index) as u64;
            }
        }
        std::hint::black_box(sum);
    }
    
    let pointer_jump_time = start.elapsed();
    println!("  指针跳跃访问: {:?}", pointer_jump_time);
    
    // 缓存行对齐访问
    println!("\n📊 缓存行对齐访问性能:");
    const CACHE_LINE_SIZE: usize = 64;
    let start = Instant::now();
    
    for _ in 0..ITERATIONS {
        let mut sum = 0u64;
        let mut i = 0;
        while i < SIZE {
            sum += data[i] as u64;
            i += CACHE_LINE_SIZE; // 跳过一个缓存行
        }
        std::hint::black_box(sum);
    }
    
    let cache_aligned_time = start.elapsed();
    println!("  缓存行对齐访问: {:?}", cache_aligned_time);
    
    // 性能比较
    println!("\n📈 缓存性能比较:");
    println!("  顺序访问: 1.00x (基准)");
    println!("  随机访问: {:.2}x", random_time.as_nanos() as f64 / sequential_time.as_nanos() as f64);
    println!("  指针跳跃: {:.2}x", pointer_jump_time.as_nanos() as f64 / sequential_time.as_nanos() as f64);
    println!("  缓存行对齐: {:.2}x", cache_aligned_time.as_nanos() as f64 / sequential_time.as_nanos() as f64);
    
    println!("  ✅ 顺序访问最快，随机访问受缓存未命中影响");
}

/// 内存使用分析
fn memory_usage_analysis() {
    println!("\n🧠 6. 内存使用分析");
    println!("{}", "-".repeat(40));
    
    // 基础类型大小
    println!("\n📏 基础类型内存大小:");
    println!("  i32: {} 字节", std::mem::size_of::<i32>());
    println!("  &i32: {} 字节", std::mem::size_of::<&i32>());
    println!("  *const i32: {} 字节", std::mem::size_of::<*const i32>());
    println!("  *mut i32: {} 字节", std::mem::size_of::<*mut i32>());
    
    // 智能指针大小
    println!("\n📏 智能指针内存大小:");
    println!("  Box<i32>: {} 字节", std::mem::size_of::<Box<i32>>());
    println!("  Rc<i32>: {} 字节", std::mem::size_of::<Rc<i32>>());
    println!("  Arc<i32>: {} 字节", std::mem::size_of::<Arc<i32>>());
    println!("  RefCell<i32>: {} 字节", std::mem::size_of::<RefCell<i32>>());
    println!("  Mutex<i32>: {} 字节", std::mem::size_of::<Mutex<i32>>());
    println!("  RwLock<i32>: {} 字节", std::mem::size_of::<RwLock<i32>>());
    
    // 复合类型大小
    println!("\n📏 复合类型内存大小:");
    println!("  Option<&i32>: {} 字节", std::mem::size_of::<Option<&i32>>());
    println!("  Option<Box<i32>>: {} 字节", std::mem::size_of::<Option<Box<i32>>>());
    println!("  Vec<i32> (空): {} 字节", std::mem::size_of::<Vec<i32>>());
    println!("  String (空): {} 字节", std::mem::size_of::<String>());
    
    // 内存对齐分析
    println!("\n📐 内存对齐分析:");
    
    #[repr(C)]
    struct AlignedStruct {
        a: u8,
        b: u32,
        c: u8,
    }
    
    #[repr(packed)]
    struct PackedStruct {
        a: u8,
        b: u32,
        c: u8,
    }
    
    println!("  对齐结构体: {} 字节", std::mem::size_of::<AlignedStruct>());
    println!("  紧凑结构体: {} 字节", std::mem::size_of::<PackedStruct>());
    println!("  u8 对齐: {} 字节", std::mem::align_of::<u8>());
    println!("  u32 对齐: {} 字节", std::mem::align_of::<u32>());
    println!("  u64 对齐: {} 字节", std::mem::align_of::<u64>());
    
    // 堆栈内存使用
    println!("\n🏗️ 堆栈内存使用模式:");
    
    // 栈上数据
    let stack_array = [0i32; 1000];
    println!("  栈数组 (1000 个 i32): {} 字节", std::mem::size_of_val(&stack_array));
    
    // 堆上数据
    let heap_vec = vec![0i32; 1000];
    println!("  Vec 控制结构: {} 字节", std::mem::size_of_val(&heap_vec));
    println!("  Vec 数据大小: {} 字节", heap_vec.len() * std::mem::size_of::<i32>());
    
    let heap_box = Box::new([0i32; 1000]);
    println!("  Box 指针: {} 字节", std::mem::size_of_val(&heap_box));
    println!("  Box 数据大小: {} 字节", std::mem::size_of_val(&*heap_box));
    
    println!("  ✅ 引用和裸指针只占用指针大小，智能指针有额外开销");
}

/// 编译时优化分析
fn compile_time_optimization_analysis() {
    println!("\n🔧 7. 编译时优化分析");
    println!("{}", "-".repeat(40));
    
    const ITERATIONS: usize = 1_000_000;
    
    // 内联函数测试
    println!("\n📊 内联函数优化:");
    
    #[inline(always)]
    fn inlined_add(a: i64, b: i64) -> i64 {
        a + b
    }
    
    #[inline(never)]
    fn not_inlined_add(a: i64, b: i64) -> i64 {
        a + b
    }
    
    // 内联函数性能
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..ITERATIONS {
        sum = inlined_add(sum, i as i64);
    }
    let inlined_time = start.elapsed();
    println!("  内联函数: {:?} (结果: {})", inlined_time, sum);
    
    // 非内联函数性能
    let start = Instant::now();
    let mut sum = 0i64;
    for i in 0..ITERATIONS {
        sum = not_inlined_add(sum, i as i64);
    }
    let not_inlined_time = start.elapsed();
    println!("  非内联函数: {:?} (结果: {})", not_inlined_time, sum);
    
    // 零成本抽象测试
    println!("\n📊 零成本抽象:");
    
    // 迭代器链
    let data: Vec<i32> = (0..1000).collect();
    
    let start = Instant::now();
    let result: i32 = data.iter()
        .map(|x| x * 2)
        .filter(|&x| x % 4 == 0)
        .sum();
    let iterator_time = start.elapsed();
    println!("  迭代器链: {:?} (结果: {})", iterator_time, result);
    
    // 手动循环
    let start = Instant::now();
    let mut result = 0i32;
    for &x in &data {
        let doubled = x * 2;
        if doubled % 4 == 0 {
            result += doubled;
        }
    }
    let manual_time = start.elapsed();
    println!("  手动循环: {:?} (结果: {})", manual_time, result);
    
    // 引用解引用优化
    println!("\n📊 引用解引用优化:");
    
    let data = vec![1, 2, 3, 4, 5];
    let data_ref = &data;
    
    let start = Instant::now();
    let mut sum = 0i32;
    for _ in 0..ITERATIONS {
        sum += data_ref[2]; // 通过引用访问
    }
    let ref_access_time = start.elapsed();
    println!("  引用访问: {:?} (结果: {})", ref_access_time, sum);
    
    let start = Instant::now();
    let mut sum = 0i32;
    for _ in 0..ITERATIONS {
        sum += (*data_ref)[2]; // 显式解引用
    }
    let deref_access_time = start.elapsed();
    println!("  显式解引用: {:?} (结果: {})", deref_access_time, sum);
    
    // 性能比较
    println!("\n📈 优化效果比较:");
    println!("  内联 vs 非内联: {:.2}x", not_inlined_time.as_nanos() as f64 / inlined_time.as_nanos() as f64);
    println!("  迭代器 vs 手动: {:.2}x", iterator_time.as_nanos() as f64 / manual_time.as_nanos() as f64);
    println!("  引用 vs 解引用: {:.2}x", ref_access_time.as_nanos() as f64 / deref_access_time.as_nanos() as f64);
    
    println!("  ✅ Rust 编译器优化使抽象接近零成本");
}

/// 性能测试工具函数
mod benchmark_utils {
    use super::*;
    
    /// 简单的基准测试框架
    pub fn benchmark<F, R>(name: &str, iterations: usize, mut f: F) -> (Duration, R)
    where
        F: FnMut() -> R,
    {
        // 预热
        for _ in 0..iterations / 10 {
            std::hint::black_box(f());
        }
        
        let start = Instant::now();
        let mut result = None;
        
        for _ in 0..iterations {
            result = Some(f());
        }
        
        let elapsed = start.elapsed();
        println!("  {}: {:?} ({} 次迭代)", name, elapsed, iterations);
        
        (elapsed, result.unwrap())
    }
    
    /// 内存使用情况报告
    pub fn memory_report<T>(name: &str, value: &T) {
        println!("  {}: {} 字节 (对齐: {} 字节)", 
                 name, 
                 std::mem::size_of_val(value),
                 std::mem::align_of_val(value));
    }
}

/// 综合性能测试
pub fn comprehensive_performance_test() {
    println!("\n🎯 综合性能测试");
    println!("{}", "=".repeat(60));
    
    use benchmark_utils::*;
    
    const TEST_SIZE: usize = 100_000;
    let test_data: Vec<i32> = (0..TEST_SIZE as i32).collect();
    
    // 不同访问模式的性能对比
    println!("\n🔍 访问模式性能对比:");
    
    // 直接访问
    let (direct_time, direct_sum) = benchmark("直接访问", 100, || {
        test_data.iter().sum::<i32>()
    });
    
    // 引用访问
    let data_ref = &test_data;
    let (ref_time, ref_sum) = benchmark("引用访问", 100, || {
        data_ref.iter().sum::<i32>()
    });
    
    // Box 访问
    let boxed_data = Box::new(test_data.clone());
    let (box_time, box_sum) = benchmark("Box 访问", 100, || {
        boxed_data.iter().sum::<i32>()
    });
    
    // Rc 访问
    let rc_data = Rc::new(test_data.clone());
    let (rc_time, rc_sum) = benchmark("Rc 访问", 100, || {
        rc_data.iter().sum::<i32>()
    });
    
    // 验证结果一致性
    assert_eq!(direct_sum, ref_sum);
    assert_eq!(direct_sum, box_sum);
    assert_eq!(direct_sum, rc_sum);
    
    println!("\n📊 性能总结 (相对于直接访问):");
    println!("  直接访问: 1.00x");
    println!("  引用访问: {:.2}x", ref_time.as_nanos() as f64 / direct_time.as_nanos() as f64);
    println!("  Box 访问: {:.2}x", box_time.as_nanos() as f64 / direct_time.as_nanos() as f64);
    println!("  Rc 访问: {:.2}x", rc_time.as_nanos() as f64 / direct_time.as_nanos() as f64);
    
    // 内存使用报告
    println!("\n💾 内存使用报告:");
    memory_report("Vec<i32>", &test_data);
    memory_report("&Vec<i32>", &data_ref);
    memory_report("Box<Vec<i32>>", &boxed_data);
    memory_report("Rc<Vec<i32>>", &rc_data);
    
    println!("\n🎉 性能测试完成！");
    println!("\n💡 关键发现:");
    println!("  • 引用访问与直接访问性能相当");
    println!("  • Box 增加一层间接访问，但影响很小");
    println!("  • Rc 有引用计数开销，但在读取时影响不大");
    println!("  • 内存布局对性能有重要影响");
    println!("  • 编译器优化能消除大部分抽象开销");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_sizes() {
        // 验证基本类型大小
        assert_eq!(std::mem::size_of::<&i32>(), std::mem::size_of::<usize>());
        assert_eq!(std::mem::size_of::<*const i32>(), std::mem::size_of::<usize>());
        assert_eq!(std::mem::size_of::<*mut i32>(), std::mem::size_of::<usize>());
        
        // Box 只是一个指针
        assert_eq!(std::mem::size_of::<Box<i32>>(), std::mem::size_of::<usize>());
        
        // Rc 包含指针和引用计数
        assert!(std::mem::size_of::<Rc<i32>>() >= std::mem::size_of::<usize>());
    }
    
    #[test]
    fn test_performance_consistency() {
        // 确保不同访问方式结果一致
        let data = vec![1, 2, 3, 4, 5];
        let data_ref = &data;
        let boxed_data = Box::new(data.clone());
        let rc_data = Rc::new(data.clone());
        
        let sum1: i32 = data.iter().sum();
        let sum2: i32 = data_ref.iter().sum();
        let sum3: i32 = boxed_data.iter().sum();
        let sum4: i32 = rc_data.iter().sum();
        
        assert_eq!(sum1, sum2);
        assert_eq!(sum1, sum3);
        assert_eq!(sum1, sum4);
    }
    
    #[test]
    fn test_atomic_operations() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        
        let counter = AtomicUsize::new(0);
        
        // 测试不同的内存顺序
        counter.store(10, Ordering::Relaxed);
        assert_eq!(counter.load(Ordering::Relaxed), 10);
        
        counter.store(20, Ordering::SeqCst);
        assert_eq!(counter.load(Ordering::SeqCst), 20);
        
        let old = counter.fetch_add(5, Ordering::Relaxed);
        assert_eq!(old, 20);
        assert_eq!(counter.load(Ordering::Relaxed), 25);
    }
}