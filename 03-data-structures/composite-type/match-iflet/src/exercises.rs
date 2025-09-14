//! Rust match 和 if let 模式匹配练习题
//!
//! 本模块包含递进式的练习题，帮助掌握：
//! - match 表达式的各种用法
//! - if let 和 while let 的应用
//! - matches! 宏的使用
//! - 模式匹配的高级技巧
//! - 实际项目中的应用场景

use std::collections::HashMap;

/// 练习1：基础 match 表达式
pub mod exercise1_basic_match {
    /// 定义一个表示交通信号灯的枚举
    #[derive(Debug, Clone, PartialEq)]
    pub enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    impl TrafficLight {
        /// 练习：实现获取下一个信号灯状态的方法
        pub fn next(&self) -> TrafficLight {
            // TODO: 使用 match 表达式实现信号灯的循环
            // Red -> Green -> Yellow -> Red
            match self {
                TrafficLight::Red => TrafficLight::Green,
                TrafficLight::Green => TrafficLight::Yellow,
                TrafficLight::Yellow => TrafficLight::Red,
            }
        }

        /// 练习：实现获取等待时间的方法
        pub fn wait_time(&self) -> u32 {
            // TODO: 使用 match 返回不同信号灯的等待时间（秒）
            // Red: 30, Yellow: 5, Green: 25
            match self {
                TrafficLight::Red => 30,
                TrafficLight::Yellow => 5,
                TrafficLight::Green => 25,
            }
        }

        /// 练习：判断是否可以通行
        pub fn can_pass(&self) -> bool {
            // TODO: 使用 match 判断是否可以通行
            // 只有绿灯可以通行
            match self {
                TrafficLight::Green => true,
                _ => false,
            }
        }
    }

    /// 练习函数：模拟交通信号灯运行
    pub fn simulate_traffic_light() {
        println!("=== 练习1：交通信号灯模拟 ===");

        let mut current = TrafficLight::Red;

        for i in 0..6 {
            println!(
                "第{}秒: {:?} - 等待时间: {}秒, 可通行: {}",
                i,
                current,
                current.wait_time(),
                current.can_pass()
            );
            current = current.next();
        }
    }
}

/// 练习2：带数据的枚举和解构
pub mod exercise2_enum_with_data {
    /// 定义一个表示几何图形的枚举
    #[derive(Debug, Clone)]
    pub enum Shape {
        Circle { radius: f64 },
        Rectangle { width: f64, height: f64 },
        Triangle { base: f64, height: f64 },
        Square { side: f64 },
    }

    impl Shape {
        /// 练习：计算图形面积
        pub fn area(&self) -> f64 {
            // TODO: 使用 match 解构枚举并计算面积
            match self {
                Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
                Shape::Rectangle { width, height } => width * height,
                Shape::Triangle { base, height } => 0.5 * base * height,
                Shape::Square { side } => side * side,
            }
        }

        /// 练习：计算图形周长
        pub fn perimeter(&self) -> f64 {
            // TODO: 使用 match 解构枚举并计算周长
            match self {
                Shape::Circle { radius } => 2.0 * std::f64::consts::PI * radius,
                Shape::Rectangle { width, height } => 2.0 * (width + height),
                Shape::Triangle { base, height } => {
                    // 假设是等腰三角形，计算斜边
                    let side = (height * height + (base / 2.0) * (base / 2.0)).sqrt();
                    base + 2.0 * side
                }
                Shape::Square { side } => 4.0 * side,
            }
        }

        /// 练习：判断图形类型
        pub fn shape_type(&self) -> &str {
            // TODO: 使用 match 返回图形类型名称
            match self {
                Shape::Circle { .. } => "圆形",
                Shape::Rectangle { .. } => "矩形",
                Shape::Triangle { .. } => "三角形",
                Shape::Square { .. } => "正方形",
            }
        }
    }

    /// 练习函数：几何图形计算
    pub fn geometry_calculations() {
        println!("\n=== 练习2：几何图形计算 ===");

        let shapes = vec![
            Shape::Circle { radius: 5.0 },
            Shape::Rectangle {
                width: 4.0,
                height: 6.0,
            },
            Shape::Triangle {
                base: 8.0,
                height: 6.0,
            },
            Shape::Square { side: 4.0 },
        ];

        for shape in shapes {
            println!(
                "{}: 面积 = {:.2}, 周长 = {:.2}",
                shape.shape_type(),
                shape.area(),
                shape.perimeter()
            );
        }
    }
}

