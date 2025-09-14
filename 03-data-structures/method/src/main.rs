/*
 * Rust 方法 (Method) 全面深入分析
 * 基于 https://course.rs/basic/method.html
 *
 * 本文档全面分析 Rust 中的方法概念，包括：
 * 1. 方法的基本概念和语法
 * 2. 方法与函数的区别
 * 3. self 参数的三种形式
 * 4. 关联函数 (Associated Functions)
 * 5. 多个 impl 块
 * 6. 方法链 (Method Chaining)
 * 7. 方法的可见性
 * 8. 泛型方法
 * 9. 特征方法
 * 10. 实际应用场景
 */

use std::collections::HashMap;
use std::fmt;

fn main() {
    println!("=== Rust 方法 (Method) 全面深入分析 ===");

    // 1. 基本方法演示
    basic_method_demo();

    // 2. self 参数形式演示
    self_parameter_demo();

    // 3. 关联函数演示
    associated_function_demo();

    // 4. 多个 impl 块演示
    multiple_impl_demo();

    // 5. 方法链演示
    method_chaining_demo();

    // 6. 方法可见性演示
    method_visibility_demo();

    // 7. 泛型方法演示
    generic_method_demo();

    // 8. 特征方法演示
    trait_method_demo();

    // 9. 实际应用场景演示
    real_world_applications_demo();

    println!("\n=== 方法分析完成 ===");
}

// 1. 基本方法演示
fn basic_method_demo() {
    println!("\n--- 1. 基本方法演示 ---");

    // 定义一个基本的结构体
    #[derive(Debug, Clone)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // 为 Rectangle 实现方法
    impl Rectangle {
        // 计算面积的方法
        fn area(&self) -> u32 {
            self.width * self.height
        }

        // 计算周长的方法
        fn perimeter(&self) -> u32 {
            2 * (self.width + self.height)
        }

        // 判断是否为正方形
        fn is_square(&self) -> bool {
            self.width == self.height
        }

        // 缩放方法（修改自身）
        fn scale(&mut self, factor: u32) {
            self.width *= factor;
            self.height *= factor;
        }

        // 创建缩放后的新实例
        fn scaled(self, factor: u32) -> Rectangle {
            Rectangle {
                width: self.width * factor,
                height: self.height * factor,
            }
        }
    }

    let mut rect = Rectangle {
        width: 10,
        height: 20,
    };
    println!("原始矩形: {:?}", rect);
    println!("面积: {}", rect.area());
    println!("周长: {}", rect.perimeter());
    println!("是否为正方形: {}", rect.is_square());

    // 修改原实例
    rect.scale(2);
    println!("缩放后的矩形: {:?}", rect);

    // 创建新实例
    let new_rect = rect.clone().scaled(3);
    println!("新缩放的矩形: {:?}", new_rect);
}

// 2. self 参数形式演示
fn self_parameter_demo() {
    println!("\n--- 2. self 参数形式演示 ---");

    #[derive(Debug, Clone)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        // &self - 不可变借用，最常用的形式
        fn distance_from_origin(&self) -> f64 {
            (self.x * self.x + self.y * self.y).sqrt()
        }

        // &mut self - 可变借用，用于修改实例
        fn translate(&mut self, dx: f64, dy: f64) {
            self.x += dx;
            self.y += dy;
        }

        // self - 获取所有权，消费实例
        fn into_tuple(self) -> (f64, f64) {
            (self.x, self.y)
        }

        // 演示不同 self 形式的使用场景
        fn normalize(&mut self) {
            let length = self.distance_from_origin();
            if length != 0.0 {
                self.x /= length;
                self.y /= length;
            }
        }

        // 返回标准化后的新点（不修改原点）
        fn normalized(&self) -> Point {
            let length = self.distance_from_origin();
            if length != 0.0 {
                Point {
                    x: self.x / length,
                    y: self.y / length,
                }
            } else {
                Point { x: 0.0, y: 0.0 }
            }
        }
    }

    let mut point = Point { x: 3.0, y: 4.0 };
    println!("原始点: {:?}", point);
    println!("到原点距离: {:.2}", point.distance_from_origin());

    // 使用 &mut self
    point.translate(1.0, 1.0);
    println!("平移后的点: {:?}", point);

    // 使用 &self 创建新实例
    let normalized_point = point.normalized();
    println!("标准化的点: {:?}", normalized_point);

    // 使用 &mut self 修改原实例
    point.normalize();
    println!("原地标准化后的点: {:?}", point);

    // 使用 self 消费实例
    let tuple = point.into_tuple();
    println!("转换为元组: {:?}", tuple);
    // 注意：point 在这里已经被移动，不能再使用
}

