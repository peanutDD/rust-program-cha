//! # 生命周期约束（Lifetime Bounds）
//!
//! 本模块深入探讨 Rust 的生命周期约束机制，包括：
//! - where 子句中的生命周期约束
//! - 复杂的生命周期约束组合
//! - 多重生命周期参数的管理
//! - 生命周期约束的最佳实践
//! - 约束推断和显式标注

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;

/// # 基础生命周期约束
///
/// 探讨生命周期约束的基本概念和语法
pub mod basic_lifetime_bounds {
    use std::fmt::Display;

    /// 演示基础生命周期约束
    pub fn demonstrate_basic_bounds() {
        println!("=== 基础生命周期约束 ===");

        // 1. 简单的生命周期约束
        demonstrate_simple_bounds();

        // 2. T: 'a 约束的含义
        demonstrate_t_outlives_a();

        // 3. 引用类型的约束
        demonstrate_reference_bounds();

        // 4. 约束的传递性
        demonstrate_bound_transitivity();
    }

    /// 演示简单的生命周期约束
    fn demonstrate_simple_bounds() {
        println!("\n--- 简单生命周期约束 ---");

        let data = String::from("测试数据");

        // 基本的生命周期约束：T: 'a
        let result = process_with_bound(&data);
        println!("处理结果: {}", result);

        // 演示约束如何确保安全性
        demonstrate_safety_guarantee();
    }

    /// 带生命周期约束的函数
    fn process_with_bound<'a, T>(input: &'a T) -> &'a T
    where
        T: 'a + Display, // T 必须至少活到 'a，并且实现 Display
    {
        println!("处理输入: {}", input);
        input
    }

    fn demonstrate_safety_guarantee() {
        println!("\n--- 约束的安全性保证 ---");

        let outer_data = String::from("外部数据");

        {
            let inner_data = String::from("内部数据");

            // 这些调用都是安全的，因为约束确保了数据的有效性
            let result1 = safe_process(&outer_data);
            let result2 = safe_process(&inner_data);

            println!("安全处理结果1: {}", result1);
            println!("安全处理结果2: {}", result2);
        }

        // outer_data 仍然有效
        println!("外部数据仍然有效: {}", outer_data);
    }

    fn safe_process<'a, T: 'a>(data: &'a T) -> String
    where
        T: Display,
    {
        format!("安全处理: {}", data)
    }

    /// 演示 T: 'a 约束的含义
    fn demonstrate_t_outlives_a() {
        println!("\n--- T: 'a 约束含义 ---");

        println!("T: 'a 意味着：");
        println!("1. 类型 T 中的所有引用都至少活到 'a");
        println!("2. T 不包含任何生命周期短于 'a 的引用");
        println!("3. T 可以安全地在 'a 生命周期内使用");

        // 演示不同类型如何满足 T: 'a 约束
        demonstrate_types_satisfying_bounds();
    }

    fn demonstrate_types_satisfying_bounds() {
        println!("\n--- 满足 T: 'a 约束的类型 ---");

        let string_data = String::from("拥有所有权的字符串");
        let number_data = 42i32;
        let static_str = "静态字符串";

        // 这些类型都满足 T: 'static 约束
        store_with_static_bound(string_data); // 拥有所有权
        store_with_static_bound(number_data); // 基本类型
        store_with_static_bound(static_str); // 静态引用

        // 演示结构体的约束
        let container = DataContainer {
            owned: String::from("拥有的数据"),
            number: 100,
        };

        store_with_static_bound(container);
    }

    #[derive(Debug)]
    struct DataContainer {
        owned: String,
        number: i32,
    }

    fn store_with_static_bound<T: 'static + std::fmt::Debug>(data: T) {
        println!("存储满足 'static 约束的数据: {:?}", data);
    }

    /// 演示引用类型的约束
    fn demonstrate_reference_bounds() {
        println!("\n--- 引用类型约束 ---");

        let data1 = String::from("数据1");
        let data2 = String::from("数据2");

        // 引用的生命周期约束
        let result = compare_references(&data1, &data2);
        println!("比较结果: {}", result);

        // 演示嵌套引用的约束
        demonstrate_nested_reference_bounds();
    }

    fn compare_references<'a, T>(first: &'a T, second: &'a T) -> &'a T
    where
        T: 'a + PartialOrd, // T 必须活到 'a 并且可比较
    {
        if first > second { first } else { second }
    }

    fn demonstrate_nested_reference_bounds() {
        println!("\n--- 嵌套引用约束 ---");

        let data = String::from("嵌套数据");
        let reference = &data;

        // 处理引用的引用
        let result = process_nested_reference(&reference);
        println!("嵌套引用处理结果: {}", result);
    }

    fn process_nested_reference<'a, T>(input: &'a &'a T) -> &'a T
    where
        T: 'a + Display,
    {
        println!("处理嵌套引用: {}", input);
        *input
    }

    /// 演示约束的传递性
    fn demonstrate_bound_transitivity() {
        println!("\n--- 约束传递性 ---");

        println!("如果 'a: 'b 且 'b: 'c，那么 'a: 'c");
        println!("如果 T: 'a 且 'a: 'b，那么 T: 'b");

        let long_lived = String::from("长生命周期数据");

        {
            let medium_lived = String::from("中等生命周期数据");

            {
                let short_lived = String::from("短生命周期数据");

                // 演示传递性
                demonstrate_transitive_bounds(&long_lived, &medium_lived, &short_lived);
            }
        }
    }

    fn demonstrate_transitive_bounds<'long, 'medium, 'short>(
        long: &'long str,
        medium: &'medium str,
        short: &'short str,
    ) where
        'long: 'medium, // 'long 比 'medium 活得更久
        'medium: 'short, // 'medium 比 'short 活得更久
                        // 因此 'long: 'short（传递性）
    {
        println!("长生命周期: {}", long);
        println!("中等生命周期: {}", medium);
        println!("短生命周期: {}", short);

        // 可以将长生命周期的引用用在需要短生命周期的地方
        let _: &'short str = long; // 合法，因为 'long: 'short
    }
}

