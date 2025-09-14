//! async/await 和 Stream 流处理 - 全面深度分析
//! 
//! 基于 https://course.rs/advance/async/async-await.html 的内容
//! 深入探讨 Rust 异步编程的核心概念和实际应用

use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
    time::{Duration, Instant},
    sync::{Arc, Mutex},
};

use tokio::{
    time::{sleep, timeout},
    sync::{mpsc, oneshot, Semaphore},
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

use futures::{
    stream::{Stream, StreamExt},
    future::{join_all, FutureExt},
    sink::SinkExt,
    join,
};

use serde::{Deserialize, Serialize};
use rand::Rng;
use chrono::{DateTime, Utc};

#[tokio::main]
async fn main() {
    println!("🚀 async/await 和 Stream 流处理 - 全面深度分析");
    println!("{}", "=".repeat(60));
    
    // 1. async/await 基础概念
    demonstrate_async_await_basics().await;
    
    // 2. Future trait 深入分析
    demonstrate_future_trait().await;
    
    // 3. 异步运行时机制
    demonstrate_async_runtime().await;
    
    // 4. Stream 流处理
    demonstrate_stream_processing().await;
    
    // 5. 异步编程模式
    demonstrate_async_patterns().await;
    
    // 6. 实际应用案例
    demonstrate_practical_examples().await;
    
    // 7. 性能优化技巧
    demonstrate_performance_optimization().await;
    
    // 8. 高级主题
    demonstrate_advanced_topics().await;
    
    println!("\n🎉 async/await 和 Stream 流处理分析完成！");
    println!("{}", "=".repeat(60));
}

/// 演示 async/await 基础概念
async fn demonstrate_async_await_basics() {
    println!("\n🔍 1. async/await 基础概念");
    demonstrate_lazy_execution().await;
    demonstrate_await_mechanism().await;
    demonstrate_async_blocks().await;
    demonstrate_thread_movement().await;
    demonstrate_send_sync_constraints().await;
}

/// 演示懒惰执行机制
async fn demonstrate_lazy_execution() {
    println!("\n🔍 1.1 懒惰执行机制");
    
    // 创建 Future 但不立即执行
    let future1 = async {
        println!("      🔄 Future 1 开始执行");
        sleep(Duration::from_millis(100)).await;
        println!("      ✅ Future 1 执行完成");
        "Future 1 结果"
    };
    
    let future2 = async {
        println!("      🔄 Future 2 开始执行");
        sleep(Duration::from_millis(50)).await;
        println!("      ✅ Future 2 执行完成");
        "Future 2 结果"
    };
    
    println!("      📝 Future 已创建但未执行");
    
    // 只有在 await 时才开始执行
    println!("      🚀 开始执行 Future...");
    let (result1, result2) = join!(future1, future2);
    
    println!("      📊 执行结果: {} | {}", result1, result2);
    println!("      💡 关键点: Future 是懒惰的，只有被 await 时才执行");
}

/// 演示 await 机制
async fn demonstrate_await_mechanism() {
    println!("\n🔍 1.2 await 机制");
    
    async fn async_operation(name: &str, delay: u64) -> String {
        println!("      🔄 {} 开始", name);
        sleep(Duration::from_millis(delay)).await;
        println!("      ✅ {} 完成", name);
        format!("{} 结果", name)
    }
    
    // 顺序执行
    println!("      📝 顺序执行:");
    let start = Instant::now();
    let _result1 = async_operation("操作A", 100).await;
    let _result2 = async_operation("操作B", 100).await;
    let sequential_time = start.elapsed();
    
    // 并发执行
    println!("      📝 并发执行:");
    let start = Instant::now();
    let (result1, result2) = join!(
        async_operation("操作C", 100),
        async_operation("操作D", 100)
    );
    let concurrent_time = start.elapsed();
    
    println!("      📊 顺序执行耗时: {:?}", sequential_time);
    println!("      📊 并发执行耗时: {:?}", concurrent_time);
    println!("      💡 并发执行显著提升性能");
}

/// 演示异步块
async fn demonstrate_async_blocks() {
    println!("\n🔍 1.3 异步块");
    
    // 异步块创建 Future
    let async_block = async {
        println!("      🔄 异步块开始执行");
        
        let data = vec![1, 2, 3, 4, 5];
        let mut results = Vec::new();
        
        for item in data {
            // 模拟异步处理
            sleep(Duration::from_millis(10)).await;
            results.push(item * 2);
        }
        
        println!("      ✅ 异步块执行完成");
        results
    };
    
    // 异步块与闭包结合
    let async_closure = |multiplier: i32| async move {
        println!("      🔄 异步闭包执行，倍数: {}", multiplier);
        sleep(Duration::from_millis(20)).await;
        multiplier * 10
    };
    
    let block_result = async_block.await;
    let closure_result = async_closure(5).await;
    
    println!("      📊 异步块结果: {:?}", block_result);
    println!("      📊 异步闭包结果: {}", closure_result);
    println!("      💡 异步块提供了灵活的异步代码组织方式");
}

/// 演示线程间移动
async fn demonstrate_thread_movement() {
    println!("\n🔍 1.4 线程间移动");
    
    let thread_id = std::thread::current().id();
    println!("      📍 开始线程 ID: {:?}", thread_id);
    
    // 异步操作可能在不同线程上执行
    sleep(Duration::from_millis(1)).await;
    
    let new_thread_id = std::thread::current().id();
    println!("      📍 await 后线程 ID: {:?}", new_thread_id);
    
    if thread_id == new_thread_id {
        println!("      ✅ 在同一线程上执行");
    } else {
        println!("      🔄 切换到不同线程执行");
    }
    
    println!("      💡 异步任务可能在不同线程间移动，这是运行时调度的结果");
}

/// 演示 Send + Sync 约束
async fn demonstrate_send_sync_constraints() {
    println!("\n🔍 1.5 Send + Sync 约束");
    
    // Send 类型可以在线程间安全传递
    let send_data = Arc::new("可以在线程间传递的数据");
    let cloned_data = send_data.clone();
    
    let handle = tokio::spawn(async move {
        println!("      📦 在新任务中访问数据: {}", cloned_data);
        "任务完成"
    });
    
    let result = handle.await.unwrap();
    println!("      ✅ 任务结果: {}", result);
    println!("      💡 Send + Sync 约束确保异步代码的线程安全性");
}

/// 演示 Future trait 深入分析
async fn demonstrate_future_trait() {
    println!("\n🔍 2. Future trait 深入分析");
    demonstrate_custom_future().await;
    demonstrate_state_machine().await;
    demonstrate_waker_mechanism().await;
    demonstrate_poll_mechanism().await;
}

/// 自定义 Future 实现
struct DelayFuture {
    when: Instant,
    waker: Option<Waker>,
}

impl DelayFuture {
    fn new(duration: Duration) -> Self {
        Self {
            when: Instant::now() + duration,
            waker: None,
        }
    }
}

impl Future for DelayFuture {
    type Output = ();
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("      ✅ DelayFuture 完成");
            Poll::Ready(())
        } else {
            println!("      ⏳ DelayFuture 等待中...");
            
            // 保存 waker 以便稍后唤醒
            self.waker = Some(cx.waker().clone());
            
            // 模拟异步唤醒机制
            let waker = cx.waker().clone();
            let when = self.when;
            tokio::spawn(async move {
                let now = Instant::now();
                if when > now {
                    sleep(when - now).await;
                }
                waker.wake();
            });
            
            Poll::Pending
        }
    }
}

