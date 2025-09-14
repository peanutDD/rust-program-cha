//! Rust 异步编程疑难问题解决方案深度分析
//! 
//! 本项目全面分析和解决 Rust 异步编程中的各种疑难问题，
//! 提供最新、最高效的解决方案和实际应用案例。
//!
//! ## 涵盖的疑难问题清单：
//! 1. async trait 问题：trait 中的异步方法限制
//! 2. Send/Sync 约束问题：跨线程异步编程约束
//! 3. 递归异步函数：无限递归和栈溢出问题
//! 4. 异步闭包问题：闭包中的异步操作和生命周期
//! 5. Stream 流处理：背压控制和错误处理
//! 6. Pin/Unpin 问题：自引用结构体和内存安全
//! 7. 异步 Drop 问题：资源清理和生命周期管理
//! 8. 性能优化：零成本抽象和编译器优化
//! 9. 实际应用案例：Web 服务、数据库连接池等
//! 10. 异步迭代器问题：AsyncIterator 和流式处理
//! 11. 取消和超时机制：优雅的任务取消和超时处理
//! 12. 异步锁竞争：死锁预防和性能优化
//! 13. 内存泄漏问题：循环引用和资源泄漏
//! 14. 异步测试难题：测试异步代码的最佳实践
//! 15. 错误传播问题：异步环境中的错误处理策略

use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use async_trait::async_trait;
use futures::{
    future::{BoxFuture, FutureExt},
    stream::{Stream, StreamExt},
    // sink::SinkExt,
};
use pin_project::pin_project;
use tokio::{
    sync::{mpsc, RwLock, Semaphore},
    time::{sleep, timeout},
};
use tracing::{info, warn, error, debug};
use anyhow::{Result, anyhow};
use thiserror::Error;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志系统
    tracing_subscriber::fmt::init();
    
    info!("🚀 开始 Rust 异步编程疑难问题解决方案演示");
    
    // 1. async trait 问题解决方案
    demonstrate_async_trait_solutions().await?;
    
    // 2. Send 和 Sync 约束问题
    demonstrate_send_sync_solutions().await?;
    
    // 3. 递归异步函数问题
    demonstrate_recursive_async_solutions().await?;
    
    // 4. 异步闭包问题
    demonstrate_async_closure_solutions().await?;
    
    // 5. Stream 流处理疑难问题
    demonstrate_stream_processing_solutions().await?;
    
    // 6. Pin 和 Unpin 相关问题
    demonstrate_pin_unpin_solutions().await?;
    
    // 7. 异步 Drop 问题
    demonstrate_async_drop_solutions().await?;
    
    // 8. 性能优化技巧
    demonstrate_performance_optimization().await?;
    
    // 9. 实际应用解决方案
    demonstrate_real_world_solutions().await?;
    
    // 10. 异步迭代器问题
    demonstrate_async_iterator_solutions().await?;
    
    // 11. 取消和超时机制
    demonstrate_cancellation_timeout_solutions().await?;
    
    // 12. 异步锁竞争问题
    demonstrate_async_lock_solutions().await?;
    
    // 13. 内存泄漏问题
    demonstrate_memory_leak_solutions().await?;
    
    // 14. 异步测试难题
    demonstrate_async_testing_solutions().await?;
    
    // 15. 错误传播问题
    demonstrate_error_propagation_solutions().await?;
    
    info!("✅ 所有 15 个异步编程疑难问题解决方案演示完成！");
    Ok(())
}

// ==================== 1. async trait 问题解决方案 ====================

/// 传统方式：使用 Box<dyn Future> (旧方法，不推荐)
#[allow(dead_code)]
trait OldAsyncTrait {
    fn async_method(&self) -> BoxFuture<'_, Result<String>>;
}

/// 现代解决方案：使用 async-trait crate (推荐)
#[async_trait]
trait ModernAsyncTrait: Send + Sync {
    async fn async_method(&self) -> Result<String>;
    async fn async_method_with_params(&self, data: &str) -> Result<String>;
}

/// 数据库连接抽象
#[async_trait]
trait DatabaseConnection: Send + Sync {
    async fn connect(&self) -> Result<()>;
    async fn execute_query(&self, query: &str) -> Result<Vec<String>>;
    async fn close(&self) -> Result<()>;
}

/// MySQL 连接实现
struct MySqlConnection {
    host: String,
    connected: Arc<RwLock<bool>>,
}

#[async_trait]
impl DatabaseConnection for MySqlConnection {
    async fn connect(&self) -> Result<()> {
        info!("🔌 连接到 MySQL: {}", self.host);
        sleep(Duration::from_millis(100)).await;
        *self.connected.write().await = true;
        Ok(())
    }
    
    async fn execute_query(&self, query: &str) -> Result<Vec<String>> {
        if !*self.connected.read().await {
            return Err(anyhow!("数据库未连接"));
        }
        
        info!("📊 执行查询: {}", query);
        sleep(Duration::from_millis(50)).await;
        
        Ok(vec![
            format!("结果1: {}", query),
            format!("结果2: {}", query),
        ])
    }
    
    async fn close(&self) -> Result<()> {
        info!("🔌 关闭 MySQL 连接");
        *self.connected.write().await = false;
        Ok(())
    }
}

/// PostgreSQL 连接实现
struct PostgreSqlConnection {
    host: String,
    connected: Arc<RwLock<bool>>,
}

#[async_trait]
impl DatabaseConnection for PostgreSqlConnection {
    async fn connect(&self) -> Result<()> {
        info!("🔌 连接到 PostgreSQL: {}", self.host);
        sleep(Duration::from_millis(120)).await;
        *self.connected.write().await = true;
        Ok(())
    }
    
    async fn execute_query(&self, query: &str) -> Result<Vec<String>> {
        if !*self.connected.read().await {
            return Err(anyhow!("数据库未连接"));
        }
        
        info!("📊 执行 PostgreSQL 查询: {}", query);
        sleep(Duration::from_millis(60)).await;
        
        Ok(vec![
            format!("PG结果1: {}", query),
            format!("PG结果2: {}", query),
            format!("PG结果3: {}", query),
        ])
    }
    
    async fn close(&self) -> Result<()> {
        info!("🔌 关闭 PostgreSQL 连接");
        *self.connected.write().await = false;
        Ok(())
    }
}

async fn demonstrate_async_trait_solutions() -> Result<()> {
    info!("\n📋 1. async trait 问题解决方案");
    
    // 1.1 多态数据库连接
    info!("\n🔍 1.1 多态数据库连接");
    
    let connections: Vec<Box<dyn DatabaseConnection>> = vec![
        Box::new(MySqlConnection {
            host: "mysql://localhost:3306".to_string(),
            connected: Arc::new(RwLock::new(false)),
        }),
        Box::new(PostgreSqlConnection {
            host: "postgresql://localhost:5432".to_string(),
            connected: Arc::new(RwLock::new(false)),
        }),
    ];
    
    for (i, conn) in connections.iter().enumerate() {
        info!("      📊 测试连接 {}", i + 1);
        conn.connect().await?;
        let results = conn.execute_query("SELECT * FROM users").await?;
        info!("      ✅ 查询结果: {:?}", results);
        conn.close().await?;
    }
    
    // 1.2 泛型异步 trait 使用
    info!("\n🔍 1.2 泛型异步 trait 使用");
    
    let service = AsyncService::new();
    let result1 = service.process_data("重要数据").await?;
    let result2 = service.batch_process(vec!["数据1", "数据2", "数据3"]).await?;
    
    info!("      ✅ 处理结果1: {}", result1);
    info!("      ✅ 批量处理结果: {:?}", result2);
    
    Ok(())
}

/// 异步服务示例
struct AsyncService;

#[async_trait]
impl ModernAsyncTrait for AsyncService {
    async fn async_method(&self) -> Result<String> {
        sleep(Duration::from_millis(50)).await;
        Ok("异步方法执行完成".to_string())
    }
    
    async fn async_method_with_params(&self, data: &str) -> Result<String> {
        sleep(Duration::from_millis(30)).await;
        Ok(format!("处理数据: {}", data))
    }
}

impl AsyncService {
    fn new() -> Self {
        Self
    }
    
    async fn process_data(&self, data: &str) -> Result<String> {
        self.async_method_with_params(data).await
    }
    
    async fn batch_process(&self, data_list: Vec<&str>) -> Result<Vec<String>> {
        let mut results = Vec::new();
        for data in data_list {
            let result = self.process_data(data).await?;
            results.push(result);
        }
        Ok(results)
    }
}

// ==================== 2. Send 和 Sync 约束问题 ====================

