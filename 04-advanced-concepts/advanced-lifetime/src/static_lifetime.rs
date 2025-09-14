//! # 静态生命周期（'static Lifetime）
//!
//! 本模块深入探讨 Rust 的静态生命周期，包括：
//! - 'static 生命周期的含义和特性
//! - 字符串字面量和静态生命周期
//! - T: 'static 约束的理解和应用
//! - 静态变量和常量的生命周期
//! - 常见误解和最佳实践

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

/// # 静态生命周期基础概念
///
/// 'static 是 Rust 中最长的生命周期，表示数据在整个程序运行期间都有效
pub mod static_basics {
    /// 演示静态生命周期的基础概念
    pub fn demonstrate_static_basics() {
        println!("=== 静态生命周期基础概念 ===");

        // 1. 字符串字面量的静态生命周期
        demonstrate_string_literals();

        // 2. 静态变量
        demonstrate_static_variables();

        // 3. 常量的生命周期
        demonstrate_constants();
    }

    /// 演示字符串字面量的静态生命周期
    fn demonstrate_string_literals() {
        println!("\n--- 字符串字面量的静态生命周期 ---");

        // 字符串字面量具有 'static 生命周期
        let static_str: &'static str = "这是一个静态字符串";
        println!("静态字符串: {}", static_str);

        // 可以在任何作用域中使用
        {
            let inner_static: &'static str = "内部作用域的静态字符串";
            println!("内部静态字符串: {}", inner_static);
        }

        // 演示静态字符串的传递
        let result = process_static_str("传递给函数的静态字符串");
        println!("处理结果: {}", result);

        // 演示静态字符串数组
        let static_array: &'static [&'static str] = &["apple", "banana", "cherry"];
        println!("静态字符串数组: {:?}", static_array);
    }

    /// 处理静态字符串的函数
    fn process_static_str(s: &'static str) -> &'static str {
        println!("处理静态字符串: {}", s);
        s // 可以直接返回，因为它有 'static 生命周期
    }

    /// 演示静态变量
    fn demonstrate_static_variables() {
        println!("\n--- 静态变量 ---");

        // 访问静态变量
        println!("全局计数器: {}", super::GLOBAL_COUNTER);
        println!("应用名称: {}", super::APP_NAME);

        // 演示静态变量的修改（需要 unsafe）
        unsafe {
            super::MUTABLE_COUNTER += 1;
            let counter_value = super::MUTABLE_COUNTER;
            println!("可变计数器: {}", counter_value);
        }

        // 演示静态变量的引用
        let counter_ref: &'static i32 = &super::GLOBAL_COUNTER;
        println!("计数器引用: {}", counter_ref);

        // 演示复杂静态数据
        println!("配置数据: {:?}", super::CONFIG_DATA);
    }

    /// 演示常量的生命周期
    fn demonstrate_constants() {
        println!("\n--- 常量的生命周期 ---");

        // 常量在编译时确定，具有 'static 生命周期
        println!("最大用户数: {}", super::MAX_USERS);
        println!("默认超时: {:?}", super::DEFAULT_TIMEOUT);

        // 常量可以在任何地方使用
        let timeout_ref: &'static std::time::Duration = &super::DEFAULT_TIMEOUT;
        println!("超时引用: {:?}", timeout_ref);

        // 演示常量数组
        println!("支持的格式: {:?}", super::SUPPORTED_FORMATS);

        // 演示常量结构体
        println!("默认配置: {:?}", super::DEFAULT_CONFIG);
    }
}

// 静态变量定义
static GLOBAL_COUNTER: i32 = 42;
static APP_NAME: &str = "Rust 生命周期教程";
static mut MUTABLE_COUNTER: i32 = 0;
static CONFIG_DATA: &[(&str, &str)] = &[
    ("version", "1.0.0"),
    ("author", "Rust Team"),
    ("license", "MIT"),
];

// 常量定义
const MAX_USERS: usize = 1000;
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const SUPPORTED_FORMATS: &[&str] = &["json", "xml", "yaml"];

#[derive(Debug)]
struct Config {
    debug: bool,
    max_connections: usize,
}

const DEFAULT_CONFIG: Config = Config {
    debug: false,
    max_connections: 100,
};

