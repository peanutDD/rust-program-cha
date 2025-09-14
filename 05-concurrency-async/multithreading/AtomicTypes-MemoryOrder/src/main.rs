//! Rust 原子类型和内存顺序全面教程
//! 
//! 本教程深入探讨 Rust 中的原子类型、内存顺序、无锁编程等核心概念
//! 涵盖从基础概念到高级应用的完整知识体系

use std::sync::atomic::{
    AtomicBool, AtomicI32, AtomicIsize, AtomicPtr, AtomicU32, AtomicU64, AtomicUsize,
    Ordering, fence
};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{Duration, Instant};
use std::ptr;
use std::cell::UnsafeCell;

fn main() {
    println!("{}", "=".repeat(80));
    println!("🚀 Rust 原子类型和内存顺序全面教程");
    println!("{}", "=".repeat(80));
    
    // 1. 原子类型基础
    atomic_types_basics_demo();
    
    // 2. 内存顺序概念
    memory_ordering_concepts_demo();
    
    // 3. 原子操作详解
    atomic_operations_demo();
    
    // 4. 内存屏障和栅栏
    memory_barriers_fences_demo();
    
    // 5. 无锁数据结构
    lock_free_data_structures_demo();
    
    // 6. 性能分析
    performance_analysis_demo();
    
    // 7. 高级模式
    advanced_patterns_demo();
    
    // 8. 实际应用案例
    real_world_applications_demo();
    
    println!("{}", "=".repeat(80));
    println!("✅ 教程完成！掌握了 Rust 原子类型和内存顺序的核心知识");
    println!("{}", "=".repeat(80));
}

/// 1. 原子类型基础演示
fn atomic_types_basics_demo() {
    println!("\n📚 1. 原子类型基础");
    println!("{}", "-".repeat(50));
    
    // AtomicBool 基础操作
    println!("\n🔹 AtomicBool 基础操作:");
    let atomic_bool = AtomicBool::new(false);
    println!("初始值: {}", atomic_bool.load(Ordering::Relaxed));
    
    atomic_bool.store(true, Ordering::Relaxed);
    println!("存储后: {}", atomic_bool.load(Ordering::Relaxed));
    
    let old_value = atomic_bool.swap(false, Ordering::Relaxed);
    println!("交换操作 - 旧值: {}, 新值: {}", old_value, atomic_bool.load(Ordering::Relaxed));
    
    // AtomicI32 基础操作
    println!("\n🔹 AtomicI32 基础操作:");
    let atomic_i32 = AtomicI32::new(10);
    println!("初始值: {}", atomic_i32.load(Ordering::Relaxed));
    
    let old_val = atomic_i32.fetch_add(5, Ordering::Relaxed);
    println!("fetch_add(5) - 旧值: {}, 新值: {}", old_val, atomic_i32.load(Ordering::Relaxed));
    
    let old_val = atomic_i32.fetch_sub(3, Ordering::Relaxed);
    println!("fetch_sub(3) - 旧值: {}, 新值: {}", old_val, atomic_i32.load(Ordering::Relaxed));
    
    // AtomicUsize 基础操作
    println!("\n🔹 AtomicUsize 基础操作:");
    let atomic_usize = AtomicUsize::new(100);
    println!("初始值: {}", atomic_usize.load(Ordering::Relaxed));
    
    // 位运算操作
    let old_val = atomic_usize.fetch_and(0b1111_0000, Ordering::Relaxed);
    println!("fetch_and(0b1111_0000) - 旧值: {}, 新值: {}", old_val, atomic_usize.load(Ordering::Relaxed));
    
    let old_val = atomic_usize.fetch_or(0b0000_1111, Ordering::Relaxed);
    println!("fetch_or(0b0000_1111) - 旧值: {}, 新值: {}", old_val, atomic_usize.load(Ordering::Relaxed));
    
    // AtomicPtr 基础操作
    println!("\n🔹 AtomicPtr 基础操作:");
    let mut data = 42i32;
    let atomic_ptr: AtomicPtr<i32> = AtomicPtr::new(&mut data);
    
    let ptr = atomic_ptr.load(Ordering::Relaxed);
    if !ptr.is_null() {
        unsafe {
            println!("指针指向的值: {}", *ptr);
        }
    }
    
    // 设置为空指针
    atomic_ptr.store(ptr::null_mut(), Ordering::Relaxed);
    println!("设置为空指针后: {:?}", atomic_ptr.load(Ordering::Relaxed));
    
    // 多线程原子操作示例
    println!("\n🔹 多线程原子操作示例:");
    atomic_counter_example();
}

/// 原子计数器多线程示例
fn atomic_counter_example() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    // 启动10个线程，每个线程增加1000次
    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
            println!("线程 {} 完成", i);
        });
        handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终计数器值: {} (期望: 10000)", counter.load(Ordering::Relaxed));
}

/// 2. 内存顺序概念演示
fn memory_ordering_concepts_demo() {
    println!("\n📚 2. 内存顺序概念");
    println!("{}", "-".repeat(50));
    
    println!("\n🔹 内存顺序类型说明:");
    println!("• Relaxed: 最宽松的顺序，只保证原子性，不保证顺序");
    println!("• Acquire: 获取语义，用于读操作，防止后续操作重排到此操作之前");
    println!("• Release: 释放语义，用于写操作，防止前面操作重排到此操作之后");
    println!("• AcqRel: 获取-释放语义，结合了 Acquire 和 Release");
    println!("• SeqCst: 顺序一致性，最强的内存顺序，全局统一顺序");
    
    // Relaxed 顺序示例
    println!("\n🔹 Relaxed 顺序示例:");
    relaxed_ordering_example();
    
    // Acquire-Release 顺序示例
    println!("\n🔹 Acquire-Release 顺序示例:");
    acquire_release_example();
    
    // SeqCst 顺序示例
    println!("\n🔹 SeqCst 顺序示例:");
    seq_cst_example();
    
    // 内存重排序演示
    println!("\n🔹 内存重排序演示:");
    memory_reordering_demo();
}

