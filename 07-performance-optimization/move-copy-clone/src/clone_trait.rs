//! # Clone Trait 深度解析
//!
//! Clone trait 是 Rust 中用于显式复制的重要机制，它允许类型定义自己的复制行为。
//! 本模块将深入解析 Clone trait 的工作原理、实现模式和最佳实践。

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::sync::Mutex;

/// Clone trait 的基本概念演示
pub fn basic_clone_concepts() {
    println!("=== Clone Trait 基本概念 ===");
    
    // 1. 基本类型的 Clone 行为
    println!("\n1. 基本类型的 Clone 行为:");
    let x = 42;
    let y = x.clone(); // 对于 Copy 类型，clone() 等同于 copy
    println!("x: {}, y: {}", x, y);
    
    // 2. String 的 Clone
    println!("\n2. String 的 Clone:");
    let s1 = String::from("Hello, World!");
    let s2 = s1.clone(); // 深拷贝，创建新的堆内存
    println!("s1: {}, s2: {}", s1, s2);
    println!("s1 地址: {:p}, s2 地址: {:p}", s1.as_ptr(), s2.as_ptr());
    
    // 3. Vec 的 Clone
    println!("\n3. Vec 的 Clone:");
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = v1.clone(); // 深拷贝，包括所有元素
    println!("v1: {:?}, v2: {:?}", v1, v2);
    println!("v1 地址: {:p}, v2 地址: {:p}", v1.as_ptr(), v2.as_ptr());
    
    // 4. HashMap 的 Clone
    println!("\n4. HashMap 的 Clone:");
    let mut map1 = HashMap::new();
    map1.insert("key1", "value1");
    map1.insert("key2", "value2");
    
    let map2 = map1.clone(); // 深拷贝整个哈希表
    println!("map1: {:?}", map1);
    println!("map2: {:?}", map2);
}

/// Clone trait 的实现模式分析
pub fn clone_implementation_patterns() {
    println!("\n=== Clone Trait 实现模式分析 ===");
    
    // 1. 自动派生 Clone
    println!("\n1. 自动派生 Clone:");
    demonstrate_derived_clone();
    
    // 2. 手动实现 Clone
    println!("\n2. 手动实现 Clone:");
    demonstrate_manual_clone();
    
    // 3. 条件 Clone 实现
    println!("\n3. 条件 Clone 实现:");
    demonstrate_conditional_clone();
    
    // 4. 复杂类型的 Clone
    println!("\n4. 复杂类型的 Clone:");
    demonstrate_complex_clone();
}

/// 演示自动派生的 Clone
fn demonstrate_derived_clone() {
    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        age: u32,
        email: String,
    }
    
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
        email: String::from("alice@example.com"),
    };
    
    let person2 = person1.clone();
    println!("person1: {:?}", person1);
    println!("person2: {:?}", person2);
    
    // 验证深拷贝
    println!("name 地址不同: {}", 
             person1.name.as_ptr() != person2.name.as_ptr());
    
    #[derive(Debug, Clone)]
    struct Team {
        name: String,
        members: Vec<Person>,
        metadata: HashMap<String, String>,
    }
    
    let mut team1 = Team {
        name: String::from("开发团队"),
        members: vec![person1],
        metadata: HashMap::new(),
    };
    team1.metadata.insert("department".to_string(), "Engineering".to_string());
    
    let team2 = team1.clone();
    println!("\nteam1: {:?}", team1);
    println!("team2: {:?}", team2);
    println!("嵌套结构也被深拷贝");
}

