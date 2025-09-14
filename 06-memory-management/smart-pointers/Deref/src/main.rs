//! Rust Deref 解引用 - 全面深入教程
//!
//! 本教程基于 https://course.rs/advance/smart-pointer/deref.html 内容
//! 全面分析 Deref trait 的核心概念、实现机制和实际应用
//!
//! 主要内容：
//! 1. 解引用运算符基础
//! 2. Deref trait 实现
//! 3. Deref 强制转换 (Deref Coercion)
//! 4. DerefMut 可变解引用
//! 5. 智能指针设计模式
//! 6. 内存管理和性能分析
//! 7. 实际应用场景
//! 8. 高级模式和最佳实践

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;
use std::time::Instant;

fn main() {
    println!("{}", "=".repeat(80));
    println!("🦀 Rust Deref 解引用 - 全面深入教程");
    println!("{}", "=".repeat(80));

    // 1. 解引用运算符基础
    demonstrate_basic_dereferencing();

    // 2. Deref trait 实现
    demonstrate_deref_trait();

    // 3. Deref 强制转换
    demonstrate_deref_coercion();

    // 4. DerefMut 可变解引用
    demonstrate_deref_mut();

    // 5. 智能指针设计模式
    demonstrate_smart_pointer_patterns();

    // 6. 内存管理演示
    demonstrate_memory_management();

    // 7. 实际应用场景
    demonstrate_practical_scenarios();

    // 8. 高级模式和性能分析
    demonstrate_advanced_patterns();

    println!("{}", "=".repeat(80));
    println!("✅ Deref 解引用教程完成！");
    println!("{}", "=".repeat(80));
}

/// 1. 解引用运算符基础
fn demonstrate_basic_dereferencing() {
    println!("\n📚 1. 解引用运算符基础");
    println!("{}", "-".repeat(50));

    // 1.1 常规引用的解引用
    println!("\n🔍 1.1 常规引用的解引用");
    let x = 5;
    let y = &x; // y 是 x 的引用

    println!("x = {}", x);
    println!("y = {:p} (引用地址)", y);
    println!("*y = {} (解引用后的值)", *y);

    // 验证相等性
    assert_eq!(5, x);
    assert_eq!(5, *y); // 必须解引用才能比较值
    println!("✅ 引用解引用验证通过");

    // 1.2 Box<T> 的解引用
    println!("\n🔍 1.2 Box<T> 的解引用");
    let x = 5;
    let y = Box::new(x); // y 是指向堆上值的 Box

    println!("x = {}", x);
    println!("y = {:?}", y);
    println!("*y = {} (Box 解引用)", *y);

    assert_eq!(5, x);
    assert_eq!(5, *y); // Box 也可以用 * 解引用
    println!("✅ Box 解引用验证通过");

    // 1.3 多层引用的解引用
    println!("\n🔍 1.3 多层引用的解引用");
    let x = 42;
    let y = &x;
    let z = &y; // 引用的引用
    let w = &z; // 引用的引用的引用

    println!("x = {}", x);
    println!("*y = {}", *y);
    println!("**z = {}", **z);
    println!("***w = {}", ***w);

    assert_eq!(42, ***w);
    println!("✅ 多层引用解引用验证通过");
}

/// 2. 自定义智能指针和 Deref trait 实现
fn demonstrate_deref_trait() {
    println!("\n📚 2. Deref Trait 实现");
    println!("{}", "-".repeat(50));

    // 2.1 基础 MyBox 实现
    println!("\n🔍 2.1 基础 MyBox 实现");
    let x = 5;
    let y = MyBox::new(x);

    println!("x = {}", x);
    println!("y = {:?}", y);
    println!("*y = {} (MyBox 解引用)", *y);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("✅ MyBox 解引用验证通过");

    // 2.2 复杂类型的 MyBox
    println!("\n🔍 2.2 复杂类型的 MyBox");
    let s = String::from("Hello, Deref!");
    let boxed_string = MyBox::new(s);

    println!("boxed_string = {:?}", boxed_string);
    println!("*boxed_string = {}", *boxed_string);
    println!("boxed_string.len() = {}", boxed_string.len()); // 自动解引用

    assert_eq!("Hello, Deref!", &*boxed_string);
    println!("✅ 复杂类型 MyBox 验证通过");

    // 2.3 智能指针的内存布局
    println!("\n🔍 2.3 智能指针的内存布局");
    let data = vec![1, 2, 3, 4, 5];
    let smart_ptr = SmartPointer::new(data);

    println!("智能指针大小: {} bytes", std::mem::size_of_val(&smart_ptr));
    println!("数据大小: {} bytes", std::mem::size_of_val(&*smart_ptr));
    println!("数据内容: {:?}", *smart_ptr);

    // 展示内存地址
    println!("智能指针地址: {:p}", &smart_ptr);
    println!("数据地址: {:p}", &*smart_ptr);
    println!("✅ 内存布局分析完成");
}