/// 自定义错误类型
#[derive(Error, Debug)]
enum AsyncError {
    #[error("网络错误: {0}")]
    NetworkError(String),
    #[error("超时错误")]
    TimeoutError,
    #[error("数据处理错误: {0}")]
    ProcessingError(String),
}

/// 线程安全的共享状态
#[derive(Clone)]
struct SharedState {
    data: Arc<RwLock<HashMap<String, String>>>,
    counter: Arc<RwLock<u64>>,
}

impl SharedState {
    fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            counter: Arc::new(RwLock::new(0)),
        }
    }
    
    async fn insert(&self, key: String, value: String) -> Result<()> {
        let mut data = self.data.write().await;
        let mut counter = self.counter.write().await;
        
        data.insert(key.clone(), value.clone());
        *counter += 1;
        
        info!("      📝 插入数据: {} -> {}, 总数: {}", key, value, *counter);
        Ok(())
    }
    
    async fn get(&self, key: &str) -> Option<String> {
        let data = self.data.read().await;
        data.get(key).cloned()
    }
    
    async fn get_count(&self) -> u64 {
        *self.counter.read().await
    }
}

async fn demonstrate_send_sync_solutions() -> Result<()> {
    info!("\n📋 2. Send 和 Sync 约束问题解决方案");
    
    // 2.1 跨线程共享状态
    info!("\n🔍 2.1 跨线程共享状态");
    
    let shared_state = SharedState::new();
    let mut handles = Vec::new();
    
    // 启动多个任务并发操作共享状态
    for i in 0..5 {
        let state = shared_state.clone();
        let handle = tokio::spawn(async move {
            let key = format!("key_{}", i);
            let value = format!("value_{}", i);
            
            // 模拟一些异步工作
            sleep(Duration::from_millis(rand::random::<u64>() % 100)).await;
            
            state.insert(key.clone(), value).await?;
            
            // 读取数据
            if let Some(retrieved_value) = state.get(&key).await {
                info!("      ✅ 任务 {} 成功读取: {}", i, retrieved_value);
            }
            
            Ok::<(), anyhow::Error>(())
        });
        handles.push(handle);
    }
    
    // 等待所有任务完成
    for handle in handles {
        handle.await??;
    }
    
    let final_count = shared_state.get_count().await;
    info!("      📊 最终数据总数: {}", final_count);
    
    // 2.2 Send 约束的异步闭包
    info!("\n🔍 2.2 Send 约束的异步闭包");
    
    let processor = AsyncProcessor::new();
    let results = processor.process_with_closure().await?;
    info!("      ✅ 闭包处理结果: {:?}", results);
    
    Ok(())
}

/// 异步处理器
struct AsyncProcessor {
    semaphore: Arc<Semaphore>,
}

impl AsyncProcessor {
    fn new() -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(3)), // 限制并发数
        }
    }
    
    async fn process_with_closure(&self) -> Result<Vec<String>> {
        let data = vec!["数据A", "数据B", "数据C", "数据D", "数据E"];
        let mut results = Vec::new();
        
        // 使用 Send 约束的异步闭包
        let process_fn = |item: String| -> BoxFuture<'static, Result<String>> {
            async move {
                sleep(Duration::from_millis(100)).await;
                Ok(format!("处理完成: {}", item))
            }.boxed()
        };
        
        for item in data {
            let _permit = self.semaphore.acquire().await?;
            let result = process_fn(item.to_string()).await?;
            results.push(result);
        }
        
        Ok(results)
    }
}

// ==================== 3. 递归异步函数问题 ====================

/// 传统方式：使用 Box<dyn Future> (旧方法)
#[allow(dead_code)]
fn old_recursive_async(n: u32) -> BoxFuture<'static, u32> {
    async move {
        if n <= 1 {
            1
        } else {
            let prev = old_recursive_async(n - 1).await;
            n + prev
        }
    }.boxed()
}

/// 现代解决方案：使用 async-recursion crate (推荐)
#[async_recursion::async_recursion]
async fn modern_recursive_fibonacci(n: u64) -> u64 {
    if n <= 1 {
        n
    } else if n > 50 {
        // 防止溢出，限制最大值
        u64::MAX
    } else {
        let a = modern_recursive_fibonacci(n - 1).await;
        let b = modern_recursive_fibonacci(n - 2).await;
        a.saturating_add(b)
    }
}

/// 优化的递归异步函数：带缓存
#[async_recursion::async_recursion]
async fn cached_recursive_fibonacci(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if let Some(&result) = cache.get(&n) {
        return result;
    }
    
    let result = if n <= 1 {
        n
    } else if n > 50 {
        // 防止溢出，限制最大值
        u64::MAX
    } else {
        let a = cached_recursive_fibonacci(n - 1, cache).await;
        let b = cached_recursive_fibonacci(n - 2, cache).await;
        a.saturating_add(b)
    };
    
    cache.insert(n, result);
    result
}

/// 递归目录遍历
#[async_recursion::async_recursion]
async fn recursive_directory_scan(path: String, depth: usize) -> Result<Vec<String>> {
    if depth > 10 {
        return Ok(vec![]);
    }
    
    // 模拟文件系统操作
    sleep(Duration::from_millis(10)).await;
    
    let mut results = vec![path.clone()];
    
    // 模拟子目录
    if depth < 3 {
        for i in 1..=2 {
            let sub_path = format!("{}/子目录{}", path, i);
            let mut sub_results = recursive_directory_scan(sub_path, depth + 1).await?;
            results.append(&mut sub_results);
        }
    }
    
    Ok(results)
}

async fn demonstrate_recursive_async_solutions() -> Result<()> {
    info!("\n📋 3. 递归异步函数问题解决方案");
    
    // 3.1 基础递归异步函数
    info!("\n🔍 3.1 基础递归异步函数");
    
    let start = Instant::now();
    let fib_result = modern_recursive_fibonacci(10).await;
    let duration = start.elapsed();
    info!("      ✅ 斐波那契(10) = {}, 耗时: {:?}", fib_result, duration);
    
    // 3.2 带缓存的递归异步函数
    info!("\n🔍 3.2 带缓存的递归异步函数");
    
    let start = Instant::now();
    let mut cache = HashMap::new();
    let cached_result = cached_recursive_fibonacci(15, &mut cache).await; // 减小数值避免溢出
    let duration = start.elapsed();
    info!("      ✅ 缓存斐波那契(15) = {}, 耗时: {:?}", cached_result, duration);
    info!("      📊 缓存大小: {}", cache.len());
    
    // 3.3 递归目录遍历
    info!("\n🔍 3.3 递归目录遍历");
    
    let paths = recursive_directory_scan("/根目录".to_string(), 0).await?;
    info!("      ✅ 扫描到的路径:");
    for path in paths {
        info!("        - {}", path);
    }
    
    Ok(())
}

// ==================== 4. 异步闭包问题 ====================

/// 异步闭包处理器
struct AsyncClosureProcessor;

impl AsyncClosureProcessor {
    /// 使用异步闭包进行数据转换
    async fn transform_data<F, Fut>(&self, data: Vec<i32>, transform_fn: F) -> Result<Vec<String>>
    where
        F: Fn(i32) -> Fut + Send + Sync,
        Fut: Future<Output = Result<String>> + Send + 'static,
    {
        let mut results = Vec::new();
        
        for item in data {
            let result = transform_fn(item).await?;
            results.push(result);
        }
        
        Ok(results)
    }
    
    /// 并行处理数据
    async fn parallel_transform<F, Fut>(&self, data: Vec<i32>, transform_fn: F) -> Result<Vec<String>>
    where
        F: Fn(i32) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = Result<String>> + Send + 'static,
    {
        let handles: Vec<_> = data
            .into_iter()
            .map(|item| {
                let transform_fn = transform_fn.clone();
                tokio::spawn(async move {
                    transform_fn(item).await
                })
            })
            .collect();
        
        let mut results = Vec::new();
        for handle in handles {
            let result = handle.await??;
            results.push(result);
        }
        
        Ok(results)
    }
}

