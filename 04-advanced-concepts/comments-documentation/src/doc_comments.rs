//! # Rust 文档注释系统全面分析
//!
//! 本模块深入分析 Rust 的文档注释系统，包括语法、Markdown 支持、
//! 代码示例嵌入等所有相关特性。
//!
//! ## 文档注释类型
//!
//! Rust 提供两种主要的文档注释类型：
//! - `///` - 外部文档注释（documenting the next item）
//! - `//!` - 内部文档注释（documenting the enclosing item）
//!
//! ## Markdown 支持
//!
//! 文档注释完全支持 **Markdown** 语法，包括：
//! - *斜体* 和 **粗体** 文本
//! - `代码片段` 和代码块
//! - [链接](https://doc.rust-lang.org/)
//! - 列表和表格
//! - 标题和分隔符
//!
//! ## 代码示例
//!
//! ```rust
//! // 这是一个在文档中的代码示例
//! let example = "Hello, rustdoc!";
//! println!("{}", example);
//! ```
//!
//! ## 特殊标记
//!
//! 文档注释支持特殊的标记来组织内容：
//! - `# Examples` - 使用示例
//! - `# Panics` - 可能的 panic 情况
//! - `# Errors` - 可能的错误
//! - `# Safety` - 安全性说明（unsafe 代码）
//! - `# Arguments` - 参数说明
//! - `# Returns` - 返回值说明

use std::collections::HashMap;
use std::fmt::Display;

/// 演示文档注释系统的主函数
///
/// 这个函数展示了 Rust 文档注释的各种特性和用法。
///
/// # Examples
///
/// ```rust
/// # use comments_documentation::doc_comments::demonstrate_doc_comments;
/// demonstrate_doc_comments();
/// ```
///
/// # Note
///
/// 这个函数会打印详细的演示信息到标准输出。
pub fn demonstrate_doc_comments() {
    println!("\n=== Rust 文档注释系统演示 ===");

    // 演示各种文档注释特性
    demonstrate_outer_doc_comments();
    demonstrate_inner_doc_comments();
    demonstrate_markdown_features();
    demonstrate_code_examples();

    // 演示特殊段落功能
    let mut demo_options = HashMap::new();
    demo_options.insert("format".to_string(), "json".to_string());
    let _result = demonstrate_special_sections("演示数据", &demo_options);

    demonstrate_linking_and_references();
    demonstrate_advanced_formatting();
}

/// 演示外部文档注释（`///`）的用法
///
/// 外部文档注释用于为紧跟其后的项目（函数、结构体、枚举等）提供文档。
///
/// # 语法规则
///
/// - 使用 `///` 开头
/// - 支持多行，每行都需要 `///`
/// - 支持完整的 Markdown 语法
/// - 会被 rustdoc 工具处理生成 HTML 文档
///
/// # Examples
///
/// ```rust
/// /// 这是一个示例函数
/// ///
/// /// # Arguments
/// ///
/// /// * `name` - 用户名称
/// ///
/// /// # Returns
/// ///
/// /// 返回格式化的问候语
/// fn greet(name: &str) -> String {
///     format!("Hello, {}!", name)
/// }
/// ```
fn demonstrate_outer_doc_comments() {
    println!("\n--- 外部文档注释演示 ---");

    // 调用带有详细文档的函数
    let result1 = documented_function("Rust");
    let result2 = complex_calculation(10, 20);
    let user = create_user("Alice", 25);

    println!("函数调用结果:");
    println!("  - documented_function: {}", result1);
    println!("  - complex_calculation: {}", result2);
    println!("  - create_user: {:?}", user);

    // 演示枚举和结构体的文档
    let status = ProcessStatus::Running;
    let config = Configuration::new();

    println!("数据结构:");
    println!("  - status: {:?}", status);
    println!("  - config: {:?}", config);
}

