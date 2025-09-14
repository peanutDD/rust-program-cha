//! Rust HashMap 深度分析与实战应用
//!
//! 本文档基于 https://course.rs/basic/collections/hashmap.html 内容
//! 提供 HashMap 的全面分析，包括原理、操作、性能优化和实际应用案例

use std::collections::HashMap;
use std::collections::hash_map::{DefaultHasher, Entry};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::time::Instant;

fn main() {
    println!("=== Rust HashMap 深度分析与实战应用 ===");
    println!();

    // 1. HashMap 基础概念和原理
    hashmap_basics_and_principles();

    // 2. HashMap 创建方法全解析
    hashmap_creation_methods();

    // 3. HashMap 基本操作深入分析
    hashmap_basic_operations();

    // 4. HashMap 迭代方法和性能对比
    hashmap_iteration_methods();

    // 5. HashMap 所有权和借用机制
    hashmap_ownership_and_borrowing();

    // 6. HashMap 性能分析和优化
    hashmap_performance_analysis();

    // 7. 自定义键类型的实现
    hashmap_custom_key_types();

    // 8. HashMap 高级特性
    hashmap_advanced_features();

    // 9. HashMap 实际应用案例
    hashmap_real_world_applications();

    println!("\n=== HashMap 深度分析完成 ===");
}

/// 1. HashMap 基础概念和原理
fn hashmap_basics_and_principles() {
    println!("\n1. === HashMap 基础概念和原理 ===");

    // 1.1 哈希表基本原理
    println!("\n1.1 哈希表基本原理:");
    println!("- HashMap 是基于哈希表实现的键值对集合");
    println!("- 使用哈希函数将键映射到数组索引");
    println!("- 平均时间复杂度: O(1) 插入、查找、删除");
    println!("- 最坏时间复杂度: O(n) (哈希冲突严重时)");

    // 1.2 内存布局分析
    println!("\n1.2 HashMap 内存布局:");
    let mut map: HashMap<String, i32> = HashMap::new();
    println!("空 HashMap 容量: {}", map.capacity());

    map.insert("key1".to_string(), 100);
    println!("插入一个元素后容量: {}", map.capacity());
    println!("当前元素数量: {}", map.len());

    // 1.3 所有权机制演示
    println!("\n1.3 HashMap 所有权机制:");
    let key = String::from("owned_key");
    let value = String::from("owned_value");

    let mut ownership_map = HashMap::new();
    ownership_map.insert(key, value); // key 和 value 的所有权转移到 HashMap

    // println!("{}", key); // 编译错误：key 已被移动
    println!("键值对已插入，所有权已转移到 HashMap");

    // 使用引用避免所有权转移
    let ref_key = "reference_key";
    let ref_value = "reference_value";
    let mut ref_map = HashMap::new();
    ref_map.insert(ref_key, ref_value); // &str 实现了 Copy，不会转移所有权

    println!("使用 &str 类型: key = {}, value = {}", ref_key, ref_value);

    // 1.4 哈希函数演示
    println!("\n1.4 哈希函数工作原理:");
    demonstrate_hash_function();
}

/// 演示哈希函数的工作原理
fn demonstrate_hash_function() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let keys = ["apple", "banana", "cherry", "date"];

    for key in &keys {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash_value = hasher.finish();
        println!("键 '{}' 的哈希值: {}", key, hash_value);
    }

    // 演示相同键的哈希值一致性
    let key = "consistent";
    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();

    key.hash(&mut hasher1);
    key.hash(&mut hasher2);

    println!(
        "相同键 '{}' 的两次哈希值: {} vs {}",
        key,
        hasher1.finish(),
        hasher2.finish()
    );
}

