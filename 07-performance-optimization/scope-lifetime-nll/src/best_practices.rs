//! # 最佳实践指南
//!
//! 本模块提供在实际开发中使用作用域、生命周期和 NLL 的最佳实践指南。
//! 包括设计原则、常见模式、性能优化和代码质量提升等方面的建议。

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::RefCell;

/// 运行最佳实践分析
pub fn run_best_practices_analysis() {
    println!("\n🎯 ===== 作用域、生命周期和 NLL 最佳实践指南 ===== 🎯");
    println!("本分析提供实际开发中的最佳实践建议和设计模式。");
    
    scope_best_practices();
    lifetime_best_practices();
    nll_best_practices();
    design_patterns();
    performance_optimization();
    code_quality_guidelines();
    common_antipatterns();
    testing_strategies();
}

/// 1. 作用域最佳实践
fn scope_best_practices() {
    println!("\n🔍 1. 作用域最佳实践");
    println!("合理使用作用域可以提高代码的可读性、安全性和性能。");
    
    minimize_scope_principle();
    resource_management_patterns();
    scope_based_error_handling();
    modular_scope_design();
}

/// 最小作用域原则
fn minimize_scope_principle() {
    println!("\n📏 最小作用域原则:");
    
    // ✅ 好的实践：最小化变量作用域
    {
        println!("   ✅ 好的实践 - 最小化变量作用域:");
        
        let data = vec![1, 2, 3, 4, 5];
        
        // 只在需要时创建变量
        let sum = {
            let mut total = 0;
            for &item in &data {
                total += item;
            }
            total
        };
        
        println!("   📊 计算结果: {}", sum);
        // total 在这里已经不可访问，内存已释放
    }
    
    // ❌ 不好的实践：过大的作用域
    {
        println!("   ❌ 避免的实践 - 过大的作用域:");
        
        let data = vec![1, 2, 3, 4, 5];
        let mut total = 0; // 作用域过大
        let mut temp_var = String::new(); // 不必要的长生命周期
        
        for &item in &data {
            temp_var.clear();
            temp_var.push_str(&format!("Processing {}", item));
            total += item;
        }
        
        println!("   📊 结果: {}", total);
        // temp_var 在循环外仍然存在，占用内存
    }
    
    println!("   💡 原则: 变量的作用域应该尽可能小，在最晚的时候创建，最早的时候销毁");
}

/// 资源管理模式
fn resource_management_patterns() {
    println!("\n🔧 资源管理模式:");
    
    // RAII 模式
    {
        println!("   🏗️ RAII (Resource Acquisition Is Initialization) 模式:");
        
        struct FileManager {
            filename: String,
        }
        
        impl FileManager {
            fn new(filename: &str) -> Self {
                println!("   📂 打开文件: {}", filename);
                FileManager {
                    filename: filename.to_string(),
                }
            }
            
            fn process(&self) {
                println!("   ⚙️ 处理文件: {}", self.filename);
            }
        }
        
        impl Drop for FileManager {
            fn drop(&mut self) {
                println!("   🗑️ 关闭文件: {}", self.filename);
            }
        }
        
        {
            let file = FileManager::new("data.txt");
            file.process();
        } // 文件在这里自动关闭
        
        println!("   ✅ 文件已自动关闭，无需手动管理");
    }
    
    // 作用域守卫模式
    {
        println!("   🛡️ 作用域守卫模式:");
        
        struct ScopeGuard<F: FnOnce()> {
            cleanup: Option<F>,
        }
        
        impl<F: FnOnce()> ScopeGuard<F> {
            fn new(cleanup: F) -> Self {
                ScopeGuard {
                    cleanup: Some(cleanup),
                }
            }
        }
        
        impl<F: FnOnce()> Drop for ScopeGuard<F> {
            fn drop(&mut self) {
                if let Some(cleanup) = self.cleanup.take() {
                    cleanup();
                }
            }
        }
        
        {
            let _guard = ScopeGuard::new(|| {
                println!("   🧹 执行清理操作");
            });
            
            println!("   🔄 执行主要逻辑");
        } // 清理操作在这里自动执行
    }
    
    println!("   💡 使用 RAII 和作用域守卫确保资源的正确管理");
}

/// 基于作用域的错误处理
fn scope_based_error_handling() {
    println!("\n⚠️ 基于作用域的错误处理:");
    
    // 使用 ? 操作符的最佳实践
    {
        fn process_data() -> Result<i32, &'static str> {
            let data = vec![1, 2, 3, 4, 5];
            
            // 在小作用域中处理可能失败的操作
            let validated_data = {
                if data.is_empty() {
                    return Err("数据为空");
                }
                data
            };
            
            let sum = validated_data.iter().sum();
            Ok(sum)
        }
        
        match process_data() {
            Ok(result) => println!("   ✅ 处理成功: {}", result),
            Err(e) => println!("   ❌ 处理失败: {}", e),
        }
    }
    
    // 错误恢复模式
    {
        fn robust_operation() -> i32 {
            // 尝试主要操作
            let result = {
                let data = vec![1, 2, 3];
                if data.len() > 10 {
                    Some(data.iter().sum())
                } else {
                    None
                }
            };
            
            // 如果失败，使用备用方案
            result.unwrap_or_else(|| {
                println!("   🔄 使用备用方案");
                42 // 默认值
            })
        }
        
        let result = robust_operation();
        println!("   📊 最终结果: {}", result);
    }
    
    println!("   💡 在小作用域中处理错误，保持主逻辑的清晰");
}

/// 模块化作用域设计
fn modular_scope_design() {
    println!("\n🏗️ 模块化作用域设计:");
    
    // 功能分组
    {
        println!("   📦 功能分组:");
        
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        
        // 数据预处理
        let processed_data = {
            println!("   🔄 数据预处理阶段");
            data.iter().filter(|&&x| x % 2 == 0).collect::<Vec<_>>()
        };
        
        // 数据分析
        let analysis_result = {
            println!("   📊 数据分析阶段");
            let sum: i32 = processed_data.iter().map(|&&x| x).sum();
            let count = processed_data.len();
            (sum, count)
        };
        
        // 结果输出
        {
            println!("   📋 结果输出阶段");
            let (sum, count) = analysis_result;
            println!("   📈 偶数和: {}, 数量: {}", sum, count);
        }
    }
    
    println!("   💡 将相关功能组织在同一作用域中，提高代码的可读性和维护性");
}

