//! # 同时运行多个 Future - 深度分析与实践
//! 
//! 本项目全面分析 Rust 异步编程中同时运行多个 Future 的各种方式和模式
//! 涵盖 join!、try_join!、select!、spawn 等核心概念的深入解析

use futures::{
    future::{join_all, try_join_all},
    stream::{FuturesUnordered, StreamExt},
    FutureExt,
};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{
    sync::{mpsc, Semaphore},
    task::JoinHandle,
    time::sleep,
};
use anyhow::{anyhow, Result};
use rand::Rng;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Rust 异步编程 - 多 Future 并发执行深度分析\n");
    
    // 1. join! 宏详解
    println!("📋 1. join! 宏详解");
    demonstrate_join_macro().await?;
    
    // 2. try_join! 宏分析
    println!("\n📋 2. try_join! 宏分析");
    demonstrate_try_join_macro().await?;
    
    // 3. select! 宏深入
    println!("\n📋 3. select! 宏深入");
    demonstrate_select_macro().await?;
    
    // 4. spawn 任务创建
    println!("\n📋 4. spawn 任务创建");
    demonstrate_spawn_tasks().await?;
    
    // 5. 并发模式分析
    println!("\n📋 5. 并发模式分析");
    demonstrate_concurrent_patterns().await?;
    
    // 6. 错误处理策略
    println!("\n📋 6. 错误处理策略");
    demonstrate_error_handling().await?;
    
    // 7. 性能优化技巧
    println!("\n📋 7. 性能优化技巧");
    demonstrate_performance_optimization().await?;
    
    // 8. 实际应用案例
    println!("\n📋 8. 实际应用案例");
    demonstrate_real_world_examples().await?;
    
    println!("\n✅ 所有演示完成！");
    Ok(())
}

/// 1. join! 宏详解
async fn demonstrate_join_macro() -> Result<()> {
    println!("\n🔍 1.1 基础 join! 用法");
    
    // 创建三个异步任务
    let task1 = async_task("任务1".to_string(), 100);
    let task2 = async_task("任务2".to_string(), 150);
    let task3 = async_task("任务3".to_string(), 80);
    
    let start = Instant::now();
    let (result1, result2, result3) = futures::join!(task1, task2, task3);
    let elapsed = start.elapsed();
    
    println!("      ✅ {}", result1);
    println!("      ✅ {}", result2);
    println!("      ✅ {}", result3);
    println!("      ⏱️ 总耗时: {:?} (应该约等于最长任务时间)", elapsed);
    
    println!("\n🔍 1.2 join_all 批量处理");
    
    let tasks: Vec<_> = (1..=5)
        .map(|i| async_task(format!("批量任务{}", i), 50 + i * 20))
        .collect();
    
    let start = Instant::now();
    let results = join_all(tasks).await;
    let elapsed = start.elapsed();
    
    for result in results {
        println!("      ✅ {}", result);
    }
    println!("      ⏱️ 批量处理耗时: {:?}", elapsed);
    
    Ok(())
}

/// 2. try_join! 宏分析
async fn demonstrate_try_join_macro() -> Result<()> {
    println!("\n🔍 2.1 成功场景的 try_join!");
    
    let task1 = fallible_task("可靠任务1".to_string(), 100, false);
     let task2 = fallible_task("可靠任务2".to_string(), 120, false);
     let task3 = fallible_task("可靠任务3".to_string(), 80, false);
    
    match futures::try_join!(task1, task2, task3) {
        Ok((result1, result2, result3)) => {
            println!("      ✅ {}", result1);
            println!("      ✅ {}", result2);
            println!("      ✅ {}", result3);
        }
        Err(e) => println!("      ❌ 错误: {}", e),
    }
    
    println!("\n🔍 2.2 失败场景的 try_join!");
    
    let task1 = fallible_task("任务1".to_string(), 100, false);
     let task2 = fallible_task("失败任务2".to_string(), 120, true); // 这个会失败
     let task3 = fallible_task("任务3".to_string(), 200, false);
    
    let start = Instant::now();
    match futures::try_join!(task1, task2, task3) {
        Ok((result1, result2, result3)) => {
            println!("      ✅ 全部成功: {}, {}, {}", result1, result2, result3);
        }
        Err(e) => {
            let elapsed = start.elapsed();
            println!("      ❌ 快速失败: {} (耗时: {:?})", e, elapsed);
            println!("      📝 注意: 一旦有任务失败，其他任务也会被取消");
        }
    }
    
    println!("\n🔍 2.3 try_join_all 批量错误处理");
    
    let tasks: Vec<_> = (1..=5)
         .map(|i| fallible_task(format!("批量任务{}", i), 50, i == 3)) // 第3个任务失败
         .collect();
    
    match try_join_all(tasks).await {
        Ok(results) => {
            println!("      ✅ 全部成功:");
            for result in results {
                println!("        - {}", result);
            }
        }
        Err(e) => {
            println!("      ❌ 批量处理失败: {}", e);
        }
    }
    
    Ok(())
}

