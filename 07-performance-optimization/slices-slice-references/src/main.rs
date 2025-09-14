//! Rust 切片(Slices)与切片引用(Slice References)全面示例
//! 
//! 本文件包含了切片和切片引用的所有重要概念和实际应用示例
//! 涵盖基础概念、内存布局、性能分析、实际应用等各个方面

use std::fmt::Display;
use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;

mod exercises_test;
mod comparison_examples;

use exercises_test::run_all_exercises;
use comparison_examples::run_all_comparisons;

fn main() {
    println!("🦀 Rust 切片与切片引用深度解析 🦀");
    println!("=====================================\n");
    
    // 运行切片 vs 切片引用对比示例
    run_all_comparisons();
    
    // 运行所有练习
    run_all_exercises();
    
    println!("\n=== Rust 切片与切片引用深度解析 ===");
    
    // 1. 基础概念演示
    basic_concepts_demo();
    
    // 2. 类型系统演示
    type_system_demo();
    
    // 3. 内存布局演示
    memory_layout_demo();
    
    // 4. 生命周期与借用演示
    lifetime_borrowing_demo();
    
    // 5. 性能分析演示
    performance_analysis_demo();
    
    // 6. 实际应用场景演示
    practical_applications_demo();
    
    // 7. 高级特性演示
    advanced_features_demo();
    
    // 8. 常见陷阱与最佳实践
    pitfalls_and_best_practices_demo();
    
    println!("\n📚 更多详细信息请查看:");
    println!("- SLICE_VS_SLICE_REFERENCE_COMPARISON.md - 切片与切片引用详细对比");
    println!("- SLICES_COMPREHENSIVE_ANALYSIS.md - 理论分析文档");
    println!("- SLICE_EXERCISES.md - 练习题集");
    println!("- src/comparison_examples.rs - 对比示例代码");
    println!("- src/exercises_test.rs - 可运行的练习代码");
    
    println!("\n🎯 学习建议:");
    println!("1. 先查看对比文档理解切片和切片引用的区别");
    println!("2. 运行对比示例观察实际差异");
    println!("3. 阅读理论分析文档深入理解概念");
    println!("4. 运行练习代码观察实际效果");
    println!("5. 尝试修改代码验证你的理解");
    println!("6. 完成练习题巩固知识");
}

/// 1. 基础概念演示
fn basic_concepts_demo() {
    println!("\n1. === 基础概念演示 ===");
    
    // 1.1 字符串切片
    println!("\n1.1 字符串切片演示:");
    let string_literal: &str = "Hello, Rust!"; // 字符串字面量是 &str 类型
    let owned_string = String::from("Hello, World!");
    let string_slice: &str = &owned_string[0..5]; // 从 String 创建切片引用
    
    println!("字符串字面量: {}", string_literal);
    println!("字符串切片: {}", string_slice);
    println!("字符串切片长度: {}", string_slice.len());
    
    // 1.2 数组切片
    println!("\n1.2 数组切片演示:");
    let array: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let array_slice: &[i32] = &array[1..4]; // [2, 3, 4]
    let full_slice: &[i32] = &array[..]; // 完整数组的切片引用
    
    println!("原数组: {:?}", array);
    println!("部分切片: {:?}", array_slice);
    println!("完整切片: {:?}", full_slice);
    
    // 1.3 Vector 切片
    println!("\n1.3 Vector 切片演示:");
    let vector = vec![10, 20, 30, 40, 50];
    let vec_slice: &[i32] = &vector[2..];
    
    println!("Vector: {:?}", vector);
    println!("Vector 切片: {:?}", vec_slice);
}

