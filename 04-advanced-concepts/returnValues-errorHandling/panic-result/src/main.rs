//! Rust panic! 深度分析文档
//!
//! 本文档全面分析 Rust 中的 panic! 机制，包括基础概念、工作原理、
//! 使用场景、错误处理策略和实际应用案例。
//!
//! 基于 https://course.rs/basic/result-error/panic.html 的深度扩展分析

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::panic;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    println!("=== Rust panic! 深度分析文档 ===");
    println!();

    // 1. panic! 基础概念
    demonstrate_panic_basics();

    // 2. panic 类型分析
    demonstrate_panic_types();

    // 3. panic 展开机制
    demonstrate_panic_unwinding();

    // 4. panic 处理策略
    demonstrate_panic_handling();

    // 5. panic 调试技巧
    demonstrate_panic_debugging();

    // 6. panic 最佳实践
    demonstrate_panic_best_practices();

    // 7. 高级 panic 特性
    demonstrate_panic_advanced();

    // 8. 实际应用案例
    demonstrate_panic_real_world();

    println!("\n=== panic! 分析完成 ===");
}

/// 1. panic! 基础概念演示
fn demonstrate_panic_basics() {
    println!("\n=== 1. panic! 基础概念 ===");

    // 1.1 什么是 panic
    println!("\n1.1 panic 的定义和作用:");
    println!("panic 是 Rust 中处理不可恢复错误的机制");
    println!("当程序遇到无法继续执行的严重错误时，会触发 panic");
    println!("panic 会导致程序终止，但会进行栈展开和资源清理");

    // 1.2 panic 与错误处理的关系
    println!("\n1.2 panic 与其他错误处理机制的关系:");
    println!("- panic: 不可恢复的错误，程序终止");
    println!("- Result<T, E>: 可恢复的错误，返回错误值");
    println!("- Option<T>: 值可能不存在的情况");

    // 1.3 panic 的触发条件
    println!("\n1.3 panic 的常见触发条件:");
    let conditions = vec![
        "数组越界访问",
        "整数溢出（debug模式）",
        "除零操作",
        "unwrap() 调用失败",
        "expect() 调用失败",
        "显式调用 panic!()",
        "断言失败 assert!()",
    ];

    for (i, condition) in conditions.iter().enumerate() {
        println!("{}. {}", i + 1, condition);
    }

    // 1.4 panic 机制的工作流程
    println!("\n1.4 panic 机制的工作流程:");
    println!("1. 触发 panic 条件");
    println!("2. 调用 panic handler");
    println!("3. 开始栈展开（unwind）");
    println!("4. 执行 Drop trait 清理资源");
    println!("5. 程序终止或被捕获");
}

