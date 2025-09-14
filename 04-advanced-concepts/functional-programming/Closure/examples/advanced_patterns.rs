//! # 高级闭包模式示例
//!
//! 这个示例展示了闭包的高级用法和设计模式，适合有一定经验的开发者。

use closure_tutorial::{advanced, performance};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("🔥 高级闭包模式示例");
    println!("{}", "=".repeat(50));

    // 1. 闭包工厂模式
    demonstrate_closure_factory();

    // 2. 函数组合
    demonstrate_function_composition();

    // 3. 策略模式
    demonstrate_strategy_pattern();

    // 4. 装饰器模式
    demonstrate_decorator_pattern();

    // 5. 状态机模式
    demonstrate_state_machine();

    // 6. 缓存模式
    demonstrate_memoization();

    // 7. 运行库中的高级演示
    println!("\n📚 库演示");
    advanced::demonstrate();
    performance::demonstrate();

    println!("\n✅ 高级模式示例完成！");
}

/// 演示闭包工厂模式
fn demonstrate_closure_factory() {
    println!("\n🏭 1. 闭包工厂模式");

    // 创建不同的数学运算工厂
    let add_factory = |n: i32| move |x: i32| x + n;
    let multiply_factory = |n: i32| move |x: i32| x * n;

    let add_5 = add_factory(5);
    let multiply_3 = multiply_factory(3);

    println!("add_5(10) = {}", add_5(10));
    println!("multiply_3(7) = {}", multiply_3(7));

    // 验证器工厂
    let range_validator = |min: i32, max: i32| move |value: i32| value >= min && value <= max;

    let age_validator = range_validator(0, 120);
    let percentage_validator = range_validator(0, 100);

    println!("年龄 25 有效: {}", age_validator(25));
    println!("年龄 150 有效: {}", age_validator(150));
    println!("百分比 85 有效: {}", percentage_validator(85));
}

/// 演示函数组合
fn demonstrate_function_composition() {
    println!("\n🔗 2. 函数组合");

    // 基础函数
    let add_one = |x: i32| x + 1;
    let double = |x: i32| x * 2;
    let square = |x: i32| x * x;

    // 函数组合器
    let compose = |f: fn(i32) -> i32, g: fn(i32) -> i32| move |x: i32| f(g(x));

    let add_one_then_double = compose(double, add_one);
    let double_then_square = compose(square, double);

    println!("(5 + 1) * 2 = {}", add_one_then_double(5));
    println!("(3 * 2)² = {}", double_then_square(3));

    // 管道操作
    let pipeline = |x: i32| [add_one, double, square].iter().fold(x, |acc, f| f(acc));

    println!("管道 ((4 + 1) * 2)² = {}", pipeline(4));
}

/// 演示策略模式
fn demonstrate_strategy_pattern() {
    println!("\n🎯 3. 策略模式");

    // 不同的排序策略
    let bubble_sort = |mut vec: Vec<i32>| {
        let len = vec.len();
        for i in 0..len {
            for j in 0..len - 1 - i {
                if vec[j] > vec[j + 1] {
                    vec.swap(j, j + 1);
                }
            }
        }
        vec
    };

    let quick_sort_strategy = |mut vec: Vec<i32>| {
        vec.sort_unstable();
        vec
    };

    // 排序上下文
    struct Sorter<F>
    where
        F: Fn(Vec<i32>) -> Vec<i32>,
    {
        strategy: F,
    }

    impl<F> Sorter<F>
    where
        F: Fn(Vec<i32>) -> Vec<i32>,
    {
        fn new(strategy: F) -> Self {
            Self { strategy }
        }

        fn sort(&self, data: Vec<i32>) -> Vec<i32> {
            (self.strategy)(data)
        }
    }

    let data = vec![64, 34, 25, 12, 22, 11, 90];

    let bubble_sorter = Sorter::new(bubble_sort);
    let quick_sorter = Sorter::new(quick_sort_strategy);

    println!("原数据: {:?}", data);
    println!("冒泡排序: {:?}", bubble_sorter.sort(data.clone()));
    println!("快速排序: {:?}", quick_sorter.sort(data));
}

