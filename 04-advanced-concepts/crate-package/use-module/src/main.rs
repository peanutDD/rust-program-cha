//! # Rust use 引入系统深度分析
//!
//! 本文档全面分析 Rust 中的 use 引入系统，涵盖基础概念、语法形式、
//! 引入模式、高级特性和实际应用案例。
//!
//! ## 主要内容
//! 1. use 基础概念
//! 2. use 语法形式
//! 3. use 引入模式
//! 4. glob 通配符

// 示例代码模块，允许未定义的 cfg 条件、未使用的导入和未使用的代码（用于演示 use 语句）
#![allow(unexpected_cfgs)]
#![allow(unused_imports)]
#![allow(dead_code)]
//! 5. 重命名和别名
//! 6. 嵌套引入
//! 7. 预导入模式
//! 8. 高级特性和实际应用

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::{self, Debug, Display};
use std::io::{self, BufRead, Read, Write};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    println!("Rust use 引入系统深度分析");
    println!("=================================\n");

    // 1. use 基础概念
    demonstrate_use_basics();

    // 2. use 语法形式
    demonstrate_use_syntax();

    // 3. use 引入模式
    demonstrate_use_patterns();

    // 4. glob 通配符
    demonstrate_glob_imports();

    // 5. 重命名和别名
    demonstrate_renaming_aliases();

    // 6. 嵌套引入
    demonstrate_nested_imports();

    // 7. 预导入模式
    demonstrate_prelude_patterns();

    // 8. 高级特性和实际应用
    demonstrate_advanced_features();

    println!("\nuse 引入系统分析完成！");
}

// ============================================================================
// 1. use 基础概念
// ============================================================================

/// 演示 use 基础概念
fn demonstrate_use_basics() {
    println!("1. use 基础概念:");
    println!("use 关键字用于将模块、类型、函数等引入当前作用域");
    println!();

    // 演示基本引入概念
    demonstrate_basic_import_concept();

    // 演示路径解析
    demonstrate_path_resolution();

    // 演示作用域影响
    demonstrate_scope_impact();

    println!();
}

/// 演示基本引入概念
fn demonstrate_basic_import_concept() {
    println!("\n基本引入概念:");

    // 定义示例模块
    mod math_utils {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }

        pub mod advanced {
            pub fn power(base: i32, exp: u32) -> i32 {
                base.pow(exp)
            }
        }
    }

    // 不使用 use 的情况
    {
        println!("不使用 use - 需要完整路径:");
        let result1 = math_utils::add(5, 3);
        let result2 = math_utils::advanced::power(2, 3);
        println!("5 + 3 = {}", result1);
        println!("2^3 = {}", result2);
    }

    // 使用 use 的情况
    {
        use math_utils::add;
        use math_utils::advanced::power;

        println!("\n使用 use - 简化调用:");
        let result1 = add(5, 3);
        let result2 = power(2, 3);
        println!("5 + 3 = {}", result1);
        println!("2^3 = {}", result2);
    }

    println!("use 关键字将远程路径引入本地作用域，简化代码");
}

/// 演示路径解析规则
fn demonstrate_path_resolution() {
    println!("\n路径解析规则:");

    mod network {
        pub mod tcp {
            pub fn connect() {
                println!("TCP 连接建立");
            }
        }

        pub mod udp {
            pub fn send() {
                println!("UDP 数据发送");
            }
        }
    }

    // 绝对路径 - 从 crate 根开始
    {
        use network::tcp::connect;
        println!("绝对路径引入:");
        connect();
    }

    // 相对路径 - 从当前模块开始
    {
        use network::udp::send;
        println!("相对路径引入:");
        send();
    }

    println!("路径解析支持绝对路径(crate::)和相对路径(self::)");
}

/// 演示作用域影响
fn demonstrate_scope_impact() {
    println!("\n作用域影响:");

    mod data_structures {
        pub struct Stack<T> {
            items: Vec<T>,
        }

        impl<T> Stack<T> {
            pub fn new() -> Self {
                Stack { items: Vec::new() }
            }

            pub fn push(&mut self, item: T) {
                self.items.push(item);
            }

            pub fn pop(&mut self) -> Option<T> {
                self.items.pop()
            }
        }
    }

    // use 在不同作用域中的效果
    {
        use data_structures::Stack;

        println!("在块作用域中使用 use:");
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        println!("栈顶元素: {:?}", stack.pop());
    }

    // 这里 Stack 不再可用
    // let stack = Stack::new(); // 编译错误

    println!("use 引入的名称只在其声明的作用域内有效");
}

