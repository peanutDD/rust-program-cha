//! # Move、Copy 和 Clone 的详细对比分析
//!
//! 本模块提供 Move、Copy 和 Clone 三种机制的全面对比分析，
//! 包括概念差异、性能对比、使用场景和选择指南。

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Instant;

/// 核心概念对比
pub fn core_concepts_comparison() {
    println!("=== Move、Copy 和 Clone 核心概念对比 ===");
    
    println!("\n📋 基本定义对比:");
    println!("┌─────────────┬──────────────────────────────────────────────────────────┐");
    println!("│ 机制        │ 定义                                                     │");
    println!("├─────────────┼──────────────────────────────────────────────────────────┤");
    println!("│ Move        │ 所有权转移，原变量失效，零运行时成本                     │");
    println!("│ Copy        │ 按位复制，原变量保持有效，栈上操作                       │");
    println!("│ Clone       │ 显式深拷贝，可自定义复制行为，可能涉及堆分配             │");
    println!("└─────────────┴──────────────────────────────────────────────────────────┘");
    
    println!("\n🔄 操作方式对比:");
    println!("┌─────────────┬──────────────────────────────────────────────────────────┐");
    println!("│ 机制        │ 触发方式                                                 │");
    println!("├─────────────┼──────────────────────────────────────────────────────────┤");
    println!("│ Move        │ 自动触发（赋值、函数参数、返回值）                       │");
    println!("│ Copy        │ 自动触发（实现Copy trait的类型）                        │");
    println!("│ Clone       │ 显式调用 .clone() 方法                                  │");
    println!("└─────────────┴──────────────────────────────────────────────────────────┘");
    
    println!("\n💾 内存行为对比:");
    println!("┌─────────────┬──────────────────────────────────────────────────────────┐");
    println!("│ 机制        │ 内存操作                                                 │");
    println!("├─────────────┼──────────────────────────────────────────────────────────┤");
    println!("│ Move        │ 转移栈上指针/数据，不复制堆内容                         │");
    println!("│ Copy        │ 按位复制栈上所有数据                                     │");
    println!("│ Clone       │ 可能复制栈和堆上的数据，取决于实现                       │");
    println!("└─────────────┴──────────────────────────────────────────────────────────┘");
    
    // 实际演示
    demonstrate_basic_differences();
}

/// 演示基本差异
fn demonstrate_basic_differences() {
    println!("\n🔍 实际演示:");
    
    // Move 示例
    println!("\n1️⃣ Move 示例:");
    let s1 = String::from("Hello");
    println!("   创建 s1: {}", s1);
    let s2 = s1; // Move 发生
    println!("   s1 移动到 s2: {}", s2);
    // println!("   s1: {}", s1); // 编译错误！s1 已失效
    println!("   ❌ s1 不再可用");
    
    // Copy 示例
    println!("\n2️⃣ Copy 示例:");
    let x1 = 42;
    println!("   创建 x1: {}", x1);
    let x2 = x1; // Copy 发生
    println!("   x1 复制到 x2: {}", x2);
    println!("   ✅ x1 仍可用: {}", x1);
    
    // Clone 示例
    println!("\n3️⃣ Clone 示例:");
    let v1 = vec![1, 2, 3];
    println!("   创建 v1: {:?}", v1);
    let v2 = v1.clone(); // 显式 Clone
    println!("   v1 克隆到 v2: {:?}", v2);
    println!("   ✅ v1 仍可用: {:?}", v1);
    println!("   📍 v1 和 v2 在不同内存地址");
}

/// 类型系统对比
pub fn type_system_comparison() {
    println!("\n=== 类型系统对比 ===");
    
    println!("\n🏷️ Trait 要求对比:");
    println!("┌─────────────┬──────────────────────────────────────────────────────────┐");
    println!("│ 机制        │ Trait 要求                                               │");
    println!("├─────────────┼──────────────────────────────────────────────────────────┤");
    println!("│ Move        │ 无特殊要求（默认行为）                                   │");
    println!("│ Copy        │ 必须实现 Copy + Clone trait                              │");
    println!("│ Clone       │ 必须实现 Clone trait                                     │");
    println!("└─────────────┴──────────────────────────────────────────────────────────┘");
    
    println!("\n📝 实现条件对比:");
    demonstrate_implementation_conditions();
    
    println!("\n🔗 Trait 关系:");
    demonstrate_trait_relationships();
}

