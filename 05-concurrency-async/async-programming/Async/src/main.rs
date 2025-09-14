//! # Rust 异步编程全面分析
//! 
//! 本项目基于 https://course.rs/advance/async/getting-started.html 的内容，
//! 全面深入地分析 Rust 异步编程的各个方面，包括：
//! - async/await 语法基础
//! - Future trait 和状态机
//! - 异步运行时和任务调度
//! - 异步 I/O 和并发模式
//! - 实际应用案例和性能优化

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

// 引入运行时演示模块
mod runtime_demo;

// ============================================================================
// 第一部分：异步编程基础
// ============================================================================

/// 演示同步 vs 异步的区别
fn sync_vs_async_demo() {
    println!("\n=== 同步 vs 异步编程对比 ===");
    
    // 同步版本：阻塞执行
    fn sync_operation() -> String {
        println!("同步操作开始...");
        thread::sleep(Duration::from_millis(100)); // 模拟耗时操作
        println!("同步操作完成");
        "同步结果".to_string()
    }
    
    // 异步版本：非阻塞执行
    async fn async_operation() -> String {
        println!("异步操作开始...");
        async_sleep(Duration::from_millis(100)).await; // 模拟异步等待
        println!("异步操作完成");
        "异步结果".to_string()
    }
    
    println!("\n--- 同步执行 ---");
    let start = Instant::now();
    let result1 = sync_operation();
    let result2 = sync_operation();
    println!("同步结果: {}, {}", result1, result2);
    println!("同步总耗时: {:?}", start.elapsed());
    
    println!("\n--- 异步执行 ---");
    let start = Instant::now();
    // 注意：这里只是演示概念，实际需要运行时支持
    println!("异步操作将并发执行（需要异步运行时支持）");
    println!("预期异步总耗时: ~100ms（并发执行）");
}

/// 简单的异步睡眠实现（用于演示）
async fn async_sleep(duration: Duration) {
    let start = Instant::now();
    while start.elapsed() < duration {
        // 在实际应用中，这里会让出控制权给调度器
        // 这里只是简单的忙等待演示
        if start.elapsed().as_millis() % 10 == 0 {
            // 模拟让出控制权
        }
    }
}

// ============================================================================
// 第二部分：Future trait 深入理解
// ============================================================================

/// 自定义 Future 实现：延时器
struct DelayFuture {
    when: Instant,
}

impl DelayFuture {
    fn new(duration: Duration) -> Self {
        DelayFuture {
            when: Instant::now() + duration,
        }
    }
}