/// # where 子句中的生命周期约束
///
/// 深入探讨 where 子句的高级用法
pub mod where_clause_bounds {
    use std::collections::HashMap;
    use std::fmt::{Debug, Display};

    /// 演示 where 子句中的生命周期约束
    pub fn demonstrate_where_clause_bounds() {
        println!("\n=== where 子句生命周期约束 ===");

        // 1. 基本 where 子句
        demonstrate_basic_where_clause();

        // 2. 复杂的 where 约束
        demonstrate_complex_where_constraints();

        // 3. 多重约束组合
        demonstrate_multiple_constraints();

        // 4. 条件约束
        demonstrate_conditional_bounds();
    }

    /// 演示基本 where 子句
    fn demonstrate_basic_where_clause() {
        println!("\n--- 基本 where 子句 ---");

        let data1 = String::from("数据1");
        let data2 = String::from("数据2");

        // 使用 where 子句指定约束
        let result = process_with_where_clause(&data1, &data2);
        println!("where 子句处理结果: {}", result);

        // 演示更复杂的 where 约束
        let container = ComplexContainer {
            data: &data1,
            metadata: HashMap::new(),
        };

        process_complex_container(&container);
    }

    /// 使用 where 子句的函数
    fn process_with_where_clause<'a, T, U>(first: &'a T, second: &'a U) -> String
    where
        T: 'a + Display + Debug,
        U: 'a + Display + Debug,
        T: PartialEq<U>,
    {
        if first == second {
            format!("相等: {:?} == {:?}", first, second)
        } else {
            format!("不等: {} != {}", first, second)
        }
    }

