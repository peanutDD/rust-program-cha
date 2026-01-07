// 示例代码模块，允许未定义的 cfg 条件（用于演示条件编译）
#![allow(unexpected_cfgs)]

//! # Rust 文档属性系统深度分析
//!
//! 本模块全面分析 Rust 的文档属性系统，包括 `#[doc]` 属性、
//! 条件文档、文档别名、重导出等高级功能。
//!
//! ## 核心概念
//!
//! 文档属性提供了比普通文档注释更强大和灵活的文档化方式：
//! - `#[doc = "..."]` - 基本文档属性
//! - `#[doc(hidden)]` - 隐藏文档
//! - `#[doc(inline)]` - 内联文档
//! - `#[doc(no_inline)]` - 禁止内联
//! - `#[doc(alias = "...")]` - 文档别名
//! - `#[cfg_attr(..., doc = "...")]` - 条件文档
//!
//! ## 高级特性
//!
//! - 动态文档生成
//! - 平台特定文档
//! - 版本特定文档
//! - 文档重导出控制
//! - 搜索优化

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

/// 演示文档属性系统的主函数
///
/// 展示各种文档属性的使用方法和效果。
pub fn demonstrate_doc_attributes() {
    println!("\n=== Rust 文档属性系统演示 ===");

    demonstrate_basic_doc_attributes();
    demonstrate_visibility_control();
    demonstrate_inline_control();
    demonstrate_doc_aliases();
    demonstrate_conditional_docs();
    demonstrate_reexport_control();
    demonstrate_advanced_attributes();
}

/// 演示基本的 #[doc] 属性用法
///
/// `#[doc]` 属性是文档注释的底层实现，提供了更多控制选项。
fn demonstrate_basic_doc_attributes() {
    println!("\n--- 基本文档属性演示 ---");

    // 使用基本文档属性的示例
    let basic_example = BasicDocExample::new("示例数据");
    let computed_example = ComputedDocExample { value: 42 };

    println!("基本文档属性示例:");
    println!("  - BasicDocExample: {:?}", basic_example);
    println!("  - ComputedDocExample: {:?}", computed_example);

    // 演示多行文档属性
    let multiline_example = MultilineDocExample {
        data: vec![1, 2, 3, 4, 5],
    };

    println!("  - MultilineDocExample: {:?}", multiline_example);
}

/// 使用基本 #[doc] 属性的结构体
#[doc = "这是使用 #[doc] 属性定义的文档"]
#[doc = ""]
#[doc = "可以使用多个 #[doc] 属性来构建多行文档。"]
#[doc = "每个属性对应文档的一行。"]
#[derive(Debug)]
pub struct BasicDocExample {
    #[doc = "存储的数据字符串"]
    pub data: String,
}

impl BasicDocExample {
    #[doc = "创建新的 BasicDocExample 实例"]
    #[doc = ""]
    #[doc = "# Arguments"]
    #[doc = ""]
    #[doc = "* `data` - 要存储的字符串数据"]
    pub fn new(data: &str) -> Self {
        Self {
            data: data.to_string(),
        }
    }
}

/// 使用计算得出的文档内容
#[doc = concat!("这是一个计算文档示例，编译时间: ", env!("CARGO_PKG_VERSION"))]
#[derive(Debug)]
pub struct ComputedDocExample {
    /// 数值字段
    pub value: i32,
}

/// 使用多行文档属性的复杂示例
#[doc = "# MultilineDocExample"]
#[doc = ""]
#[doc = "这个结构体演示了如何使用多个 #[doc] 属性"]
#[doc = "来创建复杂的多行文档。"]
#[doc = ""]
#[doc = "## 特性"]
#[doc = ""]
#[doc = "- 支持 **Markdown** 格式"]
#[doc = "- 可以包含 `代码片段`"]
#[doc = "- 支持列表和其他格式"]
#[doc = ""]
#[doc = "## 使用示例"]
#[doc = ""]
#[doc = "```rust"]
#[doc = "let example = MultilineDocExample { data: vec![1, 2, 3] };"]
#[doc = "```"]
#[derive(Debug)]
pub struct MultilineDocExample {
    /// 存储的数据向量
    pub data: Vec<i32>,
}