/// 2. panic 类型分析演示
fn demonstrate_panic_types() {
    println!("\n=== 2. panic 类型分析 ===");

    // 2.1 主动 panic
    println!("\n2.1 主动 panic 示例:");

    // 显式 panic
    println!("显式 panic! 宏调用:");
    let result = panic::catch_unwind(|| {
        panic!("这是一个显式的 panic 示例");
    });
    match result {
        Ok(_) => println!("正常执行"),
        Err(_) => println!("捕获到 panic: 显式 panic 调用"),
    }

    // unwrap panic
    println!("\nunwrap() 导致的 panic:");
    let result = panic::catch_unwind(|| {
        let option: Option<i32> = None;
        option.unwrap(); // 这会 panic
    });
    match result {
        Ok(_) => println!("正常执行"),
        Err(_) => println!("捕获到 panic: unwrap 调用失败"),
    }

    // expect panic
    println!("\nexpect() 导致的 panic:");
    let result = panic::catch_unwind(|| {
        let result: Result<i32, &str> = Err("错误信息");
        result.expect("期望得到正确值，但失败了");
    });
    match result {
        Ok(_) => println!("正常执行"),
        Err(_) => println!("捕获到 panic: expect 调用失败"),
    }

    // 2.2 被动 panic
    println!("\n2.2 被动 panic 示例:");

    // 数组越界
    println!("数组越界访问:");
    let result = panic::catch_unwind(|| {
        let arr = [1, 2, 3];
        let index = std::env::args().len() + 10; // 运行时确定的索引
        let _value = arr[index]; // 越界访问
    });
    match result {
        Ok(_) => println!("正常执行"),
        Err(_) => println!("捕获到 panic: 数组越界访问"),
    }

    // 整数溢出（在 debug 模式下）
    println!("\n整数溢出检查（debug模式）:");
    let result = panic::catch_unwind(|| {
        let max_val = i32::MAX;
        // 在 debug 模式下会 panic，release 模式下会环绕
        let _overflow = max_val.checked_add(1).unwrap();
    });
    match result {
        Ok(val) => println!("正常执行，结果: {:?}", val),
        Err(_) => println!("捕获到 panic: 整数溢出"),
    }

    // 2.3 断言 panic
    println!("\n2.3 断言导致的 panic:");

    // assert! 宏
    let result = panic::catch_unwind(|| {
        let x = 5;
        assert!(x > 10, "x 应该大于 10，但实际值是 {}", x);
    });
    match result {
        Ok(_) => println!("断言通过"),
        Err(_) => println!("捕获到 panic: 断言失败"),
    }

    // debug_assert! 宏（仅在 debug 模式下生效）
    let result = panic::catch_unwind(|| {
        let y = 3;
        debug_assert!(y > 5, "debug 断言: y 应该大于 5");
    });
    match result {
        Ok(_) => println!("debug 断言通过或被忽略（release模式）"),
        Err(_) => println!("捕获到 panic: debug 断言失败"),
    }
}

/// 3. panic 展开机制演示
fn demonstrate_panic_unwinding() {
    println!("\n=== 3. panic 展开机制 ===");

    // 3.1 栈展开过程
    println!("\n3.1 栈展开过程演示:");

    struct DropDemo {
        name: String,
    }

    impl Drop for DropDemo {
        fn drop(&mut self) {
            println!("正在清理资源: {}", self.name);
        }
    }

    fn level_3() {
        let _resource3 = DropDemo {
            name: "Level 3 Resource".to_string(),
        };
        println!("进入 level_3 函数");
        panic!("在 level_3 中发生 panic");
    }

    fn level_2() {
        let _resource2 = DropDemo {
            name: "Level 2 Resource".to_string(),
        };
        println!("进入 level_2 函数");
        level_3();
    }

    fn level_1() {
        let _resource1 = DropDemo {
            name: "Level 1 Resource".to_string(),
        };
        println!("进入 level_1 函数");
        level_2();
    }

    let result = panic::catch_unwind(|| {
        level_1();
    });

    match result {
        Ok(_) => println!("正常执行完成"),
        Err(_) => println!("捕获到 panic，栈展开完成"),
    }

    // 3.2 Drop trait 执行顺序
    println!("\n3.2 Drop trait 执行顺序:");
    println!("从上面的输出可以看到，Drop 按照 LIFO（后进先出）顺序执行");
    println!("这确保了资源的正确清理");

    // 3.3 panic 安全性
    println!("\n3.3 panic 安全性分析:");

    struct PanicSafeContainer {
        data: Vec<i32>,
    }

    impl PanicSafeContainer {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        // panic 安全的操作
        fn safe_push(&mut self, value: i32) {
            // 使用 RAII 确保 panic 安全
            self.data.push(value);
        }

        // 可能不是 panic 安全的操作
        fn unsafe_operation(&mut self) -> Result<(), &'static str> {
            // 模拟可能 panic 的操作
            if self.data.len() > 5 {
                return Err("容器太大");
            }

            // 这个操作可能在中途 panic
            for i in 0..3 {
                self.data.push(i);
                if i == 1 {
                    // 模拟在操作中途 panic
                    // panic!("操作中途失败");
                }
            }
            Ok(())
        }
    }

    let mut container = PanicSafeContainer::new();
    container.safe_push(1);
    container.safe_push(2);

    match container.unsafe_operation() {
        Ok(_) => println!("不安全操作成功完成"),
        Err(e) => println!("不安全操作失败: {}", e),
    }

    println!("容器最终状态: {:?}", container.data);
}

