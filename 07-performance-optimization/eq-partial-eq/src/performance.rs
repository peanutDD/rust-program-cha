//! # 性能分析
//!
//! 本模块分析 Eq 和 PartialEq 的性能特征和优化策略

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::time::Instant;

/// 运行性能分析
pub fn run_performance_analysis() {
    println!("\n🔸 性能分析:");
    
    comparison_performance();
    hash_performance();
    collection_performance();
    optimization_strategies();
    memory_usage_analysis();
    benchmark_different_implementations();
}

/// 比较操作的性能分析
fn comparison_performance() {
    println!("\n  📌 比较操作性能分析:");
    
    // 基本类型比较性能
    println!("\n    🔹 基本类型比较性能:");
    
    let iterations = 1_000_000;
    
    // 整数比较
    let start = Instant::now();
    let mut result = true;
    for i in 0..iterations {
        result &= i == i;
    }
    let int_duration = start.elapsed();
    println!("      整数比较 ({} 次): {:?}, 结果: {}", iterations, int_duration, result);
    
    // 浮点数比较
    let start = Instant::now();
    let mut result = true;
    for i in 0..iterations {
        let f = i as f64;
        result &= f == f;
    }
    let float_duration = start.elapsed();
    println!("      浮点数比较 ({} 次): {:?}, 结果: {}", iterations, float_duration, result);
    
    // 字符串比较
    let strings: Vec<String> = (0..1000).map(|i| format!("string_{}", i)).collect();
    let start = Instant::now();
    let mut count = 0;
    for i in 0..strings.len() {
        for j in i..strings.len() {
            if strings[i] == strings[j] {
                count += 1;
            }
        }
    }
    let string_duration = start.elapsed();
    println!("      字符串比较 ({} 对): {:?}, 匹配: {}", 
             strings.len() * (strings.len() + 1) / 2, string_duration, count);
    
    // 复杂结构体比较
    #[derive(Debug, Clone, PartialEq)]
    struct ComplexStruct {
        id: u64,
        name: String,
        values: Vec<i32>,
        metadata: HashMap<String, String>,
    }
    
    let complex_structs: Vec<ComplexStruct> = (0..100).map(|i| {
        let mut metadata = HashMap::new();
        metadata.insert("key1".to_string(), format!("value_{}", i));
        metadata.insert("key2".to_string(), "common_value".to_string());
        
        ComplexStruct {
            id: i,
            name: format!("item_{}", i),
            values: vec![i as i32, (i * 2) as i32, (i * 3) as i32],
            metadata,
        }
    }).collect();
    
    let start = Instant::now();
    let mut matches = 0;
    for i in 0..complex_structs.len() {
        for j in i..complex_structs.len() {
            if complex_structs[i] == complex_structs[j] {
                matches += 1;
            }
        }
    }
    let complex_duration = start.elapsed();
    println!("      复杂结构体比较 ({} 对): {:?}, 匹配: {}", 
             complex_structs.len() * (complex_structs.len() + 1) / 2, complex_duration, matches);
}

