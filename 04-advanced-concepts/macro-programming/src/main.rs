//! # Rust 宏编程全面分析
//! 
//! 本项目基于 https://course.rs/advance/macro.html 的内容，
//! 全面深入地分析 Rust 宏编程的各个方面，包括：
//! - 宏的基本概念和分类
//! - 声明式宏 (macro_rules!)
//! - 过程宏 (Procedural Macros)
//! - 宏的高级特性和最佳实践
//! - 实际应用案例和生态系统

#![recursion_limit = "256"]

use std::collections::HashMap;
use std::fmt;

// ============================================================================
// 第一部分：宏编程基础
// ============================================================================

/// 演示宏与函数的区别
fn macro_vs_function_demo() {
    println!("\n=== 宏与函数的区别 ===");
    
    // 函数在运行时执行，参数必须是具体的值
    fn add_function(a: i32, b: i32) -> i32 {
        a + b
    }
    
    // 宏在编译时展开，可以接受任意类型的参数
    macro_rules! add_macro {
        ($a:expr, $b:expr) => {
            $a + $b
        };
    }
    
    println!("函数调用: add_function(2, 3) = {}", add_function(2, 3));
    println!("宏调用: add_macro!(2, 3) = {}", add_macro!(2, 3));
    println!("宏调用: add_macro!(2.5, 3.7) = {}", add_macro!(2.5, 3.7));
    println!("宏调用: add_macro!(\"Hello, \", \"World!\") = {}", add_macro!("Hello, ".to_string(), "World!"));
}

// ============================================================================
// 第二部分：声明式宏 (macro_rules!)
// ============================================================================

