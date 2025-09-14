//! Rust 迭代器 (Iterator) 深度分析
//! 
//! 本项目全面分析 Rust 迭代器的核心概念、实现原理和实际应用
//! 基于 https://course.rs/advance/functional-programing/iterator.html

use std::collections::HashMap;

fn main() {
    println!("=== Rust 迭代器深度分析 ===");
    
    // 1. 迭代器基础概念
    iterator_basics();
    
    // 2. 迭代器创建方式
    iterator_creation();
    
    // 3. 迭代器适配器 (Iterator Adaptors)
    iterator_adaptors();
    
    // 4. 消费者适配器 (Consumer Adaptors)
    consumer_adaptors();
    
    // 5. 自定义迭代器
    custom_iterators();
    
    // 6. 迭代器性能分析
    iterator_performance();
    
    // 7. 实际应用案例
    practical_examples();
    
    // 8. 高级迭代器模式
    advanced_iterator_patterns();
    
    // 9. 错误处理与迭代器
    error_handling_with_iterators();
    
    // 10. 并行迭代器简介
    parallel_iterators_intro();
    
    println!("\n=== 迭代器分析完成 ===");
}

/// 1. 迭代器基础概念
fn iterator_basics() {
    println!("\n--- 1. 迭代器基础概念 ---");
    
    // 1.1 什么是迭代器
    println!("\n1.1 迭代器定义:");
    println!("迭代器是一种设计模式，它提供了一种方法来顺序访问集合中的元素，而不暴露集合的内部表示。");
    
    // 1.2 Iterator trait
    println!("\n1.2 Iterator trait 核心定义:");
    println!("trait Iterator {{");
    println!("    type Item;  // 关联类型，表示迭代器产生的元素类型");
    println!("    fn next(&mut self) -> Option<Self::Item>;  // 核心方法");
    println!("}}");
    
    // 1.3 三种迭代器创建方式
    let vec = vec![1, 2, 3, 4, 5];
    
    // iter() - 创建不可变引用迭代器
    println!("\n1.3 三种迭代器创建方式:");
    println!("iter(): 创建 &T 类型的迭代器");
    for item in vec.iter() {
        print!("{} ", item);  // item 类型是 &i32
    }
    println!();
    
    // into_iter() - 创建所有权迭代器
    let vec2 = vec![1, 2, 3];
    println!("into_iter(): 创建 T 类型的迭代器");
    for item in vec2.into_iter() {
        print!("{} ", item);  // item 类型是 i32
    }
    println!();
    
    // iter_mut() - 创建可变引用迭代器
    let mut vec3 = vec![1, 2, 3];
    println!("iter_mut(): 创建 &mut T 类型的迭代器");
    for item in vec3.iter_mut() {
        *item *= 2;  // item 类型是 &mut i32
        print!("{} ", item);
    }
    println!();
    
    // 1.4 惰性求值 (Lazy Evaluation)
    println!("\n1.4 惰性求值特性:");
    let vec4 = vec![1, 2, 3, 4, 5];
    let iter = vec4.iter().map(|x| {
        println!("处理元素: {}", x);
        x * 2
    });
    println!("迭代器已创建，但尚未执行任何操作");
    
    // 只有在消费时才会执行
    println!("开始消费迭代器:");
    let _: Vec<_> = iter.collect();
}