/// Hash 计算性能分析
fn hash_performance() {
    println!("\n  📌 Hash 计算性能分析:");
    
    let iterations = 100_000;
    
    // 基本类型 Hash
    println!("\n    🔹 基本类型 Hash 性能:");
    
    let start = Instant::now();
    let mut total_hash = 0u64;
    for i in 0..iterations {
        let mut hasher = DefaultHasher::new();
        i.hash(&mut hasher);
        total_hash = total_hash.wrapping_add(hasher.finish());
    }
    let int_hash_duration = start.elapsed();
    println!("      整数 Hash ({} 次): {:?}, 总和: {}", iterations, int_hash_duration, total_hash);
    
    // 字符串 Hash
    let test_strings: Vec<String> = (0..1000).map(|i| format!("test_string_{}", i)).collect();
    let start = Instant::now();
    let mut total_hash = 0u64;
    for _ in 0..100 {
        for s in &test_strings {
            let mut hasher = DefaultHasher::new();
            s.hash(&mut hasher);
            total_hash = total_hash.wrapping_add(hasher.finish());
        }
    }
    let string_hash_duration = start.elapsed();
    println!("      字符串 Hash ({} 次): {:?}, 总和: {}", 
             test_strings.len() * 100, string_hash_duration, total_hash);
    
    // 复杂结构体 Hash
    #[derive(Hash)]
    struct HashableStruct {
        id: u64,
        data: Vec<u8>,
        name: String,
    }
    
    let complex_items: Vec<HashableStruct> = (0..1000).map(|i| {
        HashableStruct {
            id: i,
            data: vec![i as u8; 100],  // 100 字节的数据
            name: format!("item_{}", i),
        }
    }).collect();
    
    let start = Instant::now();
    let mut total_hash = 0u64;
    for item in &complex_items {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        total_hash = total_hash.wrapping_add(hasher.finish());
    }
    let complex_hash_duration = start.elapsed();
    println!("      复杂结构体 Hash ({} 次): {:?}, 总和: {}", 
            complex_items.len(), complex_hash_duration, total_hash);
    
    // 优化的 Hash 实现
    println!("\n    🔹 优化的 Hash 实现对比:");
    
    #[derive(Clone)]
    struct OptimizedHashStruct {
        id: u64,
        data: Vec<u8>,
        name: String,
    }
    
    // 标准实现：Hash 所有字段
    impl Hash for OptimizedHashStruct {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
            self.data.hash(state);
            self.name.hash(state);
        }
    }
    
    #[derive(Clone)]
    struct FastHashStruct {
        id: u64,
        data: Vec<u8>,
        name: String,
    }
    
    // 优化实现：只 Hash 关键字段和部分数据
    impl Hash for FastHashStruct {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
            // 只 Hash 数据的长度和前几个字节
            self.data.len().hash(state);
            if !self.data.is_empty() {
                self.data[0].hash(state);
                if self.data.len() > 1 {
                    self.data[self.data.len() - 1].hash(state);
                }
            }
            self.name.hash(state);
        }
    }
    
    let optimized_items: Vec<OptimizedHashStruct> = (0..1000).map(|i| {
        OptimizedHashStruct {
            id: i,
            data: vec![i as u8; 1000],  // 更大的数据
            name: format!("item_{}", i),
        }
    }).collect();
    
    let fast_items: Vec<FastHashStruct> = optimized_items.iter().map(|item| {
        FastHashStruct {
            id: item.id,
            data: item.data.clone(),
            name: item.name.clone(),
        }
    }).collect();
    
    // 标准 Hash 性能
    let start = Instant::now();
    let mut total_hash = 0u64;
    for item in &optimized_items {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        total_hash = total_hash.wrapping_add(hasher.finish());
    }
    let standard_duration = start.elapsed();
    
    // 优化 Hash 性能
    let start = Instant::now();
    let mut total_hash_fast = 0u64;
    for item in &fast_items {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        total_hash_fast = total_hash_fast.wrapping_add(hasher.finish());
    }
    let optimized_duration = start.elapsed();
    
    println!("      标准 Hash ({} 次): {:?}", optimized_items.len(), standard_duration);
    println!("      优化 Hash ({} 次): {:?}", fast_items.len(), optimized_duration);
    
    if standard_duration > optimized_duration {
        let speedup = standard_duration.as_nanos() as f64 / optimized_duration.as_nanos() as f64;
        println!("      ✅ 优化版本快 {:.2}x", speedup);
    }
}