/// # T: 'static 约束的理解
///
/// T: 'static 约束表示类型 T 不包含任何非静态引用
pub mod static_bounds {
    use std::sync::Arc;
    use std::thread;

    /// 演示 T: 'static 约束
    pub fn demonstrate_static_bounds() {
        println!("\n=== T: 'static 约束演示 ===");

        // 1. 基本的 T: 'static 约束
        demonstrate_basic_static_bounds();

        // 2. 线程中的 T: 'static 约束
        demonstrate_thread_static_bounds();

        // 3. 拥有所有权的数据
        demonstrate_owned_data();

        // 4. 常见误解
        demonstrate_common_misconceptions();
    }

    /// 演示基本的 T: 'static 约束
    fn demonstrate_basic_static_bounds() {
        println!("\n--- 基本 T: 'static 约束 ---");

        // 可以传递拥有所有权的数据
        let owned_string = String::from("拥有所有权的字符串");
        store_static_data(owned_string);

        // 可以传递静态引用
        let static_str: &'static str = "静态字符串";
        store_static_reference(static_str);

        // 可以传递基本类型
        store_static_data(42i32);
        store_static_data(true);

        // 演示复杂类型
        let owned_vec = vec![1, 2, 3, 4, 5];
        store_static_data(owned_vec);
    }

    /// 存储具有 'static 约束的数据
    fn store_static_data<T: 'static>(data: T) {
        println!("存储静态数据: {:?}", std::any::type_name::<T>());
        // 这里可以安全地存储数据，因为它不包含任何非静态引用
        drop(data); // 为了演示，直接丢弃
    }

    /// 存储静态引用
    fn store_static_reference(data: &'static str) {
        println!("存储静态引用: {}", data);
    }

    /// 演示线程中的 T: 'static 约束
    fn demonstrate_thread_static_bounds() {
        println!("\n--- 线程中的 T: 'static 约束 ---");

        // 线程闭包必须是 'static 的
        let owned_data = String::from("线程数据");

        let handle = thread::spawn(move || {
            println!("线程中的数据: {}", owned_data);
            owned_data.len()
        });

        let result = handle.join().unwrap();
        println!("线程返回结果: {}", result);

        // 使用 Arc 共享数据
        let shared_data = Arc::new(String::from("共享数据"));
        let shared_clone = Arc::clone(&shared_data);

        let handle2 = thread::spawn(move || {
            println!("共享数据: {}", shared_clone);
        });

        handle2.join().unwrap();
        println!("原始共享数据: {}", shared_data);
    }

    /// 演示拥有所有权的数据
    fn demonstrate_owned_data() {
        println!("\n--- 拥有所有权的数据 ---");

        // 创建拥有所有权的复杂数据结构
        let mut data_map = std::collections::HashMap::new();
        data_map.insert("key1".to_string(), vec![1, 2, 3]);
        data_map.insert("key2".to_string(), vec![4, 5, 6]);

        // 可以传递给需要 T: 'static 的函数
        process_owned_data(data_map);

        // 演示自定义结构体
        let user_data = UserData {
            name: String::from("Alice"),
            age: 30,
            emails: vec!["alice@example.com".to_string()],
        };

        process_owned_data(user_data);
    }

    #[derive(Debug)]
    struct UserData {
        name: String,
        age: u32,
        emails: Vec<String>,
    }

    /// 处理拥有所有权的数据
    fn process_owned_data<T: 'static + std::fmt::Debug>(data: T) {
        println!("处理拥有所有权的数据: {:?}", data);
    }

    /// 演示常见误解
    fn demonstrate_common_misconceptions() {
        println!("\n--- 常见误解 ---");

        println!("误解1: T: 'static 意味着 T 必须活到程序结束");
        println!("实际: T: 'static 意味着 T 不包含任何非静态引用");

        // 这个函数可以接受任何拥有所有权的数据
        let temp_string = String::from("临时字符串");
        takes_static_bound(temp_string); // temp_string 在这里被移动，不需要活到程序结束

        println!("\n误解2: &'static T 和 T: 'static 是一样的");
        println!("实际: &'static T 是对静态数据的引用，T: 'static 是类型约束");

        // &'static str 是对静态字符串的引用
        let static_ref: &'static str = "静态引用";
        takes_static_reference(static_ref);

        // String: 'static 是拥有所有权的字符串
        let owned_string = String::from("拥有所有权的字符串");
        takes_static_bound(owned_string);
    }

    fn takes_static_bound<T: 'static>(data: T) {
        println!("接受 T: 'static 约束的数据");
        drop(data);
    }

    fn takes_static_reference(data: &'static str) {
        println!("接受 &'static 引用: {}", data);
    }
}

