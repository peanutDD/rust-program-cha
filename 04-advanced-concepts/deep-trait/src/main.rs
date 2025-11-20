//! # Rust 特征进阶深度分析
//! 
//! 基于 https://course.rs/basic/trait/advance-trait.html 的全面深入分析
//! 本文档深入探讨 Rust 特征系统的高级特性，包括关联类型、默认泛型参数、
//! 完全限定语法、超特征、newtype 模式等核心概念，并提供详尽的实际案例。

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::ops::{Add, Deref};

// 导入模块化的特征实现
mod type_aliases;
mod default_generics;
mod qualified_syntax;
mod supertraits;
mod newtype;

fn main() {
    println!("🦀 Rust 特征进阶分析 - 深度解析与实战应用");
    println!("============================================");

    // 执行所有演示
    type_aliases::demonstrate_type_aliases();
    type_aliases::demonstrate_never_type();
    default_generics::default_generic_params_analysis();
    qualified_syntax::fully_qualified_syntax_analysis();
    supertraits::supertraits_analysis();
    newtype::newtype_pattern_analysis();

    println!("\n============================================");
    println!("🎉 Rust 特征进阶分析完成！");
    println!("\n📚 本分析涵盖了以下核心概念:");
    println!("   • 关联类型的深入应用和设计模式");
    println!("   • 默认泛型参数的最佳实践");
    println!("   • 完全限定语法和消歧义调用");
    println!("   • 超特征的概念和继承关系");
    println!("   • Newtype 模式的设计思想");
    println!("   • 类型别名的高级用法");
    println!("   • Never 类型的概念和应用");
    println!("\n💡 这些概念是 Rust 高级编程的基础，");
    println!("   掌握它们将大大提升你的 Rust 编程能力！");
}

// ============================================================================
// 6. 类型别名的高级用法
// ============================================================================

fn demonstrate_type_aliases() {
    println!("\n=== 类型别名的高级用法 ===");

    // 基础类型别名
    type UserId = u64;
    type UserName = String;
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    // 复杂类型别名
    type EventHandler<T> = Box<dyn Fn(T) -> Result<()> + Send + Sync>;
    type AsyncResult<T> = Pin<Box<dyn Future<Output = Result<T>> + Send>>;

    use std::collections::HashMap;
    use std::future::Future;
    use std::pin::Pin;

    // 泛型类型别名
    type Cache<K, V> = HashMap<K, V>;
    type UserCache = Cache<UserId, UserName>;

    // 特征对象类型别名
    trait Processor {
        fn process(&self, data: &str) -> String;
    }

    type ProcessorBox = Box<dyn Processor + Send>;
    type ProcessorRef<'a> = &'a dyn Processor;

    struct TextProcessor;
    impl Processor for TextProcessor {
        fn process(&self, data: &str) -> String {
            format!("Processed: {}", data)
        }
    }

    // 使用类型别名简化复杂签名
    fn create_user_cache() -> UserCache {
        let mut cache = UserCache::new();
        cache.insert(1, "Alice".to_string());
        cache.insert(2, "Bob".to_string());
        cache
    }

    fn create_processor() -> ProcessorBox {
        Box::new(TextProcessor)
    }

    // 关联类型别名
    trait Iterator2 {
        type Item;
        type IntoIter: Iterator<Item = Self::Item>;

        fn into_iter(self) -> Self::IntoIter;
    }

    // 在impl块中使用类型别名
    impl<T> Iterator2 for Vec<T> {
        type Item = T;
        type IntoIter = std::vec::IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            IntoIterator::into_iter(self)
        }
    }

    let cache = create_user_cache();
    println!("用户缓存大小: {}", cache.len());

    let processor = create_processor();
    println!("处理结果: {}", processor.process("test data"));

    println!("\n=== 类型别名演示完成 ===");
}

// ============================================================================
// 7. Never 类型 (!) 的概念和应用
// ============================================================================

fn demonstrate_never_type() {
    println!("\n=== Never 类型 (!) 的概念和应用 ===");

    // Never 类型表示永远不会返回的函数
    fn panic_function() -> ! {
        panic!("This function never returns normally")
    }

    fn infinite_loop() -> ! {
        loop {
            println!("This loop never ends");
            std::thread::sleep(std::time::Duration::from_millis(1000));
            break; // 为了演示，这里break
        }
        unreachable!()
    }

    // Never 类型在枚举中的应用
    enum MyResult<T, E> {
        Ok(T),
        Err(E),
    }

    // 当错误类型是Never时，表示不可能出错
    // 注意：Never类型目前是实验性的，这里用std::convert::Infallible代替
    type InfallibleResult<T> = MyResult<T, std::convert::Infallible>;

    // Never 类型可以转换为任何类型
    fn example_with_never() -> i32 {
        let condition = true;

        if condition {
            42
        } else {
            // panic! 返回 !，可以转换为 i32
            panic!("This branch is never taken")
        }
    }

    // 在匹配中使用Never类型
    enum Event {
        Click(i32, i32),
        KeyPress(char),
        Quit,
    }

    fn handle_event(event: Event) -> String {
        match event {
            Event::Click(x, y) => format!("Clicked at ({}, {})", x, y),
            Event::KeyPress(c) => format!("Key pressed: {}", c),
            Event::Quit => {
                println!("Quitting application");
                std::process::exit(0); // 返回 !
            }
        }
    }

    // Never 类型在泛型中的应用
    trait TryFrom<T> {
        type Error;
        fn try_from(value: T) -> Result<Self, Self::Error>
        where
            Self: Sized;
    }

    // 当转换总是成功时，Error 类型可以是 !
    struct AlwaysSucceed(i32);

    impl TryFrom<i32> for AlwaysSucceed {
        type Error = std::convert::Infallible; // 表示转换永远不会失败

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            Ok(AlwaysSucceed(value))
        }
    }

    println!("Never 类型示例: {}", example_with_never());

    let event = Event::Click(10, 20);
    println!("事件处理: {}", handle_event(event));

    println!("\n=== Never 类型演示完成 ===");
}

// ============================================================================
// 8. 特征对象的高级特性和限制
// ============================================================================

