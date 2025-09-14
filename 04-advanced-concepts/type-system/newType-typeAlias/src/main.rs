//! # Rust NewType 模式与类型别名深度教程
//!
//! 基于 https://course.rs/advance/into-types/custom-type.html
//! 全面分析 newtype 模式和类型别名的区别、用途、优缺点及实际应用

use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, Deref};

fn main() {
    println!("=== Rust NewType 模式与类型别名深度教程 ===");

    // 1. 基础概念演示
    demonstrate_basic_concepts();

    // 2. NewType 模式详解
    demonstrate_newtype_pattern();

    // 3. 类型别名详解
    demonstrate_type_alias();

    // 4. 关键差异对比
    demonstrate_key_differences();

    // 5. 孤儿规则绕过
    demonstrate_orphan_rule_bypass();

    // 6. 实际应用场景
    demonstrate_real_world_scenarios();

    // 7. 高级模式
    demonstrate_advanced_patterns();

    // 8. 性能分析
    demonstrate_performance_analysis();

    // 9. 最佳实践
    demonstrate_best_practices();
}

/// 1. 基础概念演示
fn demonstrate_basic_concepts() {
    println!("\n=== 1. 基础概念演示 ===");

    // 问题：为什么需要自定义类型？
    println!("\n问题场景：使用原始类型的困扰");

    // 使用原始类型的问题
    fn calculate_distance_bad(meters: u32, kilometers: u32) -> u32 {
        meters + kilometers * 1000 // 容易混淆单位
    }

    let distance1 = 500u32; // 这是米还是千米？
    let distance2 = 2u32; // 这是米还是千米？
    let total = calculate_distance_bad(distance1, distance2);
    println!("原始类型计算结果: {} (单位不明确)", total);

    println!("\n解决方案：使用自定义类型提高可读性和类型安全性");
}

/// 2. NewType 模式详解
fn demonstrate_newtype_pattern() {
    println!("\n=== 2. NewType 模式详解 ===");

    // 2.1 基本 NewType 定义
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Meters(u32);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Kilometers(u32);

    println!("\n2.1 基本 NewType 定义:");
    let distance_m = Meters(500);
    let distance_km = Kilometers(2);
    println!("距离(米): {:?}", distance_m);
    println!("距离(千米): {:?}", distance_km);

    // 2.2 类型安全性演示
    println!("\n2.2 类型安全性演示:");

    fn calculate_distance_safe(meters: Meters, kilometers: Kilometers) -> Meters {
        Meters(meters.0 + kilometers.0 * 1000)
    }

    let total_safe = calculate_distance_safe(distance_m, distance_km);
    println!("类型安全计算结果: {:?}", total_safe);

    // 编译时错误演示（注释掉的代码会导致编译错误）
    // let wrong = calculate_distance_safe(distance_km, distance_m); // 编译错误！

    // 2.3 NewType 的零成本抽象
    println!("\n2.3 NewType 的零成本抽象:");
    println!("Meters 大小: {} bytes", std::mem::size_of::<Meters>());
    println!("u32 大小: {} bytes", std::mem::size_of::<u32>());
    println!("NewType 在运行时没有额外开销！");

    // 2.4 为 NewType 实现方法
    impl Meters {
        fn new(value: u32) -> Self {
            Meters(value)
        }

        fn as_kilometers(&self) -> f64 {
            self.0 as f64 / 1000.0
        }

        fn add_meters(&mut self, other: u32) {
            self.0 += other;
        }
    }

    println!("\n2.4 NewType 方法演示:");
    let mut distance = Meters::new(1500);
    println!("原始距离: {:?}", distance);
    println!("转换为千米: {:.2} km", distance.as_kilometers());
    distance.add_meters(500);
    println!("增加500米后: {:?}", distance);
}

