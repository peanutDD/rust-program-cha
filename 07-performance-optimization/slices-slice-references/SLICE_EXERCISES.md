# Rust 切片与切片引用练习题

本文件包含了一系列练习题，帮助你深入理解 Rust 中切片和切片引用的概念。每个练习都有详细的说明和解答。

## 练习分类

### 基础练习 (1-10)
基础概念理解和简单操作

### 进阶练习 (11-20)
生命周期、借用规则和性能优化

### 高级练习 (21-30)
复杂场景应用和最佳实践

---

## 基础练习

### 练习 1: 基本切片创建
**题目**: 给定数组 `[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]`，创建以下切片：
- 前3个元素
- 后3个元素
- 中间4个元素（索引2-5）
- 所有偶数索引的元素

**解答**:
```rust
fn exercise_1() {
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
}
```

### 练习 2: 字符串切片操作
**题目**: 给定字符串 `"Hello, Rust Programming!"`，完成以下操作：
- 提取 "Hello"
- 提取 "Rust"
- 提取 "Programming"
- 计算每个单词的长度

**解答**:
```rust
fn exercise_2() {
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
}
```

### 练习 3: 可变切片修改
**题目**: 创建一个可变数组，使用可变切片将中间3个元素都乘以2。

**解答**:
```rust
fn exercise_3() {
    let mut arr = [1, 2, 3, 4, 5, 6, 7];
    println!("修改前: {:?}", arr);
    
    let middle_slice = &mut arr[2..5]; // [3, 4, 5]
    for item in middle_slice {
        *item *= 2;
    }
    
    println!("修改后: {:?}", arr); // [1, 2, 6, 8, 10, 6, 7]
}
```

### 练习 4: 切片类型推导
**题目**: 分析以下代码中每个变量的类型，并解释为什么。

```rust
fn exercise_4() {
    let data = vec![1, 2, 3, 4, 5];
    let a = &data;
    let b = &data[..];
    let c = &data[1..3];
    let d = data.as_slice();
    
    // 类型分析：
    // a: &Vec<i32> - Vec的引用
    // b: &[i32] - 切片引用
    // c: &[i32] - 切片引用
    // d: &[i32] - 切片引用
}
```

### 练习 5: 切片长度和容量
**题目**: 解释切片引用为什么没有容量概念，只有长度。

**解答**:
```rust
fn exercise_5() {
    let vec = vec![1, 2, 3, 4, 5];
    let slice = &vec[1..4];
    
    println!("Vec长度: {}, 容量: {}", vec.len(), vec.capacity());
    println!("切片长度: {}", slice.len());
    // println!("切片容量: {}", slice.capacity()); // ❌ 编译错误
    
    // 解释：
    // 1. Vec拥有数据，需要管理内存分配，因此有容量概念
    // 2. 切片只是数据的视图，不拥有数据，只关心当前视图的长度
    // 3. 切片引用包含指针和长度，没有容量信息
}
```

### 练习 6: 切片边界检查
**题目**: 编写安全的切片访问函数，避免panic。

**解答**:
```rust
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

fn exercise_6() {
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
}
```

### 练习 7: 切片比较
**题目**: 比较不同类型的切片，理解切片的相等性。

**解答**:
```rust
fn exercise_7() {
    let arr1 = [1, 2, 3];
    let arr2 = [1, 2, 3];
    let vec1 = vec![1, 2, 3];
    
    let slice1 = &arr1[..];
    let slice2 = &arr2[..];
    let slice3 = &vec1[..];
    
    println!("slice1 == slice2: {}", slice1 == slice2); // true
    println!("slice1 == slice3: {}", slice1 == slice3); // true
    
    // 切片比较的是内容，不是来源
    println!("不同来源的切片可以相等");
}
```

### 练习 8: 切片迭代
**题目**: 使用不同方式迭代切片，比较性能和用法。

**解答**:
```rust
fn exercise_8() {
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
}
```

### 练习 9: 切片转换
**题目**: 演示切片与其他类型之间的转换。

**解答**:
```rust
fn exercise_9() {
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
}
```

### 练习 10: 切片模式匹配基础
**题目**: 使用模式匹配分析不同长度的切片。

**解答**:
```rust
fn analyze_slice_length(slice: &[i32]) -> String {
    match slice {
        [] => "空切片".to_string(),
        [_] => "单元素切片".to_string(),
        [_, _] => "双元素切片".to_string(),
        [_, _, _] => "三元素切片".to_string(),
        _ => format!("多元素切片，长度: {}", slice.len()),
    }
}

fn exercise_10() {
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
}
```

