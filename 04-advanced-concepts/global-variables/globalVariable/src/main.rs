//! Rust 全局变量深度教程
//! 
//! 本教程全面深入地探讨 Rust 中的全局变量机制，包括：
//! - 静态变量 (static)
//! - 常量 (const)
//! - 可变静态变量 (static mut)
//! - 延迟初始化 (lazy_static, once_cell)
//! - 线程局部存储 (thread_local)
//! - 原子全局变量
//! - 全局状态管理模式

use std::sync::{Mutex, Once, RwLock};
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::cell::RefCell;

// ============================================================================
// 1. 全局变量基础概念
// ============================================================================

/// 演示全局变量的基础概念
fn demonstrate_global_variable_basics() {
    println!("\n=== 1. 全局变量基础概念 ===");
    
    println!("\n1.1 全局变量的定义和特点：");
    println!("- 在程序的整个生命周期内存在");
    println!("- 存储在程序的数据段或BSS段");
    println!("- 在程序启动时初始化");
    println!("- 可以在程序的任何地方访问");
    println!("- 需要考虑线程安全性");
    
    println!("\n1.2 Rust中全局变量的类型：");
    println!("- const: 编译时常量");
    println!("- static: 静态变量");
    println!("- static mut: 可变静态变量");
    println!("- thread_local!: 线程局部变量");
    println!("- 原子类型全局变量");
    
    println!("\n1.3 作用域和生命周期：");
    println!("- 全局作用域：可以在任何模块中访问");
    println!("- 'static 生命周期：与程序生命周期相同");
    println!("- 内存布局：存储在程序的静态内存区域");
}

// ============================================================================
// 2. 静态变量 (static)
// ============================================================================

// 不可变静态变量
static GLOBAL_COUNTER: AtomicI32 = AtomicI32::new(0);
static PROGRAM_NAME: &str = "Rust全局变量教程";
static VERSION: (u32, u32, u32) = (1, 0, 0);

// 复杂类型的静态变量
static GLOBAL_CONFIG: RwLock<Option<Config>> = RwLock::new(None);

#[derive(Debug, Clone)]
struct Config {
    debug_mode: bool,
    max_connections: usize,
    timeout: Duration,
}

/// 演示静态变量的使用
fn demonstrate_static_variables() {
    println!("\n=== 2. 静态变量 (static) ===");
    
    println!("\n2.1 基本静态变量：");
    println!("程序名称: {}", PROGRAM_NAME);
    println!("版本: {}.{}.{}", VERSION.0, VERSION.1, VERSION.2);
    
    println!("\n2.2 原子静态变量：");
    let old_value = GLOBAL_COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("计数器旧值: {}, 新值: {}", old_value, GLOBAL_COUNTER.load(Ordering::SeqCst));
    
    println!("\n2.3 复杂类型静态变量：");
    // 初始化全局配置
    {
        let mut config = GLOBAL_CONFIG.write().unwrap();
        *config = Some(Config {
            debug_mode: true,
            max_connections: 100,
            timeout: Duration::from_secs(30),
        });
    }
    
    // 读取全局配置
    {
        let config = GLOBAL_CONFIG.read().unwrap();
        if let Some(ref cfg) = *config {
            println!("全局配置: {:?}", cfg);
        }
    }
    
    println!("\n2.4 静态变量的特点：");
    println!("- 编译时必须能确定初始值");
    println!("- 存储在程序的数据段");
    println!("- 整个程序生命周期内存在");
    println!("- 默认是不可变的");
    println!("- 可以使用同步原语实现线程安全");
}

// ============================================================================
// 3. 常量 (const)
// ============================================================================

// 基本常量
const MAX_BUFFER_SIZE: usize = 1024;
const PI: f64 = 3.14159265359;
const DEFAULT_NAME: &str = "未知";

// 复杂常量表达式
const BUFFER_SIZES: [usize; 4] = [256, 512, 1024, 2048];
const MAX_USERS: usize = MAX_BUFFER_SIZE * 10;

// 常量函数
const fn calculate_max_memory() -> usize {
    MAX_BUFFER_SIZE * 1024
}

const MAX_MEMORY: usize = calculate_max_memory();