/// 3. 类型别名详解
fn demonstrate_type_alias() {
    println!("\n=== 3. 类型别名详解 ===");

    // 3.1 基本类型别名
    type MetersAlias = u32;
    type KilometersAlias = u32;

    println!("\n3.1 基本类型别名:");
    let distance_m: MetersAlias = 500;
    let distance_km: KilometersAlias = 2;
    println!("距离(米别名): {}", distance_m);
    println!("距离(千米别名): {}", distance_km);

    // 3.2 类型别名的互换性问题
    println!("\n3.2 类型别名的互换性问题:");

    fn calculate_with_alias(meters: MetersAlias, kilometers: KilometersAlias) -> MetersAlias {
        meters + kilometers * 1000
    }

    // 类型别名可以互换使用（这可能导致错误）
    let result1 = calculate_with_alias(distance_m, distance_km);
    let result2 = calculate_with_alias(distance_km, distance_m); // 参数顺序错误但编译通过！

    println!("正确调用结果: {}", result1);
    println!("错误调用结果: {} (参数顺序错误但编译通过)", result2);

    // 3.3 复杂类型的别名
    println!("\n3.3 复杂类型的别名:");

    type UserDatabase = HashMap<String, (String, u32)>; // 用户ID -> (姓名, 年龄)
    type ResultType<T> = Result<T, Box<dyn std::error::Error>>;

    let mut users: UserDatabase = HashMap::new();
    users.insert("user1".to_string(), ("Alice".to_string(), 25));
    users.insert("user2".to_string(), ("Bob".to_string(), 30));

    println!("用户数据库: {:?}", users);

    // 3.4 泛型类型别名
    println!("\n3.4 泛型类型别名:");

    type GenericResult<T, E> = Result<T, E>;
    type StringResult = GenericResult<String, &'static str>;

    let success: StringResult = Ok("操作成功".to_string());
    let failure: StringResult = Err("操作失败");

    println!("成功结果: {:?}", success);
    println!("失败结果: {:?}", failure);
}

/// 4. 关键差异对比
fn demonstrate_key_differences() {
    println!("\n=== 4. NewType vs 类型别名关键差异对比 ===");

    // NewType 定义
    #[derive(Debug)]
    struct UserId(u32);

    #[derive(Debug)]
    struct ProductId(u32);

    // 类型别名定义
    type UserIdAlias = u32;
    type ProductIdAlias = u32;

    println!("\n4.1 类型安全性对比:");

    fn process_user_newtype(user_id: UserId) {
        println!("处理用户(NewType): {:?}", user_id);
    }

    fn process_user_alias(user_id: UserIdAlias) {
        println!("处理用户(别名): {}", user_id);
    }

    let user_id_nt = UserId(123);
    let product_id_nt = ProductId(456);
    let user_id_alias: UserIdAlias = 123;
    let product_id_alias: ProductIdAlias = 456;

    // NewType: 类型安全
    process_user_newtype(user_id_nt);
    // process_user_newtype(product_id_nt); // 编译错误！类型不匹配

    // 类型别名: 可以互换（潜在问题）
    process_user_alias(user_id_alias);
    process_user_alias(product_id_alias); // 编译通过，但逻辑错误！

    println!("\n4.2 编译时检查:");
    println!("NewType: 提供强类型检查，防止类型混淆");
    println!("类型别名: 仅提供可读性，无类型安全保障");

    println!("\n4.3 运行时性能:");
    println!("NewType: 零成本抽象，运行时无开销");
    println!("类型别名: 编译时完全展开，运行时等同于原类型");
}

