//! # 闭包作为参数和返回值
//!
//! 本模块详细介绍闭包在函数参数和返回值中的使用，包括：
//! - 闭包作为函数参数
//! - 闭包作为返回值
//! - 高阶函数的设计模式
//! - impl Trait 语法的使用
//! - 生命周期和所有权问题

use std::collections::HashMap;

/// 演示闭包作为参数和返回值
pub fn demonstrate() {
    println!("\n🔍 5. 闭包作为参数和返回值");
    println!("{}", "-".repeat(40));

    closures_as_parameters();
    closures_as_return_values();
    higher_order_functions();
    impl_trait_syntax();
    lifetime_and_ownership();
}

/// 演示闭包作为参数
fn closures_as_parameters() {
    println!("\n📝 闭包作为参数:");

    // 1. 基本的闭包参数
    fn apply_operation<F>(x: i32, y: i32, op: F) -> i32
    where
        F: Fn(i32, i32) -> i32,
    {
        println!("应用操作到 {} 和 {}", x, y);
        op(x, y)
    }

    // 使用不同的闭包
    let add = |a, b| a + b;
    let multiply = |a, b| a * b;
    let max = |a, b| if a > b { a } else { b };

    println!("加法: {}", apply_operation(5, 3, add));
    println!("乘法: {}", apply_operation(5, 3, multiply));
    println!("最大值: {}", apply_operation(5, 3, max));

    // 2. 直接传递闭包字面量
    println!("\n直接传递闭包字面量:");
    println!("减法: {}", apply_operation(10, 4, |a, b| a - b));
    println!("幂运算: {}", apply_operation(2, 3, |a, b| a.pow(b as u32)));

    demonstrate_different_closure_traits();
}

/// 演示不同 Trait 的闭包参数
fn demonstrate_different_closure_traits() {
    println!("\n📝 不同 Trait 的闭包参数:");

    // Fn - 可以多次调用
    fn repeat_operation<F>(times: usize, op: F) -> Vec<i32>
    where
        F: Fn() -> i32,
    {
        (0..times).map(|_| op()).collect()
    }

    let counter_start = std::time::SystemTime::now();
    let get_elapsed = move || counter_start.elapsed().unwrap().as_millis() as i32;

    let results = repeat_operation(3, get_elapsed);
    println!("重复操作结果: {:?}", results);

    // FnMut - 可以修改捕获的变量
    fn accumulate_with_closure<F>(values: Vec<i32>, mut accumulator: F) -> i32
    where
        F: FnMut(i32) -> i32,
    {
        let mut result = 0;
        for value in values {
            result = accumulator(value);
        }
        result
    }

    let mut sum = 0;
    let add_to_sum = |x| {
        sum += x;
        sum
    };

    let final_sum = accumulate_with_closure(vec![1, 2, 3, 4, 5], add_to_sum);
    println!("累加结果: {}", final_sum);

    // FnOnce - 只能调用一次
    fn consume_and_process<F, T>(data: Vec<String>, processor: F) -> T
    where
        F: FnOnce(Vec<String>) -> T,
    {
        println!("处理数据: {:?}", data);
        processor(data)
    }

    let words = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];
    let result = consume_and_process(words, |data| data.join(" ").to_uppercase());
    println!("处理结果: {}", result);
}

/// 演示闭包作为返回值
fn closures_as_return_values() {
    println!("\n📝 闭包作为返回值:");

    // 1. 返回简单闭包
    fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
        move |x| x * factor
    }

    let double = create_multiplier(2);
    let triple = create_multiplier(3);
    let ten_times = create_multiplier(10);

    println!("double(7) = {}", double(7));
    println!("triple(7) = {}", triple(7));
    println!("ten_times(7) = {}", ten_times(7));

    // 2. 返回复杂闭包
    fn create_validator(min: i32, max: i32) -> impl Fn(i32) -> Result<i32, String> {
        move |value| {
            if value < min {
                Err(format!("值 {} 小于最小值 {}", value, min))
            } else if value > max {
                Err(format!("值 {} 大于最大值 {}", value, max))
            } else {
                Ok(value)
            }
        }
    }

    let validator = create_validator(1, 100);

    let test_values = vec![-5, 50, 150];
    for value in test_values {
        match validator(value) {
            Ok(v) => println!("验证通过: {}", v),
            Err(e) => println!("验证失败: {}", e),
        }
    }

    demonstrate_closure_factories();
}