/// 演示实现条件
fn demonstrate_implementation_conditions() {
    println!("\n1️⃣ 可以实现 Copy 的类型:");
    
    #[derive(Debug, Copy, Clone)]
    struct CopyableStruct {
        x: i32,
        y: f64,
        flag: bool,
    }
    
    let copyable1 = CopyableStruct { x: 10, y: 3.14, flag: true };
    let copyable2 = copyable1; // Copy 发生
    println!("   ✅ 简单值类型结构体可以实现 Copy");
    println!("   copyable1: {:?}", copyable1);
    println!("   copyable2: {:?}", copyable2);
    
    println!("\n2️⃣ 不能实现 Copy 的类型:");
    
    #[derive(Debug, Clone)] // 不能 derive Copy
    struct NonCopyableStruct {
        data: String, // String 不实现 Copy
        items: Vec<i32>, // Vec 不实现 Copy
    }
    
    let non_copyable1 = NonCopyableStruct {
        data: String::from("data"),
        items: vec![1, 2, 3],
    };
    let non_copyable2 = non_copyable1.clone(); // 必须显式 clone
    println!("   ❌ 包含堆分配类型的结构体不能实现 Copy");
    println!("   non_copyable2: {:?}", non_copyable2);
    
    println!("\n3️⃣ 只能 Clone 的类型:");
    
    #[derive(Debug, Clone)]
    struct CloneOnlyStruct {
        shared_data: Rc<Vec<i32>>,
        metadata: HashMap<String, String>,
    }
    
    let clone_only1 = CloneOnlyStruct {
        shared_data: Rc::new(vec![1, 2, 3, 4, 5]),
        metadata: {
            let mut map = HashMap::new();
            map.insert("key".to_string(), "value".to_string());
            map
        },
    };
    let clone_only2 = clone_only1.clone();
    println!("   📋 复杂类型只能实现 Clone");
    println!("   引用计数: {}", Rc::strong_count(&clone_only1.shared_data));
}

/// 演示 Trait 关系
fn demonstrate_trait_relationships() {
    println!("\n🔗 Trait 继承关系:");
    println!("   Copy: Clone  (Copy 是 Clone 的子 trait)");
    println!("   实现 Copy 必须同时实现 Clone");
    
    // 演示 Copy 类型的 Clone 行为
    #[derive(Debug, Copy, Clone)]
    struct Point { x: i32, y: i32 }
    
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1; // Copy
    let p3 = p1.clone(); // Clone（实际上调用 Copy）
    
    println!("\n   对于 Copy 类型:");
    println!("   p1 (原始): {:?}", p1);
    println!("   p2 (Copy): {:?}", p2);
    println!("   p3 (Clone): {:?}", p3);
    println!("   所有变量都可用，Clone 等效于 Copy");
}

/// 性能对比分析
pub fn performance_comparison() {
    println!("\n=== 性能对比分析 ===");
    
    println!("\n⚡ 理论性能对比:");
    println!("┌─────────────┬──────────────────────────────────────────────────────────┐");
    println!("│ 机制        │ 性能特征                                                 │");
    println!("├─────────────┼──────────────────────────────────────────────────────────┤");
    println!("│ Move        │ O(1) - 零成本，只转移所有权                             │");
    println!("│ Copy        │ O(n) - n为数据大小，栈上按位复制                        │");
    println!("│ Clone       │ O(n) - n为数据复杂度，可能涉及堆分配                    │");
    println!("└─────────────┴──────────────────────────────────────────────────────────┘");
    
    // 实际性能测试
    benchmark_all_mechanisms();
}

/// 综合性能基准测试
fn benchmark_all_mechanisms() {
    println!("\n📊 实际性能测试:");
    
    const ITERATIONS: usize = 100_000;
    
    // 1. 小型数据性能测试
    println!("\n1️⃣ 小型数据 (12 bytes) 性能测试:");
    benchmark_small_data(ITERATIONS);
    
    // 2. 中型数据性能测试
    println!("\n2️⃣ 中型数据 (1KB) 性能测试:");
    benchmark_medium_data(ITERATIONS / 10);
    
    // 3. 大型数据性能测试
    println!("\n3️⃣ 大型数据 (1MB) 性能测试:");
    benchmark_large_data(ITERATIONS / 100);
    
    // 4. 复杂结构性能测试
    println!("\n4️⃣ 复杂结构性能测试:");
    benchmark_complex_data(ITERATIONS / 10);
}

