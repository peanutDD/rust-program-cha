//! # 闭包的高级模式和特性
//!
//! 本模块展示闭包的高级用法，包括：
//! - 递归闭包
//! - 闭包工厂
//! - 闭包组合子
//! - 状态机闭包
//! - 高阶函数模式
//! - 函数式编程技巧

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// 演示闭包的高级模式和特性
pub fn demonstrate() {
    println!("\n🎯 8. 闭包的高级模式和特性");
    println!("{}", "-".repeat(40));

    recursive_closures();
    closure_factories();
    closure_combinators();
    state_machine_closures();
    higher_order_patterns();
    functional_programming_techniques();
}

/// 递归闭包演示
fn recursive_closures() {
    println!("\n🔄 递归闭包:");

    // 1. 使用 Rc<RefCell<>> 实现递归闭包
    type RecursiveFn = Rc<dyn Fn(i32) -> i32>;

    let factorial: RecursiveFn = {
        let factorial_ref = Rc::new(RefCell::new(None::<RecursiveFn>));
        let factorial_ref_clone = factorial_ref.clone();

        let f = Rc::new(move |n: i32| -> i32 {
            if n <= 1 {
                1
            } else {
                let factorial_fn = factorial_ref_clone.borrow();
                let factorial_fn = factorial_fn.as_ref().unwrap();
                n * factorial_fn(n - 1)
            }
        });

        *factorial_ref.borrow_mut() = Some(f.clone());
        f
    };

    println!("递归阶乘计算:");
    for i in 0..=6 {
        println!("{}! = {}", i, factorial(i));
    }

    // 2. 斐波那契数列递归闭包
    demonstrate_fibonacci_closure();

    // 3. 树遍历递归闭包
    demonstrate_tree_traversal();
}

/// 演示斐波那契递归闭包
fn demonstrate_fibonacci_closure() {
    println!("\n📝 斐波那契递归闭包:");

    // 使用记忆化的递归闭包
    let fibonacci = {
        let memo = Rc::new(RefCell::new(HashMap::new()));
        let fib_ref = Rc::new(RefCell::new(None::<Rc<dyn Fn(u32) -> u64>>));

        let f = {
            let memo = memo.clone();
            let fib_ref = fib_ref.clone();

            Rc::new(move |n: u32| -> u64 {
                // 检查缓存
                if let Some(&result) = memo.borrow().get(&n) {
                    return result;
                }

                let result = if n <= 1 {
                    n as u64
                } else {
                    let fib_fn = fib_ref.borrow();
                    let fib_fn = fib_fn.as_ref().unwrap();
                    fib_fn(n - 1) + fib_fn(n - 2)
                };

                // 缓存结果
                memo.borrow_mut().insert(n, result);
                result
            })
        };

        *fib_ref.borrow_mut() = Some(f.clone());
        f
    };

    println!("斐波那契数列 (带记忆化):");
    for i in 0..=10 {
        println!("fib({}) = {}", i, fibonacci(i));
    }
}

/// 演示树遍历递归闭包
fn demonstrate_tree_traversal() {
    println!("\n🌳 树遍历递归闭包:");

    #[derive(Debug, Clone)]
    struct TreeNode {
        value: i32,
        children: Vec<TreeNode>,
    }

    impl TreeNode {
        fn new(value: i32) -> Self {
            TreeNode {
                value,
                children: Vec::new(),
            }
        }

        fn add_child(mut self, child: TreeNode) -> Self {
            self.children.push(child);
            self
        }
    }

    // 构建示例树
    let tree = TreeNode::new(1)
        .add_child(
            TreeNode::new(2)
                .add_child(TreeNode::new(4))
                .add_child(TreeNode::new(5)),
        )
        .add_child(TreeNode::new(3).add_child(TreeNode::new(6)));

    // 深度优先遍历闭包
    let dfs_traversal = {
        let traverse_ref = Rc::new(RefCell::new(None::<Rc<dyn Fn(&TreeNode, &mut Vec<i32>)>>));

        let f = {
            let traverse_ref = traverse_ref.clone();
            Rc::new(move |node: &TreeNode, result: &mut Vec<i32>| {
                result.push(node.value);

                let traverse_fn = traverse_ref.borrow();
                let traverse_fn = traverse_fn.as_ref().unwrap();

                for child in &node.children {
                    traverse_fn(child, result);
                }
            })
        };

        *traverse_ref.borrow_mut() = Some(f.clone());
        f
    };

    let mut result = Vec::new();
    dfs_traversal(&tree, &mut result);
    println!("深度优先遍历结果: {:?}", result);
}