/// # 静态生命周期的实际应用
///
/// 探讨静态生命周期在实际编程中的应用场景
pub mod static_applications {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex, Once, RwLock};
    use std::thread;

    /// 演示静态生命周期的实际应用
    pub fn demonstrate_static_applications() {
        println!("\n=== 静态生命周期实际应用 ===");

        // 1. 全局配置管理
        demonstrate_global_configuration();

        // 2. 单例模式
        demonstrate_singleton_pattern();

        // 3. 线程安全的全局状态
        demonstrate_thread_safe_global_state();

        // 4. 缓存系统
        demonstrate_caching_system();
    }

    /// 演示全局配置管理
    fn demonstrate_global_configuration() {
        println!("\n--- 全局配置管理 ---");

        // 初始化配置
        GlobalConfig::init(AppConfig {
            app_name: "我的应用".to_string(),
            version: "1.0.0".to_string(),
            debug_mode: true,
            max_connections: 1000,
        });

        // 在不同地方访问配置
        let config = GlobalConfig::get();
        println!("应用配置: {:?}", config);

        // 在函数中使用配置
        use_global_config();
    }

    #[derive(Debug, Clone)]
    struct AppConfig {
        app_name: String,
        version: String,
        debug_mode: bool,
        max_connections: usize,
    }

    struct GlobalConfig;

    static mut GLOBAL_CONFIG: Option<AppConfig> = None;
    static CONFIG_INIT: Once = Once::new();

    impl GlobalConfig {
        fn init(config: AppConfig) {
            unsafe {
                CONFIG_INIT.call_once(|| {
                    GLOBAL_CONFIG = Some(config);
                });
            }
        }

        fn get() -> &'static AppConfig {
            unsafe {
                let ptr = std::ptr::addr_of!(GLOBAL_CONFIG);
                match (*ptr).as_ref() {
                    Some(config) => config,
                    None => panic!("配置未初始化"),
                }
            }
        }
    }

    fn use_global_config() {
        let config = GlobalConfig::get();
        println!("在函数中使用配置: {}", config.app_name);
    }

    /// 演示单例模式
    fn demonstrate_singleton_pattern() {
        println!("\n--- 单例模式 ---");

        // 获取数据库连接池实例
        let pool1 = DatabasePool::instance();
        let pool2 = DatabasePool::instance();

        println!("连接池1地址: {:p}", pool1);
        println!("连接池2地址: {:p}", pool2);
        println!("是否为同一实例: {}", std::ptr::eq(pool1, pool2));

        // 使用连接池
        pool1.get_connection();
        pool2.execute_query("SELECT * FROM users");
    }

    struct DatabasePool {
        connections: Vec<String>,
    }

    impl DatabasePool {
        fn instance() -> &'static Self {
            static mut INSTANCE: Option<DatabasePool> = None;
            static INIT: Once = Once::new();

            unsafe {
                INIT.call_once(|| {
                    INSTANCE = Some(DatabasePool {
                        connections: vec![
                            "连接1".to_string(),
                            "连接2".to_string(),
                            "连接3".to_string(),
                        ],
                    });
                });

                let ptr = std::ptr::addr_of!(INSTANCE);
                match (*ptr).as_ref() {
                    Some(instance) => instance,
                    None => panic!("实例初始化失败"),
                }
            }
        }

        fn get_connection(&self) -> &str {
            println!("获取数据库连接");
            &self.connections[0]
        }

        fn execute_query(&self, query: &str) {
            println!("执行查询: {}", query);
        }
    }

    /// 演示线程安全的全局状态
    fn demonstrate_thread_safe_global_state() {
        println!("\n--- 线程安全的全局状态 ---");

        // 初始化全局计数器
        let handles: Vec<_> = (0..5)
            .map(|i| {
                thread::spawn(move || {
                    for _ in 0..10 {
                        GlobalCounter::increment();
                        thread::sleep(std::time::Duration::from_millis(1));
                    }
                    println!("线程 {} 完成", i);
                })
            })
            .collect();

        // 等待所有线程完成
        for handle in handles {
            handle.join().unwrap();
        }

        println!("最终计数: {}", GlobalCounter::get());
    }

    struct GlobalCounter;

    static GLOBAL_COUNTER: Mutex<i32> = Mutex::new(0);

    impl GlobalCounter {
        fn increment() {
            let mut counter = GLOBAL_COUNTER.lock().unwrap();
            *counter += 1;
        }

        fn get() -> i32 {
            *GLOBAL_COUNTER.lock().unwrap()
        }
    }

    /// 演示缓存系统
    fn demonstrate_caching_system() {
        println!("\n--- 缓存系统 ---");

        // 使用缓存
        let result1 = GlobalCache::get_or_compute("key1", || {
            println!("计算 key1 的值");
            "value1".to_string()
        });
        println!("第一次获取: {}", result1);

        let result2 = GlobalCache::get_or_compute("key1", || {
            println!("这不应该被执行");
            "new_value1".to_string()
        });
        println!("第二次获取: {}", result2);

        // 添加更多缓存项
        let result3 = GlobalCache::get_or_compute("key2", || {
            println!("计算 key2 的值");
            "value2".to_string()
        });
        println!("获取 key2: {}", result3);

        // 显示缓存统计
        GlobalCache::show_stats();
    }

    struct GlobalCache;

    static CACHE: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);

    impl GlobalCache {
        fn get_or_compute<F>(key: &str, compute: F) -> String
        where
            F: FnOnce() -> String,
        {
            let mut cache_guard = CACHE.lock().unwrap();

            // 初始化缓存（如果还没有初始化）
            if cache_guard.is_none() {
                *cache_guard = Some(HashMap::new());
            }

            let cache = cache_guard.as_mut().unwrap();

            // 首先尝试读取
            if let Some(value) = cache.get(key) {
                return value.clone();
            }

            // 如果不存在，计算并存储
            let value = compute();
            cache.insert(key.to_string(), value.clone());

            value
        }

        fn show_stats() {
            let mut cache_guard = CACHE.lock().unwrap();

            // 初始化缓存（如果还没有初始化）
            if cache_guard.is_none() {
                *cache_guard = Some(HashMap::new());
            }

            let cache = cache_guard.as_ref().unwrap();
            println!("缓存项数量: {}", cache.len());
            for (key, value) in cache.iter() {
                println!("  {}: {}", key, value);
            }
        }
    }
}

