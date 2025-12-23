//! # 优化策略

/// 演示优化策略
pub fn demo_optimization_strategies() {
    println!("\n=== 优化策略 ===");

    println!("\n关键优化建议:");
    println!("1. 避免不必要的捕获");
    println!("2. 选择合适的捕获方式");
    println!("3. 使用适当的 trait 边界");
    println!("4. 优先使用 Fn，其次 FnMut，最后 FnOnce");

    demo_avoid_unnecessary_captures();
}

fn demo_avoid_unnecessary_captures() {
    println!("\n--- 避免不必要的捕获 ---");

    let _large_data = vec![0; 10000];
    let small_value = 42;

    // 好的做法：只捕获需要的部分
    let needed_value = small_value;
    let good_closure = move || {
        println!("只捕获需要的: {}", needed_value);
    };

    good_closure();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_minimal_capture() {
        let x = 10;
        let closure = || x;
        assert_eq!(closure(), 10);
    }
}