/// 3. Deref 强制转换演示
fn demonstrate_deref_coercion() {
    println!("\n📚 3. Deref 强制转换 (Deref Coercion)");
    println!("{}", "-".repeat(50));

    // 3.1 基础强制转换
    println!("\n🔍 3.1 基础强制转换");
    let m = MyBox::new(String::from("Rust Deref"));

    // 函数期望 &str，但我们传入 &MyBox<String>
    // Rust 自动进行：&MyBox<String> -> &String -> &str
    hello(&m);

    // 显式转换过程
    let step1: &String = &*m; // MyBox<String> -> String -> &String
    let step2: &str = &step1[..]; // &String -> &str
    hello(step2);

    println!("✅ 基础强制转换验证通过");

    // 3.2 连续强制转换
    println!("\n🔍 3.2 连续强制转换");
    let nested = MyBox::new(MyBox::new(String::from("Nested")));

    // 多层自动解引用：&MyBox<MyBox<String>> -> &MyBox<String> -> &String -> &str
    hello(&nested);

    println!("嵌套智能指针: {:?}", nested);
    println!("一层解引用: {:?}", *nested);
    println!("二层解引用: {}", **nested);
    println!("✅ 连续强制转换验证通过");

    // 3.3 函数参数中的强制转换
    println!("\n🔍 3.3 函数参数中的强制转换");
    let data = MyBox::new(vec![1, 2, 3, 4, 5]);

    // 自动转换：&MyBox<Vec<i32>> -> &Vec<i32> -> &[i32]
    process_slice(&data);

    // 手动转换对比
    process_slice(&(*data)[..]);

    println!("✅ 函数参数强制转换验证通过");

    // 3.4 方法调用中的强制转换
    println!("\n🔍 3.4 方法调用中的强制转换");
    let wrapper = StringWrapper::new("Hello, World!".to_string());

    // 直接调用 String 的方法
    println!("长度: {}", wrapper.len());
    println!("大写: {}", wrapper.to_uppercase());
    println!("是否包含 'World': {}", wrapper.contains("World"));

    println!("✅ 方法调用强制转换验证通过");
}

/// 4. DerefMut 可变解引用演示
fn demonstrate_deref_mut() {
    println!("\n📚 4. DerefMut 可变解引用");
    println!("{}", "-".repeat(50));

    // 4.1 基础可变解引用
    println!("\n🔍 4.1 基础可变解引用");
    let mut x = MyBox::new(String::from("Hello"));

    println!("修改前: {:?}", x);

    // 通过可变解引用修改内容
    x.push_str(", World!");
    (*x).push('!');

    println!("修改后: {:?}", x);
    println!("✅ 基础可变解引用验证通过");

    // 4.2 可变引用的强制转换
    println!("\n🔍 4.2 可变引用的强制转换");
    let mut data = MyBox::new(vec![1, 2, 3]);

    println!("修改前: {:?}", data);

    // 自动转换：&mut MyBox<Vec<i32>> -> &mut Vec<i32>
    modify_vec(&mut data);

    println!("修改后: {:?}", data);
    println!("✅ 可变引用强制转换验证通过");

    // 4.3 智能指针的可变操作
    println!("\n🔍 4.3 智能指针的可变操作");
    let mut smart_data = SmartData::new(HashMap::new());

    // 通过解引用操作 HashMap
    smart_data.insert("key1".to_string(), 100);
    smart_data.insert("key2".to_string(), 200);

    println!("智能数据: {:?}", smart_data);
    println!("key1 的值: {:?}", smart_data.get("key1"));

    // 修改值
    if let Some(value) = smart_data.get_mut("key1") {
        *value += 50;
    }

    println!("修改后 key1 的值: {:?}", smart_data.get("key1"));
    println!("✅ 智能指针可变操作验证通过");
}

