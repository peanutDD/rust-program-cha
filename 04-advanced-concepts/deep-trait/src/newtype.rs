//! Newtype 模式模块
//! 
//! 本模块展示了 Rust 中 Newtype 模式的定义和使用，
//! 包括基础用法、类型安全应用、为外部类型实现特征和高级应用。

use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Deref;

/// 基础 newtype 模式演示
pub fn basic_newtype_demo() {
    println!("\n--- 5.1 基础 newtype 模式 ---");

    // 基础 newtype 包装
    pub struct Meters(f64);
    pub struct Kilometers(f64);

    impl Meters {
        pub fn new(value: f64) -> Self {
            Meters(value)
        }

        pub fn value(&self) -> f64 {
            self.0
        }

        pub fn to_kilometers(&self) -> Kilometers {
            Kilometers(self.0 / 1000.0)
        }
    }

    impl Kilometers {
        pub fn new(value: f64) -> Self {
            Kilometers(value)
        }

        pub fn value(&self) -> f64 {
            self.0
        }

        pub fn to_meters(&self) -> Meters {
            Meters(self.0 * 1000.0)
        }
    }

    // 类型安全的距离计算
    let distance_m = Meters::new(1500.0);
    let distance_km = distance_m.to_kilometers();

    println!(
        "Distance: {:.1} meters = {:.2} kilometers",
        distance_m.value(),
        distance_km.value()
    );

    // 防止类型混淆
    fn calculate_speed(distance: Meters, time_seconds: f64) -> f64 {
        distance.value() / time_seconds
    }

    let speed = calculate_speed(distance_m, 60.0);
    println!("Speed: {:.2} m/s", speed);
}

/// 类型安全的 newtype 演示
pub fn type_safe_newtype_demo() {
    println!("\n--- 5.2 类型安全的 newtype ---");

    // 用户 ID 和订单 ID 的类型安全包装
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct UserId(u64);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct OrderId(u64);

    impl UserId {
        pub fn new(id: u64) -> Self {
            UserId(id)
        }

        pub fn value(&self) -> u64 {
            self.0
        }
    }

    impl OrderId {
        pub fn new(id: u64) -> Self {
            OrderId(id)
        }

        pub fn value(&self) -> u64 {
            self.0
        }
    }

    // 类型安全的函数
    fn get_user_orders(user_id: UserId) -> Vec<OrderId> {
        println!("Getting orders for user {}", user_id.value());
        vec![OrderId::new(1001), OrderId::new(1002), OrderId::new(1003)]
    }

    fn get_order_details(order_id: OrderId) -> String {
        format!("Order {} details", order_id.value())
    }

    let user_id = UserId::new(42);
    let order_id = OrderId::new(1001);

    let orders = get_user_orders(user_id);
    println!("User orders: {:?}", orders);

    let details = get_order_details(order_id);
    println!("Order details: {}", details);

    // 密码和令牌的安全包装
    #[derive(Debug)]
    pub struct Password(String);

    #[derive(Debug)]
    pub struct Token(String);

    impl Password {
        pub fn new(password: String) -> Self {
            Password(password)
        }

        pub fn verify(&self, input: &str) -> bool {
            self.0 == input
        }

        // 不提供直接访问密码的方法，增强安全性
    }

    impl Token {
        pub fn new(token: String) -> Self {
            Token(token)
        }

        pub fn is_valid(&self) -> bool {
            !self.0.is_empty() && self.0.len() > 10
        }

        pub fn value(&self) -> &str {
            &self.0
        }
    }

    let password = Password::new("secret123".to_string());
    let token = Token::new("abc123def456ghi789".to_string());

    println!("Password verification: {}", password.verify("secret123"));
    println!("Token is valid: {}", token.is_valid());
    println!("Token value: {}", token.value());
}

/// 为外部类型实现特征
pub fn external_type_traits_demo() {
    println!("\n--- 5.3 为外部类型实现特征 ---");

    // 为 Vec<String> 实现自定义特征（通过 newtype 模式绕过孤儿规则）
    pub struct Wrapper(Vec<String>);

    trait Summary {
        fn summarize(&self) -> String;
    }

    impl Summary for Wrapper {
        fn summarize(&self) -> String {
            format!("List with {} items: [{}]", self.0.len(), self.0.join(", "))
        }
    }

    impl Deref for Wrapper {
        type Target = Vec<String>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl From<Vec<String>> for Wrapper {
        fn from(vec: Vec<String>) -> Self {
            Wrapper(vec)
        }
    }

    let items = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
    ];

    let wrapper = Wrapper::from(items);
    println!("Summary: {}", wrapper.summarize());
    println!("First item: {}", wrapper[0]); // 通过 Deref 可以像 Vec 一样使用
    println!("Length: {}", wrapper.len());

    // 为 HashMap 实现自定义特征
    pub struct ConfigMap(HashMap<String, String>);

    pub trait Configuration {
        fn get_config(&self, key: &str) -> Option<&String>;
        fn set_config(&mut self, key: String, value: String);
        fn has_config(&self, key: &str) -> bool;
        fn list_configs(&self) -> Vec<(&String, &String)>;
    }

    impl Configuration for ConfigMap {
        fn get_config(&self, key: &str) -> Option<&String> {
            self.0.get(key)
        }

        fn set_config(&mut self, key: String, value: String) {
            self.0.insert(key, value);
        }

        fn has_config(&self, key: &str) -> bool {
            self.0.contains_key(key)
        }

        fn list_configs(&self) -> Vec<(&String, &String)> {
            self.0.iter().collect()
        }
    }

    impl Default for ConfigMap {
        fn default() -> Self {
            ConfigMap(HashMap::new())
        }
    }

    let mut config = ConfigMap::default();
    config.set_config("database_url".to_string(), "localhost:5432".to_string());
    config.set_config("api_key".to_string(), "secret_key_123".to_string());

    println!("\nConfiguration:");
    for (key, value) in config.list_configs() {
        println!("  {}: {}", key, value);
    }

    if let Some(db_url) = config.get_config("database_url") {
        println!("Database URL: {}", db_url);
    }

    println!("Has API key: {}", config.has_config("api_key"));
    println!("Has password: {}", config.has_config("password"));
}