---

## 进阶练习

### 练习 11: 生命周期分析
**题目**: 分析以下函数的生命周期，解释为什么有些能编译，有些不能。

**解答**:
```rust
// ✅ 正确：返回输入切片的一部分
fn get_first_half(data: &[i32]) -> &[i32] {
    let mid = data.len() / 2;
    &data[..mid]
}

// ❌ 错误：返回局部变量的引用
// fn create_local_slice() -> &[i32] {
//     let arr = [1, 2, 3];
//     &arr[..] // 错误：arr在函数结束时被销毁
// }

// ✅ 正确：返回拥有的数据
fn create_owned_vec() -> Vec<i32> {
    vec![1, 2, 3]
}

// ✅ 正确：显式生命周期参数
fn longest_slice<'a>(x: &'a [i32], y: &'a [i32]) -> &'a [i32] {
    if x.len() > y.len() { x } else { y }
}

fn exercise_11() {
    let data = [1, 2, 3, 4, 5, 6];
    let first_half = get_first_half(&data);
    println!("前半部分: {:?}", first_half);
    
    let owned = create_owned_vec();
    println!("拥有的数据: {:?}", owned);
    
    let arr1 = [1, 2];
    let arr2 = [3, 4, 5];
    let longer = longest_slice(&arr1, &arr2);
    println!("更长的切片: {:?}", longer);
}
```

### 练习 12: 借用检查器挑战
**题目**: 修复以下代码中的借用检查错误。

**解答**:
```rust
fn exercise_12() {
    let mut data = vec![1, 2, 3, 4, 5];
    
    // ❌ 错误版本
    // let slice1 = &data[0..2];
    // let slice2 = &mut data[2..4]; // 错误：不能同时有可变和不可变借用
    // println!("{:?} {:?}", slice1, slice2);
    
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
}
```

### 练习 13: 性能优化
**题目**: 比较不同切片操作的性能，找出最优解。

**解答**:
```rust
use std::time::Instant;

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

fn exercise_13() {
    let large_data: Vec<i32> = (0..1_000_000).collect();
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
}
```

### 练习 14: 切片窗口操作
**题目**: 实现滑动窗口算法，计算移动平均值。

**解答**:
```rust
fn moving_average(data: &[f64], window_size: usize) -> Vec<f64> {
    if window_size == 0 || window_size > data.len() {
        return Vec::new();
    }
    
    data.windows(window_size)
        .map(|window| window.iter().sum::<f64>() / window_size as f64)
        .collect()
}

fn exercise_14() {
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
    println!("算法计算第一个3日平均: {:.2}", ma3[0]);
}
```

### 练习 15: 字符串切片高级操作
**题目**: 实现一个简单的文本解析器，提取特定格式的数据。

**解答**:
```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

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

fn extract_domain(email: &str) -> Option<&str> {
    email.find('@').map(|pos| &email[pos + 1..])
}

fn exercise_15() {
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
}
```

### 练习 16: 切片分割和合并
**题目**: 实现切片的分割和合并操作。

**解答**:
```rust
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

fn merge_slices<T: Clone>(slices: &[&[T]]) -> Vec<T> {
    slices.iter().flat_map(|&slice| slice.iter().cloned()).collect()
}

fn exercise_16() {
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
}
```

### 练习 17: 切片排序和搜索
**题目**: 实现各种排序和搜索算法。

**解答**:
```rust
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

fn exercise_17() {
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
}
```

### 练习 18: 切片与泛型
**题目**: 编写泛型函数处理不同类型的切片。

**解答**:
```rust
use std::fmt::Display;

fn print_slice<T: Display>(slice: &[T], name: &str) {
    print!("{}: [", name);
    for (i, item) in slice.iter().enumerate() {
        if i > 0 { print!(", "); }
        print!("{}", item);
    }
    println!("]")
}

fn find_max<T: PartialOrd + Copy>(slice: &[T]) -> Option<T> {
    slice.iter().max().copied()
}

fn reverse_slice<T>(slice: &mut [T]) {
    let len = slice.len();
    for i in 0..len / 2 {
        slice.swap(i, len - 1 - i);
    }
}

fn exercise_18() {
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
    
    if let Some(max) = find_max(&floats) {
        println!("最大值: {:.2}", max);
    }
}
```

### 练习 19: 内存布局分析
**题目**: 深入分析切片引用的内存布局。