/// 2. 迭代器创建方式详解
fn iterator_creation() {
    println!("\n--- 2. 迭代器创建方式详解 ---");
    
    // 2.1 从集合创建
    println!("\n2.1 从各种集合创建迭代器:");
    
    // Vec
    let vec = vec![1, 2, 3];
    let sum: i32 = vec.iter().sum();
    println!("Vec 迭代器求和: {}", sum);
    
    // Array
    let arr = [1, 2, 3, 4];
    let product: i32 = arr.iter().product();
    println!("Array 迭代器求积: {}", product);
    
    // HashMap
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    
    println!("HashMap 键值对迭代:");
    for (key, value) in map.iter() {
        println!("  {} => {}", key, value);
    }
    
    // 2.2 范围迭代器
    println!("\n2.2 范围迭代器:");
    let range_sum: i32 = (1..=10).sum();
    println!("1到10的和: {}", range_sum);
    
    let range_vec: Vec<i32> = (0..5).collect();
    println!("范围转换为Vec: {:?}", range_vec);
    
    // 2.3 字符串迭代器
    println!("\n2.3 字符串迭代器:");
    let text = "Hello, 世界!";
    
    // 字符迭代
    println!("字符迭代:");
    for ch in text.chars() {
        print!("'{}' ", ch);
    }
    println!();
    
    // 字节迭代
    println!("字节迭代:");
    for byte in text.bytes() {
        print!("{} ", byte);
    }
    println!();
    
    // 2.4 无限迭代器
    println!("\n2.4 无限迭代器:");
    
    // repeat - 重复值
    let repeated: Vec<i32> = std::iter::repeat(42).take(5).collect();
    println!("重复值: {:?}", repeated);
    
    // repeat_with - 重复函数调用
    let random_like: Vec<usize> = std::iter::repeat_with(|| fastrand::usize(1..100))
        .take(5)
        .collect();
    println!("重复函数调用: {:?}", random_like);
    
    // successors - 基于前一个值生成下一个值
    let fibonacci: Vec<u64> = std::iter::successors(Some((0, 1)), |(a, b)| {
        Some((*b, a + b))
    })
    .map(|(a, _)| a)
    .take(10)
    .collect();
    println!("斐波那契数列: {:?}", fibonacci);
}

/// 3. 迭代器适配器详解
fn iterator_adaptors() {
    println!("\n--- 3. 迭代器适配器 (Iterator Adaptors) ---");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 3.1 map - 转换每个元素
    println!("\n3.1 map - 转换每个元素:");
    let squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("平方: {:?}", squared);
    
    // 链式 map
    let processed: Vec<i32> = numbers.iter()
        .map(|&x| x * 2)     // 乘以2
        .map(|x| x + 1)      // 加1
        .map(|x| x * x)      // 平方
        .collect();
    println!("链式处理: {:?}", processed);
    
    // 3.2 filter - 过滤元素
    println!("\n3.2 filter - 过滤元素:");
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("偶数: {:?}", evens);
    
    // 复杂过滤条件
    let complex_filter: Vec<&i32> = numbers.iter()
        .filter(|&&x| x > 3)
        .filter(|&&x| x < 8)
        .filter(|&&x| x % 2 == 1)
        .collect();
    println!("3 < x < 8 且为奇数: {:?}", complex_filter);
    
    // 3.3 enumerate - 添加索引
    println!("\n3.3 enumerate - 添加索引:");
    for (index, value) in numbers.iter().enumerate() {
        if index < 5 {
            println!("  索引 {}: 值 {}", index, value);
        }
    }
    
    // 3.4 zip - 组合两个迭代器
    println!("\n3.4 zip - 组合迭代器:");
    let letters = vec!['a', 'b', 'c', 'd', 'e'];
    let zipped: Vec<(i32, char)> = numbers.iter()
        .take(5)
        .cloned()
        .zip(letters.iter().cloned())
        .collect();
    println!("组合结果: {:?}", zipped);
    
    // 3.5 take 和 skip
    println!("\n3.5 take 和 skip:");
    let first_five: Vec<&i32> = numbers.iter().take(5).collect();
    println!("前5个: {:?}", first_five);
    
    let skip_first_three: Vec<&i32> = numbers.iter().skip(3).collect();
    println!("跳过前3个: {:?}", skip_first_three);
    
    let middle: Vec<&i32> = numbers.iter().skip(3).take(4).collect();
    println!("中间4个: {:?}", middle);
    
    // 3.6 take_while 和 skip_while
    println!("\n3.6 take_while 和 skip_while:");
    let take_while_small: Vec<&i32> = numbers.iter().take_while(|&&x| x < 6).collect();
    println!("取小于6的: {:?}", take_while_small);
    
    let skip_while_small: Vec<&i32> = numbers.iter().skip_while(|&&x| x < 6).collect();
    println!("跳过小于6的: {:?}", skip_while_small);
    
    // 3.7 rev - 反转迭代器
    println!("\n3.7 rev - 反转迭代器:");
    let reversed: Vec<&i32> = numbers.iter().rev().collect();
    println!("反转: {:?}", reversed);
    
    // 3.8 cycle - 循环迭代器
    println!("\n3.8 cycle - 循环迭代器:");
    let cycled: Vec<i32> = vec![1, 2, 3].iter().cycle().take(10).cloned().collect();
    println!("循环10次: {:?}", cycled);
    
    // 3.9 chain - 连接迭代器
    println!("\n3.9 chain - 连接迭代器:");
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let chained: Vec<i32> = vec1.iter().chain(vec2.iter()).cloned().collect();
    println!("连接结果: {:?}", chained);
    
    // 3.10 flat_map - 扁平化映射
    println!("\n3.10 flat_map - 扁平化映射:");
    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let flattened: Vec<i32> = nested.iter().flat_map(|v| v.iter()).cloned().collect();
    println!("扁平化: {:?}", flattened);
    
    // 字符串分割扁平化
    let words = vec!["hello world", "rust programming"];
    let all_words: Vec<&str> = words.iter()
        .flat_map(|s| s.split_whitespace())
        .collect();
    println!("单词扁平化: {:?}", all_words);
}