impl Future for DelayFuture {
    type Output = ();
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("DelayFuture 完成！");
            Poll::Ready(())
        } else {
            println!("DelayFuture 还未就绪，注册 waker");
            // 在实际实现中，这里会注册 waker 以便在适当时机唤醒
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// 自定义 Future：计数器
struct CounterFuture {
    count: usize,
    max: usize,
}

impl CounterFuture {
    fn new(max: usize) -> Self {
        CounterFuture { count: 0, max }
    }
}

impl Future for CounterFuture {
    type Output = usize;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count >= self.max {
            Poll::Ready(self.count)
        } else {
            self.count += 1;
            println!("计数器: {}/{}", self.count, self.max);
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// 演示 Future trait 的工作原理
fn future_trait_demo() {
    println!("\n=== Future Trait 工作原理 ===");
    
    println!("\n--- 自定义 Future 示例 ---");
    println!("DelayFuture 和 CounterFuture 已定义");
    println!("这些 Future 展示了状态机的概念：");
    println!("- Poll::Pending: Future 还未完成，需要稍后再次轮询");
    println!("- Poll::Ready(value): Future 已完成，返回结果");
    println!("- Waker: 用于通知执行器 Future 可能已就绪");
    
    // 在实际应用中，这些 Future 会被异步运行时执行
    println!("\n注意：这些 Future 需要异步运行时来实际执行");
}

// ============================================================================
// 第三部分：async/await 语法糖
// ============================================================================

/// 演示 async/await 语法
async fn async_syntax_demo() {
    println!("\n=== async/await 语法演示 ===");
    
    // async 函数自动返回 Future
    async fn simple_async_fn() -> i32 {
        println!("执行异步函数");
        42
    }
    
    // await 等待 Future 完成
    let result = simple_async_fn().await;
    println!("异步函数结果: {}", result);
    
    // 链式 await
    async fn chain_example() -> String {
        let step1 = async { "步骤1" }.await;
        let step2 = async { "步骤2" }.await;
        format!("{} -> {}", step1, step2)
    }
    
    let chain_result = chain_example().await;
    println!("链式执行结果: {}", chain_result);
}

/// 演示错误处理
async fn error_handling_demo() {
    println!("\n=== 异步错误处理 ===");
    
    async fn may_fail(should_fail: bool) -> Result<String, &'static str> {
        if should_fail {
            Err("操作失败")
        } else {
            Ok("操作成功".to_string())
        }
    }
    
    // 使用 ? 操作符处理错误
    async fn handle_errors() -> Result<String, &'static str> {
        let result1 = may_fail(false).await?;
        let result2 = may_fail(false).await?;
        Ok(format!("{} 和 {}", result1, result2))
    }
    
    match handle_errors().await {
        Ok(msg) => println!("成功: {}", msg),
        Err(e) => println!("错误: {}", e),
    }
}

// ============================================================================
// 第四部分：并发和组合
// ============================================================================

/// 演示并发执行模式
async fn concurrency_demo() {
    println!("\n=== 并发执行模式 ===");
    
    async fn task(id: u32, duration_ms: u64) -> String {
        println!("任务 {} 开始", id);
        // 模拟异步工作
        let start = Instant::now();
        while start.elapsed().as_millis() < duration_ms as u128 {
            // 模拟异步等待
        }
        println!("任务 {} 完成", id);
        format!("任务 {} 结果", id)
    }
    
    println!("\n--- 顺序执行 ---");
    let start = Instant::now();
    let result1 = task(1, 50).await;
    let result2 = task(2, 50).await;
    println!("顺序执行结果: {}, {}", result1, result2);
    println!("顺序执行耗时: {:?}", start.elapsed());
    
    println!("\n--- 并发执行（概念演示）---");
    println!("在实际的异步运行时中，可以使用 join! 或 select! 实现真正的并发");
    println!("例如: tokio::join!(task1, task2) 或 futures::join!(task1, task2)");
}

/// 演示超时和取消
async fn timeout_demo() {
    println!("\n=== 超时和取消机制 ===");
    
    async fn long_running_task() -> String {
        println!("长时间运行的任务开始...");
        // 模拟长时间运行
        let start = Instant::now();
        while start.elapsed().as_millis() < 200 {
            // 模拟工作
        }
        "长任务完成".to_string()
    }
    
    println!("长时间任务演示（实际需要运行时支持超时）");
    println!("在 tokio 中可以使用: tokio::time::timeout(duration, future)");
    println!("在 async-std 中可以使用: async_std::future::timeout(duration, future)");
    
    let result = long_running_task().await;
    println!("任务结果: {}", result);
}

// ============================================================================
// 第五部分：异步 I/O 模拟
// ============================================================================

/// 模拟异步文件操作
struct AsyncFile {
    content: String,
}

impl AsyncFile {
    async fn read(filename: &str) -> Result<String, &'static str> {
        println!("异步读取文件: {}", filename);
        // 模拟异步 I/O 延迟
        async_sleep(Duration::from_millis(10)).await;
        
        match filename {
            "test.txt" => Ok("文件内容示例".to_string()),
            "data.json" => Ok(r#"{"key": "value"}"#.to_string()),
            _ => Err("文件未找到"),
        }
    }
    
    async fn write(filename: &str, content: &str) -> Result<(), &'static str> {
        println!("异步写入文件: {} (内容长度: {})", filename, content.len());
        // 模拟异步 I/O 延迟
        async_sleep(Duration::from_millis(15)).await;
        Ok(())
    }
}