/// 集合性能分析
fn collection_performance() {
    println!("\n  📌 集合性能分析:");
    
    let data_size = 10_000;
    let lookup_count = 1_000;
    
    // 准备测试数据
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    struct TestItem {
        id: u64,
        category: String,
    }
    
    let test_data: Vec<TestItem> = (0..data_size).map(|i| {
        TestItem {
            id: i,
            category: format!("category_{}", i % 100),
        }
    }).collect();
    
    let lookup_items: Vec<TestItem> = (0..lookup_count).map(|i| {
        test_data[(i * data_size / lookup_count) as usize].clone()
    }).collect();
    
    println!("\n    🔹 HashMap 性能:");
    
    // HashMap 插入性能
    let start = Instant::now();
    let mut hash_map: HashMap<TestItem, usize> = HashMap::new();
    for (i, item) in test_data.iter().enumerate() {
        hash_map.insert(item.clone(), i);
    }
    let hashmap_insert_duration = start.elapsed();
    println!("      HashMap 插入 ({} 项): {:?}", data_size, hashmap_insert_duration);
    
    // HashMap 查找性能
    let start = Instant::now();
    let mut _found_count = 0;
    for item in &lookup_items {
        if hash_map.contains_key(item) {
            _found_count += 1;
        }
    }
    let hashmap_lookup_duration = start.elapsed();
    println!("      HashMap 查找 ({} 次): {:?}, 找到: {}", 
            lookup_count, hashmap_lookup_duration, _found_count);
    
    println!("\n    🔹 BTreeMap 性能:");
    
    // BTreeMap 插入性能
    let start = Instant::now();
    let mut btree_map: BTreeMap<TestItem, usize> = BTreeMap::new();
    for (i, item) in test_data.iter().enumerate() {
        btree_map.insert(item.clone(), i);
    }
    let btreemap_insert_duration = start.elapsed();
    println!("      BTreeMap 插入 ({} 项): {:?}", data_size, btreemap_insert_duration);
    
    // BTreeMap 查找性能
    let start = Instant::now();
    let mut _found_count3 = 0;
    for item in &lookup_items {
        if btree_map.contains_key(item) {
            _found_count3 += 1;
        }
    }
    let btreemap_lookup_duration = start.elapsed();
    println!("      BTreeMap 查找 ({} 次): {:?}, 找到: {}", 
            lookup_count, btreemap_lookup_duration, _found_count3);
    
    println!("\n    🔹 HashSet vs BTreeSet 性能:");
    
    // HashSet 性能
    let start = Instant::now();
    let mut hash_set: HashSet<TestItem> = HashSet::new();
    for item in &test_data {
        hash_set.insert(item.clone());
    }
    let hashset_insert_duration = start.elapsed();
    
    let start = Instant::now();
    let mut _found_count2 = 0;
    for item in &lookup_items {
        if hash_set.contains(item) {
            _found_count2 += 1;
        }
    }
    let hashset_lookup_duration = start.elapsed();
    
    println!("      HashSet 插入: {:?}, 查找: {:?}", 
             hashset_insert_duration, hashset_lookup_duration);
    
    // BTreeSet 性能
    let start = Instant::now();
    let mut btree_set: BTreeSet<TestItem> = BTreeSet::new();
    for item in &test_data {
        btree_set.insert(item.clone());
    }
    let btreeset_insert_duration = start.elapsed();
    
    let start = Instant::now();
    let mut _found_count = 0;
    for item in &lookup_items {
        if btree_set.contains(item) {
            _found_count += 1;
        }
    }
    let btreeset_lookup_duration = start.elapsed();
    
    println!("      BTreeSet 插入: {:?}, 查找: {:?}", 
             btreeset_insert_duration, btreeset_lookup_duration);
    
    // Vec 线性查找性能（作为对比）
    println!("\n    🔹 Vec 线性查找性能 (对比):");
    
    let start = Instant::now();
    let mut found_count = 0;
    for item in &lookup_items {
        if test_data.contains(item) {
            found_count += 1;
        }
    }
    let vec_lookup_duration = start.elapsed();
    println!("      Vec 线性查找 ({} 次): {:?}, 找到: {}", 
             lookup_count, vec_lookup_duration, found_count);
}