fn demonstrate_trait_objects_advanced() {
    println!("\n=== 特征对象的高级特性和限制 ===");

    // 对象安全的特征
    trait ObjectSafe {
        fn method(&self) -> String;
        // 不能有泛型方法
        // fn generic_method<T>(&self, value: T); // 这会使特征不是对象安全的

        // 不能有Self类型的参数或返回值（除了&self, &mut self）
        // fn return_self() -> Self; // 这会使特征不是对象安全的

        // 可以有默认实现
        fn default_method(&self) -> String {
            "Default implementation".to_string()
        }
    }

    // 不是对象安全的特征
    trait NotObjectSafe {
        fn generic_method<T>(&self, value: T) -> T;
        fn return_self() -> Self;
        fn static_method() -> String;
    }

    struct MyStruct;
    impl ObjectSafe for MyStruct {
        fn method(&self) -> String {
            "MyStruct implementation".to_string()
        }
    }

    // 特征对象的动态分发
    let obj: Box<dyn ObjectSafe> = Box::new(MyStruct);
    println!("特征对象调用: {}", obj.method());

    // 多特征对象
    trait Display2 {
        fn display(&self) -> String;
    }

    trait Debug2 {
        fn debug(&self) -> String;
    }

    impl Display2 for MyStruct {
        fn display(&self) -> String {
            "Display for MyStruct".to_string()
        }
    }

    impl Debug2 for MyStruct {
        fn debug(&self) -> String {
            "Debug for MyStruct".to_string()
        }
    }

    // 组合多个特征 - 需要创建一个新的特征来组合
    trait DisplayDebug: Display2 + Debug2 {}
    impl DisplayDebug for MyStruct {}

    let multi_obj: Box<dyn DisplayDebug> = Box::new(MyStruct);
    println!("多特征对象 - Display: {}", multi_obj.display());
    println!("多特征对象 - Debug: {}", multi_obj.debug());

    // 特征对象的生命周期
    trait LifetimeTrait {
        fn get_data(&self) -> &str;
    }

    struct DataHolder {
        data: String,
    }

    impl LifetimeTrait for DataHolder {
        fn get_data(&self) -> &str {
            &self.data
        }
    }

    let holder = DataHolder {
        data: "Some data".to_string(),
    };

    // 带生命周期的特征对象
    let lifetime_obj: &dyn LifetimeTrait = &holder;
    println!("生命周期特征对象: {}", lifetime_obj.get_data());

    // 特征对象的向下转型
    use std::any::Any;

    trait AnyTrait: Any {
        fn as_any(&self) -> &dyn Any;
        fn method(&self) -> String;
    }

    impl AnyTrait for MyStruct {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn method(&self) -> String {
            "AnyTrait for MyStruct".to_string()
        }
    }

    let any_obj: Box<dyn AnyTrait> = Box::new(MyStruct);

    // 尝试向下转型
    if let Some(_concrete) = any_obj.as_any().downcast_ref::<MyStruct>() {
        println!("成功向下转型到 MyStruct");
    }

    println!("\n=== 特征对象高级特性演示完成 ===");
}

// ============================================================================
// 9. 孤儿规则的深入理解和绕过技巧
// ============================================================================

fn demonstrate_orphan_rule() {
    println!("\n=== 孤儿规则的深入理解和绕过技巧 ===");

    // 孤儿规则：只能为自己的类型实现外部特征，或为外部类型实现自己的特征

    // 1. 为自己的类型实现外部特征 ✓
    struct MyType(i32);

    impl std::fmt::Display for MyType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MyType({})", self.0)
        }
    }

    // 2. 为外部类型实现自己的特征 ✓
    trait MyTrait {
        fn my_method(&self) -> String;
    }

    impl MyTrait for i32 {
        fn my_method(&self) -> String {
            format!("i32 value: {}", self)
        }
    }

    // 3. 不能为外部类型实现外部特征 ✗
    // impl std::fmt::Display for Vec<i32> { // 这会违反孤儿规则
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         write!(f, "Vec with {} elements", self.len())
    //     }
    // }

    // 绕过技巧1: 使用 newtype 模式
    struct MyVec(Vec<i32>);

    impl std::fmt::Display for MyVec {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MyVec with {} elements", self.0.len())
        }
    }

    // 绕过技巧2: 使用泛型参数
    trait LocalTrait<T> {
        fn process(&self, value: T) -> String;
    }

    // 为外部类型实现带有本地类型参数的特征
    impl LocalTrait<MyType> for Vec<i32> {
        fn process(&self, value: MyType) -> String {
            format!("Processing {} with vec of length {}", value, self.len())
        }
    }

    // 绕过技巧3: 使用扩展特征
    trait VecExt<T> {
        fn my_custom_method(&self) -> String;
    }

    impl<T: std::fmt::Debug> VecExt<T> for Vec<T> {
        fn my_custom_method(&self) -> String {
            format!("Custom method for Vec with {} elements", self.len())
        }
    }

    // 使用示例
    let my_type = MyType(42);
    println!("MyType Display: {}", my_type);

    let number = 123;
    println!("i32 MyTrait: {}", number.my_method());

    let my_vec = MyVec(vec![1, 2, 3]);
    println!("MyVec Display: {}", my_vec);

    let vec = vec![1, 2, 3, 4];
    println!("Vec LocalTrait: {}", vec.process(my_type));

    let debug_vec = vec![1, 2, 3];
    println!("Vec Extension: {}", debug_vec.my_custom_method());

    println!("\n=== 孤儿规则演示完成 ===");
}

// ============================================================================
// 10. 一致性规则和特征实现的冲突解决
// ============================================================================

