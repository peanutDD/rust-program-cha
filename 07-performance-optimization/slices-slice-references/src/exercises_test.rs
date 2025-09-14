//! 切片练习测试文件
//! 包含所有练习的可运行测试代码

use std::collections::HashMap;
use std::time::Instant;
use std::thread;
use std::sync::Arc;
use std::ops::{Index, IndexMut};
// use std::slice::SliceIndex; // 暂时不需要

/// 练习1: 基本切片创建
pub fn exercise_1() {
    println!("=== 练习1: 基本切片创建 ===");
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let first_three = &arr[0..3];        // [1, 2, 3]
    let last_three = &arr[7..];          // [8, 9, 10]
    let middle_four = &arr[2..6];        // [3, 4, 5, 6]
    
    // 偶数索引元素需要通过迭代器实现
    let even_indexed: Vec<i32> = arr
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, &val)| val)
        .collect();
    
    println!("前3个: {:?}", first_three);
    println!("后3个: {:?}", last_three);
    println!("中间4个: {:?}", middle_four);
    println!("偶数索引: {:?}", even_indexed);
    println!();
}

/// 练习2: 字符串切片操作
pub fn exercise_2() {
    println!("=== 练习2: 字符串切片操作 ===");
    let text = "Hello, Rust Programming!";
    
    let hello = &text[0..5];
    let rust = &text[7..11];
    let programming = &text[12..23];
    
    println!("Hello: '{}' (长度: {})", hello, hello.len());
    println!("Rust: '{}' (长度: {})", rust, rust.len());
    println!("Programming: '{}' (长度: {})", programming, programming.len());
    
    // 更安全的方式
    let words: Vec<&str> = text.split_whitespace().collect();
    for (i, word) in words.iter().enumerate() {
        let clean_word = word.trim_matches(|c: char| !c.is_alphabetic());
        println!("单词 {}: '{}' (长度: {})", i + 1, clean_word, clean_word.len());
    }
    println!();
}

/// 练习3: 可变切片修改
pub fn exercise_3() {
    println!("=== 练习3: 可变切片修改 ===");
    let mut arr = [1, 2, 3, 4, 5, 6, 7];
    println!("修改前: {:?}", arr);
    
    let middle_slice = &mut arr[2..5]; // [3, 4, 5]
    for item in middle_slice {
        *item *= 2;
    }
    
    println!("修改后: {:?}", arr); // [1, 2, 6, 8, 10, 6, 7]
    println!();
}

/// 练习4: 切片类型推导
pub fn exercise_4() {
    println!("=== 练习4: 切片类型推导 ===");
    let data = vec![1, 2, 3, 4, 5];
    let a = &data;
    let b = &data[..];
    let c = &data[1..3];
    let d = data.as_slice();
    
    println!("a: &Vec<i32> - Vec的引用, 值: {:?}", a);
    println!("b: &[i32] - 切片引用, 值: {:?}", b);
    println!("c: &[i32] - 切片引用, 值: {:?}", c);
    println!("d: &[i32] - 切片引用, 值: {:?}", d);
    println!();
}

/// 练习5: 切片长度和容量
pub fn exercise_5() {
    println!("=== 练习5: 切片长度和容量 ===");
    let vec = vec![1, 2, 3, 4, 5];
    let slice = &vec[1..4];
    
    println!("Vec长度: {}, 容量: {}", vec.len(), vec.capacity());
    println!("切片长度: {}", slice.len());
    // println!("切片容量: {}", slice.capacity()); // ❌ 编译错误
    
    println!("解释:");
    println!("1. Vec拥有数据，需要管理内存分配，因此有容量概念");
    println!("2. 切片只是数据的视图，不拥有数据，只关心当前视图的长度");
    println!("3. 切片引用包含指针和长度，没有容量信息");
    println!();
}