/// 演示手动实现的 Clone
fn demonstrate_manual_clone() {
    #[derive(Debug)]
    struct CustomStruct {
        id: u64,
        data: Vec<i32>,
        metadata: String,
    }
    
    impl Clone for CustomStruct {
        fn clone(&self) -> Self {
            println!("正在克隆 CustomStruct (id: {})", self.id);
            
            CustomStruct {
                id: self.id,
                data: self.data.clone(), // 深拷贝 Vec
                metadata: format!("{}_cloned", self.metadata), // 自定义克隆逻辑
            }
        }
    }
    
    let original = CustomStruct {
        id: 123,
        data: vec![1, 2, 3, 4, 5],
        metadata: String::from("original"),
    };
    
    let cloned = original.clone();
    println!("original: {:?}", original);
    println!("cloned: {:?}", cloned);
    
    // 演示自定义 Clone 行为
    #[derive(Debug)]
    struct Counter {
        value: u32,
        clone_count: RefCell<u32>,
    }
    
    impl Clone for Counter {
        fn clone(&self) -> Self {
            // 增加克隆计数
            *self.clone_count.borrow_mut() += 1;
            
            Counter {
                value: self.value,
                clone_count: RefCell::new(0), // 新实例的克隆计数从0开始
            }
        }
    }
    
    let counter1 = Counter {
        value: 42,
        clone_count: RefCell::new(0),
    };
    
    let counter2 = counter1.clone();
    let counter3 = counter1.clone();
    
    println!("\ncounter1 被克隆了 {} 次", counter1.clone_count.borrow());
    println!("counter2: {:?}", counter2);
    println!("counter3: {:?}", counter3);
}

/// 演示条件 Clone 实现
fn demonstrate_conditional_clone() {
    // 只有当 T 实现 Clone 时，Container<T> 才实现 Clone
    #[derive(Debug)]
    struct Container<T> {
        items: Vec<T>,
        name: String,
    }
    
    impl<T: Clone> Clone for Container<T> {
        fn clone(&self) -> Self {
            println!("克隆容器: {}", self.name);
            Container {
                items: self.items.clone(), // 需要 T: Clone
                name: format!("{}_copy", self.name),
            }
        }
    }
    
    // Clone 类型的容器
    let container1 = Container {
        items: vec![1, 2, 3, 4, 5],
        name: String::from("numbers"),
    };
    
    let container2 = container1.clone();
    println!("container1: {:?}", container1);
    println!("container2: {:?}", container2);
    
    // 包含 String 的容器
    let container3 = Container {
        items: vec![String::from("hello"), String::from("world")],
        name: String::from("strings"),
    };
    
    let container4 = container3.clone();
    println!("\ncontainer3: {:?}", container3);
    println!("container4: {:?}", container4);
}

/// 演示复杂类型的 Clone
fn demonstrate_complex_clone() {
    // 包含智能指针的结构体
    #[derive(Debug)]
    struct SharedData {
        rc_data: Rc<Vec<i32>>,
        arc_data: Arc<String>,
        unique_data: Box<i32>,
    }
    
    impl Clone for SharedData {
        fn clone(&self) -> Self {
            SharedData {
                rc_data: Rc::clone(&self.rc_data), // 增加引用计数
                arc_data: Arc::clone(&self.arc_data), // 增加引用计数
                unique_data: self.unique_data.clone(), // 深拷贝 Box 内容
            }
        }
    }
    
    let shared1 = SharedData {
        rc_data: Rc::new(vec![1, 2, 3]),
        arc_data: Arc::new(String::from("shared string")),
        unique_data: Box::new(42),
    };
    
    println!("克隆前 Rc 引用计数: {}", Rc::strong_count(&shared1.rc_data));
    println!("克隆前 Arc 引用计数: {}", Arc::strong_count(&shared1.arc_data));
    
    let shared2 = shared1.clone();
    
    println!("克隆后 Rc 引用计数: {}", Rc::strong_count(&shared1.rc_data));
    println!("克隆后 Arc 引用计数: {}", Arc::strong_count(&shared1.arc_data));
    
    println!("shared1.unique_data 地址: {:p}", &*shared1.unique_data);
    println!("shared2.unique_data 地址: {:p}", &*shared2.unique_data);
    println!("Box 内容被深拷贝到不同地址");
    
    // 验证 Rc 和 Arc 指向相同数据
    println!("Rc 数据地址相同: {}", 
             Rc::as_ptr(&shared1.rc_data) == Rc::as_ptr(&shared2.rc_data));
    println!("Arc 数据地址相同: {}", 
             Arc::as_ptr(&shared1.arc_data) == Arc::as_ptr(&shared2.arc_data));
}