fn demonstrate_coherence_rules() {
    println!("\n=== 一致性规则和特征实现的冲突解决 ===");

    // 一致性规则确保特征实现的唯一性

    // 1. 重叠实现检测
    trait Process<T> {
        fn process(&self, value: T) -> String;
    }

    struct Processor;

    // 为具体类型实现
    impl Process<i32> for Processor {
        fn process(&self, value: i32) -> String {
            format!("Processing i32: {}", value)
        }
    }

    // 为不同类型实现（不冲突）
    impl Process<String> for Processor {
        fn process(&self, value: String) -> String {
            format!("Processing String: {}", value)
        }
    }

    // 2. 使用关联类型避免冲突
    trait Convert {
        type Input;
        type Output;

        fn convert(&self, input: Self::Input) -> Self::Output;
    }

    struct StringToInt;
    struct IntToString;

    impl Convert for StringToInt {
        type Input = String;
        type Output = Result<i32, std::num::ParseIntError>;

        fn convert(&self, input: Self::Input) -> Self::Output {
            input.parse()
        }
    }

    impl Convert for IntToString {
        type Input = i32;
        type Output = String;

        fn convert(&self, input: Self::Input) -> Self::Output {
            input.to_string()
        }
    }

    // 3. 特化（Specialization）概念（目前不稳定）
    trait SpecializedTrait<T> {
        fn method(&self, value: T) -> String;
    }

    struct SpecializedStruct;

    // 通用实现
    impl<T: std::fmt::Debug> SpecializedTrait<T> for SpecializedStruct {
        fn method(&self, value: T) -> String {
            format!("Generic implementation for: {:?}", value)
        }
    }

    // 如果特化可用，可以为特定类型提供更优化的实现
    // impl SpecializedTrait<i32> for SpecializedStruct {
    //     fn method(&self, value: i32) -> String {
    //         format!("Specialized implementation for i32: {}", value)
    //     }
    // }

    // 4. 使用 where 子句避免冲突
    trait ConditionalTrait<T> {
        fn conditional_method(&self, value: T) -> String;
    }

    struct ConditionalStruct;

    // 为实现了 Display 的类型实现
    impl<T: std::fmt::Display> ConditionalTrait<T> for ConditionalStruct {
        fn conditional_method(&self, value: T) -> String {
            format!("Display implementation: {}", value)
        }
    }

    // 为实现了 Debug 但不实现 Display 的类型实现
    // 注意：这在实际中可能导致冲突，需要更精确的约束

    // 5. 使用标记特征区分实现
    struct Marker1;
    struct Marker2;

    trait MarkerBasedTrait<T, M> {
        fn marker_method(&self, value: T) -> String;
    }

    struct MarkerStruct;

    impl<T: std::fmt::Display> MarkerBasedTrait<T, Marker1> for MarkerStruct {
        fn marker_method(&self, value: T) -> String {
            format!("Marker1 implementation: {}", value)
        }
    }

    impl<T: std::fmt::Debug> MarkerBasedTrait<T, Marker2> for MarkerStruct {
        fn marker_method(&self, value: T) -> String {
            format!("Marker2 implementation: {:?}", value)
        }
    }

    // 使用示例
    let processor = Processor;
    println!("处理 i32: {}", processor.process(42));
    println!("处理 String: {}", processor.process("hello".to_string()));

    let str_to_int = StringToInt;
    let int_to_str = IntToString;

    match str_to_int.convert("123".to_string()) {
        Ok(num) => println!("字符串转整数: {}", num),
        Err(e) => println!("转换失败: {}", e),
    }

    println!("整数转字符串: {}", int_to_str.convert(456));

    let specialized = SpecializedStruct;
    println!("特化示例: {}", specialized.method("test"));

    let conditional = ConditionalStruct;
    println!("条件实现: {}", conditional.conditional_method(42));

    let marker_struct = MarkerStruct;
    println!(
        "标记特征1: {}",
        MarkerBasedTrait::<i32, Marker1>::marker_method(&marker_struct, 42)
    );
    println!(
        "标记特征2: {}",
        MarkerBasedTrait::<&str, Marker2>::marker_method(&marker_struct, "test")
    );

    println!("\n=== 一致性规则演示完成 ===");
}

// ============================================================================
// 1. 关联类型 (Associated Types) 深入分析
// ============================================================================

/// 关联类型是 Rust 特征系统中最重要的高级特性之一
/// 它允许我们在特征定义中声明一个或多个类型占位符
/// 这些类型将在实现特征时被具体化
fn associated_types_analysis() {
    println!("\n=== 1. 关联类型深入分析 ===");

    // 1.1 基础概念：关联类型 vs 泛型参数
    basic_associated_types_demo();

    // 1.2 迭代器模式中的关联类型
    iterator_associated_types_demo();

    // 1.3 复杂关联类型应用
    complex_associated_types_demo();

    // 1.4 关联类型的约束
    associated_types_constraints_demo();
}

/// 1.1 基础概念演示：关联类型 vs 泛型参数
fn basic_associated_types_demo() {
    println!("\n--- 1.1 关联类型 vs 泛型参数 ---");

    // 使用泛型参数的版本（不推荐）
    trait ContainerGeneric<T> {
        fn get(&self) -> &T;
    }

    // 使用关联类型的版本（推荐）
    trait Container {
        type Item; // 关联类型
        fn get(&self) -> &Self::Item;
    }

    // 实现关联类型版本
    struct IntContainer {
        value: i32,
    }

    impl Container for IntContainer {
        type Item = i32; // 具体化关联类型

        fn get(&self) -> &Self::Item {
            &self.value
        }
    }

    struct StringContainer {
        value: String,
    }

    impl Container for StringContainer {
        type Item = String;

        fn get(&self) -> &Self::Item {
            &self.value
        }
    }

    // 使用示例
    let int_container = IntContainer { value: 42 };
    let string_container = StringContainer {
        value: "Hello".to_string(),
    };

    println!("IntContainer value: {}", int_container.get());
    println!("StringContainer value: {}", string_container.get());

    // 关联类型的优势：
    // 1. 每个类型只能有一个实现
    // 2. 更清晰的 API 设计
    // 3. 避免了类型参数的歧义
}

/// 1.2 迭代器模式中的关联类型
fn iterator_associated_types_demo() {
    println!("\n--- 1.2 迭代器模式中的关联类型 ---");

    // 自定义迭代器实现
    struct Counter {
        current: usize,
        max: usize,
    }

    impl Counter {
        fn new(max: usize) -> Counter {
            Counter { current: 0, max }
        }
    }

    impl Iterator for Counter {
        type Item = usize; // 关联类型指定迭代器产生的元素类型

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.max {
                let current = self.current;
                self.current += 1;
                Some(current)
            } else {
                None
            }
        }
    }

    // 使用自定义迭代器
    let mut counter = Counter::new(3);
    println!("Counter iteration:");
    while let Some(value) = counter.next() {
        println!("  {}", value);
    }

    // 使用 for 循环
    println!("Counter with for loop:");
    for value in Counter::new(3) {
        println!("  {}", value);
    }

    // 链式调用
    let sum: usize = Counter::new(5).filter(|x| x % 2 == 0).map(|x| x * x).sum();
    println!("Sum of squares of even numbers: {}", sum);
}