async fn demonstrate_async_closure_solutions() -> Result<()> {
    info!("\n📋 4. 异步闭包问题解决方案");
    
    let processor = AsyncClosureProcessor;
    
    // 4.1 基础异步闭包
    info!("\n🔍 4.1 基础异步闭包");
    
    let data = vec![1, 2, 3, 4, 5];
    
    // 定义异步转换闭包
    let transform_fn = |x: i32| async move {
        sleep(Duration::from_millis(50)).await;
        Ok(format!("转换结果: {}", x * x))
    };
    
    let results = processor.transform_data(data.clone(), transform_fn).await?;
    info!("      ✅ 转换结果: {:?}", results);
    
    // 4.2 并行异步闭包处理
    info!("\n🔍 4.2 并行异步闭包处理");
    
    let parallel_transform_fn = |x: i32| async move {
        sleep(Duration::from_millis(100)).await;
        Ok(format!("并行处理: {} -> {}", x, x * 2))
    };
    
    let start = Instant::now();
    let parallel_results = processor.parallel_transform(data, parallel_transform_fn).await?;
    let duration = start.elapsed();
    
    info!("      ✅ 并行处理结果: {:?}", parallel_results);
    info!("      ⏱️ 并行处理耗时: {:?}", duration);
    
    // 4.3 复杂异步闭包：带错误处理
    info!("\n🔍 4.3 复杂异步闭包：带错误处理");
    
    let error_prone_data = vec![1, 2, -1, 4, 5]; // -1 会导致错误
    
    let error_handling_fn = |x: i32| async move {
        sleep(Duration::from_millis(30)).await;
        if x < 0 {
            Err(anyhow!("负数处理错误: {}", x))
        } else {
            Ok(format!("安全处理: {}", x))
        }
    };
    
    match processor.transform_data(error_prone_data, error_handling_fn).await {
        Ok(results) => info!("      ✅ 处理成功: {:?}", results),
        Err(e) => info!("      ❌ 处理失败: {}", e),
    }
    
    Ok(())
}

// ==================== 5. Stream 流处理疑难问题 ====================

use futures::stream;
// use tokio_stream::wrappers::ReceiverStream;

/// 自定义流处理器
#[pin_project]
struct CustomStream<S> {
    #[pin]
    inner: S,
    buffer_size: usize,
    processed_count: usize,
}

impl<S> CustomStream<S> {
    fn new(stream: S, buffer_size: usize) -> Self {
        Self {
            inner: stream,
            buffer_size,
            processed_count: 0,
        }
    }
}

impl<S> Stream for CustomStream<S>
where
    S: Stream,
{
    type Item = S::Item;
    
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.project();
        
        match this.inner.poll_next(cx) {
            std::task::Poll::Ready(Some(item)) => {
                *this.processed_count += 1;
                if *this.processed_count % 100 == 0 {
                    debug!("已处理 {} 个项目", *this.processed_count);
                }
                std::task::Poll::Ready(Some(item))
            }
            other => other,
        }
    }
}

/// 流处理管道
struct StreamPipeline;

impl StreamPipeline {
    /// 创建数据流
    async fn create_data_stream() -> impl Stream<Item = i32> {
        stream::iter(1..=1000)
    }
    
    /// 背压控制流处理
    async fn backpressure_processing() -> Result<()> {
        info!("\n🔍 5.1 背压控制流处理");
        
        let (tx, mut rx) = mpsc::channel(10); // 小缓冲区模拟背压
        
        // 生产者任务
        let producer = tokio::spawn(async move {
            for i in 1..=100 {
                if let Err(_) = tx.send(i).await {
                    warn!("生产者发送失败: {}", i);
                    break;
                }
                
                if i % 10 == 0 {
                    info!("      📤 生产者已发送: {}", i);
                }
                
                // 模拟生产延迟
                sleep(Duration::from_millis(10)).await;
            }
            info!("      ✅ 生产者完成");
        });
        
        // 消费者任务
        let consumer = tokio::spawn(async move {
            let mut count = 0;
            
            while let Some(item) = rx.recv().await {
                // 模拟处理时间
                sleep(Duration::from_millis(50)).await;
                count += 1;
                
                if count % 10 == 0 {
                    info!("      📥 消费者已处理: {} 项，当前项: {}", count, item);
                }
            }
            
            info!("      ✅ 消费者完成，总处理: {} 项", count);
        });
        
        // 等待完成
        let _ = tokio::try_join!(producer, consumer)?;
        
        Ok(())
    }
    
    /// 流错误处理和恢复
    async fn error_handling_stream() -> Result<()> {
        info!("\n🔍 5.2 流错误处理和恢复");
        
        let data_stream = stream::iter(1..=20)
            .map(|x| async move {
                sleep(Duration::from_millis(20)).await;
                if x % 7 == 0 {
                    Err(AsyncError::ProcessingError(format!("处理项目 {} 失败", x)))
                } else {
                    Ok(x * 2)
                }
            })
            .buffer_unordered(5); // 并发处理
        
        let mut success_count = 0;
        let mut error_count = 0;
        
        tokio::pin!(data_stream);
        
        while let Some(result) = data_stream.next().await {
            match result {
                Ok(value) => {
                    success_count += 1;
                    if success_count % 5 == 0 {
                        info!("      ✅ 成功处理: {}, 累计成功: {}", value, success_count);
                    }
                }
                Err(e) => {
                    error_count += 1;
                    warn!("      ❌ 处理错误: {}, 累计错误: {}", e, error_count);
                }
            }
        }
        
        info!("      📊 流处理统计 - 成功: {}, 错误: {}", success_count, error_count);
        
        Ok(())
    }
    
    /// 流性能优化
    async fn optimized_stream_processing() -> Result<()> {
        info!("\n🔍 5.3 流性能优化");
        
        let start = Instant::now();
        
        // 创建高性能流处理管道
        let data_stream = Self::create_data_stream().await;
        let filtered_data: Vec<i32> = data_stream
            .map(|x| x * 2)
            .filter(|&x| async move { x % 3 != 0 })
            .collect()
            .await;
        
        let processed_count = stream::iter(filtered_data.chunks(50))
            .map(|chunk| async move {
                // 批量异步处理
                let sum: i64 = chunk.iter().map(|&x| x as i64).sum(); // 使用i64避免溢出
                sleep(Duration::from_millis(10)).await;
                sum
            })
            .buffer_unordered(4) // 并发批次处理
            .fold(0i64, |acc, batch_sum| async move { acc.saturating_add(batch_sum) }) // 使用饱和加法
            .await;
        
        let duration = start.elapsed();
        
        info!("      ✅ 优化流处理完成");
        info!("      📊 处理结果: {}", processed_count);
        info!("      ⏱️ 处理耗时: {:?}", duration);
        
        Ok(())
    }
}

async fn demonstrate_stream_processing_solutions() -> Result<()> {
    info!("\n📋 5. Stream 流处理疑难问题解决方案");
    
    StreamPipeline::backpressure_processing().await?;
    StreamPipeline::error_handling_stream().await?;
    StreamPipeline::optimized_stream_processing().await?;
    
    Ok(())
}

// ==================== 6. Pin 和 Unpin 相关问题 ====================

use std::marker::PhantomPinned;

/// 自引用结构体示例
#[pin_project]
struct SelfReferential {
    data: String,
    #[pin]
    pointer: *const u8,
    _pin: PhantomPinned,
}

impl SelfReferential {
    fn new(data: String) -> Pin<Box<Self>> {
        let mut boxed = Box::pin(SelfReferential {
            pointer: std::ptr::null(),
            data,
            _pin: PhantomPinned,
        });
        
        // 安全地设置自引用指针
        let ptr = boxed.as_ref().data.as_ptr();
        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);
            mut_ref.get_unchecked_mut().pointer = ptr;
        }
        
        boxed
    }
    
    fn get_data(&self) -> &str {
        &self.data
    }
    
    fn get_pointer_value(&self) -> Option<u8> {
        if self.pointer.is_null() {
            None
        } else {
            unsafe { Some(*self.pointer) }
        }
    }
}

/// Pin 相关的异步结构体
#[pin_project]
struct PinnedAsyncStruct {
    #[pin]
    future: BoxFuture<'static, Result<String>>,
    state: String,
}

impl PinnedAsyncStruct {
    fn new(delay_ms: u64, message: String) -> Self {
        let future = async move {
            sleep(Duration::from_millis(delay_ms)).await;
            Ok(format!("异步完成: {}", message))
        }.boxed();
        
        Self {
            future,
            state: "初始化".to_string(),
        }
    }
}

impl Future for PinnedAsyncStruct {
    type Output = Result<String>;
    
    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let this = self.project();
        
        *this.state = "轮询中".to_string();
        
        match this.future.poll(cx) {
            std::task::Poll::Ready(result) => {
                *this.state = "已完成".to_string();
                std::task::Poll::Ready(result)
            }
            std::task::Poll::Pending => {
                *this.state = "等待中".to_string();
                std::task::Poll::Pending
            }
        }
    }
}

