//! # Rust 模块系统深度分析
//!
//! 本文档全面分析 Rust 的模块系统，包括模块定义、组织方式、可见性控制、
//! use 引入系统、路径解析、预导入机制、高级特性和实际应用案例。
//!
//! 基于 https://course.rs/basic/crate-module/module.html 的深度扩展分析

// 示例代码模块，允许未定义的 cfg 条件、未使用的导入和代码（用于演示条件编译）
#![allow(unexpected_cfgs)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::{self, Display};
use std::io::{self, Write};

/// 主函数 - 演示模块系统的各个方面
fn main() {
    println!("=== Rust 模块系统深度分析 ===");
    println!();

    // 1. 模块基础概念
    demonstrate_module_basics();

    // 2. 模块组织方式
    demonstrate_module_organization();

    // 3. 可见性控制
    demonstrate_visibility_control();

    // 4. use 引入系统
    demonstrate_use_imports();

    // 5. 模块路径系统
    demonstrate_module_paths();

    // 6. 预导入和标准库
    demonstrate_prelude_and_std();

    // 7. 高级特性
    demonstrate_advanced_features();

    // 8. 实际应用案例
    demonstrate_real_world_examples();

    println!("\n=== 模块系统分析完成 ===");
}

// ============================================================================
// 1. 模块基础概念
// ============================================================================

/// 演示模块基础概念
fn demonstrate_module_basics() {
    println!("1. 模块基础概念:");
    println!("模块是 Rust 中组织代码的基本单位，提供命名空间和封装机制。");
    println!();

    // 演示内联模块定义
    mod basic_example {
        pub fn public_function() {
            println!("这是一个公共函数");
            private_function();
        }

        fn private_function() {
            println!("这是一个私有函数");
        }

        pub mod nested {
            pub fn nested_function() {
                println!("这是嵌套模块中的函数");
            }
        }
    }

    // 使用模块中的函数
    basic_example::public_function();
    basic_example::nested::nested_function();

    // 演示模块树结构
    demonstrate_module_tree();

    // 演示命名空间
    demonstrate_namespaces();

    println!();
}

/// 演示模块树结构
fn demonstrate_module_tree() {
    println!("\n模块树结构:");

    mod library {
        pub mod frontend {
            pub mod components {
                pub fn button() {
                    println!("Button 组件");
                }
                pub fn input() {
                    println!("Input 组件");
                }
            }

            pub mod pages {
                pub fn home() {
                    println!("Home 页面");
                }
                pub fn about() {
                    println!("About 页面");
                }
            }
        }

        pub mod backend {
            pub mod api {
                pub fn users() {
                    println!("Users API");
                }
                pub fn posts() {
                    println!("Posts API");
                }
            }

            pub mod database {
                pub fn connect() {
                    println!("数据库连接");
                }
                pub fn migrate() {
                    println!("数据库迁移");
                }
            }
        }
    }

    // 演示模块树的使用
    library::frontend::components::button();
    library::frontend::pages::home();
    library::backend::api::users();
    library::backend::database::connect();

    println!("模块树提供了清晰的代码组织结构");
}

/// 演示命名空间
fn demonstrate_namespaces() {
    println!("\n命名空间演示:");

    mod math {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
    }

    mod string_utils {
        pub fn add(a: &str, b: &str) -> String {
            format!("{}{}", a, b)
        }
    }

    // 同名函数在不同命名空间中不冲突
    let num_result = math::add(5, 3);
    let str_result = string_utils::add("Hello, ", "World!");

    println!("数学加法: {}", num_result);
    println!("字符串拼接: {}", str_result);
    println!("命名空间避免了函数名冲突");
}

// ============================================================================
// 2. 模块组织方式
// ============================================================================

/// 演示模块组织方式
fn demonstrate_module_organization() {
    println!("2. 模块组织方式:");
    println!("Rust 提供多种模块组织方式：内联模块、文件模块、目录模块");
    println!();

    // 演示内联模块
    demonstrate_inline_modules();

    // 演示文件模块概念
    demonstrate_file_modules();

    // 演示目录模块概念
    demonstrate_directory_modules();

    // 演示模块层次结构
    demonstrate_module_hierarchy();

    println!();
}

/// 演示内联模块
fn demonstrate_inline_modules() {
    println!("\n内联模块:");

    mod inline_example {
        pub struct Config {
            pub name: String,
            pub version: String,
        }

        impl Config {
            pub fn new(name: &str, version: &str) -> Self {
                Config {
                    name: name.to_string(),
                    version: version.to_string(),
                }
            }

            pub fn display(&self) {
                println!("{} v{}", self.name, self.version);
            }
        }

        pub mod utils {
            pub fn validate_version(version: &str) -> bool {
                version.chars().any(|c| c.is_ascii_digit())
            }
        }
    }

    let config = inline_example::Config::new("MyApp", "1.0.0");
    config.display();

    let is_valid = inline_example::utils::validate_version("1.0.0");
    println!("版本有效性: {}", is_valid);

    println!("内联模块直接在源文件中定义，适合小型模块");
}

