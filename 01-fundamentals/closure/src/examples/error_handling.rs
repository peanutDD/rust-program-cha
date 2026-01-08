//! # 错误处理中的闭包
//!
//! 本模块演示高效的错误处理模式，避免不必要的分配。

/// 演示错误处理 - 优化版本
pub fn demo_error_handling() {
    println!("\n=== 错误处理与闭包 ===");

    let numbers = vec!["1", "2", "invalid", "4"];

    // ✅ 优化：使用 filter_map 处理可能的错误，零成本抽象
    let parsed: Vec<i32> = numbers
        .iter()
        .filter_map(|s| s.parse().ok()) // 只保留成功解析的值
        .collect(); // collect 会智能预分配容量

    println!("成功解析的数字: {:?}", parsed);
    println!("原始数组仍可用: {:?}", numbers);

    // ✅ 使用 Result 收集所有错误（需要所有值都成功）
    let results: Result<Vec<i32>, _> = numbers
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();

    match results {
        Ok(nums) => println!("所有解析成功: {:?}", nums),
        Err(e) => println!("解析失败: {}", e),
    }
    
    // 性能提示：filter_map 比先 map 再 filter 更高效
    demonstrate_performance_comparison();
}

/// 演示性能对比
fn demonstrate_performance_comparison() {
    println!("\n--- 性能对比 ---");
    
    let strings = vec!["1", "2", "3", "invalid", "5"];
    
    // 方式1：filter_map（推荐，更高效）
    let start = std::time::Instant::now();
    let result1: Vec<i32> = strings
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    let duration1 = start.elapsed();
    
    // 方式2：map + filter（需要两次遍历）
    let start = std::time::Instant::now();
    let result2: Vec<i32> = strings
        .iter()
        .map(|s| s.parse::<i32>())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect();
    let duration2 = start.elapsed();
    
    println!("filter_map 结果: {:?}, 耗时: {:?}", result1, duration1);
    println!("map+filter 结果: {:?}, 耗时: {:?}", result2, duration2);
    println!("💡 filter_map 更高效：单次遍历，避免中间 Result 分配");
}

