//! # 作用域、生命周期和NLL的深度对比分析
//!
//! 本模块提供了作用域(Scope)、生命周期(Lifetime)和非词法生命周期(NLL)之间的
//! 详细对比分析，帮助理解它们的区别、联系和相互作用。
//!
//! ## 核心概念对比
//!
//! | 概念 | 定义 | 作用时机 | 主要目的 |
//! |------|------|----------|----------|
//! | 作用域 | 变量可见性和有效性的代码区域 | 编译时 | 内存管理和变量访问控制 |
//! | 生命周期 | 引用有效性的时间范围 | 编译时 | 防止悬垂指针和内存安全 |
//! | NLL | 基于实际使用的生命周期分析 | 编译时 | 更精确的借用检查 |

#[allow(unused_imports)] // 示例代码中可能使用
use std::collections::HashMap;
#[allow(unused_imports)] // 示例代码中可能使用
use std::rc::Rc;
#[allow(unused_imports)] // 示例代码中可能使用
use std::cell::RefCell;

/// 运行所有对比分析示例
pub fn run_comparison_examples() {
    println!("\n=== 作用域、生命周期和NLL的深度对比分析 ===");
    
    fundamental_differences();
    interaction_patterns();
    evolution_timeline();
    practical_implications();
    common_misconceptions();
    advanced_comparisons();
    real_world_scenarios();
    performance_impact_comparison();
    debugging_differences();
    best_practice_comparison();
}

/// 1. 基本概念差异
fn fundamental_differences() {
    println!("\n🔍 1. 基本概念差异");
    println!("深入理解三个概念的本质区别。");
    
    // 作用域的基本特征
    demonstrate_scope_characteristics();
    
    // 生命周期的基本特征
    demonstrate_lifetime_characteristics();
    
    // NLL的基本特征
    demonstrate_nll_characteristics();
    
    // 三者的关系图
    show_relationship_diagram();
}

/// 演示作用域的基本特征
fn demonstrate_scope_characteristics() {
    println!("\n📦 作用域特征:");
    
    {
        let scope_var = "我在内部作用域中";
        println!("作用域特征1 - 词法边界: {}", scope_var);
        
        {
            let nested_var = "我在嵌套作用域中";
            println!("作用域特征2 - 嵌套访问: {} 和 {}", scope_var, nested_var);
            // nested_var在这里结束生命
        }
        
        // println!("{}", nested_var); // 编译错误：超出作用域
        println!("作用域特征3 - 严格边界: scope_var仍可访问");
        // scope_var在这里结束生命
    }
    
    // println!("{}", scope_var); // 编译错误：超出作用域
    
    println!("\n作用域的关键特征:");
    println!("✓ 词法边界明确（花括号定义）");
    println!("✓ 嵌套作用域可访问外层变量");
    println!("✓ 变量在作用域结束时自动销毁");
    println!("✓ 编译时确定，运行时执行");
}

/// 演示生命周期的基本特征
fn demonstrate_lifetime_characteristics() {
    println!("\n⏰ 生命周期特征:");
    
    let data = String::from("原始数据");
    
    // 生命周期特征1: 引用的有效期
    let reference1 = &data;
    println!("生命周期特征1 - 引用有效期: {}", reference1);
    
    {
        let reference2 = &data;
        println!("生命周期特征2 - 嵌套引用: {}", reference2);
        // reference2的生命周期在这里结束
    }
    
    // reference1仍然有效
    println!("生命周期特征3 - 引用延续: {}", reference1);
    
    // 演示生命周期约束
    fn demonstrate_lifetime_constraints<'a>(input: &'a str) -> &'a str {
        // 返回的引用必须与输入引用有相同的生命周期
        input.chars().take(2).collect::<String>().leak()
    }
    
    let result = demonstrate_lifetime_constraints(&data);
    println!("生命周期特征4 - 约束关系: {}", result);
    
    println!("\n生命周期的关键特征:");
    println!("✓ 确保引用的安全性");
    println!("✓ 防止悬垂指针");
    println!("✓ 编译时验证");
    println!("✓ 可以显式标注");
}

/// 演示NLL的基本特征
fn demonstrate_nll_characteristics() {
    println!("\n🚀 NLL特征:");
    
    let mut data = vec![1, 2, 3, 4, 5];
    
    // NLL特征1: 基于使用的分析
    let reference = &data[0];
    println!("NLL特征1 - 智能分析: {}", reference);
    // reference在这里最后一次使用，NLL知道它的生命周期可以结束
    
    // NLL特征2: 允许更灵活的借用
    data.push(6); // 在传统系统中可能不被允许
    println!("NLL特征2 - 灵活借用: {:?}", data);
    
    // NLL特征3: 条件性生命周期
    let condition = data.len() > 3;
    let conditional_ref = if condition {
        &data[0]
    } else {
        &data[data.len() - 1]
    };
    
    println!("NLL特征3 - 条件生命周期: {}", conditional_ref);
    // conditional_ref使用完毕，可以继续修改data
    
    data.push(7);
    println!("NLL特征4 - 精确控制: {:?}", data);
    
    println!("\nNLL的关键特征:");
    println!("✓ 基于实际使用分析生命周期");
    println!("✓ 减少不必要的借用检查错误");
    println!("✓ 保持内存安全");
    println!("✓ 向后兼容");
}

/// 显示三者关系图
fn show_relationship_diagram() {
    println!("\n🔗 三者关系图:");
    println!("```");
    println!("┌─────────────┐    ┌──────────────┐    ┌─────────────┐");
    println!("│    作用域    │    │   生命周期    │    │     NLL     │");
    println!("│  (Scope)    │    │ (Lifetime)   │    │    (2018)   │");
    println!("└─────────────┘    └──────────────┘    └─────────────┘");
    println!("       │                   │                   │");
    println!("       │                   │                   │");
    println!("       ▼                   ▼                   ▼");
    println!("  变量可见性           引用有效性           智能分析");
    println!("  内存管理             内存安全             精确控制");
    println!("  词法边界             编译时检查           使用时检查");
    println!("       │                   │                   │");
    println!("       └───────────────────┼───────────────────┘");
    println!("                           │");
    println!("                           ▼");
    println!("                    协同工作确保");
    println!("                    内存安全和性能");
    println!("```");
    
    println!("\n关系说明:");
    println!("• 作用域定义变量的可见范围");
    println!("• 生命周期确保引用的安全性");
    println!("• NLL优化生命周期的分析精度");
    println!("• 三者协同工作，确保Rust的内存安全");
}