/// 基础的声明式宏示例
macro_rules! say_hello {
    () => {
        println!("Hello from macro!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
    ($name:expr, $age:expr) => {
        println!("Hello, {}! You are {} years old.", $name, $age);
    };
}

/// 演示片段指示符 (Fragment Specifiers)
macro_rules! fragment_demo {
    // expr: 表达式
    (expr $e:expr) => {
        println!("Expression: {} = {}", stringify!($e), $e);
    };
    // ident: 标识符
    (ident $i:ident) => {
        let $i = 42;
        println!("Identifier: {} = {}", stringify!($i), $i);
    };
    // ty: 类型
    (ty $t:ty) => {
        let _var: $t = Default::default();
        println!("Type: {}", stringify!($t));
    };
    // stmt: 语句
    (stmt $s:stmt) => {
        $s
        println!("Statement executed: {}", stringify!($s));
    };
    // pat: 模式
    (pat $p:pat) => {
        match 42 {
            $p => println!("Pattern matched: {}", stringify!($p)),
            _ => println!("Pattern not matched"),
        }
    };
}

/// 重复模式宏
macro_rules! create_function {
    // 创建多个函数
    ($($func_name:ident),*) => {
        $(
            fn $func_name() {
                println!("Function {} called", stringify!($func_name));
            }
        )*
    };
}

// 使用重复模式创建函数
create_function!(foo, bar, baz);

/// 复杂的重复模式：创建结构体和实现
macro_rules! create_struct {
    ($struct_name:ident { $($field_name:ident: $field_type:ty),* }) => {
        #[derive(Debug, Clone)]
        struct $struct_name {
            $($field_name: $field_type,)*
        }
        
        impl $struct_name {
            fn new($($field_name: $field_type),*) -> Self {
                Self {
                    $($field_name,)*
                }
            }
            
            fn describe(&self) {
                println!("Struct {}: ", stringify!($struct_name));
                $(
                    println!("  {}: {:?}", stringify!($field_name), self.$field_name);
                )*
            }
        }
    };
}

// 使用宏创建结构体
create_struct!(Person { name: String, age: u32, email: String });
create_struct!(Product { id: u64, name: String, price: f64 });

/// 演示声明式宏的各种特性
fn declarative_macros_demo() {
    println!("\n=== 声明式宏演示 ===");
    
    // 基础宏调用
    say_hello!();
    say_hello!("Alice");
    say_hello!("Bob", 25);
    
    println!("\n--- 片段指示符演示 ---");
    fragment_demo!(expr 2 + 3 * 4);
    fragment_demo!(ident my_var);
    fragment_demo!(ty Vec<i32>);
    fragment_demo!(stmt let x = 10);
    fragment_demo!(pat 42);
    
    println!("\n--- 重复模式演示 ---");
    foo();
    bar();
    baz();
    
    println!("\n--- 结构体创建演示 ---");
    let person = Person::new("Alice".to_string(), 30, "alice@example.com".to_string());
    person.describe();
    
    let product = Product::new(1, "Laptop".to_string(), 999.99);
    product.describe();
}

// ============================================================================
// 第三部分：高级宏模式
// ============================================================================

/// 递归宏：计算阶乘（使用编译时常量避免无限递归）
macro_rules! factorial {
    (0) => { 1 };
    (1) => { 1 };
    (2) => { 2 };
    (3) => { 6 };
    (4) => { 24 };
    (5) => { 120 };
    (6) => { 720 };
    ($n:expr) => {
        {
            // 运行时计算，避免编译时递归
            fn factorial_runtime(n: u64) -> u64 {
                if n <= 1 { 1 } else { n * factorial_runtime(n - 1) }
            }
            factorial_runtime($n)
        }
    };
}

/// 条件宏：根据条件生成不同代码
macro_rules! conditional_code {
    (debug) => {
        fn log_message(msg: &str) {
            println!("[DEBUG] {}", msg);
        }
    };
    (release) => {
        fn log_message(_msg: &str) {
            // 在 release 模式下不输出日志
        }
    };
}

// 根据编译配置选择不同的实现
#[cfg(debug_assertions)]
conditional_code!(debug);

#[cfg(not(debug_assertions))]
conditional_code!(release);

/// 宏卫生性演示
macro_rules! hygiene_demo {
    ($var:ident) => {
        let $var = "from macro";
        println!("Inside macro: {}", $var);
    };
}

/// TT Muncher 模式：处理任意数量的参数
macro_rules! tt_muncher {
    () => {
        println!("No more tokens to process");
    };
    ($first:tt $($rest:tt)*) => {
        println!("Processing token: {}", stringify!($first));
        tt_muncher!($($rest)*);
    };
}

/// 内部规则宏：使用 @ 前缀的私有规则
macro_rules! internal_rules {
    // 公共接口
    ($($items:expr),*) => {
        internal_rules!(@process [] $($items),*)
    };
    
    // 内部规则：累积处理
    (@process [$($acc:expr),*]) => {
        vec![$($acc),*]
    };
    (@process [$($acc:expr),*] $head:expr $(, $tail:expr)*) => {
        internal_rules!(@process [$($acc,)* $head] $($tail),*)
    };
}

/// 高级宏模式演示
fn advanced_macro_patterns_demo() {
    println!("\n=== 高级宏模式演示 ===");
    
    // 递归宏
    println!("factorial!(5) = {}", factorial!(5));
    
    // 宏卫生性
    let my_var = "from main";
    println!("Before macro: {}", my_var);
    hygiene_demo!(my_var);
    println!("After macro: {}", my_var); // 不会被宏内的变量影响
    
    // TT Muncher
    println!("\n--- TT Muncher 演示 ---");
    tt_muncher!(hello world 123 + - *);
    
    // 内部规则
    let result = internal_rules!(1, 2, 3, 4, 5);
    println!("Internal rules result: {:?}", result);
    
    // 条件编译日志
    log_message("This is a test message");
}

// ============================================================================
// 第四部分：实际应用案例
// ============================================================================

/// DSL (Domain Specific Language) 示例：HTML 生成器
macro_rules! html {
    // 自闭合标签
    ($tag:ident) => {
        format!("<{}>", stringify!($tag))
    };
    
    // 带内容的标签
    ($tag:ident { $content:expr }) => {
        format!("<{}>{}</{}>", stringify!($tag), $content, stringify!($tag))
    };
    
    // 带属性的标签
    ($tag:ident [$($attr:ident = $value:expr),*] { $content:expr }) => {
        format!("<{} {}>{}</{}>", 
                stringify!($tag),
                html!(@attrs $($attr = $value),*),
                $content,
                stringify!($tag))
    };
    
    // 处理属性
    (@attrs $($attr:ident = $value:expr),*) => {
        vec![$(format!("{}=\"{}\"", stringify!($attr), $value)),*].join(" ")
    };
    
    // 文本内容
    ($text:expr) => {
        $text.to_string()
    };
    
    // 多个元素
    ($($element:tt)*) => {
        vec![$(html!($element)),*].join("")
    };
}

/// 配置宏：生成配置结构体
macro_rules! config {
    (
        $struct_name:ident {
            $($field:ident: $type:ty = $default:expr),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            $(pub $field: $type,)*
        }
        
        impl Default for $struct_name {
            fn default() -> Self {
                Self {
                    $($field: $default,)*
                }
            }
        }
        
        impl $struct_name {
            pub fn new() -> Self {
                Default::default()
            }
            
            // 手动实现几个常用的 with_ 方法
            pub fn with_host(mut self, value: String) -> Self {
                self.host = value;
                self
            }
            
            pub fn with_port(mut self, value: u16) -> Self {
                self.port = value;
                self
            }
            
            pub fn with_database(mut self, value: String) -> Self {
                self.database = value;
                self
            }
        }
    };
}

// 使用配置宏
config! {
    DatabaseConfig {
        host: String = "localhost".to_string(),
        port: u16 = 5432,
        database: String = "mydb".to_string(),
        username: String = "user".to_string(),
        password: String = "password".to_string(),
        max_connections: u32 = 10,
        timeout: u64 = 30,
    }
}

/// 测试宏：简化测试编写
macro_rules! test_case {
    ($name:ident: $input:expr => $expected:expr) => {
        #[test]
        fn $name() {
            assert_eq!($input, $expected);
        }
    };
    
    ($name:ident: $input:expr => $expected:expr, $description:expr) => {
        #[test]
        fn $name() {
            assert_eq!($input, $expected, $description);
        }
    };
}

/// 性能测量宏
macro_rules! benchmark {
    ($name:expr, $code:block) => {
        {
            let start = std::time::Instant::now();
            let result = $code;
            let duration = start.elapsed();
            println!("Benchmark {}: {:?}", $name, duration);
            result
        }
    };
}

/// 实际应用案例演示
fn real_world_examples_demo() {
    println!("\n=== 实际应用案例演示 ===");
    
    // HTML DSL（简化版）
    println!("\n--- HTML DSL ---");
    let div_content = html!(div [class = "container", id = "main"] { "Welcome to Rust Macros" });
    let h1_content = html!(h1 { "Welcome to Rust Macros" });
    let p_content = html!(p [class = "description"] { "This is generated using a custom HTML DSL macro." });
    println!("HTML examples:");
    println!("{}", div_content);
    println!("{}", h1_content);
    println!("{}", p_content);
    
    // 配置系统
    println!("\n--- 配置系统 ---");
    let config = DatabaseConfig::new()
        .with_host("production.db.com".to_string())
        .with_port(3306)
        .with_database("prod_db".to_string());
    println!("Database config: {:#?}", config);
    
    // 性能测量
    println!("\n--- 性能测量 ---");
    let result = benchmark!("Vector creation", {
        let mut vec = Vec::new();
        for i in 0..1000 {
            vec.push(i);
        }
        vec.len()
    });
    println!("Vector length: {}", result);
}

// ============================================================================
// 第五部分：宏的调试和最佳实践
// ============================================================================

/// 调试宏：显示宏展开结果
macro_rules! debug_macro {
    ($($tt:tt)*) => {
        {
            println!("Macro input: {}", stringify!($($tt)*));
            $($tt)*
        }
    };
}

/// 错误处理宏
macro_rules! try_or_return {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Error in {}: {:?}", stringify!($expr), err);
                return;
            }
        }
    };
}

