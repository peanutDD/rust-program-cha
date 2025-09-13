//! # 实现示例模块
//!
//! 本模块展示各种 PartialEq 和 Eq 的实现示例

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// 展示实现示例
pub fn show_implementation_examples() {
    println!("\n🔸 实现示例演示:");
    
    basic_implementations();
    custom_implementations();
    complex_implementations();
    generic_implementations();
}

/// 基础实现示例
fn basic_implementations() {
    println!("\n  📌 基础实现示例:");
    
    // 简单结构体
   #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Person {
        id: u32,
        name: String,
        age: u32,
    }
    let person1 = Person {
        id: 1,
        name: "Alice".to_string(),
        age: 30,
    };
    let person3 = Person {
        id: 3,
        name: "Alice".to_string(),
        age: 30,
    };
    let person2 = Person {
        id: 2,
        name: "Bob".to_string(),
        age: 25,
    };
    
    println!("\n    🔹 derive 自动实现:");
    println!("      {:?} == {:?} : {}", person1, person2, person1 == person2);
    println!("      {:?} == {:?} : {}", person1, person3, person1 == person3);
    
    // 可以用作 HashMap 键
    let mut people: HashMap<Person, String> = HashMap::new();
    people.insert(person1.clone(), "Engineer".to_string());
    people.insert(person3.clone(), "Designer".to_string());
    println!("      作为 HashMap 键: {:?}", people);
}

/// 自定义实现示例
fn custom_implementations() {
    println!("\n  📌 自定义实现示例:");
    
    // 忽略大小写的字符串比较
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
    
    println!("\n    🔹 忽略大小写比较:");
    println!("      {:?} == {:?} : {}", s1, s2, s1 == s2);
    println!("      {:?} == {:?} : {}", s1, s3, s1 == s3);
    
    let mut case_map: HashMap<CaseInsensitiveString, i32> = HashMap::new();
    case_map.insert(s1, 1);
    case_map.insert(s2, 2);  // 应该覆盖第一个
    println!("      HashMap 行为: {:?}", case_map);
    
    // 只比较部分字段
    #[derive(Debug)]
    struct Employee {
        id: u32,
        name: String,
        salary: f64,  // 不参与比较
    }
    
    impl PartialEq for Employee {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id && self.name == other.name
            // 故意忽略 salary
        }
    }
    
    impl Eq for Employee {}
    
    impl Hash for Employee {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
            self.name.hash(state);
            // 不包含 salary
        }
    }
    
    let emp1 = Employee {
        id: 1,
        name: "John".to_string(),
        salary: 50000.0,
    };
    let emp2 = Employee {
        id: 1,
        name: "John".to_string(),
        salary: 60000.0,  // 不同的薪水
    };
    
    println!("\n    🔹 部分字段比较:");
    println!("      {:?} == {:?} : {} (忽略薪水)", emp1, emp2, emp1 == emp2);
}