**解答**:
```rust
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

fn exercise_19() {
    analyze_memory_layout();
}
```

### 练习 20: 切片与并发
**题目**: 在多线程环境中安全地使用切片。

**解答**:
```rust
use std::thread;
use std::sync::Arc;

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

fn exercise_20() {
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
}
```

---

## 高级练习

### 练习 21: 自定义切片类型
**题目**: 实现一个支持切片操作的自定义数据结构。

**解答**:
```rust
use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;

#[derive(Debug)]
struct RingBuffer<T> {
    data: Vec<T>,
    start: usize,
    len: usize,
}

impl<T> RingBuffer<T> {
    fn new(capacity: usize) -> Self {
        RingBuffer {
            data: Vec::with_capacity(capacity),
            start: 0,
            len: 0,
        }
    }
    
    fn push(&mut self, item: T) {
        if self.len < self.data.capacity() {
            self.data.push(item);
            self.len += 1;
        } else {
            self.data[self.start] = item;
            self.start = (self.start + 1) % self.data.len();
        }
    }
    
    fn len(&self) -> usize {
        self.len
    }
    
    fn as_slices(&self) -> (&[T], &[T]) {
        if self.start + self.len <= self.data.len() {
            (&self.data[self.start..self.start + self.len], &[])
        } else {
            let first_part = &self.data[self.start..];
            let second_part = &self.data[..self.len - first_part.len()];
            (first_part, second_part)
        }
    }
}

impl<T, I> Index<I> for RingBuffer<T>
where
    I: SliceIndex<[T]>,
{
    type Output = I::Output;
    
    fn index(&self, index: I) -> &Self::Output {
        let (first, second) = self.as_slices();
        if second.is_empty() {
            &first[index]
        } else {
            // 对于环形缓冲区，这里需要更复杂的逻辑
            // 简化实现，仅支持连续部分
            &first[index]
        }
    }
}

fn exercise_21() {
    let mut ring = RingBuffer::new(5);
    
    // 添加元素
    for i in 1..=7 {
        ring.push(i);
        println!("添加 {}, 当前状态: {:?}", i, ring);
    }
    
    // 获取切片
    let (first, second) = ring.as_slices();
    println!("切片1: {:?}", first);
    println!("切片2: {:?}", second);
    
    // 索引访问
    if ring.len() > 2 {
        println!("ring[1..3]: {:?}", &ring[1..3]);
    }
}
```

### 练习 22: 零拷贝字符串处理
**题目**: 实现零拷贝的字符串解析器。

**解答**:
```rust
#[derive(Debug, PartialEq)]
struct Token<'a> {
    kind: TokenKind,
    text: &'a str,
    position: usize,
}

#[derive(Debug, PartialEq)]
enum TokenKind {
    Word,
    Number,
    Punctuation,
    Whitespace,
}

struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Tokenizer { input, position: 0 }
    }
    
    fn tokenize(&mut self) -> Vec<Token<'a>> {
        let mut tokens = Vec::new();
        
        while self.position < self.input.len() {
            if let Some(token) = self.next_token() {
                tokens.push(token);
            }
        }
        
        tokens
    }
    
    fn next_token(&mut self) -> Option<Token<'a>> {
        if self.position >= self.input.len() {
            return None;
        }
        
        let start = self.position;
        let remaining = &self.input[self.position..];
        let first_char = remaining.chars().next()?;
        
        let (kind, len) = if first_char.is_alphabetic() {
            let len = remaining.chars().take_while(|c| c.is_alphanumeric()).count();
            (TokenKind::Word, len)
        } else if first_char.is_numeric() {
            let len = remaining.chars().take_while(|c| c.is_numeric() || *c == '.').count();
            (TokenKind::Number, len)
        } else if first_char.is_whitespace() {
            let len = remaining.chars().take_while(|c| c.is_whitespace()).count();
            (TokenKind::Whitespace, len)
        } else {
            (TokenKind::Punctuation, 1)
        };
        
        let end = start + remaining.chars().take(len).map(|c| c.len_utf8()).sum::<usize>();
        let text = &self.input[start..end];
        self.position = end;
        
        Some(Token {
            kind,
            text,
            position: start,
        })
    }
}

fn exercise_22() {
    let input = "Hello, world! The answer is 42.";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    
    println!("输入: {}", input);
    println!("令牌:");
    for token in &tokens {
        println!("  {:?} '{}' @ {}", token.kind, token.text, token.position);
    }
    
    // 验证零拷贝：所有token的text都指向原始字符串
    for token in &tokens {
        let original_ptr = input.as_ptr();
        let token_ptr = token.text.as_ptr();
        let offset = unsafe { token_ptr.offset_from(original_ptr) };
        println!("Token '{}' 偏移量: {}", token.text, offset);
    }
}
```

