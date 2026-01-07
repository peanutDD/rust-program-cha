//! # 边界情况分析模块
//!
//! 本模块深入分析 Rust 中作用域、生命周期和 NLL 的边界情况、特殊情况和常见陷阱。
//! 通过具体的代码示例展示这些边界情况，帮助开发者避免常见错误。

// 示例代码模块，允许未使用的代码
#![allow(dead_code)]

// 以下导入用于示例代码演示，部分可能未在当前模块直接使用
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::rc::Rc;
#[allow(unused_imports)]
use std::cell::RefCell;
#[allow(unused_imports)]
use std::sync::{Arc, Mutex};
#[allow(unused_imports)]
use std::thread;
#[allow(unused_imports)]
use std::time::Duration;

// 添加process_static_data函数定义
fn process_static_data(data: &'static str) -> String {
    format!("处理静态数据: {}", data)
}

/// 运行所有边界情况分析
pub fn run_edge_cases_analysis() {
    println!("\n🚨 边界情况分析 - 作用域、生命周期和 NLL 的陷阱与特殊情况");
    println!("{}", "=".repeat(80));
    
    scope_edge_cases();
    lifetime_edge_cases();
    nll_edge_cases();
    interaction_edge_cases();
    memory_safety_edge_cases();
    performance_edge_cases();
    debugging_edge_cases();
}

/// 1. 作用域边界情况
fn scope_edge_cases() {
    println!("\n🔍 1. 作用域边界情况");
    println!("分析作用域中的特殊情况和常见陷阱。");
    
    variable_shadowing_traps();
    closure_capture_issues();
    temporary_lifetime_issues();
    drop_order_surprises();
}

/// 变量遮蔽陷阱
fn variable_shadowing_traps() {
    println!("\n👥 变量遮蔽陷阱:");
    
    // 陷阱1: 意外的变量遮蔽
    {
        let x = 10;
        println!("   📍 外层 x = {}", x);
        
        {
            let x = "hello"; // 遮蔽了外层的 x
            println!("   📍 内层 x = {}", x);
            
            // 这里无法访问外层的数值 x
            // let sum = x + 5; // 编译错误！
        }
        
        println!("   📍 回到外层 x = {}", x);
    }
    
    // 陷阱2: 循环中的变量遮蔽
    {
        let mut results = Vec::new();
        
        for i in 0..3 {
            let i = i * 2; // 遮蔽了循环变量 i
            results.push(i);
        }
        
        println!("   📊 循环结果: {:?}", results); // [0, 2, 4]
    }
    
    // 陷阱3: 条件分支中的遮蔽
    {
        let value = Some(42);
        
        if let Some(value) = value {
            // 这里的 value 是 i32，遮蔽了外层的 Option<i32>
            println!("   🎯 解包后的值: {}", value);
            
            // 无法再访问原始的 Option
            // let is_some = value.is_some(); // 编译错误！
        }
    }
    
    println!("   ⚠️ 变量遮蔽可能导致意外的行为和难以调试的问题");
}

/// 闭包捕获问题
fn closure_capture_issues() {
    println!("\n🎣 闭包捕获问题:");
    
    // 问题1: 意外的移动捕获
    {
        let data = vec![1, 2, 3, 4, 5];
        
        let closure = move || {
            println!("   📦 闭包中的数据: {:?}", data);
        };
        
        closure();
        
        // data 已经被移动到闭包中
        // println!("外部数据: {:?}", data); // 编译错误！
    }
    
    // 问题2: 引用捕获的生命周期问题
    {
        let mut closures = Vec::new();
        
        for i in 0..3 {
            // 这种写法会有问题
            // let closure = || println!("值: {}", i); // i 的生命周期问题
            
            // 正确的写法：捕获值而不是引用
            let closure = move || println!("   🔢 值: {}", i);
            closures.push(Box::new(closure) as Box<dyn Fn()>);
        }
        
        for closure in closures {
            closure();
        }
    }
    
    // 问题3: 可变引用捕获冲突
    {
        let mut counter = 0;
        
        {
            let mut increment = || {
                counter += 1;
            };
            
            increment();
            increment();
            
            // 在闭包存在期间，不能直接访问 counter
            // println!("计数器: {}", counter); // 编译错误！
        }
        
        println!("   📈 最终计数器: {}", counter);
    }
    
    println!("   ⚠️ 闭包捕获需要仔细考虑所有权和生命周期");
}

/// 临时值生命周期问题
fn temporary_lifetime_issues() {
    println!("\n⏱️ 临时值生命周期问题:");
    
    // 问题1: 临时值的引用
    {
        // 这种写法是危险的
        // let r = &String::from("hello"); // 临时值在语句结束后被销毁
        
        // 正确的写法
        let s = String::from("hello");
        let r = &s;
        println!("   📝 字符串引用: {}", r);
    }
    
    // 问题2: 函数返回值的临时引用
    fn get_string() -> String {
        "temporary".to_string()
    }
    
    {
        // 危险的写法
        // let r = &get_string(); // 临时值立即被销毁
        
        // 正确的写法
        let s = get_string();
        let r = &s;
        println!("   🔄 函数返回值引用: {}", r);
    }
    
    // 问题3: 结构体字段的临时引用
    #[derive(Debug)]
    struct Container {
        data: String,
    }
    
    impl Container {
        fn get_data(&self) -> &str {
            &self.data
        }
        
        fn create_temp() -> Container {
            Container {
                data: "temporary data".to_string(),
            }
        }
    }
    
    {
        // 危险的写法
        // let data_ref = Container::create_temp().get_data(); // 临时对象被销毁
        
        // 正确的写法
        let container = Container::create_temp();
        let data_ref = container.get_data();
        println!("   📦 容器数据: {}", data_ref);
    }
    
    println!("   ⚠️ 临时值的生命周期只持续到语句结束");
}