/// 演示自定义 Future
async fn demonstrate_custom_future() {
    println!("\n🔍 2.1 自定义 Future");
    
    println!("      🔄 创建自定义 DelayFuture...");
    let custom_future = DelayFuture::new(Duration::from_millis(100));
    
    let start = Instant::now();
    custom_future.await;
    let elapsed = start.elapsed();
    
    println!("      📊 自定义 Future 执行时间: {:?}", elapsed);
    println!("      💡 自定义 Future 展示了异步机制的底层实现");
}

/// 演示状态机机制
async fn demonstrate_state_machine() {
    println!("\n🔍 2.2 状态机机制");
    
    // 复杂的异步函数会被编译器转换为状态机
    async fn complex_async_function() -> String {
        println!("      📍 状态 1: 开始执行");
        
        sleep(Duration::from_millis(50)).await;
        println!("      📍 状态 2: 第一个 await 完成");
        
        let data = "中间数据";
        sleep(Duration::from_millis(50)).await;
        println!("      📍 状态 3: 第二个 await 完成");
        
        format!("处理完成: {}", data)
    }
    
    println!("      🔄 执行复杂异步函数...");
    let result = complex_async_function().await;
    println!("      📊 结果: {}", result);
    println!("      💡 编译器将异步函数转换为状态机，每个 await 点是一个状态转换");
}