/// 2. 生命周期最佳实践
fn lifetime_best_practices() {
    println!("\n⏰ 2. 生命周期最佳实践");
    println!("正确使用生命周期可以避免内存安全问题，提高代码的表达力。");
    
    lifetime_elision_guidelines();
    explicit_lifetime_when_needed();
    lifetime_bounds_best_practices();
    avoiding_lifetime_complexity();
}

/// 生命周期省略指南
fn lifetime_elision_guidelines() {
    println!("\n✂️ 生命周期省略指南:");
    
    // ✅ 利用生命周期省略规则
    {
        println!("   ✅ 利用生命周期省略规则:");
        
        // 规则1: 每个输入引用参数都有自己的生命周期
        fn first_word(s: &str) -> &str {
            s.split_whitespace().next().unwrap_or("")
        }
        
        // 规则2: 如果只有一个输入生命周期，它被赋予所有输出生命周期
        fn get_prefix(text: &str) -> &str {
            &text[..3.min(text.len())]
        }
        
        let text = "hello world";
        let first = first_word(text);
        let prefix = get_prefix(text);
        
        println!("   📝 第一个词: '{}', 前缀: '{}'", first, prefix);
    }
    
    // ✅ 方法中的生命周期省略
    {
        println!("   ✅ 方法中的生命周期省略:");
        
        struct TextProcessor {
            prefix: String,
        }
        
        impl TextProcessor {
            // 规则3: 如果有 &self 或 &mut self，self 的生命周期被赋予所有输出
            fn process(&self, input: &str) -> String {
                format!("{}: {}", self.prefix, input)
            }
            
            fn get_prefix(&self) -> &str {
                &self.prefix
            }
        }
        
        let processor = TextProcessor {
            prefix: "处理".to_string(),
        };
        
        let result = processor.process("数据");
        let prefix = processor.get_prefix();
        
        println!("   🔧 处理结果: '{}', 前缀: '{}'", result, prefix);
    }
    
    println!("   💡 优先使用生命周期省略规则，只在必要时显式标注");
}

/// 何时需要显式生命周期
fn explicit_lifetime_when_needed() {
    println!("\n🏷️ 何时需要显式生命周期:");
    
    // 多个输入引用时
    {
        println!("   📋 多个输入引用时:");
        
        // 需要显式生命周期
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        
        let string1 = "short";
        let string2 = "longer string";
        let result = longest(string1, string2);
        
        println!("   📏 更长的字符串: '{}'", result);
    }
    
    // 结构体中的引用
    {
        println!("   🏗️ 结构体中的引用:");
        
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }
        
        impl<'a> ImportantExcerpt<'a> {
            fn level(&self) -> i32 {
                3
            }
            
            fn announce_and_return_part(&self, announcement: &str) -> &str {
                println!("   📢 注意: {}", announcement);
                self.part
            }
        }
        
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        
        let excerpt = ImportantExcerpt {
            part: first_sentence,
        };
        
        let part = excerpt.announce_and_return_part("重要摘录");
        println!("   📖 摘录: '{}', 级别: {}", part, excerpt.level());
    }
    
    println!("   💡 在编译器无法推断生命周期关系时，需要显式标注");
}

/// 生命周期约束最佳实践
fn lifetime_bounds_best_practices() {
    println!("\n🔗 生命周期约束最佳实践:");
    
    // 生命周期子类型
    {
        println!("   📊 生命周期子类型:");
        
        fn choose_str<'a, 'b>(first: &'a str, _second: &'b str, use_first: bool) -> &'a str 
        where
            'b: 'a, // 'b 必须至少与 'a 一样长
        {
            if use_first {
                first
            } else {
                first // 总是返回 first 以满足返回类型
            }
        }
        
        let long_string = String::from("这是一个很长的字符串");
        {
            let short_string = String::from("短");
            let result = choose_str(&long_string, &short_string, true);
            println!("   🔤 选择的字符串: '{}'", result);
        }
    }
    
    // 静态生命周期的使用
    {
        println!("   🌟 静态生命周期的使用:");
        
        // 字符串字面量具有 'static 生命周期
        const GREETING: &'static str = "Hello, World!";
        
        fn get_greeting() -> &'static str {
            GREETING
        }
        
        // 泄漏内存以获得 'static 生命周期（谨慎使用）
        fn leak_string(s: String) -> &'static str {
            Box::leak(s.into_boxed_str())
        }
        
        let greeting = get_greeting();
        let leaked = leak_string("泄漏的字符串".to_string());
        
        println!("   👋 问候: '{}'", greeting);
        println!("   💧 泄漏的: '{}'", leaked);
    }
    
    println!("   💡 谨慎使用生命周期约束，避免过度复杂化");
}

/// 避免生命周期复杂性
fn avoiding_lifetime_complexity() {
    println!("\n🎯 避免生命周期复杂性:");
    
    // 使用拥有的数据而不是引用
    {
        println!("   📦 使用拥有的数据:");
        
        // ❌ 复杂的生命周期
        // struct ComplexStruct<'a, 'b> {
        //     field1: &'a str,
        //     field2: &'b str,
        // }
        
        // ✅ 简单的拥有数据
        #[derive(Debug)]
        struct SimpleStruct {
            field1: String,
            field2: String,
        }
        
        let simple = SimpleStruct {
            field1: "第一个字段".to_string(),
            field2: "第二个字段".to_string(),
        };
        
        println!("   🏗️ 简单结构: {:?}", simple);
    }
    
    // 使用智能指针
    {
        println!("   🧠 使用智能指针:");
        
        use std::rc::Rc;
        
        #[derive(Debug)]
        struct Node {
            value: i32,
            children: Vec<Rc<Node>>,
        }
        
        let leaf1 = Rc::new(Node {
            value: 1,
            children: vec![],
        });
        
        let leaf2 = Rc::new(Node {
            value: 2,
            children: vec![],
        });
        
        let root = Node {
            value: 0,
            children: vec![leaf1.clone(), leaf2.clone()],
        };
        
        println!("   🌳 树结构: {:?}", root);
        println!("   📊 leaf1 引用计数: {}", Rc::strong_count(&leaf1));
    }
    
    println!("   💡 优先考虑拥有的数据和智能指针，而不是复杂的生命周期");
}