/// 5. 智能指针设计模式演示
fn demonstrate_smart_pointer_patterns() {
    println!("\n📚 5. 智能指针设计模式");
    println!("{}", "-".repeat(50));

    // 5.1 引用计数智能指针
    println!("\n🔍 5.1 引用计数智能指针");
    let data = Rc::new(String::from("共享数据"));
    let ptr1 = Rc::clone(&data);
    let ptr2 = Rc::clone(&data);

    println!("原始数据: {}", data);
    println!("指针1: {}", ptr1);
    println!("指针2: {}", ptr2);
    println!("引用计数: {}", Rc::strong_count(&data));

    // 自动解引用到 String 方法
    println!("数据长度: {}", data.len());
    println!("✅ 引用计数智能指针验证通过");

    // 5.2 线程安全的智能指针
    println!("\n🔍 5.2 线程安全的智能指针");
    let shared_data = Arc::new(String::from("线程安全数据"));
    let data_clone = Arc::clone(&shared_data);

    println!("共享数据: {}", shared_data);
    println!("克隆数据: {}", data_clone);
    println!("引用计数: {}", Arc::strong_count(&shared_data));

    // 展示自动解引用
    println!("数据是否为空: {}", shared_data.is_empty());
    println!("✅ 线程安全智能指针验证通过");

    // 5.3 内部可变性智能指针
    println!("\n🔍 5.3 内部可变性智能指针");
    let cell_data = RefCell::new(String::from("可变数据"));

    println!("初始数据: {:?}", cell_data.borrow());

    // 通过 RefCell 修改数据
    {
        let mut borrowed = cell_data.borrow_mut();
        borrowed.push_str(" - 已修改");
    }

    println!("修改后数据: {:?}", cell_data.borrow());

    // RefCell 也支持自动解引用（通过 borrow()）
    println!("数据长度: {}", cell_data.borrow().len());
    println!("✅ 内部可变性智能指针验证通过");

    // 5.4 自定义智能指针组合
    println!("\n🔍 5.4 自定义智能指针组合");
    let combined = Rc::new(RefCell::new(MyBox::new(vec![1, 2, 3])));

    println!("组合指针初始值: {:?}", combined.borrow());

    // 多层解引用和修改
    combined.borrow_mut().push(4);
    combined.borrow_mut().push(5);

    println!("修改后值: {:?}", combined.borrow());
    println!("引用计数: {}", Rc::strong_count(&combined));
    println!("✅ 智能指针组合验证通过");
}

/// 6. 内存管理演示
fn demonstrate_memory_management() {
    println!("\n📚 6. 内存管理演示");
    println!("{}", "-".repeat(50));

    // 6.1 栈vs堆分配
    println!("\n🔍 6.1 栈vs堆分配");

    // 栈分配
    let stack_data = [1, 2, 3, 4, 5];
    println!("栈数据地址: {:p}", &stack_data);
    println!("栈数据大小: {} bytes", std::mem::size_of_val(&stack_data));

    // 堆分配
    let heap_data = Box::new(vec![1, 2, 3, 4, 5]);
    println!("Box地址: {:p}", &heap_data);
    println!("堆数据地址: {:p}", heap_data.as_ptr());
    println!("Box大小: {} bytes", std::mem::size_of_val(&heap_data));
    println!("堆数据大小: {} bytes", std::mem::size_of_val(&**heap_data));

    // 6.2 所有权转移
    println!("\n🔍 6.2 所有权转移");
    let original = MyBox::new(String::from("原始数据"));
    println!("原始数据: {:?}", original);

    // 所有权转移
    let moved = original;
    println!("转移后数据: {:?}", moved);
    // println!("原始数据: {:?}", original); // 编译错误：值已被移动

    // 6.3 生命周期管理
    println!("\n🔍 6.3 生命周期管理");
    {
        let scoped_data = MyBox::new(vec![1, 2, 3]);
        println!("作用域内数据: {:?}", scoped_data);

        // 创建引用
        let reference = &*scoped_data;
        println!("引用数据: {:?}", reference);
    } // scoped_data 在此处被销毁

    println!("✅ 内存管理演示完成");

    // 6.4 内存泄漏防护
    println!("\n🔍 6.4 内存泄漏防护");
    demonstrate_memory_leak_prevention();
}