/// 小型数据基准测试
fn benchmark_small_data(iterations: usize) {
    #[derive(Copy, Clone)]
    struct SmallData {
        a: i32,
        b: i32,
        c: i32,
    }
    
    let data = SmallData { a: 1, b: 2, c: 3 };
    
    // Move 测试（通过函数参数）
    fn consume_move(data: SmallData) -> SmallData { data }
    let start = Instant::now();
    for _ in 0..iterations {
        let temp = SmallData { a: 1, b: 2, c: 3 };
        let _result = consume_move(temp);
    }
    let move_duration = start.elapsed();
    
    // Copy 测试
    let start = Instant::now();
    for _ in 0..iterations {
        let _temp = data; // Copy
    }
    let copy_duration = start.elapsed();
    
    // Clone 测试
    let start = Instant::now();
    for _ in 0..iterations {
        let _temp = data.clone(); // Clone
    }
    let clone_duration = start.elapsed();
    
    println!("   Move:  {:?} ({:.2} ns/op)", move_duration, move_duration.as_nanos() as f64 / iterations as f64);
    println!("   Copy:  {:?} ({:.2} ns/op)", copy_duration, copy_duration.as_nanos() as f64 / iterations as f64);
    println!("   Clone: {:?} ({:.2} ns/op)", clone_duration, clone_duration.as_nanos() as f64 / iterations as f64);
}

/// 中型数据基准测试
fn benchmark_medium_data(iterations: usize) {
    #[derive(Copy, Clone)]
    struct MediumData {
        data: [u8; 1024], // 1KB
    }
    
    let data = MediumData { data: [42; 1024] };
    
    // Move 测试
    fn consume_move(data: MediumData) -> MediumData { data }
    let start = Instant::now();
    for _ in 0..iterations {
        let temp = MediumData { data: [42; 1024] };
        let _result = consume_move(temp);
    }
    let move_duration = start.elapsed();
    
    // Copy 测试
    let start = Instant::now();
    for _ in 0..iterations {
        let _temp = data; // Copy
    }
    let copy_duration = start.elapsed();
    
    // Clone 测试
    let start = Instant::now();
    for _ in 0..iterations {
        let _temp = data.clone(); // Clone
    }
    let clone_duration = start.elapsed();
    
    println!("   Move:  {:?} ({:.2} μs/op)", move_duration, move_duration.as_micros() as f64 / iterations as f64);
    println!("   Copy:  {:?} ({:.2} μs/op)", copy_duration, copy_duration.as_micros() as f64 / iterations as f64);
    println!("   Clone: {:?} ({:.2} μs/op)", clone_duration, clone_duration.as_micros() as f64 / iterations as f64);
}

/// 大型数据基准测试
fn benchmark_large_data(iterations: usize) {
    // 使用 Vec 模拟大型数据
    let large_vec = vec![42u8; 1024 * 1024]; // 1MB
    
    // Move 测试
    let start = Instant::now();
    for _ in 0..iterations {
        let temp = vec![42u8; 1024 * 1024];
        let _moved = temp; // Move
    }
    let move_duration = start.elapsed();
    
    // Clone 测试
    let start = Instant::now();
    for _ in 0..iterations {
        let _cloned = large_vec.clone(); // Clone
    }
    let clone_duration = start.elapsed();
    
    println!("   Move:  {:?} ({:.2} ms/op)", move_duration, move_duration.as_millis() as f64 / iterations as f64);
    println!("   Clone: {:?} ({:.2} ms/op)", clone_duration, clone_duration.as_millis() as f64 / iterations as f64);
    println!("   📝 大型数据：Move 显著快于 Clone");
}