/// 4. panic 处理策略演示
fn demonstrate_panic_handling() {
    println!("\n=== 4. panic 处理策略 ===");

    // 4.1 catch_unwind 使用
    println!("\n4.1 使用 catch_unwind 捕获 panic:");

    let result = panic::catch_unwind(|| {
        println!("执行可能 panic 的代码");
        let numbers = vec![1, 2, 3];
        numbers[10] // 这会 panic
    });

    match result {
        Ok(value) => println!("操作成功，结果: {}", value),
        Err(err) => {
            println!("捕获到 panic");
            // 尝试获取 panic 信息
            if let Some(s) = err.downcast_ref::<&str>() {
                println!("panic 消息: {}", s);
            } else if let Some(s) = err.downcast_ref::<String>() {
                println!("panic 消息: {}", s);
            } else {
                println!("未知的 panic 类型");
            }
        }
    }

    // 4.2 panic hook 设置
    println!("\n4.2 设置自定义 panic hook:");

    // 保存原始的 panic hook
    let original_hook = panic::take_hook();

    // 设置自定义 panic hook
    panic::set_hook(Box::new(|panic_info| {
        println!("🚨 自定义 panic hook 被触发!");

        // 获取 panic 位置信息
        if let Some(location) = panic_info.location() {
            println!(
                "panic 发生在: {}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            );
        }

        // 获取 panic 消息
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            println!("panic 消息: {}", s);
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            println!("panic 消息: {}", s);
        }

        println!("正在进行清理工作...");
    }));

    // 测试自定义 hook
    let result = panic::catch_unwind(|| {
        panic!("测试自定义 panic hook");
    });

    match result {
        Ok(_) => println!("正常执行"),
        Err(_) => println!("panic 被捕获，自定义 hook 已执行"),
    }

    // 恢复原始 hook
    panic::set_hook(original_hook);

    // 4.3 AssertUnwindSafe 使用
    println!("\n4.3 使用 AssertUnwindSafe:");

    let mut data = vec![1, 2, 3];
    let data_ref = &mut data;

    // 使用 AssertUnwindSafe 包装不是 UnwindSafe 的类型
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        data_ref.push(4);
        // 模拟可能的 panic
        if data_ref.len() > 3 {
            // panic!("数据太多了");
        }
        data_ref.len()
    }));

    match result {
        Ok(len) => println!("操作成功，数据长度: {}", len),
        Err(_) => println!("操作中发生 panic"),
    }

    println!("最终数据: {:?}", data);
}