/// 2. 交互模式分析
fn interaction_patterns() {
    println!("\n🔄 2. 交互模式分析");
    println!("分析三个概念如何相互作用和影响。");
    
    // 作用域与生命周期的交互
    scope_lifetime_interaction();
    
    // 生命周期与NLL的交互
    lifetime_nll_interaction();
    
    // 三者的综合交互
    comprehensive_interaction();
}

/// 作用域与生命周期的交互
fn scope_lifetime_interaction() {
    println!("\n🔗 作用域与生命周期的交互:");
    
    let outer_data = String::from("外层数据");
    
    {
        // 内层作用域
        let inner_data = String::from("内层数据");
        
        // 创建对外层数据的引用
        let outer_ref = &outer_data;
        
        // 创建对内层数据的引用
        let inner_ref = &inner_data;
        
        println!("交互1 - 不同作用域的引用: {} 和 {}", outer_ref, inner_ref);
        
        // 演示生命周期如何受作用域影响
        fn analyze_references<'a, 'b>(outer: &'a str, inner: &'b str) -> &'a str {
            println!("分析引用: 外层='{}', 内层='{}'", outer, inner);
            outer // 只能返回外层引用，因为内层引用生命周期更短
        }
        
        let result = analyze_references(outer_ref, inner_ref);
        println!("交互2 - 生命周期约束: {}", result);
        
        // inner_ref和inner_data在这里结束生命周期
    }
    
    // outer_ref在这里仍然有效（如果在作用域外定义）
    println!("交互3 - 作用域结束后: {}", outer_data);
    
    println!("\n作用域与生命周期交互规律:");
    println!("• 引用的生命周期不能超过其指向数据的作用域");
    println!("• 内层作用域的引用不能传递到外层作用域");
    println!("• 生命周期参数反映了作用域的约束关系");
}

/// 生命周期与NLL的交互
fn lifetime_nll_interaction() {
    println!("\n⚡ 生命周期与NLL的交互:");
    
    let mut data = vec!["hello", "world", "rust"];
    
    // 传统生命周期分析 vs NLL分析
    println!("\n对比传统分析与NLL分析:");
    
    // 场景1: 条件性借用
    let condition = data.len() > 2;
    if condition {
        let reference = &data[0];
        println!("NLL交互1 - 条件借用: {}", reference);
        // 在NLL中，reference的生命周期在这里结束
    }
    
    // NLL允许这种修改，传统分析可能不允许
    data.push("nll");
    println!("NLL交互2 - 智能分析允许修改: {:?}", data);
    
    // 场景2: 早期返回
    fn process_with_early_return(data: &mut Vec<&str>) -> Option<String> {
        let first_value = data.get(0)?.to_string();
        
        if first_value.len() < 3 {
            return None; // 早期返回
        }
        
        // NLL知道first不再使用，允许修改data
        data.push("added");
        Some(first_value.to_uppercase())
    }
    
    match process_with_early_return(&mut data) {
        Some(result) => println!("NLL交互3 - 早期返回处理: {}", result),
        None => println!("处理失败"),
    }
    
    println!("修改后的数据: {:?}", data);
    
    println!("\n生命周期与NLL交互特点:");
    println!("• NLL基于生命周期概念，但分析更精确");
    println!("• NLL不改变生命周期规则，只是更智能地应用");
    println!("• NLL减少了显式生命周期标注的需要");
}

/// 综合交互分析
fn comprehensive_interaction() {
    println!("\n🎯 三者综合交互:");
    
    // 复杂的综合场景
    demonstrate_complex_interaction();
    
    // 错误场景分析
    analyze_error_scenarios();
}

/// 演示复杂交互
fn demonstrate_complex_interaction() {
    println!("\n🔄 复杂交互场景:");
    
    struct DataProcessor {
        cache: HashMap<String, Vec<i32>>,
    }
    
    impl DataProcessor {
        fn new() -> Self {
            let mut cache = HashMap::new();
            cache.insert("default".to_string(), vec![1, 2, 3]);
            DataProcessor { cache }
        }
        
        // 综合展示作用域、生命周期和NLL的交互
        fn process_data(&mut self, key: &str) -> Option<i32> {
            // 作用域1: 方法作用域
            {
                // 生命周期: 创建对缓存数据的引用
                let data = self.cache.get(key)?;
                
                // NLL: 智能分析允许在引用使用完毕后修改
                let sum = data.iter().sum::<i32>();
                println!("综合交互1 - 数据处理: key={}, sum={}", key, sum);
                
                // data的使用在这里结束，NLL知道可以安全修改cache
            }
            
            // 作用域2: NLL允许在这里修改cache
            self.cache.entry("processed".to_string())
                .or_insert_with(|| vec![]);
            
            // 作用域3: 返回处理结果
            self.cache.get(key).map(|data| data.len() as i32)
        }
        
        fn get_cache_info(&self) -> (usize, Vec<String>) {
            let size = self.cache.len();
            let keys: Vec<String> = self.cache.keys().cloned().collect();
            (size, keys)
        }
    }
    
    let mut processor = DataProcessor::new();
    
    match processor.process_data("default") {
        Some(result) => println!("综合交互2 - 处理结果: {}", result),
        None => println!("处理失败"),
    }
    
    let (size, keys) = processor.get_cache_info();
    println!("综合交互3 - 缓存信息: 大小={}, 键={:?}", size, keys);
    
    println!("\n综合交互的关键点:");
    println!("• 作用域定义了变量和引用的可见范围");
    println!("• 生命周期确保了引用的安全性");
    println!("• NLL优化了借用检查的精确性");
    println!("• 三者协同工作，实现安全高效的内存管理");
}