    struct ComplexContainer<'a> {
        data: &'a str,
        metadata: HashMap<String, String>,
    }

    fn process_complex_container<'a>(container: &ComplexContainer<'a>) {
        println!("处理复杂容器: {}", container.data);
    }

    /// 演示复杂的 where 约束
    fn demonstrate_complex_where_constraints() {
        println!("\n--- 复杂 where 约束 ---");

        let data = vec!["item1".to_string(), "item2".to_string()];

        // 复杂的约束组合
        let result = advanced_processing(&data);
        println!("高级处理结果: {:?}", result);

        // 演示关联类型的约束
        demonstrate_associated_type_bounds();
    }

    /// 高级处理函数，展示复杂约束
    fn advanced_processing<'a, T, I>(input: &'a [T]) -> Vec<String>
    where
        T: 'a + Clone + Display + Debug,
        I: Iterator<Item = &'a T>,
        &'a [T]: IntoIterator<Item = &'a T, IntoIter = I>,
    {
        input.iter().map(|item| format!("处理: {}", item)).collect()
    }

    fn demonstrate_associated_type_bounds() {
        println!("\n--- 关联类型约束 ---");

        let processor = AssociatedTypeProcessor;
        let data = String::from("关联类型测试");

        let result = processor.process(&data);
        println!("关联类型处理结果: {}", result);
    }

    /// 带关联类型的特征
    trait Processor<'a> {
        type Input: 'a + Display;
        type Output;

        fn process(&self, input: &'a Self::Input) -> Self::Output
        where
            Self::Input: 'a,
            Self::Output: Display;
    }

    struct AssociatedTypeProcessor;

    impl<'a> Processor<'a> for AssociatedTypeProcessor {
        type Input = String;
        type Output = String;

        fn process(&self, input: &'a Self::Input) -> Self::Output
        where
            Self::Input: 'a,
            Self::Output: Display,
        {
            format!("处理关联类型: {}", input)
        }
    }

    /// 演示多重约束组合
    fn demonstrate_multiple_constraints() {
        println!("\n--- 多重约束组合 ---");

        let data1 = String::from("多重约束数据1");
        let data2 = String::from("多重约束数据2");

        // 多重生命周期和特征约束
        let result = multi_constraint_function(&data1, &data2);
        println!("多重约束结果: {}", result);

        // 演示约束的组合
        demonstrate_constraint_combinations();
    }

    /// 多重约束函数
    fn multi_constraint_function<'a, 'b, T, U>(first: &'a T, second: &'b U) -> String
    where
        'a: 'b, // 'a 必须比 'b 活得更久
        T: 'a + Display + Debug + Clone,
        U: 'b + Display + Debug + Clone,
        T: PartialEq<U>,
    {
        if first == second {
            format!("多重约束匹配: {:?}", first)
        } else {
            format!("多重约束不匹配: {} vs {}", first, second)
        }
    }

    fn demonstrate_constraint_combinations() {
        println!("\n--- 约束组合 ---");

        let container = ConstraintContainer {
            data: String::from("约束数据"),
            reference: &"静态引用",
        };

        process_constraint_container(&container);
    }

    struct ConstraintContainer<'a> {
        data: String,
        reference: &'a str,
    }

    fn process_constraint_container<'a, 'b>(container: &'b ConstraintContainer<'a>)
    where
        'a: 'b, // 引用的生命周期必须比容器的引用活得更久
    {
        println!("约束容器数据: {}", container.data);
        println!("约束容器引用: {}", container.reference);
    }

    /// 演示条件约束
    fn demonstrate_conditional_bounds() {
        println!("\n--- 条件约束 ---");

        let data = ConditionalData {
            value: String::from("条件数据"),
        };

        // 条件约束：只有当 T 满足某些条件时才应用约束
        process_conditional(&data);

        // 演示泛型约束的条件性
        demonstrate_generic_conditional_bounds();
    }

    struct ConditionalData<T = String> {
        value: T,
    }

    fn process_conditional<T>(data: &ConditionalData<T>)
    where
        T: Display, // 只有当 T 实现 Display 时才能调用
    {
        println!("条件处理: {}", data.value);
    }

    fn demonstrate_generic_conditional_bounds() {
        println!("\n--- 泛型条件约束 ---");

        // 演示条件实现
        let wrapper1 = ConditionalWrapper { data: "字符串" };
        let wrapper2 = ConditionalWrapper { data: 42 };

        // 只有实现了 Display 的类型才能调用这个方法
        wrapper1.display_if_possible();
        wrapper2.display_if_possible();
    }

    struct ConditionalWrapper<T> {
        data: T,
    }

    impl<T> ConditionalWrapper<T>
    where
        T: Display, // 条件实现：只有 T 实现 Display 时才有这个方法
    {
        fn display_if_possible(&self) {
            println!("条件显示: {}", self.data);
        }
    }
}