/// 2. 类型系统演示
fn type_system_demo() {
    println!("\n2. === 类型系统演示 ===");
    
    // 2.1 切片类型 vs 切片引用类型
    println!("\n2.1 类型对比:");
    let data = [1, 2, 3, 4, 5];
    
    // 这些都是切片引用类型
    let slice_ref1: &[i32] = &data[..];
    let slice_ref2: &[i32] = &data;
    let slice_ref3 = &data[1..4];
    
    println!("slice_ref1 类型: &[i32], 值: {:?}", slice_ref1);
    println!("slice_ref2 类型: &[i32], 值: {:?}", slice_ref2);
    println!("slice_ref3 类型: &[i32], 值: {:?}", slice_ref3);
    
    // 2.2 可变切片引用
    println!("\n2.2 可变切片引用:");
    let mut mutable_data = [1, 2, 3, 4, 5];
    let mutable_slice: &mut [i32] = &mut mutable_data[1..4];
    
    println!("修改前: {:?}", mutable_slice);
    mutable_slice[0] = 100;
    mutable_slice[2] = 300;
    println!("修改后: {:?}", mutable_slice);
    println!("原数组: {:?}", mutable_data);
    
    // 2.3 类型推导
    println!("\n2.3 类型推导演示:");
    demonstrate_type_inference();
}

/// 演示类型推导
fn demonstrate_type_inference() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 编译器自动推导类型
    let slice1 = &numbers[..]; // 推导为 &[i32]
    let slice2 = &numbers[1..3]; // 推导为 &[i32]
    
    // 显式类型注解
    let slice3: &[i32] = &numbers;
    
    println!("自动推导切片1: {:?}", slice1);
    println!("自动推导切片2: {:?}", slice2);
    println!("显式类型切片: {:?}", slice3);
}

/// 3. 内存布局演示
fn memory_layout_demo() {
    println!("\n3. === 内存布局演示 ===");
    
    // 3.1 切片引用的内存表示
    println!("\n3.1 切片引用内存布局:");
    let data = [1, 2, 3, 4, 5];
    let slice = &data[1..4];
    
    println!("原数组地址: {:p}", &data);
    println!("切片数据指针: {:p}", slice.as_ptr());
    println!("切片长度: {}", slice.len());
    println!("切片引用大小: {} bytes", std::mem::size_of_val(&slice));
    
    // 3.2 不同类型的大小对比
    println!("\n3.2 类型大小对比:");
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let slice_ref: &[i32] = &array;
    let array_ref: &[i32; 5] = &array;
    let string = String::from("hello");
    let str_slice: &str = &string;
    
    println!("数组 [i32; 5] 大小: {} bytes", std::mem::size_of::<[i32; 5]>());
    println!("切片引用 &[i32] 大小: {} bytes", std::mem::size_of_val(&slice_ref));
    println!("数组引用 &[i32; 5] 大小: {} bytes", std::mem::size_of_val(&array_ref));
    println!("String 大小: {} bytes", std::mem::size_of::<String>());
    println!("字符串切片 &str 大小: {} bytes", std::mem::size_of_val(&str_slice));
    
    // 3.3 内存连续性验证
    println!("\n3.3 内存连续性验证:");
    verify_memory_continuity();
}

/// 验证切片的内存连续性
fn verify_memory_continuity() {
    let data = [10, 20, 30, 40, 50];
    let slice = &data[1..4]; // [20, 30, 40]
    
    println!("验证切片内存连续性:");
    for (i, item) in slice.iter().enumerate() {
        println!("  元素[{}]: 值={}, 地址={:p}", i, item, item);
    }
    
    // 验证地址连续性
    let ptr = slice.as_ptr();
    unsafe {
        for i in 0..slice.len() {
            let addr = ptr.add(i);
            println!("  计算地址[{}]: {:p}, 值: {}", i, addr, *addr);
        }
    }
}

