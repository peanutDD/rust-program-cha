//! Rust match 和 if let 模式匹配全面教程
//!
//! 本模块详细讲解 Rust 中的模式匹配机制，包括：
//! - match 表达式的基础用法和高级特性
//! - if let 简洁控制流
//! - while let 循环模式匹配
//! - matches! 宏的使用
//! - 模式匹配的各种语法和技巧
//! - 实际应用场景和最佳实践

use std::collections::HashMap;

/// 演示基础的 match 表达式用法
pub mod basic_match {
    /// 基础枚举类型用于演示
    #[derive(Debug, Clone, PartialEq)]
    pub enum Direction {
        North,
        South,
        East,
        West,
    }

    /// 带有数据的枚举类型
    #[derive(Debug, Clone)]
    pub enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    /// 基础 match 表达式演示
    pub fn basic_match_demo() {
        println!("=== 基础 match 表达式演示 ===");

        let direction = Direction::North;

        // 基础匹配
        match direction {
            Direction::North => println!("向北前进！"),
            Direction::South => println!("向南前进！"),
            Direction::East => println!("向东前进！"),
            Direction::West => println!("向西前进！"),
        }

        // match 作为表达式返回值
        let direction_name = match direction {
            Direction::North => "北",
            Direction::South => "南",
            Direction::East => "东",
            Direction::West => "西",
        };
        println!("方向: {}", direction_name);
    }

    /// 演示从枚举中提取数据
    pub fn extract_data_demo() {
        println!("\n=== 从枚举中提取数据 ===");

        let messages = vec![
            Message::Quit,
            Message::Move { x: 10, y: 20 },
            Message::Write("Hello, Rust!".to_string()),
            Message::ChangeColor(255, 0, 0),
        ];

        for msg in messages {
            match msg {
                Message::Quit => {
                    println!("收到退出消息");
                }
                Message::Move { x, y } => {
                    println!("移动到坐标: ({}, {})", x, y);
                }
                Message::Write(text) => {
                    println!("写入文本: {}", text);
                }
                Message::ChangeColor(r, g, b) => {
                    println!("改变颜色为 RGB({}, {}, {})", r, g, b);
                }
            }
        }
    }

    /// 演示通配符模式
    pub fn wildcard_demo() {
        println!("\n=== 通配符模式演示 ===");

        let number = 7;

        match number {
            1 => println!("一"),
            3 => println!("三"),
            5 => println!("五"),
            7 => println!("七"),
            _ => println!("其他数字: {}", number),
        }

        // 使用 _ 忽略部分值
        let color = Message::ChangeColor(255, 128, 0);
        match color {
            Message::ChangeColor(r, _, _) => {
                println!("红色分量: {}", r);
            }
            _ => println!("不是颜色消息"),
        }
    }
}

/// 运行所有演示
pub fn demonstrate_all() {
    println!("\n🎯 开始 Rust match 和 if let 模式匹配全面教程");
    println!("{}", "=".repeat(60));

    // 基础 match 演示
    basic_match::basic_match_demo();
    basic_match::extract_data_demo();
    basic_match::wildcard_demo();

    // match 守卫演示
    match_guards::match_guards_demo();
    match_guards::multiple_patterns_demo();

    // @ 绑定演示
    at_bindings::at_bindings_demo();

    // if let 模式演示
    if_let_patterns::basic_if_let_demo();
    if_let_patterns::complex_if_let_demo();
    if_let_patterns::if_let_with_hashmap_demo();

    // while let 模式演示
    while_let_patterns::basic_while_let_demo();
    while_let_patterns::complex_while_let_demo();

    // matches! 宏演示
    matches_macro::basic_matches_demo();
    matches_macro::advanced_matches_demo();

    // 解构模式演示
    destructuring_patterns::tuple_destructuring_demo();
    destructuring_patterns::struct_destructuring_demo();
    destructuring_patterns::array_slice_destructuring_demo();

    // 实际应用演示
    practical_examples::practical_demo();

    // 最佳实践演示
    best_practices::performance_demo();
    best_practices::error_handling_demo();

    println!("\n✅ 所有演示完成！");
    println!("{}", "=".repeat(60));
}