/// 1.3 复杂关联类型应用
fn complex_associated_types_demo() {
    println!("\n--- 1.3 复杂关联类型应用 ---");

    // 图数据结构的关联类型设计
    trait Graph {
        type Node;
        type Edge;

        fn nodes(&self) -> Vec<&Self::Node>;
        fn edges(&self) -> Vec<&Self::Edge>;
        fn neighbors(&self, node: &Self::Node) -> Vec<&Self::Node>;
    }

    // 简单图实现
    #[derive(Debug, Clone, PartialEq)]
    struct SimpleNode {
        id: usize,
        name: String,
    }

    #[derive(Debug, Clone)]
    struct SimpleEdge {
        from: usize,
        to: usize,
        weight: f64,
    }

    struct SimpleGraph {
        nodes: Vec<SimpleNode>,
        edges: Vec<SimpleEdge>,
    }

    impl Graph for SimpleGraph {
        type Node = SimpleNode;
        type Edge = SimpleEdge;

        fn nodes(&self) -> Vec<&Self::Node> {
            self.nodes.iter().collect()
        }

        fn edges(&self) -> Vec<&Self::Edge> {
            self.edges.iter().collect()
        }

        fn neighbors(&self, node: &Self::Node) -> Vec<&Self::Node> {
            self.edges
                .iter()
                .filter(|edge| edge.from == node.id)
                .filter_map(|edge| self.nodes.iter().find(|n| n.id == edge.to))
                .collect()
        }
    }

    // 使用图结构
    let graph = SimpleGraph {
        nodes: vec![
            SimpleNode {
                id: 0,
                name: "A".to_string(),
            },
            SimpleNode {
                id: 1,
                name: "B".to_string(),
            },
            SimpleNode {
                id: 2,
                name: "C".to_string(),
            },
        ],
        edges: vec![
            SimpleEdge {
                from: 0,
                to: 1,
                weight: 1.0,
            },
            SimpleEdge {
                from: 1,
                to: 2,
                weight: 2.0,
            },
            SimpleEdge {
                from: 0,
                to: 2,
                weight: 3.0,
            },
        ],
    };

    println!("Graph nodes: {:?}", graph.nodes());
    println!("Graph edges: {:?}", graph.edges());

    if let Some(node_a) = graph.nodes().first() {
        let neighbors = graph.neighbors(node_a);
        println!("Neighbors of {}: {:?}", node_a.name, neighbors);
    }
}

/// 1.4 关联类型的约束
fn associated_types_constraints_demo() {
    println!("\n--- 1.4 关联类型的约束 ---");

    // 关联类型可以有约束
    trait Collect<T> {
        type Output: IntoIterator<Item = T>; // 约束关联类型

        fn collect(self) -> Self::Output;
    }

    // 为 Vec 实现 Collect
    impl<T> Collect<T> for Vec<T> {
        type Output = Vec<T>;

        fn collect(self) -> Self::Output {
            self
        }
    }

    // 复杂约束示例
    trait Repository {
        type Item: Clone + Debug; // 多重约束
        type Error: Display;

        fn find_by_id(&self, id: u64) -> Result<Self::Item, Self::Error>;
        fn save(&mut self, item: Self::Item) -> Result<(), Self::Error>;
    }

    // 用户仓库实现
    #[derive(Clone, Debug)]
    struct User {
        id: u64,
        name: String,
        email: String,
    }

    #[derive(Debug)]
    struct UserRepositoryError {
        message: String,
    }

    impl Display for UserRepositoryError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "UserRepositoryError: {}", self.message)
        }
    }

    struct UserRepository {
        users: HashMap<u64, User>,
    }

    impl Repository for UserRepository {
        type Item = User;
        type Error = UserRepositoryError;

        fn find_by_id(&self, id: u64) -> Result<Self::Item, Self::Error> {
            self.users.get(&id).cloned().ok_or(UserRepositoryError {
                message: format!("User with id {} not found", id),
            })
        }

        fn save(&mut self, item: Self::Item) -> Result<(), Self::Error> {
            self.users.insert(item.id, item);
            Ok(())
        }
    }

    // 使用仓库
    let mut repo = UserRepository {
        users: HashMap::new(),
    };

    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    match repo.save(user.clone()) {
        Ok(()) => println!("User saved successfully: {:?}", user),
        Err(e) => println!("Failed to save user: {}", e),
    }

    match repo.find_by_id(1) {
        Ok(found_user) => println!("Found user: {:?}", found_user),
        Err(e) => println!("Error finding user: {}", e),
    }

    match repo.find_by_id(999) {
        Ok(found_user) => println!("Found user: {:?}", found_user),
        Err(e) => println!("Error finding user: {}", e),
    }
}

// ============================================================================
// 2. 默认泛型类型参数 (Default Generic Type Parameters) 分析
// ============================================================================

fn default_generic_params_analysis() {
    println!("\n=== 2. 默认泛型类型参数分析 ===");

    // 2.1 基础默认泛型参数
    basic_default_generics_demo();

    // 2.2 运算符重载中的默认泛型参数
    operator_overloading_defaults_demo();

    // 2.3 复杂默认泛型参数应用
    complex_default_generics_demo();
}

/// 2.1 基础默认泛型参数演示
fn basic_default_generics_demo() {
    println!("\n--- 2.1 基础默认泛型参数 ---");

    // 定义带有默认泛型参数的结构体
    struct Point<T = f64> {
        // 默认类型为 f64
        x: T,
        y: T,
    }

    impl<T> Point<T>
    where
        T: Add<Output = T> + Copy + Display,
    {
        fn new(x: T, y: T) -> Self {
            Point { x, y }
        }

        fn add(&self, other: &Point<T>) -> Point<T> {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }

        fn display(&self) {
            println!("Point({}, {})", self.x, self.y);
        }
    }

    // 使用默认类型参数
    let point1 = Point::new(1.0, 2.0); // 自动推断为 Point<f64>
    let point2 = Point::new(3.0, 4.0);
    let sum = point1.add(&point2);

    println!("Default f64 points:");
    point1.display();
    point2.display();
    sum.display();

    // 显式指定类型参数
    let int_point1 = Point::<i32>::new(1, 2);
    let int_point2 = Point::<i32>::new(3, 4);
    let int_sum = int_point1.add(&int_point2);

    println!("\nExplicit i32 points:");
    int_point1.display();
    int_point2.display();
    int_sum.display();
}

