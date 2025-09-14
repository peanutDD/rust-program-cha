//! # 闭包的实际应用场景
//!
//! 本模块展示闭包在实际开发中的各种应用场景，包括：
//! - 迭代器和函数式编程
//! - 事件处理和回调
//! - 策略模式和配置
//! - 错误处理和资源管理
//! - 异步编程和并发

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// 演示闭包的实际应用场景
pub fn demonstrate() {
    println!("\n🔍 6. 闭包的实际应用场景");
    println!("{}", "-".repeat(40));

    iterator_applications();
    event_handling();
    strategy_pattern();
    error_handling_applications();
    resource_management();
    async_and_concurrency();
}

/// 演示迭代器和函数式编程应用
fn iterator_applications() {
    println!("\n📝 迭代器和函数式编程:");

    // 1. 数据处理管道
    let sales_data = vec![
        ("产品A", 100, 25.0),
        ("产品B", 150, 30.0),
        ("产品C", 80, 15.0),
        ("产品D", 200, 45.0),
        ("产品E", 120, 20.0),
    ];

    println!("原始销售数据: {:?}", sales_data);

    // 复杂的数据处理链
    let high_value_products: Vec<_> = sales_data
        .iter()
        .filter(|(_, quantity, _price)| *quantity > 100) // 数量大于100
        .map(|(name, quantity, price)| {
            let total = *quantity as f64 * price;
            (name, quantity, price, total)
        })
        .filter(|(_, _, _, total)| *total > 3000.0) // 总价值大于3000
        .collect();

    println!("高价值产品: {:?}", high_value_products);

    // 2. 统计分析
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let stats = numbers.iter().fold((0, 0, i32::MAX, i32::MIN), |acc, &x| {
        let (sum, count, min, max) = acc;
        (sum + x, count + 1, min.min(x), max.max(x))
    });

    let (sum, count, min, max) = stats;
    let average = sum as f64 / count as f64;

    println!("\n统计分析:");
    println!("数据: {:?}", numbers);
    println!(
        "总和: {}, 数量: {}, 最小: {}, 最大: {}, 平均: {:.2}",
        sum, count, min, max, average
    );

    // 3. 分组和聚合
    demonstrate_grouping_and_aggregation();
}

/// 演示分组和聚合
fn demonstrate_grouping_and_aggregation() {
    println!("\n📝 分组和聚合:");

    #[derive(Debug, Clone)]
    struct Employee {
        name: String,
        department: String,
        salary: u32,
        age: u32,
    }

    let employees = vec![
        Employee {
            name: "张三".to_string(),
            department: "技术部".to_string(),
            salary: 8000,
            age: 28,
        },
        Employee {
            name: "李四".to_string(),
            department: "销售部".to_string(),
            salary: 6000,
            age: 32,
        },
        Employee {
            name: "王五".to_string(),
            department: "技术部".to_string(),
            salary: 9000,
            age: 30,
        },
        Employee {
            name: "赵六".to_string(),
            department: "销售部".to_string(),
            salary: 7000,
            age: 26,
        },
        Employee {
            name: "钱七".to_string(),
            department: "技术部".to_string(),
            salary: 7500,
            age: 29,
        },
    ];

    // 按部门分组统计
    let mut dept_stats: HashMap<String, (u32, u32, u32)> = HashMap::new();

    employees.iter().for_each(|emp| {
        let entry = dept_stats
            .entry(emp.department.clone())
            .or_insert((0, 0, 0));
        entry.0 += emp.salary; // 总薪资
        entry.1 += 1; // 人数
        entry.2 += emp.age; // 总年龄
    });

    println!("部门统计:");
    for (dept, (total_salary, count, total_age)) in dept_stats {
        let avg_salary = total_salary / count;
        let avg_age = total_age / count;
        println!(
            "{}: 平均薪资={}, 平均年龄={}, 人数={}",
            dept, avg_salary, avg_age, count
        );
    }

    // 查找高薪员工
    let high_earners: Vec<_> = employees
        .iter()
        .filter(|emp| emp.salary > 7500)
        .map(|emp| format!("{} ({}部, {}元)", emp.name, emp.department, emp.salary))
        .collect();

    println!("\n高薪员工 (>7500): {:?}", high_earners);
}

