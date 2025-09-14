//! # Rust 线程同步：锁、Condvar 和信号量 - 全面深入教程
//! 
//! 本教程全面深入地分析 Rust 中的线程同步机制，包括：
//! - 互斥锁 (Mutex)
//! - 读写锁 (RwLock) 
//! - 条件变量 (Condvar)
//! - 信号量 (Semaphore)
//! - 死锁分析与避免
//! - 性能对比与最佳实践
//! - 实际应用案例

use std::sync::{Arc, Mutex, RwLock, Condvar};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    println!("🚀 Rust 线程同步：锁、Condvar 和信号量 - 全面深入教程");
    println!("{}", "=".repeat(60));
    
    // 1. 互斥锁基础演示
    println!("\n📌 1. 互斥锁 (Mutex) 基础演示");
    mutex_basics_demo();
    
    // 2. 读写锁深入分析
    println!("\n📌 2. 读写锁 (RwLock) 深入分析");
    rwlock_analysis_demo();
    
    // 3. 条件变量实现
    println!("\n📌 3. 条件变量 (Condvar) 实现");
    condvar_implementation_demo();
    
    // 4. 信号量概念
    println!("\n📌 4. 信号量 (Semaphore) 概念");
    semaphore_concepts_demo();
    
    // 5. 死锁分析
    println!("\n📌 5. 死锁分析与避免");
    deadlock_analysis_demo();
    
    // 6. 同步原语性能对比
    println!("\n📌 6. 同步原语性能对比");
    sync_primitives_comparison_demo();
    
    // 7. 高级同步模式
    println!("\n📌 7. 高级同步模式");
    advanced_patterns_demo();
    
    // 8. 实际应用案例
    println!("\n📌 8. 实际应用案例");
    real_world_examples_demo();
    
    println!("\n✅ 教程演示完成！");
}

/// 1. 互斥锁 (Mutex) 基础演示
fn mutex_basics_demo() {
    println!("\n🔒 互斥锁基础概念：");
    println!("   - 确保同一时间只有一个线程可以访问共享数据");
    println!("   - lock() 方法获取锁，返回 MutexGuard");
    println!("   - MutexGuard 实现 Drop，自动释放锁");
    
    // 单线程中使用 Mutex
    println!("\n🔹 单线程中使用 Mutex:");
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
        println!("   修改后的值: {}", *num);
        // 锁在此处自动释放
    }
    println!("   最终值: {:?}", m);
    
    // 多线程中使用 Mutex
    println!("\n🔹 多线程中使用 Mutex:");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("   线程 {} 增加计数器，当前值: {}", i, *num);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("   最终计数器值: {}", *counter.lock().unwrap());
    
    // Mutex 的错误处理
    println!("\n🔹 Mutex 错误处理:");
    demonstrate_mutex_error_handling();
}

/// 演示 Mutex 错误处理
fn demonstrate_mutex_error_handling() {
    let mutex = Arc::new(Mutex::new(0));
    let mutex_clone = Arc::clone(&mutex);
    
    // 模拟线程 panic 导致锁中毒
    let handle = thread::spawn(move || {
        let _guard = mutex_clone.lock().unwrap();
        panic!("线程 panic，导致锁中毒");
    });
    
    // 等待线程结束（会 panic）
    let _ = handle.join();
    
    // 尝试获取被中毒的锁
    match mutex.lock() {
        Ok(_) => println!("   成功获取锁"),
        Err(poisoned) => {
            println!("   锁已中毒，但仍可使用: {:?}", poisoned.into_inner());
        }
    }
 }

/// 3. 条件变量 (Condvar) 实现
fn condvar_implementation_demo() {
    println!("\n🔔 条件变量基础概念：");
    println!("   - 与 Mutex 配合使用，实现线程间的条件等待");
    println!("   - wait() 方法释放锁并等待通知");
    println!("   - notify_one() 唤醒一个等待线程");
    println!("   - notify_all() 唤醒所有等待线程");
    
    // 生产者消费者模式
    println!("\n🔹 生产者消费者模式:");
    producer_consumer_demo();
    
    // 条件变量的虚假唤醒处理
    println!("\n🔹 条件变量虚假唤醒处理:");
    spurious_wakeup_demo();
}