// ============================================================================
// 2. use 语法形式
// ============================================================================

/// 演示 use 语法形式
fn demonstrate_use_syntax() {
    println!("2. use 语法形式:");
    println!("use 支持多种语法形式来引入不同类型的项目");
    println!();

    // 演示绝对路径语法
    demonstrate_absolute_path_syntax();

    // 演示相对路径语法
    demonstrate_relative_path_syntax();

    // 演示关键字语法
    demonstrate_keyword_syntax();

    println!();
}

/// 演示绝对路径语法
fn demonstrate_absolute_path_syntax() {
    println!("\n绝对路径语法:");

    // 标准库绝对路径
    use std::collections::HashMap as StdHashMap;
    use std::sync::Arc as StdArc;
    use std::thread::spawn as StdSpawn;

    println!("标准库绝对路径引入:");
    let mut map: StdHashMap<String, i32> = StdHashMap::new();
    map.insert("key1".to_string(), 42);
    println!("HashMap 创建成功，包含 {} 个元素", map.len());

    let data = StdArc::new(vec![1, 2, 3]);
    println!("Arc 包装数据: {:?}", data);

    // crate 根路径
    mod local_module {
        pub fn local_function() {
            println!("本地模块函数");
        }
    }

    use local_module::local_function;
    println!("\ncrate 根路径引入:");
    local_function();

    println!("绝对路径从 std:: 或 crate:: 开始，提供明确的引用");
}

/// 演示相对路径语法
fn demonstrate_relative_path_syntax() {
    println!("\n相对路径语法:");

    mod parent {
        pub mod child {
            pub fn child_function() {
                println!("子模块函数");
            }
        }

        pub fn parent_function() {
            // 使用 self 引用当前模块
            use child::child_function;

            println!("父模块函数调用:");
            child_function();
        }
    }

    mod sibling {
        pub fn sibling_function() {
            // 使用 super 引用父模块
            // use super::parent::parent_function; // 注释掉，因为没有父模块

            println!("兄弟模块函数调用:");
            // super::parent::parent_function(); // 注释掉，因为没有父模块
        }
    }

    sibling::sibling_function();

    println!("相对路径使用 self:: 和 super:: 进行模块导航");
}

/// 演示关键字语法
fn demonstrate_keyword_syntax() {
    println!("\n关键字语法:");

    mod demo_module {
        pub struct DemoStruct;

        impl DemoStruct {
            pub fn method(&self) {
                println!("演示结构体方法");
            }
        }

        pub fn demo_function() {
            println!("演示模块函数");
        }
    }

    // self 关键字 - 当前模块
    {
        use demo_module::DemoStruct;
        let demo = DemoStruct;
        demo.method();
        println!("self:: 引用当前模块内容");
    }

    // crate 关键字 - crate 根
    {
        use demo_module::demo_function;
        demo_function();
        println!("crate:: 引用 crate 根内容");
    }

    println!("关键字提供了灵活的模块路径导航机制");
}

// ============================================================================
// 3. use 引入模式
// ============================================================================

/// 演示 use 引入模式
fn demonstrate_use_patterns() {
    println!("3. use 引入模式:");
    println!("use 支持多种引入模式来满足不同的使用需求");
    println!();

    // 演示单项引入
    demonstrate_single_imports();

    // 演示批量引入
    demonstrate_batch_imports();

    // 演示选择性引入
    demonstrate_selective_imports();

    println!();
}

/// 演示单项引入
fn demonstrate_single_imports() {
    println!("\n单项引入:");

    mod utilities {
        pub fn format_string(s: &str) -> String {
            format!("[{}]", s)
        }

        pub fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
            s.parse()
        }

        pub struct Logger {
            prefix: String,
        }

        impl Logger {
            pub fn new(prefix: &str) -> Self {
                Logger {
                    prefix: prefix.to_string(),
                }
            }

            pub fn log(&self, message: &str) {
                println!("{}: {}", self.prefix, message);
            }
        }
    }

    // 单独引入函数
    use utilities::format_string;
    let formatted = format_string("Hello");
    println!("格式化字符串: {}", formatted);

    // 单独引入结构体
    use utilities::Logger;
    let logger = Logger::new("INFO");
    logger.log("单项引入演示");

    println!("单项引入适合只需要少数几个项目的场景");
}

