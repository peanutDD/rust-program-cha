//! Option 解构 - 全面深入分析
//!
//! 本文档基于 https://course.rs/basic/match-pattern/option.html 的内容
//! 对 Rust 中的 Option 类型进行全面深入的分析，包含所有相关知识点和实际案例

/// # Option 类型的核心概念
///
/// Option<T> 是 Rust 标准库中最重要的枚举类型之一，用于表示一个值可能存在或不存在的情况。
/// 它是 Rust 处理空值的安全方式，避免了其他语言中常见的空指针异常。
///
/// ## Option 的定义
/// ```rust
/// enum Option<T> {
///     Some(T),
///     None,
/// }
/// ```

fn main() {
    println!("=== Option 解构 - 全面深入分析 ===");

    // 1. 基础概念演示
    basic_option_concepts();

    // 2. 模式匹配解构
    pattern_matching_examples();

    // 3. if let 语法糖
    if_let_examples();

    // 4. while let 循环
    while_let_examples();

    // 5. 函数参数中的解构
    function_parameter_destructuring();

    // 6. 嵌套 Option 的处理
    nested_option_handling();

    // 7. Option 的方法链式调用
    option_method_chaining();

    // 8. 错误处理与 Option 结合
    error_handling_with_option();

    // 9. 自定义类型中的 Option
    custom_types_with_option();

    // 10. 性能优化技巧
    performance_optimization();

    // 11. 实际应用场景
    real_world_scenarios();
}

/// ## 1. 基础概念演示
/// 展示 Option 的基本用法和创建方式
fn basic_option_concepts() {
    println!("\n=== 1. 基础概念演示 ===");

    // 创建 Some 值
    let some_number: Option<i32> = Some(42);
    let some_string: Option<String> = Some(String::from("Hello"));

    // 创建 None 值
    let none_number: Option<i32> = None;
    let none_string: Option<String> = None;

    println!("Some(42): {:?}", some_number);
    println!("Some(\"Hello\"): {:?}", some_string);
    println!("None (i32): {:?}", none_number);
    println!("None (String): {:?}", none_string);

    // 类型推断
    let inferred_some = Some(3.14); // 编译器推断为 Option<f64>
    let inferred_none: Option<f64> = None; // 需要显式类型注解

    println!("推断类型 Some(3.14): {:?}", inferred_some);
    println!("推断类型 None: {:?}", inferred_none);
}

/// ## 2. 模式匹配解构
/// 使用 match 表达式对 Option 进行完整的模式匹配
fn pattern_matching_examples() {
    println!("\n=== 2. 模式匹配解构 ===");

    // 基础 match 模式
    let x: Option<i32> = Some(5);
    let result = match x {
        Some(value) => {
            println!("找到值: {}", value);
            value * 2
        }
        None => {
            println!("没有值");
            0
        }
    };
    println!("match 结果: {}", result);

    // 使用守卫条件
    let numbers = vec![Some(1), Some(5), Some(10), None, Some(15)];
    for opt_num in numbers {
        match opt_num {
            Some(n) if n < 5 => println!("小数字: {}", n),
            Some(n) if n >= 10 => println!("大数字: {}", n),
            Some(n) => println!("中等数字: {}", n),
            None => println!("空值"),
        }
    }

    // 解构复杂类型
    let complex_option: Option<(i32, String)> = Some((42, String::from("answer")));
    match complex_option {
        Some((num, text)) => {
            println!("解构元组: 数字={}, 文本={}", num, text);
        }
        None => println!("空的复杂类型"),
    }

    // 引用模式匹配
    let opt_ref = &Some(String::from("borrowed"));
    match opt_ref {
        Some(s) => println!("借用的字符串: {}", s),
        None => println!("空的借用"),
    }
}