/// Drop 顺序意外情况
fn drop_order_surprises() {
    println!("\n🗑️ Drop 顺序意外情况:");
    
    struct DropTracker {
        name: String,
    }
    
    impl Drop for DropTracker {
        fn drop(&mut self) {
            println!("   🗑️ 销毁: {}", self.name);
        }
    }
    
    // 情况1: 变量声明顺序影响销毁顺序
    {
        println!("   📋 变量声明顺序测试:");
        let _first = DropTracker { name: "第一个".to_string() };
        let _second = DropTracker { name: "第二个".to_string() };
        let _third = DropTracker { name: "第三个".to_string() };
        // 销毁顺序与声明顺序相反
    }
    
    // 情况2: 结构体字段的销毁顺序
    {
        println!("   🏗️ 结构体字段销毁顺序:");
        
        struct Container {
            first: DropTracker,
            second: DropTracker,
        }
        
        impl Drop for Container {
            fn drop(&mut self) {
                println!("   📦 销毁容器");
            }
        }
        
        let _container = Container {
            first: DropTracker { name: "字段1".to_string() },
            second: DropTracker { name: "字段2".to_string() },
        };
        // 先销毁容器，再按字段声明的逆序销毁字段
    }
    
    // 情况3: 函数参数的销毁顺序
    {
        println!("   🔧 函数参数销毁顺序:");
        
        fn process_items(_a: DropTracker, _b: DropTracker) {
            println!("   ⚙️ 处理项目");
        }
        
        process_items(
            DropTracker { name: "参数1".to_string() },
            DropTracker { name: "参数2".to_string() },
        );
        // 参数按从右到左的顺序销毁
    }
    
    println!("   ⚠️ Drop 顺序可能影响程序的正确性，特别是在资源管理中");
}

/// 2. 生命周期边界情况
fn lifetime_edge_cases() {
    println!("\n⏳ 2. 生命周期边界情况");
    println!("分析生命周期中的复杂情况和陷阱。");
    
    lifetime_elision_traps();
    higher_ranked_lifetime_issues();
    self_referential_structs();
    lifetime_variance_issues();
}

/// 生命周期省略陷阱
fn lifetime_elision_traps() {
    println!("\n🎭 生命周期省略陷阱:");
    
    // 陷阱1: 多个输入参数的省略规则
    {
        // 这个函数的生命周期省略可能不是你想要的
        fn first_word(s: &str) -> &str {
            s.split_whitespace().next().unwrap_or("")
        }
        
        let sentence = "hello world";
        let word = first_word(sentence);
        println!("   📝 第一个单词: {}", word);
        
        // 这个函数需要显式生命周期标注
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() { x } else { y }
        }
        
        let str1 = "short";
        let str2 = "longer string";
        let result = longest(str1, str2);
        println!("   📏 更长的字符串: {}", result);
    }
    
    // 陷阱2: 结构体方法中的生命周期省略
    {
        struct TextProcessor {
            prefix: String,
        }
        
        impl TextProcessor {
            // 这里的生命周期省略可能导致混淆
            fn process<'a>(&self, input: &'a str) -> &'a str {
                // 返回值的生命周期与 input 绑定，而不是 self
                input.trim()
            }
            
            // 如果想返回 self 的引用，需要显式标注
            fn get_prefix(&self) -> &str {
                &self.prefix
            }
        }
        
        let processor = TextProcessor {
            prefix: "PREFIX:".to_string(),
        };
        
        let input = "  hello world  ";
        let processed = processor.process(input);
        println!("   🔄 处理后: '{}'", processed);
        
        let prefix = processor.get_prefix();
        println!("   🏷️ 前缀: {}", prefix);
    }
    
    println!("   ⚠️ 生命周期省略规则可能不总是符合预期");
}

/// 高阶生命周期问题
fn higher_ranked_lifetime_issues() {
    println!("\n🎯 高阶生命周期问题:");
    
    // 问题1: for<'a> 语法的使用
    {
        // 定义一个接受任意生命周期闭包的函数
        fn call_with_ref<F>(f: F) 
        where
            F: for<'a> Fn(&'a str) -> &'a str,
        {
            let s = "test string";
            let result = f(s);
            println!("   🔧 闭包结果: {}", result);
        }
        
        // 使用高阶生命周期
        call_with_ref(|s| s.trim());
        call_with_ref(|s| &s[0..4]);
    }
    
    // 问题2: 复杂的生命周期约束
    {
        trait Process {
            fn process<'a>(&self, input: &'a str) -> &'a str;
        }
        
        struct Trimmer;
        
        impl Process for Trimmer {
            fn process<'a>(&self, input: &'a str) -> &'a str {
                input.trim()
            }
        }
        
        fn apply_processor<'a, P>(processor: &P, inputs: &'a [&'a str]) -> Vec<&'a str>
        where
            P: Process,
        {
            inputs.iter().map(|&input| processor.process(input)).collect()
        }
        
        let trimmer = Trimmer;
        let inputs = vec!["  hello  ", "  world  ", "  rust  "];
        let results = apply_processor(&trimmer, &inputs);
        println!("   📋 处理结果: {:?}", results);
    }
    
    println!("   ⚠️ 高阶生命周期在复杂泛型场景中可能难以理解");
}