/// 演示闭包工厂
fn demonstrate_closure_factories() {
    println!("\n📝 闭包工厂:");

    // 创建不同类型的处理器
    fn create_string_processor(operation: &str) -> Box<dyn Fn(&str) -> String> {
        match operation {
            "uppercase" => Box::new(|s| s.to_uppercase()),
            "lowercase" => Box::new(|s| s.to_lowercase()),
            "reverse" => Box::new(|s| s.chars().rev().collect()),
            "length" => Box::new(|s| format!("长度: {}", s.len())),
            _ => Box::new(|s| format!("未知操作: {}", s)),
        }
    }

    let processors = vec![
        ("uppercase", create_string_processor("uppercase")),
        ("lowercase", create_string_processor("lowercase")),
        ("reverse", create_string_processor("reverse")),
        ("length", create_string_processor("length")),
    ];

    let test_string = "Hello World";
    println!("测试字符串: {}", test_string);

    for (name, processor) in processors {
        let result = processor(test_string);
        println!("{}: {}", name, result);
    }

    // 条件闭包工厂
    fn create_conditional_processor(condition: bool) -> impl Fn(i32) -> String {
        if condition {
            |x| format!("正数处理: {}", x * x)
        } else {
            |x: i32| format!("负数处理: {}", x.abs())
        }
    }

    let positive_processor = create_conditional_processor(true);
    let negative_processor = create_conditional_processor(false);

    println!("\n条件处理器:");
    println!("{}", positive_processor(5));
    println!("{}", negative_processor(-5));
}

/// 演示高阶函数
fn higher_order_functions() {
    println!("\n📝 高阶函数设计模式:");

    // 1. 函数组合
    fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
    where
        F: Fn(B) -> C,
        G: Fn(A) -> B,
    {
        move |x| f(g(x))
    }

    let add_one = |x: i32| x + 1;
    let multiply_two = |x: i32| x * 2;

    let composed = compose(multiply_two, add_one);
    println!("组合函数 (x+1)*2, x=5: {}", composed(5));

    // 2. 柯里化
    fn curry_add(a: i32) -> impl Fn(i32) -> i32 {
        move |b| a + b
    }

    let add_10 = curry_add(10);
    let add_100 = curry_add(100);

    println!("柯里化加法 add_10(5): {}", add_10(5));
    println!("柯里化加法 add_100(5): {}", add_100(5));

    // 3. 部分应用
    fn partial_apply<F, A, B, C>(f: F, a: A) -> impl Fn(B) -> C
    where
        F: Fn(A, B) -> C,
        A: Clone,
    {
        move |b| f(a.clone(), b)
    }

    let power = |base: i32, exp: i32| base.pow(exp as u32);
    let square = partial_apply(power, 2); // 2的幂
    let cube_base = partial_apply(|base, exp| power(base, exp), 3); // 立方

    println!("2的3次方: {}", square(3));
    println!("3的4次方: {}", cube_base(4));

    demonstrate_pipeline_pattern();
}

/// 演示管道模式
fn demonstrate_pipeline_pattern() {
    println!("\n📝 管道模式:");

    // 创建管道处理器
    struct Pipeline<T> {
        value: T,
    }

    impl<T> Pipeline<T> {
        fn new(value: T) -> Self {
            Pipeline { value }
        }

        fn pipe<F, U>(self, f: F) -> Pipeline<U>
        where
            F: FnOnce(T) -> U,
        {
            Pipeline::new(f(self.value))
        }

        fn finish(self) -> T {
            self.value
        }
    }

    // 使用管道处理数据
    let result = Pipeline::new("  hello world  ")
        .pipe(|s| s.trim())
        .pipe(|s| s.to_uppercase())
        .pipe(|s| s.replace(" ", "_"))
        .pipe(|s| format!("[{}]", s))
        .finish();

    println!("管道处理结果: {}", result);

    // 数值处理管道
    let number_result = Pipeline::new(vec![1, 2, 3, 4, 5])
        .pipe(|v| v.into_iter().filter(|&x| x % 2 == 0).collect::<Vec<_>>())
        .pipe(|v| v.into_iter().map(|x| x * x).collect::<Vec<_>>())
        .pipe(|v| v.into_iter().sum::<i32>())
        .finish();

    println!("数值管道结果: {}", number_result);
}