/// 复杂结构基准测试
fn benchmark_complex_data(iterations: usize) {
    #[derive(Clone)]
    struct ComplexData {
        strings: Vec<String>,
        map: HashMap<String, Vec<i32>>,
        shared: Rc<Vec<u8>>,
    }
    
    let complex_data = ComplexData {
        strings: (0..100).map(|i| format!("string_{}", i)).collect(),
        map: {
            let mut map = HashMap::new();
            for i in 0..50 {
                map.insert(format!("key_{}", i), (0..20).collect());
            }
            map
        },
        shared: Rc::new(vec![42; 1000]),
    };
    
    // Move 测试
    let start = Instant::now();
    for _ in 0..iterations {
        let temp = ComplexData {
            strings: vec![String::from("test")],
            map: HashMap::new(),
            shared: Rc::new(vec![1]),
        };
        let _moved = temp; // Move
    }
    let move_duration = start.elapsed();
    
    // Clone 测试
    let start = Instant::now();
    for _ in 0..iterations {
        let _cloned = complex_data.clone(); // Clone
    }
    let clone_duration = start.elapsed();
    
    println!("   Move:  {:?} ({:.2} μs/op)", move_duration, move_duration.as_micros() as f64 / iterations as f64);
    println!("   Clone: {:?} ({:.2} μs/op)", clone_duration, clone_duration.as_micros() as f64 / iterations as f64);
    println!("   📝 复杂结构：Clone 成本取决于数据复杂度");
}

/// 使用场景对比
pub fn use_case_comparison() {
    println!("\n=== 使用场景对比 ===");
    
    println!("\n🎯 适用场景对比:");
    println!("┌─────────────┬──────────────────────────────────────────────────────────┐");
    println!("│ 机制        │ 最佳使用场景                                             │");
    println!("├─────────────┼──────────────────────────────────────────────────────────┤");
    println!("│ Move        │ • 转移大对象所有权                                       │");
    println!("│             │ • 避免不必要的复制                                       │");
    println!("│             │ • 函数返回值                                             │");
    println!("│             │ • 构建器模式                                             │");
    println!("├─────────────┼──────────────────────────────────────────────────────────┤");
    println!("│ Copy        │ • 小型值类型                                             │");
    println!("│             │ • 频繁传递的简单数据                                     │");
    println!("│             │ • 数值计算                                               │");
    println!("│             │ • 配置参数                                               │");
    println!("├─────────────┼──────────────────────────────────────────────────────────┤");
    println!("│ Clone       │ • 需要独立副本                                           │");
    println!("│             │ • 复杂数据结构                                           │");
    println!("│             │ • 原型模式                                               │");
    println!("│             │ • 缓存和备份                                             │");
    println!("└─────────────┴──────────────────────────────────────────────────────────┘");
    
    // 实际场景演示
    demonstrate_use_cases();
}

/// 演示实际使用场景
fn demonstrate_use_cases() {
    println!("\n🔍 实际场景演示:");
    
    // Move 场景：构建器模式
    println!("\n1️⃣ Move 场景 - 构建器模式:");
    demonstrate_move_use_case();
    
    // Copy 场景：配置传递
    println!("\n2️⃣ Copy 场景 - 配置传递:");
    demonstrate_copy_use_case();
    
    // Clone 场景：数据备份
    println!("\n3️⃣ Clone 场景 - 数据备份:");
    demonstrate_clone_use_case();
}

/// 演示 Move 使用场景
fn demonstrate_move_use_case() {
    #[derive(Debug)]
    struct DatabaseConnection {
        url: String,
        pool_size: usize,
        timeout: u64,
    }
    
    struct ConnectionBuilder {
        connection: DatabaseConnection,
    }
    
    impl ConnectionBuilder {
        fn new(url: String) -> Self {
            ConnectionBuilder {
                connection: DatabaseConnection {
                    url,
                    pool_size: 10,
                    timeout: 30,
                },
            }
        }
        
        fn pool_size(mut self, size: usize) -> Self {
            self.connection.pool_size = size;
            self // Move self
        }
        
        fn timeout(mut self, timeout: u64) -> Self {
            self.connection.timeout = timeout;
            self // Move self
        }
        
        fn build(self) -> DatabaseConnection {
            self.connection // Move connection
        }
    }
    
    let connection = ConnectionBuilder::new(String::from("postgresql://localhost:5432/db"))
        .pool_size(20)
        .timeout(60)
        .build();
    
    println!("   ✅ 构建器模式使用 Move 避免不必要的复制");
    println!("   连接配置: {:?}", connection);
}