/// 演示批量引入
fn demonstrate_batch_imports() {
    println!("\n批量引入:");

    mod math_ops {
        pub fn add(a: f64, b: f64) -> f64 {
            a + b
        }
        pub fn subtract(a: f64, b: f64) -> f64 {
            a - b
        }
        pub fn multiply(a: f64, b: f64) -> f64 {
            a * b
        }
        pub fn divide(a: f64, b: f64) -> f64 {
            a / b
        }

        pub const PI: f64 = 3.14159265359;
        pub const E: f64 = 2.71828182846;
    }

    // 批量引入函数
    use math_ops::{add, divide, multiply, subtract};

    println!("数学运算演示:");
    let a = 10.0;
    let b = 3.0;

    println!("{} + {} = {}", a, b, add(a, b));
    println!("{} - {} = {}", a, b, subtract(a, b));
    println!("{} * {} = {}", a, b, multiply(a, b));
    println!("{} / {} = {:.2}", a, b, divide(a, b));

    // 批量引入常量
    use math_ops::{E, PI};
    println!("数学常量: π = {:.5}, e = {:.5}", PI, E);

    println!("批量引入适合需要多个相关项目的场景");
}

/// 演示选择性引入
fn demonstrate_selective_imports() {
    println!("\n选择性引入:");

    mod web_framework {
        pub mod http {
            pub struct Request {
                pub method: String,
                pub path: String,
            }

            pub struct Response {
                pub status: u16,
                pub body: String,
            }

            pub fn get(path: &str) -> Request {
                Request {
                    method: "GET".to_string(),
                    path: path.to_string(),
                }
            }

            pub fn post(path: &str) -> Request {
                Request {
                    method: "POST".to_string(),
                    path: path.to_string(),
                }
            }
        }

        pub mod json {
            pub fn serialize<T>(_data: &T) -> String {
                "{\"serialized\": true}".to_string()
            }

            pub fn deserialize<T>(_json: &str) -> Result<T, String> {
                Err("演示用途".to_string())
            }
        }
    }

    // 选择性引入 HTTP 相关
    use web_framework::http::{Request, Response, get};

    let request = get("/api/users");
    println!("HTTP 请求: {} {}", request.method, request.path);

    let response = Response {
        status: 200,
        body: "Success".to_string(),
    };
    println!("HTTP 响应: {} - {}", response.status, response.body);

    // 选择性引入 JSON 相关
    use web_framework::json::serialize;

    let data = "example";
    let json = serialize(&data);
    println!("JSON 序列化: {}", json);

    println!("选择性引入避免命名空间污染，提高代码清晰度");
}

// ============================================================================
// 4. glob 通配符
// ============================================================================

/// 演示 glob 通配符
fn demonstrate_glob_imports() {
    println!("4. glob 通配符:");
    println!("* 通配符可以引入模块中的所有公共项目");
    println!();

    // 演示基本 glob 使用
    demonstrate_basic_glob();

    // 演示 glob 的优缺点
    demonstrate_glob_pros_cons();

    // 演示 glob 最佳实践
    demonstrate_glob_best_practices();

    println!();
}

/// 演示基本 glob 使用
fn demonstrate_basic_glob() {
    println!("\n基本 glob 使用:");

    mod color_constants {
        pub const RED: &str = "#FF0000";
        pub const GREEN: &str = "#00FF00";
        pub const BLUE: &str = "#0000FF";
        pub const WHITE: &str = "#FFFFFF";
        pub const BLACK: &str = "#000000";

        // 私有常量不会被 glob 引入
        const INTERNAL_COLOR: &str = "#INTERNAL";
    }

    // 使用 glob 引入所有公共项目
    {
        use color_constants::*;

        println!("颜色常量:");
        println!("红色: {}", RED);
        println!("绿色: {}", GREEN);
        println!("蓝色: {}", BLUE);
        println!("白色: {}", WHITE);
        println!("黑色: {}", BLACK);

        // INTERNAL_COLOR 不可用，因为它是私有的
    }

    println!("glob 引入所有公共项目，简化大量引入的场景");
}

/// 演示 glob 的优缺点
fn demonstrate_glob_pros_cons() {
    println!("\n glob 的优缺点:");

    mod large_api {
        pub fn function_a() {
            println!("函数 A");
        }
        pub fn function_b() {
            println!("函数 B");
        }
        pub fn function_c() {
            println!("函数 C");
        }
        pub fn function_d() {
            println!("函数 D");
        }
        pub fn function_e() {
            println!("函数 E");
        }

        pub struct TypeA;
        pub struct TypeB;
        pub struct TypeC;

        pub const CONST_X: i32 = 1;
        pub const CONST_Y: i32 = 2;
        pub const CONST_Z: i32 = 3;
    }

    // 优点：简洁的引入
    {
        use large_api::*;

        println!("优点 - 简洁引入:");
        function_a();
        function_b();

        let _type_a = TypeA;
        let _type_b = TypeB;

        println!("常量: {}, {}, {}", CONST_X, CONST_Y, CONST_Z);
    }

    println!("\n缺点分析:");
    println!("1. 命名空间污染 - 引入大量可能不需要的名称");
    println!("2. 命名冲突风险 - 可能与本地定义冲突");
    println!("3. 代码可读性 - 不清楚哪些名称来自外部模块");
    println!("4. IDE 支持 - 自动补全和重构可能受影响");
}

