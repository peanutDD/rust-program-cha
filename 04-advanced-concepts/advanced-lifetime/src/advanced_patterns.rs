//! # 高级生命周期模式（Advanced Lifetime Patterns）
//!
//! 本模块深入探讨 Rust 的高级生命周期概念，包括：
//! - 高阶生命周期（Higher-Ranked Trait Bounds, HRTB）
//! - 生命周期子类型化（Lifetime Subtyping）
//! - 协变与逆变（Covariance and Contravariance）
//! - 生命周期多态性（Lifetime Polymorphism）
//! - 复杂的生命周期推断场景

use std::cell::Cell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

/// # 高阶生命周期（Higher-Ranked Trait Bounds）
///
/// HRTB 允许我们表达"对于任何生命周期"的约束
pub mod higher_ranked_trait_bounds {
    use std::fmt::Debug;

    /// 演示高阶生命周期的基础概念
    pub fn demonstrate_hrtb_basics() {
        println!("=== 高阶生命周期基础 ===");

        // 1. 基本的 HRTB 语法
        demonstrate_basic_hrtb();

        // 2. 闭包中的 HRTB
        demonstrate_hrtb_with_closures();

        // 3. 函数指针中的 HRTB
        demonstrate_hrtb_with_function_pointers();

        // 4. 复杂的 HRTB 场景
        demonstrate_complex_hrtb();
    }

    /// 演示基本的 HRTB 语法
    fn demonstrate_basic_hrtb() {
        println!("\n--- 基本 HRTB 语法 ---");

        // for<'a> 语法表示"对于任何生命周期 'a"
        let processor = StringProcessor;

        let text1 = String::from("第一个字符串");
        let text2 = String::from("第二个字符串");

        let result = processor.process_strings(&text1, &text2);
        println!("处理结果: {}", result);

        // 演示不同生命周期的字符串
        {
            let temp_text = String::from("临时字符串");
            let result2 = processor.process_strings(&text1, &temp_text);
            println!("混合生命周期处理结果: {}", result2);
        }
    }

    /// 字符串处理器，演示 HRTB
    struct StringProcessor;

    impl StringProcessor {
        /// 使用 HRTB 处理任意生命周期的字符串引用
        fn process_strings(&self, s1: &str, s2: &str) -> String {
            // 这个函数可以接受任何生命周期的字符串引用
            format!("处理: {} 和 {}", s1, s2)
        }

        /// 演示真正的 HRTB 用法
        fn process_with_function<F>(&self, s1: &str, s2: &str, f: F) -> String
        where
            F: for<'a> Fn(&'a str) -> &'a str,
        {
            let processed_s1 = f(s1);
            let processed_s2 = f(s2);
            format!("处理: {} 和 {}", processed_s1, processed_s2)
        }
    }

    /// 演示闭包中的 HRTB
    fn demonstrate_hrtb_with_closures() {
        println!("\n--- 闭包中的 HRTB ---");

        // 定义一个接受 HRTB 闭包的函数
        fn apply_to_strings<F>(f: F, strings: &[String]) -> Vec<usize>
        where
            F: for<'a> Fn(&'a str) -> usize,
        {
            strings.iter().map(|s| f(s)).collect()
        }

        let strings = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];

        // 使用闭包计算字符串长度
        let lengths = apply_to_strings(|s| s.len(), &strings);
        println!("字符串长度: {:?}", lengths);

        // 使用闭包计算字符数
        let char_counts = apply_to_strings(|s| s.chars().count(), &strings);
        println!("字符数量: {:?}", char_counts);
    }

    /// 演示函数指针中的 HRTB
    fn demonstrate_hrtb_with_function_pointers() {
        println!("\n--- 函数指针中的 HRTB ---");

        // 定义函数指针类型，使用 HRTB
        type StringTransformer = for<'a> fn(&'a str) -> &'a str;

        // 实现几个转换函数
        fn identity(s: &str) -> &str {
            s
        }

        fn first_word(s: &str) -> &str {
            s.split_whitespace().next().unwrap_or("")
        }

        // 使用函数指针
        let transformers: Vec<StringTransformer> = vec![identity, first_word];

        let test_string = "hello world rust";

        for (i, transformer) in transformers.iter().enumerate() {
            let result = transformer(test_string);
            println!("转换器 {}: {} -> {}", i, test_string, result);
        }
    }

    /// 演示复杂的 HRTB 场景
    fn demonstrate_complex_hrtb() {
        println!("\n--- 复杂 HRTB 场景 ---");

        // 复杂的 HRTB：涉及多个生命周期参数
        fn complex_processor<F, T>(f: F, data: &[T]) -> Vec<String>
        where
            F: for<'a> Fn(&'a T) -> &'a str,
            T: Debug,
        {
            data.iter().map(|item| f(item).to_string()).collect()
        }

        // 定义一个包含字符串的结构体
        #[derive(Debug)]
        struct Container {
            name: String,
            value: i32,
        }

        impl Container {
            fn get_name(&self) -> &str {
                &self.name
            }
        }

        let containers = vec![
            Container {
                name: "容器1".to_string(),
                value: 10,
            },
            Container {
                name: "容器2".to_string(),
                value: 20,
            },
            Container {
                name: "容器3".to_string(),
                value: 30,
            },
        ];

        // 使用复杂的 HRTB 处理器
        let names = complex_processor(|c: &Container| c.get_name(), &containers);
        println!("提取的名称: {:?}", names);
    }
}

