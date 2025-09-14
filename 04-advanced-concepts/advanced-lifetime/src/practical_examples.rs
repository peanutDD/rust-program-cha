//! # 生命周期实际应用案例（Practical Lifetime Examples）
//!
//! 本模块通过实际案例深入探讨 Rust 生命周期的应用，包括：
//! - 结构体中的生命周期设计
//! - 方法中的生命周期处理
//! - 复杂数据结构的生命周期管理
//! - 实际项目中的生命周期模式
//! - 性能优化与生命周期的平衡

use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::rc::Rc;

/// # 结构体生命周期设计案例
///
/// 展示如何在结构体中正确设计和使用生命周期
pub mod struct_lifetime_design {
    use std::collections::HashMap;
    use std::fmt::Display;

    /// 演示结构体生命周期设计
    pub fn demonstrate_struct_lifetime_design() {
        println!("=== 结构体生命周期设计案例 ===");

        // 1. 基础结构体生命周期
        demonstrate_basic_struct_lifetimes();

        // 2. 多重生命周期结构体
        demonstrate_multiple_lifetime_structs();

        // 3. 嵌套结构体生命周期
        demonstrate_nested_struct_lifetimes();

        // 4. 生命周期与所有权的平衡
        demonstrate_lifetime_ownership_balance();
    }

    /// 演示基础结构体生命周期
    fn demonstrate_basic_struct_lifetimes() {
        println!("\n--- 基础结构体生命周期 ---");

        // 案例1：配置管理器
        let config_data = "database_url=localhost:5432;timeout=30";
        let config = ConfigManager::new(config_data);

        config.display_config();
        println!("数据库URL: {}", config.get_database_url());

        // 案例2：文档解析器
        demonstrate_document_parser();
    }

