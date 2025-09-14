//! Rust 线程编程全面教程
//! 
//! 本教程深入探讨 Rust 中的线程编程，包括：
//! - 线程创建与管理
//! - 线程间通信
//! - 线程安全特征
//! - 线程本地存储
//! - 线程池实现
//! - 性能分析与优化
//! - 高级线程主题

use std::{
    thread,
    time::{Duration, Instant},
    sync::{Arc, Mutex, mpsc, Barrier, Condvar},
    collections::HashMap,
    cell::RefCell,
    rc::Rc,
    panic,
};

fn main() {
    println!("{}", "=".repeat(60));
    println!("🦀 Rust 线程编程全面教程");
    println!("{}", "=".repeat(60));
    
    // 1. 线程基础概念
    println!("\n📚 1. 线程基础概念");
    println!("{}", "-".repeat(30));
    thread_basics_concepts();
    
    // 2. 线程创建与管理
    println!("\n🔧 2. 线程创建与管理");
    println!("{}", "-".repeat(30));
    thread_creation_management();
    
    // 3. 线程间通信
    println!("\n📡 3. 线程间通信");
    println!("{}", "-".repeat(30));
    thread_communication();
    
    // 4. 线程安全特征
    println!("\n🔒 4. 线程安全特征 (Send & Sync)");
    println!("{}", "-".repeat(30));
    thread_safety_traits();
    
    // 5. 线程本地存储
    println!("\n🗄️ 5. 线程本地存储");
    println!("{}", "-".repeat(30));
    thread_local_storage();
    
    // 6. 线程池实现
    println!("\n🏊 6. 线程池实现");
    println!("{}", "-".repeat(30));
    thread_pool_implementation();
    
    // 7. 线程性能分析
    println!("\n📊 7. 线程性能分析");
    println!("{}", "-".repeat(30));
    thread_performance_analysis();
    
    // 8. 高级线程主题
    println!("\n🚀 8. 高级线程主题");
    println!("{}", "-".repeat(30));
    advanced_thread_topics();
    
    println!("\n{}", "=".repeat(60));
    println!("✅ 线程教程完成！");
    println!("{}", "=".repeat(60));
}

/// 1. 线程基础概念
fn thread_basics_concepts() {
    println!("   🔍 线程 vs 进程：");
    println!("   • 进程：独立的内存空间，进程间通信需要特殊机制");
    println!("   • 线程：共享进程内存空间，轻量级，创建开销小");
    
    println!("\n   🎯 Rust 线程模型特点：");
    println!("   • 1:1 线程模型（每个 Rust 线程对应一个 OS 线程）");
    println!("   • 零成本抽象，性能接近系统调用");
    println!("   • 编译时保证线程安全");
    println!("   • 所有权系统防止数据竞争");
    
    println!("\n   ⚙️ 线程调度机制：");
    println!("   • 抢占式调度：操作系统控制线程切换");
    println!("   • 时间片轮转：每个线程获得固定执行时间");
    println!("   • 优先级调度：高优先级线程优先执行");
    
    // 演示线程执行的不确定性
    println!("\n   🎲 线程执行顺序的不确定性演示：");
    for i in 0..3 {
        let handle = thread::spawn(move || {
            println!("     线程 {} 开始执行", i);
            thread::sleep(Duration::from_millis(10));
            println!("     线程 {} 结束执行", i);
        });
        // 不等待线程完成，展示并发执行
        let _ = handle;
    }
    
    thread::sleep(Duration::from_millis(100)); // 等待所有线程完成
}

