//! # NLL (Non-Lexical Lifetimes) 深度分析
//!
//! 非词法生命周期 (NLL) 是Rust 2018引入的一项重要改进，它使借用检查器更加智能和精确。
//! NLL允许编译器更准确地分析引用的实际使用范围，而不是仅仅依赖词法作用域。
//!
//! ## NLL的核心改进
//!
//! 1. **更精确的生命周期分析**: 基于实际使用而非词法作用域
//! 2. **减少不必要的借用检查错误**: 允许更多安全的代码模式
//! 3. **改进的错误消息**: 更清晰的借用检查错误提示
//! 4. **更好的性能**: 减少不必要的克隆和复制
//! 5. **向后兼容**: 不破坏现有代码的同时提供更好的体验

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// 运行所有NLL示例
pub fn run_nll_examples() {
    println!("\n=== NLL (Non-Lexical Lifetimes) 深度分析 ===");
    
    nll_basic_concepts();
    lexical_vs_non_lexical();
    nll_improvements();
    nll_with_mutable_references();
    nll_with_loops();
    nll_with_conditionals();
    nll_with_match_expressions();
    nll_error_improvements();
    nll_performance_benefits();
    nll_limitations();
}

/// 1. NLL基本概念
fn nll_basic_concepts() {
    println!("\n🚀 1. NLL基本概念");
    println!("NLL使借用检查器能够更精确地分析引用的实际生命周期。");
    
    // 传统词法生命周期的限制
    println!("\n📚 传统词法生命周期 vs NLL:");
    
    // 在NLL之前，这种代码可能不会编译
    let mut data = vec![1, 2, 3, 4, 5];
    
    // 创建一个不可变引用
    let first = &data[0];
    println!("第一个元素: {}", first);
    
    // 在NLL中，这是安全的，因为first在这之后不再使用
    data.push(6);
    println!("修改后的数据: {:?}", data);
    
    // 演示NLL的核心原理
    demonstrate_nll_core_principle();
}

/// 演示NLL的核心原理
fn demonstrate_nll_core_principle() {
    println!("\n🎯 NLL核心原理 - 基于使用的生命周期:");
    
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    
    // 获取一个引用
    let value_ref = map.get("key1");
    
    // 在传统系统中，这里可能会有借用检查错误
    // 但在NLL中，由于value_ref在下面的match中使用完毕，这是安全的
    match value_ref {
        Some(value) => {
            println!("找到值: {}", value);
            // value_ref的生命周期在这里结束
        }
        None => println!("未找到值"),
    }
    
    // 现在可以安全地修改map，因为value_ref不再使用
    map.insert("key3", "value3");
    println!("修改后的map: {:?}", map);
    
    println!("✅ NLL允许编译器理解引用的实际使用范围");
}

/// 2. 词法生命周期 vs 非词法生命周期对比
fn lexical_vs_non_lexical() {
    println!("\n⚖️  2. 词法生命周期 vs 非词法生命周期");
    println!("对比传统词法生命周期和NLL的差异。");
    
    // 示例1: 早期返回的情况
    demonstrate_early_return_scenario();
    
    // 示例2: 条件借用的情况
    demonstrate_conditional_borrowing();
    
    // 示例3: 循环中的借用
    demonstrate_loop_borrowing();
}

/// 演示早期返回场景
fn demonstrate_early_return_scenario() {
    println!("\n🔄 早期返回场景:");
    
    fn process_data(data: &mut Vec<i32>) -> Option<i32> {
        // 创建一个不可变引用并立即复制值
        let first_value = *data.get(0)?;
        
        // 在传统系统中，这里可能有问题
        // 但在NLL中，如果first不再使用，这是安全的
        if first_value == 0 {
            return None; // 早期返回
        }
        
        // 现在可以安全地修改data，因为我们使用的是值而不是引用
        data.push(first_value * 2);
        Some(first_value)
    }
    
    let mut numbers = vec![5, 10, 15];
    match process_data(&mut numbers) {
        Some(result) => println!("处理结果: {}, 修改后数据: {:?}", result, numbers),
        None => println!("处理失败"),
    }
    
    println!("✅ NLL支持更灵活的早期返回模式");
}

