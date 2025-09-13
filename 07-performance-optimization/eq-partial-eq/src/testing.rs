//! # 测试模块
//!
//! 本模块提供全面的测试和验证功能

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// 运行综合测试
pub fn run_comprehensive_tests() {
    println!("\n🔸 综合测试和验证:");
    
    test_equivalence_relations();
    test_hash_consistency();
    test_collection_behavior();
    test_edge_cases();
    run_property_tests();
}

/// 测试等价关系性质
fn test_equivalence_relations() {
    println!("\n  📌 等价关系性质测试:");
    
    // 测试自反性
    println!("\n    🔹 自反性测试 (x == x):");
    test_reflexivity();
    
    // 测试对称性
    println!("\n    🔹 对称性测试 (x == y ⟺ y == x):");
    test_symmetry();
    
    // 测试传递性
    println!("\n    🔹 传递性测试 (x == y && y == z ⟹ x == z):");
    test_transitivity();
}

/// 测试自反性
fn test_reflexivity() {
    // 整数类型 - 应该满足自反性
    let x = 42;
    assert_eq!(x, x);
    println!("      ✅ 整数自反性: {} == {}", x, x);
    
    // 字符串类型 - 应该满足自反性
    let s = "hello";
    assert_eq!(s, s);
    println!("      ✅ 字符串自反性: \"{}\" == \"{}\"", s, s);
    
    // 浮点数 - 普通值满足自反性
    let f = 1.0;
    assert_eq!(f, f);
    println!("      ✅ 浮点数自反性: {} == {}", f, f);
    
    // NaN - 违反自反性
    let nan = f64::NAN;
    assert_ne!(nan, nan);
    println!("      ❌ NaN 违反自反性: NaN != NaN");
    
    // 自定义类型
    #[derive(Debug, PartialEq, Eq)]
    struct Point { x: i32, y: i32 }
    
    let p = Point { x: 1, y: 2 };
    assert_eq!(p, p);
    println!("      ✅ 自定义类型自反性: {:?} == {:?}", p, p);
}

/// 测试对称性
fn test_symmetry() {
    // 基本类型
    let a = 42;
    let b = 42;
    assert_eq!(a == b, b == a);
    println!("      ✅ 整数对称性: {} == {} ⟺ {} == {}", a, b, b, a);
    
    // 字符串
    let s1 = "hello";
    let s2 = "hello";
    assert_eq!(s1 == s2, s2 == s1);
    println!("      ✅ 字符串对称性: \"{}\" == \"{}\" ⟺ \"{}\" == \"{}\"", s1, s2, s2, s1);
    
    // 自定义类型
    #[derive(Debug, PartialEq)]
    struct Person { name: String, age: u32 }
    
    let p1 = Person { name: "Alice".to_string(), age: 30 };
    let p2 = Person { name: "Alice".to_string(), age: 30 };
    assert_eq!(p1 == p2, p2 == p1);
    println!("      ✅ 自定义类型对称性验证通过");
    
    // 跨类型比较的对称性
    #[derive(Debug)]
    struct IntValue(i32);
    
    #[derive(Debug)]
    struct FloatValue(f64);
    
    impl PartialEq<FloatValue> for IntValue {
        fn eq(&self, other: &FloatValue) -> bool {
            self.0 as f64 == other.0
        }
    }
    
    impl PartialEq<IntValue> for FloatValue {
        fn eq(&self, other: &IntValue) -> bool {
            self.0 == other.0 as f64
        }
    }
    
    let int_val = IntValue(42);
    let float_val = FloatValue(42.0);
    assert_eq!(int_val == float_val, float_val == int_val);
    println!("      ✅ 跨类型对称性: {:?} == {:?} ⟺ {:?} == {:?}", 
             int_val, float_val, float_val, int_val);
}