/// 高级 newtype 应用
pub fn advanced_newtype_demo() {
    println!("\n--- 5.4 高级 newtype 应用 ---");

    // 状态机模式中的 newtype
    pub struct Locked;
    pub struct Unlocked;

    pub struct StateMachine<State> {
        data: String,
        _state: PhantomData<State>,
    }

    impl StateMachine<Locked> {
        pub fn new(data: String) -> Self {
            StateMachine {
                data,
                _state: PhantomData,
            }
        }

        pub fn unlock(self, password: &str) -> Result<StateMachine<Unlocked>, Self> {
            if password == "correct_password" {
                Ok(StateMachine {
                    data: self.data,
                    _state: PhantomData,
                })
            } else {
                Err(self)
            }
        }
    }

    impl StateMachine<Unlocked> {
        pub fn read_data(&self) -> &str {
            &self.data
        }

        pub fn write_data(&mut self, new_data: String) {
            self.data = new_data;
        }

        pub fn lock(self) -> StateMachine<Locked> {
            StateMachine {
                data: self.data,
                _state: PhantomData,
            }
        }
    }

    // 使用状态机
    let locked_machine = StateMachine::<Locked>::new("Secret data".to_string());

    // 尝试解锁
    match locked_machine.unlock("wrong_password") {
        Ok(_) => println!("Unlocked successfully"),
        Err(machine) => {
            println!("Failed to unlock, trying again...");
            match machine.unlock("correct_password") {
                Ok(mut unlocked) => {
                    println!("Unlocked successfully!");
                    println!("Data: {}", unlocked.read_data());
                    unlocked.write_data("Modified data".to_string());
                    println!("Modified data: {}", unlocked.read_data());

                    let _locked_again = unlocked.lock();
                    println!("Machine locked again");
                }
                Err(_) => println!("Still failed to unlock"),
            }
        }
    }

    // 单位系统的 newtype
    #[derive(Debug, Clone, Copy)]
    pub struct Celsius(f64);

    #[derive(Debug, Clone, Copy)]
    pub struct Fahrenheit(f64);

    #[derive(Debug, Clone, Copy)]
    pub struct Kelvin(f64);

    impl Celsius {
        pub fn new(temp: f64) -> Self {
            Celsius(temp)
        }

        pub fn to_fahrenheit(self) -> Fahrenheit {
            Fahrenheit(self.0 * 9.0 / 5.0 + 32.0)
        }

        pub fn to_kelvin(self) -> Kelvin {
            Kelvin(self.0 + 273.15)
        }

        pub fn value(self) -> f64 {
            self.0
        }
    }

    impl Fahrenheit {
        pub fn new(temp: f64) -> Self {
            Fahrenheit(temp)
        }

        pub fn to_celsius(self) -> Celsius {
            Celsius((self.0 - 32.0) * 5.0 / 9.0)
        }

        pub fn to_kelvin(self) -> Kelvin {
            self.to_celsius().to_kelvin()
        }

        pub fn value(self) -> f64 {
            self.0
        }
    }

    impl Kelvin {
        pub fn new(temp: f64) -> Self {
            Kelvin(temp)
        }

        pub fn to_celsius(self) -> Celsius {
            Celsius(self.0 - 273.15)
        }

        pub fn to_fahrenheit(self) -> Fahrenheit {
            self.to_celsius().to_fahrenheit()
        }

        pub fn value(self) -> f64 {
            self.0
        }
    }

    // 温度转换示例 - 使用链式调用，更简洁
    println!("\nTemperature conversions:");
    let temp_c = Celsius::new(25.0);
    println!(
        "{:.1}°C = {:.1}°F = {:.1}K",
        temp_c.value(),
        temp_c.to_fahrenheit().value(), // 直接链式调用，避免临时变量
        temp_c.to_kelvin().value()
    );

    // 展示反向转换
    println!("{:.1}°F = {:.1}°C", 
        Fahrenheit::new(100.0).value(), 
        Fahrenheit::new(100.0).to_celsius().value()
    );

    // 添加绝对零度验证
    let absolute_zero_c = Celsius::new(-273.15);
    println!("Absolute zero: {:.2}°C = {:.2}K", 
        absolute_zero_c.value(), 
        absolute_zero_c.to_kelvin().value()
    );
}

/// Newtype 模式主入口函数
pub fn newtype_pattern_analysis() {
    println!("\n=== 5. newtype 模式分析 ===");
    
    basic_newtype_demo();
    type_safe_newtype_demo();
    external_type_traits_demo();
    advanced_newtype_demo();
}