/// 演示文档可见性控制
///
/// 使用 `#[doc(hidden)]` 可以隐藏不希望在公开文档中显示的项目。
fn demonstrate_visibility_control() {
    println!("\n--- 文档可见性控制演示 ---");

    // 使用公开和隐藏的 API
    let public_api = PublicApi::new();
    let _internal_result = internal_helper_function();

    println!("可见性控制示例:");
    println!("  - PublicApi: {:?}", public_api);
    println!("  - 内部函数已调用（在文档中隐藏）");

    // 演示条件可见性
    #[cfg(feature = "advanced")]
    {
        println!("  - 高级功能已启用");
    }
}

/// 公开的 API 结构体
///
/// 这个结构体在生成的文档中可见。
#[derive(Debug)]
pub struct PublicApi {
    /// 公开字段
    pub visible_field: String,

    /// 隐藏字段（在文档中不显示）
    #[doc(hidden)]
    pub hidden_field: i32,
}

impl PublicApi {
    /// 创建新的 PublicApi 实例
    pub fn new() -> Self {
        Self {
            visible_field: "公开数据".to_string(),
            hidden_field: 42,
        }
    }

    /// 公开方法
    ///
    /// 这个方法在文档中可见。
    pub fn public_method(&self) -> &str {
        &self.visible_field
    }

    /// 隐藏方法
    ///
    /// 这个方法在生成的文档中不会显示，但仍然是公开的 API。
    #[doc(hidden)]
    pub fn hidden_method(&self) -> i32 {
        self.hidden_field
    }
}

/// 内部辅助函数
///
/// 这个函数在文档中被隐藏，通常用于内部实现。
#[doc(hidden)]
pub fn internal_helper_function() -> String {
    "内部辅助功能".to_string()
}

/// 演示内联控制
///
/// `#[doc(inline)]` 和 `#[doc(no_inline)]` 控制重导出项目的文档显示方式。
fn demonstrate_inline_control() {
    println!("\n--- 内联控制演示 ---");

    // 使用内联和非内联的重导出
    let inlined = InlinedStruct::new();
    let not_inlined = NotInlinedStruct::new();

    println!("内联控制示例:");
    println!("  - InlinedStruct: {:?}", inlined);
    println!("  - NotInlinedStruct: {:?}", not_inlined);
}

/// 内联文档的结构体
///
/// 当这个结构体被重导出时，其文档会内联显示。
#[derive(Debug)]
pub struct InlinedStruct {
    /// 内联结构体的数据
    pub data: String,
}

impl InlinedStruct {
    /// 创建新的内联结构体实例
    pub fn new() -> Self {
        Self {
            data: "内联数据".to_string(),
        }
    }
}

/// 非内联文档的结构体
///
/// 当这个结构体被重导出时，只显示链接而不内联文档。
#[derive(Debug)]
pub struct NotInlinedStruct {
    /// 非内联结构体的数据
    pub data: String,
}

impl NotInlinedStruct {
    /// 创建新的非内联结构体实例
    pub fn new() -> Self {
        Self {
            data: "非内联数据".to_string(),
        }
    }
}

// 重导出示例
#[doc(inline)]
pub use self::InlinedStruct as ReexportedInlined;

#[doc(no_inline)]
pub use self::NotInlinedStruct as ReexportedNotInlined;

/// 演示文档别名功能
///
/// `#[doc(alias = "...")]` 为项目提供搜索别名。
fn demonstrate_doc_aliases() {
    println!("\n--- 文档别名演示 ---");

    // 使用带别名的结构体和函数
    let mut calculator = MathCalculator::new();
    let result1 = calculator.add(10, 20);
    let result2 = calculator.multiply(5, 6);

    println!("文档别名示例:");
    println!("  - 计算器创建成功");
    println!("  - 加法结果: {}", result1);
    println!("  - 乘法结果: {}", result2);

    // 使用别名搜索的函数
    let formatted = format_number(12345);
    println!("  - 格式化数字: {}", formatted);
}

/// 数学计算器
///
/// 提供基本的数学运算功能。
#[doc(alias = "calculator")]
#[doc(alias = "math")]
#[doc(alias = "arithmetic")]
#[derive(Debug)]
pub struct MathCalculator {
    /// 计算历史
    history: Vec<String>,
}

