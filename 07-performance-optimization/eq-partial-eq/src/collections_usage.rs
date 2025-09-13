//! # 集合使用模块
//!
//! 本模块演示 PartialEq 和 Eq 在各种集合中的使用

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet, VecDeque, LinkedList};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// 演示集合中的使用
pub fn demonstrate_collections_usage() {
    println!("\n🔸 集合中的 PartialEq/Eq 使用:");
    
    vector_usage();
    hashmap_usage();
    hashset_usage();
    btree_usage();
    other_collections_usage();
    performance_considerations();
}

/// Vector 中的使用
fn vector_usage() {
    println!("\n  📌 Vector 中的使用:");
    
    let numbers = vec![1, 2, 3, 4, 5, 2, 3];
    
    println!("\n    🔹 Vec 的查找操作 (需要 PartialEq):");
    println!("      contains(3): {}", numbers.contains(&3));
    println!("      position of 2: {:?}", numbers.iter().position(|&x| x == 2));
    println!("      rposition of 2: {:?}", numbers.iter().rposition(|&x| x == 2));
    
    // 自定义类型的 Vec
    #[derive(Debug, PartialEq, Clone)]
    struct Person {
        name: String,
        age: u32,
    }
    
    let people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];
    
    let alice = Person { name: "Alice".to_string(), age: 30 };
    
    println!("\n    🔹 自定义类型的查找:");
    println!("      contains Alice: {}", people.contains(&alice));
    
    // 使用闭包进行复杂查找
    let found_bob = people.iter().find(|p| p.name == "Bob");
    println!("      find Bob: {:?}", found_bob);
    
    // Vec 的比较
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![1, 2, 3];
    let vec3 = vec![3, 2, 1];
    
    println!("\n    🔹 Vec 的比较 (需要元素实现 PartialEq):");
    println!("      {:?} == {:?} : {}", vec1, vec2, vec1 == vec2);
    println!("      {:?} == {:?} : {} (顺序重要)", vec1, vec3, vec1 == vec3);
    
    // 去重操作
    let mut duplicates = vec![1, 2, 2, 3, 3, 3, 4];
    duplicates.dedup();  // 需要 PartialEq
    println!("      去重后: {:?}", duplicates);
}

/// HashMap 中的使用
fn hashmap_usage() {
    println!("\n  📌 HashMap 中的使用:");
    
    println!("\n    🔹 HashMap 键的要求 (Eq + Hash):");
    
    // 基本类型作为键
    let mut int_map: HashMap<i32, &str> = HashMap::new();
    int_map.insert(1, "one");
    int_map.insert(2, "two");
    int_map.insert(1, "ONE");  // 覆盖
    
    println!("      整数键: {:?}", int_map);
    
    // 字符串作为键
    let mut string_map: HashMap<String, i32> = HashMap::new();
    string_map.insert("hello".to_string(), 1);
    string_map.insert("world".to_string(), 2);
    
    println!("      字符串键: {:?}", string_map);
    
    // 自定义类型作为键
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct Coordinate {
        x: i32,
        y: i32,
    }
    
    let mut coord_map: HashMap<Coordinate, &str> = HashMap::new();
    coord_map.insert(Coordinate { x: 0, y: 0 }, "origin");
    coord_map.insert(Coordinate { x: 1, y: 1 }, "diagonal");
    
    println!("      坐标键: {:?}", coord_map);
    
    // 查找操作
    let key = Coordinate { x: 0, y: 0 };
    println!("      查找 {:?}: {:?}", key, coord_map.get(&key));
    
    // 浮点数不能作为键
    println!("\n    ❌ 浮点数不能作为 HashMap 键:");
    println!("      原因: f64 只实现 PartialEq，不实现 Eq");
    // let mut float_map: HashMap<f64, i32> = HashMap::new(); // 编译错误!
    
    // 解决方案：包装浮点数
    #[derive(Debug, Clone)]
    struct OrderedFloat(f64);
    
    impl PartialEq for OrderedFloat {
        fn eq(&self, other: &Self) -> bool {
            if self.0.is_nan() && other.0.is_nan() {
                true
            } else {
                self.0 == other.0
            }
        }
    }
    
    impl Eq for OrderedFloat {}
    
    impl Hash for OrderedFloat {
        fn hash<H: Hasher>(&self, state: &mut H) {
            if self.0.is_nan() {
                0u64.hash(state);
            } else {
                self.0.to_bits().hash(state);
            }
        }
    }
    
    let mut ordered_float_map: HashMap<OrderedFloat, &str> = HashMap::new();
    ordered_float_map.insert(OrderedFloat(1.0), "one");
    ordered_float_map.insert(OrderedFloat(f64::NAN), "not a number");
    
    println!("      有序浮点数键: {:?}", ordered_float_map);
}