/// 演示条件借用
fn demonstrate_conditional_borrowing() {
    println!("\n🔀 条件借用场景:");
    
    let mut data = String::from("Hello");
    let should_read = true;
    
    if should_read {
        // 创建不可变引用
        let content = &data;
        println!("读取内容: {}", content);
        // content的生命周期在这个if块结束时结束
    }
    
    // 在NLL中，这是安全的，因为content不再存在
    data.push_str(", World!");
    println!("修改后的数据: {}", data);
    
    // 更复杂的条件借用
    let condition = data.len() > 5;
    let reference = if condition {
        &data[0..5]
    } else {
        &data[..]
    };
    
    println!("条件引用: {}", reference);
    // reference的使用结束后，可以继续修改data
    
    println!("✅ NLL改善了条件借用的灵活性");
}

/// 演示循环中的借用
fn demonstrate_loop_borrowing() {
    println!("\n🔁 循环中的借用:");
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // 在循环中使用引用
    for i in 0..numbers.len() {
        let current = &numbers[i];
        println!("当前元素: {}", current);
        
        // 在传统系统中，这里可能有借用检查问题
        // 但在NLL中，由于current在每次迭代结束时不再使用，这是安全的
        if *current % 2 == 0 {
            // 注意：这里我们不能直接修改numbers，因为我们在迭代它
            // 但我们可以收集需要修改的索引
        }
    }
    
    // 循环结束后，可以安全地修改
    numbers.push(6);
    println!("修改后的数组: {:?}", numbers);
    
    // 更安全的循环修改模式
    let mut to_double = Vec::new();
    for (index, value) in numbers.iter().enumerate() {
        if *value < 3 {
            to_double.push(index);
        }
    }
    
    // 现在安全地修改
    for &index in to_double.iter().rev() {
        numbers[index] *= 2;
    }
    
    println!("加倍后的数组: {:?}", numbers);
    println!("✅ NLL改善了循环中的借用处理");
}

/// 3. NLL的具体改进
fn nll_improvements() {
    println!("\n📈 3. NLL的具体改进");
    println!("NLL在多个方面改进了Rust的借用检查。");
    
    // 改进1: 减少不必要的克隆
    demonstrate_reduced_cloning();
    
    // 改进2: 更好的API设计
    demonstrate_better_api_design();
    
    // 改进3: 简化的错误处理
    demonstrate_simplified_error_handling();
}

/// 演示减少不必要的克隆
fn demonstrate_reduced_cloning() {
    println!("\n📋 减少不必要的克隆:");
    
    let mut cache = HashMap::new();
    cache.insert("expensive_computation", 42);
    
    // 在NLL之前，可能需要克隆来避免借用检查错误
    // let value = cache.get("expensive_computation").cloned();
    
    // 在NLL中，可以直接使用引用
    if let Some(cached_value) = cache.get("expensive_computation") {
        println!("使用缓存值: {}", cached_value);
        // cached_value的使用在这里结束
    }
    
    // 现在可以安全地修改cache
    cache.insert("new_computation", 84);
    println!("更新后的缓存: {:?}", cache);
    
    println!("✅ NLL减少了不必要的克隆操作");
}

/// 演示更好的API设计
fn demonstrate_better_api_design() {
    println!("\n🎨 更好的API设计:");
    
    struct DataProcessor {
        data: Vec<String>,
    }
    
    impl DataProcessor {
        fn new() -> Self {
            DataProcessor {
                data: vec!["item1".to_string(), "item2".to_string()],
            }
        }
        
        // 在NLL中，这种API设计更加自然
        fn process_and_add(&mut self, new_item: String) -> Option<&str> {
            // 先检查现有数据
            let _first_item = self.data.get(0)?;
            
            // 在传统系统中，这里可能有借用检查问题
            // 但在NLL中，由于first_item在下面不再使用，这是安全的
            self.data.push(new_item);
            
            // 返回新添加的项的引用
            self.data.last().map(|s| s.as_str())
        }
        
        fn get_data(&self) -> &[String] {
            &self.data
        }
    }
    
    let mut processor = DataProcessor::new();
    
    if let Some(new_item) = processor.process_and_add("item3".to_string()) {
        println!("添加的新项: {}", new_item);
    }
    
    println!("所有数据: {:?}", processor.get_data());
    println!("✅ NLL支持更自然的API设计");
}