/// 演示常量的使用
fn demonstrate_const_variables() {
    println!("\n=== 3. 常量 (const) ===");
    
    println!("\n3.1 基本常量：");
    println!("最大缓冲区大小: {} bytes", MAX_BUFFER_SIZE);
    println!("圆周率: {}", PI);
    println!("默认名称: {}", DEFAULT_NAME);
    
    println!("\n3.2 复杂常量表达式：");
    println!("缓冲区大小选项: {:?}", BUFFER_SIZES);
    println!("最大用户数: {}", MAX_USERS);
    println!("最大内存: {} bytes", MAX_MEMORY);
    
    println!("\n3.3 const vs static 的区别：");
    println!("const:");
    println!("  - 编译时内联到使用处");
    println!("  - 没有固定的内存地址");
    println!("  - 每次使用都是一个新的副本");
    println!("  - 更快的访问速度");
    println!("static:");
    println!("  - 有固定的内存地址");
    println!("  - 整个程序共享同一个实例");
    println!("  - 可以获取引用");
    println!("  - 支持更复杂的初始化");
    
    // 演示内联特性
    let buffer1 = vec![0u8; MAX_BUFFER_SIZE];
    let buffer2 = vec![0u8; MAX_BUFFER_SIZE];
    println!("\n3.4 常量内联演示：");
    println!("buffer1 长度: {}", buffer1.len());
    println!("buffer2 长度: {}", buffer2.len());
    println!("每个缓冲区都使用了常量的内联副本");
}

// ============================================================================
// 4. 可变静态变量 (static mut)
// ============================================================================

// 可变静态变量 - 需要 unsafe 访问
static mut GLOBAL_STATE: i32 = 0;
static mut GLOBAL_BUFFER: [u8; 1024] = [0; 1024];

// 使用 Mutex 的安全替代方案
static SAFE_GLOBAL_STATE: Mutex<i32> = Mutex::new(0);
static SAFE_GLOBAL_BUFFER: Mutex<Vec<u8>> = Mutex::new(Vec::new());

/// 演示可变静态变量
fn demonstrate_mutable_static() {
    println!("\n=== 4. 可变静态变量 (static mut) ===");
    
    println!("\n4.1 unsafe 访问可变静态变量：");
    unsafe {
        GLOBAL_STATE += 1;
        let state_value = GLOBAL_STATE;
        println!("全局状态: {}", state_value);
        
        GLOBAL_BUFFER[0] = 42;
        let buffer_value = GLOBAL_BUFFER[0];
        println!("全局缓冲区第一个字节: {}", buffer_value);
    }
    
    println!("\n4.2 static mut 的问题：");
    println!("- 需要 unsafe 代码块访问");
    println!("- 没有线程安全保证");
    println!("- 容易导致数据竞争");
    println!("- 违反 Rust 的内存安全原则");
    
    println!("\n4.3 安全的替代方案：");
    // 使用 Mutex 保护的全局状态
    {
        let mut state = SAFE_GLOBAL_STATE.lock().unwrap();
        *state += 1;
        println!("安全的全局状态: {}", *state);
    }
    
    // 使用 Mutex 保护的全局缓冲区
    {
        let mut buffer = SAFE_GLOBAL_BUFFER.lock().unwrap();
        buffer.push(42);
        buffer.push(24);
        println!("安全的全局缓冲区: {:?}", *buffer);
    }
    
    println!("\n4.4 推荐的替代方案：");
    println!("- 使用 Mutex<T> 或 RwLock<T>");
    println!("- 使用原子类型 (AtomicBool, AtomicI32 等)");
    println!("- 使用 lazy_static 或 once_cell");
    println!("- 使用 thread_local! 宏");
}

// ============================================================================
// 5. 延迟初始化
// ============================================================================

// 使用 std::sync::Once 实现延迟初始化
static INIT: Once = Once::new();
static mut EXPENSIVE_RESOURCE: Option<ExpensiveResource> = None;

#[derive(Debug)]
struct ExpensiveResource {
    data: Vec<i32>,
    name: String,
}

