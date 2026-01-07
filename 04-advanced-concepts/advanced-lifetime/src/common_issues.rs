//! # Rust 深入生命周期 - 常见问题和解决方案篇
//!
//! 本模块深入分析 Rust 生命周期编程中的常见问题、编译错误、最佳实践和性能考虑。
//! 通过实际案例帮助开发者理解和解决生命周期相关的问题。

use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

/// 常见编译错误分析模块
pub mod compilation_errors {
    use super::*;

    /// 演示常见的生命周期编译错误
    pub fn demonstrate_compilation_errors() {
        println!("\n=== 常见编译错误分析 ===");

        // 1. 悬垂引用错误
        demonstrate_dangling_reference_errors();

        // 2. 借用检查器错误
        demonstrate_borrow_checker_errors();

        // 3. 生命周期不匹配错误
        demonstrate_lifetime_mismatch_errors();

        // 4. 结构体生命周期错误
        demonstrate_struct_lifetime_errors();

        // 5. 闭包生命周期错误
        demonstrate_closure_lifetime_errors();
    }

    /// 演示悬垂引用错误及解决方案
    fn demonstrate_dangling_reference_errors() {
        println!("\n--- 悬垂引用错误分析 ---");

        // ❌ 错误示例：返回局部变量的引用
        // fn create_dangling_reference() -> &str {
        //     let s = String::from("hello");
        //     &s  // 错误：返回对局部变量的引用
        // }

        // ✅ 正确解决方案1：返回拥有的值
        fn create_owned_string() -> String {
            String::from("hello")
        }

        // ✅ 正确解决方案2：使用生命周期参数
        fn get_first_word(s: &str) -> &str {
            s.split_whitespace().next().unwrap_or("")
        }

        // ✅ 正确解决方案3：使用静态生命周期
        fn get_static_str() -> &'static str {
            "hello world"
        }

        let owned = create_owned_string();
        println!("拥有的字符串: {}", owned);

        let text = "hello rust world";
        let first = get_first_word(text);
        println!("第一个单词: {}", first);