/// 演示简化的错误处理
fn demonstrate_simplified_error_handling() {
    println!("\n🛠️  简化的错误处理:");
    
    fn process_config(config: &mut HashMap<String, String>) -> Result<(), String> {
        // 检查必需的配置项
        let required_key = "database_url";
        let db_url = config.get(required_key)
            .ok_or_else(|| format!("缺少必需的配置: {}", required_key))?;
        
        // 验证配置值
        if db_url.is_empty() {
            return Err("数据库URL不能为空".to_string());
        }
        
        // 在NLL中，db_url的使用结束后，可以安全地修改config
        config.insert("processed".to_string(), "true".to_string());
        
        Ok(())
    }
    
    let mut config = HashMap::new();
    config.insert("database_url".to_string(), "postgresql://localhost".to_string());
    
    match process_config(&mut config) {
        Ok(()) => println!("配置处理成功: {:?}", config),
        Err(e) => println!("配置处理失败: {}", e),
    }
    
    println!("✅ NLL简化了错误处理模式");
}

/// 4. NLL与可变引用
fn nll_with_mutable_references() {
    println!("\n🔧 4. NLL与可变引用");
    println!("NLL对可变引用的处理更加智能和灵活。");
    
    let mut data = vec![1, 2, 3, 4, 5];
    
    // 创建可变引用
    let mutable_ref = &mut data;
    
    // 使用可变引用
    mutable_ref.push(6);
    println!("通过可变引用修改: {:?}", mutable_ref);
    
    // 在NLL中，mutable_ref的生命周期在这里结束
    // 所以可以创建新的引用
    let immutable_ref = &data;
    println!("创建不可变引用: {:?}", immutable_ref);
    
    // 演示更复杂的可变引用场景
    demonstrate_complex_mutable_scenarios();
}

/// 演示复杂的可变引用场景
fn demonstrate_complex_mutable_scenarios() {
    println!("\n🎯 复杂的可变引用场景:");
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // 场景1: 条件性可变借用
    let should_modify = numbers.len() < 10;
    if should_modify {
        let mutable_slice = &mut numbers[1..4];
        for item in mutable_slice {
            *item *= 2;
        }
        println!("条件修改后: {:?}", numbers);
    }
    
    // 场景2: 分离的可变借用
    let (left, right) = numbers.split_at_mut(3);
    left[0] = 100;
    right[0] = 200;
    println!("分离修改后: {:?}", numbers);
    
    // 场景3: 迭代器中的可变引用
    for (index, value) in numbers.iter_mut().enumerate() {
        if index % 2 == 0 {
            *value += 10;
        }
    }
    println!("迭代器修改后: {:?}", numbers);
    
    println!("✅ NLL改善了可变引用的使用体验");
}

/// 5. NLL与循环
fn nll_with_loops() {
    println!("\n🔁 5. NLL与循环");
    println!("NLL在循环中提供了更精确的借用分析。");
    
    // while循环中的NLL
    demonstrate_nll_in_while_loops();
    
    // for循环中的NLL
    demonstrate_nll_in_for_loops();
    
    // loop循环中的NLL
    demonstrate_nll_in_loop();
}

/// 演示while循环中的NLL
fn demonstrate_nll_in_while_loops() {
    println!("\n🔄 while循环中的NLL:");
    
    let mut data = vec!["hello", "world", "rust", "nll"];
    let mut index = 0;
    
    while index < data.len() {
        let current = &data[index];
        println!("处理: {}", current);
        
        // 在NLL中，current的生命周期在这里结束
        // 所以可以安全地修改data（虽然在while循环中要小心）
        if current.len() > 4 {
            // 注意：这里我们不直接修改data，因为会影响循环条件
            println!("找到长字符串: {}", current);
        }
        
        index += 1;
    }
    
    // 循环结束后可以安全修改
    data.push("added");
    println!("修改后的数据: {:?}", data);
    
    println!("✅ NLL改善了while循环中的借用处理");
}

/// 演示for循环中的NLL
fn demonstrate_nll_in_for_loops() {
    println!("\n🔁 for循环中的NLL:");
    
    let mut items = vec!["apple", "banana", "cherry"];
    
    // 使用索引的for循环
    for i in 0..items.len() {
        let item = &items[i];
        println!("索引 {}: {}", i, item);
        // item的生命周期在每次迭代结束时结束
    }
    
    // 现在可以修改items
    items.push("date");
    
    // 使用迭代器的for循环
    for (index, item) in items.iter().enumerate() {
        println!("迭代器 {}: {}", index, item);
    }
    
    println!("最终数据: {:?}", items);
    println!("✅ NLL在for循环中提供了更好的灵活性");
}

