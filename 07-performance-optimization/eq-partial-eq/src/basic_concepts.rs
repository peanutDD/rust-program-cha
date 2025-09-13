//! # 基础概念模块
//!
//! 本模块演示 Eq 和 PartialEq 的基础概念和核心区别

use std::collections::HashMap;

/// 演示基础概念
pub fn demonstrate_basic_concepts() {
    println!("\n🔸 PartialEq vs Eq 核心区别:");
    
    // PartialEq: 部分等价关系
    println!("\n  📌 PartialEq (部分等价关系):");
    println!("     - 只需要满足对称性和传递性");
    println!("     - 不要求自反性 (x == x 可能为 false)");
    println!("     - 典型例子: 浮点数 (NaN != NaN)");
    
    demonstrate_partialeq_properties();
    
    // Eq: 完全等价关系
    println!("\n  📌 Eq (完全等价关系):");
    println!("     - 必须满足自反性、对称性和传递性");
    println!("     - 要求 x == x 总是为 true");
    println!("     - 是 PartialEq 的子 trait");
    
    demonstrate_eq_properties();
    
    // 实际应用差异
    println!("\n  📌 实际应用差异:");
    demonstrate_practical_differences();
}

/// 演示 PartialEq 的性质
fn demonstrate_partialeq_properties() {
    println!("\n    🔹 PartialEq 示例:");
    
    // 浮点数的 PartialEq 实现
    let a = 1.0;
    let b = 1.0;
    let nan = f64::NAN;
    
    println!("      {} == {} : {}", a, b, a == b);  // true
    println!("      NaN == NaN : {} (违反自反性)", nan == nan);  // false
    println!("      {} == NaN : {}", a, a == nan);  // false
    
    // 对称性
    println!("\n    🔹 对称性验证:");
    println!("      {} == {} : {}", a, b, a == b);
    println!("      {} == {} : {}", b, a, b == a);
    
    // 传递性
    let c = 1.0;
    println!("\n    🔹 传递性验证:");
    println!("      {} == {} : {}", a, b, a == b);
    println!("      {} == {} : {}", b, c, b == c);
    println!("      {} == {} : {}", a, c, a == c);
}

/// 演示 Eq 的性质
fn demonstrate_eq_properties() {
    println!("\n    🔹 Eq 示例:");
    
    // 整数的 Eq 实现
    let x = 42;
    let y = 42;
    let z = 42;
    
    // 自反性
    println!("      自反性: {} == {} : {}", x, x, x == x);
    
    // 对称性
    println!("      对称性: {} == {} : {}, {} == {} : {}", 
             x, y, x == y, y, x, y == x);
    
    // 传递性
    println!("      传递性: {} == {} : {}, {} == {} : {}, {} == {} : {}", 
             x, y, x == y, y, z, y == z, x, z, x == z);
    
    // 字符串也实现了 Eq
    let s1 = "hello";
    let s2 = "hello";
    println!("\n    🔹 字符串 Eq:");
    println!("      \"{}\" == \"{}\" : {}", s1, s2, s1 == s2);
    println!("      自反性: \"{}\" == \"{}\" : {}", s1, s1, s1 == s1);
}

/// 演示实际应用中的差异
fn demonstrate_practical_differences() {
    println!("\n    🔹 HashMap 键的要求:");
    
    // 可以用整数作为键 (实现了 Eq + Hash)
    let mut int_map: HashMap<i32, &str> = HashMap::new();
    int_map.insert(42, "answer");
    println!("      整数键: {:?}", int_map);
    
    // 不能用浮点数作为键 (只实现了 PartialEq)
    println!("      浮点数不能作为 HashMap 键 (只实现 PartialEq)");
    // let mut float_map: HashMap<f64, &str> = HashMap::new(); // 编译错误!
    
    println!("\n    🔹 集合操作的要求:");
    
    // Vec 的 contains 方法只需要 PartialEq
    let numbers = vec![1.0, 2.0, 3.0, f64::NAN];
    println!("      Vec::contains (需要 PartialEq): {}", numbers.contains(&2.0));
    println!("      Vec::contains NaN: {}", numbers.contains(&f64::NAN));
    
    // HashSet 需要 Eq + Hash
    use std::collections::HashSet;
    let mut int_set: HashSet<i32> = HashSet::new();
    int_set.insert(1);
    int_set.insert(2);
    println!("      HashSet (需要 Eq + Hash): {:?}", int_set);
}

/// 自定义类型演示 PartialEq 和 Eq 的区别
#[derive(Debug, Clone)]
pub struct FloatWrapper(pub f64);

// 只实现 PartialEq
impl PartialEq for FloatWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

// 不能实现 Eq，因为包含浮点数
// impl Eq for FloatWrapper {} // 这会违反 Eq 的契约

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntWrapper(pub i32);

/// 演示自定义类型的行为
pub fn demonstrate_custom_wrapper_types() {
    println!("\n🔸 自定义包装类型:");
    
    let float_wrap1 = FloatWrapper(1.0);
    let float_wrap2 = FloatWrapper(1.0);
    let float_wrap_nan = FloatWrapper(f64::NAN);
    
    println!("  FloatWrapper(1.0) == FloatWrapper(1.0): {}", 
             float_wrap1 == float_wrap2);
    println!("  FloatWrapper(NaN) == FloatWrapper(NaN): {}", 
             float_wrap_nan == FloatWrapper(f64::NAN));
    
    let int_wrap1 = IntWrapper(42);
    let int_wrap2 = IntWrapper(42);
    
    println!("  IntWrapper(42) == IntWrapper(42): {}", 
             int_wrap1 == int_wrap2);
    
    // IntWrapper 可以用作 HashMap 键
    let mut wrapper_map: HashMap<IntWrapper, &str> = HashMap::new();
    wrapper_map.insert(int_wrap1, "wrapped answer");
    println!("  IntWrapper 作为 HashMap 键: {:?}", wrapper_map);
}