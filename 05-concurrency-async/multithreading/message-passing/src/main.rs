//! Rust 线程同步：消息传递 (Message Passing) 全面教程
//! 
//! 本教程基于 https://course.rs/advance/concurrency-with-threads/message-passing.html
//! 深入分析 Rust 中的消息传递机制，涵盖所有相关知识点

use std::sync::mpsc::{self, Receiver, Sender, SyncSender};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn main() {
    println!("=== Rust 线程同步：消息传递 (Message Passing) 全面教程 ===");
    println!();
    
    // 1. MPSC 通道基础
    println!("1. MPSC 通道基础演示");
    basic_channel_demo();
    println!();
    
    // 2. 多发送者模式
    println!("2. 多发送者模式演示");
    multiple_producers_demo();
    println!();
    
    // 3. 同步与异步通道对比
    println!("3. 同步与异步通道对比");
    sync_vs_async_channels();
    println!();
    
    // 4. 错误处理机制
    println!("4. 错误处理机制演示");
    error_handling_demo();
    println!();
    
    // 5. 通道模式应用
    println!("5. 通道模式应用演示");
    channel_patterns_demo();
    println!();
    
    // 6. 性能分析
    println!("6. 性能分析");
    performance_analysis();
    println!();
    
    // 7. 高级特性
    println!("7. 高级特性演示");
    advanced_features_demo();
    println!();
    
    // 8. 实际应用案例
    println!("8. 实际应用案例");
    real_world_examples();
    println!();
}

/// 1. MPSC 通道基础演示
/// MPSC = Multiple Producer, Single Consumer
/// 支持多个发送者，但只有一个接收者
fn basic_channel_demo() {
    println!("  1.1 基本通道创建和使用");
    
    // 创建异步通道
    let (tx, rx) = mpsc::channel::<i32>();
    
    // 在子线程中发送消息
    thread::spawn(move || {
        println!("    发送者：准备发送消息 42");
        tx.send(42).unwrap();
        println!("    发送者：消息已发送");
    });
    
    // 在主线程中接收消息
    println!("    接收者：等待接收消息...");
    let received = rx.recv().unwrap();
    println!("    接收者：收到消息 {}", received);
    
    println!("  1.2 类型推导和类型安全");
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        // 编译器会自动推导类型
        tx.send("Hello, World!").unwrap();
        // 下面这行会编译错误，因为类型不匹配
        // tx.send(123).unwrap(); // Error: expected `&str`, found integer
    });
    
    let message = rx.recv().unwrap();
    println!("    收到字符串消息: {}", message);
    
    println!("  1.3 通道的所有权转移");
    demonstrate_ownership_transfer();
}

/// 演示通道中的所有权转移
fn demonstrate_ownership_transfer() {
    #[derive(Debug)]
    struct Data {
        id: u32,
        content: String,
    }
    
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let data = Data {
            id: 1,
            content: "重要数据".to_string(),
        };
        
        println!("    发送前 data: {:?}", data);
        tx.send(data).unwrap(); // data 的所有权被转移
        // println!("    发送后 data: {:?}", data); // 编译错误：value borrowed here after move
    });
    
    let received_data = rx.recv().unwrap();
    println!("    接收到的数据: {:?}", received_data);
}

/// 2. 多发送者模式演示
fn multiple_producers_demo() {
    println!("  2.1 多个发送者向单个接收者发送消息");
    
    let (tx, rx) = mpsc::channel();
    let num_senders = 5;
    
    // 创建多个发送者线程
    for i in 0..num_senders {
        let tx_clone = tx.clone(); // 克隆发送者
        thread::spawn(move || {
            let message = format!("来自线程 {} 的消息", i);
            println!("    线程 {} 发送: {}", i, message);
            tx_clone.send(message).unwrap();
            
            // 模拟一些工作
            thread::sleep(Duration::from_millis(100 * i as u64));
        });
    }
    
    // 必须丢弃原始发送者，否则接收者会一直等待
    drop(tx);
    
    // 接收所有消息
    println!("    开始接收消息:");
    for received in rx {
        println!("    收到: {}", received);
    }
    
    println!("  2.2 工作分发模式");
    work_distribution_pattern();
}