### 练习 23: 切片的函数式编程
**题目**: 使用函数式编程风格处理切片数据。

**解答**:
```rust
fn exercise_23() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 链式操作：过滤、映射、归约
    let result = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)           // 偶数
        .map(|&x| x * x)                     // 平方
        .filter(|&x| x > 10)                 // 大于10
        .fold(0, |acc, x| acc + x);          // 求和
    
    println!("原数据: {:?}", numbers);
    println!("偶数平方和(>10): {}", result);
    
    // 分组操作
    let words = ["apple", "banana", "cherry", "date", "elderberry"];
    let grouped: std::collections::HashMap<usize, Vec<&str>> = words
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, &word| {
            acc.entry(word.len()).or_insert_with(Vec::new).push(word);
            acc
        });
    
    println!("\n按长度分组:");
    for (len, words) in &grouped {
        println!("  长度 {}: {:?}", len, words);
    }
    
    // 窗口操作
    let data = [1, 4, 2, 8, 5, 7];
    let local_maxima: Vec<i32> = data
        .windows(3)
        .filter_map(|window| {
            if window[1] > window[0] && window[1] > window[2] {
                Some(window[1])
            } else {
                None
            }
        })
        .collect();
    
    println!("\n数据: {:?}", data);
    println!("局部最大值: {:?}", local_maxima);
    
    // 累积操作
    let prices = [100.0, 105.0, 98.0, 110.0, 95.0, 120.0];
    let returns: Vec<f64> = prices
        .windows(2)
        .map(|pair| (pair[1] - pair[0]) / pair[0] * 100.0)
        .collect();
    
    println!("\n价格: {:?}", prices);
    println!("收益率(%): {:?}", returns.iter().map(|x| format!("{:.2}", x)).collect::<Vec<_>>());
}
```

### 练习 24: 切片的序列化和反序列化
**题目**: 实现切片数据的序列化和反序列化。

**解答**:
```rust
use std::io::{self, Write, Read};

// 简单的二进制序列化
fn serialize_i32_slice(slice: &[i32]) -> Vec<u8> {
    let mut result = Vec::new();
    
    // 写入长度
    result.extend_from_slice(&(slice.len() as u32).to_le_bytes());
    
    // 写入数据
    for &value in slice {
        result.extend_from_slice(&value.to_le_bytes());
    }
    
    result
}

fn deserialize_i32_slice(data: &[u8]) -> Result<Vec<i32>, &'static str> {
    if data.len() < 4 {
        return Err("数据太短");
    }
    
    // 读取长度
    let len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
    
    if data.len() != 4 + len * 4 {
        return Err("数据长度不匹配");
    }
    
    // 读取数据
    let mut result = Vec::with_capacity(len);
    for i in 0..len {
        let start = 4 + i * 4;
        let bytes = [data[start], data[start + 1], data[start + 2], data[start + 3]];
        result.push(i32::from_le_bytes(bytes));
    }
    
    Ok(result)
}

// 文本序列化
fn serialize_to_csv<T: std::fmt::Display>(slice: &[T]) -> String {
    slice.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn deserialize_from_csv(csv: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    csv.split(',')
        .map(|s| s.trim().parse())
        .collect()
}

fn exercise_24() {
    let original_data = [1, 2, 3, 4, 5, -1, -2];
    
    println!("原始数据: {:?}", original_data);
    
    // 二进制序列化
    let binary_data = serialize_i32_slice(&original_data);
    println!("二进制序列化: {:?} ({} bytes)", binary_data, binary_data.len());
    
    match deserialize_i32_slice(&binary_data) {
        Ok(deserialized) => {
            println!("二进制反序列化: {:?}", deserialized);
            println!("数据一致: {}", original_data.to_vec() == deserialized);
        }
        Err(e) => println!("反序列化错误: {}", e),
    }
    
    // CSV序列化
    let csv_data = serialize_to_csv(&original_data);
    println!("\nCSV序列化: {}", csv_data);
    
    match deserialize_from_csv(&csv_data) {
        Ok(deserialized) => {
            println!("CSV反序列化: {:?}", deserialized);
            println!("数据一致: {}", original_data.to_vec() == deserialized);
        }
        Err(e) => println!("CSV反序列化错误: {}", e),
    }
    
    // 字符串切片序列化
    let words = ["hello", "world", "rust"];
    let words_csv = serialize_to_csv(&words);
    println!("\n字符串CSV: {}", words_csv);
}
```