/// # 生命周期子类型化（Lifetime Subtyping）
///
/// 探讨生命周期之间的子类型关系
pub mod lifetime_subtyping {
    /// 演示生命周期子类型化
    pub fn demonstrate_lifetime_subtyping() {
        println!("\n=== 生命周期子类型化 ===");

        // 1. 基本的生命周期子类型
        demonstrate_basic_subtyping();

        // 2. 结构体中的生命周期子类型
        demonstrate_struct_subtyping();

        // 3. 函数参数中的子类型
        demonstrate_function_subtyping();

        // 4. 复杂的子类型场景
        demonstrate_complex_subtyping();
    }

    /// 演示基本的生命周期子类型
    fn demonstrate_basic_subtyping() {
        println!("\n--- 基本生命周期子类型 ---");

        // 较长的生命周期可以被强制转换为较短的生命周期
        let long_lived = String::from("长生命周期字符串");

        {
            let short_lived = String::from("短生命周期字符串");

            // 这里演示生命周期子类型化
            // 'long 生命周期可以被用在需要 'short 生命周期的地方
            let result = choose_string(&long_lived, &short_lived);
            println!("选择的字符串: {}", result);
        }

        // long_lived 仍然有效
        println!("长生命周期字符串仍然有效: {}", long_lived);
    }

    /// 选择两个字符串中的一个
    fn choose_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() > s2.len() { s1 } else { s2 }
    }

    /// 演示结构体中的生命周期子类型
    fn demonstrate_struct_subtyping() {
        println!("\n--- 结构体中的生命周期子类型 ---");

        let data = String::from("结构体数据");

        // 创建具有较长生命周期的结构体
        let container = DataContainer { data: &data };

        {
            let temp_data = String::from("临时数据");

            // 可以将较长生命周期的引用传递给需要较短生命周期的函数
            process_container(&container);

            // 演示生命周期协变
            let temp_container = DataContainer { data: &temp_data };
            process_container(&temp_container);
        }

        // 原始容器仍然有效
        println!("原始容器数据: {}", container.data);
    }

    #[derive(Debug)]
    struct DataContainer<'a> {
        data: &'a str,
    }

    fn process_container(container: &DataContainer) {
        println!("处理容器: {:?}", container);
    }

    /// 演示函数参数中的子类型
    fn demonstrate_function_subtyping() {
        println!("\n--- 函数参数中的子类型 ---");

        // 定义一个函数类型
        type StringProcessor<'a> = fn(&'a str) -> &'a str;

        // 实现一个具体的处理函数
        fn trim_string(s: &str) -> &str {
            s.trim()
        }

        let processor: StringProcessor = trim_string;

        let text = String::from("  需要修剪的文本  ");
        let result = processor(&text);
        println!("处理结果: '{}'", result);

        // 演示函数指针的生命周期子类型化
        demonstrate_function_pointer_subtyping();
    }

    fn demonstrate_function_pointer_subtyping() {
        println!("\n--- 函数指针生命周期子类型化 ---");

        // 较长生命周期的函数可以被用作较短生命周期的函数
        fn long_lived_function(s: &str) -> &str {
            s
        }

        // 这个函数接受任何生命周期的函数指针
        fn use_processor<'a>(f: fn(&'a str) -> &'a str, input: &'a str) -> &'a str {
            f(input)
        }

        let text = String::from("测试文本");
        let result = use_processor(long_lived_function, &text);
        println!("函数指针处理结果: {}", result);
    }

    /// 演示复杂的子类型场景
    fn demonstrate_complex_subtyping() {
        println!("\n--- 复杂子类型场景 ---");

        // 演示嵌套结构中的生命周期子类型化
        let outer_data = String::from("外部数据");

        {
            let inner_data = String::from("内部数据");

            let nested = NestedContainer {
                outer: &outer_data,
                inner: &inner_data,
            };

            process_nested(&nested);
        }

        // 演示生命周期参数的协变性
        demonstrate_covariance();
    }

    struct NestedContainer<'a, 'b> {
        outer: &'a str,
        inner: &'b str,
    }

    fn process_nested(container: &NestedContainer) {
        println!(
            "嵌套容器 - 外部: {}, 内部: {}",
            container.outer, container.inner
        );
    }

    /// 演示协变性
    fn demonstrate_covariance() {
        println!("\n--- 生命周期协变性 ---");

        // &'a T 对于 'a 是协变的
        // 这意味着如果 'long: 'short，那么 &'long T 可以被用作 &'short T

        let long_string = String::from("长生命周期");

        {
            // 在这个作用域中，我们可以使用长生命周期的引用
            let short_ref: &str = &long_string; // 协变：&'long str -> &'short str
            println!("协变引用: {}", short_ref);
        }

        // 演示 Box<T> 的协变性
        let boxed_long: Box<&str> = Box::new(&long_string);
        // Box<&'long str> 可以被用作 Box<&'short str>

        println!("装箱的引用: {}", boxed_long);
    }
}