/// 安全的切片访问函数
fn safe_slice_access<T>(data: &[T], start: usize, end: usize) -> Option<&[T]> {
    if start <= end && end <= data.len() {
        Some(&data[start..end])
    } else {
        None
    }
}

fn safe_element_access<T>(data: &[T], index: usize) -> Option<&T> {
    data.get(index)
}

/// 练习6: 切片边界检查
pub fn exercise_6() {
    println!("=== 练习6: 切片边界检查 ===");
    let data = [1, 2, 3, 4, 5];
    
    // 安全访问
    match safe_slice_access(&data, 1, 4) {
        Some(slice) => println!("安全切片: {:?}", slice),
        None => println!("切片访问越界"),
    }
    
    match safe_element_access(&data, 10) {
        Some(element) => println!("元素: {}", element),
        None => println!("元素访问越界"),
    }
    println!();
}

/// 练习7: 切片比较
pub fn exercise_7() {
    println!("=== 练习7: 切片比较 ===");
    let arr1 = [1, 2, 3];
    let arr2 = [1, 2, 3];
    let vec1 = vec![1, 2, 3];
    
    let slice1 = &arr1[..];
    let slice2 = &arr2[..];
    let slice3 = &vec1[..];
    
    println!("slice1 == slice2: {}", slice1 == slice2); // true
    println!("slice1 == slice3: {}", slice1 == slice3); // true
    
    println!("切片比较的是内容，不是来源");
    println!();
}

/// 练习8: 切片迭代
pub fn exercise_8() {
    println!("=== 练习8: 切片迭代 ===");
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice = &data[2..8];
    
    println!("方式1: 索引迭代");
    for i in 0..slice.len() {
        println!("  [{}] = {}", i, slice[i]);
    }
    
    println!("方式2: 值迭代");
    for &value in slice {
        println!("  值: {}", value);
    }
    
    println!("方式3: 引用迭代");
    for value in slice {
        println!("  引用: {}", value);
    }
    
    println!("方式4: 枚举迭代");
    for (i, &value) in slice.iter().enumerate() {
        println!("  [{}] = {}", i, value);
    }
    println!();
}

/// 练习9: 切片转换
pub fn exercise_9() {
    println!("=== 练习9: 切片转换 ===");
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    
    // 切片转Vec
    let vec_from_slice: Vec<i32> = slice.to_vec();
    println!("切片转Vec: {:?}", vec_from_slice);
    
    // 切片转数组（固定大小）
    let array_from_slice: [i32; 3] = slice.try_into().unwrap();
    println!("切片转数组: {:?}", array_from_slice);
    
    // Vec转切片
    let vec = vec![10, 20, 30];
    let slice_from_vec: &[i32] = &vec;
    println!("Vec转切片: {:?}", slice_from_vec);
    
    // 字符串切片转String
    let str_slice = "hello";
    let string_from_slice = str_slice.to_string();
    println!("&str转String: {}", string_from_slice);
    println!();
}

/// 分析切片长度的函数
fn analyze_slice_length(slice: &[i32]) -> String {
    match slice {
        [] => "空切片".to_string(),
        [_] => "单元素切片".to_string(),
        [_, _] => "双元素切片".to_string(),
        [_, _, _] => "三元素切片".to_string(),
        _ => format!("多元素切片，长度: {}", slice.len()),
    }
}

/// 练习10: 切片模式匹配基础
pub fn exercise_10() {
    println!("=== 练习10: 切片模式匹配基础 ===");
    let test_cases = [
        &[][..],
        &[1][..],
        &[1, 2][..],
        &[1, 2, 3][..],
        &[1, 2, 3, 4, 5][..],
    ];
    
    for (i, case) in test_cases.iter().enumerate() {
        println!("测试 {}: {:?} -> {}", i + 1, case, analyze_slice_length(case));
    }
    println!();
}

/// 练习11: 生命周期分析

// ✅ 正确：返回输入切片的一部分
fn get_first_half(data: &[i32]) -> &[i32] {
    let mid = data.len() / 2;
    &data[..mid]
}

