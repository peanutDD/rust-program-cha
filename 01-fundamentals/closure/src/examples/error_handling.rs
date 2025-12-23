//! # 错误处理中的闭包

/// 演示错误处理
pub fn demo_error_handling() {
    println!("\n=== 错误处理与闭包 ===");

    let numbers = vec!["1", "2", "invalid", "4"];

    // 使用 filter_map 处理可能的错误
    let parsed: Vec<i32> = numbers
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();

    println!("成功解析的数字: {:?}", parsed);

    // 使用 Result 收集所有错误
    let results: Result<Vec<i32>, _> = numbers
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();

    match results {
        Ok(nums) => println!("所有解析成功: {:?}", nums),
        Err(e) => println!("解析失败: {}", e),
    }
}

