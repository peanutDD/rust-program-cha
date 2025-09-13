//! # 自定义类型模块
//!
//! 本模块演示如何为自定义类型实现 PartialEq 和 Eq

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
// use std::cmp::Ordering; // 暂时不需要

/// 演示自定义类型的实现
pub fn demonstrate_custom_types() {
    println!("\n🔸 自定义类型实现:");
    
    simple_struct_examples();
    enum_examples();
    complex_struct_examples();
    generic_type_examples();
    special_case_examples();
}

/// 简单结构体示例
fn simple_struct_examples() {
    println!("\n  📌 简单结构体示例:");
    
    // 使用 derive 的简单结构体
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = Point { x: 2, y: 1 };
    
    println!("\n    🔹 derive 自动实现:");
    println!("      {:?} == {:?} : {}", p1, p2, p1 == p2);
    println!("      {:?} == {:?} : {}", p1, p3, p1 == p3);
    
    // 可以用作 HashMap 键
    let mut point_map: HashMap<Point, &str> = HashMap::new();
    point_map.insert(p1.clone(), "origin area");
    point_map.insert(p3.clone(), "different area");
    println!("      作为 HashMap 键: {:?}", point_map);
    
    // 手动实现的结构体 - 只比较距离
    #[derive(Debug, Clone)]
    struct DistancePoint {
        x: i32,
        y: i32,
        label: String,  // 不参与比较
    }
    
    impl PartialEq for DistancePoint {
        fn eq(&self, other: &Self) -> bool {
            // 只比较到原点的距离
            let dist1 = self.x * self.x + self.y * self.y;
            let dist2 = other.x * other.x + other.y * other.y;
            dist1 == dist2
        }
    }
    
    impl Eq for DistancePoint {}
    
    impl Hash for DistancePoint {
        fn hash<H: Hasher>(&self, state: &mut H) {
            let distance = self.x * self.x + self.y * self.y;
            distance.hash(state);
        }
    }
    
    let dp1 = DistancePoint { x: 3, y: 4, label: "A".to_string() };
    let dp2 = DistancePoint { x: 4, y: 3, label: "B".to_string() };
    let dp3 = DistancePoint { x: 1, y: 1, label: "C".to_string() };
    
    println!("\n    🔹 基于距离的比较:");
    println!("      {:?} == {:?} : {} (相同距离)", dp1, dp2, dp1 == dp2);
    println!("      {:?} == {:?} : {} (不同距离)", dp1, dp3, dp1 == dp3);
    
    let mut distance_map: HashMap<DistancePoint, Vec<String>> = HashMap::new();
    distance_map.entry(dp1.clone()).or_insert_with(Vec::new).push(dp1.label.clone());
    distance_map.entry(dp2.clone()).or_insert_with(Vec::new).push(dp2.label.clone());
    println!("      按距离分组: {:?}", distance_map);
}

/// 枚举示例
fn enum_examples() {
    println!("\n  📌 枚举示例:");
    
    // 简单枚举
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    enum Color {
        Red,
        Green,
        Blue,
        RGB(u8, u8, u8),
    }
    
    let c1 = Color::Red;
    let c2 = Color::Red;
    let c3 = Color::Blue;
    let c4 = Color::RGB(255, 0, 0);
    let c5 = Color::RGB(255, 0, 0);
    
    println!("\n    🔹 枚举比较:");
    println!("      {:?} == {:?} : {}", c1, c2, c1 == c2);
    println!("      {:?} == {:?} : {}", c1, c3, c1 == c3);
    println!("      {:?} == {:?} : {}", c4, c5, c4 == c5);
    println!("      {:?} == {:?} : {}", c1, c4, c1 == c4);
    
    // 自定义枚举比较 - 忽略 Alpha 通道
    #[derive(Debug, Clone)]
    enum ColorWithAlpha {
        Named(String),
        RGBA(u8, u8, u8, u8),
    }
    
    impl PartialEq for ColorWithAlpha {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (ColorWithAlpha::Named(n1), ColorWithAlpha::Named(n2)) => n1 == n2,
                (ColorWithAlpha::RGBA(r1, g1, b1, _), ColorWithAlpha::RGBA(r2, g2, b2, _)) => {
                    r1 == r2 && g1 == g2 && b1 == b2  // 忽略 alpha
                }
                _ => false,
            }
        }
    }
    
    impl Eq for ColorWithAlpha {}
    
    let ca1 = ColorWithAlpha::RGBA(255, 0, 0, 128);
    let ca2 = ColorWithAlpha::RGBA(255, 0, 0, 255);  // 不同的 alpha
    let ca3 = ColorWithAlpha::RGBA(0, 255, 0, 128);
    
    println!("\n    🔹 忽略 Alpha 通道的比较:");
    println!("      {:?} == {:?} : {} (忽略 alpha)", ca1, ca2, ca1 == ca2);
    println!("      {:?} == {:?} : {}", ca1, ca3, ca1 == ca3);
}