// ✅ 正确：返回拥有的数据
fn create_owned_vec() -> Vec<i32> {
    vec![1, 2, 3]
}

// ✅ 正确：显式生命周期参数
fn longest_slice<'a>(x: &'a [i32], y: &'a [i32]) -> &'a [i32] {
    if x.len() > y.len() { x } else { y }
}

pub fn exercise_11() {
    println!("=== 练习11: 生命周期分析 ===");
    let data = [1, 2, 3, 4, 5, 6];
    let first_half = get_first_half(&data);
    println!("前半部分: {:?}", first_half);
    
    let owned = create_owned_vec();
    println!("拥有的数据: {:?}", owned);
    
    let arr1 = [1, 2];
    let arr2 = [3, 4, 5];
    let longer = longest_slice(&arr1, &arr2);
    println!("更长的切片: {:?}", longer);
    println!();
}

/// 练习12: 借用检查器挑战
pub fn exercise_12() {
    println!("=== 练习12: 借用检查器挑战 ===");
    let mut data = vec![1, 2, 3, 4, 5];
    
    // ✅ 正确版本1：分别使用
    {
        let slice1 = &data[0..2];
        println!("不可变借用: {:?}", slice1);
    } // 不可变借用结束
    
    {
        let slice2 = &mut data[2..4];
        slice2[0] = 100;
        println!("可变借用: {:?}", slice2);
    } // 可变借用结束
    
    // ✅ 正确版本2：使用split_at_mut
    let (left, right) = data.split_at_mut(2);
    println!("左半部分: {:?}", left);
    println!("右半部分: {:?}", right);
    
    println!("最终数据: {:?}", data);
    println!();
}

/// 性能测试函数
fn sum_by_index(data: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..data.len() {
        sum += data[i]; // 每次访问都有边界检查
    }
    sum
}

fn sum_by_iterator(data: &[i32]) -> i32 {
    data.iter().sum() // 编译器优化，无边界检查
}

fn sum_by_unsafe(data: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..data.len() {
        unsafe {
            sum += data.get_unchecked(i); // 无边界检查
        }
    }
    sum
}

/// 练习13: 性能优化
pub fn exercise_13() {
    println!("=== 练习13: 性能优化 ===");
    let large_data: Vec<i32> = (0..10_000).collect(); // 进一步减少数据量避免溢出
    let slice = &large_data[..];
    
    // 测试索引访问
    let start = Instant::now();
    let sum1 = sum_by_index(slice);
    let time1 = start.elapsed();
    
    // 测试迭代器
    let start = Instant::now();
    let sum2 = sum_by_iterator(slice);
    let time2 = start.elapsed();
    
    // 测试不安全访问
    let start = Instant::now();
    let sum3 = sum_by_unsafe(slice);
    let time3 = start.elapsed();
    
    println!("索引访问: {} (耗时: {:?})", sum1, time1);
    println!("迭代器: {} (耗时: {:?})", sum2, time2);
    println!("不安全访问: {} (耗时: {:?})", sum3, time3);
    
    assert_eq!(sum1, sum2);
    assert_eq!(sum2, sum3);
    println!("所有结果一致");
    
    // 预期结果: 0+1+2+...+9999 = 9999*10000/2 = 49995000
    let expected = (large_data.len() - 1) * large_data.len() / 2;
    println!("预期结果: {}, 实际结果: {}", expected, sum1);
    println!();
}

/// 移动平均值计算
fn moving_average(data: &[f64], window_size: usize) -> Vec<f64> {
    if window_size == 0 || window_size > data.len() {
        return Vec::new();
    }
    
    data.windows(window_size)
        .map(|window| window.iter().sum::<f64>() / window_size as f64)
        .collect()
}