/// 2.2 运算符重载中的默认泛型参数
fn operator_overloading_defaults_demo() {
    println!("\n--- 2.2 运算符重载中的默认泛型参数 ---");

    // Add trait 的定义（简化版）
    // trait Add<Rhs = Self> {
    //     type Output;
    //     fn add(self, rhs: Rhs) -> Self::Output;
    // }

    #[derive(Debug, Clone, Copy)]
    struct Millimeters(u32);

    #[derive(Debug, Clone, Copy)]
    struct Meters(u32);

    // 为 Millimeters 实现 Add，默认与自身相加
    impl Add for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Millimeters) -> Millimeters {
            Millimeters(self.0 + other.0)
        }
    }

    // 为 Millimeters 实现与 Meters 相加
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    // 使用示例
    let mm1 = Millimeters(1000);
    let mm2 = Millimeters(500);
    let m1 = Meters(2);

    let result1 = mm1 + mm2; // 使用默认的 Rhs = Self
    let result2 = mm1 + m1; // 使用显式的 Rhs = Meters

    println!("Millimeters + Millimeters: {:?}", result1);
    println!("Millimeters + Meters: {:?}", result2);

    // 自定义数值类型的运算符重载
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Complex {
        real: f64,
        imag: f64,
    }

    impl Complex {
        fn new(real: f64, imag: f64) -> Self {
            Complex { real, imag }
        }
    }

    impl Add for Complex {
        type Output = Complex;

        fn add(self, other: Complex) -> Complex {
            Complex {
                real: self.real + other.real,
                imag: self.imag + other.imag,
            }
        }
    }

    // 复数与实数相加
    impl Add<f64> for Complex {
        type Output = Complex;

        fn add(self, other: f64) -> Complex {
            Complex {
                real: self.real + other,
                imag: self.imag,
            }
        }
    }

    let c1 = Complex::new(1.0, 2.0);
    let c2 = Complex::new(3.0, 4.0);
    let real_num = 5.0;

    let complex_sum = c1 + c2;
    let mixed_sum = c1 + real_num;

    println!("Complex + Complex: {:?}", complex_sum);
    println!("Complex + f64: {:?}", mixed_sum);
}

/// 2.3 复杂默认泛型参数应用
fn complex_default_generics_demo() {
    println!("\n--- 2.3 复杂默认泛型参数应用 ---");

    // 带有多个默认泛型参数的容器
    struct Container<T, E = String, const N: usize = 10> {
        items: Vec<Option<T>>,
        error_handler: fn(E),
    }

    impl<T, E, const N: usize> Container<T, E, N>
    where
        T: Clone + Debug,
        E: Display,
    {
        fn new(error_handler: fn(E)) -> Self {
            Container {
                items: vec![None; N],
                error_handler,
            }
        }

        fn add(&mut self, item: T) -> Result<(), E>
        where
            E: From<&'static str>,
        {
            for slot in &mut self.items {
                if slot.is_none() {
                    *slot = Some(item);
                    return Ok(());
                }
            }
            Err(E::from("Container is full"))
        }

        fn get(&self, index: usize) -> Option<&T> {
            if index < self.items.len() {
                self.items[index].as_ref()
            } else {
                None
            }
        }

        fn len(&self) -> usize {
            self.items.iter().filter(|item| item.is_some()).count()
        }
    }

    // 使用默认参数
    let mut default_container = Container::<i32>::new(|e: String| {
        println!("Error: {}", e);
    });

    // 添加元素
    for i in 0..12 {
        match default_container.add(i) {
            Ok(()) => println!("Added {}", i),
            Err(e) => {
                (default_container.error_handler)(e);
                break;
            }
        }
    }

    println!("Container length: {}", default_container.len());

    // 使用自定义参数
    #[derive(Debug)]
    struct CustomError {
        code: u32,
        message: String,
    }

    impl Display for CustomError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error {}: {}", self.code, self.message)
        }
    }

    impl From<&'static str> for CustomError {
        fn from(msg: &'static str) -> Self {
            CustomError {
                code: 500,
                message: msg.to_string(),
            }
        }
    }

    let mut custom_container = Container::<String, CustomError, 5>::new(|e: CustomError| {
        println!("Custom error handler: {}", e);
    });

    for i in 0..7 {
        let item = format!("Item {}", i);
        match custom_container.add(item.clone()) {
            Ok(()) => println!("Added {}", item),
            Err(e) => {
                (custom_container.error_handler)(e);
                break;
            }
        }
    }

    println!("Custom container length: {}", custom_container.len());
}

// ============================================================================
// 3. 完全限定语法 (Fully Qualified Syntax) 分析
// ============================================================================

fn fully_qualified_syntax_analysis() {
    println!("\n=== 3. 完全限定语法分析 ===");

    // 3.1 基础完全限定语法
    basic_fully_qualified_syntax_demo();

    // 3.2 消歧义调用
    disambiguation_demo();

    // 3.3 关联函数的完全限定调用
    associated_functions_demo();
}

/// 3.1 基础完全限定语法演示
fn basic_fully_qualified_syntax_demo() {
    println!("\n--- 3.1 基础完全限定语法 ---");

    trait Pilot {
        fn fly(&self);
        fn name() -> String;
    }

    trait Wizard {
        fn fly(&self);
        fn name() -> String;
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }

        fn name() -> String {
            String::from("Captain")
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }

        fn name() -> String {
            String::from("Gandalf")
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }

        fn name() -> String {
            String::from("Human")
        }
    }

    let person = Human;

    // 默认调用 Human 的方法
    person.fly();

    // 使用完全限定语法调用特定 trait 的方法
    Pilot::fly(&person);
    Wizard::fly(&person);

    // 对于关联函数，必须使用完全限定语法
    println!("Human name: {}", Human::name());
    println!("Pilot name: {}", <Human as Pilot>::name());
    println!("Wizard name: {}", <Human as Wizard>::name());
}