/// 自引用结构体问题
fn self_referential_structs() {
    println!("\n🔄 自引用结构体问题:");
    
    // 问题：Rust 不允许直接的自引用结构体
    // struct SelfRef {
    //     data: String,
    //     reference: &str, // 无法引用同一结构体中的 data
    // }
    
    // 解决方案1: 使用 Pin 和 unsafe
    {
        use std::pin::Pin;
        
        struct SelfReferential {
            data: String,
            // 使用原始指针避免生命周期问题
            reference: *const str,
        }
        
        impl SelfReferential {
            fn new(data: String) -> Pin<Box<Self>> {
                let mut boxed = Box::pin(SelfReferential {
                    reference: std::ptr::slice_from_raw_parts(std::ptr::null::<u8>(), 0) as *const str,
                    data,
                });
                
                // 安全地设置自引用
                unsafe {
                    let ptr = boxed.as_mut().get_unchecked_mut();
                    ptr.reference = std::ptr::slice_from_raw_parts(ptr.data.as_ptr(), ptr.data.len()) as *const str;
                }
                
                boxed
            }
            
            fn get_reference(&self) -> &str {
                unsafe {
                    std::str::from_utf8_unchecked(
                        std::slice::from_raw_parts(
                            self.reference as *const u8,
                            self.data.len(),
                        )
                    )
                }
            }
        }
        
        let self_ref = SelfReferential::new("hello self-reference".to_string());
        println!("   🔗 自引用数据: {}", self_ref.get_reference());
    }
    
    // 解决方案2: 使用 Rc 和 RefCell
    {
        use std::rc::{Rc, Weak};
        use std::cell::RefCell;
        
        struct Node {
            data: i32,
            parent: Option<Weak<RefCell<Node>>>,
            children: Vec<Rc<RefCell<Node>>>,
        }
        
        impl Node {
            fn new(data: i32) -> Rc<RefCell<Self>> {
                Rc::new(RefCell::new(Node {
                    data,
                    parent: None,
                    children: Vec::new(),
                }))
            }
            
            fn add_child(parent: &Rc<RefCell<Node>>, child_data: i32) -> Rc<RefCell<Node>> {
                let child = Node::new(child_data);
                child.borrow_mut().parent = Some(Rc::downgrade(parent));
                parent.borrow_mut().children.push(child.clone());
                child
            }
        }
        
        let root = Node::new(1);
        let child1 = Node::add_child(&root, 2);
        let child2 = Node::add_child(&root, 3);
        let _grandchild = Node::add_child(&child1, 4);
        
        println!("   🌳 根节点数据: {}", root.borrow().data);
        println!("   🌿 子节点数量: {}", root.borrow().children.len());
        
        // 验证父子关系
        {
            let borrowed_child2 = child2.borrow();
            if let Some(parent) = borrowed_child2.parent.as_ref() {
                if let Some(parent) = parent.upgrade() {
                    println!("   👨‍👦 子节点 {} 的父节点: {}", borrowed_child2.data, parent.borrow().data);
                }
            }
        }
    }
    
    println!("   ⚠️ 自引用结构体需要特殊的设计模式来避免生命周期问题");
}

/// 生命周期变异问题
fn lifetime_variance_issues() {
    println!("\n🔄 生命周期变异问题:");
    
    // 协变性示例
    {
        // &'a T 对于 'a 是协变的
        fn covariance_example() {
            let long_lived = String::from("long lived");
            
            {
                let _short_lived = String::from("short");
                
                // 可以将长生命周期的引用赋给短生命周期的变量
                let _: &str = &long_lived; // 'long -> 'short 是允许的
                
                // 但不能反过来
                // let long_ref: &'long str = &short_lived; // 编译错误
                
                println!("   ↗️ 协变性：长生命周期可以转换为短生命周期");
            }
        }
        
        covariance_example();
    }
    
    // 逆变性示例（函数参数）
    {
        // fn(&'a T) 对于 'a 是逆变的
        fn contravariance_example() {
            fn process_short(_: &str) {
                println!("   ↙️ 处理短生命周期字符串");
            }
            
            fn process_long(_: &str) {
                println!("   ↙️ 处理长生命周期字符串");
            }
            
            // 可以将接受短生命周期的函数用于长生命周期
            let _: fn(&str) = process_short;
            let _: fn(&str) = process_long;
            
            println!("   ↙️ 逆变性：接受短生命周期的函数可以处理长生命周期");
        }
        
        contravariance_example();
    }
    
    // 不变性示例（可变引用）
    {
        // &'a mut T 对于 'a 是不变的
        fn invariance_example() {
            let mut long_lived = String::from("mutable");
            
            {
                let mut short_lived = String::from("short");
                
                // 可变引用必须精确匹配生命周期
                let long_ref: &mut String = &mut long_lived;
                let short_ref: &mut String = &mut short_lived;
                
                // 不能将不同生命周期的可变引用互相赋值
                // let _: &mut String = long_ref; // 如果类型不匹配会编译错误
                
                long_ref.push_str(" modified");
                short_ref.push_str(" too");
                
                println!("   ↔️ 不变性：可变引用的生命周期必须精确匹配");
            }
        }
        
        invariance_example();
    }
    
    println!("   ⚠️ 生命周期变异规则影响类型系统的灵活性和安全性");
}

/// 3. NLL 边界情况
fn nll_edge_cases() {
    println!("\n🆕 3. NLL 边界情况");
    println!("分析非词法生命周期中的特殊情况。");
    
    nll_limitations();
    nll_interaction_with_closures();
    nll_and_async_issues();
    nll_performance_edge_cases();
}

/// NLL 的限制
fn nll_limitations() {
    println!("\n🚧 NLL 的限制:");
    
    // 限制1: 复杂的控制流
    {
        fn complex_control_flow(condition: bool) -> &'static str {
            let data = vec![1, 2, 3, 4, 5];
            
            if condition {
                // NLL 在某些复杂控制流中仍有限制
                return "early return";
            }
            
            // 这种情况 NLL 可能无法完全优化
            match data.len() {
                0 => "empty",
                1..=3 => "small",
                _ => "large",
            }
        }
        
        let result = complex_control_flow(false);
        println!("   🔀 复杂控制流结果: {}", result);
    }
    
    // 限制2: 跨函数边界的优化
    {
        struct DataProcessor {
            buffer: Vec<i32>,
        }
        
        impl DataProcessor {
            fn new() -> Self {
                DataProcessor {
                    buffer: vec![1, 2, 3, 4, 5],
                }
            }
            
            // NLL 无法跨函数边界优化
            fn get_first(&self) -> Option<&i32> {
                self.buffer.first()
            }
            
            fn process_data(&mut self) {
                // 即使使用 NLL，这里仍然需要小心处理借用
                if let Some(first) = self.get_first() {
                    println!("   🔢 第一个元素: {}", first);
                    // 不能在这里修改 buffer，因为 first 仍然借用着它
                    // self.buffer.push(6); // 编译错误
                }
                
                // 在这里可以修改，因为借用已经结束
                self.buffer.push(6);
                println!("   📈 添加元素后长度: {}", self.buffer.len());
            }
        }
        
        let mut processor = DataProcessor::new();
        processor.process_data();
    }
    
    // 限制3: 泛型和 trait 对象
    {
        trait Processor {
            fn process<'a>(&self, data: &'a str) -> &'a str;
        }
        
        struct SimpleProcessor;
        
        impl Processor for SimpleProcessor {
            fn process<'a>(&self, data: &'a str) -> &'a str {
                data.trim()
            }
        }
        
        fn use_processor<'a>(processor: &'a dyn Processor, data: &'a str) -> &'a str {
            // NLL 在 trait 对象中的优化有限
            processor.process(data)
        }
        
        let processor = SimpleProcessor;
        let result = use_processor(&processor, "  hello  ");
        println!("   🔧 处理器结果: '{}'", result);
    }
    
    println!("   ⚠️ NLL 虽然改进了很多情况，但仍有一些限制");
}