/// 练习14: 切片窗口操作
pub fn exercise_14() {
    println!("=== 练习14: 切片窗口操作 ===");
    let prices = [10.0, 12.0, 13.0, 15.0, 14.0, 16.0, 18.0, 17.0, 19.0, 20.0];
    
    let ma3 = moving_average(&prices, 3);
    let ma5 = moving_average(&prices, 5);
    
    println!("原始价格: {:?}", prices);
    println!("3日移动平均: {:?}", ma3);
    println!("5日移动平均: {:?}", ma5);
    
    // 手动验证第一个3日移动平均
    let first_window = &prices[0..3];
    let manual_avg = first_window.iter().sum::<f64>() / 3.0;
    println!("手动计算第一个3日平均: {:.2}", manual_avg);
    if !ma3.is_empty() {
        println!("算法计算第一个3日平均: {:.2}", ma3[0]);
    }
    println!();
}

/// 人员信息结构体
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

/// 解析人员信息行
fn parse_person_line(line: &str) -> Option<Person> {
    let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
    
    if parts.len() != 3 {
        return None;
    }
    
    let name = parts[0].to_string();
    let age = parts[1].parse().ok()?;
    let email = parts[2].to_string();
    
    Some(Person { name, age, email })
}

/// 提取邮箱域名
fn extract_domain(email: &str) -> Option<&str> {
    email.find('@').map(|pos| &email[pos + 1..])
}

/// 练习15: 字符串切片高级操作
pub fn exercise_15() {
    println!("=== 练习15: 字符串切片高级操作 ===");
    let data = "John Doe, 30, john@example.com\nJane Smith, 25, jane@gmail.com\nBob Johnson, 35, bob@company.org";
    
    let people: Vec<Person> = data
        .lines()
        .filter_map(parse_person_line)
        .collect();
    
    println!("解析的人员信息:");
    for person in &people {
        let domain = extract_domain(&person.email).unwrap_or("未知");
        println!("  姓名: {}, 年龄: {}, 邮箱域名: {}", person.name, person.age, domain);
    }
    println!();
}

/// 按条件分割切片
fn split_slice_by_predicate<T, F>(slice: &[T], predicate: F) -> Vec<&[T]>
where
    F: Fn(&T) -> bool,
{
    let mut result = Vec::new();
    let mut start = 0;
    
    for (i, item) in slice.iter().enumerate() {
        if predicate(item) {
            if start < i {
                result.push(&slice[start..i]);
            }
            start = i + 1;
        }
    }
    
    if start < slice.len() {
        result.push(&slice[start..]);
    }
    
    result
}

/// 合并切片
fn merge_slices<T: Clone>(slices: &[&[T]]) -> Vec<T> {
    slices.iter().flat_map(|&slice| slice.iter().cloned()).collect()
}

/// 练习16: 切片分割和合并
pub fn exercise_16() {
    println!("=== 练习16: 切片分割和合并 ===");
    let data = [1, 2, 0, 3, 4, 0, 5, 6, 7];
    
    // 按0分割
    let parts = split_slice_by_predicate(&data, |&x| x == 0);
    println!("原数据: {:?}", data);
    println!("按0分割: {:?}", parts);
    
    // 合并切片
    let merged = merge_slices(&parts);
    println!("合并结果: {:?}", merged);
    
    // 字符串分割示例
    let text = "hello,world,rust,programming";
    let words: Vec<&str> = text.split(',').collect();
    println!("单词分割: {:?}", words);
    println!();
}

/// 冒泡排序
fn bubble_sort(slice: &mut [i32]) {
    let len = slice.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1);
            }
        }
    }
}