/// 演示文件模块概念
fn demonstrate_file_modules() {
    println!("\n文件模块概念:");
    println!("文件模块将模块代码放在单独的 .rs 文件中");
    println!("例如：mod network; 会查找 network.rs 或 network/mod.rs");

    // 模拟文件模块结构
    mod network {
        pub fn connect() {
            println!("网络连接建立");
        }

        pub fn disconnect() {
            println!("网络连接断开");
        }

        pub mod tcp {
            pub fn listen(port: u16) {
                println!("TCP 监听端口: {}", port);
            }
        }

        pub mod udp {
            pub fn bind(port: u16) {
                println!("UDP 绑定端口: {}", port);
            }
        }
    }

    network::connect();
    network::tcp::listen(8080);
    network::udp::bind(9090);
    network::disconnect();

    println!("文件模块提供了更好的代码组织和可维护性");
}

/// 演示目录模块概念
fn demonstrate_directory_modules() {
    println!("\n目录模块概念:");
    println!("目录模块使用目录结构组织复杂的模块层次");

    // 模拟目录模块结构
    mod web_framework {
        pub mod routing {
            pub fn get(path: &str) {
                println!("GET {}", path);
            }

            pub fn post(path: &str) {
                println!("POST {}", path);
            }
        }

        pub mod middleware {
            pub fn auth() {
                println!("认证中间件");
            }

            pub fn cors() {
                println!("CORS 中间件");
            }
        }

        pub mod handlers {
            pub fn user_handler() {
                println!("用户处理器");
            }

            pub fn product_handler() {
                println!("产品处理器");
            }
        }
    }

    web_framework::routing::get("/api/users");
    web_framework::middleware::auth();
    web_framework::handlers::user_handler();

    println!("目录模块适合大型项目的模块组织");
}

/// 演示模块层次结构
fn demonstrate_module_hierarchy() {
    println!("\n模块层次结构:");

    mod application {
        pub mod core {
            pub struct Engine {
                name: String,
            }

            impl Engine {
                pub fn new(name: &str) -> Self {
                    Engine {
                        name: name.to_string(),
                    }
                }

                pub fn start(&self) {
                    println!("{} 引擎启动", self.name);
                }
            }

            pub mod components {
                pub fn initialize() {
                    println!("组件初始化");
                }
            }
        }

        pub mod services {
            pub mod database {
                pub fn connect() {
                    println!("数据库服务连接");
                }
            }

            pub mod cache {
                pub fn initialize() {
                    println!("缓存服务初始化");
                }
            }
        }

        pub mod utils {
            pub fn log(message: &str) {
                println!("[LOG] {}", message);
            }
        }
    }

    let engine = application::core::Engine::new("WebServer");
    engine.start();
    application::core::components::initialize();
    application::services::database::connect();
    application::services::cache::initialize();
    application::utils::log("应用程序启动完成");

    println!("层次结构提供了清晰的架构组织");
}

// ============================================================================
// 3. 可见性控制
// ============================================================================

/// 演示可见性控制
fn demonstrate_visibility_control() {
    println!("3. 可见性控制:");
    println!("Rust 提供了精细的可见性控制机制：pub、pub(crate)、pub(super)、pub(in path)");
    println!();

    // 演示基本可见性
    demonstrate_basic_visibility();

    // 演示 pub(crate) 可见性
    demonstrate_crate_visibility();

    // 演示 pub(super) 可见性
    demonstrate_super_visibility();

    // 演示 pub(in path) 可见性
    demonstrate_path_visibility();

    println!();
}

/// 演示基本可见性
fn demonstrate_basic_visibility() {
    println!("\n基本可见性控制:");

    mod visibility_example {
        pub struct PublicStruct {
            pub public_field: i32,
            private_field: i32,
        }

        impl PublicStruct {
            pub fn new(public: i32, private: i32) -> Self {
                PublicStruct {
                    public_field: public,
                    private_field: private,
                }
            }

            pub fn get_private(&self) -> i32 {
                self.private_field
            }

            fn private_method(&self) {
                println!("私有方法");
            }

            pub fn call_private(&self) {
                self.private_method();
            }
        }

        struct PrivateStruct {
            value: i32,
        }

        pub fn create_private() -> PrivateStruct {
            PrivateStruct { value: 42 }
        }
    }

    let public_struct = visibility_example::PublicStruct::new(10, 20);
    println!("公共字段: {}", public_struct.public_field);
    println!("私有字段 (通过方法): {}", public_struct.get_private());
    public_struct.call_private();

    // 私有结构体只能通过公共函数创建
    // let _private_struct = visibility_example::create_private(); // 注释掉私有类型的使用

    println!("基本可见性控制了模块外部的访问权限");
}