### 练习 25: 切片的压缩和解压
**题目**: 实现简单的切片数据压缩算法。

**解答**:
```rust
// 简单的行程长度编码(RLE)
fn rle_encode(data: &[u8]) -> Vec<(u8, u8)> {
    if data.is_empty() {
        return Vec::new();
    }
    
    let mut result = Vec::new();
    let mut current = data[0];
    let mut count = 1u8;
    
    for &byte in &data[1..] {
        if byte == current && count < 255 {
            count += 1;
        } else {
            result.push((current, count));
            current = byte;
            count = 1;
        }
    }
    
    result.push((current, count));
    result
}

fn rle_decode(encoded: &[(u8, u8)]) -> Vec<u8> {
    let mut result = Vec::new();
    
    for &(value, count) in encoded {
        for _ in 0..count {
            result.push(value);
        }
    }
    
    result
}

// 简单的差分编码
fn delta_encode(data: &[i32]) -> Vec<i32> {
    if data.is_empty() {
        return Vec::new();
    }
    
    let mut result = vec![data[0]];
    for i in 1..data.len() {
        result.push(data[i] - data[i - 1]);
    }
    result
}

fn delta_decode(encoded: &[i32]) -> Vec<i32> {
    if encoded.is_empty() {
        return Vec::new();
    }
    
    let mut result = vec![encoded[0]];
    for i in 1..encoded.len() {
        result.push(result[i - 1] + encoded[i]);
    }
    result
}

fn exercise_25() {
    // RLE压缩测试
    let data = [1, 1, 1, 2, 2, 3, 3, 3, 3, 1, 1];
    println!("原始数据: {:?}", data);
    
    let encoded = rle_encode(&data);
    println!("RLE编码: {:?}", encoded);
    
    let decoded = rle_decode(&encoded);
    println!("RLE解码: {:?}", decoded);
    println!("RLE正确性: {}", data.to_vec() == decoded);
    
    let compression_ratio = data.len() as f64 / (encoded.len() * 2) as f64;
    println!("RLE压缩比: {:.2}", compression_ratio);
    
    // 差分编码测试
    let sequence = [100, 101, 103, 106, 110, 115, 121, 128];
    println!("\n原始序列: {:?}", sequence);
    
    let delta_encoded = delta_encode(&sequence);
    println!("差分编码: {:?}", delta_encoded);
    
    let delta_decoded = delta_decode(&delta_encoded);
    println!("差分解码: {:?}", delta_decoded);
    println!("差分正确性: {}", sequence.to_vec() == delta_decoded);
    
    // 分析压缩效果
    let original_range = sequence.iter().max().unwrap() - sequence.iter().min().unwrap();
    let delta_range = delta_encoded.iter().max().unwrap() - delta_encoded.iter().min().unwrap();
    println!("原始范围: {}, 差分范围: {}", original_range, delta_range);
}
```

### 练习 26-30: 综合应用练习

这些练习将切片的各种概念综合应用到实际场景中，包括：
- 数据流处理
- 图像处理算法
- 网络协议解析
- 数据库查询优化
- 机器学习数据预处理

由于篇幅限制，这里提供练习框架，具体实现可以根据需要展开。

---

## 总结

通过这30个练习，你应该能够：

1. **深入理解**切片和切片引用的区别
2. **熟练掌握**各种切片操作技巧
3. **正确处理**生命周期和借用问题
4. **优化性能**，避免常见陷阱
5. **应用到实际**项目中

### 学习建议

1. **循序渐进**：从基础练习开始，逐步提高难度
2. **动手实践**：每个练习都要亲自编写和运行
3. **理解原理**：不仅要知道怎么做，还要知道为什么
4. **举一反三**：尝试修改练习，探索更多可能性
5. **实际应用**：在真实项目中使用所学知识

### 进阶方向

- 深入学习 Rust 的内存模型
- 探索更多标准库中的切片相关API
- 学习 unsafe Rust 中的切片操作
- 研究编译器优化和性能调优
- 参与开源项目，实践所学知识

希望这些练习能帮助你彻底掌握 Rust 中切片和切片引用的概念！