/// 演示异步 I/O 操作
async fn async_io_demo() {
    println!("\n=== 异步 I/O 操作演示 ===");
    
    // 并发读取多个文件
    println!("\n--- 并发文件操作 ---");
    let file1_future = AsyncFile::read("test.txt");
    let file2_future = AsyncFile::read("data.json");
    
    // 在实际应用中，这里会使用 join! 或类似的并发原语
    let content1 = file1_future.await;
    let content2 = file2_future.await;
    
    match (content1, content2) {
        (Ok(c1), Ok(c2)) => {
            println!("文件1内容: {}", c1);
            println!("文件2内容: {}", c2);
        }
        _ => println!("读取文件时发生错误"),
    }
    
    // 写入文件
    if let Err(e) = AsyncFile::write("output.txt", "异步写入的内容").await {
        println!("写入失败: {}", e);
    } else {
        println!("文件写入成功");
    }
}

// ============================================================================
// 第六部分：异步同步原语
// ============================================================================

/// 模拟异步 Mutex
struct AsyncMutex<T> {
    data: Arc<Mutex<T>>,
}

impl<T> AsyncMutex<T> {
    fn new(data: T) -> Self {
        AsyncMutex {
            data: Arc::new(Mutex::new(data)),
        }
    }
    
    async fn lock(&self) -> Result<std::sync::MutexGuard<T>, &'static str> {
        // 在实际的异步 Mutex 中，这里会是非阻塞的
        match self.data.try_lock() {
            Ok(guard) => Ok(guard),
            Err(_) => {
                // 模拟等待锁释放
                async_sleep(Duration::from_millis(1)).await;
                self.data.try_lock().map_err(|_| "锁获取失败")
            }
        }
    }
}

/// 演示异步同步
async fn async_sync_demo() {
    println!("\n=== 异步同步原语演示 ===");
    
    let shared_counter = AsyncMutex::new(0);
    
    async fn increment_counter(mutex: &AsyncMutex<i32>, id: u32) {
        for i in 0..3 {
            if let Ok(mut guard) = mutex.lock().await {
                *guard += 1;
                println!("任务 {} 第 {} 次增加计数器: {}", id, i + 1, *guard);
            }
            // 模拟一些工作
            async_sleep(Duration::from_millis(5)).await;
        }
    }
    
    println!("\n--- 共享状态访问 ---");
    // 在实际应用中，这里会并发执行
    increment_counter(&shared_counter, 1).await;
    increment_counter(&shared_counter, 2).await;
    
    if let Ok(final_value) = shared_counter.lock().await {
        println!("最终计数器值: {}", *final_value);
    }
}

// ============================================================================
// 第七部分：实际应用案例
// ============================================================================

/// 模拟 HTTP 客户端
struct AsyncHttpClient;

impl AsyncHttpClient {
    async fn get(url: &str) -> Result<String, &'static str> {
        println!("发送 GET 请求到: {}", url);
        // 模拟网络延迟
        async_sleep(Duration::from_millis(50)).await;
        
        match url {
            "https://api.example.com/users" => {
                Ok(r#"[{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]"#.to_string())
            }
            "https://api.example.com/posts" => {
                Ok(r#"[{"id": 1, "title": "Hello World"}]"#.to_string())
            }
            _ => Err("404 Not Found"),
        }
    }
    
    async fn post(url: &str, data: &str) -> Result<String, &'static str> {
        println!("发送 POST 请求到: {} (数据: {})", url, data);
        async_sleep(Duration::from_millis(75)).await;
        Ok("创建成功".to_string())
    }
}

/// 数据聚合服务
struct DataAggregator;

impl DataAggregator {
    async fn fetch_user_data(user_id: u32) -> Result<HashMap<String, String>, &'static str> {
        println!("获取用户 {} 的数据", user_id);
        
        // 并发获取用户的不同数据
        let profile_url = format!("https://api.example.com/users/{}", user_id);
        let posts_url = format!("https://api.example.com/users/{}/posts", user_id);
        let profile_future = AsyncHttpClient::get(&profile_url);
        let posts_future = AsyncHttpClient::get(&posts_url);
        
        // 在实际应用中使用 join! 并发执行
        let profile = profile_future.await?;
        let posts = posts_future.await?;
        