/// # 复杂生命周期约束场景
///
/// 探讨实际应用中的复杂约束场景
pub mod complex_constraint_scenarios {
    use std::collections::HashMap;
    use std::fmt::{Debug, Display};
    use std::marker::PhantomData;

    /// 演示复杂生命周期约束场景
    pub fn demonstrate_complex_scenarios() {
        println!("\n=== 复杂生命周期约束场景 ===");

        // 1. 自引用结构体约束
        demonstrate_self_referential_constraints();

        // 2. 迭代器生命周期约束
        demonstrate_iterator_constraints();

        // 3. 闭包生命周期约束
        demonstrate_closure_constraints();

        // 4. 异步代码中的约束
        demonstrate_async_constraints();
    }

    /// 演示自引用结构体约束
    fn demonstrate_self_referential_constraints() {
        println!("\n--- 自引用结构体约束 ---");

        // 注意：真正的自引用结构体需要特殊处理，这里演示相关概念
        let data = String::from("自引用数据");
        let container = SelfReferencialContainer::new(&data);

        container.process();

        // 演示生命周期依赖
        demonstrate_lifetime_dependencies();
    }

    /// 模拟自引用结构体（实际的自引用需要 Pin 等高级技术）
    struct SelfReferencialContainer<'a> {
        data: &'a str,
        processed: Option<String>,
    }

    impl<'a> SelfReferencialContainer<'a> {
        fn new(data: &'a str) -> Self {
            Self {
                data,
                processed: None,
            }
        }

        fn process(&self) {
            println!("处理自引用数据: {}", self.data);
        }
    }

    fn demonstrate_lifetime_dependencies() {
        println!("\n--- 生命周期依赖 ---");

        let primary = String::from("主要数据");
        let secondary = String::from("次要数据");

        let dependent = DependentContainer::new(&primary, &secondary);
        dependent.analyze();
    }

    /// 具有生命周期依赖的容器
    struct DependentContainer<'primary, 'secondary> {
        primary: &'primary str,
        secondary: &'secondary str,
    }

    impl<'primary, 'secondary> DependentContainer<'primary, 'secondary>
    where
        'primary: 'secondary, // 主要数据必须比次要数据活得更久
    {
        fn new(primary: &'primary str, secondary: &'secondary str) -> Self {
            Self { primary, secondary }
        }

        fn analyze(&self) {
            println!("分析依赖数据: {} -> {}", self.primary, self.secondary);
        }

        fn get_primary_as_secondary(&self) -> &'secondary str {
            self.primary // 合法，因为 'primary: 'secondary
        }
    }

    /// 演示迭代器生命周期约束
    fn demonstrate_iterator_constraints() {
        println!("\n--- 迭代器生命周期约束 ---");

        let data = vec![
            "item1".to_string(),
            "item2".to_string(),
            "item3".to_string(),
        ];

        // 创建自定义迭代器
        let custom_iter = CustomIterator::new(&data);

        for item in custom_iter {
            println!("迭代项: {}", item);
        }

        // 演示迭代器适配器的约束
        demonstrate_iterator_adapter_constraints(&data);
    }

    /// 自定义迭代器，演示生命周期约束
    struct CustomIterator<'a, T> {
        data: &'a [T],
        index: usize,
    }

    impl<'a, T> CustomIterator<'a, T>
    where
        T: 'a, // T 必须至少活到 'a
    {
        fn new(data: &'a [T]) -> Self {
            Self { data, index: 0 }
        }
    }

    impl<'a, T> Iterator for CustomIterator<'a, T>
    where
        T: 'a + Clone,
    {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = &self.data[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    fn demonstrate_iterator_adapter_constraints(data: &[String]) {
        println!("\n--- 迭代器适配器约束 ---");

        // 使用带约束的迭代器适配器
        let filtered: Vec<_> = data.iter().filter(|item| item.len() > 4).collect();

        println!("过滤结果: {:?}", filtered);

        // 演示自定义适配器
        let mapped = ConstrainedMap::new(data.iter());
        for item in mapped {
            println!("映射项: {}", item);
        }
    }

    /// 带约束的映射适配器
    struct ConstrainedMap<'a, I, T>
    where
        I: Iterator<Item = &'a T>,
        T: 'a + Display,
    {
        inner: I,
        _phantom: PhantomData<&'a T>,
    }

    impl<'a, I, T> ConstrainedMap<'a, I, T>
    where
        I: Iterator<Item = &'a T>,
        T: 'a + Display,
    {
        fn new(iter: I) -> Self {
            Self {
                inner: iter,
                _phantom: PhantomData,
            }
        }
    }

    impl<'a, I, T> Iterator for ConstrainedMap<'a, I, T>
    where
        I: Iterator<Item = &'a T>,
        T: 'a + Display,
    {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next().map(|item| format!("映射: {}", item))
        }
    }

    /// 演示闭包生命周期约束
    fn demonstrate_closure_constraints() {
        println!("\n--- 闭包生命周期约束 ---");

        let data = String::from("闭包数据");

        // 带生命周期约束的闭包
        let result = process_with_closure(&data, |s| s.len());
        println!("闭包处理结果: {}", result);

        // 演示复杂闭包约束
        demonstrate_complex_closure_constraints();
    }

    /// 处理带约束闭包的函数
    fn process_with_closure<'a, T, F, R>(data: &'a T, f: F) -> R
    where
        T: 'a + Display,
        F: Fn(&'a T) -> R,
        R: Display,
    {
        println!("处理数据: {}", data);
        f(data)
    }

    fn demonstrate_complex_closure_constraints() {
        println!("\n--- 复杂闭包约束 ---");

        let data1 = String::from("数据1");
        let data2 = String::from("数据2");

        // 复杂的闭包约束场景
        let processor = ClosureProcessor::new();

        let result = processor.process_multiple(&data1, &data2, |a, b| format!("{} + {}", a, b));

        println!("复杂闭包结果: {}", result);
    }

    struct ClosureProcessor;

    impl ClosureProcessor {
        fn new() -> Self {
            Self
        }

        fn process_multiple<'a, 'b, T, U, F, R>(&self, first: &'a T, second: &'b U, f: F) -> R
        where
            T: 'a + Display,
            U: 'b + Display,
            F: Fn(&'a T, &'b U) -> R,
            R: Display,
        {
            f(first, second)
        }
    }

    /// 演示异步代码中的约束（模拟）
    fn demonstrate_async_constraints() {
        println!("\n--- 异步代码约束（模拟）---");

        let data = String::from("异步数据");

        // 模拟异步约束（实际异步需要 async/await）
        let future_like = AsyncLikeProcessor::new(&data);
        future_like.process();

        // 演示 Send + Sync 约束
        demonstrate_send_sync_constraints();
    }

    /// 模拟异步处理器
    struct AsyncLikeProcessor<'a> {
        data: &'a str,
    }

    impl<'a> AsyncLikeProcessor<'a> {
        fn new(data: &'a str) -> Self {
            Self { data }
        }

        fn process(&self) {
            println!("异步处理数据: {}", self.data);
        }
    }

    fn demonstrate_send_sync_constraints() {
        println!("\n--- Send + Sync 约束 ---");

        let data = String::from("线程安全数据");

        // 演示需要 Send + Sync 的场景
        let processor = ThreadSafeProcessor::new(data);
        processor.process();
    }

    struct ThreadSafeProcessor<T> {
        data: T,
    }

    impl<T> ThreadSafeProcessor<T>
    where
        T: Send + Sync + 'static + Display, // 线程安全约束
    {
        fn new(data: T) -> Self {
            Self { data }
        }

        fn process(&self) {
            println!("线程安全处理: {}", self.data);
        }
    }
}