        let static_str = get_static_str();
        println!("静态字符串: {}", static_str);
    }

    /// 演示借用检查器错误及解决方案
    fn demonstrate_borrow_checker_errors() {
        println!("\n--- 借用检查器错误分析 ---");

        // ❌ 错误示例：同时存在可变和不可变借用
        // let mut vec = vec![1, 2, 3];
        // let first = &vec[0];  // 不可变借用
        // vec.push(4);          // 错误：可变借用与不可变借用冲突
        // println!("First: {}", first);

        // ✅ 正确解决方案1：缩短借用作用域
        let mut vec = vec![1, 2, 3];
        {
            let first = &vec[0];
            println!("第一个元素: {}", first);
        } // 不可变借用在这里结束
        vec.push(4); // 现在可以进行可变借用
        println!("向量: {:?}", vec);

        // ✅ 正确解决方案2：克隆值
        let mut vec2 = vec![1, 2, 3];
        let first_copy = vec2[0]; // 复制值而不是借用
        vec2.push(4);
        println!("第一个元素副本: {}, 向量: {:?}", first_copy, vec2);

        // ✅ 正确解决方案3：重新设计数据流
        let mut vec3 = vec![1, 2, 3];
        vec3.push(4);
        let first = vec3[0];
        println!(
            "重新设计后 - 第一个元素: {}, 向量长度: {}",
            first,
            vec3.len()
        );
    }

    /// 演示生命周期不匹配错误及解决方案
    fn demonstrate_lifetime_mismatch_errors() {
        println!("\n--- 生命周期不匹配错误分析 ---");

        // ❌ 错误示例：生命周期参数不匹配
        // fn longest_wrong<'a>(x: &'a str, y: &str) -> &'a str {
        //     if x.len() > y.len() { x } else { y }  // 错误：y的生命周期不够长
        // }

        // ✅ 正确解决方案1：统一生命周期参数
        fn longest_correct<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() { x } else { y }
        }

        // ✅ 正确解决方案2：返回拥有的值
        fn longest_owned(x: &str, y: &str) -> String {
            if x.len() > y.len() {
                x.to_string()
            } else {
                y.to_string()
            }
        }

        let str1 = "hello";
        let str2 = "world";

        let result1 = longest_correct(str1, str2);
        println!("最长字符串: {}", result1);

        let result2 = longest_owned(str1, str2);
        println!("最长字符串(拥有): {}", result2);

        // 演示生命周期子类型化
        demonstrate_lifetime_subtyping();
    }

    /// 演示生命周期子类型化
    fn demonstrate_lifetime_subtyping() {
        println!("\n--- 生命周期子类型化 ---");

        // 较长的生命周期可以强制转换为较短的生命周期
        fn use_shorter_lifetime<'short>(s: &'short str) {
            println!("使用较短生命周期: {}", s);
        }

        let long_lived_string = String::from("long lived");
        {
            let short_lived_string = String::from("short lived");

            // 长生命周期可以用于需要短生命周期的地方
            use_shorter_lifetime(&long_lived_string);
            use_shorter_lifetime(&short_lived_string);
        }

        // 但反过来不行：短生命周期不能用于需要长生命周期的地方
    }

    /// 演示结构体生命周期错误及解决方案
    fn demonstrate_struct_lifetime_errors() {
        println!("\n--- 结构体生命周期错误分析 ---");

        // ❌ 错误示例：结构体字段生命周期不足
        // struct BadStruct<'a> {
        //     data: &'a str,
        // }
        //
        // fn create_bad_struct() -> BadStruct<'static> {
        //     let s = String::from("hello");
        //     BadStruct { data: &s }  // 错误：s的生命周期不够长
        // }

        // ✅ 正确解决方案1：使用拥有的数据
        #[derive(Debug)]
        struct OwnedStruct {
            data: String,
        }

        fn create_owned_struct() -> OwnedStruct {
            OwnedStruct {
                data: String::from("hello"),
            }
        }

        // ✅ 正确解决方案2：正确使用生命周期参数
        #[derive(Debug)]
        struct BorrowedStruct<'a> {
            data: &'a str,
        }

        fn use_borrowed_struct<'a>(s: &'a str) -> BorrowedStruct<'a> {
            BorrowedStruct { data: s }
        }

        let owned = create_owned_struct();
        println!("拥有数据的结构体: {:?}", owned);

        let text = "borrowed data";
        let borrowed = use_borrowed_struct(text);
        println!("借用数据的结构体: {:?}", borrowed);

        // 演示复杂结构体生命周期
        demonstrate_complex_struct_lifetimes();
    }

    /// 演示复杂结构体生命周期
    fn demonstrate_complex_struct_lifetimes() {
        println!("\n--- 复杂结构体生命周期 ---");

        #[derive(Debug)]
        struct ComplexStruct<'a, 'b> {
            primary: &'a str,
            secondary: &'b str,
            owned: String,
        }

        impl<'a, 'b> ComplexStruct<'a, 'b> {
            fn new(primary: &'a str, secondary: &'b str, owned: String) -> Self {
                Self {
                    primary,
                    secondary,
                    owned,
                }
            }

            fn get_primary(&self) -> &'a str {
                self.primary
            }

            fn get_secondary(&self) -> &'b str {
                self.secondary
            }

            fn get_owned(&self) -> &str {
                &self.owned
            }
        }

        let primary_data = "primary";
        let secondary_data = "secondary";
        let owned_data = String::from("owned");

        let complex = ComplexStruct::new(primary_data, secondary_data, owned_data);
        println!("复杂结构体: {:?}", complex);
        println!("主要数据: {}", complex.get_primary());
        println!("次要数据: {}", complex.get_secondary());
        println!("拥有数据: {}", complex.get_owned());
    }

    /// 演示闭包生命周期错误及解决方案
    fn demonstrate_closure_lifetime_errors() {
        println!("\n--- 闭包生命周期错误分析 ---");

        // ❌ 错误示例：闭包捕获的引用生命周期不足
        // fn create_bad_closure() -> impl Fn() -> &'static str {
        //     let s = String::from("hello");
        //     move || &s  // 错误：s被移动到闭包中，但返回的引用需要'static生命周期
        // }

        // ✅ 正确解决方案1：返回拥有的值
        fn create_owned_closure() -> impl Fn() -> String {
            let s = String::from("hello");
            move || s.clone()
        }

        // ✅ 正确解决方案2：使用静态数据
        fn create_static_closure() -> impl Fn() -> &'static str {
            || "hello world"
        }

        // ✅ 正确解决方案3：使用生命周期参数
        fn create_parameterized_closure<'a>(s: &'a str) -> impl Fn() -> &'a str + 'a {
            move || s
        }

        let owned_closure = create_owned_closure();
        println!("拥有值的闭包: {}", owned_closure());

        let static_closure = create_static_closure();
        println!("静态闭包: {}", static_closure());

        let text = "parameterized";
        let param_closure = create_parameterized_closure(text);
        println!("参数化闭包: {}", param_closure());

        // 演示闭包捕获模式
        demonstrate_closure_capture_patterns();
    }

    /// 演示闭包捕获模式
    fn demonstrate_closure_capture_patterns() {
        println!("\n--- 闭包捕获模式 ---");

        let mut counter = 0;
        let data = vec![1, 2, 3, 4, 5];

        // 不可变借用捕获
        let immutable_capture = || {
            println!("数据长度: {}", data.len());
        };

        // 可变借用捕获
        let mut mutable_capture = || {
            counter += 1;
            println!("计数器: {}", counter);
        };

        // 移动捕获
        let owned_data = vec![6, 7, 8];
        let move_capture = move || {
            println!("拥有的数据: {:?}", owned_data);
        };

        immutable_capture();
        mutable_capture();
        move_capture();

        println!("原始数据仍可访问: {:?}", data);
        println!("最终计数器值: {}", counter);
        // println!("拥有的数据已被移动: {:?}", owned_data);  // 错误：值已被移动
    }
}