/// 分析错误场景
fn analyze_error_scenarios() {
    println!("\n❌ 错误场景分析:");
    
    println!("常见的错误模式和三个概念的作用:");
    
    // 错误类型1: 作用域错误
    println!("\n1. 作用域错误:");
    println!("   • 变量超出作用域后被访问");
    println!("   • 解决方案: 扩大变量作用域或重新设计数据流");
    
    // 错误类型2: 生命周期错误
    println!("\n2. 生命周期错误:");
    println!("   • 返回指向局部变量的引用");
    println!("   • 引用的生命周期超过被引用数据的生命周期");
    println!("   • 解决方案: 调整生命周期参数或使用所有权转移");
    
    // 错误类型3: 借用检查错误（NLL改善的）
    println!("\n3. 借用检查错误（NLL改善）:");
    println!("   • 同时存在可变和不可变引用");
    println!("   • NLL通过精确分析减少了这类错误");
    
    // 演示NLL如何改善错误处理
    demonstrate_nll_error_improvements();
}

/// 演示NLL如何改善错误处理
fn demonstrate_nll_error_improvements() {
    println!("\n✅ NLL错误改善示例:");
    
    let mut data = vec!["rust", "is", "awesome"];
    
    // 在NLL之前可能有问题的代码，现在可以工作
    let first_word = data.get(0).unwrap_or(&"default");
    println!("第一个词: {}", first_word);
    
    // NLL知道first_word不再使用，允许修改
    data.push("indeed");
    println!("修改后: {:?}", data);
    
    println!("\nNLL改善的错误类型:");
    println!("• 减少了不必要的借用冲突错误");
    println!("• 提供了更精确的错误消息");
    println!("• 允许更自然的代码模式");
}

/// 3. 演进时间线
fn evolution_timeline() {
    println!("\n📅 3. 演进时间线");
    println!("回顾Rust中这三个概念的发展历程。");
    
    show_evolution_timeline();
    analyze_evolution_impact();
}

/// 显示演进时间线
fn show_evolution_timeline() {
    println!("\n🕐 Rust内存管理概念演进:");
    println!("```");
    println!("2010-2012: Rust早期设计");
    println!("    ├─ 作用域概念确立");
    println!("    └─ 基本内存安全模型");
    println!("");
    println!("2012-2015: Rust 1.0开发");
    println!("    ├─ 生命周期系统完善");
    println!("    ├─ 借用检查器实现");
    println!("    └─ 所有权模型稳定");
    println!("");
    println!("2015: Rust 1.0发布");
    println!("    ├─ 稳定的作用域规则");
    println!("    ├─ 完整的生命周期系统");
    println!("    └─ 严格的借用检查");
    println!("");
    println!("2015-2018: 用户反馈期");
    println!("    ├─ 借用检查过于严格的问题");
    println!("    ├─ 学习曲线陡峭");
    println!("    └─ 需要更智能的分析");
    println!("");
    println!("2018: Rust 2018 Edition");
    println!("    ├─ NLL (Non-Lexical Lifetimes)");
    println!("    ├─ 更智能的借用检查");
    println!("    └─ 向后兼容的改进");
    println!("");
    println!("2018-现在: 持续优化");
    println!("    ├─ 错误消息改进");
    println!("    ├─ 性能优化");
    println!("    └─ 开发体验提升");
    println!("```");
}

/// 分析演进影响
fn analyze_evolution_impact() {
    println!("\n📊 演进影响分析:");
    
    println!("\n各阶段的主要改进:");
    
    println!("\n🏗️  Rust 1.0 (2015):");
    println!("• 建立了坚实的内存安全基础");
    println!("• 作用域和生命周期规则严格但有效");
    println!("• 学习曲线较陡，但安全性极高");
    
    println!("\n🚀 Rust 2018 (NLL引入):");
    println!("• 保持了所有安全保证");
    println!("• 显著改善了开发体验");
    println!("• 减少了不必要的编译错误");
    println!("• 提高了代码的可读性和自然性");
    
    // 演示演进的具体影响
    demonstrate_evolution_impact();
}

/// 演示演进的具体影响
fn demonstrate_evolution_impact() {
    println!("\n🎯 演进影响的具体示例:");
    
    // 示例：在Rust 2015中可能需要的变通方法
    println!("\n在Rust 2015中的典型模式:");
    
    let mut data = vec![1, 2, 3, 4, 5];
    
    // Rust 2015可能需要的模式
    let first_value = {
        let first_ref = &data[0];
        *first_ref // 复制值以避免借用检查问题
    };
    
    // 现在可以修改data
    data.push(6);
    println!("Rust 2015模式 - 复制值: {}, 数据: {:?}", first_value, data);
    
    // Rust 2018 (NLL) 允许的更自然模式
    let first_ref = &data[0];
    println!("Rust 2018模式 - 直接引用: {}", first_ref);
    // NLL知道first_ref不再使用
    
    data.push(7);
    println!("NLL允许的修改: {:?}", data);
    
    println!("\n演进带来的好处:");
    println!("• 代码更自然，更接近直觉");
    println!("• 减少了不必要的克隆和复制");
    println!("• 保持了相同的安全保证");
    println!("• 改善了学习体验");
}

/// 4. 实际应用影响
fn practical_implications() {
    println!("\n💼 4. 实际应用影响");
    println!("分析三个概念在实际开发中的影响。");
    
    // API设计影响
    api_design_implications();
    
    // 性能影响
    performance_implications();
    
    // 学习曲线影响
    learning_curve_implications();
}

/// API设计影响
fn api_design_implications() {
    println!("\n🎨 API设计影响:");
    
    // 展示不同概念如何影响API设计
    struct ModernAPI {
        data: Vec<String>,
    }
    
    impl ModernAPI {
        fn new() -> Self {
            ModernAPI {
                data: vec!["item1".to_string(), "item2".to_string()],
            }
        }
        
        // NLL使这种API设计成为可能
        fn get_and_modify(&mut self, index: usize) -> Option<&str> {
            // 先检查索引
            let _item = self.data.get(index)?;
            
            // 在传统系统中，这里可能有借用检查问题
            // 但NLL知道item的使用已经结束
            self.data.push(format!("new_item_{}", self.data.len()));
            
            // 返回新添加项的引用
            self.data.last().map(|s| s.as_str())
        }
        
        // 作用域和生命周期影响的API设计
        fn process_items<F>(&self, mut processor: F) 
        where 
            F: FnMut(&str) -> bool,
        {
            for item in &self.data {
                if !processor(item) {
                    break;
                }
            }
        }
    }
    
    let mut api = ModernAPI::new();
    
    if let Some(new_item) = api.get_and_modify(0) {
        println!("API设计1 - NLL支持的模式: {}", new_item);
    }
    
    api.process_items(|item| {
        println!("API设计2 - 闭包处理: {}", item);
        item.len() < 10 // 继续处理条件
    });
    
    println!("\nAPI设计的影响:");
    println!("• NLL允许更自然的API设计");
    println!("• 减少了API使用者的心智负担");
    println!("• 支持更复杂的操作模式");
}