/// 工作分发模式：主线程分发任务，多个工作线程处理
fn work_distribution_pattern() {
    let (job_tx, job_rx) = mpsc::channel();
    let (result_tx, result_rx) = mpsc::channel();
    
    let job_rx = Arc::new(Mutex::new(job_rx));
    
    // 创建工作线程池
    let num_workers = 3;
    for worker_id in 0..num_workers {
        let job_rx = Arc::clone(&job_rx);
        let result_tx = result_tx.clone();
        
        thread::spawn(move || {
            loop {
                let job = {
                    let rx = job_rx.lock().unwrap();
                    rx.recv()
                };
                
                match job {
                    Ok(n) => {
                        println!("    工作线程 {} 处理任务: {}", worker_id, n);
                        let result = n * n; // 简单的计算任务
                        result_tx.send((worker_id, n, result)).unwrap();
                    }
                    Err(_) => {
                        println!("    工作线程 {} 退出", worker_id);
                        break;
                    }
                }
            }
        });
    }
    
    // 分发任务
    for i in 1..=10 {
        job_tx.send(i).unwrap();
    }
    drop(job_tx); // 关闭任务通道
    drop(result_tx); // 关闭结果通道
    
    // 收集结果
    println!("    收集处理结果:");
    for (worker_id, input, result) in result_rx {
        println!("    工作线程 {} 完成: {} -> {}", worker_id, input, result);
    }
}

/// 3. 同步与异步通道对比
fn sync_vs_async_channels() {
    println!("  3.1 异步通道 (无界)");
    async_channel_demo();
    
    println!("  3.2 同步通道 (有界)");
    sync_channel_demo();
    
    println!("  3.3 缓冲区大小对性能的影响");
    buffer_size_impact();
}