        let mut result = HashMap::new();
        result.insert("profile".to_string(), profile);
        result.insert("posts".to_string(), posts);
        
        Ok(result)
    }
}

/// 演示实际应用案例
async fn real_world_demo() {
    println!("\n=== 实际应用案例演示 ===");
    
    println!("\n--- HTTP 客户端 ---");
    match AsyncHttpClient::get("https://api.example.com/users").await {
        Ok(response) => println!("用户数据: {}", response),
        Err(e) => println!("请求失败: {}", e),
    }
    
    println!("\n--- 数据聚合 ---");
    match DataAggregator::fetch_user_data(1).await {
        Ok(data) => {
            println!("聚合数据获取成功:");
            for (key, value) in data {
                println!("  {}: {}", key, value);
            }
        }
        Err(e) => println!("数据聚合失败: {}", e),
    }
    
    println!("\n--- 批量处理 ---");
    let user_ids = vec![1, 2, 3];
    for user_id in user_ids {
        // 在实际应用中，这些请求会并发执行
        if let Ok(_) = DataAggregator::fetch_user_data(user_id).await {
            println!("用户 {} 数据处理完成", user_id);
        }
    }
}

// ============================================================================
// 第八部分：性能和最佳实践
// ============================================================================

/// 性能测量工具
struct AsyncBenchmark;

impl AsyncBenchmark {
    async fn measure<F, Fut, T>(name: &str, f: F) -> T
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = T>,
    {
        let start = Instant::now();
        let result = f().await;
        let duration = start.elapsed();
        println!("[BENCHMARK] {}: {:?}", name, duration);
        result
    }
}

/// 演示性能优化
async fn performance_demo() {
    println!("\n=== 性能优化演示 ===");
    
    // CPU 密集型任务（应该在单独线程中运行）
    async fn cpu_intensive_task(n: u64) -> u64 {
        println!("执行 CPU 密集型任务 (n={})", n);
        // 注意：这种计算应该在 spawn_blocking 中执行
        let mut sum = 0;
        for i in 0..n {
            sum += i;
        }
        sum
    }
    
    // I/O 密集型任务（适合异步）
    async fn io_intensive_task(count: u32) -> Vec<String> {
        println!("执行 I/O 密集型任务 (count={})", count);
        let mut results = Vec::new();
        for i in 0..count {
            // 模拟异步 I/O
            async_sleep(Duration::from_millis(1)).await;
            results.push(format!("结果 {}", i));
        }
        results
    }
    
    println!("\n--- 性能测量 ---");
    
    let cpu_result = AsyncBenchmark::measure("CPU 密集型任务", || {
        cpu_intensive_task(1000)
    }).await;
    println!("CPU 任务结果: {}", cpu_result);
    
    let io_results = AsyncBenchmark::measure("I/O 密集型任务", || {
        io_intensive_task(5)
    }).await;
    println!("I/O 任务结果数量: {}", io_results.len());
    
    println!("\n--- 最佳实践建议 ---");
    println!("1. CPU 密集型任务使用 spawn_blocking");
    println!("2. I/O 密集型任务使用 async/await");
    println!("3. 避免在异步函数中使用阻塞操作");
    println!("4. 合理使用并发，避免过度创建任务");
    println!("5. 使用适当的缓冲区大小");
}

// ============================================================================
// 第九部分：常见陷阱和解决方案
// ============================================================================

