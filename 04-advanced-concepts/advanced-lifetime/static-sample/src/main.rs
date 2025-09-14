//! # Rust 静态生命周期深度解析
//! 
//! 本模块全面分析 `&'static` 和 `T: 'static` 的区别、使用场景和最佳实践
//! 基于 https://course.rs/advance/lifetime/static.html 的内容进行深度扩展

use std::fmt::Display;
use std::thread;
use std::sync::Arc;
use std::collections::HashMap;

/// # 1. 静态生命周期基础概念
/// 
/// `'static` 是 Rust 中最长的生命周期，表示数据在整个程序运行期间都有效
fn demonstrate_static_basics() {
    println!("=== 1. 静态生命周期基础概念 ===");
    
    // 1.1 字符串字面量具有 'static 生命周期
    let static_str: &'static str = "这是一个静态字符串";
    println!("静态字符串: {}", static_str);
    
    // 1.2 静态变量
    static GLOBAL_COUNT: i32 = 42;
    println!("全局静态变量: {}", GLOBAL_COUNT);
    
    // 1.3 常量也具有 'static 生命周期
    const PI: f64 = 3.14159;
    println!("常量 PI: {}", PI);
}

/// # 2. &'static 引用类型详解
/// 
/// `&'static` 表示一个引用，该引用指向的数据在整个程序运行期间都有效
fn demonstrate_static_reference() {
    println!("\n=== 2. &'static 引用类型详解 ===");
    
    // 2.1 字符串字面量的 &'static str
    let name: &'static str = "Rust";
    println!("静态字符串引用: {}", name);
    
    // 2.2 静态数组
    static NUMBERS: [i32; 5] = [1, 2, 3, 4, 5];
    let static_slice: &'static [i32] = &NUMBERS;
    println!("静态数组切片: {:?}", static_slice);
    
    // 2.3 Box::leak 创建 'static 引用
    let boxed_string = Box::new(String::from("泄露的字符串"));
    let leaked_str: &'static str = Box::leak(boxed_string);
    println!("泄露的静态引用: {}", leaked_str);
    
    // 2.4 静态引用在函数间传递
    process_static_ref(name);
    process_static_ref(leaked_str);
}

fn process_static_ref(s: &'static str) {
    println!("处理静态引用: {}", s);
}

/// # 3. T: 'static 类型约束详解
/// 
/// `T: 'static` 表示类型 T 不包含任何非静态引用
fn demonstrate_static_bound() {
    println!("\n=== 3. T: 'static 类型约束详解 ===");
    
    // 3.1 拥有所有权的类型满足 T: 'static
    let owned_string = String::from("拥有所有权的字符串");
    process_static_type(owned_string);
    
    // 3.2 基本类型满足 T: 'static
    process_static_type(42i32);
    process_static_type(3.14f64);
    process_static_type(true);
    
    // 3.3 包含静态引用的类型满足 T: 'static
    let static_ref = "静态字符串";
    process_static_type(static_ref);
    
    // 3.4 结构体满足 T: 'static 的条件
    let person = Person {
        name: String::from("张三"),
        age: 30,
    };
    process_static_type(person);
}

#[derive(Debug)]
struct Person {
    name: String,  // 拥有所有权，满足 'static
    age: i32,      // 基本类型，满足 'static
}

fn process_static_type<T: 'static + std::fmt::Debug>(value: T) {
    println!("处理静态类型: {:?}", value);
}

/// # 4. &'static 与 T: 'static 的关键区别
fn demonstrate_static_differences() {
    println!("\n=== 4. &'static 与 T: 'static 的关键区别 ===");
    
    // 4.1 &'static 是引用类型
    let static_str: &'static str = "静态字符串";
    println!("&'static 引用: {}", static_str);
    
    // 4.2 T: 'static 是类型约束
    let owned_data = vec![1, 2, 3, 4, 5];
    send_to_thread(owned_data); // Vec<i32> 满足 T: 'static
    
    // 4.3 错误示例：非静态引用不满足 T: 'static
    // let local_string = String::from("本地字符串");
    // let local_ref = &local_string;
    // send_to_thread(local_ref); // 编译错误！&String 不满足 T: 'static
    
    // 4.4 正确做法：传递拥有所有权的数据
    let local_string = String::from("本地字符串");
    send_to_thread(local_string); // String 满足 T: 'static
}