/// 3. NLL 最佳实践
fn nll_best_practices() {
    println!("\n🚀 3. NLL 最佳实践");
    println!("充分利用 NLL 的优势，编写更自然和高效的代码。");
    
    leveraging_nll_improvements();
    nll_friendly_patterns();
    migration_strategies();
    nll_limitations_awareness();
}

/// 充分利用 NLL 改进
fn leveraging_nll_improvements() {
    println!("\n⚡ 充分利用 NLL 改进:");
    
    // 更自然的借用模式
    {
        println!("   🔄 更自然的借用模式:");
        
        let mut data = vec![1, 2, 3, 4, 5];
        
        // NLL 允许这种模式
        let first = data.first();
        if let Some(&value) = first {
            println!("   📍 第一个元素: {}", value);
        }
        
        // 在 NLL 之前，这里可能需要额外的作用域
        data.push(6);
        println!("   📈 添加元素后: {:?}", data);
    }
    
    // 条件借用
    {
        println!("   🔀 条件借用:");
        
        let mut map = HashMap::new();
        map.insert("key1", 1);
        map.insert("key2", 2);
        
        let key = "key1";
        
        // NLL 使这种模式成为可能
        if let Some(value) = map.get(key) {
            println!("   🔑 找到值: {}", value);
        } else {
            map.insert(key, 0);
        }
        
        // 继续使用 map
        map.insert("key3", 3);
        println!("   📊 最终映射: {:?}", map);
    }
    
    println!("   💡 NLL 使借用检查更加智能和灵活");
}

/// NLL 友好的模式
fn nll_friendly_patterns() {
    println!("\n🤝 NLL 友好的模式:");
    
    // 早期返回模式
    {
        println!("   🚪 早期返回模式:");
        
        fn process_option(data: &mut Vec<i32>) -> Option<i32> {
            // NLL 理解这里的借用在 return 后结束
            let first = data.first()?;
            let value = *first;
            
            // 可以继续修改 data
            data.push(value * 2);
            
            Some(value)
        }
        
        let mut numbers = vec![5, 10, 15];
        if let Some(result) = process_option(&mut numbers) {
            println!("   📊 处理结果: {}, 数据: {:?}", result, numbers);
        }
    }
    
    // 循环中的借用
    {
        println!("   🔄 循环中的借用:");
        
        let mut data = vec!["hello", "world", "rust"];
        
        for i in 0..data.len() {
            // NLL 理解每次迭代的借用是独立的
            if let Some(item) = data.get(i) {
                if item.len() > 4 {
                    println!("   📏 长字符串: {}", item);
                }
            }
        }
        
        // 循环后可以修改
        data.push("programming");
        println!("   📋 最终数据: {:?}", data);
    }
    
    println!("   💡 设计代码时考虑 NLL 的借用分析能力");
}

/// 迁移策略
fn migration_strategies() {
    println!("\n🔄 迁移策略:");
    
    // 从旧代码迁移
    {
        println!("   📦 从旧代码迁移:");
        
        // 旧风格：显式作用域
        let mut old_style_data = vec![1, 2, 3];
        let old_result = {
            let borrowed = &old_style_data;
            borrowed.len()
        };
        old_style_data.push(4);
        
        // 新风格：利用 NLL
        let mut new_style_data = vec![1, 2, 3];
        let new_result = new_style_data.len();
        new_style_data.push(4);
        
        println!("   📊 旧风格结果: {}, 新风格结果: {}", old_result, new_result);
    }
    
    // 渐进式重构
    {
        println!("   🔧 渐进式重构:");
        
        struct DataProcessor {
            data: Vec<String>,
        }
        
        impl DataProcessor {
            fn new() -> Self {
                DataProcessor {
                    data: vec!["item1".to_string(), "item2".to_string()],
                }
            }
            
            // 利用 NLL 简化方法
            fn process_and_add(&mut self, new_item: String) -> usize {
                let current_len = self.data.len();
                
                // NLL 允许在借用后修改
                self.data.push(new_item);
                
                current_len
            }
        }
        
        let mut processor = DataProcessor::new();
        let old_len = processor.process_and_add("item3".to_string());
        
        println!("   📈 处理前长度: {}, 处理后长度: {}", old_len, processor.data.len());
    }
    
    println!("   💡 逐步利用 NLL 的优势，简化现有代码");
}

/// NLL 限制意识
fn nll_limitations_awareness() {
    println!("\n⚠️ NLL 限制意识:");
    
    // 仍然存在的限制
    {
        println!("   🚧 仍然存在的限制:");
        
        let mut data = vec![1, 2, 3, 4, 5];
        
        // 这种模式仍然不被允许
        // let borrowed = &data;
        // data.push(6); // 错误：不能在借用存在时修改
        // println!("Borrowed: {:?}", borrowed);
        
        // 正确的方式
        {
            let borrowed = &data;
            println!("   📋 借用的数据: {:?}", borrowed);
        } // 借用在这里结束
        
        data.push(6);
        println!("   📈 修改后的数据: {:?}", data);
    }
    
    // 复杂情况下的限制
    {
        println!("   🔍 复杂情况下的限制:");
        
        struct Container {
            items: Vec<String>,
        }
        
        impl Container {
            fn get_or_insert(&mut self, index: usize, default: String) -> &String {
                // 这种模式可能仍然需要特殊处理
                if index >= self.items.len() {
                    self.items.resize(index + 1, default);
                }
                &self.items[index]
            }
        }
        
        let mut container = Container {
            items: vec!["first".to_string()],
        };
        
        let item = container.get_or_insert(2, "default".to_string());
        println!("   📦 获取的项目: {}", item);
    }
    
    println!("   💡 了解 NLL 的限制，在必要时使用传统的解决方案");
}