/// 手动二分搜索
fn binary_search_manual(slice: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = slice.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        match slice[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    None
}

/// 练习17: 切片排序和搜索
pub fn exercise_17() {
    println!("=== 练习17: 切片排序和搜索 ===");
    let mut data = [64, 34, 25, 12, 22, 11, 90];
    println!("排序前: {:?}", data);
    
    bubble_sort(&mut data);
    println!("冒泡排序后: {:?}", data);
    
    // 使用标准库排序
    let mut data2 = [64, 34, 25, 12, 22, 11, 90];
    data2.sort();
    println!("标准库排序: {:?}", data2);
    
    // 二分搜索
    let target = 25;
    match binary_search_manual(&data, target) {
        Some(index) => println!("手动二分搜索: 在位置 {} 找到 {}", index, target),
        None => println!("未找到 {}", target),
    }
    
    // 标准库二分搜索
    match data.binary_search(&target) {
        Ok(index) => println!("标准库二分搜索: 在位置 {} 找到 {}", index, target),
        Err(index) => println!("未找到，应插入位置: {}", index),
    }
    println!();
}

/// 泛型打印函数
use std::fmt::Display;

fn print_slice<T: Display>(slice: &[T], name: &str) {
    print!("{}: [", name);
    for (i, item) in slice.iter().enumerate() {
        if i > 0 { print!(", "); }
        print!("{}", item);
    }
    println!("]")
}

/// 泛型查找最大值
fn find_max<T: Ord + Copy>(slice: &[T]) -> Option<T> {
    slice.iter().max().copied()
}

/// 浮点数查找最大值（使用PartialOrd）
fn find_max_float<T: PartialOrd + Copy>(slice: &[T]) -> Option<T> {
    slice.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).copied()
}

/// 泛型反转切片
fn reverse_slice<T>(slice: &mut [T]) {
    let len = slice.len();
    for i in 0..len / 2 {
        slice.swap(i, len - 1 - i);
    }
}

/// 练习18: 切片与泛型
pub fn exercise_18() {
    println!("=== 练习18: 切片与泛型 ===");
    // 整数切片
    let mut numbers = [1, 5, 3, 9, 2];
    print_slice(&numbers, "原始数字");
    
    if let Some(max) = find_max(&numbers) {
        println!("最大值: {}", max);
    }
    
    reverse_slice(&mut numbers);
    print_slice(&numbers, "反转后");
    
    // 字符串切片
    let mut words = ["hello", "world", "rust", "programming"];
    print_slice(&words, "原始单词");
    
    if let Some(max) = find_max(&words) {
        println!("字典序最大: {}", max);
    }
    
    reverse_slice(&mut words);
    print_slice(&words, "反转后");
    
    // 浮点数切片
    let floats = [3.14, 2.71, 1.41, 1.73];
    print_slice(&floats, "浮点数");
    
    if let Some(max) = find_max_float(&floats) {
        println!("最大值: {:.2}", max);
    }
    println!();
}

/// 内存布局分析
fn analyze_memory_layout() {
    let array = [1i32, 2, 3, 4, 5];
    let slice = &array[1..4];
    
    println!("=== 内存布局分析 ===");
    
    // 数组信息
    println!("数组地址: {:p}", &array);
    println!("数组大小: {} bytes", std::mem::size_of_val(&array));
    println!("数组元素大小: {} bytes", std::mem::size_of::<i32>());
    
    // 切片引用信息
    println!("切片引用地址: {:p}", &slice);
    println!("切片引用大小: {} bytes", std::mem::size_of_val(&slice));
    println!("切片数据指针: {:p}", slice.as_ptr());
    println!("切片长度: {}", slice.len());
    
    // 验证胖指针结构
    unsafe {
        let fat_ptr: (*const i32, usize) = std::mem::transmute(slice);
        println!("胖指针解构 - 指针: {:p}, 长度: {}", fat_ptr.0, fat_ptr.1);
    }
    
    // 元素地址分析
    println!("\n=== 元素地址分析 ===");
    for (i, &value) in slice.iter().enumerate() {
        let element_addr = &value as *const i32;
        println!("元素[{}]: 值={}, 地址={:p}", i, value, element_addr);
    }
    
    // 地址连续性验证
    let ptr = slice.as_ptr();
    println!("\n=== 地址连续性验证 ===");
    unsafe {
        for i in 0..slice.len() {
            let addr = ptr.add(i);
            let value = *addr;
            println!("计算地址[{}]: {:p} -> {}", i, addr, value);
        }
    }
}