/// 3.2 消歧义调用演示
fn disambiguation_demo() {
    println!("\n--- 3.2 消歧义调用 ---");

    trait Animal {
        fn baby_name() -> String;
        fn make_sound(&self);
    }

    trait Dog {
        fn baby_name() -> String;
        fn make_sound(&self);
    }

    struct Puppy;

    impl Animal for Puppy {
        fn baby_name() -> String {
            String::from("puppy")
        }

        fn make_sound(&self) {
            println!("Animal sound: generic animal noise");
        }
    }

    impl Dog for Puppy {
        fn baby_name() -> String {
            String::from("Spot")
        }

        fn make_sound(&self) {
            println!("Dog sound: Woof!");
        }
    }

    impl Puppy {
        fn baby_name() -> String {
            String::from("puppy instance")
        }

        fn make_sound(&self) {
            println!("Puppy sound: Yip!");
        }
    }

    let puppy = Puppy;

    // 实例方法调用
    puppy.make_sound(); // 默认调用 Puppy 的方法
    Animal::make_sound(&puppy); // 调用 Animal trait 的方法
    Dog::make_sound(&puppy); // 调用 Dog trait 的方法

    // 关联函数调用需要完全限定语法
    println!("Puppy baby name: {}", Puppy::baby_name());
    println!("Animal baby name: {}", <Puppy as Animal>::baby_name());
    println!("Dog baby name: {}", <Puppy as Dog>::baby_name());

    // 复杂的消歧义场景
    trait Display {
        fn fmt(&self) -> String;
    }

    trait Debug {
        fn fmt(&self) -> String;
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl Display for Point {
        fn fmt(&self) -> String {
            format!("({}, {})", self.x, self.y)
        }
    }

    impl Debug for Point {
        fn fmt(&self) -> String {
            format!("Point {{ x: {}, y: {} }}", self.x, self.y)
        }
    }

    let point = Point { x: 1, y: 2 };

    println!("Display format: {}", <Point as Display>::fmt(&point));
    println!("Debug format: {}", <Point as Debug>::fmt(&point));
}

/// 3.3 关联函数的完全限定调用
fn associated_functions_demo() {
    println!("\n--- 3.3 关联函数的完全限定调用 ---");

    trait Factory<T> {
        fn create() -> T;
        fn create_with_value(value: i32) -> T;
    }

    #[derive(Debug)]
    struct Product {
        id: i32,
        name: String,
    }

    impl Factory<Product> for Product {
        fn create() -> Product {
            Product {
                id: 0,
                name: "Default Product".to_string(),
            }
        }

        fn create_with_value(value: i32) -> Product {
            Product {
                id: value,
                name: format!("Product {}", value),
            }
        }
    }

    // 使用完全限定语法调用关联函数
    let default_product = <Product as Factory<Product>>::create();
    let custom_product = <Product as Factory<Product>>::create_with_value(42);

    println!("Default product: {:?}", default_product);
    println!("Custom product: {:?}", custom_product);

    // 泛型关联函数的完全限定调用
    trait Parser<T> {
        type Error;
        fn parse(input: &str) -> Result<T, Self::Error>;
    }

    struct IntParser;

    impl Parser<i32> for IntParser {
        type Error = std::num::ParseIntError;

        fn parse(input: &str) -> Result<i32, Self::Error> {
            input.parse()
        }
    }

    struct FloatParser;

    impl Parser<f64> for FloatParser {
        type Error = std::num::ParseFloatError;

        fn parse(input: &str) -> Result<f64, Self::Error> {
            input.parse()
        }
    }

    // 完全限定语法调用泛型关联函数
    match <IntParser as Parser<i32>>::parse("42") {
        Ok(value) => println!("Parsed integer: {}", value),
        Err(e) => println!("Failed to parse integer: {}", e),
    }

    match <FloatParser as Parser<f64>>::parse("3.14") {
        Ok(value) => println!("Parsed float: {}", value),
        Err(e) => println!("Failed to parse float: {}", e),
    }

    match <IntParser as Parser<i32>>::parse("invalid") {
        Ok(value) => println!("Parsed integer: {}", value),
        Err(e) => println!("Failed to parse integer: {}", e),
    }
}

// ============================================================================
// 4. 超特征 (Supertraits) 分析
// ============================================================================

fn supertraits_analysis() {
    println!("\n=== 4. 超特征分析 ===");

    // 4.1 基础超特征概念
    basic_supertraits_demo();

    // 4.2 多重超特征
    multiple_supertraits_demo();

    // 4.3 超特征的实际应用
    supertraits_practical_demo();
}

/// 4.1 基础超特征概念演示
fn basic_supertraits_demo() {
    println!("\n--- 4.1 基础超特征概念 ---");

    // 定义超特征关系
    trait Animal {
        fn name(&self) -> &str;
        fn make_sound(&self);
    }

    // Dog 特征依赖于 Animal 特征（Animal 是 Dog 的超特征）
    trait Dog: Animal {
        fn breed(&self) -> &str;
        fn fetch(&self) {
            println!("{} is fetching!", self.name());
        }
    }

    struct GoldenRetriever {
        name: String,
    }

    impl Animal for GoldenRetriever {
        fn name(&self) -> &str {
            &self.name
        }

        fn make_sound(&self) {
            println!("{} says: Woof! Woof!", self.name());
        }
    }

    impl Dog for GoldenRetriever {
        fn breed(&self) -> &str {
            "Golden Retriever"
        }
    }

    let dog = GoldenRetriever {
        name: "Buddy".to_string(),
    };

    // 可以调用超特征的方法
    println!("Dog name: {}", dog.name());
    dog.make_sound();

    // 也可以调用子特征的方法
    println!("Dog breed: {}", dog.breed());
    dog.fetch();

    // 超特征约束的函数
    fn train_dog<T: Dog>(dog: &T) {
        println!("Training {} ({})", dog.name(), dog.breed());
        dog.fetch();
        dog.make_sound();
    }

    train_dog(&dog);
}

/// 4.2 多重超特征演示
fn multiple_supertraits_demo() {
    println!("\n--- 4.2 多重超特征 ---");

    // 多重超特征约束
    trait Drawable {
        fn draw(&self);
    }

    trait Clickable {
        fn click(&self);
    }

    // Button 需要同时实现 Drawable 和 Clickable
    trait Button: Drawable + Clickable {
        fn label(&self) -> &str;

        fn render(&self) {
            println!("Rendering button: {}", self.label());
            self.draw();
        }
    }

    struct SubmitButton {
        text: String,
    }

    impl Drawable for SubmitButton {
        fn draw(&self) {
            println!("Drawing submit button with text: {}", self.text);
        }
    }

    impl Clickable for SubmitButton {
        fn click(&self) {
            println!("Submit button clicked! Submitting form...");
        }
    }

    impl Button for SubmitButton {
        fn label(&self) -> &str {
            &self.text
        }
    }

    let submit_btn = SubmitButton {
        text: "Submit".to_string(),
    };

    submit_btn.render();
    submit_btn.click();

    // 复杂的多重超特征约束
    trait Serializable {
        fn serialize(&self) -> String;
    }

    trait Deserializable {
        fn deserialize(data: &str) -> Self;
    }

    trait Persistable: Serializable + Deserializable + Clone + Debug {
        fn save(&self) -> Result<(), String> {
            let data = self.serialize();
            println!("Saving data: {}", data);
            // 模拟保存操作
            Ok(())
        }

        fn load(data: &str) -> Result<Self, String> {
            let obj = Self::deserialize(data);
            println!("Loaded object: {:?}", obj);
            Ok(obj)
        }
    }

    #[derive(Clone, Debug)]
    struct User {
        id: u32,
        name: String,
        email: String,
    }

    impl Serializable for User {
        fn serialize(&self) -> String {
            format!("{}|{}|{}", self.id, self.name, self.email)
        }
    }

    impl Deserializable for User {
        fn deserialize(data: &str) -> Self {
            let parts: Vec<&str> = data.split('|').collect();
            User {
                id: parts[0].parse().unwrap_or(0),
                name: parts[1].to_string(),
                email: parts[2].to_string(),
            }
        }
    }

    impl Persistable for User {}

    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    match user.save() {
        Ok(()) => println!("User saved successfully"),
        Err(e) => println!("Failed to save user: {}", e),
    }

    let data = "2|Bob|bob@example.com";
    match User::load(data) {
        Ok(loaded_user) => println!("Loaded user: {:?}", loaded_user),
        Err(e) => println!("Failed to load user: {}", e),
    }
}

/// 4.3 超特征的实际应用
fn supertraits_practical_demo() {
    println!("\n--- 4.3 超特征的实际应用 ---");

    // 构建一个图形系统的特征层次结构
    trait Shape {
        fn area(&self) -> f64;
        fn perimeter(&self) -> f64;
    }

    trait Drawable: Shape {
        fn draw(&self) {
            println!(
                "Drawing shape with area: {:.2}, perimeter: {:.2}",
                self.area(),
                self.perimeter()
            );
        }
    }

    trait Transformable: Shape {
        fn translate(&mut self, dx: f64, dy: f64);
        fn scale(&mut self, factor: f64);
    }

    // 高级图形特征需要同时支持绘制和变换
    trait AdvancedShape: Drawable + Transformable {
        fn render_with_transform(&mut self, dx: f64, dy: f64, scale: f64) {
            self.translate(dx, dy);
            self.scale(scale);
            self.draw();
        }
    }

    #[derive(Debug)]
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }

        fn perimeter(&self) -> f64 {
            2.0 * std::f64::consts::PI * self.radius
        }
    }

    impl Drawable for Circle {
        fn draw(&self) {
            println!(
                "Drawing circle at ({:.1}, {:.1}) with radius {:.1}",
                self.x, self.y, self.radius
            );
            println!(
                "  Area: {:.2}, Perimeter: {:.2}",
                self.area(),
                self.perimeter()
            );
        }
    }

    impl Transformable for Circle {
        fn translate(&mut self, dx: f64, dy: f64) {
            self.x += dx;
            self.y += dy;
            println!("Translated circle to ({:.1}, {:.1})", self.x, self.y);
        }

        fn scale(&mut self, factor: f64) {
            self.radius *= factor;
            println!("Scaled circle radius to {:.1}", self.radius);
        }
    }

    impl AdvancedShape for Circle {}

    let mut circle = Circle {
        x: 0.0,
        y: 0.0,
        radius: 5.0,
    };

    println!("Initial circle:");
    circle.draw();

    println!("\nApplying transformations:");
    circle.render_with_transform(10.0, 20.0, 1.5);

    // 使用超特征约束的泛型函数
    fn process_advanced_shape<T: AdvancedShape>(shape: &mut T) {
        println!("\nProcessing advanced shape:");
        shape.draw();
        shape.translate(5.0, 5.0);
        shape.scale(0.8);
        shape.draw();
    }

    process_advanced_shape(&mut circle);
}

