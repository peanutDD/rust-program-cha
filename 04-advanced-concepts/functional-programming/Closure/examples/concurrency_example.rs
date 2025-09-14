//! # 并发编程中的闭包示例
//!
//! 这个示例展示了闭包在多线程和异步编程中的应用。

use closure_tutorial::concurrency;
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("🚀 并发编程中的闭包示例");
    println!("{}", "=".repeat(50));

    // 1. 线程中的闭包
    demonstrate_thread_closures();

    // 2. 共享状态
    demonstrate_shared_state();

    // 3. 消息传递
    demonstrate_message_passing();

    // 4. 线程池模式
    demonstrate_thread_pool();

    // 5. 并行计算
    demonstrate_parallel_computation();

    // 6. 运行库中的并发演示
    println!("\n📚 库演示");
    concurrency::demonstrate();

    println!("\n✅ 并发示例完成！");
}

/// 演示线程中的闭包使用
fn demonstrate_thread_closures() {
    println!("\n🧵 1. 线程中的闭包");

    // 基本的线程闭包
    let data = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        println!("线程中处理数据: {:?}", data);
        data.iter().sum::<i32>()
    });

    let result = handle.join().unwrap();
    println!("线程计算结果: {}", result);

    // 多个线程处理不同任务
    let handles: Vec<_> = (0..3)
        .map(|i| {
            thread::spawn(move || {
                let task_id = i;
                println!("任务 {} 开始执行", task_id);
                thread::sleep(Duration::from_millis(100 * (i + 1) as u64));
                println!("任务 {} 完成", task_id);
                task_id * task_id
            })
        })
        .collect();

    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    println!("所有任务结果: {:?}", results);
}