/// Clone trait 的深拷贝 vs 浅拷贝分析
pub fn deep_vs_shallow_clone_analysis() {
    println!("\n=== 深拷贝 vs 浅拷贝分析 ===");
    
    // 1. 深拷贝示例
    println!("\n1. 深拷贝示例:");
    demonstrate_deep_clone();
    
    // 2. 浅拷贝示例（使用智能指针）
    println!("\n2. 浅拷贝示例:");
    demonstrate_shallow_clone();
    
    // 3. 混合拷贝示例
    println!("\n3. 混合拷贝示例:");
    demonstrate_mixed_clone();
    
    // 4. 自定义拷贝策略
    println!("\n4. 自定义拷贝策略:");
    demonstrate_custom_clone_strategy();
}

/// 演示深拷贝
fn demonstrate_deep_clone() {
    #[derive(Debug, Clone)]
    struct DeepCloneStruct {
        data: Vec<String>,
        nested: Box<Vec<i32>>,
    }
    
    let original = DeepCloneStruct {
        data: vec![String::from("hello"), String::from("world")],
        nested: Box::new(vec![1, 2, 3, 4, 5]),
    };
    
    let cloned = original.clone();
    
    println!("原始数据: {:?}", original);
    println!("克隆数据: {:?}", cloned);
    
    // 验证深拷贝
    println!("\n深拷贝验证:");
    println!("Vec 地址不同: {}", 
             original.data.as_ptr() != cloned.data.as_ptr());
    println!("Box 地址不同: {}", 
             &*original.nested as *const _ != &*cloned.nested as *const _);
    println!("Box 内 Vec 地址不同: {}", 
             original.nested.as_ptr() != cloned.nested.as_ptr());
    
    // 修改克隆不影响原始数据
    let mut cloned_mut = cloned;
    cloned_mut.data.push(String::from("modified"));
    cloned_mut.nested.push(999);
    
    println!("\n修改后:");
    println!("原始数据: {:?}", original);
    println!("修改的克隆: {:?}", cloned_mut);
    println!("原始数据未受影响");
}

/// 演示浅拷贝（使用智能指针）
fn demonstrate_shallow_clone() {
    #[derive(Debug)]
    struct ShallowCloneStruct {
        shared_data: Rc<RefCell<Vec<String>>>,
        thread_safe_data: Arc<Mutex<i32>>,
        id: u32,
    }
    
    impl Clone for ShallowCloneStruct {
        fn clone(&self) -> Self {
            ShallowCloneStruct {
                shared_data: Rc::clone(&self.shared_data), // 浅拷贝：共享数据
                thread_safe_data: Arc::clone(&self.thread_safe_data), // 浅拷贝：共享数据
                id: self.id + 1, // 深拷贝：独立的 ID
            }
        }
    }
    
    let original = ShallowCloneStruct {
        shared_data: Rc::new(RefCell::new(vec![
            String::from("shared1"),
            String::from("shared2"),
        ])),
        thread_safe_data: Arc::new(Mutex::new(100)),
        id: 1,
    };
    
    let cloned = original.clone();
    
    println!("原始结构: id={}, 共享数据引用计数={}", 
             original.id, Rc::strong_count(&original.shared_data));
    println!("克隆结构: id={}, 共享数据引用计数={}", 
             cloned.id, Rc::strong_count(&cloned.shared_data));
    
    // 修改共享数据
    original.shared_data.borrow_mut().push(String::from("added by original"));
    *original.thread_safe_data.lock().unwrap() = 200;
    
    println!("\n修改共享数据后:");
    println!("原始结构看到的数据: {:?}", original.shared_data.borrow());
    println!("克隆结构看到的数据: {:?}", cloned.shared_data.borrow());
    println!("原始结构的线程安全数据: {}", *original.thread_safe_data.lock().unwrap());
    println!("克隆结构的线程安全数据: {}", *cloned.thread_safe_data.lock().unwrap());
    println!("共享数据被两个结构同时看到");
}