/// 演示 pub(crate) 可见性
fn demonstrate_crate_visibility() {
    println!("\n包级可见性 pub(crate):");

    mod crate_visibility_example {
        pub(crate) struct CrateStruct {
            pub(crate) value: i32,
        }

        impl CrateStruct {
            pub(crate) fn new(value: i32) -> Self {
                CrateStruct { value }
            }

            pub(crate) fn process(&self) {
                println!("处理包级结构体: {}", self.value);
            }
        }

        pub(crate) fn crate_function() {
            println!("包级函数");
        }
    }

    let crate_struct = crate_visibility_example::CrateStruct::new(100);
    crate_struct.process();
    crate_visibility_example::crate_function();

    println!("pub(crate) 限制可见性在当前包内");
}

/// 演示 pub(super) 可见性
fn demonstrate_super_visibility() {
    println!("\n父模块可见性 pub(super):");

    mod parent {
        pub mod child {
            pub(super) struct SuperStruct {
                pub(super) data: String,
            }

            impl SuperStruct {
                pub(super) fn new(data: &str) -> Self {
                    SuperStruct {
                        data: data.to_string(),
                    }
                }

                pub(super) fn display(&self) {
                    println!("父模块可见数据: {}", self.data);
                }
            }

            pub(super) fn super_function() {
                println!("父模块可见函数");
            }
        }

        pub fn use_child_items() {
            let super_struct = child::SuperStruct::new("测试数据");
            super_struct.display();
            child::super_function();
        }
    }

    parent::use_child_items();
    println!("pub(super) 限制可见性在父模块内");
}

/// 演示 pub(in path) 可见性
fn demonstrate_path_visibility() {
    println!("\n路径限制可见性 pub(in path):");

    mod organization {
        pub mod team_a {
            pub(super) struct TeamResource {
                name: String,
            }

            impl TeamResource {
                pub(super) fn new(name: String) -> Self {
                    TeamResource { name }
                }

                pub(super) fn get_name(&self) -> &str {
                    &self.name
                }
            }
        }

        pub mod team_b {
            use super::team_a::TeamResource;

            pub fn use_team_resource() {
                // 可以在指定路径内访问
                let resource = TeamResource::new("共享资源".to_string());
                println!("Team B 使用资源: {}", resource.get_name());
            }
        }

        pub fn organization_function() {
            println!("组织级函数");
            team_b::use_team_resource();
        }
    }

    organization::organization_function();
    println!("pub(in path) 提供了精确的可见性控制");
}

// ============================================================================
// 4. use 引入系统
// ============================================================================

/// 演示 use 引入系统
fn demonstrate_use_imports() {
    println!("4. use 引入系统:");
    println!("use 语句提供了灵活的模块引入机制，支持绝对路径、相对路径、重命名等");
    println!();

    // 演示基本 use 语句
    demonstrate_basic_use();

    // 演示路径类型
    demonstrate_path_types();

    // 演示重命名和别名
    demonstrate_renaming_and_aliases();

    // 演示 glob 导入
    demonstrate_glob_imports();

    // 演示嵌套导入
    demonstrate_nested_imports();

    println!();
}

/// 演示基本 use 语句
fn demonstrate_basic_use() {
    println!("\n基本 use 语句:");

    mod math_operations {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        pub fn subtract(a: i32, b: i32) -> i32 {
            a - b
        }
        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }
        pub fn divide(a: i32, b: i32) -> Option<i32> {
            if b != 0 { Some(a / b) } else { None }
        }

        pub mod advanced {
            pub fn power(base: i32, exp: u32) -> i32 {
                base.pow(exp)
            }

            pub fn factorial(n: u32) -> u64 {
                (1..=n as u64).product()
            }
        }
    }

    // 不使用 use 的情况
    let result1 = math_operations::add(5, 3);
    let result2 = math_operations::advanced::power(2, 3);

    println!("不使用 use: {} + 3 = {}", 5, result1);
    println!("不使用 use: 2^3 = {}", result2);

    // 使用 use 简化调用
    {
        use math_operations::advanced::factorial;
        use math_operations::{add, multiply, subtract};

        let sum = add(10, 20);
        let diff = subtract(20, 10);
        let product = multiply(5, 6);
        let fact = factorial(5);

        println!("使用 use: 10 + 20 = {}", sum);
        println!("使用 use: 20 - 10 = {}", diff);
        println!("使用 use: 5 * 6 = {}", product);
        println!("使用 use: 5! = {}", fact);
    }

    println!("use 语句简化了函数调用");
}