/// 演示共享状态
fn demonstrate_shared_state() {
    println!("\n🔄 2. 共享状态");

    // 使用 Arc<Mutex<T>> 共享状态
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let thread_id = i;
            for _ in 0..10 {
                let mut num = counter.lock().unwrap();
                *num += 1;
                println!("线程 {} 增加计数器到: {}", thread_id, *num);
                thread::sleep(Duration::from_millis(10));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终计数器值: {}", *counter.lock().unwrap());

    // 共享数据结构
    let shared_map = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    for i in 0..3 {
        let map = Arc::clone(&shared_map);
        let handle = thread::spawn(move || {
            let key = format!("key_{}", i);
            let value = i * 10;

            {
                let mut map = map.lock().unwrap();
                map.insert(key.clone(), value);
                println!("线程 {} 插入: {} -> {}", i, key, value);
            }

            thread::sleep(Duration::from_millis(50));

            {
                let map = map.lock().unwrap();
                if let Some(val) = map.get(&key) {
                    println!("线程 {} 读取: {} -> {}", i, key, val);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终 map 内容: {:?}", *shared_map.lock().unwrap());
}

/// 演示消息传递
fn demonstrate_message_passing() {
    println!("\n📨 3. 消息传递");

    // 基本的消息传递
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec!["Hello", "from", "the", "thread"];

        for (i, msg) in messages.into_iter().enumerate() {
            tx.send(format!("{}: {}", i, msg)).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for received in rx {
        println!("收到消息: {}", received);
    }

    // 多生产者单消费者
    let (tx, rx) = mpsc::channel();

    for i in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            let producer_id = i;
            for j in 0..3 {
                let message = format!("生产者 {} 的消息 {}", producer_id, j);
                tx.send(message).unwrap();
                thread::sleep(Duration::from_millis(50));
            }
        });
    }

    // 关闭原始发送者
    drop(tx);

    // 接收所有消息
    let mut messages = vec![];
    for received in rx {
        messages.push(received);
    }

    println!("收到的所有消息:");
    for msg in messages {
        println!("  - {}", msg);
    }
}

/// 演示简单的线程池模式
fn demonstrate_thread_pool() {
    println!("\n🏊 4. 线程池模式");

    // 简单的任务类型
    type Task = Box<dyn FnOnce() + Send + 'static>;

    // 简单的线程池实现
    struct SimpleThreadPool {
        workers: Vec<thread::JoinHandle<()>>,
        sender: mpsc::Sender<Task>,
    }

    impl SimpleThreadPool {
        fn new(size: usize) -> Self {
            let (sender, receiver) = mpsc::channel::<Task>();
            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                let receiver = Arc::clone(&receiver);
                let worker = thread::spawn(move || loop {
                    let task = {
                        let receiver = receiver.lock().unwrap();
                        receiver.recv()
                    };

                    match task {
                        Ok(task) => {
                            println!("工作线程 {} 执行任务", id);
                            task();
                        }
                        Err(_) => {
                            println!("工作线程 {} 关闭", id);
                            break;
                        }
                    }
                });
                workers.push(worker);
            }

            Self { workers, sender }
        }

        fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let task = Box::new(f);
            self.sender.send(task).unwrap();
        }
    }

    impl Drop for SimpleThreadPool {
        fn drop(&mut self) {
            drop(self.sender.clone());

            for worker in self.workers.drain(..) {
                worker.join().unwrap();
            }
        }
    }

    // 使用线程池
    let pool = SimpleThreadPool::new(3);

    for i in 0..6 {
        pool.execute(move || {
            println!("任务 {} 开始", i);
            thread::sleep(Duration::from_millis(200));
            println!("任务 {} 完成", i);
        });
    }

    // 等待一段时间让任务完成
    thread::sleep(Duration::from_millis(1000));
    println!("线程池演示完成");
}

/// 演示并行计算
fn demonstrate_parallel_computation() {
    println!("\n⚡ 5. 并行计算");

    // 并行计算数组元素的平方和
    let data: Vec<i32> = (1..=1000).collect();
    let chunk_size = data.len() / 4;

    let chunks: Vec<_> = data.chunks(chunk_size).collect();
    let mut handles = vec![];

    for (i, chunk) in chunks.into_iter().enumerate() {
        let chunk = chunk.to_vec();
        let handle = thread::spawn(move || {
            let sum: i64 = chunk.iter().map(|&x| (x as i64) * (x as i64)).sum();
            println!("块 {} 的平方和: {}", i, sum);
            sum
        });
        handles.push(handle);
    }

    let total_sum: i64 = handles.into_iter().map(|h| h.join().unwrap()).sum();

    println!("总平方和: {}", total_sum);

    // 并行搜索
    let search_data: Vec<i32> = (1..=10000).collect();
    let target = 7777;
    let chunk_size = search_data.len() / 4;

    let (tx, rx) = mpsc::channel();

    for (i, chunk) in search_data.chunks(chunk_size).enumerate() {
        let chunk = chunk.to_vec();
        let tx = tx.clone();
        thread::spawn(move || {
            for (j, &value) in chunk.iter().enumerate() {
                if value == target {
                    let global_index = i * chunk_size + j;
                    tx.send(Some(global_index)).unwrap();
                    return;
                }
            }
            tx.send(None).unwrap();
        });
    }

    drop(tx); // 关闭发送端

    let mut found_index = None;
    for result in rx {
        if let Some(index) = result {
            found_index = Some(index);
            break;
        }
    }

    match found_index {
        Some(index) => println!("在索引 {} 找到目标值 {}", index, target),
        None => println!("未找到目标值 {}", target),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_closure() {
        let data = vec![1, 2, 3, 4, 5];
        let handle = thread::spawn(move || data.iter().sum::<i32>());

        let result = handle.join().unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_shared_counter() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..5 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..10 {
                    let mut num = counter.lock().unwrap();
                    *num += 1;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 50);
    }

    #[test]
    fn test_message_passing() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            tx.send("Hello").unwrap();
            tx.send("World").unwrap();
        });

        let messages: Vec<_> = rx.iter().collect();
        assert_eq!(messages, vec!["Hello", "World"]);
    }
}
