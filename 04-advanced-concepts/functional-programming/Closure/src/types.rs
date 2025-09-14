//! # 闭包的类型推导和类型注解
//!
//! 本模块详细介绍 Rust 闭包的类型系统，包括：
//! - 闭包的类型推导机制
//! - 显式类型注解
//! - 闭包类型的唯一性
//! - 类型推导的限制和规则
//! - 实际应用中的类型处理

/// 演示闭包的类型推导和类型注解
pub fn demonstrate() {
    println!("\n🔍 4. 闭包的类型推导和类型注解");
    println!("{}", "-".repeat(40));

    type_inference_basics();
    explicit_type_annotations();
    closure_type_uniqueness();
    type_inference_limitations();
    practical_type_handling();
}

/// 演示类型推导基础
fn type_inference_basics() {
    println!("\n📝 类型推导基础:");

    // 1. 基本类型推导
    let simple_closure = |x| x + 1;

    // 第一次使用确定类型
    let result1 = simple_closure(5i32); // 推导为 |i32| -> i32
    println!("simple_closure(5i32) = {}", result1);

    // 后续使用必须保持一致
    let result2 = simple_closure(10i32);
    println!("simple_closure(10i32) = {}", result2);

    // 以下代码会编译错误，因为类型已经确定为 i32
    // let result3 = simple_closure(3.14f64);  // 编译错误！

    // 2. 从上下文推导类型
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("从上下文推导: {:?} -> {:?}", numbers, doubled);

    // 3. 返回值类型推导
    let calculate = |a, b| {
        if a > b {
            a - b
        } else {
            b - a
        }
    };

    let diff = calculate(10, 7); // 推导为 i32
    println!("calculate(10, 7) = {}", diff);

    demonstrate_inference_with_generics();
}

/// 演示泛型中的类型推导
fn demonstrate_inference_with_generics() {
    println!("\n📝 泛型中的类型推导:");

    // 泛型函数中的闭包类型推导
    fn process_with_closure<T, F>(value: T, processor: F) -> T
    where
        F: Fn(T) -> T,
        T: std::fmt::Display + Copy,
    {
        println!("处理前: {}", value);
        let result = processor(value);
        println!("处理后: {}", result);
        result
    }

    // 整数处理
    let int_result = process_with_closure(42, |x| x * 2);
    println!("整数处理结果: {}", int_result);

    // 浮点数处理
    let float_result = process_with_closure(3.14, |x| x * x);
    println!("浮点数处理结果: {}", float_result);

    // 字符处理
    let char_result = process_with_closure('A', |c| char::from_u32(c as u32 + 1).unwrap_or(c));
    println!("字符处理结果: {}", char_result);
}

/// 演示显式类型注解
fn explicit_type_annotations() {
    println!("\n📝 显式类型注解:");

    // 1. 参数类型注解
    let typed_closure =
        |x: i32, y: f64| -> String { format!("整数: {}, 浮点数: {:.2}", x, y) };

    let result = typed_closure(42, 3.14159);
    println!("显式类型注解结果: {}", result);

    // 2. 复杂类型注解
    let complex_closure = |data: Vec<String>| -> (usize, String) {
        let count = data.len();
        let joined = data.join(", ");
        println!("处理 {} 个字符串: {}", count, joined);
        (count, joined)
    };

    let strings = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];
    let (count, joined) = complex_closure(strings);
    println!("结果: 数量={}, 连接={}", count, joined);

    // 3. 引用类型注解
    let ref_closure = |s: &str, n: &mut i32| -> String {
        *n += s.len() as i32;
        format!("字符串 '{}' 长度: {}, 累计: {}", s, s.len(), *n)
    };

    let mut total = 0;
    let result1 = ref_closure("hello", &mut total);
    let result2 = ref_closure("world", &mut total);
    println!("{}", result1);
    println!("{}", result2);

    demonstrate_lifetime_annotations();
}

/// 演示生命周期注解
fn demonstrate_lifetime_annotations() {
    println!("\n📝 生命周期注解:");

    // 带生命周期的闭包
    fn create_formatter<'a>() -> impl Fn(&'a str) -> String + 'a {
        |s: &'a str| format!("格式化: [{}]", s.to_uppercase())
    }

    let formatter = create_formatter();
    let text = "hello world";
    let formatted = formatter(text);
    println!("{}", formatted);

    // 多个生命周期参数
    let combine_strings =
        |s1: &str, s2: &str| -> String { format!("{} + {} = {}", s1, s2, format!("{}{}", s1, s2)) };

    let str1 = "Hello";
    let str2 = "World";
    let combined = combine_strings(str1, str2);
    println!("{}", combined);
}