/// HashSet 中的使用
fn hashset_usage() {
    println!("\n  📌 HashSet 中的使用:");
    
    println!("\n    🔹 HashSet 元素要求 (Eq + Hash):");
    
    let mut int_set: HashSet<i32> = HashSet::new();
    int_set.insert(1);
    int_set.insert(2);
    int_set.insert(1);  // 重复，不会插入
    
    println!("      整数集合: {:?}", int_set);
    
    // 字符串集合
    let mut string_set: HashSet<String> = HashSet::new();
    string_set.insert("hello".to_string());
    string_set.insert("world".to_string());
    string_set.insert("hello".to_string());  // 重复
    
    println!("      字符串集合: {:?}", string_set);
    
    // 集合操作
    let set1: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let set2: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();
    
    println!("\n    🔹 集合操作:");
    println!("      set1: {:?}", set1);
    println!("      set2: {:?}", set2);
    
    let intersection: HashSet<_> = set1.intersection(&set2).collect();
    println!("      交集: {:?}", intersection);
    
    let union: HashSet<_> = set1.union(&set2).collect();
    println!("      并集: {:?}", union);
    
    let difference: HashSet<_> = set1.difference(&set2).collect();
    println!("      差集: {:?}", difference);
    
    // 自定义类型的集合
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct Tag {
        name: String,
        category: String,
    }
    
    let mut tag_set: HashSet<Tag> = HashSet::new();
    tag_set.insert(Tag { name: "rust".to_string(), category: "language".to_string() });
    tag_set.insert(Tag { name: "python".to_string(), category: "language".to_string() });
    tag_set.insert(Tag { name: "rust".to_string(), category: "language".to_string() });  // 重复
    
    println!("\n    🔹 自定义类型集合:");
    println!("      标签集合: {:?}", tag_set);
    
    let rust_tag = Tag { name: "rust".to_string(), category: "language".to_string() };
    println!("      contains rust: {}", tag_set.contains(&rust_tag));
}

/// BTree 集合中的使用
fn btree_usage() {
    println!("\n  📌 BTree 集合中的使用:");
    
    println!("\n    🔹 BTreeMap (需要 Ord):");
    
    let mut btree_map: BTreeMap<i32, &str> = BTreeMap::new();
    btree_map.insert(3, "three");
    btree_map.insert(1, "one");
    btree_map.insert(2, "two");
    
    println!("      有序映射: {:?}", btree_map);
    
    // 范围查询
    let range: BTreeMap<_, _> = btree_map.range(1..=2).collect();
    println!("      范围 [1,2]: {:?}", range);
    
    println!("\n    🔹 BTreeSet (需要 Ord):");
    
    let mut btree_set: BTreeSet<i32> = BTreeSet::new();
    btree_set.insert(3);
    btree_set.insert(1);
    btree_set.insert(2);
    
    println!("      有序集合: {:?}", btree_set);
    
    // 自定义类型需要实现 Ord
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
    struct Priority {
        level: u32,
        name: String,
    }
    
    let mut priority_set: BTreeSet<Priority> = BTreeSet::new();
    priority_set.insert(Priority { level: 2, name: "medium".to_string() });
    priority_set.insert(Priority { level: 1, name: "low".to_string() });
    priority_set.insert(Priority { level: 3, name: "high".to_string() });
    
    println!("      优先级集合: {:?}", priority_set);
    
    println!("\n    ❌ 浮点数不能用于 BTreeMap/BTreeSet:");
    println!("      原因: f64 不实现 Ord (因为 NaN 的存在)");
    // let mut float_btree: BTreeSet<f64> = BTreeSet::new(); // 编译错误!
}

/// 其他集合中的使用
fn other_collections_usage() {
    println!("\n  📌 其他集合中的使用:");
    
    // VecDeque
    println!("\n    🔹 VecDeque (需要 PartialEq 进行查找):");
    let mut deque: VecDeque<i32> = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    
    println!("      deque: {:?}", deque);
    println!("      contains 1: {}", deque.contains(&1));
    
    // LinkedList
    println!("\n    🔹 LinkedList (需要 PartialEq 进行查找):");
    let mut list: LinkedList<&str> = LinkedList::new();
    list.push_back("hello");
    list.push_back("world");
    list.push_front("hi");
    
    println!("      list: {:?}", list);
    println!("      contains 'world': {}", list.contains(&"world"));
    
    // 集合的比较
    println!("\n    🔹 集合的比较:");
    
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![1, 2, 3];
    println!("      Vec 比较: {:?} == {:?} : {}", vec1, vec2, vec1 == vec2);
    
    let mut map1: HashMap<i32, &str> = HashMap::new();
    map1.insert(1, "one");
    map1.insert(2, "two");
    
    let mut map2: HashMap<i32, &str> = HashMap::new();
    map2.insert(2, "two");
    map2.insert(1, "one");
    
    println!("      HashMap 比较: {:?} == {:?} : {} (忽略顺序)", map1, map2, map1 == map2);
}