/// NLL 与闭包的交互
fn nll_interaction_with_closures() {
    println!("\n🎣 NLL 与闭包的交互:");
    
    // 情况1: 闭包中的借用优化
    {
        let mut data = vec![1, 2, 3, 4, 5];
        
        // NLL 允许这种模式
        let result = {
            let first = data.first().unwrap();
            *first * 2
        };
        
        // 现在可以修改 data，因为借用已经结束
        data.push(result);
        println!("   📊 数据: {:?}", data);
    }
    
    // 情况2: 闭包捕获的精确分析
    {
        let mut counter = 0;
        let mut items = vec!["a", "b", "c"];
        
        // NLL 能够精确分析闭包捕获
        items.iter().for_each(|item| {
            println!("   📝 项目: {}", item);
            counter += 1; // 可以修改 counter
        });
        
        // 闭包结束后可以修改 items
        items.push("d");
        println!("   📈 计数: {}, 项目数: {}", counter, items.len());
    }
    
    // 情况3: 复杂闭包场景
    {
        let mut map: HashMap<String, Vec<i32>> = HashMap::new();
        map.insert("numbers".to_string(), vec![1, 2, 3]);
        
        // NLL 改进了这种复杂借用场景
        if let Some(numbers) = map.get("numbers") {
            let sum: i32 = numbers.iter().sum();
            println!("   🧮 数字和: {}", sum);
            
            // 在 NLL 之前，这里可能需要额外的作用域
            map.insert("sum".to_string(), vec![sum]);
        }
        
        println!("   🗺️ 映射键: {:?}", map.keys().collect::<Vec<_>>());
    }
    
    println!("   ✅ NLL 显著改进了闭包中的借用检查");
}

/// NLL 与异步代码的问题
fn nll_and_async_issues() {
    println!("\n⚡ NLL 与异步代码的问题:");
    
    // 注意：这里只是演示概念，实际的异步代码需要运行时
    
    // 问题1: 跨 await 点的借用
    {
        struct AsyncProcessor {
            data: Vec<String>,
        }
        
        impl AsyncProcessor {
            fn new() -> Self {
                AsyncProcessor {
                    data: vec!["item1".to_string(), "item2".to_string()],
                }
            }
            
            // 模拟异步方法
            fn simulate_async_process(&self, item: &str) -> String {
                format!("processed: {}", item)
            }
            
            fn process_all(&mut self) {
                // 在真实的异步代码中，这种模式可能有问题
                for item in &self.data {
                    let result = self.simulate_async_process(item);
                    println!("   ⚡ 异步结果: {}", result);
                }
                
                // 处理完成后修改数据
                self.data.push("new_item".to_string());
            }
        }
        
        let mut processor = AsyncProcessor::new();
        processor.process_all();
    }
    
    // 问题2: Future 和生命周期
    {
        use std::future::Future;
        use std::pin::Pin;
        use std::task::{Context, Poll};
        
        struct SimpleTask {
            completed: bool,
        }
        
        impl Future for SimpleTask {
            type Output = String;
            
            fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
                if self.completed {
                    Poll::Ready("任务完成".to_string())
                } else {
                    self.completed = true;
                    Poll::Pending
                }
            }
        }
        
        let task = SimpleTask { completed: false };
        println!("   🔮 创建了模拟异步任务");
        
        // 在实际使用中，需要执行器来运行 Future
        drop(task); // 简单地丢弃任务
    }
    
    println!("   ⚠️ 异步代码中的生命周期管理比同步代码更复杂");
}

/// NLL 性能边界情况
fn nll_performance_edge_cases() {
    println!("\n🚀 NLL 性能边界情况:");
    
    // 情况1: 编译时间影响
    {
        // 复杂的借用模式可能增加编译时间
        let mut complex_data = HashMap::new();
        
        for i in 0..10 {
            complex_data.insert(i, vec![i * 2, i * 3, i * 4]);
        }
        
        // 复杂的借用和修改模式
        for (key, values) in &complex_data {
            if values.len() > 2 {
                println!("   📊 键 {} 有 {} 个值", key, values.len());
            }
        }
        
        // NLL 允许在迭代后修改
        complex_data.insert(100, vec![200, 300]);
    }
    
    // 情况2: 运行时性能
    {
        let mut large_vec: Vec<i32> = (0..1000).collect();
        
        // NLL 优化了这种模式，避免不必要的克隆
        let sum = {
            let slice = &large_vec[100..200];
            slice.iter().sum::<i32>()
        };
        
        // 可以继续修改原始向量
        large_vec.push(sum);
        
        println!("   📈 大向量长度: {}, 部分和: {}", large_vec.len(), sum);
    }
    
    // 情况3: 内存使用优化
    {
        struct LargeStruct {
            data: [i32; 1000],
        }
        
        impl LargeStruct {
            fn new() -> Self {
                LargeStruct {
                    data: [0; 1000],
                }
            }
            
            fn get_slice(&self, start: usize, end: usize) -> &[i32] {
                &self.data[start..end]
            }
        }
        
        let large_struct = LargeStruct::new();
        
        // NLL 优化了临时借用，减少内存压力
        let slice_sum = large_struct.get_slice(0, 10).iter().sum::<i32>();
        
        println!("   💾 大结构体切片和: {}", slice_sum);
    }
    
    println!("   ✅ NLL 在大多数情况下改进了性能，但复杂场景可能影响编译时间");
}

/// 4. 交互边界情况
fn interaction_edge_cases() {
    println!("\n🔄 4. 交互边界情况");
    println!("分析作用域、生命周期和 NLL 之间复杂交互的边界情况。");
    
    scope_lifetime_nll_conflicts();
    macro_expansion_issues();
    generic_lifetime_inference();
    trait_object_lifetime_issues();
}