/// 演示混合拷贝策略
fn demonstrate_mixed_clone() {
    #[derive(Debug)]
    struct MixedCloneStruct {
        deep_data: Vec<String>,      // 深拷贝
        shallow_data: Rc<Vec<i32>>,  // 浅拷贝
        copy_data: u32,              // 按位复制
    }
    
    impl Clone for MixedCloneStruct {
        fn clone(&self) -> Self {
            println!("执行混合克隆策略");
            MixedCloneStruct {
                deep_data: self.deep_data.clone(),      // 深拷贝
                shallow_data: Rc::clone(&self.shallow_data), // 浅拷贝
                copy_data: self.copy_data,              // 按位复制
            }
        }
    }
    
    let original = MixedCloneStruct {
        deep_data: vec![String::from("deep1"), String::from("deep2")],
        shallow_data: Rc::new(vec![1, 2, 3, 4, 5]),
        copy_data: 42,
    };
    
    let cloned = original.clone();
    
    println!("\n混合拷贝验证:");
    println!("深拷贝数据地址不同: {}", 
             original.deep_data.as_ptr() != cloned.deep_data.as_ptr());
    println!("浅拷贝数据地址相同: {}", 
             Rc::as_ptr(&original.shallow_data) == Rc::as_ptr(&cloned.shallow_data));
    println!("按位复制数据值相同: {}", 
             original.copy_data == cloned.copy_data);
    
    println!("Rc 引用计数: {}", Rc::strong_count(&original.shallow_data));
}

/// 演示自定义拷贝策略
fn demonstrate_custom_clone_strategy() {
    #[derive(Debug)]
    struct CustomCloneStrategy {
        data: Vec<i32>,
        clone_mode: CloneMode,
    }
    
    #[derive(Debug, Clone, Copy)]
    enum CloneMode {
        Full,      // 完整克隆
        Partial,   // 部分克隆
        Reference, // 引用克隆
    }
    
    impl Clone for CustomCloneStrategy {
        fn clone(&self) -> Self {
            let cloned_data = match self.clone_mode {
                CloneMode::Full => {
                    println!("执行完整克隆");
                    self.data.clone()
                }
                CloneMode::Partial => {
                    println!("执行部分克隆（只克隆前一半）");
                    let mid = self.data.len() / 2;
                    self.data[..mid].to_vec()
                }
                CloneMode::Reference => {
                    println!("执行引用克隆（创建空向量）");
                    Vec::new()
                }
            };
            
            CustomCloneStrategy {
                data: cloned_data,
                clone_mode: self.clone_mode,
            }
        }
    }
    
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 完整克隆
    let full_original = CustomCloneStrategy {
        data: data.clone(),
        clone_mode: CloneMode::Full,
    };
    let full_cloned = full_original.clone();
    println!("完整克隆: {:?}", full_cloned);
    
    // 部分克隆
    let partial_original = CustomCloneStrategy {
        data: data.clone(),
        clone_mode: CloneMode::Partial,
    };
    let partial_cloned = partial_original.clone();
    println!("部分克隆: {:?}", partial_cloned);
    
    // 引用克隆
    let reference_original = CustomCloneStrategy {
        data: data.clone(),
        clone_mode: CloneMode::Reference,
    };
    let reference_cloned = reference_original.clone();
    println!("引用克隆: {:?}", reference_cloned);
}

/// Clone trait 的性能分析
pub fn performance_analysis() {
    println!("\n=== Clone Trait 性能分析 ===");
    
    // 1. 不同数据结构的 Clone 性能
    println!("\n1. 不同数据结构的 Clone 性能:");
    benchmark_clone_performance();
    
    // 2. 深度嵌套结构的 Clone 性能
    println!("\n2. 深度嵌套结构的 Clone 性能:");
    benchmark_nested_clone();
    
    // 3. 智能指针的 Clone 性能
    println!("\n3. 智能指针的 Clone 性能:");
    benchmark_smart_pointer_clone();
    
    // 4. Clone vs 手动复制性能对比
    println!("\n4. Clone vs 手动复制性能对比:");
    compare_clone_vs_manual_copy();
}