/// 练习3：if let 和 Option 处理
pub mod exercise3_if_let_option {
    use std::collections::HashMap;

    /// 学生信息结构体
    #[derive(Debug, Clone)]
    pub struct Student {
        pub name: String,
        pub age: u32,
        pub grade: Option<char>,
        pub subjects: Vec<String>,
    }

    /// 学生管理系统
    pub struct StudentManager {
        students: HashMap<String, Student>,
    }

    impl StudentManager {
        pub fn new() -> Self {
            Self {
                students: HashMap::new(),
            }
        }

        pub fn add_student(&mut self, student: Student) {
            self.students.insert(student.name.clone(), student);
        }

        /// 练习：使用 if let 查找学生
        pub fn find_student(&self, name: &str) -> Option<&Student> {
            // TODO: 使用 if let 查找学生
            if let Some(student) = self.students.get(name) {
                Some(student)
            } else {
                None
            }
        }

        /// 练习：使用 if let 处理学生成绩
        pub fn print_student_grade(&self, name: &str) {
            // TODO: 使用嵌套的 if let 处理 Option
            if let Some(student) = self.students.get(name) {
                if let Some(grade) = student.grade {
                    println!("{} 的成绩是: {}", name, grade);
                } else {
                    println!("{} 还没有成绩", name);
                }
            } else {
                println!("找不到学生: {}", name);
            }
        }

        /// 练习：统计有成绩的学生
        pub fn count_graded_students(&self) -> usize {
            // TODO: 使用 if let 和迭代器统计有成绩的学生
            self.students
                .values()
                .filter(|student| {
                    if let Some(_) = student.grade {
                        true
                    } else {
                        false
                    }
                })
                .count()
        }

        /// 练习：获取所有优秀学生（成绩 A 或 B）
        pub fn get_excellent_students(&self) -> Vec<&Student> {
            // TODO: 使用 if let 和 matches! 筛选优秀学生
            self.students
                .values()
                .filter(|student| {
                    if let Some(grade) = student.grade {
                        matches!(grade, 'A' | 'B')
                    } else {
                        false
                    }
                })
                .collect()
        }
    }

    /// 练习函数：学生管理系统演示
    pub fn student_management_demo() {
        println!("\n=== 练习3：学生管理系统 ===");

        let mut manager = StudentManager::new();

        // 添加学生
        manager.add_student(Student {
            name: "Alice".to_string(),
            age: 20,
            grade: Some('A'),
            subjects: vec!["数学".to_string(), "物理".to_string()],
        });

        manager.add_student(Student {
            name: "Bob".to_string(),
            age: 19,
            grade: Some('B'),
            subjects: vec!["化学".to_string(), "生物".to_string()],
        });

        manager.add_student(Student {
            name: "Charlie".to_string(),
            age: 21,
            grade: None,
            subjects: vec!["历史".to_string()],
        });

        // 测试功能
        manager.print_student_grade("Alice");
        manager.print_student_grade("Charlie");
        manager.print_student_grade("David");

        println!("有成绩的学生数量: {}", manager.count_graded_students());

        let excellent = manager.get_excellent_students();
        println!(
            "优秀学生: {:?}",
            excellent.iter().map(|s| &s.name).collect::<Vec<_>>()
        );
    }
}

/// 练习4：while let 和迭代器处理
pub mod exercise4_while_let_iterator {
    /// 任务状态枚举
    #[derive(Debug, Clone)]
    pub enum TaskStatus {
        Pending,
        InProgress,
        Completed,
        Failed(String),
    }

    /// 任务结构体
    #[derive(Debug, Clone)]
    pub struct Task {
        pub id: u32,
        pub name: String,
        pub status: TaskStatus,
        pub priority: u32,
    }

    /// 任务处理器
    pub struct TaskProcessor {
        tasks: Vec<Task>,
    }

    impl TaskProcessor {
        pub fn new() -> Self {
            Self { tasks: Vec::new() }
        }