/// 闭包工厂演示
fn closure_factories() {
    println!("\n🏭 闭包工厂:");

    // 1. 数学函数工厂
    fn create_polynomial(coefficients: Vec<f64>) -> impl Fn(f64) -> f64 {
        move |x| {
            coefficients
                .iter()
                .enumerate()
                .map(|(i, &coeff)| coeff * x.powi(i as i32))
                .sum()
        }
    }

    // 创建不同的多项式函数
    let linear = create_polynomial(vec![1.0, 2.0]); // f(x) = 1 + 2x
    let quadratic = create_polynomial(vec![0.0, 1.0, 1.0]); // f(x) = x + x²
    let cubic = create_polynomial(vec![1.0, 0.0, 0.0, 1.0]); // f(x) = 1 + x³

    println!("多项式函数工厂:");
    let x = 2.0;
    println!("线性函数 f({}) = {}", x, linear(x));
    println!("二次函数 f({}) = {}", x, quadratic(x));
    println!("三次函数 f({}) = {}", x, cubic(x));

    // 2. 验证器工厂
    demonstrate_validator_factory();

    // 3. 配置驱动的闭包工厂
    demonstrate_config_driven_factory();
}

/// 演示验证器工厂
fn demonstrate_validator_factory() {
    println!("\n✅ 验证器工厂:");

    // 创建不同类型的验证器
    fn create_range_validator(min: i32, max: i32) -> impl Fn(i32) -> Result<i32, String> {
        move |value| {
            if value >= min && value <= max {
                Ok(value)
            } else {
                Err(format!("值 {} 不在范围 [{}, {}] 内", value, min, max))
            }
        }
    }

    fn create_string_validator(
        min_len: usize,
        max_len: usize,
    ) -> impl Fn(&str) -> Result<String, String> {
        move |value| {
            let len = value.len();
            if len >= min_len && len <= max_len {
                Ok(value.to_string())
            } else {
                Err(format!(
                    "字符串长度 {} 不在范围 [{}, {}] 内",
                    len, min_len, max_len
                ))
            }
        }
    }

    // 组合验证器
    fn create_composite_validator<T, F1, F2>(
        validator1: F1,
        validator2: F2,
    ) -> impl Fn(T) -> Result<T, String>
    where
        T: Clone,
        F1: Fn(T) -> Result<T, String>,
        F2: Fn(T) -> Result<T, String>,
    {
        move |value| validator1(value.clone()).and_then(|v| validator2(v))
    }

    let age_validator = create_range_validator(0, 120);
    let name_validator = create_string_validator(2, 50);

    // 使用组合验证器的示例
    let positive_age_validator =
        create_composite_validator(create_range_validator(1, 120), |age: i32| {
            if age > 0 {
                Ok(age)
            } else {
                Err("年龄必须为正数".to_string())
            }
        });

    println!("\n组合验证器测试:");
    println!("{:?}", positive_age_validator(25));
    println!("{:?}", positive_age_validator(0));
    println!("{:?}", positive_age_validator(150));

    // 测试验证器
    println!("年龄验证:");
    println!("{:?}", age_validator(25));
    println!("{:?}", age_validator(150));

    println!("\n姓名验证:");
    println!("{:?}", name_validator("张三"));
    println!("{:?}", name_validator("A"));
}