/// 演示 impl Trait 语法
fn impl_trait_syntax() {
    println!("\n📝 impl Trait 语法:");

    // 1. 简化返回类型
    fn create_formatter(prefix: String) -> impl Fn(&str) -> String {
        move |text| format!("{}: {}", prefix, text)
    }

    let info_formatter = create_formatter("INFO".to_string());
    let error_formatter = create_formatter("ERROR".to_string());

    println!("{}", info_formatter("系统启动"));
    println!("{}", error_formatter("连接失败"));

    // 2. 参数中的 impl Trait
    fn process_with_impl_trait(data: Vec<i32>, processor: impl Fn(i32) -> String) -> Vec<String> {
        data.into_iter().map(processor).collect()
    }

    let numbers = vec![1, 2, 3, 4, 5];
    let formatted = process_with_impl_trait(numbers, |x| format!("数字: {}", x * x));
    println!("impl Trait 处理结果: {:?}", formatted);

    // 3. 复杂的 impl Trait 返回类型
    fn create_complex_processor() -> impl Fn(Vec<String>) -> HashMap<String, usize> {
        |strings| {
            let mut map = HashMap::new();
            for s in strings {
                let len = s.len();
                map.insert(s, len);
            }
            map
        }
    }

    let processor = create_complex_processor();
    let words = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];
    let word_map = processor(words);
    println!("复杂处理器结果: {:?}", word_map);

    demonstrate_trait_bounds_with_impl();
}

/// 演示 impl Trait 的约束
fn demonstrate_trait_bounds_with_impl() {
    println!("\n📝 impl Trait 的约束:");

    // 多个 trait 约束
    fn create_debug_processor<T>() -> impl Fn(T) -> String
    where
        T: std::fmt::Debug + Clone,
    {
        |value| format!("调试信息: {:?}", value)
    }

    let debug_int = create_debug_processor::<i32>();
    let debug_string = create_debug_processor::<String>();

    println!("{}", debug_int(42));
    println!("{}", debug_string("test".to_string()));

    // 生命周期约束
    fn create_ref_processor<'a>() -> impl Fn(&'a str) -> String + 'a {
        |s| format!("处理引用: {}", s.len())
    }

    let ref_processor = create_ref_processor();
    let text = "hello world";
    println!("{}", ref_processor(text));

    // Send + Sync 约束（用于多线程）
    fn create_thread_safe_processor() -> impl Fn(i32) -> i32 + Send + Sync {
        |x| x * 2
    }

    let thread_processor = create_thread_safe_processor();
    let handle = std::thread::spawn(move || thread_processor(21));

    match handle.join() {
        Ok(result) => println!("线程安全处理器结果: {}", result),
        Err(_) => println!("线程执行失败"),
    }
}

/// 演示生命周期和所有权问题
fn lifetime_and_ownership() {
    println!("\n📝 生命周期和所有权问题:");

    // 1. 生命周期问题
    demonstrate_lifetime_issues();

    // 2. 所有权转移
    demonstrate_ownership_transfer();

    // 3. 借用检查器的挑战
    demonstrate_borrow_checker_challenges();
}

/// 演示生命周期问题
fn demonstrate_lifetime_issues() {
    println!("\n📝 生命周期问题:");

    // 正确的生命周期处理
    fn create_string_ref_processor<'a>() -> Box<dyn Fn(&'a str) -> String + 'a> {
        Box::new(|s| format!("长度: {}, 内容: {}", s.len(), s))
    }

    let processor = create_string_ref_processor();
    let text = "hello world";
    let result = processor(text);
    println!("{}", result);

    // 避免悬垂引用
    fn safe_closure_creator(prefix: String) -> impl Fn(String) -> String {
        move |suffix| format!("{}-{}", prefix, suffix)
    }

    let combiner = safe_closure_creator("PREFIX".to_string());
    let combined = combiner("SUFFIX".to_string());
    println!("安全组合: {}", combined);
}