/// 2. HashMap 创建方法全解析
fn hashmap_creation_methods() {
    println!("\n2. === HashMap 创建方法全解析 ===");

    // 2.1 使用 new() 创建空 HashMap
    println!("\n2.1 使用 new() 创建:");
    let mut map1: HashMap<String, i32> = HashMap::new();
    println!("空 HashMap 创建成功，容量: {}", map1.capacity());

    // 2.2 使用 with_capacity() 预分配容量
    println!("\n2.2 使用 with_capacity() 预分配容量:");
    let mut map2: HashMap<String, i32> = HashMap::with_capacity(10);
    println!("预分配容量 HashMap，容量: {}", map2.capacity());

    // 2.3 使用 from() 从数组创建
    println!("\n2.3 使用 from() 从数组创建:");
    let map3 = HashMap::from([("apple", 5), ("banana", 3), ("cherry", 8)]);
    println!("从数组创建: {:?}", map3);

    // 2.4 使用 collect() 从迭代器创建
    println!("\n2.4 使用 collect() 从迭代器创建:");
    let teams = vec![
        ("Blue".to_string(), 10),
        ("Red".to_string(), 50),
        ("Green".to_string(), 30),
    ];

    let map4: HashMap<String, i32> = teams.into_iter().collect();
    println!("从 Vec 创建: {:?}", map4);

    // 2.5 使用 zip() 组合两个向量
    println!("\n2.5 使用 zip() 组合两个向量:");
    let keys = vec!["first", "second", "third"];
    let values = vec![1, 2, 3];

    let map5: HashMap<&str, i32> = keys.into_iter().zip(values.into_iter()).collect();
    println!("zip 组合创建: {:?}", map5);

    // 2.6 性能对比：不同创建方法的效率
    println!("\n2.6 创建方法性能对比:");
    performance_comparison_creation();
}

/// 创建方法性能对比
fn performance_comparison_creation() {
    const SIZE: usize = 10000;

    // 测试 new() + insert
    let start = Instant::now();
    let mut map1 = HashMap::new();
    for i in 0..SIZE {
        map1.insert(i, i * 2);
    }
    let duration1 = start.elapsed();

    // 测试 with_capacity() + insert
    let start = Instant::now();
    let mut map2 = HashMap::with_capacity(SIZE);
    for i in 0..SIZE {
        map2.insert(i, i * 2);
    }
    let duration2 = start.elapsed();

    // 测试 collect()
    let start = Instant::now();
    let map3: HashMap<usize, usize> = (0..SIZE).map(|i| (i, i * 2)).collect();
    let duration3 = start.elapsed();

    println!("new() + insert: {:?}", duration1);
    println!("with_capacity() + insert: {:?}", duration2);
    println!("collect(): {:?}", duration3);
    println!(
        "预分配容量提升: {:.2}x",
        duration1.as_nanos() as f64 / duration2.as_nanos() as f64
    );
}

/// 3. HashMap 基本操作深入分析
fn hashmap_basic_operations() {
    println!("\n3. === HashMap 基本操作深入分析 ===");

    let mut scores = HashMap::new();

    // 3.1 插入操作 (insert)
    println!("\n3.1 插入操作分析:");

    // insert 返回 Option<V>，如果键已存在则返回旧值
    let old_value = scores.insert("Alice", 95);
    println!(
        "插入 Alice: {:?} (返回旧值: {:?})",
        scores.get("Alice"),
        old_value
    );

    let old_value = scores.insert("Alice", 98); // 更新已存在的键
    println!(
        "更新 Alice: {:?} (返回旧值: {:?})",
        scores.get("Alice"),
        old_value
    );

    scores.insert("Bob", 87);
    scores.insert("Charlie", 92);

    // 3.2 查询操作 (get)
    println!("\n3.2 查询操作分析:");

    // get 返回 Option<&V>
    match scores.get("Alice") {
        Some(score) => println!("Alice 的分数: {}", score),
        None => println!("未找到 Alice"),
    }

    // 使用 get 的不同方式
    println!("Bob 的分数: {:?}", scores.get("Bob"));
    println!("David 的分数: {:?}", scores.get("David")); // None

    // get_mut 返回可变引用
    if let Some(score) = scores.get_mut("Bob") {
        *score += 5; // 修改值
        println!("Bob 分数加 5 后: {}", score);
    }

    // 3.3 删除操作 (remove)
    println!("\n3.3 删除操作分析:");

    let removed = scores.remove("Charlie");
    println!("删除 Charlie: {:?}", removed);
    println!("删除后的 HashMap: {:?}", scores);

    // 3.4 检查键是否存在 (contains_key)
    println!("\n3.4 键存在性检查:");
    println!("Alice 存在: {}", scores.contains_key("Alice"));
    println!("Charlie 存在: {}", scores.contains_key("Charlie"));

    // 3.5 获取长度和检查是否为空
    println!("\n3.5 长度和空检查:");
    println!("HashMap 长度: {}", scores.len());
    println!("HashMap 是否为空: {}", scores.is_empty());

    // 3.6 清空操作
    println!("\n3.6 清空操作:");
    let original_capacity = scores.capacity();
    scores.clear();
    println!(
        "清空后长度: {}, 容量: {} (原容量: {})",
        scores.len(),
        scores.capacity(),
        original_capacity
    );
}