async fn demonstrate_pin_unpin_solutions() -> Result<()> {
    info!("\n📋 6. Pin 和 Unpin 相关问题解决方案");
    
    // 6.1 自引用结构体
    info!("\n🔍 6.1 自引用结构体");
    
    let self_ref = SelfReferential::new("Hello, Pin!".to_string());
    info!("      ✅ 自引用数据: {}", self_ref.get_data());
    if let Some(first_byte) = self_ref.get_pointer_value() {
        info!("      ✅ 指针指向的第一个字节: {} ('{}')", first_byte, first_byte as char);
    }
    
    // 6.2 Pin 异步结构体
    info!("\n🔍 6.2 Pin 异步结构体");
    
    let mut pinned_async = PinnedAsyncStruct::new(100, "Pin 测试".to_string());
    
    // 使用 Pin 包装
    let pinned = unsafe { Pin::new_unchecked(&mut pinned_async) };
    
    let result = pinned.await?;
    info!("      ✅ Pin 异步结果: {}", result);
    
    // 6.3 Pin 在实际应用中的使用
    info!("\n🔍 6.3 Pin 在实际应用中的使用");
    
    let pin_manager = PinManager::new();
    let results = pin_manager.process_pinned_futures().await?;
    
    for (i, result) in results.iter().enumerate() {
        info!("      ✅ Pin 处理结果 {}: {}", i + 1, result);
    }
    
    Ok(())
}

/// Pin 管理器
struct PinManager;

impl PinManager {
    fn new() -> Self {
        Self
    }
    
    async fn process_pinned_futures(&self) -> Result<Vec<String>> {
        let futures = vec![
            PinnedAsyncStruct::new(50, "任务A".to_string()),
            PinnedAsyncStruct::new(75, "任务B".to_string()),
            PinnedAsyncStruct::new(100, "任务C".to_string()),
        ];
        
        let mut results = Vec::new();
        
        for future in futures {
            let result = future.await?;
            results.push(result);
        }
        
        Ok(results)
    }
}

// ==================== 7. 异步 Drop 问题 ====================

/// 需要异步清理的资源
struct AsyncResource {
    id: String,
    cleanup_sender: Option<mpsc::UnboundedSender<String>>,
}

impl AsyncResource {
    fn new(id: String, cleanup_sender: mpsc::UnboundedSender<String>) -> Self {
        info!("      🔧 创建异步资源: {}", id);
        Self {
            id,
            cleanup_sender: Some(cleanup_sender),
        }
    }
    
    /// 手动异步清理方法
    async fn async_cleanup(&mut self) -> Result<()> {
        if let Some(sender) = self.cleanup_sender.take() {
            info!("      🧹 开始异步清理资源: {}", self.id);
            
            // 模拟异步清理工作
            sleep(Duration::from_millis(50)).await;
            
            // 通知清理完成
            if let Err(_) = sender.send(self.id.clone()) {
                warn!("      ⚠️ 清理通知发送失败: {}", self.id);
            }
            
            info!("      ✅ 异步清理完成: {}", self.id);
        }
        Ok(())
    }
}

// 同步 Drop 实现（作为备用）
impl Drop for AsyncResource {
    fn drop(&mut self) {
        if self.cleanup_sender.is_some() {
            warn!("      ⚠️ 资源 {} 在 Drop 中清理（非异步）", self.id);
        }
    }
}

/// 异步资源管理器
struct AsyncResourceManager {
    cleanup_sender: mpsc::UnboundedSender<String>,
}

impl AsyncResourceManager {
    fn new() -> Self {
        let (cleanup_sender, _) = mpsc::unbounded_channel();
        Self {
            cleanup_sender,
        }
    }
}

async fn demonstrate_async_drop_solutions() -> Result<()> {
    info!("\n📋 7. 异步 Drop 问题解决方案");
    
    // 7.1 手动异步清理
    info!("\n🔍 7.1 手动异步清理");
    
    let mut manager = AsyncResourceManager::new();
    
    // 启动清理监控器
    let cleanup_handle = {
        let (cleanup_sender, mut cleanup_receiver) = mpsc::unbounded_channel();
        manager.cleanup_sender = cleanup_sender;
        tokio::spawn(async move {
            while let Some(resource_id) = cleanup_receiver.recv().await {
                info!("      📝 记录资源清理: {}", resource_id);
            }
            info!("      ✅ 清理监控器结束");
        })
    };
    
    // 创建和清理资源
    {
        let mut resource1 = AsyncResource::new("资源1".to_string(), manager.cleanup_sender.clone());
        let mut resource2 = AsyncResource::new("资源2".to_string(), manager.cleanup_sender.clone());
        let mut resource3 = AsyncResource::new("资源3".to_string(), manager.cleanup_sender.clone());
        
        // 手动异步清理
        resource1.async_cleanup().await?;
        resource2.async_cleanup().await?;
        resource3.async_cleanup().await?;
    }
    
    // 关闭清理通道
    drop(manager.cleanup_sender);
    
    // 等待清理监控器完成
    cleanup_handle.await?;
    
    // 7.2 RAII 模式的异步清理
    info!("\n🔍 7.2 RAII 模式的异步清理");
    
    let raii_manager = RaiiAsyncManager::new().await?;
    raii_manager.demonstrate_raii_cleanup().await?;
    
    Ok(())
}

/// RAII 异步管理器
struct RaiiAsyncManager {
    resources: Vec<String>,
}

impl RaiiAsyncManager {
    async fn new() -> Result<Self> {
        info!("      🔧 初始化 RAII 异步管理器");
        sleep(Duration::from_millis(30)).await;
        
        Ok(Self {
            resources: Vec::new(),
        })
    }
    
    async fn demonstrate_raii_cleanup(&self) -> Result<()> {
        info!("      🔍 演示 RAII 异步清理模式");
        
        // 使用作用域确保资源清理
        {
            let _guard1 = AsyncGuard::new("守护资源1".to_string()).await?;
            let _guard2 = AsyncGuard::new("守护资源2".to_string()).await?;
            
            // 模拟一些工作
            sleep(Duration::from_millis(100)).await;
            
            info!("      ✅ 工作完成，守护资源即将清理");
        } // 守护资源在此处自动清理
        
        info!("      ✅ RAII 清理演示完成");
        Ok(())
    }
}

/// 异步守护资源
struct AsyncGuard {
    resource_name: String,
}

impl AsyncGuard {
    async fn new(resource_name: String) -> Result<Self> {
        info!("      🛡️ 创建异步守护: {}", resource_name);
        sleep(Duration::from_millis(20)).await;
        
        Ok(Self { resource_name })
    }
}

impl Drop for AsyncGuard {
    fn drop(&mut self) {
        info!("      🧹 异步守护清理: {}", self.resource_name);
        // 注意：这里只能做同步清理
        // 异步清理需要在 Drop 之前手动调用
    }
}

// ==================== 8. 性能优化技巧 ====================

/// 性能优化器
struct PerformanceOptimizer;

impl PerformanceOptimizer {
    /// 零成本抽象演示
    async fn zero_cost_abstraction() -> Result<()> {
        info!("\n🔍 8.1 零成本抽象演示");
        
        let data = (1..=100).collect::<Vec<_>>();
        
        // 使用迭代器链（零成本抽象）
        let start = Instant::now();
        let result: i64 = data
            .iter()
            .filter(|&&x| x % 2 == 0)
            .map(|&x| (x as i64) * (x as i64))
            .sum();
        let duration = start.elapsed();
        
        info!("      ✅ 零成本抽象结果: {}", result);
        info!("      ⏱️ 处理耗时: {:?}", duration);
        
        Ok(())
    }
    
    /// 异步任务池优化
    async fn optimized_task_pool() -> Result<()> {
        info!("\n🔍 8.2 异步任务池优化");
        
        let semaphore = Arc::new(Semaphore::new(10)); // 限制并发数
        let tasks = (1..=100).map(|i| {
            let semaphore = semaphore.clone();
            tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                
                // 模拟 CPU 密集型工作
                let result = (1..=1000).fold(0, |acc, x| acc + x * i);
                
                sleep(Duration::from_millis(10)).await;
                result
            })
        }).collect::<Vec<_>>();
        
        let start = Instant::now();
        let results = futures::future::try_join_all(tasks).await?;
        let duration = start.elapsed();
        
        let sum: i64 = results.iter().sum();
        info!("      ✅ 任务池处理完成，总和: {}", sum);
        info!("      ⏱️ 总耗时: {:?}", duration);
        
        Ok(())
    }
    
    /// 内存优化技巧
    async fn memory_optimization() -> Result<()> {
        info!("\n🔍 8.3 内存优化技巧");
        
        // 使用流式处理避免大量内存分配
        let start = Instant::now();
        
        let result = stream::iter(1..=1000) // 减少数据量避免溢出
            .map(|x| async move {
                // 避免不必要的分配
                (x * 2) as i64
            })
            .buffered(100) // 控制并发数
            .fold(0i64, |acc, x| async move { acc.saturating_add(x) }) // 使用饱和加法避免溢出
            .await;
        
        let duration = start.elapsed();
        
        info!("      ✅ 流式处理结果: {}", result);
        info!("      ⏱️ 处理耗时: {:?}", duration);
        
        Ok(())
    }
    
    /// 编译器优化提示
    async fn compiler_optimization_hints() -> Result<()> {
        info!("\n🔍 8.4 编译器优化提示");
        
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        
        // 使用 likely/unlikely 提示（需要 nightly）
        let start = Instant::now();
        let mut sum = 0;
        
        for &item in &data {
            if item > 5 {
                // 大概率分支
                sum += item * 2;
            } else {
                // 小概率分支
                sum += item;
            }
        }
        
        let duration = start.elapsed();
        
        info!("      ✅ 优化计算结果: {}", sum);
        info!("      ⏱️ 计算耗时: {:?}", duration);
        
        Ok(())
    }
}

