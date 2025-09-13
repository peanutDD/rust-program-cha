//! # 边界情况分析
//!
//! 本模块深入分析 Eq 和 PartialEq 的边界情况、常见陷阱和解决方案

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::f64;

/// 运行所有边界情况分析
pub fn run_edge_case_analysis() {
    println!("\n🔸 边界情况分析:");
    
    floating_point_edge_cases();
    hash_eq_inconsistency();
    partial_eq_without_eq();
    cross_type_comparisons();
    recursive_structures();
    zero_sized_types();
    unsafe_implementations();
    performance_traps();
}

/// 浮点数边界情况
fn floating_point_edge_cases() {
    println!("\n  📌 浮点数边界情况:");
    
    // NaN 的特殊行为
    println!("\n    🔹 NaN 的特殊行为:");
    
    let nan1 = f64::NAN;
    let nan2 = f64::NAN;
    let nan3 = 0.0 / 0.0;
    
    // NaN 不等于任何值，包括自身
    assert_ne!(nan1, nan1);
    assert_ne!(nan1, nan2);
    assert_ne!(nan1, nan3);
    assert_ne!(nan1, 1.0);
    assert_ne!(nan1, f64::INFINITY);
    
    println!("      ❌ NaN != NaN (违反自反性)");
    println!("      ❌ NaN != 任何其他值");
    
    // 这导致 f64 只实现 PartialEq，不实现 Eq
    // let _: Box<dyn Eq> = Box::new(1.0f64); // 编译错误！
    
    // 无穷大的行为
    println!("\n    🔹 无穷大的行为:");
    
    let inf = f64::INFINITY;
    let neg_inf = f64::NEG_INFINITY;
    
    assert_eq!(inf, inf);
    assert_eq!(neg_inf, neg_inf);
    assert_ne!(inf, neg_inf);
    assert_ne!(inf, 1.0);
    
    println!("      ✅ INFINITY == INFINITY");
    println!("      ✅ NEG_INFINITY == NEG_INFINITY");
    println!("      ✅ INFINITY != NEG_INFINITY");
    
    // 零的特殊情况
    println!("\n    🔹 零的特殊情况:");
    
    let pos_zero: f64 = 0.0;
    let neg_zero: f64 = -0.0;
    
    assert_eq!(pos_zero, neg_zero);  // IEEE 754 标准
    println!("      ✅ +0.0 == -0.0 (IEEE 754 标准)");
    
    // 但在某些上下文中可能需要区分
    assert_eq!(pos_zero.to_bits(), 0x0000000000000000);
    assert_eq!(neg_zero.to_bits(), 0x8000000000000000);
    println!("      ⚠️  但二进制表示不同");
    
    // 浮点数精度问题
    println!("\n    🔹 浮点数精度问题:");
    
    let a: f64 = 0.1 + 0.2;
    let b: f64 = 0.3;
    
    assert_ne!(a, b);  // 由于精度问题
    println!("      ❌ 0.1 + 0.2 != 0.3 (精度问题)");
    println!("         实际值: {} vs {}", a, b);
    
    // 解决方案：使用 epsilon 比较
    let epsilon = f64::EPSILON * 10.0;
    assert!((a - b).abs() < epsilon);
    println!("      ✅ 使用 epsilon 比较: |{} - {}| < {}", a, b, epsilon);
}