impl ExpensiveResource {
    fn new() -> Self {
        println!("正在创建昂贵的资源...");
        thread::sleep(Duration::from_millis(100)); // 模拟昂贵的初始化
        ExpensiveResource {
            data: (0..1000).collect(),
            name: "昂贵资源".to_string(),
        }
    }
}

// 获取延迟初始化的资源
fn get_expensive_resource() -> String {
    unsafe {
        INIT.call_once(|| {
            EXPENSIVE_RESOURCE = Some(ExpensiveResource::new());
        });
        if let Some(ref resource) = EXPENSIVE_RESOURCE {
            format!("资源名称: {}, 数据长度: {}", resource.name, resource.data.len())
        } else {
            "资源未初始化".to_string()
        }
    }
}

// 使用 Mutex + Option 的简单延迟初始化
static LAZY_CONFIG: Mutex<Option<HashMap<String, String>>> = Mutex::new(None);

fn get_config() -> HashMap<String, String> {
    let mut config = LAZY_CONFIG.lock().unwrap();
    if config.is_none() {
        println!("首次初始化配置...");
        let mut map = HashMap::new();
        map.insert("host".to_string(), "localhost".to_string());
        map.insert("port".to_string(), "8080".to_string());
        map.insert("debug".to_string(), "true".to_string());
        *config = Some(map);
    }
    config.as_ref().unwrap().clone()
}

/// 演示延迟初始化
fn demonstrate_lazy_initialization() {
    println!("\n=== 5. 延迟初始化 ===");
    
    println!("\n5.1 使用 std::sync::Once：");
    println!("第一次访问资源：");
    let resource1 = get_expensive_resource();
    println!("{}", resource1);
    
    println!("\n第二次访问资源（不会重新初始化）：");
    let resource2 = get_expensive_resource();
    println!("{}", resource2);
    
    println!("\n5.2 使用 Mutex + Option：");
    println!("第一次获取配置：");
    let config1 = get_config();
    println!("配置: {:?}", config1);
    
    println!("\n第二次获取配置（不会重新初始化）：");
    let config2 = get_config();
    println!("配置: {:?}", config2);
    
    println!("\n5.3 延迟初始化的优势：");
    println!("- 避免程序启动时的昂贵初始化");
    println!("- 只在需要时才创建资源");
    println!("- 减少内存使用");
    println!("- 提高程序启动速度");
    
    println!("\n5.4 实现方式对比：");
    println!("std::sync::Once:");
    println!("  - 标准库提供");
    println!("  - 线程安全");
    println!("  - 只能初始化一次");
    println!("  - 需要 unsafe 代码");
    println!("Mutex + Option:");
    println!("  - 简单易懂");
    println!("  - 完全安全");
    println!("  - 可以重新初始化");
    println!("  - 每次访问都需要锁");
}

// ============================================================================
// 6. 线程局部存储
// ============================================================================

// 线程局部变量
thread_local! {
    static THREAD_COUNTER: RefCell<i32> = RefCell::new(0);
    static THREAD_NAME: RefCell<String> = RefCell::new(String::from("未命名线程"));
    static THREAD_DATA: RefCell<Vec<i32>> = RefCell::new(Vec::new());
}