/// Relaxed 顺序示例
fn relaxed_ordering_example() {
    let flag = Arc::new(AtomicBool::new(false));
    let data = Arc::new(AtomicI32::new(0));
    
    let flag_clone = Arc::clone(&flag);
    let data_clone = Arc::clone(&data);
    
    // 生产者线程
    let producer = thread::spawn(move || {
        data_clone.store(42, Ordering::Relaxed);
        flag_clone.store(true, Ordering::Relaxed);
        println!("生产者: 数据已设置，标志已设置");
    });
    
    // 消费者线程
    let consumer = thread::spawn(move || {
        while !flag.load(Ordering::Relaxed) {
            thread::yield_now();
        }
        let value = data.load(Ordering::Relaxed);
        println!("消费者: 读取到数据 = {}", value);
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
    println!("注意: Relaxed 顺序不保证数据和标志的可见性顺序");
}

/// Acquire-Release 顺序示例
fn acquire_release_example() {
    let flag = Arc::new(AtomicBool::new(false));
    let data = Arc::new(AtomicI32::new(0));
    
    let flag_clone = Arc::clone(&flag);
    let data_clone = Arc::clone(&data);
    
    // 生产者线程 - 使用 Release
    let producer = thread::spawn(move || {
        data_clone.store(100, Ordering::Relaxed);
        flag_clone.store(true, Ordering::Release); // Release 确保数据写入在标志设置之前可见
        println!("生产者: 使用 Release 顺序设置标志");
    });
    
    // 消费者线程 - 使用 Acquire
    let consumer = thread::spawn(move || {
        while !flag.load(Ordering::Acquire) { // Acquire 确保看到标志后能看到之前的数据写入
            thread::yield_now();
        }
        let value = data.load(Ordering::Relaxed);
        println!("消费者: 使用 Acquire 顺序读取，数据 = {}", value);
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
    println!("Acquire-Release 保证了数据的可见性顺序");
}

/// SeqCst 顺序示例
fn seq_cst_example() {
    let x = Arc::new(AtomicI32::new(0));
    let y = Arc::new(AtomicI32::new(0));
    let z1 = Arc::new(AtomicI32::new(0));
    let z2 = Arc::new(AtomicI32::new(0));
    
    let x1 = Arc::clone(&x);
    let y1 = Arc::clone(&y);
    let z1_clone = Arc::clone(&z1);
    
    let x2 = Arc::clone(&x);
    let y2 = Arc::clone(&y);
    let z2_clone = Arc::clone(&z2);
    
    // 线程1
    let t1 = thread::spawn(move || {
        x1.store(1, Ordering::SeqCst);
        let val = y1.load(Ordering::SeqCst);
        z1_clone.store(val, Ordering::SeqCst);
    });
    
    // 线程2
    let t2 = thread::spawn(move || {
        y2.store(1, Ordering::SeqCst);
        let val = x2.load(Ordering::SeqCst);
        z2_clone.store(val, Ordering::SeqCst);
    });
    
    t1.join().unwrap();
    t2.join().unwrap();
    
    let z1_val = z1.load(Ordering::SeqCst);
    let z2_val = z2.load(Ordering::SeqCst);
    
    println!("SeqCst 结果: z1 = {}, z2 = {}", z1_val, z2_val);
    println!("SeqCst 保证全局统一的操作顺序，不会出现 z1=0 且 z2=0 的情况");
}

/// 内存重排序演示
fn memory_reordering_demo() {
    struct ReorderingExample {
        data: AtomicI32,
        flag: AtomicBool,
    }
    
    let example = Arc::new(ReorderingExample {
        data: AtomicI32::new(0),
        flag: AtomicBool::new(false),
    });
    
    let example_clone = Arc::clone(&example);
    
    // 写线程
    let writer = thread::spawn(move || {
        example_clone.data.store(42, Ordering::Relaxed);
        example_clone.flag.store(true, Ordering::Relaxed);
    });
    
    // 读线程
    let reader = thread::spawn(move || {
        if example.flag.load(Ordering::Relaxed) {
            let data = example.data.load(Ordering::Relaxed);
            println!("读取到数据: {}", data);
        }
    });
    
    writer.join().unwrap();
    reader.join().unwrap();
    
    println!("在弱内存模型的处理器上，可能出现重排序现象");
}

/// 3. 原子操作详解演示
fn atomic_operations_demo() {
    println!("\n📚 3. 原子操作详解");
    println!("{}", "-".repeat(50));
    
    // compare_exchange 操作
    println!("\n🔹 compare_exchange 操作:");
    compare_exchange_demo();
    
    // compare_exchange_weak 操作
    println!("\n🔹 compare_exchange_weak 操作:");
    compare_exchange_weak_demo();
    
    // fetch 系列操作
    println!("\n🔹 fetch 系列操作:");
    fetch_operations_demo();
    
    // 原子操作的返回值
    println!("\n🔹 原子操作的返回值:");
    atomic_return_values_demo();
}

/// compare_exchange 操作演示
fn compare_exchange_demo() {
    let atomic = AtomicI32::new(10);
    
    // 成功的 CAS 操作
    match atomic.compare_exchange(10, 20, Ordering::SeqCst, Ordering::Relaxed) {
        Ok(old_val) => println!("CAS 成功: 旧值 = {}, 新值 = {}", old_val, atomic.load(Ordering::Relaxed)),
        Err(current) => println!("CAS 失败: 期望 10, 实际 {}", current),
    }
    
    // 失败的 CAS 操作
    match atomic.compare_exchange(10, 30, Ordering::SeqCst, Ordering::Relaxed) {
        Ok(old_val) => println!("CAS 成功: 旧值 = {}", old_val),
        Err(current) => println!("CAS 失败: 期望 10, 实际 {}", current),
    }
    
    // 实现原子递增
    println!("\n原子递增实现:");
    let counter = AtomicI32::new(0);
    for _ in 0..5 {
        let mut current = counter.load(Ordering::Relaxed);
        loop {
            match counter.compare_exchange_weak(current, current + 1, Ordering::Relaxed, Ordering::Relaxed) {
                Ok(_) => break,
                Err(new_current) => current = new_current,
            }
        }
        println!("递增后的值: {}", counter.load(Ordering::Relaxed));
    }
}

/// compare_exchange_weak 操作演示
fn compare_exchange_weak_demo() {
    let atomic = AtomicI32::new(100);
    
    println!("使用 compare_exchange_weak 实现自旋锁式递增:");
    
    // 模拟多次尝试
    let mut attempts = 0;
    let mut current = atomic.load(Ordering::Relaxed);
    
    loop {
        attempts += 1;
        match atomic.compare_exchange_weak(current, current + 10, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(old_val) => {
                println!("成功! 尝试次数: {}, 旧值: {}, 新值: {}", attempts, old_val, atomic.load(Ordering::Relaxed));
                break;
            }
            Err(new_current) => {
                current = new_current;
                if attempts > 10 {
                    println!("尝试次数过多，退出");
                    break;
                }
            }
        }
    }
}

/// fetch 系列操作演示
fn fetch_operations_demo() {
    let atomic = AtomicI32::new(50);
    
    println!("初始值: {}", atomic.load(Ordering::Relaxed));
    
    // fetch_add
    let old = atomic.fetch_add(10, Ordering::Relaxed);
    println!("fetch_add(10): 旧值 = {}, 新值 = {}", old, atomic.load(Ordering::Relaxed));
    
    // fetch_sub
    let old = atomic.fetch_sub(5, Ordering::Relaxed);
    println!("fetch_sub(5): 旧值 = {}, 新值 = {}", old, atomic.load(Ordering::Relaxed));
    
    // fetch_and (位与)
    let old = atomic.fetch_and(0b111111, Ordering::Relaxed);
    println!("fetch_and(0b111111): 旧值 = {}, 新值 = {}", old, atomic.load(Ordering::Relaxed));
    
    // fetch_or (位或)
    let old = atomic.fetch_or(0b1000000, Ordering::Relaxed);
    println!("fetch_or(0b1000000): 旧值 = {}, 新值 = {}", old, atomic.load(Ordering::Relaxed));
    
    // fetch_xor (位异或)
    let old = atomic.fetch_xor(0b1111111, Ordering::Relaxed);
    println!("fetch_xor(0b1111111): 旧值 = {}, 新值 = {}", old, atomic.load(Ordering::Relaxed));
    
    // fetch_max 和 fetch_min (需要 nightly 或较新版本)
    let atomic_u32 = AtomicU32::new(30);
    let old = atomic_u32.fetch_max(25, Ordering::Relaxed);
    println!("fetch_max(25): 旧值 = {}, 新值 = {}", old, atomic_u32.load(Ordering::Relaxed));
    
    let old = atomic_u32.fetch_min(35, Ordering::Relaxed);
    println!("fetch_min(35): 旧值 = {}, 新值 = {}", old, atomic_u32.load(Ordering::Relaxed));
}

/// 原子操作返回值演示
fn atomic_return_values_demo() {
    println!("原子操作返回值的重要性:");
    
    let counter = AtomicUsize::new(0);
    
    // 使用返回值实现条件逻辑
    for i in 0..10 {
        let old_val = counter.fetch_add(1, Ordering::Relaxed);
        if old_val % 3 == 0 {
            println!("第 {} 次操作，旧值 {} 是3的倍数", i + 1, old_val);
        }
    }
    
    println!("最终计数器值: {}", counter.load(Ordering::Relaxed));
    
    // 使用 swap 实现状态切换
    let state = AtomicI32::new(1);
    println!("\n状态切换演示:");
    for i in 0..5 {
        let old_state = state.swap(if state.load(Ordering::Relaxed) == 1 { 2 } else { 1 }, Ordering::Relaxed);
        println!("第 {} 次切换: {} -> {}", i + 1, old_state, state.load(Ordering::Relaxed));
    }
}

/// 4. 内存屏障和栅栏演示
fn memory_barriers_fences_demo() {
    println!("\n📚 4. 内存屏障和栅栏");
    println!("{}", "-".repeat(50));
    
    // fence 操作基础
    println!("\n🔹 fence 操作基础:");
    fence_basics_demo();
    
    // 编译器重排序控制
    println!("\n🔹 编译器重排序控制:");
    compiler_reordering_demo();
    
    // CPU 重排序控制
    println!("\n🔹 CPU 重排序控制:");
    cpu_reordering_demo();
    
    // fence 与原子操作的配合
    println!("\n🔹 fence 与原子操作的配合:");
    fence_with_atomics_demo();
}

/// fence 操作基础演示
fn fence_basics_demo() {
    println!("fence 操作类型:");
    println!("• fence(Acquire): 获取栅栏，防止后续读操作重排到栅栏之前");
    println!("• fence(Release): 释放栅栏，防止前面写操作重排到栅栏之后");
    println!("• fence(AcqRel): 获取-释放栅栏，结合两者效果");
    println!("• fence(SeqCst): 顺序一致性栅栏，最强的内存屏障");
    
    // 基本 fence 使用
    let data = AtomicI32::new(0);
    let flag = AtomicBool::new(false);
    
    // 写操作
    data.store(42, Ordering::Relaxed);
    fence(Ordering::Release); // 确保数据写入在栅栏之前完成
    flag.store(true, Ordering::Relaxed);
    
    // 读操作
    if flag.load(Ordering::Relaxed) {
        fence(Ordering::Acquire); // 确保后续读操作在栅栏之后
        let value = data.load(Ordering::Relaxed);
        println!("通过 fence 同步读取到数据: {}", value);
    }
}

/// 编译器重排序控制演示
fn compiler_reordering_demo() {
    println!("编译器重排序控制示例:");
    
    let x = AtomicI32::new(0);
    let y = AtomicI32::new(0);
    
    // 没有 fence 的情况下，编译器可能重排序
    x.store(1, Ordering::Relaxed);
    y.store(2, Ordering::Relaxed);
    
    println!("普通存储: x = {}, y = {}", x.load(Ordering::Relaxed), y.load(Ordering::Relaxed));
    
    // 使用 fence 防止重排序
    x.store(10, Ordering::Relaxed);
    fence(Ordering::Release);
    y.store(20, Ordering::Relaxed);
    
    println!("使用 fence 后: x = {}, y = {}", x.load(Ordering::Relaxed), y.load(Ordering::Relaxed));
}

/// CPU 重排序控制演示
fn cpu_reordering_demo() {
    println!("CPU 重排序控制在多线程环境中的作用:");
    
    let data = Arc::new(AtomicI32::new(0));
    let ready = Arc::new(AtomicBool::new(false));
    
    let data_clone = Arc::clone(&data);
    let ready_clone = Arc::clone(&ready);
    
    // 生产者线程
    let producer = thread::spawn(move || {
        data_clone.store(100, Ordering::Relaxed);
        fence(Ordering::Release); // CPU 级别的内存屏障
        ready_clone.store(true, Ordering::Relaxed);
        println!("生产者: 数据已准备，使用 Release fence");
    });
    
    // 消费者线程
    let consumer = thread::spawn(move || {
        while !ready.load(Ordering::Relaxed) {
            thread::yield_now();
        }
        fence(Ordering::Acquire); // CPU 级别的内存屏障
        let value = data.load(Ordering::Relaxed);
        println!("消费者: 使用 Acquire fence 读取数据 = {}", value);
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
}

/// fence 与原子操作配合演示
fn fence_with_atomics_demo() {
    println!("fence 与原子操作的配合使用:");
    
    struct Message {
        data: [AtomicI32; 4],
        ready: AtomicBool,
    }
    
    let message = Arc::new(Message {
        data: [AtomicI32::new(0), AtomicI32::new(0), AtomicI32::new(0), AtomicI32::new(0)],
        ready: AtomicBool::new(false),
    });
    
    let message_clone = Arc::clone(&message);
    
    // 发送者
    let sender = thread::spawn(move || {
        // 写入数据
        for (i, atomic) in message_clone.data.iter().enumerate() {
            atomic.store((i + 1) as i32 * 10, Ordering::Relaxed);
        }
        
        // 使用 fence 确保所有数据写入完成
        fence(Ordering::Release);
        message_clone.ready.store(true, Ordering::Relaxed);
        println!("发送者: 消息已发送");
    });
    
    // 接收者
    let receiver = thread::spawn(move || {
        // 等待消息准备就绪
        while !message.ready.load(Ordering::Relaxed) {
            thread::yield_now();
        }
        
        // 使用 fence 确保能看到所有数据
        fence(Ordering::Acquire);
        
        print!("接收者: 收到消息 [");
        for (i, atomic) in message.data.iter().enumerate() {
            if i > 0 { print!(", "); }
            print!("{}", atomic.load(Ordering::Relaxed));
        }
        println!("]");
    });
    
    sender.join().unwrap();
    receiver.join().unwrap();
}

/// 5. 无锁数据结构演示
fn lock_free_data_structures_demo() {
    println!("\n📚 5. 无锁数据结构");
    println!("{}", "-".repeat(50));
    
    // 无锁栈
    println!("\n🔹 无锁栈实现:");
    lock_free_stack_demo();
    
    // 无锁队列
    println!("\n🔹 无锁队列实现:");
    lock_free_queue_demo();
    
    // 无锁计数器
    println!("\n🔹 无锁计数器实现:");
    lock_free_counter_demo();
    
    // 无锁哈希表
    println!("\n🔹 简单无锁哈希表:");
    simple_lock_free_hash_demo();
}

/// 无锁栈实现
struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> LockFreeStack<T> {
    fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }
    
    fn push(&self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: ptr::null_mut(),
        }));
        
        loop {
            let head = self.head.load(Ordering::Relaxed);
            unsafe {
                (*new_node).next = head;
            }
            
            match self.head.compare_exchange_weak(
                head,
                new_node,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(_) => continue,
            }
        }
    }
    
    fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }
            
            let next = unsafe { (*head).next };
            
            match self.head.compare_exchange_weak(
                head,
                next,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    let data = unsafe { Box::from_raw(head).data };
                    return Some(data);
                }
                Err(_) => continue,
            }
        }
    }
}