// ============================================================================
// 5. newtype 模式分析
// ============================================================================

fn newtype_pattern_analysis() {
    println!("\n=== 5. newtype 模式分析 ===");

    // 5.1 基础 newtype 模式
    basic_newtype_demo();

    // 5.2 类型安全的 newtype
    type_safe_newtype_demo();

    // 5.3 为外部类型实现特征
    external_type_traits_demo();

    // 5.4 高级 newtype 应用
    advanced_newtype_demo();
}

/// 5.1 基础 newtype 模式演示
fn basic_newtype_demo() {
    println!("\n--- 5.1 基础 newtype 模式 ---");

    // 基础 newtype 包装
    struct Meters(f64);
    struct Kilometers(f64);

    impl Meters {
        fn new(value: f64) -> Self {
            Meters(value)
        }

        fn value(&self) -> f64 {
            self.0
        }

        fn to_kilometers(&self) -> Kilometers {
            Kilometers(self.0 / 1000.0)
        }
    }

    impl Kilometers {
        fn new(value: f64) -> Self {
            Kilometers(value)
        }

        fn value(&self) -> f64 {
            self.0
        }

        fn to_meters(&self) -> Meters {
            Meters(self.0 * 1000.0)
        }
    }

    // 类型安全的距离计算
    let distance_m = Meters::new(1500.0);
    let distance_km = distance_m.to_kilometers();

    println!(
        "Distance: {:.1} meters = {:.2} kilometers",
        distance_m.value(),
        distance_km.value()
    );

    // 防止类型混淆
    fn calculate_speed(distance: Meters, time_seconds: f64) -> f64 {
        distance.value() / time_seconds
    }

    let speed = calculate_speed(distance_m, 60.0);
    println!("Speed: {:.2} m/s", speed);

    // 如果传入错误类型，编译器会报错
    // let wrong_speed = calculate_speed(distance_km, 60.0); // 编译错误
}

/// 5.2 类型安全的 newtype 演示
fn type_safe_newtype_demo() {
    println!("\n--- 5.2 类型安全的 newtype ---");

    // 用户 ID 和订单 ID 的类型安全包装
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct UserId(u64);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct OrderId(u64);

    impl UserId {
        fn new(id: u64) -> Self {
            UserId(id)
        }

        fn value(&self) -> u64 {
            self.0
        }
    }

    impl OrderId {
        fn new(id: u64) -> Self {
            OrderId(id)
        }

        fn value(&self) -> u64 {
            self.0
        }
    }

    // 类型安全的函数
    fn get_user_orders(user_id: UserId) -> Vec<OrderId> {
        println!("Getting orders for user {}", user_id.value());
        vec![OrderId::new(1001), OrderId::new(1002), OrderId::new(1003)]
    }

    fn get_order_details(order_id: OrderId) -> String {
        format!("Order {} details", order_id.value())
    }

    let user_id = UserId::new(42);
    let order_id = OrderId::new(1001);

    let orders = get_user_orders(user_id);
    println!("User orders: {:?}", orders);

    let details = get_order_details(order_id);
    println!("Order details: {}", details);

    // 防止参数传递错误
    // let wrong_orders = get_user_orders(order_id); // 编译错误
    // let wrong_details = get_order_details(user_id); // 编译错误

    // 密码和令牌的安全包装
    #[derive(Debug)]
    struct Password(String);

    #[derive(Debug)]
    struct Token(String);

    impl Password {
        fn new(password: String) -> Self {
            Password(password)
        }

        fn verify(&self, input: &str) -> bool {
            self.0 == input
        }

        // 不提供直接访问密码的方法，增强安全性
    }

    impl Token {
        fn new(token: String) -> Self {
            Token(token)
        }

        fn is_valid(&self) -> bool {
            !self.0.is_empty() && self.0.len() > 10
        }

        fn value(&self) -> &str {
            &self.0
        }
    }

    let password = Password::new("secret123".to_string());
    let token = Token::new("abc123def456ghi789".to_string());

    println!("Password verification: {}", password.verify("secret123"));
    println!("Token is valid: {}", token.is_valid());
    println!("Token value: {}", token.value());
}