// 线程局部的复杂结构
thread_local! {
    static THREAD_CACHE: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

/// 线程工作函数
fn thread_worker(thread_id: usize) {
    // 设置线程名称
    THREAD_NAME.with(|name| {
        *name.borrow_mut() = format!("工作线程-{}", thread_id);
    });
    
    // 操作线程计数器
    for i in 0..5 {
        THREAD_COUNTER.with(|counter| {
            let mut c = counter.borrow_mut();
            *c += 1;
            println!("线程 {} 计数器: {}", thread_id, *c);
        });
        
        // 操作线程数据
        THREAD_DATA.with(|data| {
            data.borrow_mut().push(i * thread_id as i32);
        });
        
        thread::sleep(Duration::from_millis(10));
    }
    
    // 显示线程最终状态
    THREAD_NAME.with(|name| {
        THREAD_COUNTER.with(|counter| {
            THREAD_DATA.with(|data| {
                println!(
                    "线程 '{}' 完成，计数器: {}, 数据: {:?}",
                    name.borrow(),
                    counter.borrow(),
                    data.borrow()
                );
            });
        });
    });
}

/// 演示线程局部存储
fn demonstrate_thread_local() {
    println!("\n=== 6. 线程局部存储 ===");
    
    println!("\n6.1 线程局部变量的特点：");
    println!("- 每个线程都有独立的副本");
    println!("- 线程间不会相互影响");
    println!("- 不需要同步机制");
    println!("- 线程结束时自动清理");
    
    println!("\n6.2 多线程演示：");
    let mut handles = vec![];
    
    // 创建多个工作线程
    for i in 0..3 {
        let handle = thread::spawn(move || {
            thread_worker(i);
        });
        handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("\n6.3 主线程的线程局部变量：");
    THREAD_NAME.with(|name| {
        *name.borrow_mut() = "主线程".to_string();
    });
    
    THREAD_COUNTER.with(|counter| {
        *counter.borrow_mut() = 100;
    });
    
    THREAD_NAME.with(|name| {
        THREAD_COUNTER.with(|counter| {
            println!(
                "线程 '{}' 计数器: {}",
                name.borrow(),
                counter.borrow()
            );
        });
    });
    
    println!("\n6.4 线程局部缓存演示：");
    THREAD_CACHE.with(|cache| {
        let mut c = cache.borrow_mut();
        c.insert("key1".to_string(), "value1".to_string());
        c.insert("key2".to_string(), "value2".to_string());
        println!("主线程缓存: {:?}", *c);
    });
}

// ============================================================================
// 7. 原子全局变量
// ============================================================================

// 各种原子类型的全局变量
static ATOMIC_COUNTER: AtomicUsize = AtomicUsize::new(0);
static ATOMIC_FLAG: AtomicBool = AtomicBool::new(false);
static ATOMIC_STATE: AtomicI32 = AtomicI32::new(-1);

// 原子操作演示
fn atomic_worker(worker_id: usize) {
    for i in 0..10 {
        // 原子递增
        let old_count = ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst);
        println!("工作线程 {} 第 {} 次操作，计数器: {} -> {}", 
                worker_id, i, old_count, old_count + 1);
        
        // 原子比较交换
        if i == 5 {
            let expected = -1;
            let new_value = worker_id as i32;
            match ATOMIC_STATE.compare_exchange(expected, new_value, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(old) => println!("工作线程 {} 成功设置状态: {} -> {}", worker_id, old, new_value),
                Err(current) => println!("工作线程 {} 设置状态失败，当前值: {}", worker_id, current),
            }
        }
        
        thread::sleep(Duration::from_millis(1));
    }
    
    // 设置完成标志
    if worker_id == 0 {
        ATOMIC_FLAG.store(true, Ordering::SeqCst);
        println!("工作线程 {} 设置完成标志", worker_id);
    }
}

/// 演示原子全局变量
fn demonstrate_atomic_globals() {
    println!("\n=== 7. 原子全局变量 ===");
    
    println!("\n7.1 原子类型的特点：");
    println!("- 无锁并发访问");
    println!("- 硬件级别的原子操作");
    println!("- 比互斥锁更高效");
    println!("- 适合简单的共享状态");
    
    println!("\n7.2 多线程原子操作演示：");
    let mut handles = vec![];
    
    // 重置原子变量
    ATOMIC_COUNTER.store(0, Ordering::SeqCst);
    ATOMIC_FLAG.store(false, Ordering::SeqCst);
    ATOMIC_STATE.store(-1, Ordering::SeqCst);
    
    // 创建多个工作线程
    for i in 0..3 {
        let handle = thread::spawn(move || {
            atomic_worker(i);
        });
        handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("\n7.3 最终结果：");
    println!("原子计数器: {}", ATOMIC_COUNTER.load(Ordering::SeqCst));
    println!("原子标志: {}", ATOMIC_FLAG.load(Ordering::SeqCst));
    println!("原子状态: {}", ATOMIC_STATE.load(Ordering::SeqCst));
    
    println!("\n7.4 内存顺序说明：");
    println!("- Relaxed: 最宽松，只保证原子性");
    println!("- Acquire: 获取语义，防止后续读写重排到前面");
    println!("- Release: 释放语义，防止前面读写重排到后面");
    println!("- AcqRel: 同时具有获取和释放语义");
    println!("- SeqCst: 顺序一致性，最严格的内存顺序");
}

// ============================================================================
// 8. 全局状态管理模式
// ============================================================================

// 单例模式实现
struct DatabaseConnection {
    url: String,
    connected: bool,
}

impl DatabaseConnection {
    fn new(url: &str) -> Self {
        println!("创建数据库连接: {}", url);
        DatabaseConnection {
            url: url.to_string(),
            connected: true,
        }
    }
    
    fn query(&self, sql: &str) -> String {
        if self.connected {
            format!("执行查询 '{}' 在 {}", sql, self.url)
        } else {
            "连接已断开".to_string()
        }
    }
}

// 全局数据库连接单例
static DB_CONNECTION: Mutex<Option<DatabaseConnection>> = Mutex::new(None);

fn get_db_connection() -> std::sync::MutexGuard<'static, Option<DatabaseConnection>> {
    let mut conn = DB_CONNECTION.lock().unwrap();
    if conn.is_none() {
        *conn = Some(DatabaseConnection::new("postgresql://localhost:5432/mydb"));
    }
    conn
}

// 配置管理器
#[derive(Debug, Clone)]
struct AppConfig {
    server_port: u16,
    database_url: String,
    log_level: String,
    max_connections: usize,
}

static APP_CONFIG: RwLock<Option<AppConfig>> = RwLock::new(None);

fn init_config() {
    let mut config = APP_CONFIG.write().unwrap();
    *config = Some(AppConfig {
        server_port: 8080,
        database_url: "postgresql://localhost:5432/app".to_string(),
        log_level: "info".to_string(),
        max_connections: 100,
    });
}

fn get_config_value<T, F>(getter: F) -> Option<T>
where
    F: FnOnce(&AppConfig) -> T,
{
    let config = APP_CONFIG.read().unwrap();
    config.as_ref().map(getter)
}

// 缓存系统
struct Cache {
    data: HashMap<String, String>,
    hits: usize,
    misses: usize,
}

impl Cache {
    fn new() -> Self {
        Cache {
            data: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }
    
    fn get(&mut self, key: &str) -> Option<&String> {
        match self.data.get(key) {
            Some(value) => {
                self.hits += 1;
                Some(value)
            }
            None => {
                self.misses += 1;
                None
            }
        }
    }
    
    fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
    
    fn stats(&self) -> (usize, usize) {
        (self.hits, self.misses)
    }
}

// 使用 lazy_static 或者简化的初始化
static GLOBAL_CACHE: Mutex<Option<Cache>> = Mutex::new(None);

fn get_global_cache() -> std::sync::MutexGuard<'static, Option<Cache>> {
    let mut cache = GLOBAL_CACHE.lock().unwrap();
    if cache.is_none() {
        *cache = Some(Cache::new());
    }
    cache
}

// 资源池
struct ConnectionPool {
    connections: Vec<String>,
    in_use: Vec<bool>,
    max_size: usize,
}

impl ConnectionPool {
    fn new(max_size: usize) -> Self {
        let mut connections = Vec::new();
        let mut in_use = Vec::new();
        
        for i in 0..max_size {
            connections.push(format!("连接-{}", i));
            in_use.push(false);
        }
        
        ConnectionPool {
            connections,
            in_use,
            max_size,
        }
    }
    
    fn acquire(&mut self) -> Option<(usize, String)> {
        for (i, in_use) in self.in_use.iter_mut().enumerate() {
            if !*in_use {
                *in_use = true;
                return Some((i, self.connections[i].clone()));
            }
        }
        None
    }
    
    fn release(&mut self, index: usize) {
        if index < self.max_size {
            self.in_use[index] = false;
        }
    }
    
    fn stats(&self) -> (usize, usize) {
        let in_use_count = self.in_use.iter().filter(|&&x| x).count();
        (in_use_count, self.max_size - in_use_count)
    }
}

static CONNECTION_POOL: Mutex<ConnectionPool> = Mutex::new(ConnectionPool {
    connections: Vec::new(),
    in_use: Vec::new(),
    max_size: 0,
});

fn init_connection_pool() {
    let mut pool = CONNECTION_POOL.lock().unwrap();
    *pool = ConnectionPool::new(5);
}

/// 演示全局状态管理模式
fn demonstrate_global_state_patterns() {
    println!("\n=== 8. 全局状态管理模式 ===");
    
    println!("\n8.1 单例模式 - 数据库连接：");
    {
        let conn = get_db_connection();
        if let Some(ref db) = *conn {
            println!("{}", db.query("SELECT * FROM users"));
        }
    }
    
    // 第二次访问，不会重新创建
    {
        let conn = get_db_connection();
        if let Some(ref db) = *conn {
            println!("{}", db.query("SELECT COUNT(*) FROM orders"));
        }
    }
    
    println!("\n8.2 配置管理器：");
    init_config();
    
    if let Some(port) = get_config_value(|config| config.server_port) {
        println!("服务器端口: {}", port);
    }
    
    if let Some(db_url) = get_config_value(|config| config.database_url.clone()) {
        println!("数据库URL: {}", db_url);
    }
    
    if let Some(log_level) = get_config_value(|config| config.log_level.clone()) {
        println!("日志级别: {}", log_level);
    }
    
    println!("\n8.3 缓存系统：");
    {
        let mut cache_guard = get_global_cache();
        if let Some(ref mut cache) = *cache_guard {
            // 设置缓存
            cache.set("user:1".to_string(), "Alice".to_string());
            cache.set("user:2".to_string(), "Bob".to_string());
            
            // 获取缓存
            if let Some(value) = cache.get("user:1") {
                println!("缓存命中: user:1 = {}", value);
            }
            
            if let Some(value) = cache.get("user:2") {
                println!("缓存命中: user:2 = {}", value);
            }
            
            // 缓存未命中
            if cache.get("user:3").is_none() {
                println!("缓存未命中: user:3");
            }
            
            let (hits, misses) = cache.stats();
            println!("缓存统计 - 命中: {}, 未命中: {}", hits, misses);
        }
    }
    
    println!("\n8.4 连接池：");
    init_connection_pool();
    
    {
        let mut pool = CONNECTION_POOL.lock().unwrap();
        
        // 获取连接
        let mut acquired_connections = Vec::new();
        
        // 获取第一个连接
        if let Some((index1, conn1_name)) = pool.acquire() {
            println!("获取连接: {} (索引: {})", conn1_name, index1);
            acquired_connections.push(index1);
        }
        
        // 获取第二个连接
        if let Some((index2, conn2_name)) = pool.acquire() {
            println!("获取连接: {} (索引: {})", conn2_name, index2);
            acquired_connections.push(index2);
        }
        
        // 显示连接池状态
        let (in_use, available) = pool.stats();
        println!("连接池状态 - 使用中: {}, 可用: {}", in_use, available);
        
        // 释放连接
        for index in &acquired_connections {
            pool.release(*index);
            println!("释放连接: 索引 {}", index);
        }
        
        // 显示最终状态
        let (final_in_use, final_available) = pool.stats();
        println!("连接池最终状态 - 使用中: {}, 可用: {}", final_in_use, final_available);
    }
    
    println!("\n8.5 全局状态管理最佳实践：");
    println!("- 使用适当的同步原语 (Mutex, RwLock, Atomic)");
    println!("- 避免过度使用全局状态");
    println!("- 考虑依赖注入替代全局状态");
    println!("- 使用类型安全的封装");
    println!("- 提供清晰的初始化和清理机制");
    println!("- 考虑线程安全性和性能影响");
}

// ============================================================================
// 主函数 - 运行所有演示
// ============================================================================

fn main() {
    println!("🦀 Rust 全局变量深度教程");
    println!("{}", "=".repeat(50));
    
    // 1. 全局变量基础概念
    demonstrate_global_variable_basics();
    
    // 2. 静态变量
    demonstrate_static_variables();
    
    // 3. 常量
    demonstrate_const_variables();
    
    // 4. 可变静态变量
    demonstrate_mutable_static();
    
    // 5. 延迟初始化
    demonstrate_lazy_initialization();
    
    // 6. 线程局部存储
    demonstrate_thread_local();
    
    // 7. 原子全局变量
    demonstrate_atomic_globals();
    
    // 8. 全局状态管理模式
    demonstrate_global_state_patterns();
    
    println!("\n{}", "=".repeat(50));
    println!("🎉 教程完成！");
    println!("\n📚 总结：");
    println!("- 掌握了 Rust 中各种全局变量的使用方法");
    println!("- 理解了线程安全和内存安全的重要性");
    println!("- 学会了选择合适的全局状态管理模式");
    println!("- 了解了最佳实践和常见陷阱");
    
    println!("\n🚀 下一步学习建议：");
    println!("- 深入学习 Rust 的内存模型");
    println!("- 探索异步编程中的全局状态管理");
    println!("- 学习更高级的并发模式");
    println!("- 实践构建大型应用的状态管理架构");
}

// ============================================================================
// 测试模块
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_static_variables() {
        // 测试静态变量的基本功能
        assert_eq!(PROGRAM_NAME, "Rust全局变量教程");
        assert_eq!(VERSION, (1, 0, 0));
        
        // 测试原子操作
        let old_value = GLOBAL_COUNTER.load(Ordering::SeqCst);
        GLOBAL_COUNTER.fetch_add(1, Ordering::SeqCst);
        assert_eq!(GLOBAL_COUNTER.load(Ordering::SeqCst), old_value + 1);
    }
    
    #[test]
    fn test_const_variables() {
        // 测试常量
        assert_eq!(MAX_BUFFER_SIZE, 1024);
        assert_eq!(MAX_USERS, 10240);
        assert_eq!(MAX_MEMORY, 1048576);
        
        // 测试常量数组
        assert_eq!(BUFFER_SIZES.len(), 4);
        assert_eq!(BUFFER_SIZES[0], 256);
    }
    
    #[test]
    fn test_atomic_operations() {
        // 测试原子操作
        let counter = AtomicI32::new(0);
        
        // 基本操作
        assert_eq!(counter.load(Ordering::SeqCst), 0);
        counter.store(42, Ordering::SeqCst);
        assert_eq!(counter.load(Ordering::SeqCst), 42);
        
        // fetch_add
        let old = counter.fetch_add(8, Ordering::SeqCst);
        assert_eq!(old, 42);
        assert_eq!(counter.load(Ordering::SeqCst), 50);
        
        // compare_exchange
        let result = counter.compare_exchange(50, 100, Ordering::SeqCst, Ordering::SeqCst);
        assert_eq!(result, Ok(50));
        assert_eq!(counter.load(Ordering::SeqCst), 100);
    }
    
    #[test]
    fn test_thread_local() {
        thread_local! {
            static TEST_COUNTER: RefCell<i32> = RefCell::new(0);
        }
        
        TEST_COUNTER.with(|counter| {
            *counter.borrow_mut() = 42;
        });
        
        TEST_COUNTER.with(|counter| {
            assert_eq!(*counter.borrow(), 42);
        });
    }
    
    #[test]
    fn test_cache_system() {
        let mut cache = Cache::new();
        
        // 测试缓存设置和获取
        cache.set("key1".to_string(), "value1".to_string());
        assert_eq!(cache.get("key1"), Some(&"value1".to_string()));
        assert_eq!(cache.get("key2"), None);
        
        // 测试统计
        let (hits, misses) = cache.stats();
        assert_eq!(hits, 1);
        assert_eq!(misses, 1);
    }
    
    #[test]
    fn test_connection_pool() {
        let mut pool = ConnectionPool::new(3);
        
        // 测试连接获取
        let conn1 = pool.acquire();
        assert!(conn1.is_some());
        
        let conn2 = pool.acquire();
        assert!(conn2.is_some());
        
        let conn3 = pool.acquire();
        assert!(conn3.is_some());
        
        // 池已满
        let conn4 = pool.acquire();
        assert!(conn4.is_none());
        
        // 释放连接
        if let Some((index, _)) = conn1 {
            pool.release(index);
            let conn5 = pool.acquire();
            assert!(conn5.is_some());
        }
    }
}