/// 5. 孤儿规则绕过演示
fn demonstrate_orphan_rule_bypass() {
    println!("\n=== 5. 孤儿规则绕过演示 ===");

    println!("\n5.1 孤儿规则问题:");
    println!("无法为外部类型实现外部trait，如为 Vec<T> 实现 Display");

    // 5.2 使用 NewType 绕过孤儿规则
    #[derive(Debug)]
    struct PrettyVec<T>(Vec<T>);

    impl<T: fmt::Display> fmt::Display for PrettyVec<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[");
            for (i, item) in self.0.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ");
                }
                write!(f, "{}", item);
            }
            write!(f, "]")
        }
    }

    println!("\n5.2 NewType 绕过孤儿规则:");
    let numbers = PrettyVec(vec![1, 2, 3, 4, 5]);
    println!("美化显示: {}", numbers);

    let words = PrettyVec(vec!["hello".to_string(), "world".to_string()]);
    println!("美化显示: {}", words);

    // 5.3 为外部类型实现自定义 trait
    trait Summary {
        fn summarize(&self) -> String;
    }

    struct StringWrapper(String);

    impl Summary for StringWrapper {
        fn summarize(&self) -> String {
            format!("Summary: {} (length: {})", self.0, self.0.len())
        }
    }

    println!("\n5.3 为包装类型实现自定义trait:");
    let wrapped = StringWrapper("Hello, Rust!".to_string());
    println!("{}", wrapped.summarize());
}

/// 6. 实际应用场景演示
fn demonstrate_real_world_scenarios() {
    println!("\n=== 6. 实际应用场景演示 ===");

    // 6.1 单位类型系统
    println!("\n6.1 单位类型系统:");

    #[derive(Debug, Clone, Copy)]
    struct Temperature(f64); // 摄氏度

    #[derive(Debug, Clone, Copy)]
    struct Fahrenheit(f64);

    impl Temperature {
        fn new(celsius: f64) -> Self {
            Temperature(celsius)
        }

        fn to_fahrenheit(&self) -> Fahrenheit {
            Fahrenheit(self.0 * 9.0 / 5.0 + 32.0)
        }

        fn is_freezing(&self) -> bool {
            self.0 <= 0.0
        }
    }

    impl Fahrenheit {
        fn to_celsius(&self) -> Temperature {
            Temperature((self.0 - 32.0) * 5.0 / 9.0)
        }
    }

    let temp_c = Temperature::new(25.0);
    let temp_f = temp_c.to_fahrenheit();
    println!("温度转换: {:?} = {:?}", temp_c, temp_f);
    println!("是否结冰: {}", temp_c.is_freezing());

    // 6.2 ID 包装类型
    println!("\n6.2 ID 包装类型:");

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct UserId(u64);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct OrderId(u64);

    #[derive(Debug)]
    struct User {
        id: UserId,
        name: String,
    }

    #[derive(Debug)]
    struct Order {
        id: OrderId,
        user_id: UserId,
        amount: f64,
    }

    impl UserId {
        fn new(id: u64) -> Self {
            UserId(id)
        }
    }

    impl OrderId {
        fn new(id: u64) -> Self {
            OrderId(id)
        }
    }

    let user = User {
        id: UserId::new(1001),
        name: "Alice".to_string(),
    };

    let order = Order {
        id: OrderId::new(2001),
        user_id: user.id,
        amount: 99.99,
    };

    println!("用户: {:?}", user);
    println!("订单: {:?}", order);

    // 6.3 状态机类型
    println!("\n6.3 状态机类型:");

    #[derive(Debug)]
    struct Draft;

    #[derive(Debug)]
    struct Published;

    #[derive(Debug)]
    struct Archived;

    #[derive(Debug)]
    struct Post<State> {
        content: String,
        state: std::marker::PhantomData<State>,
    }

    impl Post<Draft> {
        fn new(content: String) -> Self {
            Post {
                content,
                state: std::marker::PhantomData,
            }
        }

        fn publish(self) -> Post<Published> {
            Post {
                content: self.content,
                state: std::marker::PhantomData,
            }
        }
    }

    impl Post<Published> {
        fn archive(self) -> Post<Archived> {
            Post {
                content: self.content,
                state: std::marker::PhantomData,
            }
        }
    }

    let draft = Post::new("我的第一篇文章".to_string());
    println!("草稿: {:?}", draft);

    let published = draft.publish();
    println!("已发布: {:?}", published);

    let archived = published.archive();
    println!("已归档: {:?}", archived);
}