/// 4. 生命周期与借用演示
fn lifetime_borrowing_demo() {
    println!("\n4. === 生命周期与借用演示 ===");
    
    // 4.1 生命周期基础
    println!("\n4.1 生命周期基础:");
    let data = vec![1, 2, 3, 4, 5];
    let result = get_middle_slice(&data);
    println!("中间切片: {:?}", result);
    
    // 4.2 借用规则演示
    println!("\n4.2 借用规则演示:");
    demonstrate_borrowing_rules();
    
    // 4.3 生命周期省略规则
    println!("\n4.3 生命周期省略规则:");
    demonstrate_lifetime_elision();
}

/// 获取中间部分的切片
fn get_middle_slice(data: &[i32]) -> &[i32] {
    let len = data.len();
    if len < 3 {
        data
    } else {
        &data[1..len-1]
    }
}

/// 演示借用规则
fn demonstrate_borrowing_rules() {
    let mut data = vec![1, 2, 3, 4, 5];
    
    // 多个不可变借用可以共存
    {
        let slice1 = &data[0..2];
        let slice2 = &data[2..4];
        println!("不可变借用1: {:?}", slice1);
        println!("不可变借用2: {:?}", slice2);
    } // 不可变借用结束
    
    // 可变借用是排他的
    {
        let slice_mut = &mut data[1..4];
        slice_mut[0] = 100;
        println!("可变借用修改后: {:?}", slice_mut);
    } // 可变借用结束
    
    println!("最终数据: {:?}", data);
}

/// 演示生命周期省略规则
fn demonstrate_lifetime_elision() {
    let text = "Hello, Rust Programming!";
    
    // 生命周期省略：编译器自动推导
    let first_word = get_first_word(text);
    let last_word = get_last_word(text);
    
    println!("原文本: {}", text);
    println!("第一个单词: {}", first_word);
    println!("最后一个单词: {}", last_word);
}

/// 获取第一个单词（生命周期省略）
fn get_first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

/// 获取最后一个单词（生命周期省略）
fn get_last_word(text: &str) -> &str {
    text.split_whitespace().last().unwrap_or("")
}

/// 5. 性能分析演示
fn performance_analysis_demo() {
    println!("\n5. === 性能分析演示 ===");
    
    // 5.1 零成本抽象演示
    println!("\n5.1 零成本抽象演示:");
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let sum1 = sum_by_index(&data);
    let sum2 = sum_by_iterator(&data[..]);
    let sum3 = sum_by_slice(&data[2..8]);
    
    println!("索引求和: {}", sum1);
    println!("迭代器求和: {}", sum2);
    println!("切片求和: {}", sum3);
    
    // 5.2 内存效率对比
    println!("\n5.2 内存效率对比:");
    memory_efficiency_comparison();
    
    // 5.3 缓存友好性演示
    println!("\n5.3 缓存友好性演示:");
    cache_friendly_demo();
}

/// 通过索引求和
fn sum_by_index(data: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in 0..data.len() {
        sum += data[i];
    }
    sum
}

/// 通过迭代器求和
fn sum_by_iterator(data: &[i32]) -> i32 {
    data.iter().sum()
}

/// 通过切片求和
fn sum_by_slice(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for &item in slice {
        sum += item;
    }
    sum
}

/// 内存效率对比
fn memory_efficiency_comparison() {
    let data = vec![1, 2, 3, 4, 5];
    
    // 不同方式的内存使用
    let vec_clone = data.clone(); // 堆分配 + 数据复制
    let slice_ref = &data[..]; // 仅栈上16字节
    let array_ref = &data; // 仅栈上8字节（Vec引用）
    
    println!("Vec克隆大小: {} bytes (堆) + {} bytes (栈)", 
             data.len() * std::mem::size_of::<i32>(),
             std::mem::size_of::<Vec<i32>>());
    println!("切片引用大小: {} bytes (栈)", std::mem::size_of_val(&slice_ref));
    println!("Vec引用大小: {} bytes (栈)", std::mem::size_of_val(&array_ref));
    
    // 使用数据避免编译器优化
    println!("数据验证: {} {} {}", vec_clone.len(), slice_ref.len(), array_ref.len());
}