/// # 生命周期约束的最佳实践
///
/// 总结生命周期约束的最佳实践和常见模式
pub mod best_practices {
    use std::fmt::{Debug, Display};
    use std::marker::PhantomData;

    /// 演示生命周期约束的最佳实践
    pub fn demonstrate_best_practices() {
        println!("\n=== 生命周期约束最佳实践 ===");

        // 1. 最小化约束
        demonstrate_minimal_constraints();

        // 2. 清晰的约束表达
        demonstrate_clear_constraint_expression();

        // 3. 避免过度约束
        demonstrate_avoiding_over_constraints();

        // 4. 约束文档化
        demonstrate_constraint_documentation();
    }

    /// 演示最小化约束
    fn demonstrate_minimal_constraints() {
        println!("\n--- 最小化约束 ---");

        let data = String::from("最小约束测试");

        // ❌ 过度约束
        // fn bad_function<T: 'static + Clone + Debug + Display + Send + Sync>(data: T) -> T {
        //     data
        // }

        // ✅ 最小约束
        let result = good_minimal_function(&data);
        println!("最小约束结果: {}", result);
    }

    /// 好的实践：只使用必要的约束
    fn good_minimal_function<T>(data: &T) -> String
    where
        T: Display, // 只需要 Display，不需要其他约束
    {
        format!("处理: {}", data)
    }