/// 4. 设计模式
fn design_patterns() {
    println!("\n🎨 4. 设计模式");
    println!("结合作用域、生命周期和 NLL 的常见设计模式。");
    
    builder_pattern_with_lifetimes();
    visitor_pattern_considerations();
    observer_pattern_with_ownership();
    factory_pattern_best_practices();
}

/// 构建器模式与生命周期
fn builder_pattern_with_lifetimes() {
    println!("\n🏗️ 构建器模式与生命周期:");
    
    // 拥有数据的构建器
    {
        println!("   📦 拥有数据的构建器:");
        
        #[derive(Debug)]
        struct Config {
            name: String,
            timeout: u64,
            retries: u32,
        }
        
        struct ConfigBuilder {
            name: Option<String>,
            timeout: Option<u64>,
            retries: Option<u32>,
        }
        
        impl ConfigBuilder {
            fn new() -> Self {
                ConfigBuilder {
                    name: None,
                    timeout: None,
                    retries: None,
                }
            }
            
            fn name(mut self, name: impl Into<String>) -> Self {
                self.name = Some(name.into());
                self
            }
            
            fn timeout(mut self, timeout: u64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            
            fn retries(mut self, retries: u32) -> Self {
                self.retries = Some(retries);
                self
            }
            
            fn build(self) -> Config {
                Config {
                    name: self.name.unwrap_or_else(|| "default".to_string()),
                    timeout: self.timeout.unwrap_or(30),
                    retries: self.retries.unwrap_or(3),
                }
            }
        }
        
        let config = ConfigBuilder::new()
            .name("my_service")
            .timeout(60)
            .retries(5)
            .build();
        
        println!("   ⚙️ 构建的配置: {:?}", config);
    }
    
    println!("   💡 构建器模式通过拥有数据避免复杂的生命周期问题");
}

/// 访问者模式考虑
fn visitor_pattern_considerations() {
    println!("\n👁️ 访问者模式考虑:");
    
    // 使用 trait 对象避免生命周期复杂性
    {
        println!("   🎭 使用 trait 对象:");
        
        trait Visitor {
            fn visit_number(&mut self, n: i32);
            fn visit_string(&mut self, s: &str);
        }
        
        struct PrintVisitor;
        
        impl Visitor for PrintVisitor {
            fn visit_number(&mut self, n: i32) {
                println!("   🔢 访问数字: {}", n);
            }
            
            fn visit_string(&mut self, s: &str) {
                println!("   📝 访问字符串: {}", s);
            }
        }
        
        enum Value {
            Number(i32),
            Text(String),
        }
        
        impl Value {
            fn accept(&self, visitor: &mut dyn Visitor) {
                match self {
                    Value::Number(n) => visitor.visit_number(*n),
                    Value::Text(s) => visitor.visit_string(s),
                }
            }
        }
        
        let values = vec![
            Value::Number(42),
            Value::Text("hello".to_string()),
            Value::Number(100),
        ];
        
        let mut visitor = PrintVisitor;
        for value in &values {
            value.accept(&mut visitor);
        }
    }
    
    println!("   💡 使用 trait 对象和拥有的数据简化访问者模式");
}

/// 观察者模式与所有权
fn observer_pattern_with_ownership() {
    println!("\n👀 观察者模式与所有权:");
    
    // 使用回调函数
    {
        println!("   📞 使用回调函数:");
        
        struct EventEmitter {
            callbacks: Vec<Box<dyn Fn(&str)>>,
        }
        
        impl EventEmitter {
            fn new() -> Self {
                EventEmitter {
                    callbacks: Vec::new(),
                }
            }
            
            fn subscribe<F>(&mut self, callback: F)
            where
                F: Fn(&str) + 'static,
            {
                self.callbacks.push(Box::new(callback));
            }
            
            fn emit(&self, event: &str) {
                for callback in &self.callbacks {
                    callback(event);
                }
            }
        }
        
        let mut emitter = EventEmitter::new();
        
        emitter.subscribe(|event| {
            println!("   📢 观察者1收到事件: {}", event);
        });
        
        emitter.subscribe(|event| {
            println!("   📢 观察者2收到事件: {}", event);
        });
        
        emitter.emit("用户登录");
        emitter.emit("数据更新");
    }
    
    println!("   💡 使用闭包和 Box 实现灵活的观察者模式");
}

/// 工厂模式最佳实践
fn factory_pattern_best_practices() {
    println!("\n🏭 工厂模式最佳实践:");
    
    // 类型安全的工厂
    {
        println!("   🔒 类型安全的工厂:");
        
        trait Shape {
            fn area(&self) -> f64;
            fn name(&self) -> &str;
        }
        
        struct Circle {
            radius: f64,
        }
        
        impl Shape for Circle {
            fn area(&self) -> f64 {
                std::f64::consts::PI * self.radius * self.radius
            }
            
            fn name(&self) -> &str {
                "圆形"
            }
        }
        
        struct Rectangle {
            width: f64,
            height: f64,
        }
        
        impl Shape for Rectangle {
            fn area(&self) -> f64 {
                self.width * self.height
            }
            
            fn name(&self) -> &str {
                "矩形"
            }
        }
        
        enum ShapeType {
            Circle { radius: f64 },
            Rectangle { width: f64, height: f64 },
        }
        
        struct ShapeFactory;
        
        impl ShapeFactory {
            fn create_shape(shape_type: ShapeType) -> Box<dyn Shape> {
                match shape_type {
                    ShapeType::Circle { radius } => Box::new(Circle { radius }),
                    ShapeType::Rectangle { width, height } => {
                        Box::new(Rectangle { width, height })
                    }
                }
            }
        }
        
        let circle = ShapeFactory::create_shape(ShapeType::Circle { radius: 5.0 });
        let rectangle = ShapeFactory::create_shape(ShapeType::Rectangle {
            width: 4.0,
            height: 6.0,
        });
        
        println!("   🔵 {}: 面积 = {:.2}", circle.name(), circle.area());
        println!("   ⬜ {}: 面积 = {:.2}", rectangle.name(), rectangle.area());
    }
    
    println!("   💡 使用枚举和 Box<dyn Trait> 实现类型安全的工厂模式");
}

/// 5. 性能优化
fn performance_optimization() {
    println!("\n⚡ 5. 性能优化");
    println!("利用作用域、生命周期和 NLL 进行性能优化。");
    
    zero_copy_optimizations();
    memory_layout_considerations();
    borrowing_vs_cloning_strategies();
    compiler_optimizations();
}

/// 零拷贝优化
fn zero_copy_optimizations() {
    println!("\n📋 零拷贝优化:");
    
    // 字符串切片而不是拷贝
    {
        println!("   ✂️ 字符串切片:");
        
        let large_text = "这是一个很长的文本，包含很多信息，我们想要提取其中的部分内容而不进行拷贝";
        
        // ✅ 零拷贝：使用切片
        let words: Vec<&str> = large_text.split('，').collect();
        
        // ❌ 有拷贝：转换为 String
        // let words: Vec<String> = large_text.split('，').map(|s| s.to_string()).collect();
        
        println!("   📝 提取的词语数量: {}", words.len());
        for (i, word) in words.iter().enumerate() {
            if i < 3 {
                println!("   📄 词语 {}: '{}'", i + 1, word);
            }
        }
    }
    
    // 引用传递而不是值传递
    {
        println!("   📦 引用传递:");
        
        #[derive(Debug)]
        struct LargeStruct {
            data: Vec<i32>,
            metadata: HashMap<String, String>,
        }
        
        impl LargeStruct {
            fn new() -> Self {
                let mut metadata = HashMap::new();
                metadata.insert("type".to_string(), "large".to_string());
                metadata.insert("version".to_string(), "1.0".to_string());
                
                LargeStruct {
                    data: (0..1000).collect(),
                    metadata,
                }
            }
        }
        
        // ✅ 零拷贝：传递引用
        fn process_large_struct(s: &LargeStruct) -> usize {
            s.data.len() + s.metadata.len()
        }
        
        let large_struct = LargeStruct::new();
        let size = process_large_struct(&large_struct);
        
        println!("   📊 结构体大小指标: {}", size);
    }
    
    println!("   💡 优先使用引用和切片，避免不必要的数据拷贝");
}

/// 内存布局考虑
fn memory_layout_considerations() {
    println!("\n🧠 内存布局考虑:");
    
    // 结构体字段顺序
    {
        println!("   📐 结构体字段顺序:");
        
        // ❌ 内存效率较低
        #[allow(dead_code)]
        struct BadLayout {
            flag: bool,    // 1 byte
            number: u64,   // 8 bytes (可能有7字节填充)
            another_flag: bool, // 1 byte
        }
        
        // ✅ 内存效率较高
        #[allow(dead_code)]
        struct GoodLayout {
            number: u64,   // 8 bytes
            flag: bool,    // 1 byte
            another_flag: bool, // 1 byte (总共10字节，可能填充到16字节)
        }
        
        println!("   📏 BadLayout 大小: {} bytes", std::mem::size_of::<BadLayout>());
        println!("   📏 GoodLayout 大小: {} bytes", std::mem::size_of::<GoodLayout>());
    }
    
    // 使用 Box 减少栈使用
    {
        println!("   📦 使用 Box 减少栈使用:");
        
        // 大型数据结构
        struct LargeData {
            buffer: [u8; 1024],
        }
        
        impl LargeData {
            fn new() -> Self {
                LargeData { buffer: [0; 1024] }
            }
        }
        
        // ✅ 在堆上分配大型数据
        let _large_data = Box::new(LargeData::new());
        
        println!("   💾 大型数据已在堆上分配，栈使用量: {} bytes", std::mem::size_of::<Box<LargeData>>());
        println!("   📊 实际数据大小: {} bytes", std::mem::size_of::<LargeData>());
    }
    
    println!("   💡 考虑内存布局和分配策略以优化性能");
}

/// 借用 vs 克隆策略
fn borrowing_vs_cloning_strategies() {
    println!("\n🔄 借用 vs 克隆策略:");
    
    // 何时借用
    {
        println!("   📋 何时借用:");
        
        let data = vec!["item1", "item2", "item3", "item4", "item5"];
        
        // ✅ 只读访问时借用
        fn count_long_items(items: &[&str]) -> usize {
            items.iter().filter(|item| item.len() > 4).count()
        }
        
        let long_count = count_long_items(&data);
        println!("   📊 长项目数量: {}", long_count);
        
        // 数据仍然可用
        println!("   📋 原始数据: {:?}", data);
    }
    
    // 何时克隆
    {
        println!("   📄 何时克隆:");
        
        let original = vec!["important".to_string(), "data".to_string()];
        
        // ✅ 需要修改或长期持有时克隆
        fn process_and_modify(mut items: Vec<String>) -> Vec<String> {
            items.push("processed".to_string());
            items.iter_mut().for_each(|item| {
                *item = format!("[{}]", item);
            });
            items
        }
        
        let processed = process_and_modify(original.clone());
        
        println!("   📋 原始数据: {:?}", original);
        println!("   🔧 处理后数据: {:?}", processed);
    }
    
    // 智能克隆策略
    {
        println!("   🧠 智能克隆策略:");
        
        use std::borrow::Cow;
        
        fn process_text(input: &str) -> Cow<str> {
            if input.contains("special") {
                // 需要修改时才克隆
                Cow::Owned(input.replace("special", "SPECIAL"))
            } else {
                // 不需要修改时借用
                Cow::Borrowed(input)
            }
        }
        
        let text1 = "normal text";
        let text2 = "special text";
        
        let result1 = process_text(text1);
        let result2 = process_text(text2);
        
        println!("   📝 结果1 (借用): '{}'", result1);
        println!("   📝 结果2 (拥有): '{}'", result2);
    }
    
    println!("   💡 根据使用场景选择借用或克隆，使用 Cow 实现智能策略");
}

/// 编译器优化
fn compiler_optimizations() {
    println!("\n🔧 编译器优化:");
    
    // 内联优化
    {
        println!("   ⚡ 内联优化:");
        
        // 小函数通常会被内联
        #[inline]
        fn fast_add(a: i32, b: i32) -> i32 {
            a + b
        }
        
        // 复杂函数可能不适合内联
        #[inline(never)]
        fn complex_calculation(data: &[i32]) -> i32 {
            data.iter()
                .enumerate()
                .map(|(i, &x)| x * (i as i32 + 1))
                .sum()
        }
        
        let numbers = vec![1, 2, 3, 4, 5];
        let sum = fast_add(10, 20);
        let complex_result = complex_calculation(&numbers);
        
        println!("   ➕ 快速加法: {}", sum);
        println!("   🔢 复杂计算: {}", complex_result);
    }
    
    // 迭代器优化
    {
        println!("   🔄 迭代器优化:");
        
        let data: Vec<i32> = (1..=1000).collect();
        
        // 这种链式操作通常会被优化为单个循环
        let result: Vec<i32> = data
            .iter()
            .filter(|&&x| x % 2 == 0)
            .map(|&x| x * x)
            .take(10)
            .collect();
        
        println!("   📊 优化后的结果: {:?}", result);
    }
    
    println!("   💡 编写编译器友好的代码，利用 Rust 的零成本抽象");
}

/// 6. 代码质量指南
fn code_quality_guidelines() {
    println!("\n📋 6. 代码质量指南");
    println!("提高代码可读性、可维护性和可测试性的指南。");
    
    naming_conventions();
    documentation_best_practices();
    error_handling_patterns();
    testing_with_lifetimes();
}

/// 命名约定
fn naming_conventions() {
    println!("\n🏷️ 命名约定:");
    
    // 生命周期参数命名
    {
        println!("   ⏰ 生命周期参数命名:");
        
        // ✅ 描述性的生命周期名称
        struct Parser<'input, 'output> {
            input: &'input str,
            output: &'output mut String,
        }
        
        impl<'input, 'output> Parser<'input, 'output> {
            fn new(input: &'input str, output: &'output mut String) -> Self {
                Parser { input, output }
            }
            
            fn parse(&mut self) {
                self.output.push_str("Parsed: ");
                self.output.push_str(self.input);
            }
        }
        
        let input = "hello world";
        let mut output = String::new();
        
        let mut parser = Parser::new(input, &mut output);
        parser.parse();
        
        println!("   📝 解析结果: '{}'", output);
    }
    
    // 变量和函数命名
    {
        println!("   📛 变量和函数命名:");
        
        // ✅ 清晰的命名
        fn calculate_user_score(user_actions: &[&str]) -> i32 {
            let base_score = 100;
            let action_bonus: i32 = user_actions
                .iter()
                .map(|action| match *action {
                    "login" => 10,
                    "post" => 20,
                    "comment" => 5,
                    _ => 0,
                })
                .sum();
            
            base_score + action_bonus
        }
        
        let user_actions = vec!["login", "post", "comment", "post"];
        let final_score = calculate_user_score(&user_actions);
        
        println!("   🏆 用户最终得分: {}", final_score);
    }
    
    println!("   💡 使用描述性的名称，让代码自文档化");
}

/// 文档最佳实践
fn documentation_best_practices() {
    println!("\n📚 文档最佳实践:");
    
    // 生命周期文档
    {
        println!("   📖 生命周期文档:");
        
        /// 查找两个字符串中较长的一个
        /// 
        /// # 生命周期
        /// 
        /// 返回值的生命周期与输入参数中较短的那个相同。
        /// 这意味着返回的引用在两个输入参数都有效时才有效。
        /// 
        /// # 示例
        /// 
        /// ```
        /// # fn find_longer<'a>(x: &'a str, y: &'a str) -> &'a str {
        /// #     if x.len() > y.len() { x } else { y }
        /// # }
        /// let s1 = "short";
        /// let s2 = "longer string";
        /// let result = find_longer(s1, s2);
        /// assert_eq!(result, "longer string");
        /// ```
        fn find_longer<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        
        let string1 = "hello";
        let string2 = "world!";
        let longer = find_longer(string1, string2);
        
        println!("   📏 更长的字符串: '{}'", longer);
    }
    
    println!("   💡 为复杂的生命周期关系提供清晰的文档");
}

/// 错误处理模式
fn error_handling_patterns() {
    println!("\n⚠️ 错误处理模式:");
    
    // 自定义错误类型
    {
        println!("   🎯 自定义错误类型:");
        
        #[derive(Debug)]
        enum ProcessingError {
            InvalidInput(String),
            InsufficientData,
            ProcessingFailed(String),
        }
        
        impl std::fmt::Display for ProcessingError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    ProcessingError::InvalidInput(msg) => write!(f, "无效输入: {}", msg),
                    ProcessingError::InsufficientData => write!(f, "数据不足"),
                    ProcessingError::ProcessingFailed(msg) => write!(f, "处理失败: {}", msg),
                }
            }
        }
        
        impl std::error::Error for ProcessingError {}
        
        fn process_data(data: &[i32]) -> Result<i32, ProcessingError> {
            if data.is_empty() {
                return Err(ProcessingError::InsufficientData);
            }
            
            if data.iter().any(|&x| x < 0) {
                return Err(ProcessingError::InvalidInput(
                    "不能包含负数".to_string(),
                ));
            }
            
            let sum: i32 = data.iter().sum();
            if sum > 1000 {
                return Err(ProcessingError::ProcessingFailed(
                    "结果超出范围".to_string(),
                ));
            }
            
            Ok(sum)
        }
        
        let test_data = vec![10, 20, 30, 40];
        match process_data(&test_data) {
            Ok(result) => println!("   ✅ 处理成功: {}", result),
            Err(e) => println!("   ❌ 处理失败: {}", e),
        }
    }
    
    println!("   💡 使用类型化的错误处理，提供清晰的错误信息");
}