        pub fn add_task(&mut self, task: Task) {
            self.tasks.push(task);
        }

        /// 练习：使用 while let 处理任务队列
        pub fn process_pending_tasks(&mut self) {
            println!("\n处理待处理任务:");

            // TODO: 使用 while let 处理所有待处理的任务
            let mut i = 0;
            while i < self.tasks.len() {
                if let TaskStatus::Pending = self.tasks[i].status {
                    println!(
                        "处理任务: {} (ID: {})",
                        self.tasks[i].name, self.tasks[i].id
                    );
                    self.tasks[i].status = TaskStatus::InProgress;
                }
                i += 1;
            }
        }

        /// 练习：使用 while let 和迭代器处理高优先级任务
        pub fn process_high_priority_tasks(&mut self) {
            println!("\n处理高优先级任务:");

            // TODO: 筛选并处理优先级大于等于8的任务
            let mut high_priority_tasks: Vec<_> = self
                .tasks
                .iter_mut()
                .filter(|task| task.priority >= 8)
                .collect();

            while let Some(task) = high_priority_tasks.pop() {
                match &task.status {
                    TaskStatus::Pending | TaskStatus::InProgress => {
                        println!(
                            "完成高优先级任务: {} (优先级: {})",
                            task.name, task.priority
                        );
                        task.status = TaskStatus::Completed;
                    }
                    _ => {}
                }
            }
        }

        /// 练习：统计任务状态
        pub fn task_statistics(&self) {
            println!("\n任务统计:");

            let mut pending = 0;
            let mut in_progress = 0;
            let mut completed = 0;
            let mut failed = 0;

            // TODO: 使用 match 统计各种状态的任务数量
            for task in &self.tasks {
                match task.status {
                    TaskStatus::Pending => pending += 1,
                    TaskStatus::InProgress => in_progress += 1,
                    TaskStatus::Completed => completed += 1,
                    TaskStatus::Failed(_) => failed += 1,
                }
            }

            println!(
                "待处理: {}, 进行中: {}, 已完成: {}, 失败: {}",
                pending, in_progress, completed, failed
            );
        }

        /// 练习：处理失败任务的错误信息
        pub fn handle_failed_tasks(&self) {
            println!("\n处理失败任务:");

            // TODO: 使用 if let 处理失败任务的错误信息
            for task in &self.tasks {
                if let TaskStatus::Failed(error) = &task.status {
                    println!("任务 '{}' 失败，错误: {}", task.name, error);
                }
            }
        }
    }

    /// 练习函数：任务处理演示
    pub fn task_processing_demo() {
        println!("\n=== 练习4：任务处理系统 ===");

        let mut processor = TaskProcessor::new();

        // 添加任务
        processor.add_task(Task {
            id: 1,
            name: "数据备份".to_string(),
            status: TaskStatus::Pending,
            priority: 9,
        });

        processor.add_task(Task {
            id: 2,
            name: "发送邮件".to_string(),
            status: TaskStatus::Pending,
            priority: 5,
        });

        processor.add_task(Task {
            id: 3,
            name: "生成报告".to_string(),
            status: TaskStatus::InProgress,
            priority: 8,
        });

        processor.add_task(Task {
            id: 4,
            name: "清理缓存".to_string(),
            status: TaskStatus::Failed("磁盘空间不足".to_string()),
            priority: 3,
        });

        // 处理任务
        processor.process_pending_tasks();
        processor.process_high_priority_tasks();
        processor.task_statistics();
        processor.handle_failed_tasks();
    }
}

/// 练习5：matches! 宏和复杂模式匹配
pub mod exercise5_matches_macro {
    /// 网络请求类型
    #[derive(Debug, Clone)]
    pub enum HttpMethod {
        Get,
        Post,
        Put,
        Delete,
        Patch,
    }

    /// HTTP 请求结构体
    #[derive(Debug, Clone)]
    pub struct HttpRequest {
        pub method: HttpMethod,
        pub path: String,
        pub status_code: Option<u16>,
        pub body_size: usize,
    }

    /// 日志分析器
    pub struct LogAnalyzer {
        requests: Vec<HttpRequest>,
    }