/// 演示 glob 最佳实践
fn demonstrate_glob_best_practices() {
    println!("\n glob 最佳实践:");

    // 适合使用 glob 的场景
    mod prelude {
        pub use std::collections::HashMap;
        pub use std::collections::HashSet;
        pub use std::sync::{Arc, Mutex};
        pub use std::thread;

        pub trait CommonTrait {
            fn common_method(&self);
        }

        pub struct CommonStruct;

        impl CommonTrait for CommonStruct {
            fn common_method(&self) {
                println!("通用方法实现");
            }
        }
    }

    // 在 prelude 模式中使用 glob
    {
        use prelude::*;

        println!("prelude 模式中的 glob 使用:");
        let mut map = HashMap::new();
        map.insert("key", "value");

        let common = CommonStruct;
        common.common_method();

        println!("prelude 提供了精心设计的公共 API");
    }

    println!("\nglob 最佳实践:");
    println!("1. 主要用于 prelude 模块");
    println!("2. 用于测试模块引入测试工具");
    println!("3. 避免在生产代码中过度使用");
    println!("4. 确保引入的模块设计良好，避免命名冲突");
}

// ============================================================================
// 5. 重命名和别名
// ============================================================================

/// 演示重命名和别名
fn demonstrate_renaming_aliases() {
    println!("5. 重命名和别名:");
    println!("as 关键字用于重命名引入的项目，解决命名冲突");
    println!();

    // 演示基本重命名
    demonstrate_basic_renaming();

    // 演示命名冲突解决
    demonstrate_conflict_resolution();

    // 演示 API 简化
    demonstrate_api_simplification();

    println!();
}

/// 演示基本重命名
fn demonstrate_basic_renaming() {
    println!("\n基本重命名:");

    mod graphics {
        pub struct Point {
            pub x: f64,
            pub y: f64,
        }

        impl Point {
            pub fn new(x: f64, y: f64) -> Self {
                Point { x, y }
            }

            pub fn distance_from_origin(&self) -> f64 {
                (self.x * self.x + self.y * self.y).sqrt()
            }
        }
    }

    // 使用 as 重命名
    use graphics::Point as GraphicsPoint;

    let point = GraphicsPoint::new(3.0, 4.0);
    println!("图形点: ({}, {})", point.x, point.y);
    println!("距离原点: {:.2}", point.distance_from_origin());

    // 重命名函数
    use std::collections::HashMap as Map;

    let mut data: Map<String, i32> = Map::new();
    data.insert("count".to_string(), 42);
    println!("数据映射: {:?}", data);

    println!("as 关键字提供了灵活的重命名机制");
}

/// 演示命名冲突解决
fn demonstrate_conflict_resolution() {
    println!("\n命名冲突解决:");

    mod database {
        pub struct Connection {
            pub host: String,
        }

        impl Connection {
            pub fn new(host: &str) -> Self {
                Connection {
                    host: host.to_string(),
                }
            }

            pub fn execute(&self, query: &str) {
                println!("数据库 {} 执行: {}", self.host, query);
            }
        }
    }

    mod network {
        pub struct Connection {
            pub address: String,
        }

        impl Connection {
            pub fn new(address: &str) -> Self {
                Connection {
                    address: address.to_string(),
                }
            }

            pub fn send(&self, data: &str) {
                println!("网络 {} 发送: {}", self.address, data);
            }
        }
    }

    // 使用别名解决命名冲突
    use database::Connection as DbConnection;
    use network::Connection as NetConnection;

    let db_conn = DbConnection::new("localhost:5432");
    db_conn.execute("SELECT * FROM users");

    let net_conn = NetConnection::new("192.168.1.1:8080");
    net_conn.send("Hello, Server!");

    println!("别名成功解决了同名类型的冲突");
}