    /// 配置管理器 - 持有配置字符串的引用
    pub struct ConfigManager<'config> {
        raw_config: &'config str,
        parsed_values: HashMap<String, String>,
    }

    impl<'config> ConfigManager<'config> {
        /// 创建新的配置管理器
        pub fn new(config_str: &'config str) -> Self {
            let mut parsed = HashMap::new();

            // 解析配置字符串
            for pair in config_str.split(';') {
                if let Some((key, value)) = pair.split_once('=') {
                    parsed.insert(key.to_string(), value.to_string());
                }
            }

            Self {
                raw_config: config_str,
                parsed_values: parsed,
            }
        }

        /// 显示配置信息
        fn display_config(&self) {
            println!("原始配置: {}", self.raw_config);
            println!("解析后的配置: {:?}", self.parsed_values);
        }

        /// 获取数据库URL
        fn get_database_url(&self) -> &str {
            self.parsed_values
                .get("database_url")
                .map(|s| s.as_str())
                .unwrap_or("未找到数据库URL")
        }

        /// 获取原始配置的引用（生命周期与输入相同）
        pub fn get_raw_config(&self) -> &'config str {
            self.raw_config
        }
    }

    fn demonstrate_document_parser() {
        println!("\n--- 文档解析器案例 ---");

        let document_content = "# 标题\n\n这是一个段落。\n\n## 子标题\n\n另一个段落。";
        let parser = DocumentParser::new(document_content);

        parser.analyze();

        // 获取文档片段
        if let Some(title) = parser.get_title() {
            println!("文档标题: {}", title);
        }

        let sections = parser.get_sections();
        println!("文档章节数: {}", sections.len());
    }

    /// 文档解析器 - 解析 Markdown 文档
    struct DocumentParser<'doc> {
        content: &'doc str,
        lines: Vec<&'doc str>,
    }

    impl<'doc> DocumentParser<'doc> {
        fn new(content: &'doc str) -> Self {
            let lines = content.lines().collect();
            Self { content, lines }
        }

        fn analyze(&self) {
            println!("分析文档，总行数: {}", self.lines.len());
            println!("文档长度: {} 字符", self.content.len());
        }

        /// 获取文档标题（第一个 # 开头的行）
        fn get_title(&self) -> Option<&'doc str> {
            self.lines
                .iter()
                .find(|line| line.starts_with("# "))
                .map(|line| &line[2..]) // 移除 "# "
        }

        /// 获取所有章节
        fn get_sections(&self) -> Vec<&'doc str> {
            self.lines
                .iter()
                .filter(|line| line.starts_with('#'))
                .map(|line| line.trim_start_matches('#').trim())
                .collect()
        }

        /// 获取指定行的引用
        fn get_line(&self, index: usize) -> Option<&'doc str> {
            self.lines.get(index).copied()
        }
    }

    /// 演示多重生命周期结构体
    fn demonstrate_multiple_lifetime_structs() {
        println!("\n--- 多重生命周期结构体 ---");

        let primary_data = "主要数据源";
        let secondary_data = "次要数据源";
        let metadata = "元数据";

        // 创建多重生命周期的数据处理器
        let processor = MultiSourceProcessor::new(primary_data, secondary_data, metadata);
        processor.process();

        // 演示生命周期关系
        demonstrate_lifetime_relationships();
    }

    /// 多数据源处理器 - 具有多个生命周期参数
    struct MultiSourceProcessor<'primary, 'secondary, 'meta> {
        primary: &'primary str,
        secondary: &'secondary str,
        metadata: &'meta str,
        processing_stats: ProcessingStats,
    }

    #[derive(Debug, Default)]
    struct ProcessingStats {
        primary_processed: usize,
        secondary_processed: usize,
        total_operations: usize,
    }

    impl<'primary, 'secondary, 'meta> MultiSourceProcessor<'primary, 'secondary, 'meta> {
        fn new(primary: &'primary str, secondary: &'secondary str, metadata: &'meta str) -> Self {
            Self {
                primary,
                secondary,
                metadata,
                processing_stats: ProcessingStats::default(),
            }
        }

        fn process(&self) {
            println!("处理主要数据: {}", self.primary);
            println!("处理次要数据: {}", self.secondary);
            println!("使用元数据: {}", self.metadata);
        }

        /// 获取主要数据的引用
        fn get_primary(&self) -> &'primary str {
            self.primary
        }

        /// 获取次要数据的引用
        fn get_secondary(&self) -> &'secondary str {
            self.secondary
        }

        /// 合并数据（返回最短生命周期）
        fn merge_data(&self) -> String
        where
            'primary: 'secondary, // 要求主要数据比次要数据活得更久
        {
            format!("{} + {} ({})", self.primary, self.secondary, self.metadata)
        }
    }

    fn demonstrate_lifetime_relationships() {
        println!("\n--- 生命周期关系演示 ---");

        let long_lived = String::from("长生命周期数据");

        {
            let medium_lived = String::from("中等生命周期数据");

            {
                let short_lived = String::from("短生命周期数据");

                // 创建具有不同生命周期关系的处理器
                let relationship_demo =
                    LifetimeRelationshipDemo::new(&long_lived, &medium_lived, &short_lived);

                relationship_demo.demonstrate_relationships();
            }
        }
    }

    /// 生命周期关系演示结构体
    struct LifetimeRelationshipDemo<'long, 'medium, 'short> {
        long_data: &'long str,
        medium_data: &'medium str,
        short_data: &'short str,
    }

    impl<'long, 'medium, 'short> LifetimeRelationshipDemo<'long, 'medium, 'short>
    where
        'long: 'medium,  // 长生命周期比中等生命周期活得更久
        'medium: 'short, // 中等生命周期比短生命周期活得更久
    {
        fn new(long_data: &'long str, medium_data: &'medium str, short_data: &'short str) -> Self {
            Self {
                long_data,
                medium_data,
                short_data,
            }
        }

        fn demonstrate_relationships(&self) {
            println!("长生命周期数据: {}", self.long_data);
            println!("中等生命周期数据: {}", self.medium_data);
            println!("短生命周期数据: {}", self.short_data);

            // 由于生命周期约束，可以将长生命周期的数据赋给短生命周期的变量
            let _short_ref: &'short str = self.long_data; // 合法
            let _short_ref2: &'short str = self.medium_data; // 合法
        }

        /// 返回最短生命周期的数据
        fn get_shortest_lived(&self) -> &'short str {
            // 可以返回任何一个，因为它们都至少活到 'short
            self.short_data
        }
    }

    /// 演示嵌套结构体生命周期
    fn demonstrate_nested_struct_lifetimes() {
        println!("\n--- 嵌套结构体生命周期 ---");

        let user_data = "用户:张三,年龄:30,邮箱:zhangsan@example.com";
        let session_data = "会话ID:abc123,过期时间:2024-12-31";

        // 创建用户会话
        let user_session = UserSession::new(user_data, session_data);
        user_session.display_info();

        // 演示嵌套访问
        if let Some(user_info) = user_session.get_user_info() {
            println!("用户信息: {}", user_info.get_name());
        }
    }

    /// 用户会话 - 包含嵌套的用户信息
    struct UserSession<'session> {
        session_data: &'session str,
        user_info: Option<UserInfo<'session>>,
    }

    /// 用户信息 - 嵌套在用户会话中
    struct UserInfo<'user> {
        raw_data: &'user str,
        name: String,
        age: u32,
        email: String,
    }

    impl<'session> UserSession<'session> {
        fn new(user_data: &'session str, session_data: &'session str) -> Self {
            let user_info = UserInfo::parse(user_data);

            Self {
                session_data,
                user_info: Some(user_info),
            }
        }

        fn display_info(&self) {
            println!("会话数据: {}", self.session_data);
            if let Some(ref user) = self.user_info {
                println!("用户: {} ({}岁)", user.name, user.age);
            }
        }

        fn get_user_info(&self) -> Option<&UserInfo<'session>> {
            self.user_info.as_ref()
        }
    }

    impl<'user> UserInfo<'user> {
        fn parse(data: &'user str) -> Self {
            let mut name = String::new();
            let mut age = 0;
            let mut email = String::new();

            // 简单解析（实际项目中会更复杂）
            for part in data.split(',') {
                if let Some((key, value)) = part.split_once(':') {
                    match key {
                        "用户" => name = value.to_string(),
                        "年龄" => age = value.parse().unwrap_or(0),
                        "邮箱" => email = value.to_string(),
                        _ => {}
                    }
                }
            }

            Self {
                raw_data: data,
                name,
                age,
                email,
            }
        }

        fn get_name(&self) -> &str {
            &self.name
        }

        fn get_raw_data(&self) -> &'user str {
            self.raw_data
        }
    }

    /// 演示生命周期与所有权的平衡
    fn demonstrate_lifetime_ownership_balance() {
        println!("\n--- 生命周期与所有权平衡 ---");

        // 案例1：缓存系统
        demonstrate_cache_system();

        // 案例2：资源管理器
        demonstrate_resource_manager();
    }

    fn demonstrate_cache_system() {
        println!("\n--- 缓存系统案例 ---");

        let key1 = "user:123";
        let value1 = "张三的用户数据";

        let mut cache = SimpleCache::new();
        cache.insert(key1, value1);

        if let Some(cached_value) = cache.get(key1) {
            println!("缓存命中: {}", cached_value);
        }

        cache.display_stats();
    }

    /// 简单缓存系统 - 平衡生命周期和所有权
    struct SimpleCache<'cache> {
        data: HashMap<&'cache str, &'cache str>,
        hit_count: usize,
        miss_count: usize,
    }

    impl<'cache> SimpleCache<'cache> {
        fn new() -> Self {
            Self {
                data: HashMap::new(),
                hit_count: 0,
                miss_count: 0,
            }
        }

        fn insert(&mut self, key: &'cache str, value: &'cache str) {
            self.data.insert(key, value);
        }

        fn get(&mut self, key: &str) -> Option<&'cache str> {
            match self.data.get(key) {
                Some(value) => {
                    self.hit_count += 1;
                    Some(*value)
                }
                None => {
                    self.miss_count += 1;
                    None
                }
            }
        }

        fn display_stats(&self) {
            println!(
                "缓存统计 - 命中: {}, 未命中: {}",
                self.hit_count, self.miss_count
            );
            println!("缓存条目数: {}", self.data.len());
        }
    }

    fn demonstrate_resource_manager() {
        println!("\n--- 资源管理器案例 ---");

        let resource_config = "资源配置:最大连接数=100,超时=30秒";
        let mut manager = ResourceManager::new(resource_config);

        // 获取资源
        if let Some(resource) = manager.acquire_resource("database") {
            println!("获取资源: {}", resource.get_name());
            resource.use_resource();
        }

        manager.display_status();
    }

    /// 资源管理器
    struct ResourceManager<'config> {
        config: &'config str,
        active_resources: Vec<Resource>,
        resource_count: usize,
    }

    /// 资源
    struct Resource {
        name: String,
        id: usize,
        in_use: bool,
    }

    impl<'config> ResourceManager<'config> {
        fn new(config: &'config str) -> Self {
            Self {
                config,
                active_resources: Vec::new(),
                resource_count: 0,
            }
        }

        fn acquire_resource(&mut self, name: &str) -> Option<&mut Resource> {
            // 创建新资源
            let resource = Resource {
                name: name.to_string(),
                id: self.resource_count,
                in_use: true,
            };

            self.resource_count += 1;
            self.active_resources.push(resource);

            self.active_resources.last_mut()
        }

        fn display_status(&self) {
            println!("资源管理器状态:");
            println!("配置: {}", self.config);
            println!("活跃资源数: {}", self.active_resources.len());
        }
    }

    impl Resource {
        fn get_name(&self) -> &str {
            &self.name
        }

        fn use_resource(&self) {
            println!("使用资源 {} (ID: {})", self.name, self.id);
        }
    }
}