fn send_to_thread<T: 'static + Send + std::fmt::Debug>(data: T) {
    thread::spawn(move || {
        println!("线程中处理数据: {:?}", data);
    }).join().unwrap();
}

/// # 5. 静态生命周期的实际应用场景
fn demonstrate_static_use_cases() {
    println!("\n=== 5. 静态生命周期的实际应用场景 ===");
    
    // 5.1 全局配置
    println!("应用配置: {}", get_app_config());
    
    // 5.2 错误消息
    let error = create_error("文件未找到");
    println!("错误信息: {}", error.message);
    
    // 5.3 缓存系统
    let mut cache = StaticCache::new();
    cache.insert("key1", "value1");
    println!("缓存值: {:?}", cache.get("key1"));
    
    // 5.4 线程间共享数据
    demonstrate_thread_sharing();
}

// 5.1 全局配置示例
static APP_NAME: &str = "Rust静态生命周期示例";
static VERSION: &str = "1.0.0";

fn get_app_config() -> &'static str {
    APP_NAME
}

// 5.2 错误处理示例
#[derive(Debug)]
struct StaticError {
    message: &'static str,
    code: i32,
}

fn create_error(msg: &'static str) -> StaticError {
    StaticError {
        message: msg,
        code: 404,
    }
}

// 5.3 缓存系统示例
struct StaticCache {
    data: HashMap<&'static str, &'static str>,
}

impl StaticCache {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    
    fn insert(&mut self, key: &'static str, value: &'static str) {
        self.data.insert(key, value);
    }
    
    fn get(&self, key: &str) -> Option<&&'static str> {
        self.data.get(key)
    }
}