/// 7. 高级模式演示
fn demonstrate_advanced_patterns() {
    println!("\n=== 7. 高级模式演示 ===");

    // 7.1 Deref 强制转换
    println!("\n7.1 Deref 强制转换:");

    #[derive(Debug)]
    struct SmartString(String);

    impl Deref for SmartString {
        type Target = String;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl SmartString {
        fn new(s: &str) -> Self {
            SmartString(s.to_string())
        }

        fn word_count(&self) -> usize {
            self.split_whitespace().count()
        }
    }

    let smart_str = SmartString::new("Hello Rust World");
    println!("智能字符串: {:?}", smart_str);
    println!("字符串长度: {}", smart_str.len()); // 通过 Deref 调用 String 方法
    println!("单词数量: {}", smart_str.word_count());

    // 7.2 From/Into 转换
    println!("\n7.2 From/Into 转换:");

    #[derive(Debug)]
    struct Email(String);

    impl From<String> for Email {
        fn from(email: String) -> Self {
            Email(email)
        }
    }

    impl From<&str> for Email {
        fn from(email: &str) -> Self {
            Email(email.to_string())
        }
    }

    let email1: Email = "user@example.com".into();
    let email2: Email = String::from("admin@example.com").into();

    println!("邮箱1: {:?}", email1);
    println!("邮箱2: {:?}", email2);

    // 7.3 运算符重载
    println!("\n7.3 运算符重载:");

    #[derive(Debug, Clone, Copy)]
    struct Money(f64);

    impl Add for Money {
        type Output = Money;

        fn add(self, other: Money) -> Money {
            Money(self.0 + other.0)
        }
    }

    impl fmt::Display for Money {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "${:.2}", self.0)
        }
    }

    let price1 = Money(19.99);
    let price2 = Money(5.50);
    let total = price1 + price2;

    println!("价格1: {}", price1);
    println!("价格2: {}", price2);
    println!("总计: {}", total);
}

/// 8. 性能分析演示
fn demonstrate_performance_analysis() {
    println!("\n=== 8. 性能分析演示 ===");

    // 8.1 内存布局分析
    println!("\n8.1 内存布局分析:");

    #[derive(Debug)]
    struct WrappedU64(u64);

    #[derive(Debug)]
    struct WrappedString(String);

    type AliasU64 = u64;
    type AliasString = String;

    println!("u64 大小: {} bytes", std::mem::size_of::<u64>());
    println!(
        "WrappedU64 大小: {} bytes",
        std::mem::size_of::<WrappedU64>()
    );
    println!("AliasU64 大小: {} bytes", std::mem::size_of::<AliasU64>());

    println!("String 大小: {} bytes", std::mem::size_of::<String>());
    println!(
        "WrappedString 大小: {} bytes",
        std::mem::size_of::<WrappedString>()
    );
    println!(
        "AliasString 大小: {} bytes",
        std::mem::size_of::<AliasString>()
    );

    // 8.2 编译时优化验证
    println!("\n8.2 编译时优化验证:");

    #[derive(Debug)]
    struct Counter(usize);

    impl Counter {
        fn new() -> Self {
            Counter(0)
        }

        fn increment(&mut self) {
            self.0 += 1;
        }

        fn get(&self) -> usize {
            self.0
        }
    }

    let mut counter = Counter::new();
    for _ in 0..1000 {
        counter.increment();
    }

    println!("计数器最终值: {}", counter.get());
    println!("NewType 包装的计数器与原始 usize 性能相同");

    // 8.3 零成本抽象演示
    println!("\n8.3 零成本抽象演示:");

    fn process_raw(value: u32) -> u32 {
        value * 2
    }

    fn process_wrapped(value: Meters) -> Meters {
        Meters(value.0 * 2)
    }

    #[derive(Debug, Clone, Copy)]
    struct Meters(u32);

    let raw_value = 100u32;
    let wrapped_value = Meters(100);

    let raw_result = process_raw(raw_value);
    let wrapped_result = process_wrapped(wrapped_value);

    println!("原始值处理结果: {}", raw_result);
    println!("包装值处理结果: {:?}", wrapped_result);
    println!("两种方式生成的汇编代码完全相同！");
}