/// # 协变与逆变（Variance）
///
/// 深入探讨 Rust 中的协变和逆变概念
pub mod variance {
    use std::marker::PhantomData;

    /// 演示协变与逆变
    pub fn demonstrate_variance() {
        println!("\n=== 协变与逆变 ===");

        // 1. 协变性演示
        demonstrate_covariance();

        // 2. 逆变性演示
        demonstrate_contravariance();

        // 3. 不变性演示
        demonstrate_invariance();

        // 4. 实际应用中的变性
        demonstrate_variance_in_practice();
    }

    /// 演示协变性
    fn demonstrate_covariance() {
        println!("\n--- 协变性（Covariance）---");

        // &T, Box<T>, Vec<T> 等对 T 是协变的
        // 如果 'a: 'b（'a 比 'b 活得更久），那么 &'a T 可以被用作 &'b T

        let long_lived = String::from("长生命周期数据");

        {
            let short_lived = String::from("短生命周期数据");

            // 协变：可以将长生命周期的引用赋给短生命周期的变量
            let covariant_demo = CovariantStruct {
                data: &long_lived, // &'long str
            };

            // 可以传递给期望较短生命周期的函数
            use_covariant_struct(&covariant_demo);
        }

        println!("协变性允许生命周期的安全收缩");
    }