/// 5.3 为外部类型实现特征
fn external_type_traits_demo() {
    println!("\n--- 5.3 为外部类型实现特征 ---");

    // 为 Vec<String> 实现自定义特征（通过 newtype 模式绕过孤儿规则）
    struct Wrapper(Vec<String>);

    trait Summary {
        fn summarize(&self) -> String;
    }

    impl Summary for Wrapper {
        fn summarize(&self) -> String {
            format!("List with {} items: [{}]", self.0.len(), self.0.join(", "))
        }
    }

    impl Deref for Wrapper {
        type Target = Vec<String>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl From<Vec<String>> for Wrapper {
        fn from(vec: Vec<String>) -> Self {
            Wrapper(vec)
        }
    }

    let items = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
    ];

    let wrapper = Wrapper::from(items);
    println!("Summary: {}", wrapper.summarize());
    println!("First item: {}", wrapper[0]); // 通过 Deref 可以像 Vec 一样使用
    println!("Length: {}", wrapper.len());

    // 为 HashMap 实现自定义特征
    struct ConfigMap(HashMap<String, String>);

    trait Configuration {
        fn get_config(&self, key: &str) -> Option<&String>;
        fn set_config(&mut self, key: String, value: String);
        fn has_config(&self, key: &str) -> bool;
        fn list_configs(&self) -> Vec<(&String, &String)>;
    }

    impl Configuration for ConfigMap {
        fn get_config(&self, key: &str) -> Option<&String> {
            self.0.get(key)
        }

        fn set_config(&mut self, key: String, value: String) {
            self.0.insert(key, value);
        }

        fn has_config(&self, key: &str) -> bool {
            self.0.contains_key(key)
        }

        fn list_configs(&self) -> Vec<(&String, &String)> {
            self.0.iter().collect()
        }
    }

    impl Default for ConfigMap {
        fn default() -> Self {
            ConfigMap(HashMap::new())
        }
    }

    let mut config = ConfigMap::default();
    config.set_config("database_url".to_string(), "localhost:5432".to_string());
    config.set_config("api_key".to_string(), "secret_key_123".to_string());

    println!("\nConfiguration:");
    for (key, value) in config.list_configs() {
        println!("  {}: {}", key, value);
    }

    if let Some(db_url) = config.get_config("database_url") {
        println!("Database URL: {}", db_url);
    }

    println!("Has API key: {}", config.has_config("api_key"));
    println!("Has password: {}", config.has_config("password"));
}

/// 5.4 高级 newtype 应用
fn advanced_newtype_demo() {
    println!("\n--- 5.4 高级 newtype 应用 ---");

    // 状态机模式中的 newtype
    struct Locked;
    struct Unlocked;

    struct StateMachine<State> {
        data: String,
        _state: PhantomData<State>,
    }

    impl StateMachine<Locked> {
        fn new(data: String) -> Self {
            StateMachine {
                data,
                _state: PhantomData,
            }
        }

        fn unlock(self, password: &str) -> Result<StateMachine<Unlocked>, Self> {
            if password == "correct_password" {
                Ok(StateMachine {
                    data: self.data,
                    _state: PhantomData,
                })
            } else {
                Err(self)
            }
        }
    }

    impl StateMachine<Unlocked> {
        fn read_data(&self) -> &str {
            &self.data
        }

        fn write_data(&mut self, new_data: String) {
            self.data = new_data;
        }

        fn lock(self) -> StateMachine<Locked> {
            StateMachine {
                data: self.data,
                _state: PhantomData,
            }
        }
    }

    // 使用状态机
    let locked_machine = StateMachine::<Locked>::new("Secret data".to_string());

    // 尝试解锁
    match locked_machine.unlock("wrong_password") {
        Ok(_) => println!("Unlocked successfully"),
        Err(machine) => {
            println!("Failed to unlock, trying again...");
            match machine.unlock("correct_password") {
                Ok(mut unlocked) => {
                    println!("Unlocked successfully!");
                    println!("Data: {}", unlocked.read_data());
                    unlocked.write_data("Modified data".to_string());
                    println!("Modified data: {}", unlocked.read_data());

                    let _locked_again = unlocked.lock();
                    println!("Machine locked again");
                }
                Err(_) => println!("Still failed to unlock"),
            }
        }
    }

    // 单位系统的 newtype
    #[derive(Debug, Clone, Copy)]
    struct Celsius(f64);

    #[derive(Debug, Clone, Copy)]
    struct Fahrenheit(f64);

    #[derive(Debug, Clone, Copy)]
    struct Kelvin(f64);

    impl Celsius {
        fn new(temp: f64) -> Self {
            Celsius(temp)
        }

        fn to_fahrenheit(self) -> Fahrenheit {
            Fahrenheit(self.0 * 9.0 / 5.0 + 32.0)
        }

        fn to_kelvin(self) -> Kelvin {
            Kelvin(self.0 + 273.15)
        }

        fn value(self) -> f64 {
            self.0
        }
    }

    impl Fahrenheit {
        fn new(temp: f64) -> Self {
            Fahrenheit(temp)
        }

        fn to_celsius(self) -> Celsius {
            Celsius((self.0 - 32.0) * 5.0 / 9.0)
        }

        fn to_kelvin(self) -> Kelvin {
            self.to_celsius().to_kelvin()
        }

        fn value(self) -> f64 {
            self.0
        }
    }

    impl Kelvin {
        fn new(temp: f64) -> Self {
            Kelvin(temp)
        }

        fn to_celsius(self) -> Celsius {
            Celsius(self.0 - 273.15)
        }

        fn to_fahrenheit(self) -> Fahrenheit {
            self.to_celsius().to_fahrenheit()
        }

        fn value(self) -> f64 {
            self.0
        }
    }

    // 温度转换示例
    let temp_c = Celsius::new(25.0);
    let temp_f = temp_c.to_fahrenheit();
    let temp_k = temp_c.to_kelvin();

    println!("\nTemperature conversions:");
    println!(
        "{:.1}°C = {:.1}°F = {:.1}K",
        temp_c.value(),
        temp_f.value(),
        temp_k.value()
    );

    let temp_f2 = Fahrenheit::new(100.0);
    let temp_c2 = temp_f2.to_celsius();
    println!("{:.1}°F = {:.1}°C", temp_f2.value(), temp_c2.value());
}