/// 生命周期测试
fn testing_with_lifetimes() {
    println!("\n🧪 生命周期测试:");
    
    // 测试生命周期相关的函数
    {
        println!("   🔬 测试生命周期相关的函数:");
        
        fn extract_word<'a>(text: &'a str, index: usize) -> Option<&'a str> {
            text.split_whitespace().nth(index)
        }
        
        // 测试函数
        fn test_extract_word() {
            let text = "hello world rust programming";
            
            assert_eq!(extract_word(text, 0), Some("hello"));
            assert_eq!(extract_word(text, 2), Some("rust"));
            assert_eq!(extract_word(text, 10), None);
            
            println!("   ✅ 所有测试通过");
        }
        
        test_extract_word();
    }
    
    println!("   💡 为涉及生命周期的函数编写全面的测试");
}

/// 7. 常见反模式
fn common_antipatterns() {
    println!("\n🚫 7. 常见反模式");
    println!("识别和避免常见的反模式和错误用法。");
    
    unnecessary_cloning();
    overuse_of_static_lifetime();
    complex_lifetime_hierarchies();
    premature_optimization();
}

/// 不必要的克隆
fn unnecessary_cloning() {
    println!("\n📄 不必要的克隆:");
    
    // ❌ 反模式：过度克隆
    {
        println!("   ❌ 反模式 - 过度克隆:");
        
        fn bad_process_strings(strings: Vec<String>) -> Vec<String> {
            let mut result = Vec::new();
            for s in strings {
                // 不必要的克隆
                let cloned = s.clone();
                if cloned.len() > 3 {
                    result.push(cloned);
                }
            }
            result
        }
        
        let test_strings = vec![
            "hi".to_string(),
            "hello".to_string(),
            "world".to_string(),
        ];
        
        let bad_result = bad_process_strings(test_strings.clone());
        println!("   📋 过度克隆结果: {:?}", bad_result);
    }
    
    // ✅ 好的实践：避免不必要的克隆
    {
        println!("   ✅ 好的实践 - 避免不必要的克隆:");
        
        fn good_process_strings(strings: Vec<String>) -> Vec<String> {
            strings.into_iter().filter(|s| s.len() > 3).collect()
        }
        
        let test_strings = vec![
            "hi".to_string(),
            "hello".to_string(),
            "world".to_string(),
        ];
        
        let good_result = good_process_strings(test_strings);
        println!("   📋 优化后结果: {:?}", good_result);
    }
    
    println!("   💡 避免不必要的克隆，使用移动语义和借用");
}