    #[derive(Debug)]
    pub struct CovariantStruct<'a> {
        pub data: &'a str, // &T 对 T 协变
    }

    fn use_covariant_struct(s: &CovariantStruct) {
        println!("使用协变结构体: {:?}", s);
    }

    /// 演示逆变性
    fn demonstrate_contravariance() {
        println!("\n--- 逆变性（Contravariance）---");

        // 函数参数类型是逆变的
        // 如果 'a: 'b，那么 fn(&'b T) 可以被用作 fn(&'a T)

        // 定义函数类型
        type LongProcessor = for<'a> fn(&'a str) -> ();
        type ShortProcessor<'b> = fn(&'b str) -> ();

        // 实现一个处理函数
        fn process_string(s: &str) {
            println!("处理字符串: {}", s);
        }

        let processor: LongProcessor = process_string;

        // 演示逆变性的实际应用
        demonstrate_function_contravariance();
    }

    fn demonstrate_function_contravariance() {
        println!("\n--- 函数逆变性实例 ---");

        // 定义一个接受函数的高阶函数
        fn apply_to_data<F>(f: F, data: &str)
        where
            F: Fn(&str),
        {
            f(data);
        }

        // 定义处理函数
        fn long_lived_processor(s: &str) {
            println!("长生命周期处理器: {}", s);
        }

        let data = String::from("测试数据");

        // 逆变性：长生命周期的函数可以用于短生命周期的场景
        apply_to_data(long_lived_processor, &data);
    }

    /// 演示不变性
    fn demonstrate_invariance() {
        println!("\n--- 不变性（Invariance）---");

        // &mut T 对 T 是不变的
        // Cell<T> 和 RefCell<T> 对 T 也是不变的

        let mut data = String::from("可变数据");

        {
            // 不变性：&mut T 既不协变也不逆变
            let invariant_ref = &mut data;

            // 不能将 &mut 'long T 用作 &mut 'short T
            // 也不能将 &mut 'short T 用作 &mut 'long T

            modify_data(invariant_ref);
        }

        println!("修改后的数据: {}", data);

        // 演示 Cell 的不变性
        demonstrate_cell_invariance();
    }

    fn modify_data(data: &mut String) {
        data.push_str(" - 已修改");
    }

    fn demonstrate_cell_invariance() {
        use std::cell::Cell;

        println!("\n--- Cell 不变性 ---");

        // Cell<T> 对 T 是不变的，因为它允许内部可变性
        let cell_data = Cell::new("Cell 数据");

        // 不变性确保了内存安全
        let invariant_cell = InvariantStruct { cell: &cell_data };

        use_invariant_struct(&invariant_cell);
    }

    struct InvariantStruct<'a> {
        cell: &'a std::cell::Cell<&'static str>, // Cell<T> 对 T 不变
    }

    fn use_invariant_struct(s: &InvariantStruct) {
        println!("使用不变结构体，Cell 值: {:?}", s.cell.get());
    }

    /// 演示实际应用中的变性
    fn demonstrate_variance_in_practice() {
        println!("\n--- 实际应用中的变性 ---");

        // 1. 迭代器的协变性
        demonstrate_iterator_covariance();

        // 2. 智能指针的变性
        demonstrate_smart_pointer_variance();

        // 3. 自定义类型的变性设计
        demonstrate_custom_variance();
    }

    fn demonstrate_iterator_covariance() {
        println!("\n--- 迭代器协变性 ---");

        let data = vec!["item1".to_string(), "item2".to_string()];

        // Iterator<Item = &'a T> 对 'a 协变
        let iter = data.iter(); // Iterator<Item = &'long String>

        // 可以在较短的作用域中使用
        {
            for item in iter {
                println!("迭代项: {}", item);
            }
        }
    }

    fn demonstrate_smart_pointer_variance() {
        use std::rc::Rc;
        use std::sync::Arc;

        println!("\n--- 智能指针变性 ---");

        let data = String::from("智能指针数据");

        // Rc<T> 和 Arc<T> 对 T 协变
        let rc_data = Rc::new(&data);
        let arc_data = Arc::new(&data);

        // 协变性允许安全的生命周期收缩
        {
            let rc_clone = Rc::clone(&rc_data);
            let arc_clone = Arc::clone(&arc_data);

            println!("Rc 数据: {}", rc_clone);
            println!("Arc 数据: {}", arc_clone);
        }
    }

    fn demonstrate_custom_variance() {
        println!("\n--- 自定义类型变性设计 ---");

        // 设计协变的包装器
        let data = String::from("包装数据");
        let wrapper = CovariantWrapper::new(&data);

        use_covariant_wrapper(&wrapper);

        // 设计不变的包装器
        let mut mutable_data = String::from("可变包装数据");
        let invariant_wrapper = InvariantWrapper::new(&mut mutable_data);

        use_invariant_wrapper(&invariant_wrapper);
    }

    /// 协变包装器
    struct CovariantWrapper<'a> {
        data: &'a str, // 协变
        _phantom: PhantomData<&'a ()>,
    }

    impl<'a> CovariantWrapper<'a> {
        fn new(data: &'a str) -> Self {
            Self {
                data,
                _phantom: PhantomData,
            }
        }
    }

    fn use_covariant_wrapper(wrapper: &CovariantWrapper) {
        println!("协变包装器数据: {}", wrapper.data);
    }

    /// 不变包装器
    struct InvariantWrapper<'a> {
        data: &'a mut String,                        // 不变
        _phantom: PhantomData<fn(&'a ()) -> &'a ()>, // 使类型对 'a 不变
    }

    impl<'a> InvariantWrapper<'a> {
        fn new(data: &'a mut String) -> Self {
            Self {
                data,
                _phantom: PhantomData,
            }
        }
    }

    fn use_invariant_wrapper(wrapper: &InvariantWrapper) {
        println!("不变包装器数据: {}", wrapper.data);
    }
}