/// 演示路径类型
fn demonstrate_path_types() {
    println!("\n路径类型:");

    mod path_examples {
        pub mod level1 {
            pub mod level2 {
                pub fn deep_function() {
                    println!("深层函数");
                }

                pub fn demonstrate_paths() {
                    // 相对路径 (从当前模块开始)
                    self::deep_function();

                    // 父模块路径
                    super::parent_function();

                    // 祖父模块路径
                    super::super::grandparent_function();
                }
            }

            pub fn parent_function() {
                println!("父模块函数");
            }
        }

        pub fn grandparent_function() {
            println!("祖父模块函数");
        }
    }

    // 演示不同路径类型的使用
    {
        use path_examples::grandparent_function;
        use path_examples::level1::level2::deep_function;

        deep_function();
        grandparent_function();

        // 调用演示路径的函数
        path_examples::level1::level2::demonstrate_paths();
    }

    println!("不同路径类型提供了灵活的引用方式");
}

/// 演示重命名和别名
fn demonstrate_renaming_and_aliases() {
    println!("\n重命名和别名:");

    mod string_utils {
        pub fn format(text: &str) -> String {
            format!("[格式化] {}", text)
        }

        pub fn validate(text: &str) -> bool {
            !text.is_empty()
        }
    }

    mod number_utils {
        pub fn format(num: i32) -> String {
            format!("数字: {}", num)
        }

        pub fn validate(num: i32) -> bool {
            num >= 0
        }
    }

    // 使用别名避免命名冲突
    {
        use number_utils::format as number_format;
        use number_utils::validate as number_validate;
        use string_utils::format as string_format;
        use string_utils::validate as string_validate;

        let text = "Hello";
        let number = 42;

        println!("{}", string_format(text));
        println!("字符串有效: {}", string_validate(text));
        println!("{}", number_format(number));
        println!("数字有效: {}", number_validate(number));
    }

    println!("别名解决了命名冲突问题");
}

/// 演示 glob 导入
fn demonstrate_glob_imports() {
    println!("\n glob 导入:");

    mod utilities {
        pub fn helper1() {
            println!("辅助函数 1");
        }
        pub fn helper2() {
            println!("辅助函数 2");
        }
        pub fn helper3() {
            println!("辅助函数 3");
        }

        pub const CONSTANT1: i32 = 100;
        pub const CONSTANT2: i32 = 200;
    }

    // 使用 glob 导入所有公共项
    {
        use utilities::*;

        helper1();
        helper2();
        helper3();

        println!("常量1: {}", CONSTANT1);
        println!("常量2: {}", CONSTANT2);
    }

    println!("glob 导入方便但要谨慎使用，避免命名空间污染");
}

/// 演示嵌套导入
fn demonstrate_nested_imports() {
    println!("\n嵌套导入:");

    mod collections {
        pub mod list {
            pub fn create() {
                println!("创建列表");
            }
            pub fn append() {
                println!("追加元素");
            }
            pub fn remove() {
                println!("移除元素");
            }
        }

        pub mod map {
            pub fn create() {
                println!("创建映射");
            }
            pub fn insert() {
                println!("插入键值对");
            }
            pub fn get() {
                println!("获取值");
            }
        }

        pub mod set {
            pub fn create() {
                println!("创建集合");
            }
            pub fn add() {
                println!("添加元素");
            }
            pub fn contains() {
                println!("检查包含");
            }
        }
    }

    // 使用嵌套导入
    {
        use collections::{
            list::{append, create as create_list},
            map::{create as create_map, get, insert},
            set::{add, create as create_set},
        };

        create_list();
        append();

        create_map();
        insert();
        get();

        create_set();
        add();
    }

    println!("嵌套导入提供了简洁的多项导入语法");
}

// ============================================================================
// 5. 模块路径系统
// ============================================================================

/// 演示模块路径系统
fn demonstrate_module_paths() {
    println!("5. 模块路径系统:");
    println!("Rust 提供了灵活的路径解析机制：绝对路径、相对路径、关键字");
    println!();

    // 演示绝对路径 vs 相对路径
    demonstrate_absolute_vs_relative();

    // 演示路径关键字
    demonstrate_path_keywords();

    // 演示路径解析规则
    demonstrate_path_resolution();

    println!();
}

/// 演示绝对路径 vs 相对路径
fn demonstrate_absolute_vs_relative() {
    println!("\n绝对路径 vs 相对路径:");

    mod absolute_example {
        pub mod level1 {
            pub mod level2 {
                pub fn deep_function() {
                    println!("绝对路径示例中的深层函数");
                }
            }
        }
    }

    mod relative_example {
        pub mod level1 {
            pub mod level2 {
                pub fn deep_function() {
                    println!("相对路径示例中的深层函数");

                    // 相对路径调用
                    super::sibling_function();
                    super::super::parent_function();
                }
            }

            pub fn sibling_function() {
                println!("兄弟模块函数");
            }
        }

        pub fn parent_function() {
            println!("父级模块函数");
        }
    }

    // 使用绝对路径和相对路径
    absolute_example::level1::level2::deep_function();
    relative_example::level1::level2::deep_function();

    println!("绝对路径从 crate 根开始，相对路径从当前位置开始");
}