/// 4. HashMap 迭代方法和性能对比
fn hashmap_iteration_methods() {
    println!("\n4. === HashMap 迭代方法和性能对比 ===");

    let mut map = HashMap::from([("apple", 5), ("banana", 3), ("cherry", 8), ("date", 2)]);

    // 4.1 遍历键 (keys)
    println!("\n4.1 遍历键:");
    print!("所有键: ");
    for key in map.keys() {
        print!("{} ", key);
    }
    println!();

    // 4.2 遍历值 (values)
    println!("\n4.2 遍历值:");
    print!("所有值: ");
    for value in map.values() {
        print!("{} ", value);
    }
    println!();

    // 4.3 遍历键值对 (iter)
    println!("\n4.3 遍历键值对 (不可变引用):");
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // 4.4 可变遍历 (iter_mut)
    println!("\n4.4 可变遍历 (修改值):");
    for (key, value) in &mut map {
        *value *= 2; // 将所有值翻倍
        println!("{}: {} (已翻倍)", key, value);
    }

    // 4.5 消费性遍历 (into_iter)
    println!("\n4.5 消费性遍历 (获取所有权):");
    let map_clone = map.clone();
    for (key, value) in map_clone {
        println!("拥有: {} -> {}", key, value);
    }
    // map_clone 在此处已被消费，不能再使用

    // 4.6 迭代器链式操作
    println!("\n4.6 迭代器链式操作:");

    // 过滤和收集
    let high_values: HashMap<&str, i32> = map
        .iter()
        .filter(|(_, value)| *value > &10)
        .map(|(key, value)| (*key, *value))
        .collect();
    println!("值大于 10 的项: {:?}", high_values);

    // 计算总和
    let total: i32 = map.values().sum();
    println!("所有值的总和: {}", total);

    // 查找最大值
    if let Some((key, value)) = map.iter().max_by_key(|(_, v)| *v) {
        println!("最大值: {} -> {}", key, value);
    }

    // 4.7 迭代性能对比
    println!("\n4.7 迭代性能对比:");
    iteration_performance_comparison();
}

/// 迭代性能对比
fn iteration_performance_comparison() {
    const SIZE: usize = 100000;
    let map: HashMap<usize, usize> = (0..SIZE).map(|i| (i, i * 2)).collect();

    // 测试 for 循环遍历
    let start = Instant::now();
    let mut sum1 = 0;
    for (_, value) in &map {
        sum1 += value;
    }
    let duration1 = start.elapsed();

    // 测试 values().sum()
    let start = Instant::now();
    let sum2: usize = map.values().sum();
    let duration2 = start.elapsed();

    // 测试 iter().map().sum()
    let start = Instant::now();
    let sum3: usize = map.iter().map(|(_, v)| v).sum();
    let duration3 = start.elapsed();

    println!("for 循环: {} (耗时: {:?})", sum1, duration1);
    println!("values().sum(): {} (耗时: {:?})", sum2, duration2);
    println!("iter().map().sum(): {} (耗时: {:?})", sum3, duration3);
}