/// 演示 API 简化
fn demonstrate_api_simplification() {
    println!("\n API 简化:");

    mod very_long_module_name {
        pub mod deeply_nested_submodule {
            pub mod specific_functionality {
                pub struct VeryLongStructureName {
                    pub value: i32,
                }

                impl VeryLongStructureName {
                    pub fn new(value: i32) -> Self {
                        VeryLongStructureName { value }
                    }

                    pub fn very_long_method_name(&self) -> i32 {
                        self.value * 2
                    }
                }

                pub fn very_long_function_name(x: i32) -> i32 {
                    x + 1
                }
            }
        }
    }

    // 使用别名简化长名称
    use very_long_module_name::deeply_nested_submodule::specific_functionality::{
        VeryLongStructureName as SimpleStruct, very_long_function_name as simple_func,
    };

    let obj = SimpleStruct::new(10);
    println!("简化结构体: value = {}", obj.value);
    println!("方法调用: {}", obj.very_long_method_name());
    println!("简化函数: {}", simple_func(5));

    // 模块别名
    use very_long_module_name::deeply_nested_submodule as nested;

    let obj2 = nested::specific_functionality::VeryLongStructureName::new(20);
    println!("模块别名使用: value = {}", obj2.value);

    println!("别名显著提高了代码的可读性和简洁性");
}

// ============================================================================
// 6. 嵌套引入
// ============================================================================

/// 演示嵌套引入
fn demonstrate_nested_imports() {
    println!("6. 嵌套引入:");
    println!("花括号语法支持层次化的批量引入");
    println!();

    // 演示基本嵌套引入
    demonstrate_basic_nesting();

    // 演示复杂嵌套引入
    demonstrate_complex_nesting();

    // 演示嵌套引入优化
    demonstrate_nesting_optimization();

    println!();
}

/// 演示基本嵌套引入
fn demonstrate_basic_nesting() {
    println!("\n基本嵌套引入:");

    // 标准库嵌套引入
    use std::{
        collections::{BTreeMap, HashMap, HashSet},
        sync::{Arc, Mutex, RwLock},
        thread,
    };

    println!("标准库嵌套引入演示:");

    let mut map = HashMap::new();
    map.insert("key1", "value1");

    let mut set = HashSet::new();
    set.insert(42);

    let tree_map: BTreeMap<i32, String> = BTreeMap::new();

    let shared_data = Arc::new(Mutex::new(vec![1, 2, 3]));

    println!("HashMap: {:?}", map);
    println!("HashSet: {:?}", set);
    println!("BTreeMap 已创建");
    println!("Arc<Mutex<Vec>> 已创建");

    println!("嵌套引入组织相关的导入项");
}

/// 演示复杂嵌套引入
fn demonstrate_complex_nesting() {
    println!("\n复杂嵌套引入:");

    mod web_server {
        pub mod http {
            pub struct Request;
            pub struct Response;

            pub mod methods {
                pub const GET: &str = "GET";
                pub const POST: &str = "POST";
                pub const PUT: &str = "PUT";
                pub const DELETE: &str = "DELETE";
            }

            pub mod status {
                pub const OK: u16 = 200;
                pub const NOT_FOUND: u16 = 404;
                pub const INTERNAL_ERROR: u16 = 500;
            }
        }

        pub mod middleware {
            pub fn cors() {
                println!("CORS 中间件");
            }
            pub fn auth() {
                println!("认证中间件");
            }
            pub fn logging() {
                println!("日志中间件");
            }
        }

        pub mod database {
            pub struct Pool;
            pub struct Transaction;

            pub mod models {
                pub struct User;
                pub struct Post;
                pub struct Comment;
            }
        }
    }

    // 复杂嵌套引入
    use web_server::{
        database::{
            Pool,
            models::{Post, User},
        },
        http::{
            Request, Response,
            methods::{GET, POST},
            status::{NOT_FOUND, OK},
        },
        middleware::{auth, cors, logging},
    };

    println!("Web 服务器组件引入:");
    println!("HTTP 方法: {}, {}", GET, POST);
    println!("状态码: {}, {}", OK, NOT_FOUND);

    cors();
    auth();
    logging();

    println!("数据库池和模型已引入");

    println!("复杂嵌套引入支持深层次的模块结构");
}