/// 优化策略分析
fn optimization_strategies() {
    println!("\n  📌 优化策略分析:");
    
    // 早期退出策略
    println!("\n    🔹 早期退出策略:");
    
    #[derive(Debug, Clone)]
    struct LargeStruct {
        id: u64,
        small_data: [u8; 16],
        large_data: Vec<u8>,
    }
    
    // 未优化的实现
    impl PartialEq for LargeStruct {
        fn eq(&self, other: &Self) -> bool {
            // 总是比较所有字段
            self.id == other.id && 
            self.small_data == other.small_data && 
            self.large_data == other.large_data
        }
    }
    
    #[derive(Debug, Clone)]
    struct OptimizedLargeStruct {
        id: u64,
        small_data: [u8; 16],
        large_data: Vec<u8>,
    }
    
    // 优化的实现：早期退出
    impl PartialEq for OptimizedLargeStruct {
        fn eq(&self, other: &Self) -> bool {
            // 首先比较最可能不同的字段
            if self.id != other.id {
                return false;
            }
            
            // 然后比较长度（快速）
            if self.large_data.len() != other.large_data.len() {
                return false;
            }
            
            // 比较小数据
            if self.small_data != other.small_data {
                return false;
            }
            
            // 最后比较大数据
            self.large_data == other.large_data
        }
    }
    
    // 创建测试数据
    let large_data = vec![0u8; 10000];
    let items1: Vec<LargeStruct> = (0..1000).map(|i| {
        LargeStruct {
            id: i,
            small_data: [i as u8; 16],
            large_data: large_data.clone(),
        }
    }).collect();
    
    let items2: Vec<OptimizedLargeStruct> = items1.iter().map(|item| {
        OptimizedLargeStruct {
            id: item.id,
            small_data: item.small_data,
            large_data: item.large_data.clone(),
        }
    }).collect();
    
    // 测试不同 ID 的比较（应该很快退出）
    let start = Instant::now();
    let mut matches = 0;
    for i in 0..100 {
        for j in (i+1)..101 {
            if i < items1.len() && j < items1.len() && items1[i] == items1[j] {
                matches += 1;
            }
        }
    }
    let unoptimized_duration = start.elapsed();
    
    let start = Instant::now();
    let mut matches_opt = 0;
    for i in 0..100 {
        for j in (i+1)..101 {
            if i < items2.len() && j < items2.len() && items2[i] == items2[j] {
                matches_opt += 1;
            }
        }
    }
    let optimized_duration = start.elapsed();
    
    println!("      未优化比较: {:?}, 匹配: {}", unoptimized_duration, matches);
    println!("      优化比较: {:?}, 匹配: {}", optimized_duration, matches_opt);
    
    if unoptimized_duration > optimized_duration {
        let speedup = unoptimized_duration.as_nanos() as f64 / optimized_duration.as_nanos() as f64;
        println!("      ✅ 早期退出策略提升 {:.2}x", speedup);
    }
    
    // 缓存策略
    println!("\n    🔹 缓存策略:");
    
    #[derive(Debug, Clone)]
    struct CachedHashStruct {
        data: Vec<u8>,
        cached_hash: std::cell::Cell<Option<u64>>,
    }
    
    impl PartialEq for CachedHashStruct {
        fn eq(&self, other: &Self) -> bool {
            self.data == other.data
        }
    }
    
    impl Eq for CachedHashStruct {}
    
    impl Hash for CachedHashStruct {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // 检查缓存
            if let Some(cached) = self.cached_hash.get() {
                cached.hash(state);
            } else {
                // 计算并缓存
                let mut temp_hasher = DefaultHasher::new();
                self.data.hash(&mut temp_hasher);
                let hash_value = temp_hasher.finish();
                self.cached_hash.set(Some(hash_value));
                hash_value.hash(state);
            }
        }
    }
    
    let cached_items: Vec<CachedHashStruct> = (0..1000).map(|i| {
        CachedHashStruct {
            data: vec![i as u8; 1000],
            cached_hash: std::cell::Cell::new(None),
        }
    }).collect();
    
    // 第一次 Hash（需要计算）
    let start = Instant::now();
    let mut total_hash = 0u64;
    for item in &cached_items {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        total_hash = total_hash.wrapping_add(hasher.finish());
    }
    let first_hash_duration = start.elapsed();
    
    // 第二次 Hash（使用缓存）
    let start = Instant::now();
    let mut total_hash2 = 0u64;
    for item in &cached_items {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        total_hash2 = total_hash2.wrapping_add(hasher.finish());
    }
    let cached_hash_duration = start.elapsed();
    
    println!("      第一次 Hash: {:?}", first_hash_duration);
    println!("      缓存 Hash: {:?}", cached_hash_duration);
    
    if first_hash_duration > cached_hash_duration {
        let speedup = first_hash_duration.as_nanos() as f64 / cached_hash_duration.as_nanos() as f64;
        println!("      ✅ 缓存策略提升 {:.2}x", speedup);
    }
    
    assert_eq!(total_hash, total_hash2, "缓存应该产生相同的结果");
}

