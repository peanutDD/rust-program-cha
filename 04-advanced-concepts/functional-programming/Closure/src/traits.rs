//! # 闭包的三种 Trait
//!
//! 本模块详细介绍 Rust 闭包的三种 Trait，包括：
//! - Fn: 可以被多次调用的闭包
//! - FnMut: 可以修改捕获变量的闭包
//! - FnOnce: 只能被调用一次的闭包
//! - Trait 之间的继承关系
//! - 实际应用场景

/// 演示闭包的三种 Trait
pub fn demonstrate() {
    println!("\n🔍 3. 闭包的三种 Trait: Fn, FnMut, FnOnce");
    println!("{}", "-".repeat(40));

    fn_trait_demo();
    fn_mut_trait_demo();
    fn_once_trait_demo();
    trait_hierarchy();
    practical_examples();
}

/// 演示 Fn Trait
fn fn_trait_demo() {
    println!("\n📝 Fn Trait - 可多次调用的闭包:");

    let x = 10;
    let y = 20;

    // Fn 闭包：只进行不可变借用，可以被多次调用
    let fn_closure = || {
        println!("Fn 闭包访问: x = {}, y = {}", x, y);
        x + y
    };

    // 可以多次调用
    println!("第1次调用: {}", fn_closure());
    println!("第2次调用: {}", fn_closure());
    println!("第3次调用: {}", fn_closure());

    // 原变量仍然可以使用
    println!("原变量仍可用: x = {}, y = {}", x, y);

    // 使用 Fn 作为参数
    fn call_fn_closure<F>(f: F) -> i32
    where
        F: Fn() -> i32,
    {
        println!("调用 Fn 闭包:");
        f() + f() + f() // 可以多次调用
    }

    let result = call_fn_closure(fn_closure);
    println!("call_fn_closure 结果: {}", result);

    demonstrate_fn_with_parameters();
}

/// 演示带参数的 Fn 闭包
fn demonstrate_fn_with_parameters() {
    println!("\n📝 带参数的 Fn 闭包:");

    let multiplier = 3;

    // Fn 闭包可以接受参数
    let multiply_closure = |x: i32| {
        println!("计算 {} * {}", x, multiplier);
        x * multiplier
    };

    // 使用泛型函数处理 Fn 闭包
    fn apply_to_list<F>(list: &[i32], f: F) -> Vec<i32>
    where
        F: Fn(i32) -> i32,
    {
        list.iter().map(|&x| f(x)).collect()
    }

    let numbers = vec![1, 2, 3, 4, 5];
    let results = apply_to_list(&numbers, multiply_closure);
    println!("应用到列表: {:?} -> {:?}", numbers, results);

    // 闭包仍然可以使用
    println!("单独调用: multiply_closure(10) = {}", multiply_closure(10));
}

/// 演示 FnMut Trait
fn fn_mut_trait_demo() {
    println!("\n📝 FnMut Trait - 可修改捕获变量的闭包:");

    let mut counter = 0;

    // FnMut 闭包：进行可变借用，可以修改捕获的变量
    let mut fn_mut_closure = || {
        counter += 1;
        println!("FnMut 闭包调用第 {} 次", counter);
        counter
    };

    // 可以多次调用，但需要 mut
    println!("第1次调用: {}", fn_mut_closure());
    println!("第2次调用: {}", fn_mut_closure());
    println!("第3次调用: {}", fn_mut_closure());

    // 使用 FnMut 作为参数
    fn call_fn_mut_closure<F>(mut f: F) -> Vec<i32>
    where
        F: FnMut() -> i32,
    {
        println!("调用 FnMut 闭包多次:");
        vec![f(), f(), f()]
    }

    // 注意：传递给函数后，原闭包不能再使用（因为可变借用）
    let results = call_fn_mut_closure(fn_mut_closure);
    println!("call_fn_mut_closure 结果: {:?}", results);

    demonstrate_fn_mut_with_state();
}