/// 5. HashMap 所有权和借用机制
fn hashmap_ownership_and_borrowing() {
    println!("\n5. === HashMap 所有权和借用机制 ===");

    // 5.1 所有权转移示例
    println!("\n5.1 所有权转移:");

    let key1 = String::from("owned_key");
    let value1 = String::from("owned_value");

    let mut map = HashMap::new();
    map.insert(key1, value1); // 所有权转移

    // println!("{}", key1); // 编译错误：key1 已被移动
    println!("String 类型的键值对已插入，所有权已转移");

    // 5.2 使用引用避免所有权转移
    println!("\n5.2 使用引用:");

    let key2 = "reference_key";
    let value2 = "reference_value";

    let mut ref_map = HashMap::new();
    ref_map.insert(key2, value2); // &str 实现了 Copy

    println!("插入后仍可使用: key = {}, value = {}", key2, value2);

    // 5.3 借用检查器示例
    println!("\n5.3 借用检查器:");

    let mut scores = HashMap::from([("Alice", 95), ("Bob", 87)]);

    // 不可变借用
    let alice_score = scores.get("Alice");
    let bob_score = scores.get("Bob");

    println!("Alice: {:?}, Bob: {:?}", alice_score, bob_score);

    // 可变借用
    if let Some(score) = scores.get_mut("Alice") {
        *score += 5;
    }

    println!("Alice 更新后: {:?}", scores.get("Alice"));

    // 5.4 Entry API 避免重复查找
    println!("\n5.4 Entry API 优化:");
    entry_api_examples(&mut scores);
}

/// Entry API 示例
fn entry_api_examples(scores: &mut HashMap<&str, i32>) {
    // or_insert: 键不存在时插入
    let charlie_score = scores.entry("Charlie").or_insert(0);
    *charlie_score += 10;
    println!("Charlie 初始化并加 10: {}", charlie_score);

    // or_insert_with: 使用闭包计算默认值
    scores.entry("David").or_insert_with(|| {
        println!("计算 David 的默认分数");
        85
    });

    // and_modify: 修改已存在的值
    scores
        .entry("Alice")
        .and_modify(|score| *score += 2)
        .or_insert(90);

    println!("Entry API 操作后: {:?}", scores);

    // Entry 模式匹配
    match scores.entry("Eve") {
        Entry::Occupied(mut entry) => {
            println!("Eve 已存在，当前分数: {}", entry.get());
            *entry.get_mut() += 1;
        }
        Entry::Vacant(entry) => {
            println!("Eve 不存在，插入默认分数");
            entry.insert(88);
        }
    }
}

/// 6. HashMap 性能分析和优化
fn hashmap_performance_analysis() {
    println!("\n6. === HashMap 性能分析和优化 ===");

    // 6.1 负载因子和容量管理
    println!("\n6.1 负载因子和容量管理:");

    let mut map = HashMap::new();
    println!("初始容量: {}", map.capacity());

    // 观察容量变化
    for i in 0..20 {
        map.insert(i, i * 2);
        if i % 5 == 4 {
            println!(
                "插入 {} 个元素后 - 长度: {}, 容量: {}, 负载因子: {:.2}",
                i + 1,
                map.len(),
                map.capacity(),
                map.len() as f64 / map.capacity() as f64
            );
        }
    }

    // 6.2 容量预分配的重要性
    println!("\n6.2 容量预分配性能测试:");
    capacity_preallocation_test();

    // 6.3 哈希冲突分析
    println!("\n6.3 哈希冲突分析:");
    hash_collision_analysis();

    // 6.4 内存使用优化
    println!("\n6.4 内存使用优化:");
    memory_optimization_strategies();
}

/// 容量预分配性能测试
fn capacity_preallocation_test() {
    const SIZE: usize = 50000;

    // 不预分配容量
    let start = Instant::now();
    let mut map1 = HashMap::new();
    for i in 0..SIZE {
        map1.insert(i, i.to_string());
    }
    let duration1 = start.elapsed();

    // 预分配容量
    let start = Instant::now();
    let mut map2 = HashMap::with_capacity(SIZE);
    for i in 0..SIZE {
        map2.insert(i, i.to_string());
    }
    let duration2 = start.elapsed();

    println!("不预分配: {:?}", duration1);
    println!("预分配: {:?}", duration2);
    println!(
        "性能提升: {:.2}x",
        duration1.as_nanos() as f64 / duration2.as_nanos() as f64
    );
}