/// 演示 match 守卫（match guards）
pub mod match_guards {
    /// 演示 match 守卫的使用
    pub fn match_guards_demo() {
        println!("\n=== Match 守卫演示 ===");

        let number = Some(4);

        match number {
            Some(x) if x < 5 => println!("小于 5 的数字: {}", x),
            Some(x) => println!("大于等于 5 的数字: {}", x),
            None => println!("没有数字"),
        }

        // 复杂的守卫条件
        let point = (3, 4);
        match point {
            (x, y) if x == y => println!("对角线上的点: ({}, {})", x, y),
            (x, y) if x > y => println!("x 大于 y: ({}, {})", x, y),
            (x, y) if x < y => println!("x 小于 qy: ({}, {})", x, y),
            _ => println!("其他情况"),
        }
    }

    /// 演示多个模式匹配
    pub fn multiple_patterns_demo() {
        println!("\n=== 多个模式匹配演示 ===");

        let x = 1;

        match x {
            1 | 2 => println!("一或二"),
            3..=5 => println!("三到五"),
            _ => println!("其他"),
        }

        // 字符范围匹配
        let letter = 'c';
        match letter {
            'a'..='j' => println!("前半部分字母"),
            'k'..='z' => println!("后半部分字母"),
            _ => println!("不是小写字母"),
        }
    }
}

/// 演示 @ 绑定
pub mod at_bindings {
    /// 演示 @ 绑定的使用
    pub fn at_bindings_demo() {
        println!("\n=== @ 绑定演示 ===");

        let age = 25;

        match age {
            young @ 0..=17 => println!("年轻人，年龄: {}", young),
            adult @ 18..=64 => println!("成年人，年龄: {}", adult),
            senior @ 65.. => println!("老年人，年龄: {}", senior),
            _ => println!("无效年龄: {}", age),
        }

        // 在枚举中使用 @ 绑定
        #[derive(Debug)]
        enum Status {
            Active(u32),
            Inactive,
        }

        let status = Status::Active(42);
        match status {
            Status::Active(id @ 1..=100) => {
                println!("活跃状态，ID 在有效范围内: {}", id);
            }
            Status::Active(id) => {
                println!("活跃状态，ID 超出范围: {}", id);
            }
            Status::Inactive => {
                println!("非活跃状态");
            }
        }
    }
}

/// 演示 if let 简洁控制流
pub mod if_let_patterns {
    use std::collections::HashMap;

    /// 基础 if let 演示
    pub fn basic_if_let_demo() {
        println!("\n=== 基础 if let 演示 ===");

        let config_max = Some(3u8);

        // 使用 match 的冗长方式
        match config_max {
            Some(max) => println!("最大值配置为: {}", max),
            _ => (),
        }

        // 使用 if let 的简洁方式
        if let Some(max) = config_max {
            println!("使用 if let - 最大值配置为: {}", max);
        }

        // if let 与 else 结合
        let optional_word = Some("Hello".to_string());
        if let Some(word) = optional_word {
            println!("找到单词: {}", word);
        } else {
            println!("没有找到单词");
        }
    }

    /// 复杂的 if let 模式
    pub fn complex_if_let_demo() {
        println!("\n=== 复杂 if let 模式演示 ===");

        // 解构元组
        let tuple = Some((1, 2, 3));
        if let Some((x, y, z)) = tuple {
            println!("元组值: x={}, y={}, z={}", x, y, z);
        }

        // 解构结构体
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }

        let point = Some(Point { x: 10, y: 20 });
        if let Some(Point { x, y }) = point {
            println!("点坐标: ({}, {})", x, y);
        }

        // 嵌套的 if let
        let nested = Some(Some(42));
        if let Some(inner) = nested {
            if let Some(value) = inner {
                println!("嵌套值: {}", value);
            }
        }
    }

    /// 演示 if let 与 HashMap
    pub fn if_let_with_hashmap_demo() {
        println!("\n=== if let 与 HashMap 演示 ===");

        let mut scores = HashMap::new();
        scores.insert("Alice", 95);
        scores.insert("Bob", 87);

        // 检查特定键是否存在
        if let Some(score) = scores.get("Alice") {
            println!("Alice 的分数: {}", score);
        }

        if let Some(score) = scores.get("Charlie") {
            println!("Charlie 的分数: {}", score);
        } else {
            println!("Charlie 不在分数表中");
        }
    }
}

/// 演示 while let 循环模式匹配
pub mod while_let_patterns {
    /// while let 基础演示
    pub fn basic_while_let_demo() {
        println!("\n=== 基础 while let 演示 ===");

        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        // 使用 while let 处理栈
        while let Some(top) = stack.pop() {
            println!("弹出元素: {}", top);
        }

        println!("栈已清空");
    }