// 3. 关联函数演示
fn associated_function_demo() {
    println!("\n--- 3. 关联函数演示 ---");

    #[derive(Debug)]
    struct Circle {
        radius: f64,
    }

    impl Circle {
        // 关联函数 - 构造函数
        fn new(radius: f64) -> Circle {
            Circle { radius }
        }

        // 关联函数 - 从直径创建
        fn from_diameter(diameter: f64) -> Circle {
            Circle {
                radius: diameter / 2.0,
            }
        }

        // 关联函数 - 单位圆
        fn unit_circle() -> Circle {
            Circle { radius: 1.0 }
        }

        // 关联函数 - 验证半径有效性
        fn is_valid_radius(radius: f64) -> bool {
            radius > 0.0
        }

        // 实例方法
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }

        fn circumference(&self) -> f64 {
            2.0 * std::f64::consts::PI * self.radius
        }
    }

    // 使用关联函数创建实例
    let circle1 = Circle::new(5.0);
    let circle2 = Circle::from_diameter(10.0);
    let unit_circle = Circle::unit_circle();

    println!("圆1: {:?}, 面积: {:.2}", circle1, circle1.area());
    println!("圆2: {:?}, 周长: {:.2}", circle2, circle2.circumference());
    println!("单位圆: {:?}", unit_circle);

    // 使用关联函数进行验证
    println!("半径 5.0 是否有效: {}", Circle::is_valid_radius(5.0));
    println!("半径 -1.0 是否有效: {}", Circle::is_valid_radius(-1.0));
}

// 4. 多个 impl 块演示
fn multiple_impl_demo() {
    println!("\n--- 4. 多个 impl 块演示 ---");

    #[derive(Debug)]
    struct Calculator {
        value: f64,
    }

    // 第一个 impl 块 - 基本运算
    impl Calculator {
        fn new(value: f64) -> Calculator {
            Calculator { value }
        }

        fn add(&mut self, other: f64) -> &mut Self {
            self.value += other;
            self
        }

        fn subtract(&mut self, other: f64) -> &mut Self {
            self.value -= other;
            self
        }
    }

    // 第二个 impl 块 - 高级运算
    impl Calculator {
        fn multiply(&mut self, other: f64) -> &mut Self {
            self.value *= other;
            self
        }

        fn divide(&mut self, other: f64) -> &mut Self {
            if other != 0.0 {
                self.value /= other;
            }
            self
        }

        fn power(&mut self, exp: f64) -> &mut Self {
            self.value = self.value.powf(exp);
            self
        }
    }

    // 第三个 impl 块 - 工具方法
    impl Calculator {
        fn reset(&mut self) -> &mut Self {
            self.value = 0.0;
            self
        }

        fn get_value(&self) -> f64 {
            self.value
        }

        fn abs(&mut self) -> &mut Self {
            self.value = self.value.abs();
            self
        }
    }

    let mut calc = Calculator::new(10.0);
    println!("初始值: {}", calc.get_value());

    // 链式调用
    calc.add(5.0).multiply(2.0).subtract(10.0);
    println!("计算后的值: {}", calc.get_value());

    calc.power(2.0).abs();
    println!("平方并取绝对值: {}", calc.get_value());
}

// 5. 方法链演示
fn method_chaining_demo() {
    println!("\n--- 5. 方法链演示 ---");

    #[derive(Debug, Clone)]
    struct StringBuilder {
        content: String,
    }

    impl StringBuilder {
        fn new() -> StringBuilder {
            StringBuilder {
                content: String::new(),
            }
        }

        fn append(&mut self, text: &str) -> &mut Self {
            self.content.push_str(text);
            self
        }

        fn append_line(&mut self, text: &str) -> &mut Self {
            self.content.push_str(text);
            self.content.push('\n');
            self
        }

        fn append_char(&mut self, ch: char) -> &mut Self {
            self.content.push(ch);
            self
        }

        fn clear(&mut self) -> &mut Self {
            self.content.clear();
            self
        }

        fn to_uppercase(&mut self) -> &mut Self {
            self.content = self.content.to_uppercase();
            self
        }

        fn build(&self) -> String {
            self.content.clone()
        }

        fn len(&self) -> usize {
            self.content.len()
        }
    }

    let mut builder = StringBuilder::new();

    // 演示方法链的强大功能
    let result = builder
        .append("Hello, ")
        .append("World!")
        .append_line("")
        .append_line("This is a test.")
        .append_char('!')
        .build();

    println!("构建的字符串:\n{}", result);
    println!("字符串长度: {}", builder.len());

    // 继续链式操作
    builder.clear().append("New content").to_uppercase();
    println!("清空并重新构建: {}", builder.build());
}