/// 一个带有完整文档的示例函数
///
/// 这个函数演示了如何为函数编写全面的文档注释。
///
/// # Arguments
///
/// * `input` - 输入字符串，将被处理并返回
///
/// # Returns
///
/// 返回一个格式化的字符串，包含输入内容和额外信息。
///
/// # Examples
///
/// ```rust
/// # use comments_documentation::doc_comments::documented_function;
/// let result = documented_function("test");
/// assert!(result.contains("test"));
/// ```
///
/// # Panics
///
/// 当输入字符串为空时，此函数会 panic。
///
/// ```should_panic
/// # use comments_documentation::doc_comments::documented_function;
/// documented_function(""); // 这会 panic
/// ```
pub fn documented_function(input: &str) -> String {
    if input.is_empty() {
        panic!("输入不能为空");
    }
    format!("处理结果: [{}] - 长度: {}", input, input.len())
}

/// 执行复杂计算的函数
///
/// 这个函数演示了数学运算的文档化。
///
/// # Arguments
///
/// * `a` - 第一个操作数
/// * `b` - 第二个操作数
///
/// # Returns
///
/// 返回计算结果，使用以下公式：
///
/// ```text
/// result = (a + b) * 2 + a^2
/// ```
///
/// # Examples
///
/// ```rust
/// # use comments_documentation::doc_comments::complex_calculation;
/// let result = complex_calculation(3, 4);
/// assert_eq!(result, 23); // (3 + 4) * 2 + 3^2 = 14 + 9 = 23
/// ```
///
/// # Performance
///
/// 时间复杂度: O(1)  
/// 空间复杂度: O(1)
pub fn complex_calculation(a: i32, b: i32) -> i32 {
    (a + b) * 2 + a * a
}

/// 用户信息结构体
///
/// 这个结构体存储用户的基本信息。
///
/// # Fields
///
/// * `name` - 用户姓名，不能为空
/// * `age` - 用户年龄，必须为正数
///
/// # Examples
///
/// ```rust
/// # use comments_documentation::doc_comments::User;
/// let user = User {
///     name: "Alice".to_string(),
///     age: 30,
/// };
/// assert_eq!(user.name, "Alice");
/// assert_eq!(user.age, 30);
/// ```
#[derive(Debug, Clone)]
pub struct User {
    /// 用户姓名
    ///
    /// 必须是非空字符串，建议长度在 1-50 字符之间。
    pub name: String,

    /// 用户年龄
    ///
    /// 必须是正整数，有效范围 0-150。
    pub age: u32,
}

/// 创建新用户的便利函数
///
/// # Arguments
///
/// * `name` - 用户姓名
/// * `age` - 用户年龄
///
/// # Returns
///
/// 返回新创建的 [`User`] 实例。
///
/// # Examples
///
/// ```rust
/// # use comments_documentation::doc_comments::create_user;
/// let user = create_user("Bob", 25);
/// assert_eq!(user.name, "Bob");
/// assert_eq!(user.age, 25);
/// ```
pub fn create_user(name: &str, age: u32) -> User {
    User {
        name: name.to_string(),
        age,
    }
}

/// 进程状态枚举
///
/// 表示系统进程的各种可能状态。
///
/// # Variants
///
/// * [`Running`](ProcessStatus::Running) - 进程正在运行
/// * [`Stopped`](ProcessStatus::Stopped) - 进程已停止
/// * [`Error`](ProcessStatus::Error) - 进程遇到错误
///
/// # Examples
///
/// ```rust
/// # use comments_documentation::doc_comments::ProcessStatus;
/// let status = ProcessStatus::Running;
/// match status {
///     ProcessStatus::Running => println!("进程运行中"),
///     ProcessStatus::Stopped => println!("进程已停止"),
///     ProcessStatus::Error(msg) => println!("错误: {}", msg),
/// }
/// ```
#[derive(Debug)]
pub enum ProcessStatus {
    /// 进程正在正常运行
    ///
    /// 这表示进程处于活跃状态，正在执行任务。
    Running,

    /// 进程已停止
    ///
    /// 进程已经完成或被手动停止。
    Stopped,

    /// 进程遇到错误
    ///
    /// 包含错误信息的字符串，描述具体的错误原因。
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_comments::ProcessStatus;
    /// let error_status = ProcessStatus::Error("内存不足".to_string());
    /// ```
    Error(String),
}

