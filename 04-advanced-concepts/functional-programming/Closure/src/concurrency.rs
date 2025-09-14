//! # 并发编程中的闭包
//!
//! 本模块演示闭包在并发编程中的应用，包括线程、异步编程、
//! 以及闭包需要满足的 Send 和 Sync 约束。

use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

/// 演示并发编程中的闭包使用
pub fn demonstrate() {
    println!("\n=== 并发编程中的闭包 ===");

    demonstrate_thread_closures();
    demonstrate_send_sync_constraints();
    demonstrate_shared_state();
    demonstrate_message_passing();
    demonstrate_thread_pool_pattern();
}

/// 演示线程中的闭包使用
fn demonstrate_thread_closures() {
    println!("\n10.1 线程中的闭包使用:");

    // 基本的线程闭包
    let data = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        println!("线程中处理数据: {:?}", data);
        data.iter().sum::<i32>()
    });

    let result = handle.join().unwrap();
    println!("线程计算结果: {}", result);

    // 多个线程并行处理
    let numbers: Vec<i32> = (1..=10).collect();
    let chunk_size = 2;
    let mut handles = vec![];

    for chunk in numbers.chunks(chunk_size) {
        let chunk = chunk.to_vec(); // 克隆数据
        let handle = thread::spawn(move || {
            let sum: i32 = chunk.iter().sum();
            println!("线程处理 {:?}, 和为: {}", chunk, sum);
            sum
        });
        handles.push(handle);
    }

    let total: i32 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("所有线程计算总和: {}", total);
}

/// 演示 Send 和 Sync 约束
fn demonstrate_send_sync_constraints() {
    println!("\n10.2 Send 和 Sync 约束:");

    // Send: 可以在线程间转移所有权
    let send_closure = move || {
        println!("这个闭包实现了 Send trait");
        42
    };

    let handle = thread::spawn(send_closure);
    let result = handle.join().unwrap();
    println!("Send 闭包结果: {}", result);

    // Sync: 可以在多个线程间共享引用
    let sync_data = Arc::new("共享数据");
    let mut handles = vec![];

    for i in 0..3 {
        let data = Arc::clone(&sync_data);
        let handle = thread::spawn(move || {
            println!("线程 {} 访问: {}", i, data);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/// 演示共享状态的闭包
fn demonstrate_shared_state() {
    println!("\n10.3 共享状态的闭包:");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let increment_closure = || {
                let mut num = counter.lock().unwrap();
                *num += 1;
                println!("线程 {} 增加计数器到: {}", i, *num);
            };

            // 模拟一些工作
            thread::sleep(Duration::from_millis(10));
            increment_closure();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终计数器值: {}", *counter.lock().unwrap());
}

/// 演示消息传递模式
fn demonstrate_message_passing() {
    println!("\n10.4 消息传递模式:");

    let (tx, rx) = mpsc::channel();

    // 生产者线程
    let producer = thread::spawn(move || {
        let produce_message = |id: i32| {
            let message = format!("消息 {}", id);
            println!("生产: {}", message);
            message
        };

        for i in 1..=5 {
            let msg = produce_message(i);
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 消费者线程
    let consumer = thread::spawn(move || {
        let process_message = |msg: String| {
            println!("处理: {}", msg);
            msg.len()
        };

        let mut total_length = 0;
        while let Ok(msg) = rx.recv() {
            let length = process_message(msg);
            total_length += length;
        }
        total_length
    });

    producer.join().unwrap();
    let total = consumer.join().unwrap();
    println!("消息总长度: {}", total);
}

/// 演示线程池模式
fn demonstrate_thread_pool_pattern() {
    println!("\n10.5 线程池模式:");

    // 简单的任务队列
    let tasks = vec![
        Box::new(|| {
            thread::sleep(Duration::from_millis(100));
            println!("任务 1 完成");
            1
        }) as Box<dyn Fn() -> i32 + Send>,
        Box::new(|| {
            thread::sleep(Duration::from_millis(150));
            println!("任务 2 完成");
            2
        }),
        Box::new(|| {
            thread::sleep(Duration::from_millis(80));
            println!("任务 3 完成");
            3
        }),
        Box::new(|| {
            thread::sleep(Duration::from_millis(120));
            println!("任务 4 完成");
            4
        }),
    ];

    let task_queue = Arc::new(Mutex::new(tasks));
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    // 创建工作线程
    for worker_id in 0..2 {
        let queue = Arc::clone(&task_queue);
        let results = Arc::clone(&results);

        let handle = thread::spawn(move || loop {
            let task = {
                let mut queue = queue.lock().unwrap();
                queue.pop()
            };

            match task {
                Some(task) => {
                    println!("工作线程 {} 执行任务", worker_id);
                    let result = task();
                    results.lock().unwrap().push(result);
                }
                None => break,
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_results = results.lock().unwrap();
    println!("所有任务完成，结果: {:?}", *final_results);
}

/// 演示闭包在异步上下文中的使用（模拟）
pub fn demonstrate_async_closures() {
    println!("\n10.6 异步闭包模式（模拟）:");

    // 模拟异步操作的闭包
    let async_operation = |delay_ms: u64, value: i32| {
        thread::sleep(Duration::from_millis(delay_ms));
        value * 2
    };

    // 并行执行多个"异步"操作
    let operations = vec![(100, 1), (150, 2), (80, 3), (120, 4)];

    let handles: Vec<_> = operations
        .into_iter()
        .map(|(delay, value)| {
            thread::spawn(move || {
                println!("开始异步操作，值: {}", value);
                let result = async_operation(delay, value);
                println!("异步操作完成，结果: {}", result);
                result
            })
        })
        .collect();

    let results: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    println!("所有异步操作完成: {:?}", results);
}

/// 演示闭包的并发安全性
pub fn demonstrate_closure_safety() {
    println!("\n10.7 闭包的并发安全性:");

    // 安全的闭包：只使用不可变数据
    let safe_data = vec![1, 2, 3, 4, 5];
    let safe_closure = move || safe_data.iter().sum::<i32>();

    // 可以安全地在多个线程中使用
    let handles: Vec<_> = (0..3)
        .map(|i| {
            let closure = safe_closure.clone(); // 注意：这里需要 Clone
            thread::spawn(move || {
                println!("线程 {} 计算结果: {}", i, closure());
            })
        })
        .collect();

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_thread_closure() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let increment = || counter_clone.fetch_add(1, Ordering::SeqCst);
            increment()
        });

        let old_value = handle.join().unwrap();
        assert_eq!(old_value, 0);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_send_closure() {
        let data = vec![1, 2, 3];
        let closure = move || data.len();

        // 验证闭包实现了 Send
        fn assert_send<T: Send>(_: T) {}
        assert_send(closure);
    }

    #[test]
    fn test_message_passing_closure() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let send_message = |msg: &str| {
                tx.send(msg.to_string()).unwrap();
            };

            send_message("测试消息");
        });

        let received = rx.recv().unwrap();
        assert_eq!(received, "测试消息");
    }
}