/// # 方法中的生命周期处理
///
/// 展示如何在方法中正确处理生命周期
pub mod method_lifetime_handling {
    use std::collections::HashMap;
    use std::fmt::Display;

    /// 演示方法中的生命周期处理
    pub fn demonstrate_method_lifetime_handling() {
        println!("\n=== 方法中的生命周期处理 ===");

        // 1. 基础方法生命周期
        demonstrate_basic_method_lifetimes();

        // 2. 方法链式调用
        demonstrate_method_chaining();

        // 3. 生命周期省略在方法中的应用
        demonstrate_lifetime_elision_in_methods();

        // 4. 复杂方法生命周期场景
        demonstrate_complex_method_scenarios();
    }

    /// 演示基础方法生命周期
    fn demonstrate_basic_method_lifetimes() {
        println!("\n--- 基础方法生命周期 ---");

        let text = "这是一个测试文本，包含多个单词和句子。";
        let analyzer = TextAnalyzer::new(text);

        // 调用各种分析方法
        println!("文本长度: {}", analyzer.get_length());
        println!("单词数: {}", analyzer.count_words());

        if let Some(first_word) = analyzer.get_first_word() {
            println!("第一个单词: {}", first_word);
        }

        let words = analyzer.get_words();
        println!("所有单词: {:?}", words);
    }