/// ## 3. if let 语法糖
/// 当只关心某一种模式时，使用 if let 简化代码
fn if_let_examples() {
    println!("\n=== 3. if let 语法糖 ===");

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // 基础 if let
    if let Some(color) = favorite_color {
        println!("使用你最喜欢的颜色 {} 作为背景", color);
    } else {
        println!("没有最喜欢的颜色，使用默认颜色");
    }

    // 复合条件
    if let Some(color) = favorite_color {
        println!("使用颜色: {}", color);
    } else if is_tuesday {
        println!("星期二是绿色的日子！");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("使用紫色作为背景色");
        } else {
            println!("使用橙色作为背景色");
        }
    } else {
        println!("使用蓝色作为背景色");
    }

    // if let 与解构
    let data: Option<(i32, i32)> = Some((3, 4));
    if let Some((x, y)) = data {
        println!(
            "坐标: ({}, {}), 距离原点: {:.2}",
            x,
            y,
            ((x * x + y * y) as f64).sqrt()
        );
    }

    // 多重 if let
    let first: Option<i32> = Some(1);
    let second: Option<i32> = Some(2);

    if let (Some(a), Some(b)) = (first, second) {
        println!("两个值都存在: {} + {} = {}", a, b, a + b);
    }
}

/// ## 4. while let 循环
/// 使用 while let 处理连续的 Option 值
fn while_let_examples() {
    println!("\n=== 4. while let 循环 ===");

    // 模拟一个返回 Option 的迭代器
    let mut stack = vec![1, 2, 3, 4, 5];

    println!("使用 while let 弹出栈中的元素:");
    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }

    // 处理字符串解析
    let mut input = "1,2,3,4,invalid,5".split(',');
    let mut numbers = Vec::new();

    println!("\n解析数字直到遇到无效输入:");
    while let Some(token) = input.next() {
        match token.parse::<i32>() {
            Ok(num) => {
                numbers.push(num);
                println!("解析成功: {}", num);
            }
            Err(_) => {
                println!("遇到无效输入: {}, 停止解析", token);
                break;
            }
        }
    }
    println!("解析的数字: {:?}", numbers);
}

/// ## 5. 函数参数中的解构
/// 在函数参数中直接解构 Option
fn function_parameter_destructuring() {
    println!("\n=== 5. 函数参数中的解构 ===");

    // 接受 Option 参数的函数
    fn process_optional_number(opt_num: Option<i32>) -> String {
        match opt_num {
            Some(n) if n > 0 => format!("正数: {}", n),
            Some(n) if n < 0 => format!("负数: {}", n),
            Some(n) => format!("零: {}", n), // 处理 n == 0 的情况
            None => "无值".to_string(),
        }
    }

    // 使用闭包处理 Option
    let numbers = vec![Some(5), None, Some(-3), Some(0)];
    let results: Vec<String> = numbers.into_iter().map(process_optional_number).collect();

    println!("处理结果: {:?}", results);

    // 高阶函数中的 Option 处理
    fn apply_if_some<T, F>(opt: Option<T>, f: F) -> Option<T>
    where
        F: FnOnce(T) -> T,
    {
        opt.map(f)
    }

    let result = apply_if_some(Some(10), |x| x * x);
    println!("应用函数后: {:?}", result);

    let result = apply_if_some(None::<i32>, |x| x * x);
    println!("对 None 应用函数: {:?}", result);
}

/// ## 6. 嵌套 Option 的处理
/// 处理 Option<Option<T>> 等嵌套情况
fn nested_option_handling() {
    println!("\n=== 6. 嵌套 Option 的处理 ===");

    // 创建嵌套 Option
    let nested: Option<Option<i32>> = Some(Some(42));
    let nested_none_inner: Option<Option<i32>> = Some(None);
    let nested_none_outer: Option<Option<i32>> = None;

    // 处理嵌套 Option
    fn process_nested(nested: Option<Option<i32>>) -> String {
        match nested {
            Some(Some(value)) => format!("双重包装的值: {}", value),
            Some(None) => "外层有值，内层为空".to_string(),
            None => "外层为空".to_string(),
        }
    }

    println!("{}", process_nested(nested));
    println!("{}", process_nested(nested_none_inner));
    println!("{}", process_nested(nested_none_outer));

    // 使用 flatten 方法
    let flattened = Some(Some(42)).flatten();
    println!("flatten 后: {:?}", flattened);

    let flattened = Some(None::<i32>).flatten();
    println!("flatten None: {:?}", flattened);

    // 复杂嵌套结构
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Option<u32>,
    }

    let maybe_person: Option<Person> = Some(Person {
        name: "Alice".to_string(),
        age: Some(30),
    });

    match maybe_person {
        Some(Person {
            name,
            age: Some(age_val),
        }) => {
            println!("找到人员: {}, 年龄: {}", name, age_val);
        }
        Some(Person { name, age: None }) => {
            println!("找到人员: {}, 年龄未知", name);
        }
        None => println!("没有找到人员"),
    }
}

