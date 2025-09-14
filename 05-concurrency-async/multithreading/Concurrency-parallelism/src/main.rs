//! Rust 并发与并行深度教程
//! 
//! 本教程全面分析 Rust 中的并发与并行编程，涵盖：
//! 1. 并发与并行概念深度分析
//! 2. 线程基础与生命周期管理
//! 3. 共享状态并发编程
//! 4. 消息传递并发模式
//! 5. Send 和 Sync 特征深度解析
//! 6. 原子操作与无锁编程
//! 7. 实际应用案例
//! 8. 性能分析与优化

use std::{
    sync::{Arc, Mutex, RwLock, mpsc, Barrier},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
    collections::HashMap,
    sync::atomic::{AtomicUsize, AtomicBool, Ordering},
    cell::RefCell,
};

fn main() {
    println!("🚀 Rust 并发与并行深度教程");
    println!("{}", "=".repeat(50));
    
    // 1. 并发与并行概念分析
    analyze_concurrency_parallelism_concepts();
    
    // 2. 线程基础演示
    demonstrate_thread_fundamentals();
    
    // 3. 共享状态并发
    demonstrate_shared_state_concurrency();
    
    // 4. 消息传递
    demonstrate_message_passing();
    
    // 5. Send 和 Sync 特征
    analyze_send_sync_traits();
    
    // 6. 原子操作
    demonstrate_atomic_operations();
    
    // 7. 实际应用案例
    build_practical_examples();
    
    // 8. 性能分析
    performance_analysis();
    
    println!("\n✅ 教程完成！");
}

/// 1. 并发与并行概念深度分析
fn analyze_concurrency_parallelism_concepts() {
    println!("\n📚 1. 并发与并行概念深度分析");
    println!("{}", "-".repeat(30));
    
    // 并发 vs 并行的核心区别
    println!("🔍 并发 vs 并行的核心区别：");
    println!("   • 并发(Concurrency): 同时处理多个任务，但不一定同时执行");
    println!("   • 并行(Parallelism): 真正同时执行多个任务");
    
    // CPU 核心与线程关系
    let cpu_cores = thread::available_parallelism().unwrap().get();
    println!("\n💻 系统信息：");
    println!("   • 可用 CPU 核心数: {}", cpu_cores);
    println!("   • 理论最大并行线程数: {}", cpu_cores);
    
    // 演示并发执行
    println!("\n🔄 并发执行演示（单核模拟）：");
    let start = Instant::now();
    
    let handles: Vec<JoinHandle<()>> = (0..4).map(|i| {
        thread::spawn(move || {
            let thread_id = format!("Thread-{}", i);
            for j in 0..3 {
                println!("   {} 执行任务 {}", thread_id, j + 1);
                thread::sleep(Duration::from_millis(100));
            }
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("   总耗时: {:?}", start.elapsed());
    
    // M:N 线程模型说明
    println!("\n🏗️ Rust 线程模型特点：");
    println!("   • 1:1 线程模型（每个 Rust 线程对应一个 OS 线程）");
    println!("   • 相比 M:N 模型，提供更好的性能预测性");
    println!("   • 通过 async/await 实现轻量级并发");
}

/// 2. 线程基础与生命周期管理
fn demonstrate_thread_fundamentals() {
    println!("\n🧵 2. 线程基础与生命周期管理");
    println!("{}", "-".repeat(30));
    
    // 基本线程创建
    println!("🚀 基本线程创建：");
    let handle = thread::spawn(|| {
        println!("   子线程执行中...");
        thread::current().id()
    });
    
    let thread_id = handle.join().unwrap();
    println!("   子线程 ID: {:?}", thread_id);
    
    // move 语义演示
    println!("\n📦 move 语义演示：");
    let data = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        println!("   线程中的数据: {:?}", data);
        data.len()
    });
    
    let len = handle.join().unwrap();
    println!("   数据长度: {}", len);
    // 注意：这里 data 已经被 move，无法再使用
    
    // 线程生命周期管理
    println!("\n⏰ 线程生命周期管理：");
    let handles: Vec<_> = (0..3).map(|i| {
        thread::Builder::new()
            .name(format!("worker-{}", i))
            .spawn(move || {
                println!("   {} 开始工作", thread::current().name().unwrap());
                thread::sleep(Duration::from_millis(100 * (i + 1)));
                println!("   {} 完成工作", thread::current().name().unwrap());
                i * 2
            })
            .unwrap()
    }).collect();
    
    let results: Vec<_> = handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    println!("   工作结果: {:?}", results);
}

/// 3. 共享状态并发编程
fn demonstrate_shared_state_concurrency() {
    println!("\n🔒 3. 共享状态并发编程");
    println!("{}", "-".repeat(30));
    
    // Mutex 基础使用
    println!("🔐 Mutex 基础使用：");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let mut num = counter.lock().unwrap();
                *num += 1;
                // 锁在这里自动释放
            }
            println!("   线程 {} 完成计数", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("   最终计数: {}", *counter.lock().unwrap());
    
    // RwLock 演示
    println!("\n📖 RwLock 读写锁演示：");
    let data = Arc::new(RwLock::new(HashMap::new()));
    let mut handles = vec![];
    
    // 写入线程
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut map = data.write().unwrap();
            map.insert(format!("key{}", i), i * 10);
            println!("   写入线程 {} 完成", i);
        });
        handles.push(handle);
    }
    
    // 读取线程
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10)); // 确保有数据可读
            let map = data.read().unwrap();
            println!("   读取线程 {} 看到 {} 个条目", i, map.len());
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // 死锁预防演示
    println!("\n⚠️ 死锁预防策略：");
    demonstrate_deadlock_prevention();
}