/// 演示 FnMut 的状态管理
fn demonstrate_fn_mut_with_state() {
    println!("\n📝 FnMut 的状态管理:");

    let mut sum = 0;
    let mut count = 0;

    // 复杂的 FnMut 闭包，管理多个状态
    let mut accumulator = |value: i32| {
        sum += value;
        count += 1;
        let average = sum as f64 / count as f64;
        println!(
            "添加 {}: 总和={}, 计数={}, 平均值={:.2}",
            value, sum, count, average
        );
        (sum, count, average)
    };

    // 使用累加器
    let values = vec![10, 20, 30, 40, 50];
    for value in values {
        let (s, c, avg) = accumulator(value);
        println!("当前状态: sum={}, count={}, avg={:.2}", s, c, avg);
    }

    // 使用 FnMut 处理迭代器
    fn process_with_fn_mut<F>(data: Vec<i32>, mut processor: F) -> Vec<String>
    where
        F: FnMut(i32) -> String,
    {
        data.into_iter().map(|x| processor(x)).collect()
    }

    let mut index = 0;
    let formatter = |x: i32| {
        index += 1;
        format!("第{}个元素: {}", index, x * x)
    };

    let data = vec![1, 2, 3, 4];
    let formatted = process_with_fn_mut(data, formatter);
    println!("格式化结果: {:?}", formatted);
}

/// 演示 FnOnce Trait
fn fn_once_trait_demo() {
    println!("\n📝 FnOnce Trait - 只能调用一次的闭包:");

    let data = vec![1, 2, 3, 4, 5];
    let name = String::from("my_data");

    // FnOnce 闭包：获取所有权，只能调用一次
    let fn_once_closure = move || {
        println!("FnOnce 闭包处理 {}: {:?}", name, data);
        let sum: i32 = data.iter().sum();
        let result = format!("{} 的总和是 {}", name, sum);
        println!("{}", result);
        (data, result) // 返回拥有的数据
    };

    // 只能调用一次
    let (returned_data, message) = fn_once_closure();
    println!("返回的数据: {:?}", returned_data);
    println!("返回的消息: {}", message);

    // 再次调用会编译错误
    // let result2 = fn_once_closure();  // 编译错误！

    // 使用 FnOnce 作为参数
    fn call_fn_once_closure<F, T>(f: F) -> T
    where
        F: FnOnce() -> T,
    {
        println!("调用 FnOnce 闭包:");
        f() // 只能调用一次
    }

    let expensive_data = vec!["hello".to_string(), "world".to_string()];
    let once_closure = move || {
        println!("处理昂贵的数据: {:?}", expensive_data);
        expensive_data.join(" ")
    };

    let result = call_fn_once_closure(once_closure);
    println!("FnOnce 结果: {}", result);

    demonstrate_fn_once_scenarios();
}

/// 演示 FnOnce 的典型场景
fn demonstrate_fn_once_scenarios() {
    println!("\n📝 FnOnce 的典型使用场景:");

    // 场景1: 资源清理
    {
        let resource = String::from("重要资源");
        let cleanup = move || {
            println!("清理资源: {}", resource);
            // 模拟资源清理操作
            drop(resource);
            println!("资源已清理");
        };

        // 在适当的时候清理资源
        cleanup();
    }

    // 场景2: 一次性配置
    {
        let config = vec![
            ("host".to_string(), "localhost".to_string()),
            ("port".to_string(), "8080".to_string()),
        ];

        let configure = move || {
            println!("应用配置:");
            for (key, value) in config {
                println!("  {} = {}", key, value);
            }
            "配置完成"
        };

        let status = configure();
        println!("状态: {}", status);
    }

    // 场景3: 线程间数据传递
    demonstrate_fn_once_in_threads();
}

/// 演示 FnOnce 在线程中的使用
fn demonstrate_fn_once_in_threads() {
    println!("\n📝 FnOnce 在线程中的使用:");

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let task_name = "计算任务".to_string();

    // 创建一个 FnOnce 闭包用于线程
    let computation = move || {
        println!("线程开始执行: {}", task_name);
        let sum: i32 = data.iter().sum();
        let product: i32 = data.iter().product();
        println!("数据: {:?}", data);
        println!("总和: {}, 乘积: {}", sum, product);
        (sum, product)
    };

    // 在新线程中执行
    let handle = std::thread::spawn(computation);

    // 等待结果
    match handle.join() {
        Ok((sum, product)) => {
            println!("线程计算完成: 总和={}, 乘积={}", sum, product);
        }
        Err(_) => println!("线程执行失败"),
    }
}