    impl LogAnalyzer {
        pub fn new() -> Self {
            Self {
                requests: Vec::new(),
            }
        }

        pub fn add_request(&mut self, request: HttpRequest) {
            self.requests.push(request);
        }

        /// 练习：使用 matches! 筛选 GET 和 POST 请求
        pub fn filter_main_methods(&self) -> Vec<&HttpRequest> {
            // TODO: 使用 matches! 宏筛选 GET 和 POST 请求
            self.requests
                .iter()
                .filter(|req| matches!(req.method, HttpMethod::Get | HttpMethod::Post))
                .collect()
        }

        /// 练习：使用 matches! 筛选成功的请求
        pub fn filter_successful_requests(&self) -> Vec<&HttpRequest> {
            // TODO: 使用 matches! 筛选状态码在 200-299 范围的请求
            self.requests
                .iter()
                .filter(|req| {
                    if let Some(code) = req.status_code {
                        matches!(code, 200..=299)
                    } else {
                        false
                    }
                })
                .collect()
        }

        /// 练习：使用 matches! 筛选大请求
        pub fn filter_large_requests(&self) -> Vec<&HttpRequest> {
            // TODO: 使用 matches! 筛选 body 大小超过 1024 字节的请求
            self.requests
                .iter()
                .filter(|req| matches!(req.body_size, size if size > 1024))
                .collect()
        }

        /// 练习：使用 matches! 筛选 API 路径
        pub fn filter_api_requests(&self) -> Vec<&HttpRequest> {
            // TODO: 使用 matches! 和字符串模式筛选以 "/api/" 开头的请求
            self.requests
                .iter()
                .filter(|req| req.path.starts_with("/api/"))
                .collect()
        }

        /// 练习：复杂条件筛选
        pub fn filter_complex_conditions(&self) -> Vec<&HttpRequest> {
            // TODO: 筛选满足以下条件的请求：
            // 1. 方法是 POST 或 PUT
            // 2. 状态码是 4xx 或 5xx
            // 3. body 大小在 100-1000 字节之间
            self.requests
                .iter()
                .filter(|req| {
                    matches!(req.method, HttpMethod::Post | HttpMethod::Put)
                        && req
                            .status_code
                            .map_or(false, |code| matches!(code, 400..=599))
                        && matches!(req.body_size, 100..=1000)
                })
                .collect()
        }

        /// 练习：统计分析
        pub fn analyze_requests(&self) {
            println!("\n请求分析结果:");

            let total = self.requests.len();
            let main_methods = self.filter_main_methods().len();
            let successful = self.filter_successful_requests().len();
            let large = self.filter_large_requests().len();
            let api = self.filter_api_requests().len();
            let complex = self.filter_complex_conditions().len();

            println!("总请求数: {}", total);
            println!("GET/POST 请求: {}", main_methods);
            println!("成功请求 (2xx): {}", successful);
            println!("大请求 (>1024字节): {}", large);
            println!("API 请求: {}", api);
            println!("复杂条件匹配: {}", complex);
        }
    }

    /// 练习函数：日志分析演示
    pub fn log_analysis_demo() {
        println!("\n=== 练习5：HTTP 日志分析 ===");

        let mut analyzer = LogAnalyzer::new();

        // 添加请求日志
        analyzer.add_request(HttpRequest {
            method: HttpMethod::Get,
            path: "/api/users".to_string(),
            status_code: Some(200),
            body_size: 512,
        });

        analyzer.add_request(HttpRequest {
            method: HttpMethod::Post,
            path: "/api/users".to_string(),
            status_code: Some(201),
            body_size: 256,
        });

        analyzer.add_request(HttpRequest {
            method: HttpMethod::Put,
            path: "/api/users/123".to_string(),
            status_code: Some(404),
            body_size: 128,
        });

        analyzer.add_request(HttpRequest {
            method: HttpMethod::Get,
            path: "/static/image.jpg".to_string(),
            status_code: Some(200),
            body_size: 2048,
        });

        analyzer.add_request(HttpRequest {
            method: HttpMethod::Post,
            path: "/login".to_string(),
            status_code: Some(500),
            body_size: 800,
        });

        // 分析请求
        analyzer.analyze_requests();
    }
}