/// 演示配置驱动的闭包工厂
fn demonstrate_config_driven_factory() {
    println!("\n⚙️ 配置驱动的闭包工厂:");

    #[derive(Debug, Clone)]
    enum Operation {
        Add(f64),
        Multiply(f64),
        Power(f64),
        Compose(Box<Operation>, Box<Operation>),
    }

    fn create_operation_closure(op: Operation) -> Box<dyn Fn(f64) -> f64> {
        match op {
            Operation::Add(n) => Box::new(move |x| x + n),
            Operation::Multiply(n) => Box::new(move |x| x * n),
            Operation::Power(n) => Box::new(move |x| x.powf(n)),
            Operation::Compose(op1, op2) => {
                let f1 = create_operation_closure(*op1);
                let f2 = create_operation_closure(*op2);
                Box::new(move |x| f2(f1(x)))
            }
        }
    }

    // 创建复合操作：(x + 2) * 3
    let complex_op = Operation::Compose(
        Box::new(Operation::Add(2.0)),
        Box::new(Operation::Multiply(3.0)),
    );

    let operation_fn = create_operation_closure(complex_op);

    println!("配置驱动的操作:");
    for x in [1.0, 2.0, 3.0, 4.0] {
        println!("f({}) = {}", x, operation_fn(x));
    }
}

/// 闭包组合子演示
fn closure_combinators() {
    println!("\n🔗 闭包组合子:");

    // 1. 函数组合
    fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
    where
        F: Fn(A) -> B,
        G: Fn(B) -> C,
    {
        move |x| g(f(x))
    }

    let add_one = |x: i32| x + 1;
    let multiply_two = |x: i32| x * 2;
    let square = |x: i32| x * x;

    // 组合函数：先加1，再乘2
    let add_then_multiply = compose(add_one, multiply_two);

    // 组合函数：先加1，再平方
    let add_then_square = compose(add_one, square);

    println!("函数组合:");
    println!("(5 + 1) * 2 = {}", add_then_multiply(5));
    println!("(5 + 1)² = {}", add_then_square(5));

    // 2. 管道操作
    demonstrate_pipeline_combinators();

    // 3. 条件组合子
    demonstrate_conditional_combinators();
}

/// 演示管道组合子
fn demonstrate_pipeline_combinators() {
    println!("\n🚰 管道组合子:");

    // 管道宏（简化版）
    macro_rules! pipe {
        ($value:expr) => { $value };
        ($value:expr, $func:expr) => { $func($value) };
        ($value:expr, $func:expr, $($rest:expr),+) => {
            pipe!($func($value), $($rest),+)
        };
    }

    let add_10 = |x: i32| x + 10;
    let multiply_3 = |x: i32| x * 3;
    let subtract_5 = |x: i32| x - 5;

    let result = pipe!(5, add_10, multiply_3, subtract_5);
    println!("管道操作 5 |> (+10) |> (*3) |> (-5) = {}", result);

    // 更复杂的管道
    let to_string = |x: i32| x.to_string();
    let add_prefix = |s: String| format!("结果: {}", s);
    let to_uppercase = |s: String| s.to_uppercase();

    let final_result = pipe!(42, add_10, multiply_3, to_string, add_prefix, to_uppercase);
    println!("复杂管道: {}", final_result);
}