/// 4. 消费者适配器详解
fn consumer_adaptors() {
    println!("\n--- 4. 消费者适配器 (Consumer Adaptors) ---");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 4.1 collect - 收集到集合
    println!("\n4.1 collect - 收集到集合:");
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("收集到Vec: {:?}", doubled);
    
    // 收集到HashMap
    let map: HashMap<i32, i32> = numbers.iter()
        .map(|&x| (x, x * x))
        .collect();
    println!("收集到HashMap: {:?}", map);
    
    // 收集到String
    let string: String = vec!['H', 'e', 'l', 'l', 'o'].iter().collect();
    println!("收集到String: {}", string);
    
    // 4.2 reduce 和 fold
    println!("\n4.2 reduce 和 fold:");
    
    // reduce - 无初始值的归约
    let sum_reduce = numbers.iter().cloned().reduce(|acc, x| acc + x);
    println!("reduce求和: {:?}", sum_reduce);
    
    // fold - 有初始值的归约
    let sum_fold = numbers.iter().fold(0, |acc, x| acc + x);
    println!("fold求和: {}", sum_fold);
    
    // 复杂fold示例 - 计算统计信息
    let stats = numbers.iter().fold((0, 0, i32::MAX, i32::MIN), |(sum, count, min, max), &x| {
        (sum + x, count + 1, min.min(x), max.max(x))
    });
    println!("统计信息 (sum, count, min, max): {:?}", stats);
    
    // 4.3 for_each - 对每个元素执行操作
    println!("\n4.3 for_each - 对每个元素执行操作:");
    print!("for_each输出: ");
    numbers.iter().take(5).for_each(|x| print!("{} ", x));
    println!();
    
    // 4.4 find 和 find_map
    println!("\n4.4 find 和 find_map:");
    let found = numbers.iter().find(|&&x| x > 5);
    println!("找到第一个大于5的数: {:?}", found);
    
    let found_mapped = numbers.iter().find_map(|&x| {
        if x > 7 { Some(x * 10) } else { None }
    });
    println!("找到并映射: {:?}", found_mapped);
    
    // 4.5 any 和 all
    println!("\n4.5 any 和 all:");
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    println!("是否有偶数: {}", has_even);
    
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("是否都是正数: {}", all_positive);
    
    let all_small = numbers.iter().all(|&x| x < 5);
    println!("是否都小于5: {}", all_small);
    
    // 4.6 count
    println!("\n4.6 count:");
    let even_count = numbers.iter().filter(|&&x| x % 2 == 0).count();
    println!("偶数个数: {}", even_count);
    
    // 4.7 min, max, min_by, max_by
    println!("\n4.7 min, max, min_by, max_by:");
    let min_val = numbers.iter().min();
    let max_val = numbers.iter().max();
    println!("最小值: {:?}, 最大值: {:?}", min_val, max_val);
    
    // 自定义比较
    let words = vec!["apple", "banana", "cherry", "date"];
    let longest = words.iter().max_by(|a, b| a.len().cmp(&b.len()));
    println!("最长单词: {:?}", longest);
    
    // 4.8 position 和 rposition
    println!("\n4.8 position 和 rposition:");
    let pos = numbers.iter().position(|&x| x == 5);
    println!("数字5的位置: {:?}", pos);
    
    let rpos = numbers.iter().rposition(|&x| x > 5);
    println!("最后一个大于5的数的位置: {:?}", rpos);
    
    // 4.9 sum 和 product
    println!("\n4.9 sum 和 product:");
    let sum: i32 = numbers.iter().sum();
    let product: i64 = numbers.iter().map(|&x| x as i64).product();
    println!("和: {}, 积: {}", sum, product);
}