    /// 演示清晰的约束表达
    fn demonstrate_clear_constraint_expression() {
        println!("\n--- 清晰的约束表达 ---");

        let data1 = String::from("清晰约束数据1");
        let data2 = String::from("清晰约束数据2");

        // ✅ 清晰的约束表达
        let result = clear_constraint_function(&data1, &data2);
        println!("清晰约束结果: {}", result);
    }

    /// 清晰表达约束的函数
    fn clear_constraint_function<'a, T, U>(first: &'a T, second: &'a U) -> String
    where
        // 清晰地分组相关约束
        // 生命周期约束
        T: 'a,
        U: 'a,
        // 特征约束
        T: Display + Debug,
        U: Display + Debug,
        // 关系约束
        T: PartialEq<U>,
    {
        if first == second {
            format!("匹配: {:?} == {:?}", first, second)
        } else {
            format!("不匹配: {} != {}", first, second)
        }
    }

    /// 演示避免过度约束
    fn demonstrate_avoiding_over_constraints() {
        println!("\n--- 避免过度约束 ---");

        let data = String::from("避免过度约束测试");

        // ✅ 好的做法：使用泛型而不是具体类型
        let result1 = flexible_function(&data);
        println!("灵活函数结果: {}", result1);

        // ✅ 好的做法：条件约束
        let wrapper = FlexibleWrapper::new(data);
        wrapper.display_if_possible();
    }

    /// 灵活的函数，避免过度约束
    fn flexible_function<T>(input: &T) -> String
    where
        T: ?Sized + Display, // 使用 ?Sized 增加灵活性
    {
        format!("灵活处理: {}", input)
    }