/// 演示 Waker 机制
async fn demonstrate_waker_mechanism() {
    println!("\n🔍 2.3 Waker 机制");
    
    let (tx, rx) = oneshot::channel::<String>();
    
    // 模拟异步操作
    tokio::spawn(async move {
        sleep(Duration::from_millis(100)).await;
        let _ = tx.send("Waker 唤醒的结果".to_string());
    });
    
    println!("      ⏳ 等待 Waker 唤醒...");
    match rx.await {
        Ok(result) => {
            println!("      ✅ 收到结果: {}", result);
            println!("      💡 Waker 机制允许 Future 在准备好时被唤醒");
        }
        Err(e) => println!("      ❌ 接收失败: {}", e),
    }
}

/// 演示 Poll 机制
async fn demonstrate_poll_mechanism() {
    println!("\n🔍 2.4 Poll 机制");
    
    struct CounterFuture {
        count: usize,
        max: usize,
    }
    
    impl CounterFuture {
        fn new(max: usize) -> Self {
            Self { count: 0, max }
        }
    }
    
    impl Future for CounterFuture {
        type Output = usize;
        
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.count >= self.max {
                println!("      ✅ CounterFuture 完成，计数: {}", self.count);
                Poll::Ready(self.count)
            } else {
                self.count += 1;
                println!("      🔄 CounterFuture 计数: {}/{}", self.count, self.max);
                
                // 请求再次轮询
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
    
    println!("      🔄 启动 CounterFuture...");
    let counter = CounterFuture::new(3);
    let final_count = counter.await;
    
    println!("      📊 最终计数: {}", final_count);
    println!("      💡 Poll 机制是 Future 执行的核心，通过返回 Ready 或 Pending 控制执行流程");
}

/// 演示异步运行时机制
async fn demonstrate_async_runtime() {
    println!("\n🔍 3. 异步运行时机制");
    demonstrate_tokio_runtime().await;
    demonstrate_task_scheduling().await;
    demonstrate_executor_principles().await;
}

/// 演示 Tokio 运行时
async fn demonstrate_tokio_runtime() {
    println!("\n🔍 3.1 Tokio 运行时");
    
    // 获取运行时信息
    println!("      📊 当前运行时信息:");
    println!("      - 运行时类型: Tokio");
    
    // 并发任务执行
    let tasks: Vec<_> = (1..=5)
        .map(|i| {
            tokio::spawn(async move {
                let delay = rand::thread_rng().gen_range(10..100);
                sleep(Duration::from_millis(delay)).await;
                format!("任务 {} 完成，延迟 {}ms", i, delay)
            })
        })
        .collect();
    
    println!("      🔄 执行 5 个并发任务...");
    let results = join_all(tasks).await;
    
    for result in results {
        match result {
            Ok(msg) => println!("      ✅ {}", msg),
            Err(e) => println!("      ❌ 任务失败: {}", e),
        }
    }
    
    println!("      💡 Tokio 运行时高效管理异步任务的执行");
}

/// 演示任务调度
async fn demonstrate_task_scheduling() {
    println!("\n🔍 3.2 任务调度");
    
    // 高优先级任务
    let high_priority = tokio::spawn(async {
        println!("      🔥 高优先级任务开始");
        for i in 1..=3 {
            println!("      🔥 高优先级任务步骤 {}", i);
            tokio::task::yield_now().await; // 主动让出控制权
        }
        "高优先级完成"
    });
    
    // 低优先级任务
    let low_priority = tokio::spawn(async {
        println!("      📝 低优先级任务开始");
        for i in 1..=3 {
            println!("      📝 低优先级任务步骤 {}", i);
            tokio::task::yield_now().await;
        }
        "低优先级完成"
    });
    
    let (high_result, low_result) = join!(high_priority, low_priority);
    
    println!("      📊 高优先级结果: {:?}", high_result);
    println!("      📊 低优先级结果: {:?}", low_result);
    println!("      💡 任务调度器协调多个任务的执行顺序");
}

/// 演示执行器原理
async fn demonstrate_executor_principles() {
    println!("\n🔍 3.3 执行器原理");
    
    // 模拟执行器的工作原理
    let (tx, mut rx) = mpsc::channel::<String>(10);
    
    // 生产者任务
    let producer = tokio::spawn(async move {
        for i in 1..=5 {
            let message = format!("消息 {}", i);
            println!("      📤 发送: {}", message);
            tx.send(message).await.unwrap();
            sleep(Duration::from_millis(50)).await;
        }
        println!("      ✅ 生产者完成");
    });
    
    // 消费者任务
    let consumer = tokio::spawn(async move {
        let mut count = 0;
        while let Some(message) = rx.recv().await {
            count += 1;
            println!("      📥 接收: {} (第{}条)", message, count);
            sleep(Duration::from_millis(30)).await;
        }
        println!("      ✅ 消费者完成，共处理 {} 条消息", count);
        count
    });
    
    let (_, message_count) = join!(producer, consumer);
    
    println!("      📊 总共处理消息数: {:?}", message_count);
    println!("      💡 执行器协调生产者和消费者的异步执行");
}

/// 演示 Stream 流处理
async fn demonstrate_stream_processing() {
    println!("\n🔍 4. Stream 流处理");
    demonstrate_stream_basics().await;
    demonstrate_stream_vs_iterator().await;
    demonstrate_stream_combinators().await;
    demonstrate_custom_stream().await;
}

/// 演示 Stream 基础
async fn demonstrate_stream_basics() {
    println!("\n🔍 4.1 Stream 基础");
    
    use futures::stream;
    
    // 从迭代器创建 Stream
    let number_stream = stream::iter(1..=5);
    
    println!("      🔄 处理数字流...");
    let results: Vec<_> = number_stream
        .map(|x| x * 2)
        .collect()
        .await;
    
    println!("      📊 流处理结果: {:?}", results);
    
    // 异步 Stream 处理
    let async_stream = stream::iter(1..=3)
        .then(|x| async move {
            sleep(Duration::from_millis(50)).await;
            x * x
        });
    
    println!("      🔄 处理异步流...");
    let async_results: Vec<_> = async_stream.collect().await;
    
    println!("      📊 异步流结果: {:?}", async_results);
    println!("      💡 Stream 提供了处理异步数据序列的强大能力");
}

/// 演示 Stream vs Iterator
async fn demonstrate_stream_vs_iterator() {
    println!("\n🔍 4.2 Stream vs Iterator");
    
    // Iterator 同步处理
    println!("      📝 Iterator 同步处理:");
    let start = Instant::now();
    let iter_results: Vec<_> = (1..=5)
        .map(|x| {
            std::thread::sleep(Duration::from_millis(20)); // 同步延迟
            x * 2
        })
        .collect();
    let iter_time = start.elapsed();
    
    // Stream 异步处理
    println!("      📝 Stream 异步处理:");
    let start = Instant::now();
    use futures::stream;
    let stream_results: Vec<_> = stream::iter(1..=5)
        .then(|x| async move {
            sleep(Duration::from_millis(20)).await; // 异步延迟
            x * 2
        })
        .collect()
        .await;
    let stream_time = start.elapsed();
    
    println!("      📊 Iterator 结果: {:?}, 耗时: {:?}", iter_results, iter_time);
    println!("      📊 Stream 结果: {:?}, 耗时: {:?}", stream_results, stream_time);
    println!("      💡 Stream 支持异步操作，Iterator 只能同步处理");
}

/// 演示 Stream 组合子
async fn demonstrate_stream_combinators() {
    println!("\n🔍 4.3 Stream 组合子");
    
    use futures::stream;
    
    let data_stream = stream::iter(1..=10)
        .filter(|&x| async move { x % 2 == 0 }) // 过滤偶数
        .map(|x| async move { x * x }) // 平方
        .buffer_unordered(3) // 并发处理，最多3个
        .take(3); // 只取前3个
    
    println!("      🔄 应用 Stream 组合子...");
    let results: Vec<_> = data_stream.collect().await;
    
    println!("      📊 组合子处理结果: {:?}", results);
    println!("      💡 Stream 组合子提供了强大的数据流处理能力");
}

/// 自定义 Stream 实现
struct NumberStream {
    current: usize,
    max: usize,
    delay: Duration,
}

impl NumberStream {
    fn new(max: usize, delay: Duration) -> Self {
        Self { current: 0, max, delay }
    }
}

impl Stream for NumberStream {
    type Item = usize;
    
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.current >= self.max {
            return Poll::Ready(None);
        }
        
        let current = self.current;
        self.current += 1;
        
        // 模拟异步延迟
        let delay = self.delay;
        let waker = cx.waker().clone();
        
        tokio::spawn(async move {
            sleep(delay).await;
            waker.wake();
        });
        
        Poll::Ready(Some(current))
    }
}

/// 演示自定义 Stream
async fn demonstrate_custom_stream() {
    println!("\n🔍 4.4 自定义 Stream");
    
    let custom_stream = NumberStream::new(5, Duration::from_millis(100));
    
    println!("      🔄 处理自定义 Stream...");
    tokio::pin!(custom_stream);
    
    while let Some(value) = custom_stream.next().await {
        println!("      📊 Stream 产生值: {}", value);
    }
    
    println!("      ✅ 自定义 Stream 处理完成");
    println!("      💡 自定义 Stream 允许实现特定的异步数据生成逻辑");
}

/// 演示异步编程模式
async fn demonstrate_async_patterns() {
    println!("\n🔍 5. 异步编程模式");
    demonstrate_error_handling().await;
    demonstrate_timeout_cancellation().await;
    demonstrate_concurrency_control().await;
}

/// 演示错误处理
async fn demonstrate_error_handling() {
    println!("\n🔍 5.1 错误处理");
    
    async fn fallible_operation(should_fail: bool) -> Result<String, &'static str> {
        sleep(Duration::from_millis(50)).await;
        
        if should_fail {
            Err("操作失败")
        } else {
            Ok("操作成功".to_string())
        }
    }
    
    // 处理成功情况
    match fallible_operation(false).await {
        Ok(result) => println!("      ✅ 成功: {}", result),
        Err(e) => println!("      ❌ 失败: {}", e),
    }
    
    // 处理失败情况
    match fallible_operation(true).await {
        Ok(result) => println!("      ✅ 成功: {}", result),
        Err(e) => println!("      ❌ 失败: {}", e),
    }
    
    // 错误恢复
    let result = match fallible_operation(true).await {
        Ok(result) => result,
        Err(_) => "恢复后的结果".to_string(),
    };
    
    println!("      🔄 错误恢复结果: {:?}", result);
    println!("      💡 异步错误处理需要特别注意 Result 类型的传播");
}