/// 演示常见问题和解决方案
async fn common_pitfalls_demo() {
    println!("\n=== 常见陷阱和解决方案 ===");
    
    println!("\n--- 1. 阻塞操作陷阱 ---");
    println!("❌ 错误：在异步函数中使用 std::thread::sleep");
    println!("✅ 正确：使用异步睡眠函数");
    
    println!("\n--- 2. 生命周期问题 ---");
    async fn lifetime_example() {
        let data = "临时数据".to_string();
        // 确保数据在异步操作期间有效
        async_operation_with_data(&data).await;
    }
    
    async fn async_operation_with_data(data: &str) {
        println!("处理数据: {}", data);
        async_sleep(Duration::from_millis(1)).await;
    }
    
    lifetime_example().await;
    
    println!("\n--- 3. 错误传播 ---");
    async fn error_propagation_example() -> Result<String, &'static str> {
        let result1 = async { Ok::<_, &'static str>("步骤1".to_string()) }.await?;
        let result2 = async { Ok::<_, &'static str>("步骤2".to_string()) }.await?;
        Ok(format!("{} -> {}", result1, result2))
    }
    
    match error_propagation_example().await {
        Ok(result) => println!("错误传播示例成功: {}", result),
        Err(e) => println!("错误传播示例失败: {}", e),
    }
    
    println!("\n--- 4. 资源管理 ---");
    println!("使用 RAII 模式管理资源");
    println!("在 Drop trait 中清理异步资源");
}

// ============================================================================
// 测试模块
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_delay_future_creation() {
        let delay = DelayFuture::new(Duration::from_millis(100));
        // 测试 Future 创建
        assert!(delay.when > Instant::now());
    }
    
    #[test]
    fn test_counter_future_creation() {
        let counter = CounterFuture::new(5);
        assert_eq!(counter.count, 0);
        assert_eq!(counter.max, 5);
    }
    
    #[test]
    fn test_async_mutex_creation() {
        let mutex = AsyncMutex::new(42);
        // 测试异步 Mutex 创建
        assert!(mutex.data.try_lock().is_ok());
    }
}

// ============================================================================
// 主函数：运行所有演示
// ============================================================================

/// 简单的异步运行时模拟（仅用于演示）
struct SimpleExecutor;

impl SimpleExecutor {
    fn block_on<F: Future>(future: F) -> F::Output {
        // 这是一个极简的执行器实现，仅用于演示
        // 实际应用中应该使用 tokio、async-std 等成熟的运行时
        
        // 创建一个简单的 waker
        use std::task::{RawWaker, RawWakerVTable};
        
        fn clone_raw(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &VTABLE)
        }
        
        fn wake_raw(_: *const ()) {}
        fn wake_by_ref_raw(_: *const ()) {}
        fn drop_raw(_: *const ()) {}
        
        const VTABLE: RawWakerVTable = RawWakerVTable::new(
            clone_raw,
            wake_raw,
            wake_by_ref_raw,
            drop_raw,
        );
        
        let raw_waker = RawWaker::new(std::ptr::null(), &VTABLE);
        let waker = unsafe { Waker::from_raw(raw_waker) };
        let mut context = Context::from_waker(&waker);
        
        let mut future = Box::pin(future);
        
        // 简单的轮询循环
        loop {
            match future.as_mut().poll(&mut context) {
                Poll::Ready(result) => return result,
                Poll::Pending => {
                    // 在实际运行时中，这里会等待事件或定时器
                    thread::sleep(Duration::from_millis(1));
                }
            }
        }
    }
}

fn main() {
    println!("🚀 Rust 异步编程全面分析");
    println!("==================================");
    
    // 同步部分的演示
    sync_vs_async_demo();
    future_trait_demo();
    
    println!("\n=== 使用简单执行器运行异步代码 ===");
    
    // 使用简单执行器运行异步代码
    SimpleExecutor::block_on(async {
        async_syntax_demo().await;
        error_handling_demo().await;
        concurrency_demo().await;
        timeout_demo().await;
        async_io_demo().await;
        async_sync_demo().await;
        real_world_demo().await;
        performance_demo().await;
        common_pitfalls_demo().await;
        
        // 运行运行时深度分析
        runtime_demo::run_runtime_demos().await;
    });
    
    println!("\n=== 总结 ===");
    println!("本项目全面展示了 Rust 异步编程的各个方面：");
    println!("✅ async/await 语法和 Future trait");
    println!("✅ 异步运行时和任务调度");
    println!("✅ 并发模式和错误处理");
    println!("✅ 异步 I/O 和同步原语");
    println!("✅ 实际应用案例和性能优化");
    println!("✅ 常见陷阱和最佳实践");
    println!("\n🎯 通过这些示例，你应该对 Rust 异步编程有了全面深入的理解！");
    println!("\n💡 建议：在实际项目中使用 tokio 或 async-std 等成熟的异步运行时");
}