/// 演示条件组合子
fn demonstrate_conditional_combinators() {
    println!("\n🔀 条件组合子:");

    // 条件执行组合子
    fn when<T, F>(condition: bool, f: F) -> impl Fn(T) -> T
    where
        F: Fn(T) -> T,
        T: Clone,
    {
        move |x| {
            if condition {
                f(x)
            } else {
                x
            }
        }
    }

    // 条件选择组合子
    fn if_else<T, P, F1, F2>(predicate: P, then_fn: F1, else_fn: F2) -> impl Fn(T) -> T
    where
        P: Fn(&T) -> bool,
        F1: Fn(T) -> T,
        F2: Fn(T) -> T,
    {
        move |x| {
            if predicate(&x) {
                then_fn(x)
            } else {
                else_fn(x)
            }
        }
    }

    let double = |x: i32| x * 2;
    let negate = |x: i32| -x;
    let _identity = |x: i32| x;

    // 条件执行
    let maybe_double = when(true, double);
    println!("条件执行 (true): {}", maybe_double(5));

    let maybe_double_false = when(false, double);
    println!("条件执行 (false): {}", maybe_double_false(5));

    // 条件选择
    let process_number = if_else(|&x| x > 0, double, negate);

    println!("条件选择:");
    println!("正数 5: {}", process_number(5));
    println!("负数 -3: {}", process_number(-3));
}

/// 状态机闭包演示
fn state_machine_closures() {
    println!("\n🎰 状态机闭包:");

    // 1. 简单计数器状态机
    fn create_counter() -> impl FnMut() -> i32 {
        let mut count = 0;
        move || {
            count += 1;
            count
        }
    }

    let mut counter = create_counter();
    println!("计数器状态机:");
    for _ in 0..5 {
        println!("计数: {}", counter());
    }

    // 2. 有限状态机
    demonstrate_finite_state_machine();

    // 3. 状态转换闭包
    demonstrate_state_transitions();
}

/// 演示有限状态机
fn demonstrate_finite_state_machine() {
    println!("\n🔄 有限状态机:");

    #[derive(Debug, Clone, PartialEq)]
    enum State {
        Idle,
        Running,
        Paused,
        Stopped,
    }

    #[derive(Debug, Clone)]
    enum Event {
        Start,
        Pause,
        Resume,
        Stop,
        Reset,
    }

    fn create_state_machine() -> impl FnMut(Event) -> State {
        let mut current_state = State::Idle;

        move |event| {
            current_state = match (&current_state, event) {
                (State::Idle, Event::Start) => State::Running,
                (State::Running, Event::Pause) => State::Paused,
                (State::Running, Event::Stop) => State::Stopped,
                (State::Paused, Event::Resume) => State::Running,
                (State::Paused, Event::Stop) => State::Stopped,
                (State::Stopped, Event::Reset) => State::Idle,
                _ => current_state.clone(), // 无效转换，保持当前状态
            };

            current_state.clone()
        }
    }

    let mut state_machine = create_state_machine();

    let events = vec![
        Event::Start,
        Event::Pause,
        Event::Resume,
        Event::Stop,
        Event::Reset,
    ];

    println!("状态机转换:");
    for event in events {
        let new_state = state_machine(event.clone());
        println!("{:?} -> {:?}", event, new_state);
    }
}

/// 演示状态转换闭包
fn demonstrate_state_transitions() {
    println!("\n⚡ 状态转换闭包:");

    // 累积器状态机
    fn create_accumulator<T, F>(initial: T, operation: F) -> impl FnMut(T) -> T
    where
        T: Clone,
        F: Fn(T, T) -> T,
    {
        let mut state = initial;
        move |input| {
            state = operation(state.clone(), input);
            state.clone()
        }
    }

    // 数字累加器
    let mut sum_accumulator = create_accumulator(0, |acc, x| acc + x);

    println!("累加器:");
    for i in 1..=5 {
        let result = sum_accumulator(i);
        println!("累加 {} -> 总和: {}", i, result);
    }

    // 字符串连接器
    let mut string_accumulator = create_accumulator(String::new(), |mut acc, s: String| {
        if !acc.is_empty() {
            acc.push_str(", ");
        }
        acc.push_str(&s);
        acc
    });

    println!("\n字符串连接器:");
    let words = vec!["Hello", "World", "from", "Rust"];
    for word in words {
        let result = string_accumulator(word.to_string());
        println!("添加 '{}' -> 结果: '{}'", word, result);
    }
}