/// 最佳实践指南模块
pub mod best_practices {
    use super::*;

    /// 拥有所有权的配置结构体
    #[derive(Debug, Clone)]
    pub struct OwnedConfig {
        pub name: String,
        pub version: String,
        pub settings: HashMap<String, String>,
    }

    impl OwnedConfig {
        pub fn new(name: String, version: String) -> Self {
            Self {
                name,
                version,
                settings: HashMap::new(),
            }
        }

        pub fn add_setting(&mut self, key: String, value: String) {
            self.settings.insert(key, value);
        }
    }

    /// 演示生命周期最佳实践
    pub fn demonstrate_best_practices() {
        println!("\n=== 生命周期最佳实践 ===");

        // 1. 生命周期设计原则
        demonstrate_lifetime_design_principles();

        // 2. 结构体设计最佳实践
        demonstrate_struct_design_practices();

        // 3. 函数签名最佳实践
        demonstrate_function_signature_practices();

        // 4. 错误处理最佳实践
        demonstrate_error_handling_practices();

        // 5. 性能优化最佳实践
        demonstrate_performance_practices();
    }

    /// 演示生命周期设计原则
    fn demonstrate_lifetime_design_principles() {
        println!("\n--- 生命周期设计原则 ---");

        // 原则1：优先使用拥有的数据
        println!("\n1. 优先使用拥有的数据");

        let mut config = OwnedConfig::new("MyApp".to_string(), "1.0.0".to_string());
        config.add_setting("debug".to_string(), "true".to_string());
        println!("拥有数据的配置: {:?}", config);

        // 原则2：只在必要时使用借用
        println!("\n2. 只在必要时使用借用");

        fn process_text(text: &str) -> usize {
            text.lines().count()
        }

        fn analyze_config(config: &OwnedConfig) -> String {
            format!(
                "配置 {} v{} 有 {} 个设置",
                config.name,
                config.version,
                config.settings.len()
            )
        }

        let text = "line1\nline2\nline3";
        let line_count = process_text(text);
        println!("文本行数: {}", line_count);

        let analysis = analyze_config(&config);
        println!("配置分析: {}", analysis);

        // 原则3：使用生命周期省略规则
        println!("\n3. 利用生命周期省略规则");

        // 自动推断生命周期
        fn get_first_line(text: &str) -> &str {
            text.lines().next().unwrap_or("")
        }

        // 显式生命周期（仅在必要时）
        fn choose_longer<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() { x } else { y }
        }

        let first_line = get_first_line(text);
        println!("第一行: {}", first_line);