async fn demonstrate_performance_optimization() -> Result<()> {
    info!("\n📋 8. 性能优化技巧");
    
    PerformanceOptimizer::zero_cost_abstraction().await?;
    PerformanceOptimizer::optimized_task_pool().await?;
    PerformanceOptimizer::memory_optimization().await?;
    PerformanceOptimizer::compiler_optimization_hints().await?;
    
    Ok(())
}

// ==================== 9. 实际应用解决方案 ====================

/// Web 服务器模拟
struct AsyncWebServer {
    connection_pool: Arc<Semaphore>,
    request_count: Arc<RwLock<u64>>,
}

impl AsyncWebServer {
    fn new(max_connections: usize) -> Self {
        Self {
            connection_pool: Arc::new(Semaphore::new(max_connections)),
            request_count: Arc::new(RwLock::new(0)),
        }
    }
    
    async fn handle_request(&self, request_id: u64) -> Result<String> {
        let _permit = self.connection_pool.acquire().await?;
        
        // 增加请求计数
        {
            let mut count = self.request_count.write().await;
            *count += 1;
        }
        
        // 模拟请求处理
        sleep(Duration::from_millis(rand::random::<u64>() % 100 + 50)).await;
        
        Ok(format!("请求 {} 处理完成", request_id))
    }
    
    async fn get_stats(&self) -> u64 {
        *self.request_count.read().await
    }
}

/// 数据库连接池模拟
struct AsyncDatabasePool {
    connections: Arc<Semaphore>,
    active_queries: Arc<RwLock<HashMap<u64, String>>>,
}

impl AsyncDatabasePool {
    fn new(pool_size: usize) -> Self {
        Self {
            connections: Arc::new(Semaphore::new(pool_size)),
            active_queries: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    async fn execute_query(&self, query_id: u64, sql: String) -> Result<Vec<String>> {
        let _permit = self.connections.acquire().await?;
        
        // 记录活跃查询
        {
            let mut queries = self.active_queries.write().await;
            queries.insert(query_id, sql.clone());
        }
        
        // 模拟查询执行
        sleep(Duration::from_millis(rand::random::<u64>() % 200 + 100)).await;
        
        // 移除活跃查询记录
        {
            let mut queries = self.active_queries.write().await;
            queries.remove(&query_id);
        }
        
        Ok(vec![
            format!("查询 {} 结果1", query_id),
            format!("查询 {} 结果2", query_id),
        ])
    }
    
    async fn get_active_query_count(&self) -> usize {
        self.active_queries.read().await.len()
    }
}

/// 文件处理管道
struct AsyncFileProcessor;

impl AsyncFileProcessor {
    async fn process_files(&self, file_paths: Vec<String>) -> Result<Vec<String>> {
        let results = stream::iter(file_paths)
            .map(|path| async move {
                self.process_single_file(path).await
            })
            .buffered(5) // 并发处理 5 个文件
            .collect::<Vec<_>>()
            .await;
        
        results.into_iter().collect()
    }
    
    async fn process_single_file(&self, file_path: String) -> Result<String> {
        // 模拟文件读取
        sleep(Duration::from_millis(50)).await;
        
        // 模拟文件处理
        sleep(Duration::from_millis(100)).await;
        
        // 模拟文件写入
        sleep(Duration::from_millis(30)).await;
        
        Ok(format!("处理完成: {}", file_path))
    }
}

async fn demonstrate_real_world_solutions() -> Result<()> {
    info!("\n📋 9. 实际应用解决方案");
    
    // 9.1 Web 服务器负载测试
    info!("\n🔍 9.1 Web 服务器负载测试");
    
    let web_server = Arc::new(AsyncWebServer::new(10));
    let mut request_handles = Vec::new();
    
    // 模拟并发请求
    for i in 1..=50 {
        let server = web_server.clone();
        let handle = tokio::spawn(async move {
            server.handle_request(i).await
        });
        request_handles.push(handle);
    }
    
    // 等待所有请求完成
    let start = Instant::now();
    let results = futures::future::try_join_all(request_handles).await?;
    let duration = start.elapsed();
    
    let success_count = results.iter().filter(|r| r.is_ok()).count();
    let total_requests = web_server.get_stats().await;
    
    info!("      ✅ Web 服务器测试完成");
    info!("      📊 成功请求: {}/{}", success_count, results.len());
    info!("      📊 总请求数: {}", total_requests);
    info!("      ⏱️ 总耗时: {:?}", duration);
    
    // 9.2 数据库连接池测试
    info!("\n🔍 9.2 数据库连接池测试");
    
    let db_pool = Arc::new(AsyncDatabasePool::new(5));
    let mut query_handles = Vec::new();
    
    // 模拟并发数据库查询
    for i in 1..=20 {
        let pool = db_pool.clone();
        let handle = tokio::spawn(async move {
            let sql = format!("SELECT * FROM table_{}", i);
            pool.execute_query(i, sql).await
        });
        query_handles.push(handle);
    }
    
    // 监控活跃查询
    let monitor_handle = {
        let pool = db_pool.clone();
        tokio::spawn(async move {
            for _ in 0..10 {
                let active_count = pool.get_active_query_count().await;
                info!("      📊 活跃查询数: {}", active_count);
                sleep(Duration::from_millis(100)).await;
            }
        })
    };
    
    // 等待查询完成
    let query_results = futures::future::try_join_all(query_handles).await?;
    monitor_handle.await?;
    
    let successful_queries = query_results.iter().filter(|r| r.is_ok()).count();
    info!("      ✅ 数据库查询完成: {}/{}", successful_queries, query_results.len());
    
    // 9.3 文件处理管道测试
    info!("\n🔍 9.3 文件处理管道测试");
    
    let file_processor = AsyncFileProcessor;
    let file_paths = (1..=15)
        .map(|i| format!("/path/to/file_{}.txt", i))
        .collect();
    
    let start = Instant::now();
    let processing_results = file_processor.process_files(file_paths).await?;
    let duration = start.elapsed();
    
    info!("      ✅ 文件处理完成: {} 个文件", processing_results.len());
    info!("      ⏱️ 处理耗时: {:?}", duration);
    
    for (i, result) in processing_results.iter().take(5).enumerate() {
        info!("      📄 {}: {}", i + 1, result);
    }
    
    if processing_results.len() > 5 {
        info!("      ... 还有 {} 个文件处理结果", processing_results.len() - 5);
    }
    
    Ok(())
}

// ============================================================================
// 10. 异步迭代器疑难问题解决方案
// ============================================================================

use std::collections::VecDeque;

struct AsyncIteratorProcessor {
    buffer: VecDeque<i32>,
}

impl AsyncIteratorProcessor {
    fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
        }
    }

    // 异步迭代器适配器
    async fn async_map_collect<T, F, Fut>(&self, items: Vec<T>, mapper: F) -> Result<Vec<String>>
    where
        T: Send + 'static,
        F: Fn(T) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = Result<String>> + Send + 'static,
    {
        let stream = stream::iter(items)
            .map(|item| {
                let mapper = mapper.clone();
                async move { mapper(item).await }
            })
            .buffer_unordered(10); // 并发处理

        stream.collect::<Vec<_>>().await
            .into_iter()
            .collect::<Result<Vec<_>>>()
    }