/// ## 7. Option 的方法链式调用
/// 展示 Option 提供的各种方法及其链式调用
fn option_method_chaining() {
    println!("\n=== 7. Option 的方法链式调用 ===");

    let numbers = vec![Some(1), Some(2), None, Some(4), Some(5)];

    // map 方法
    let doubled: Vec<Option<i32>> = numbers.iter().map(|opt| opt.map(|x| x * 2)).collect();
    println!("map 加倍: {:?}", doubled);

    // filter_map 方法
    let filtered: Vec<i32> = numbers.iter().filter_map(|&opt| opt).collect();
    println!("filter_map 过滤: {:?}", filtered);

    // and_then 方法（flatMap）
    let result: Option<i32> = Some(2)
        .and_then(|x| if x > 0 { Some(x * x) } else { None })
        .and_then(|x| if x < 10 { Some(x + 1) } else { None });
    println!("and_then 链式: {:?}", result);

    // or_else 方法
    let backup = None::<i32>.or_else(|| Some(42)).or_else(|| Some(100));
    println!("or_else 备选: {:?}", backup);

    // unwrap_or 和 unwrap_or_else
    let value1 = Some(10).unwrap_or(0);
    let value2 = None.unwrap_or(0);
    println!("unwrap_or: {} 和 {}", value1, value2);

    let expensive_default = || {
        println!("计算昂贵的默认值...");
        999
    };
    let value3 = Some(10).unwrap_or_else(expensive_default);
    let value4 = None.unwrap_or_else(expensive_default);
    println!("unwrap_or_else: {} 和 {}", value3, value4);

    // take 方法
    let mut opt = Some(42);
    let taken = opt.take();
    println!("take 后: opt={:?}, taken={:?}", opt, taken);

    // replace 方法
    let mut opt = Some(42);
    let old = opt.replace(100);
    println!("replace 后: opt={:?}, old={:?}", opt, old);
}

/// ## 8. 错误处理与 Option 结合
/// Option 与 Result 的转换和组合使用
fn error_handling_with_option() {
    println!("\n=== 8. 错误处理与 Option 结合 ===");

    // Option 转 Result
    let opt: Option<i32> = Some(42);
    let result: Result<i32, &str> = opt.ok_or("没有值");
    println!("Option 转 Result: {:?}", result);

    let opt: Option<i32> = None;
    let result: Result<i32, String> = opt.ok_or_else(|| "动态错误消息".to_string());
    println!("None 转 Result: {:?}", result);

    // Result 转 Option
    let result: Result<i32, &str> = Ok(42);
    let opt = result.ok();
    println!("Result 转 Option: {:?}", opt);

    let result: Result<i32, &str> = Err("错误");
    let opt = result.ok();
    println!("错误 Result 转 Option: {:?}", opt);

    // 组合使用
    fn safe_divide(a: f64, b: f64) -> Option<f64> {
        if b != 0.0 { Some(a / b) } else { None }
    }

    fn parse_and_divide(input: &str) -> Option<f64> {
        let parts: Vec<&str> = input.split('/').collect();
        if parts.len() != 2 {
            return None;
        }

        let a = parts[0].parse::<f64>().ok()?;
        let b = parts[1].parse::<f64>().ok()?;
        safe_divide(a, b)
    }

    let test_cases = vec!["10/2", "15/3", "8/0", "invalid", "10/abc"];
    for case in test_cases {
        match parse_and_divide(case) {
            Some(result) => println!("{} = {}", case, result),
            None => println!("{} 无法计算", case),
        }
    }
}