/// 哈希冲突分析
fn hash_collision_analysis() {
    // 创建可能产生冲突的键
    let keys = vec!["abc", "bca", "cab", "acb", "bac", "cba"];

    println!("分析字符串键的哈希分布:");
    for key in &keys {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash_value = hasher.finish();
        println!(
            "'{}' -> 哈希值: {} (模 16: {})",
            key,
            hash_value,
            hash_value % 16
        );
    }

    // 测试数字键的分布
    println!("\n数字键的哈希分布:");
    for i in 0..10 {
        let mut hasher = DefaultHasher::new();
        i.hash(&mut hasher);
        let hash_value = hasher.finish();
        println!(
            "{} -> 哈希值: {} (模 16: {})",
            i,
            hash_value,
            hash_value % 16
        );
    }
}

/// 内存使用优化策略
fn memory_optimization_strategies() {
    println!("内存优化策略演示:");

    // 1. 使用 shrink_to_fit 释放多余容量
    let mut map = HashMap::with_capacity(1000);
    for i in 0..100 {
        map.insert(i, i.to_string());
    }

    println!("插入 100 个元素:");
    println!("  长度: {}, 容量: {}", map.len(), map.capacity());

    map.shrink_to_fit();
    println!("shrink_to_fit 后:");
    println!("  长度: {}, 容量: {}", map.len(), map.capacity());

    // 2. 选择合适的键类型
    println!("\n键类型选择建议:");
    println!("- 使用 &str 而不是 String (如果可能)");
    println!("- 使用数字类型作为键 (性能更好)");
    println!("- 避免复杂的自定义类型作为键");
}

/// 7. 自定义键类型的实现
fn hashmap_custom_key_types() {
    println!("\n7. === 自定义键类型的实现 ===");

    // 7.1 自定义结构体作为键
    println!("\n7.1 自定义结构体作为键:");

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Person {
        name: String,
        age: u32,
    }

    let mut people_scores = HashMap::new();

    let alice = Person {
        name: "Alice".to_string(),
        age: 25,
    };

    let bob = Person {
        name: "Bob".to_string(),
        age: 30,
    };

    people_scores.insert(alice.clone(), 95);
    people_scores.insert(bob.clone(), 87);

    println!("自定义键 HashMap: {:?}", people_scores);

    // 7.2 手动实现 Hash trait
    println!("\n7.2 手动实现 Hash trait:");

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct CustomKey {
        id: u32,
        category: String,
    }

    impl Hash for CustomKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // 只基于 id 计算哈希，忽略 category
            self.id.hash(state);
        }
    }

    let mut custom_map = HashMap::new();

    let key1 = CustomKey {
        id: 1,
        category: "A".to_string(),
    };

    let key2 = CustomKey {
        id: 1,
        category: "B".to_string(), // 不同的 category，但相同的 id
    };

    custom_map.insert(key1.clone(), "value1");
    println!("插入 key1: {:?}", custom_map);

    // key2 会覆盖 key1，因为它们的哈希值相同且 PartialEq 返回 true
    custom_map.insert(key2.clone(), "value2");
    println!("插入 key2 (相同 id): {:?}", custom_map);

    // 7.3 Hash trait 实现要求
    println!("\n7.3 Hash trait 实现要求:");
    println!("- 如果 k1 == k2，则 hash(k1) == hash(k2)");
    println!("- Hash 值应该尽可能均匀分布");
    println!("- Hash 计算应该快速且确定性");

    // 7.4 常见陷阱和最佳实践
    println!("\n7.4 自定义键类型最佳实践:");
    custom_key_best_practices();
}

/// 自定义键类型最佳实践
fn custom_key_best_practices() {
    println!("最佳实践:");
    println!("1. 确保 Hash 和 Eq 的一致性");
    println!("2. 避免在 Hash 实现中使用浮点数");
    println!("3. 考虑使用 derive 宏自动实现");
    println!("4. 测试哈希分布的均匀性");

    // 演示错误的实现
    println!("\n❌ 错误示例 - Hash 和 Eq 不一致:");

    #[derive(Debug, Clone)]
    struct BadKey {
        value: f64,
    }

    impl PartialEq for BadKey {
        fn eq(&self, other: &Self) -> bool {
            (self.value - other.value).abs() < 0.001 // 浮点数近似相等
        }
    }

    impl Eq for BadKey {}

    impl Hash for BadKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // 错误：直接哈希浮点数，可能导致相等的键有不同哈希值
            self.value.to_bits().hash(state);
        }
    }

    println!("BadKey 可能导致相等的键有不同的哈希值");

    // 正确的实现
    println!("\n✅ 正确示例 - 一致的 Hash 和 Eq:");

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct GoodKey {
        id: u32,
        name: String,
    }

    println!("GoodKey 使用 derive 宏确保一致性");
}