/// 异步通道演示
fn async_channel_demo() {
    let (tx, rx) = mpsc::channel();
    
    let start = Instant::now();
    thread::spawn(move || {
        for i in 0..5 {
            println!("    异步发送: {}", i);
            tx.send(i).unwrap(); // 不会阻塞
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    thread::sleep(Duration::from_millis(300)); // 让发送者先发送几个消息
    
    for received in rx {
        println!("    异步接收: {}", received);
    }
    
    println!("    异步通道总耗时: {:?}", start.elapsed());
}

/// 同步通道演示
fn sync_channel_demo() {
    println!("    3.2.1 零缓冲区同步通道");
    let (tx, rx) = mpsc::sync_channel(0); // 零缓冲区
    
    let start = Instant::now();
    thread::spawn(move || {
        for i in 0..3 {
            println!("    同步发送 (缓冲区=0): {} - 开始", i);
            tx.send(i).unwrap(); // 会阻塞直到接收者接收
            println!("    同步发送 (缓冲区=0): {} - 完成", i);
        }
    });
    
    thread::sleep(Duration::from_millis(200)); // 模拟接收者延迟
    
    for received in rx {
        println!("    同步接收 (缓冲区=0): {}", received);
        thread::sleep(Duration::from_millis(300)); // 模拟处理时间
    }
    
    println!("    零缓冲区同步通道总耗时: {:?}", start.elapsed());
    
    println!("    3.2.2 有缓冲区同步通道");
    let (tx, rx) = mpsc::sync_channel(2); // 缓冲区大小为2
    
    let start = Instant::now();
    thread::spawn(move || {
        for i in 0..5 {
            println!("    同步发送 (缓冲区=2): {} - 开始", i);
            tx.send(i).unwrap();
            println!("    同步发送 (缓冲区=2): {} - 完成", i);
        }
    });
    
    thread::sleep(Duration::from_millis(200));
    
    for received in rx {
        println!("    同步接收 (缓冲区=2): {}", received);
        thread::sleep(Duration::from_millis(100));
    }
    
    println!("    有缓冲区同步通道总耗时: {:?}", start.elapsed());
}

/// 缓冲区大小对性能的影响
fn buffer_size_impact() {
    let buffer_sizes = vec![0, 1, 5, 10, 100];
    let message_count = 1000;
    
    for buffer_size in buffer_sizes {
        let start = Instant::now();
        let (tx, rx) = mpsc::sync_channel(buffer_size);
        
        thread::spawn(move || {
            for i in 0..message_count {
                tx.send(i).unwrap();
            }
        });
        
        for _ in 0..message_count {
            rx.recv().unwrap();
        }
        
        let duration = start.elapsed();
        println!("    缓冲区大小 {}: 发送 {} 条消息耗时 {:?}", 
                buffer_size, message_count, duration);
    }
}

/// 4. 错误处理机制演示
fn error_handling_demo() {
    println!("  4.1 发送者关闭后的接收错误");
    sender_closed_error();
    
    println!("  4.2 接收者关闭后的发送错误");
    receiver_closed_error();
    
    println!("  4.3 超时接收");
    timeout_receive_demo();
    
    println!("  4.4 非阻塞操作");
    non_blocking_operations();
}

/// 发送者关闭后的接收错误
fn sender_closed_error() {
    let (tx, rx) = mpsc::channel::<i32>();
    
    // 立即关闭发送者
    drop(tx);
    
    // 尝试接收消息
    match rx.recv() {
        Ok(msg) => println!("    收到消息: {}", msg),
        Err(e) => println!("    接收错误: {} (发送者已关闭)", e),
    }
}

/// 接收者关闭后的发送错误
fn receiver_closed_error() {
    let (tx, rx) = mpsc::channel();
    
    // 关闭接收者
    drop(rx);
    
    // 尝试发送消息
    match tx.send(42) {
        Ok(_) => println!("    消息发送成功"),
        Err(e) => println!("    发送错误: {} (接收者已关闭)", e),
    }
}

/// 超时接收演示
fn timeout_receive_demo() {
    let (tx, rx) = mpsc::channel();
    
    // 在另一个线程中延迟发送
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        tx.send("延迟消息").unwrap();
    });
    
    // 模拟超时接收（使用 recv_timeout 的替代方案）
    let start = Instant::now();
    let timeout = Duration::from_millis(200);
    
    loop {
        match rx.try_recv() {
            Ok(msg) => {
                println!("    在 {:?} 后收到消息: {}", start.elapsed(), msg);
                break;
            }
            Err(mpsc::TryRecvError::Empty) => {
                if start.elapsed() > timeout {
                    println!("    接收超时 ({:?})", timeout);
                    break;
                }
                thread::sleep(Duration::from_millis(10));
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("    通道已断开");
                break;
            }
        }
    }
    
    // 继续等待真正的消息
    if let Ok(msg) = rx.recv() {
        println!("    最终收到消息: {}", msg);
    }
}

/// 非阻塞操作演示
fn non_blocking_operations() {
    let (tx, rx) = mpsc::sync_channel(1);
    
    // 非阻塞发送
    println!("    尝试非阻塞发送...");
    match tx.try_send(1) {
        Ok(_) => println!("    第一次发送成功"),
        Err(e) => println!("    第一次发送失败: {}", e),
    }
    
    match tx.try_send(2) {
        Ok(_) => println!("    第二次发送成功"),
        Err(e) => println!("    第二次发送失败: {} (缓冲区已满)", e),
    }
    
    // 非阻塞接收
    println!("    尝试非阻塞接收...");
    match rx.try_recv() {
        Ok(msg) => println!("    接收到: {}", msg),
        Err(e) => println!("    接收失败: {}", e),
    }
    
    // 现在可以再次发送
    match tx.try_send(3) {
        Ok(_) => println!("    第三次发送成功"),
        Err(e) => println!("    第三次发送失败: {}", e),
    }
}

/// 5. 通道模式应用演示
fn channel_patterns_demo() {
    println!("  5.1 生产者-消费者模式");
    producer_consumer_pattern();
    
    println!("  5.2 扇出模式 (Fan-out)");
    fan_out_pattern();
    
    println!("  5.3 扇入模式 (Fan-in)");
    fan_in_pattern();
    
    println!("  5.4 管道模式 (Pipeline)");
    pipeline_pattern();
}

/// 生产者-消费者模式
fn producer_consumer_pattern() {
    let (tx, rx) = mpsc::sync_channel(5); // 有界队列
    
    // 生产者
    let producer = thread::spawn(move || {
        for i in 0..10 {
            let item = format!("产品-{}", i);
            println!("    生产: {}", item);
            tx.send(item).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
        println!("    生产者完成");
    });
    
    // 消费者
    let consumer = thread::spawn(move || {
        for item in rx {
            println!("    消费: {}", item);
            thread::sleep(Duration::from_millis(150)); // 消费比生产慢
        }
        println!("    消费者完成");
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
}

/// 扇出模式：一个生产者，多个消费者
fn fan_out_pattern() {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    
    // 生产者
    thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
            println!("    生产任务: {}", i);
        }
    });
    
    // 多个消费者
    let mut handles = vec![];
    for consumer_id in 0..3 {
        let rx = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            loop {
                let task = {
                    let rx = rx.lock().unwrap();
                    rx.recv()
                };
                
                match task {
                    Ok(task) => {
                        println!("    消费者 {} 处理任务: {}", consumer_id, task);
                        thread::sleep(Duration::from_millis(200));
                    }
                    Err(_) => {
                        println!("    消费者 {} 退出", consumer_id);
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

/// 扇入模式：多个生产者，一个消费者
fn fan_in_pattern() {
    let (tx, rx) = mpsc::channel();
    
    // 多个生产者
    let mut handles = vec![];
    for producer_id in 0..3 {
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            for i in 0..3 {
                let data = format!("P{}-D{}", producer_id, i);
                tx.send(data.clone()).unwrap();
                println!("    生产者 {} 发送: {}", producer_id, data);
                thread::sleep(Duration::from_millis(100));
            }
        });
        handles.push(handle);
    }
    
    drop(tx); // 关闭原始发送者
    
    // 单个消费者
    let consumer = thread::spawn(move || {
        for data in rx {
            println!("    消费者收到: {}", data);
        }
    });
    
    for handle in handles {
        handle.join().unwrap();
    }
    consumer.join().unwrap();
}

/// 管道模式：数据流经多个处理阶段
fn pipeline_pattern() {
    let (input_tx, input_rx) = mpsc::channel();
    let (stage1_tx, stage1_rx) = mpsc::channel();
    let (stage2_tx, stage2_rx) = mpsc::channel();
    
    // 阶段1：数据预处理
    thread::spawn(move || {
        for data in input_rx {
            let processed = format!("[预处理]{}", data);
            println!("    阶段1处理: {} -> {}", data, processed);
            stage1_tx.send(processed).unwrap();
        }
    });
    
    // 阶段2：数据转换
    thread::spawn(move || {
        for data in stage1_rx {
            let transformed = format!("[转换]{}", data);
            println!("    阶段2处理: {} -> {}", data, transformed);
            stage2_tx.send(transformed).unwrap();
        }
    });
    
    // 阶段3：最终处理
    let final_processor = thread::spawn(move || {
        for data in stage2_rx {
            let final_result = format!("[完成]{}", data);
            println!("    最终结果: {}", final_result);
        }
    });
    
    // 输入数据
    for i in 0..5 {
        let data = format!("数据{}", i);
        input_tx.send(data).unwrap();
    }
    drop(input_tx);
    
    final_processor.join().unwrap();
}

/// 6. 性能分析
fn performance_analysis() {
    println!("  6.1 通道创建开销");
    channel_creation_overhead();
    
    println!("  6.2 消息传递吞吐量");
    message_throughput_test();
    
    println!("  6.3 内存使用分析");
    memory_usage_analysis();
    
    println!("  6.4 与其他同步机制对比");
    sync_mechanism_comparison();
}

/// 通道创建开销测试
fn channel_creation_overhead() {
    let iterations = 10000;
    
    // 测试异步通道创建
    let start = Instant::now();
    for _ in 0..iterations {
        let (_tx, _rx) = mpsc::channel::<i32>();
    }
    let async_duration = start.elapsed();
    
    // 测试同步通道创建
    let start = Instant::now();
    for _ in 0..iterations {
        let (_tx, _rx) = mpsc::sync_channel::<i32>(1);
    }
    let sync_duration = start.elapsed();
    
    println!("    创建 {} 个异步通道耗时: {:?}", iterations, async_duration);
    println!("    创建 {} 个同步通道耗时: {:?}", iterations, sync_duration);
    println!("    平均每个异步通道: {:?}", async_duration / iterations as u32);
    println!("    平均每个同步通道: {:?}", sync_duration / iterations as u32);
}

/// 消息传递吞吐量测试
fn message_throughput_test() {
    let message_counts = vec![1000, 10000, 100000];
    
    for count in message_counts {
        // 异步通道吞吐量
        let start = Instant::now();
        let (tx, rx) = mpsc::channel();
        
        thread::spawn(move || {
            for i in 0..count {
                tx.send(i).unwrap();
            }
        });
        
        for _ in 0..count {
            rx.recv().unwrap();
        }
        
        let async_duration = start.elapsed();
        let async_throughput = count as f64 / async_duration.as_secs_f64();
        
        // 同步通道吞吐量
        let start = Instant::now();
        let (tx, rx) = mpsc::sync_channel(100);
        
        thread::spawn(move || {
            for i in 0..count {
                tx.send(i).unwrap();
            }
        });
        
        for _ in 0..count {
            rx.recv().unwrap();
        }
        
        let sync_duration = start.elapsed();
        let sync_throughput = count as f64 / sync_duration.as_secs_f64();
        
        println!("    {} 条消息:", count);
        println!("      异步通道: {:?} ({:.0} msg/s)", async_duration, async_throughput);
        println!("      同步通道: {:?} ({:.0} msg/s)", sync_duration, sync_throughput);
    }
}

/// 内存使用分析
fn memory_usage_analysis() {
    println!("    通道内存使用特点:");
    println!("      - 异步通道: 无界，内存使用随消息积压增长");
    println!("      - 同步通道: 有界，内存使用固定");
    
    // 演示异步通道的内存积压
    let (tx, rx) = mpsc::channel();
    
    // 快速发送大量消息
    for i in 0..1000 {
        tx.send(format!("消息{}", i)).unwrap();
    }
    
    println!("    异步通道已发送 1000 条消息，内存中积压");
    
    // 慢速消费
    let mut count = 0;
    for msg in rx {
        count += 1;
        if count <= 5 {
            println!("    消费: {}", msg);
        } else if count == 6 {
            println!("    ... (继续消费剩余消息)");
        }
        
        if count >= 1000 {
            break;
        }
    }
    
    println!("    所有消息已消费完毕");
}

/// 与其他同步机制对比
fn sync_mechanism_comparison() {
    use std::sync::{Arc, Mutex, Condvar};
    use std::collections::VecDeque;
    
    let iterations = 10000;
    
    // 1. 通道方式
    let start = Instant::now();
    let (tx, rx) = mpsc::channel();
    
    let sender = thread::spawn(move || {
        for i in 0..iterations {
            tx.send(i).unwrap();
        }
    });
    
    let receiver = thread::spawn(move || {
        for _ in 0..iterations {
            rx.recv().unwrap();
        }
    });
    
    sender.join().unwrap();
    receiver.join().unwrap();
    let channel_duration = start.elapsed();
    
    // 2. Mutex + Condvar 方式
    let start = Instant::now();
    let data = Arc::new(Mutex::new(VecDeque::new()));
    let condvar = Arc::new(Condvar::new());
    
    let data_clone = Arc::clone(&data);
    let condvar_clone = Arc::clone(&condvar);
    let sender = thread::spawn(move || {
        for i in 0..iterations {
            {
                let mut queue = data_clone.lock().unwrap();
                queue.push_back(i);
            }
            condvar_clone.notify_one();
        }
    });
    
    let receiver = thread::spawn(move || {
        for _ in 0..iterations {
            let mut queue = data.lock().unwrap();
            while queue.is_empty() {
                queue = condvar.wait(queue).unwrap();
            }
            queue.pop_front();
        }
    });
    
    sender.join().unwrap();
    receiver.join().unwrap();
    let mutex_duration = start.elapsed();
    
    println!("    {} 次操作性能对比:", iterations);
    println!("      通道方式: {:?}", channel_duration);
    println!("      Mutex+Condvar: {:?}", mutex_duration);
    println!("      通道相对性能: {:.2}x", 
             mutex_duration.as_nanos() as f64 / channel_duration.as_nanos() as f64);
}

/// 7. 高级特性演示
fn advanced_features_demo() {
    println!("  7.1 迭代器接口");
    iterator_interface_demo();
    
    println!("  7.2 通道组合");
    channel_composition_demo();
    
    println!("  7.3 Select 模拟");
    select_simulation_demo();
    
    println!("  7.4 通道适配器");
    channel_adapter_demo();
}

/// 迭代器接口演示
fn iterator_interface_demo() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        for i in 0..5 {
            tx.send(i * i).unwrap();
        }
        // 发送者被丢弃，迭代器会自动结束
    });
    
    println!("    使用迭代器接收消息:");
    for (index, value) in rx.iter().enumerate() {
        println!("    第 {} 个消息: {}", index + 1, value);
    }
}

/// 通道组合演示
fn channel_composition_demo() {
    // 创建多个通道形成复杂的数据流
    let (input_tx, input_rx) = mpsc::channel::<i32>();
    let (filter_tx, filter_rx) = mpsc::channel::<i32>();
    let (transform_tx, transform_rx) = mpsc::channel::<i32>();
    
    // 过滤器：只传递偶数
    thread::spawn(move || {
        for num in input_rx {
            if num % 2 == 0 {
                println!("    过滤器通过: {}", num);
                filter_tx.send(num).unwrap();
            } else {
                println!("    过滤器拒绝: {}", num);
            }
        }
    });
    
    // 转换器：平方运算
    thread::spawn(move || {
        for num in filter_rx {
            let squared = num * num;
            println!("    转换器: {} -> {}", num, squared);
            transform_tx.send(squared).unwrap();
        }
    });
    
    // 最终收集器
    let collector = thread::spawn(move || {
        let mut results = Vec::new();
        for result in transform_rx {
            println!("    收集器收到: {}", result);
            results.push(result);
        }
        results
    });
    
    // 输入数据
    for i in 1..=10 {
        input_tx.send(i).unwrap();
    }
    drop(input_tx);
    
    let final_results = collector.join().unwrap();
    println!("    最终结果: {:?}", final_results);
}

/// Select 模拟演示
fn select_simulation_demo() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    
    // 发送者1：快速发送
    thread::spawn(move || {
        for i in 0..3 {
            thread::sleep(Duration::from_millis(100));
            tx1.send(format!("快速-{}", i)).unwrap();
        }
    });
    
    // 发送者2：慢速发送
    thread::spawn(move || {
        for i in 0..3 {
            thread::sleep(Duration::from_millis(300));
            tx2.send(format!("慢速-{}", i)).unwrap();
        }
    });
    
    // 模拟 select 行为
    println!("    模拟 select 操作 (非阻塞轮询):");
    let mut rx1_closed = false;
    let mut rx2_closed = false;
    
    while !rx1_closed || !rx2_closed {
        // 尝试从 rx1 接收
        match rx1.try_recv() {
            Ok(msg) => println!("    从通道1收到: {}", msg),
            Err(mpsc::TryRecvError::Empty) => {}, // 继续
            Err(mpsc::TryRecvError::Disconnected) => {
                if !rx1_closed {
                    println!("    通道1已关闭");
                    rx1_closed = true;
                }
            }
        }
        
        // 尝试从 rx2 接收
        match rx2.try_recv() {
            Ok(msg) => println!("    从通道2收到: {}", msg),
            Err(mpsc::TryRecvError::Empty) => {}, // 继续
            Err(mpsc::TryRecvError::Disconnected) => {
                if !rx2_closed {
                    println!("    通道2已关闭");
                    rx2_closed = true;
                }
            }
        }
        
        thread::sleep(Duration::from_millis(50)); // 避免忙等待
    }
}