/// 配置信息结构体
///
/// 存储应用程序的配置参数。
#[derive(Debug)]
pub struct Configuration {
    /// 服务器端口号
    pub port: u16,
    /// 是否启用调试模式
    pub debug: bool,
    /// 配置参数映射
    pub settings: HashMap<String, String>,
}

impl Configuration {
    /// 创建默认配置
    ///
    /// # Returns
    ///
    /// 返回包含默认值的配置实例。
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use comments_documentation::doc_comments::Configuration;
    /// let config = Configuration::new();
    /// assert_eq!(config.port, 8080);
    /// assert_eq!(config.debug, false);
    /// ```
    pub fn new() -> Self {
        Self {
            port: 8080,
            debug: false,
            settings: HashMap::new(),
        }
    }
}

/// 演示内部文档注释（`//!`）的用法
fn demonstrate_inner_doc_comments() {
    //! 这个函数内部使用内部文档注释
    //!
    //! 内部文档注释用于为包含它的项目提供文档，
    //! 通常用于模块、crate 或函数的开头。
    //!
    //! # 使用场景
    //!
    //! - 模块级别的文档
    //! - Crate 级别的文档
    //! - 函数内部的详细说明

    println!("\n--- 内部文档注释演示 ---");

    {
        //! 这是一个代码块的内部文档
        //!
        //! 可以用来解释复杂的算法或逻辑。

        let data = vec![1, 2, 3, 4, 5];
        let sum: i32 = data.iter().sum();
        println!("数据求和: {:?} -> {}", data, sum);
    }

    // 内部文档注释 vs 外部文档注释的对比
    inner_documented_helper();
}

fn inner_documented_helper() {
    //! 这个辅助函数使用内部文档注释
    //!
    //! 内部文档注释的特点：
    //! - 使用 `//!` 语法
    //! - 为包含它的项目提供文档
    //! - 常用于模块和 crate 级别
    //! - 可以包含完整的 Markdown 语法

    println!("内部文档注释辅助函数执行");
}

/// 演示 Markdown 功能的全面支持
///
/// Rust 文档注释支持完整的 Markdown 语法，包括各种格式化选项。
///
/// # 文本格式化
///
/// - **粗体文本** 使用 `**text**`
/// - *斜体文本* 使用 `*text*`
/// - `内联代码` 使用反引号
/// - ~~删除线~~ 使用 `~~text~~`
///
/// # 列表支持
///
/// ## 无序列表
///
/// - 第一项
/// - 第二项
///   - 嵌套项目
///   - 另一个嵌套项目
/// - 第三项
///
/// ## 有序列表
///
/// 1. 首先做这个
/// 2. 然后做那个
/// 3. 最后完成
///
/// # 链接和引用
///
/// - [Rust 官方文档](https://doc.rust-lang.org/)
/// - [The Rust Book](https://doc.rust-lang.org/book/)
/// - 内部链接: [`User`] 结构体
/// - 函数链接: [`documented_function`]
///
/// # 表格支持
///
/// | 功能 | 支持 | 说明 |
/// |------|------|------|
/// | 粗体 | ✅ | **完全支持** |
/// | 斜体 | ✅ | *完全支持* |
/// | 代码 | ✅ | `完全支持` |
/// | 表格 | ✅ | 如本表所示 |
///
/// # 引用块
///
/// > 这是一个引用块
/// >
/// > 可以包含多行内容
/// > 和 **格式化** 文本
///
/// # 分隔符
///
/// ---
///
/// 上面是一个水平分隔符。
fn demonstrate_markdown_features() {
    println!("\n--- Markdown 功能演示 ---");
    println!("Markdown 功能在生成的文档中可见，这里展示基本信息:");

    // 演示各种 Markdown 元素的实际应用
    let features = vec![
        "粗体和斜体文本",
        "内联代码片段",
        "有序和无序列表",
        "表格和引用块",
        "链接和引用",
        "水平分隔符",
    ];

    println!("支持的 Markdown 功能:");
    for (i, feature) in features.iter().enumerate() {
        println!("  {}. {}", i + 1, feature);
    }
}