// 6. 方法可见性演示
fn method_visibility_demo() {
    println!("\n--- 6. 方法可见性演示 ---");

    #[derive(Debug)]
    pub struct BankAccount {
        account_number: String,
        balance: f64,
    }

    impl BankAccount {
        // 公共构造函数
        pub fn new(account_number: String, initial_balance: f64) -> BankAccount {
            BankAccount {
                account_number,
                balance: initial_balance,
            }
        }

        // 公共方法 - 查询余额
        pub fn get_balance(&self) -> f64 {
            self.balance
        }

        // 公共方法 - 存款
        pub fn deposit(&mut self, amount: f64) -> Result<(), String> {
            if self.is_valid_amount(amount) {
                self.balance += amount;
                self.log_transaction("deposit", amount);
                Ok(())
            } else {
                Err("无效的存款金额".to_string())
            }
        }

        // 公共方法 - 取款
        pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
            if !self.is_valid_amount(amount) {
                return Err("无效的取款金额".to_string());
            }

            if self.has_sufficient_funds(amount) {
                self.balance -= amount;
                self.log_transaction("withdraw", amount);
                Ok(())
            } else {
                Err("余额不足".to_string())
            }
        }

        // 私有方法 - 验证金额
        fn is_valid_amount(&self, amount: f64) -> bool {
            amount > 0.0 && amount.is_finite()
        }

        // 私有方法 - 检查余额是否充足
        fn has_sufficient_funds(&self, amount: f64) -> bool {
            self.balance >= amount
        }

        // 私有方法 - 记录交易
        fn log_transaction(&self, transaction_type: &str, amount: f64) {
            println!(
                "账户 {}: {} ${:.2}, 当前余额: ${:.2}",
                self.account_number, transaction_type, amount, self.balance
            );
        }
    }

    let mut account = BankAccount::new("ACC001".to_string(), 1000.0);
    println!("初始余额: ${:.2}", account.get_balance());

    // 使用公共方法
    match account.deposit(500.0) {
        Ok(_) => println!("存款成功"),
        Err(e) => println!("存款失败: {}", e),
    }

    match account.withdraw(200.0) {
        Ok(_) => println!("取款成功"),
        Err(e) => println!("取款失败: {}", e),
    }

    // 尝试无效操作
    match account.withdraw(2000.0) {
        Ok(_) => println!("取款成功"),
        Err(e) => println!("取款失败: {}", e),
    }
}

// 7. 泛型方法演示
fn generic_method_demo() {
    println!("\n--- 7. 泛型方法演示 ---");

    #[derive(Debug)]
    struct Container<T> {
        items: Vec<T>,
    }

    impl<T> Container<T> {
        fn new() -> Container<T> {
            Container { items: Vec::new() }
        }

        fn add(&mut self, item: T) {
            self.items.push(item);
        }

        fn len(&self) -> usize {
            self.items.len()
        }

        fn is_empty(&self) -> bool {
            self.items.is_empty()
        }

        fn get(&self, index: usize) -> Option<&T> {
            self.items.get(index)
        }
    }

    // 为特定类型实现额外方法
    impl Container<i32> {
        fn sum(&self) -> i32 {
            self.items.iter().sum()
        }

        fn average(&self) -> Option<f64> {
            if self.items.is_empty() {
                None
            } else {
                Some(self.sum() as f64 / self.items.len() as f64)
            }
        }
    }

    impl Container<String> {
        fn join(&self, separator: &str) -> String {
            self.items.join(separator)
        }

        fn total_length(&self) -> usize {
            self.items.iter().map(|s| s.len()).sum()
        }
    }

    // 泛型方法 - 适用于所有类型
    impl<T: Clone> Container<T> {
        fn duplicate_last(&mut self) {
            if let Some(last) = self.items.last() {
                let cloned = last.clone();
                self.items.push(cloned);
            }
        }
    }

    // 使用整数容器
    let mut int_container = Container::new();
    int_container.add(1);
    int_container.add(2);
    int_container.add(3);

    println!("整数容器: {:?}", int_container);
    println!("总和: {}", int_container.sum());
    println!("平均值: {:?}", int_container.average());

    int_container.duplicate_last();
    println!("复制最后一个元素后: {:?}", int_container);

    // 使用字符串容器
    let mut string_container = Container::new();
    string_container.add("Hello".to_string());
    string_container.add("World".to_string());
    string_container.add("Rust".to_string());

    println!("字符串容器: {:?}", string_container);
    println!("连接结果: {}", string_container.join(" "));
    println!("总长度: {}", string_container.total_length());
}