/// 演示loop循环中的NLL
fn demonstrate_nll_in_loop() {
    println!("\n♾️  loop循环中的NLL:");
    
    let mut counter = 0;
    let mut data = vec![10, 20, 30];
    
    loop {
        if counter >= data.len() {
            break;
        }
        
        let current = &data[counter];
        println!("循环 {}: {}", counter, current);
        
        // current的使用结束
        counter += 1;
        
        if counter == 2 {
            // 在NLL中，这是安全的，因为current不再使用
            data.push(40);
        }
    }
    
    println!("循环后的数据: {:?}", data);
    println!("✅ NLL在无限循环中也能正确分析生命周期");
}

/// 6. NLL与条件语句
fn nll_with_conditionals() {
    println!("\n🔀 6. NLL与条件语句");
    println!("NLL在条件语句中提供了更精确的分析。");
    
    // if-else中的NLL
    demonstrate_nll_in_if_else();
    
    // 嵌套条件中的NLL
    demonstrate_nll_in_nested_conditions();
}

/// 演示if-else中的NLL
fn demonstrate_nll_in_if_else() {
    println!("\n🔀 if-else中的NLL:");
    
    let mut data = String::from("Hello");
    let condition = data.len() > 3;
    
    if condition {
        let reference = &data;
        println!("条件为真，数据: {}", reference);
        // reference的生命周期在这个分支结束时结束
    } else {
        let reference = &data;
        println!("条件为假，数据: {}", reference);
        // 这个reference也在分支结束时结束
    }
    
    // 在NLL中，所有分支的引用都已结束，可以安全修改
    data.push_str(", World!");
    println!("修改后: {}", data);
    
    // 更复杂的条件借用
    let result = if data.contains("Hello") {
        &data[0..5]
    } else {
        &data[..]
    };
    
    println!("条件结果: {}", result);
    // result使用完毕后，可以继续操作data
    
    println!("✅ NLL改善了条件语句中的借用处理");
}

/// 演示嵌套条件中的NLL
fn demonstrate_nll_in_nested_conditions() {
    println!("\n🎯 嵌套条件中的NLL:");
    
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    if numbers.len() > 3 {
        let slice = &numbers[1..4];
        
        if slice.len() == 3 {
            let first = &slice[0];
            println!("嵌套条件中的第一个元素: {}", first);
            
            if *first > 1 {
                println!("元素大于1");
            }
            // first的生命周期在这里结束
        }
        // slice的生命周期在这里结束
    }
    
    // 所有引用都已结束，可以安全修改
    numbers.push(6);
    println!("修改后的数组: {:?}", numbers);
    
    println!("✅ NLL正确处理了嵌套条件中的复杂借用关系");
}

/// 7. NLL与match表达式
fn nll_with_match_expressions() {
    println!("\n🎯 7. NLL与match表达式");
    println!("NLL在match表达式中提供了更智能的分析。");
    
    // 基本match中的NLL
    demonstrate_nll_in_basic_match();
    
    // 复杂match模式中的NLL
    demonstrate_nll_in_complex_match();
}

/// 演示基本match中的NLL
fn demonstrate_nll_in_basic_match() {
    println!("\n🎯 基本match中的NLL:");
    
    let mut data = Some(String::from("Hello, Rust!"));
    
    match &data {
        Some(content) => {
            println!("匹配到内容: {}", content);
            // content的生命周期在这个分支结束时结束
        }
        None => {
            println!("没有内容");
        }
    }
    
    // 在NLL中，match中的引用已经结束，可以安全修改
    if let Some(ref mut content) = data {
        content.push_str(" - Modified");
    }
    
    println!("修改后的数据: {:?}", data);
    
    // 更复杂的match场景
    let numbers = vec![1, 2, 3, 4, 5];
    
    match numbers.get(2) {
        Some(value) => {
            println!("第三个元素: {}", value);
            // value的使用在这里结束
        }
        None => println!("没有第三个元素"),
    }
    
    // 可以继续使用numbers
    println!("完整数组: {:?}", numbers);
    
    println!("✅ NLL改善了match表达式中的借用处理");
}