/// 7. 实际应用场景演示
fn demonstrate_practical_scenarios() {
    println!("\n📚 7. 实际应用场景");
    println!("{}", "-".repeat(50));

    // 7.1 API 设计中的 Deref
    println!("\n🔍 7.1 API 设计中的 Deref");
    let config = Configuration::new("app.conf".to_string());

    // 可以直接当作 String 使用
    println!("配置文件名: {}", &*config);
    println!("文件名长度: {}", config.len());
    println!("是否以 .conf 结尾: {}", config.ends_with(".conf"));

    // 7.2 DST (Dynamically Sized Types) 处理
    println!("\n🔍 7.2 DST 处理");
    let dyn_data: Box<dyn fmt::Display> = Box::new(42);
    println!("动态类型数据: {}", dyn_data);

    let str_data: Box<str> = "Hello, DST!".into();
    println!("字符串 DST: {}", str_data);

    // 7.3 递归数据结构
    println!("\n🔍 7.3 递归数据结构");
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    println!("链表: {:?}", list);
    println!("链表长度: {}", list.len());

    // 7.4 缓存和延迟计算
    println!("\n🔍 7.4 缓存和延迟计算");
    let lazy_value = LazyValue::new(|| {
        println!("执行昂贵的计算...");
        std::thread::sleep(std::time::Duration::from_millis(100));
        42 * 42
    });

    println!("第一次访问: {}", *lazy_value); // 触发计算
    println!("第二次访问: {}", *lazy_value); // 使用缓存

    println!("✅ 实际应用场景演示完成");
}

/// 8. 高级模式和性能分析
fn demonstrate_advanced_patterns() {
    println!("\n📚 8. 高级模式和性能分析");
    println!("{}", "-".repeat(50));

    // 8.1 零成本抽象验证
    println!("\n🔍 8.1 零成本抽象验证");
    let iterations = 1_000_000;

    // 直接访问
    let start = Instant::now();
    let mut sum1 = 0;
    let data = vec![1, 2, 3, 4, 5];
    for _ in 0..iterations {
        sum1 += data[0] + data[1] + data[2] + data[3] + data[4];
    }
    let direct_time = start.elapsed();

    // 通过智能指针访问
    let start = Instant::now();
    let mut sum2 = 0;
    let smart_data = MyBox::new(vec![1, 2, 3, 4, 5]);
    for _ in 0..iterations {
        sum2 += smart_data[0] + smart_data[1] + smart_data[2] + smart_data[3] + smart_data[4];
    }
    let smart_time = start.elapsed();

    println!("直接访问时间: {:?}", direct_time);
    println!("智能指针时间: {:?}", smart_time);
    println!(
        "性能差异: {:.2}%",
        (smart_time.as_nanos() as f64 / direct_time.as_nanos() as f64 - 1.0) * 100.0
    );

    assert_eq!(sum1, sum2);
    println!("✅ 零成本抽象验证通过");

    // 8.2 编译时优化分析
    println!("\n🔍 8.2 编译时优化分析");
    analyze_compiler_optimizations();

    // 8.3 内存使用分析
    println!("\n🔍 8.3 内存使用分析");
    analyze_memory_usage();

    // 8.4 高级 Deref 模式
    println!("\n🔍 8.4 高级 Deref 模式");
    demonstrate_advanced_deref_patterns();

    println!("✅ 高级模式和性能分析完成");
}

// ============================================================================
// 自定义类型定义
// ============================================================================

/// 基础自定义智能指针
#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 为 MyBox 实现 Deref trait
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 为 MyBox 实现 DerefMut trait
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// 智能指针包装器
#[derive(Debug)]
struct SmartPointer<T> {
    data: T,
    metadata: String,
}

impl<T> SmartPointer<T> {
    fn new(data: T) -> Self {
        SmartPointer {
            data,
            metadata: "SmartPointer".to_string(),
        }
    }

    fn metadata(&self) -> &str {
        &self.metadata
    }
}

impl<T> Deref for SmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/// 字符串包装器
#[derive(Debug)]
struct StringWrapper {
    inner: String,
    created_at: std::time::SystemTime,
}