/// 演示所有权转移
fn demonstrate_ownership_transfer() {
    println!("\n📝 所有权转移:");

    // 闭包获取所有权
    fn create_consumer() -> impl FnOnce(Vec<String>) -> String {
        |mut data| {
            data.sort();
            data.join(", ")
        }
    }

    let consumer = create_consumer();
    let words = vec!["world".to_string(), "hello".to_string(), "rust".to_string()];
    let result = consumer(words);
    println!("消费结果: {}", result);

    // 克隆避免所有权问题
    fn create_cloning_processor<T: Clone>() -> impl Fn(T) -> (T, T) {
        |value| {
            let cloned = value.clone();
            (value, cloned)
        }
    }

    let cloner = create_cloning_processor::<String>();
    let original = "test".to_string();
    let (orig, copy) = cloner(original);
    println!("原始: {}, 克隆: {}", orig, copy);
}

/// 演示借用检查器的挑战
fn demonstrate_borrow_checker_challenges() {
    println!("\n📝 借用检查器的挑战:");

    // 使用 Rc 和 RefCell 解决复杂借用问题
    use std::cell::RefCell;
    use std::rc::Rc;

    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));

    // 创建可以修改共享数据的闭包
    let data_clone = shared_data.clone();
    let modifier = move |value: i32| {
        data_clone.borrow_mut().push(value);
        data_clone.borrow().len()
    };

    println!("添加元素前长度: {}", shared_data.borrow().len());
    let new_len = modifier(4);
    println!("添加元素后长度: {}", new_len);
    println!("最终数据: {:?}", shared_data.borrow());

    // 使用 Arc 和 Mutex 处理多线程场景
    use std::sync::{Arc, Mutex};

    let thread_safe_data = Arc::new(Mutex::new(0));
    let data_clone = thread_safe_data.clone();

    let incrementer = move || {
        let mut data = data_clone.lock().unwrap();
        *data += 1;
        *data
    };

    let handles: Vec<_> = (0..3)
        .map(|_| {
            let inc = incrementer.clone();
            std::thread::spawn(move || inc())
        })
        .collect();

    // 等待所有线程完成
    for handle in handles {
        if let Ok(result) = handle.join() {
            println!("线程结果: {}", result);
        }
    }

    println!("最终计数: {}", *thread_safe_data.lock().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_as_parameter() {
        fn apply<F>(x: i32, f: F) -> i32
        where
            F: Fn(i32) -> i32,
        {
            f(x)
        }

        let double = |x| x * 2;
        assert_eq!(apply(5, double), 10);
        assert_eq!(apply(3, |x| x + 1), 4);
    }

    #[test]
    fn test_closure_as_return_value() {
        fn create_adder(n: i32) -> impl Fn(i32) -> i32 {
            move |x| x + n
        }

        let add_5 = create_adder(5);
        assert_eq!(add_5(3), 8);
        assert_eq!(add_5(10), 15);
    }

    #[test]
    fn test_higher_order_function() {
        fn compose<F, G>(f: F, g: G) -> impl Fn(i32) -> i32
        where
            F: Fn(i32) -> i32,
            G: Fn(i32) -> i32,
        {
            move |x| f(g(x))
        }

        let add_one = |x| x + 1;
        let double = |x| x * 2;

        let composed = compose(double, add_one);
        assert_eq!(composed(3), 8); // (3 + 1) * 2 = 8
    }

    #[test]
    fn test_impl_trait_syntax() {
        fn create_formatter(prefix: &str) -> impl Fn(&str) -> String {
            let prefix = prefix.to_string();
            move |text| format!("{}: {}", prefix, text)
        }

        let formatter = create_formatter("TEST");
        assert_eq!(formatter("message"), "TEST: message");
    }
}