/// 演示超时和取消
async fn demonstrate_timeout_cancellation() {
    println!("\n🔍 5.2 超时和取消");
    
    async fn long_running_task() -> String {
        sleep(Duration::from_millis(200)).await;
        "长时间任务完成".to_string()
    }
    
    // 超时处理
    match timeout(Duration::from_millis(100), long_running_task()).await {
        Ok(result) => println!("      ✅ 任务完成: {}", result),
        Err(_) => println!("      ⏰ 任务超时"),
    }
    
    // 成功的超时处理
    match timeout(Duration::from_millis(300), long_running_task()).await {
        Ok(result) => println!("      ✅ 任务完成: {}", result),
        Err(_) => println!("      ⏰ 任务超时"),
    }
    
    println!("      💡 超时机制防止任务无限期等待");
}

/// 演示并发控制
async fn demonstrate_concurrency_control() {
    println!("\n🔍 5.3 并发控制");
    
    // 使用信号量限制并发数
    let semaphore = Arc::new(Semaphore::new(2)); // 最多2个并发
    
    let tasks: Vec<_> = (1..=5)
        .map(|i| {
            let sem = semaphore.clone();
            tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                println!("      🔄 任务 {} 开始执行", i);
                sleep(Duration::from_millis(100)).await;
                println!("      ✅ 任务 {} 完成", i);
                i
            })
        })
        .collect();
    
    println!("      🔄 执行受限并发任务...");
    let results = join_all(tasks).await;
    
    let successful_results: Vec<_> = results
        .into_iter()
        .filter_map(|r| r.ok())
        .collect();
    
    println!("      📊 完成的任务: {:?}", successful_results);
    println!("      💡 信号量有效控制了并发执行的数量");
}