/// 复杂结构体示例
fn complex_struct_examples() {
    println!("\n  📌 复杂结构体示例:");
    
    // 包含浮点数的结构体 - 特殊处理
    #[derive(Debug, Clone)]
    struct Measurement {
        value: f64,
        unit: String,
        precision: u32,
    }
    
    impl PartialEq for Measurement {
        fn eq(&self, other: &Self) -> bool {
            // 根据精度比较浮点数
            if self.unit != other.unit {
                return false;
            }
            
            let precision = self.precision.min(other.precision);
            let factor = 10_f64.powi(precision as i32);
            
            let rounded_self = (self.value * factor).round() / factor;
            let rounded_other = (other.value * factor).round() / factor;
            
            rounded_self == rounded_other
        }
    }
    
    // 注意: 不实现 Eq，因为浮点数比较
    
    let m1 = Measurement { value: 1.234, unit: "m".to_string(), precision: 2 };
    let m2 = Measurement { value: 1.235, unit: "m".to_string(), precision: 2 };
    let m3 = Measurement { value: 1.24, unit: "m".to_string(), precision: 2 };
    
    println!("\n    🔹 基于精度的浮点数比较:");
    println!("      {:?} == {:?} : {} (精度内相等)", m1, m2, m1 == m2);
    println!("      {:?} == {:?} : {} (精度内相等)", m1, m3, m1 == m3);
    
    // 版本号结构体
    #[derive(Debug, Clone)]
    struct Version {
        major: u32,
        minor: u32,
        patch: u32,
        pre_release: Option<String>,
    }
    
    impl PartialEq for Version {
        fn eq(&self, other: &Self) -> bool {
            self.major == other.major
                && self.minor == other.minor
                && self.patch == other.patch
                && self.pre_release == other.pre_release
        }
    }
    
    impl Eq for Version {}
    
    impl Hash for Version {
        fn hash<H: Hasher>(&self, state: &mut H) {
            (self.major, self.minor, self.patch, &self.pre_release).hash(state);
        }
    }
    
    let v1 = Version { major: 1, minor: 2, patch: 3, pre_release: None };
    let v2 = Version { major: 1, minor: 2, patch: 3, pre_release: None };
    let v3 = Version { major: 1, minor: 2, patch: 3, pre_release: Some("alpha".to_string()) };
    
    println!("\n    🔹 版本号比较:");
    println!("      {:?} == {:?} : {}", v1, v2, v1 == v2);
    println!("      {:?} == {:?} : {}", v1, v3, v1 == v3);
}

/// 泛型类型示例
fn generic_type_examples() {
    println!("\n  📌 泛型类型示例:");
    
    // 泛型容器
    #[derive(Debug)]
    struct Container<T> {
        data: T,
        metadata: String,
    }
    
    impl<T: PartialEq> PartialEq for Container<T> {
        fn eq(&self, other: &Self) -> bool {
            self.data == other.data
            // 忽略 metadata
        }
    }
    
    impl<T: Eq> Eq for Container<T> {}
    
    impl<T: Hash> Hash for Container<T> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.data.hash(state);
        }
    }
    
    let c1 = Container { data: 42, metadata: "first".to_string() };
    let c2 = Container { data: 42, metadata: "second".to_string() };
    let c3 = Container { data: 24, metadata: "third".to_string() };
    
    println!("\n    🔹 泛型容器比较:");
    println!("      {:?} == {:?} : {} (忽略元数据)", c1, c2, c1 == c2);
    println!("      {:?} == {:?} : {}", c1, c3, c1 == c3);
    
    // 可以用不同类型实例化
    let cs1 = Container { data: "hello".to_string(), metadata: "string".to_string() };
    let cs2 = Container { data: "hello".to_string(), metadata: "another".to_string() };
    
    println!("      字符串容器: {:?} == {:?} : {}", cs1, cs2, cs1 == cs2);
    
    // 多类型参数的泛型
    #[derive(Debug)]
    struct Pair<T, U> {
        first: T,
        second: U,
    }
    
    impl<T: PartialEq, U: PartialEq> PartialEq for Pair<T, U> {
        fn eq(&self, other: &Self) -> bool {
            self.first == other.first && self.second == other.second
        }
    }
    
    impl<T: Eq, U: Eq> Eq for Pair<T, U> {}
    
    let pair1 = Pair { first: 1, second: "hello" };
    let pair2 = Pair { first: 1, second: "hello" };
    let pair3 = Pair { first: 1, second: "world" };
    
    println!("\n    🔹 多类型参数泛型:");
    println!("      {:?} == {:?} : {}", pair1, pair2, pair1 == pair2);
    println!("      {:?} == {:?} : {}", pair1, pair3, pair1 == pair3);
}