/// 8. HashMap 高级特性
fn hashmap_advanced_features() {
    println!("\n8. === HashMap 高级特性 ===");

    // 8.1 Entry API 深入使用
    println!("\n8.1 Entry API 深入使用:");

    let mut word_count = HashMap::new();
    let text = "hello world hello rust world";

    // 统计单词频率
    for word in text.split_whitespace() {
        *word_count.entry(word).or_insert(0) += 1;
    }

    println!("单词频率统计: {:?}", word_count);

    // 8.2 复杂的 Entry 操作
    println!("\n8.2 复杂的 Entry 操作:");

    let mut scores = HashMap::new();

    // 链式操作
    scores
        .entry("Alice")
        .and_modify(|score| *score += 10)
        .or_insert_with(|| {
            println!("为 Alice 创建新分数");
            85
        });

    scores
        .entry("Bob")
        .and_modify(|score| *score += 5)
        .or_insert(90);

    println!("Entry 链式操作结果: {:?}", scores);

    // 8.3 retain 方法
    println!("\n8.3 retain 方法 - 条件性保留元素:");

    let mut numbers = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
    ]);

    println!("原始 HashMap: {:?}", numbers);

    // 只保留偶数值
    numbers.retain(|_, &mut value| value % 2 == 0);
    println!("保留偶数后: {:?}", numbers);

    // 8.4 drain 方法
    println!("\n8.4 drain 方法 - 移除并迭代:");

    let mut items = HashMap::from([("apple", 5), ("banana", 3), ("cherry", 8)]);

    println!("原始 items: {:?}", items);

    // 移除所有元素并收集
    let drained: Vec<_> = items.drain().collect();
    println!("drain 后的元素: {:?}", drained);
    println!("drain 后的 HashMap: {:?}", items);

    // 8.5 try_insert 方法 (如果可用)
    println!("\n8.5 高级插入操作:");

    let mut map = HashMap::new();
    map.insert("key1", "value1");

    // 使用 entry 实现 try_insert 语义
    match map.entry("key1") {
        Entry::Vacant(entry) => {
            entry.insert("new_value");
            println!("插入成功");
        }
        Entry::Occupied(_) => {
            println!("键已存在，插入失败");
        }
    }

    // 8.6 自定义哈希器
    println!("\n8.6 自定义哈希器 (概念演示):");
    demonstrate_custom_hasher();
}

/// 演示自定义哈希器概念
fn demonstrate_custom_hasher() {
    use std::collections::HashMap;
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};

    // 使用默认哈希器
    let map1: HashMap<String, i32> = HashMap::new();
    println!("默认哈希器 HashMap 创建成功");

    // 演示哈希器的作用
    let hasher_builder = RandomState::new();
    let mut hasher1 = hasher_builder.build_hasher();
    let mut hasher2 = hasher_builder.build_hasher();

    "test_key".hash(&mut hasher1);
    "test_key".hash(&mut hasher2);

    println!(
        "相同键的哈希值: {} vs {}",
        hasher1.finish(),
        hasher2.finish()
    );
}

/// 9. HashMap 实际应用案例
fn hashmap_real_world_applications() {
    println!("\n9. === HashMap 实际应用案例 ===");

    // 9.1 缓存系统实现
    println!("\n9.1 缓存系统实现:");
    cache_system_example();

    // 9.2 计数器和统计
    println!("\n9.2 计数器和统计应用:");
    counter_and_statistics_example();

    // 9.3 索引构建
    println!("\n9.3 索引构建应用:");
    index_building_example();

    // 9.4 配置管理
    println!("\n9.4 配置管理应用:");
    configuration_management_example();

    // 9.5 图算法中的应用
    println!("\n9.5 图算法应用:");
    graph_algorithm_example();
}