/// 演示 Trait 的继承关系
fn trait_hierarchy() {
    println!("\n📝 Trait 继承关系: FnOnce ← FnMut ← Fn");

    // Fn 实现了 FnMut 和 FnOnce
    let fn_closure = || 42;

    // FnMut 实现了 FnOnce
    let mut counter = 0;
    let fn_mut_closure = || {
        counter += 1;
        counter
    };

    // FnOnce 只实现了自己
    let data = String::from("test");
    let fn_once_closure = move || data.len();

    // 演示 Trait 的兼容性
    demonstrate_trait_compatibility(fn_closure, fn_mut_closure, fn_once_closure);
}

/// 演示 Trait 兼容性
fn demonstrate_trait_compatibility<F1, F2, F3>(
    fn_closure: F1,
    fn_mut_closure: F2,
    fn_once_closure: F3,
) where
    F1: Fn() -> i32,
    F2: FnMut() -> i32,
    F3: FnOnce() -> usize,
{
    println!("\n📝 Trait 兼容性演示:");

    // Fn 可以用在需要 FnMut 的地方
    fn use_fn_mut_trait<F: FnMut() -> i32>(mut f: F) -> i32 {
        f() + f()
    }

    // Fn 可以用在需要 FnOnce 的地方
    fn use_fn_once_trait<F: FnOnce() -> i32>(f: F) -> i32 {
        f()
    }

    println!("Fn 闭包作为 FnMut 使用: {}", use_fn_mut_trait(&fn_closure));
    println!("Fn 闭包作为 FnOnce 使用: {}", use_fn_once_trait(fn_closure));

    // FnMut 可以用在需要 FnOnce 的地方
    println!(
        "FnMut 闭包作为 FnOnce 使用: {}",
        use_fn_once_trait(fn_mut_closure)
    );

    // FnOnce 只能用在需要 FnOnce 的地方
    println!("FnOnce 闭包: {}", fn_once_closure());

    demonstrate_trait_bounds();
}

/// 演示不同的 Trait 约束
fn demonstrate_trait_bounds() {
    println!("\n📝 不同的 Trait 约束:");

    // 需要 Fn 的函数：可以多次调用
    fn repeat_call<F>(f: F, times: usize) -> Vec<i32>
    where
        F: Fn() -> i32,
    {
        (0..times).map(|_| f()).collect()
    }

    // 需要 FnMut 的函数：可以修改状态
    fn accumulate<F>(mut f: F, times: usize) -> Vec<i32>
    where
        F: FnMut() -> i32,
    {
        (0..times).map(|_| f()).collect()
    }

    // 需要 FnOnce 的函数：只调用一次
    fn call_once<F, T>(f: F) -> T
    where
        F: FnOnce() -> T,
    {
        f()
    }

    // 测试不同的闭包
    let constant = || 42;
    println!("重复调用常量闭包: {:?}", repeat_call(constant, 3));

    let mut counter = 0;
    let incrementer = || {
        counter += 1;
        counter
    };
    println!("累加器闭包: {:?}", accumulate(incrementer, 5));

    let expensive = move || {
        println!("执行昂贵操作...");
        std::thread::sleep(std::time::Duration::from_millis(1));
        "完成".to_string()
    };
    println!("一次性操作: {}", call_once(expensive));
}

/// 实际应用示例
fn practical_examples() {
    println!("\n📝 实际应用示例:");

    // 示例1: 事件处理器 (Fn)
    event_handler_example();

    // 示例2: 状态机 (FnMut)
    state_machine_example();

    // 示例3: 资源管理 (FnOnce)
    resource_management_example();
}