/// Hash 和 Eq 不一致的问题
fn hash_eq_inconsistency() {
    println!("\n  📌 Hash 和 Eq 不一致的问题:");
    
    // 错误示例：Hash 和 Eq 不一致
    println!("\n    🔹 错误示例 - Hash 和 Eq 不一致:");
    
    #[derive(Debug, Clone)]
    struct BadExample {
        id: u32,
        name: String,
        ignored_field: String,  // 在 PartialEq 中被忽略，但在 Hash 中被包含
    }
    
    impl PartialEq for BadExample {
        fn eq(&self, other: &Self) -> bool {
            // 只比较 id 和 name，忽略 ignored_field
            self.id == other.id && self.name == other.name
        }
    }
    
    impl Eq for BadExample {}
    
    impl Hash for BadExample {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // 错误：包含了在 PartialEq 中被忽略的字段
            self.id.hash(state);
            self.name.hash(state);
            self.ignored_field.hash(state);  // 这是错误的！
        }
    }
    
    let item1 = BadExample {
        id: 1,
        name: "test".to_string(),
        ignored_field: "field1".to_string(),
    };
    
    let item2 = BadExample {
        id: 1,
        name: "test".to_string(),
        ignored_field: "field2".to_string(),  // 不同的值
    };
    
    // 这两个对象相等
    assert_eq!(item1, item2);
    println!("      ✅ item1 == item2 (根据 PartialEq)");
    
    // 但 hash 值不同
    let mut hasher1 = DefaultHasher::new();
    item1.hash(&mut hasher1);
    let hash1 = hasher1.finish();
    
    let mut hasher2 = DefaultHasher::new();
    item2.hash(&mut hasher2);
    let hash2 = hasher2.finish();
    
    if hash1 != hash2 {
        println!("      ❌ 但 hash 值不同: {} vs {} (违反 Hash 一致性)", hash1, hash2);
        
        // 这会导致 HashMap 行为异常
        let mut map: HashMap<BadExample, &str> = HashMap::new();
        map.insert(item1.clone(), "value1");
        
        // 可能找不到相等的键！
        if map.get(&item2).is_none() {
            println!("      ❌ HashMap 中找不到相等的键！");
        }
    }
    
    // 正确示例：Hash 和 Eq 一致
    println!("\n    🔹 正确示例 - Hash 和 Eq 一致:");
    
    #[derive(Debug, Clone)]
    struct GoodExample {
        id: u32,
        name: String,
        ignored_field: String,
    }
    
    impl PartialEq for GoodExample {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id && self.name == other.name
        }
    }
    
    impl Eq for GoodExample {}
    
    impl Hash for GoodExample {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // 正确：只包含在 PartialEq 中使用的字段
            self.id.hash(state);
            self.name.hash(state);
            // 不包含 ignored_field
        }
    }
    
    let good1 = GoodExample {
        id: 1,
        name: "test".to_string(),
        ignored_field: "field1".to_string(),
    };
    
    let good2 = GoodExample {
        id: 1,
        name: "test".to_string(),
        ignored_field: "field2".to_string(),
    };
    
    assert_eq!(good1, good2);
    
    let mut hasher3 = DefaultHasher::new();
    good1.hash(&mut hasher3);
    let hash3 = hasher3.finish();
    
    let mut hasher4 = DefaultHasher::new();
    good2.hash(&mut hasher4);
    let hash4 = hasher4.finish();
    
    assert_eq!(hash3, hash4);
    println!("      ✅ 相等对象有相同 hash 值: {}", hash3);
    
    // HashMap 行为正常
    let mut good_map: HashMap<GoodExample, &str> = HashMap::new();
    good_map.insert(good1.clone(), "value1");
    assert_eq!(good_map.get(&good2), Some(&"value1"));
    println!("      ✅ HashMap 行为正常");
}