// 5.4 线程间共享数据
fn demonstrate_thread_sharing() {
    let shared_data = Arc::new(vec![1, 2, 3, 4, 5]);
    
    let handles: Vec<_> = (0..3).map(|i| {
        let data = Arc::clone(&shared_data);
        thread::spawn(move || {
            println!("线程 {} 访问数据: {:?}", i, data);
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
}

/// # 6. 静态生命周期的高级模式
fn demonstrate_advanced_static_patterns() {
    println!("\n=== 6. 静态生命周期的高级模式 ===");
    
    // 6.1 静态分发与动态分发
    demonstrate_static_dispatch();
    
    // 6.2 静态单例模式
    let singleton = StaticSingleton::instance();
    println!("单例实例: {}", singleton.get_value());
    
    // 6.3 静态工厂模式
    let processor = ProcessorFactory::create_processor("default");
    processor.process("测试数据");
    
    // 6.4 静态注册表模式
    StaticRegistry::register("handler1", Box::new(DefaultHandler));
    if let Some(handler) = StaticRegistry::get("handler1") {
        handler.handle("事件数据");
    }
}

// 6.1 静态分发示例
trait StaticProcessor {
    fn process(&self, data: &str);
}

struct ConcreteProcessor;

impl StaticProcessor for ConcreteProcessor {
    fn process(&self, data: &str) {
        println!("静态分发处理: {}", data);
    }
}

fn demonstrate_static_dispatch() {
    let processor = ConcreteProcessor;
    process_with_static_dispatch(&processor, "静态分发数据");
}

fn process_with_static_dispatch<T: StaticProcessor>(processor: &T, data: &str) {
    processor.process(data);
}

// 6.2 静态单例模式
struct StaticSingleton {
    value: i32,
}

impl StaticSingleton {
    fn instance() -> &'static Self {
        static mut INSTANCE: Option<StaticSingleton> = None;
        static ONCE: std::sync::Once = std::sync::Once::new();
        
        unsafe {
            ONCE.call_once(|| {
                INSTANCE = Some(StaticSingleton { value: 42 });
            });
            INSTANCE.as_ref().unwrap()
        }
    }
    
    fn get_value(&self) -> i32 {
        self.value
    }
}

// 6.3 静态工厂模式
struct ProcessorFactory;

impl ProcessorFactory {
    fn create_processor(name: &'static str) -> &'static dyn StaticProcessor {
        match name {
            "default" => &ConcreteProcessor,
            _ => &ConcreteProcessor,
        }
    }
}

// 6.4 静态注册表模式
use std::sync::Mutex;
use std::collections::HashMap as StdHashMap;

trait EventHandler {
    fn handle(&self, data: &str);
}

struct DefaultHandler;

impl EventHandler for DefaultHandler {
    fn handle(&self, data: &str) {
        println!("默认处理器处理: {}", data);
    }
}

struct StaticRegistry;

impl StaticRegistry {
    fn get_registry() -> &'static Mutex<StdHashMap<&'static str, Box<dyn EventHandler + Send + Sync>>> {
        static REGISTRY: std::sync::OnceLock<Mutex<StdHashMap<&'static str, Box<dyn EventHandler + Send + Sync>>>> = std::sync::OnceLock::new();
        REGISTRY.get_or_init(|| Mutex::new(StdHashMap::new()))
    }
    
    fn register(name: &'static str, handler: Box<dyn EventHandler + Send + Sync>) {
        let mut registry = Self::get_registry().lock().unwrap();
        registry.insert(name, handler);
    }
    
    fn get(_name: &str) -> Option<Box<dyn EventHandler + Send + Sync>> {
        let _registry = Self::get_registry().lock().unwrap();
        // 注意：这里为了演示简化了实现，实际使用中需要考虑所有权问题
        None // 简化实现
    }
}

/// # 7. 静态生命周期的常见陷阱和解决方案
fn demonstrate_static_pitfalls() {
    println!("\n=== 7. 静态生命周期的常见陷阱和解决方案 ===");
    
    // 7.1 内存泄漏风险
    demonstrate_memory_leak_risk();
    
    // 7.2 过度使用 'static 的问题
    demonstrate_overuse_problems();
    
    // 7.3 正确的替代方案
    demonstrate_alternatives();
}

// 7.1 内存泄漏风险
fn demonstrate_memory_leak_risk() {
    println!("\n7.1 内存泄漏风险:");
    
    // 危险：Box::leak 会造成内存泄漏
    let leaked_string = Box::leak(Box::new(String::from("这会造成内存泄漏")));
    println!("泄漏的字符串: {}", leaked_string);
    
    // 更好的做法：使用 Cow 或其他智能指针
    use std::borrow::Cow;
    let cow_str: Cow<'static, str> = Cow::Borrowed("借用的静态字符串");
    println!("Cow 字符串: {}", cow_str);
}

// 7.2 过度使用 'static 的问题
fn demonstrate_overuse_problems() {
    println!("\n7.2 过度使用 'static 的问题:");
    
    // 问题：强制要求 'static 限制了灵活性
    // 不好的设计
    fn bad_design(data: &'static str) -> &'static str {
        data
    }
    
    // 更好的设计：使用生命周期参数
    fn better_design<'a>(data: &'a str) -> &'a str {
        data
    }
    
    let static_data = "静态数据";
    let local_data = String::from("本地数据");
    
    println!("处理静态数据: {}", bad_design(static_data));
    // println!("处理本地数据: {}", bad_design(&local_data)); // 编译错误！
    
    println!("灵活处理静态数据: {}", better_design(static_data));
    println!("灵活处理本地数据: {}", better_design(&local_data)); // 正常工作！
}

// 7.3 正确的替代方案
fn demonstrate_alternatives() {
    println!("\n7.3 正确的替代方案:");
    
    // 方案1：使用 Arc 共享所有权
    let shared_data = Arc::new(String::from("共享数据"));
    let cloned_data = Arc::clone(&shared_data);
    
    thread::spawn(move || {
        println!("线程中的共享数据: {}", cloned_data);
    }).join().unwrap();
    
    // 方案2：使用 'static 约束而不是 'static 引用
    process_owned_data(String::from("拥有所有权的数据"));
    
    // 方案3：使用生命周期参数
    let local_string = String::from("本地字符串");
    let result = process_with_lifetime(&local_string);
    println!("生命周期处理结果: {}", result);
}

fn process_owned_data<T: 'static + Display>(data: T) {
    println!("处理拥有所有权的数据: {}", data);
}

fn process_with_lifetime<'a>(data: &'a str) -> &'a str {
    data
}

/// # 8. 静态生命周期的性能考虑
fn demonstrate_performance_considerations() {
    println!("\n=== 8. 静态生命周期的性能考虑 ===");
    
    // 8.1 编译时优化
    demonstrate_compile_time_optimization();
    
    // 8.2 运行时性能
    demonstrate_runtime_performance();
    
    // 8.3 内存使用模式
    demonstrate_memory_patterns();
}

// 8.1 编译时优化
fn demonstrate_compile_time_optimization() {
    println!("\n8.1 编译时优化:");
    
    // 静态字符串在编译时就确定，无需运行时分配
    const COMPILE_TIME_STR: &str = "编译时字符串";
    println!("编译时字符串: {}", COMPILE_TIME_STR);
    
    // 静态数组也在编译时确定
    static COMPILE_TIME_ARRAY: [i32; 1000] = [0; 1000];
    println!("静态数组长度: {}", COMPILE_TIME_ARRAY.len());
}