/// 演示路径关键字
fn demonstrate_path_keywords() {
    println!("\n路径关键字:");

    mod keyword_example {
        pub fn crate_function() {
            println!("使用 crate 关键字");
        }

        pub mod inner {
            pub fn self_function() {
                println!("使用 self 关键字");
            }

            pub fn super_function() {
                println!("使用 super 关键字");
            }

            pub fn demonstrate_keywords() {
                // self: 当前模块
                self::self_function();

                // super: 父模块
                super::crate_function();

                // crate: 包根
                crate::demonstrate_module_paths();
            }
        }
    }

    keyword_example::inner::demonstrate_keywords();

    println!("路径关键字提供了明确的路径引用方式");
}

/// 演示路径解析规则
fn demonstrate_path_resolution() {
    println!("\n路径解析规则:");

    mod resolution_example {
        pub struct Item {
            pub name: String,
        }

        impl Item {
            pub fn new(name: &str) -> Self {
                Item {
                    name: name.to_string(),
                }
            }
        }

        pub mod nested {
            use super::Item;

            pub fn create_item() -> Item {
                Item::new("嵌套项")
            }

            pub fn demonstrate_resolution() {
                // 1. 当前作用域查找
                let item1 = create_item();

                // 2. 父模块查找
                let item2 = super::Item::new("父模块项");

                // 3. 使用 use 引入的项
                let item3 = Item::new("引入项");

                println!("项1: {}", item1.name);
                println!("项2: {}", item2.name);
                println!("项3: {}", item3.name);
            }
        }
    }

    resolution_example::nested::demonstrate_resolution();

    println!("路径解析遵循作用域查找规则");
}

// ============================================================================
// 6. 预导入和标准库
// ============================================================================

/// 演示预导入和标准库
fn demonstrate_prelude_and_std() {
    println!("6. 预导入和标准库:");
    println!("Rust 自动导入标准库的预导入模块，提供常用类型和特征");
    println!();

    // 演示标准库预导入
    demonstrate_std_prelude();

    // 演示自定义预导入
    demonstrate_custom_prelude();

    // 演示标准库模块结构
    demonstrate_std_structure();

    println!();
}

/// 演示标准库预导入
fn demonstrate_std_prelude() {
    println!("\n标准库预导入:");

    // 这些类型无需显式导入，因为它们在预导入中
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    let map: HashMap<String, i32> = HashMap::new();
    let string: String = String::from("Hello, World!");
    let option: Option<i32> = Some(42);
    let result: Result<i32, &str> = Ok(100);

    println!("Vector: {:?}", vector);
    println!("HashMap 创建成功");
    println!("String: {}", string);
    println!("Option: {:?}", option);
    println!("Result: {:?}", result);

    // 预导入的特征也可以直接使用
    let formatted = format!("格式化字符串: {}", 42);
    println!("{}", formatted);

    println!("预导入使常用类型和特征可以直接使用");
}

/// 演示自定义预导入
fn demonstrate_custom_prelude() {
    println!("\n自定义预导入:");

    mod my_library {
        pub mod prelude {
            pub use super::core::{MyStruct, MyTrait};
            pub use super::utils::{CONSTANT, helper_function};
        }

        pub mod core {
            pub struct MyStruct {
                pub value: i32,
            }

            pub trait MyTrait {
                fn process(&self);
            }

            impl MyTrait for MyStruct {
                fn process(&self) {
                    println!("处理值: {}", self.value);
                }
            }
        }

        pub mod utils {
            pub fn helper_function() {
                println!("辅助函数");
            }

            pub const CONSTANT: i32 = 42;
        }
    }

    // 使用自定义预导入
    {
        use my_library::prelude::*;

        let my_struct = MyStruct { value: 100 };
        my_struct.process();
        helper_function();
        println!("常量值: {}", CONSTANT);
    }

    println!("自定义预导入简化了库的使用");
}

/// 演示标准库模块结构
fn demonstrate_std_structure() {
    println!("\n标准库模块结构:");

    // std::collections - 集合类型
    {
        use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

        let mut map = HashMap::new();
        map.insert("key", "value");

        let mut set = HashSet::new();
        set.insert(42);

        let mut btree = BTreeMap::new();
        btree.insert(1, "one");

        let mut deque = VecDeque::new();
        deque.push_back(1);

        println!("集合类型演示完成");
    }

    // std::io - 输入输出
    {
        use std::io::{self, Write};

        let mut buffer = Vec::new();
        writeln!(buffer, "写入缓冲区").unwrap();
        println!("IO 操作演示完成");
    }

    // std::fs - 文件系统 (概念演示)
    println!("文件系统模块: std::fs");

    // std::net - 网络 (概念演示)
    println!("网络模块: std::net");

    // std::thread - 线程 (概念演示)
    println!("线程模块: std::thread");

    println!("标准库提供了丰富的模块和功能");
}