/// 演示嵌套引入优化
fn demonstrate_nesting_optimization() {
    println!("\n嵌套引入优化:");

    mod api_client {
        pub mod v1 {
            pub struct Client;
            pub struct Config;

            pub mod endpoints {
                pub fn users() -> &'static str {
                    "/api/v1/users"
                }
                pub fn posts() -> &'static str {
                    "/api/v1/posts"
                }
            }
        }

        pub mod v2 {
            pub struct Client;
            pub struct Config;

            pub mod endpoints {
                pub fn users() -> &'static str {
                    "/api/v2/users"
                }
                pub fn posts() -> &'static str {
                    "/api/v2/posts"
                }
            }
        }
    }

    // 优化前：多个单独的 use 语句
    println!("优化前的引入方式:");
    println!("use api_client::v1::Client;");
    println!("use api_client::v1::Config;");
    println!("use api_client::v1::endpoints::users;");
    println!("use api_client::v1::endpoints::posts;");

    // 优化后：嵌套引入
    use api_client::v1::{
        Client as V1Client, Config as V1Config,
        endpoints::{posts as v1_posts, users as v1_users},
    };

    use api_client::v2::{Client as V2Client, endpoints::users as v2_users};

    println!("\n优化后的嵌套引入:");
    println!("V1 用户端点: {}", v1_users());
    println!("V1 文章端点: {}", v1_posts());
    println!("V2 用户端点: {}", v2_users());

    println!("\n嵌套引入优势:");
    println!("1. 减少重复的模块路径");
    println!("2. 逻辑分组相关的引入");
    println!("3. 提高代码组织性和可读性");
    println!("4. 便于维护和重构");
}

// ============================================================================
// 7. 预导入模式
// ============================================================================

/// 演示预导入模式
fn demonstrate_prelude_patterns() {
    println!("7. 预导入模式:");
    println!("prelude 模式提供常用项目的便捷引入");
    println!();

    // 演示标准库 prelude
    demonstrate_std_prelude();

    // 演示自定义 prelude
    demonstrate_custom_prelude();

    // 演示库设计 prelude
    demonstrate_library_prelude();

    println!();
}

/// 演示标准库 prelude
fn demonstrate_std_prelude() {
    println!("\n标准库 prelude:");

    // 标准库自动引入的项目（无需显式 use）
    println!("自动可用的标准库项目:");

    // Vec - 来自 std::vec::Vec
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Vec: {:?}", vec);

    // String - 来自 std::string::String
    let s = String::from("Hello, World!");
    println!("String: {}", s);

    // Option 和 Result - 来自 std::option 和 std::result
    let opt: Option<i32> = Some(42);
    let res: Result<i32, &str> = Ok(100);
    println!("Option: {:?}, Result: {:?}", opt, res);

    // 迭代器 trait
    let numbers: Vec<i32> = (1..5).collect();
    println!("迭代器收集: {:?}", numbers);

    println!("标准库 prelude 提供了最常用的类型和 trait");
}

/// 演示自定义 prelude
fn demonstrate_custom_prelude() {
    println!("\n自定义 prelude:");

    mod my_library {
        // 核心类型和函数
        pub struct MyStruct {
            pub value: i32,
        }

        impl MyStruct {
            pub fn new(value: i32) -> Self {
                MyStruct { value }
            }
        }

        pub trait MyTrait {
            fn my_method(&self);
        }

        impl MyTrait for MyStruct {
            fn my_method(&self) {
                println!("MyStruct 方法，值: {}", self.value);
            }
        }

        pub fn my_function() {
            println!("我的库函数");
        }

        // prelude 模块
        pub mod prelude {
            pub use super::MyStruct;
            pub use super::MyTrait;
            pub use super::my_function;

            // 重导出常用的标准库项目
            pub use std::collections::{HashMap, HashSet};
            pub use std::sync::{Arc, Mutex};
        }
    }

    // 使用自定义 prelude
    {
        use my_library::prelude::*;

        println!("使用自定义 prelude:");

        let obj = MyStruct::new(42);
        obj.my_method();

        my_function();

        let mut map = HashMap::new();
        map.insert("key", "value");
        println!("HashMap 从 prelude: {:?}", map);
    }

    println!("自定义 prelude 简化了库的使用");
}