/// ## 9. 自定义类型中的 Option
/// 在自定义结构体和枚举中使用 Option
fn custom_types_with_option() {
    println!("\n=== 9. 自定义类型中的 Option ===");

    #[derive(Debug, Clone)]
    struct User {
        id: u32,
        name: String,
        email: Option<String>,
        age: Option<u32>,
        profile_picture: Option<String>,
    }

    impl User {
        fn new(id: u32, name: String) -> Self {
            User {
                id,
                name,
                email: None,
                age: None,
                profile_picture: None,
            }
        }

        fn with_email(mut self, email: String) -> Self {
            self.email = Some(email);
            self
        }

        fn with_age(mut self, age: u32) -> Self {
            self.age = Some(age);
            self
        }

        fn display_info(&self) {
            println!("用户 ID: {}, 姓名: {}", self.id, self.name);

            match &self.email {
                Some(email) => println!("  邮箱: {}", email),
                None => println!("  邮箱: 未提供"),
            }

            if let Some(age) = self.age {
                println!("  年龄: {}", age);
            } else {
                println!("  年龄: 未知");
            }

            match &self.profile_picture {
                Some(pic) => println!("  头像: {}", pic),
                None => println!("  头像: 使用默认头像"),
            }
        }
    }

    // 创建用户实例
    let user1 = User::new(1, "Alice".to_string())
        .with_email("alice@example.com".to_string())
        .with_age(25);

    let user2 = User::new(2, "Bob".to_string());

    user1.display_info();
    println!();
    user2.display_info();

    // 枚举中的 Option
    #[derive(Debug)]
    enum ConfigValue {
        Text(String),
        Number(i32),
        Boolean(bool),
        Optional(Option<String>),
    }

    let configs = vec![
        ConfigValue::Text("localhost".to_string()),
        ConfigValue::Number(8080),
        ConfigValue::Boolean(true),
        ConfigValue::Optional(Some("debug".to_string())),
        ConfigValue::Optional(None),
    ];

    println!("\n配置值:");
    for (i, config) in configs.iter().enumerate() {
        match config {
            ConfigValue::Text(s) => println!("  配置 {}: 文本 = {}", i, s),
            ConfigValue::Number(n) => println!("  配置 {}: 数字 = {}", i, n),
            ConfigValue::Boolean(b) => println!("  配置 {}: 布尔 = {}", i, b),
            ConfigValue::Optional(Some(s)) => println!("  配置 {}: 可选值 = {}", i, s),
            ConfigValue::Optional(None) => println!("  配置 {}: 可选值 = 未设置", i),
        }
    }
}

/// ## 10. 性能优化技巧
/// Option 使用中的性能考虑和优化技巧
fn performance_optimization() {
    println!("\n=== 10. 性能优化技巧 ===");

    // 避免不必要的克隆
    let expensive_data = vec![1, 2, 3, 4, 5];
    let opt_data: Option<&Vec<i32>> = Some(&expensive_data);

    // 好的做法：使用引用
    match opt_data {
        Some(data) => println!("数据长度: {}", data.len()),
        None => println!("没有数据"),
    }

    // 使用 as_ref 避免移动
    let opt_string: Option<String> = Some("Hello".to_string());
    let length = opt_string.as_ref().map(|s| s.len());
    println!("字符串长度: {:?}", length);
    // opt_string 仍然可用
    println!("原始字符串: {:?}", opt_string);

    // 使用 take 进行所有权转移
    let mut opt_vec: Option<Vec<i32>> = Some(vec![1, 2, 3]);
    if let Some(vec) = opt_vec.take() {
        println!("取出的向量: {:?}", vec);
        // 在这里可以消费 vec
    }
    println!("取出后的 Option: {:?}", opt_vec);

    // 延迟计算
    fn expensive_computation() -> i32 {
        println!("执行昂贵的计算...");
        42
    }

    let opt: Option<i32> = None;
    // 使用 unwrap_or_else 而不是 unwrap_or 来延迟计算
    let result = opt.unwrap_or_else(expensive_computation);
    println!("延迟计算结果: {}", result);

    // 批量处理 Option
    let options = vec![Some(1), None, Some(3), Some(4), None];

    // 高效的过滤和收集
    let values: Vec<i32> = options.into_iter().flatten().collect();
    println!("过滤后的值: {:?}", values);

    // 使用迭代器适配器
    let numbers = vec![Some(1), Some(2), None, Some(4)];
    let sum: i32 = numbers.iter().filter_map(|&opt| opt).map(|x| x * x).sum();
    println!("平方和: {}", sum);
}