/// 基准测试不同数据结构的 Clone 性能
fn benchmark_clone_performance() {
    use std::time::Instant;
    
    // 测试数据
    let string_data = "Hello, World!".repeat(1000);
    let vec_data: Vec<i32> = (0..10000).collect();
    let mut map_data = HashMap::new();
    for i in 0..1000 {
        map_data.insert(format!("key_{}", i), i);
    }
    
    const ITERATIONS: usize = 1000;
    
    // String Clone 性能
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _cloned = string_data.clone();
    }
    let string_duration = start.elapsed();
    
    // Vec Clone 性能
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _cloned = vec_data.clone();
    }
    let vec_duration = start.elapsed();
    
    // HashMap Clone 性能
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _cloned = map_data.clone();
    }
    let map_duration = start.elapsed();
    
    println!("String ({} chars) Clone {} 次: {:?}", 
             string_data.len(), ITERATIONS, string_duration);
    println!("Vec<i32> ({} elements) Clone {} 次: {:?}", 
             vec_data.len(), ITERATIONS, vec_duration);
    println!("HashMap ({} entries) Clone {} 次: {:?}", 
             map_data.len(), ITERATIONS, map_duration);
    
    // 计算每次操作的平均时间
    println!("\n平均每次 Clone 时间:");
    println!("String: {:?}", string_duration / ITERATIONS as u32);
    println!("Vec: {:?}", vec_duration / ITERATIONS as u32);
    println!("HashMap: {:?}", map_duration / ITERATIONS as u32);
}

/// 基准测试深度嵌套结构的 Clone 性能
fn benchmark_nested_clone() {
    use std::time::Instant;
    
    #[derive(Clone)]
    struct NestedStruct {
        level1: Vec<Vec<String>>,
        level2: HashMap<String, Vec<i32>>,
        level3: Box<Vec<HashMap<i32, String>>>,
    }
    
    // 创建复杂嵌套结构
    let mut nested = NestedStruct {
        level1: Vec::new(),
        level2: HashMap::new(),
        level3: Box::new(Vec::new()),
    };
    
    // 填充数据
    for i in 0..100 {
        nested.level1.push(vec![format!("item_{}", i); 10]);
        nested.level2.insert(format!("key_{}", i), (0..50).collect());
        
        let mut inner_map = HashMap::new();
        for j in 0..20 {
            inner_map.insert(j, format!("value_{}_{}", i, j));
        }
        nested.level3.push(inner_map);
    }
    
    // 测试 Clone 性能
    let start = Instant::now();
    let _cloned = nested.clone();
    let clone_duration = start.elapsed();
    
    println!("复杂嵌套结构 Clone 耗时: {:?}", clone_duration);
    
    // 测试多次 Clone
    let start = Instant::now();
    for _ in 0..10 {
        let _cloned = nested.clone();
    }
    let multiple_clone_duration = start.elapsed();
    
    println!("10 次复杂结构 Clone 耗时: {:?}", multiple_clone_duration);
    println!("平均每次耗时: {:?}", multiple_clone_duration / 10);
}

/// 基准测试智能指针的 Clone 性能
fn benchmark_smart_pointer_clone() {
    use std::time::Instant;
    
    let data = vec![42; 10000];
    let rc_data = Rc::new(data.clone());
    let arc_data = Arc::new(data.clone());
    let box_data = Box::new(data);
    
    const ITERATIONS: usize = 100000;
    
    // Rc Clone 性能（浅拷贝）
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _cloned = Rc::clone(&rc_data);
    }
    let rc_duration = start.elapsed();
    
    // Arc Clone 性能（浅拷贝）
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _cloned = Arc::clone(&arc_data);
    }
    let arc_duration = start.elapsed();
    
    // Box Clone 性能（深拷贝）
    let start = Instant::now();
    for _ in 0..1000 { // 减少迭代次数，因为 Box clone 更昂贵
        let _cloned = box_data.clone();
    }
    let box_duration = start.elapsed();
    
    println!("Rc Clone {} 次 (浅拷贝): {:?}", ITERATIONS, rc_duration);
    println!("Arc Clone {} 次 (浅拷贝): {:?}", ITERATIONS, arc_duration);
    println!("Box Clone {} 次 (深拷贝): {:?}", 1000, box_duration);
    
    println!("\n性能比较:");
    println!("Rc vs Arc: {:.2}x", 
             arc_duration.as_nanos() as f64 / rc_duration.as_nanos() as f64);
    println!("Box vs Rc: {:.2}x (注意：Box 测试次数少100倍)", 
             (box_duration.as_nanos() / 100) as f64 / rc_duration.as_nanos() as f64);
}