// 8.2 运行时性能
fn demonstrate_runtime_performance() {
    println!("\n8.2 运行时性能:");
    
    use std::time::Instant;
    
    // 测试静态引用访问性能
    let start = Instant::now();
    for _ in 0..1_000_000 {
        let _ = get_static_string();
    }
    let static_duration = start.elapsed();
    
    // 测试动态分配性能
    let start = Instant::now();
    for _ in 0..1_000_000 {
        let _ = get_dynamic_string();
    }
    let dynamic_duration = start.elapsed();
    
    println!("静态引用访问时间: {:?}", static_duration);
    println!("动态分配时间: {:?}", dynamic_duration);
}

fn get_static_string() -> &'static str {
    "静态字符串"
}

fn get_dynamic_string() -> String {
    String::from("动态字符串")
}

// 8.3 内存使用模式
fn demonstrate_memory_patterns() {
    println!("\n8.3 内存使用模式:");
    
    // 静态数据存储在程序的数据段
    static STATIC_DATA: [u8; 1024] = [0; 1024];
    println!("静态数据地址: {:p}", &STATIC_DATA);
    
    // 堆分配数据
    let heap_data = vec![0u8; 1024];
    println!("堆数据地址: {:p}", heap_data.as_ptr());
    
    // 栈分配数据
    let stack_data = [0u8; 1024];
    println!("栈数据地址: {:p}", &stack_data);
}

/// # 9. 静态生命周期的最佳实践
fn demonstrate_best_practices() {
    println!("\n=== 9. 静态生命周期的最佳实践 ===");
    
    // 9.1 何时使用 'static
    println!("\n9.1 何时使用 'static:");
    println!("- 全局配置和常量");
    println!("- 错误消息和静态字符串");
    println!("- 线程间共享的不可变数据");
    println!("- 插件系统和注册表");
    
    // 9.2 何时避免使用 'static
    println!("\n9.2 何时避免使用 'static:");
    println!("- 临时数据处理");
    println!("- 可变状态管理");
    println!("- 短生命周期的数据");
    
    // 9.3 设计原则
    demonstrate_design_principles();
}

fn demonstrate_design_principles() {
    println!("\n9.3 设计原则:");
    
    // 原则1：优先使用生命周期参数而不是 'static
    fn flexible_function<'a>(data: &'a str) -> &'a str {
        data
    }
    
    // 原则2：使用 T: 'static 约束而不是 &'static T 参数
    fn thread_safe_function<T: 'static + Send>(_data: T) {
        thread::spawn(move || {
            // 处理数据
        });
    }
    
    // 原则3：考虑使用 Cow 处理可能的静态或动态数据
    use std::borrow::Cow;
    
    fn cow_function(data: Cow<'_, str>) -> String {
        format!("处理: {}", data)
    }
    
    let static_str = "静态字符串";
    let dynamic_str = String::from("动态字符串");
    
    println!("{}", flexible_function(static_str));
    println!("{}", cow_function(Cow::Borrowed(static_str)));
    println!("{}", cow_function(Cow::Owned(dynamic_str)));
}

/// # 10. 实际项目中的静态生命周期应用
fn demonstrate_real_world_applications() {
    println!("\n=== 10. 实际项目中的静态生命周期应用 ===");
    
    // 10.1 配置管理系统
    let config = AppConfig::global();
    println!("应用配置: 数据库URL = {}", config.database_url);
    
    // 10.2 日志系统
    Logger::log(LogLevel::Info, "应用启动");
    Logger::log(LogLevel::Error, "发生错误");
    
    // 10.3 插件系统
    PluginManager::register("auth", Box::new(AuthPlugin));
    PluginManager::execute("auth", "用户登录");
    
    // 10.4 国际化系统
    I18n::set_language("zh-CN");
    println!("本地化消息: {}", I18n::get_message("welcome"));
}

// 10.1 配置管理系统
struct AppConfig {
    database_url: &'static str,
    api_key: &'static str,
    debug_mode: bool,
}

impl AppConfig {
    fn global() -> &'static Self {
        static CONFIG: AppConfig = AppConfig {
            database_url: "postgresql://localhost/myapp",
            api_key: "your-api-key-here",
            debug_mode: true,
        };
        &CONFIG
    }
}