// ============================================================================
// 7. 高级特性
// ============================================================================

/// 演示高级特性
fn demonstrate_advanced_features() {
    println!("7. 高级特性:");
    println!("模块系统的高级特性：条件编译、cfg 属性、feature gates、模块别名");
    println!();

    // 演示条件编译
    demonstrate_conditional_compilation();

    // 演示 cfg 属性
    demonstrate_cfg_attributes();

    // 演示 feature gates
    demonstrate_feature_gates();

    // 演示模块别名和重导出
    demonstrate_module_aliases();

    println!();
}

/// 演示条件编译
fn demonstrate_conditional_compilation() {
    println!("\n条件编译:");

    mod platform_specific {
        #[cfg(target_os = "windows")]
        pub fn platform_function() {
            println!("Windows 特定功能");
        }

        #[cfg(target_os = "macos")]
        pub fn platform_function() {
            println!("macOS 特定功能");
        }

        #[cfg(target_os = "linux")]
        pub fn platform_function() {
            println!("Linux 特定功能");
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        pub fn platform_function() {
            println!("其他平台功能");
        }

        #[cfg(debug_assertions)]
        pub fn debug_function() {
            println!("调试模式功能");
        }

        #[cfg(not(debug_assertions))]
        pub fn debug_function() {
            println!("发布模式功能");
        }
    }

    platform_specific::platform_function();
    platform_specific::debug_function();

    println!("条件编译允许平台和配置特定的代码");
}

/// 演示 cfg 属性
fn demonstrate_cfg_attributes() {
    println!("\n cfg 属性:");

    // 示例代码：演示 cfg 条件编译，feature 未在 Cargo.toml 中定义
    #[allow(unexpected_cfgs)]
    mod cfg_example {
        #[cfg(feature = "advanced")]
        pub fn advanced_feature() {
            println!("高级功能已启用");
        }

        #[cfg(not(feature = "advanced"))]
        pub fn advanced_feature() {
            println!("高级功能未启用");
        }

        #[cfg(test)]
        pub fn test_helper() {
            println!("测试辅助函数");
        }

        #[cfg(all(unix, target_pointer_width = "64"))]
        pub fn unix_64bit_function() {
            println!("64位 Unix 系统功能");
        }

        #[cfg(any(feature = "serde", feature = "json"))]
        pub fn serialization_function() {
            println!("序列化功能");
        }
    }

    cfg_example::advanced_feature();

    println!("cfg 属性提供了灵活的条件编译控制");
}

/// 演示 feature gates
fn demonstrate_feature_gates() {
    println!("\n feature gates:");

    mod feature_example {
        // 模拟 feature gate 功能
        pub mod basic {
            pub fn basic_function() {
                println!("基础功能");
            }
        }

        // 示例代码：演示 feature 条件编译
        #[allow(unexpected_cfgs)]
        #[cfg(feature = "experimental")]
        pub mod experimental {
            pub fn experimental_function() {
                println!("实验性功能");
            }
        }

        // 示例代码：演示 feature 条件编译
        #[allow(unexpected_cfgs)]
        #[cfg(feature = "networking")]
        pub mod networking {
            pub fn network_function() {
                println!("网络功能");
            }
        }

        #[allow(unexpected_cfgs)]
        pub fn demonstrate_features() {
            basic::basic_function();

            #[cfg(feature = "experimental")]
            experimental::experimental_function();

            #[cfg(feature = "networking")]
            networking::network_function();
        }
    }

    feature_example::demonstrate_features();

    println!("feature gates 允许可选功能的条件编译");
}

/// 演示模块别名和重导出
fn demonstrate_module_aliases() {
    println!("\n模块别名和重导出:");

    mod internal {
        pub mod deep {
            pub mod nested {
                pub struct ImportantType {
                    pub value: i32,
                }

                pub fn utility_function() {
                    println!("实用工具函数");
                }
            }
        }

        pub mod algorithms {
            pub fn sort() {
                println!("排序算法");
            }

            pub fn search() {
                println!("搜索算法");
            }
        }
    }

    mod public_api {
        // 直接定义公共 API 类型和函数
        pub struct ImportantType {
            pub value: i32,
        }

        pub fn utility_function() {
            println!("实用工具函数");
        }

        // 算法模块
        pub mod algo {
            pub fn sort() {
                println!("排序算法");
            }

            pub fn search() {
                println!("搜索算法");
            }
        }

        // 创建模块别名
        pub mod types {
            pub use super::ImportantType;
        }
    }

    // 使用重导出的 API
    let important = public_api::ImportantType { value: 42 };
    println!("重要类型值: {}", important.value);

    public_api::utility_function();
    public_api::algo::sort();
    public_api::algo::search();

    // 使用别名
    let _aliased = public_api::types::ImportantType { value: 100 };

    println!("重导出和别名简化了复杂的模块结构");
}

// ============================================================================
// 8. 实际应用案例
// ============================================================================

/// 演示实际应用案例
fn demonstrate_real_world_examples() {
    println!("8. 实际应用案例:");
    println!("展示模块系统在实际项目中的应用模式");
    println!();

    // 演示库设计模式
    demonstrate_library_design();

    // 演示微服务架构
    demonstrate_microservice_architecture();

    // 演示游戏引擎结构
    demonstrate_game_engine_structure();

    println!();
}

/// 演示库设计模式
fn demonstrate_library_design() {
    println!("\n库设计模式:");

    mod web_framework {
        // 公共 API 模块
        pub mod prelude {
            pub use super::core::{App, Handler, Router};
            pub use super::middleware::{Middleware, auth, cors};
            pub use super::response::{Html, Json, Response};
        }

        // 核心功能
        pub mod core {
            pub struct App {
                router: Router,
            }

            impl App {
                pub fn new() -> Self {
                    App {
                        router: Router::new(),
                    }
                }

                pub fn run(&self) {
                    println!("Web 应用启动");
                }
            }

            pub struct Router {
                routes: Vec<String>,
            }

            impl Router {
                pub fn new() -> Self {
                    Router { routes: Vec::new() }
                }
            }

            pub trait Handler {
                fn handle(&self);
            }
        }

        // 中间件系统
        pub mod middleware {
            pub trait Middleware {
                fn process(&self);
            }

            pub fn cors() {
                println!("CORS 中间件");
            }

            pub fn auth() {
                println!("认证中间件");
            }
        }

        // 响应类型
        pub mod response {
            pub struct Response {
                status: u16,
            }

            impl Response {
                pub fn new(status: u16) -> Self {
                    Response { status }
                }
            }

            pub fn Json(data: &str) -> Response {
                println!("JSON 响应: {}", data);
                Response::new(200)
            }

            pub fn Html(content: &str) -> Response {
                println!("HTML 响应: {}", content);
                Response::new(200)
            }
        }

        // 内部实现细节
        mod internal {
            pub fn parse_request() {
                println!("解析请求");
            }

            pub fn handle_error() {
                println!("处理错误");
            }
        }
    }

    // 使用库的简洁 API
    {
        use web_framework::prelude::*;

        let app = App::new();
        cors();
        auth();
        Json("Hello, World!");
        Html("<h1>Welcome</h1>");
        app.run();
    }

    println!("库设计模式提供了清晰的公共 API 和隐藏的实现细节");
}

/// 演示微服务架构
fn demonstrate_microservice_architecture() {
    println!("\n微服务架构:");

    mod microservices {
        // 共享类型和工具
        pub mod shared {
            pub mod types {
                pub struct UserId(pub u64);
                pub struct ProductId(pub u64);
                pub struct OrderId(pub u64);
            }

            pub mod utils {
                pub fn validate_id(id: u64) -> bool {
                    id > 0
                }

                pub fn generate_uuid() -> String {
                    "uuid-12345".to_string()
                }
            }
        }

        // 用户服务
        pub mod user_service {
            use super::shared::types::UserId;

            pub struct UserService;

            impl UserService {
                pub fn new() -> Self {
                    UserService
                }

                pub fn create_user(&self, name: &str) -> UserId {
                    println!("创建用户: {}", name);
                    UserId(1)
                }

                pub fn get_user(&self, id: UserId) -> Option<String> {
                    println!("获取用户: {}", id.0);
                    Some("John Doe".to_string())
                }
            }
        }

        // 产品服务
        pub mod product_service {
            use super::shared::types::ProductId;

            pub struct ProductService;

            impl ProductService {
                pub fn new() -> Self {
                    ProductService
                }

                pub fn create_product(&self, name: &str, price: f64) -> ProductId {
                    println!("创建产品: {} - ${}", name, price);
                    ProductId(1)
                }

                pub fn get_product(&self, id: ProductId) -> Option<(String, f64)> {
                    println!("获取产品: {}", id.0);
                    Some(("Laptop".to_string(), 999.99))
                }
            }
        }

        // 订单服务
        pub mod order_service {
            use super::shared::types::{OrderId, ProductId};

            pub struct OrderService;

            impl OrderService {
                pub fn new() -> Self {
                    OrderService
                }

                pub fn create_order(&self, product_id: ProductId, quantity: u32) -> OrderId {
                    println!("创建订单: 产品 {} x {}", product_id.0, quantity);
                    OrderId(1)
                }

                pub fn process_order(&self, order_id: OrderId) {
                    println!("处理订单: {}", order_id.0);
                }
            }
        }
    }

    // 使用微服务
    {
        use microservices::shared::types::*;
        use microservices::{
            order_service::OrderService, product_service::ProductService, user_service::UserService,
        };

        let user_svc = UserService::new();
        let product_svc = ProductService::new();
        let order_svc = OrderService::new();

        let user_id = user_svc.create_user("Alice");
        let product_id = product_svc.create_product("Smartphone", 699.99);
        let order_id = order_svc.create_order(product_id, 2);

        order_svc.process_order(order_id);
    }

    println!("微服务架构通过模块分离实现了服务的独立性");
}

/// 演示游戏引擎结构
fn demonstrate_game_engine_structure() {
    println!("\n游戏引擎结构:");

    mod game_engine {
        // 核心引擎
        pub mod core {
            pub struct Engine {
                running: bool,
            }

            impl Engine {
                pub fn new() -> Self {
                    Engine { running: false }
                }

                pub fn start(&mut self) {
                    self.running = true;
                    println!("游戏引擎启动");
                }

                pub fn update(&mut self, delta_time: f32) {
                    if self.running {
                        println!("引擎更新: {:.2}ms", delta_time);
                    }
                }

                pub fn shutdown(&mut self) {
                    self.running = false;
                    println!("游戏引擎关闭");
                }
            }
        }

        // 渲染系统
        pub mod rendering {
            pub mod renderer {
                pub struct Renderer;

                impl Renderer {
                    pub fn new() -> Self {
                        Renderer
                    }

                    pub fn render(&self) {
                        println!("渲染帧");
                    }
                }
            }

            pub mod mesh {
                pub struct Vertex {
                    pub x: f32,
                    pub y: f32,
                    pub z: f32,
                }

                pub struct Mesh {
                    vertices: Vec<Vertex>,
                }

                impl Mesh {
                    pub fn new() -> Self {
                        Mesh {
                            vertices: Vec::new(),
                        }
                    }
                }
            }

            pub use mesh::Mesh;
            pub use renderer::Renderer;
        }

        // 物理系统
        pub mod physics {
            pub struct PhysicsWorld {
                gravity: f32,
            }

            impl PhysicsWorld {
                pub fn new() -> Self {
                    PhysicsWorld { gravity: -9.81 }
                }

                pub fn step(&self, delta_time: f32) {
                    println!("物理模拟步进: {:.2}s", delta_time);
                }
            }

            pub struct RigidBody {
                mass: f32,
            }

            impl RigidBody {
                pub fn new(mass: f32) -> Self {
                    RigidBody { mass }
                }
            }
        }

        // 音频系统
        pub mod audio {
            pub struct AudioEngine;

            impl AudioEngine {
                pub fn new() -> Self {
                    AudioEngine
                }

                pub fn play_sound(&self, sound: &str) {
                    println!("播放音效: {}", sound);
                }

                pub fn play_music(&self, music: &str) {
                    println!("播放音乐: {}", music);
                }
            }
        }

        // 游戏对象系统
        pub mod gameobject {
            use super::physics::RigidBody;
            use super::rendering::Mesh;

            pub struct GameObject {
                name: String,
                mesh: Option<Mesh>,
                rigidbody: Option<RigidBody>,
            }

            impl GameObject {
                pub fn new(name: &str) -> Self {
                    GameObject {
                        name: name.to_string(),
                        mesh: None,
                        rigidbody: None,
                    }
                }

                pub fn add_mesh(&mut self, mesh: Mesh) {
                    self.mesh = Some(mesh);
                }

                pub fn add_rigidbody(&mut self, rigidbody: RigidBody) {
                    self.rigidbody = Some(rigidbody);
                }

                pub fn update(&mut self, _delta_time: f32) {
                    println!("更新游戏对象: {}", self.name);
                }
            }
        }
    }

    // 使用游戏引擎
    {
        use game_engine::{
            audio::AudioEngine,
            core::Engine,
            gameobject::GameObject,
            physics::{PhysicsWorld, RigidBody},
            rendering::{Mesh, Renderer},
        };

        let mut engine = Engine::new();
        let renderer = Renderer::new();
        let physics_world = PhysicsWorld::new();
        let audio_engine = AudioEngine::new();

        engine.start();

        let mut player = GameObject::new("Player");
        player.add_mesh(Mesh::new());
        player.add_rigidbody(RigidBody::new(1.0));

        // 游戏循环模拟
        for frame in 1..=3 {
            let delta_time = 16.67; // 60 FPS

            engine.update(delta_time);
            physics_world.step(delta_time / 1000.0);
            player.update(delta_time);
            renderer.render();

            if frame == 1 {
                audio_engine.play_sound("jump.wav");
                audio_engine.play_music("background.mp3");
            }
        }

        engine.shutdown();
    }

    println!("游戏引擎结构展示了复杂系统的模块化组织");
}