/// 演示事件处理应用
fn event_handling() {
    println!("\n📝 事件处理和回调:");

    // 1. 简单事件系统
    struct EventBus {
        listeners: HashMap<String, Vec<Box<dyn Fn(&str) -> ()>>>,
    }

    impl EventBus {
        fn new() -> Self {
            EventBus {
                listeners: HashMap::new(),
            }
        }

        fn on<F>(&mut self, event: &str, callback: F)
        where
            F: Fn(&str) -> () + 'static,
        {
            self.listeners
                .entry(event.to_string())
                .or_insert_with(Vec::new)
                .push(Box::new(callback));
        }

        fn emit(&self, event: &str, data: &str) {
            if let Some(callbacks) = self.listeners.get(event) {
                for callback in callbacks {
                    callback(data);
                }
            }
        }
    }

    let mut event_bus = EventBus::new();

    // 注册事件监听器
    event_bus.on("user_login", |data| {
        println!("[日志] 用户登录: {}", data);
    });

    event_bus.on("user_login", |data| {
        println!("[统计] 记录登录事件: {}", data);
    });

    event_bus.on("user_logout", |data| {
        println!("[清理] 用户登出: {}", data);
    });

    // 触发事件
    event_bus.emit("user_login", "张三");
    event_bus.emit("user_logout", "李四");

    // 2. 状态变化监听
    demonstrate_state_monitoring();
}

/// 演示状态监听
fn demonstrate_state_monitoring() {
    println!("\n📝 状态变化监听:");

    struct Observable<T> {
        value: T,
        observers: Vec<Box<dyn Fn(&T, &T) -> ()>>,
    }

    impl<T: Clone> Observable<T> {
        fn new(initial_value: T) -> Self {
            Observable {
                value: initial_value,
                observers: Vec::new(),
            }
        }

        fn subscribe<F>(&mut self, observer: F)
        where
            F: Fn(&T, &T) -> () + 'static,
        {
            self.observers.push(Box::new(observer));
        }

        fn set(&mut self, new_value: T) {
            let old_value = self.value.clone();
            self.value = new_value;

            for observer in &self.observers {
                observer(&old_value, &self.value);
            }
        }

        fn get(&self) -> &T {
            &self.value
        }
    }

    let mut temperature = Observable::new(20.0);

    // 订阅温度变化
    temperature.subscribe(|old, new| {
        println!("温度变化: {:.1}°C -> {:.1}°C", old, new);
    });

    temperature.subscribe(|_old, new| {
        if *new > 30.0 {
            println!("⚠️ 高温警告: {:.1}°C", new);
        } else if *new < 10.0 {
            println!("❄️ 低温警告: {:.1}°C", new);
        }
    });

    // 模拟温度变化
    let temperatures = vec![25.0, 32.0, 8.0, 22.0];
    for temp in temperatures {
        temperature.set(temp);
    }
}

/// 演示策略模式应用
fn strategy_pattern() {
    println!("\n📝 策略模式和配置:");

    // 1. 计算策略
    struct Calculator {
        strategy: Box<dyn Fn(f64, f64) -> f64>,
    }

    impl Calculator {
        fn new<F>(strategy: F) -> Self
        where
            F: Fn(f64, f64) -> f64 + 'static,
        {
            Calculator {
                strategy: Box::new(strategy),
            }
        }

        fn calculate(&self, a: f64, b: f64) -> f64 {
            (self.strategy)(a, b)
        }

        fn set_strategy<F>(&mut self, strategy: F)
        where
            F: Fn(f64, f64) -> f64 + 'static,
        {
            self.strategy = Box::new(strategy);
        }
    }

    // 不同的计算策略
    let mut calc = Calculator::new(|a, b| a + b);
    println!("加法: 5 + 3 = {}", calc.calculate(5.0, 3.0));

    calc.set_strategy(|a, b| a * b);
    println!("乘法: 5 * 3 = {}", calc.calculate(5.0, 3.0));

    calc.set_strategy(|a, b| a.powf(b));
    println!("幂运算: 5^3 = {}", calc.calculate(5.0, 3.0));

    // 2. 排序策略
    demonstrate_sorting_strategies();
}