/// 5. 自定义迭代器
fn custom_iterators() {
    println!("\n--- 5. 自定义迭代器 ---");
    
    // 5.1 简单的计数器迭代器
    println!("\n5.1 计数器迭代器:");
    
    struct Counter {
        current: usize,
        max: usize,
    }
    
    impl Counter {
        fn new(max: usize) -> Counter {
            Counter { current: 0, max }
        }
    }
    
    impl Iterator for Counter {
        type Item = usize;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.max {
                let current = self.current;
                self.current += 1;
                Some(current)
            } else {
                None
            }
        }
    }
    
    let counter = Counter::new(5);
    let values: Vec<usize> = counter.collect();
    println!("计数器输出: {:?}", values);
    
    // 5.2 斐波那契迭代器
    println!("\n5.2 斐波那契迭代器:");
    
    struct Fibonacci {
        current: u64,
        next: u64,
    }
    
    impl Fibonacci {
        fn new() -> Fibonacci {
            Fibonacci { current: 0, next: 1 }
        }
    }
    
    impl Iterator for Fibonacci {
        type Item = u64;
        
        fn next(&mut self) -> Option<Self::Item> {
            let current = self.current;
            self.current = self.next;
            self.next = current + self.next;
            Some(current)
        }
    }
    
    let fib: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("斐波那契数列: {:?}", fib);
    
    // 5.3 自定义集合的迭代器
    println!("\n5.3 自定义集合迭代器:");
    
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
        
        fn iter(&self) -> MyVecIter<T> {
            MyVecIter {
                vec: self,
                index: 0,
            }
        }
    }
    
    struct MyVecIter<'a, T> {
        vec: &'a MyVec<T>,
        index: usize,
    }
    
    impl<'a, T> Iterator for MyVecIter<'a, T> {
        type Item = &'a T;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.vec.data.len() {
                let item = &self.vec.data[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }
    
    let mut my_vec = MyVec::new();
    my_vec.push(1);
    my_vec.push(2);
    my_vec.push(3);
    
    let collected: Vec<&i32> = my_vec.iter().collect();
    println!("自定义集合迭代: {:?}", collected);
    
    // 5.4 实现 IntoIterator
    println!("\n5.4 实现 IntoIterator:");
    
    impl<T> IntoIterator for MyVec<T> {
        type Item = T;
        type IntoIter = std::vec::IntoIter<T>;
        
        fn into_iter(self) -> Self::IntoIter {
            self.data.into_iter()
        }
    }
    
    let mut my_vec2 = MyVec::new();
    my_vec2.push("hello");
    my_vec2.push("world");
    
    for item in my_vec2 {
        print!("{} ", item);
    }
    println!();
}

/// 6. 迭代器性能分析
fn iterator_performance() {
    println!("\n--- 6. 迭代器性能分析 ---");
    
    // 6.1 零成本抽象
    println!("\n6.1 零成本抽象 (Zero-Cost Abstractions):");
    println!("Rust 的迭代器是零成本抽象，编译器会将迭代器链优化为等效的循环代码。");
    
    let data = (0..1000000).collect::<Vec<i32>>();
    
    // 使用迭代器的函数式风格
    let start = std::time::Instant::now();
    let sum1: i64 = data.iter().map(|&x| x as i64).filter(|&x| x % 2 == 0).sum();
    let duration1 = start.elapsed();
    
    // 使用传统循环
    let start = std::time::Instant::now();
    let mut sum2: i64 = 0;
    for &x in &data {
        let x = x as i64;
        if x % 2 == 0 {
            sum2 += x;
        }
    }
    let duration2 = start.elapsed();
    
    println!("迭代器方式: sum = {}, 耗时: {:?}", sum1, duration1);
    println!("循环方式: sum = {}, 耗时: {:?}", sum2, duration2);
    println!("性能差异微乎其微，但迭代器更具表达力和安全性。");
    
    // 6.2 惰性求值的优势
    println!("\n6.2 惰性求值的优势:");
    println!("迭代器只在需要时才计算，可以处理无限序列和大数据集。");
    
    // 无限序列示例
    let first_even_square_over_1000 = (1..)
        .map(|x| x * x)
        .filter(|&x| x % 2 == 0)
        .find(|&x| x > 1000);
    
    println!("第一个大于1000的偶数平方: {:?}", first_even_square_over_1000);
    
    // 6.3 内存效率
    println!("\n6.3 内存效率:");
    println!("迭代器适配器不会创建中间集合，节省内存。");
    
    // 低效方式 - 创建多个中间Vec
    let _inefficient = {
        let step1: Vec<i32> = (1..1000).collect();
        let step2: Vec<i32> = step1.iter().map(|&x| x * 2).collect();
        let step3: Vec<i32> = step2.iter().filter(|&&x| x % 3 == 0).cloned().collect();
        step3.len()
    };
    
    // 高效方式 - 使用迭代器链
    let efficient = (1..1000)
        .map(|x| x * 2)
        .filter(|&x| x % 3 == 0)
        .count();
    
    println!("高效方式结果: {}", efficient);
}