/// 作用域、生命周期和 NLL 冲突
fn scope_lifetime_nll_conflicts() {
    println!("\n⚔️ 作用域、生命周期和 NLL 冲突:");
    
    // 冲突1: 作用域规则 vs NLL 优化
    {
        let mut data = vec![1, 2, 3, 4, 5];
        
        // 传统作用域规则会要求这样写
        let result = {
            let borrowed = &data;
            borrowed.len()
        }; // borrowed 在这里结束
        
        // NLL 允许更自然的写法
        let len = data.len(); // 直接借用
        data.push(6); // 立即可以修改
        
        println!("   📏 长度: {}, 结果: {}", len, result);
    }
    
    // 冲突2: 显式生命周期 vs NLL 推断
    {
        struct Container<'a> {
            data: &'a str,
        }
        
        impl<'a> Container<'a> {
            fn new(data: &'a str) -> Self {
                Container { data }
            }
            
            fn get_data(&self) -> &str {
                self.data
            }
        }
        
        let text = "hello world";
        let container = Container::new(text);
        let data = container.get_data();
        
        // NLL 优化了这种情况下的借用检查
        println!("   📦 容器数据: {}", data);
    }
    
    // 冲突3: 复杂嵌套作用域
    {
        let mut outer_data = vec!["outer"];
        
        {
            let inner_data = vec!["inner"];
            
            // 复杂的借用模式
            let combined: Vec<&str> = outer_data.iter()
                .chain(inner_data.iter())
                .map(|s| s.as_ref())
                .collect();
            
            println!("   🔗 组合数据: {:?}", combined);
            
            // NLL 允许在这里修改 outer_data
            // 因为 combined 不会超出这个作用域
        }
        
        outer_data.push("added");
        println!("   📈 外部数据: {:?}", outer_data);
    }
    
    println!("   ⚠️ 三者的交互可能产生意外的行为，需要仔细理解");
}

/// 宏展开问题
fn macro_expansion_issues() {
    println!("\n🎭 宏展开问题:");
    
    // 宏中的生命周期问题
    macro_rules! create_processor {
        ($name:ident, $process_fn:expr) => {
            struct $name;
            
            impl $name {
                fn process<'a>(&self, input: &'a str) -> &'a str {
                    let f: fn(&str) -> &str = $process_fn;
                    f(input)
                }
            }
        };
    }
    
    create_processor!(TrimProcessor, |s: &str| s.trim());
    create_processor!(UpperProcessor, |s: &str| {
        // 这里有一个问题：我们不能返回新创建的字符串的引用
        // s.to_uppercase() // 这会编译错误
        s // 只能返回输入的引用
    });
    
    let trim_processor = TrimProcessor;
    let upper_processor = UpperProcessor;
    
    let input = "  hello world  ";
    let trimmed = trim_processor.process(input);
    let upper = upper_processor.process(input);
    
    println!("   ✂️ 修剪后: '{}'", trimmed);
    println!("   🔤 大写处理: '{}'", upper);
    
    // 宏中的作用域问题
    macro_rules! with_temp_var {
        ($var:ident, $value:expr, $body:block) => {
            {
                let $var = $value;
                $body
            }
        };
    }
    
    with_temp_var!(temp, "temporary", {
        println!("   🔄 临时变量: {}", temp);
    });
    
    // temp 在这里不可访问
    // println!("外部: {}", temp); // 编译错误
    
    println!("   ⚠️ 宏展开可能导致意外的作用域和生命周期行为");
}

/// 泛型生命周期推断
fn generic_lifetime_inference() {
    println!("\n🔍 泛型生命周期推断:");
    
    // 复杂的泛型生命周期推断
    {
        trait Processor<T> {
            fn process<'a>(&self, input: &'a T) -> &'a T;
        }
        
        struct IdentityProcessor;
        
        impl<T> Processor<T> for IdentityProcessor {
            fn process<'a>(&self, input: &'a T) -> &'a T {
                input
            }
        }
        
        fn use_processor<'a, T, P>(processor: &'a P, items: &'a [T]) -> Vec<&'a T>
        where
            P: Processor<T>,
        {
            items.iter().map(|item| processor.process(item)).collect()
        }
        
        let processor = IdentityProcessor;
        let numbers = vec![1, 2, 3, 4, 5];
        let results = use_processor(&processor, &numbers);
        
        println!("   🔢 处理结果: {:?}", results);
    }
    
    // 生命周期推断的限制
    {
        struct Container<T> {
            items: Vec<T>,
        }
        
        impl<T> Container<T> {
            fn new() -> Self {
                Container { items: Vec::new() }
            }
            
            fn add(&mut self, item: T) {
                self.items.push(item);
            }
            
            // 复杂的生命周期推断
            fn get_first_and_last(&self) -> Option<(&T, &T)> {
                if self.items.len() >= 2 {
                    Some((&self.items[0], &self.items[self.items.len() - 1]))
                } else {
                    None
                }
            }
        }
        
        let mut container = Container::new();
        container.add("first");
        container.add("middle");
        container.add("last");
        
        if let Some((first, last)) = container.get_first_and_last() {
            println!("   📍 第一个: {}, 最后一个: {}", first, last);
        }
    }
    
    println!("   ⚠️ 复杂泛型场景中的生命周期推断可能不够精确");
}

/// Trait 对象生命周期问题
fn trait_object_lifetime_issues() {
    println!("\n🎯 Trait 对象生命周期问题:");
    
    // 问题1: Trait 对象的生命周期省略
    {
        trait Display {
            fn display(&self) -> &str;
        }
        
        struct Message {
            content: String,
        }
        
        impl Display for Message {
            fn display(&self) -> &str {
                &self.content
            }
        }
        
        // 这里的生命周期推断可能不明确
        fn get_display(msg: &Message) -> &dyn Display {
            msg
        }
        
        let message = Message {
            content: "Hello, trait object!".to_string(),
        };
        
        let display_obj = get_display(&message);
        println!("   📺 显示: {}", display_obj.display());
    }
    
    // 问题2: 动态分发的生命周期
    {
        trait Processor {
            fn process(&self, input: &str) -> String;
        }
        
        struct UpperCaseProcessor;
        struct LowerCaseProcessor;
        
        impl Processor for UpperCaseProcessor {
            fn process(&self, input: &str) -> String {
                input.to_uppercase()
            }
        }
        
        impl Processor for LowerCaseProcessor {
            fn process(&self, input: &str) -> String {
                input.to_lowercase()
            }
        }
        
        fn process_with_dynamic(processor: &dyn Processor, input: &str) -> String {
            processor.process(input)
        }
        
        let upper_processor = UpperCaseProcessor;
        let lower_processor = LowerCaseProcessor;
        
        let input = "Hello World";
        
        let upper_result = process_with_dynamic(&upper_processor, input);
        let lower_result = process_with_dynamic(&lower_processor, input);
        
        println!("   🔤 大写结果: {}", upper_result);
        println!("   🔡 小写结果: {}", lower_result);
    }
    
    // 问题3: Box<dyn Trait> 的生命周期
    {
        trait Task {
            fn execute(&self) -> String;
        }
        
        struct SimpleTask {
            name: String,
        }
        
        impl Task for SimpleTask {
            fn execute(&self) -> String {
                format!("执行任务: {}", self.name)
            }
        }
        
        fn create_task(name: String) -> Box<dyn Task> {
            Box::new(SimpleTask { name })
        }
        
        let task = create_task("重要任务".to_string());
        let result = task.execute();
        println!("   📋 任务结果: {}", result);
    }
    
    println!("   ⚠️ Trait 对象的生命周期管理需要特别注意");
}