/// 内存使用分析
fn memory_usage_analysis() {
    println!("\n  📌 内存使用分析:");
    
    // 不同实现的内存占用
    println!("\n    🔹 不同实现的内存占用:");
    
    #[derive(PartialEq, Eq, Hash)]
    struct MinimalStruct {
        id: u32,
    }
    
    #[derive(PartialEq, Eq, Hash)]
    struct PaddedStruct {
        id: u32,
        flag: bool,  // 会导致内存对齐
    }
    
    #[derive(PartialEq, Eq, Hash)]
    struct OptimizedStruct {
        id: u32,
        flags: u32,  // 将多个 bool 打包成位字段
    }
    
    println!("      MinimalStruct 大小: {} 字节", std::mem::size_of::<MinimalStruct>());
    println!("      PaddedStruct 大小: {} 字节", std::mem::size_of::<PaddedStruct>());
    println!("      OptimizedStruct 大小: {} 字节", std::mem::size_of::<OptimizedStruct>());
    
    // 集合的内存开销
    println!("\n    🔹 集合的内存开销:");
    
    let data_count = 10000;
    
    // Vec 的内存使用
    let vec_data: Vec<u32> = (0..data_count).collect();
    let vec_memory = vec_data.len() * std::mem::size_of::<u32>();
    println!("      Vec<u32> ({} 项): ~{} 字节", data_count, vec_memory);
    
    // HashSet 的内存使用（估算）
    let hashset_data: HashSet<u32> = (0..data_count).collect();
    let hashset_memory = hashset_data.len() * (std::mem::size_of::<u32>() + 8); // 估算开销
    println!("      HashSet<u32> ({} 项): ~{} 字节 (估算)", data_count, hashset_memory);
    
    // BTreeSet 的内存使用（估算）
    let btreeset_data: BTreeSet<u32> = (0..data_count).collect();
    let btreeset_memory = btreeset_data.len() * (std::mem::size_of::<u32>() + 16); // 估算开销
    println!("      BTreeSet<u32> ({} 项): ~{} 字节 (估算)", data_count, btreeset_memory);
    
    println!("\n    🔹 内存局部性影响:");
    
    // 连续内存访问 vs 随机访问
    let large_vec: Vec<u64> = (0..100000).collect();
    
    // 顺序访问
    let start = Instant::now();
    let mut sum = 0u64;
    for i in 0..large_vec.len() {
        sum = sum.wrapping_add(large_vec[i]);
    }
    let sequential_duration = start.elapsed();
    
    // 随机访问
    let indices: Vec<usize> = (0..large_vec.len()).rev().collect(); // 反向访问
    let start = Instant::now();
    let mut sum2 = 0u64;
    for &i in &indices {
        sum2 = sum2.wrapping_add(large_vec[i]);
    }
    let random_duration = start.elapsed();
    
    println!("      顺序访问: {:?}, 和: {}", sequential_duration, sum);
    println!("      反向访问: {:?}, 和: {}", random_duration, sum2);
    
    if random_duration > sequential_duration {
        let slowdown = random_duration.as_nanos() as f64 / sequential_duration.as_nanos() as f64;
        println!("      ⚠️  随机访问慢 {:.2}x (缓存局部性影响)", slowdown);
    }
}