/// 演示排序策略
fn demonstrate_sorting_strategies() {
    println!("\n📝 排序策略:");

    #[derive(Debug, Clone)]
    struct Product {
        name: String,
        price: f64,
        rating: f64,
    }

    let products = vec![
        Product {
            name: "产品A".to_string(),
            price: 100.0,
            rating: 4.5,
        },
        Product {
            name: "产品B".to_string(),
            price: 80.0,
            rating: 4.8,
        },
        Product {
            name: "产品C".to_string(),
            price: 120.0,
            rating: 4.2,
        },
        Product {
            name: "产品D".to_string(),
            price: 90.0,
            rating: 4.7,
        },
    ];

    println!("原始产品列表: {:?}", products);

    // 按价格排序
    let mut price_sorted = products.clone();
    price_sorted.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
    println!("\n按价格排序: {:?}", price_sorted);

    // 按评分排序
    let mut rating_sorted = products.clone();
    rating_sorted.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap());
    println!("\n按评分排序: {:?}", rating_sorted);

    // 自定义排序策略
    fn sort_products<F>(products: &mut Vec<Product>, comparator: F)
    where
        F: Fn(&Product, &Product) -> std::cmp::Ordering,
    {
        products.sort_by(comparator);
    }

    // 按性价比排序（评分/价格）
    let mut value_sorted = products.clone();
    sort_products(&mut value_sorted, |a, b| {
        let value_a = a.rating / a.price * 100.0;
        let value_b = b.rating / b.price * 100.0;
        value_b.partial_cmp(&value_a).unwrap()
    });
    println!("\n按性价比排序: {:?}", value_sorted);
}

/// 演示错误处理应用
fn error_handling_applications() {
    println!("\n📝 错误处理和恢复:");

    // 1. 链式错误处理
    fn process_data_chain() -> Result<String, Box<dyn std::error::Error>> {
        let data = "  42  ";

        let result = Some(data)
            .map(|s| s.trim()) // 去除空白
            .filter(|s| !s.is_empty()) // 检查非空
            .and_then(|s| s.parse::<i32>().ok()) // 解析数字
            .filter(|&n| n > 0) // 检查正数
            .map(|n| n * 2) // 计算
            .map(|n| format!("结果: {}", n)) // 格式化
            .ok_or("处理失败")?;

        Ok(result)
    }

    match process_data_chain() {
        Ok(result) => println!("链式处理成功: {}", result),
        Err(e) => println!("链式处理失败: {}", e),
    }

    // 2. 重试机制
    demonstrate_retry_mechanism();

    // 3. 错误恢复策略
    demonstrate_error_recovery();
}

/// 演示重试机制
fn demonstrate_retry_mechanism() {
    println!("\n📝 重试机制:");

    fn retry_with_backoff<F, T, E>(
        mut operation: F,
        max_attempts: usize,
        base_delay_ms: u64,
    ) -> Result<T, E>
    where
        F: FnMut() -> Result<T, E>,
    {
        for attempt in 1..=max_attempts {
            match operation() {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempt == max_attempts {
                        return Err(e);
                    }
                    println!(
                        "尝试 {} 失败，等待 {}ms 后重试...",
                        attempt,
                        base_delay_ms * attempt as u64
                    );
                    std::thread::sleep(std::time::Duration::from_millis(
                        base_delay_ms * attempt as u64,
                    ));
                }
            }
        }
        unreachable!()
    }

    // 模拟不稳定的操作
    let mut attempt_count = 0;
    let unstable_operation = || {
        attempt_count += 1;
        if attempt_count < 3 {
            Err(format!("操作失败 (尝试 {})", attempt_count))
        } else {
            Ok("操作成功!")
        }
    };

    match retry_with_backoff(unstable_operation, 5, 10) {
        Ok(result) => println!("重试成功: {}", result),
        Err(e) => println!("重试失败: {}", e),
    }
}

