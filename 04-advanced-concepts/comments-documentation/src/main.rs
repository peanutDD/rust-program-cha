// 示例代码库，允许未使用的导入和代码（用于教学演示）
#![allow(unused_imports)]
#![allow(dead_code)]

//! # Rust 注释和文档系统全面分析
//!
//! 本项目基于 https://course.rs/basic/comment.html 内容，
//! 全面深入分析 Rust 中的注释和文档系统。
//!
//! ## 项目结构
//! - `comment_basics.rs` - 注释基础知识
//! - `doc_comments.rs` - 文档注释详解
//! - `doc_attributes.rs` - 文档属性系统
//! - `rustdoc_features.rs` - rustdoc 工具特性
//! - `doc_tests.rs` - 文档测试机制
//! - `advanced_doc.rs` - 高级文档特性
//! - `best_practices.rs` - 文档最佳实践
//! - `practical_examples.rs` - 实际应用案例

mod advanced_doc;
mod best_practices;
mod comment_basics;
mod doc_attributes;
mod doc_comments;
mod doc_tests;
mod practical_examples;
mod rustdoc_features;

fn main() {
    println!("=== Rust 注释和文档系统全面分析 ===");

    // 演示各个模块的功能
    comment_basics::demonstrate_comment_basics();
    doc_comments::demonstrate_doc_comments();
    doc_attributes::demonstrate_doc_attributes();
    rustdoc_features::demonstrate_rustdoc_features();
    doc_tests::demonstrate_doc_tests();
    advanced_doc::demonstrate_advanced_doc();
    best_practices::demonstrate_best_practices();
    practical_examples::demonstrate_practical_examples();

    println!("\n=== 分析完成 ===");
}