/// 5. panic 调试技巧演示
fn demonstrate_panic_debugging() {
    println!("\n=== 5. panic 调试技巧 ===");

    // 5.1 backtrace 获取
    println!("\n5.1 获取 panic backtrace:");
    println!("设置环境变量 RUST_BACKTRACE=1 可以获取详细的调用栈信息");
    println!("设置环境变量 RUST_BACKTRACE=full 可以获取完整的调用栈信息");

    // 5.2 panic 信息分析
    println!("\n5.2 panic 信息分析技巧:");

    fn analyze_panic_info() {
        let original_hook = panic::take_hook();

        panic::set_hook(Box::new(|panic_info| {
            println!("\n=== Panic 信息分析 ===");

            // 分析 panic 位置
            if let Some(location) = panic_info.location() {
                println!("📍 位置信息:");
                println!("  文件: {}", location.file());
                println!("  行号: {}", location.line());
                println!("  列号: {}", location.column());
            }

            // 分析 panic 消息
            println!("💬 消息分析:");
            if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                println!("  字符串消息: {}", s);
            } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                println!("  String 消息: {}", s);
            } else {
                println!("  其他类型的 panic payload");
            }

            // 添加调试建议
            println!("🔧 调试建议:");
            println!("  1. 检查相关变量的值");
            println!("  2. 验证函数参数的有效性");
            println!("  3. 确认资源的可用性");
            println!("  4. 查看相关的错误日志");
        }));

        // 触发一个带有详细信息的 panic
        let result = panic::catch_unwind(|| {
            let user_id = 12345;
            let operation = "数据库查询";
            panic!("用户 {} 在执行 {} 时发生错误", user_id, operation);
        });

        match result {
            Ok(_) => println!("操作成功"),
            Err(_) => println!("panic 已被分析和捕获"),
        }

        panic::set_hook(original_hook);
    }

    analyze_panic_info();

    // 5.3 条件断言调试
    println!("\n5.3 条件断言调试:");

    fn debug_with_assertions(data: &[i32]) {
        // 使用断言进行调试
        assert!(!data.is_empty(), "数据不能为空");
        assert!(
            data.len() <= 1000,
            "数据长度不能超过1000，当前长度: {}",
            data.len()
        );

        for (i, &value) in data.iter().enumerate() {
            debug_assert!(value >= 0, "索引 {} 处的值 {} 应该非负", i, value);
        }

        println!("数据验证通过，长度: {}", data.len());
    }

    let test_data = vec![1, 2, 3, 4, 5];
    debug_with_assertions(&test_data);

    // 5.4 自定义错误类型用于调试
    println!("\n5.4 自定义错误类型用于调试:");

    #[derive(Debug)]
    struct DebugError {
        message: String,
        context: HashMap<String, String>,
        timestamp: Instant,
    }

    impl DebugError {
        fn new(message: &str) -> Self {
            Self {
                message: message.to_string(),
                context: HashMap::new(),
                timestamp: Instant::now(),
            }
        }

        fn add_context(mut self, key: &str, value: &str) -> Self {
            self.context.insert(key.to_string(), value.to_string());
            self
        }
    }

    fn operation_with_debug_info(should_fail: bool) -> Result<String, DebugError> {
        if should_fail {
            Err(DebugError::new("操作失败")
                .add_context("operation", "database_query")
                .add_context("user_id", "12345")
                .add_context("retry_count", "3"))
        } else {
            Ok("操作成功".to_string())
        }
    }

    match operation_with_debug_info(true) {
        Ok(result) => println!("结果: {}", result),
        Err(error) => {
            println!("调试错误信息: {:?}", error);
            // 在实际应用中，这里可以选择是否 panic
            // panic!("严重错误: {:?}", error);
        }
    }
}

/// 6. panic 最佳实践演示
fn demonstrate_panic_best_practices() {
    println!("\n=== 6. panic 最佳实践 ===");

    // 6.1 何时使用 panic
    println!("\n6.1 何时使用 panic:");
    println!("✅ 适合使用 panic 的场景:");
    println!("  - 程序逻辑错误（编程错误）");
    println!("  - 不可恢复的系统错误");
    println!("  - 原型开发和快速失败");
    println!("  - 库的内部一致性检查");

    println!("\n❌ 不适合使用 panic 的场景:");
    println!("  - 用户输入错误");
    println!("  - 网络请求失败");
    println!("  - 文件操作失败");
    println!("  - 可预期的业务逻辑错误");

    // 6.2 panic vs Result 选择
    println!("\n6.2 panic vs Result 选择示例:");

    // 好的实践：使用 Result 处理可恢复错误
    fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("除数不能为零".to_string())
        } else {
            Ok(a / b)
        }
    }

    // 不好的实践：对可恢复错误使用 panic
    fn unsafe_divide(a: f64, b: f64) -> f64 {
        if b == 0.0 {
            panic!("除数不能为零"); // 这不是好的实践
        }
        a / b
    }

    // 演示好的实践
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("安全除法结果: {}", result),
        Err(e) => println!("除法错误: {}", e),
    }

    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("安全除法结果: {}", result),
        Err(e) => println!("除法错误: {}", e),
    }

    // 6.3 错误传播策略
    println!("\n6.3 错误传播策略:");

    fn read_config_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn parse_config(contents: &str) -> Result<HashMap<String, String>, &'static str> {
        let mut config = HashMap::new();

        for line in contents.lines() {
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() != 2 {
                return Err("配置格式错误");
            }

            config.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
        }

        Ok(config)
    }

    fn load_application_config(
        path: &str,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let contents = read_config_file(path)?;
        let config = parse_config(&contents)?;
        Ok(config)
    }

    // 模拟配置内容
    let mock_config = "app_name=MyApp\nversion=1.0\n# This is a comment\nport=8080";

    match parse_config(mock_config) {
        Ok(config) => {
            println!("配置加载成功:");
            for (key, value) in &config {
                println!("  {}: {}", key, value);
            }
        }
        Err(e) => println!("配置加载失败: {}", e),
    }

    // 6.4 防御性编程
    println!("\n6.4 防御性编程实践:");

    struct SafeVector<T> {
        data: Vec<T>,
    }

    impl<T> SafeVector<T> {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn push(&mut self, item: T) {
            self.data.push(item);
        }

        // 安全的访问方法
        fn get(&self, index: usize) -> Option<&T> {
            self.data.get(index)
        }

        // 带检查的访问方法
        fn get_checked(&self, index: usize) -> Result<&T, String> {
            if index < self.data.len() {
                Ok(&self.data[index])
            } else {
                Err(format!(
                    "索引 {} 超出范围，数组长度为 {}",
                    index,
                    self.data.len()
                ))
            }
        }

        // 仅在确定安全时使用的方法
        fn get_unchecked(&self, index: usize) -> &T {
            // 添加断言确保安全性
            assert!(index < self.data.len(), "索引超出范围");
            &self.data[index]
        }
    }

    let mut safe_vec = SafeVector::new();
    safe_vec.push(1);
    safe_vec.push(2);
    safe_vec.push(3);

    // 安全访问
    match safe_vec.get(1) {
        Some(value) => println!("安全访问结果: {}", value),
        None => println!("索引不存在"),
    }

    // 带检查的访问
    match safe_vec.get_checked(5) {
        Ok(value) => println!("检查访问结果: {}", value),
        Err(e) => println!("访问错误: {}", e),
    }

    // 确定安全的访问
    let value = safe_vec.get_unchecked(0);
    println!("无检查访问结果: {}", value);
}