/// 演示实际应用案例
async fn demonstrate_practical_examples() {
    println!("\n🔍 6. 实际应用案例");
    demonstrate_network_io().await;
    demonstrate_file_io().await;
    demonstrate_concurrent_tasks().await;
}

/// 演示网络 I/O 处理
async fn demonstrate_network_io() {
    println!("\n🔍 6.1 网络 I/O 处理");
    
    // 模拟 HTTP 客户端
    #[derive(Debug, Serialize, Deserialize)]
    struct ApiResponse {
        id: u32,
        title: String,
        completed: bool,
    }
    
    async fn fetch_data(id: u32) -> Result<ApiResponse, Box<dyn std::error::Error + Send + Sync>> {
        println!("      🌐 请求数据 ID: {}", id);
        
        // 模拟网络延迟
        sleep(Duration::from_millis(100)).await;
        
        // 模拟 API 响应
        Ok(ApiResponse {
            id,
            title: format!("任务 {}", id),
            completed: rand::thread_rng().gen_bool(0.7),
        })
    }
    
    // 并发请求多个数据
    let ids = vec![1, 2, 3, 4, 5];
    let futures: Vec<_> = ids.into_iter().map(fetch_data).collect();
    
    match join_all(futures).await.into_iter().collect::<Result<Vec<_>, _>>() {
        Ok(responses) => {
            println!("      ✅ 成功获取 {} 条数据:", responses.len());
            for response in responses {
                println!("         - ID: {}, 标题: {}, 完成: {}", 
                    response.id, response.title, response.completed);
            }
        }
        Err(e) => println!("      ❌ 请求失败: {}", e),
    }
}