/// 死锁预防演示
fn demonstrate_deadlock_prevention() {
    let resource1 = Arc::new(Mutex::new("Resource1"));
    let resource2 = Arc::new(Mutex::new("Resource2"));
    
    println!("   策略1: 锁排序 - 总是按相同顺序获取锁");
    
    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);
    let handle1 = thread::spawn(move || {
        let _lock1 = r1.lock().unwrap(); // 先锁 resource1
        thread::sleep(Duration::from_millis(10));
        let _lock2 = r2.lock().unwrap(); // 再锁 resource2
        println!("     线程1 获取了两个锁");
    });
    
    let r1 = Arc::clone(&resource1);
    let r2 = Arc::clone(&resource2);
    let handle2 = thread::spawn(move || {
        let _lock1 = r1.lock().unwrap(); // 同样先锁 resource1
        thread::sleep(Duration::from_millis(10));
        let _lock2 = r2.lock().unwrap(); // 再锁 resource2
        println!("     线程2 获取了两个锁");
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("   ✅ 成功避免死锁");
}

/// 4. 消息传递并发模式
fn demonstrate_message_passing() {
    println!("\n📨 4. 消息传递并发模式");
    println!("{}", "-".repeat(30));
    
    // 基本 channel 使用
    println!("📡 基本 channel 使用：");
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let messages = vec!["Hello", "from", "thread"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    for received in rx {
        println!("   收到消息: {}", received);
    }
    
    // 多生产者单消费者
    println!("\n🏭 多生产者单消费者模式：");
    let (tx, rx) = mpsc::channel();
    
    for i in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            for j in 0..3 {
                let msg = format!("Producer {} - Message {}", i, j);
                tx.send(msg).unwrap();
                thread::sleep(Duration::from_millis(50));
            }
        });
    }
    
    drop(tx); // 关闭原始发送者
    
    let mut count = 0;
    for received in rx {
        println!("   {}", received);
        count += 1;
    }
    println!("   总共收到 {} 条消息", count);
    
    // 同步 channel
    println!("\n🔄 同步 channel (有界队列)：");
    let (tx, rx) = mpsc::sync_channel(2); // 缓冲区大小为 2
    
    let handle = thread::spawn(move || {
        for i in 0..5 {
            println!("   发送消息 {}", i);
            tx.send(i).unwrap();
            println!("   消息 {} 已发送", i);
        }
    });
    
    thread::sleep(Duration::from_millis(100));
    
    for received in rx {
        println!("   处理消息: {}", received);
        thread::sleep(Duration::from_millis(200)); // 模拟慢消费者
    }
    
    handle.join().unwrap();
}