/// 缓存系统示例
fn cache_system_example() {
    struct SimpleCache<K, V> {
        data: HashMap<K, V>,
        max_size: usize,
    }

    impl<K: Clone + Eq + Hash, V: Clone> SimpleCache<K, V> {
        fn new(max_size: usize) -> Self {
            Self {
                data: HashMap::with_capacity(max_size),
                max_size,
            }
        }

        fn get(&self, key: &K) -> Option<&V> {
            self.data.get(key)
        }

        fn put(&mut self, key: K, value: V) {
            if self.data.len() >= self.max_size {
                // 简单的清理策略：清空一半
                let keys_to_remove: Vec<_> =
                    self.data.keys().take(self.max_size / 2).cloned().collect();

                for k in keys_to_remove {
                    self.data.remove(&k);
                }
            }

            self.data.insert(key, value);
        }

        fn size(&self) -> usize {
            self.data.len()
        }
    }

    let mut cache = SimpleCache::new(3);

    cache.put("key1", "value1");
    cache.put("key2", "value2");
    cache.put("key3", "value3");

    println!("缓存大小: {}", cache.size());
    println!("获取 key1: {:?}", cache.get(&"key1"));

    // 触发缓存清理
    cache.put("key4", "value4");
    println!("添加 key4 后缓存大小: {}", cache.size());
}

/// 计数器和统计示例
fn counter_and_statistics_example() {
    // 字符频率统计
    let text = "hello world, this is a test message for character frequency analysis";
    let mut char_count = HashMap::new();

    for ch in text.chars() {
        if ch.is_alphabetic() {
            *char_count
                .entry(ch.to_lowercase().next().unwrap())
                .or_insert(0) += 1;
        }
    }

    println!("字符频率统计 (前 10 个):");
    let mut sorted_chars: Vec<_> = char_count.iter().collect();
    sorted_chars.sort_by(|a, b| b.1.cmp(a.1));

    for (ch, count) in sorted_chars.iter().take(10) {
        println!("  '{}': {} 次", ch, count);
    }

    // 网站访问统计
    let visits = vec![
        "/home",
        "/about",
        "/home",
        "/contact",
        "/home",
        "/about",
        "/products",
    ];

    let mut page_views = HashMap::new();
    for page in visits {
        *page_views.entry(page).or_insert(0) += 1;
    }

    println!("\n页面访问统计:");
    for (page, views) in &page_views {
        println!("  {}: {} 次访问", page, views);
    }
}

/// 索引构建示例
fn index_building_example() {
    #[derive(Debug, Clone)]
    struct Document {
        id: usize,
        title: String,
        content: String,
    }

    let documents = vec![
        Document {
            id: 1,
            title: "Rust Programming".to_string(),
            content: "Rust is a systems programming language".to_string(),
        },
        Document {
            id: 2,
            title: "HashMap Guide".to_string(),
            content: "HashMap is a key-value data structure".to_string(),
        },
        Document {
            id: 3,
            title: "Data Structures".to_string(),
            content: "Various data structures in programming".to_string(),
        },
    ];

    // 构建倒排索引
    let mut inverted_index: HashMap<String, Vec<usize>> = HashMap::new();

    for doc in &documents {
        let combined_text = format!("{} {}", doc.title, doc.content);
        let lowercase_text = combined_text.to_lowercase();
        let words = lowercase_text
            .split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphabetic()))
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        for word in words {
            inverted_index
                .entry(word.to_string())
                .or_insert_with(Vec::new)
                .push(doc.id);
        }
    }

    println!("倒排索引构建完成:");
    for (word, doc_ids) in &inverted_index {
        println!("  '{}': 出现在文档 {:?}", word, doc_ids);
    }

    // 搜索功能
    let search_term = "rust";
    if let Some(doc_ids) = inverted_index.get(search_term) {
        println!("\n搜索 '{}' 的结果:", search_term);
        for &doc_id in doc_ids {
            if let Some(doc) = documents.iter().find(|d| d.id == doc_id) {
                println!("  文档 {}: {}", doc.id, doc.title);
            }
        }
    }
}