/// 2. 线程创建与管理
fn thread_creation_management() {
    println!("   🚀 基本线程创建：");
    
    // 基本线程创建
    let handle = thread::spawn(|| {
        println!("     子线程执行中...");
        "线程返回值"
    });
    
    // 等待线程完成并获取返回值
    match handle.join() {
        Ok(result) => println!("     线程返回: {}", result),
        Err(_) => println!("     线程执行失败"),
    }
    
    println!("\n   📦 线程参数传递：");
    
    // 通过 move 闭包传递参数
    let data = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        let sum: i32 = data.iter().sum();
        println!("     数据求和结果: {}", sum);
        sum
    });
    
    let result = handle.join().unwrap();
    println!("     主线程收到结果: {}", result);
    
    println!("\n   🏷️ 线程命名与标识：");
    
    // 命名线程
    let handle = thread::Builder::new()
        .name("worker-thread".to_string())
        .spawn(|| {
            let current = thread::current();
            println!("     线程名称: {:?}", current.name());
            println!("     线程ID: {:?}", current.id());
        })
        .unwrap();
    
    handle.join().unwrap();
    
    println!("\n   ⚙️ 线程配置：");
    
    // 配置线程栈大小
    let handle = thread::Builder::new()
        .name("custom-stack".to_string())
        .stack_size(2 * 1024 * 1024) // 2MB 栈大小
        .spawn(|| {
            println!("     自定义栈大小的线程执行中...");
        })
        .unwrap();
    
    handle.join().unwrap();
    
    println!("\n   🔄 多线程批量创建：");
    
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                println!("     批量线程 {} 执行", i);
                thread::sleep(Duration::from_millis(100));
                i * i
            })
        })
        .collect();
    
    // 等待所有线程完成
    let results: Vec<_> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    
    println!("     批量线程结果: {:?}", results);
}

/// 3. 线程间通信
fn thread_communication() {
    println!("   📨 消息传递通信：");
    
    // 单生产者单消费者
    let (tx, rx) = mpsc::channel();
    
    let producer = thread::spawn(move || {
        for i in 0..5 {
            tx.send(format!("消息-{}", i)).unwrap();
            println!("     发送: 消息-{}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    let consumer = thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            println!("     接收: {}", msg);
        }
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
    
    println!("\n   📡 多生产者单消费者：");
    
    let (tx, rx) = mpsc::channel();
    
    // 创建多个生产者
    let producers: Vec<_> = (0..3)
        .map(|id| {
            let tx_clone = tx.clone();
            thread::spawn(move || {
                for i in 0..3 {
                    let msg = format!("生产者{}-消息{}", id, i);
                    tx_clone.send(msg.clone()).unwrap();
                    println!("     {} 发送", msg);
                    thread::sleep(Duration::from_millis(50));
                }
            })
        })
        .collect();
    
    // 丢弃原始发送者，确保通道能正确关闭
    drop(tx);
    
    let consumer = thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            println!("     消费者接收: {}", msg);
        }
    });
    
    // 等待所有生产者完成
    for producer in producers {
        producer.join().unwrap();
    }
    
    consumer.join().unwrap();
    
    println!("\n   🔄 同步通信（rendezvous）：");
    
    let (tx, rx) = mpsc::sync_channel(0); // 容量为0，同步通信
    
    let sender = thread::spawn(move || {
        println!("     发送者准备发送...");
        tx.send("同步消息").unwrap();
        println!("     发送者发送完成");
    });
    
    thread::sleep(Duration::from_millis(100));
    
    let receiver = thread::spawn(move || {
        println!("     接收者准备接收...");
        let msg = rx.recv().unwrap();
        println!("     接收者收到: {}", msg);
    });
    
    sender.join().unwrap();
    receiver.join().unwrap();
    
    println!("\n   🤝 共享状态通信：");
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("     线程 {} 增加计数器，当前值: {}", i, *num);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("     最终计数器值: {}", *counter.lock().unwrap());
    
    println!("\n   🚧 线程同步屏障：");
    
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];
    
    for i in 0..3 {
        let barrier_clone = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("     线程 {} 到达屏障前", i);
            thread::sleep(Duration::from_millis(100 * (i + 1) as u64));
            
            barrier_clone.wait();
            
            println!("     线程 {} 通过屏障后", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

/// 4. 线程安全特征 (Send & Sync)
fn thread_safety_traits() {
    println!("   🔒 Send trait - 所有权转移安全：");
    println!("   • Send: 类型可以安全地在线程间转移所有权");
    println!("   • 大多数类型都实现了 Send（除了 Rc<T>、裸指针等）");
    
    // 演示 Send 类型
    let data = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        println!("     Send 类型 Vec 在线程间转移: {:?}", data);
    });
    handle.join().unwrap();
    
    println!("\n   🔄 Sync trait - 引用共享安全：");
    println!("   • Sync: 类型可以安全地在线程间共享引用");
    println!("   • T 是 Sync 当且仅当 &T 是 Send");
    
    // 演示 Sync 类型
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];
    
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("     线程 {} 访问 Sync 类型: {:?}", i, data_clone);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("\n   ❌ 非线程安全类型示例：");
    println!("   • Rc<T>: 不是 Send，不能跨线程转移");
    println!("   • RefCell<T>: 不是 Sync，不能跨线程共享引用");
    
    // 这些代码会编译失败，仅作说明
    // let rc = Rc::new(5);
    // thread::spawn(move || { println!("{}", rc); }); // 编译错误
    
    println!("\n   ✅ 线程安全的替代方案：");
    
    // Arc 替代 Rc
    let arc_data = Arc::new(42);
    let arc_clone = Arc::clone(&arc_data);
    let handle = thread::spawn(move || {
        println!("     Arc 替代 Rc: {}", arc_clone);
    });
    handle.join().unwrap();
    
    // Mutex 提供内部可变性
    let mutex_data = Arc::new(Mutex::new(0));
    let mutex_clone = Arc::clone(&mutex_data);
    let handle = thread::spawn(move || {
        let mut data = mutex_clone.lock().unwrap();
        *data += 10;
        println!("     Mutex 提供线程安全的可变性: {}", *data);
    });
    handle.join().unwrap();
    
    println!("\n   🔍 自定义类型的线程安全：");
    
    #[derive(Debug)]
    struct ThreadSafeStruct {
        data: i32,
    }
    
    // 自动实现 Send 和 Sync（因为所有字段都是 Send + Sync）
    unsafe impl Send for ThreadSafeStruct {}
    unsafe impl Sync for ThreadSafeStruct {}
    
    let safe_struct = Arc::new(ThreadSafeStruct { data: 100 });
    let safe_clone = Arc::clone(&safe_struct);
    
    let handle = thread::spawn(move || {
        println!("     自定义线程安全结构体: {:?}", safe_clone);
    });
    handle.join().unwrap();
}