/// 演示文件 I/O 处理
async fn demonstrate_file_io() {
    println!("\n🔍 6.2 文件 I/O 处理");
    
    let file_path = "temp_async_file.txt";
    let content = "这是异步文件操作的测试内容\n包含多行数据\n用于演示异步 I/O";
    
    // 异步写入文件
    match File::create(file_path).await {
        Ok(mut file) => {
            if let Err(e) = file.write_all(content.as_bytes()).await {
                println!("      ❌ 写入失败: {}", e);
                return;
            }
            println!("      ✅ 文件写入成功");
        }
        Err(e) => {
            println!("      ❌ 创建文件失败: {}", e);
            return;
        }
    }
    
    // 异步读取文件
    match File::open(file_path).await {
        Ok(mut file) => {
            let mut buffer = String::new();
            match file.read_to_string(&mut buffer).await {
                Ok(bytes_read) => {
                    println!("      ✅ 读取 {} 字节:", bytes_read);
                    println!("      📄 内容:\n{}", buffer);
                }
                Err(e) => println!("      ❌ 读取失败: {}", e),
            }
        }
        Err(e) => println!("      ❌ 打开文件失败: {}", e),
    }
    
    // 清理临时文件
    let _ = std::fs::remove_file(file_path);
}

/// 演示并发任务处理
async fn demonstrate_concurrent_tasks() {
    println!("\n🔍 6.3 并发任务处理");
    
    async fn cpu_intensive_task(id: u32, duration: u64) -> String {
        println!("      🔄 任务 {} 开始执行", id);
        
        // 模拟 CPU 密集型任务
        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(duration) {
            // 模拟计算
            let _ = (0..1000).map(|i| i * i).collect::<Vec<_>>();
        }
        
        let result = format!("任务 {} 完成，耗时 {}ms", id, duration);
        println!("      ✅ {}", result);
        result
    }
    
    // 并发执行多个任务
    let tasks = vec![
        cpu_intensive_task(1, 200),
        cpu_intensive_task(2, 150),
        cpu_intensive_task(3, 300),
        cpu_intensive_task(4, 100),
    ];
    
    let start_time = Instant::now();
    let results = join_all(tasks).await;
    let total_time = start_time.elapsed();
    
    println!("      📊 所有任务完成，总耗时: {:?}", total_time);
    println!("      📋 结果汇总:");
    for (i, result) in results.iter().enumerate() {
        println!("         {}. {}", i + 1, result);
    }
}