/// 5. 内存安全边界情况
fn memory_safety_edge_cases() {
    println!("\n🛡️ 5. 内存安全边界情况");
    println!("分析可能导致内存安全问题的边界情况。");
    
    dangling_pointer_prevention();
    use_after_free_prevention();
    data_race_prevention();
    memory_leak_scenarios();
}

/// 悬垂指针预防
fn dangling_pointer_prevention() {
    println!("\n🎯 悬垂指针预防:");
    
    // Rust 如何防止悬垂指针
    {
        // 这种代码会被编译器拒绝
        // fn create_dangling() -> &str {
        //     let s = String::from("hello");
        //     &s // 错误：返回局部变量的引用
        // }
        
        // 正确的做法：返回拥有的值
        fn create_owned() -> String {
            String::from("hello")
        }
        
        let owned = create_owned();
        println!("   ✅ 拥有的字符串: {}", owned);
    }
    
    // 复杂的悬垂指针场景
    {
        struct Container {
            data: Vec<String>,
        }
        
        impl Container {
            fn new() -> Self {
                Container {
                    data: vec!["item1".to_string(), "item2".to_string()],
                }
            }
            
            // 这个方法是安全的
            fn get_first(&self) -> Option<&String> {
                self.data.first()
            }
            
            // 这种模式会被编译器拒绝
            // fn get_first_after_clear(&mut self) -> Option<&String> {
            //     let first = self.data.first();
            //     self.data.clear(); // 错误：在借用期间修改
            //     first
            // }
        }
        
        let container = Container::new();
        if let Some(first) = container.get_first() {
            println!("   📦 第一个项目: {}", first);
        }
    }
    
    println!("   ✅ Rust 的借用检查器有效防止了悬垂指针");
}

/// Use-after-free 预防
fn use_after_free_prevention() {
    println!("\n🚫 Use-after-free 预防:");
    
    // Rust 如何防止 use-after-free
    {
        let data = vec![1, 2, 3, 4, 5];
        
        // 这种代码会被编译器拒绝
        // let reference = &data[0];
        // drop(data); // 错误：在借用期间销毁
        // println!("值: {}", reference); // use-after-free
        
        // 正确的做法：确保引用的生命周期
        {
            let reference = &data[0];
            println!("   🔢 值: {}", reference);
        } // reference 在这里结束
        
        drop(data); // 现在可以安全销毁
    }
    
    // 复杂的 use-after-free 场景
    {
        use std::collections::HashMap;
        
        let mut map = HashMap::new();
        map.insert("key1", vec![1, 2, 3]);
        map.insert("key2", vec![4, 5, 6]);
        
        // 这种模式是安全的
        if let Some(values) = map.get("key1") {
            let sum: i32 = values.iter().sum();
            println!("   🧮 key1 的和: {}", sum);
        }
        
        // 移除键值对
        map.remove("key1");
        println!("   🗑️ 移除 key1 后，剩余键: {:?}", map.keys().collect::<Vec<_>>());
    }
    
    println!("   ✅ Rust 的所有权系统防止了 use-after-free 错误");
}

/// 数据竞争预防
fn data_race_prevention() {
    println!("\n🏃‍♂️ 数据竞争预防:");
    
    // Rust 如何在编译时防止数据竞争
    {
        let mut data = vec![1, 2, 3, 4, 5];
        
        // 这种代码会被编译器拒绝
        // let r1 = &data;
        // let r2 = &mut data; // 错误：不能同时有可变和不可变引用
        // println!("r1: {:?}, r2: {:?}", r1, r2);
        
        // 正确的做法：分离借用
        {
            let r1 = &data;
            println!("   📖 只读引用: {:?}", r1);
        } // r1 结束
        
        {
            let r2 = &mut data;
            r2.push(6);
            println!("   ✏️ 可变引用: {:?}", r2);
        } // r2 结束
    }
    
    // 多线程数据竞争预防
    {
        use std::sync::{Arc, Mutex};
        use std::thread;
        
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        
        for i in 0..3 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += i;
                println!("   🧵 线程 {} 更新计数器", i);
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let final_count = *counter.lock().unwrap();
        println!("   🏁 最终计数: {}", final_count);
    }
    
    println!("   ✅ Rust 的类型系统防止了数据竞争");
}