/// 5. 线程本地存储
fn thread_local_storage() {
    println!("   🗄️ thread_local! 宏使用：");
    
    thread_local! {
        static COUNTER: RefCell<i32> = RefCell::new(0);
        static NAME: RefCell<String> = RefCell::new(String::new());
    }
    
    fn increment_counter() {
        COUNTER.with(|c| {
            let mut counter = c.borrow_mut();
            *counter += 1;
            println!("     当前线程计数器: {}", *counter);
        });
    }
    
    fn set_thread_name(name: &str) {
        NAME.with(|n| {
            *n.borrow_mut() = name.to_string();
        });
    }
    
    fn get_thread_name() -> String {
        NAME.with(|n| n.borrow().clone())
    }
    
    // 主线程使用
    set_thread_name("主线程");
    increment_counter();
    increment_counter();
    println!("     {} 计数器最终值", get_thread_name());
    
    // 子线程使用
    let handles: Vec<_> = (0..3)
        .map(|i| {
            thread::spawn(move || {
                set_thread_name(&format!("工作线程-{}", i));
                
                for _ in 0..2 {
                    increment_counter();
                }
                
                println!("     {} 完成工作", get_thread_name());
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("\n   📊 线程本地存储的应用场景：");
    println!("   • 线程特定的配置信息");
    println!("   • 性能统计和监控数据");
    println!("   • 缓存和临时数据");
    println!("   • 避免锁竞争的线程私有状态");
    
    // 实际应用示例：线程本地缓存
    thread_local! {
        static CACHE: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    }
    
    fn cache_get(key: &str) -> Option<String> {
        CACHE.with(|cache| {
            cache.borrow().get(key).cloned()
        })
    }
    
    fn cache_set(key: String, value: String) {
        CACHE.with(|cache| {
            cache.borrow_mut().insert(key, value);
        });
    }
    
    let handles: Vec<_> = (0..2)
        .map(|i| {
            thread::spawn(move || {
                let key = format!("key-{}", i);
                let value = format!("value-{}", i);
                
                cache_set(key.clone(), value.clone());
                
                if let Some(cached_value) = cache_get(&key) {
                    println!("     线程 {} 缓存命中: {} = {}", i, key, cached_value);
                }
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
}

/// 6. 线程池实现
fn thread_pool_implementation() {
    println!("   🏊 简单线程池实现：");
    
    use std::sync::mpsc;
    
    struct SimpleThreadPool {
        workers: Vec<Worker>,
        sender: mpsc::Sender<Job>,
    }
    
    struct Worker {
        id: usize,
        thread: thread::JoinHandle<()>,
    }
    
    type Job = Box<dyn FnOnce() + Send + 'static>;
    
    impl SimpleThreadPool {
        fn new(size: usize) -> SimpleThreadPool {
            assert!(size > 0);
            
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            
            let mut workers = Vec::with_capacity(size);
            
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }
            
            SimpleThreadPool { workers, sender }
        }
        
        fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
    }
    
    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv();
                
                match job {
                    Ok(job) => {
                        println!("     Worker {} 执行任务", id);
                        job();
                    }
                    Err(_) => {
                        println!("     Worker {} 关闭", id);
                        break;
                    }
                }
            });
            
            Worker { id, thread }
        }
    }
    
    // 使用线程池
    let pool = SimpleThreadPool::new(3);
    
    for i in 0..6 {
        pool.execute(move || {
            println!("     任务 {} 开始执行", i);
            thread::sleep(Duration::from_millis(100));
            println!("     任务 {} 执行完成", i);
        });
    }
    
    thread::sleep(Duration::from_millis(1000)); // 等待任务完成
    
    println!("\n   ⚡ 线程池优势：");
    println!("   • 减少线程创建/销毁开销");
    println!("   • 控制并发线程数量");
    println!("   • 提高资源利用率");
    println!("   • 更好的任务调度控制");
    
    println!("\n   🎯 线程池应用场景：");
    println!("   • Web 服务器请求处理");
    println!("   • 批量数据处理");
    println!("   • I/O 密集型任务");
    println!("   • 后台任务调度");
}

/// 7. 线程性能分析
fn thread_performance_analysis() {
    println!("   ⏱️ 线程创建开销测试：");
    
    let start = Instant::now();
    let handles: Vec<_> = (0..1000)
        .map(|_| {
            thread::spawn(|| {
                // 最小工作量
                let _ = 1 + 1;
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let duration = start.elapsed();
    println!("     创建并等待 1000 个线程耗时: {:?}", duration);
    println!("     平均每个线程创建耗时: {:?}", duration / 1000);
    
    println!("\n   🔄 上下文切换开销：");
    
    let iterations = 10000;
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    
    let start = Instant::now();
    
    let handle1 = thread::spawn(move || {
        for i in 0..iterations {
            tx1.send(i).unwrap();
            rx2.recv().unwrap();
        }
    });
    
    let handle2 = thread::spawn(move || {
        for _i in 0..iterations {
            rx1.recv().unwrap();
            tx2.send(()).unwrap();
        }
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    let duration = start.elapsed();
     println!("     {} 次上下文切换耗时: {:?}", iterations * 2, duration);
     println!("     平均每次切换耗时: {:?}", duration / (iterations * 2) as u32);
    
    println!("\n   💾 内存使用分析：");
    
    // 测量线程栈内存使用
    let default_stack = thread::Builder::new()
        .spawn(|| {
            println!("     默认栈大小线程执行");
        })
        .unwrap();
    
    let large_stack = thread::Builder::new()
        .stack_size(8 * 1024 * 1024) // 8MB
        .spawn(|| {
            println!("     大栈线程执行");
        })
        .unwrap();
    
    default_stack.join().unwrap();
    large_stack.join().unwrap();
    
    println!("     • 默认栈大小通常为 2MB");
    println!("     • 可通过 Builder::stack_size 自定义");
    println!("     • 栈大小影响内存使用和线程数量上限");
    
    println!("\n   📈 并发性能对比：");
    
    // 单线程计算
     let work_size = 1000000usize;
     let start = Instant::now();
     let mut sum = 0u64;
     for i in 0..work_size {
         sum += i as u64;
     }
     let single_thread_time = start.elapsed();
     println!("     单线程计算耗时: {:?}, 结果: {}", single_thread_time, sum);
     
     // 多线程计算
     let num_threads = thread::available_parallelism().unwrap().get();
     let chunk_size = work_size / num_threads;
     
     let start = Instant::now();
     let handles: Vec<_> = (0..num_threads)
         .map(|i| {
             let start_idx = i * chunk_size;
             let end_idx = if i == num_threads - 1 {
                 work_size
             } else {
                 (i + 1) * chunk_size
             };
             
             thread::spawn(move || {
                 let mut local_sum = 0u64;
                 for j in start_idx..end_idx {
                     local_sum += j as u64;
                 }
                 local_sum
             })
         })
         .collect();
    
    let mut total_sum = 0u64;
    for handle in handles {
        total_sum += handle.join().unwrap();
    }
    
    let multi_thread_time = start.elapsed();
    println!("     多线程({})计算耗时: {:?}, 结果: {}", num_threads, multi_thread_time, total_sum);
    
    if multi_thread_time.as_nanos() > 0 {
        let speedup = single_thread_time.as_nanos() as f64 / multi_thread_time.as_nanos() as f64;
        println!("     加速比: {:.2}x", speedup);
        println!("     并行效率: {:.1}%", (speedup / num_threads as f64) * 100.0);
    }
    
    println!("\n   🎯 性能优化建议：");
    println!("   • 避免过度创建线程");
    println!("   • 使用线程池复用线程");
    println!("   • 合理设置线程数量（通常等于 CPU 核心数）");
    println!("   • 减少锁竞争和上下文切换");
    println!("   • 考虑使用异步编程模型");
}

/// 8. 高级线程主题
fn advanced_thread_topics() {
    println!("   💥 线程 Panic 处理：");
    
    // 捕获线程 panic
    let handle = thread::spawn(|| {
        panic!("线程发生 panic!");
    });
    
    match handle.join() {
        Ok(_) => println!("     线程正常结束"),
        Err(e) => {
            println!("     捕获到线程 panic: {:?}", e);
            if let Some(s) = e.downcast_ref::<&str>() {
                println!("     Panic 消息: {}", s);
            }
        }
    }
    
    println!("\n   🛡️ 线程安全的 Panic 处理：");
    
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let counter_clone = Arc::clone(&counter);
            thread::spawn(move || {
                if i == 2 {
                    panic!("线程 {} panic!", i);
                }
                
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
                println!("     线程 {} 正常执行，计数器: {}", i, *num);
            })
        })
        .collect();
    
    for (i, handle) in handles.into_iter().enumerate() {
        match handle.join() {
            Ok(_) => println!("     线程 {} 正常结束", i),
            Err(_) => println!("     线程 {} 发生 panic", i),
        }
    }
    
    println!("     最终计数器值: {}", *counter.lock().unwrap());
    
    println!("\n   🔧 优雅关闭模式：");
    
    let (shutdown_tx, shutdown_rx) = mpsc::channel();
    let (work_tx, work_rx) = mpsc::channel();
    
    // 工作线程
     let worker = thread::spawn(move || {
         loop {
             // 检查工作任务
             match work_rx.try_recv() {
                 Ok(task) => {
                     println!("     处理任务: {}", task);
                     thread::sleep(Duration::from_millis(100));
                 }
                 Err(mpsc::TryRecvError::Empty) => {
                     // 没有工作，检查关闭信号
                     if shutdown_rx.try_recv().is_ok() {
                         println!("     收到关闭信号，优雅退出");
                         break;
                     }
                     thread::sleep(Duration::from_millis(10));
                 }
                 Err(mpsc::TryRecvError::Disconnected) => {
                     println!("     工作通道关闭，退出");
                     break;
                 }
             }
         }
     });
    
    // 发送一些工作
    for i in 0..3 {
        work_tx.send(format!("任务-{}", i)).unwrap();
    }
    
    thread::sleep(Duration::from_millis(500));
    
    // 发送关闭信号
    shutdown_tx.send(()).unwrap();
    worker.join().unwrap();
    
    println!("\n   📊 线程监控与调试：");
    
    struct ThreadMonitor {
        name: String,
        start_time: Instant,
    }
    
    impl ThreadMonitor {
        fn new(name: String) -> Self {
            println!("     线程 {} 开始监控", name);
            ThreadMonitor {
                name,
                start_time: Instant::now(),
            }
        }
        
        fn checkpoint(&self, message: &str) {
            let elapsed = self.start_time.elapsed();
            println!("     [{}] {}: {:?}", self.name, message, elapsed);
        }
    }
    
    impl Drop for ThreadMonitor {
        fn drop(&mut self) {
            let total_time = self.start_time.elapsed();
            println!("     线程 {} 结束，总耗时: {:?}", self.name, total_time);
        }
    }
    
    let handle = thread::spawn(|| {
        let monitor = ThreadMonitor::new("工作线程".to_string());
        
        monitor.checkpoint("开始初始化");
        thread::sleep(Duration::from_millis(100));
        
        monitor.checkpoint("初始化完成，开始工作");
        thread::sleep(Duration::from_millis(200));
        
        monitor.checkpoint("工作完成");
        // monitor 在这里自动 drop，触发析构函数
    });
    
    handle.join().unwrap();
    
    println!("\n   🚀 高级线程模式：");
    println!("   • Actor 模式：每个线程作为独立的 Actor");
    println!("   • Producer-Consumer 模式：生产者消费者解耦");
    println!("   • Worker Pool 模式：任务分发到工作线程");
    println!("   • Pipeline 模式：流水线处理数据");
    println!("   • Fork-Join 模式：分治算法并行化");
}

// 简化的 select! 宏模拟
macro_rules! select {
    ($($pattern:pat = $expr:expr => $body:block)*) => {
        $(
            match $expr {
                $pattern => $body
                _ => {}
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_thread_creation() {
        let handle = thread::spawn(|| {
            42
        });
        
        let result = handle.join().unwrap();
        assert_eq!(result, 42);
    }
    
    #[test]
    fn test_thread_communication() {
        let (tx, rx) = mpsc::channel();
        
        thread::spawn(move || {
            tx.send("Hello from thread").unwrap();
        });
        
        let received = rx.recv().unwrap();
        assert_eq!(received, "Hello from thread");
    }
    
    #[test]
    fn test_shared_state() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(*counter.lock().unwrap(), 10);
    }
    
    #[test]
    fn test_thread_local_storage() {
        thread_local! {
            static TEST_COUNTER: RefCell<i32> = RefCell::new(0);
        }
        
        TEST_COUNTER.with(|c| {
            *c.borrow_mut() = 42;
        });
        
        let handle = thread::spawn(|| {
            TEST_COUNTER.with(|c| {
                assert_eq!(*c.borrow(), 0); // 新线程中的初始值
                *c.borrow_mut() = 100;
            });
        });
        
        handle.join().unwrap();
        
        TEST_COUNTER.with(|c| {
            assert_eq!(*c.borrow(), 42); // 主线程中的值不变
        });
    }
}