impl MathCalculator {
    /// 创建新的计算器
    #[doc(alias = "create")]
    #[doc(alias = "init")]
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    /// 加法运算
    ///
    /// 计算两个数的和。
    #[doc(alias = "plus")]
    #[doc(alias = "sum")]
    #[doc(alias = "+")]
    pub fn add(&mut self, a: i32, b: i32) -> i32 {
        let result = a + b;
        self.history.push(format!("{} + {} = {}", a, b, result));
        result
    }

    /// 乘法运算
    ///
    /// 计算两个数的积。
    #[doc(alias = "times")]
    #[doc(alias = "product")]
    #[doc(alias = "*")]
    pub fn multiply(&mut self, a: i32, b: i32) -> i32 {
        let result = a * b;
        self.history.push(format!("{} * {} = {}", a, b, result));
        result
    }

    /// 获取计算历史
    #[doc(alias = "log")]
    #[doc(alias = "record")]
    pub fn get_history(&self) -> &[String] {
        &self.history
    }
}

/// 格式化数字
///
/// 将数字格式化为易读的字符串。
#[doc(alias = "format")]
#[doc(alias = "pretty_print")]
#[doc(alias = "display")]
pub fn format_number(num: i32) -> String {
    // 简单的千位分隔符格式化
    let num_str = num.to_string();
    let chars: Vec<char> = num_str.chars().collect();
    let mut result = String::new();

    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*ch);
    }

    result
}

/// 演示条件文档
///
/// 使用 `#[cfg_attr]` 可以根据编译条件生成不同的文档。
fn demonstrate_conditional_docs() {
    println!("\n--- 条件文档演示 ---");

    // 使用条件文档的结构体
    let platform_api = PlatformSpecificApi::new();
    let feature_api = FeatureGatedApi::new();

    println!("条件文档示例:");
    println!("  - PlatformSpecificApi: {:?}", platform_api);
    println!("  - FeatureGatedApi: {:?}", feature_api);

    // 演示平台特定功能
    platform_api.platform_method();
    feature_api.feature_method();
}

/// 平台特定的 API
///
/// 这个 API 在不同平台上有不同的文档。
#[cfg_attr(target_os = "windows", doc = "Windows 平台专用 API")]
#[cfg_attr(target_os = "macos", doc = "macOS 平台专用 API")]
#[cfg_attr(target_os = "linux", doc = "Linux 平台专用 API")]
#[cfg_attr(
    not(any(target_os = "windows", target_os = "macos", target_os = "linux")),
    doc = "通用平台 API"
)]
#[derive(Debug)]
pub struct PlatformSpecificApi {
    /// 平台相关数据
    data: String,
}

impl PlatformSpecificApi {
    /// 创建平台特定 API 实例
    pub fn new() -> Self {
        Self {
            data: "平台数据".to_string(),
        }
    }

    /// 平台特定方法
    #[cfg_attr(target_os = "windows", doc = "在 Windows 上执行特定操作")]
    #[cfg_attr(target_os = "macos", doc = "在 macOS 上执行特定操作")]
    #[cfg_attr(target_os = "linux", doc = "在 Linux 上执行特定操作")]
    #[cfg_attr(
        not(any(target_os = "windows", target_os = "macos", target_os = "linux")),
        doc = "执行通用操作"
    )]
    pub fn platform_method(&self) {
        #[cfg(target_os = "windows")]
        println!("  - 执行 Windows 特定操作");

        #[cfg(target_os = "macos")]
        println!("  - 执行 macOS 特定操作");

        #[cfg(target_os = "linux")]
        println!("  - 执行 Linux 特定操作");

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        println!("  - 执行通用操作");
    }
}

/// 功能门控的 API
///
/// 根据启用的功能显示不同的文档。
#[cfg_attr(feature = "advanced", doc = "高级功能已启用的 API")]
#[cfg_attr(not(feature = "advanced"), doc = "基础功能 API")]
#[derive(Debug)]
pub struct FeatureGatedApi {
    /// 功能相关数据
    data: HashMap<String, String>,
}