// 10.2 日志系统
#[derive(Debug)]
enum LogLevel {
    Info,
    Warning,
    Error,
}

struct Logger;

impl Logger {
    fn log(level: LogLevel, message: &str) {
        println!("[{:?}] {}", level, message);
    }
}

// 10.3 插件系统
trait Plugin {
    fn execute(&self, data: &str);
}

struct AuthPlugin;

impl Plugin for AuthPlugin {
    fn execute(&self, data: &str) {
        println!("认证插件处理: {}", data);
    }
}

struct PluginManager;

impl PluginManager {
    fn get_plugins() -> &'static Mutex<StdHashMap<&'static str, Box<dyn Plugin + Send + Sync>>> {
        static PLUGINS: std::sync::OnceLock<Mutex<StdHashMap<&'static str, Box<dyn Plugin + Send + Sync>>>> = std::sync::OnceLock::new();
        PLUGINS.get_or_init(|| Mutex::new(StdHashMap::new()))
    }
    
    fn register(name: &'static str, plugin: Box<dyn Plugin + Send + Sync>) {
        let mut plugins = Self::get_plugins().lock().unwrap();
        plugins.insert(name, plugin);
    }
    
    fn execute(name: &str, data: &str) {
        let plugins = Self::get_plugins().lock().unwrap();
        if let Some(plugin) = plugins.get(name) {
            plugin.execute(data);
        }
    }
}

// 10.4 国际化系统
struct I18n;

impl I18n {
    fn get_messages() -> &'static Mutex<StdHashMap<&'static str, StdHashMap<&'static str, &'static str>>> {
        static MESSAGES: std::sync::OnceLock<Mutex<StdHashMap<&'static str, StdHashMap<&'static str, &'static str>>>> = std::sync::OnceLock::new();
        MESSAGES.get_or_init(|| {
            let mut messages = StdHashMap::new();
            let mut zh_cn = StdHashMap::new();
            zh_cn.insert("welcome", "欢迎使用我们的应用");
            zh_cn.insert("goodbye", "再见");
            messages.insert("zh-CN", zh_cn);
            
            let mut en_us = StdHashMap::new();
            en_us.insert("welcome", "Welcome to our application");
            en_us.insert("goodbye", "Goodbye");
            messages.insert("en-US", en_us);
            
            Mutex::new(messages)
        })
    }
    
    fn get_current_language() -> &'static Mutex<&'static str> {
        static CURRENT_LANG: std::sync::OnceLock<Mutex<&'static str>> = std::sync::OnceLock::new();
        CURRENT_LANG.get_or_init(|| Mutex::new("en-US"))
    }
    
    fn set_language(lang: &'static str) {
        let mut current = Self::get_current_language().lock().unwrap();
        *current = lang;
    }
    
    fn get_message(key: &str) -> &'static str {
        let messages = Self::get_messages().lock().unwrap();
        let current_lang = *Self::get_current_language().lock().unwrap();
        
        messages
            .get(current_lang)
            .and_then(|lang_messages| lang_messages.get(key))
            .unwrap_or(&"Message not found")
    }
}

fn main() {
    println!("🦀 Rust 静态生命周期深度解析");
    println!("基于 https://course.rs/advance/lifetime/static.html");
    println!("{}", "=".repeat(60));
    
    // 执行所有演示
    demonstrate_static_basics();
    demonstrate_static_reference();
    demonstrate_static_bound();
    demonstrate_static_differences();
    demonstrate_static_use_cases();
    demonstrate_advanced_static_patterns();
    demonstrate_static_pitfalls();
    demonstrate_performance_considerations();
    demonstrate_best_practices();
    demonstrate_real_world_applications();
    
    println!("{}", "=".repeat(60));
    println!("✅ 静态生命周期深度解析完成！");
    println!("\n📚 关键要点总结:");
    println!("1. &'static 是引用类型，指向整个程序期间有效的数据");
    println!("2. T: 'static 是类型约束，要求类型不包含非静态引用");
    println!("3. 静态生命周期适用于全局配置、错误消息、线程共享数据");
    println!("4. 避免过度使用 'static，优先考虑生命周期参数");
    println!("5. 使用 Arc、Cow 等智能指针作为 'static 的替代方案");
    println!("6. 注意内存泄漏风险，谨慎使用 Box::leak");
    println!("7. 静态数据存储在程序数据段，具有更好的性能特征");
}