// 8. 特征方法演示
fn trait_method_demo() {
    println!("\n--- 8. 特征方法演示 ---");

    // 定义一个图形特征
    trait Shape {
        fn area(&self) -> f64;
        fn perimeter(&self) -> f64;

        // 默认实现
        fn describe(&self) -> String {
            format!("面积: {:.2}, 周长: {:.2}", self.area(), self.perimeter())
        }

        // 关联函数
        fn shape_type() -> &'static str
        where
            Self: Sized,
        {
            "未知图形"
        }
    }

    // 可绘制特征
    trait Drawable {
        fn draw(&self);
        fn set_color(&mut self, color: &str);
    }

    #[derive(Debug)]
    struct Rectangle {
        width: f64,
        height: f64,
        color: String,
    }

    impl Rectangle {
        fn new(width: f64, height: f64) -> Rectangle {
            Rectangle {
                width,
                height,
                color: "black".to_string(),
            }
        }
    }

    impl Shape for Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }

        fn perimeter(&self) -> f64 {
            2.0 * (self.width + self.height)
        }

        fn shape_type() -> &'static str {
            "矩形"
        }
    }

    impl Drawable for Rectangle {
        fn draw(&self) {
            println!("绘制{}色矩形: {}x{}", self.color, self.width, self.height);
        }

        fn set_color(&mut self, color: &str) {
            self.color = color.to_string();
        }
    }

    #[derive(Debug)]
    struct Circle {
        radius: f64,
        color: String,
    }

    impl Circle {
        fn new(radius: f64) -> Circle {
            Circle {
                radius,
                color: "black".to_string(),
            }
        }
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }

        fn perimeter(&self) -> f64 {
            2.0 * std::f64::consts::PI * self.radius
        }

        fn shape_type() -> &'static str {
            "圆形"
        }
    }

    impl Drawable for Circle {
        fn draw(&self) {
            println!("绘制{}色圆形: 半径{}", self.color, self.radius);
        }

        fn set_color(&mut self, color: &str) {
            self.color = color.to_string();
        }
    }

    // 使用特征方法
    let mut rect = Rectangle::new(10.0, 5.0);
    let mut circle = Circle::new(3.0);

    println!("矩形类型: {}", Rectangle::shape_type());
    println!("矩形描述: {}", rect.describe());
    rect.set_color("红");
    rect.draw();

    println!("\n圆形类型: {}", Circle::shape_type());
    println!("圆形描述: {}", circle.describe());
    circle.set_color("蓝");
    circle.draw();

    // 多态使用
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle::new(4.0, 6.0)),
        Box::new(Circle::new(2.5)),
    ];

    println!("\n多态演示:");
    for (i, shape) in shapes.iter().enumerate() {
        println!("图形 {}: {}", i + 1, shape.describe());
    }
}