/// ## 11. 实际应用场景
/// Option 在实际开发中的常见应用场景
fn real_world_scenarios() {
    println!("\n=== 11. 实际应用场景 ===");

    // 场景1: 配置管理
    #[derive(Debug)]
    struct AppConfig {
        database_url: String,
        port: u16,
        debug_mode: Option<bool>,
        max_connections: Option<u32>,
        ssl_cert_path: Option<String>,
    }

    impl AppConfig {
        fn new(database_url: String, port: u16) -> Self {
            AppConfig {
                database_url,
                port,
                debug_mode: None,
                max_connections: None,
                ssl_cert_path: None,
            }
        }

        fn is_debug(&self) -> bool {
            self.debug_mode.unwrap_or(false)
        }

        fn get_max_connections(&self) -> u32 {
            self.max_connections.unwrap_or(100)
        }

        fn has_ssl(&self) -> bool {
            self.ssl_cert_path.is_some()
        }
    }

    let config = AppConfig::new("postgresql://localhost/mydb".to_string(), 8080);
    println!("配置: {:?}", config);
    println!("调试模式: {}", config.is_debug());
    println!("最大连接数: {}", config.get_max_connections());
    println!("启用 SSL: {}", config.has_ssl());

    // 场景2: 缓存系统
    use std::collections::HashMap;

    struct Cache<K, V> {
        data: HashMap<K, V>,
    }

    impl<K, V> Cache<K, V>
    where
        K: std::hash::Hash + Eq + Clone,
        V: Clone,
    {
        fn new() -> Self {
            Cache {
                data: HashMap::new(),
            }
        }

        fn get(&self, key: &K) -> Option<&V> {
            self.data.get(key)
        }

        fn get_or_insert_with<F>(&mut self, key: K, f: F) -> &V
        where
            F: FnOnce() -> V,
        {
            self.data.entry(key).or_insert_with(f)
        }

        fn try_get_or_compute<F>(&mut self, key: K, compute: F) -> Option<&V>
        where
            F: FnOnce() -> Option<V>,
        {
            if self.data.contains_key(&key) {
                self.data.get(&key)
            } else {
                if let Some(value) = compute() {
                    self.data.insert(key.clone(), value);
                    self.data.get(&key)
                } else {
                    None
                }
            }
        }
    }

    let mut cache: Cache<String, i32> = Cache::new();

    // 缓存未命中，计算并存储
    let value = cache.get_or_insert_with("key1".to_string(), || {
        println!("计算 key1 的值...");
        42
    });
    println!("缓存值: {}", value);

    // 缓存命中
    let value = cache.get(&"key1".to_string());
    println!("从缓存获取: {:?}", value);

    // 场景3: 链表节点
    #[derive(Debug)]
    struct ListNode<T> {
        value: T,
        next: Option<Box<ListNode<T>>>,
    }

    impl<T> ListNode<T> {
        fn new(value: T) -> Self {
            ListNode { value, next: None }
        }

        fn append(&mut self, value: T) {
            match &mut self.next {
                Some(next_node) => next_node.append(value),
                None => {
                    self.next = Some(Box::new(ListNode::new(value)));
                }
            }
        }

        fn find(&self, target: &T) -> bool
        where
            T: PartialEq,
        {
            if self.value == *target {
                return true;
            }

            match &self.next {
                Some(next_node) => next_node.find(target),
                None => false,
            }
        }

        fn length(&self) -> usize {
            match &self.next {
                Some(next_node) => 1 + next_node.length(),
                None => 1,
            }
        }
    }

    let mut list = ListNode::new(1);
    list.append(2);
    list.append(3);
    list.append(4);

    println!("\n链表: {:?}", list);
    println!("链表长度: {}", list.length());
    println!("查找 3: {}", list.find(&3));
    println!("查找 5: {}", list.find(&5));

    // 场景4: 状态机
    #[derive(Debug, PartialEq)]
    enum State {
        Idle,
        Processing,
        Completed,
        Error(String),
    }

    struct StateMachine {
        current_state: State,
        data: Option<String>,
    }

    impl StateMachine {
        fn new() -> Self {
            StateMachine {
                current_state: State::Idle,
                data: None,
            }
        }

        fn start_processing(&mut self, input: String) -> Result<(), String> {
            match self.current_state {
                State::Idle => {
                    self.current_state = State::Processing;
                    self.data = Some(input);
                    Ok(())
                }
                _ => Err("无法在当前状态开始处理".to_string()),
            }
        }

        fn complete_processing(&mut self) -> Result<Option<String>, String> {
            match self.current_state {
                State::Processing => {
                    self.current_state = State::Completed;
                    Ok(self.data.take())
                }
                _ => Err("没有正在进行的处理".to_string()),
            }
        }

        fn reset(&mut self) {
            self.current_state = State::Idle;
            self.data = None;
        }

        fn get_state(&self) -> &State {
            &self.current_state
        }
    }

    let mut machine = StateMachine::new();
    println!("\n状态机演示:");
    println!("初始状态: {:?}", machine.get_state());

    match machine.start_processing("重要数据".to_string()) {
        Ok(()) => println!("开始处理数据"),
        Err(e) => println!("错误: {}", e),
    }
    println!("当前状态: {:?}", machine.get_state());

    match machine.complete_processing() {
        Ok(Some(data)) => println!("处理完成，数据: {}", data),
        Ok(None) => println!("处理完成，无数据"),
        Err(e) => println!("错误: {}", e),
    }
    println!("最终状态: {:?}", machine.get_state());

    println!("\n=== Option 解构分析完成 ===");
    println!("本文档涵盖了 Option 类型的所有核心概念和实际应用场景");
    println!("包括基础用法、模式匹配、方法链、错误处理、性能优化等方面");
}