/// 演示复杂match模式中的NLL
fn demonstrate_nll_in_complex_match() {
    println!("\n🔄 复杂match模式中的NLL:");
    
    #[derive(Debug)]
    enum DataType {
        Text(String),
        Numbers(Vec<i32>),
        Mixed(String, Vec<i32>),
    }
    
    let mut data = DataType::Mixed(
        "标签".to_string(),
        vec![1, 2, 3]
    );
    
    match &data {
        DataType::Text(text) => {
            println!("文本数据: {}", text);
        }
        DataType::Numbers(nums) => {
            println!("数字数据: {:?}", nums);
        }
        DataType::Mixed(label, numbers) => {
            println!("混合数据 - 标签: {}, 数字: {:?}", label, numbers);
            // label和numbers的生命周期在这里结束
        }
    }
    
    // 在NLL中，match中的引用已结束，可以安全修改
    match &mut data {
        DataType::Mixed(label, numbers) => {
            label.push_str(" - 修改");
            numbers.push(4);
        }
        _ => {}
    }
    
    println!("修改后的数据: {:?}", data);
    
    println!("✅ NLL支持复杂match模式中的灵活借用");
}

/// 8. NLL错误消息改进
fn nll_error_improvements() {
    println!("\n📝 8. NLL错误消息改进");
    println!("NLL提供了更清晰、更有帮助的错误消息。");
    
    // 演示改进的错误消息（这些代码被注释掉，因为它们不会编译）
    demonstrate_improved_error_messages();
}

/// 演示改进的错误消息
fn demonstrate_improved_error_messages() {
    println!("\n💬 改进的错误消息示例:");
    
    // 在NLL之前，错误消息可能不够清晰
    // 现在的错误消息更加具体和有帮助
    
    println!("NLL改进了以下方面的错误消息:");
    println!("1. 更精确的生命周期信息");
    println!("2. 更清晰的借用冲突描述");
    println!("3. 更好的修复建议");
    println!("4. 更准确的错误位置标注");
    
    // 示例：改进的借用检查错误
    /*
    // 这种代码在NLL之前可能产生令人困惑的错误消息
    let mut data = vec![1, 2, 3];
    let reference = &data[0];
    data.push(4); // 在NLL中，如果reference不再使用，这是允许的
    println!("{}", reference); // 但如果在这里使用，就会有清晰的错误消息
    */
    
    println!("✅ NLL的错误消息更加用户友好");
}

/// 9. NLL性能优势
fn nll_performance_benefits() {
    println!("\n⚡ 9. NLL性能优势");
    println!("NLL不仅改善了开发体验，还带来了性能优势。");
    
    // 减少不必要的克隆
    demonstrate_clone_reduction();
    
    // 更好的内存使用
    demonstrate_memory_efficiency();
    
    // 编译时优化
    demonstrate_compile_time_optimizations();
}

/// 演示克隆减少
fn demonstrate_clone_reduction() {
    println!("\n📋 减少克隆操作:");
    
    let mut cache: HashMap<String, Vec<i32>> = HashMap::new();
    cache.insert("data1".to_string(), vec![1, 2, 3]);
    cache.insert("data2".to_string(), vec![4, 5, 6]);
    
    // 在NLL之前，可能需要克隆来避免借用检查错误
    // let data = cache.get("data1").cloned().unwrap_or_default();
    
    // 在NLL中，可以直接使用引用，减少克隆
    if let Some(data) = cache.get("data1") {
        let sum: i32 = data.iter().sum();
        println!("数据总和: {}", sum);
        // data的使用在这里结束
    }
    
    // 现在可以安全地修改cache
    cache.insert("data3".to_string(), vec![7, 8, 9]);
    
    println!("缓存大小: {}", cache.len());
    println!("✅ NLL减少了不必要的克隆，提高了性能");
}

/// 演示内存效率
fn demonstrate_memory_efficiency() {
    println!("\n💾 内存使用效率:");
    
    // NLL允许更精确的内存管理
    let mut large_data = vec![0; 1000]; // 模拟大数据
    
    // 处理数据的一部分
    {
        let slice = &large_data[100..200];
        let processed: Vec<i32> = slice.iter().map(|&x| x + 1).collect();
        println!("处理了 {} 个元素", processed.len());
        // slice和processed在这里被释放
    }
    
    // 在NLL中，可以更早地释放不需要的引用
    // 从而减少内存压力
    large_data.truncate(500); // 减少数据大小
    
    println!("优化后数据大小: {}", large_data.len());
    println!("✅ NLL支持更精确的内存管理");
}

