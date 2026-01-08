//! # 迭代器与闭包
//!
//! 本模块演示高效的迭代器使用模式，避免不必要的分配和移动。

/// 演示迭代器模式 - 优化版本
pub fn demo_iterator_patterns() {
    println!("\n=== 迭代器与闭包 ===");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // ✅ 优化：使用引用迭代，避免移动 numbers
    let even_squares: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect(); // collect 会智能预分配容量

    println!("偶数的平方: {:?}", even_squares);
    println!("原始数组仍可用: {:?}", numbers); // numbers 仍然可用

    // ✅ 优化：使用 Option 进行安全的查找
    let first_gt_5 = numbers.iter().find(|&&x| x > 5);
    println!("第一个大于5的数: {:?}", first_gt_5);

    // ✅ 优化：使用 fold 进行累积计算（零成本抽象）
    let sum: i32 = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("所有数的和: {}", sum);

    // 性能提示：迭代器链是零成本抽象，编译器会优化
    demonstrate_performance_tips();
}

/// 演示性能优化技巧
fn demonstrate_performance_tips() {
    println!("\n--- 性能优化技巧 ---");
    
    let numbers: Vec<i32> = (1..=1000).collect();
    
    // 如果知道结果大小，可以预分配
    let capacity_hint = numbers.len() / 2; // 大约一半是偶数
    let mut preallocated = Vec::with_capacity(capacity_hint);
    for &x in &numbers {
        if x % 2 == 0 {
            preallocated.push(x * x);
        }
    }
    println!("预分配版本（已知容量）: {} 个元素", preallocated.len());
    
    // 使用迭代器版本（同样高效，代码更简洁）
    let iterator_version: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("迭代器版本: {} 个元素", iterator_version.len());
    
    println!("💡 提示：迭代器会自动优化，通常无需手动预分配");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_iterator_filter_map() {
        let numbers = vec![1, 2, 3, 4, 5];
        let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    }
}