impl FeatureGatedApi {
    /// 创建功能门控 API 实例
    #[cfg_attr(feature = "advanced", doc = "创建高级功能实例")]
    #[cfg_attr(not(feature = "advanced"), doc = "创建基础功能实例")]
    pub fn new() -> Self {
        let mut data = HashMap::new();
        data.insert("type".to_string(), "feature_gated".to_string());

        #[cfg(feature = "advanced")]
        data.insert("level".to_string(), "advanced".to_string());

        #[cfg(not(feature = "advanced"))]
        data.insert("level".to_string(), "basic".to_string());

        Self { data }
    }

    /// 功能特定方法
    #[cfg_attr(feature = "advanced", doc = "执行高级功能操作")]
    #[cfg_attr(not(feature = "advanced"), doc = "执行基础功能操作")]
    pub fn feature_method(&self) {
        #[cfg(feature = "advanced")]
        println!("  - 执行高级功能操作");

        #[cfg(not(feature = "advanced"))]
        println!("  - 执行基础功能操作");
    }
}

/// 演示重导出控制
///
/// 控制重导出项目在文档中的显示方式。
fn demonstrate_reexport_control() {
    println!("\n--- 重导出控制演示 ---");

    // 使用重导出的类型
    let original = OriginalStruct::new();
    let reexported = ReexportedStruct::new();

    println!("重导出控制示例:");
    println!("  - OriginalStruct: {:?}", original);
    println!("  - ReexportedStruct: {:?}", reexported);
}

/// 原始结构体
///
/// 这是原始定义的结构体。
#[derive(Debug)]
pub struct OriginalStruct {
    /// 原始数据
    pub data: String,
}

impl OriginalStruct {
    /// 创建原始结构体实例
    pub fn new() -> Self {
        Self {
            data: "原始数据".to_string(),
        }
    }
}

// 重导出类型别名
/// 重导出的结构体类型
///
/// 这是 [`OriginalStruct`] 的重导出版本。
pub type ReexportedStruct = OriginalStruct;

// 模块重导出示例
pub mod reexports {
    //! 重导出模块
    //!
    //! 这个模块演示了如何控制重导出的文档显示。

    /// 内联重导出
    ///
    /// 这个重导出会内联显示完整文档。
    #[doc(inline)]
    pub use super::OriginalStruct as InlineReexport;

    /// 非内联重导出
    ///
    /// 这个重导出只显示链接。
    #[doc(no_inline)]
    pub use super::OriginalStruct as NoInlineReexport;

    /// 隐藏重导出
    ///
    /// 这个重导出在文档中不显示。
    #[doc(hidden)]
    pub use super::OriginalStruct as HiddenReexport;
}

/// 演示高级文档属性
///
/// 包括自定义属性和复杂的文档生成场景。
fn demonstrate_advanced_attributes() {
    println!("\n--- 高级文档属性演示 ---");

    // 使用高级属性的示例
    let advanced = AdvancedDocStruct::new();
    let versioned = VersionedApi::new();

    println!("高级文档属性示例:");
    println!("  - AdvancedDocStruct: {:?}", advanced);
    println!("  - VersionedApi: {:?}", versioned);

    // 演示动态文档
    advanced.demonstrate_method();
    versioned.version_specific_method();
}

/// 高级文档结构体
///
/// 演示复杂的文档属性组合。
#[doc = "# AdvancedDocStruct"]
#[doc = ""]
#[doc = concat!("构建时间: ", env!("CARGO_PKG_VERSION"))]
#[doc = "Rust 版本: 1.70+"]
#[doc = ""]
#[doc = "这个结构体演示了高级文档属性的使用。"]
#[doc(alias = "advanced")]
#[doc(alias = "complex")]
#[derive(Debug)]
pub struct AdvancedDocStruct {
    /// 配置映射
    #[doc = "存储配置信息的哈希映射"]
    #[doc(alias = "configuration")]
    #[doc(alias = "settings")]
    pub config: HashMap<String, String>,
}

impl AdvancedDocStruct {
    /// 创建高级文档结构体
    #[doc = "创建新的 AdvancedDocStruct 实例"]
    #[doc = ""]
    #[doc = "# Returns"]
    #[doc = ""]
    #[doc = "返回初始化的实例"]
    pub fn new() -> Self {
        let mut config = HashMap::new();
        config.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
        config.insert("name".to_string(), env!("CARGO_PKG_NAME").to_string());

        Self { config }
    }