/// 演示错误恢复策略
fn demonstrate_error_recovery() {
    println!("\n📝 错误恢复策略:");

    // 多种恢复策略
    fn process_with_fallback<F1, F2, F3, T>(primary: F1, secondary: F2, fallback: F3) -> T
    where
        F1: FnOnce() -> Result<T, String>,
        F2: FnOnce() -> Result<T, String>,
        F3: FnOnce() -> T,
    {
        primary()
            .or_else(|e1| {
                println!("主要方法失败: {}, 尝试备用方法", e1);
                secondary()
            })
            .unwrap_or_else(|e2| {
                println!("备用方法也失败: {}, 使用默认值", e2);
                fallback()
            })
    }

    let result = process_with_fallback(
        || Err("网络连接失败".to_string()),
        || Err("缓存读取失败".to_string()),
        || "默认数据".to_string(),
    );

    println!("最终结果: {}", result);

    // 成功的情况
    let result2 = process_with_fallback(
        || Ok("网络数据".to_string()),
        || Ok("缓存数据".to_string()),
        || "默认数据".to_string(),
    );

    println!("成功结果: {}", result2);
}

/// 演示资源管理应用
fn resource_management() {
    println!("\n📝 资源管理:");

    // 1. RAII 模式
    struct ResourceGuard<F>
    where
        F: FnOnce(),
    {
        cleanup: Option<F>,
    }

    impl<F> ResourceGuard<F>
    where
        F: FnOnce(),
    {
        fn new(cleanup: F) -> Self {
            ResourceGuard {
                cleanup: Some(cleanup),
            }
        }
    }

    impl<F> Drop for ResourceGuard<F>
    where
        F: FnOnce(),
    {
        fn drop(&mut self) {
            if let Some(cleanup) = self.cleanup.take() {
                cleanup();
            }
        }
    }

    // 使用资源守卫
    {
        println!("获取资源...");
        let _guard = ResourceGuard::new(|| {
            println!("清理资源...");
        });

        println!("使用资源...");
        // 资源会在作用域结束时自动清理
    }

    // 2. 连接池模拟
    demonstrate_connection_pool();
}

/// 演示连接池
fn demonstrate_connection_pool() {
    println!("\n📝 连接池管理:");

    struct Connection {
        id: usize,
    }

    impl Connection {
        fn new(id: usize) -> Self {
            println!("创建连接 {}", id);
            Connection { id }
        }

        fn execute(&self, query: &str) -> String {
            format!("连接 {} 执行: {}", self.id, query)
        }
    }

    impl Drop for Connection {
        fn drop(&mut self) {
            println!("关闭连接 {}", self.id);
        }
    }

    struct ConnectionPool {
        connections: Arc<Mutex<Vec<Connection>>>,
        next_id: Arc<Mutex<usize>>,
    }

    impl ConnectionPool {
        fn new() -> Self {
            ConnectionPool {
                connections: Arc::new(Mutex::new(Vec::new())),
                next_id: Arc::new(Mutex::new(1)),
            }
        }

        fn with_connection<F, R>(&self, operation: F) -> R
        where
            F: FnOnce(&Connection) -> R,
        {
            let connection = {
                let mut pool = self.connections.lock().unwrap();
                pool.pop().unwrap_or_else(|| {
                    let mut id = self.next_id.lock().unwrap();
                    let conn = Connection::new(*id);
                    *id += 1;
                    conn
                })
            };

            let result = operation(&connection);

            // 归还连接到池中
            self.connections.lock().unwrap().push(connection);

            result
        }
    }

    let pool = ConnectionPool::new();

    // 使用连接池
    let result1 = pool.with_connection(|conn| conn.execute("SELECT * FROM users"));
    println!("{}", result1);

    let result2 = pool.with_connection(|conn| conn.execute("INSERT INTO logs VALUES (...)"));
    println!("{}", result2);

    println!(
        "连接池中的连接数: {}",
        pool.connections.lock().unwrap().len()
    );
}