/// 演示 Copy 使用场景
fn demonstrate_copy_use_case() {
    #[derive(Debug, Copy, Clone)]
    struct RenderConfig {
        width: u32,
        height: u32,
        anti_aliasing: bool,
        vsync: bool,
    }
    
    fn render_frame(config: RenderConfig) {
        println!("   渲染帧: {}x{}, AA: {}, VSync: {}", 
                config.width, config.height, config.anti_aliasing, config.vsync);
    }
    
    fn update_ui(config: RenderConfig) {
        println!("   更新UI: {}x{}", config.width, config.height);
    }
    
    fn calculate_fps(config: RenderConfig) -> f64 {
        if config.vsync { 60.0 } else { 120.0 }
    }
    
    let config = RenderConfig {
        width: 1920,
        height: 1080,
        anti_aliasing: true,
        vsync: true,
    };
    
    // 可以多次传递，无需担心所有权
    render_frame(config);
    update_ui(config);
    let fps = calculate_fps(config);
    
    println!("   ✅ Copy 类型可以轻松传递给多个函数");
    println!("   计算得到 FPS: {}", fps);
}

/// 演示 Clone 使用场景
fn demonstrate_clone_use_case() {
    #[derive(Debug, Clone)]
    struct UserData {
        id: u64,
        name: String,
        email: String,
        preferences: HashMap<String, String>,
        history: Vec<String>,
    }
    
    impl UserData {
        fn new(id: u64, name: String, email: String) -> Self {
            UserData {
                id,
                name,
                email,
                preferences: HashMap::new(),
                history: Vec::new(),
            }
        }
        
        fn add_preference(&mut self, key: String, value: String) {
            self.preferences.insert(key, value);
        }
        
        fn add_history(&mut self, action: String) {
            self.history.push(action);
        }
    }
    
    let mut user = UserData::new(
        12345,
        String::from("Alice"),
        String::from("alice@example.com"),
    );
    
    user.add_preference("theme".to_string(), "dark".to_string());
    user.add_preference("language".to_string(), "zh-CN".to_string());
    user.add_history("login".to_string());
    user.add_history("view_profile".to_string());
    
    // 创建备份
    let backup = user.clone();
    
    // 继续修改原数据
    user.add_history("logout".to_string());
    
    println!("   ✅ Clone 创建独立副本用于备份");
    println!("   原始用户历史记录数: {}", user.history.len());
    println!("   备份用户历史记录数: {}", backup.history.len());
    println!("   备份不受后续修改影响");
}

/// 选择指南
pub fn selection_guide() {
    println!("\n=== 选择指南 ===");
    
    println!("\n🤔 如何选择合适的机制？");
    
    println!("\n📋 决策流程图:");
    println!("```");
    println!("需要复制数据？");
    println!("├─ 否 → 使用 Move（默认行为）");
    println!("└─ 是 → 数据类型是什么？");
    println!("    ├─ 简单值类型（无堆分配）→ 实现 Copy");
    println!("    ├─ 复杂类型（有堆分配）→ 实现 Clone");
    println!("    └─ 需要自定义复制逻辑 → 手动实现 Clone");
    println!("```");
    
    println!("\n✅ 最佳实践建议:");
    
    println!("\n1️⃣ 优先级排序:");
    println!("   1. Move（零成本，默认行为）");
    println!("   2. Copy（简单高效，适用于小型数据）");
    println!("   3. Clone（灵活但可能昂贵）");
    
    println!("\n2️⃣ 类型设计建议:");
    println!("   • 小型值类型：优先实现 Copy");
    println!("   • 大型数据：考虑使用智能指针 + Clone");
    println!("   • 资源类型：只使用 Move，不实现 Copy/Clone");
    
    println!("\n3️⃣ 性能考虑:");
    println!("   • 频繁传递的小数据：Copy");
    println!("   • 偶尔复制的大数据：Clone");
    println!("   • 一次性转移：Move");
    
    println!("\n4️⃣ 内存安全:");
    println!("   • Move 提供最强的内存安全保证");
    println!("   • Copy 适用于无资源管理的类型");
    println!("   • Clone 需要正确实现以避免内存泄漏");
    
    // 实际选择示例
    demonstrate_selection_examples();
}