    /// 复杂的 while let 模式
    pub fn complex_while_let_demo() {
        println!("\n=== 复杂 while let 演示 ===");

        let mut iter = vec![1, 2, 3, 4, 5].into_iter();

        // 处理迭代器
        while let Some(value) = iter.next() {
            if value % 2 == 0 {
                println!("偶数: {}", value);
            } else {
                println!("奇数: {}", value);
            }
        }

        // 处理嵌套的 Option
        let mut nested_options = vec![Some(Some(1)), Some(None), None, Some(Some(2))];

        while let Some(outer) = nested_options.pop() {
            match outer {
                Some(Some(value)) => println!("找到值: {}", value),
                Some(None) => println!("外层有值，内层为空"),
                None => println!("外层为空"),
            }
        }
    }
}

/// 演示 matches! 宏
pub mod matches_macro {
    /// matches! 宏基础演示
    pub fn basic_matches_demo() {
        println!("\n=== matches! 宏演示 ===");

        let number = 42;

        // 基础用法
        if matches!(number, 42) {
            println!("数字是 42");
        }

        // 范围匹配
        if matches!(number, 1..=100) {
            println!("数字在 1 到 100 之间");
        }

        // 多个模式
        if matches!(number, 1 | 2 | 42 | 100) {
            println!("数字是特殊值");
        }

        // 与枚举一起使用
        #[derive(Debug)]
        enum Status {
            Active,
            Inactive,
            Pending(u32),
        }

        let status = Status::Pending(5);

        if matches!(status, Status::Active | Status::Pending(_)) {
            println!("状态是活跃或待处理");
        }

        // 在过滤器中使用
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let even_count = numbers.iter().filter(|&&x| matches!(x, 2 | 4 | 6)).count();
        println!("偶数个数: {}", even_count);
    }

    /// matches! 宏高级用法
    pub fn advanced_matches_demo() {
        println!("\n=== matches! 宏高级用法 ===");

        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }

        let point = Point { x: 0, y: 0 };

        // 结构体模式匹配
        if matches!(point, Point { x: 0, y: 0 }) {
            println!("点在原点");
        }

        // 复杂的嵌套模式
        let data = Some((1, "hello"));
        if matches!(data, Some((1, _))) {
            println!("第一个元素是 1 的元组");
        }

        // 在断言中使用
        assert!(matches!(data, Some(_)));
        println!("断言通过：data 包含值");
    }
}

/// 演示解构模式
pub mod destructuring_patterns {
    /// 解构元组
    pub fn tuple_destructuring_demo() {
        println!("\n=== 元组解构演示 ===");

        let tuple = (1, 2, 3, 4, 5);

        // 完全解构
        let (a, b, c, d, e) = tuple;
        println!("完全解构: a={}, b={}, c={}, d={}, e={}", a, b, c, d, e);

        // 部分解构，忽略某些值
        let (first, _, third, ..) = tuple;
        println!("部分解构: first={}, third={}", first, third);

        // 在 match 中解构
        match tuple {
            (1, 2, ..) => println!("以 (1, 2) 开头的元组"),
            _ => println!("其他元组"),
        }
    }

    /// 解构结构体
    pub fn struct_destructuring_demo() {
        println!("\n=== 结构体解构演示 ===");

        #[derive(Debug)]
        struct Person {
            name: String,
            age: u32,
            email: String,
        }

        let person = Person {
            name: "Alice".to_string(),
            age: 30,
            email: "alice@example.com".to_string(),
        };

        // 完全解构
        let Person { name, age, email } = &person;
        println!("姓名: {}, 年龄: {}, 邮箱: {}", name, age, email);

        // 部分解构，重命名字段
        let Person {
            name: person_name,
            age: person_age,
            ..
        } = &person;
        println!("人员信息: {} ({}岁)", person_name, person_age);

        // 在 match 中解构
        match person {
            Person { age: 30, .. } => println!("30岁的人"),
            Person { name, .. } if name.starts_with('A') => {
                println!("名字以 A 开头的人: {}", name);
            }
            _ => println!("其他人"),
        }
    }