/// 缓存友好性演示
fn cache_friendly_demo() {
    let large_data: Vec<i32> = (0..1000).collect();
    
    // 顺序访问（缓存友好）
    let sequential_sum = process_sequential(&large_data);
    
    // 切片操作保持内存局部性
    let chunk_sums: Vec<i32> = large_data
        .chunks(100)
        .map(|chunk| chunk.iter().sum())
        .collect();
    
    println!("顺序访问总和: {}", sequential_sum);
    println!("分块处理结果: {:?}", &chunk_sums[..5]); // 显示前5个结果
}

/// 顺序处理数据
fn process_sequential(data: &[i32]) -> i32 {
    data.iter().map(|&x| x * 2).sum()
}

/// 6. 实际应用场景演示
fn practical_applications_demo() {
    println!("\n6. === 实际应用场景演示 ===");
    
    // 6.1 字符串处理
    println!("\n6.1 字符串处理应用:");
    string_processing_demo();
    
    // 6.2 数据分析
    println!("\n6.2 数据分析应用:");
    data_analysis_demo();
    
    // 6.3 算法实现
    println!("\n6.3 算法实现应用:");
    algorithm_demo();
}

/// 字符串处理演示
fn string_processing_demo() {
    let email = "user@example.com";
    let url = "https://www.rust-lang.org/learn";
    let csv_line = "John,25,Engineer,New York";
    
    // 提取邮箱域名
    if let Some(domain) = extract_domain(email) {
        println!("邮箱域名: {}", domain);
    }
    
    // 提取URL路径
    if let Some(path) = extract_url_path(url) {
        println!("URL路径: {}", path);
    }
    
    // 解析CSV
    let fields = parse_csv_line(csv_line);
    println!("CSV字段: {:?}", fields);
}

/// 提取邮箱域名
fn extract_domain(email: &str) -> Option<&str> {
    email.find('@').map(|pos| &email[pos + 1..])
}

/// 提取URL路径
fn extract_url_path(url: &str) -> Option<&str> {
    url.find("://")
        .and_then(|pos| url[pos + 3..].find('/'))
        .map(|path_start| &url[url.find("://").unwrap() + 3 + path_start..])
}

/// 解析CSV行
fn parse_csv_line(line: &str) -> Vec<&str> {
    line.split(',').collect()
}

/// 数据分析演示
fn data_analysis_demo() {
    let sales_data = [120, 150, 180, 90, 200, 170, 160, 140, 190, 210];
    
    // 计算移动平均
    let window_size = 3;
    let moving_averages = calculate_moving_average(&sales_data, window_size);
    println!("原始数据: {:?}", sales_data);
    println!("移动平均(窗口={}): {:?}", window_size, moving_averages);
    
    // 查找峰值
    let peaks = find_peaks(&sales_data);
    println!("峰值位置: {:?}", peaks);
    
    // 数据统计
    let stats = calculate_statistics(&sales_data);
    println!("统计信息: {:?}", stats);
}

/// 计算移动平均
fn calculate_moving_average(data: &[i32], window_size: usize) -> Vec<f64> {
    data.windows(window_size)
        .map(|window| window.iter().sum::<i32>() as f64 / window_size as f64)
        .collect()
}

/// 查找峰值
fn find_peaks(data: &[i32]) -> Vec<usize> {
    let mut peaks = Vec::new();
    for i in 1..data.len() - 1 {
        if data[i] > data[i - 1] && data[i] > data[i + 1] {
            peaks.push(i);
        }
    }
    peaks
}

/// 计算统计信息
#[derive(Debug)]
struct Statistics {
    min: i32,
    max: i32,
    mean: f64,
    median: f64,
}