    /// 演示方法
    #[doc = "演示高级文档功能的方法"]
    #[doc(alias = "demo")]
    #[doc(alias = "show")]
    pub fn demonstrate_method(&self) {
        println!("  - 高级文档结构体演示方法执行");
        println!("  - 配置项数量: {}", self.config.len());
    }
}

/// 版本化 API
///
/// 根据版本显示不同的文档。
#[cfg_attr(feature = "v1", doc = "API 版本 1.0")]
#[cfg_attr(feature = "v2", doc = "API 版本 2.0")]
#[cfg_attr(not(any(feature = "v1", feature = "v2")), doc = "默认 API 版本")]
#[derive(Debug)]
pub struct VersionedApi {
    /// 版本信息
    version: String,
}

impl VersionedApi {
    /// 创建版本化 API
    pub fn new() -> Self {
        let version = if cfg!(feature = "v2") {
            "2.0".to_string()
        } else if cfg!(feature = "v1") {
            "1.0".to_string()
        } else {
            "default".to_string()
        };

        Self { version }
    }

    /// 版本特定方法
    #[cfg_attr(feature = "v1", doc = "版本 1.0 的方法实现")]
    #[cfg_attr(feature = "v2", doc = "版本 2.0 的方法实现（增强功能）")]
    #[cfg_attr(not(any(feature = "v1", feature = "v2")), doc = "默认版本的方法实现")]
    pub fn version_specific_method(&self) {
        println!("  - 版本化 API 方法执行，版本: {}", self.version);
    }
}

/// 自定义文档属性宏示例
///
/// 演示如何创建自定义的文档生成宏。
macro_rules! doc_with_example {
    ($item:item, $example:expr) => {
        #[doc = concat!("# Example\n\n```rust\n", $example, "\n```")]
        $item
    };
}

// 使用自定义文档宏
doc_with_example! {
    /// 使用宏生成文档的函数
    ///
    /// 这个函数演示了如何使用宏来生成文档。
    pub fn macro_documented_function() -> String {
        "宏文档示例".to_string()
    },
    "let result = macro_documented_function();\nassert_eq!(result, \"宏文档示例\");"
}

/// 文档属性的最佳实践
///
/// 总结文档属性使用的最佳实践。
pub struct DocAttributesBestPractices;

impl DocAttributesBestPractices {
    /// 获取最佳实践列表
    ///
    /// 返回使用文档属性的最佳实践建议。
    pub fn get_best_practices() -> Vec<&'static str> {
        vec![
            "使用 #[doc(hidden)] 隐藏内部 API",
            "为公共 API 提供搜索别名",
            "使用条件文档支持多平台",
            "合理控制重导出的内联行为",
            "使用计算文档提供动态信息",
            "组合多个属性实现复杂文档",
            "保持文档属性的一致性",
            "测试文档在不同条件下的生成",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_attributes() {
        demonstrate_doc_attributes();
    }

    #[test]
    fn test_basic_doc_example() {
        let example = BasicDocExample::new("test");
        assert_eq!(example.data, "test");
    }

    #[test]
    fn test_math_calculator() {
        let mut calc = MathCalculator::new();
        assert_eq!(calc.add(2, 3), 5);
        assert_eq!(calc.multiply(4, 5), 20);
        assert_eq!(calc.get_history().len(), 2);
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(1234), "1,234");
        assert_eq!(format_number(1234567), "1,234,567");
    }

    #[test]
    fn test_platform_specific_api() {
        let api = PlatformSpecificApi::new();
        api.platform_method(); // 不会 panic
    }

    #[test]
    fn test_feature_gated_api() {
        let api = FeatureGatedApi::new();
        api.feature_method(); // 不会 panic
    }

    #[test]
    fn test_advanced_doc_struct() {
        let advanced = AdvancedDocStruct::new();
        assert!(!advanced.config.is_empty());
        advanced.demonstrate_method();
    }

    #[test]
    fn test_versioned_api() {
        let api = VersionedApi::new();
        api.version_specific_method();
    }

    #[test]
    fn test_macro_documented_function() {
        let result = macro_documented_function();
        assert_eq!(result, "宏文档示例");
    }

    #[test]
    fn test_best_practices() {
        let practices = DocAttributesBestPractices::get_best_practices();
        assert!(!practices.is_empty());
        assert!(practices.len() >= 8);
    }
}