/// 过度使用静态生命周期
fn overuse_of_static_lifetime() {
    println!("\n🌟 过度使用静态生命周期:");
    
    // ❌ 反模式：过度使用 'static
    {
        println!("   ❌ 反模式 - 过度使用 'static:");
        
        // 这种设计限制了灵活性
        struct BadConfig {
            name: &'static str,
            description: &'static str,
        }
        
        let bad_config = BadConfig {
            name: "service",
            description: "A service configuration",
        };
        
        println!("   ⚙️ 受限配置: {} - {}", bad_config.name, bad_config.description);
    }
    
    // ✅ 好的实践：使用拥有的数据
    {
        println!("   ✅ 好的实践 - 使用拥有的数据:");
        
        #[derive(Debug)]
        struct GoodConfig {
            name: String,
            description: String,
        }
        
        impl GoodConfig {
            fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
                GoodConfig {
                    name: name.into(),
                    description: description.into(),
                }
            }
        }
        
        let good_config = GoodConfig::new("dynamic_service", "A flexible service configuration");
        println!("   ⚙️ 灵活配置: {:?}", good_config);
    }
    
    println!("   💡 只在真正需要时使用 'static 生命周期");
}

/// 复杂的生命周期层次
fn complex_lifetime_hierarchies() {
    println!("\n🕸️ 复杂的生命周期层次:");
    
    // ❌ 反模式：过度复杂的生命周期
    {
        println!("   ❌ 反模式 - 过度复杂的生命周期:");
        
        // 这种设计过于复杂
        struct OverComplexStruct<'a, 'b, 'c> {
            field1: &'a str,
            field2: &'b str,
            field3: &'c str,
        }
        
        // 使用起来很困难
        let s1 = "first";
        let s2 = "second";
        let s3 = "third";
        
        let _complex = OverComplexStruct {
            field1: s1,
            field2: s2,
            field3: s3,
        };
        
        println!("   🕷️ 创建了过度复杂的结构");
    }
    
    // ✅ 好的实践：简化设计
    {
        println!("   ✅ 好的实践 - 简化设计:");
        
        #[derive(Debug)]
        struct SimpleStruct {
            field1: String,
            field2: String,
            field3: String,
        }
        
        let simple = SimpleStruct {
            field1: "first".to_string(),
            field2: "second".to_string(),
            field3: "third".to_string(),
        };
        
        println!("   🎯 简单结构: {:?}", simple);
    }
    
    println!("   💡 优先考虑简单的设计，避免复杂的生命周期层次");
}