/// 比较 Clone 和手动复制的性能
fn compare_clone_vs_manual_copy() {
    use std::time::Instant;
    
    #[derive(Clone)]
    struct Data {
        numbers: Vec<i32>,
        text: String,
        flags: Vec<bool>,
    }
    
    impl Data {
        fn manual_copy(&self) -> Self {
            Data {
                numbers: self.numbers.iter().copied().collect(),
                text: self.text.chars().collect(),
                flags: self.flags.iter().copied().collect(),
            }
        }
    }
    
    let data = Data {
        numbers: (0..1000).collect(),
        text: "Hello, World!".repeat(100),
        flags: vec![true, false].repeat(500),
    };
    
    const ITERATIONS: usize = 1000;
    
    // 测试 Clone 性能
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _cloned = data.clone();
    }
    let clone_duration = start.elapsed();
    
    // 测试手动复制性能
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _copied = data.manual_copy();
    }
    let manual_duration = start.elapsed();
    
    println!("Clone {} 次耗时: {:?}", ITERATIONS, clone_duration);
    println!("手动复制 {} 次耗时: {:?}", ITERATIONS, manual_duration);
    println!("性能比例 (手动/Clone): {:.2}x", 
             manual_duration.as_nanos() as f64 / clone_duration.as_nanos() as f64);
    
    if clone_duration < manual_duration {
        println!("Clone 实现更优化");
    } else {
        println!("手动实现更高效");
    }
}

/// Clone trait 的高级应用
pub fn advanced_clone_patterns() {
    println!("\n=== Clone Trait 高级应用 ===");
    
    // 1. Clone 在设计模式中的应用
    println!("\n1. Clone 在设计模式中的应用:");
    demonstrate_clone_in_patterns();
    
    // 2. Clone 和生命周期
    println!("\n2. Clone 和生命周期:");
    demonstrate_clone_and_lifetimes();
    
    // 3. Clone 的延迟计算
    println!("\n3. Clone 的延迟计算:");
    demonstrate_lazy_clone();
    
    // 4. Clone 的优化策略
    println!("\n4. Clone 的优化策略:");
    demonstrate_clone_optimization();
}

/// 演示 Clone 在设计模式中的应用
fn demonstrate_clone_in_patterns() {
    // 原型模式
    trait Prototype {
        fn clone_prototype(&self) -> Box<dyn Prototype>;
        fn get_info(&self) -> String;
    }
    
    #[derive(Clone)]
    struct ConcretePrototype {
        data: String,
        config: HashMap<String, i32>,
    }
    
    impl Prototype for ConcretePrototype {
        fn clone_prototype(&self) -> Box<dyn Prototype> {
            Box::new(self.clone())
        }
        
        fn get_info(&self) -> String {
            format!("Data: {}, Config: {:?}", self.data, self.config)
        }
    }
    
    let mut original = ConcretePrototype {
        data: String::from("原型数据"),
        config: HashMap::new(),
    };
    original.config.insert("setting1".to_string(), 100);
    original.config.insert("setting2".to_string(), 200);
    
    let cloned = original.clone_prototype();
    println!("原型: {}", original.get_info());
    println!("克隆: {}", cloned.get_info());
    
    // 建造者模式中的 Clone
    #[derive(Debug, Clone)]
    struct Product {
        name: String,
        features: Vec<String>,
        price: f64,
    }
    
    #[derive(Clone)]
    struct ProductBuilder {
        product: Product,
    }
    
    impl ProductBuilder {
        fn new(name: &str) -> Self {
            ProductBuilder {
                product: Product {
                    name: name.to_string(),
                    features: Vec::new(),
                    price: 0.0,
                },
            }
        }
        
        fn add_feature(mut self, feature: &str) -> Self {
            self.product.features.push(feature.to_string());
            self
        }
        
        fn set_price(mut self, price: f64) -> Self {
            self.product.price = price;
            self
        }
        
        fn build(self) -> Product {
            self.product
        }
        
        // 创建变体
        fn create_variant(&self, suffix: &str) -> Self {
            let mut variant = self.clone();
            variant.product.name = format!("{} {}", variant.product.name, suffix);
            variant
        }
    }
    
    let base_builder = ProductBuilder::new("基础产品")
        .add_feature("功能1")
        .add_feature("功能2")
        .set_price(99.99);
    
    let product1 = base_builder.clone().build();
    let product2 = base_builder.create_variant("Pro").add_feature("高级功能").set_price(199.99).build();
    
    println!("\n产品1: {:?}", product1);
    println!("产品2: {:?}", product2);
}