/// # 生命周期多态性（Lifetime Polymorphism）
///
/// 探讨如何在泛型中使用生命周期参数
pub mod lifetime_polymorphism {
    use std::collections::HashMap;
    use std::marker::PhantomData;

    /// 演示生命周期多态性
    pub fn demonstrate_lifetime_polymorphism() {
        println!("\n=== 生命周期多态性 ===");

        // 1. 泛型结构体中的生命周期
        demonstrate_generic_structs_with_lifetimes();

        // 2. 泛型函数中的生命周期
        demonstrate_generic_functions_with_lifetimes();

        // 3. 特征中的生命周期多态性
        demonstrate_trait_lifetime_polymorphism();

        // 4. 复杂的生命周期多态场景
        demonstrate_complex_lifetime_polymorphism();
    }

    /// 演示泛型结构体中的生命周期
    fn demonstrate_generic_structs_with_lifetimes() {
        println!("\n--- 泛型结构体中的生命周期 ---");

        let data1 = String::from("数据1");
        let data2 = 42i32;

        // 创建包含不同类型和生命周期的容器
        let container1 = GenericContainer::new(&data1, data2);
        println!("容器1: {:?}", container1);

        let data3 = vec![1, 2, 3, 4, 5];
        let container2 = GenericContainer::new(&data1, &data3);
        println!("容器2: {:?}", container2);

        // 演示生命周期参数的推断
        demonstrate_lifetime_inference();
    }

    #[derive(Debug)]
    struct GenericContainer<'a, T, U> {
        reference: &'a T,
        value: U,
        _phantom: PhantomData<&'a ()>,
    }

    impl<'a, T, U> GenericContainer<'a, T, U>
    where
        T: std::fmt::Debug,
        U: std::fmt::Debug,
    {
        fn new(reference: &'a T, value: U) -> Self {
            Self {
                reference,
                value,
                _phantom: PhantomData,
            }
        }

        fn get_reference(&self) -> &T {
            self.reference
        }

        fn get_value(&self) -> &U {
            &self.value
        }
    }

    fn demonstrate_lifetime_inference() {
        println!("\n--- 生命周期推断 ---");

        let text = String::from("推断测试");

        // 编译器可以推断生命周期参数
        let inferred_container = create_container(&text, 100);
        println!("推断的容器: {:?}", inferred_container);
    }

    fn create_container<'a, T, U>(reference: &'a T, value: U) -> GenericContainer<'a, T, U>
    where
        T: std::fmt::Debug,
        U: std::fmt::Debug,
    {
        GenericContainer::new(reference, value)
    }

    /// 演示泛型函数中的生命周期
    fn demonstrate_generic_functions_with_lifetimes() {
        println!("\n--- 泛型函数中的生命周期 ---");

        let data1 = String::from("第一个数据");
        let data2 = String::from("第二个数据");

        // 使用泛型函数处理不同类型
        let result1 = process_two_references(&data1, &data2);
        println!("字符串处理结果: {}", result1);

        let num1 = 10;
        let num2 = 20;
        let result2 = process_two_references(&num1, &num2);
        println!("数字处理结果: {}", result2);

        // 演示多个生命周期参数
        demonstrate_multiple_lifetime_parameters();
    }

    /// 处理两个引用的泛型函数
    fn process_two_references<'a, T>(first: &'a T, second: &'a T) -> &'a T
    where
        T: PartialOrd,
    {
        if first > second { first } else { second }
    }

    fn demonstrate_multiple_lifetime_parameters() {
        println!("\n--- 多个生命周期参数 ---");

        let long_data = String::from("长生命周期数据");

        {
            let short_data = String::from("短生命周期数据");

            // 函数可以接受不同生命周期的参数
            let result = combine_references(&long_data, &short_data);
            println!("组合结果: {}", result);
        }
    }

    /// 组合不同生命周期的引用
    fn combine_references<'a, 'b, T>(first: &'a T, second: &'b T) -> String
    where
        T: std::fmt::Display,
    {
        format!("{} + {}", first, second)
    }

    /// 演示特征中的生命周期多态性
    fn demonstrate_trait_lifetime_polymorphism() {
        println!("\n--- 特征中的生命周期多态性 ---");

        let data = String::from("特征数据");

        // 实现带生命周期的特征
        let processor = StringProcessor;
        let result = processor.process(&data);
        println!("特征处理结果: {}", result);

        // 演示泛型特征实现
        let generic_processor = GenericProcessorImpl;
        let result2 = generic_processor.process_generic(&data);
        println!("泛型特征处理结果: {}", result2);
    }

    /// 带生命周期参数的特征
    trait Processor<'a> {
        type Output;

        fn process(&self, input: &'a str) -> Self::Output;
    }

    struct StringProcessor;

    impl<'a> Processor<'a> for StringProcessor {
        type Output = &'a str;

        fn process(&self, input: &'a str) -> Self::Output {
            input.trim()
        }
    }

    /// 泛型特征
    trait GenericProcessor<T> {
        fn process_generic<'a>(&self, input: &'a T) -> String
        where
            T: std::fmt::Display;
    }

    struct GenericProcessorImpl;

    impl<T> GenericProcessor<T> for GenericProcessorImpl {
        fn process_generic<'a>(&self, input: &'a T) -> String
        where
            T: std::fmt::Display,
        {
            format!("处理: {}", input)
        }
    }

    /// 演示复杂的生命周期多态场景
    fn demonstrate_complex_lifetime_polymorphism() {
        println!("\n--- 复杂生命周期多态场景 ---");

        // 创建复杂的数据结构
        let mut registry = DataRegistry::new();

        let key1 = String::from("key1");
        let value1 = String::from("value1");

        registry.register(&key1, &value1);

        {
            let key2 = String::from("key2");
            let value2 = String::from("value2");

            registry.register(&key2, &value2);

            // 查询数据
            if let Some(result) = registry.lookup(&key1) {
                println!("查找结果: {}", result);
            }
        }

        // 演示生命周期约束的组合
        demonstrate_lifetime_bound_combinations();
    }

    /// 数据注册表，演示复杂的生命周期管理
    struct DataRegistry<'a> {
        data: HashMap<&'a str, &'a str>,
    }

    impl<'a> DataRegistry<'a> {
        fn new() -> Self {
            Self {
                data: HashMap::new(),
            }
        }

        fn register(&mut self, key: &'a str, value: &'a str) {
            self.data.insert(key, value);
        }

        fn lookup(&self, key: &str) -> Option<&'a str> {
            self.data.get(key).copied()
        }
    }

    fn demonstrate_lifetime_bound_combinations() {
        println!("\n--- 生命周期约束组合 ---");

        let data = String::from("约束测试数据");

        // 演示复杂的生命周期约束
        let result = complex_lifetime_function(&data, |s| s.len());
        println!("复杂生命周期函数结果: {}", result);
    }

    /// 复杂的生命周期约束函数
    fn complex_lifetime_function<'a, T, F, R>(input: &'a T, f: F) -> R
    where
        T: std::fmt::Display + 'a,
        F: Fn(&T) -> R,
        R: std::fmt::Display,
    {
        println!("处理输入: {}", input);
        f(input)
    }
}