/// 测试传递性
fn test_transitivity() {
    // 基本类型
    let a = 42;
    let b = 42;
    let c = 42;
    
    if a == b && b == c {
        assert_eq!(a, c);
        println!("      ✅ 整数传递性: {} == {} && {} == {} ⟹ {} == {}", 
                 a, b, b, c, a, c);
    }
    
    // 字符串
    let s1 = "hello";
    let s2 = "hello";
    let s3 = "hello";
    
    if s1 == s2 && s2 == s3 {
        assert_eq!(s1, s3);
        println!("      ✅ 字符串传递性验证通过");
    }
    
    // 自定义类型 - 忽略大小写的字符串
    #[derive(Debug, Clone)]
    struct CaseInsensitive(String);
    
    impl PartialEq for CaseInsensitive {
        fn eq(&self, other: &Self) -> bool {
            self.0.to_lowercase() == other.0.to_lowercase()
        }
    }
    
    let ci1 = CaseInsensitive("Hello".to_string());
    let ci2 = CaseInsensitive("HELLO".to_string());
    let ci3 = CaseInsensitive("hello".to_string());
    
    if ci1 == ci2 && ci2 == ci3 {
        assert_eq!(ci1, ci3);
        println!("      ✅ 自定义类型传递性: {:?} == {:?} == {:?}", ci1, ci2, ci3);
    }
}

/// 测试 Hash 一致性
fn test_hash_consistency() {
    println!("\n  📌 Hash 一致性测试:");
    
    println!("\n    🔹 相等对象必须有相同的 hash 值:");
    
    // 基本类型
    let a = 42;
    let b = 42;
    assert_eq!(a, b);
    
    let mut hasher1 = DefaultHasher::new();
    a.hash(&mut hasher1);
    let hash_a = hasher1.finish();
    
    let mut hasher2 = DefaultHasher::new();
    b.hash(&mut hasher2);
    let hash_b = hasher2.finish();
    
    assert_eq!(hash_a, hash_b);
    println!("      ✅ 整数 hash 一致性: hash({}) == hash({}) = {}", a, b, hash_a);
    
    // 字符串
    let s1 = "hello".to_string();
    let s2 = "hello".to_string();
    assert_eq!(s1, s2);
    
    let mut hasher3 = DefaultHasher::new();
    s1.hash(&mut hasher3);
    let hash_s1 = hasher3.finish();
    
    let mut hasher4 = DefaultHasher::new();
    s2.hash(&mut hasher4);
    let hash_s2 = hasher4.finish();
    
    assert_eq!(hash_s1, hash_s2);
    println!("      ✅ 字符串 hash 一致性验证通过");
    
    // 自定义类型
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct Coordinate { x: i32, y: i32 }
    
    let c1 = Coordinate { x: 1, y: 2 };
    let c2 = Coordinate { x: 1, y: 2 };
    assert_eq!(c1, c2);
    
    let mut hasher5 = DefaultHasher::new();
    c1.hash(&mut hasher5);
    let hash_c1 = hasher5.finish();
    
    let mut hasher6 = DefaultHasher::new();
    c2.hash(&mut hasher6);
    let hash_c2 = hasher6.finish();
    
    assert_eq!(hash_c1, hash_c2);
    println!("      ✅ 自定义类型 hash 一致性: {:?} hash = {}", c1, hash_c1);
    
    // 测试不相等对象的 hash 值（不要求不同，但最好不同）
    let c3 = Coordinate { x: 2, y: 1 };
    assert_ne!(c1, c3);
    
    let mut hasher7 = DefaultHasher::new();
    c3.hash(&mut hasher7);
    let hash_c3 = hasher7.finish();
    
    if hash_c1 != hash_c3 {
        println!("      ✅ 不同对象有不同 hash 值: {:?} hash = {}", c3, hash_c3);
    } else {
        println!("      ⚠️  不同对象有相同 hash 值 (hash 冲突): {:?} hash = {}", c3, hash_c3);
    }
}

/// 测试集合行为
fn test_collection_behavior() {
    println!("\n  📌 集合行为测试:");
    
    // HashMap 测试
    println!("\n    🔹 HashMap 行为测试:");
    
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct Key { id: u32, name: String }
    
    let mut map: HashMap<Key, &str> = HashMap::new();
    
    let key1 = Key { id: 1, name: "test".to_string() };
    let key2 = Key { id: 1, name: "test".to_string() };  // 相等的键
    
    map.insert(key1.clone(), "value1");
    map.insert(key2.clone(), "value2");  // 应该覆盖
    
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&key1), Some(&"value2"));
    println!("      ✅ HashMap 正确处理相等的键");
    
    // HashSet 测试
    println!("\n    🔹 HashSet 行为测试:");
    
    let mut set: HashSet<Key> = HashSet::new();
    set.insert(key1.clone());
    set.insert(key2.clone());  // 重复，不应该插入
    
    assert_eq!(set.len(), 1);
    assert!(set.contains(&key1));
    assert!(set.contains(&key2));
    println!("      ✅ HashSet 正确处理重复元素");
    
    // Vec 查找测试
    println!("\n    🔹 Vec 查找测试:");
    
    let vec = vec![key1.clone(), Key { id: 2, name: "other".to_string() }];
    assert!(vec.contains(&key1));
    assert!(vec.contains(&key2));  // 相等的键
    println!("      ✅ Vec 正确进行相等性查找");
}