    /// 文本分析器 - 演示方法中的生命周期
    pub struct TextAnalyzer<'text> {
        content: &'text str,
        word_cache: Option<Vec<&'text str>>,
    }

    impl<'text> TextAnalyzer<'text> {
        /// 创建新的文本分析器
        pub fn new(content: &'text str) -> Self {
            Self {
                content,
                word_cache: None,
            }
        }

        /// 获取文本长度（不涉及生命周期）
        pub fn get_length(&self) -> usize {
            self.content.len()
        }

        /// 计算单词数（不涉及生命周期）
        pub fn count_words(&self) -> usize {
            self.content.split_whitespace().count()
        }

        /// 获取第一个单词（返回与输入相同的生命周期）
        pub fn get_first_word(&self) -> Option<&'text str> {
            self.content.split_whitespace().next()
        }

        /// 获取最后一个单词
        fn get_last_word(&self) -> Option<&'text str> {
            self.content.split_whitespace().last()
        }

        /// 获取所有单词（返回与输入相同的生命周期）
        fn get_words(&self) -> Vec<&'text str> {
            self.content.split_whitespace().collect()
        }

        /// 查找包含指定子串的单词
        fn find_words_containing(&self, pattern: &str) -> Vec<&'text str> {
            self.content
                .split_whitespace()
                .filter(|word| word.contains(pattern))
                .collect()
        }

        /// 获取指定范围的文本
        fn get_substring(&self, start: usize, end: usize) -> Option<&'text str> {
            if end <= self.content.len() && start <= end {
                Some(&self.content[start..end])
            } else {
                None
            }
        }

        /// 比较两个文本分析器的内容
        fn compare_with(&self, other: &TextAnalyzer<'_>) -> std::cmp::Ordering {
            self.content.cmp(other.content)
        }
    }

    /// 演示方法链式调用
    fn demonstrate_method_chaining() {
        println!("\n--- 方法链式调用 ---");

        let data = "原始数据";
        let result = DataProcessor::new(data)
            .transform("步骤1")
            .transform("步骤2")
            .transform("步骤3")
            .finalize();

        println!("链式处理结果: {}", result);

        // 演示更复杂的链式调用
        demonstrate_complex_chaining();
    }

    /// 数据处理器 - 支持链式调用
    struct DataProcessor<'data> {
        original: &'data str,
        transformations: Vec<String>,
    }

    impl<'data> DataProcessor<'data> {
        fn new(data: &'data str) -> Self {
            Self {
                original: data,
                transformations: Vec::new(),
            }
        }

        /// 转换数据（返回 self 以支持链式调用）
        fn transform(mut self, step: &str) -> Self {
            self.transformations.push(format!("应用 {}", step));
            self
        }

        /// 完成处理
        fn finalize(self) -> String {
            format!(
                "原始: {} -> 转换: {:?}",
                self.original, self.transformations
            )
        }

        /// 获取原始数据
        fn get_original(&self) -> &'data str {
            self.original
        }
    }

    fn demonstrate_complex_chaining() {
        println!("\n--- 复杂链式调用 ---");

        let input = "复杂数据处理示例";
        let mut builder = ComplexBuilder::new(input);

        let result = builder
            .add_metadata("版本", "1.0")
            .add_metadata("作者", "Rust开发者")
            .process_with_filter(|s| s.len() > 2)
            .build();

        println!("复杂构建结果: {:?}", result);
    }

    /// 复杂构建器
    struct ComplexBuilder<'input> {
        input: &'input str,
        metadata: HashMap<String, String>,
        filters: Vec<Box<dyn Fn(&str) -> bool>>,
    }

    #[derive(Debug)]
    struct ComplexResult<'input> {
        original: &'input str,
        metadata: HashMap<String, String>,
        processed_data: Vec<String>,
    }

    impl<'input> ComplexBuilder<'input> {
        fn new(input: &'input str) -> Self {
            Self {
                input,
                metadata: HashMap::new(),
                filters: Vec::new(),
            }
        }

        fn add_metadata(mut self, key: &str, value: &str) -> Self {
            self.metadata.insert(key.to_string(), value.to_string());
            self
        }

        fn process_with_filter<F>(mut self, filter: F) -> Self
        where
            F: Fn(&str) -> bool + 'static,
        {
            self.filters.push(Box::new(filter));
            self
        }

        fn build(self) -> ComplexResult<'input> {
            let processed_data = self
                .input
                .split_whitespace()
                .filter(|word| self.filters.iter().all(|f| f(word)))
                .map(|s| s.to_string())
                .collect();

            ComplexResult {
                original: self.input,
                metadata: self.metadata,
                processed_data,
            }
        }
    }

    /// 演示生命周期省略在方法中的应用
    fn demonstrate_lifetime_elision_in_methods() {
        println!("\n--- 方法中的生命周期省略 ---");

        let text = "生命周期省略演示文本";
        let processor = ElidedLifetimeProcessor::new(text);

        // 这些方法都使用了生命周期省略
        let first_char = processor.get_first_char();
        let prefix = processor.get_prefix(4);
        let suffix = processor.get_suffix(4);

        println!("第一个字符: {:?}", first_char);
        println!("前缀: {:?}", prefix);
        println!("后缀: {:?}", suffix);

        // 演示省略规则的应用
        demonstrate_elision_rules(&processor);
    }

    /// 生命周期省略处理器
    struct ElidedLifetimeProcessor<'a> {
        data: &'a str,
    }

    impl<'a> ElidedLifetimeProcessor<'a> {
        // 规则1：输入生命周期自动应用到 &self
        fn new(data: &'a str) -> Self {
            Self { data }
        }

        // 规则2：单个输入生命周期自动应用到输出
        fn get_first_char(&self) -> Option<char> {
            self.data.chars().next()
        }

        // 规则2：&self 的生命周期自动应用到输出
        fn get_prefix(&self, len: usize) -> Option<&str> {
            let char_count = self.data.chars().count();
            if len <= char_count {
                let mut byte_index = 0;
                for (i, (idx, _)) in self.data.char_indices().enumerate() {
                    if i == len {
                        byte_index = idx;
                        break;
                    }
                }
                if len == char_count {
                    Some(self.data)
                } else {
                    Some(&self.data[..byte_index])
                }
            } else {
                None
            }
        }

        // 规则2：&self 的生命周期自动应用到输出
        fn get_suffix(&self, len: usize) -> Option<&str> {
            let char_count = self.data.chars().count();
            if len <= char_count {
                if len == 0 {
                    Some("")
                } else if len == char_count {
                    Some(self.data)
                } else {
                    let skip_chars = char_count - len;
                    let mut byte_index = 0;
                    for (i, (idx, _)) in self.data.char_indices().enumerate() {
                        if i == skip_chars {
                            byte_index = idx;
                            break;
                        }
                    }
                    Some(&self.data[byte_index..])
                }
            } else {
                None
            }
        }

        // 需要显式生命周期：多个输入参数
        fn find_common_prefix<'b>(&'a self, other: &'b str) -> &'a str
        where
            'a: 'b, // 确保 self 的生命周期比 other 长
        {
            let mut byte_index = 0;
            let mut self_chars = self.data.char_indices();
            let mut other_chars = other.chars();

            while let (Some((idx, self_char)), Some(other_char)) =
                (self_chars.next(), other_chars.next())
            {
                if self_char == other_char {
                    byte_index = idx + self_char.len_utf8();
                } else {
                    break;
                }
            }

            &self.data[..byte_index]
        }
    }

    fn demonstrate_elision_rules(processor: &ElidedLifetimeProcessor) {
        println!("\n--- 生命周期省略规则演示 ---");

        // 演示不同的省略场景
        let other_text = "生命周期省略";
        let common = processor.find_common_prefix(other_text);
        println!("公共前缀: '{}'", common);

        // 演示省略规则的限制
        demonstrate_elision_limitations();
    }

    fn demonstrate_elision_limitations() {
        println!("\n--- 省略规则的限制 ---");

        let text1 = "第一个文本";
        let text2 = "第二个文本";

        // 这种情况需要显式生命周期标注
        let result = compare_texts(text1, text2);
        println!("文本比较结果: {}", result);
    }

    // 需要显式生命周期：两个输入参数，返回其中一个
    fn compare_texts<'a>(first: &'a str, second: &'a str) -> &'a str {
        if first.len() > second.len() {
            first
        } else {
            second
        }
    }

    /// 演示复杂方法生命周期场景
    fn demonstrate_complex_method_scenarios() {
        println!("\n--- 复杂方法生命周期场景 ---");

        // 案例1：数据库查询构建器
        demonstrate_query_builder();

        // 案例2：配置验证器
        demonstrate_config_validator();
    }

    fn demonstrate_query_builder() {
        println!("\n--- 数据库查询构建器 ---");

        let table_name = "users";
        let condition = "age > 18";
        let order_by = "name ASC";

        let query = QueryBuilder::new(table_name)
            .where_clause(condition)
            .order_by(order_by)
            .limit(10)
            .build();

        println!("构建的查询: {}", query);
    }

    /// 查询构建器 - 复杂的生命周期管理
    struct QueryBuilder<'query> {
        table: &'query str,
        where_clauses: Vec<&'query str>,
        order_clauses: Vec<&'query str>,
        limit_value: Option<usize>,
    }

    impl<'query> QueryBuilder<'query> {
        fn new(table: &'query str) -> Self {
            Self {
                table,
                where_clauses: Vec::new(),
                order_clauses: Vec::new(),
                limit_value: None,
            }
        }

        fn where_clause(mut self, condition: &'query str) -> Self {
            self.where_clauses.push(condition);
            self
        }

        fn order_by(mut self, order: &'query str) -> Self {
            self.order_clauses.push(order);
            self
        }

        fn limit(mut self, limit: usize) -> Self {
            self.limit_value = Some(limit);
            self
        }

        fn build(self) -> String {
            let mut query = format!("SELECT * FROM {}", self.table);

            if !self.where_clauses.is_empty() {
                query.push_str(" WHERE ");
                query.push_str(&self.where_clauses.join(" AND "));
            }

            if !self.order_clauses.is_empty() {
                query.push_str(" ORDER BY ");
                query.push_str(&self.order_clauses.join(", "));
            }

            if let Some(limit) = self.limit_value {
                query.push_str(&format!(" LIMIT {}", limit));
            }

            query
        }
    }

    fn demonstrate_config_validator() {
        println!("\n--- 配置验证器 ---");

        let config_content = r#"
            server_port=8080
            database_url=localhost:5432
            max_connections=100
            timeout=30
        "#;

        let validator = ConfigValidator::new(config_content);
        let validation_result = validator.validate();

        match validation_result {
            Ok(config) => {
                println!("配置验证成功:");
                config.display();
            }
            Err(errors) => {
                println!("配置验证失败: {:?}", errors);
            }
        }
    }

    /// 配置验证器
    struct ConfigValidator<'config> {
        raw_config: &'config str,
    }

    /// 验证后的配置
    struct ValidatedConfig<'config> {
        raw: &'config str,
        server_port: u16,
        database_url: &'config str,
        max_connections: usize,
        timeout: u32,
    }

    impl<'config> ConfigValidator<'config> {
        fn new(config: &'config str) -> Self {
            Self { raw_config: config }
        }

        fn validate(self) -> Result<ValidatedConfig<'config>, Vec<String>> {
            let mut errors = Vec::new();
            let mut server_port = 8080u16;
            let mut database_url = "";
            let mut max_connections = 100usize;
            let mut timeout = 30u32;

            // 解析配置
            for line in self.raw_config.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                if let Some((key, value)) = line.split_once('=') {
                    match key.trim() {
                        "server_port" => match value.trim().parse::<u16>() {
                            Ok(port) => server_port = port,
                            Err(_) => errors.push("无效的服务器端口".to_string()),
                        },
                        "database_url" => {
                            database_url = value.trim();
                            if database_url.is_empty() {
                                errors.push("数据库URL不能为空".to_string());
                            }
                        }
                        "max_connections" => match value.trim().parse::<usize>() {
                            Ok(conn) => max_connections = conn,
                            Err(_) => errors.push("无效的最大连接数".to_string()),
                        },
                        "timeout" => match value.trim().parse::<u32>() {
                            Ok(t) => timeout = t,
                            Err(_) => errors.push("无效的超时时间".to_string()),
                        },
                        _ => errors.push(format!("未知配置项: {}", key)),
                    }
                }
            }

            if errors.is_empty() {
                Ok(ValidatedConfig {
                    raw: self.raw_config,
                    server_port,
                    database_url,
                    max_connections,
                    timeout,
                })
            } else {
                Err(errors)
            }
        }
    }

    impl<'config> ValidatedConfig<'config> {
        fn display(&self) {
            println!("服务器端口: {}", self.server_port);
            println!("数据库URL: {}", self.database_url);
            println!("最大连接数: {}", self.max_connections);
            println!("超时时间: {}秒", self.timeout);
        }

        fn get_database_url(&self) -> &'config str {
            self.database_url
        }
    }
}

