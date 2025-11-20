//! Rust 特征 (Trait) 进阶分析模块
//! 
//! 这个库提供了 Rust 语言中特征系统的深入分析和实用技术，包括：
//! - 类型别名与特征对象
//! - 默认泛型参数
//! - 完全限定语法
//! - 超特征
//! - Newtype 模式

/// 类型别名与特征对象相关功能
pub mod type_aliases;

/// 默认泛型参数相关功能
pub mod default_generics;

/// 完全限定语法相关功能
pub mod qualified_syntax;

/// 超特征相关功能
pub mod supertraits;

/// Newtype 模式相关功能
pub mod newtype;