    // 异步过滤器
    async fn async_filter<T, F, Fut>(&self, items: Vec<T>, predicate: F) -> Result<Vec<T>>
    where
        T: Send + Clone + 'static,
        F: Fn(T) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = bool> + Send + 'static,
    {
        let mut results = Vec::new();
        let stream = stream::iter(items)
            .filter_map(|item| {
                let predicate = predicate.clone();
                let item_clone = item.clone();
                async move {
                    if predicate(item_clone.clone()).await {
                        Some(item_clone)
                    } else {
                        None
                    }
                }
            });

        let mut stream = Box::pin(stream);
        while let Some(item) = stream.next().await {
            results.push(item);
        }
        Ok(results)
    }
}

async fn demonstrate_async_iterator_solutions() -> Result<()> {
    info!("=== 异步迭代器疑难问题解决方案 ===");
    
    let processor = AsyncIteratorProcessor::new();
    
    // 异步映射
    let numbers = vec![1, 2, 3, 4, 5];
    let mapped_results = processor.async_map_collect(numbers, |n| async move {
        sleep(Duration::from_millis(10)).await;
        Ok(format!("处理结果: {}", n * 2))
    }).await?;
    
    info!("异步映射结果: {:?}", mapped_results);
    
    // 异步过滤
    let test_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let filtered_results = processor.async_filter(test_data, |n| async move {
        sleep(Duration::from_millis(5)).await;
        n % 2 == 0
    }).await?;
    
    info!("异步过滤结果: {:?}", filtered_results);
    
    Ok(())
}

// ============================================================================
// 11. 取消和超时机制疑难问题解决方案
// ============================================================================

use tokio::sync::oneshot;
use tokio::select;

#[derive(Clone)]
struct CancellationManager {
    active_tasks: Arc<RwLock<HashMap<String, oneshot::Sender<()>>>>,
}