unsafe impl<T: Send> Send for LockFreeStack<T> {}
unsafe impl<T: Send> Sync for LockFreeStack<T> {}

fn lock_free_stack_demo() {
    let stack = Arc::new(LockFreeStack::new());
    let mut handles = vec![];
    
    // 生产者线程
    for i in 0..5 {
        let stack_clone = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            for j in 0..10 {
                stack_clone.push(i * 10 + j);
            }
            println!("生产者 {} 完成", i);
        });
        handles.push(handle);
    }
    
    // 消费者线程
    for i in 0..3 {
        let stack_clone = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            let mut count = 0;
            while count < 15 {
                if let Some(value) = stack_clone.pop() {
                    count += 1;
                    if count <= 5 {
                        println!("消费者 {} 弹出: {}", i, value);
                    }
                } else {
                    thread::yield_now();
                }
            }
            println!("消费者 {} 完成，共弹出 {} 个元素", i, count);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

/// 简单无锁队列实现（单生产者单消费者）
struct SimpleLockFreeQueue<T> {
    buffer: Vec<UnsafeCell<Option<T>>>,
    head: AtomicUsize,
    tail: AtomicUsize,
    capacity: usize,
}

impl<T> SimpleLockFreeQueue<T> {
    fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buffer.push(UnsafeCell::new(None));
        }
        
        Self {
            buffer,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            capacity,
        }
    }
    
    fn enqueue(&self, item: T) -> Result<(), T> {
        let tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (tail + 1) % self.capacity;
        
        if next_tail == self.head.load(Ordering::Acquire) {
            return Err(item); // 队列满
        }
        
        unsafe {
            *self.buffer[tail].get() = Some(item);
        }
        
        self.tail.store(next_tail, Ordering::Release);
        Ok(())
    }
    
    fn dequeue(&self) -> Option<T> {
        let head = self.head.load(Ordering::Relaxed);
        
        if head == self.tail.load(Ordering::Acquire) {
            return None; // 队列空
        }
        
        let item = unsafe { (*self.buffer[head].get()).take() };
        let next_head = (head + 1) % self.capacity;
        self.head.store(next_head, Ordering::Release);
        
        item
    }
}