/// 演示代码示例的嵌入和测试
///
/// 文档注释中的代码示例不仅用于展示，还可以作为测试运行。
///
/// # 基本代码块
///
/// ```rust
/// let x = 42;
/// let y = x * 2;
/// assert_eq!(y, 84);
/// ```
///
/// # 带有隐藏行的代码块
///
/// ```rust
/// # // 这行在文档中隐藏，但在测试中运行
/// # use std::collections::HashMap;
/// let mut map = HashMap::new();
/// map.insert("key", "value");
/// assert_eq!(map.get("key"), Some(&"value"));
/// ```
///
/// # 不同语言的代码块
///
/// ```bash
/// cargo doc --open
/// ```
///
/// ```json
/// {
///   "name": "example",
///   "version": "1.0.0"
/// }
/// ```
///
/// # 带有编译标志的代码块
///
/// ```rust,no_run
/// // 这个代码会编译但不会运行
/// std::process::exit(0);
/// ```
///
/// ```rust,ignore
/// // 这个代码会被完全忽略
/// some_undefined_function();
/// ```
///
/// ```rust,should_panic
/// // 这个代码应该 panic
/// panic!("这是预期的 panic");
/// ```
fn demonstrate_code_examples() {
    println!("\n--- 代码示例演示 ---");

    // 演示文档中提到的功能
    let x = 42;
    let y = x * 2;
    println!("基本计算: {} * 2 = {}", x, y);

    // HashMap 示例
    let mut map = HashMap::new();
    map.insert("language", "Rust");
    map.insert("feature", "doc comments");

    println!("HashMap 示例:");
    for (key, value) in &map {
        println!("  {}: {}", key, value);
    }

    // 演示不同类型的代码块用途
    println!("代码块类型说明:");
    println!("  - 普通代码块: 编译并运行测试");
    println!("  - no_run: 编译但不运行");
    println!("  - ignore: 完全忽略");
    println!("  - should_panic: 预期 panic");
    println!("  - 隐藏行 (#): 在测试中运行但在文档中隐藏");
}