/// 日志宏系列
macro_rules! log_info {
    ($($arg:tt)*) => {
        println!("[INFO] {}", format!($($arg)*));
    };
}

macro_rules! log_warn {
    ($($arg:tt)*) => {
        eprintln!("[WARN] {}", format!($($arg)*));
    };
}

macro_rules! log_error {
    ($($arg:tt)*) => {
        eprintln!("[ERROR] {}", format!($($arg)*));
    };
}

/// 条件编译宏
macro_rules! feature_gate {
    ($feature:expr, $code:block) => {
        #[cfg(feature = $feature)]
        $code
    };
}

/// 调试和最佳实践演示
fn debugging_and_best_practices_demo() {
    println!("\n=== 调试和最佳实践演示 ===");
    
    // 调试宏
    debug_macro! {
        let x = 42;
        println!("x = {}", x);
    }
    
    // 日志宏
    log_info!("Application started");
    log_warn!("This is a warning message");
    log_error!("An error occurred: {}", "file not found");
    
    // 错误处理
    let parse_result: Result<i32, _> = "not_a_number".parse();
    // try_or_return!(parse_result); // 这会返回并打印错误
    
    println!("Continuing after error handling demo...");
}

// ============================================================================
// 第六部分：宏生态系统和常用库
// ============================================================================

/// 模拟 serde 风格的序列化宏
macro_rules! derive_serialize {
    ($struct_name:ident { $($field:ident),* }) => {
        impl $struct_name {
            fn serialize(&self) -> HashMap<String, String> {
                let mut map = HashMap::new();
                $(
                    map.insert(
                        stringify!($field).to_string(),
                        format!("{:?}", self.$field)
                    );
                )*
                map
            }
        }
    };
}