/// 演示异步和并发应用
fn async_and_concurrency() {
    println!("\n📝 异步编程和并发:");

    // 1. 并行处理
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 使用线程进行并行计算
    let chunk_size = 3;
    let handles: Vec<_> = data
        .chunks(chunk_size)
        .enumerate()
        .map(|(i, chunk)| {
            let chunk = chunk.to_vec();
            std::thread::spawn(move || {
                let sum: i32 = chunk.iter().map(|&x| x * x).sum();
                println!("线程 {} 处理 {:?}, 平方和: {}", i, chunk, sum);
                sum
            })
        })
        .collect();

    // 收集结果
    let total: i32 = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum();

    println!("总平方和: {}", total);

    // 2. 工作队列
    demonstrate_work_queue();
}

/// 演示工作队列
fn demonstrate_work_queue() {
    println!("\n📝 工作队列:");

    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    // 创建工作队列
    let (sender, receiver) = mpsc::channel::<Box<dyn FnOnce() -> String + Send>>();
    let receiver = Arc::new(Mutex::new(receiver));

    // 启动工作线程
    let workers: Vec<_> = (0..3)
        .map(|id| {
            let receiver = receiver.clone();
            thread::spawn(move || loop {
                let task = {
                    let receiver = receiver.lock().unwrap();
                    receiver.recv()
                };

                match task {
                    Ok(task) => {
                        println!("工作线程 {} 开始执行任务", id);
                        let result = task();
                        println!("工作线程 {} 完成任务: {}", id, result);
                    }
                    Err(_) => {
                        println!("工作线程 {} 退出", id);
                        break;
                    }
                }
            })
        })
        .collect();

    // 提交任务
    for i in 1..=6 {
        let task = Box::new(move || {
            thread::sleep(Duration::from_millis(100 * i));
            format!("任务 {} 完成", i)
        });

        if sender.send(task).is_err() {
            break;
        }
    }

    // 关闭发送端，让工作线程退出
    drop(sender);

    // 等待所有工作线程完成
    for worker in workers {
        worker.join().unwrap();
    }

    println!("所有任务完成");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_applications() {
        let numbers = vec![1, 2, 3, 4, 5];
        let sum: i32 = numbers.iter().map(|x| x * x).sum();
        assert_eq!(sum, 55); // 1 + 4 + 9 + 16 + 25
    }

    #[test]
    fn test_strategy_pattern() {
        struct Calculator {
            strategy: Box<dyn Fn(i32, i32) -> i32>,
        }

        impl Calculator {
            fn new<F>(strategy: F) -> Self
            where
                F: Fn(i32, i32) -> i32 + 'static,
            {
                Calculator {
                    strategy: Box::new(strategy),
                }
            }

            fn calculate(&self, a: i32, b: i32) -> i32 {
                (self.strategy)(a, b)
            }
        }

        let calc = Calculator::new(|a, b| a + b);
        assert_eq!(calc.calculate(3, 4), 7);
    }

    #[test]
    fn test_error_handling() {
        let process = |input: &str| -> Result<i32, String> {
            input.parse().map_err(|_| "解析失败".to_string())
        };

        assert_eq!(process("42"), Ok(42));
        assert!(process("abc").is_err());
    }

    #[test]
    fn test_resource_management() {
        use std::sync::{Arc, Mutex};

        let cleanup_called = Arc::new(Mutex::new(false));
        let cleanup_flag = cleanup_called.clone();

        {
            struct Guard<F: FnOnce()> {
                cleanup: Option<F>,
            }

            impl<F: FnOnce()> Guard<F> {
                fn new(cleanup: F) -> Self {
                    Guard {
                        cleanup: Some(cleanup),
                    }
                }
            }

            impl<F: FnOnce()> Drop for Guard<F> {
                fn drop(&mut self) {
                    if let Some(cleanup) = self.cleanup.take() {
                        cleanup();
                    }
                }
            }

            let _guard = Guard::new(move || {
                *cleanup_flag.lock().unwrap() = true;
            });
        }

        assert!(*cleanup_called.lock().unwrap());
    }
}