/// 演示性能优化技巧
async fn demonstrate_performance_optimization() {
    println!("\n🔍 7. 性能优化技巧");
    demonstrate_zero_cost_abstraction().await;
    demonstrate_memory_efficiency().await;
    demonstrate_scheduling_optimization().await;
}

/// 演示零成本抽象
async fn demonstrate_zero_cost_abstraction() {
    println!("\n🔍 7.1 零成本抽象");
    
    // 编译时优化的异步函数
    async fn optimized_computation(n: u64) -> u64 {
        // 这个函数会被编译器内联优化
        (0..n).map(|i| i * 2).sum()
    }
    
    let start = Instant::now();
    let result = optimized_computation(1_000_000).await;
    let duration = start.elapsed();
    
    println!("      ⚡ 计算结果: {}", result);
    println!("      ⏱️ 执行时间: {:?}", duration);
    println!("      💡 编译器优化使异步调用几乎无开销");
}

/// 演示内存使用效率
async fn demonstrate_memory_efficiency() {
    println!("\n🔍 7.2 内存使用效率");
    
    // 使用 Box::pin 减少栈内存使用
    let large_future = Box::pin(async {
        let large_data = vec![0u8; 1024 * 1024]; // 1MB 数据
        sleep(Duration::from_millis(10)).await;
        large_data.len()
    });
    
    let size = large_future.await;
    println!("      📦 处理了 {} 字节的大型数据", size);
    println!("      💾 使用 Box::pin 避免栈溢出");
    
    // 流式处理避免内存积累
    use futures::stream;
    let data_stream = stream::iter(0..1000)
        .map(|i| async move { i * i })
        .buffer_unordered(10); // 限制并发数
    
    let mut count = 0;
    let mut sum = 0;
    
    tokio::pin!(data_stream);
    while let Some(value) = data_stream.next().await {
        sum += value;
        count += 1;
        if count % 100 == 0 {
            println!("      📊 已处理 {} 项，当前和: {}", count, sum);
        }
    }
    
    println!("      ✅ 流式处理完成，总和: {}", sum);
}