// 9. 实际应用场景演示
fn real_world_applications_demo() {
    println!("\n--- 9. 实际应用场景演示 ---");

    // 场景1: HTTP 客户端构建器模式
    #[derive(Debug)]
    struct HttpClient {
        base_url: String,
        timeout: u64,
        headers: HashMap<String, String>,
        user_agent: String,
    }

    impl HttpClient {
        fn new(base_url: &str) -> HttpClient {
            HttpClient {
                base_url: base_url.to_string(),
                timeout: 30,
                headers: HashMap::new(),
                user_agent: "RustClient/1.0".to_string(),
            }
        }

        fn timeout(&mut self, seconds: u64) -> &mut Self {
            self.timeout = seconds;
            self
        }

        fn header(&mut self, key: &str, value: &str) -> &mut Self {
            self.headers.insert(key.to_string(), value.to_string());
            self
        }

        fn user_agent(&mut self, agent: &str) -> &mut Self {
            self.user_agent = agent.to_string();
            self
        }

        fn get(&self, path: &str) -> String {
            format!(
                "GET {}{} (超时: {}s, 头部: {:?})",
                self.base_url, path, self.timeout, self.headers
            )
        }

        fn post(&self, path: &str, body: &str) -> String {
            format!(
                "POST {}{} 数据: {} (用户代理: {})",
                self.base_url, path, body, self.user_agent
            )
        }
    }

    let mut client = HttpClient::new("https://api.example.com");
    client
        .timeout(60)
        .header("Authorization", "Bearer token123")
        .header("Content-Type", "application/json")
        .user_agent("MyApp/2.0");

    println!("HTTP 客户端配置: {:?}", client);
    println!("{}", client.get("/users"));
    println!("{}", client.post("/users", "{\"name\": \"Alice\"}"));

    // 场景2: 数据库查询构建器
    #[derive(Debug)]
    struct QueryBuilder {
        table: String,
        select_fields: Vec<String>,
        where_conditions: Vec<String>,
        order_by: Vec<String>,
        limit: Option<usize>,
    }

    impl QueryBuilder {
        fn new(table: &str) -> QueryBuilder {
            QueryBuilder {
                table: table.to_string(),
                select_fields: vec!["*".to_string()],
                where_conditions: Vec::new(),
                order_by: Vec::new(),
                limit: None,
            }
        }

        fn select(&mut self, fields: &[&str]) -> &mut Self {
            self.select_fields = fields.iter().map(|s| s.to_string()).collect();
            self
        }

        fn where_clause(&mut self, condition: &str) -> &mut Self {
            self.where_conditions.push(condition.to_string());
            self
        }

        fn order_by(&mut self, field: &str, direction: &str) -> &mut Self {
            self.order_by.push(format!("{} {}", field, direction));
            self
        }

        fn limit(&mut self, count: usize) -> &mut Self {
            self.limit = Some(count);
            self
        }

        fn build(&self) -> String {
            let mut query = format!(
                "SELECT {} FROM {}",
                self.select_fields.join(", "),
                self.table
            );

            if !self.where_conditions.is_empty() {
                query.push_str(&format!(" WHERE {}", self.where_conditions.join(" AND ")));
            }

            if !self.order_by.is_empty() {
                query.push_str(&format!(" ORDER BY {}", self.order_by.join(", ")));
            }

            if let Some(limit) = self.limit {
                query.push_str(&format!(" LIMIT {}", limit));
            }

            query
        }
    }

    let mut query = QueryBuilder::new("users");
    let sql = query
        .select(&["id", "name", "email"])
        .where_clause("age > 18")
        .where_clause("status = 'active'")
        .order_by("name", "ASC")
        .limit(10)
        .build();

    println!("\n生成的 SQL 查询: {}", sql);

    // 场景3: 配置管理器
    #[derive(Debug, Clone)]
    struct Config {
        database_url: Option<String>,
        port: u16,
        debug: bool,
        features: Vec<String>,
    }

    impl Config {
        fn new() -> Config {
            Config {
                database_url: None,
                port: 8080,
                debug: false,
                features: Vec::new(),
            }
        }

        fn database_url(&mut self, url: &str) -> &mut Self {
            self.database_url = Some(url.to_string());
            self
        }

        fn port(&mut self, port: u16) -> &mut Self {
            self.port = port;
            self
        }

        fn enable_debug(&mut self) -> &mut Self {
            self.debug = true;
            self
        }

        fn add_feature(&mut self, feature: &str) -> &mut Self {
            self.features.push(feature.to_string());
            self
        }

        fn validate(&self) -> Result<(), String> {
            if self.database_url.is_none() {
                return Err("数据库 URL 未设置".to_string());
            }

            if self.port == 0 {
                return Err("端口号无效".to_string());
            }

            Ok(())
        }

        fn summary(&self) -> String {
            format!(
                "配置摘要: 端口={}, 调试={}, 功能数量={}",
                self.port,
                self.debug,
                self.features.len()
            )
        }
    }

    let mut config = Config::new();
    config
        .database_url("postgresql://localhost/mydb")
        .port(3000)
        .enable_debug()
        .add_feature("authentication")
        .add_feature("logging")
        .add_feature("metrics");

    println!("\n应用配置: {:?}", config);
    println!("{}", config.summary());

    match config.validate() {
        Ok(_) => println!("配置验证通过"),
        Err(e) => println!("配置验证失败: {}", e),
    }
}