/// # 静态生命周期的陷阱和最佳实践
///
/// 探讨使用静态生命周期时的常见陷阱和最佳实践
pub mod static_pitfalls_and_best_practices {
    use std::sync::{Arc, Mutex};
    use std::thread;

    /// 演示静态生命周期的陷阱和最佳实践
    pub fn demonstrate_pitfalls_and_best_practices() {
        println!("\n=== 静态生命周期陷阱和最佳实践 ===");

        // 1. 内存泄漏风险
        demonstrate_memory_leak_risks();

        // 2. 过度使用静态生命周期
        demonstrate_overuse_of_static();

        // 3. 最佳实践
        demonstrate_best_practices();
    }

    /// 演示内存泄漏风险
    fn demonstrate_memory_leak_risks() {
        println!("\n--- 内存泄漏风险 ---");

        println!("⚠️  静态变量会在程序结束时才被释放");
        println!("⚠️  大量使用静态变量可能导致内存占用过高");

        // 演示静态集合的增长
        static STATIC_VEC: Mutex<Vec<String>> = Mutex::new(Vec::new());

        // 模拟添加数据到静态集合
        for i in 0..5 {
            let mut vec = STATIC_VEC.lock().unwrap();
            vec.push(format!("数据项 {}", i));
        }

        let vec = STATIC_VEC.lock().unwrap();
        println!("静态向量大小: {}", vec.len());
        // 注意：这些数据会一直存在直到程序结束
    }