/// 通道适配器演示
fn channel_adapter_demo() {
    println!("    实现通道适配器模式:");
    
    // 创建一个将字符串转换为大写的适配器
    fn uppercase_adapter(input: Receiver<String>) -> Receiver<String> {
        let (tx, rx) = mpsc::channel();
        
        thread::spawn(move || {
            for msg in input {
                let uppercase_msg = msg.to_uppercase();
                if tx.send(uppercase_msg).is_err() {
                    break; // 接收者已关闭
                }
            }
        });
        
        rx
    }
    
    // 使用适配器
    let (input_tx, input_rx) = mpsc::channel();
    let output_rx = uppercase_adapter(input_rx);
    
    // 发送数据
    thread::spawn(move || {
        let messages = vec!["hello", "world", "rust", "channel"];
        for msg in messages {
            input_tx.send(msg.to_string()).unwrap();
        }
    });
    
    // 接收转换后的数据
    for msg in output_rx {
        println!("    适配器输出: {}", msg);
    }
}

/// 8. 实际应用案例
fn real_world_examples() {
    println!("  8.1 任务队列系统");
    task_queue_system();
    
    println!("  8.2 事件系统");
    event_system_demo();
    
    println!("  8.3 数据流处理");
    data_stream_processing();
    
    println!("  8.4 分布式计算模拟");
    distributed_computing_simulation();
}

