//! 类型别名与特征对象模块
//! 
//! 本模块展示了 Rust 中类型别名的定义和使用，以及特征对象的高级特性和限制。

use std::fmt::{Debug, Display};

/// 演示类型别名的使用
pub fn demonstrate_type_aliases() {
    println!("\n--- 类型别名演示 ---");
    
    // 1. 基础类型别名
    type Kilometers = i32;
    let distance: Kilometers = 5;
    println!("Distance: {} km", distance);
    
    // 2. 复杂类型别名
    type Result<T> = std::result::Result<T, String>;
    type IntegerResult = Result<i32>;
    
    fn compute() -> IntegerResult {
        Ok(42)
    }
    
    match compute() {
        Ok(value) => println!("Computed value: {}", value),
        Err(e) => println!("Error: {}", e),
    }
    
    // 3. 泛型类型别名
    type Map<K, V> = std::collections::HashMap<K, V>;
    type StringMap<V> = Map<String, V>;
    
    let mut scores: StringMap<i32> = Map::new();
    scores.insert("Alice".to_string(), 100);
    scores.insert("Bob".to_string(), 85);
    
    println!("Scores: {:?}", scores);
    
    // 4. 特征对象类型别名
    type AnyDisplay = Box<dyn Display>;
    
    let values: Vec<AnyDisplay> = vec![
        Box::new(42),
        Box::new(3.14),
        Box::new("Hello".to_string()),
    ];
    
    for value in values {
        println!("Value: {}", value);
    }
    
    // 如果需要组合多个非 auto trait，需要创建一个新特征
    trait DebugDisplay: Debug + Display {}
    impl<T: Debug + Display> DebugDisplay for T {}
    
    type AnyDebugDisplay = Box<dyn DebugDisplay>;
    
    let debug_values: Vec<AnyDebugDisplay> = vec![
        Box::new(42),
        Box::new("Combined traits".to_string()),
    ];
    
    for value in debug_values {
        println!("Debug + Display value: {}", value);
    }
}

/// 定义 Iterator2 特征
pub trait Iterator2<T> {
    fn next2(&mut self) -> Option<T>;
}

/// 为 Vec 实现 Iterator2 特征
impl<T> Iterator2<T> for Vec<T> {
    fn next2(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.remove(0))
        }
    }
}

/// 演示 Never 类型的使用
pub fn demonstrate_never_type() {
    println!("\n--- Never 类型 (!) 演示 ---");
    
    // 1. panic 函数返回 !
    #[allow(dead_code)]
    fn always_panic() -> ! {
        panic!("This function never returns!");
    }
    
    // 2. Never 类型在枚举中的应用
    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    // 3. Never 类型可以转换为任何类型
    // 注意：Result<(), !> 需要 never_type feature，这里展示概念
    // let result: Result<(), !> = Ok(());
    // let () = result.unwrap();
    
    println!("Never 类型 (!) 表示永远不会返回的函数");
    println!("示例：panic!、loop {{}}、std::process::exit() 等");
}

/// 定义对象安全的特征
pub trait ObjectSafe {
    fn method1(&self);
    fn method2(&mut self, input: i32) -> bool;
}

/// 定义非对象安全的特征（因为有泛型方法）
pub trait NonObjectSafe {
    fn generic_method<T>(&self, input: T) where T: Debug;
}

/// 定义可组合的特征
pub trait Printable {
    fn print(&self);
}

pub trait Loggable {
    fn log(&self) -> String;
}

/// 创建一个组合特征来支持多特征对象
pub trait PrintableLoggable: Printable + Loggable {}

/// 为所有实现了 Printable 和 Loggable 的类型自动实现 PrintableLoggable
impl<T: Printable + Loggable> PrintableLoggable for T {}

/// 多特征对象的组合
pub type PrintableLogger = Box<dyn PrintableLoggable>;

/// 演示特征对象的生命周期
pub fn demonstrate_trait_object_lifetimes() {
    println!("\n--- 特征对象生命周期演示 ---");
    
    let string = "Hello".to_string();
    
    // 使用静态生命周期的特征对象
    let static_display: Box<dyn Display + 'static> = Box::new(string.clone());
    println!("Static display: {}", static_display);
    
    // 使用引用的特征对象（具有显式生命周期）
    let ref_display: &dyn Display = &string;
    println!("Reference display: {}", ref_display);
}

/// 演示特征对象的向下转型
pub fn demonstrate_downcasting() {
    println!("\n--- 特征对象向下转型演示 ---");
    
    use std::any::Any;
    
    // 将不同类型存储为 Any 特征对象
    let objects: Vec<Box<dyn Any>> = vec![
        Box::new(42),
        Box::new("Hello"),
        Box::new(3.14),
    ];
    
    for obj in objects {
        // 尝试向下转型为特定类型
        if let Some(i) = obj.downcast_ref::<i32>() {
            println!("Found an integer: {}", i);
        } else if let Some(s) = obj.downcast_ref::<&str>() {
            println!("Found a string: {}", s);
        } else if let Some(f) = obj.downcast_ref::<f64>() {
            println!("Found a float: {}", f);
        }
    }
}