/// 性能影响
fn performance_implications() {
    println!("\n⚡ 性能影响:");
    
    // 演示不同概念对性能的影响
    let mut large_data: Vec<String> = (0..1000)
        .map(|i| format!("item_{}", i))
        .collect();
    
    // 作用域的性能影响
    {
        let _scoped_data = &large_data[100..200];
        // 作用域确保及时释放资源
    } // _scoped_data在这里被释放
    
    // NLL的性能优势
    let first_item = large_data.get(0).map(|s| s.as_str()).unwrap_or("default");
    println!("性能优化1 - 避免不必要克隆: {}", &first_item[0..5]);
    
    // NLL允许这种优化
    large_data.push("optimized_item".to_string());
    
    // 生命周期的性能影响
    fn efficient_processing(data: &[String]) -> usize {
        // 生命周期确保引用的有效性，避免运行时检查
        data.iter().map(|s| s.len()).sum()
    }
    
    let total_length = efficient_processing(&large_data);
    println!("性能优化2 - 零成本抽象: 总长度={}", total_length);
    
    println!("\n性能影响总结:");
    println!("• 作用域: 确保及时资源释放");
    println!("• 生命周期: 零成本的安全保证");
    println!("• NLL: 减少不必要的克隆和复制");
}

/// 学习曲线影响
fn learning_curve_implications() {
    println!("\n📚 学习曲线影响:");
    
    println!("\n学习难度对比:");
    println!("```");
    println!("难度级别    概念        学习要点");
    println!("─────────────────────────────────────");
    println!("⭐         作用域      • 相对简单，类似其他语言");
    println!("                      • 花括号定义边界");
    println!("                      • 嵌套规则直观");
    println!("");
    println!("⭐⭐⭐      生命周期    • Rust独有概念");
    println!("                      • 需要理解借用和所有权");
    println!("                      • 显式标注语法");
    println!("");
    println!("⭐⭐        NLL        • 概念理解相对简单");
    println!("                      • 主要是工具改进");
    println!("                      • 减少了学习障碍");
    println!("```");
    
    // 演示学习路径
    demonstrate_learning_path();
}

/// 演示学习路径
fn demonstrate_learning_path() {
    println!("\n🛤️  推荐学习路径:");
    
    println!("\n1️⃣  第一阶段 - 作用域理解:");
    {
        let basic_var = "学习作用域";
        println!("   • 理解变量的可见范围: {}", basic_var);
        {
            let nested_var = "嵌套作用域";
            println!("   • 理解嵌套关系: {}", nested_var);
        }
        println!("   • 掌握资源管理基础");
    }
    
    println!("\n2️⃣  第二阶段 - 生命周期理解:");
    let data = String::from("学习生命周期");
    let reference = &data;
    println!("   • 理解引用的安全性: {}", reference);
    println!("   • 学习生命周期标注");
    println!("   • 掌握借用规则");
    
    println!("\n3️⃣  第三阶段 - NLL应用:");
    let mut advanced_data = vec!["NLL", "makes", "Rust", "easier"];
    let first = advanced_data.get(0).unwrap();
    println!("   • 体验更自然的代码: {}", first);
    advanced_data.push("indeed");
    println!("   • 理解智能分析: {:?}", advanced_data);
    
    println!("\n学习建议:");
    println!("• 循序渐进，先掌握基础概念");
    println!("• 通过实践加深理解");
    println!("• 利用NLL的改进降低学习难度");
    println!("• 重点理解概念之间的关系");
}

/// 5. 常见误解
fn common_misconceptions() {
    println!("\n❓ 5. 常见误解");
    println!("澄清关于三个概念的常见误解。");
    
    clarify_scope_misconceptions();
    clarify_lifetime_misconceptions();
    clarify_nll_misconceptions();
}

/// 澄清作用域误解
fn clarify_scope_misconceptions() {
    println!("\n🔍 作用域常见误解:");
    
    println!("\n❌ 误解1: 作用域只是语法糖");
    println!("✅ 事实: 作用域直接影响内存管理和资源释放");
    
    {
        let important_resource = String::from("重要资源");
        println!("   资源创建: {}", important_resource);
        // 资源在作用域结束时自动释放
    } // <- 这里资源被释放，不是语法糖！
    
    println!("\n❌ 误解2: 作用域和生命周期是同一概念");
    println!("✅ 事实: 作用域是词法概念，生命周期是语义概念");
    
    let data = String::from("数据");
    let reference = &data; // 引用的生命周期可能短于data的作用域
    println!("   数据: {}, 引用: {}", data, reference);
    // reference的生命周期在这里结束，但data的作用域继续
    
    println!("\n❌ 误解3: 作用域嵌套没有性能影响");
    println!("✅ 事实: 深度嵌套可能影响编译时间和代码可读性");
}

/// 澄清生命周期误解
fn clarify_lifetime_misconceptions() {
    println!("\n⏰ 生命周期常见误解:");
    
    println!("\n❌ 误解1: 生命周期是运行时概念");
    println!("✅ 事实: 生命周期是编译时概念，运行时没有开销");
    
    fn demonstrate_zero_cost<'a>(input: &'a str) -> &'a str {
        // 生命周期标注不产生运行时代码
        &input[0..3]
    }
    
    let result = demonstrate_zero_cost("zero_cost_abstraction");
    println!("   零成本抽象示例: {}", result);
    
    println!("\n❌ 误解2: 所有引用都需要显式生命周期标注");
    println!("✅ 事实: 大多数情况下可以通过省略规则自动推导");
    
    fn auto_lifetime(input: &str) -> &str {
        // 编译器自动推导生命周期
        &input[0..1]
    }
    
    let auto_result = auto_lifetime("automatic");
    println!("   自动推导示例: {}", auto_result);
    
    println!("\n❌ 误解3: 生命周期越长越好");
    println!("✅ 事实: 生命周期应该尽可能短，以提高安全性和性能");
}