/// 演示 Clone 和生命周期
fn demonstrate_clone_and_lifetimes() {
    // Clone 可以避免复杂的生命周期问题
    fn process_data_with_clone(data: &[String]) -> Vec<String> {
        // 克隆数据，避免生命周期问题
        data.iter().cloned().map(|s| format!("处理后: {}", s)).collect()
    }
    
    fn process_data_with_borrow(data: &[String]) -> Vec<&str> {
        // 使用借用，需要考虑生命周期
        data.iter().map(|s| s.as_str()).collect()
    }
    
    let original_data = vec![
        String::from("数据1"),
        String::from("数据2"),
        String::from("数据3"),
    ];
    
    let cloned_result = process_data_with_clone(&original_data);
    let borrowed_result = process_data_with_borrow(&original_data);
    
    println!("克隆结果: {:?}", cloned_result);
    println!("借用结果: {:?}", borrowed_result);
    
    // cloned_result 可以独立存在，不受 original_data 生命周期限制
    // borrowed_result 的生命周期受 original_data 限制
}

/// 演示延迟 Clone
fn demonstrate_lazy_clone() {
    use std::cell::RefCell;
    
    #[derive(Debug)]
    struct LazyClone<T: Clone> {
        original: Rc<T>,
        cloned: RefCell<Option<T>>,
    }
    
    impl<T: Clone> LazyClone<T> {
        fn new(data: T) -> Self {
            LazyClone {
                original: Rc::new(data),
                cloned: RefCell::new(None),
            }
        }
        
        fn get_cloned(&self) -> T {
            let mut cloned = self.cloned.borrow_mut();
            if cloned.is_none() {
                println!("执行延迟克隆");
                *cloned = Some((*self.original).clone());
            }
            cloned.as_ref().unwrap().clone()
        }
        
        fn get_original(&self) -> &T {
            &self.original
        }
    }
    
    let expensive_data = vec![String::from("昂贵的数据"); 1000];
    let lazy = LazyClone::new(expensive_data);
    
    println!("创建 LazyClone，尚未执行实际克隆");
    println!("原始数据长度: {}", lazy.get_original().len());
    
    // 第一次访问时才执行克隆
    let cloned_data = lazy.get_cloned();
    println!("克隆数据长度: {}", cloned_data.len());
    
    // 后续访问使用缓存的克隆
    let cloned_data2 = lazy.get_cloned();
    println!("再次获取克隆数据长度: {}", cloned_data2.len());
}

/// 演示 Clone 的优化策略
fn demonstrate_clone_optimization() {
    // 写时复制 (Copy-on-Write) 策略
    use std::borrow::Cow;
    
    fn process_string(input: &str) -> Cow<str> {
        if input.contains("特殊") {
            // 需要修改，返回拥有的字符串
            Cow::Owned(input.replace("特殊", "普通"))
        } else {
            // 不需要修改，返回借用
            Cow::Borrowed(input)
        }
    }
    
    let text1 = "普通文本";
    let text2 = "包含特殊标记的文本";
    
    let result1 = process_string(text1);
    let result2 = process_string(text2);
    
    println!("结果1 (借用): {:?}", result1);
    println!("结果2 (拥有): {:?}", result2);
    
    // 引用计数优化
    #[derive(Debug)]
    struct OptimizedClone {
        data: Rc<Vec<String>>,
        metadata: String,
    }
    
    impl Clone for OptimizedClone {
        fn clone(&self) -> Self {
            // 只克隆元数据，数据使用引用计数共享
            OptimizedClone {
                data: Rc::clone(&self.data), // 浅拷贝
                metadata: self.metadata.clone(), // 深拷贝
            }
        }
    }
    
    let original = OptimizedClone {
        data: Rc::new(vec![String::from("共享数据"); 1000]),
        metadata: String::from("独立元数据"),
    };
    
    let cloned = original.clone();
    
    println!("\n优化的克隆:");
    println!("原始引用计数: {}", Rc::strong_count(&original.data));
    println!("克隆后引用计数: {}", Rc::strong_count(&cloned.data));
    println!("数据地址相同: {}", 
             Rc::as_ptr(&original.data) == Rc::as_ptr(&cloned.data));
    println!("元数据地址不同: {}", 
             original.metadata.as_ptr() != cloned.metadata.as_ptr());
}