/// 演示选择示例
fn demonstrate_selection_examples() {
    println!("\n🔍 选择示例:");
    
    println!("\n1️⃣ 坐标点 - 选择 Copy:");
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point { x: f64, y: f64 }
    
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1; // Copy
    println!("   理由：小型值类型，频繁使用，Copy 最合适");
    println!("   p1: {:?}, p2: {:?}", p1, p2);
    
    println!("\n2️⃣ 用户信息 - 选择 Clone:");
    #[derive(Debug, Clone)]
    struct User {
        name: String,
        emails: Vec<String>,
    }
    
    let user1 = User {
        name: String::from("Alice"),
        emails: vec![String::from("alice@example.com")],
    };
    let user2 = user1.clone(); // Clone
    println!("   理由：包含堆分配数据，需要深拷贝，Clone 合适");
    println!("   user2: {:?}", user2);
    
    println!("\n3️⃣ 文件句柄 - 选择 Move:");
    #[derive(Debug)]
    struct FileHandle {
        path: String,
        fd: i32,
    }
    
    impl Drop for FileHandle {
        fn drop(&mut self) {
            println!("   关闭文件: {}", self.path);
        }
    }
    
    let file1 = FileHandle {
        path: String::from("/tmp/test.txt"),
        fd: 42,
    };
    let file2 = file1; // Move
    println!("   理由：资源类型，不应复制，Move 确保唯一所有权");
    println!("   file2: {:?}", file2);
    // file1 已失效，file2 拥有所有权
}

/// 常见陷阱和误区
pub fn common_pitfalls() {
    println!("\n=== 常见陷阱和误区 ===");
    
    println!("\n⚠️ 常见误区:");
    
    println!("\n1️⃣ 误区：认为 Copy 总是比 Clone 快");
    println!("   现实：大型 Copy 类型可能比智能指针的 Clone 更慢");
    demonstrate_copy_vs_clone_pitfall();
    
    println!("\n2️⃣ 误区：过度使用 Clone 避免借用检查器");
    println!("   现实：应该学会正确使用借用，而不是逃避");
    demonstrate_clone_overuse_pitfall();
    
    println!("\n3️⃣ 误区：认为 Move 会复制数据");
    println!("   现实：Move 只转移所有权，不复制数据");
    demonstrate_move_misconception();
    
    println!("\n4️⃣ 误区：在 Copy 类型上实现 Drop");
    println!("   现实：Copy 类型不能实现 Drop trait");
    demonstrate_copy_drop_conflict();
}

/// 演示 Copy vs Clone 性能陷阱
fn demonstrate_copy_vs_clone_pitfall() {
    use std::time::Instant;
    
    // 大型 Copy 类型
    #[derive(Copy, Clone)]
    struct LargeCopyType {
        data: [u8; 4096], // 4KB
    }
    
    // 使用智能指针的 Clone 类型
    #[derive(Clone)]
    struct SmartCloneType {
        data: Rc<[u8; 4096]>, // 共享 4KB 数据
    }
    
    let large_copy = LargeCopyType { data: [42; 4096] };
    let smart_clone = SmartCloneType { data: Rc::new([42; 4096]) };
    
    const ITERATIONS: usize = 10000;
    
    // 测试大型 Copy
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _temp = large_copy; // Copy 4KB
    }
    let copy_duration = start.elapsed();
    
    // 测试智能指针 Clone
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _temp = smart_clone.clone(); // Clone 指针
    }
    let clone_duration = start.elapsed();
    
    println!("   大型 Copy (4KB): {:?}", copy_duration);
    println!("   智能指针 Clone: {:?}", clone_duration);
    
    if clone_duration < copy_duration {
        println!("   ✅ 智能指针 Clone 比大型 Copy 更快！");
    } else {
        println!("   📝 Copy 仍然更快，但差距缩小");
    }
}