        let longer = choose_longer("short", "longer string");
        println!("更长的字符串: {}", longer);
    }

    /// 演示结构体设计最佳实践
    fn demonstrate_struct_design_practices() {
        println!("\n--- 结构体设计最佳实践 ---");

        // 实践1：混合拥有和借用数据
        #[derive(Debug)]
        struct Document<'content> {
            title: String,                     // 拥有的数据
            author: String,                    // 拥有的数据
            content: &'content str,            // 借用的数据
            metadata: HashMap<String, String>, // 拥有的数据
        }

        impl<'content> Document<'content> {
            fn new(title: String, author: String, content: &'content str) -> Self {
                Self {
                    title,
                    author,
                    content,
                    metadata: HashMap::new(),
                }
            }

            fn add_metadata(&mut self, key: String, value: String) {
                self.metadata.insert(key, value);
            }

            fn word_count(&self) -> usize {
                self.content.split_whitespace().count()
            }

            fn summary(&self) -> String {
                format!(
                    "《{}》 by {} ({} words)",
                    self.title,
                    self.author,
                    self.word_count()
                )
            }
        }

        let content = "This is a sample document content with multiple words.";
        let mut doc = Document::new(
            "Sample Document".to_string(),
            "John Doe".to_string(),
            content,
        );

        doc.add_metadata("category".to_string(), "sample".to_string());
        doc.add_metadata("language".to_string(), "english".to_string());

        println!("文档: {:?}", doc);
        println!("文档摘要: {}", doc.summary());

        // 实践2：使用构建器模式
        demonstrate_builder_pattern();
    }

    /// 演示构建器模式最佳实践
    fn demonstrate_builder_pattern() {
        println!("\n--- 构建器模式最佳实践 ---");

        #[derive(Debug)]
        struct HttpRequest<'a> {
            method: String,
            url: String,
            headers: HashMap<String, String>,
            body: Option<&'a str>,
        }

        struct HttpRequestBuilder<'a> {
            method: Option<String>,
            url: Option<String>,
            headers: HashMap<String, String>,
            body: Option<&'a str>,
        }

        impl<'a> HttpRequestBuilder<'a> {
            fn new() -> Self {
                Self {
                    method: None,
                    url: None,
                    headers: HashMap::new(),
                    body: None,
                }
            }

            fn method(mut self, method: &str) -> Self {
                self.method = Some(method.to_string());
                self
            }

            fn url(mut self, url: &str) -> Self {
                self.url = Some(url.to_string());
                self
            }

            fn header(mut self, key: &str, value: &str) -> Self {
                self.headers.insert(key.to_string(), value.to_string());
                self
            }

            fn body(mut self, body: &'a str) -> Self {
                self.body = Some(body);
                self
            }

            fn build(self) -> Result<HttpRequest<'a>, &'static str> {
                let method = self.method.ok_or("Method is required")?;
                let url = self.url.ok_or("URL is required")?;

                Ok(HttpRequest {
                    method,
                    url,
                    headers: self.headers,
                    body: self.body,
                })
            }
        }

        let request_body = r#"{"name": "John", "age": 30}"#;

        let request = HttpRequestBuilder::new()
            .method("POST")
            .url("https://api.example.com/users")
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer token123")
            .body(request_body)
            .build()
            .expect("Failed to build request");

        println!("HTTP 请求: {:?}", request);
    }

    /// 演示函数签名最佳实践
    fn demonstrate_function_signature_practices() {
        println!("\n--- 函数签名最佳实践 ---");

        // 实践1：返回拥有的数据而不是借用
        fn process_and_own(input: &str) -> String {
            input.to_uppercase()
        }

        // 实践2：使用泛型减少生命周期复杂性
        fn find_item<T: PartialEq>(items: &[T], target: &T) -> Option<usize> {
            items.iter().position(|item| item == target)
        }

        // 实践3：合理使用生命周期参数
        fn merge_strings<'a>(parts: &[&'a str], separator: &str) -> String {
            parts.join(separator)
        }

        let input = "hello world";
        let processed = process_and_own(input);
        println!("处理后的字符串: {}", processed);

        let numbers = vec![1, 2, 3, 4, 5];
        let index = find_item(&numbers, &3);
        println!("找到数字3的索引: {:?}", index);

        let parts = vec!["hello", "world", "rust"];
        let merged = merge_strings(&parts, " ");
        println!("合并的字符串: {}", merged);
    }

    /// 演示错误处理最佳实践
    fn demonstrate_error_handling_practices() {
        println!("\n--- 错误处理最佳实践 ---");

        #[derive(Debug)]
        enum ParseError {
            InvalidFormat,
            MissingField(String),
            InvalidValue(String),
        }

        impl std::fmt::Display for ParseError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    ParseError::InvalidFormat => write!(f, "Invalid format"),
                    ParseError::MissingField(field) => write!(f, "Missing field: {}", field),
                    ParseError::InvalidValue(value) => write!(f, "Invalid value: {}", value),
                }
            }
        }

        impl std::error::Error for ParseError {}

        #[derive(Debug)]
        struct User {
            name: String,
            age: u32,
            email: String,
        }

        fn parse_user(data: &str) -> Result<User, ParseError> {
            let parts: Vec<&str> = data.split(',').collect();

            if parts.len() != 3 {
                return Err(ParseError::InvalidFormat);
            }

            let name = parts[0].trim();
            if name.is_empty() {
                return Err(ParseError::MissingField("name".to_string()));
            }

            let age = parts[1]
                .trim()
                .parse::<u32>()
                .map_err(|_| ParseError::InvalidValue(parts[1].to_string()))?;

            let email = parts[2].trim();
            if !email.contains('@') {
                return Err(ParseError::InvalidValue(email.to_string()));
            }

            Ok(User {
                name: name.to_string(),
                age,
                email: email.to_string(),
            })
        }

        let valid_data = "John Doe, 30, john@example.com";
        let invalid_data = "Jane, invalid_age, jane@example.com";

        match parse_user(valid_data) {
            Ok(user) => println!("解析成功: {:?}", user),
            Err(e) => println!("解析失败: {}", e),
        }

        match parse_user(invalid_data) {
            Ok(user) => println!("解析成功: {:?}", user),
            Err(e) => println!("解析失败: {}", e),
        }
    }

    /// 演示性能优化最佳实践
    fn demonstrate_performance_practices() {
        println!("\n--- 性能优化最佳实践 ---");

        // 实践1：避免不必要的克隆
        fn process_efficiently(data: &[String]) -> Vec<usize> {
            data.iter().map(|s| s.len()).collect()
        }

        // 实践2：使用字符串切片而不是String
        fn count_words_efficiently(text: &str) -> HashMap<&str, usize> {
            let mut counts = HashMap::new();
            for word in text.split_whitespace() {
                *counts.entry(word).or_insert(0) += 1;
            }
            counts
        }

        // 实践3：合理使用Cow（Clone on Write）
        use std::borrow::Cow;

        fn normalize_text<'a>(text: &'a str) -> Cow<'a, str> {
            if text.chars().all(|c| c.is_lowercase()) {
                Cow::Borrowed(text) // 不需要修改，直接借用
            } else {
                Cow::Owned(text.to_lowercase()) // 需要修改，创建拥有的版本
            }
        }

        let data = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];

        let lengths = process_efficiently(&data);
        println!("字符串长度: {:?}", lengths);

        let text = "hello world hello rust world";
        let word_counts = count_words_efficiently(text);
        println!("单词计数: {:?}", word_counts);

        let text1 = "already lowercase";
        let text2 = "NEEDS CONVERSION";

        let normalized1 = normalize_text(text1);
        let normalized2 = normalize_text(text2);

        println!(
            "规范化文本1: {} (borrowed: {})",
            normalized1,
            matches!(normalized1, Cow::Borrowed(_))
        );
        println!(
            "规范化文本2: {} (borrowed: {})",
            normalized2,
            matches!(normalized2, Cow::Borrowed(_))
        );
    }
}

