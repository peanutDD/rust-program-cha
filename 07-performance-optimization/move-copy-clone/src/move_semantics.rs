//! # Move 语义深度解析
//!
//! Move 是 Rust 所有权系统的核心概念，理解 Move 语义对于掌握 Rust 至关重要。
//! 本模块将深入解析 Move 的工作原理、应用场景和最佳实践。

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

/// Move 语义的基本概念演示
pub fn basic_move_concepts() {
    println!("=== Move 语义基本概念 ===");
    
    // 1. 基本的 Move 操作
    println!("\n1. 基本 Move 操作:");
    let s1 = String::from("Hello, World!");
    println!("s1 创建: {}", s1);
    
    let s2 = s1; // s1 的所有权移动到 s2
    println!("s2 获得所有权: {}", s2);
    // println!("s1: {}", s1); // 编译错误！s1 已被移动
    
    // 2. 函数参数的 Move
    println!("\n2. 函数参数 Move:");
    let data = vec![1, 2, 3, 4, 5];
    println!("原始数据: {:?}", data);
    
    take_ownership(data); // data 被移动到函数中
    // println!("data: {:?}", data); // 编译错误！data 已被移动
    
    // 3. 函数返回值的 Move
    println!("\n3. 函数返回值 Move:");
    let returned_data = create_and_return();
    println!("返回的数据: {:?}", returned_data);
}

/// 接受所有权的函数
fn take_ownership(vec: Vec<i32>) {
    println!("函数内部处理: {:?}", vec);
    // vec 在函数结束时被销毁
}

/// 创建并返回数据的函数
fn create_and_return() -> Vec<i32> {
    let data = vec![10, 20, 30];
    data // 所有权移动给调用者
}

/// Move 语义的内存模型分析
pub fn memory_model_analysis() {
    println!("\n=== Move 语义内存模型分析 ===");
    
    // 1. 栈上数据的 Move
    println!("\n1. 栈上数据 Move:");
    demonstrate_stack_move();
    
    // 2. 堆上数据的 Move
    println!("\n2. 堆上数据 Move:");
    demonstrate_heap_move();
    
    // 3. 复合类型的 Move
    println!("\n3. 复合类型 Move:");
    demonstrate_compound_move();
}

/// 演示栈上数据的 Move
fn demonstrate_stack_move() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p1 = Point { x: 10, y: 20 };
    println!("p1: {:?}", p1);
    
    let p2 = p1; // 栈上按位复制，但仍然是 Move 语义
    println!("p2: {:?}", p2);
    // println!("p1: {:?}", p1); // 编译错误！即使是栈上数据也遵循 Move 语义
}

/// 演示堆上数据的 Move
fn demonstrate_heap_move() {
    let s1 = String::from("堆上字符串");
    println!("s1 地址: {:p}, 内容: {}", s1.as_ptr(), s1);
    
    let s2 = s1; // 只移动栈上的指针、长度、容量，堆上数据不变
    println!("s2 地址: {:p}, 内容: {}", s2.as_ptr(), s2);
    
    // 关键点：堆上的实际数据没有被复制，只是所有权转移了
}

/// 演示复合类型的 Move
fn demonstrate_compound_move() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        hobbies: Vec<String>,
    }
    
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
        hobbies: vec![String::from("读书"), String::from("游泳")],
    };
    
    println!("person1: {:?}", person1);
    
    let person2 = person1; // 整个结构体被移动
    println!("person2: {:?}", person2);
    // println!("person1: {:?}", person1); // 编译错误！
}

/// Move 语义的高级应用
pub fn advanced_move_patterns() {
    println!("\n=== Move 语义高级应用 ===");
    
    // 1. 部分 Move
    println!("\n1. 部分 Move:");
    demonstrate_partial_move();
    
    // 2. Move 和模式匹配
    println!("\n2. Move 和模式匹配:");
    demonstrate_move_in_pattern_matching();
    
    // 3. Move 闭包
    println!("\n3. Move 闭包:");
    demonstrate_move_closures();
    
    // 4. 智能指针的 Move
    println!("\n4. 智能指针 Move:");
    demonstrate_smart_pointer_move();
}

/// 演示部分 Move
fn demonstrate_partial_move() {
    #[derive(Debug)]
    struct Data {
        field1: String,
        field2: i32,
        field3: Vec<i32>,
    }
    
    let data = Data {
        field1: String::from("字段1"),
        field2: 42,
        field3: vec![1, 2, 3],
    };
    
    // 部分字段被移动
    let moved_field1 = data.field1;
    let moved_field3 = data.field3;
    
    println!("移动的字段1: {}", moved_field1);
    println!("移动的字段3: {:?}", moved_field3);
    
    // 仍然可以访问未被移动的字段
    println!("未移动的字段2: {}", data.field2);
    
    // println!("整个结构体: {:?}", data); // 编译错误！部分字段已被移动
}