/// 3. select! 宏深入
async fn demonstrate_select_macro() -> Result<()> {
    println!("\n🔍 3.1 基础 select! 竞争");
    
    let task1 = async_task("快速任务".to_string(), 50);
    let task2 = async_task("慢速任务".to_string(), 200);
    
    let start = Instant::now();
    tokio::select! {
        result = task1 => {
            let elapsed = start.elapsed();
            println!("      🏆 快速任务获胜: {} (耗时: {:?})", result, elapsed);
        }
        result = task2 => {
            let elapsed = start.elapsed();
            println!("      🏆 慢速任务获胜: {} (耗时: {:?})", result, elapsed);
        }
    }
    
    println!("\n🔍 3.2 带超时的 select!");
    
    let long_task = async_task("长时间任务".to_string(), 300);
    let timeout = sleep(Duration::from_millis(150));
    
    let start = Instant::now();
    tokio::select! {
        result = long_task => {
            let elapsed = start.elapsed();
            println!("      ✅ 任务完成: {} (耗时: {:?})", result, elapsed);
        }
        _ = timeout => {
            let elapsed = start.elapsed();
            println!("      ⏰ 任务超时 (耗时: {:?})", elapsed);
        }
    }
    
    println!("\n🔍 3.3 多路 select! 与消息处理");
    
    let (tx1, mut rx1) = mpsc::channel::<String>(10);
    let (tx2, mut rx2) = mpsc::channel::<i32>(10);
    
    // 发送一些测试消息
    tokio::spawn(async move {
        sleep(Duration::from_millis(50)).await;
        let _ = tx1.send("Hello".to_string()).await;
        sleep(Duration::from_millis(30)).await;
        let _ = tx2.send(42).await;
        sleep(Duration::from_millis(20)).await;
        let _ = tx1.send("World".to_string()).await;
    });
    
    let mut message_count = 0;
    let start = Instant::now();
    
    while message_count < 3 {
        tokio::select! {
            Some(msg) = rx1.recv() => {
                println!("      📨 收到字符串消息: {}", msg);
                message_count += 1;
            }
            Some(num) = rx2.recv() => {
                println!("      📨 收到数字消息: {}", num);
                message_count += 1;
            }
            _ = sleep(Duration::from_millis(200)) => {
                println!("      ⏰ 消息接收超时");
                break;
            }
        }
    }
    
    let elapsed = start.elapsed();
    println!("      📊 消息处理完成，总耗时: {:?}", elapsed);
    
    Ok(())
}

/// 4. spawn 任务创建
async fn demonstrate_spawn_tasks() -> Result<()> {
    println!("\n🔍 4.1 基础任务生成");
    
    let mut handles: Vec<JoinHandle<String>> = Vec::new();
    
    // 生成多个并发任务
    for i in 1..=5 {
        let handle = tokio::spawn(async move {
            let delay = 50 + (i * 20);
            sleep(Duration::from_millis(delay)).await;
            format!("Spawn任务{} 完成", i)
        });
        handles.push(handle);
    }
    
    let start = Instant::now();
    
    // 等待所有任务完成
    for handle in handles {
        match handle.await {
            Ok(result) => println!("      ✅ {}", result),
            Err(e) => println!("      ❌ 任务失败: {}", e),
        }
    }
    
    let elapsed = start.elapsed();
    println!("      ⏱️ 所有spawn任务完成，耗时: {:?}", elapsed);
    
    println!("\n🔍 4.2 任务取消机制");
    
    let handle = tokio::spawn(async {
        for i in 1..=10 {
            println!("      🔄 长任务执行中... {}/10", i);
            sleep(Duration::from_millis(100)).await;
        }
        "长任务完成"
    });
    
    // 让任务运行一段时间后取消
    sleep(Duration::from_millis(350)).await;
    handle.abort();
    
    match handle.await {
        Ok(result) => println!("      ✅ 任务完成: {}", result),
        Err(e) if e.is_cancelled() => println!("      🛑 任务被取消"),
        Err(e) => println!("      ❌ 任务错误: {}", e),
    }
    
    Ok(())
}