/// 7. 高级 panic 特性演示
fn demonstrate_panic_advanced() {
    println!("\n=== 7. 高级 panic 特性 ===");

    // 7.1 panic 安全性分析
    println!("\n7.1 panic 安全性分析:");

    // UnwindSafe 和 RefUnwindSafe trait
    println!("UnwindSafe 和 RefUnwindSafe trait 的作用:");
    println!("- UnwindSafe: 类型在 panic 展开后仍然安全");
    println!("- RefUnwindSafe: 类型的引用在 panic 展开后仍然安全");

    use std::sync::Mutex;

    fn demonstrate_unwind_safety() {
        // Mutex 不是 UnwindSafe 的，因为 panic 可能导致死锁
        let mutex_data = Arc::new(Mutex::new(vec![1, 2, 3]));
        let mutex_clone = Arc::clone(&mutex_data);

        let result = panic::catch_unwind(move || {
            let mut data = mutex_clone.lock().unwrap();
            data.push(4);
            // 如果这里 panic，mutex 可能处于不一致状态
            // panic!("模拟 panic");
            data.len()
        });

        match result {
            Ok(len) => println!("操作成功，数据长度: {}", len),
            Err(_) => println!("操作失败，但 mutex 状态可能不一致"),
        }

        // 检查 mutex 状态
        match mutex_data.lock() {
            Ok(data) => println!("Mutex 状态正常，数据: {:?}", *data),
            Err(_) => println!("Mutex 已损坏（poisoned）"),
        }
    }

    demonstrate_unwind_safety();

    // 7.2 自定义 panic 处理
    println!("\n7.2 高级 panic 处理技术:");

    struct PanicRecoverySystem {
        recovery_count: Arc<Mutex<u32>>,
        max_recoveries: u32,
    }

    impl PanicRecoverySystem {
        fn new(max_recoveries: u32) -> Self {
            Self {
                recovery_count: Arc::new(Mutex::new(0)),
                max_recoveries,
            }
        }

        fn execute_with_recovery<F, T>(&self, operation: F) -> Result<T, String>
        where
            F: FnOnce() -> T + panic::UnwindSafe,
        {
            let result = panic::catch_unwind(operation);

            match result {
                Ok(value) => Ok(value),
                Err(_) => {
                    let mut count = self.recovery_count.lock().unwrap();
                    *count += 1;

                    if *count <= self.max_recoveries {
                        println!("Panic 恢复 #{}, 继续执行", *count);
                        Err(format!("操作失败，已恢复 {} 次", *count))
                    } else {
                        println!("超过最大恢复次数，停止执行");
                        Err("超过最大恢复次数".to_string())
                    }
                }
            }
        }
    }

    let recovery_system = PanicRecoverySystem::new(3);

    // 模拟多次失败的操作
    for i in 1..=5 {
        let result = recovery_system.execute_with_recovery(|| {
            println!("执行操作 #{}", i);
            if i <= 2 {
                panic!("操作 {} 失败", i);
            }
            format!("操作 {} 成功", i)
        });

        match result {
            Ok(msg) => {
                println!("✅ {}", msg);
                break;
            }
            Err(e) => println!("❌ {}", e),
        }
    }

    // 7.3 panic 性能影响分析
    println!("\n7.3 panic 性能影响分析:");

    fn benchmark_panic_vs_result() {
        use std::time::Instant;

        // 测试 Result 版本
        let start = Instant::now();
        let mut result_count = 0;
        for i in 0..10000 {
            match safe_operation_result(i) {
                Ok(_) => result_count += 1,
                Err(_) => {}
            }
        }
        let result_duration = start.elapsed();

        // 测试 panic 版本（使用 catch_unwind）
        let start = Instant::now();
        let mut panic_count = 0;
        for i in 0..1000 {
            // 减少次数，因为 panic 更慢
            let result = panic::catch_unwind(|| safe_operation_panic(i));
            match result {
                Ok(_) => panic_count += 1,
                Err(_) => {}
            }
        }
        let panic_duration = start.elapsed();

        println!(
            "Result 版本: {} 次操作，耗时 {:?}",
            result_count, result_duration
        );
        println!(
            "Panic 版本: {} 次操作，耗时 {:?}",
            panic_count, panic_duration
        );
        println!(
            "性能比较: panic 版本比 Result 版本慢约 {}x",
            panic_duration.as_nanos() as f64 / result_duration.as_nanos() as f64 * 10.0
        );
    }

    fn safe_operation_result(value: i32) -> Result<i32, &'static str> {
        if value % 100 == 0 {
            Err("值是100的倍数")
        } else {
            Ok(value * 2)
        }
    }

    fn safe_operation_panic(value: i32) -> i32 {
        if value % 100 == 0 {
            panic!("值是100的倍数");
        }
        value * 2
    }

    benchmark_panic_vs_result();
}