/// # 复杂数据结构的生命周期管理
///
/// 展示在复杂数据结构中如何管理生命周期
pub mod complex_data_structures {
    use std::cell::{Ref, RefCell};
    use std::collections::{BTreeMap, HashMap};
    use std::marker::PhantomData;
    use std::rc::Rc;

    /// 演示复杂数据结构的生命周期管理
    pub fn demonstrate_complex_data_structures() {
        println!("\n=== 复杂数据结构生命周期管理 ===");

        // 1. 树形结构
        demonstrate_tree_structures();

        // 2. 图结构
        demonstrate_graph_structures();

        // 3. 缓存和索引结构
        demonstrate_cache_and_index_structures();

        // 4. 异构数据结构
        demonstrate_heterogeneous_structures();
    }

    /// 演示树形结构
    fn demonstrate_tree_structures() {
        println!("\n--- 树形结构 ---");

        // 创建文件系统树
        let root_name = "root";
        let mut fs_tree = FileSystemTree::new(root_name);

        // 添加文件和目录
        let home_id = fs_tree.add_directory("home", 0);
        let user_id = fs_tree.add_directory("user", home_id);
        fs_tree.add_file("document.txt", user_id);
        fs_tree.add_file("photo.jpg", user_id);

        let etc_id = fs_tree.add_directory("etc", 0);
        fs_tree.add_file("config.conf", etc_id);

        fs_tree.display_tree();

        // 演示树遍历
        demonstrate_tree_traversal(&fs_tree);
    }