/// 运行所有高级生命周期模式的演示
pub fn run_all_demonstrations() {
    run_all_advanced_patterns_examples();
}

/// 运行所有高级生命周期模式的演示（别名函数）
pub fn run_all_advanced_patterns_examples() {
    println!("🦀 Rust 深入生命周期 - 高级模式篇 🦀");
    println!("===========================================");

    // 1. 高阶生命周期
    higher_ranked_trait_bounds::demonstrate_hrtb_basics();

    // 2. 生命周期子类型化
    lifetime_subtyping::demonstrate_lifetime_subtyping();

    // 3. 协变与逆变
    variance::demonstrate_variance();

    // 4. 生命周期多态性
    lifetime_polymorphism::demonstrate_lifetime_polymorphism();

    println!("\n===========================================");
    println!("✅ 高级生命周期模式演示完成！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hrtb_function() {
        fn takes_hrtb<F>(f: F) -> String
        where
            F: for<'a> Fn(&'a str) -> &'a str,
        {
            let test_str = "test";
            f(test_str).to_string()
        }

        let result = takes_hrtb(|s| s);
        assert_eq!(result, "test");
    }

    #[test]
    fn test_lifetime_subtyping() {
        fn longer_lifetime<'a>(s: &'a str) -> &'a str {
            s
        }

        let long_string = String::from("long");
        let result = longer_lifetime(&long_string);
        assert_eq!(result, "long");
    }

    #[test]
    fn test_variance() {
        // 测试协变性
        let data = String::from("variance test");
        let container = variance::CovariantStruct { data: &data };

        // 这应该编译成功，因为 &T 是协变的
        fn use_container(_: &variance::CovariantStruct) {}
        use_container(&container);
    }
}