unsafe impl<T: Send> Send for SimpleLockFreeQueue<T> {}
unsafe impl<T: Send> Sync for SimpleLockFreeQueue<T> {}

fn lock_free_queue_demo() {
    let queue = Arc::new(SimpleLockFreeQueue::new(100));
    
    let queue_producer = Arc::clone(&queue);
    let queue_consumer = Arc::clone(&queue);
    
    // 生产者
    let producer = thread::spawn(move || {
        for i in 0..50 {
            while queue_producer.enqueue(i).is_err() {
                thread::yield_now();
            }
            if i < 10 {
                println!("入队: {}", i);
            }
        }
        println!("生产者完成，共入队 50 个元素");
    });
    
    // 消费者
    let consumer = thread::spawn(move || {
        let mut count = 0;
        while count < 50 {
            if let Some(item) = queue_consumer.dequeue() {
                count += 1;
                if count <= 10 {
                    println!("出队: {}", item);
                }
            } else {
                thread::yield_now();
            }
        }
        println!("消费者完成，共出队 {} 个元素", count);
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
}

/// 无锁计数器演示
fn lock_free_counter_demo() {
    struct LockFreeCounter {
        value: AtomicU64,
        increments: AtomicU64,
        decrements: AtomicU64,
    }
    
    impl LockFreeCounter {
        fn new() -> Self {
            Self {
                value: AtomicU64::new(0),
                increments: AtomicU64::new(0),
                decrements: AtomicU64::new(0),
            }
        }
        
        fn increment(&self) -> u64 {
            self.increments.fetch_add(1, Ordering::Relaxed);
            self.value.fetch_add(1, Ordering::Relaxed)
        }
        
        fn decrement(&self) -> u64 {
            self.decrements.fetch_add(1, Ordering::Relaxed);
            self.value.fetch_sub(1, Ordering::Relaxed)
        }
        
        fn get(&self) -> u64 {
            self.value.load(Ordering::Relaxed)
        }
        
        fn stats(&self) -> (u64, u64, u64) {
            (
                self.value.load(Ordering::Relaxed),
                self.increments.load(Ordering::Relaxed),
                self.decrements.load(Ordering::Relaxed),
            )
        }
    }
    
    let counter = Arc::new(LockFreeCounter::new());
    let mut handles = vec![];
    
    // 递增线程
    for i in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter_clone.increment();
            }
            println!("递增线程 {} 完成", i);
        });
        handles.push(handle);
    }
    
    // 递减线程
    for i in 0..3 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..500 {
                counter_clone.decrement();
            }
            println!("递减线程 {} 完成", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let (value, increments, decrements) = counter.stats();
    println!("最终统计: 值={}, 递增次数={}, 递减次数={}", value, increments, decrements);
    println!("期望值: {} (5*1000 - 3*500 = 3500)", 5 * 1000 - 3 * 500);
}