    /// 文件系统树节点
    #[derive(Debug, Clone)]
    enum FsNode<'name> {
        Directory {
            name: &'name str,
            children: Vec<usize>, // 子节点的索引
        },
        File {
            name: &'name str,
            size: usize,
        },
    }

    /// 文件系统树
    struct FileSystemTree<'name> {
        nodes: Vec<FsNode<'name>>,
        root_id: usize,
    }

    impl<'name> FileSystemTree<'name> {
        fn new(root_name: &'name str) -> Self {
            let root = FsNode::Directory {
                name: root_name,
                children: Vec::new(),
            };

            Self {
                nodes: vec![root],
                root_id: 0,
            }
        }

        fn add_directory(&mut self, name: &'name str, parent_id: usize) -> usize {
            let new_id = self.nodes.len();
            let new_dir = FsNode::Directory {
                name,
                children: Vec::new(),
            };

            self.nodes.push(new_dir);

            // 将新目录添加到父目录的子节点列表中
            if let Some(FsNode::Directory { children, .. }) = self.nodes.get_mut(parent_id) {
                children.push(new_id);
            }

            new_id
        }

        fn add_file(&mut self, name: &'name str, parent_id: usize) -> usize {
            let new_id = self.nodes.len();
            let new_file = FsNode::File {
                name,
                size: name.len() * 10, // 模拟文件大小
            };

            self.nodes.push(new_file);

            // 将新文件添加到父目录的子节点列表中
            if let Some(FsNode::Directory { children, .. }) = self.nodes.get_mut(parent_id) {
                children.push(new_id);
            }

            new_id
        }

        fn display_tree(&self) {
            println!("文件系统树:");
            self.display_node(self.root_id, 0);
        }

        fn display_node(&self, node_id: usize, depth: usize) {
            let indent = "  ".repeat(depth);

            match &self.nodes[node_id] {
                FsNode::Directory { name, children } => {
                    println!("{}📁 {}/", indent, name);
                    for &child_id in children {
                        self.display_node(child_id, depth + 1);
                    }
                }
                FsNode::File { name, size } => {
                    println!("{}📄 {} ({}B)", indent, name, size);
                }
            }
        }

        fn get_node(&self, id: usize) -> Option<&FsNode<'name>> {
            self.nodes.get(id)
        }
    }

    fn demonstrate_tree_traversal(tree: &FileSystemTree) {
        println!("\n--- 树遍历演示 ---");

        // 深度优先遍历
        let mut visited = Vec::new();
        tree.dfs_traverse(tree.root_id, &mut visited);

        println!("深度优先遍历结果:");
        for (depth, node_id) in visited {
            if let Some(node) = tree.get_node(node_id) {
                let indent = "  ".repeat(depth);
                match node {
                    FsNode::Directory { name, .. } => println!("{}目录: {}", indent, name),
                    FsNode::File { name, .. } => println!("{}文件: {}", indent, name),
                }
            }
        }
    }

    impl<'name> FileSystemTree<'name> {
        fn dfs_traverse(&self, node_id: usize, visited: &mut Vec<(usize, usize)>) {
            self.dfs_traverse_helper(node_id, 0, visited);
        }

        fn dfs_traverse_helper(
            &self,
            node_id: usize,
            depth: usize,
            visited: &mut Vec<(usize, usize)>,
        ) {
            visited.push((depth, node_id));

            if let Some(FsNode::Directory { children, .. }) = self.get_node(node_id) {
                for &child_id in children {
                    self.dfs_traverse_helper(child_id, depth + 1, visited);
                }
            }
        }
    }

    /// 演示图结构
    fn demonstrate_graph_structures() {
        println!("\n--- 图结构 ---");

        // 创建社交网络图
        let mut social_graph = SocialGraph::new();

        // 添加用户
        let alice_id = social_graph.add_user("Alice");
        let bob_id = social_graph.add_user("Bob");
        let charlie_id = social_graph.add_user("Charlie");
        let diana_id = social_graph.add_user("Diana");

        // 添加关系
        social_graph.add_friendship(alice_id, bob_id);
        social_graph.add_friendship(bob_id, charlie_id);
        social_graph.add_friendship(charlie_id, diana_id);
        social_graph.add_friendship(alice_id, diana_id);

        social_graph.display_graph();

        // 查找朋友
        if let Some(friends) = social_graph.get_friends(alice_id) {
            println!("Alice 的朋友: {:?}", friends);
        }
    }

    /// 社交网络图
    struct SocialGraph<'names> {
        users: Vec<User<'names>>,
        adjacency_list: HashMap<usize, Vec<usize>>,
    }

    #[derive(Debug, Clone)]
    struct User<'name> {
        id: usize,
        name: &'name str,
    }

    impl<'names> SocialGraph<'names> {
        fn new() -> Self {
            Self {
                users: Vec::new(),
                adjacency_list: HashMap::new(),
            }
        }

        fn add_user(&mut self, name: &'names str) -> usize {
            let user_id = self.users.len();
            let user = User { id: user_id, name };

            self.users.push(user);
            self.adjacency_list.insert(user_id, Vec::new());

            user_id
        }

        fn add_friendship(&mut self, user1_id: usize, user2_id: usize) {
            // 双向关系
            if let Some(friends) = self.adjacency_list.get_mut(&user1_id) {
                friends.push(user2_id);
            }
            if let Some(friends) = self.adjacency_list.get_mut(&user2_id) {
                friends.push(user1_id);
            }
        }

        fn get_friends(&self, user_id: usize) -> Option<Vec<&'names str>> {
            self.adjacency_list.get(&user_id).map(|friend_ids| {
                friend_ids
                    .iter()
                    .filter_map(|&id| self.users.get(id).map(|user| user.name))
                    .collect()
            })
        }

        fn display_graph(&self) {
            println!("社交网络图:");
            for user in &self.users {
                if let Some(friends) = self.get_friends(user.id) {
                    println!("{} -> {:?}", user.name, friends);
                }
            }
        }
    }

    /// 演示缓存和索引结构
    fn demonstrate_cache_and_index_structures() {
        println!("\n--- 缓存和索引结构 ---");

        // 创建带索引的文档存储
        let mut doc_store = DocumentStore::new();

        // 添加文档
        let doc1 = "Rust 是一种系统编程语言";
        let doc2 = "生命周期是 Rust 的核心特性";
        let doc3 = "内存安全是 Rust 的重要目标";

        doc_store.add_document("doc1", doc1);
        doc_store.add_document("doc2", doc2);
        doc_store.add_document("doc3", doc3);

        // 搜索文档
        let search_results = doc_store.search("Rust");
        println!("搜索 'Rust' 的结果: {:?}", search_results);

        // 获取文档
        if let Some(content) = doc_store.get_document("doc2") {
            println!("文档 doc2 内容: {}", content);
        }
    }

    /// 文档存储系统
    pub struct DocumentStore<'docs> {
        documents: HashMap<&'docs str, &'docs str>,   // ID -> 内容
        word_index: HashMap<String, Vec<&'docs str>>, // 单词 -> 文档ID列表
    }

    impl<'docs> DocumentStore<'docs> {
        pub fn new() -> Self {
            Self {
                documents: HashMap::new(),
                word_index: HashMap::new(),
            }
        }

        pub fn add_document(&mut self, id: &'docs str, content: &'docs str) {
            // 存储文档
            self.documents.insert(id, content);

            // 建立索引
            for word in content.split_whitespace() {
                let word_key = word.to_lowercase();
                self.word_index
                    .entry(word_key)
                    .or_insert_with(Vec::new)
                    .push(id);
            }
        }

        pub fn get_document(&self, id: &str) -> Option<&'docs str> {
            self.documents.get(id).copied()
        }

        fn search(&self, query: &str) -> Vec<&'docs str> {
            let query_lower = query.to_lowercase();
            self.word_index
                .get(&query_lower)
                .map(|doc_ids| doc_ids.clone())
                .unwrap_or_default()
        }

        fn get_all_documents(&self) -> Vec<(&'docs str, &'docs str)> {
            self.documents
                .iter()
                .map(|(&id, &content)| (id, content))
                .collect()
        }
    }

    /// 演示异构数据结构
    fn demonstrate_heterogeneous_structures() {
        println!("\n--- 异构数据结构 ---");

        // 创建多类型容器
        let mut container = HeterogeneousContainer::new();

        // 添加不同类型的数据
        let text_data = "文本数据";
        let number_data = 42;
        let bool_data = true;

        container.add_text("text1", text_data);
        container.add_number("num1", number_data);
        container.add_boolean("bool1", bool_data);

        container.display_all();

        // 查询数据
        if let Some(text) = container.get_text("text1") {
            println!("获取文本数据: {}", text);
        }
    }

    /// 异构数据容器
    struct HeterogeneousContainer<'data> {
        text_data: HashMap<&'data str, &'data str>,
        number_data: HashMap<&'data str, i32>,
        boolean_data: HashMap<&'data str, bool>,
    }

    impl<'data> HeterogeneousContainer<'data> {
        fn new() -> Self {
            Self {
                text_data: HashMap::new(),
                number_data: HashMap::new(),
                boolean_data: HashMap::new(),
            }
        }

        fn add_text(&mut self, key: &'data str, value: &'data str) {
            self.text_data.insert(key, value);
        }

        fn add_number(&mut self, key: &'data str, value: i32) {
            self.number_data.insert(key, value);
        }

        fn add_boolean(&mut self, key: &'data str, value: bool) {
            self.boolean_data.insert(key, value);
        }

        fn get_text(&self, key: &str) -> Option<&'data str> {
            self.text_data.get(key).copied()
        }

        fn get_number(&self, key: &str) -> Option<i32> {
            self.number_data.get(key).copied()
        }

        fn get_boolean(&self, key: &str) -> Option<bool> {
            self.boolean_data.get(key).copied()
        }

        fn display_all(&self) {
            println!("异构容器内容:");

            println!("文本数据:");
            for (key, value) in &self.text_data {
                println!("  {}: {}", key, value);
            }

            println!("数字数据:");
            for (key, value) in &self.number_data {
                println!("  {}: {}", key, value);
            }

            println!("布尔数据:");
            for (key, value) in &self.boolean_data {
                println!("  {}: {}", key, value);
            }
        }
    }
}

/// # 实际项目中的生命周期模式
///
/// 展示在实际项目中常见的生命周期使用模式
pub mod real_world_patterns {
    use std::collections::HashMap;
    use std::fmt::{Debug, Display};