/// 8. 实际应用案例演示
fn demonstrate_panic_real_world() {
    println!("\n=== 8. 实际应用案例 ===");

    // 8.1 Web 服务器 panic 处理
    println!("\n8.1 Web 服务器 panic 处理模拟:");

    struct WebServer {
        request_count: Arc<Mutex<u64>>,
        panic_count: Arc<Mutex<u64>>,
    }

    impl WebServer {
        fn new() -> Self {
            Self {
                request_count: Arc::new(Mutex::new(0)),
                panic_count: Arc::new(Mutex::new(0)),
            }
        }

        fn handle_request(&self, request_id: u64, should_panic: bool) -> Result<String, String> {
            // 增加请求计数
            {
                let mut count = self.request_count.lock().unwrap();
                *count += 1;
            }

            // 使用 catch_unwind 隔离 panic
            let result = panic::catch_unwind(|| self.process_request(request_id, should_panic));

            match result {
                Ok(response) => Ok(response),
                Err(_) => {
                    // 记录 panic
                    {
                        let mut count = self.panic_count.lock().unwrap();
                        *count += 1;
                    }

                    println!("⚠️  请求 {} 发生 panic，已隔离", request_id);
                    Err("内部服务器错误".to_string())
                }
            }
        }

        fn process_request(&self, request_id: u64, should_panic: bool) -> String {
            println!("处理请求 {}", request_id);

            if should_panic {
                panic!("请求 {} 处理失败", request_id);
            }

            format!("请求 {} 处理成功", request_id)
        }

        fn get_stats(&self) -> (u64, u64) {
            let request_count = *self.request_count.lock().unwrap();
            let panic_count = *self.panic_count.lock().unwrap();
            (request_count, panic_count)
        }
    }

    let server = WebServer::new();

    // 模拟处理多个请求
    let requests = vec![
        (1, false),
        (2, true), // 这个会 panic
        (3, false),
        (4, true), // 这个也会 panic
        (5, false),
    ];

    for (id, should_panic) in requests {
        match server.handle_request(id, should_panic) {
            Ok(response) => println!("✅ {}", response),
            Err(error) => println!("❌ 请求 {} 失败: {}", id, error),
        }
    }

    let (total_requests, total_panics) = server.get_stats();
    println!(
        "服务器统计: 总请求 {}, panic 次数 {}",
        total_requests, total_panics
    );

    // 8.2 多线程 panic 隔离
    println!("\n8.2 多线程 panic 隔离:");

    fn worker_thread(
        worker_id: u32,
        should_panic: bool,
    ) -> thread::JoinHandle<Result<String, String>> {
        thread::spawn(move || {
            let result = panic::catch_unwind(|| {
                println!("工作线程 {} 开始工作", worker_id);

                if should_panic {
                    panic!("工作线程 {} 发生错误", worker_id);
                }

                // 模拟工作
                thread::sleep(Duration::from_millis(100));
                format!("工作线程 {} 完成工作", worker_id)
            });

            match result {
                Ok(message) => Ok(message),
                Err(_) => Err(format!("工作线程 {} 发生 panic", worker_id)),
            }
        })
    }

    let mut handles = Vec::new();

    // 创建多个工作线程
    for i in 1..=5 {
        let should_panic = i % 2 == 0; // 偶数线程会 panic
        handles.push(worker_thread(i, should_panic));
    }

    // 等待所有线程完成
    for handle in handles {
        match handle.join() {
            Ok(Ok(message)) => println!("✅ {}", message),
            Ok(Err(error)) => println!("❌ {}", error),
            Err(_) => println!("❌ 线程 join 失败"),
        }
    }

    // 8.3 系统级错误恢复
    println!("\n8.3 系统级错误恢复模拟:");

    struct SystemRecovery {
        recovery_strategies: Vec<Box<dyn Fn() -> Result<(), String> + Send + Sync>>,
    }

    impl SystemRecovery {
        fn new() -> Self {
            Self {
                recovery_strategies: Vec::new(),
            }
        }

        fn add_strategy<F>(&mut self, strategy: F)
        where
            F: Fn() -> Result<(), String> + Send + Sync + 'static,
        {
            self.recovery_strategies.push(Box::new(strategy));
        }

        fn execute_with_recovery<F, T>(&self, operation: F) -> Result<T, String>
        where
            F: FnOnce() -> T + panic::UnwindSafe,
        {
            let result = panic::catch_unwind(operation);

            match result {
                Ok(value) => Ok(value),
                Err(_) => {
                    println!("系统发生严重错误，开始恢复程序...");

                    for (i, strategy) in self.recovery_strategies.iter().enumerate() {
                        match strategy() {
                            Ok(_) => println!("恢复策略 {} 执行成功", i + 1),
                            Err(e) => println!("恢复策略 {} 执行失败: {}", i + 1, e),
                        }
                    }

                    Err("系统错误，已执行恢复程序".to_string())
                }
            }
        }
    }

    let mut recovery = SystemRecovery::new();

    // 添加恢复策略
    recovery.add_strategy(|| {
        println!("  - 清理临时文件");
        Ok(())
    });

    recovery.add_strategy(|| {
        println!("  - 重置数据库连接");
        Ok(())
    });

    recovery.add_strategy(|| {
        println!("  - 发送错误报告");
        Ok(())
    });

    recovery.add_strategy(|| {
        println!("  - 记录系统状态");
        Ok(())
    });

    // 模拟系统操作
    let result = recovery.execute_with_recovery(|| {
        println!("执行关键系统操作...");
        // 模拟系统错误
        panic!("系统内存不足");
        #[allow(unreachable_code)]
        "操作成功"
    });

    match result {
        Ok(msg) => println!("系统操作成功: {}", msg),
        Err(e) => println!("系统操作失败: {}", e),
    }

    // 8.4 性能监控和 panic 统计
    println!("\n8.4 性能监控和 panic 统计:");

    struct PanicMonitor {
        panic_stats: Arc<Mutex<HashMap<String, u32>>>,
        start_time: Instant,
    }

    impl PanicMonitor {
        fn new() -> Self {
            Self {
                panic_stats: Arc::new(Mutex::new(HashMap::new())),
                start_time: Instant::now(),
            }
        }

        fn monitor_operation<F, T>(&self, operation_name: &str, operation: F) -> Result<T, String>
        where
            F: FnOnce() -> T + panic::UnwindSafe,
        {
            let start = Instant::now();
            let result = panic::catch_unwind(operation);
            let duration = start.elapsed();

            match result {
                Ok(value) => {
                    println!("✅ {} 执行成功，耗时 {:?}", operation_name, duration);
                    Ok(value)
                }
                Err(_) => {
                    // 记录 panic 统计
                    {
                        let mut stats = self.panic_stats.lock().unwrap();
                        *stats.entry(operation_name.to_string()).or_insert(0) += 1;
                    }

                    println!("❌ {} 发生 panic，耗时 {:?}", operation_name, duration);
                    Err(format!("{} 执行失败", operation_name))
                }
            }
        }

        fn get_report(&self) -> String {
            let stats = self.panic_stats.lock().unwrap();
            let uptime = self.start_time.elapsed();

            let mut report = format!("\n=== Panic 监控报告 ===\n");
            report.push_str(&format!("运行时间: {:?}\n", uptime));
            report.push_str(&format!("总 panic 类型数: {}\n", stats.len()));

            if stats.is_empty() {
                report.push_str("没有记录到 panic 事件\n");
            } else {
                report.push_str("Panic 统计:\n");
                for (operation, count) in stats.iter() {
                    report.push_str(&format!("  {}: {} 次\n", operation, count));
                }
            }

            report
        }
    }

    let monitor = PanicMonitor::new();

    // 模拟各种操作
    let operations = vec![
        ("数据库查询", false),
        ("文件读取", true),
        ("网络请求", false),
        ("数据处理", true),
        ("缓存更新", false),
        ("文件读取", true), // 重复操作
    ];

    for (op_name, should_panic) in operations {
        let _ = monitor.monitor_operation(op_name, || {
            if should_panic {
                panic!("{} 失败", op_name);
            }
            format!("{} 成功", op_name)
        });
    }

    // 输出监控报告
    println!("{}", monitor.get_report());
}