/// 9. 最佳实践演示
fn demonstrate_best_practices() {
    println!("\n=== 9. 最佳实践演示 ===");

    println!("\n9.1 何时使用 NewType:");
    println!("✓ 需要类型安全时（防止参数混淆）");
    println!("✓ 需要为外部类型实现 trait 时");
    println!("✓ 需要添加领域特定方法时");
    println!("✓ 需要状态机或类型级编程时");

    println!("\n9.2 何时使用类型别名:");
    println!("✓ 简化复杂类型签名时");
    println!("✓ 提高代码可读性时");
    println!("✓ 创建泛型类型的特化时");
    println!("✓ 不需要额外类型安全时");

    // 9.3 命名约定
    println!("\n9.3 命名约定示例:");

    // NewType 使用描述性名称
    #[derive(Debug)]
    struct AccountBalance(f64);

    #[derive(Debug)]
    struct TransactionId(String);

    // 类型别名使用 Type 后缀或描述性名称
    type DatabaseConnection = std::collections::HashMap<String, String>;
    type ApiResult<T> = Result<T, Box<dyn std::error::Error>>;

    let balance = AccountBalance(1000.50);
    let tx_id = TransactionId("TX123456".to_string());

    println!("账户余额: {:?}", balance);
    println!("交易ID: {:?}", tx_id);

    // 9.4 文档和测试
    println!("\n9.4 文档和测试重要性:");
    println!("✓ 为 NewType 提供清晰的文档说明");
    println!("✓ 编写单元测试验证类型安全性");
    println!("✓ 提供使用示例和最佳实践");

    println!("\n=== 教程总结 ===");
    println!("NewType 模式和类型别名各有优势：");
    println!("• NewType: 强类型安全，零成本抽象，适合复杂业务逻辑");
    println!("• 类型别名: 简化类型签名，提高可读性，适合简单场景");
    println!("选择合适的方式能让代码更安全、更清晰、更易维护！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newtype_type_safety() {
        #[derive(Debug, PartialEq)]
        struct UserId(u32);

        #[derive(Debug, PartialEq)]
        struct ProductId(u32);

        let user_id = UserId(123);
        let product_id = ProductId(456);

        // NewType 提供类型安全
        assert_ne!(user_id.0, product_id.0);

        // 不能直接比较不同的 NewType
        // assert_ne!(user_id, product_id); // 编译错误
    }

    #[test]
    fn test_type_alias_interchangeability() {
        type UserId = u32;
        type ProductId = u32;

        let user_id: UserId = 123;
        let product_id: ProductId = 456;

        // 类型别名可以互换
        fn process_id(id: UserId) -> UserId {
            id * 2
        }

        let result1 = process_id(user_id);
        let result2 = process_id(product_id); // 可以传入 ProductId

        assert_eq!(result1, 246);
        assert_eq!(result2, 912);
    }

    #[test]
    fn test_zero_cost_abstraction() {
        #[derive(Debug)]
        struct Wrapper(u64);

        // 验证零成本抽象
        assert_eq!(std::mem::size_of::<u64>(), std::mem::size_of::<Wrapper>());
    }

    #[test]
    fn test_deref_coercion() {
        use std::ops::Deref;

        #[derive(Debug)]
        struct MyString(String);

        impl Deref for MyString {
            type Target = String;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        let my_str = MyString("Hello".to_string());
        assert_eq!(my_str.len(), 5); // 通过 Deref 调用 String 方法
    }

    #[test]
    fn test_from_into_conversion() {
        #[derive(Debug, PartialEq)]
        struct Email(String);

        impl From<&str> for Email {
            fn from(s: &str) -> Self {
                Email(s.to_string())
            }
        }

        let email: Email = "test@example.com".into();
        assert_eq!(email.0, "test@example.com");
    }
}