/// 5. Send 和 Sync 特征深度解析
fn analyze_send_sync_traits() {
    println!("\n🏷️ 5. Send 和 Sync 特征深度解析");
    println!("{}", "-".repeat(30));
    
    println!("📋 Send 和 Sync 特征说明：");
    println!("   • Send: 类型可以安全地在线程间转移所有权");
    println!("   • Sync: 类型可以安全地在线程间共享引用");
    
    // 演示 Send 类型
    println!("\n✅ Send 类型示例：");
    demonstrate_send_types();
    
    // 演示 Sync 类型
    println!("\n🔄 Sync 类型示例：");
    demonstrate_sync_types();
    
    // 非 Send/Sync 类型
    println!("\n❌ 非 Send/Sync 类型示例：");
    demonstrate_non_send_sync_types();
}

fn demonstrate_send_types() {
    // 大多数基本类型都是 Send
    let data = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        println!("   Vec<i32> 是 Send: {:?}", data);
    });
    handle.join().unwrap();
    
    // Arc<T> 是 Send（如果 T 是 Send + Sync）
    let shared_data = Arc::new(vec![1, 2, 3]);
    let shared_clone = Arc::clone(&shared_data);
    
    let handle = thread::spawn(move || {
        println!("   Arc<Vec<i32>> 是 Send: {:?}", shared_clone);
    });
    handle.join().unwrap();
}