/// 性能考虑
fn performance_considerations() {
    println!("\n  📌 性能考虑:");
    
    println!("\n    🔹 Hash 函数的重要性:");
    
    // 演示不同 Hash 实现的影响
    #[derive(Debug, PartialEq, Eq, Clone)]
    struct BadHash {
        value: i32,
    }
    
    impl Hash for BadHash {
        fn hash<H: Hasher>(&self, _state: &mut H) {
            // 糟糕的 hash 实现 - 所有值都有相同的 hash
            42u64.hash(_state);
        }
    }
    
    #[derive(Debug, PartialEq, Eq, Clone)]
    struct GoodHash {
        value: i32,
    }
    
    impl Hash for GoodHash {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.value.hash(state);
        }
    }
    
    println!("      糟糕的 Hash 实现会导致所有元素聚集在同一个桶中");
    println!("      好的 Hash 实现能均匀分布元素，提高性能");
    
    // 演示 Hash 计算
    let bad = BadHash { value: 123 };
    let good = GoodHash { value: 123 };
    
    let mut hasher1 = DefaultHasher::new();
    bad.hash(&mut hasher1);
    let bad_hash = hasher1.finish();
    
    let mut hasher2 = DefaultHasher::new();
    good.hash(&mut hasher2);
    let good_hash = hasher2.finish();
    
    println!("      BadHash(123) hash: {}", bad_hash);
    println!("      GoodHash(123) hash: {}", good_hash);
    
    println!("\n    🔹 比较操作的复杂度:");
    println!("      - 基本类型: O(1)");
    println!("      - 字符串: O(n) where n = 字符串长度");
    println!("      - 结构体: 取决于字段数量和类型");
    println!("      - 集合: O(n) where n = 元素数量");
    
    println!("\n    🔹 选择合适的集合类型:");
    println!("      - 需要快速查找: HashMap/HashSet (平均 O(1))");
    println!("      - 需要有序: BTreeMap/BTreeSet (O(log n))");
    println!("      - 简单线性查找: Vec (O(n))");
    println!("      - 频繁插入/删除两端: VecDeque");
}

/// 演示集合中的常见陷阱
pub fn demonstrate_collection_pitfalls() {
    println!("\n🔸 集合使用中的常见陷阱:");
    
    println!("\n  📌 Hash 和 Eq 不一致的后果:");
    
    #[derive(Debug, Clone)]
    struct InconsistentType {
        a: i32,
        b: i32,
    }
    
    impl PartialEq for InconsistentType {
        fn eq(&self, other: &Self) -> bool {
            self.a == other.a  // 只比较 a
        }
    }
    
    impl Eq for InconsistentType {}
    
    impl Hash for InconsistentType {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.a.hash(state);
            self.b.hash(state);  // 包含 b，但 eq 不比较 b
        }
    }
    
    let item1 = InconsistentType { a: 1, b: 2 };
    let item2 = InconsistentType { a: 1, b: 3 };
    
    println!("    {:?} == {:?} : {}", item1, item2, item1 == item2);
    
    let mut inconsistent_map: HashMap<InconsistentType, &str> = HashMap::new();
    inconsistent_map.insert(item1.clone(), "first");
    inconsistent_map.insert(item2.clone(), "second");
    
    println!("    HashMap 可能出现异常行为: {:?}", inconsistent_map);
    println!("    查找可能失败，即使键在逻辑上存在");
    
    println!("\n  📌 修改已插入的键:");
    println!("    ❌ 不要修改已经插入到 HashMap/HashSet 中的键");
    println!("    ❌ 这会破坏内部数据结构的一致性");
    
    println!("\n  📌 浮点数的特殊处理:");
    println!("    ❌ 直接使用 f64 作为键会编译失败");
    println!("    ✅ 使用包装类型处理 NaN 和精度问题");
    
    println!("\n  📌 性能陷阱:");
    println!("    ❌ 糟糕的 Hash 实现导致性能退化");
    println!("    ❌ 复杂的 PartialEq 实现影响查找性能");
    println!("    ✅ 简单、快速、均匀分布的 Hash 函数");
    println!("    ✅ 高效的相等性比较实现");
}