/// PartialEq 但不是 Eq 的情况
fn partial_eq_without_eq() {
    println!("\n  📌 PartialEq 但不是 Eq 的情况:");
    
    // 浮点数包装器 - 不满足 Eq
    println!("\n    🔹 浮点数包装器:");
    
    #[derive(Debug, Clone, Copy)]
    struct FloatValue(f64);
    
    impl PartialEq for FloatValue {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    
    // 不能实现 Eq，因为 NaN != NaN
    // impl Eq for FloatValue {}  // 这是错误的！
    
    let f1 = FloatValue(1.0);
    let f2 = FloatValue(1.0);
    let nan_f = FloatValue(f64::NAN);
    
    assert_eq!(f1, f2);
    assert_ne!(nan_f, nan_f);  // 违反自反性
    
    println!("      ✅ 普通浮点数相等");
    println!("      ❌ NaN 不等于自身 (不满足 Eq)");
    
    // 部分排序的例子
    println!("\n    🔹 部分排序的例子:");
    
    #[derive(Debug, Clone)]
    struct PartiallyOrdered {
        value: Option<i32>,
    }
    
    impl PartialEq for PartiallyOrdered {
        fn eq(&self, other: &Self) -> bool {
            match (self.value, other.value) {
                (Some(a), Some(b)) => a == b,
                (None, None) => true,
                _ => false,
            }
        }
    }
    
    // 如果我们定义某些值不可比较，就不能实现 Eq
    // 例如：某些特殊状态的对象
    
    let p1 = PartiallyOrdered { value: Some(1) };
    let p2 = PartiallyOrdered { value: Some(1) };
    let p3 = PartiallyOrdered { value: None };
    let p4 = PartiallyOrdered { value: None };
    
    assert_eq!(p1, p2);
    assert_eq!(p3, p4);
    assert_ne!(p1, p3);
    
    println!("      ✅ 部分排序类型的相等性比较");
}

/// 跨类型比较的复杂性
fn cross_type_comparisons() {
    println!("\n  📌 跨类型比较的复杂性:");
    
    // 数值类型之间的比较
    println!("\n    🔹 数值类型之间的比较:");
    
    #[derive(Debug)]
    struct IntWrapper(i32);
    
    #[derive(Debug)]
    struct FloatWrapper(f64);
    
    impl PartialEq<FloatWrapper> for IntWrapper {
        fn eq(&self, other: &FloatWrapper) -> bool {
            self.0 as f64 == other.0
        }
    }
    
    impl PartialEq<IntWrapper> for FloatWrapper {
        fn eq(&self, other: &IntWrapper) -> bool {
            self.0 == other.0 as f64
        }
    }
    
    let int_val = IntWrapper(42);
    let float_val = FloatWrapper(42.0);
    let float_val2 = FloatWrapper(42.1);
    
    assert_eq!(int_val, float_val);
    assert_ne!(int_val, float_val2);
    assert_eq!(float_val, int_val);  // 对称性
    
    println!("      ✅ 跨类型数值比较: {:?} == {:?}", int_val, float_val);
    
    // 字符串类型之间的比较
    println!("\n    🔹 字符串类型之间的比较:");
    
    let string = "hello".to_string();
    let str_slice = "hello";
    let cow_str = std::borrow::Cow::Borrowed("hello");
    
    assert_eq!(string, str_slice);
    assert_eq!(string, cow_str);
    assert_eq!(str_slice, cow_str);
    
    println!("      ✅ 不同字符串类型之间的比较");
    
    // 复杂的跨类型比较陷阱
    println!("\n    🔹 跨类型比较的陷阱:");
    
    #[derive(Debug)]
    struct Version {
        major: u32,
        minor: u32,
        patch: u32,
    }
    
    #[derive(Debug)]
    struct VersionString(String);
    
    impl PartialEq<VersionString> for Version {
        fn eq(&self, other: &VersionString) -> bool {
            let version_str = format!("{}.{}.{}", self.major, self.minor, self.patch);
            version_str == other.0
        }
    }
    
    impl PartialEq<Version> for VersionString {
        fn eq(&self, other: &Version) -> bool {
            let version_str = format!("{}.{}.{}", other.major, other.minor, other.patch);
            self.0 == version_str
        }
    }
    
    let version = Version { major: 1, minor: 2, patch: 3 };
    let version_str = VersionString("1.2.3".to_string());
    let bad_version_str = VersionString("01.02.03".to_string());
    
    assert_eq!(version, version_str);
    assert_ne!(version, bad_version_str);  // 格式不同
    
    println!("      ✅ 版本比较: {:?} == {:?}", version, version_str);
    println!("      ❌ 格式敏感: {:?} != {:?}", version, bad_version_str);
}

/// 递归结构的处理
fn recursive_structures() {
    println!("\n  📌 递归结构的处理:");
    
    // 简单的递归结构
    println!("\n    🔹 简单的递归结构:");
    
    #[derive(Debug, Clone)]
    struct Node {
        value: i32,
        next: Option<Box<Node>>,
    }
    
    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value && self.next == other.next
        }
    }
    
    impl Eq for Node {}
    
    let node1 = Node {
        value: 1,
        next: Some(Box::new(Node {
            value: 2,
            next: None,
        })),
    };
    
    let node2 = Node {
        value: 1,
        next: Some(Box::new(Node {
            value: 2,
            next: None,
        })),
    };
    
    let node3 = Node {
        value: 1,
        next: Some(Box::new(Node {
            value: 3,  // 不同的值
            next: None,
        })),
    };
    
    assert_eq!(node1, node2);
    assert_ne!(node1, node3);
    
    println!("      ✅ 递归结构比较正常工作");
    
    // 循环引用的问题（使用 Rc 和 RefCell）
    println!("\n    🔹 循环引用的潜在问题:");
    
    use std::rc::Rc;
    use std::cell::RefCell;
    
    #[derive(Debug)]
    struct CyclicNode {
        value: i32,
        next: Option<Rc<RefCell<CyclicNode>>>,
    }
    
    // 注意：为循环结构实现 PartialEq 需要特别小心
    // 可能导致无限递归！
    
    impl PartialEq for CyclicNode {
        fn eq(&self, other: &Self) -> bool {
            // 简单实现：只比较值，不比较引用
            // 这避免了循环引用的问题，但可能不是期望的语义
            self.value == other.value
        }
    }
    
    let cyclic1 = Rc::new(RefCell::new(CyclicNode {
        value: 1,
        next: None,
    }));
    
    let cyclic2 = Rc::new(RefCell::new(CyclicNode {
        value: 1,
        next: None,
    }));
    
    // 创建循环引用
    cyclic1.borrow_mut().next = Some(cyclic1.clone());
    cyclic2.borrow_mut().next = Some(cyclic2.clone());
    
    // 比较（只比较值，避免无限递归）
    assert_eq!(*cyclic1.borrow(), *cyclic2.borrow());
    
    println!("      ⚠️  循环结构需要特殊处理以避免无限递归");
}