fn calculate_statistics(data: &[i32]) -> Statistics {
    let mut sorted_data = data.to_vec();
    sorted_data.sort();
    
    let min = *sorted_data.first().unwrap();
    let max = *sorted_data.last().unwrap();
    let mean = data.iter().sum::<i32>() as f64 / data.len() as f64;
    let median = if sorted_data.len() % 2 == 0 {
        let mid = sorted_data.len() / 2;
        (sorted_data[mid - 1] + sorted_data[mid]) as f64 / 2.0
    } else {
        sorted_data[sorted_data.len() / 2] as f64
    };
    
    Statistics { min, max, mean, median }
}

/// 算法演示
fn algorithm_demo() {
    let numbers = [64, 34, 25, 12, 22, 11, 90, 88, 76, 50, 42];
    
    // 二分查找
    let mut sorted_numbers = numbers;
    sorted_numbers.sort();
    let target = 25;
    if let Some(index) = binary_search(&sorted_numbers, target) {
        println!("二分查找: 在位置 {} 找到 {}", index, target);
    }
    
    // 滑动窗口最大值
    let window_max = sliding_window_maximum(&numbers, 3);
    println!("滑动窗口最大值: {:?}", window_max);
    
    // 快速排序演示
    let mut quick_sort_data = numbers;
    quick_sort(&mut quick_sort_data);
    println!("快速排序结果: {:?}", quick_sort_data);
}

/// 二分查找
fn binary_search(data: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = data.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        match data[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    None
}

/// 滑动窗口最大值
fn sliding_window_maximum(data: &[i32], window_size: usize) -> Vec<i32> {
    data.windows(window_size)
        .map(|window| *window.iter().max().unwrap())
        .collect()
}

/// 快速排序
fn quick_sort(data: &mut [i32]) {
    if data.len() <= 1 {
        return;
    }
    
    let pivot_index = partition(data);
    let (left, right) = data.split_at_mut(pivot_index);
    
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

/// 分区函数
fn partition(data: &mut [i32]) -> usize {
    let pivot = data.len() - 1;
    let mut i = 0;
    
    for j in 0..pivot {
        if data[j] <= data[pivot] {
            data.swap(i, j);
            i += 1;
        }
    }
    
    data.swap(i, pivot);
    i
}

/// 7. 高级特性演示
fn advanced_features_demo() {
    println!("\n7. === 高级特性演示 ===");
    
    // 7.1 自定义切片类型
    println!("\n7.1 自定义切片类型:");
    custom_slice_demo();
    
    // 7.2 切片模式匹配
    println!("\n7.2 切片模式匹配:");
    slice_pattern_matching_demo();
    
    // 7.3 切片与迭代器
    println!("\n7.3 切片与迭代器:");
    slice_iterator_demo();
    
    // 7.4 不安全切片操作
    println!("\n7.4 不安全切片操作:");
    unsafe_slice_demo();
}

/// 自定义支持切片操作的类型
#[derive(Debug)]
struct MyVec<T> {
    data: Vec<T>,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        MyVec { data: Vec::new() }
    }
    
    fn push(&mut self, item: T) {
        self.data.push(item);
    }
    
    fn len(&self) -> usize {
        self.data.len()
    }
}

// 实现切片索引操作（包含单个索引和范围索引）
impl<T, I> Index<I> for MyVec<T>
where
    I: SliceIndex<[T]>,
{
    type Output = I::Output;
    
    fn index(&self, index: I) -> &Self::Output {
        &self.data[index]
    }
}

/// 自定义切片类型演示
fn custom_slice_demo() {
    let mut my_vec = MyVec::new();
    my_vec.push(10);
    my_vec.push(20);
    my_vec.push(30);
    my_vec.push(40);
    my_vec.push(50);
    
    println!("自定义Vec: {:?}", my_vec);
    println!("单个元素访问: {}", my_vec[2]);
    println!("切片访问: {:?}", &my_vec[1..4]);
    println!("完整切片: {:?}", &my_vec[..]);
}

/// 切片模式匹配演示
fn slice_pattern_matching_demo() {
    let test_cases = [
        vec![],
        vec![1],
        vec![1, 2],
        vec![1, 2, 3],
        vec![1, 2, 3, 4, 5],
    ];
    
    for (i, case) in test_cases.iter().enumerate() {
        println!("测试用例 {}: {:?}", i + 1, case);
        analyze_slice_pattern(&case[..]);
    }
}

/// 分析切片模式
fn analyze_slice_pattern(slice: &[i32]) {
    match slice {
        [] => println!("  -> 空切片"),
        [x] => println!("  -> 单元素切片: {}", x),
        [x, y] => println!("  -> 双元素切片: {}, {}", x, y),
        [first, .., last] => println!("  -> 多元素切片: 首={}, 尾={}, 长度={}", first, last, slice.len()),
    }
}

/// 切片与迭代器演示
fn slice_iterator_demo() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 链式操作
    let result: Vec<i32> = data
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0) // 偶数索引
        .map(|(_, &x)| x * x) // 平方
        .collect();
    
    println!("原数据: {:?}", data);
    println!("处理结果: {:?}", result);
    
    // 分块处理
    println!("\n分块处理:");
    for (i, chunk) in data.chunks(3).enumerate() {
        let sum: i32 = chunk.iter().sum();
        println!("  块 {}: {:?}, 和: {}", i, chunk, sum);
    }
    
    // 窗口处理
    println!("\n窗口处理:");
    for (i, window) in data.windows(3).enumerate() {
        let avg = window.iter().sum::<i32>() as f64 / window.len() as f64;
        println!("  窗口 {}: {:?}, 平均: {:.2}", i, window, avg);
    }
}