/// 练习6：综合应用 - 配置文件解析器
pub mod exercise6_config_parser {
    use std::collections::HashMap;

    /// 配置值类型
    #[derive(Debug, Clone, PartialEq)]
    pub enum ConfigValue {
        String(String),
        Integer(i64),
        Float(f64),
        Boolean(bool),
        Array(Vec<ConfigValue>),
        Object(HashMap<String, ConfigValue>),
    }

    /// 配置解析错误
    #[derive(Debug, Clone)]
    pub enum ParseError {
        InvalidSyntax(String),
        TypeMismatch(String),
        MissingKey(String),
        InvalidValue(String),
    }

    /// 配置解析器
    pub struct ConfigParser {
        config: HashMap<String, ConfigValue>,
    }

    impl ConfigParser {
        pub fn new() -> Self {
            Self {
                config: HashMap::new(),
            }
        }

        pub fn set(&mut self, key: String, value: ConfigValue) {
            self.config.insert(key, value);
        }

        /// 练习：使用 if let 获取字符串值
        pub fn get_string(&self, key: &str) -> Result<String, ParseError> {
            // TODO: 使用 if let 获取字符串值
            if let Some(value) = self.config.get(key) {
                if let ConfigValue::String(s) = value {
                    Ok(s.clone())
                } else {
                    Err(ParseError::TypeMismatch(format!(
                        "Expected string for key '{}', found {:?}",
                        key, value
                    )))
                }
            } else {
                Err(ParseError::MissingKey(key.to_string()))
            }
        }

        /// 练习：使用 match 获取数值
        pub fn get_number(&self, key: &str) -> Result<f64, ParseError> {
            // TODO: 使用 match 处理整数和浮点数
            match self.config.get(key) {
                Some(ConfigValue::Integer(i)) => Ok(*i as f64),
                Some(ConfigValue::Float(f)) => Ok(*f),
                Some(other) => Err(ParseError::TypeMismatch(format!(
                    "Expected number for key '{}', found {:?}",
                    key, other
                ))),
                None => Err(ParseError::MissingKey(key.to_string())),
            }
        }

        /// 练习：使用 matches! 验证配置
        pub fn validate_config(&self) -> Result<(), Vec<ParseError>> {
            let mut errors = Vec::new();

            // TODO: 验证必需的配置项
            let required_keys = ["database_url", "port", "debug"];

            for &key in &required_keys {
                if !self.config.contains_key(key) {
                    errors.push(ParseError::MissingKey(key.to_string()));
                }
            }

            // TODO: 验证端口号范围
            if let Ok(port) = self.get_number("port") {
                if !matches!(port as u16, 1024..=65535) {
                    errors.push(ParseError::InvalidValue(
                        "Port must be between 1024 and 65535".to_string(),
                    ));
                }
            }

            // TODO: 验证数据库 URL 格式
            if let Ok(url) = self.get_string("database_url") {
                if !url.starts_with("postgresql://") && !url.starts_with("mysql://") {
                    errors.push(ParseError::InvalidValue(
                        "Database URL must start with postgresql:// or mysql://".to_string(),
                    ));
                }
            }

            if errors.is_empty() {
                Ok(())
            } else {
                Err(errors)
            }
        }

        /// 练习：使用模式匹配处理嵌套配置
        pub fn get_nested_value(&self, path: &str) -> Result<&ConfigValue, ParseError> {
            // TODO: 处理形如 "database.host" 的嵌套路径
            let parts: Vec<&str> = path.split('.').collect();

            match parts.as_slice() {
                [key] => self
                    .config
                    .get(*key)
                    .ok_or_else(|| ParseError::MissingKey(key.to_string())),
                [parent, child] => {
                    if let Some(ConfigValue::Object(obj)) = self.config.get(*parent) {
                        obj.get(*child)
                            .ok_or_else(|| ParseError::MissingKey(path.to_string()))
                    } else {
                        Err(ParseError::TypeMismatch(format!(
                            "Expected object for key '{}'",
                            parent
                        )))
                    }
                }
                _ => Err(ParseError::InvalidSyntax(
                    "Path too deep, maximum 2 levels supported".to_string(),
                )),
            }
        }