    /// 解构数组和切片
    pub fn array_slice_destructuring_demo() {
        println!("\n=== 数组和切片解构演示 ===");

        let array = [1, 2, 3, 4, 5];

        // 数组解构
        let [first, second, ..] = array;
        println!("前两个元素: {}, {}", first, second);

        // 切片模式匹配
        let slice = &array[..];
        match slice {
            [] => println!("空切片"),
            [x] => println!("单元素切片: {}", x),
            [x, y] => println!("双元素切片: {}, {}", x, y),
            [first, .., last] => println!("多元素切片，首尾: {}, {}", first, last),
        }

        // 向量解构
        let vec = vec![1, 2, 3];
        match vec.as_slice() {
            [1, rest @ ..] => println!("以 1 开头，其余: {:?}", rest),
            _ => println!("其他模式"),
        }
    }
}

/// 实际应用场景演示
pub mod practical_examples {
    use std::collections::HashMap;

    /// HTTP 状态码处理
    #[derive(Debug, Clone, PartialEq)]
    pub enum HttpStatus {
        Ok,
        NotFound,
        InternalServerError,
        Custom(u16),
    }

    impl HttpStatus {
        pub fn from_code(code: u16) -> Self {
            match code {
                200 => HttpStatus::Ok,
                404 => HttpStatus::NotFound,
                500 => HttpStatus::InternalServerError,
                other => HttpStatus::Custom(other),
            }
        }

        pub fn message(&self) -> &str {
            match self {
                HttpStatus::Ok => "请求成功",
                HttpStatus::NotFound => "资源未找到",
                HttpStatus::InternalServerError => "服务器内部错误",
                HttpStatus::Custom(code) => match code {
                    400..=499 => "客户端错误",
                    500..=599 => "服务器错误",
                    _ => "未知状态码",
                },
            }
        }
    }

    /// JSON 值处理
    #[derive(Debug, Clone)]
    pub enum JsonValue {
        Null,
        Bool(bool),
        Number(f64),
        String(String),
        Array(Vec<JsonValue>),
        Object(HashMap<String, JsonValue>),
    }

    impl JsonValue {
        pub fn get_type(&self) -> &str {
            match self {
                JsonValue::Null => "null",
                JsonValue::Bool(_) => "boolean",
                JsonValue::Number(_) => "number",
                JsonValue::String(_) => "string",
                JsonValue::Array(_) => "array",
                JsonValue::Object(_) => "object",
            }
        }

        pub fn is_truthy(&self) -> bool {
            match self {
                JsonValue::Null => false,
                JsonValue::Bool(b) => *b,
                JsonValue::Number(n) => *n != 0.0,
                JsonValue::String(s) => !s.is_empty(),
                JsonValue::Array(arr) => !arr.is_empty(),
                JsonValue::Object(obj) => !obj.is_empty(),
            }
        }
    }

    /// 配置解析器
    #[derive(Debug)]
    pub struct Config {
        pub database_url: Option<String>,
        pub port: Option<u16>,
        pub debug: bool,
    }

    impl Config {
        pub fn from_env() -> Self {
            Config {
                database_url: std::env::var("DATABASE_URL").ok(),
                port: std::env::var("PORT").ok().and_then(|s| s.parse().ok()),
                debug: matches!(std::env::var("DEBUG").as_deref(), Ok("true" | "1")),
            }
        }

        pub fn validate(&self) -> Result<(), &'static str> {
            match (&self.database_url, &self.port) {
                (None, _) => Err("数据库 URL 是必需的"),
                (Some(url), _) if url.is_empty() => Err("数据库 URL 不能为空"),
                (Some(_), Some(port)) if *port < 1024 => Err("端口号不能小于 1024"),
                _ => Ok(()),
            }
        }
    }

    /// 实际应用演示
    pub fn practical_demo() {
        println!("\n=== 实际应用场景演示 ===");

        // HTTP 状态码处理
        let statuses = vec![200, 404, 500, 418];
        for code in statuses {
            let status = HttpStatus::from_code(code);
            println!("状态码 {}: {}", code, status.message());
        }

        // JSON 值处理
        let json_values = vec![
            JsonValue::Null,
            JsonValue::Bool(true),
            JsonValue::Number(42.0),
            JsonValue::String("hello".to_string()),
            JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Number(2.0)]),
        ];

        for value in json_values {
            println!(
                "JSON 值类型: {}, 真值性: {}",
                value.get_type(),
                value.is_truthy()
            );
        }

        // 配置处理
        let config = Config {
            database_url: Some("postgresql://localhost/mydb".to_string()),
            port: Some(8080),
            debug: true,
        };

        match config.validate() {
            Ok(()) => println!("配置验证通过"),
            Err(msg) => println!("配置验证失败: {}", msg),
        }
    }
}