/// 不安全切片操作演示
fn unsafe_slice_demo() {
    let data = [1, 2, 3, 4, 5];
    
    println!("原数据: {:?}", data);
    
    // 安全访问
    if let Some(value) = data.get(2) {
        println!("安全访问 data[2]: {}", value);
    }
    
    // 不安全访问（在确保安全的前提下）
    unsafe {
        let value = data.get_unchecked(2);
        println!("不安全访问 data[2]: {}", value);
    }
    
    // 从原始指针创建切片
    let ptr = data.as_ptr();
    let len = data.len();
    
    unsafe {
        let slice_from_raw = std::slice::from_raw_parts(ptr, len);
        println!("从原始指针创建的切片: {:?}", slice_from_raw);
    }
    
    // 演示指针算术
    unsafe {
        let ptr = data.as_ptr();
        for i in 0..data.len() {
            let element_ptr = ptr.add(i);
            println!("  元素 {}: 地址={:p}, 值={}", i, element_ptr, *element_ptr);
        }
    }
}

/// 8. 常见陷阱与最佳实践演示
fn pitfalls_and_best_practices_demo() {
    println!("\n8. === 常见陷阱与最佳实践演示 ===");
    
    // 8.1 边界检查
    println!("\n8.1 边界检查最佳实践:");
    boundary_check_demo();
    
    // 8.2 生命周期陷阱
    println!("\n8.2 生命周期最佳实践:");
    lifetime_best_practices_demo();
    
    // 8.3 性能优化技巧
    println!("\n8.3 性能优化技巧:");
    performance_tips_demo();
    
    // 8.4 API设计最佳实践
    println!("\n8.4 API设计最佳实践:");
    api_design_demo();
}

/// 边界检查演示
fn boundary_check_demo() {
    let data = [1, 2, 3, 4, 5];
    
    // ❌ 危险：可能panic
    // let slice = &data[0..10]; // 会panic
    
    // ✅ 安全：使用get方法
    match data.get(2..4) {
        Some(slice) => println!("安全切片: {:?}", slice),
        None => println!("切片越界"),
    }
    
    // ✅ 安全：边界检查函数
    if let Some(slice) = safe_slice(&data, 1, 4) {
        println!("安全切片函数: {:?}", slice);
    }
    
    if let Some(_slice) = safe_slice(&data, 1, 10) {
        println!("不会执行");
    } else {
        println!("边界检查阻止了越界访问");
    }
}