// 额外的辅助函数和示例

/// 演示 Option 与迭代器的结合使用
#[allow(dead_code)]
fn option_with_iterators() {
    let data = vec!["1", "2", "invalid", "4", "5"];

    // 解析数字，跳过无效输入
    let numbers: Vec<i32> = data.iter().filter_map(|s| s.parse().ok()).collect();

    println!("解析的数字: {:?}", numbers);

    // 查找第一个偶数
    let first_even = numbers.iter().find(|&&n| n % 2 == 0).copied();

    println!("第一个偶数: {:?}", first_even);
}

/// 演示 Option 的序列化和反序列化（概念性）
#[allow(dead_code)]
fn option_serialization_concept() {
    // 在实际项目中，通常使用 serde 进行序列化
    // 这里展示概念性的实现

    fn serialize_option<T: std::fmt::Display>(opt: &Option<T>) -> String {
        match opt {
            Some(value) => format!("Some({})", value),
            None => "None".to_string(),
        }
    }

    let opt1: Option<i32> = Some(42);
    let opt2: Option<i32> = None;

    println!("序列化: {}", serialize_option(&opt1));
    println!("序列化: {}", serialize_option(&opt2));
}

/// 演示 Option 在异步编程中的应用（概念性）
#[allow(dead_code)]
fn option_in_async_context() {
    // 在实际的异步代码中，Option 经常用于表示可能的超时或取消

    struct AsyncResult<T> {
        data: Option<T>,
        is_timeout: bool,
    }

    impl<T> AsyncResult<T> {
        fn success(data: T) -> Self {
            AsyncResult {
                data: Some(data),
                is_timeout: false,
            }
        }

        fn timeout() -> Self {
            AsyncResult {
                data: None,
                is_timeout: true,
            }
        }

        fn cancelled() -> Self {
            AsyncResult {
                data: None,
                is_timeout: false,
            }
        }
    }

    let results = vec![
        AsyncResult::success("数据1"),
        AsyncResult::timeout(),
        AsyncResult::cancelled(),
    ];

    for (i, result) in results.iter().enumerate() {
        match (&result.data, result.is_timeout) {
            (Some(data), _) => println!("结果 {}: 成功 - {}", i, data),
            (None, true) => println!("结果 {}: 超时", i),
            (None, false) => println!("结果 {}: 已取消", i),
        }
    }
}