    /// 灵活的包装器
    struct FlexibleWrapper<T> {
        data: T,
    }

    impl<T> FlexibleWrapper<T> {
        fn new(data: T) -> Self {
            Self { data }
        }
    }

    // 条件实现：只有当 T 实现 Display 时才有这个方法
    impl<T> FlexibleWrapper<T>
    where
        T: Display,
    {
        fn display_if_possible(&self) {
            println!("条件显示: {}", self.data);
        }
    }

    /// 演示约束文档化
    fn demonstrate_constraint_documentation() {
        println!("\n--- 约束文档化 ---");

        let data = DocumentedData {
            content: String::from("文档化数据"),
        };

        let processor = DocumentedProcessor::new();
        processor.process(&data);
    }

    /// 文档化的数据结构
    ///
    /// # 生命周期约束
    ///
    /// 这个结构体不包含任何引用，因此满足 `'static` 约束。
    #[derive(Debug)]
    struct DocumentedData {
        content: String,
    }

    impl Display for DocumentedData {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.content)
        }
    }

    /// 文档化的处理器
    ///
    /// # 类型参数约束
    ///
    /// - `T: Display` - 用于输出显示
    /// - `T: Debug` - 用于调试输出
    /// - `T: 'static` - 确保类型不包含非静态引用
    pub struct DocumentedProcessor;

    impl DocumentedProcessor {
        /// 创建新的处理器实例
        pub fn new() -> Self {
            Self
        }

        /// 处理数据
        ///
        /// # 约束说明
        ///
        /// - `T: 'static` - 确保数据在整个处理过程中有效
        /// - `T: Display + Debug` - 用于输出和调试
        ///
        /// # 示例
        ///
        /// ```rust
        /// use advanced_lifetime::lifetime_bounds::best_practices::DocumentedProcessor;
        /// let processor = DocumentedProcessor::new();
        /// let data = String::from("test");
        /// processor.process(&data);
        /// ```
        pub fn process<T>(&self, data: &T)
        where
            T: 'static + Display + Debug,
        {
            println!("文档化处理: {}", data);
            println!("调试信息: {:?}", data);
        }
    }
}

/// 运行所有生命周期约束的演示
pub fn run_all_demonstrations() {
    run_all_lifetime_bounds_examples();
}

/// 运行所有生命周期约束的演示（别名函数）
pub fn run_all_lifetime_bounds_examples() {
    println!("🦀 Rust 深入生命周期 - 生命周期约束篇 🦀");
    println!("=============================================");

    // 1. 基础生命周期约束
    basic_lifetime_bounds::demonstrate_basic_bounds();

    // 2. where 子句约束
    where_clause_bounds::demonstrate_where_clause_bounds();

    // 3. 复杂约束场景
    complex_constraint_scenarios::demonstrate_complex_scenarios();

    // 4. 最佳实践
    best_practices::demonstrate_best_practices();

    println!("\n=============================================");
    println!("✅ 生命周期约束演示完成！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_lifetime_bound() {
        fn test_function<'a, T: 'a>(data: &'a T) -> &'a T {
            data
        }

        let data = String::from("test");
        let result = test_function(&data);
        assert_eq!(result, &data);
    }

    #[test]
    fn test_where_clause_bound() {
        fn test_where<T>(data: T) -> T
        where
            T: Clone + 'static,
        {
            data.clone()
        }

        let data = String::from("where test");
        let result = test_where(data.clone());
        assert_eq!(result, data);
    }

    #[test]
    fn test_multiple_lifetime_bounds() {
        fn test_multiple<'a, 'b>(first: &'a str, second: &'b str) -> String
        where
            'a: 'b, // 'a 必须比 'b 活得更久
        {
            format!("{} {}", first, second)
        }

        let long_lived = String::from("long");
        {
            let short_lived = String::from("short");
            let result = test_multiple(&long_lived, &short_lived);
            assert_eq!(result, "long short");
        }
    }
}