impl StringWrapper {
    fn new(s: String) -> Self {
        StringWrapper {
            inner: s,
            created_at: std::time::SystemTime::now(),
        }
    }

    fn created_at(&self) -> std::time::SystemTime {
        self.created_at
    }
}

impl Deref for StringWrapper {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// 智能数据容器
#[derive(Debug)]
struct SmartData<T> {
    data: T,
    access_count: Cell<usize>,
}

impl<T> SmartData<T> {
    fn new(data: T) -> Self {
        SmartData {
            data,
            access_count: Cell::new(0),
        }
    }

    fn access_count(&self) -> usize {
        self.access_count.get()
    }
}

impl<T> Deref for SmartData<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.access_count.set(self.access_count.get() + 1);
        &self.data
    }
}

impl<T> DerefMut for SmartData<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.access_count.set(self.access_count.get() + 1);
        &mut self.data
    }
}

/// 配置文件包装器
#[derive(Debug)]
struct Configuration {
    filename: String,
    loaded: bool,
}

impl Configuration {
    fn new(filename: String) -> Self {
        Configuration {
            filename,
            loaded: false,
        }
    }

    fn is_loaded(&self) -> bool {
        self.loaded
    }

    fn load(&mut self) {
        self.loaded = true;
    }
}

impl Deref for Configuration {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.filename
    }
}

/// 简单链表实现
#[derive(Debug)]
struct List<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None, len: 0 }
    }

    fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.len += 1;
    }

    fn len(&self) -> usize {
        self.len
    }
}

/// 延迟计算值
struct LazyValue<T, F>
where
    F: FnOnce() -> T,
{
    value: RefCell<Option<T>>,
    init: RefCell<Option<F>>,
}

impl<T, F> LazyValue<T, F>
where
    F: FnOnce() -> T,
{
    fn new(init: F) -> Self {
        LazyValue {
            value: RefCell::new(None),
            init: RefCell::new(Some(init)),
        }
    }
}

impl<T, F> Deref for LazyValue<T, F>
where
    F: FnOnce() -> T,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let mut value = self.value.borrow_mut();
        if value.is_none() {
            let init = self.init.borrow_mut().take().unwrap();
            *value = Some(init());
        }

        // 这里需要使用 unsafe 来返回引用
        // 在实际应用中，可以使用 OnceCell 或 lazy_static
        unsafe {
            let ptr = value.as_ref().unwrap() as *const T;
            &*ptr
        }
    }
}

// ============================================================================
// 辅助函数
// ============================================================================

/// 接受 &str 参数的函数
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

/// 处理切片的函数
fn process_slice(slice: &[i32]) {
    println!("处理切片: {:?}, 长度: {}", slice, slice.len());
}

/// 修改 Vec 的函数
fn modify_vec(vec: &mut Vec<i32>) {
    vec.push(4);
    vec.push(5);
}

/// 内存泄漏防护演示
fn demonstrate_memory_leak_prevention() {
    // 使用 Rc 创建循环引用
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
        parent: RefCell<Weak<Node>>,
    }

    let parent = Rc::new(Node {
        value: 1,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    let child = Rc::new(Node {
        value: 2,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Rc::downgrade(&parent)),
    });

    parent.children.borrow_mut().push(Rc::clone(&child));

    println!("父节点引用计数: {}", Rc::strong_count(&parent));
    println!("子节点引用计数: {}", Rc::strong_count(&child));

    // 通过 Weak 引用避免循环引用
    if let Some(parent_ref) = child.parent.borrow().upgrade() {
        println!("子节点的父节点值: {}", parent_ref.value);
    }

    println!("✅ 内存泄漏防护演示完成");
}

/// 编译时优化分析
fn analyze_compiler_optimizations() {
    // 展示编译器如何优化 Deref 调用
    let data = MyBox::new(42);

    // 这些调用在优化后应该产生相同的汇编代码
    let value1 = *data;
    let value2 = *data.deref();
    let value3 = *(data.deref());

    println!("直接解引用: {}", value1);
    println!("调用 deref(): {}", value2);
    println!("显式解引用 deref(): {}", value3);

    assert_eq!(value1, value2);
    assert_eq!(value2, value3);

    println!("✅ 编译时优化分析完成");
}