/// 练习19: 内存布局分析
pub fn exercise_19() {
    println!("=== 练习19: 内存布局分析 ===");
    analyze_memory_layout();
    println!();
}

/// 并行求和
fn parallel_sum(data: &[i32]) -> i32 {
    let chunk_size = (data.len() + 3) / 4; // 分成4块
    let data = Arc::new(data.to_vec()); // 需要拥有数据才能跨线程
    
    let handles: Vec<_> = (0..4)
        .map(|i| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let start = i * chunk_size;
                let end = std::cmp::min(start + chunk_size, data.len());
                if start < data.len() {
                    data[start..end].iter().sum::<i32>()
                } else {
                    0
                }
            })
        })
        .collect();
    
    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

/// 练习20: 切片与并发
pub fn exercise_20() {
    println!("=== 练习20: 切片与并发 ===");
    let large_data: Vec<i32> = (1..=1000).collect();
    
    // 单线程计算
    let single_thread_sum: i32 = large_data.iter().sum();
    
    // 多线程计算
    let multi_thread_sum = parallel_sum(&large_data);
    
    println!("数据长度: {}", large_data.len());
    println!("单线程求和: {}", single_thread_sum);
    println!("多线程求和: {}", multi_thread_sum);
    println!("结果一致: {}", single_thread_sum == multi_thread_sum);
    
    // 预期结果: 1+2+...+1000 = 1000*1001/2 = 500500
    println!("预期结果: {}", 1000 * 1001 / 2);
    println!();
}

/// 运行所有练习
pub fn run_all_exercises() {
    println!("🦀 Rust 切片与切片引用练习测试 🦀\n");
    
    exercise_1();
    exercise_2();
    exercise_3();
    exercise_4();
    exercise_5();
    exercise_6();
    exercise_7();
    exercise_8();
    exercise_9();
    exercise_10();
    exercise_11();
    exercise_12();
    exercise_13();
    exercise_14();
    exercise_15();
    exercise_16();
    exercise_17();
    exercise_18();
    exercise_19();
    exercise_20();
    
    println!("✅ 所有练习完成！");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_safe_slice_access() {
        let data = [1, 2, 3, 4, 5];
        assert_eq!(safe_slice_access(&data, 1, 3), Some(&[2, 3][..]));
        assert_eq!(safe_slice_access(&data, 1, 10), None);
        assert_eq!(safe_slice_access(&data, 3, 1), None);
    }
    
    #[test]
    fn test_moving_average() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        let ma = moving_average(&data, 3);
        assert_eq!(ma.len(), 3);
        assert!((ma[0] - 2.0).abs() < f64::EPSILON);
        assert!((ma[1] - 3.0).abs() < f64::EPSILON);
        assert!((ma[2] - 4.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_binary_search() {
        let data = [1, 3, 5, 7, 9];
        assert_eq!(binary_search_manual(&data, 5), Some(2));
        assert_eq!(binary_search_manual(&data, 6), None);
    }
    
    #[test]
    fn test_split_slice() {
        let data = [1, 2, 0, 3, 4, 0, 5];
        let parts = split_slice_by_predicate(&data, |&x| x == 0);
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], &[1, 2]);
        assert_eq!(parts[1], &[3, 4]);
        assert_eq!(parts[2], &[5]);
    }
    
    #[test]
    fn test_parse_person() {
        let line = "John Doe, 30, john@example.com";
        let person = parse_person_line(line).unwrap();
        assert_eq!(person.name, "John Doe");
        assert_eq!(person.age, 30);
        assert_eq!(person.email, "john@example.com");
    }
    
    #[test]
    fn test_extract_domain() {
        assert_eq!(extract_domain("user@example.com"), Some("example.com"));
        assert_eq!(extract_domain("invalid-email"), None);
    }
}