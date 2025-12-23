//! # 迭代器与闭包

/// 演示迭代器模式
pub fn demo_iterator_patterns() {
    println!("\n=== 迭代器与闭包 ===");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 过滤和映射
    let even_squares: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();

    println!("偶数的平方: {:?}", even_squares);

    // 查找
    let first_gt_5 = numbers.iter().find(|&&x| x > 5);
    println!("第一个大于5的数: {:?}", first_gt_5);

    // 折叠
    let sum: i32 = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("所有数的和: {}", sum);
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