/// 澄清NLL误解
fn clarify_nll_misconceptions() {
    println!("\n🚀 NLL常见误解:");
    
    println!("\n❌ 误解1: NLL改变了Rust的安全保证");
    println!("✅ 事实: NLL保持了相同的安全保证，只是分析更精确");
    
    let mut data = vec![1, 2, 3];
    let reference = &data[0];
    println!("   安全保证不变: {}", reference);
    // NLL确保这里reference不再使用后才允许修改
    data.push(4);
    println!("   仍然安全: {:?}", data);
    
    println!("\n❌ 误解2: NLL只是编译器优化");
    println!("✅ 事实: NLL是语言特性的改进，影响代码的可写性");
    
    println!("\n❌ 误解3: NLL解决了所有借用检查问题");
    println!("✅ 事实: NLL改善了很多情况，但某些限制仍然存在");
    
    // 仍然不允许的模式
    let immutable_ref = &data;
    // let mutable_ref = &mut data; // 这仍然不被允许
    println!("   某些限制仍存在，如同时可变和不可变借用: {:?}", immutable_ref);
}

/// 6. 高级对比
fn advanced_comparisons() {
    println!("\n🎓 6. 高级对比");
    println!("深入分析三个概念的高级特性和应用。");
    
    advanced_scope_patterns();
    advanced_lifetime_patterns();
    advanced_nll_patterns();
}

/// 高级作用域模式
fn advanced_scope_patterns() {
    println!("\n🔬 高级作用域模式:");
    
    // 模式1: RAII (Resource Acquisition Is Initialization)
    println!("\n1. RAII模式:");
    {
        struct ResourceGuard {
            name: String,
        }
        
        impl ResourceGuard {
            fn new(name: &str) -> Self {
                println!("   获取资源: {}", name);
                ResourceGuard { name: name.to_string() }
            }
        }
        
        impl Drop for ResourceGuard {
            fn drop(&mut self) {
                println!("   释放资源: {}", self.name);
            }
        }
        
        let _guard = ResourceGuard::new("数据库连接");
        println!("   使用资源...");
        // 资源在作用域结束时自动释放
    }
    
    // 模式2: 作用域保护
    println!("\n2. 作用域保护模式:");
    let mut sensitive_data = vec!["secret1", "secret2"];
    
    {
        let _protection_scope = "保护作用域";
        // 在这个作用域中进行敏感操作
        sensitive_data.push("secret3");
        println!("   在保护作用域中操作: {:?}", sensitive_data);
        // 敏感操作的影响被限制在这个作用域中
    }
    
    println!("   保护作用域结束");
}

/// 高级生命周期模式
fn advanced_lifetime_patterns() {
    println!("\n⚗️  高级生命周期模式:");
    
    // 模式1: 生命周期子类型
    println!("\n1. 生命周期子类型:");
    
    fn lifetime_subtyping<'a, 'b: 'a>(long: &'b str, short: &'a str) -> &'a str {
        // 'b: 'a 表示 'b 至少和 'a 一样长
        println!("   长生命周期: {}, 短生命周期: {}", long, short);
        short // 返回较短的生命周期
    }
    
    let long_lived = String::from("长期存在的数据");
    {
        let short_lived = String::from("短期数据");
        let result = lifetime_subtyping(&long_lived, &short_lived);
        println!("   子类型结果: {}", result);
    }
    
    // 模式2: 高阶生命周期
    println!("\n2. 高阶生命周期 (HRTB):");
    
    fn higher_ranked_lifetime<F>(f: F) -> String 
    where 
        F: for<'a> Fn(&'a str) -> &'a str,
    {
        let data = "测试数据";
        let result = f(data);
        format!("HRTB结果: {}", result)
    }
    
    let hrtb_result = higher_ranked_lifetime(|s| s.chars().take(1).collect::<String>().leak());
    println!("   {}", hrtb_result);
}

/// 高级NLL模式
fn advanced_nll_patterns() {
    println!("\n🔮 高级NLL模式:");
    
    // 模式1: 复杂的控制流分析
    println!("\n1. 复杂控制流分析:");
    
    let mut data = vec!["rust".to_string(), "nll".to_string(), "advanced".to_string()];
    
    for (index, item) in data.iter().enumerate() {
        if item.len() > 3 {
            println!("   长项目 {}: {}", index, item);
            // NLL知道item在这里的使用结束
            break;
        }
    }
    
    // NLL允许在循环后修改
    data.push("pattern".to_string());
    println!("   修改后的数据: {:?}", data);
    
    // 模式2: 条件性借用优化
    println!("\n2. 条件性借用优化:");
    
    fn conditional_borrow_optimization(data: &mut Vec<String>, condition: bool) -> Option<String> {
        if condition {
            let first = data.get(0)?;
            let result = first.clone();
            
            // NLL知道first不再使用，允许修改
            data.push("conditional_add".to_string());
            Some(result)
        } else {
            data.push("else_add".to_string());
            None
        }
    }
    
    match conditional_borrow_optimization(&mut data, true) {
        Some(result) => println!("   条件优化结果: {}", result),
        None => println!("   无结果"),
    }
    
    println!("   最终数据: {:?}", data);
}

/// 7. 真实世界场景
fn real_world_scenarios() {
    println!("\n🌍 7. 真实世界场景");
    println!("分析三个概念在实际项目中的应用。");
    
    web_server_scenario();
    database_connection_scenario();
    concurrent_programming_scenario();
}