/// 演示编译时优化
fn demonstrate_compile_time_optimizations() {
    println!("\n🚀 编译时优化:");
    
    // NLL允许编译器进行更多优化
    let data = vec![1, 2, 3, 4, 5];
    
    // 更精确的生命周期分析允许编译器优化内存访问
    let result = {
        let slice = &data[1..4];
        slice.iter().sum::<i32>()
    }; // slice的生命周期在这里结束，编译器可以优化
    
    println!("计算结果: {}", result);
    
    // 编译器可以更好地优化这种模式
    let optimized_result = data[1..4].iter().sum::<i32>();
    println!("优化后结果: {}", optimized_result);
    
    println!("✅ NLL使编译器能够进行更多优化");
}

/// 10. NLL的限制
fn nll_limitations() {
    println!("\n⚠️  10. NLL的限制");
    println!("虽然NLL带来了很多改进，但仍有一些限制。");
    
    // 仍然存在的借用检查限制
    demonstrate_remaining_limitations();
    
    // 复杂场景中的限制
    demonstrate_complex_limitations();
}

/// 演示仍然存在的限制
fn demonstrate_remaining_limitations() {
    println!("\n🚫 仍然存在的限制:");
    
    let mut data = vec![1, 2, 3, 4, 5];
    
    // 限制1: 同时存在可变和不可变引用仍然不被允许
    let immutable_ref = &data;
    // let mutable_ref = &mut data; // 这仍然会导致编译错误
    println!("不可变引用: {:?}", immutable_ref);
    
    // 限制2: 跨函数边界的复杂借用仍然有限制
    fn complex_borrowing_scenario(data: &mut Vec<i32>) -> &i32 {
        data.push(6);
        &data[0] // 这可能在某些复杂情况下仍有限制
    }
    
    let first = complex_borrowing_scenario(&mut data);
    println!("第一个元素: {}", first);
    
    // 限制3: 某些高级模式仍需要显式生命周期
    println!("\n一些仍然存在的限制:");
    println!("1. 同时可变和不可变借用的限制");
    println!("2. 某些跨函数的复杂借用模式");
    println!("3. 高级生命周期模式仍需显式标注");
    
    println!("✅ 理解NLL的限制有助于更好地使用Rust");
}

/// 演示复杂场景中的限制
fn demonstrate_complex_limitations() {
    println!("\n🎯 复杂场景中的限制:");
    
    // 使用Rc和RefCell来演示一些仍然需要特殊处理的场景
    let data = Rc::new(RefCell::new(vec![1, 2, 3]));
    
    {
        let borrowed = data.borrow();
        println!("借用的数据: {:?}", *borrowed);
        // 在这个作用域中，不能进行可变借用
    }
    
    // 现在可以进行可变借用
    {
        let mut borrowed_mut = data.borrow_mut();
        borrowed_mut.push(4);
        println!("修改后的数据长度: {}", borrowed_mut.len());
    }
    
    println!("最终数据: {:?}", data.borrow());
    
    println!("\n复杂场景的限制:");
    println!("1. 运行时借用检查仍然存在");
    println!("2. 某些并发模式需要特殊处理");
    println!("3. 跨线程的借用仍有严格限制");
    
    println!("✅ NLL虽然强大，但理解其限制同样重要");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nll_examples() {
        // 测试所有NLL示例是否能正常运行
        run_nll_examples();
    }

    #[test]
    fn test_nll_basic_pattern() {
        let mut data = vec![1, 2, 3];
        let first = &data[0];
        // 在NLL中，如果first不再使用，这是安全的
        drop(first); // 显式结束first的生命周期
        data.push(4);
        assert_eq!(data.len(), 4);
    }

    #[test]
    fn test_nll_conditional_borrowing() {
        let mut data = String::from("test");
        let condition = true;
        
        if condition {
            let reference = &data;
            assert_eq!(reference, "test");
        }
        
        // 在NLL中，这是安全的
        data.push_str("_modified");
        assert_eq!(data, "test_modified");
    }
}