/// 过早优化
fn premature_optimization() {
    println!("\n⚡ 过早优化:");
    
    // ❌ 反模式：过早的微优化
    {
        println!("   ❌ 反模式 - 过早的微优化:");
        
        // 为了避免一次克隆而使代码复杂化
        fn over_optimized_function<'a>(data: &'a [String]) -> Vec<&'a str> {
            // 复杂的生命周期管理只是为了避免克隆
            data.iter().map(|s| s.as_str()).collect()
        }
        
        let strings = vec!["hello".to_string(), "world".to_string()];
        let result = over_optimized_function(&strings);
        
        println!("   🔧 过度优化结果: {:?}", result);
    }
    
    // ✅ 好的实践：先保证正确性
    {
        println!("   ✅ 好的实践 - 先保证正确性:");
        
        fn simple_function(data: &[String]) -> Vec<String> {
            // 简单直接的实现
            data.iter().cloned().collect()
        }
        
        let strings = vec!["hello".to_string(), "world".to_string()];
        let result = simple_function(&strings);
        
        println!("   🎯 简单实现结果: {:?}", result);
    }
    
    println!("   💡 先让代码工作，再考虑优化");
}

/// 8. 测试策略
fn testing_strategies() {
    println!("\n🧪 8. 测试策略");
    println!("针对作用域、生命周期和 NLL 的测试策略。");
    
    unit_testing_approaches();
    integration_testing_considerations();
    property_based_testing();
    benchmarking_best_practices();
}