/// 简单无锁哈希表演示
fn simple_lock_free_hash_demo() {
    const BUCKET_COUNT: usize = 16;
    
    struct SimpleLockFreeHashMap {
        buckets: [AtomicPtr<HashNode>; BUCKET_COUNT],
    }
    
    struct HashNode {
        key: u32,
        value: u32,
        next: *mut HashNode,
    }
    
    impl SimpleLockFreeHashMap {
        fn new() -> Self {
            const NULL_PTR: AtomicPtr<HashNode> = AtomicPtr::new(ptr::null_mut());
            Self {
                buckets: [NULL_PTR; BUCKET_COUNT],
            }
        }
        
        fn hash(&self, key: u32) -> usize {
            (key as usize) % BUCKET_COUNT
        }
        
        fn insert(&self, key: u32, value: u32) {
            let bucket_idx = self.hash(key);
            let new_node = Box::into_raw(Box::new(HashNode {
                key,
                value,
                next: ptr::null_mut(),
            }));
            
            loop {
                let head = self.buckets[bucket_idx].load(Ordering::Acquire);
                unsafe {
                    (*new_node).next = head;
                }
                
                match self.buckets[bucket_idx].compare_exchange_weak(
                    head,
                    new_node,
                    Ordering::Release,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => break,
                    Err(_) => continue,
                }
            }
        }
        
        fn get(&self, key: u32) -> Option<u32> {
            let bucket_idx = self.hash(key);
            let mut current = self.buckets[bucket_idx].load(Ordering::Acquire);
            
            while !current.is_null() {
                unsafe {
                    if (*current).key == key {
                        return Some((*current).value);
                    }
                    current = (*current).next;
                }
            }
            
            None
        }
    }
    
    let hashmap = Arc::new(SimpleLockFreeHashMap::new());
    let mut handles = vec![];
    
    // 插入线程
    for i in 0..4 {
        let hashmap_clone = Arc::clone(&hashmap);
        let handle = thread::spawn(move || {
            for j in 0..25 {
                let key = i * 25 + j;
                let value = key * 10;
                hashmap_clone.insert(key, value);
            }
            println!("插入线程 {} 完成", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // 验证插入的数据
    println!("验证插入的数据:");
    for i in 0..10 {
        if let Some(value) = hashmap.get(i) {
            println!("key: {}, value: {}", i, value);
        }
    }
    
    // 统计每个桶的元素数量
    println!("\n桶分布统计:");
    for i in 0..BUCKET_COUNT {
        let mut count = 0;
        let mut current = hashmap.buckets[i].load(Ordering::Acquire);
        while !current.is_null() {
            count += 1;
            unsafe {
                current = (*current).next;
            }
        }
        if count > 0 {
            println!("桶 {}: {} 个元素", i, count);
        }
    }
}

/// 6. 性能分析演示
fn performance_analysis_demo() {
    println!("\n📚 6. 性能分析");
    println!("{}", "-".repeat(50));
    
    // 原子操作 vs 锁性能对比
    println!("\n🔹 原子操作 vs 锁性能对比:");
    atomic_vs_mutex_performance();
    
    // 不同内存顺序性能对比
    println!("\n🔹 不同内存顺序性能对比:");
    memory_ordering_performance();
    
    // 硬件架构影响
    println!("\n🔹 硬件架构影响分析:");
    hardware_architecture_analysis();
    
    // 缓存行影响
    println!("\n🔹 缓存行影响分析:");
    cache_line_analysis();
}

/// 原子操作 vs 锁性能对比
fn atomic_vs_mutex_performance() {
    use std::sync::Mutex;
    
    const ITERATIONS: usize = 1_000_000;
    const THREADS: usize = 4;
    
    // 原子操作性能测试
    let atomic_counter = Arc::new(AtomicUsize::new(0));
    let start = Instant::now();
    
    let mut handles = vec![];
    for _ in 0..THREADS {
        let counter = Arc::clone(&atomic_counter);
        let handle = thread::spawn(move || {
            for _ in 0..ITERATIONS / THREADS {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let atomic_duration = start.elapsed();
    let atomic_result = atomic_counter.load(Ordering::Relaxed);
    
    // 互斥锁性能测试
    let mutex_counter = Arc::new(Mutex::new(0usize));
    let start = Instant::now();
    
    let mut handles = vec![];
    for _ in 0..THREADS {
        let counter = Arc::clone(&mutex_counter);
        let handle = thread::spawn(move || {
            for _ in 0..ITERATIONS / THREADS {
                let mut guard = counter.lock().unwrap();
                *guard += 1;
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let mutex_duration = start.elapsed();
    let mutex_result = *mutex_counter.lock().unwrap();
    
    println!("原子操作: {} 次操作，耗时 {:?}，结果 {}", ITERATIONS, atomic_duration, atomic_result);
    println!("互斥锁:   {} 次操作，耗时 {:?}，结果 {}", ITERATIONS, mutex_duration, mutex_result);
    
    let speedup = mutex_duration.as_nanos() as f64 / atomic_duration.as_nanos() as f64;
    println!("原子操作比互斥锁快 {:.2} 倍", speedup);
}

/// 不同内存顺序性能对比
fn memory_ordering_performance() {
    const ITERATIONS: usize = 10_000_000;
    
    let orderings = [
        ("Relaxed", Ordering::Relaxed),
        ("Acquire", Ordering::Acquire),
        ("Release", Ordering::Release),
        ("SeqCst", Ordering::SeqCst),
    ];
    
    for (name, ordering) in orderings.iter() {
        let counter = AtomicUsize::new(0);
        let start = Instant::now();
        
        for _ in 0..ITERATIONS {
            counter.fetch_add(1, *ordering);
        }
        
        let duration = start.elapsed();
        println!("{:8}: {} 次操作，耗时 {:?}", name, ITERATIONS, duration);
    }
    
    // 读操作性能对比
    println!("\n读操作性能对比:");
    let value = AtomicUsize::new(42);
    
    let read_orderings = [
        ("Relaxed", Ordering::Relaxed),
        ("Acquire", Ordering::Acquire),
        ("SeqCst", Ordering::SeqCst),
    ];
    
    for (name, ordering) in read_orderings.iter() {
        let start = Instant::now();
        
        for _ in 0..ITERATIONS {
            let _ = value.load(*ordering);
        }
        
        let duration = start.elapsed();
        println!("{:8}: {} 次读取，耗时 {:?}", name, ITERATIONS, duration);
    }
}

/// 硬件架构影响分析
fn hardware_architecture_analysis() {
    println!("硬件架构对原子操作的影响:");
    println!("• x86/x86_64: 强内存模型，Acquire/Release 开销较小");
    println!("• ARM: 弱内存模型，需要更多内存屏障指令");
    println!("• RISC-V: 弱内存模型，类似 ARM");
    
    // 模拟不同架构下的性能差异
    let counter = AtomicUsize::new(0);
    
    // 测试 compare_exchange 性能
    let start = Instant::now();
    let mut successes = 0;
    
    for i in 0..100_000 {
        match counter.compare_exchange(
            i,
            i + 1,
            Ordering::SeqCst,
            Ordering::Relaxed,
        ) {
            Ok(_) => successes += 1,
            Err(_) => {},
        }
    }
    
    let duration = start.elapsed();
    println!("compare_exchange 测试: {} 次成功，耗时 {:?}", successes, duration);
    
    // CPU 缓存行大小影响
    println!("\n当前平台信息:");
    println!("• 指针大小: {} 字节", std::mem::size_of::<*const u8>());
    println!("• AtomicUsize 大小: {} 字节", std::mem::size_of::<AtomicUsize>());
    println!("• AtomicBool 大小: {} 字节", std::mem::size_of::<AtomicBool>());
}

/// 缓存行影响分析
fn cache_line_analysis() {
    const CACHE_LINE_SIZE: usize = 64; // 典型的缓存行大小
    
    println!("缓存行对性能的影响:");
    
    // 创建紧密排列的原子变量（可能在同一缓存行）
    #[repr(align(64))]
    struct AlignedAtomics {
        counters: [AtomicUsize; 8],
    }
    
    let aligned_atomics = Arc::new(AlignedAtomics {
        counters: [
            AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0),
            AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0),
        ],
    });
    
    // 测试同一缓存行上的竞争
    println!("\n测试缓存行竞争:");
    let start = Instant::now();
    
    let mut handles = vec![];
    for i in 0..4 {
        let atomics = Arc::clone(&aligned_atomics);
        let handle = thread::spawn(move || {
            for _ in 0..100_000 {
                // 所有线程操作相邻的原子变量，可能导致缓存行竞争
                atomics.counters[i].fetch_add(1, Ordering::Relaxed);
                atomics.counters[i + 1].fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let duration = start.elapsed();
    println!("缓存行竞争测试耗时: {:?}", duration);
    
    // 显示结果
    for (i, counter) in aligned_atomics.counters.iter().enumerate() {
        println!("计数器 {}: {}", i, counter.load(Ordering::Relaxed));
    }
    
    println!("\n缓存行优化建议:");
    println!("• 使用 #[repr(align(64))] 对齐到缓存行边界");
    println!("• 避免多个线程频繁访问同一缓存行的不同原子变量");
    println!("• 考虑使用填充（padding）分离热点数据");
}

/// 7. 高级模式演示
fn advanced_patterns_demo() {
    println!("\n📚 7. 高级模式");
    println!("{}", "-".repeat(50));
    
    // ABA 问题演示
    println!("\n🔹 ABA 问题演示:");
    aba_problem_demo();
    
    // 内存回收策略
    println!("\n🔹 内存回收策略:");
    memory_reclamation_demo();
    
    // 无锁算法设计模式
    println!("\n🔹 无锁算法设计模式:");
    lock_free_patterns_demo();
    
    // 原子操作组合
    println!("\n🔹 原子操作组合:");
    atomic_composition_demo();
}

/// ABA 问题演示
fn aba_problem_demo() {
    println!("ABA 问题说明:");
    println!("当一个值从 A 变为 B 再变回 A 时，CAS 操作可能错误地认为值没有改变");
    
    // 模拟 ABA 问题
    struct ABAExample {
        value: AtomicUsize,
        version: AtomicUsize, // 版本号解决 ABA 问题
    }
    
    impl ABAExample {
        fn new(initial: usize) -> Self {
            Self {
                value: AtomicUsize::new(initial),
                version: AtomicUsize::new(0),
            }
        }
        
        // 有 ABA 问题的 CAS
        fn unsafe_cas(&self, expected: usize, new: usize) -> Result<usize, usize> {
            self.value.compare_exchange(
                expected,
                new,
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
        }
        
        // 使用版本号解决 ABA 问题
        fn safe_cas(&self, expected: usize, new: usize) -> Result<(usize, usize), (usize, usize)> {
            let current_version = self.version.load(Ordering::SeqCst);
            let current_value = self.value.load(Ordering::SeqCst);
            
            if current_value != expected {
                return Err((current_value, current_version));
            }
            
            // 原子地更新值和版本号（这里简化处理）
            match self.value.compare_exchange(
                expected,
                new,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(old_val) => {
                    self.version.fetch_add(1, Ordering::SeqCst);
                    Ok((old_val, current_version))
                }
                Err(actual) => Err((actual, current_version)),
            }
        }
        
        fn get(&self) -> (usize, usize) {
            let value = self.value.load(Ordering::SeqCst);
            let version = self.version.load(Ordering::SeqCst);
            (value, version)
        }
    }
    
    let example = Arc::new(ABAExample::new(10));
    
    // 模拟 ABA 场景
    let example1 = Arc::clone(&example);
    let example2 = Arc::clone(&example);
    let example3 = Arc::clone(&example);
    
    let barrier = Arc::new(Barrier::new(3));
    let barrier1 = Arc::clone(&barrier);
    let barrier2 = Arc::clone(&barrier);
    let barrier3 = Arc::clone(&barrier);
    
    // 线程1: 读取初始值，然后等待
    let t1 = thread::spawn(move || {
        let (initial_value, initial_version) = example1.get();
        println!("线程1: 读取初始值 {} (版本 {})", initial_value, initial_version);
        
        barrier1.wait();
        
        // 尝试 CAS 操作
         match example1.unsafe_cas(10, 30) {
             Ok(old) => println!("线程1: 不安全CAS成功，旧值 {}", old),
             Err(actual) => println!("线程1: 不安全CAS失败，实际值 {}", actual),
         }
         
         // 尝试安全 CAS 操作
         match example1.safe_cas(10, 30) {
             Ok((old_val, old_version)) => println!("线程1: 安全CAS成功，旧值 {} (版本 {})", old_val, old_version),
             Err((actual_val, actual_version)) => println!("线程1: 安全CAS失败，实际值 {} (版本 {})", actual_val, actual_version),
         }
     });
     
     // 线程2: A -> B
     let t2 = thread::spawn(move || {
         barrier2.wait();
         
         match example2.unsafe_cas(10, 20) {
             Ok(old) => println!("线程2: 将值从 {} 改为 20", old),
             Err(actual) => println!("线程2: CAS失败，实际值 {}", actual),
         }
     });
     
     // 线程3: B -> A
     let t3 = thread::spawn(move || {
         barrier3.wait();
         thread::sleep(Duration::from_millis(1)); // 确保线程2先执行
         
         match example3.unsafe_cas(20, 10) {
             Ok(old) => println!("线程3: 将值从 {} 改回 10 (ABA!)", old),
             Err(actual) => println!("线程3: CAS失败，实际值 {}", actual),
         }
     });
     
     t1.join().unwrap();
     t2.join().unwrap();
     t3.join().unwrap();
     
     let (final_value, final_version) = example.get();
     println!("最终状态: 值 = {}, 版本 = {}", final_value, final_version);
}

/// 内存回收策略演示
fn memory_reclamation_demo() {
    println!("内存回收策略:");
    println!("• 引用计数: 使用 Arc 自动管理内存");
    println!("• 危险指针 (Hazard Pointers): 标记正在使用的指针");
    println!("• 时代回收 (Epoch-based Reclamation): 延迟回收直到安全");
    println!("• RCU (Read-Copy-Update): 读者无锁，写者复制更新");
    
    // 简单的引用计数示例
    struct RefCountedData {
        data: i32,
        ref_count: AtomicUsize,
    }
    
    impl RefCountedData {
        fn new(data: i32) -> Arc<Self> {
            Arc::new(Self {
                data,
                ref_count: AtomicUsize::new(1),
            })
        }
        
        fn acquire(self: &Arc<Self>) -> Arc<Self> {
            self.ref_count.fetch_add(1, Ordering::Relaxed);
            Arc::clone(self)
        }
        
        fn release(self: Arc<Self>) {
            if self.ref_count.fetch_sub(1, Ordering::Release) == 1 {
                fence(Ordering::Acquire);
                println!("数据 {} 被回收", self.data);
                // 在实际实现中，这里会释放内存
            }
        }
    }
    
    let data = RefCountedData::new(42);
    println!("创建数据: {}", data.data);
    
    let data_copy = data.acquire();
    println!("获取引用，当前引用计数: {}", data.ref_count.load(Ordering::Relaxed));
    
    data_copy.release();
    println!("释放引用，当前引用计数: {}", data.ref_count.load(Ordering::Relaxed));
    
    // 注意：这里 data 仍然存在，所以不会被回收
    println!("原始数据仍存在: {}", data.data);
}

/// 无锁算法设计模式演示
fn lock_free_patterns_demo() {
    println!("无锁算法设计模式:");
    println!("• CAS 循环: 重试直到成功");
    println!("• 帮助机制: 线程互相帮助完成操作");
    println!("• 分离关注点: 将复杂操作分解为简单步骤");
    println!("• 版本控制: 使用版本号避免 ABA 问题");
    
    // CAS 循环模式示例
    fn cas_loop_increment(counter: &AtomicUsize) -> usize {
        loop {
            let current = counter.load(Ordering::Relaxed);
            match counter.compare_exchange_weak(
                current,
                current + 1,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(old) => return old,
                Err(_) => continue, // 重试
            }
        }
    }
    
    let counter = AtomicUsize::new(100);
    let old_value = cas_loop_increment(&counter);
    println!("CAS循环递增: {} -> {}", old_value, counter.load(Ordering::Relaxed));
    
    // 帮助机制示例（简化版）
    struct HelpingExample {
        value: AtomicUsize,
        helper_flag: AtomicBool,
    }
    
    impl HelpingExample {
        fn new() -> Self {
            Self {
                value: AtomicUsize::new(0),
                helper_flag: AtomicBool::new(false),
            }
        }
        
        fn increment_with_help(&self) -> usize {
            // 尝试设置帮助标志
            if self.helper_flag.compare_exchange(
                false,
                true,
                Ordering::Acquire,
                Ordering::Relaxed,
            ).is_ok() {
                // 成功设置标志，执行操作
                let old = self.value.fetch_add(1, Ordering::Relaxed);
                self.helper_flag.store(false, Ordering::Release);
                old
            } else {
                // 其他线程正在操作，等待完成
                while self.helper_flag.load(Ordering::Acquire) {
                    thread::yield_now();
                }
                self.value.load(Ordering::Relaxed)
            }
        }
    }
    
    let helper_example = Arc::new(HelpingExample::new());
    let mut handles = vec![];
    
    for i in 0..5 {
        let example = Arc::clone(&helper_example);
        let handle = thread::spawn(move || {
            let result = example.increment_with_help();
            println!("线程 {} 帮助机制结果: {}", i, result);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终值: {}", helper_example.value.load(Ordering::Relaxed));
}

/// 原子操作组合演示
fn atomic_composition_demo() {
    println!("原子操作组合模式:");
    
    // 原子状态机
    #[derive(Debug, PartialEq)]
    enum State {
        Idle = 0,
        Working = 1,
        Completed = 2,
        Error = 3,
    }
    
    struct AtomicStateMachine {
        state: AtomicUsize,
        data: AtomicI32,
    }
    
    impl AtomicStateMachine {
        fn new() -> Self {
            Self {
                state: AtomicUsize::new(State::Idle as usize),
                data: AtomicI32::new(0),
            }
        }
        
        fn start_work(&self, input: i32) -> Result<(), &'static str> {
            match self.state.compare_exchange(
                State::Idle as usize,
                State::Working as usize,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => {
                    self.data.store(input, Ordering::Relaxed);
                    println!("开始工作，输入: {}", input);
                    Ok(())
                }
                Err(_) => Err("状态机不在空闲状态"),
            }
        }
        
        fn complete_work(&self) -> Result<i32, &'static str> {
            match self.state.compare_exchange(
                State::Working as usize,
                State::Completed as usize,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => {
                    let result = self.data.load(Ordering::Relaxed) * 2;
                    self.data.store(result, Ordering::Relaxed);
                    println!("工作完成，结果: {}", result);
                    Ok(result)
                }
                Err(_) => Err("状态机不在工作状态"),
            }
        }
        
        fn reset(&self) -> Result<(), &'static str> {
            let current_state = self.state.load(Ordering::SeqCst);
            if current_state == State::Completed as usize || current_state == State::Error as usize {
                self.state.store(State::Idle as usize, Ordering::SeqCst);
                self.data.store(0, Ordering::Relaxed);
                println!("状态机已重置");
                Ok(())
            } else {
                Err("只能从完成或错误状态重置")
            }
        }
        
        fn get_state(&self) -> State {
            match self.state.load(Ordering::SeqCst) {
                0 => State::Idle,
                1 => State::Working,
                2 => State::Completed,
                3 => State::Error,
                _ => State::Error,
            }
        }
    }
    
    let state_machine = Arc::new(AtomicStateMachine::new());
    
    // 测试状态转换
    println!("初始状态: {:?}", state_machine.get_state());
    
    if let Err(e) = state_machine.start_work(10) {
        println!("错误: {}", e);
    }
    
    println!("当前状态: {:?}", state_machine.get_state());
    
    if let Err(e) = state_machine.complete_work() {
        println!("错误: {}", e);
    }
    
    println!("当前状态: {:?}", state_machine.get_state());
    
    if let Err(e) = state_machine.reset() {
        println!("错误: {}", e);
    }
    
    println!("最终状态: {:?}", state_machine.get_state());
    
    // 多线程测试
    println!("\n多线程状态机测试:");
    let mut handles = vec![];
    
    for i in 0..3 {
        let sm = Arc::clone(&state_machine);
        let handle = thread::spawn(move || {
            if sm.start_work(i * 10).is_ok() {
                thread::sleep(Duration::from_millis(10));
                if sm.complete_work().is_ok() {
                    thread::sleep(Duration::from_millis(5));
                    let _ = sm.reset();
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

/// 8. 实际应用案例演示
fn real_world_applications_demo() {
    println!("\n📚 8. 实际应用案例");
    println!("{}", "-".repeat(50));
    
    // 无锁缓存系统
    println!("\n🔹 无锁缓存系统:");
    lock_free_cache_demo();
    
    // 原子计数器服务
    println!("\n🔹 原子计数器服务:");
    atomic_counter_service_demo();
    
    // 无锁消息传递
    println!("\n🔹 无锁消息传递:");
    lock_free_messaging_demo();
    
    // 原子配置管理
    println!("\n🔹 原子配置管理:");
    atomic_config_demo();
}

/// 无锁缓存系统演示
fn lock_free_cache_demo() {
    const CACHE_SIZE: usize = 16;
    
    struct CacheEntry {
        key: AtomicU64,
        value: AtomicU64,
        timestamp: AtomicU64,
        valid: AtomicBool,
    }
    
    impl CacheEntry {
        fn new() -> Self {
            Self {
                key: AtomicU64::new(0),
                value: AtomicU64::new(0),
                timestamp: AtomicU64::new(0),
                valid: AtomicBool::new(false),
            }
        }
    }
    
    struct LockFreeCache {
        entries: [CacheEntry; CACHE_SIZE],
        clock: AtomicU64,
    }
    
    impl LockFreeCache {
        fn new() -> Self {
            const ENTRY: CacheEntry = CacheEntry {
                key: AtomicU64::new(0),
                value: AtomicU64::new(0),
                timestamp: AtomicU64::new(0),
                valid: AtomicBool::new(false),
            };
            
            Self {
                entries: [ENTRY; CACHE_SIZE],
                clock: AtomicU64::new(0),
            }
        }
        
        fn hash(&self, key: u64) -> usize {
            (key as usize) % CACHE_SIZE
        }
        
        fn put(&self, key: u64, value: u64) {
            let index = self.hash(key);
            let entry = &self.entries[index];
            let now = self.clock.fetch_add(1, Ordering::Relaxed);
            
            entry.key.store(key, Ordering::Relaxed);
            entry.value.store(value, Ordering::Relaxed);
            entry.timestamp.store(now, Ordering::Relaxed);
            entry.valid.store(true, Ordering::Release);
        }
        
        fn get(&self, key: u64) -> Option<u64> {
            let index = self.hash(key);
            let entry = &self.entries[index];
            
            if entry.valid.load(Ordering::Acquire) {
                let stored_key = entry.key.load(Ordering::Relaxed);
                if stored_key == key {
                    let value = entry.value.load(Ordering::Relaxed);
                    // 更新访问时间
                    let now = self.clock.fetch_add(1, Ordering::Relaxed);
                    entry.timestamp.store(now, Ordering::Relaxed);
                    return Some(value);
                }
            }
            
            None
        }
        
        fn stats(&self) -> (usize, u64) {
            let mut valid_entries = 0;
            let current_time = self.clock.load(Ordering::Relaxed);
            
            for entry in &self.entries {
                if entry.valid.load(Ordering::Relaxed) {
                    valid_entries += 1;
                }
            }
            
            (valid_entries, current_time)
        }
    }
    
    let cache = Arc::new(LockFreeCache::new());
    let mut handles = vec![];
    
    // 写入线程
    for i in 0..4 {
        let cache_clone = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            for j in 0..25 {
                let key = (i * 25 + j) as u64;
                let value = key * 100;
                cache_clone.put(key, value);
            }
            println!("缓存写入线程 {} 完成", i);
        });
        handles.push(handle);
    }
    
    // 读取线程
    for i in 0..2 {
        let cache_clone = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            let mut hits = 0;
            let mut misses = 0;
            
            for j in 0..50 {
                let key = (j * 2) as u64;
                match cache_clone.get(key) {
                    Some(value) => {
                        hits += 1;
                        if j < 5 {
                            println!("缓存命中: key={}, value={}", key, value);
                        }
                    }
                    None => misses += 1,
                }
            }
            
            println!("缓存读取线程 {} 完成: 命中={}, 未命中={}", i, hits, misses);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let (valid_entries, clock_time) = cache.stats();
    println!("缓存统计: 有效条目={}, 时钟时间={}", valid_entries, clock_time);
}

/// 原子计数器服务演示
fn atomic_counter_service_demo() {
    struct CounterService {
        counters: Vec<AtomicU64>,
        total_operations: AtomicU64,
    }
    
    impl CounterService {
        fn new(num_counters: usize) -> Self {
            let mut counters = Vec::with_capacity(num_counters);
            for _ in 0..num_counters {
                counters.push(AtomicU64::new(0));
            }
            
            Self {
                counters,
                total_operations: AtomicU64::new(0),
            }
        }
        
        fn increment(&self, counter_id: usize) -> Result<u64, &'static str> {
            if counter_id >= self.counters.len() {
                return Err("计数器ID无效");
            }
            
            let old_value = self.counters[counter_id].fetch_add(1, Ordering::Relaxed);
            self.total_operations.fetch_add(1, Ordering::Relaxed);
            Ok(old_value + 1)
        }
        
        fn decrement(&self, counter_id: usize) -> Result<u64, &'static str> {
             if counter_id >= self.counters.len() {
                 return Err("计数器ID无效");
             }
             
             let old_value = self.counters[counter_id].fetch_sub(1, Ordering::Relaxed);
             self.total_operations.fetch_add(1, Ordering::Relaxed);
             Ok(old_value.saturating_sub(1))
         }
        
        fn get(&self, counter_id: usize) -> Result<u64, &'static str> {
            if counter_id >= self.counters.len() {
                return Err("计数器ID无效");
            }
            
            Ok(self.counters[counter_id].load(Ordering::Relaxed))
        }
        
        fn get_total(&self) -> u64 {
            self.counters.iter()
                .map(|counter| counter.load(Ordering::Relaxed))
                .sum()
        }
        
        fn get_stats(&self) -> (u64, u64) {
            (self.get_total(), self.total_operations.load(Ordering::Relaxed))
        }
    }
    
    let service = Arc::new(CounterService::new(8));
    let mut handles = vec![];
    
    // 模拟多个客户端
    for client_id in 0..6 {
        let service_clone = Arc::clone(&service);
        let handle = thread::spawn(move || {
            let counter_id = client_id % 8;
            
            for i in 0..100 {
                if i % 3 == 0 {
                    if let Ok(new_value) = service_clone.increment(counter_id) {
                        if i < 10 {
                            println!("客户端 {} 递增计数器 {}: {}", client_id, counter_id, new_value);
                        }
                    }
                } else if i % 7 == 0 {
                    if let Ok(new_value) = service_clone.decrement(counter_id) {
                        if i < 10 {
                            println!("客户端 {} 递减计数器 {}: {}", client_id, counter_id, new_value);
                        }
                    }
                } else {
                    if let Ok(value) = service_clone.get(counter_id) {
                        if i < 5 {
                            println!("客户端 {} 读取计数器 {}: {}", client_id, counter_id, value);
                        }
                    }
                }
            }
            
            println!("客户端 {} 完成操作", client_id);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let (total_value, total_ops) = service.get_stats();
    println!("服务统计: 总计数值={}, 总操作数={}", total_value, total_ops);
    
    // 显示各个计数器的值
    println!("各计数器状态:");
    for i in 0..8 {
        if let Ok(value) = service.get(i) {
            println!("  计数器 {}: {}", i, value);
        }
    }
}

/// 无锁消息传递演示
fn lock_free_messaging_demo() {
    const QUEUE_SIZE: usize = 64;
    
    struct Message {
        id: u64,
        data: [u8; 32],
        timestamp: u64,
    }
    
    impl Message {
        fn new(id: u64, data: &str) -> Self {
            let mut msg_data = [0u8; 32];
            let bytes = data.as_bytes();
            let len = bytes.len().min(32);
            msg_data[..len].copy_from_slice(&bytes[..len]);
            
            Self {
                id,
                data: msg_data,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64,
            }
        }
        
        fn get_data_str(&self) -> String {
            let end = self.data.iter().position(|&b| b == 0).unwrap_or(32);
            String::from_utf8_lossy(&self.data[..end]).to_string()
        }
    }
    
    struct LockFreeMessageQueue {
        buffer: Vec<UnsafeCell<Option<Message>>>,
        head: AtomicUsize,
        tail: AtomicUsize,
        capacity: usize,
        message_count: AtomicU64,
    }
    
    impl LockFreeMessageQueue {
        fn new() -> Self {
            let mut buffer = Vec::with_capacity(QUEUE_SIZE);
            for _ in 0..QUEUE_SIZE {
                buffer.push(UnsafeCell::new(None));
            }
            
            Self {
                buffer,
                head: AtomicUsize::new(0),
                tail: AtomicUsize::new(0),
                capacity: QUEUE_SIZE,
                message_count: AtomicU64::new(0),
            }
        }
        
        fn send(&self, message: Message) -> Result<(), Message> {
            let tail = self.tail.load(Ordering::Relaxed);
            let next_tail = (tail + 1) % self.capacity;
            
            if next_tail == self.head.load(Ordering::Acquire) {
                return Err(message); // 队列满
            }
            
            unsafe {
                *self.buffer[tail].get() = Some(message);
            }
            
            self.tail.store(next_tail, Ordering::Release);
            self.message_count.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
        
        fn receive(&self) -> Option<Message> {
            let head = self.head.load(Ordering::Relaxed);
            
            if head == self.tail.load(Ordering::Acquire) {
                return None; // 队列空
            }
            
            let message = unsafe { (*self.buffer[head].get()).take() };
            let next_head = (head + 1) % self.capacity;
            self.head.store(next_head, Ordering::Release);
            
            message
        }
        
        fn len(&self) -> usize {
            let head = self.head.load(Ordering::Relaxed);
            let tail = self.tail.load(Ordering::Relaxed);
            
            if tail >= head {
                tail - head
            } else {
                self.capacity - head + tail
            }
        }
        
        fn total_messages(&self) -> u64 {
            self.message_count.load(Ordering::Relaxed)
        }
    }
    
    unsafe impl Send for LockFreeMessageQueue {}
    unsafe impl Sync for LockFreeMessageQueue {}
    
    let queue = Arc::new(LockFreeMessageQueue::new());
    let mut handles = vec![];
    
    // 发送者线程
    for sender_id in 0..3 {
        let queue_clone = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            for i in 0..20 {
                let message = Message::new(
                    (sender_id * 100 + i) as u64,
                    &format!("消息来自发送者{}-{}", sender_id, i),
                );
                
                loop {
                     let msg = Message::new(
                         (sender_id * 100 + i) as u64,
                         &format!("消息来自发送者{}-{}", sender_id, i),
                     );
                     
                     if queue_clone.send(msg).is_ok() {
                         break;
                     }
                     
                     thread::yield_now();
                     // 在实际应用中，这里可能需要更复杂的退避策略
                 }
                
                if i < 5 {
                    println!("发送者 {} 发送消息 {}", sender_id, i);
                }
            }
            
            println!("发送者 {} 完成", sender_id);
        });
        handles.push(handle);
    }
    
    // 接收者线程
    for receiver_id in 0..2 {
        let queue_clone = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            let mut received_count = 0;
            
            while received_count < 30 {
                if let Some(message) = queue_clone.receive() {
                    received_count += 1;
                    
                    if received_count <= 5 {
                        println!(
                            "接收者 {} 收到消息 ID={}: {}",
                            receiver_id,
                            message.id,
                            message.get_data_str()
                        );
                    }
                } else {
                    thread::yield_now();
                }
            }
            
            println!("接收者 {} 完成，共接收 {} 条消息", receiver_id, received_count);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("消息队列统计: 当前长度={}, 总消息数={}", queue.len(), queue.total_messages());
}

/// 原子配置管理演示
fn atomic_config_demo() {
    #[derive(Clone, Debug)]
    struct Config {
        max_connections: u32,
        timeout_ms: u32,
        enable_logging: bool,
        version: u64,
    }
    
    impl Config {
        fn new() -> Self {
            Self {
                max_connections: 100,
                timeout_ms: 5000,
                enable_logging: true,
                version: 1,
            }
        }
    }
    
    struct AtomicConfigManager {
        config_ptr: AtomicPtr<Config>,
        update_count: AtomicU64,
    }
    
    impl AtomicConfigManager {
        fn new(initial_config: Config) -> Self {
            let config_box = Box::new(initial_config);
            let config_ptr = Box::into_raw(config_box);
            
            Self {
                config_ptr: AtomicPtr::new(config_ptr),
                update_count: AtomicU64::new(0),
            }
        }
        
        fn get_config(&self) -> Config {
            let ptr = self.config_ptr.load(Ordering::Acquire);
            unsafe { (*ptr).clone() }
        }
        
        fn update_config(&self, new_config: Config) {
            let new_box = Box::new(new_config);
            let new_ptr = Box::into_raw(new_box);
            
            let old_ptr = self.config_ptr.swap(new_ptr, Ordering::AcqRel);
            self.update_count.fetch_add(1, Ordering::Relaxed);
            
            // 在实际应用中，这里需要安全的内存回收策略
            // 这里为了演示简化处理
            unsafe {
                let _ = Box::from_raw(old_ptr);
            }
        }
        
        fn get_update_count(&self) -> u64 {
            self.update_count.load(Ordering::Relaxed)
        }
    }
    
    let config_manager = Arc::new(AtomicConfigManager::new(Config::new()));
    
    println!("初始配置: {:?}", config_manager.get_config());
    
    let mut handles = vec![];
    
    // 配置更新线程
    let manager_clone = Arc::clone(&config_manager);
    let updater = thread::spawn(move || {
        for i in 1..=5 {
             let mut new_config = Config::new();
             new_config.max_connections = 100 + (i as u32) * 50;
             new_config.timeout_ms = 5000 + (i as u32) * 1000;
             new_config.enable_logging = i % 2 == 0;
             new_config.version = (i + 1) as u64;
            
            manager_clone.update_config(new_config);
            println!("配置已更新到版本 {}", i + 1);
            
            thread::sleep(Duration::from_millis(10));
        }
    });
    handles.push(updater);
    
    // 配置读取线程
    for reader_id in 0..3 {
        let manager_clone = Arc::clone(&config_manager);
        let reader = thread::spawn(move || {
            for i in 0..10 {
                let config = manager_clone.get_config();
                
                if i < 3 {
                    println!(
                        "读取者 {} 读取配置: 连接数={}, 超时={}ms, 日志={}, 版本={}",
                        reader_id,
                        config.max_connections,
                        config.timeout_ms,
                        config.enable_logging,
                        config.version
                    );
                }
                
                thread::sleep(Duration::from_millis(5));
            }
            
            println!("读取者 {} 完成", reader_id);
        });
        handles.push(reader);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_config = config_manager.get_config();
    let update_count = config_manager.get_update_count();
    
    println!("最终配置: {:?}", final_config);
    println!("总更新次数: {}", update_count);
}