    /// 演示实际项目中的生命周期模式
    pub fn demonstrate_real_world_patterns() {
        println!("\n=== 实际项目生命周期模式 ===");

        // 1. Web 服务器模式
        demonstrate_web_server_patterns();

        // 2. 数据库连接池模式
        demonstrate_connection_pool_patterns();

        // 3. 配置管理模式
        demonstrate_config_management_patterns();

        // 4. 事件处理模式
        demonstrate_event_handling_patterns();
    }

    /// 演示 Web 服务器模式
    fn demonstrate_web_server_patterns() {
        println!("\n--- Web 服务器模式 ---");

        // 模拟 HTTP 请求处理
        let request_data = "GET /api/users HTTP/1.1\nHost: example.com\nUser-Agent: Mozilla/5.0";
        let mut server = WebServer::new();

        // 注册路由处理器
        server.register_route("/api/users", Box::new(users_handler));
        server.register_route("/api/posts", Box::new(posts_handler));

        // 处理请求
        let response = server.handle_request(request_data);
        println!("服务器响应: {}", response);
    }

    /// 用户处理器函数
    fn users_handler(request: &HttpRequest) -> String {
        format!("处理用户请求: {}", request.path)
    }

    /// 文章处理器函数
    fn posts_handler(request: &HttpRequest) -> String {
        format!("处理文章请求: {}", request.path)
    }

    /// HTTP 请求结构
    #[derive(Debug)]
    struct HttpRequest<'req> {
        method: &'req str,
        path: &'req str,
        headers: HashMap<&'req str, &'req str>,
        body: Option<&'req str>,
    }

    /// Web 服务器
    struct WebServer<'server> {
        routes: HashMap<&'server str, Box<dyn Fn(&HttpRequest) -> String + 'server>>,
    }

    impl<'server> WebServer<'server> {
        fn new() -> Self {
            Self {
                routes: HashMap::new(),
            }
        }

        fn register_route<F>(&mut self, path: &'server str, handler: F)
        where
            F: Fn(&HttpRequest) -> String + 'server,
        {
            self.routes.insert(path, Box::new(handler));
        }

        fn handle_request(&self, request_data: &'server str) -> String {
            let request = self.parse_request(request_data);

            if let Some(handler) = self.routes.get(request.path) {
                handler(&request)
            } else {
                "404 Not Found".to_string()
            }
        }

        fn parse_request(&self, data: &'server str) -> HttpRequest<'server> {
            let lines: Vec<&str> = data.lines().collect();
            let mut headers = HashMap::new();

            // 解析请求行
            let method = if let Some(first_line) = lines.first() {
                first_line.split_whitespace().next().unwrap_or("GET")
            } else {
                "GET"
            };

            let path = if let Some(first_line) = lines.first() {
                first_line.split_whitespace().nth(1).unwrap_or("/")
            } else {
                "/"
            };

            // 解析头部
            for line in lines.iter().skip(1) {
                if let Some((key, value)) = line.split_once(": ") {
                    headers.insert(key, value);
                }
            }

            HttpRequest {
                method,
                path,
                headers,
                body: None,
            }
        }
    }

    /// 演示数据库连接池模式
    fn demonstrate_connection_pool_patterns() {
        println!("\n--- 数据库连接池模式 ---");

        let connection_string = "postgresql://localhost:5432/mydb";
        let mut pool = ConnectionPool::new(connection_string, 5);

        // 获取连接并执行查询
        if let Some(mut conn) = pool.get_connection() {
            let result = conn.execute_query("SELECT * FROM users WHERE id = 1");
            println!("查询结果: {}", result);

            // 连接会自动归还到池中
        }

        pool.display_stats();
    }

    /// 数据库连接
    struct DatabaseConnection<'pool> {
        connection_string: &'pool str,
        id: usize,
        in_use: bool,
    }

    /// 连接池
    struct ConnectionPool<'pool> {
        connection_string: &'pool str,
        connections: Vec<DatabaseConnection<'pool>>,
        max_size: usize,
    }

    impl<'pool> ConnectionPool<'pool> {
        fn new(connection_string: &'pool str, max_size: usize) -> Self {
            let mut connections = Vec::new();

            // 预创建连接
            for i in 0..max_size {
                connections.push(DatabaseConnection {
                    connection_string,
                    id: i,
                    in_use: false,
                });
            }

            Self {
                connection_string,
                connections,
                max_size,
            }
        }

        fn get_connection(&mut self) -> Option<&mut DatabaseConnection<'pool>> {
            self.connections
                .iter_mut()
                .find(|conn| !conn.in_use)
                .map(|conn| {
                    conn.in_use = true;
                    conn
                })
        }

        fn return_connection(&mut self, connection_id: usize) {
            if let Some(conn) = self.connections.get_mut(connection_id) {
                conn.in_use = false;
            }
        }

        fn display_stats(&self) {
            let in_use_count = self.connections.iter().filter(|c| c.in_use).count();
            println!(
                "连接池统计: {}/{} 连接正在使用",
                in_use_count, self.max_size
            );
        }
    }

    impl<'pool> DatabaseConnection<'pool> {
        fn execute_query(&self, query: &str) -> String {
            format!("连接 {} 执行查询: {}", self.id, query)
        }
    }

    /// 演示配置管理模式
    fn demonstrate_config_management_patterns() {
        println!("\n--- 配置管理模式 ---");

        let config_content = r#"
            [database]
            host = "localhost"
            port = 5432
            name = "myapp"
            
            [server]
            host = "0.0.0.0"
            port = 8080
            workers = 4
        "#;

        let config_manager = ConfigManager::new(config_content);

        // 获取配置值
        if let Some(db_host) = config_manager.get_database_config().get_host() {
            println!("数据库主机: {}", db_host);
        }

        if let Some(server_port) = config_manager.get_server_config().get_port() {
            println!("服务器端口: {}", server_port);
        }

        config_manager.display_all_configs();
    }

    /// 配置管理器
    struct ConfigManager<'config> {
        raw_config: &'config str,
        database_config: DatabaseConfig<'config>,
        server_config: ServerConfig<'config>,
    }

    /// 数据库配置
    struct DatabaseConfig<'config> {
        host: Option<&'config str>,
        port: Option<u16>,
        name: Option<&'config str>,
    }

    /// 服务器配置
    struct ServerConfig<'config> {
        host: Option<&'config str>,
        port: Option<u16>,
        workers: Option<usize>,
    }

    impl<'config> ConfigManager<'config> {
        fn new(config_content: &'config str) -> Self {
            let (db_config, server_config) = Self::parse_config(config_content);

            Self {
                raw_config: config_content,
                database_config: db_config,
                server_config: server_config,
            }
        }

        fn parse_config(content: &'config str) -> (DatabaseConfig<'config>, ServerConfig<'config>) {
            let mut db_config = DatabaseConfig {
                host: None,
                port: None,
                name: None,
            };

            let mut server_config = ServerConfig {
                host: None,
                port: None,
                workers: None,
            };

            let mut current_section = "";

            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                if line.starts_with('[') && line.ends_with(']') {
                    current_section = &line[1..line.len() - 1];
                    continue;
                }

                if let Some((key, value)) = line.split_once('=') {
                    let key = key.trim();
                    let value = value.trim().trim_matches('"');

                    match current_section {
                        "database" => match key {
                            "host" => db_config.host = Some(value),
                            "port" => db_config.port = value.parse().ok(),
                            "name" => db_config.name = Some(value),
                            _ => {}
                        },
                        "server" => match key {
                            "host" => server_config.host = Some(value),
                            "port" => server_config.port = value.parse().ok(),
                            "workers" => server_config.workers = value.parse().ok(),
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }

            (db_config, server_config)
        }

        fn get_database_config(&self) -> &DatabaseConfig<'config> {
            &self.database_config
        }

        fn get_server_config(&self) -> &ServerConfig<'config> {
            &self.server_config
        }

        fn display_all_configs(&self) {
            println!("所有配置:");
            println!("数据库配置:");
            if let Some(host) = self.database_config.host {
                println!("  主机: {}", host);
            }
            if let Some(port) = self.database_config.port {
                println!("  端口: {}", port);
            }
            if let Some(name) = self.database_config.name {
                println!("  数据库名: {}", name);
            }

            println!("服务器配置:");
            if let Some(host) = self.server_config.host {
                println!("  主机: {}", host);
            }
            if let Some(port) = self.server_config.port {
                println!("  端口: {}", port);
            }
            if let Some(workers) = self.server_config.workers {
                println!("  工作线程数: {}", workers);
            }
        }
    }

    impl<'config> DatabaseConfig<'config> {
        fn get_host(&self) -> Option<&'config str> {
            self.host
        }

        fn get_port(&self) -> Option<u16> {
            self.port
        }

        fn get_name(&self) -> Option<&'config str> {
            self.name
        }
    }

    impl<'config> ServerConfig<'config> {
        fn get_host(&self) -> Option<&'config str> {
            self.host
        }

        fn get_port(&self) -> Option<u16> {
            self.port
        }

        fn get_workers(&self) -> Option<usize> {
            self.workers
        }
    }

    /// 演示事件处理模式
    fn demonstrate_event_handling_patterns() {
        println!("\n--- 事件处理模式 ---");

        let mut event_system = EventSystem::new();

        // 注册事件处理器
        event_system.register_handler("user_login", Box::new(handle_user_login));
        event_system.register_handler("user_logout", Box::new(handle_user_logout));
        event_system.register_handler("data_update", Box::new(handle_data_update));

        // 触发事件
        let login_event = "user_login:张三:2024-01-15T10:30:00";
        let logout_event = "user_logout:张三:2024-01-15T11:30:00";
        let update_event = "data_update:用户数据:2024-01-15T10:35:00";

        event_system.handle_event("user_login", login_event);
        event_system.handle_event("user_logout", logout_event);
        event_system.handle_event("data_update", update_event);

        event_system.display_stats();
    }

    /// 事件处理器类型
    type EventHandler<'event> = Box<dyn Fn(&'event str) + 'event>;

    /// 事件系统
    struct EventSystem<'event> {
        handlers: HashMap<&'event str, EventHandler<'event>>,
        event_count: HashMap<String, usize>,
    }

    impl<'event> EventSystem<'event> {
        fn new() -> Self {
            Self {
                handlers: HashMap::new(),
                event_count: HashMap::new(),
            }
        }

        fn register_handler(&mut self, event_type: &'event str, handler: EventHandler<'event>) {
            self.handlers.insert(event_type, handler);
        }

        fn handle_event(&mut self, event_type: &str, event_data: &'event str) {
            if let Some(handler) = self.handlers.get(event_type) {
                handler(event_data);

                // 更新统计
                *self.event_count.entry(event_type.to_string()).or_insert(0) += 1;
            } else {
                println!("未找到事件类型 '{}' 的处理器", event_type);
            }
        }

        fn display_stats(&self) {
            println!("事件处理统计:");
            for (event_type, count) in &self.event_count {
                println!("  {}: {} 次", event_type, count);
            }
        }
    }

    /// 用户登录事件处理器
    fn handle_user_login(event_data: &str) {
        let parts: Vec<&str> = event_data.split(':').collect();
        if parts.len() >= 3 {
            println!("用户登录: {} 在 {}", parts[1], parts[2]);
        }
    }

    /// 用户登出事件处理器
    fn handle_user_logout(event_data: &str) {
        let parts: Vec<&str> = event_data.split(':').collect();
        if parts.len() >= 3 {
            println!("用户登出: {} 在 {}", parts[1], parts[2]);
        }
    }

    /// 数据更新事件处理器
    fn handle_data_update(event_data: &str) {
        let parts: Vec<&str> = event_data.split(':').collect();
        if parts.len() >= 3 {
            println!("数据更新: {} 在 {}", parts[1], parts[2]);
        }
    }
}