// 辅助函数和结构体

/// 演示 panic 安全的数据结构
struct PanicSafeQueue<T> {
    items: Vec<T>,
    head: usize,
}

impl<T> PanicSafeQueue<T> {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            head: 0,
        }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        if self.head < self.items.len() {
            let item = self.items.swap_remove(self.head);
            Some(item)
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.items.len() - self.head
    }
}

/// 自定义错误类型
#[derive(Debug)]
enum CustomError {
    InvalidInput(String),
    SystemError(String),
    NetworkError(String),
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::InvalidInput(msg) => write!(f, "输入错误: {}", msg),
            CustomError::SystemError(msg) => write!(f, "系统错误: {}", msg),
            CustomError::NetworkError(msg) => write!(f, "网络错误: {}", msg),
        }
    }
}

impl std::error::Error for CustomError {}

/// panic 安全的操作包装器
struct SafeOperation;

impl SafeOperation {
    fn execute<F, T, E>(operation: F) -> Result<T, E>
    where
        F: FnOnce() -> Result<T, E> + panic::UnwindSafe,
        E: From<String>,
    {
        match panic::catch_unwind(operation) {
            Ok(result) => result,
            Err(_) => Err(E::from("操作中发生 panic".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panic_recovery() {
        let result = panic::catch_unwind(|| {
            panic!("测试 panic");
        });

        assert!(result.is_err());
    }

    #[test]
    fn test_safe_queue() {
        let mut queue = PanicSafeQueue::new();
        queue.push(1);
        queue.push(2);

        assert_eq!(queue.len(), 2);
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.len(), 1);
    }

    #[test]
    #[should_panic(expected = "测试断言")]
    fn test_assertion_panic() {
        assert!(false, "测试断言");
    }
}