/// 演示特殊文档段落的使用
///
/// Rust 文档注释支持特殊的段落标记来组织内容。
///
/// # Arguments
///
/// * `input` - 输入参数，用于演示
/// * `options` - 配置选项映射
///
/// # Returns
///
/// 返回处理结果的字符串表示。
///
/// # Examples
///
/// ```rust
/// # use comments_documentation::doc_comments::demonstrate_special_sections;
/// # use std::collections::HashMap;
/// let mut options = HashMap::new();
/// options.insert("format".to_string(), "json".to_string());
/// let result = demonstrate_special_sections("test data", &options);
/// assert!(result.contains("test data"));
/// ```
///
/// # Panics
///
/// 当输入字符串长度超过 1000 字符时会 panic：
///
/// ```should_panic
/// # use comments_documentation::doc_comments::demonstrate_special_sections;
/// # use std::collections::HashMap;
/// let long_input = "a".repeat(1001);
/// let options = HashMap::new();
/// demonstrate_special_sections(&long_input, &options); // panic!
/// ```
///
/// # Errors
///
/// 虽然这个函数不返回 `Result`，但在实际应用中，
/// 可能的错误情况包括：
///
/// - 输入格式无效
/// - 配置选项冲突
/// - 系统资源不足
///
/// # Safety
///
/// 这个函数是安全的，不使用 unsafe 代码。
/// 但如果使用 unsafe 代码，这里会说明：
///
/// - 内存安全保证
/// - 线程安全性
/// - 不变量维护
///
/// # Performance
///
/// - 时间复杂度: O(n)，其中 n 是输入长度
/// - 空间复杂度: O(n)，用于存储结果
/// - 内存分配: 最多分配 2 * input.len() 字节
///
/// # See Also
///
/// - [`documented_function`] - 相关的文档化函数
/// - [`User`] - 相关的数据结构
/// - [Rust 文档指南](https://doc.rust-lang.org/rustdoc/)
pub fn demonstrate_special_sections(input: &str, options: &HashMap<String, String>) -> String {
    println!("\n--- 特殊文档段落演示 ---");

    // 检查输入长度限制
    if input.len() > 1000 {
        panic!("输入长度不能超过 1000 字符");
    }

    // 处理配置选项
    let format = options.get("format").map(|s| s.as_str()).unwrap_or("text");
    let verbose = options.get("verbose").map(|s| s == "true").unwrap_or(false);

    if verbose {
        println!("详细模式: 处理输入 '{}' (长度: {})", input, input.len());
        println!("输出格式: {}", format);
    }

    // 根据格式生成结果
    let result = match format {
        "json" => format!(r#"{{"data": "{}", "length": {}}}"#, input, input.len()),
        "xml" => format!("<data length=\"{}\">{}</data>", input.len(), input),
        _ => format!("Data: {} (Length: {})", input, input.len()),
    };

    println!("处理完成，格式: {}", format);
    result
}

/// 演示链接和引用功能
///
/// 文档注释支持多种类型的链接和引用。
///
/// # 内部链接类型
///
/// - 结构体链接: [`User`]
/// - 函数链接: [`documented_function`]
/// - 枚举链接: [`ProcessStatus`]
/// - 方法链接: [`Configuration::new`]
/// - 字段链接: [`User::name`]
///
/// # 外部链接
///
/// - [Rust 官方网站](https://www.rust-lang.org/)
/// - [Rust 文档](https://doc.rust-lang.org/)
/// - [Rustdoc 指南](https://doc.rust-lang.org/rustdoc/)
///
/// # 模块和路径引用
///
/// - [`std::collections::HashMap`]
/// - [`std::fmt::Display`]
///
/// # 带有自定义文本的链接
///
/// - [用户结构体][`User`] - 自定义链接文本
/// - [创建用户的函数][`create_user`] - 另一个自定义链接
fn demonstrate_linking_and_references() {
    println!("\n--- 链接和引用演示 ---");

    println!("文档中的链接类型:");
    println!("  - 内部链接: 指向同一 crate 中的项目");
    println!("  - 外部链接: 指向外部网站或文档");
    println!("  - 路径引用: 指向标准库或其他 crate");
    println!("  - 自定义文本: 使用自定义显示文本的链接");

    // 演示引用的实际使用
    let user = User {
        name: "Reference Demo".to_string(),
        age: 30,
    };

    let config = Configuration::new();

    println!("引用的实际对象:");
    println!("  - User: {:?}", user);
    println!("  - Configuration: {:?}", config);
}

/// 演示高级格式化功能
///
/// 包括复杂的 Markdown 结构和特殊格式。
///
/// # 复杂表格
///
/// | 功能类别 | 具体功能 | 支持程度 | 示例 |
/// |----------|----------|----------|------|
/// | 基础格式 | 粗体斜体 | ✅ 完全支持 | **粗体** *斜体* |
/// | 代码相关 | 内联代码 | ✅ 完全支持 | `let x = 42;` |
/// | 列表结构 | 嵌套列表 | ✅ 完全支持 | 见下方示例 |
/// | 链接功能 | 内外链接 | ✅ 完全支持 | [`User`] [Rust](https://rust-lang.org) |
///
/// # 复杂嵌套列表
///
/// 1. **第一级项目**
///    - 第二级子项目 A
///      - 第三级子项目 1
///      - 第三级子项目 2
///    - 第二级子项目 B
///      1. 编号子项目 α
///      2. 编号子项目 β
/// 2. **第二个第一级项目**
///    - 包含 `代码` 的子项目
///    - 包含 **粗体** 和 *斜体* 的子项目
///
/// # 多种引用块
///
/// > **重要提示**
/// >
/// > 这是一个重要的引用块，包含关键信息。
/// >
/// > ```rust
/// > // 引用块中也可以包含代码
/// > let important = "记住这一点";
/// > ```
///
/// > **警告**
/// >
/// > 这是一个警告类型的引用块。
///
/// # 数学和特殊符号
///
/// 虽然 Markdown 不直接支持 LaTeX，但可以使用 Unicode：
///
/// - 希腊字母: α β γ δ ε ζ η θ ι κ λ μ ν ξ ο π ρ σ τ υ φ χ ψ ω
/// - 数学符号: ∑ ∏ ∫ ∂ ∇ ∆ ∞ ≈ ≠ ≤ ≥ ± × ÷ √
/// - 箭头符号: → ← ↑ ↓ ↔ ⇒ ⇐ ⇑ ⇓ ⇔
/// - 其他符号: ℝ ℕ ℤ ℚ ℂ ∅ ∈ ∉ ⊂ ⊃ ∪ ∩
///
/// # 代码块的高级用法
///
/// ```rust,edition2021
/// // 指定 Rust 版本
/// #![feature(generic_associated_types)]
///
/// trait Iterator {
///     type Item<'a> where Self: 'a;
/// }
/// ```
///
/// ```text
/// 纯文本块，不会进行语法高亮
/// 适合显示配置文件或输出结果
/// ```
///
/// ```console
/// $ cargo doc --open
/// Documenting my_crate v0.1.0
///     Finished dev [unoptimized + debuginfo] target(s) in 2.34s
///      Opening target/doc/my_crate/index.html
/// ```
fn demonstrate_advanced_formatting() {
    println!("\n--- 高级格式化演示 ---");

    println!("高级格式化功能包括:");
    println!("  - 复杂表格结构");
    println!("  - 多层嵌套列表");
    println!("  - 多种引用块样式");
    println!("  - Unicode 数学符号");
    println!("  - 特殊代码块标记");

    // 演示 Unicode 符号的实际使用
    let symbols = vec![
        ("求和", "∑"),
        ("积分", "∫"),
        ("无穷", "∞"),
        ("约等于", "≈"),
        ("不等于", "≠"),
        ("箭头", "→"),
    ];

    println!("\nUnicode 符号示例:");
    for (name, symbol) in symbols {
        println!("  {}: {}", name, symbol);
    }

    // 演示复杂数据结构
    let mut complex_map = HashMap::new();
    complex_map.insert("支持度".to_string(), "完全支持".to_string());
    complex_map.insert("复杂度".to_string(), "高".to_string());

    let complex_data = (
        "高级格式化",
        vec!["表格", "列表", "引用", "符号"],
        complex_map,
    );

    println!("\n复杂数据结构: {:?}", complex_data);
}

/// 模块级别的文档注释示例
///
/// # 模块级文档注释
///
/// 这种注释风格也可以用于模块级别的文档，
/// 虽然通常推荐使用 `//!` 语法。
///
/// ## 使用场景
///
/// - 当需要大块的文档内容时
/// - 当文档包含复杂的格式时
/// - 当需要临时注释掉大段文档时
fn _module_level_doc_example() {
    // 这是一个占位函数，用于演示模块级文档注释
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试文档注释功能
    #[test]
    fn test_doc_comments() {
        demonstrate_doc_comments();
    }

    /// 测试文档化函数
    #[test]
    fn test_documented_function() {
        let result = documented_function("test");
        assert!(result.contains("test"));
        assert!(result.contains("4")); // 长度
    }

    /// 测试复杂计算函数
    #[test]
    fn test_complex_calculation() {
        assert_eq!(complex_calculation(3, 4), 23);
        assert_eq!(complex_calculation(0, 0), 0);
        assert_eq!(complex_calculation(1, 1), 5);
    }

    /// 测试用户创建
    #[test]
    fn test_create_user() {
        let user = create_user("Alice", 30);
        assert_eq!(user.name, "Alice");
        assert_eq!(user.age, 30);
    }

    /// 测试配置创建
    #[test]
    fn test_configuration_new() {
        let config = Configuration::new();
        assert_eq!(config.port, 8080);
        assert_eq!(config.debug, false);
        assert!(config.settings.is_empty());
    }

    /// 测试特殊段落函数
    #[test]
    fn test_special_sections() {
        let mut options = HashMap::new();
        options.insert("format".to_string(), "json".to_string());

        let result = demonstrate_special_sections("test", &options);
        assert!(result.contains("test"));
        assert!(result.contains("4")); // 长度
    }

    /// 测试 panic 情况
    #[test]
    #[should_panic(expected = "输入不能为空")]
    fn test_documented_function_panic() {
        documented_function("");
    }

    /// 测试长输入 panic
    #[test]
    #[should_panic(expected = "输入长度不能超过 1000 字符")]
    fn test_special_sections_panic() {
        let long_input = "a".repeat(1001);
        let options = HashMap::new();
        demonstrate_special_sections(&long_input, &options);
    }
}