/// Clone trait 的最佳实践
pub fn best_practices() {
    println!("\n=== Clone Trait 最佳实践 ===");
    
    println!("\n1. 何时实现 Clone:");
    println!("   - 需要创建数据的独立副本");
    println!("   - 避免复杂的生命周期管理");
    println!("   - 实现原型模式或建造者模式");
    println!("   - 在函数间传递数据的所有权");
    
    println!("\n2. Clone 的优势:");
    println!("   - 明确的复制语义");
    println!("   - 灵活的实现策略");
    println!("   - 避免生命周期复杂性");
    println!("   - 支持深拷贝和浅拷贝");
    
    println!("\n3. Clone 的注意事项:");
    println!("   - 可能有较高的性能成本");
    println!("   - 需要仔细设计拷贝策略");
    println!("   - 考虑使用智能指针优化");
    println!("   - 避免不必要的深拷贝");
    
    println!("\n4. 设计建议:");
    println!("   - 优先使用 derive(Clone) 自动实现");
    println!("   - 手动实现时考虑性能影响");
    println!("   - 使用 Rc/Arc 共享不可变数据");
    println!("   - 考虑写时复制 (COW) 策略");
    println!("   - 在文档中说明拷贝行为");
    
    // 实际示例
    demonstrate_best_practice_examples();
}

/// 演示最佳实践示例
fn demonstrate_best_practice_examples() {
    println!("\n=== 最佳实践示例 ===");
    
    // 1. 良好的 Clone 设计
    println!("\n1. 良好的 Clone 设计:");
    
    #[derive(Debug, Clone)]
    struct GoodDesign {
        id: u64,
        name: String,
        tags: Vec<String>,
        shared_config: Rc<HashMap<String, String>>, // 共享不可变配置
    }
    
    let config = Rc::new({
        let mut map = HashMap::new();
        map.insert("theme".to_string(), "dark".to_string());
        map.insert("language".to_string(), "zh-CN".to_string());
        map
    });
    
    let item1 = GoodDesign {
        id: 1,
        name: String::from("项目1"),
        tags: vec![String::from("重要"), String::from("紧急")],
        shared_config: Rc::clone(&config),
    };
    
    let item2 = item1.clone();
    println!("item1: {:?}", item1);
    println!("item2: {:?}", item2);
    println!("配置引用计数: {}", Rc::strong_count(&config));
    
    // 2. 避免的设计模式
    println!("\n2. 应该避免的设计（仅作演示）:");
    
    #[derive(Debug, Clone)]
    struct BadDesign {
        id: u64,
        large_data: Vec<u8>, // 大型数据直接包含
        nested_maps: Vec<HashMap<String, Vec<String>>>, // 深度嵌套
    }
    
    let bad_item = BadDesign {
        id: 1,
        large_data: vec![0; 10000], // 10KB 数据
        nested_maps: vec![HashMap::new(); 100], // 100个空哈希表
    };
    
    println!("BadDesign 大小: {} bytes", std::mem::size_of_val(&bad_item));
    println!("每次 Clone 都会复制所有数据，成本很高");
    
    // 3. 改进的设计
    println!("\n3. 改进的设计:");
    
    #[derive(Debug, Clone)]
    struct ImprovedDesign {
        id: u64,
        large_data: Arc<Vec<u8>>, // 使用 Arc 共享大型数据
        nested_maps: Rc<Vec<HashMap<String, Vec<String>>>>, // 共享嵌套结构
    }
    
    let improved_item = ImprovedDesign {
        id: 1,
        large_data: Arc::new(vec![0; 10000]),
        nested_maps: Rc::new(vec![HashMap::new(); 100]),
    };
    
    let improved_clone = improved_item.clone();
    println!("ImprovedDesign 大小: {} bytes", std::mem::size_of_val(&improved_item));
    println!("Clone 只复制智能指针，成本很低");
    println!("大型数据引用计数: {}", Arc::strong_count(&improved_item.large_data));
}

/// 运行所有 Clone trait 示例
pub fn run_clone_examples() {
    basic_clone_concepts();
    clone_implementation_patterns();
    deep_vs_shallow_clone_analysis();
    performance_analysis();
    advanced_clone_patterns();
    best_practices();
}