/// 测试边界情况
fn test_edge_cases() {
    println!("\n  📌 边界情况测试:");
    
    // 空值测试
    println!("\n    🔹 空值测试:");
    
    let empty_string1 = String::new();
    let empty_string2 = "";
    assert_eq!(empty_string1, empty_string2);
    println!("      ✅ 空字符串相等性");
    
    let empty_vec1: Vec<i32> = Vec::new();
    let empty_vec2: Vec<i32> = vec![];
    assert_eq!(empty_vec1, empty_vec2);
    println!("      ✅ 空向量相等性");
    
    // 特殊浮点值测试
    println!("\n    🔹 特殊浮点值测试:");
    
    let inf = f64::INFINITY;
    let neg_inf = f64::NEG_INFINITY;
    let nan = f64::NAN;
    
    assert_eq!(inf, inf);
    assert_eq!(neg_inf, neg_inf);
    assert_ne!(inf, neg_inf);
    assert_ne!(nan, nan);  // NaN 不等于自身
    assert_ne!(inf, nan);
    
    println!("      ✅ INFINITY == INFINITY");
    println!("      ✅ NEG_INFINITY == NEG_INFINITY");
    println!("      ✅ INFINITY != NEG_INFINITY");
    println!("      ✅ NaN != NaN (违反自反性)");
    
    // 大数值测试
    println!("\n    🔹 大数值测试:");
    
    let max_int = i64::MAX;
    let min_int = i64::MIN;
    
    assert_eq!(max_int, max_int);
    assert_eq!(min_int, min_int);
    assert_ne!(max_int, min_int);
    
    println!("      ✅ 极值整数相等性测试通过");
    
    // Unicode 字符串测试
    println!("\n    🔹 Unicode 字符串测试:");
    
    let unicode1 = "café";
    let unicode2 = "café";
    let unicode3 = "cafe\u{0301}";  // 使用组合字符
    
    assert_eq!(unicode1, unicode2);
    assert_ne!(unicode1, unicode3);  // 不同的 Unicode 表示
    
    println!("      ✅ 相同 Unicode 字符串相等");
    println!("      ✅ 不同 Unicode 表示不相等 (按字节比较)");
}

/// 运行属性测试
fn run_property_tests() {
    println!("\n  📌 属性测试:");
    
    // 测试多个随机值的等价关系性质
    println!("\n    🔹 随机值属性测试:");
    
    use std::collections::HashSet;
    
    // 生成一些测试数据
    let test_values = vec![1, 2, 3, 1, 2, 4, 5, 3];
    
    // 测试自反性
    for &val in &test_values {
        assert_eq!(val, val);
    }
    println!("      ✅ 所有值满足自反性");
    
    // 测试对称性
    for &a in &test_values {
        for &b in &test_values {
            assert_eq!(a == b, b == a);
        }
    }
    println!("      ✅ 所有值对满足对称性");
    
    // 测试传递性
    for &a in &test_values {
        for &b in &test_values {
            for &c in &test_values {
                if a == b && b == c {
                    assert_eq!(a, c);
                }
            }
        }
    }
    println!("      ✅ 所有值三元组满足传递性");
    
    // 测试 HashSet 的一致性
    let mut set: HashSet<i32> = HashSet::new();
    for val in test_values {
        set.insert(val);
    }
    
    // 验证所有插入的值都能找到
    for &val in set.iter() {
        assert!(set.contains(&val));
    }
    println!("      ✅ HashSet 一致性测试通过");
    
    // 性能基准测试提示
    println!("\n    🔹 性能测试提示:");
    println!("      运行 `cargo bench` 执行性能基准测试");
    println!("      测试不同实现的 PartialEq 和 Hash 性能");
}