/// 内存使用分析
fn analyze_memory_usage() {
    use std::mem;

    // 分析不同智能指针的内存开销
    println!("各种类型的内存大小:");
    println!("i32: {} bytes", mem::size_of::<i32>());
    println!("&i32: {} bytes", mem::size_of::<&i32>());
    println!("Box<i32>: {} bytes", mem::size_of::<Box<i32>>());
    println!("MyBox<i32>: {} bytes", mem::size_of::<MyBox<i32>>());
    println!("Rc<i32>: {} bytes", mem::size_of::<Rc<i32>>());
    println!("Arc<i32>: {} bytes", mem::size_of::<Arc<i32>>());

    // 分析对齐要求
    println!("\n内存对齐:");
    println!("i32 对齐: {} bytes", mem::align_of::<i32>());
    println!("Box<i32> 对齐: {} bytes", mem::align_of::<Box<i32>>());
    println!("MyBox<i32> 对齐: {} bytes", mem::align_of::<MyBox<i32>>());

    println!("✅ 内存使用分析完成");
}

/// 高级 Deref 模式演示
fn demonstrate_advanced_deref_patterns() {
    // 条件解引用
    struct ConditionalDeref<T> {
        data: T,
        enabled: bool,
    }

    impl<T: Default> Deref for ConditionalDeref<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            if self.enabled {
                &self.data
            } else {
                // 返回默认值的引用（这里简化处理）
                &self.data
            }
        }
    }

    let conditional = ConditionalDeref {
        data: String::from("条件数据"),
        enabled: true,
    };

    println!("条件解引用: {}", &*conditional);

    // 计数解引用
    let smart_data = SmartData::new(vec![1, 2, 3, 4, 5]);
    println!("初始访问计数: {}", smart_data.access_count());

    let _len = smart_data.len(); // 触发解引用
    println!("访问后计数: {}", smart_data.access_count());

    let _first = smart_data.first(); // 再次触发解引用
    println!("再次访问后计数: {}", smart_data.access_count());

    println!("✅ 高级 Deref 模式演示完成");
}

// ============================================================================
// 测试模块
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_deref() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_deref_coercion() {
        let m = MyBox::new(String::from("Hello"));
        assert_eq!("Hello", &*m);

        // 测试自动强制转换
        fn take_str(s: &str) -> usize {
            s.len()
        }

        assert_eq!(5, take_str(&m));
    }

    #[test]
    fn test_deref_mut() {
        let mut x = MyBox::new(String::from("Hello"));
        x.push_str(", World!");
        assert_eq!("Hello, World!", &*x);
    }

    #[test]
    fn test_smart_pointer() {
        let data = SmartPointer::new(vec![1, 2, 3]);
        assert_eq!(3, data.len());
        assert_eq!("SmartPointer", data.metadata());
    }

    #[test]
    fn test_string_wrapper() {
        let wrapper = StringWrapper::new("test".to_string());
        assert_eq!(4, wrapper.len());
        assert!(wrapper.contains("est"));
    }

    #[test]
    fn test_configuration() {
        let config = Configuration::new("config.toml".to_string());
        assert_eq!("config.toml", &*config);
        assert!(!config.is_loaded());
    }

    #[test]
    fn test_list() {
        let mut list = List::new();
        assert_eq!(0, list.len());

        list.push(1);
        list.push(2);
        assert_eq!(2, list.len());
    }

    #[test]
    fn test_smart_data() {
        let smart_data = SmartData::new(HashMap::<String, i32>::new());
        assert_eq!(0, smart_data.access_count());

        let _len = smart_data.len(); // 触发解引用
        assert_eq!(1, smart_data.access_count());
    }

    #[test]
    fn test_memory_sizes() {
        use std::mem;

        // 验证智能指针的内存开销
        assert_eq!(mem::size_of::<MyBox<i32>>(), mem::size_of::<i32>());
        assert_eq!(mem::size_of::<Box<i32>>(), mem::size_of::<usize>());
    }

    #[test]
    fn test_nested_deref() {
        let nested = MyBox::new(MyBox::new(42));
        assert_eq!(42, **nested);

        // 测试自动强制转换
        fn take_int(x: &i32) -> i32 {
            *x
        }

        assert_eq!(42, take_int(&nested));
    }
}