fn demonstrate_sync_types() {
    // Mutex<T> 是 Sync
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..3 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += i;
            println!("   Mutex<i32> 是 Sync，线程 {} 更新计数器", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("   最终值: {}", *counter.lock().unwrap());
}

fn demonstrate_non_send_sync_types() {
    println!("   • Rc<T> 不是 Send（引用计数不是原子的）");
    println!("   • RefCell<T> 不是 Sync（内部可变性不是线程安全的）");
    println!("   • 裸指针 *const T 和 *mut T 不是 Send 也不是 Sync");
    
    // 演示为什么 RefCell 不是 Sync
    let data = RefCell::new(vec![1, 2, 3]);
    println!("   RefCell 在单线程中工作正常: {:?}", data.borrow());
    
    // 以下代码会编译失败，因为 RefCell 不是 Sync
    // let shared = Arc::new(data);
    // thread::spawn(move || {
    //     shared.borrow_mut().push(4);
    // });
}

/// 6. 原子操作与无锁编程
fn demonstrate_atomic_operations() {
    println!("\n⚛️ 6. 原子操作与无锁编程");
    println!("{}", "-".repeat(30));
    
    // 原子计数器
    println!("🔢 原子计数器演示：");
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
            println!("   线程 {} 完成原子递增", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("   原子计数器最终值: {}", counter.load(Ordering::SeqCst));
    
    // 内存顺序演示
    println!("\n🧠 内存顺序演示：");
    demonstrate_memory_ordering();
    
    // Compare-and-Swap (CAS) 操作
    println!("\n🔄 Compare-and-Swap 操作：");
    demonstrate_cas_operations();
}

fn demonstrate_memory_ordering() {
    let flag = Arc::new(AtomicBool::new(false));
    let data = Arc::new(AtomicUsize::new(0));
    
    let flag_clone = Arc::clone(&flag);
    let data_clone = Arc::clone(&data);
    
    // 写入线程
    let writer = thread::spawn(move || {
        data_clone.store(42, Ordering::Relaxed);
        flag_clone.store(true, Ordering::Release); // Release 语义
        println!("   写入线程：数据已准备好");
    });
    
    // 读取线程
    let reader = thread::spawn(move || {
        while !flag.load(Ordering::Acquire) { // Acquire 语义
            thread::yield_now();
        }
        let value = data.load(Ordering::Relaxed);
        println!("   读取线程：读到数据 {}", value);
    });
    
    writer.join().unwrap();
    reader.join().unwrap();
}

fn demonstrate_cas_operations() {
    let value = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    // 多个线程尝试 CAS 操作
    for i in 0..3 {
        let value = Arc::clone(&value);
        let handle = thread::spawn(move || {
            let expected = 0;
            let new_value = (i + 1) * 10;
            
            match value.compare_exchange(
                expected,
                new_value,
                Ordering::SeqCst,
                Ordering::SeqCst
            ) {
                Ok(prev) => println!("   线程 {} CAS 成功: {} -> {}", i, prev, new_value),
                Err(actual) => println!("   线程 {} CAS 失败: 期望 {}, 实际 {}", i, expected, actual),
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("   最终值: {}", value.load(Ordering::SeqCst));
}

/// 7. 实际应用案例
fn build_practical_examples() {
    println!("\n🏗️ 7. 实际应用案例");
    println!("{}", "-".repeat(30));
    
    // 生产者-消费者模式
    println!("🏭 生产者-消费者模式：");
    producer_consumer_example();
    
    // 工作池模式
    println!("\n👷 工作池模式：");
    thread_pool_example();
    
    // 并发数据结构
    println!("\n📊 并发数据结构：");
    concurrent_data_structure_example();
}

fn producer_consumer_example() {
    let (tx, rx) = mpsc::channel();
    let _buffer_size = 10;
    
    // 生产者
    let producer = thread::spawn(move || {
        for i in 0..20 {
            let item = format!("Item-{}", i);
            tx.send(item.clone()).unwrap();
            println!("   生产: {}", item);
            thread::sleep(Duration::from_millis(50));
        }
    });
    
    // 消费者
    let consumer = thread::spawn(move || {
        for item in rx {
            println!("   消费: {}", item);
            thread::sleep(Duration::from_millis(100)); // 消费比生产慢
        }
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
}

fn thread_pool_example() {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    let mut workers = vec![];
    
    // 创建工作线程
    for id in 0..3 {
        let rx = Arc::clone(&rx);
        let worker = thread::spawn(move || {
            loop {
                let job = {
                    let rx = rx.lock().unwrap();
                    rx.recv()
                };
                
                match job {
                    Ok(job) => {
                        println!("   Worker {} 执行任务: {}", id, job);
                        thread::sleep(Duration::from_millis(100));
                    }
                    Err(_) => {
                        println!("   Worker {} 退出", id);
                        break;
                    }
                }
            }
        });
        workers.push(worker);
    }
    
    // 发送任务
    for i in 0..9 {
        tx.send(format!("Task-{}", i)).unwrap();
    }
    
    drop(tx); // 关闭发送端，让工作线程退出
    
    for worker in workers {
        worker.join().unwrap();
    }
}

fn concurrent_data_structure_example() {
    // 并发哈希表
    let map = Arc::new(RwLock::new(HashMap::new()));
    let mut handles = vec![];
    
    // 写入线程
    for i in 0..3 {
        let map = Arc::clone(&map);
        let handle = thread::spawn(move || {
            for j in 0..5 {
                let key = format!("key-{}-{}", i, j);
                let value = i * 10 + j;
                map.write().unwrap().insert(key.clone(), value);
                println!("   插入: {} = {}", key, value);
            }
        });
        handles.push(handle);
    }
    
    // 读取线程
    for i in 0..2 {
        let map = Arc::clone(&map);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50)); // 等待一些数据
            let map = map.read().unwrap();
            println!("   读取线程 {} 看到 {} 个条目", i, map.len());
            for (k, v) in map.iter().take(3) {
                println!("     {} = {}", k, v);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

/// 8. 性能分析与优化
fn performance_analysis() {
    println!("\n📈 8. 性能分析与优化");
    println!("{}", "-".repeat(30));
    
    // 锁竞争分析
    println!("🔒 锁竞争性能分析：");
    analyze_lock_contention();
    
    // 原子操作 vs 锁性能对比
    println!("\n⚛️ 原子操作 vs 锁性能对比：");
    compare_atomic_vs_mutex();
    
    // 并发性能基准测试
    println!("\n🏃 并发性能基准测试：");
    concurrency_benchmark();
}

fn analyze_lock_contention() {
    let iterations = 10000;
    
    // 高竞争场景
    let counter = Arc::new(Mutex::new(0));
    let start = Instant::now();
    
    let handles: Vec<_> = (0..4).map(|_| {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {
            for _ in 0..iterations {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let high_contention_time = start.elapsed();
    println!("   高竞争场景耗时: {:?}", high_contention_time);
    
    // 低竞争场景（每个线程有自己的工作）
    let counters: Vec<_> = (0..4).map(|_| Arc::new(Mutex::new(0))).collect();
    let start = Instant::now();
    
    let handles: Vec<_> = counters.into_iter().enumerate().map(|(_i, counter)| {
        thread::spawn(move || {
            for _ in 0..iterations {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let low_contention_time = start.elapsed();
    println!("   低竞争场景耗时: {:?}", low_contention_time);
    println!("   性能提升: {:.2}x", 
             high_contention_time.as_nanos() as f64 / low_contention_time.as_nanos() as f64);
}

fn compare_atomic_vs_mutex() {
    let iterations = 100000;
    
    // 原子操作性能
    let atomic_counter = Arc::new(AtomicUsize::new(0));
    let start = Instant::now();
    
    let handles: Vec<_> = (0..4).map(|_| {
        let counter = Arc::clone(&atomic_counter);
        thread::spawn(move || {
            for _ in 0..iterations {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let atomic_time = start.elapsed();
    println!("   原子操作耗时: {:?}", atomic_time);
    
    // Mutex 性能
    let mutex_counter = Arc::new(Mutex::new(0));
    let start = Instant::now();
    
    let handles: Vec<_> = (0..4).map(|_| {
        let counter = Arc::clone(&mutex_counter);
        thread::spawn(move || {
            for _ in 0..iterations {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let mutex_time = start.elapsed();
    println!("   Mutex 耗时: {:?}", mutex_time);
    println!("   原子操作性能提升: {:.2}x", 
             mutex_time.as_nanos() as f64 / atomic_time.as_nanos() as f64);
}

fn concurrency_benchmark() {
    let work_size = 1000000usize;
    
    // 单线程基准
    let start = Instant::now();
    let mut sum = 0u64;
    for i in 0..work_size {
        sum = sum.wrapping_add(i as u64);
    }
    let single_thread_time = start.elapsed();
    println!("   单线程耗时: {:?}, 结果: {}", single_thread_time, sum);
    
    // 多线程基准
    let num_threads = thread::available_parallelism().unwrap().get();
    let chunk_size = work_size / num_threads;
    
    let start = Instant::now();
    let handles: Vec<_> = (0..num_threads).map(|i| {
        let start_idx = i * chunk_size;
        let end_idx = if i == num_threads - 1 { work_size } else { (i + 1) * chunk_size };
        
        thread::spawn(move || {
            let mut local_sum = 0u64;
            for j in start_idx..end_idx {
                local_sum = local_sum.wrapping_add(j as u64);
            }
            local_sum
        })
    }).collect();
    
    let mut total_sum = 0u64;
    for handle in handles {
        total_sum = total_sum.wrapping_add(handle.join().unwrap());
    }
    
    let multi_thread_time = start.elapsed();
    println!("   多线程({})耗时: {:?}, 结果: {}", num_threads, multi_thread_time, total_sum);
    
    if multi_thread_time.as_nanos() > 0 {
        let speedup = single_thread_time.as_nanos() as f64 / multi_thread_time.as_nanos() as f64;
        println!("   加速比: {:.2}x", speedup);
        println!("   并行效率: {:.1}%", (speedup / num_threads as f64) * 100.0);
    }
}

// 测试模块
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_thread_safety() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let mut num = counter.lock().unwrap();
                    *num += 1;
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(*counter.lock().unwrap(), 1000);
    }
    
    #[test]
    fn test_message_passing() {
        let (tx, rx) = mpsc::channel();
        
        thread::spawn(move || {
            for i in 0..5 {
                tx.send(i).unwrap();
            }
        });
        
        let mut sum = 0;
        for received in rx {
            sum += received;
        }
        
        assert_eq!(sum, 10); // 0+1+2+3+4 = 10
    }
    
    #[test]
    fn test_atomic_operations() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];
        
        for _ in 0..5 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..200 {
                    counter.fetch_add(1, Ordering::SeqCst);
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(counter.load(Ordering::SeqCst), 1000);
    }
    
    #[test]
    fn test_rwlock_functionality() {
        let data = Arc::new(RwLock::new(0));
        let mut handles = vec![];
        
        // 写入线程
        let data_clone = Arc::clone(&data);
        let writer = thread::spawn(move || {
            let mut num = data_clone.write().unwrap();
            *num = 42;
        });
        handles.push(writer);
        
        // 读取线程
        for _ in 0..3 {
            let data_clone = Arc::clone(&data);
            let reader = thread::spawn(move || {
                thread::sleep(Duration::from_millis(10));
                let num = data_clone.read().unwrap();
                assert_eq!(*num, 42);
            });
            handles.push(reader);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