/// 任务队列系统
fn task_queue_system() {
    #[derive(Debug, Clone)]
    enum Task {
        Compute(u32),
        IO(String),
        Network(String),
    }
    
    #[derive(Debug)]
    enum TaskResult {
        ComputeResult(u32, u64),
        IOResult(String, bool),
        NetworkResult(String, bool),
    }
    
    let (task_tx, task_rx) = mpsc::channel::<Task>();
    let (result_tx, result_rx) = mpsc::channel();
    
    let task_rx = Arc::new(Mutex::new(task_rx));
    
    // 创建工作线程池
    let mut workers = vec![];
    for worker_id in 0..3 {
        let task_rx = Arc::clone(&task_rx);
        let result_tx = result_tx.clone();
        
        let worker = thread::spawn(move || {
            loop {
                let task = {
                    let rx = task_rx.lock().unwrap();
                    rx.recv()
                };
                
                match task {
                    Ok(task) => {
                        println!("    工作线程 {} 处理任务: {:?}", worker_id, task);
                        
                        let result = match task {
                            Task::Compute(n) => {
                                thread::sleep(Duration::from_millis(100));
                                TaskResult::ComputeResult(n, (n as u64).pow(2))
                            }
                            Task::IO(file) => {
                                thread::sleep(Duration::from_millis(200));
                                TaskResult::IOResult(file, true)
                            }
                            Task::Network(url) => {
                                thread::sleep(Duration::from_millis(150));
                                TaskResult::NetworkResult(url, true)
                            }
                        };
                        
                        result_tx.send(result).unwrap();
                    }
                    Err(_) => {
                        println!("    工作线程 {} 退出", worker_id);
                        break;
                    }
                }
            }
        });
        
        workers.push(worker);
    }
    
    // 任务调度器
    let task_tx_clone = task_tx.clone();
    let scheduler = thread::spawn(move || {
        let tasks = vec![
            Task::Compute(5),
            Task::IO("file1.txt".to_string()),
            Task::Network("http://example.com".to_string()),
            Task::Compute(10),
            Task::IO("file2.txt".to_string()),
        ];
        
        for task in tasks {
            task_tx_clone.send(task).unwrap();
        }
    });
    
    // 结果收集器
    let collector = thread::spawn(move || {
        let mut results = vec![];
        for _ in 0..5 {
            let result = result_rx.recv().unwrap();
            println!("    收到结果: {:?}", result);
            results.push(result);
        }
        results
    });
    
    scheduler.join().unwrap();
    let _results = collector.join().unwrap();
    
    // 关闭工作线程
    drop(task_tx);
    for worker in workers {
        worker.join().unwrap();
    }
}