/// 5. 并发模式分析
async fn demonstrate_concurrent_patterns() -> Result<()> {
    println!("\n🔍 5.1 扇出扇入模式");
    
    // 扇出：将工作分发给多个worker
    let work_items = vec!["数据1", "数据2", "数据3", "数据4", "数据5"];
    let mut tasks = Vec::new();
    
    for (i, item) in work_items.into_iter().enumerate() {
        let task = tokio::spawn(async move {
            let processing_time = 80 + (i * 20) as u64;
            sleep(Duration::from_millis(processing_time)).await;
            format!("处理结果: {}", item)
        });
        tasks.push(task);
    }
    
    // 扇入：收集所有结果
    let mut results = Vec::new();
    for task in tasks {
        results.push(task.await?);
    }
    
    println!("      📊 扇出扇入结果:");
    for result in results {
        println!("        - {}", result);
    }
    
    println!("\n🔍 5.2 工作池模式");
    
    let semaphore = Arc::new(Semaphore::new(3)); // 限制并发数为3
    let mut tasks = Vec::new();
    
    for i in 1..=8 {
        let semaphore = Arc::clone(&semaphore);
        let task = tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            println!("      🔧 工作者开始处理任务{}", i);
            sleep(Duration::from_millis(100)).await;
            println!("      ✅ 工作者完成任务{}", i);
            format!("任务{}结果", i)
        });
        tasks.push(task);
    }
    
    let results = join_all(tasks).await;
    let success_count = results.iter().filter(|r| r.is_ok()).count();
    println!("      📊 工作池完成: {}/{} 任务成功", success_count, results.len());
    
    println!("\n🔍 5.3 管道模式");
    
    let (tx1, mut rx1) = mpsc::channel::<i32>(10);
    let (tx2, mut rx2) = mpsc::channel::<String>(10);
    
    // 阶段1：数据生成
    let producer = tokio::spawn(async move {
        for i in 1..=5 {
            let _ = tx1.send(i).await;
            sleep(Duration::from_millis(50)).await;
        }
    });
    
    // 阶段2：数据处理
    let processor = tokio::spawn(async move {
        while let Some(num) = rx1.recv().await {
            let processed = format!("处理后的数据: {}", num * 2);
            let _ = tx2.send(processed).await;
        }
    });
    
    // 阶段3：结果收集
    let consumer = tokio::spawn(async move {
        let mut results = Vec::new();
        while let Some(result) = rx2.recv().await {
            results.push(result);
            if results.len() >= 5 {
                break;
            }
        }
        results
    });
    
    let (_, _, final_results) = futures::join!(producer, processor, consumer);
    
    println!("      📊 管道处理结果:");
    for result in final_results? {
        println!("        - {}", result);
    }
    
    Ok(())
}

/// 6. 错误处理策略
async fn demonstrate_error_handling() -> Result<()> {
    println!("\n🔍 6.1 部分失败处理");
    
    let tasks: Vec<_> = (1..=5)
        .map(|i| {
             tokio::spawn(async move {
                 fallible_task(format!("任务{}", i), 50, i == 3).await
             })
         })
        .collect();
    
    let results = join_all(tasks).await;
    
    let mut success_count = 0;
    let mut error_count = 0;
    
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(Ok(msg)) => {
                success_count += 1;
                println!("      ✅ 任务{}: {}", i + 1, msg);
            }
            Ok(Err(e)) => {
                error_count += 1;
                println!("      ❌ 任务{}: {}", i + 1, e);
            }
            Err(e) => {
                error_count += 1;
                println!("      💥 任务{}崩溃: {}", i + 1, e);
            }
        }
    }
    
    println!("      📊 处理统计: 成功 {}, 失败 {}", success_count, error_count);
    
    println!("\n🔍 6.2 超时控制");
    
    let tasks_with_timeout: Vec<_> = (1..=3)
        .map(|i| {
            let delay = i * 100;
            tokio::spawn(async move {
                tokio::time::timeout(
                    Duration::from_millis(150),
                    async_task(format!("超时任务{}", i), delay)
                ).await
            })
        })
        .collect();
    
    let results = join_all(tasks_with_timeout).await;
    
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(Ok(msg)) => println!("      ✅ 任务{}: {}", i + 1, msg),
            Ok(Err(_)) => println!("      ⏰ 任务{}超时", i + 1),
            Err(e) => println!("      💥 任务{}错误: {}", i + 1, e),
        }
    }
    
    Ok(())
}