/// 零大小类型的特殊情况
fn zero_sized_types() {
    println!("\n  📌 零大小类型 (ZST) 的特殊情况:");
    
    // 单元类型
    println!("\n    🔹 单元类型:");
    
    let unit1 = ();
    let unit2 = ();
    
    assert_eq!(unit1, unit2);
    assert_eq!(std::mem::size_of_val(&unit1), 0);
    
    println!("      ✅ 单元类型: () == (), 大小为 0 字节");
    
    // 空结构体
    println!("\n    🔹 空结构体:");
    
    #[derive(Debug, PartialEq, Eq, Hash)]
    struct EmptyStruct;
    
    #[derive(Debug, PartialEq, Eq, Hash)]
    struct AnotherEmptyStruct;
    
    let empty1 = EmptyStruct;
    let empty2 = EmptyStruct;
    // let another = AnotherEmptyStruct;  // 不同类型
    
    assert_eq!(empty1, empty2);
    assert_eq!(std::mem::size_of_val(&empty1), 0);
    // assert_ne!(empty1, another);  // 编译错误：不同类型
    
    println!("      ✅ 空结构体相等性，大小为 0 字节");
    
    // 标记类型
    println!("\n    🔹 标记类型 (Marker Types):");
    
    use std::marker::PhantomData;
    
    #[derive(Debug, PartialEq, Eq)]
    struct Marker<T> {
        _phantom: PhantomData<T>,
    }
    
    let marker1: Marker<i32> = Marker { _phantom: PhantomData };
    let marker2: Marker<i32> = Marker { _phantom: PhantomData };
    let _marker3: Marker<f64> = Marker { _phantom: PhantomData };
    
    assert_eq!(marker1, marker2);
    // assert_ne!(marker1, marker3);  // 编译错误：不同类型
    assert_eq!(std::mem::size_of_val(&marker1), 0);
    
    println!("      ✅ 标记类型相等性，大小为 0 字节");
    
    // ZST 在集合中的行为
    println!("\n    🔹 ZST 在集合中的行为:");
    
    let mut set: HashSet<EmptyStruct> = HashSet::new();
    set.insert(EmptyStruct);
    set.insert(EmptyStruct);  // 重复插入
    
    assert_eq!(set.len(), 1);
    println!("      ✅ HashSet 正确处理 ZST 重复");
    
    let mut map: HashMap<EmptyStruct, &str> = HashMap::new();
    map.insert(EmptyStruct, "value1");
    map.insert(EmptyStruct, "value2");  // 覆盖
    
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&EmptyStruct), Some(&"value2"));
    println!("      ✅ HashMap 正确处理 ZST 键");
}