/// 内存泄漏场景
fn memory_leak_scenarios() {
    println!("\n💧 内存泄漏场景:");
    
    // Rust 中可能的内存泄漏
    {
        use std::rc::{Rc, Weak};
        use std::cell::RefCell;
        
        // 循环引用可能导致内存泄漏
        #[derive(Debug)]
        struct Node {
            value: i32,
            parent: Option<Weak<RefCell<Node>>>,
            children: Vec<Rc<RefCell<Node>>>,
        }
        
        impl Node {
            fn new(value: i32) -> Rc<RefCell<Self>> {
                Rc::new(RefCell::new(Node {
                    value,
                    parent: None,
                    children: Vec::new(),
                }))
            }
            
            fn add_child(parent: &Rc<RefCell<Node>>, child_value: i32) {
                let child = Node::new(child_value);
                child.borrow_mut().parent = Some(Rc::downgrade(parent));
                parent.borrow_mut().children.push(child);
            }
        }
        
        // 创建节点树（正确使用 Weak 引用避免循环）
        let root = Node::new(1);
        Node::add_child(&root, 2);
        Node::add_child(&root, 3);
        
        println!("   🌳 根节点值: {}", root.borrow().value);
        println!("   🌿 子节点数: {}", root.borrow().children.len());
        
        // 使用 Weak 引用打破循环
        {
            let borrowed_root = root.borrow();
            if let Some(first_child) = borrowed_root.children.first() {
                let borrowed_child = first_child.borrow();
                if let Some(parent) = borrowed_child.parent.as_ref() {
                    if let Some(parent) = parent.upgrade() {
                        println!("   👨‍👦 子节点的父节点值: {}", parent.borrow().value);
                    }
                }
            }
        }
    }
    
    // 故意的内存泄漏
    {
        use std::mem;
        
        // 使用 Box::leak 故意泄漏内存
        let leaked_string = Box::leak(Box::new("这个字符串被故意泄漏了".to_string()));
        println!("   💧 泄漏的字符串: {}", leaked_string);
        
        // 使用 mem::forget 忘记值（不调用析构函数）
        let forgotten_vec = vec![1, 2, 3, 4, 5];
        mem::forget(forgotten_vec);
        println!("   🤔 忘记了一个向量（可能泄漏内存）");
    }
    
    // 长期存活的数据结构
    {
        static mut GLOBAL_CACHE: Option<HashMap<String, String>> = None;
        
        unsafe {
            GLOBAL_CACHE = Some(HashMap::new());
            
            if let Some(ref mut cache) = GLOBAL_CACHE {
                cache.insert("key1".to_string(), "value1".to_string());
                cache.insert("key2".to_string(), "value2".to_string());
                
                println!("   🌐 全局缓存大小: {}", cache.len());
            }
        }
    }
    
    println!("   ⚠️ 虽然 Rust 防止了大多数内存问题，但仍需注意循环引用和故意泄漏");
}

/// 6. 性能边界情况
fn performance_edge_cases() {
    println!("\n🚀 6. 性能边界情况");
    println!("分析可能影响性能的边界情况。");
    
    borrow_checker_overhead();
    lifetime_monomorphization();
    drop_performance_issues();
    zero_cost_abstractions_limits();
}

/// 借用检查器开销
fn borrow_checker_overhead() {
    println!("\n⚖️ 借用检查器开销:");
    
    // 编译时开销
    {
        // 复杂的借用模式可能增加编译时间
        let mut complex_structure = HashMap::new();
        
        for i in 0..100 {
            complex_structure.insert(format!("key_{}", i), vec![i; 10]);
        }
        
        // 复杂的借用和修改模式
        let mut total_elements = 0;
        for (key, values) in &complex_structure {
            total_elements += values.len();
            if key.contains("5") {
                println!("   🔑 找到包含5的键: {}", key);
            }
        }
        
        // NLL 允许在迭代后立即修改
        complex_structure.insert("new_key".to_string(), vec![999]);
        
        println!("   📊 总元素数: {}, 结构大小: {}", total_elements, complex_structure.len());
    }
    
    // 运行时开销（通常为零）
    {
        let data = vec![1, 2, 3, 4, 5];
        
        // 借用检查在编译时完成，运行时无开销
        let slice1 = &data[0..2];
        let slice2 = &data[2..4];
        
        println!("   📊 切片1: {:?}, 切片2: {:?}", slice1, slice2);
    }
    
    println!("   ✅ 借用检查器主要影响编译时间，运行时开销为零");
}

/// 生命周期单态化
fn lifetime_monomorphization() {
    println!("\n🔄 生命周期单态化:");
    
    // 泛型函数的生命周期单态化
    {
        fn process_data<'a, T>(data: &'a [T]) -> &'a [T] 
        where
            T: std::fmt::Debug,
        {
            println!("   🔧 处理数据: {:?}", data);
            data
        }
        
        let numbers = vec![1, 2, 3];
        let strings = vec!["a", "b", "c"];
        
        // 每种类型都会生成单独的代码
        let _processed_numbers = process_data(&numbers);
        let _processed_strings = process_data(&strings);
    }
    
    // 复杂的生命周期参数
    {
        struct Processor<'a, 'b> {
            input: &'a str,
            output: &'b mut String,
        }
        
        impl<'a, 'b> Processor<'a, 'b> {
            fn new(input: &'a str, output: &'b mut String) -> Self {
                Processor { input, output }
            }
            
            fn process(&mut self) {
                self.output.push_str("Processed: ");
                self.output.push_str(self.input);
            }
        }
        
        let input = "hello world";
        let mut output = String::new();
        
        let mut processor = Processor::new(input, &mut output);
        processor.process();
        
        println!("   🔄 处理结果: {}", output);
    }
    
    println!("   ⚠️ 复杂的生命周期参数可能导致代码膨胀");
}

/// Drop 性能问题
fn drop_performance_issues() {
    println!("\n🗑️ Drop 性能问题:");
    
    // 昂贵的 Drop 实现
    {
        struct ExpensiveDrop {
            data: Vec<String>,
        }
        
        impl Drop for ExpensiveDrop {
            fn drop(&mut self) {
                // 模拟昂贵的清理操作
                for item in &self.data {
                    if item.len() > 5 {
                        println!("   🧹 清理长字符串: {}", item);
                    }
                }
                println!("   🗑️ 完成昂贵的清理操作");
            }
        }
        
        {
            let _expensive = ExpensiveDrop {
                data: vec![
                    "short".to_string(),
                    "very long string".to_string(),
                    "another long string here".to_string(),
                ],
            };
            
            println!("   📦 创建了昂贵的对象");
        } // 在这里调用 drop，可能很慢
    }
    
    // Drop 顺序的性能影响
    {
        struct Counter {
            count: usize,
        }
        
        impl Drop for Counter {
            fn drop(&mut self) {
                println!("   📊 销毁计数器: {}", self.count);
            }
        }
        
        let _counters: Vec<Counter> = (0..5)
            .map(|i| Counter { count: i })
            .collect();
        
        // 向量销毁时会按逆序销毁所有元素
        println!("   📋 创建了5个计数器，即将销毁");
    }
    
    println!("   ⚠️ Drop 实现的复杂度直接影响对象销毁的性能");
}