/// 生产者消费者演示
fn producer_consumer_demo() {
    let pair = Arc::new((Mutex::new(Vec::new()), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    
    // 消费者线程
    let consumer = thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut buffer = lock.lock().unwrap();
        
        while buffer.is_empty() {
            println!("   消费者：缓冲区为空，等待生产者...");
            buffer = cvar.wait(buffer).unwrap();
        }
        
        let item = buffer.pop().unwrap();
        println!("   消费者：消费了项目 {}", item);
    });
    
    // 生产者线程
    thread::sleep(Duration::from_millis(100));
    let (lock, cvar) = &*pair;
    let mut buffer = lock.lock().unwrap();
    buffer.push(42);
    println!("   生产者：生产了项目 42");
    cvar.notify_one();
    drop(buffer);
    
    consumer.join().unwrap();
}

/// 虚假唤醒演示
fn spurious_wakeup_demo() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    
    let waiter = thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut ready = lock.lock().unwrap();
        
        // 使用 while 循环防止虚假唤醒
        while !*ready {
            println!("   等待线程：条件未满足，继续等待...");
            ready = cvar.wait(ready).unwrap();
        }
        
        println!("   等待线程：条件满足，继续执行！");
    });
    
    thread::sleep(Duration::from_millis(100));
    
    let (lock, cvar) = &*pair;
    let mut ready = lock.lock().unwrap();
    *ready = true;
    println!("   通知线程：设置条件为 true 并通知");
    cvar.notify_one();
    drop(ready);
    
    waiter.join().unwrap();
}

/// 4. 信号量 (Semaphore) 概念
fn semaphore_concepts_demo() {
    println!("\n🎫 信号量基础概念：");
    println!("   - 控制同时访问资源的线程数量");
    println!("   - 内部维护一个计数器");
    println!("   - acquire() 获取许可，计数器减1");
    println!("   - release() 释放许可，计数器加1");
    
    // 使用 AtomicUsize 模拟简单信号量
    println!("\n🔹 使用 AtomicUsize 模拟信号量:");
    simple_semaphore_demo();
    
    // 资源池管理
    println!("\n🔹 资源池管理示例:");
    resource_pool_demo();
}