/// 演示 Move 和模式匹配
fn demonstrate_move_in_pattern_matching() {
    #[derive(Debug)]
    enum Message {
        Text(String),
        Number(i32),
        Coordinates { x: i32, y: i32 },
    }
    
    let msg = Message::Text(String::from("Hello"));
    
    match msg {
        Message::Text(content) => {
            println!("文本消息: {}", content);
            // content 拥有字符串的所有权
        }
        Message::Number(num) => {
            println!("数字消息: {}", num);
        }
        Message::Coordinates { x, y } => {
            println!("坐标消息: ({}, {})", x, y);
        }
    }
    
    // println!("原始消息: {:?}", msg); // 编译错误！msg 已被移动
}

/// 演示 Move 闭包
fn demonstrate_move_closures() {
    let data = vec![1, 2, 3, 4, 5];
    println!("原始数据: {:?}", data);
    
    // move 闭包捕获所有权
    let closure = move || {
        println!("闭包内的数据: {:?}", data);
        data.len()
    };
    
    let len = closure();
    println!("数据长度: {}", len);
    
    // println!("原始数据: {:?}", data); // 编译错误！data 已被移动到闭包中
    
    // 演示 move 闭包在多线程中的应用
    let shared_data = vec![10, 20, 30];
    let handle = std::thread::spawn(move || {
        println!("线程中的数据: {:?}", shared_data);
        shared_data.iter().sum::<i32>()
    });
    
    let sum = handle.join().unwrap();
    println!("线程计算结果: {}", sum);
}

/// 演示智能指针的 Move
fn demonstrate_smart_pointer_move() {
    // Rc 的 Move
    let rc_data = Rc::new(vec![1, 2, 3]);
    println!("Rc 引用计数: {}", Rc::strong_count(&rc_data));
    
    let rc_moved = rc_data; // Rc 本身被移动，但内部数据仍可共享
    println!("移动后 Rc 引用计数: {}", Rc::strong_count(&rc_moved));
    
    // Arc 的 Move
    let arc_data = Arc::new(String::from("共享字符串"));
    let arc_clone = Arc::clone(&arc_data);
    
    let arc_moved = arc_data; // Arc 被移动
    println!("Arc 数据: {}", arc_moved);
    println!("Arc 克隆: {}", arc_clone);
}

/// Move 语义的性能分析
pub fn performance_analysis() {
    println!("\n=== Move 语义性能分析 ===");
    
    // 1. Move vs Copy 性能对比
    println!("\n1. Move vs Copy 性能对比:");
    compare_move_vs_copy_performance();
    
    // 2. 大对象 Move 的性能
    println!("\n2. 大对象 Move 性能:");
    analyze_large_object_move();
    
    // 3. Move 的零成本抽象
    println!("\n3. Move 的零成本抽象:");
    demonstrate_zero_cost_move();
}

/// 比较 Move 和 Copy 的性能
fn compare_move_vs_copy_performance() {
    use std::time::Instant;
    
    // 大型结构体（不实现 Copy）
    #[derive(Debug)]
    struct LargeStruct {
        data: Vec<i32>,
        metadata: HashMap<String, String>,
    }
    
    impl LargeStruct {
        fn new(size: usize) -> Self {
            let mut data = Vec::with_capacity(size);
            for i in 0..size {
                data.push(i as i32);
            }
            
            let mut metadata = HashMap::new();
            metadata.insert("type".to_string(), "large".to_string());
            metadata.insert("size".to_string(), size.to_string());
            
            LargeStruct { data, metadata }
        }
    }
    
    let large_obj = LargeStruct::new(10000);
    
    // 测量 Move 操作的时间
    let start = Instant::now();
    let moved_obj = large_obj; // Move 操作
    let move_duration = start.elapsed();
    
    println!("大对象 Move 耗时: {:?}", move_duration);
    println!("Move 操作几乎是零成本的，只是转移所有权");
    
    // 对比：如果是 Clone 操作
    let another_obj = LargeStruct::new(10000);
    let start = Instant::now();
    let _cloned_obj = another_obj.data.clone(); // 模拟 Clone 操作
    let clone_duration = start.elapsed();
    
    println!("相同数据 Clone 耗时: {:?}", clone_duration);
    println!("Clone 需要实际复制数据，成本较高");
}