/// 7. 性能优化技巧
async fn demonstrate_performance_optimization() -> Result<()> {
    println!("\n🔍 7.1 批量处理优化");
    
    let data: Vec<i32> = (1..=20).collect();
    let batch_size = 5;
    
    let start = Instant::now();
    
    let mut all_results = Vec::new();
    for batch in data.chunks(batch_size) {
        let batch_tasks: Vec<_> = batch.iter()
            .map(|&item| async move {
                sleep(Duration::from_millis(50)).await;
                item * 2
            })
            .collect();
        
        let batch_results = join_all(batch_tasks).await;
        all_results.extend(batch_results);
    }
    
    let elapsed = start.elapsed();
    println!("      📊 批量处理完成: {} 项，耗时: {:?}", all_results.len(), elapsed);
    
    println!("\n🔍 7.2 并发限制");
    
    let semaphore = Arc::new(Semaphore::new(3));
    let tasks: Vec<_> = (1..=10)
        .map(|i| {
            let semaphore = Arc::clone(&semaphore);
            tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                let start = Instant::now();
                sleep(Duration::from_millis(100)).await;
                let elapsed = start.elapsed();
                (i, elapsed)
            })
        })
        .collect();
    
    let start = Instant::now();
    let results = join_all(tasks).await;
    let total_elapsed = start.elapsed();
    
    println!("      📊 限制并发结果:");
    for result in results {
        if let Ok((i, task_time)) = result {
            println!("        任务{}: {:?}", i, task_time);
        }
    }
    println!("      ⏱️ 总耗时: {:?}", total_elapsed);
    
    Ok(())
}

/// 8. 实际应用案例
async fn demonstrate_real_world_examples() -> Result<()> {
    println!("\n🔍 8.1 Web 爬虫模拟");
    
    let urls = vec![
        "https://example.com/page1",
        "https://example.com/page2", 
        "https://example.com/page3",
        "https://example.com/page4",
    ];
    
    let semaphore = Arc::new(Semaphore::new(2)); // 限制并发请求数
    let mut tasks = Vec::new();
    
    for url in urls {
        let semaphore = Arc::clone(&semaphore);
        let task = tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            
            // 模拟网络请求
            let delay = 100 + rand::thread_rng().gen_range(0..100);
            sleep(Duration::from_millis(delay)).await;
            
            if url.contains("page3") {
                Err(anyhow!("网络错误: {}", url))
            } else {
                Ok(format!("页面内容: {}", url))
            }
        });
        tasks.push(task);
    }
    
    let results = join_all(tasks).await;
    
    let mut success_count = 0;
    let mut error_count = 0;
    
    for result in results {
        match result {
            Ok(Ok(content)) => {
                success_count += 1;
                println!("      ✅ 抓取成功: {}", content);
            }
            Ok(Err(e)) => {
                error_count += 1;
                println!("      ❌ 抓取失败: {}", e);
            }
            Err(e) => {
                error_count += 1;
                println!("      💥 任务崩溃: {}", e);
            }
        }
    }
    
    println!("      📊 爬虫统计: 成功 {}, 失败 {}", success_count, error_count);
    
    println!("\n🔍 8.2 数据处理管道");
    
    let (tx, mut rx) = mpsc::channel::<i32>(100);
    
    // 数据生成器
    let producer = tokio::spawn(async move {
        for i in 1..=10 {
            let _ = tx.send(i).await;
            sleep(Duration::from_millis(20)).await;
        }
    });
    
    // 数据处理器
    let processor = tokio::spawn(async move {
        let mut processed_data = Vec::new();
        
        while let Some(data) = rx.recv().await {
            // 模拟数据处理
            sleep(Duration::from_millis(30)).await;
            let processed = data * data;
            processed_data.push(processed);
            
            if processed_data.len() >= 10 {
                break;
            }
        }
        
        processed_data
    });
    
    let (_, processed_results) = futures::join!(producer, processor);
    
    println!("      📊 数据处理结果:");
    for (i, result) in processed_results?.into_iter().enumerate() {
        println!("        数据{}: {} -> {}", i + 1, i + 1, result);
    }
    
    Ok(())
}

// 辅助函数

/// 模拟异步任务
async fn async_task(name: String, duration_ms: u64) -> String {
    sleep(Duration::from_millis(duration_ms)).await;
    format!("{} 完成", name)
}

/// 模拟可能失败的异步任务
async fn fallible_task(name: String, duration_ms: u64, should_fail: bool) -> Result<String> {
    sleep(Duration::from_millis(duration_ms)).await;
    
    if should_fail {
        Err(anyhow!("{} 执行失败", name))
    } else {
        Ok(format!("{} 成功完成", name))
    }
}