/// Web服务器场景
fn web_server_scenario() {
    println!("\n🌐 Web服务器场景:");
    
    struct WebServer {
        routes: HashMap<String, String>,
        active_connections: usize,
    }
    
    impl WebServer {
        fn new() -> Self {
            let mut routes = HashMap::new();
            routes.insert("/".to_string(), "首页".to_string());
            routes.insert("/api".to_string(), "API端点".to_string());
            
            WebServer {
                routes,
                active_connections: 0,
            }
        }
        
        // 作用域: 请求处理的生命周期
        // 生命周期: 确保路由引用的安全性
        // NLL: 允许灵活的请求处理
        fn handle_request(&mut self, path: &str) -> Option<&str> {
            // 作用域1: 连接计数管理
            {
                self.active_connections += 1;
                println!("   新连接，当前活跃: {}", self.active_connections);
            }
            
            // 生命周期: 安全的路由查找
            let route_response = self.routes.get(path)?;
            
            // NLL: 智能分析允许在使用route_response后修改
            println!("   处理请求: {} -> {}", path, route_response);
            
            // 作用域2: 清理操作
            {
                self.active_connections -= 1;
                println!("   连接结束，当前活跃: {}", self.active_connections);
            }
            
            Some(route_response)
        }
    }
    
    let mut server = WebServer::new();
    
    // 模拟请求处理
    match server.handle_request("/") {
        Some(response) => println!("   响应: {}", response),
        None => println!("   404 Not Found"),
    }
    
    match server.handle_request("/api") {
        Some(response) => println!("   响应: {}", response),
        None => println!("   404 Not Found"),
    }
    
    println!("\nWeb服务器场景中的概念应用:");
    println!("• 作用域: 管理连接和资源的生命周期");
    println!("• 生命周期: 确保路由和响应的安全性");
    println!("• NLL: 允许灵活的请求处理逻辑");
}

/// 数据库连接场景
fn database_connection_scenario() {
    println!("\n🗄️  数据库连接场景:");
    
    struct DatabaseConnection {
        connection_id: usize,
        is_active: bool,
    }
    
    impl DatabaseConnection {
        fn new(id: usize) -> Self {
            println!("   建立数据库连接: {}", id);
            DatabaseConnection {
                connection_id: id,
                is_active: true,
            }
        }
        
        fn execute_query(&self, query: &str) -> Vec<String> {
            if self.is_active {
                println!("   执行查询 [连接{}]: {}", self.connection_id, query);
                vec!["结果1".to_string(), "结果2".to_string()]
            } else {
                vec![]
            }
        }
    }
    
    impl Drop for DatabaseConnection {
        fn drop(&mut self) {
            println!("   关闭数据库连接: {}", self.connection_id);
        }
    }
    
    // 作用域管理连接生命周期
    {
        let connection = DatabaseConnection::new(1);
        
        // 生命周期确保查询的安全性
        let results = connection.execute_query("SELECT * FROM users");
        
        // NLL允许灵活的结果处理
        for (index, result) in results.iter().enumerate() {
            println!("   结果 {}: {}", index, result);
        }
        
        // 连接在作用域结束时自动关闭
    }
    
    println!("\n数据库场景中的概念应用:");
    println!("• 作用域: 自动管理连接的建立和关闭");
    println!("• 生命周期: 确保查询结果的有效性");
    println!("• NLL: 优化查询结果的处理逻辑");
}

/// 并发编程场景
fn concurrent_programming_scenario() {
    println!("\n🔄 并发编程场景:");
    
    use std::sync::{Arc, Mutex};
    
    // 使用Arc和Mutex演示并发场景中的概念应用
    let shared_data = Arc::new(Mutex::new(vec!["共享数据1", "共享数据2"]));
    
    // 作用域: 锁的获取和释放
    {
        let data_guard = shared_data.lock().unwrap();
        println!("   获取锁，数据: {:?}", *data_guard);
        // 锁在作用域结束时自动释放
    }
    
    // 生命周期: 确保共享引用的安全性
    let data_clone = Arc::clone(&shared_data);
    
    // NLL: 智能分析锁的使用
    {
        let mut data_guard = data_clone.lock().unwrap();
        data_guard.push("新数据");
        println!("   修改数据: {:?}", *data_guard);
        // NLL确保guard在不再使用时释放锁
    }
    
    println!("\n并发场景中的概念应用:");
    println!("• 作用域: 自动管理锁的获取和释放");
    println!("• 生命周期: 确保共享数据的安全访问");
    println!("• NLL: 优化锁的持有时间");
}

/// 8. 性能影响对比
fn performance_impact_comparison() {
    println!("\n⚡ 8. 性能影响对比");
    println!("详细分析三个概念对性能的不同影响。");
    
    compile_time_performance();
    runtime_performance();
    memory_usage_comparison();
}

/// 编译时性能
fn compile_time_performance() {
    println!("\n🔨 编译时性能影响:");
    
    println!("\n概念\t\t编译时开销\t\t影响因素");
    println!("─────────────────────────────────────────────");
    println!("作用域\t\t很低\t\t\t词法分析简单");
    println!("生命周期\t\t中等\t\t\t需要复杂的推导和检查");
    println!("NLL\t\t\t中高\t\t\t更复杂的数据流分析");
    
    // 演示编译时分析的复杂性
    demonstrate_compile_time_complexity();
}

/// 演示编译时复杂性
fn demonstrate_compile_time_complexity() {
    println!("\n🧮 编译时分析复杂性示例:");
    
    // 简单作用域 - 编译器容易分析
    {
        let simple_var = "简单变量";
        println!("   简单作用域: {}", simple_var);
    } // 编译器很容易确定这里释放资源
    
    // 复杂生命周期 - 需要更多分析
    let data1 = String::from("数据1");
    let data2 = String::from("数据2");
    
    fn complex_lifetime_analysis<'a, 'b>(a: &'a str, b: &'b str) -> &'a str 
    where 'b: 'a 
    {
        // 编译器需要分析生命周期约束
        if a.len() > b.len() { a } else { a }
    }
    
    let result = complex_lifetime_analysis(&data1, &data2);
    println!("   复杂生命周期分析: {}", result);
    
    // NLL分析 - 最复杂的数据流分析
    let mut complex_data = vec![1, 2, 3, 4, 5];
    
    // NLL需要分析这个引用的实际使用范围
    let reference = &complex_data[0];
    
    // 复杂的控制流
    if *reference > 0 {
        println!("   NLL复杂分析: {}", reference);
        // NLL需要确定reference在这里不再使用
    }
    
    // 编译器需要验证这里的修改是安全的
    complex_data.push(6);
    println!("   NLL允许的修改: {:?}", complex_data);
    
    println!("\n编译时性能总结:");
    println!("• 作用域分析最快，基于词法结构");
    println!("• 生命周期分析中等，需要类型推导");
    println!("• NLL分析最慢，需要复杂的数据流分析");
    println!("• 但所有分析都是编译时进行，运行时无开销");
}