/// 7. 实际应用案例
fn practical_examples() {
    println!("\n--- 7. 实际应用案例 ---");
    
    // 7.1 文本处理
    println!("\n7.1 文本处理案例:");
    
    let text = "Hello world! This is a sample text for processing. 
                It contains multiple lines and various punctuation marks.";
    
    // 统计单词频率
    let word_count: HashMap<String, usize> = text
        .split_whitespace()
        .map(|word| word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase())
        .filter(|word| !word.is_empty())
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        });
    
    println!("单词频率统计:");
    for (word, count) in word_count.iter().take(5) {
        println!("  '{}': {}", word, count);
    }
    
    // 7.2 数据分析
    println!("\n7.2 数据分析案例:");
    
    #[derive(Debug, Clone)]
    struct Student {
        name: String,
        age: u8,
        grade: f64,
        subject: String,
    }
    
    let students = vec![
        Student { name: "Alice".to_string(), age: 20, grade: 85.5, subject: "Math".to_string() },
        Student { name: "Bob".to_string(), age: 19, grade: 92.0, subject: "Physics".to_string() },
        Student { name: "Charlie".to_string(), age: 21, grade: 78.5, subject: "Math".to_string() },
        Student { name: "Diana".to_string(), age: 20, grade: 88.0, subject: "Chemistry".to_string() },
        Student { name: "Eve".to_string(), age: 22, grade: 95.5, subject: "Physics".to_string() },
    ];
    
    // 按科目分组计算平均分
    let subject_averages: HashMap<String, f64> = students
        .iter()
        .fold(HashMap::new(), |mut acc, student| {
            let entry = acc.entry(student.subject.clone()).or_insert((0.0, 0));
            entry.0 += student.grade;
            entry.1 += 1;
            acc
        })
        .into_iter()
        .map(|(subject, (total, count))| (subject, total / count as f64))
        .collect();
    
    println!("各科目平均分:");
    for (subject, avg) in subject_averages {
        println!("  {}: {:.2}", subject, avg);
    }
    
    // 找出成绩最高的学生
    let top_student = students
        .iter()
        .max_by(|a, b| a.grade.partial_cmp(&b.grade).unwrap());
    
    println!("成绩最高的学生: {:?}", top_student.map(|s| &s.name));
    
    // 7.3 配置文件处理
    println!("\n7.3 配置文件处理案例:");
    
    let config_text = "# Configuration file\nname=MyApp\nversion=1.0.0\ndebug=true\nport=8080\n# End of config";
    
    let config: HashMap<String, String> = config_text
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
            } else {
                None
            }
        })
        .collect();
    
    println!("解析的配置:");
    for (key, value) in config {
        println!("  {} = {}", key, value);
    }
    
    // 7.4 数据转换管道
    println!("\n7.4 数据转换管道:");
    
    let raw_data = "1,2,3,invalid,5,6,7,not_a_number,9,10";
    
    let processed_data: Vec<i32> = raw_data
        .split(',')
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .filter(|&x| x > 0)
        .map(|x| x * 2)
        .filter(|&x| x < 20)
        .collect();
    
    println!("处理后的数据: {:?}", processed_data);
}