/// 测试实用函数
pub fn test_custom_implementations() {
    println!("\n🔸 自定义实现测试:");
    
    // 测试忽略大小写的字符串
    #[derive(Debug, Clone)]
    struct CaseInsensitiveString(String);
    
    impl PartialEq for CaseInsensitiveString {
        fn eq(&self, other: &Self) -> bool {
            self.0.to_lowercase() == other.0.to_lowercase()
        }
    }
    
    impl Eq for CaseInsensitiveString {}
    
    impl Hash for CaseInsensitiveString {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.to_lowercase().hash(state);
        }
    }
    
    let s1 = CaseInsensitiveString("Hello".to_string());
    let s2 = CaseInsensitiveString("HELLO".to_string());
    let s3 = CaseInsensitiveString("hello".to_string());
    
    // 测试相等性
    assert_eq!(s1, s2);
    assert_eq!(s2, s3);
    assert_eq!(s1, s3);
    
    // 测试 hash 一致性
    let mut hasher1 = DefaultHasher::new();
    s1.hash(&mut hasher1);
    let hash1 = hasher1.finish();
    
    let mut hasher2 = DefaultHasher::new();
    s2.hash(&mut hasher2);
    let hash2 = hasher2.finish();
    
    assert_eq!(hash1, hash2);
    
    println!("  ✅ 忽略大小写字符串实现测试通过");
    
    // 测试在 HashMap 中的使用
    let mut map: HashMap<CaseInsensitiveString, i32> = HashMap::new();
    map.insert(s1.clone(), 1);
    map.insert(s2.clone(), 2);  // 应该覆盖
    
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&s3), Some(&2));
    
    println!("  ✅ HashMap 中的行为测试通过");
}

/// 运行所有测试的便捷函数
pub fn run_all_tests() {
    println!("🧪 运行所有 Eq/PartialEq 测试...");
    
    run_comprehensive_tests();
    test_custom_implementations();
    
    println!("\n✅ 所有测试通过！");
    println!("\n📊 测试总结:");
    println!("  - 等价关系性质: 自反性、对称性、传递性");
    println!("  - Hash 一致性: 相等对象有相同 hash 值");
    println!("  - 集合行为: HashMap、HashSet、Vec 的正确行为");
    println!("  - 边界情况: 空值、特殊浮点值、Unicode");
    println!("  - 自定义实现: 正确性和一致性验证");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_equality() {
        assert_eq!(42, 42);
        assert_eq!("hello", "hello");
        assert_eq!(vec![1, 2, 3], vec![1, 2, 3]);
    }
    
    #[test]
    fn test_nan_behavior() {
        let nan = f64::NAN;
        assert_ne!(nan, nan);  // NaN 不等于自身
    }
    
    #[test]
    fn test_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let a = 42;
        let b = 42;
        
        let mut hasher1 = DefaultHasher::new();
        a.hash(&mut hasher1);
        let hash_a = hasher1.finish();
        
        let mut hasher2 = DefaultHasher::new();
        b.hash(&mut hasher2);
        let hash_b = hasher2.finish();
        
        assert_eq!(a, b);
        assert_eq!(hash_a, hash_b);
    }
    
    #[test]
    fn test_hashmap_behavior() {
        let mut map: HashMap<i32, &str> = HashMap::new();
        map.insert(1, "one");
        map.insert(1, "ONE");  // 覆盖
        
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&1), Some(&"ONE"));
    }
    
    #[test]
    fn test_custom_eq_implementation() {
        #[derive(Debug)]
        struct Point { x: i32, y: i32 }
        
        impl PartialEq for Point {
            fn eq(&self, other: &Self) -> bool {
                // 只比较到原点的距离
                let dist1 = self.x * self.x + self.y * self.y;
                let dist2 = other.x * other.x + other.y * other.y;
                dist1 == dist2
            }
        }
        
        let p1 = Point { x: 3, y: 4 };  // 距离 = 25
        let p2 = Point { x: 4, y: 3 };  // 距离 = 25
        let p3 = Point { x: 1, y: 1 };  // 距离 = 2
        
        assert_eq!(p1, p2);  // 相同距离
        assert_ne!(p1, p3);  // 不同距离
    }
}