/// 演示过度使用 Clone 的陷阱
fn demonstrate_clone_overuse_pitfall() {
    println!("   ❌ 错误做法：过度使用 Clone");
    
    fn bad_example(data: &Vec<String>) -> Vec<String> {
        let cloned = data.clone(); // 不必要的 Clone
        cloned.iter().map(|s| s.to_uppercase()).collect()
    }
    
    println!("   ✅ 正确做法：使用借用");
    
    fn good_example(data: &Vec<String>) -> Vec<String> {
        data.iter().map(|s| s.to_uppercase()).collect() // 直接使用借用
    }
    
    let data = vec![String::from("hello"), String::from("world")];
    
    let result1 = bad_example(&data);
    let result2 = good_example(&data);
    
    println!("   结果相同: {:?} == {:?}", result1, result2);
    println!("   但正确做法避免了不必要的内存分配");
}

/// 演示 Move 的误解
fn demonstrate_move_misconception() {
    println!("   📝 Move 只转移所有权，不复制数据");
    
    let large_vec = vec![42u8; 1_000_000]; // 1MB 数据
    println!("   创建 1MB 向量: 地址 {:p}", large_vec.as_ptr());
    
    let moved_vec = large_vec; // Move 发生
    println!("   Move 后地址: {:p}", moved_vec.as_ptr());
    
    println!("   ✅ 地址相同！数据没有被复制，只是所有权转移了");
    println!("   Move 是零成本操作，无论数据多大");
}

/// 演示 Copy 和 Drop 的冲突
fn demonstrate_copy_drop_conflict() {
    println!("   ❌ Copy 类型不能实现 Drop");
    
    // 这段代码无法编译：
    /*
    #[derive(Copy, Clone)]
    struct BadCopyType {
        data: i32,
    }
    
    impl Drop for BadCopyType {
        fn drop(&mut self) {
            println!("dropping");
        }
    }
    */
    
    println!("   原因：Copy 意味着可以随意复制，但 Drop 需要确定性的析构");
    println!("   解决方案：移除 Copy，只保留 Clone");
    
    #[derive(Clone)] // 只实现 Clone
    struct GoodType {
        data: i32,
    }
    
    impl Drop for GoodType {
        fn drop(&mut self) {
            println!("   正确：只有 Clone 类型可以实现 Drop");
        }
    }
    
    let good = GoodType { data: 42 };
    let _cloned = good.clone();
    println!("   ✅ Clone + Drop 组合是合法的");
}

/// 总结对比
pub fn summary_comparison() {
    println!("\n=== 总结对比 ===");
    
    println!("\n📊 综合对比表:");
    println!("┌─────────────┬─────────────┬─────────────┬─────────────┬─────────────┐");
    println!("│ 特征        │ Move        │ Copy        │ Clone       │ 推荐场景    │");
    println!("├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤");
    println!("│ 性能        │ O(1) 最快   │ O(n) 中等   │ O(n) 可变   │ 性能敏感    │");
    println!("│ 内存使用    │ 无额外开销  │ 栈上复制    │ 可能堆分配  │ 内存受限    │");
    println!("│ 安全性      │ 最高        │ 高          │ 中等        │ 安全优先    │");
    println!("│ 灵活性      │ 低          │ 低          │ 高          │ 复杂逻辑    │");
    println!("│ 易用性      │ 自动        │ 自动        │ 显式        │ 简单使用    │");
    println!("└─────────────┴─────────────┴─────────────┴─────────────┴─────────────┘");
    
    println!("\n🎯 选择建议:");
    println!("   1. 默认使用 Move（Rust 的默认行为）");
    println!("   2. 小型值类型考虑 Copy");
    println!("   3. 需要独立副本时使用 Clone");
    println!("   4. 性能敏感场景优先 Move > Copy > Clone");
    
    println!("\n🔑 关键要点:");
    println!("   • Move 是 Rust 所有权系统的核心，提供内存安全保证");
    println!("   • Copy 适用于简单值类型，提供便利性");
    println!("   • Clone 提供最大灵活性，但需要谨慎使用");
    println!("   • 三者可以组合使用，满足不同场景需求");
    
    println!("\n✨ 最终建议:");
    println!("   理解每种机制的特点和适用场景，");
    println!("   根据具体需求选择最合适的方案，");
    println!("   在性能和便利性之间找到平衡点。");
}

/// 运行所有对比分析
pub fn run_comparison_analysis() {
    core_concepts_comparison();
    type_system_comparison();
    performance_comparison();
    use_case_comparison();
    selection_guide();
    common_pitfalls();
    summary_comparison();
}