/// 特殊情况示例
fn special_case_examples() {
    println!("\n  📌 特殊情况示例:");
    
    // 循环引用安全的结构体
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;
    
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: Option<Weak<RefCell<Node>>>,
        children: Vec<Rc<RefCell<Node>>>,
    }
    
    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            // 只比较值，避免循环引用问题
            self.value == other.value
        }
    }
    
    impl Eq for Node {}
    
    let node1 = Rc::new(RefCell::new(Node {
        value: 42,
        parent: None,
        children: Vec::new(),
    }));
    
    let node2 = Rc::new(RefCell::new(Node {
        value: 42,
        parent: None,
        children: Vec::new(),
    }));
    
    println!("\n    🔹 避免循环引用的比较:");
    println!("      node1 == node2 : {} (只比较值)", 
             node1.borrow().eq(&*node2.borrow()));
    
    // 带有函数指针的结构体
    #[derive(Debug)]
    struct Calculator {
        name: String,
        operation: fn(i32, i32) -> i32,
    }
    
    impl PartialEq for Calculator {
        fn eq(&self, other: &Self) -> bool {
            // 只比较名称，函数指针比较复杂
            self.name == other.name
        }
    }
    
    impl Eq for Calculator {}
    
    fn add(a: i32, b: i32) -> i32 { a + b }
    fn multiply(a: i32, b: i32) -> i32 { a * b }
    
    let calc1 = Calculator { name: "adder".to_string(), operation: add };
    let calc2 = Calculator { name: "adder".to_string(), operation: multiply };
    
    println!("\n    🔹 包含函数指针的比较:");
    println!("      {:?} == {:?} : {} (只比较名称)", calc1, calc2, calc1 == calc2);
}

/// 演示实现陷阱
pub fn demonstrate_implementation_pitfalls() {
    println!("\n🔸 实现陷阱演示:");
    
    println!("\n  📌 常见错误:");
    
    // 错误示例 1: Hash 和 Eq 不一致
    println!("\n    ❌ Hash 和 Eq 不一致的后果:");
    
    #[derive(Debug, Clone)]
    struct BadExample {
        a: i32,
        b: i32,
    }
    
    impl PartialEq for BadExample {
        fn eq(&self, other: &Self) -> bool {
            self.a == other.a  // 只比较 a
        }
    }
    
    impl Eq for BadExample {}
    
    // 错误的 Hash 实现
    impl Hash for BadExample {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.a.hash(state);
            self.b.hash(state);  // 包含 b，但 eq 不比较 b！
        }
    }
    
    let bad1 = BadExample { a: 1, b: 2 };
    let bad2 = BadExample { a: 1, b: 3 };
    
    println!("      {:?} == {:?} : {} (相等)", bad1, bad2, bad1 == bad2);
    
    let mut bad_map: HashMap<BadExample, &str> = HashMap::new();
    bad_map.insert(bad1.clone(), "first");
    bad_map.insert(bad2.clone(), "second");
    
    println!("      HashMap 行为异常: {:?}", bad_map);
    println!("      可能导致查找失败或重复键");
    
    println!("\n  📌 正确的实现原则:");
    println!("    ✅ 如果 a == b，那么 hash(a) == hash(b)");
    println!("    ✅ 等价关系必须满足自反性、对称性、传递性");
    println!("    ✅ 浮点数需要特殊处理 NaN");
    println!("    ✅ 文档化比较逻辑和特殊情况");
}