/// 简单信号量演示
fn simple_semaphore_demo() {
    struct SimpleSemaphore {
        permits: AtomicUsize,
    }
    
    impl SimpleSemaphore {
        fn new(permits: usize) -> Self {
            Self {
                permits: AtomicUsize::new(permits),
            }
        }
        
        fn acquire(&self) -> bool {
            loop {
                let current = self.permits.load(Ordering::Acquire);
                if current == 0 {
                    return false;
                }
                
                if self.permits.compare_exchange_weak(
                    current,
                    current - 1,
                    Ordering::Release,
                    Ordering::Relaxed,
                ).is_ok() {
                    return true;
                }
            }
        }
        
        fn release(&self) {
            self.permits.fetch_add(1, Ordering::Release);
        }
    }
    
    let semaphore = Arc::new(SimpleSemaphore::new(3));
    let mut handles = vec![];
    
    for i in 0..6 {
        let sem = Arc::clone(&semaphore);
        let handle = thread::spawn(move || {
            if sem.acquire() {
                println!("   线程 {} 获取到许可，开始工作", i);
                thread::sleep(Duration::from_millis(200));
                println!("   线程 {} 完成工作，释放许可", i);
                sem.release();
            } else {
                println!("   线程 {} 未能获取许可", i);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

/// 资源池管理演示
fn resource_pool_demo() {
    struct ResourcePool {
        resources: Mutex<Vec<String>>,
        available: Condvar,
    }
    
    impl ResourcePool {
        fn new(size: usize) -> Self {
            let mut resources = Vec::new();
            for i in 0..size {
                resources.push(format!("Resource-{}", i));
            }
            
            Self {
                resources: Mutex::new(resources),
                available: Condvar::new(),
            }
        }
        
        fn acquire(&self) -> String {
            let mut resources = self.resources.lock().unwrap();
            
            while resources.is_empty() {
                resources = self.available.wait(resources).unwrap();
            }
            
            resources.pop().unwrap()
        }
        
        fn release(&self, resource: String) {
            let mut resources = self.resources.lock().unwrap();
            resources.push(resource);
            self.available.notify_one();
        }
    }
    
    let pool = Arc::new(ResourcePool::new(2));
    let mut handles = vec![];
    
    for i in 0..4 {
        let pool = Arc::clone(&pool);
        let handle = thread::spawn(move || {
            let resource = pool.acquire();
            println!("   线程 {} 获取到资源: {}", i, resource);
            thread::sleep(Duration::from_millis(300));
            println!("   线程 {} 释放资源: {}", i, resource);
            pool.release(resource);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

/// 5. 死锁分析与避免
fn deadlock_analysis_demo() {
    println!("\n💀 死锁基础概念：");
    println!("   - 互斥条件：资源不能被共享");
    println!("   - 持有并等待：线程持有资源的同时等待其他资源");
    println!("   - 不可剥夺：资源不能被强制释放");
    println!("   - 循环等待：形成资源等待环路");
    
    // 死锁示例（注释掉以避免程序卡死）
    println!("\n🔹 死锁示例（已注释避免卡死）:");
    println!("   // 两个线程以不同顺序获取两个锁");
    
    // 死锁避免策略
    println!("\n🔹 死锁避免策略:");
    deadlock_avoidance_demo();
}

/// 死锁避免演示
fn deadlock_avoidance_demo() {
    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));
    
    // 策略1：统一锁顺序
    println!("   策略1：统一锁顺序");
    let m1 = Arc::clone(&mutex1);
    let m2 = Arc::clone(&mutex2);
    
    let handle1 = thread::spawn(move || {
        let _g1 = m1.lock().unwrap();
        println!("     线程1获取锁1");
        thread::sleep(Duration::from_millis(10));
        let _g2 = m2.lock().unwrap();
        println!("     线程1获取锁2");
    });
    
    let m1 = Arc::clone(&mutex1);
    let m2 = Arc::clone(&mutex2);
    
    let handle2 = thread::spawn(move || {
        let _g1 = m1.lock().unwrap(); // 同样先获取锁1
        println!("     线程2获取锁1");
        thread::sleep(Duration::from_millis(10));
        let _g2 = m2.lock().unwrap();
        println!("     线程2获取锁2");
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    // 策略2：使用 try_lock
    println!("   策略2：使用 try_lock");
    try_lock_demo();
}

/// try_lock 演示
fn try_lock_demo() {
    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));
    
    let m1 = Arc::clone(&mutex1);
    let m2 = Arc::clone(&mutex2);
    
    let handle = thread::spawn(move || {
        let _g1 = m1.lock().unwrap();
        println!("     线程获取锁1成功");
        
        match m2.try_lock() {
            Ok(_g2) => {
                println!("     线程获取锁2成功");
            }
            Err(_) => {
                println!("     线程获取锁2失败，避免死锁");
            }
        }
    });
    
    handle.join().unwrap();
}

/// 6. 同步原语性能对比
fn sync_primitives_comparison_demo() {
    println!("\n⚡ 同步原语性能对比:");
    
    const ITERATIONS: usize = 1_000_000;
    
    // Atomic 性能测试
    let atomic = Arc::new(AtomicUsize::new(0));
    let start = Instant::now();
    
    let mut handles = vec![];
    for _ in 0..4 {
        let atomic = Arc::clone(&atomic);
        let handle = thread::spawn(move || {
            for _ in 0..ITERATIONS {
                atomic.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let atomic_duration = start.elapsed();
    println!("   Atomic 操作耗时: {:?}", atomic_duration);
    
    // Mutex 性能测试
    let mutex = Arc::new(Mutex::new(0));
    let start = Instant::now();
    
    let mut handles = vec![];
    for _ in 0..4 {
        let mutex = Arc::clone(&mutex);
        let handle = thread::spawn(move || {
            for _ in 0..ITERATIONS {
                let mut num = mutex.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let mutex_duration = start.elapsed();
    println!("   Mutex 操作耗时: {:?}", mutex_duration);
    
    println!("   性能比较: Atomic 比 Mutex 快 {:.2}x", 
             mutex_duration.as_nanos() as f64 / atomic_duration.as_nanos() as f64);
}

/// 7. 高级同步模式
fn advanced_patterns_demo() {
    println!("\n🎯 高级同步模式:");
    
    // 读写者问题
    println!("\n🔹 读写者问题:");
    readers_writers_demo();
    
    // 哲学家就餐问题
    println!("\n🔹 哲学家就餐问题:");
    philosophers_demo();
}

/// 读写者问题演示
fn readers_writers_demo() {
    let data = Arc::new(RwLock::new(0));
    let mut handles = vec![];
    
    // 启动多个读者
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let value = data.read().unwrap();
            println!("   读者 {} 读取到: {}", i, *value);
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }
    
    // 启动写者
    let data_clone = Arc::clone(&data);
    let writer = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let mut value = data_clone.write().unwrap();
        *value = 42;
        println!("   写者修改值为: {}", *value);
    });
    
    for handle in handles {
        handle.join().unwrap();
    }
    writer.join().unwrap();
}

/// 哲学家就餐问题演示
fn philosophers_demo() {
    struct Philosopher {
        name: String,
        left_fork: usize,
        right_fork: usize,
    }
    
    impl Philosopher {
        fn new(name: &str, left: usize, right: usize) -> Philosopher {
            Philosopher {
                name: name.to_string(),
                left_fork: left,
                right_fork: right,
            }
        }
        
        fn eat(&self, forks: &Vec<Mutex<()>>) {
            let _left = forks[self.left_fork].lock().unwrap();
            let _right = forks[self.right_fork].lock().unwrap();
            
            println!("   {} 正在用餐", self.name);
            thread::sleep(Duration::from_millis(100));
            println!("   {} 用餐完毕", self.name);
        }
    }
    
    let philosophers = vec![
        Philosopher::new("苏格拉底", 0, 1),
        Philosopher::new("柏拉图", 1, 2),
        Philosopher::new("亚里士多德", 2, 3),
        Philosopher::new("笛卡尔", 3, 4),
        Philosopher::new("康德", 0, 4), // 最后一个哲学家的叉子顺序相反，避免死锁
    ];
    
    let forks = Arc::new(vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]);
    
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let forks = Arc::clone(&forks);
        thread::spawn(move || {
            p.eat(&forks);
        })
    }).collect();
    
    for h in handles {
        h.join().unwrap();
    }
}

/// 8. 实际应用案例
fn real_world_examples_demo() {
    println!("\n🌍 实际应用案例:");
    
    // 缓存系统
    println!("\n🔹 线程安全缓存系统:");
    cache_system_demo();
    
    // 连接池
    println!("\n🔹 数据库连接池:");
    connection_pool_demo();
}

/// 缓存系统演示
fn cache_system_demo() {
    struct ThreadSafeCache {
        data: RwLock<HashMap<String, String>>,
    }
    
    impl ThreadSafeCache {
        fn new() -> Self {
            Self {
                data: RwLock::new(HashMap::new()),
            }
        }
        
        fn get(&self, key: &str) -> Option<String> {
            let data = self.data.read().unwrap();
            data.get(key).cloned()
        }
        
        fn set(&self, key: String, value: String) {
            let mut data = self.data.write().unwrap();
            data.insert(key, value);
        }
    }
    
    let cache = Arc::new(ThreadSafeCache::new());
    let mut handles = vec![];
    
    // 写入数据
    for i in 0..3 {
        let cache = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            cache.set(format!("key{}", i), format!("value{}", i));
            println!("   写入 key{} = value{}", i, i);
        });
        handles.push(handle);
    }
    
    // 读取数据
    for i in 0..5 {
        let cache = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            if let Some(value) = cache.get(&format!("key{}", i % 3)) {
                println!("   读取 key{} = {}", i % 3, value);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

/// 连接池演示
fn connection_pool_demo() {
    struct Connection {
        id: usize,
    }
    
    impl Connection {
        fn new(id: usize) -> Self {
            Self { id }
        }
        
        fn execute_query(&self, query: &str) {
            println!("   连接 {} 执行查询: {}", self.id, query);
            thread::sleep(Duration::from_millis(100));
        }
    }
    
    struct ConnectionPool {
        connections: Mutex<Vec<Connection>>,
        available: Condvar,
    }
    
    impl ConnectionPool {
        fn new(size: usize) -> Self {
            let mut connections = Vec::new();
            for i in 0..size {
                connections.push(Connection::new(i));
            }
            
            Self {
                connections: Mutex::new(connections),
                available: Condvar::new(),
            }
        }
        
        fn get_connection(&self) -> Connection {
            let mut connections = self.connections.lock().unwrap();
            
            while connections.is_empty() {
                connections = self.available.wait(connections).unwrap();
            }
            
            connections.pop().unwrap()
        }
        
        fn return_connection(&self, conn: Connection) {
            let mut connections = self.connections.lock().unwrap();
            connections.push(conn);
            self.available.notify_one();
        }
    }
    
    let pool = Arc::new(ConnectionPool::new(2));
    let mut handles = vec![];
    
    for i in 0..5 {
        let pool = Arc::clone(&pool);
        let handle = thread::spawn(move || {
            let conn = pool.get_connection();
            conn.execute_query(&format!("SELECT * FROM table{}", i));
            pool.return_connection(conn);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

/// 2. 读写锁 (RwLock) 深入分析
fn rwlock_analysis_demo() {
    println!("\n📖 读写锁基础概念：");
    println!("   - 允许多个读者同时访问数据");
    println!("   - 只允许一个写者独占访问数据");
    println!("   - 读写互斥，写写互斥");
    
    let rwlock = Arc::new(RwLock::new(5));
    
    // 多个读者同时访问
    println!("\n🔹 多个读者同时访问:");
    let mut read_handles = vec![];
    
    for i in 0..5 {
        let rwlock = Arc::clone(&rwlock);
        let handle = thread::spawn(move || {
            let num = rwlock.read().unwrap();
            println!("   读者线程 {} 读取到值: {}", i, *num);
            thread::sleep(Duration::from_millis(100)); // 模拟读取操作
        });
        read_handles.push(handle);
    }
    
    for handle in read_handles {
        handle.join().unwrap();
    }
    
    // 写者独占访问
    println!("\n🔹 写者独占访问:");
    let mut write_handles = vec![];
    
    for i in 0..3 {
        let rwlock = Arc::clone(&rwlock);
        let handle = thread::spawn(move || {
            let mut num = rwlock.write().unwrap();
            *num += i;
            println!("   写者线程 {} 修改值为: {}", i, *num);
            thread::sleep(Duration::from_millis(50));
        });
        write_handles.push(handle);
    }
    
    for handle in write_handles {
        handle.join().unwrap();
    }
    
    println!("   最终值: {}", *rwlock.read().unwrap());
    
    // RwLock 性能对比
    println!("\n🔹 RwLock vs Mutex 性能对比:");
    rwlock_vs_mutex_performance();
}

/// RwLock vs Mutex 性能对比
fn rwlock_vs_mutex_performance() {
    const NUM_READERS: usize = 100;
    const NUM_OPERATIONS: usize = 1000;
    
    // 测试 RwLock 读性能
    let rwlock = Arc::new(RwLock::new(0));
    let start = Instant::now();
    
    let mut handles = vec![];
    for _ in 0..NUM_READERS {
        let rwlock = Arc::clone(&rwlock);
        let handle = thread::spawn(move || {
            for _ in 0..NUM_OPERATIONS {
                let _guard = rwlock.read().unwrap();
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let rwlock_duration = start.elapsed();
    println!("   RwLock 读操作耗时: {:?}", rwlock_duration);
    
    // 测试 Mutex 读性能
    let mutex = Arc::new(Mutex::new(0));
    let start = Instant::now();
    
    let mut handles = vec![];
    for _ in 0..NUM_READERS {
        let mutex = Arc::clone(&mutex);
        let handle = thread::spawn(move || {
            for _ in 0..NUM_OPERATIONS {
                let _guard = mutex.lock().unwrap();
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let mutex_duration = start.elapsed();
    println!("   Mutex 读操作耗时: {:?}", mutex_duration);
    
    if rwlock_duration < mutex_duration {
        println!("   ✅ RwLock 在读密集场景下性能更优");
    } else {
        println!("   ⚠️  在此测试中 Mutex 性能更优（可能由于线程数量较少）");
    }
}