/// 安全的切片创建函数
fn safe_slice<T>(data: &[T], start: usize, end: usize) -> Option<&[T]> {
    if start <= end && end <= data.len() {
        Some(&data[start..end])
    } else {
        None
    }
}

/// 生命周期最佳实践演示
fn lifetime_best_practices_demo() {
    let text = "Hello, Rust Programming Language!";
    
    // ✅ 好的实践：返回输入的切片
    let words = extract_words(text);
    println!("提取的单词: {:?}", words);
    
    // ✅ 好的实践：使用owned类型避免生命周期问题
    let owned_words = extract_words_owned(text);
    println!("拥有的单词: {:?}", owned_words);
}

/// 提取单词（返回切片引用）
fn extract_words(text: &str) -> Vec<&str> {
    text.split_whitespace().collect()
}

/// 提取单词（返回拥有的字符串）
fn extract_words_owned(text: &str) -> Vec<String> {
    text.split_whitespace().map(|s| s.to_string()).collect()
}

/// 性能优化技巧演示
fn performance_tips_demo() {
    let large_data: Vec<i32> = (0..1000).collect();
    
    // ✅ 好的实践：避免不必要的分配
    let sum1 = efficient_sum(&large_data);
    
    // ✅ 好的实践：使用chunks进行批处理
    let batch_results = batch_process(&large_data, 100);
    
    // ✅ 好的实践：使用迭代器链
    let filtered_sum = large_data
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum::<i32>();
    
    println!("高效求和: {}", sum1);
    println!("批处理结果数量: {}", batch_results.len());
    println!("过滤求和: {}", filtered_sum);
}

/// 高效求和
fn efficient_sum(data: &[i32]) -> i32 {
    data.iter().sum() // 使用迭代器，避免索引检查
}

/// 批处理
fn batch_process(data: &[i32], batch_size: usize) -> Vec<i32> {
    data.chunks(batch_size)
        .map(|chunk| chunk.iter().sum())
        .collect()
}

/// API设计最佳实践演示
fn api_design_demo() {
    let data = vec![1, 2, 3, 4, 5];
    
    // ✅ 好的API设计：接受切片引用
    let result1 = flexible_function(&data);
    let result2 = flexible_function(&data[1..4]);
    let result3 = flexible_function(&[10, 20, 30]);
    
    println!("灵活函数结果1: {}", result1);
    println!("灵活函数结果2: {}", result2);
    println!("灵活函数结果3: {}", result3);
    
    // ✅ 好的API设计：泛型约束
    let string_data = vec!["hello", "world", "rust"];
    let string_result = generic_process(&string_data);
    println!("泛型处理结果: {}", string_result);
}

/// 灵活的函数设计（接受任何可以转换为切片的类型）
fn flexible_function(data: &[i32]) -> i32 {
    data.iter().sum()
}

/// 泛型处理函数
fn generic_process<T: Display>(data: &[T]) -> String {
    data.iter()
        .map(|item| format!("{}", item))
        .collect::<Vec<_>>()
        .join(", ")
}

// 程序结束标记
// 这个全面的示例涵盖了Rust切片和切片引用的所有重要概念：
// 1. 基础概念和类型系统
// 2. 内存布局和性能特性
// 3. 生命周期和借用规则
// 4. 实际应用场景
// 5. 高级特性和最佳实践
// 6. 常见陷阱和解决方案

/*
编译和运行说明：
1. 确保安装了Rust工具链
2. 在项目目录运行：cargo run
3. 观察输出，理解每个概念的实际表现
4. 尝试修改代码，加深理解

学习建议：
1. 逐步运行每个演示函数
2. 使用调试器观察内存布局
3. 尝试修改代码触发编译错误，理解借用检查器
4. 在实际项目中应用这些概念
*/