/// 分析大对象 Move 的性能
fn analyze_large_object_move() {
    // 创建一个包含大量数据的结构体
    struct MassiveData {
        numbers: Vec<i64>,
        strings: Vec<String>,
        nested: Vec<Vec<i32>>,
    }
    
    impl MassiveData {
        fn new() -> Self {
            let numbers: Vec<i64> = (0..100000).collect();
            let strings: Vec<String> = (0..10000)
                .map(|i| format!("字符串_{}", i))
                .collect();
            let nested: Vec<Vec<i32>> = (0..1000)
                .map(|i| (0..100).map(|j| i * 100 + j).collect())
                .collect();
            
            MassiveData { numbers, strings, nested }
        }
    }
    
    let massive = MassiveData::new();
    println!("创建了包含大量数据的对象");
    
    // Move 操作
    use std::time::Instant;
    let start = Instant::now();
    let moved_massive = massive;
    let duration = start.elapsed();
    
    println!("大对象 Move 耗时: {:?}", duration);
    println!("无论对象多大，Move 操作都是 O(1) 时间复杂度");
    
    // 验证数据完整性
    println!("数字数组长度: {}", moved_massive.numbers.len());
    println!("字符串数组长度: {}", moved_massive.strings.len());
    println!("嵌套数组长度: {}", moved_massive.nested.len());
}

/// 演示 Move 的零成本抽象
fn demonstrate_zero_cost_move() {
    println!("Move 是 Rust 的零成本抽象：");
    println!("- 编译时确定所有权转移");
    println!("- 运行时无额外开销");
    println!("- 内存安全保证");
    println!("- 无需垃圾回收器");
    
    // 演示编译器优化
    let data = create_large_vector();
    let processed = process_vector(data);
    let result = finalize_vector(processed);
    
    println!("最终结果长度: {}", result.len());
    println!("整个过程中没有不必要的数据复制");
}

fn create_large_vector() -> Vec<i32> {
    (0..10000).collect()
}

fn process_vector(mut vec: Vec<i32>) -> Vec<i32> {
    vec.iter_mut().for_each(|x| *x *= 2);
    vec
}

fn finalize_vector(vec: Vec<i32>) -> Vec<i32> {
    vec.into_iter().filter(|&x| x % 4 == 0).collect()
}

/// Move 语义的最佳实践
pub fn best_practices() {
    println!("\n=== Move 语义最佳实践 ===");
    
    println!("\n1. 何时使用 Move:");
    println!("   - 转移大对象的所有权");
    println!("   - 避免不必要的复制");
    println!("   - 实现零成本抽象");
    println!("   - 确保内存安全");
    
    println!("\n2. Move 的优势:");
    println!("   - 零运行时成本");
    println!("   - 内存安全保证");
    println!("   - 避免数据竞争");
    println!("   - 清晰的所有权语义");
    
    println!("\n3. 注意事项:");
    println!("   - 移动后原变量不可用");
    println!("   - 部分移动会影响整个结构体");
    println!("   - 需要理解所有权规则");
    println!("   - 与借用系统配合使用");
    
    // 实际示例
    demonstrate_best_practice_patterns();
}

/// 演示最佳实践模式
fn demonstrate_best_practice_patterns() {
    println!("\n=== 最佳实践示例 ===");
    
    // 1. 构建器模式中的 Move
    println!("\n1. 构建器模式中的 Move:");
    let config = ConfigBuilder::new()
        .set_name("MyApp")
        .set_version("1.0.0")
        .add_feature("logging")
        .add_feature("metrics")
        .build();
    
    println!("构建的配置: {:?}", config);
    
    // 2. 链式调用中的 Move
    println!("\n2. 链式调用中的 Move:");
    let result = vec![1, 2, 3, 4, 5]
        .into_iter()
        .map(|x| x * 2)
        .filter(|&x| x > 5)
        .collect::<Vec<_>>();
    
    println!("链式处理结果: {:?}", result);
    
    // 3. 错误处理中的 Move
    println!("\n3. 错误处理中的 Move:");
    match process_data("valid_data") {
        Ok(data) => println!("处理成功: {}", data),
        Err(e) => println!("处理失败: {}", e),
    }
}

#[derive(Debug)]
struct Config {
    name: String,
    version: String,
    features: Vec<String>,
}

struct ConfigBuilder {
    name: Option<String>,
    version: Option<String>,
    features: Vec<String>,
}

impl ConfigBuilder {
    fn new() -> Self {
        ConfigBuilder {
            name: None,
            version: None,
            features: Vec::new(),
        }
    }
    
    fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
    
    fn set_version(mut self, version: &str) -> Self {
        self.version = Some(version.to_string());
        self
    }
    
    fn add_feature(mut self, feature: &str) -> Self {
        self.features.push(feature.to_string());
        self
    }
    
    fn build(self) -> Config {
        Config {
            name: self.name.unwrap_or_else(|| "Unknown".to_string()),
            version: self.version.unwrap_or_else(|| "0.0.0".to_string()),
            features: self.features,
        }
    }
}

fn process_data(input: &str) -> Result<String, String> {
    if input == "valid_data" {
        Ok(format!("处理后的数据: {}", input))
    } else {
        Err(format!("无效输入: {}", input))
    }
}

/// 运行所有 Move 语义示例
pub fn run_move_examples() {
    basic_move_concepts();
    memory_model_analysis();
    advanced_move_patterns();
    performance_analysis();
    best_practices();
}