/// 8. 高级迭代器模式
fn advanced_iterator_patterns() {
    println!("\n--- 8. 高级迭代器模式 ---");
    
    // 8.1 迭代器组合
    println!("\n8.1 复杂迭代器组合:");
    
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 复杂的数据处理管道
    let result: Vec<String> = data
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)  // 偶数索引
        .map(|(_, &value)| value)
        .filter(|&x| x > 2)           // 值大于2
        .map(|x| x * x)               // 平方
        .take_while(|&x| x < 50)      // 小于50
        .map(|x| format!("#{}", x))   // 格式化
        .collect();
    
    println!("复杂处理结果: {:?}", result);
    
    // 8.2 分组和分区
    println!("\n8.2 分组和分区:");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // partition - 分区
    let (evens, odds): (Vec<i32>, Vec<i32>) = numbers
        .iter()
        .cloned()
        .partition(|&x| x % 2 == 0);
    
    println!("偶数: {:?}", evens);
    println!("奇数: {:?}", odds);
    
    // 手动分组
    let grouped: HashMap<bool, Vec<i32>> = numbers
        .iter()
        .cloned()
        .fold(HashMap::new(), |mut acc, x| {
            acc.entry(x % 2 == 0).or_insert_with(Vec::new).push(x);
            acc
        });
    
    println!("分组结果: {:?}", grouped);
    
    // 8.3 窗口操作
    println!("\n8.3 窗口操作:");
    
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    
    // 滑动窗口求和
    let window_sums: Vec<i32> = data
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();
    
    println!("滑动窗口(3)求和: {:?}", window_sums);
    
    // 分块处理
    let chunks: Vec<Vec<&i32>> = data
        .chunks(3)
        .map(|chunk| chunk.iter().collect())
        .collect();
    
    println!("分块(3): {:?}", chunks);
    
    // 8.4 迭代器状态管理
    println!("\n8.4 迭代器状态管理:");
    
    // 使用 scan 进行状态累积
    let running_sum: Vec<i32> = vec![1, 2, 3, 4, 5]
        .iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect();
    
    println!("累积和: {:?}", running_sum);
    
    // 复杂状态管理 - 移动平均
    let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let window_size = 3;
    
    let moving_average: Vec<f64> = values
        .windows(window_size)
        .map(|window| window.iter().sum::<f64>() / window.len() as f64)
        .collect();
    
    println!("移动平均({}): {:?}", window_size, moving_average);
}

/// 9. 错误处理与迭代器
fn error_handling_with_iterators() {
    println!("\n--- 9. 错误处理与迭代器 ---");
    
    // 9.1 处理 Result 的迭代器
    println!("\n9.1 处理 Result 的迭代器:");
    
    let string_numbers = vec!["1", "2", "invalid", "4", "5", "not_a_number", "7"];
    
    // filter_map 过滤错误
    let valid_numbers: Vec<i32> = string_numbers
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    
    println!("有效数字: {:?}", valid_numbers);
    
    // 收集所有结果（包括错误）
    let all_results: Vec<Result<i32, std::num::ParseIntError>> = string_numbers
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();
    
    println!("所有解析结果:");
    for (i, result) in all_results.iter().enumerate() {
        match result {
            Ok(num) => println!("  [{}]: 成功 -> {}", i, num),
            Err(e) => println!("  [{}]: 错误 -> {}", i, e),
        }
    }
    
    // 9.2 使用 collect 处理 Result
    println!("\n9.2 使用 collect 处理 Result:");
    
    let good_strings = vec!["1", "2", "3", "4", "5"];
    let bad_strings = vec!["1", "2", "invalid", "4", "5"];
    
    // 全部成功的情况
    let good_result: Result<Vec<i32>, _> = good_strings
        .iter()
        .map(|s| s.parse())
        .collect();
    
    println!("全部成功: {:?}", good_result);
    
    // 包含错误的情况
    let bad_result: Result<Vec<i32>, _> = bad_strings
        .iter()
        .map(|s| s.parse())
        .collect();
    
    println!("包含错误: {:?}", bad_result.is_err());
    
    // 9.3 分离成功和失败
    println!("\n9.3 分离成功和失败:");
    
    let mixed_strings = vec!["1", "2", "invalid", "4", "5", "bad", "7"];
    
    let (successes, failures): (Vec<_>, Vec<_>) = mixed_strings
        .iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    
    let successes: Vec<i32> = successes.into_iter().map(Result::unwrap).collect();
    let failures: Vec<_> = failures.into_iter().map(Result::unwrap_err).collect();
    
    println!("成功的解析: {:?}", successes);
    println!("失败的解析数量: {}", failures.len());
    
    // 9.4 Option 的迭代器处理
    println!("\n9.4 Option 的迭代器处理:");
    
    let maybe_numbers = vec![Some(1), None, Some(3), Some(4), None, Some(6)];
    
    // flatten 展开 Option
    let flattened: Vec<i32> = maybe_numbers.iter().flatten().cloned().collect();
    println!("展开 Option: {:?}", flattened);
    
    // 统计 Some 和 None
    let some_count = maybe_numbers.iter().filter(|opt| opt.is_some()).count();
    let none_count = maybe_numbers.iter().filter(|opt| opt.is_none()).count();
    println!("Some 数量: {}, None 数量: {}", some_count, none_count);
}