    /// 演示过度使用静态生命周期
    fn demonstrate_overuse_of_static() {
        println!("\n--- 过度使用静态生命周期 ---");

        println!("❌ 不好的做法：过度使用 'static");

        // 不好的例子：不必要的静态约束
        // fn bad_function<T: 'static>(data: T) -> T {
        //     data
        // }

        println!("✅ 好的做法：只在必要时使用 'static");

        // 好的例子：只在真正需要时使用静态约束
        let data = String::from("测试数据");
        let result = good_function(data);
        println!("处理结果长度: {}", result);

        // 演示何时真正需要 'static
        demonstrate_when_static_is_needed();
    }

    /// 好的函数设计：不过度使用静态约束
    fn good_function<T>(data: T) -> usize
    where
        T: AsRef<str>,
    {
        data.as_ref().len()
    }

    /// 演示何时真正需要 'static
    fn demonstrate_when_static_is_needed() {
        println!("\n--- 何时真正需要 'static ---");

        println!("1. 跨线程传递数据时");
        let data = Arc::new(String::from("跨线程数据"));
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            println!("线程中的数据: {}", data_clone);
        });

        handle.join().unwrap();

        println!("2. 存储在全局状态中时");
        // 这种情况下确实需要 'static 约束

        println!("3. 作为回调函数存储时");
        let callback = Box::new(|| println!("回调函数执行"));
        store_callback(callback);
    }

    /// 存储回调函数（需要 'static 约束）
    fn store_callback<F: 'static + Fn()>(callback: Box<F>) {
        println!("存储回调函数");
        // 在实际应用中，这个回调可能会被存储在全局状态中
        callback();
    }

    /// 演示最佳实践
    fn demonstrate_best_practices() {
        println!("\n--- 最佳实践 ---");

        println!("✅ 最佳实践1: 优先使用局部生命周期");
        let local_data = String::from("局部数据");
        process_with_local_lifetime(&local_data);

        println!("✅ 最佳实践2: 使用 Arc 和 Rc 共享数据");
        let shared_data = Arc::new(String::from("共享数据"));
        let clone1 = Arc::clone(&shared_data);
        let clone2 = Arc::clone(&shared_data);

        println!("共享数据引用计数: {}", Arc::strong_count(&shared_data));

        println!("✅ 最佳实践3: 明确区分 &'static T 和 T: 'static");

        // &'static T - 对静态数据的引用
        let static_str: &'static str = "静态字符串";
        use_static_reference(static_str);

        // T: 'static - 不包含非静态引用的类型
        let owned_data = String::from("拥有所有权的数据");
        use_static_bound(owned_data);

        println!("✅ 最佳实践4: 谨慎使用全局可变状态");
        // 使用 Mutex 或 RwLock 保护全局状态
        // 考虑使用 lazy_static 或 once_cell 进行延迟初始化
    }

    fn process_with_local_lifetime(data: &str) {
        println!("处理局部数据: {}", data);
    }

    fn use_static_reference(data: &'static str) {
        println!("使用静态引用: {}", data);
    }

    fn use_static_bound<T: 'static + std::fmt::Display>(data: T) {
        println!("使用静态约束数据: {}", data);
    }
}

/// 运行所有静态生命周期的演示
pub fn run_all_demonstrations() {
    run_all_static_lifetime_examples();
}

/// 运行所有静态生命周期的演示（别名函数）
pub fn run_all_static_lifetime_examples() {
    println!("🦀 Rust 深入生命周期 - 静态生命周期篇 🦀");
    println!("=============================================");

    // 1. 静态生命周期基础
    static_basics::demonstrate_static_basics();

    // 2. T: 'static 约束
    static_bounds::demonstrate_static_bounds();

    // 3. 实际应用
    static_applications::demonstrate_static_applications();

    // 4. 陷阱和最佳实践
    static_pitfalls_and_best_practices::demonstrate_pitfalls_and_best_practices();

    println!("\n=============================================");
    println!("✅ 静态生命周期演示完成！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_string_literal() {
        let s: &'static str = "test";
        assert_eq!(s, "test");
    }

    #[test]
    fn test_static_bound() {
        fn takes_static<T: 'static>(data: T) -> T {
            data
        }

        let owned = String::from("owned");
        let result = takes_static(owned);
        assert_eq!(result, "owned");
    }

    #[test]
    fn test_global_config() {
        // 测试全局配置（在实际应用中需要更仔细的测试）
        assert_eq!(GLOBAL_COUNTER, 42);
        assert_eq!(APP_NAME, "Rust 生命周期教程");
    }
}