/// 模拟 clap 风格的 CLI 参数解析宏
macro_rules! cli_args {
    (
        $struct_name:ident {
            $($field:ident: $type:ty, $help:expr),* $(,)?
        }
    ) => {
        #[derive(Debug)]
        struct $struct_name {
            $($field: $type,)*
        }
        
        impl $struct_name {
            fn help() {
                println!("Usage: {} [OPTIONS]", env!("CARGO_PKG_NAME"));
                println!("Options:");
                $(
                    println!("  --{:<15} {}", 
                            stringify!($field).replace('_', "-"), 
                            $help);
                )*
            }
        }
    };
}

// 使用生态系统风格的宏
cli_args! {
    Args {
        input_file: String, "Input file path",
        output_file: String, "Output file path",
        verbose: bool, "Enable verbose output",
        threads: usize, "Number of threads to use",
    }
}

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

derive_serialize!(User { name, age, email });

/// 宏生态系统演示
fn macro_ecosystem_demo() {
    println!("\n=== 宏生态系统演示 ===");
    
    // CLI 参数帮助
    Args::help();
    
    // 序列化演示
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };
    
    let serialized = user.serialize();
    println!("\nSerialized user: {:#?}", serialized);
}

// ============================================================================
// 第七部分：性能和编译时优化
// ============================================================================

/// 编译时计算宏
macro_rules! const_eval {
    ($expr:expr) => {
        {
            const RESULT: i32 = $expr;
            RESULT
        }
    };
}

/// 零成本抽象宏
macro_rules! for_each {
    ($item:ident in $iter:expr => $body:block) => {
        {
            let mut iter = $iter.into_iter();
            while let Some($item) = iter.next() {
                $body
            }
        }
    };
}

/// 内联优化宏
macro_rules! inline_always {
    (fn $name:ident($($param:ident: $type:ty),*) -> $ret:ty $body:block) => {
        #[inline(always)]
        fn $name($($param: $type),*) -> $ret $body
    };
}

// 使用内联宏创建高性能函数
inline_always! {
    fn fast_add(a: i32, b: i32) -> i32 {
        a + b
    }
}

inline_always! {
    fn fast_multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}

/// 性能优化演示
fn performance_optimization_demo() {
    println!("\n=== 性能优化演示 ===");
    
    // 编译时计算
    let compile_time_result = const_eval!(2 + 3 * 4);
    println!("Compile-time calculation: {}", compile_time_result);
    
    // 零成本抽象
    let numbers = vec![1, 2, 3, 4, 5];
    print!("For each demo: ");
    for_each!(num in numbers => {
        print!("{} ", num * 2);
    });
    println!();
    
    // 内联函数
    let result1 = fast_add(10, 20);
    let result2 = fast_multiply(5, 6);
    println!("Inline functions: {} + {} = {}", result1, result2, result1 + result2);
}

// ============================================================================
// 测试用例
// ============================================================================

// 使用测试宏创建测试
test_case!(test_factorial_zero: factorial!(0) => 1);
test_case!(test_factorial_five: factorial!(5) => 120);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_person_creation() {
        let person = Person::new("Test".to_string(), 25, "test@example.com".to_string());
        assert_eq!(person.name, "Test");
        assert_eq!(person.age, 25);
    }
    
    #[test]
    fn test_database_config() {
        let config = DatabaseConfig::new().with_port(3306);
        assert_eq!(config.port, 3306);
        assert_eq!(config.host, "localhost");
    }
    
    #[test]
    fn test_internal_rules_macro() {
        let result = internal_rules!(1, 2, 3);
        assert_eq!(result, vec![1, 2, 3]);
    }
}

// ============================================================================
// 主函数：运行所有演示
// ============================================================================

fn main() {
    println!("🦀 Rust 宏编程全面分析");
    println!("=======================");
    println!("基于 https://course.rs/advance/macro.html 的深度分析");
    
    // 运行所有演示
    macro_vs_function_demo();
    declarative_macros_demo();
    advanced_macro_patterns_demo();
    real_world_examples_demo();
    debugging_and_best_practices_demo();
    macro_ecosystem_demo();
    performance_optimization_demo();
    
    println!("\n=== 总结 ===");
    println!("本项目全面展示了 Rust 宏编程的各个方面：");
    println!("✅ 宏的基本概念和与函数的区别");
    println!("✅ 声明式宏的语法和模式匹配");
    println!("✅ 高级宏模式和技巧");
    println!("✅ 实际应用案例和 DSL 构建");
    println!("✅ 调试技巧和最佳实践");
    println!("✅ 宏生态系统和常用模式");
    println!("✅ 性能优化和编译时计算");
    println!("\n🎯 通过这些示例，你应该对 Rust 宏编程有了全面深入的理解！");
}