/// 事件处理器示例 (Fn)
fn event_handler_example() {
    println!("\n📝 事件处理器 (Fn):");

    struct EventSystem {
        handlers: Vec<Box<dyn Fn(&str) -> ()>>,
    }

    impl EventSystem {
        fn new() -> Self {
            EventSystem {
                handlers: Vec::new(),
            }
        }

        fn add_handler<F>(&mut self, handler: F)
        where
            F: Fn(&str) -> () + 'static,
        {
            self.handlers.push(Box::new(handler));
        }

        fn trigger_event(&self, event: &str) {
            println!("触发事件: {}", event);
            for handler in &self.handlers {
                handler(event);
            }
        }
    }

    let mut event_system = EventSystem::new();

    // 添加多个事件处理器
    let logger = |event: &str| println!("[LOG] 事件: {}", event);
    let counter_handler = {
        let prefix = "计数器";
        move |event: &str| println!("[{}] 处理: {}", prefix, event)
    };

    event_system.add_handler(logger);
    event_system.add_handler(counter_handler);

    // 触发事件（处理器可以被多次调用）
    event_system.trigger_event("用户登录");
    event_system.trigger_event("数据更新");
}

/// 状态机示例 (FnMut)
fn state_machine_example() {
    println!("\n📝 状态机 (FnMut):");

    #[derive(Debug, Clone)]
    enum State {
        Idle,
        Processing,
        Complete,
        Error,
    }

    let mut current_state = State::Idle;
    let mut step_count = 0;

    let mut state_machine = |input: &str| -> State {
        step_count += 1;
        println!(
            "步骤 {}: 当前状态 {:?}, 输入: {}",
            step_count, current_state, input
        );

        current_state = match (&current_state, input) {
            (State::Idle, "start") => State::Processing,
            (State::Processing, "finish") => State::Complete,
            (State::Processing, "error") => State::Error,
            (State::Complete, "reset") => State::Idle,
            (State::Error, "reset") => State::Idle,
            _ => current_state.clone(),
        };

        println!("新状态: {:?}", current_state);
        current_state.clone()
    };

    // 状态转换序列
    let inputs = vec!["start", "finish", "reset", "start", "error", "reset"];
    for input in inputs {
        state_machine(input);
        println!();
    }
}

/// 资源管理示例 (FnOnce)
fn resource_management_example() {
    println!("\n📝 资源管理 (FnOnce):");

    struct Resource {
        name: String,
        data: Vec<u8>,
    }

    impl Resource {
        fn new(name: &str, size: usize) -> Self {
            Resource {
                name: name.to_string(),
                data: vec![0; size],
            }
        }

        fn process(self) -> String {
            println!("处理资源: {} ({} bytes)", self.name, self.data.len());
            format!("已处理 {} ({} bytes)", self.name, self.data.len())
        }
    }

    // 创建资源处理闭包
    let resource = Resource::new("重要数据", 1024);
    let process_resource = move || {
        println!("开始资源处理流程...");
        let result = resource.process(); // 消费资源
        println!("资源处理完成");
        result
    };

    // 只能调用一次
    let result = process_resource();
    println!("处理结果: {}", result);

    // 再次调用会编译错误
    // let result2 = process_resource();  // 编译错误！
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_trait() {
        let x = 5;
        let fn_closure = || x * 2;

        // Fn 可以多次调用
        assert_eq!(fn_closure(), 10);
        assert_eq!(fn_closure(), 10);
        assert_eq!(fn_closure(), 10);
    }

    #[test]
    fn test_fn_mut_trait() {
        let mut counter = 0;
        let mut fn_mut_closure = || {
            counter += 1;
            counter
        };

        // FnMut 可以多次调用并修改状态
        assert_eq!(fn_mut_closure(), 1);
        assert_eq!(fn_mut_closure(), 2);
        assert_eq!(fn_mut_closure(), 3);
    }

    #[test]
    fn test_fn_once_trait() {
        let data = vec![1, 2, 3];
        let fn_once_closure = move || data.len();

        // FnOnce 只能调用一次
        assert_eq!(fn_once_closure(), 3);
        // 再次调用会编译错误
    }

    #[test]
    fn test_trait_hierarchy() {
        fn accept_fn_once<F: FnOnce() -> i32>(f: F) -> i32 {
            f()
        }

        // Fn 可以用作 FnOnce
        let fn_closure = || 42;
        assert_eq!(accept_fn_once(fn_closure), 42);

        // FnMut 可以用作 FnOnce
        let mut counter = 0;
        let fn_mut_closure = || {
            counter += 1;
            counter
        };
        assert_eq!(accept_fn_once(fn_mut_closure), 1);
    }
}