        /// 练习：配置摘要
        pub fn summary(&self) {
            println!("\n配置摘要:");

            for (key, value) in &self.config {
                let type_name = match value {
                    ConfigValue::String(_) => "字符串",
                    ConfigValue::Integer(_) => "整数",
                    ConfigValue::Float(_) => "浮点数",
                    ConfigValue::Boolean(_) => "布尔值",
                    ConfigValue::Array(_) => "数组",
                    ConfigValue::Object(_) => "对象",
                };
                println!("  {}: {} = {:?}", key, type_name, value);
            }
        }
    }

    /// 练习函数：配置解析演示
    pub fn config_parsing_demo() {
        println!("\n=== 练习6：配置文件解析器 ===");

        let mut parser = ConfigParser::new();

        // 设置配置
        parser.set(
            "database_url".to_string(),
            ConfigValue::String("postgresql://localhost/mydb".to_string()),
        );
        parser.set("port".to_string(), ConfigValue::Integer(8080));
        parser.set("debug".to_string(), ConfigValue::Boolean(true));
        parser.set("timeout".to_string(), ConfigValue::Float(30.5));

        // 设置嵌套配置
        let mut db_config = HashMap::new();
        db_config.insert(
            "host".to_string(),
            ConfigValue::String("localhost".to_string()),
        );
        db_config.insert("port".to_string(), ConfigValue::Integer(5432));
        parser.set("database".to_string(), ConfigValue::Object(db_config));

        // 测试配置获取
        match parser.get_string("database_url") {
            Ok(url) => println!("数据库 URL: {}", url),
            Err(e) => println!("获取数据库 URL 失败: {:?}", e),
        }

        match parser.get_number("port") {
            Ok(port) => println!("端口: {}", port),
            Err(e) => println!("获取端口失败: {:?}", e),
        }

        // 测试嵌套配置
        match parser.get_nested_value("database.host") {
            Ok(value) => println!("数据库主机: {:?}", value),
            Err(e) => println!("获取数据库主机失败: {:?}", e),
        }

        // 验证配置
        match parser.validate_config() {
            Ok(()) => println!("配置验证通过"),
            Err(errors) => {
                println!("配置验证失败:");
                for error in errors {
                    println!("  - {:?}", error);
                }
            }
        }

        parser.summary();
    }
}

/// 测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_light() {
        use exercise1_basic_match::TrafficLight;

        let red = TrafficLight::Red;
        assert_eq!(red.next(), TrafficLight::Green);
        assert_eq!(red.wait_time(), 30);
        assert!(!red.can_pass());

        let green = TrafficLight::Green;
        assert!(green.can_pass());
    }

    #[test]
    fn test_shape_area() {
        use exercise2_enum_with_data::Shape;

        let circle = Shape::Circle { radius: 5.0 };
        let area = circle.area();
        assert!((area - 78.54).abs() < 0.1);

        let square = Shape::Square { side: 4.0 };
        assert_eq!(square.area(), 16.0);
    }

    #[test]
    fn test_student_manager() {
        use exercise3_if_let_option::*;

        let mut manager = StudentManager::new();
        manager.add_student(Student {
            name: "Test".to_string(),
            age: 20,
            grade: Some('A'),
            subjects: vec![],
        });

        assert!(manager.find_student("Test").is_some());
        assert!(manager.find_student("NotFound").is_none());
        assert_eq!(manager.count_graded_students(), 1);
    }

    #[test]
    fn test_matches_macro() {
        use exercise5_matches_macro::*;

        let request = HttpRequest {
            method: HttpMethod::Get,
            path: "/api/test".to_string(),
            status_code: Some(200),
            body_size: 512,
        };

        assert!(matches!(request.method, HttpMethod::Get | HttpMethod::Post));
        assert!(request
            .status_code
            .map_or(false, |code| matches!(code, 200..=299)));
    }

    #[test]
    fn test_config_parser() {
        use exercise6_config_parser::*;

        let mut parser = ConfigParser::new();
        parser.set("test".to_string(), ConfigValue::String("value".to_string()));

        assert_eq!(parser.get_string("test").unwrap(), "value");
        assert!(parser.get_string("missing").is_err());
    }
}