/// 演示闭包类型的唯一性
fn closure_type_uniqueness() {
    println!("\n📝 闭包类型的唯一性:");

    // 每个闭包都有唯一的类型，即使签名相同
    let closure1 = |x: i32| x + 1;
    let closure2 = |x: i32| x + 1; // 与 closure1 签名相同，但类型不同

    println!("closure1(5) = {}", closure1(5));
    println!("closure2(5) = {}", closure2(5));

    // 以下代码会编译错误，因为类型不匹配
    // let same_closure = closure1;
    // same_closure = closure2;  // 编译错误！类型不匹配

    // 使用 trait object 存储不同的闭包
    let closures: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 2),
        Box::new(|x| x * x),
    ];

    println!("\n使用 trait object 存储不同闭包:");
    for (i, closure) in closures.iter().enumerate() {
        let result = closure(3);
        println!("闭包 {}: f(3) = {}", i + 1, result);
    }

    demonstrate_closure_as_type_parameter();
}

/// 演示闭包作为类型参数
fn demonstrate_closure_as_type_parameter() {
    println!("\n📝 闭包作为类型参数:");

    // 泛型结构体持有闭包
    struct Processor<F>
    where
        F: Fn(i32) -> i32,
    {
        name: String,
        operation: F,
    }

    impl<F> Processor<F>
    where
        F: Fn(i32) -> i32,
    {
        fn new(name: &str, operation: F) -> Self {
            Processor {
                name: name.to_string(),
                operation,
            }
        }

        fn process(&self, value: i32) -> i32 {
            println!("{}处理器处理 {}", self.name, value);
            (self.operation)(value)
        }
    }

    // 创建不同的处理器
    let doubler = Processor::new("双倍", |x| x * 2);
    let squarer = Processor::new("平方", |x| x * x);
    let incrementer = Processor::new("递增", |x| x + 1);

    let test_value = 5;
    println!("测试值: {}", test_value);
    println!("双倍处理器: {}", doubler.process(test_value));
    println!("平方处理器: {}", squarer.process(test_value));
    println!("递增处理器: {}", incrementer.process(test_value));
}

/// 演示类型推导的限制
fn type_inference_limitations() {
    println!("\n📝 类型推导的限制:");

    // 1. 类型一旦确定就不能改变
    let flexible_closure = |x| x;

    // 第一次使用确定类型
    let _result1 = flexible_closure(42i32);
    println!("第一次使用确定为 i32 类型");

    // 后续使用必须保持一致
    let _result2 = flexible_closure(100i32);

    // 以下会编译错误
    // let _result3 = flexible_closure(3.14f64);  // 编译错误！

    // 2. 需要明确类型的情况
    demonstrate_ambiguous_cases();

    // 3. 递归闭包的类型问题
    demonstrate_recursive_closure_types();
}

/// 演示模糊类型情况
fn demonstrate_ambiguous_cases() {
    use std::collections::HashMap;

    println!("\n📝 模糊类型情况:");

    // 情况1: 需要类型注解的泛型
    let parse_closure = |s: &str| -> Result<i32, _> { s.parse() };

    match parse_closure("42") {
        Ok(n) => println!("解析成功: {}", n),
        Err(e) => println!("解析失败: {}", e),
    }

    // 情况2: 集合类型需要明确
    let collect_closure =
        |iter: std::vec::IntoIter<i32>| -> Vec<i32> { iter.filter(|&x| x > 0).collect() };

    let numbers = vec![-2, -1, 0, 1, 2, 3];
    let positive = collect_closure(numbers.into_iter());
    println!("正数: {:?}", positive);

    // 情况3: 需要显式指定的复杂类型
    let complex_closure: Box<dyn Fn(Vec<String>) -> HashMap<String, usize>> = Box::new(|strings| {
        let mut map = HashMap::new();
        for s in strings {
            let len = s.len();
            map.insert(s, len);
        }
        map
    });

    let words = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];
    let word_lengths = complex_closure(words);
    println!("单词长度映射: {:?}", word_lengths);
}

/// 演示递归闭包的类型处理
fn demonstrate_recursive_closure_types() {
    println!("\n📝 递归闭包的类型处理:");

    use std::rc::Rc;

    // 使用 Rc 和 RefCell 实现递归闭包
    type FactorialFn = Rc<dyn Fn(i32) -> i32>;

    let factorial_ref: Rc<std::cell::RefCell<Option<FactorialFn>>> =
        Rc::new(std::cell::RefCell::new(None));

    let factorial_clone = factorial_ref.clone();
    let f = Rc::new(move |n: i32| -> i32 {
        if n <= 1 {
            1
        } else {
            let factorial_fn = factorial_clone.borrow();
            let factorial_fn = factorial_fn.as_ref().unwrap();
            n * factorial_fn(n - 1)
        }
    });

    *factorial_ref.borrow_mut() = Some(f.clone());
    let factorial: FactorialFn = f;

    // 测试递归闭包
    for i in 1..=6 {
        let result = factorial(i);
        println!("{}! = {}", i, result);
    }
}

/// 实际应用中的类型处理
fn practical_type_handling() {
    println!("\n📝 实际应用中的类型处理:");

    // 1. 函数式编程中的类型链
    functional_type_chains();

    // 2. 错误处理中的类型
    error_handling_types();

    // 3. 异步编程中的类型
    async_closure_types();
}