/// 不安全实现的陷阱
fn unsafe_implementations() {
    println!("\n  📌 不安全实现的陷阱:");
    
    // 违反安全性的实现
    println!("\n    🔹 违反安全性的实现示例:");
    
    #[derive(Debug)]
    struct UnsafeExample {
        ptr: *const i32,
    }
    
    // 危险的实现：比较指针值而不是指向的内容
    impl PartialEq for UnsafeExample {
        fn eq(&self, other: &Self) -> bool {
            // 错误：比较指针地址而不是内容
            self.ptr == other.ptr
        }
    }
    
    let value1 = 42;
    let value2 = 42;
    
    let unsafe1 = UnsafeExample { ptr: &value1 };
    let unsafe2 = UnsafeExample { ptr: &value2 };
    let unsafe3 = UnsafeExample { ptr: &value1 };
    
    // 即使指向相同的值，也可能不相等（因为地址不同）
    assert_ne!(unsafe1, unsafe2);
    assert_eq!(unsafe1, unsafe3);
    
    println!("      ❌ 指针比较可能导致语义上相等的对象不相等");
    
    // 更好的实现：比较指向的内容
    #[derive(Debug)]
    struct SaferExample {
        ptr: *const i32,
    }
    
    impl PartialEq for SaferExample {
        fn eq(&self, other: &Self) -> bool {
            unsafe {
                // 更好：比较指向的内容（但仍需要确保指针有效）
                if self.ptr.is_null() && other.ptr.is_null() {
                    true
                } else if self.ptr.is_null() || other.ptr.is_null() {
                    false
                } else {
                    *self.ptr == *other.ptr
                }
            }
        }
    }
    
    let safer1 = SaferExample { ptr: &value1 };
    let safer2 = SaferExample { ptr: &value2 };
    
    assert_eq!(safer1, safer2);  // 现在相等了
    
    println!("      ✅ 比较指向的内容更符合语义");
    
    // 空指针的处理
    let null1 = SaferExample { ptr: std::ptr::null() };
    let null2 = SaferExample { ptr: std::ptr::null() };
    
    assert_eq!(null1, null2);
    assert_ne!(safer1, null1);
    
    println!("      ✅ 正确处理空指针情况");
}

/// 性能陷阱
fn performance_traps() {
    println!("\n  📌 性能陷阱:");
    
    // 昂贵的比较操作
    println!("\n    🔹 昂贵的比较操作:");
    
    #[derive(Debug, Clone)]
    struct ExpensiveComparison {
        data: Vec<String>,
    }
    
    impl PartialEq for ExpensiveComparison {
        fn eq(&self, other: &Self) -> bool {
            // 这可能很昂贵，特别是对于大型向量
            self.data == other.data
        }
    }
    
    let expensive1 = ExpensiveComparison {
        data: vec!["a".to_string(); 1000],
    };
    
    let expensive2 = ExpensiveComparison {
        data: vec!["a".to_string(); 1000],
    };
    
    let expensive3 = ExpensiveComparison {
        data: vec!["b".to_string(); 1000],
    };
    
    // 这些比较可能很慢
    assert_eq!(expensive1, expensive2);
    assert_ne!(expensive1, expensive3);
    
    println!("      ⚠️  大型数据结构的比较可能很昂贵");
    
    // 优化策略：早期退出
    #[derive(Debug, Clone)]
    struct OptimizedComparison {
        id: u64,  // 快速比较的字段
        data: Vec<String>,
    }
    
    impl PartialEq for OptimizedComparison {
        fn eq(&self, other: &Self) -> bool {
            // 首先比较快速字段
            if self.id != other.id {
                return false;
            }
            
            // 然后比较长度
            if self.data.len() != other.data.len() {
                return false;
            }
            
            // 最后比较内容
            self.data == other.data
        }
    }
    
    let opt1 = OptimizedComparison {
        id: 1,
        data: vec!["a".to_string(); 1000],
    };
    
    let opt2 = OptimizedComparison {
        id: 2,  // 不同的 ID
        data: vec!["a".to_string(); 1000],
    };
    
    // 这个比较会很快（早期退出）
    assert_ne!(opt1, opt2);
    
    println!("      ✅ 优化：使用早期退出策略");
    
    // Hash 计算的性能陷阱
    println!("\n    🔹 Hash 计算的性能陷阱:");
    
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct ExpensiveHash {
        data: Vec<u8>,
    }
    
    impl Hash for ExpensiveHash {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // 对整个向量进行 hash 可能很昂贵
            self.data.hash(state);
        }
    }
    
    // 更好的策略：只 hash 部分数据或使用缓存
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct OptimizedHash {
        data: Vec<u8>,
        cached_hash: Option<u64>,  // 缓存 hash 值
    }
    
    impl Hash for OptimizedHash {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // 可以只 hash 前几个字节和长度
            self.data.len().hash(state);
            if !self.data.is_empty() {
                self.data[0].hash(state);
                if self.data.len() > 1 {
                    self.data[self.data.len() - 1].hash(state);
                }
            }
        }
    }
    
    println!("      ✅ 优化：只 hash 关键部分或使用缓存");
    
    // 字符串比较的性能考虑
    println!("\n    🔹 字符串比较的性能考虑:");
    
    let long_str1 = "a".repeat(10000);
    let long_str2 = "a".repeat(10000);
    let long_str3 = format!("{}b", "a".repeat(9999));
    
    // 这些比较的性能特征不同
    assert_eq!(long_str1, long_str2);  // 可能需要比较整个字符串
    assert_ne!(long_str1, long_str3);  // 可能在最后一个字符就能确定
    
    println!("      ⚠️  长字符串比较的性能取决于差异位置");
}