/// 零成本抽象的限制
fn zero_cost_abstractions_limits() {
    println!("\n🎯 零成本抽象的限制:");
    
    // 迭代器优化
    {
        let data: Vec<i32> = (0..1000).collect();
        
        // 这种链式操作通常会被优化
        let result: Vec<i32> = data
            .iter()
            .filter(|&&x| x % 2 == 0)
            .map(|&x| x * 2)
            .collect();
        
        println!("   🔗 链式操作结果长度: {}", result.len());
    }
    
    // 复杂的类型系统可能影响编译时间
    {
        trait ComplexTrait<T, U> {
            type Output;
            fn process(&self, input: T) -> Self::Output;
        }
        
        struct ComplexProcessor;
        
        impl ComplexTrait<i32, String> for ComplexProcessor {
            type Output = String;
            
            fn process(&self, input: i32) -> Self::Output {
                format!("处理数字: {}", input)
            }
        }
        
        let processor = ComplexProcessor;
        let result = processor.process(42);
        println!("   🔧 复杂处理结果: {}", result);
    }
    
    println!("   ✅ 大多数抽象确实是零成本的，但复杂类型可能影响编译时间");
}

/// 7. 调试边界情况
fn debugging_edge_cases() {
    println!("\n🐛 7. 调试边界情况");
    println!("分析在调试过程中可能遇到的特殊情况。");
    
    error_message_confusion();
    debugging_lifetime_issues();
    compiler_suggestion_limitations();
    runtime_vs_compile_time_errors();
}

/// 错误消息混淆
fn error_message_confusion() {
    println!("\n❌ 错误消息混淆:");
    
    // 复杂的借用检查错误
    {
        println!("   📝 常见的借用检查错误模式:");
        
        // 示例1: 同时借用错误
        println!("   1. 同时可变和不可变借用");
        println!("      错误: cannot borrow `x` as mutable because it is also borrowed as immutable");
        
        // 示例2: 生命周期不匹配
        println!("   2. 生命周期不匹配");
        println!("      错误: lifetime may not live long enough");
        
        // 示例3: 移动后使用
        println!("   3. 移动后使用");
        println!("      错误: borrow of moved value");
    }
    
    // 实际的正确代码示例
    {
        let mut data = vec![1, 2, 3];
        
        // 正确的借用模式
        {
            let len = data.len();
            println!("   📏 数据长度: {}", len);
        }
        
        data.push(4);
        println!("   📈 添加元素后: {:?}", data);
    }
    
    println!("   ⚠️ 错误消息虽然准确，但对初学者可能难以理解");
}

/// 调试生命周期问题
fn debugging_lifetime_issues() {
    println!("\n🔍 调试生命周期问题:");
    
    // 生命周期标注的调试技巧
    {
        // 显式标注生命周期有助于理解
        fn explicit_lifetimes<'a, 'b>(x: &'a str, y: &'b str) -> &'a str 
        where
            'b: 'a, // 'b 必须至少与 'a 一样长
        {
            if x.len() > y.len() {
                x
            } else {
                // 这里需要确保 y 的生命周期足够长
                x // 返回 x 以满足返回类型
            }
        }
        
        let string1 = "short";
        let string2 = "longer string";
        let result = explicit_lifetimes(string1, string2);
        println!("   🔤 显式生命周期结果: {}", result);
    }
    
    // 使用类型标注帮助调试
    {
        let data = vec!["hello", "world", "rust"];
        
        // 显式类型标注有助于理解生命周期
        let first: Option<&str> = data.first().copied();
        
        if let Some(first_item) = first {
            println!("   📍 第一个项目: {}", first_item);
        }
    }
    
    println!("   💡 显式生命周期标注和类型标注有助于调试");
}

/// 编译器建议的限制
fn compiler_suggestion_limitations() {
    println!("\n🤖 编译器建议的限制:");
    
    // 编译器建议可能不总是最佳的
    {
        println!("   📋 编译器建议的常见模式:");
        println!("   1. 添加生命周期参数");
        println!("   2. 使用 clone() 避免借用问题");
        println!("   3. 重构代码结构");
        
        // 有时 clone 不是最佳解决方案
        let expensive_data = vec!["large".repeat(1000); 100];
        
        // 避免不必要的克隆
        let reference = &expensive_data[0];
        println!("   📦 引用长度: {}", reference.len());
        
        // 而不是
        // let cloned = expensive_data[0].clone(); // 昂贵的克隆
    }
    
    println!("   ⚠️ 编译器建议虽然有用，但需要结合具体情况判断");
}

/// 运行时 vs 编译时错误
fn runtime_vs_compile_time_errors() {
    println!("\n⏰ 运行时 vs 编译时错误:");
    
    // Rust 将许多错误提前到编译时
    {
        println!("   ✅ 编译时捕获的错误:");
        println!("   - 空指针解引用");
        println!("   - 缓冲区溢出");
        println!("   - 数据竞争");
        println!("   - 内存泄漏（大部分）");
        
        // 仍然可能的运行时错误
        println!("   ⚠️ 可能的运行时错误:");
        println!("   - 数组越界（使用索引时）");
        println!("   - 整数溢出（debug模式下panic）");
        println!("   - 除零错误");
        println!("   - unwrap() 调用失败");
    }
    
    // 安全的运行时检查
    {
        let data = vec![1, 2, 3, 4, 5];
        
        // 安全的访问方式
        match data.get(10) {
            Some(value) => println!("   📊 值: {}", value),
            None => println!("   ❌ 索引超出范围"),
        }
        
        // 安全的数学运算
        let a = 100u8;
        let b = 200u8;
        
        match a.checked_add(b) {
            Some(sum) => println!("   ➕ 和: {}", sum),
            None => println!("   ⚠️ 整数溢出"),
        }
    }
    
    println!("   ✅ Rust 的设计目标是将尽可能多的错误提前到编译时");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_cases_analysis() {
        // 测试边界情况分析是否能正常运行
        run_edge_cases_analysis();
    }

    #[test]
    fn test_scope_edge_cases() {
        // 测试作用域边界情况
        scope_edge_cases();
    }

    #[test]
    fn test_lifetime_edge_cases() {
        // 测试生命周期边界情况
        lifetime_edge_cases();
    }

    #[test]
    fn test_nll_edge_cases() {
        // 测试 NLL 边界情况
        nll_edge_cases();
    }

    #[test]
    fn test_memory_safety() {
        // 测试内存安全边界情况
        memory_safety_edge_cases();
    }
}