/// 函数式编程中的类型链
fn functional_type_chains() {
    println!("\n📝 函数式编程中的类型链:");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 复杂的函数式链，每个闭包都有明确的类型
    let result: Vec<String> = numbers
        .into_iter()
        .filter(|&x| x % 2 == 0) // |&i32| -> bool
        .map(|x| x * x) // |i32| -> i32
        .filter(|&x| x > 10) // |&i32| -> bool
        .map(|x| format!("数字: {}", x)) // |i32| -> String
        .collect();

    println!("函数式链结果: {:?}", result);

    // 使用显式类型注解提高可读性
    let data = vec!["1", "2", "3", "abc", "4", "5"];
    let parsed_numbers: Vec<i32> = data
        .into_iter()
        .filter_map(|s: &str| -> Option<i32> { s.parse().ok() })
        .collect();

    println!("解析的数字: {:?}", parsed_numbers);
}

/// 错误处理中的类型
fn error_handling_types() {
    println!("\n📝 错误处理中的类型:");

    // 自定义错误类型的闭包
    #[derive(Debug)]
    enum ProcessError {
        InvalidInput,
        ComputationError,
    }

    let safe_divide = |a: f64, b: f64| -> Result<f64, ProcessError> {
        if b == 0.0 {
            Err(ProcessError::ComputationError)
        } else if a.is_nan() || b.is_nan() {
            Err(ProcessError::InvalidInput)
        } else {
            Ok(a / b)
        }
    };

    // 测试错误处理
    let test_cases = vec![(10.0, 2.0), (5.0, 0.0), (f64::NAN, 1.0)];

    for (a, b) in test_cases {
        match safe_divide(a, b) {
            Ok(result) => println!("{} / {} = {}", a, b, result),
            Err(e) => println!("{} / {} 错误: {:?}", a, b, e),
        }
    }

    // 链式错误处理
    let process_string = |s: &str| -> Result<i32, Box<dyn std::error::Error>> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err("空字符串".into());
        }
        let number: i32 = trimmed.parse()?;
        if number < 0 {
            return Err("负数不允许".into());
        }
        Ok(number * 2)
    };

    let test_strings = vec!["  42  ", "", "abc", "-5", "10"];
    for s in test_strings {
        match process_string(s) {
            Ok(result) => println!("'{}' -> {}", s, result),
            Err(e) => println!("'{}' 错误: {}", s, e),
        }
    }
}

/// 异步编程中的类型（模拟）
fn async_closure_types() {
    println!("\n📝 异步编程中的类型（模拟）:");

    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    // 模拟异步闭包的类型
    type AsyncClosure<T> = Box<dyn Fn() -> Pin<Box<dyn Future<Output = T>>>>;

    // 简单的 Future 实现
    struct SimpleFuture<T> {
        value: Option<T>,
    }

    impl<T> SimpleFuture<T> {
        fn new(value: T) -> Self {
            SimpleFuture { value: Some(value) }
        }
    }

    impl<T: std::marker::Unpin> Future for SimpleFuture<T> {
        type Output = T;

        fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            let this = self.get_mut();
            if let Some(value) = this.value.take() {
                Poll::Ready(value)
            } else {
                Poll::Pending
            }
        }
    }

    // 创建异步闭包
    let _async_computation: AsyncClosure<i32> = Box::new(|| Box::pin(SimpleFuture::new(42)));

    println!("创建了异步计算闭包（类型复杂但明确）");

    // 在实际应用中，通常使用 async/await 语法
    let sync_simulation = || {
        println!("模拟异步操作...");
        std::thread::sleep(std::time::Duration::from_millis(1));
        "异步操作完成".to_string()
    };

    let result = sync_simulation();
    println!("模拟结果: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_inference() {
        let closure = |x| x + 1;
        let result = closure(5i32);
        assert_eq!(result, 6);

        // 类型已确定，必须保持一致
        let result2 = closure(10i32);
        assert_eq!(result2, 11);
    }

    #[test]
    fn test_explicit_types() {
        let typed_closure = |x: i32, y: f64| -> String { format!("{}-{:.1}", x, y) };

        let result = typed_closure(42, 3.14);
        assert_eq!(result, "42-3.1");
    }

    #[test]
    fn test_closure_uniqueness() {
        let closure1 = |x: i32| x + 1;
        let closure2 = |x: i32| x + 1;

        // 虽然签名相同，但类型不同
        assert_eq!(closure1(5), 6);
        assert_eq!(closure2(5), 6);

        // 可以通过 trait object 统一处理
        let closures: Vec<Box<dyn Fn(i32) -> i32>> = vec![Box::new(closure1), Box::new(closure2)];

        assert_eq!(closures.len(), 2);
    }

    #[test]
    fn test_complex_types() {
        use std::collections::HashMap;

        let map_builder =
            |pairs: Vec<(String, i32)>| -> HashMap<String, i32> { pairs.into_iter().collect() };

        let pairs = vec![("a".to_string(), 1), ("b".to_string(), 2)];

        let map = map_builder(pairs);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("a"), Some(&1));
    }
}