impl CancellationManager {
    fn new() -> Self {
        Self {
            active_tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // 可取消的任务
    async fn cancellable_task(&self, task_id: String, work_duration: Duration) -> Result<String> {
        let (cancel_tx, cancel_rx) = oneshot::channel();
        
        // 注册取消通道
        {
            let mut tasks = self.active_tasks.write().await;
            tasks.insert(task_id.clone(), cancel_tx);
        }

        let result = select! {
            _ = cancel_rx => {
                warn!("任务 {} 被取消", task_id);
                Err(anyhow!("任务被取消"))
            }
            result = self.simulate_work(work_duration) => {
                info!("任务 {} 完成", task_id);
                Ok(result)
            }
        };

        // 清理任务记录
        {
            let mut tasks = self.active_tasks.write().await;
            tasks.remove(&task_id);
        }

        result
    }

    async fn simulate_work(&self, duration: Duration) -> String {
        sleep(duration).await;
        "工作完成".to_string()
    }

    // 取消指定任务
    async fn cancel_task(&self, task_id: &str) -> bool {
        let mut tasks = self.active_tasks.write().await;
        if let Some(cancel_tx) = tasks.remove(task_id) {
            let _ = cancel_tx.send(());
            true
        } else {
            false
        }
    }

    // 带超时的任务执行
    async fn task_with_timeout<F, Fut>(&self, timeout_duration: Duration, task: F) -> Result<String>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<String>>,
    {
        match timeout(timeout_duration, task()).await {
            Ok(result) => result,
            Err(_) => Err(anyhow!("任务超时")),
        }
    }
}

async fn demonstrate_cancellation_timeout_solutions() -> Result<()> {
    info!("=== 取消和超时机制疑难问题解决方案 ===");
    
    let manager = CancellationManager::new();
    
    // 测试可取消任务
    let task_id = "test_task_1".to_string();
    let task_handle = {
        let manager = manager.clone();
        let task_id = task_id.clone();
        tokio::spawn(async move {
            manager.cancellable_task(task_id, Duration::from_millis(1000)).await
        })
    };
    
    // 等待一段时间后取消任务
    sleep(Duration::from_millis(100)).await;
    let cancelled = manager.cancel_task(&task_id).await;
    info!("任务取消状态: {}", cancelled);
    
    match task_handle.await? {
        Ok(result) => info!("任务结果: {}", result),
        Err(e) => info!("任务错误: {}", e),
    }
    
    // 测试超时机制
    let timeout_result = manager.task_with_timeout(
        Duration::from_millis(50),
        || async {
            sleep(Duration::from_millis(100)).await;
            Ok("不应该看到这个结果".to_string())
        }
    ).await;
    
    match timeout_result {
        Ok(result) => info!("超时测试结果: {}", result),
        Err(e) => info!("超时测试错误: {}", e),
    }
    
    Ok(())
}

// ============================================================================
// 12. 异步锁竞争疑难问题解决方案
// ============================================================================

use tokio::sync::{Mutex as TokioMutex, RwLock as TokioRwLock};
use std::sync::atomic::{AtomicU64, Ordering};

struct AsyncLockManager {
    shared_counter: Arc<TokioMutex<u64>>,
    read_write_data: Arc<TokioRwLock<HashMap<String, String>>>,
    lock_free_counter: Arc<AtomicU64>,
}

impl AsyncLockManager {
    fn new() -> Self {
        Self {
            shared_counter: Arc::new(TokioMutex::new(0)),
            read_write_data: Arc::new(TokioRwLock::new(HashMap::new())),
            lock_free_counter: Arc::new(AtomicU64::new(0)),
        }
    }

    // 互斥锁竞争优化
    async fn optimized_mutex_access(&self, worker_id: usize) -> Result<()> {
        for i in 0..10 {
            // 使用try_lock避免长时间阻塞
            match self.shared_counter.try_lock() {
                Ok(mut counter) => {
                    *counter += 1;
                    info!("Worker {} 成功获取锁，计数器: {}", worker_id, *counter);
                }
                Err(_) => {
                    // 锁被占用，执行其他工作或短暂等待
                    debug!("Worker {} 锁被占用，执行其他工作", worker_id);
                    sleep(Duration::from_millis(1)).await;
                    
                    // 重试获取锁
                    let mut counter = self.shared_counter.lock().await;
                    *counter += 1;
                    info!("Worker {} 重试获取锁成功，计数器: {}", worker_id, *counter);
                }
            }
            
            // 模拟其他工作
            sleep(Duration::from_millis(5)).await;
        }
        Ok(())
    }

    // 读写锁优化
    async fn optimized_rwlock_access(&self, worker_id: usize, is_writer: bool) -> Result<()> {
        if is_writer {
            // 写操作
            let mut data = self.read_write_data.write().await;
            data.insert(format!("key_{}", worker_id), format!("value_{}", worker_id));
            info!("Writer {} 写入数据", worker_id);
        } else {
            // 读操作
            let data = self.read_write_data.read().await;
            let count = data.len();
            info!("Reader {} 读取数据，当前条目数: {}", worker_id, count);
        }
        Ok(())
    }

    // 无锁编程示例
    async fn lock_free_operations(&self, worker_id: usize) -> Result<()> {
        for _ in 0..100 {
            // 原子操作，无需锁
            let old_value = self.lock_free_counter.fetch_add(1, Ordering::Relaxed);
            if old_value % 50 == 0 {
                info!("Worker {} 无锁计数器达到: {}", worker_id, old_value + 1);
            }
        }
        Ok(())
    }
}

async fn demonstrate_async_lock_solutions() -> Result<()> {
    info!("=== 异步锁竞争疑难问题解决方案 ===");
    
    let manager = Arc::new(AsyncLockManager::new());
    
    // 测试互斥锁优化
    let mut mutex_handles = Vec::new();
    for i in 0..5 {
        let manager = manager.clone();
        let handle = tokio::spawn(async move {
            manager.optimized_mutex_access(i).await
        });
        mutex_handles.push(handle);
    }
    
    // 等待所有互斥锁任务完成
    for handle in mutex_handles {
        handle.await??;
    }
    
    // 测试读写锁优化
    let mut rwlock_handles = Vec::new();
    for i in 0..8 {
        let manager = manager.clone();
        let is_writer = i < 2; // 前两个是写者，其余是读者
        let handle = tokio::spawn(async move {
            manager.optimized_rwlock_access(i, is_writer).await
        });
        rwlock_handles.push(handle);
    }
    
    for handle in rwlock_handles {
        handle.await??;
    }
    
    // 测试无锁操作
    let mut lockfree_handles = Vec::new();
    for i in 0..4 {
        let manager = manager.clone();
        let handle = tokio::spawn(async move {
            manager.lock_free_operations(i).await
        });
        lockfree_handles.push(handle);
    }
    
    for handle in lockfree_handles {
        handle.await??;
    }
    
    let final_count = manager.lock_free_counter.load(Ordering::Relaxed);
    info!("最终无锁计数器值: {}", final_count);
    
    Ok(())
}

// ============================================================================
// 13. 内存泄漏疑难问题解决方案
// ============================================================================

use std::sync::Weak;
use tokio::sync::broadcast;

struct MemoryLeakPrevention {
    strong_refs: Arc<TokioMutex<Vec<Arc<String>>>>,
    weak_refs: Arc<TokioMutex<Vec<Weak<String>>>>,
    event_sender: broadcast::Sender<String>,
}

impl MemoryLeakPrevention {
    fn new() -> Self {
        let (event_sender, _) = broadcast::channel(100);
        Self {
            strong_refs: Arc::new(TokioMutex::new(Vec::new())),
            weak_refs: Arc::new(TokioMutex::new(Vec::new())),
            event_sender,
        }
    }

    // 循环引用检测和预防
    async fn prevent_circular_references(&self) -> Result<()> {
        info!("演示循环引用预防");
        
        // 创建一些强引用
        let data1 = Arc::new("数据1".to_string());
        let data2 = Arc::new("数据2".to_string());
        
        // 使用弱引用打破循环
        let weak_data1 = Arc::downgrade(&data1);
        let weak_data2 = Arc::downgrade(&data2);
        
        {
            let mut strong_refs = self.strong_refs.lock().await;
            strong_refs.push(data1.clone());
            strong_refs.push(data2.clone());
        }
        
        {
            let mut weak_refs = self.weak_refs.lock().await;
            weak_refs.push(weak_data1);
            weak_refs.push(weak_data2);
        }
        
        info!("强引用计数 - data1: {}, data2: {}", 
              Arc::strong_count(&data1), Arc::strong_count(&data2));
        
        // 清理强引用
        {
            let mut strong_refs = self.strong_refs.lock().await;
            strong_refs.clear();
        }
        
        // 检查弱引用是否仍然有效
        {
            let weak_refs = self.weak_refs.lock().await;
            for (i, weak_ref) in weak_refs.iter().enumerate() {
                match weak_ref.upgrade() {
                    Some(data) => info!("弱引用 {} 仍然有效: {}", i, data),
                    None => info!("弱引用 {} 已失效", i),
                }
            }
        }
        
        Ok(())
    }

    // 事件监听器泄漏预防
    async fn prevent_listener_leaks(&self) -> Result<()> {
        info!("演示事件监听器泄漏预防");
        
        let mut receivers = Vec::new();
        
        // 创建多个监听器
        for i in 0..3 {
            let mut receiver = self.event_sender.subscribe();
            let listener_id = i;
            
            let handle = tokio::spawn(async move {
                let mut count = 0;
                while let Ok(event) = receiver.recv().await {
                    count += 1;
                    info!("监听器 {} 收到事件: {} (第{}次)", listener_id, event, count);
                    
                    // 模拟监听器自动退出条件
                    if count >= 2 {
                        info!("监听器 {} 主动退出", listener_id);
                        break;
                    }
                }
            });
            
            receivers.push(handle);
        }
        
        // 发送一些事件
        for i in 0..5 {
            let _ = self.event_sender.send(format!("事件_{}", i));
            sleep(Duration::from_millis(10)).await;
        }
        
        // 等待监听器完成
        for handle in receivers {
            let _ = handle.await;
        }
        
        info!("事件发送器订阅者数量: {}", self.event_sender.receiver_count());
        
        Ok(())
    }

    // 异步任务泄漏检测
    async fn detect_task_leaks(&self) -> Result<()> {
        info!("演示异步任务泄漏检测");
        
        let task_counter = Arc::new(AtomicU64::new(0));
        let mut handles = Vec::new();
        
        // 创建一些可能泄漏的任务
        for i in 0..5 {
            let counter = task_counter.clone();
            let handle = tokio::spawn(async move {
                counter.fetch_add(1, Ordering::Relaxed);
                
                // 模拟一些任务可能永远不会完成
                if i == 2 {
                    // 这个任务会一直运行
                    loop {
                        sleep(Duration::from_millis(100)).await;
                        // 在实际应用中，这里应该有退出条件
                        break; // 为了演示，我们让它退出
                    }
                } else {
                    sleep(Duration::from_millis(50)).await;
                }
                
                counter.fetch_sub(1, Ordering::Relaxed);
                format!("任务 {} 完成", i)
            });
            
            handles.push(handle);
        }
        
        // 等待任务完成或超时
        for (i, handle) in handles.into_iter().enumerate() {
            match timeout(Duration::from_millis(200), handle).await {
                Ok(Ok(result)) => info!("{}", result),
                Ok(Err(e)) => warn!("任务 {} 出错: {}", i, e),
                Err(_) => warn!("任务 {} 超时，可能泄漏", i),
            }
        }
        
        let remaining_tasks = task_counter.load(Ordering::Relaxed);
        if remaining_tasks > 0 {
            warn!("检测到 {} 个可能泄漏的任务", remaining_tasks);
        } else {
            info!("所有任务正常完成，无泄漏");
        }
        
        Ok(())
    }
}

async fn demonstrate_memory_leak_solutions() -> Result<()> {
    info!("=== 内存泄漏疑难问题解决方案 ===");
    
    let leak_prevention = MemoryLeakPrevention::new();
    
    // 测试循环引用预防
    leak_prevention.prevent_circular_references().await?;
    
    // 测试事件监听器泄漏预防
    leak_prevention.prevent_listener_leaks().await?;
    
    // 测试异步任务泄漏检测
    leak_prevention.detect_task_leaks().await?;
    
    Ok(())
}

// ============================================================================
// 14. 异步测试疑难问题解决方案
// ============================================================================

struct AsyncTestFramework {
    test_results: Arc<TokioMutex<Vec<(String, bool, String)>>>,
}

impl AsyncTestFramework {
    fn new() -> Self {
        Self {
            test_results: Arc::new(TokioMutex::new(Vec::new())),
        }
    }

    // 异步测试断言
    async fn async_assert<F, Fut>(&self, test_name: &str, test_fn: F) -> Result<()>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<bool>>,
    {
        let start_time = Instant::now();
        
        match test_fn().await {
            Ok(true) => {
                let duration = start_time.elapsed();
                let message = format!("测试通过 (耗时: {:?})", duration);
                info!("✅ {}: {}", test_name, message);
                
                let mut results = self.test_results.lock().await;
                results.push((test_name.to_string(), true, message));
            }
            Ok(false) => {
                let message = "测试失败：断言为false".to_string();
                warn!("❌ {}: {}", test_name, message);
                
                let mut results = self.test_results.lock().await;
                results.push((test_name.to_string(), false, message));
            }
            Err(e) => {
                let message = format!("测试错误: {}", e);
                error!("💥 {}: {}", test_name, message);
                
                let mut results = self.test_results.lock().await;
                results.push((test_name.to_string(), false, message));
            }
        }
        
        Ok(())
    }

    // 并发测试
    async fn concurrent_test(&self, test_name: &str, concurrent_count: usize) -> Result<()> {
        info!("开始并发测试: {} (并发数: {})", test_name, concurrent_count);
        
        let counter = Arc::new(AtomicU64::new(0));
        let mut handles = Vec::new();
        
        for i in 0..concurrent_count {
            let counter = counter.clone();
            let handle = tokio::spawn(async move {
                // 模拟并发操作
                sleep(Duration::from_millis(10)).await;
                counter.fetch_add(1, Ordering::Relaxed);
                format!("并发任务 {} 完成", i)
            });
            handles.push(handle);
        }
        
        // 等待所有任务完成
        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => return Err(anyhow!("并发任务失败: {}", e)),
            }
        }
        
        let final_count = counter.load(Ordering::Relaxed);
        let success = final_count == concurrent_count as u64;
        
        let message = if success {
            format!("并发测试成功，计数器: {}/{}", final_count, concurrent_count)
        } else {
            format!("并发测试失败，计数器: {}/{}", final_count, concurrent_count)
        };
        
        let mut test_results = self.test_results.lock().await;
        test_results.push((test_name.to_string(), success, message.clone()));
        
        if success {
            info!("✅ {}: {}", test_name, message);
        } else {
            warn!("❌ {}: {}", test_name, message);
        }
        
        Ok(())
    }

    // 超时测试
    async fn timeout_test<F, Fut>(&self, test_name: &str, timeout_duration: Duration, test_fn: F) -> Result<()>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<()>>,
    {
        match timeout(timeout_duration, test_fn()).await {
            Ok(Ok(())) => {
                let message = format!("超时测试通过 (限时: {:?})", timeout_duration);
                info!("✅ {}: {}", test_name, message);
                
                let mut results = self.test_results.lock().await;
                results.push((test_name.to_string(), true, message));
            }
            Ok(Err(e)) => {
                let message = format!("测试函数出错: {}", e);
                warn!("❌ {}: {}", test_name, message);
                
                let mut results = self.test_results.lock().await;
                results.push((test_name.to_string(), false, message));
            }
            Err(_) => {
                let message = format!("测试超时 (限时: {:?})", timeout_duration);
                warn!("⏰ {}: {}", test_name, message);
                
                let mut results = self.test_results.lock().await;
                results.push((test_name.to_string(), false, message));
            }
        }
        
        Ok(())
    }