/// 高阶函数模式演示
fn higher_order_patterns() {
    println!("\n🎭 高阶函数模式:");

    // 1. 柯里化
    demonstrate_currying();

    // 2. 部分应用
    demonstrate_partial_application();

    // 3. 函数装饰器
    demonstrate_function_decorators();
}

/// 演示柯里化
fn demonstrate_currying() {
    println!("\n🍛 柯里化:");

    // 手动柯里化
    fn curry_add(x: i32) -> impl Fn(i32) -> i32 {
        move |y| x + y
    }

    fn curry_multiply(x: i32) -> Box<dyn Fn(i32) -> Box<dyn Fn(i32) -> i32>> {
        Box::new(move |y| Box::new(move |z| x * y * z))
    }

    let add_5 = curry_add(5);
    println!("柯里化加法: add_5(3) = {}", add_5(3));

    let multiply_2_3 = curry_multiply(2)(3);
    println!("柯里化乘法: multiply_2_3(4) = {}", multiply_2_3(4));

    // 通用柯里化宏
    macro_rules! curry {
        ($func:ident, $arg1:expr) => {
            move |arg2| $func($arg1, arg2)
        };
        ($func:ident, $arg1:expr, $arg2:expr) => {
            move |arg3| $func($arg1, $arg2, arg3)
        };
    }

    fn add_three(a: i32, b: i32, c: i32) -> i32 {
        a + b + c
    }

    let add_1_2 = curry!(add_three, 1, 2);
    println!("宏柯里化: add_1_2(3) = {}", add_1_2(3));
}

/// 演示部分应用
fn demonstrate_partial_application() {
    println!("\n🔧 部分应用:");

    // 部分应用函数
    fn partial_apply<A, B, C, F>(f: F, a: A) -> impl Fn(B) -> C
    where
        F: Fn(A, B) -> C,
        A: Clone,
    {
        move |b| f(a.clone(), b)
    }

    fn power(base: f64, exponent: f64) -> f64 {
        base.powf(exponent)
    }

    let square = partial_apply(power, 2.0);
    let cube = partial_apply(|base, _| power(base, 3.0), 0.0);

    println!("部分应用:");
    println!("2^4 = {}", square(4.0));
    println!("5³ = {}", cube(5.0));

    // 配置驱动的部分应用
    #[derive(Clone)]
    struct Config {
        prefix: String,
        suffix: String,
        uppercase: bool,
    }

    fn format_string(config: Config, text: String) -> String {
        let mut result = format!("{}{}{}", config.prefix, text, config.suffix);
        if config.uppercase {
            result = result.to_uppercase();
        }
        result
    }

    let config = Config {
        prefix: "[INFO] ".to_string(),
        suffix: " - OK".to_string(),
        uppercase: true,
    };

    let log_formatter = partial_apply(format_string, config);

    println!("配置驱动格式化:");
    println!("{}", log_formatter("System started".to_string()));
    println!("{}", log_formatter("Process completed".to_string()));
}