/// 演示库设计 prelude
fn demonstrate_library_prelude() {
    println!("\n库设计 prelude:");

    mod game_engine {
        // 核心组件
        pub mod core {
            pub struct Engine;
            pub struct Scene;
            pub struct Entity;
        }

        // 渲染系统
        pub mod render {
            pub struct Renderer;
            pub struct Camera;
            pub struct Mesh;
        }

        // 物理系统
        pub mod physics {
            pub struct World;
            pub struct RigidBody;
            pub struct Collider;
        }

        // 输入系统
        pub mod input {
            pub struct InputManager;
            pub enum Key {
                W,
                A,
                S,
                D,
                Space,
            }
        }

        // 数学库
        pub mod math {
            pub struct Vector3 {
                pub x: f32,
                pub y: f32,
                pub z: f32,
            }
            pub struct Matrix4;
        }

        // 游戏引擎 prelude
        pub mod prelude {
            // 核心类型
            pub use super::core::{Engine, Entity, Scene};

            // 渲染相关
            pub use super::render::{Camera, Renderer};

            // 物理相关
            pub use super::physics::{RigidBody, World};

            // 输入相关
            pub use super::input::{InputManager, Key};

            // 数学类型
            pub use super::math::{Matrix4, Vector3};

            // 常用 trait（如果有的话）
            // pub use super::traits::*;
        }
    }

    // 游戏开发者使用
    {
        use game_engine::prelude::*;

        println!("游戏引擎 prelude 使用:");

        // 所有核心类型都可直接使用
        let _engine = Engine;
        let _scene = Scene;
        let _renderer = Renderer;
        let _world = World;
        let _input = InputManager;

        let position = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        println!("实体位置: ({}, {}, {})", position.x, position.y, position.z);

        println!("游戏引擎的所有核心组件都可直接使用");
    }

    println!("\nprelude 设计原则:");
    println!("1. 包含最常用的 80% 功能");
    println!("2. 避免命名冲突");
    println!("3. 保持简洁和一致性");
    println!("4. 提供良好的开发体验");
}

// ============================================================================
// 8. 高级特性和实际应用
// ============================================================================

/// 演示高级特性和实际应用
fn demonstrate_advanced_features() {
    println!("8. 高级特性和实际应用:");
    println!("use 的高级用法和实际项目中的应用模式");
    println!();

    // 演示条件引入
    demonstrate_conditional_imports();

    // 演示宏引入
    demonstrate_macro_imports();

    // 演示外部 crate 引入
    demonstrate_external_crate_imports();

    // 演示实际项目模式
    demonstrate_real_world_patterns();

    println!();
}

/// 演示条件引入
fn demonstrate_conditional_imports() {
    println!("\n条件引入:");

    mod platform_specific {
        #[cfg(unix)]
        pub mod unix {
            pub fn get_system_info() -> &'static str {
                "Unix 系统信息"
            }
        }

        #[cfg(windows)]
        pub mod windows {
            pub fn get_system_info() -> &'static str {
                "Windows 系统信息"
            }
        }

        #[cfg(feature = "advanced")]
        pub mod advanced {
            pub fn advanced_feature() {
                println!("高级功能已启用");
            }
        }

        #[cfg(debug_assertions)]
        pub mod debug {
            pub fn debug_info() {
                println!("调试模式信息");
            }
        }
    }

    // 条件引入
    #[cfg(unix)]
    use platform_specific::unix::get_system_info;

    #[cfg(windows)]
    use platform_specific::windows::get_system_info;

    // 示例代码：演示条件引入，feature 未在 Cargo.toml 中定义
    #[allow(unexpected_cfgs)]
    #[cfg(feature = "advanced")]
    use platform_specific::advanced::advanced_feature;

    #[cfg(debug_assertions)]
    use platform_specific::debug::debug_info;

    println!("条件引入演示:");

    // 根据平台使用不同的实现
    #[cfg(any(unix, windows))]
    {
        println!("系统信息: {}", get_system_info());
    }

    // 功能特性引入（示例代码）
    #[allow(unexpected_cfgs)]
    #[cfg(feature = "advanced")]
    {
        advanced_feature();
    }

    // 调试模式引入
    #[cfg(debug_assertions)]
    {
        debug_info();
    }

    println!("条件引入支持平台特定和功能特定的代码组织");
}

/// 演示宏引入
fn demonstrate_macro_imports() {
    println!("\n宏引入:");

    mod macro_definitions {
        // 定义一些宏
        macro_rules! debug_print {
            ($($arg:tt)*) => {
                #[cfg(debug_assertions)]
                println!("[DEBUG] {}", format!($($arg)*));
            };
        }

        macro_rules! create_struct {
            ($name:ident, $field:ident: $type:ty) => {
                pub struct $name {
                    pub $field: $type,
                }

                impl $name {
                    pub fn new($field: $type) -> Self {
                        $name { $field }
                    }
                }
            };
        }

        // 导出宏
        pub(crate) use create_struct;
        pub(crate) use debug_print;
    }

    // 引入宏
    use macro_definitions::{create_struct, debug_print};

    println!("宏引入演示:");

    // 使用引入的宏
    debug_print!("这是一个调试消息: {}", 42);

    // 使用宏创建结构体
    create_struct!(Person, name: String);

    let person = Person::new("Alice".to_string());
    println!("创建的结构体: {}", person.name);

    println!("宏引入允许在不同模块间共享宏定义");
}