/// 运行时性能
fn runtime_performance() {
    println!("\n🏃 运行时性能影响:");
    
    println!("\n概念\t\t运行时开销\t\t性能特征");
    println!("─────────────────────────────────────────────");
    println!("作用域\t\t零开销\t\t\t确定性的资源管理");
    println!("生命周期\t\t零开销\t\t\t编译时保证，无运行时检查");
    println!("NLL\t\t\t零开销\t\t\t编译时优化，运行时无影响");
    
    // 演示零开销抽象
    demonstrate_zero_cost_abstractions();
}

/// 演示零开销抽象
fn demonstrate_zero_cost_abstractions() {
    println!("\n🎯 零开销抽象演示:");
    
    // 作用域的零开销
    let start_time = std::time::Instant::now();
    
    for _ in 0..1000 {
        // 作用域管理，无运行时开销
        let _scoped_data = vec![1, 2, 3, 4, 5];
        // 自动释放，编译为简单的栈操作
    }
    
    let scope_time = start_time.elapsed();
    
    // 生命周期的零开销
    let start_time = std::time::Instant::now();
    
    let data = vec![1, 2, 3, 4, 5];
    for _ in 0..1000 {
        let _reference = &data[0]; // 生命周期检查，无运行时开销
    }
    
    let lifetime_time = start_time.elapsed();
    
    // NLL的零开销
    let start_time = std::time::Instant::now();
    
    let mut nll_data = vec![1, 2, 3, 4, 5];
    for i in 0..1000 {
        let _ref = &nll_data[0];
        // NLL分析，编译时优化，运行时无开销
        if i % 100 == 0 {
            nll_data.push(i);
        }
    }
    
    let nll_time = start_time.elapsed();
    
    println!("   作用域操作时间: {:?}", scope_time);
    println!("   生命周期操作时间: {:?}", lifetime_time);
    println!("   NLL操作时间: {:?}", nll_time);
    
    println!("\n零开销抽象的关键点:");
    println!("• 所有安全检查都在编译时进行");
    println!("• 运行时代码与手写的C代码性能相当");
    println!("• 抽象不会带来性能损失");
}

/// 内存使用对比
fn memory_usage_comparison() {
    println!("\n💾 内存使用对比:");
    
    println!("\n概念\t\t内存影响\t\t\t特点");
    println!("─────────────────────────────────────────────────────");
    println!("作用域\t\t确定性释放\t\t\t栈上自动管理");
    println!("生命周期\t\t防止内存泄漏\t\t\t编译时保证");
    println!("NLL\t\t\t优化内存使用\t\t\t减少不必要的保持");
    
    // 演示内存使用模式
    demonstrate_memory_patterns();
}

/// 演示内存使用模式
fn demonstrate_memory_patterns() {
    println!("\n🧠 内存使用模式演示:");
    
    // 作用域的内存管理
    println!("\n1. 作用域内存管理:");
    {
        let large_data = vec![0; 1000]; // 分配大量内存
        println!("   分配了大量内存: {} 字节", large_data.len() * std::mem::size_of::<i32>());
        // 内存在作用域结束时自动释放
    } // <- 内存在这里被释放
    println!("   内存已自动释放");
    
    // 生命周期的内存安全
    println!("\n2. 生命周期内存安全:");
    let data = String::from("重要数据");
    {
        let reference = &data;
        println!("   安全引用: {}", reference);
        // 生命周期确保reference不会超过data的生命周期
    }
    println!("   数据仍然有效: {}", data);
    
    // NLL的内存优化
    println!("\n3. NLL内存优化:");
    let mut optimized_data = vec!["data1", "data2", "data3"];
    
    // NLL允许更早释放引用，优化内存使用
    let first_item = optimized_data.get(0).map(|s| s.to_string());
    if let Some(item) = first_item {
        println!("   获取项目: {}", item);
    }
    
    // NLL知道引用已结束，允许修改
    optimized_data.push("data4");
    println!("   优化后数据: {:?}", optimized_data);
    
    println!("\n内存使用总结:");
    println!("• 作用域: 提供确定性的内存释放");
    println!("• 生命周期: 防止内存安全问题");
    println!("• NLL: 优化内存使用效率");
}

/// 9. 调试差异
fn debugging_differences() {
    println!("\n🐛 9. 调试差异");
    println!("分析三个概念在调试过程中的不同表现。");
    
    scope_debugging();
    lifetime_debugging();
    nll_debugging();
}

/// 作用域调试
fn scope_debugging() {
    println!("\n🔍 作用域调试:");
    
    println!("\n作用域相关的常见调试场景:");
    println!("• 变量未定义错误");
    println!("• 变量过早释放");
    println!("• 作用域嵌套问题");
    
    // 演示作用域调试技巧
    {
        let debug_var = "调试变量";
        println!("   作用域内: {}", debug_var);
        
        {
            let nested_debug = format!("{} - 嵌套", debug_var);
            println!("   嵌套作用域: {}", nested_debug);
        }
        
        println!("   回到外层作用域: {}", debug_var);
    }
    
    println!("\n作用域调试技巧:");
    println!("• 使用println!宏跟踪变量生命周期");
    println!("• 注意花括号的配对");
    println!("• 理解变量的可见范围");
}

/// 生命周期调试
fn lifetime_debugging() {
    println!("\n⏰ 生命周期调试:");
    
    println!("\n生命周期相关的常见调试场景:");
    println!("• 借用检查错误");
    println!("• 生命周期不匹配");
    println!("• 悬垂指针问题");
    
    // 演示生命周期调试
    let data = String::from("调试数据");
    
    fn debug_lifetime_function<'a>(input: &'a str) -> &'a str {
        println!("   生命周期函数处理: {}", input);
        &input[0..3]
    }
    
    let result = debug_lifetime_function(&data);
    println!("   生命周期结果: {}", result);
    
    println!("\n生命周期调试技巧:");
    println!("• 仔细阅读编译器错误消息");
    println!("• 使用显式生命周期标注澄清意图");
    println!("• 理解借用规则");
}

