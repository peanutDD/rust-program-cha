//! # 最佳实践指南

/// 演示最佳实践
pub fn demo_best_practices() {
    println!("\n=== 最佳实践 ===");

    println!("\n关键原则:");
    println!("1. 保持闭包简洁");
    println!("2. 合理使用类型注解");
    println!("3. 注意闭包的生命周期");
    println!("4. 优先使用静态分发");
    println!("5. 只捕获实际需要的变量");

    demo_code_organization();
}

fn demo_code_organization() {
    println!("\n--- 代码组织最佳实践 ---");

    let numbers = vec![1, 2, 3, 4, 5];

    // 好的做法：简洁的闭包
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("简洁闭包结果: {:?}", doubled);

    // 复杂逻辑提取为函数
    let processed: Vec<i32> = numbers
        .iter()
        .map(|&x| complex_processing(x))
        .collect();
    println!("复杂处理结果: {:?}", processed);
}

fn complex_processing(x: i32) -> i32 {
    let mut result = x;
    result = result * 3;
    result = result + 10;
    result % 100
}