/// 演示外部 crate 引入
fn demonstrate_external_crate_imports() {
    println!("\n外部 crate 引入:");

    // 标准库 crate 引入
    use std::{
        collections::{BTreeMap, HashMap},
        sync::{Arc, Mutex},
        thread,
        time::{Duration, Instant},
    };

    println!("标准库 crate 引入演示:");

    let start = Instant::now();

    let data = Arc::new(Mutex::new(HashMap::new()));
    {
        let mut map = data.lock().unwrap();
        map.insert("key1", "value1");
        map.insert("key2", "value2");
    }

    let elapsed = start.elapsed();
    println!("操作耗时: {:?}", elapsed);

    // 模拟外部 crate 的引入模式
    println!("\n外部 crate 引入模式:");
    println!("// Cargo.toml 中的依赖");
    println!("// [dependencies]");
    println!("// serde = {{ version = \"1.0\", features = [\"derive\"] }}");
    println!("// tokio = {{ version = \"1.0\", features = [\"full\"] }}");
    println!("// reqwest = \"0.11\"");
    println!();
    println!("// 代码中的引入");
    println!("// use serde::{{Serialize, Deserialize}};");
    println!("// use tokio::{{fs, net, time}};");
    println!("// use reqwest::Client;");

    println!("\n外部 crate 引入最佳实践:");
    println!("1. 使用具体的版本号");
    println!("2. 只启用需要的 features");
    println!("3. 组织相关的引入");
    println!("4. 使用 prelude 简化引入");
}

/// 演示实际项目模式
fn demonstrate_real_world_patterns() {
    println!("\n实际项目模式:");

    // Web 应用项目结构
    mod web_app {
        // 模型层
        pub mod models {
            pub struct User {
                pub id: u32,
                pub name: String,
            }
            pub struct Post {
                pub id: u32,
                pub title: String,
            }
        }

        // 服务层
        pub mod services {
            use super::models::*;

            pub struct UserService;
            impl UserService {
                pub fn create_user(&self, name: &str) -> User {
                    User {
                        id: 1,
                        name: name.to_string(),
                    }
                }
            }

            pub struct PostService;
            impl PostService {
                pub fn create_post(&self, title: &str) -> Post {
                    Post {
                        id: 1,
                        title: title.to_string(),
                    }
                }
            }
        }

        // 控制器层
        pub mod controllers {
            use super::models::*;
            use super::services::*;

            pub struct UserController {
                user_service: UserService,
            }

            impl UserController {
                pub fn new() -> Self {
                    UserController {
                        user_service: UserService,
                    }
                }

                pub fn handle_create_user(&self, name: &str) -> User {
                    self.user_service.create_user(name)
                }
            }
        }

        // 应用 prelude
        pub mod prelude {
            pub use super::controllers::*;
            pub use super::models::*;
            pub use super::services::*;

            // 常用的标准库和外部 crate
            pub use std::collections::HashMap;
            pub use std::sync::Arc;
        }
    }

    // 使用应用 prelude
    {
        use web_app::prelude::*;

        println!("Web 应用项目模式:");

        let controller = UserController::new();
        let user = controller.handle_create_user("Alice");
        println!("创建用户: {} (ID: {})", user.name, user.id);

        let mut cache = HashMap::new();
        cache.insert(user.id, user.name.clone());
        println!("用户缓存: {:?}", cache);
    }

    println!("\n项目组织模式:");

    // 库项目结构
    mod library_project {
        // 公共 API
        pub mod api {
            pub use super::core::*;
            pub use super::utils::*;
        }

        // 核心功能
        mod core {
            pub struct LibraryCore;
            impl LibraryCore {
                pub fn process(&self) {
                    println!("库核心处理");
                }
            }
        }

        // 工具函数
        mod utils {
            pub fn helper_function() {
                println!("辅助函数");
            }
        }

        // 内部模块（不导出）
        mod internal {
            pub fn internal_function() {
                println!("内部函数");
            }
        }
    }

    // 库的使用者
    {
        use library_project::api::*;

        println!("\n库项目使用:");
        let core = LibraryCore;
        core.process();
        helper_function();

        // internal_function(); // 不可访问
    }

    println!("\n实际项目 use 模式总结:");
    println!("1. 分层架构中的模块引入");
    println!("2. prelude 模式简化 API");
    println!("3. 公共 API 和内部实现分离");
    println!("4. 依赖注入和服务定位");
    println!("5. 测试模块的引入组织");
}