/// 单元测试方法
fn unit_testing_approaches() {
    println!("\n🔬 单元测试方法:");
    
    // 测试借用相关的函数
    {
        println!("   📋 测试借用相关的函数:");
        
        fn find_max_length<'a>(strings: &'a [&str]) -> Option<&'a str> {
            strings.iter().max_by_key(|s| s.len()).copied()
        }
        
        // 测试函数
        fn test_find_max_length() {
            let empty: &[&str] = &[];
            assert_eq!(find_max_length(empty), None);
            
            let single = &["hello"];
            assert_eq!(find_max_length(single), Some("hello"));
            
            let multiple = &["hi", "hello", "world", "rust"];
            assert_eq!(find_max_length(multiple), Some("world"));  // "hello" 和 "world" 都是5个字符，max_by_key返回最后一个
            
            println!("   ✅ find_max_length 测试通过");
        }
        
        test_find_max_length();
    }
    
    // 测试所有权转移
    {
        println!("   📦 测试所有权转移:");
        
        fn take_and_return(mut data: Vec<i32>) -> Vec<i32> {
            data.push(42);
            data
        }
        
        fn test_take_and_return() {
            let original = vec![1, 2, 3];
            let result = take_and_return(original);
            
            assert_eq!(result, vec![1, 2, 3, 42]);
            // 注意：original 在这里不再可用
            
            println!("   ✅ take_and_return 测试通过");
        }
        
        test_take_and_return();
    }
    
    println!("   💡 为涉及借用和所有权的函数编写详细的单元测试");
}

/// 集成测试考虑
fn integration_testing_considerations() {
    println!("\n🔗 集成测试考虑:");
    
    // 测试组件间的生命周期交互
    {
        println!("   🧩 测试组件间的生命周期交互:");
        
        struct DataStore {
            data: Vec<String>,
        }
        
        impl DataStore {
            fn new() -> Self {
                DataStore { data: Vec::new() }
            }
            
            fn add(&mut self, item: String) {
                self.data.push(item);
            }
            
            fn get(&self, index: usize) -> Option<&str> {
                self.data.get(index).map(|s| s.as_str())
            }
        }
        
        struct DataProcessor<'a> {
            store: &'a DataStore,
        }
        
        impl<'a> DataProcessor<'a> {
            fn new(store: &'a DataStore) -> Self {
                DataProcessor { store }
            }
            
            fn process(&self, index: usize) -> Option<String> {
                self.store.get(index).map(|s| format!("Processed: {}", s))
            }
        }
        
        // 集成测试
        fn test_data_store_integration() {
            let mut store = DataStore::new();
            store.add("item1".to_string());
            store.add("item2".to_string());
            
            let processor = DataProcessor::new(&store);
            
            assert_eq!(processor.process(0), Some("Processed: item1".to_string()));
            assert_eq!(processor.process(1), Some("Processed: item2".to_string()));
            assert_eq!(processor.process(2), None);
            
            println!("   ✅ 数据存储集成测试通过");
        }
        
        test_data_store_integration();
    }
    
    println!("   💡 测试组件间的生命周期和借用关系");
}

/// 基于属性的测试
fn property_based_testing() {
    println!("\n🎲 基于属性的测试:");
    
    // 测试不变量
    {
        println!("   ⚖️ 测试不变量:");
        
        fn safe_divide(a: f64, b: f64) -> Option<f64> {
            if b.abs() < f64::EPSILON {
                None
            } else {
                Some(a / b)
            }
        }
        
        // 属性：非零除法总是产生有限结果
        fn test_division_properties() {
            let test_cases = vec![
                (10.0, 2.0, true),
                (5.0, 0.0, false),
                (0.0, 5.0, true),
                (-10.0, 2.0, true),
            ];
            
            for (a, b, should_succeed) in test_cases {
                let result = safe_divide(a, b);
                
                if should_succeed {
                    assert!(result.is_some());
                    let value = result.unwrap();
                    assert!(value.is_finite());
                } else {
                    assert!(result.is_none());
                }
            }
            
            println!("   ✅ 除法属性测试通过");
        }
        
        test_division_properties();
    }
    
    println!("   💡 使用基于属性的测试验证代码的不变量");
}

/// 基准测试最佳实践
fn benchmarking_best_practices() {
    println!("\n📊 基准测试最佳实践:");
    
    // 比较不同实现的性能
    {
        println!("   ⚡ 比较不同实现的性能:");
        
        use std::time::Instant;
        
        // 实现1：使用克隆
        fn process_with_clone(data: &[String]) -> Vec<String> {
            data.iter().cloned().map(|s| s.to_uppercase()).collect()
        }
        
        // 实现2：使用借用
        fn process_with_borrow(data: &[String]) -> Vec<String> {
            data.iter().map(|s| s.to_uppercase()).collect()
        }
        
        let test_data: Vec<String> = (0..1000)
            .map(|i| format!("item_{}", i))
            .collect();
        
        // 基准测试实现1
        let start = Instant::now();
        let _result1 = process_with_clone(&test_data);
        let duration1 = start.elapsed();
        
        // 基准测试实现2
        let start = Instant::now();
        let _result2 = process_with_borrow(&test_data);
        let duration2 = start.elapsed();
        
        println!("   📈 克隆实现耗时: {:?}", duration1);
        println!("   📈 借用实现耗时: {:?}", duration2);
        
        if duration2 < duration1 {
            println!("   🏆 借用实现更快");
        } else {
            println!("   🏆 克隆实现更快");
        }
    }
    
    println!("   💡 使用基准测试验证性能优化的效果");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_best_practices_analysis() {
        // 测试最佳实践分析是否能正常运行
        run_best_practices_analysis();
    }

    #[test]
    fn test_scope_best_practices() {
        // 测试作用域最佳实践
        scope_best_practices();
    }

    #[test]
    fn test_lifetime_best_practices() {
        // 测试生命周期最佳实践
        lifetime_best_practices();
    }

    #[test]
    fn test_nll_best_practices() {
        // 测试 NLL 最佳实践
        nll_best_practices();
    }

    #[test]
    fn test_performance_optimization() {
        // 测试性能优化
        performance_optimization();
    }
}