/// 性能考虑模块
pub mod performance_considerations {
    use super::*;
    use std::time::Instant;

    /// 内存高效的处理器
    #[derive(Debug)]
    pub struct EfficientProcessor<'data> {
        input: &'data str,
        buffer: Vec<&'data str>,
    }

    impl<'data> EfficientProcessor<'data> {
        pub fn new(input: &'data str) -> Self {
            Self {
                input,
                buffer: Vec::new(),
            }
        }

        pub fn process(&mut self) {
            self.buffer = self.input.split_whitespace().collect();
        }

        pub fn get_word_count(&self) -> usize {
            self.buffer.len()
        }

        fn memory_usage(&self) -> usize {
            std::mem::size_of_val(self) + self.buffer.capacity() * std::mem::size_of::<&str>()
        }
    }

    /// 点云数据结构
    #[derive(Debug)]
    pub struct PointCloud {
        x_coords: Vec<f64>,
        y_coords: Vec<f64>,
        z_coords: Vec<f64>,
    }

    impl PointCloud {
        pub fn new() -> Self {
            Self {
                x_coords: Vec::new(),
                y_coords: Vec::new(),
                z_coords: Vec::new(),
            }
        }

        pub fn add_point(&mut self, x: f64, y: f64, z: f64) {
            self.x_coords.push(x);
            self.y_coords.push(y);
            self.z_coords.push(z);
        }

        pub fn sum_x_coords(&self) -> f64 {
            self.x_coords.iter().sum()
        }

        pub fn len(&self) -> usize {
            self.x_coords.len()
        }
    }

    /// 演示生命周期相关的性能考虑
    pub fn demonstrate_performance_considerations() {
        println!("\n=== 性能考虑 ===");

        // 1. 借用 vs 拥有的性能对比
        demonstrate_borrowing_vs_owning_performance();

        // 2. 生命周期对内存使用的影响
        demonstrate_memory_usage_impact();

        // 3. 零成本抽象验证
        demonstrate_zero_cost_abstractions();

        // 4. 缓存友好的数据结构
        demonstrate_cache_friendly_structures();
    }

    /// 演示借用与拥有的性能对比
    fn demonstrate_borrowing_vs_owning_performance() {
        println!("\n--- 借用 vs 拥有性能对比 ---");

        const ITERATIONS: usize = 1_000_000;
        let data = "hello world rust programming language";

        // 测试借用性能
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let _result = process_borrowed(data);
        }
        let borrowed_time = start.elapsed();

        // 测试拥有性能
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let _result = process_owned(data);
        }
        let owned_time = start.elapsed();

        println!("借用处理时间: {:?}", borrowed_time);
        println!("拥有处理时间: {:?}", owned_time);
        println!(
            "性能差异: {:.2}x",
            owned_time.as_nanos() as f64 / borrowed_time.as_nanos() as f64
        );

        fn process_borrowed(s: &str) -> usize {
            s.split_whitespace().count()
        }

        fn process_owned(s: &str) -> usize {
            let owned = s.to_string();
            owned.split_whitespace().count()
        }
    }

    /// 演示生命周期对内存使用的影响
    fn demonstrate_memory_usage_impact() {
        println!("\n--- 内存使用影响 ---");

        // 内存高效的设计：使用借用

        // 内存密集的设计：使用拥有的数据
        #[derive(Debug)]
        struct IneffientProcessor {
            input: String,
            buffer: Vec<String>,
        }

        impl IneffientProcessor {
            fn new(input: &str) -> Self {
                Self {
                    input: input.to_string(),
                    buffer: Vec::new(),
                }
            }

            fn process(&mut self) {
                self.buffer = self
                    .input
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
            }

            fn get_word_count(&self) -> usize {
                self.buffer.len()
            }

            fn memory_usage(&self) -> usize {
                std::mem::size_of_val(self)
                    + self.input.capacity()
                    + self.buffer.iter().map(|s| s.capacity()).sum::<usize>()
                    + self.buffer.capacity() * std::mem::size_of::<String>()
            }
        }

        let text = "the quick brown fox jumps over the lazy dog";

        let mut efficient = EfficientProcessor::new(text);
        efficient.process();

        let mut inefficient = IneffientProcessor::new(text);
        inefficient.process();

        println!("高效处理器内存使用: {} bytes", efficient.memory_usage());
        println!("低效处理器内存使用: {} bytes", inefficient.memory_usage());
        println!(
            "内存使用差异: {:.2}x",
            inefficient.memory_usage() as f64 / efficient.memory_usage() as f64
        );
    }

    /// 演示零成本抽象验证
    fn demonstrate_zero_cost_abstractions() {
        println!("\n--- 零成本抽象验证 ---");

        const SIZE: usize = 1_000_000;
        let data: Vec<i32> = (0..SIZE as i32).collect();

        // 手动循环
        let start = Instant::now();
        let mut sum1 = 0i64;
        for i in 0..data.len() {
            sum1 += data[i] as i64;
        }
        let manual_time = start.elapsed();

        // 迭代器抽象
        let start = Instant::now();
        let sum2: i64 = data.iter().map(|&x| x as i64).sum();
        let iterator_time = start.elapsed();

        // 生命周期抽象
        let start = Instant::now();
        let sum3 = sum_with_lifetime(&data);
        let lifetime_time = start.elapsed();

        println!("手动循环结果: {}, 时间: {:?}", sum1, manual_time);
        println!("迭代器抽象结果: {}, 时间: {:?}", sum2, iterator_time);
        println!("生命周期抽象结果: {}, 时间: {:?}", sum3, lifetime_time);

        assert_eq!(sum1, sum2);
        assert_eq!(sum2, sum3);

        fn sum_with_lifetime(data: &[i32]) -> i64 {
            data.iter().map(|&x| x as i64).sum()
        }
    }

    /// 演示缓存友好的数据结构
    fn demonstrate_cache_friendly_structures() {
        println!("\n--- 缓存友好的数据结构 ---");

        // 结构体数组（缓存友好）
        #[derive(Debug, Clone)]
        struct Point {
            x: f64,
            y: f64,
            z: f64,
        }

        // 数组结构体（更缓存友好）

        const POINT_COUNT: usize = 100_000;

        // 测试结构体数组
        let mut points = Vec::new();
        for i in 0..POINT_COUNT {
            points.push(Point {
                x: i as f64,
                y: (i * 2) as f64,
                z: (i * 3) as f64,
            });
        }

        let start = Instant::now();
        let sum1: f64 = points.iter().map(|p| p.x).sum();
        let aos_time = start.elapsed();

        // 测试数组结构体
        let mut point_cloud = PointCloud::new();
        for i in 0..POINT_COUNT {
            point_cloud.add_point(i as f64, (i * 2) as f64, (i * 3) as f64);
        }

        let start = Instant::now();
        let sum2 = point_cloud.sum_x_coords();
        let soa_time = start.elapsed();

        println!("结构体数组 (AoS) 求和: {}, 时间: {:?}", sum1, aos_time);
        println!("数组结构体 (SoA) 求和: {}, 时间: {:?}", sum2, soa_time);
        println!(
            "性能提升: {:.2}x",
            aos_time.as_nanos() as f64 / soa_time.as_nanos() as f64
        );

        assert!((sum1 - sum2).abs() < f64::EPSILON);
    }
}