/// 演示函数装饰器
fn demonstrate_function_decorators() {
    println!("\n🎨 函数装饰器:");

    // 计时装饰器
    fn with_timing<F, T>(f: F) -> impl Fn() -> T
    where
        F: Fn() -> T,
    {
        move || {
            let start = std::time::Instant::now();
            let result = f();
            let duration = start.elapsed();
            println!("执行时间: {:?}", duration);
            result
        }
    }

    // 重试装饰器
    fn with_retry<F, T, E>(f: F, max_attempts: usize) -> impl Fn() -> Result<T, E>
    where
        F: Fn() -> Result<T, E>,
        E: std::fmt::Debug,
    {
        move || {
            for attempt in 1..=max_attempts {
                match f() {
                    Ok(result) => return Ok(result),
                    Err(e) => {
                        if attempt == max_attempts {
                            return Err(e);
                        }
                        println!("尝试 {} 失败: {:?}", attempt, e);
                    }
                }
            }
            unreachable!()
        }
    }

    // 缓存装饰器
    fn with_cache<F, T>(f: F) -> impl FnMut() -> T
    where
        F: Fn() -> T,
        T: Clone,
    {
        let mut cache: Option<T> = None;
        move || {
            if let Some(ref cached) = cache {
                println!("使用缓存结果");
                cached.clone()
            } else {
                println!("计算新结果");
                let result = f();
                cache = Some(result.clone());
                result
            }
        }
    }

    // 测试装饰器
    let expensive_computation = || {
        std::thread::sleep(std::time::Duration::from_millis(100));
        42
    };

    let timed_computation = with_timing(expensive_computation);
    println!("计时装饰器:");
    let result = timed_computation();
    println!("结果: {}", result);

    // 测试缓存装饰器
    let mut cached_computation = with_cache(expensive_computation);
    println!("\n缓存装饰器:");
    println!("第一次调用: {}", cached_computation());
    println!("第二次调用: {}", cached_computation());
}

/// 函数式编程技巧演示
fn functional_programming_techniques() {
    println!("\n🧮 函数式编程技巧:");

    // 1. 单子模式
    demonstrate_monad_patterns();

    // 2. 函子模式
    demonstrate_functor_patterns();

    // 3. 应用函子模式
    demonstrate_applicative_patterns();
}

/// 演示单子模式
fn demonstrate_monad_patterns() {
    println!("\n🔗 单子模式 (Option/Result):");

    // Option 单子链式操作
    let process_number = |x: i32| -> Option<String> {
        Some(x)
            .filter(|&n| n > 0) // 过滤正数
            .map(|n| n * 2) // 乘以2
            .filter(|&n| n < 100) // 过滤小于100的数
            .map(|n| format!("处理结果: {}", n)) // 格式化
    };

    println!("Option 单子链:");
    println!("{:?}", process_number(10)); // Some
    println!("{:?}", process_number(-5)); // None (负数)
    println!("{:?}", process_number(60)); // None (结果>=100)

    // Result 单子错误处理
    let safe_divide = |a: f64, b: f64| -> Result<f64, String> {
        if b == 0.0 {
            Err("除零错误".to_string())
        } else {
            Ok(a / b)
        }
    };

    let complex_calculation = |x: f64| -> Result<f64, String> {
        safe_divide(x, 2.0)
            .and_then(|result| safe_divide(result, 3.0))
            .and_then(|result| safe_divide(result, 4.0))
            .map(|result| result * 100.0)
    };

    println!("\nResult 单子链:");
    println!("{:?}", complex_calculation(24.0));
    println!("{:?}", complex_calculation(0.0));
}

/// 演示函子模式
fn demonstrate_functor_patterns() {
    println!("\n📦 函子模式:");

    // 自定义容器函子
    #[derive(Debug)]
    struct Container<T> {
        value: T,
    }

    impl<T> Container<T> {
        fn new(value: T) -> Self {
            Container { value }
        }

        fn map<U, F>(self, f: F) -> Container<U>
        where
            F: FnOnce(T) -> U,
        {
            Container::new(f(self.value))
        }
    }

    let container = Container::new(42)
        .map(|x| x * 2)
        .map(|x| x + 10)
        .map(|x| format!("结果: {}", x));

    println!("自定义函子: {:?}", container);

    // Vec 作为函子
    let numbers = vec![1, 2, 3, 4, 5];
    let processed: Vec<String> = numbers
        .into_iter()
        .map(|x| x * x) // 平方
        .map(|x| x + 1) // 加1
        .map(|x| format!("#{}", x)) // 格式化
        .collect();

    println!("Vec 函子: {:?}", processed);
}