/// 10. 并行迭代器简介
fn parallel_iterators_intro() {
    println!("\n--- 10. 并行迭代器简介 ---");
    
    println!("\n10.1 Rayon 并行迭代器概念:");
    println!("Rayon 是 Rust 的数据并行库，提供了并行迭代器。");
    println!("主要特点:");
    println!("  - par_iter(): 创建并行迭代器");
    println!("  - 自动工作窃取调度");
    println!("  - 零成本抽象");
    println!("  - 线程安全");
    
    // 模拟并行处理概念（不使用 rayon 依赖）
    println!("\n10.2 并行处理示例概念:");
    
    let large_data: Vec<i32> = (0..1000000).collect();
    
    // 串行处理
    let start = std::time::Instant::now();
    let serial_sum: i64 = large_data.iter().map(|&x| (x as i64) * (x as i64)).sum();
    let serial_time = start.elapsed();
    
    println!("串行处理: sum = {}, 耗时: {:?}", serial_sum, serial_time);
    
    // 手动分块并行概念（简化版）
    let start = std::time::Instant::now();
    let chunk_size = large_data.len() / 4;
    let chunks: Vec<_> = large_data.chunks(chunk_size).collect();
    
    // 在实际应用中，这里会使用线程或 rayon
    let parallel_sum: i64 = chunks
        .iter()
        .map(|chunk| chunk.iter().map(|&x| (x as i64) * (x as i64)).sum::<i64>())
        .sum();
    
    let parallel_time = start.elapsed();
    
    println!("模拟并行处理: sum = {}, 耗时: {:?}", parallel_sum, parallel_time);
    
    println!("\n10.3 何时使用并行迭代器:");
    println!("  - 计算密集型任务");
    println!("  - 大数据集处理");
    println!("  - 独立的元素处理");
    println!("  - CPU 密集型操作");
    
    println!("\n10.4 并行迭代器注意事项:");
    println!("  - 线程创建开销");
    println!("  - 数据竞争避免");
    println!("  - 内存访问模式");
    println!("  - 负载均衡");
}

// 简单的随机数生成器（避免外部依赖）
mod fastrand {
    use std::cell::Cell;
    
    thread_local! {
        static RNG: Cell<u64> = Cell::new(1);
    }
    
    pub fn usize(range: std::ops::Range<usize>) -> usize {
        RNG.with(|rng| {
            let mut x = rng.get();
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            rng.set(x);
            (x as usize) % (range.end - range.start) + range.start
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_iterator_basics() {
        let vec = vec![1, 2, 3, 4, 5];
        let sum: i32 = vec.iter().sum();
        assert_eq!(sum, 15);
    }
    
    #[test]
    fn test_iterator_adaptors() {
        let vec = vec![1, 2, 3, 4, 5];
        let doubled: Vec<i32> = vec.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    }
    
    #[test]
    fn test_custom_iterator() {
        struct Counter {
            current: usize,
            max: usize,
        }
        
        impl Counter {
            fn new(max: usize) -> Counter {
                Counter { current: 0, max }
            }
        }
        
        impl Iterator for Counter {
            type Item = usize;
            
            fn next(&mut self) -> Option<Self::Item> {
                if self.current < self.max {
                    let current = self.current;
                    self.current += 1;
                    Some(current)
                } else {
                    None
                }
            }
        }
        
        let counter = Counter::new(3);
        let values: Vec<usize> = counter.collect();
        assert_eq!(values, vec![0, 1, 2]);
    }
    
    #[test]
    fn test_error_handling() {
        let strings = vec!["1", "2", "invalid", "4"];
        let numbers: Vec<i32> = strings
            .iter()
            .filter_map(|s| s.parse().ok())
            .collect();
        assert_eq!(numbers, vec![1, 2, 4]);
    }
    
    #[test]
    fn test_complex_pipeline() {
        let result: Vec<i32> = (1..=10)
            .filter(|&x| x % 2 == 0)
            .map(|x| x * x)
            .filter(|&x| x < 50)
            .collect();
        assert_eq!(result, vec![4, 16, 36]);
    }
}