/// 运行所有常见问题和解决方案的演示
pub fn run_all_demonstrations() {
    run_all_common_issues_examples();
}

/// 运行所有常见问题和解决方案的演示（别名函数）
pub fn run_all_common_issues_examples() {
    println!("🦀 Rust 深入生命周期 - 常见问题和解决方案篇 🦀");
    println!("================================================");

    // 1. 常见编译错误分析
    compilation_errors::demonstrate_compilation_errors();

    // 2. 最佳实践指南
    best_practices::demonstrate_best_practices();

    // 3. 性能考虑
    performance_considerations::demonstrate_performance_considerations();

    println!("\n================================================");
    println!("✅ 常见问题和解决方案演示完成！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_owned_config() {
        let mut config = best_practices::OwnedConfig::new("Test".to_string(), "1.0".to_string());
        config.add_setting("key".to_string(), "value".to_string());
        assert_eq!(config.settings.len(), 1);
    }

    #[test]
    fn test_efficient_processor() {
        let text = "hello world";
        let mut processor = performance_considerations::EfficientProcessor::new(text);
        processor.process();
        assert_eq!(processor.get_word_count(), 2);
    }

    #[test]
    fn test_point_cloud() {
        let mut cloud = performance_considerations::PointCloud::new();
        cloud.add_point(1.0, 2.0, 3.0);
        cloud.add_point(4.0, 5.0, 6.0);
        assert_eq!(cloud.len(), 2);
        assert_eq!(cloud.sum_x_coords(), 5.0);
    }
}