/// 演示应用函子模式
fn demonstrate_applicative_patterns() {
    println!("\n⚡ 应用函子模式:");

    // 多参数函数的应用
    fn apply_to_options<A, B, C, F>(opt_a: Option<A>, opt_b: Option<B>, f: F) -> Option<C>
    where
        F: FnOnce(A, B) -> C,
    {
        opt_a.and_then(|a| opt_b.map(|b| f(a, b)))
    }

    let add = |a: i32, b: i32| a + b;
    let multiply = |a: i32, b: i32| a * b;

    println!("应用函子操作:");
    println!("{:?}", apply_to_options(Some(5), Some(3), add));
    println!("{:?}", apply_to_options(Some(5), None, add));
    println!("{:?}", apply_to_options(Some(4), Some(6), multiply));

    // 验证多个值
    fn validate_all<T, E>(validations: Vec<Result<T, E>>) -> Result<Vec<T>, Vec<E>>
    where
        E: Clone,
    {
        let mut values = Vec::new();
        let mut errors = Vec::new();

        for validation in validations {
            match validation {
                Ok(value) => values.push(value),
                Err(error) => errors.push(error),
            }
        }

        if errors.is_empty() {
            Ok(values)
        } else {
            Err(errors)
        }
    }

    let validations = vec![
        Ok(1),
        Ok(2),
        Err("错误1".to_string()),
        Ok(3),
        Err("错误2".to_string()),
    ];

    println!("\n批量验证:");
    match validate_all(validations) {
        Ok(values) => println!("所有验证通过: {:?}", values),
        Err(errors) => println!("验证失败: {:?}", errors),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive_closure() {
        type RecursiveFn = Rc<dyn Fn(i32) -> i32>;

        let factorial: RecursiveFn = {
            let factorial_ref = Rc::new(RefCell::new(None::<RecursiveFn>));
            let factorial_ref_clone = factorial_ref.clone();

            let f = Rc::new(move |n: i32| -> i32 {
                if n <= 1 {
                    1
                } else {
                    let factorial_fn = factorial_ref_clone.borrow();
                    let factorial_fn = factorial_fn.as_ref().unwrap();
                    n * factorial_fn(n - 1)
                }
            });

            *factorial_ref.borrow_mut() = Some(f.clone());
            f
        };

        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_closure_factory() {
        fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
            move |x| x * factor
        }

        let double = create_multiplier(2);
        let triple = create_multiplier(3);

        assert_eq!(double(5), 10);
        assert_eq!(triple(4), 12);
    }

    #[test]
    fn test_function_composition() {
        fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
        where
            F: Fn(A) -> B,
            G: Fn(B) -> C,
        {
            move |x| g(f(x))
        }

        let add_one = |x: i32| x + 1;
        let double = |x: i32| x * 2;

        let add_then_double = compose(add_one, double);
        assert_eq!(add_then_double(5), 12); // (5 + 1) * 2
    }

    #[test]
    fn test_state_machine() {
        fn create_counter() -> impl FnMut() -> i32 {
            let mut count = 0;
            move || {
                count += 1;
                count
            }
        }

        let mut counter = create_counter();
        assert_eq!(counter(), 1);
        assert_eq!(counter(), 2);
        assert_eq!(counter(), 3);
    }

    #[test]
    fn test_currying() {
        fn curry_add(x: i32) -> impl Fn(i32) -> i32 {
            move |y| x + y
        }

        let add_5 = curry_add(5);
        assert_eq!(add_5(3), 8);
        assert_eq!(add_5(7), 12);
    }

    #[test]
    fn test_monad_patterns() {
        let process = |x: i32| -> Option<i32> {
            Some(x)
                .filter(|&n| n > 0)
                .map(|n| n * 2)
                .filter(|&n| n < 20)
        };

        assert_eq!(process(5), Some(10));
        assert_eq!(process(-1), None);
        assert_eq!(process(15), None);
    }
}