    // 生成测试报告
    async fn generate_report(&self) -> Result<()> {
        let results = self.test_results.lock().await;
        
        let total_tests = results.len();
        let passed_tests = results.iter().filter(|(_, passed, _)| *passed).count();
        let failed_tests = total_tests - passed_tests;
        
        info!("\n=== 测试报告 ===");
        info!("总测试数: {}", total_tests);
        info!("通过: {} ✅", passed_tests);
        info!("失败: {} ❌", failed_tests);
        info!("成功率: {:.1}%", (passed_tests as f64 / total_tests as f64) * 100.0);
        
        info!("\n详细结果:");
        for (name, passed, message) in results.iter() {
            let status = if *passed { "✅" } else { "❌" };
            info!("  {} {}: {}", status, name, message);
        }
        
        Ok(())
    }
}

async fn demonstrate_async_testing_solutions() -> Result<()> {
    info!("=== 异步测试疑难问题解决方案 ===");
    
    let test_framework = AsyncTestFramework::new();
    
    // 基本异步断言测试
    test_framework.async_assert("基本异步操作测试", || async {
        sleep(Duration::from_millis(10)).await;
        Ok(true)
    }).await?;
    
    test_framework.async_assert("异步计算测试", || async {
        let result = async_computation(5).await?;
        Ok(result == 25)
    }).await?;
    
    // 并发测试
    test_framework.concurrent_test("并发计数器测试", 10).await?;
    
    // 超时测试
    test_framework.timeout_test(
        "快速操作超时测试",
        Duration::from_millis(100),
        || async {
            sleep(Duration::from_millis(50)).await;
            Ok(())
        }
    ).await?;
    
    test_framework.timeout_test(
        "慢速操作超时测试",
        Duration::from_millis(50),
        || async {
            sleep(Duration::from_millis(100)).await;
            Ok(())
        }
    ).await?;
    
    // 生成测试报告
    test_framework.generate_report().await?;
    
    Ok(())
}

// 辅助函数
async fn async_computation(n: i32) -> Result<i32> {
    sleep(Duration::from_millis(1)).await;
    Ok(n * n)
}

// ============================================================================
// 15. 错误传播疑难问题解决方案
// ============================================================================

#[derive(Error, Debug)]
enum ComplexAsyncError {
    #[error("网络连接错误: {source}")]
    NetworkError { source: std::io::Error },
    
    #[error("数据解析错误: {message}")]
    ParseError { message: String },
    
    #[error("业务逻辑错误: {code} - {description}")]
    BusinessError { code: u32, description: String },
    
    #[error("超时错误: 操作耗时 {duration:?}")]
    TimeoutError { duration: Duration },
    
    #[error("聚合错误: {errors:?}")]
    AggregateError { errors: Vec<String> },
}

struct ErrorPropagationManager {
    error_log: Arc<TokioMutex<Vec<(Instant, String)>>>,
}

impl ErrorPropagationManager {
    fn new() -> Self {
        Self {
            error_log: Arc::new(TokioMutex::new(Vec::new())),
        }
    }

    // 错误链式传播
    async fn chain_error_propagation(&self) -> Result<String> {
        self.level_1_operation().await
            .map_err(|e| anyhow!("顶层操作失败").context(e))
    }

    async fn level_1_operation(&self) -> Result<String> {
        self.level_2_operation().await
            .map_err(|e| anyhow!("第一层操作失败").context(e))
    }

    async fn level_2_operation(&self) -> Result<String> {
        self.level_3_operation().await
            .map_err(|e| anyhow!("第二层操作失败").context(e))
    }

    async fn level_3_operation(&self) -> Result<String> {
        // 模拟底层错误
        Err(ComplexAsyncError::BusinessError {
            code: 1001,
            description: "数据验证失败".to_string(),
        }.into())
    }

    // 并行错误收集
    async fn parallel_error_collection(&self) -> Result<Vec<String>> {
        let tasks = vec![
            self.potentially_failing_task("任务A", true),
            self.potentially_failing_task("任务B", false),
            self.potentially_failing_task("任务C", true),
            self.potentially_failing_task("任务D", false),
        ];

        let results = futures::future::join_all(tasks).await;
        let mut successes = Vec::new();
        let mut errors = Vec::new();

        for (i, result) in results.into_iter().enumerate() {
            match result {
                Ok(value) => successes.push(value),
                Err(e) => {
                    let error_msg = format!("任务{} 失败: {}", i, e);
                    errors.push(error_msg.clone());
                    self.log_error(&error_msg).await;
                }
            }
        }

        if !errors.is_empty() {
            return Err(ComplexAsyncError::AggregateError { errors }.into());
        }

        Ok(successes)
    }

    async fn potentially_failing_task(&self, task_name: &str, should_succeed: bool) -> Result<String> {
        sleep(Duration::from_millis(10)).await;
        
        if should_succeed {
            Ok(format!("{} 成功完成", task_name))
        } else {
            Err(ComplexAsyncError::NetworkError {
                source: std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "连接被拒绝")
            }.into())
        }
    }

    // 错误恢复策略
    async fn error_recovery_strategy(&self, max_retries: u32) -> Result<String> {
        let mut last_error = None;
        
        for attempt in 1..=max_retries {
            match self.unreliable_operation(attempt).await {
                Ok(result) => {
                    info!("操作在第 {} 次尝试后成功", attempt);
                    return Ok(result);
                }
                Err(e) => {
                    warn!("第 {} 次尝试失败: {}", attempt, e);
                    last_error = Some(e);
                    
                    if attempt < max_retries {
                        // 指数退避
                        let delay = Duration::from_millis(100 * 2_u64.pow(attempt - 1));
                        sleep(delay).await;
                    }
                }
            }
        }
        
        Err(anyhow!("所有重试都失败了，最后错误: {:?}", last_error))
    }

    async fn unreliable_operation(&self, attempt: u32) -> Result<String> {
        // 模拟不稳定的操作，前两次失败，第三次成功
        if attempt < 3 {
            Err(ComplexAsyncError::TimeoutError {
                duration: Duration::from_millis(100 * attempt as u64)
            }.into())
        } else {
            Ok("操作最终成功".to_string())
        }
    }

    // 错误日志记录
    async fn log_error(&self, error_message: &str) {
        let mut log = self.error_log.lock().await;
        log.push((Instant::now(), error_message.to_string()));
    }

    // 错误统计报告
    async fn generate_error_report(&self) -> Result<()> {
        let log = self.error_log.lock().await;
        
        info!("\n=== 错误统计报告 ===");
        info!("总错误数: {}", log.len());
        
        if !log.is_empty() {
            info!("错误详情:");
            for (i, (timestamp, message)) in log.iter().enumerate() {
                info!("  {}. [{:?}] {}", i + 1, timestamp, message);
            }
        }
        
        Ok(())
    }
}

async fn demonstrate_error_propagation_solutions() -> Result<()> {
    info!("=== 错误传播疑难问题解决方案 ===");
    
    let error_manager = ErrorPropagationManager::new();
    
    // 测试错误链式传播
    info!("\n--- 错误链式传播测试 ---");
    match error_manager.chain_error_propagation().await {
        Ok(result) => info!("链式操作成功: {}", result),
        Err(e) => {
            error!("链式操作失败: {}", e);
            
            // 打印错误链
            let mut current_error: &dyn std::error::Error = &*e;
            let mut level = 0;
            while let Some(source) = current_error.source() {
                error!("  错误层级 {}: {}", level, current_error);
                current_error = source;
                level += 1;
            }
            error!("  根本原因: {}", current_error);
        }
    }
    
    // 测试并行错误收集
    info!("\n--- 并行错误收集测试 ---");
    match error_manager.parallel_error_collection().await {
        Ok(results) => info!("并行操作成功: {:?}", results),
        Err(e) => error!("并行操作失败: {}", e),
    }
    
    // 测试错误恢复策略
    info!("\n--- 错误恢复策略测试 ---");
    match error_manager.error_recovery_strategy(5).await {
        Ok(result) => info!("恢复策略成功: {}", result),
        Err(e) => error!("恢复策略失败: {}", e),
    }
    
    // 生成错误报告
    error_manager.generate_error_report().await?;
    
    Ok(())
}