/// 运行所有实际应用案例的演示
pub fn run_all_demonstrations() {
    run_all_practical_examples();
}

/// 运行所有实际应用案例的演示
pub fn run_all_practical_examples() {
    println!("🦀 Rust 深入生命周期 - 实际应用案例篇 🦀");
    println!("=============================================");

    // 1. 结构体生命周期设计
    struct_lifetime_design::demonstrate_struct_lifetime_design();

    // 2. 方法中的生命周期处理
    method_lifetime_handling::demonstrate_method_lifetime_handling();

    // 3. 复杂数据结构的生命周期管理
    complex_data_structures::demonstrate_complex_data_structures();

    // 4. 实际项目中的生命周期模式
    real_world_patterns::demonstrate_real_world_patterns();

    println!("\n=============================================");
    println!("✅ 实际应用案例演示完成！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_manager() {
        let config = "host=localhost\nport=8080";
        let manager = struct_lifetime_design::ConfigManager::new(config);
        assert_eq!(manager.get_raw_config(), config);
    }

    #[test]
    fn test_text_analyzer() {
        let text = "hello world rust";
        let analyzer = method_lifetime_handling::TextAnalyzer::new(text);
        assert_eq!(analyzer.count_words(), 3);
        assert_eq!(analyzer.get_first_word(), Some("hello"));
    }

    #[test]
    fn test_document_store() {
        let mut store = complex_data_structures::DocumentStore::new();
        let content = "test document content";
        store.add_document("doc1", content);
        assert_eq!(store.get_document("doc1"), Some(content));
    }
}