/// NLL调试
fn nll_debugging() {
    println!("\n🚀 NLL调试:");
    
    println!("\n NLL相关的调试特点:");
    println!("• 更精确的错误消息");
    println!("• 减少了误报错误");
    println!("• 更好的错误定位");
    
    // 演示NLL调试改进
    let mut debug_data = vec![1, 2, 3, 4, 5];
    
    // NLL提供更好的调试体验
    let first = &debug_data[0];
    println!("   NLL调试: 第一个元素 = {}", first);
    
    // NLL的智能分析减少了调试困难
    debug_data.push(6);
    println!("   NLL允许的操作: {:?}", debug_data);
    
    println!("\nNLL调试优势:");
    println!("• 错误消息更加清晰和具体");
    println!("• 减少了需要调试的借用检查问题");
    println!("• 提供了更好的开发体验");
}

/// 10. 最佳实践对比
fn best_practice_comparison() {
    println!("\n📋 10. 最佳实践对比");
    println!("总结三个概念的最佳实践和使用建议。");
    
    scope_best_practices();
    lifetime_best_practices();
    nll_best_practices();
    integrated_best_practices();
}

/// 作用域最佳实践
fn scope_best_practices() {
    println!("\n📦 作用域最佳实践:");
    
    println!("\n✅ 推荐做法:");
    println!("• 保持作用域尽可能小");
    println!("• 使用RAII模式管理资源");
    println!("• 避免过深的嵌套");
    
    // 演示好的作用域实践
    {
        let resource = "重要资源";
        println!("   获取资源: {}", resource);
        
        // 在最小的作用域中使用资源
        let processed = format!("处理后的{}", resource);
        println!("   处理结果: {}", processed);
        
        // 资源自动释放
    }
    
    println!("\n❌ 避免的做法:");
    println!("• 不必要的大作用域");
    println!("• 忘记资源清理");
    println!("• 过度嵌套导致可读性差");
}

/// 生命周期最佳实践
fn lifetime_best_practices() {
    println!("\n⏰ 生命周期最佳实践:");
    
    println!("\n✅ 推荐做法:");
    println!("• 尽量使用生命周期省略规则");
    println!("• 只在必要时显式标注生命周期");
    println!("• 保持生命周期尽可能短");
    
    // 演示好的生命周期实践
    fn good_lifetime_practice(input: &str) -> &str {
        // 利用省略规则，无需显式标注
        input.chars().take(2).collect::<String>().leak()
    }
    
    let data = "生命周期最佳实践示例";
    let result = good_lifetime_practice(data);
    println!("   生命周期实践结果: {}", result);
    
    println!("\n❌ 避免的做法:");
    println!("• 过度使用显式生命周期标注");
    println!("• 创建不必要的长生命周期");
    println!("• 忽略借用检查器的建议");
}

/// NLL最佳实践
fn nll_best_practices() {
    println!("\n🚀 NLL最佳实践:");
    
    println!("\n✅ 推荐做法:");
    println!("• 信任NLL的智能分析");
    println!("• 编写更自然的代码");
    println!("• 利用NLL减少不必要的克隆");
    
    // 演示NLL最佳实践
    let mut data = vec!["rust", "nll", "best", "practices"];
    
    // 利用NLL的智能分析
    if let Some(first) = data.get(0) {
        println!("   NLL实践: 第一项 = {}", first);
        // NLL知道first不再使用
    }
    
    // 可以安全地修改
    data.push("added");
    println!("   NLL允许的修改: {:?}", data);
    
    println!("\n❌ 避免的做法:");
    println!("• 因为不信任NLL而过度使用克隆");
    println!("• 忽略NLL带来的改进");
    println!("• 仍然使用Rust 2015的编程模式");
}

/// 综合最佳实践
fn integrated_best_practices() {
    println!("\n🎯 综合最佳实践:");
    
    println!("\n🔗 三个概念的协同使用:");
    
    // 演示综合最佳实践
    struct BestPracticeExample {
        data: Vec<String>,
    }
    
    impl BestPracticeExample {
        fn new() -> Self {
            BestPracticeExample {
                data: vec!["示例1".to_string(), "示例2".to_string()],
            }
        }
        
        // 综合运用三个概念的最佳实践
        fn process_data(&mut self) -> Option<String> {
            // 作用域: 限制临时变量的生命周期
            {
                let temp_data = &self.data;
                if temp_data.is_empty() {
                    return None;
                }
            }
            
            // 生命周期: 安全的引用使用
            let first_item = self.data.get(0)?;
            let result = format!("处理: {}", first_item);
            
            // NLL: 智能分析允许在引用使用后修改
            self.data.push("新项目".to_string());
            
            Some(result)
        }
    }
    
    let mut example = BestPracticeExample::new();
    
    match example.process_data() {
        Some(result) => println!("   综合实践结果: {}", result),
        None => println!("   处理失败"),
    }
    
    println!("\n🏆 综合最佳实践原则:");
    println!("• 让作用域管理资源生命周期");
    println!("• 让生命周期确保内存安全");
    println!("• 让NLL优化借用检查");
    println!("• 三者协同工作，实现安全高效的代码");
    
    println!("\n📚 学习建议:");
    println!("• 从作用域开始，逐步理解生命周期");
    println!("• 体验NLL带来的改进");
    println!("• 在实践中加深理解");
    println!("• 阅读编译器错误消息，它们很有帮助");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparison_examples() {
        // 测试所有对比示例是否能正常运行
        run_comparison_examples();
    }

    #[test]
    fn test_scope_lifetime_interaction() {
        let data = String::from("测试数据");
        let reference = &data;
        assert_eq!(reference, "测试数据");
        // 测试作用域和生命周期的基本交互
    }

    #[test]
    fn test_nll_improvement() {
        let mut data = vec![1, 2, 3];
        let first = &data[0];
        assert_eq!(*first, 1);
        
        // 在NLL中，这是安全的
        drop(first); // 显式结束引用
        data.push(4);
        assert_eq!(data.len(), 4);
    }

    #[test]
    fn test_comprehensive_interaction() {
        // 测试三个概念的综合交互
        let mut test_data = vec!["test1", "test2"];
        
        {
            let reference = &test_data[0];
            assert_eq!(*reference, "test1");
        }
        
        test_data.push("test3");
        assert_eq!(test_data.len(), 3);
    }
}