/// 配置管理示例
fn configuration_management_example() {
    #[derive(Debug, Clone)]
    enum ConfigValue {
        String(String),
        Integer(i64),
        Boolean(bool),
        Float(f64),
    }

    impl fmt::Display for ConfigValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ConfigValue::String(s) => write!(f, "{}", s),
                ConfigValue::Integer(i) => write!(f, "{}", i),
                ConfigValue::Boolean(b) => write!(f, "{}", b),
                ConfigValue::Float(fl) => write!(f, "{}", fl),
            }
        }
    }

    struct Configuration {
        settings: HashMap<String, ConfigValue>,
    }

    impl Configuration {
        fn new() -> Self {
            Self {
                settings: HashMap::new(),
            }
        }

        fn set(&mut self, key: &str, value: ConfigValue) {
            self.settings.insert(key.to_string(), value);
        }

        fn get(&self, key: &str) -> Option<&ConfigValue> {
            self.settings.get(key)
        }

        fn get_string(&self, key: &str) -> Option<&String> {
            match self.get(key) {
                Some(ConfigValue::String(s)) => Some(s),
                _ => None,
            }
        }

        fn get_integer(&self, key: &str) -> Option<i64> {
            match self.get(key) {
                Some(ConfigValue::Integer(i)) => Some(*i),
                _ => None,
            }
        }
    }

    let mut config = Configuration::new();

    config.set(
        "database_url",
        ConfigValue::String("localhost:5432".to_string()),
    );
    config.set("max_connections", ConfigValue::Integer(100));
    config.set("debug_mode", ConfigValue::Boolean(true));
    config.set("timeout", ConfigValue::Float(30.5));

    println!("配置管理系统:");
    println!("  数据库 URL: {:?}", config.get_string("database_url"));
    println!("  最大连接数: {:?}", config.get_integer("max_connections"));
    println!("  调试模式: {:?}", config.get("debug_mode"));

    // 列出所有配置
    println!("\n所有配置项:");
    for (key, value) in &config.settings {
        println!("  {}: {}", key, value);
    }
}

/// 图算法示例
fn graph_algorithm_example() {
    // 使用 HashMap 表示图的邻接表
    type Graph = HashMap<i32, Vec<i32>>;

    let mut graph: Graph = HashMap::new();

    // 构建图
    graph.insert(1, vec![2, 3]);
    graph.insert(2, vec![1, 4]);
    graph.insert(3, vec![1, 4, 5]);
    graph.insert(4, vec![2, 3]);
    graph.insert(5, vec![3]);

    println!("图的邻接表表示:");
    for (node, neighbors) in &graph {
        println!("  节点 {}: 连接到 {:?}", node, neighbors);
    }

    // 计算每个节点的度数
    let mut degree_count = HashMap::new();
    for (node, neighbors) in &graph {
        degree_count.insert(*node, neighbors.len());
    }

    println!("\n节点度数统计:");
    for (node, degree) in &degree_count {
        println!("  节点 {}: 度数 {}", node, degree);
    }

    // 简单的 BFS 路径查找
    fn bfs_path(graph: &Graph, start: i32, end: i32) -> Option<Vec<i32>> {
        use std::collections::VecDeque;

        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();
        let mut parent = HashMap::new();

        queue.push_back(start);
        visited.insert(start, true);

        while let Some(current) = queue.pop_front() {
            if current == end {
                // 重构路径
                let mut path = Vec::new();
                let mut node = end;

                while let Some(&p) = parent.get(&node) {
                    path.push(node);
                    node = p;
                }
                path.push(start);
                path.reverse();
                return Some(path);
            }

            if let Some(neighbors) = graph.get(&current) {
                for &neighbor in neighbors {
                    if !visited.contains_key(&neighbor) {
                        visited.insert(neighbor, true);
                        parent.insert(neighbor, current);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        None
    }

    if let Some(path) = bfs_path(&graph, 1, 5) {
        println!("\n从节点 1 到节点 5 的路径: {:?}", path);
    } else {
        println!("\n未找到从节点 1 到节点 5 的路径");
    }
}