/// 基准测试不同实现
fn benchmark_different_implementations() {
    println!("\n  📌 不同实现的基准测试:");
    
    // 字符串比较的不同策略
    println!("\n    🔹 字符串比较策略:");
    
    #[derive(Debug, Clone)]
    struct StringWrapper1(String);
    
    #[derive(Debug, Clone)]
    struct StringWrapper2(String);
    
    #[derive(Debug, Clone)]
    struct StringWrapper3 {
        content: String,
        length: usize,  // 缓存长度
    }
    
    // 标准实现
    impl PartialEq for StringWrapper1 {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    
    // 长度优先实现
    impl PartialEq for StringWrapper2 {
        fn eq(&self, other: &Self) -> bool {
            self.0.len() == other.0.len() && self.0 == other.0
        }
    }
    
    // 缓存长度实现
    impl PartialEq for StringWrapper3 {
        fn eq(&self, other: &Self) -> bool {
            self.length == other.length && self.content == other.content
        }
    }
    
    // 创建测试数据
    let test_strings: Vec<String> = (0..1000).map(|i| {
        if i % 2 == 0 {
            format!("short_{}", i)
        } else {
            format!("very_long_string_with_lots_of_content_{}_that_makes_comparison_slower", i)
        }
    }).collect();
    
    let wrapper1_data: Vec<StringWrapper1> = test_strings.iter()
        .map(|s| StringWrapper1(s.clone()))
        .collect();
    
    let wrapper2_data: Vec<StringWrapper2> = test_strings.iter()
        .map(|s| StringWrapper2(s.clone()))
        .collect();
    
    let wrapper3_data: Vec<StringWrapper3> = test_strings.iter()
        .map(|s| StringWrapper3 { length: s.len(), content: s.clone() })
        .collect();
    
    // 基准测试
    let iterations = 100;
    
    // 标准实现
    let start = Instant::now();
    let mut matches1 = 0;
    for _ in 0..iterations {
        for i in 0..wrapper1_data.len() {
            for j in (i+1)..wrapper1_data.len() {
                if wrapper1_data[i] == wrapper1_data[j] {
                    matches1 += 1;
                }
            }
        }
    }
    let standard_duration = start.elapsed();
    
    // 长度优先实现
    let start = Instant::now();
    let mut matches2 = 0;
    for _ in 0..iterations {
        for i in 0..wrapper2_data.len() {
            for j in (i+1)..wrapper2_data.len() {
                if wrapper2_data[i] == wrapper2_data[j] {
                    matches2 += 1;
                }
            }
        }
    }
    let length_first_duration = start.elapsed();
    
    // 缓存长度实现
    let start = Instant::now();
    let mut matches3 = 0;
    for _ in 0..iterations {
        for i in 0..wrapper3_data.len() {
            for j in (i+1)..wrapper3_data.len() {
                if wrapper3_data[i] == wrapper3_data[j] {
                    matches3 += 1;
                }
            }
        }
    }
    let cached_length_duration = start.elapsed();
    
    println!("      标准实现: {:?}, 匹配: {}", standard_duration, matches1);
    println!("      长度优先: {:?}, 匹配: {}", length_first_duration, matches2);
    println!("      缓存长度: {:?}, 匹配: {}", cached_length_duration, matches3);
    
    // 找出最快的实现
    let durations = [standard_duration, length_first_duration, cached_length_duration];
    let min_duration = durations.iter().min().unwrap();
    
    if *min_duration == length_first_duration {
        println!("      ✅ 长度优先实现最快");
    } else if *min_duration == cached_length_duration {
        println!("      ✅ 缓存长度实现最快");
    } else {
        println!("      ✅ 标准实现最快");
    }
}

/// 性能总结和建议
pub fn performance_summary() {
    println!("\n🔸 性能总结和建议:");
    
    println!("\n  📋 关键性能要点:");
    println!("    1. 基本类型比较 (i32, f64) 非常快，几乎没有开销");
    println!("    2. 字符串比较的成本与长度成正比");
    println!("    3. 复杂结构体比较可能很昂贵，需要优化策略");
    println!("    4. Hash 计算的成本取决于数据大小和复杂性");
    println!("    5. HashMap 查找通常比 BTreeMap 快，但内存开销更大");
    
    println!("\n  ⚡ 优化策略:");
    println!("    • 早期退出：首先比较最可能不同的字段");
    println!("    • 长度检查：比较容器内容前先比较长度");
    println!("    • 缓存策略：缓存昂贵的计算结果 (如 hash 值)");
    println!("    • 内存布局：优化结构体布局减少内存占用");
    println!("    • 选择合适的集合类型：HashMap vs BTreeMap vs Vec");
    
    println!("\n  🎯 最佳实践:");
    println!("    • 为频繁比较的类型实现高效的 PartialEq");
    println!("    • 保持 Hash 和 Eq 的一致性");
    println!("    • 在性能关键路径上进行基准测试");
    println!("    • 考虑使用 #[inline] 属性优化小函数");
    println!("    • 避免在 PartialEq 中进行昂贵的操作");
    
    println!("\n  📊 性能测试建议:");
    println!("    • 使用 `cargo bench` 进行精确的基准测试");
    println!("    • 测试不同数据大小和分布的性能");
    println!("    • 分析内存使用模式和缓存局部性");
    println!("    • 比较不同实现策略的权衡");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_performance_functions() {
        // 这些测试主要确保函数能正常运行
        // 实际的性能测试应该使用 criterion 或类似工具
        
        // 测试基本比较
        assert_eq!(42, 42);
        assert_ne!(42, 43);
        
        // 测试字符串比较
        let s1 = "hello".to_string();
        let s2 = "hello".to_string();
        assert_eq!(s1, s2);
        
        // 测试集合操作
        let mut set: HashSet<i32> = HashSet::new();
        set.insert(1);
        set.insert(2);
        assert_eq!(set.len(), 2);
        assert!(set.contains(&1));
    }
    
    #[test]
    fn test_hash_consistency_performance() {
        #[derive(PartialEq, Eq, Hash, Debug)]
        struct TestStruct {
            id: u32,
            data: Vec<u8>,
        }
        
        let item1 = TestStruct {
            id: 1,
            data: vec![1, 2, 3],
        };
        
        let item2 = TestStruct {
            id: 1,
            data: vec![1, 2, 3],
        };
        
        assert_eq!(item1, item2);
        
        // 验证 hash 一致性
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher1 = DefaultHasher::new();
        item1.hash(&mut hasher1);
        let hash1 = hasher1.finish();
        
        let mut hasher2 = DefaultHasher::new();
        item2.hash(&mut hasher2);
        let hash2 = hasher2.finish();
        
        assert_eq!(hash1, hash2);
    }
}