/// 演示调度策略优化
async fn demonstrate_scheduling_optimization() {
    println!("\n🔍 7.3 调度策略优化");
    
    // 使用 yield_now 主动让出控制权
    async fn cooperative_task(id: u32) {
        for i in 0..5 {
            println!("      🔄 任务 {} 执行步骤 {}", id, i + 1);
            
            // 主动让出控制权，避免阻塞其他任务
            tokio::task::yield_now().await;
        }
    }
    
    // 并发执行协作式任务
    let tasks = (1..=3).map(cooperative_task);
    join_all(tasks).await;
    
    println!("      ✅ 协作式调度完成");
}

/// 演示高级主题
async fn demonstrate_advanced_topics() {
    println!("\n🔍 8. 高级主题");
    demonstrate_custom_future_advanced().await;
    demonstrate_stream_combinators_advanced().await;
    demonstrate_async_iterator().await;
}

/// 演示高级自定义 Future
async fn demonstrate_custom_future_advanced() {
    println!("\n🔍 8.1 高级自定义 Future");
    
    struct TimerFuture {
        start: Instant,
        duration: Duration,
    }
    
    impl TimerFuture {
        fn new(duration: Duration) -> Self {
            Self {
                start: Instant::now(),
                duration,
            }
        }
    }
    
    impl Future for TimerFuture {
        type Output = Duration;
        
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let elapsed = self.start.elapsed();
            if elapsed >= self.duration {
                Poll::Ready(elapsed)
            } else {
                // 注册 waker 以便在适当时候被唤醒
                let waker = cx.waker().clone();
                let remaining = self.duration - elapsed;
                
                tokio::spawn(async move {
                    sleep(remaining).await;
                    waker.wake();
                });
                
                Poll::Pending
            }
        }
    }
    
    let timer = TimerFuture::new(Duration::from_millis(100));
    let elapsed = timer.await;
    println!("      ⏰ 定时器完成，实际耗时: {:?}", elapsed);
}

/// 演示高级 Stream 组合子
async fn demonstrate_stream_combinators_advanced() {
    println!("\n🔍 8.2 高级 Stream 组合子");
    
    use futures::stream;
    
    // 创建复杂的数据流管道
    let data_pipeline = stream::iter(1..=20)
        .filter(|&x| async move { x % 2 == 0 }) // 过滤偶数
        .map(|x| async move { x * x }) // 平方
        .buffer_unordered(5) // 并发处理
        .filter_map(|x| async move {
            if x > 50 { Some(x) } else { None }
        }) // 过滤大于50的值
        .take(5); // 只取前5个
    
    println!("      🔄 数据流管道处理中...");
    let results: Vec<_> = data_pipeline.collect().await;
    println!("      ✅ 管道结果: {:?}", results);
}

/// 演示异步迭代器
async fn demonstrate_async_iterator() {
    println!("\n🔍 8.3 异步迭代器");
    
    struct AsyncRange {
        current: usize,
        end: usize,
        delay: Duration,
    }
    
    impl AsyncRange {
        fn new(start: usize, end: usize, delay: Duration) -> Self {
            Self {
                current: start,
                end,
                delay,
            }
        }
    }
    
    impl Stream for AsyncRange {
        type Item = usize;
        
        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            if self.current >= self.end {
                return Poll::Ready(None);
            }
            
            let current = self.current;
            self.current += 1;
            
            // 模拟异步延迟
            let delay = self.delay;
            let waker = cx.waker().clone();
            
            tokio::spawn(async move {
                sleep(delay).await;
                waker.wake();
            });
            
            Poll::Ready(Some(current))
        }
    }
    
    let async_range = AsyncRange::new(1, 6, Duration::from_millis(50));
    
    println!("      🔄 异步迭代器执行中...");
    tokio::pin!(async_range);
    while let Some(value) = async_range.next().await {
        println!("      📊 异步产生值: {}", value);
    }
    
    println!("      ✅ 异步迭代完成");
}