// 额外演示：方法的高级特性
#[allow(dead_code)]
mod advanced_features {
    use std::fmt;

    // 演示方法的生命周期参数
    #[derive(Debug)]
    pub struct TextProcessor<'a> {
        text: &'a str,
    }

    impl<'a> TextProcessor<'a> {
        pub fn new(text: &'a str) -> TextProcessor<'a> {
            TextProcessor { text }
        }

        // 返回引用的方法需要生命周期标注
        pub fn get_first_word(&self) -> Option<&'a str> {
            self.text.split_whitespace().next()
        }

        pub fn get_last_word(&self) -> Option<&'a str> {
            self.text.split_whitespace().last()
        }

        // 方法可以有多个生命周期参数
        pub fn find_common_prefix(&self, other: &str) -> String {
            let mut common = String::new();
            for (c1, c2) in self.text.chars().zip(other.chars()) {
                if c1 == c2 {
                    common.push(c1);
                } else {
                    break;
                }
            }
            common
        }
    }

    // 演示 Display 特征的实现
    impl<'a> fmt::Display for TextProcessor<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TextProcessor(\"{}\")", self.text)
        }
    }

    // 演示方法的条件编译
    #[derive(Debug)]
    pub struct DebugHelper {
        name: String,
    }

    impl DebugHelper {
        pub fn new(name: &str) -> DebugHelper {
            DebugHelper {
                name: name.to_string(),
            }
        }

        // 只在调试模式下可用的方法
        #[cfg(debug_assertions)]
        pub fn debug_info(&self) -> String {
            format!("调试信息: {}", self.name)
        }

        // 只在发布模式下可用的方法
        #[cfg(not(debug_assertions))]
        pub fn debug_info(&self) -> String {
            "调试信息在发布模式下不可用".to_string()
        }

        // 平台特定的方法
        #[cfg(target_os = "macos")]
        pub fn platform_specific(&self) -> String {
            "macOS 特定功能".to_string()
        }

        #[cfg(not(target_os = "macos"))]
        pub fn platform_specific(&self) -> String {
            "通用平台功能".to_string()
        }
    }
}

// 总结和最佳实践
#[allow(dead_code)]
mod best_practices {
    /*
     * Rust 方法最佳实践总结：
     *
     * 1. 方法命名规范：
     *    - 使用 snake_case 命名
     *    - 动词开头表示操作（如 calculate_area）
     *    - 形容词开头表示查询（如 is_empty）
     *    - get_ 前缀用于简单的字段访问
     *
     * 2. self 参数选择：
     *    - &self: 只读访问，最常用
     *    - &mut self: 需要修改实例
     *    - self: 消费实例，转移所有权
     *
     * 3. 返回值设计：
     *    - 返回 &mut Self 支持方法链
     *    - 使用 Result<T, E> 处理可能失败的操作
     *    - 返回 Option<T> 处理可能为空的结果
     *
     * 4. 方法组织：
     *    - 相关方法放在同一个 impl 块
     *    - 可以使用多个 impl 块分组功能
     *    - 构造函数使用关联函数
     *
     * 5. 可见性控制：
     *    - 公共接口使用 pub
     *    - 内部实现细节保持私有
     *    - 使用 pub(crate) 限制模块内可见
     *
     * 6. 泛型和特征：
     *    - 合理使用泛型提高代码复用
     *    - 实现标准特征（Debug, Clone, PartialEq 等）
     *    - 为特定类型提供专门的方法
     *
     * 7. 错误处理：
     *    - 使用 Result 类型处理错误
     *    - 提供清晰的错误信息
     *    - 考虑使用自定义错误类型
     *
     * 8. 性能考虑：
     *    - 避免不必要的克隆
     *    - 合理使用引用和借用
     *    - 考虑内联小方法
     *
     * 9. 文档和测试：
     *    - 为公共方法编写文档注释
     *    - 提供使用示例
     *    - 编写单元测试
     */
}