/// 演示装饰器模式
fn demonstrate_decorator_pattern() {
    println!("\n🎨 4. 装饰器模式");

    use std::time::Instant;

    // 计时装饰器
    let with_timing = |name: &str, f: fn(i32) -> i32| {
        let name = name.to_string();
        move |x: i32| {
            let start = Instant::now();
            let result = f(x);
            let duration = start.elapsed();
            println!("函数 {} 执行时间: {:?}", name, duration);
            result
        }
    };

    // 日志装饰器
    let with_logging = |name: &str, f: fn(i32) -> i32| {
        let name = name.to_string();
        move |x: i32| {
            println!("[LOG] 调用函数 {} 参数: {}", name, x);
            let result = f(x);
            println!("[LOG] 函数 {} 返回: {}", name, result);
            result
        }
    };

    // 基础函数
    fn expensive_calculation(n: i32) -> i32 {
        // 模拟耗时计算
        thread::sleep(std::time::Duration::from_millis(100));
        n * n + n + 1
    }

    let timed_calc = with_timing("expensive_calculation", expensive_calculation);
    let logged_calc = with_logging("expensive_calculation", expensive_calculation);

    println!("带计时的计算: {}", timed_calc(10));
    println!("带日志的计算: {}", logged_calc(5));
}

/// 演示状态机模式
fn demonstrate_state_machine() {
    println!("\n🤖 5. 状态机模式");

    #[derive(Debug, Clone, PartialEq)]
    enum State {
        Idle,
        Running,
        Paused,
        Stopped,
    }

    #[derive(Debug)]
    enum Event {
        Start,
        Pause,
        Resume,
        Stop,
    }

    // 状态转换函数
    let state_machine = |current_state: State| {
        move |event: Event| -> State {
            match (current_state.clone(), event) {
                (State::Idle, Event::Start) => State::Running,
                (State::Running, Event::Pause) => State::Paused,
                (State::Paused, Event::Resume) => State::Running,
                (State::Running, Event::Stop) => State::Stopped,
                (State::Paused, Event::Stop) => State::Stopped,
                _ => current_state, // 无效转换，保持当前状态
            }
        }
    };

    let mut current_state = State::Idle;
    println!("初始状态: {:?}", current_state);

    let events = vec![Event::Start, Event::Pause, Event::Resume, Event::Stop];

    for event in events {
        let transition = state_machine(current_state.clone());
        current_state = transition(event);
        println!("新状态: {:?}", current_state);
    }
}

/// 演示记忆化（缓存）模式
fn demonstrate_memoization() {
    println!("\n💾 6. 记忆化模式");

    // 简单的记忆化实现
    fn memoize<F, Arg, Ret>(f: F) -> impl Fn(Arg) -> Ret
    where
        F: Fn(Arg) -> Ret,
        Arg: Clone + std::hash::Hash + Eq + std::fmt::Debug,
        Ret: Clone,
    {
        let cache = Arc::new(Mutex::new(HashMap::<Arg, Ret>::new()));
        move |arg: Arg| {
            let mut cache = cache.lock().unwrap();
            if let Some(result) = cache.get(&arg) {
                println!("缓存命中: {:?}", arg);
                result.clone()
            } else {
                println!("计算中: {:?}", arg);
                let result = f(arg.clone());
                cache.insert(arg, result.clone());
                result
            }
        }
    }

    // 斐波那契数列（递归版本，用于演示缓存效果）
    fn fibonacci(n: u32) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                // 模拟计算延迟
                thread::sleep(std::time::Duration::from_millis(10));
                fibonacci(n - 1) + fibonacci(n - 2)
            }
        }
    }

    let memoized_fib = memoize(fibonacci);

    println!("计算 fibonacci(10):");
    let result1 = memoized_fib(10);
    println!("结果: {}", result1);

    println!("\n再次计算 fibonacci(10):");
    let result2 = memoized_fib(10);
    println!("结果: {}", result2);

    println!("\n计算 fibonacci(8) (部分缓存命中):");
    let result3 = memoized_fib(8);
    println!("结果: {}", result3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_factory() {
        let add_factory = |n: i32| move |x: i32| x + n;
        let add_5 = add_factory(5);
        assert_eq!(add_5(10), 15);
    }

    #[test]
    fn test_function_composition() {
        let add_one = |x: i32| x + 1;
        let double = |x: i32| x * 2;
        let compose = |f: fn(i32) -> i32, g: fn(i32) -> i32| move |x: i32| f(g(x));
        let add_one_then_double = compose(double, add_one);
        assert_eq!(add_one_then_double(5), 12); // (5 + 1) * 2 = 12
    }

    #[test]
    fn test_validator_factory() {
        let range_validator = |min: i32, max: i32| move |value: i32| value >= min && value <= max;
        let age_validator = range_validator(0, 120);
        assert!(age_validator(25));
        assert!(!age_validator(150));
    }
}