/// 总结所有边界情况
pub fn summarize_edge_cases() {
    println!("\n🔸 边界情况总结:");
    
    println!("\n  📋 主要陷阱和解决方案:");
    println!("    1. 浮点数 NaN 违反自反性 → 使用 PartialEq 而不是 Eq");
    println!("    2. Hash 和 Eq 不一致 → 确保相等对象有相同 hash 值");
    println!("    3. 跨类型比较复杂性 → 仔细设计对称性和传递性");
    println!("    4. 递归结构无限递归 → 使用访问标记或简化比较");
    println!("    5. 不安全指针比较 → 比较内容而不是地址");
    println!("    6. 性能陷阱 → 使用早期退出和优化策略");
    
    println!("\n  ✅ 最佳实践:");
    println!("    • 始终保持 Hash 和 Eq 的一致性");
    println!("    • 为浮点数类型只实现 PartialEq");
    println!("    • 在昂贵的比较中使用早期退出");
    println!("    • 测试边界情况和特殊值");
    println!("    • 文档化特殊的比较语义");
    println!("    • 考虑性能影响，特别是在集合中使用时");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_nan_behavior() {
        let nan = f64::NAN;
        assert_ne!(nan, nan);
    }
    
    #[test]
    fn test_infinity_behavior() {
        let inf = f64::INFINITY;
        let neg_inf = f64::NEG_INFINITY;
        
        assert_eq!(inf, inf);
        assert_eq!(neg_inf, neg_inf);
        assert_ne!(inf, neg_inf);
    }
    
    #[test]
    fn test_zero_behavior() {
        let pos_zero = 0.0;
        let neg_zero = -0.0;
        
        assert_eq!(pos_zero, neg_zero);
    }
    
    #[test]
    fn test_hash_consistency() {
        #[derive(PartialEq, Eq, Hash, Debug)]
    struct TestStruct {
        a: i32,
        b: String,
    }
        
        let s1 = TestStruct { a: 1, b: "test".to_string() };
        let s2 = TestStruct { a: 1, b: "test".to_string() };
        
        assert_eq!(s1, s2);
        
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher1 = DefaultHasher::new();
        s1.hash(&mut hasher1);
        let hash1 = hasher1.finish();
        
        let mut hasher2 = DefaultHasher::new();
        s2.hash(&mut hasher2);
        let hash2 = hasher2.finish();
        
        assert_eq!(hash1, hash2);
    }
    
    #[test]
    fn test_zst_behavior() {
        #[derive(PartialEq, Eq, Hash, Debug)]
    struct EmptyStruct;
        
        let e1 = EmptyStruct;
        let e2 = EmptyStruct;
        
        assert_eq!(e1, e2);
        assert_eq!(std::mem::size_of_val(&e1), 0);
        
        let mut set = HashSet::new();
        set.insert(e1);
        set.insert(e2);
        
        assert_eq!(set.len(), 1);
    }
}