/// 性能优化和最佳实践
pub mod best_practices {
    /// 演示模式匹配的性能考虑
    pub fn performance_demo() {
        println!("\n=== 性能优化演示 ===");

        // 避免不必要的克隆
        let data = vec!["hello".to_string(), "world".to_string()];

        // 好的做法：使用引用
        for item in &data {
            match item.as_str() {
                "hello" => println!("找到问候语"),
                "world" => println!("找到世界"),
                _ => println!("其他字符串: {}", item),
            }
        }

        // 使用 if let 替代复杂的 match
        let optional_value = Some(42);

        // 简洁的方式
        if let Some(value) = optional_value {
            println!("值: {}", value);
        }

        // 避免过度嵌套
        let nested_option = Some(Some("value"));

        // 使用 flatten 或其他方法简化
        if let Some(inner) = nested_option.flatten() {
            println!("嵌套值: {}", inner);
        }
    }

    /// 错误处理最佳实践
    pub fn error_handling_demo() {
        println!("\n=== 错误处理最佳实践 ===");

        #[derive(Debug)]
        enum ParseError {
            InvalidFormat,
            OutOfRange,
            Empty,
        }

        fn parse_number(s: &str) -> Result<i32, ParseError> {
            match s {
                "" => Err(ParseError::Empty),
                s if s.chars().all(|c| c.is_ascii_digit()) => match s.parse::<i32>() {
                    Ok(n) if n >= 0 && n <= 100 => Ok(n),
                    Ok(_) => Err(ParseError::OutOfRange),
                    Err(_) => Err(ParseError::InvalidFormat),
                },
                _ => Err(ParseError::InvalidFormat),
            }
        }

        let test_cases = vec!["", "42", "150", "abc", "12"];

        for case in test_cases {
            match parse_number(case) {
                Ok(num) => println!("解析成功: {} -> {}", case, num),
                Err(ParseError::Empty) => println!("错误: 输入为空"),
                Err(ParseError::InvalidFormat) => println!("错误: 格式无效 - {}", case),
                Err(ParseError::OutOfRange) => println!("错误: 超出范围 - {}", case),
            }
        }
    }
}

/// 测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_match() {
        use basic_match::Direction;

        let direction = Direction::North;
        let result = match direction {
            Direction::North => "north",
            Direction::South => "south",
            Direction::East => "east",
            Direction::West => "west",
        };
        assert_eq!(result, "north");
    }

    #[test]
    fn test_if_let() {
        let option = Some(42);
        let mut result = None;

        if let Some(value) = option {
            result = Some(value * 2);
        }

        assert_eq!(result, Some(84));
    }

    #[test]
    fn test_matches_macro() {
        let number = 42;
        assert!(matches!(number, 42));
        assert!(matches!(number, 1..=100));
        assert!(!matches!(number, 1..=10));
    }

    #[test]
    fn test_http_status() {
        use practical_examples::HttpStatus;

        let status = HttpStatus::from_code(404);
        assert_eq!(status, HttpStatus::NotFound);
        assert_eq!(status.message(), "资源未找到");
    }

    #[test]
    fn test_json_value() {
        use practical_examples::JsonValue;

        let value = JsonValue::String("hello".to_string());
        assert_eq!(value.get_type(), "string");
        assert!(value.is_truthy());

        let empty_string = JsonValue::String("".to_string());
        assert!(!empty_string.is_truthy());
    }

    #[test]
    fn test_destructuring() {
        let tuple = (1, 2, 3);
        let (a, b, c) = tuple;
        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(c, 3);

        let array = [1, 2, 3, 4, 5];
        let [first, second, ..] = array;
        assert_eq!(first, 1);
        assert_eq!(second, 2);
    }

    #[test]
    fn test_while_let() {
        let mut stack = vec![1, 2, 3];
        let mut sum = 0;

        while let Some(value) = stack.pop() {
            sum += value;
        }

        assert_eq!(sum, 6);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_match_guards() {
        let number = Some(4);
        let result = match number {
            Some(x) if x < 5 => "small",
            Some(x) if x >= 5 => "large",
            Some(_) => "other",
            None => "none",
        };
        assert_eq!(result, "small");
    }

    #[test]
    fn test_at_bindings() {
        let age = 25;
        let category = match age {
            young @ 0..=17 => format!("young: {}", young),
            adult @ 18..=64 => format!("adult: {}", adult),
            senior @ 65.. => format!("senior: {}", senior),
            _ => format!("invalid: {}", age),
        };
        assert_eq!(category, "adult: 25");
    }
}