/// 事件系统演示
fn event_system_demo() {
    #[derive(Debug, Clone)]
    enum Event {
        UserLogin(String),
        UserLogout(String),
        DataUpdate(String, i32),
        SystemShutdown,
    }
    
    let (event_tx, event_rx) = mpsc::channel::<Event>();
    
    // 日志监听器
    let (log_tx, log_rx) = mpsc::channel::<Event>();
    thread::spawn(move || {
        for event in log_rx {
            match event {
                Event::UserLogin(user) => println!("    [日志] 用户登录: {}", user),
                Event::UserLogout(user) => println!("    [日志] 用户登出: {}", user),
                Event::DataUpdate(key, value) => println!("    [日志] 数据更新: {} = {}", key, value),
                Event::SystemShutdown => {
                    println!("    [日志] 系统关闭");
                    break;
                }
            }
        }
    });
    
    // 统计监听器
    let (stats_tx, stats_rx) = mpsc::channel::<Event>();
    thread::spawn(move || {
        let mut login_count = 0;
        let mut logout_count = 0;
        let mut update_count = 0;
        
        for event in stats_rx {
            match event {
                Event::UserLogin(_) => login_count += 1,
                Event::UserLogout(_) => logout_count += 1,
                Event::DataUpdate(_, _) => update_count += 1,
                Event::SystemShutdown => {
                    println!("    [统计] 登录: {}, 登出: {}, 更新: {}", 
                            login_count, logout_count, update_count);
                    break;
                }
            }
        }
    });
    
    // 事件分发器
    thread::spawn(move || {
        for event in event_rx {
            // 广播事件到所有监听器
            let _ = log_tx.send(event.clone());
            let _ = stats_tx.send(event.clone());
            
            if matches!(event, Event::SystemShutdown) {
                break;
            }
        }
    });
    
    // 模拟事件发生
    let events = vec![
        Event::UserLogin("Alice".to_string()),
        Event::DataUpdate("config".to_string(), 42),
        Event::UserLogin("Bob".to_string()),
        Event::UserLogout("Alice".to_string()),
        Event::DataUpdate("counter".to_string(), 100),
        Event::SystemShutdown,
    ];
    
    for event in events {
        event_tx.send(event).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    
    thread::sleep(Duration::from_millis(500)); // 等待处理完成
}

/// 数据流处理
fn data_stream_processing() {
    #[derive(Debug, Clone)]
    struct DataPoint {
        timestamp: u64,
        value: f64,
        sensor_id: String,
    }
    
    let (raw_tx, raw_rx) = mpsc::channel::<DataPoint>();
    let (filtered_tx, filtered_rx) = mpsc::channel::<DataPoint>();
    let (aggregated_tx, aggregated_rx) = mpsc::channel::<String>();
    
    // 数据过滤器：过滤异常值
    thread::spawn(move || {
        for data in raw_rx {
            if data.value >= 0.0 && data.value <= 100.0 {
                println!("    过滤器通过: {:?}", data);
                filtered_tx.send(data).unwrap();
            } else {
                println!("    过滤器拒绝异常值: {:?}", data);
            }
        }
    });
    
    // 数据聚合器：按传感器ID聚合
    thread::spawn(move || {
        let mut sensor_data: HashMap<String, Vec<f64>> = HashMap::new();
        
        for data in filtered_rx {
            sensor_data.entry(data.sensor_id.clone())
                      .or_insert_with(Vec::new)
                      .push(data.value);
            
            // 每收集到5个数据点就进行聚合
            if let Some(values) = sensor_data.get(&data.sensor_id) {
                if values.len() >= 3 {
                    let avg = values.iter().sum::<f64>() / values.len() as f64;
                    let result = format!("传感器 {} 平均值: {:.2}", data.sensor_id, avg);
                    println!("    聚合器输出: {}", result);
                    aggregated_tx.send(result).unwrap();
                    sensor_data.get_mut(&data.sensor_id).unwrap().clear();
                }
            }
        }
    });
    
    // 结果处理器
    let processor = thread::spawn(move || {
        for result in aggregated_rx {
            println!("    最终处理: {}", result);
        }
    });
    
    // 模拟数据流
    let data_points = vec![
        DataPoint { timestamp: 1, value: 25.5, sensor_id: "temp1".to_string() },
        DataPoint { timestamp: 2, value: 26.0, sensor_id: "temp1".to_string() },
        DataPoint { timestamp: 3, value: 150.0, sensor_id: "temp1".to_string() }, // 异常值
        DataPoint { timestamp: 4, value: 24.8, sensor_id: "temp1".to_string() },
        DataPoint { timestamp: 5, value: 30.2, sensor_id: "temp2".to_string() },
        DataPoint { timestamp: 6, value: 31.0, sensor_id: "temp2".to_string() },
        DataPoint { timestamp: 7, value: 29.8, sensor_id: "temp2".to_string() },
    ];
    
    for data in data_points {
        raw_tx.send(data).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    
    drop(raw_tx);
    processor.join().unwrap();
}

/// 分布式计算模拟
fn distributed_computing_simulation() {
    #[derive(Debug)]
    struct ComputeTask {
        id: u32,
        data: Vec<u32>,
    }
    
    #[derive(Debug)]
    struct ComputeResult {
        task_id: u32,
        result: u64,
        worker_id: usize,
    }
    
    let (task_tx, task_rx) = mpsc::channel::<ComputeTask>();
    let (result_tx, result_rx) = mpsc::channel::<ComputeResult>();
    
    let task_rx = Arc::new(Mutex::new(task_rx));
    
    // 创建分布式工作节点
    let num_workers = 4;
    let mut workers = vec![];
    
    for worker_id in 0..num_workers {
        let task_rx = Arc::clone(&task_rx);
        let result_tx = result_tx.clone();
        
        let worker = thread::spawn(move || {
            println!("    工作节点 {} 启动", worker_id);
            
            loop {
                let task = {
                    let rx = task_rx.lock().unwrap();
                    rx.recv()
                };
                
                match task {
                    Ok(task) => {
                        println!("    节点 {} 处理任务 {}", worker_id, task.id);
                        
                        // 模拟计算密集型任务
                        let result: u64 = task.data.iter().map(|&x| x as u64 * x as u64).sum();
                        thread::sleep(Duration::from_millis(200)); // 模拟计算时间
                        
                        let compute_result = ComputeResult {
                            task_id: task.id,
                            result,
                            worker_id,
                        };
                        
                        result_tx.send(compute_result).unwrap();
                    }
                    Err(_) => {
                        println!("    工作节点 {} 关闭", worker_id);
                        break;
                    }
                }
            }
        });
        
        workers.push(worker);
    }
    
    // 任务分发器
    let task_tx_clone = task_tx.clone();
    let distributor = thread::spawn(move || {
        for i in 0..8 {
            let task = ComputeTask {
                id: i,
                data: (1..=10).map(|x| x + i * 10).collect(),
            };
            
            println!("    分发任务: {:?}", task);
            task_tx_clone.send(task).unwrap();
        }
    });
    
    // 结果收集器
    let collector = thread::spawn(move || {
        let mut results = vec![];
        for _ in 0..8 {
            let result = result_rx.recv().unwrap();
            println!("    收集结果: {:?}", result);
            results.push(result);
        }
        
        // 按任务ID排序
        results.sort_by_key(|r| r.task_id);
        println!("    所有任务完成，最终结果:");
        for result in &results {
            println!("      任务 {}: {} (节点 {})", 
                    result.task_id, result.result, result.worker_id);
        }
        
        results
    });
    
    distributor.join().unwrap();
    let _final_results = collector.join().unwrap();
    
    // 关闭所有工作节点
    drop(task_tx);
    for worker in workers {
        worker.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_channel() {
        let (tx, rx) = mpsc::channel();
        
        thread::spawn(move || {
            tx.send(42).unwrap();
        });
        
        let received = rx.recv().unwrap();
        assert_eq!(received, 42);
    }
    
    #[test]
    fn test_multiple_senders() {
        let (tx, rx) = mpsc::channel();
        let num_senders = 3;
        
        for i in 0..num_senders {
            let tx_clone = tx.clone();
            thread::spawn(move || {
                tx_clone.send(i).unwrap();
            });
        }
        
        drop(tx); // 关闭原始发送者
        
        let mut received = vec![];
        for msg in rx {
            received.push(msg);
        }
        
        received.sort();
        assert_eq!(received, vec![0, 1, 2]);
    }
    
    #[test]
    fn test_sync_channel() {
        let (tx, rx) = mpsc::sync_channel(1);
        
        // 第一次发送应该成功
        assert!(tx.try_send(1).is_ok());
        
        // 第二次发送应该失败（缓冲区已满）
        assert!(tx.try_send(2).is_err());
        
        // 接收后应该能再次发送
        assert_eq!(rx.recv().unwrap(), 1);
        assert!(tx.try_send(3).is_ok());
    }
    
    #[test]
    fn test_channel_error_handling() {
        let (tx, rx) = mpsc::channel::<i32>();
        
        // 关闭发送者
        drop(tx);
        
        // 接收应该失败
        assert!(rx.recv().is_err());
    }
    
    #[test]
    fn test_non_blocking_operations() {
        let (tx, rx) = mpsc::channel();
        
        // 非阻塞接收空通道应该返回错误
        assert!(matches!(rx.try_recv(), Err(mpsc::TryRecvError::Empty)));
        
        // 发送消息
        tx.send(42).unwrap();
        
        // 非阻塞接收应该成功
        assert_eq!(rx.try_recv().unwrap(), 42);
        
        // 再次非阻塞接收应该返回空错误
        assert!(matches!(rx.try_recv(), Err(mpsc::TryRecvError::Empty)));
    }
}