/// 复杂实现示例
fn complex_implementations() {
    println!("\n  📌 复杂实现示例:");
    
    // 浮点数包装器 - 处理 NaN
    #[derive(Debug, Clone)]
    struct OrderedFloat(f64);
    
    impl PartialEq for OrderedFloat {
        fn eq(&self, other: &Self) -> bool {
            // 特殊处理 NaN
            if self.0.is_nan() && other.0.is_nan() {
                true  // 认为 NaN == NaN
            } else {
                self.0 == other.0
            }
        }
    }
    
    impl Eq for OrderedFloat {}  // 现在可以实现 Eq 了
    
    impl Hash for OrderedFloat {
        fn hash<H: Hasher>(&self, state: &mut H) {
            if self.0.is_nan() {
                // 所有 NaN 使用相同的哈希值
                0u64.hash(state);
            } else {
                self.0.to_bits().hash(state);
            }
        }
    }
    
    let f1 = OrderedFloat(1.0);
    let f2 = OrderedFloat(1.0);
    let nan1 = OrderedFloat(f64::NAN);
    let nan2 = OrderedFloat(f64::NAN);
    
    println!("\n    🔹 有序浮点数:");
    println!("      OrderedFloat(1.0) == OrderedFloat(1.0) : {}", f1 == f2);
    println!("      OrderedFloat(NaN) == OrderedFloat(NaN) : {}", nan1 == nan2);
    
    let mut float_map: HashMap<OrderedFloat, &str> = HashMap::new();
    float_map.insert(f1, "one");
    float_map.insert(nan1, "not a number");
    println!("      作为 HashMap 键: {:?}", float_map);
    
    // 版本号比较
    #[derive(Debug, Clone)]
    struct Version {
        major: u32,
        minor: u32,
        patch: u32,
    }
    
    impl PartialEq for Version {
        fn eq(&self, other: &Self) -> bool {
            self.major == other.major 
                && self.minor == other.minor 
                && self.patch == other.patch
        }
    }
    
    impl Eq for Version {}
    
    impl Hash for Version {
        fn hash<H: Hasher>(&self, state: &mut H) {
            (self.major, self.minor, self.patch).hash(state);
        }
    }
    
    let v1 = Version { major: 1, minor: 2, patch: 3 };
    let v2 = Version { major: 1, minor: 2, patch: 3 };
    let v3 = Version { major: 1, minor: 2, patch: 4 };
    
    println!("\n    🔹 版本号比较:");
    println!("      {:?} == {:?} : {}", v1, v2, v1 == v2);
    println!("      {:?} == {:?} : {}", v1, v3, v1 == v3);
}

/// 泛型实现示例
fn generic_implementations() {
    println!("\n  📌 泛型实现示例:");
    
    // 泛型包装器
    #[derive(Debug)]
    struct Wrapper<T> {
        value: T,
        metadata: String,
    }
    
    impl<T: PartialEq> PartialEq for Wrapper<T> {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
            // 忽略 metadata
        }
    }
    
    impl<T: Eq> Eq for Wrapper<T> {}
    
    impl<T: Hash> Hash for Wrapper<T> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.value.hash(state);
            // 不包含 metadata
        }
    }
    
    let w1 = Wrapper {
        value: 42,
        metadata: "first".to_string(),
    };
    let w2 = Wrapper {
        value: 42,
        metadata: "second".to_string(),
    };
    let w3 = Wrapper {
        value: 24,
        metadata: "third".to_string(),
    };
    
    println!("\n    🔹 泛型包装器:");
    println!("      {:?} == {:?} : {} (忽略元数据)", w1, w2, w1 == w2);
    println!("      {:?} == {:?} : {}", w1, w3, w1 == w3);
    
    // 不同类型之间的比较
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
    
    println!("\n    🔹 跨类型比较:");
    println!("      {:?} == {:?} : {}", int_val, float_val, int_val == float_val);
    println!("      {:?} == {:?} : {}", float_val, int_val, float_val == int_val);
}

/// 演示实现陷阱和最佳实践
pub fn demonstrate_implementation_pitfalls() {
    println!("\n🔸 实现陷阱和最佳实践:");
    
    println!("\n  📌 常见陷阱:");
    println!("    ❌ Hash 和 Eq 不一致");
    println!("    ❌ 违反等价关系性质");
    println!("    ❌ 浮点数直接实现 Eq");
    println!("    ❌ 忘记实现对称性");
    
    println!("\n  📌 最佳实践:");
    println!("    ✅ 确保 Hash 和 Eq 一致性");
    println!("    ✅ 测试等价关系性质");
    println!("    ✅ 谨慎处理浮点数");
    println!("    ✅ 文档化比较逻辑");
    
    // 展示错误的实现（注释掉）
    println!("\n  📌 错误实现示例 (已注释):");
    println!("    /*");
    println!("    // 错误: Hash 和 Eq 不一致");
    println!("    impl PartialEq for BadExample {{");
    println!("        fn eq(&self, other: &Self) -> bool {{");
    println!("            self.a == other.a  // 只比较 a");
    println!("        }}");
    println!("    }}");
    println!("    impl Hash for BadExample {{");
    println!("        fn hash<H: Hasher>(&self, state: &mut H) {{");
    println!("            self.a.hash(state);");
    println!("            self.b.hash(state);  // 包含 b，不一致!");
    println!("        }}");
    println!("    }}");
    println!("    */");
}