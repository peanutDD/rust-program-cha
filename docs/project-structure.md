# Rust 学习项目结构说明

本项目已重新组织，采用主题导向的目录结构，便于系统性学习 Rust 编程语言。

## 目录结构

### 📁 01-fundamentals (基础概念)
- `basic-concepts/` - 基本概念和语法
- `closure/` - 闭包概念
- `first/` - 入门示例

### 📁 02-ownership-borrowing (所有权与借用)
- `ownership/` - 所有权机制
- `reference-borrowing/` - 引用和借用

### 📁 03-data-structures (数据结构)
- `composite-type/` - 复合类型
- `method/` - 方法定义
- `pattern-matching/` - 模式匹配

### 📁 04-advanced-concepts (高级概念)
- `advanced-lifetime/` - 高级生命周期
- `collections/` - 集合类型
- `comments-documentation/` - 注释和文档
- `crate-package/` - 包和模块
- `deep-trait/` - 深入 Trait
- `error-handling/` - 错误处理
- `formatted-output/` - 格式化输出
- `functional-programming/` - 函数式编程
- `generics-traits/` - 泛型和 Trait
- `global-variables/` - 全局变量
- `lisftime/` - 生命周期
- `macro-programming/` - 宏编程
- `returnValues-errorHandling/` - 返回值和错误处理
- `type-system/` - 类型系统

### 📁 05-concurrency-async (并发与异步)
- `async-programming/` - 异步编程
- `multithreading/` - 多线程编程

### 📁 06-memory-management (内存管理)
- `cyclic-references/` - 循环引用
- `smart-pointers/` - 智能指针
- `unsafe-rust/` - Unsafe Rust

### 📁 07-performance-optimization (性能优化)
- `eq-partial-eq/` - 相等性比较
- `move-copy-clone/` - 移动、复制和克隆
- `raw-pointers-references-smart-pointers/` - 裸指针、引用和智能指针
- `scope-lifetime-nll/` - 作用域、生命周期和 NLL
- `slices-slice-references/` - 切片和切片引用

### 📁 08-practical-examples (实际应用示例)
- `embedded-development/` - 嵌入式开发

### 📁 docs (文档)
- `api-learning-roadmap.md` - API 学习路线图
- `rust-difficulty-examples.rs` - Rust 难点代码示例
- `rust-difficulty-summary.md` - Rust 难点总结
- `rust-learning-complete-guide.md` - Rust 学习完整指南
- `rust-learning-roadmap.md` - Rust 学习路线图
- `system_design_topic_map.md` - 系统设计主题图
- `project-structure.md` - 项目结构说明（本文档）

### 📁 tools (工具)
- `restart-rust-analyzer.sh` - 重启 rust-analyzer 脚本

### 📁 archived (归档)
- 原有的按天数组织的课程目录（已归档）

## 命名规范

- **目录命名**: 使用 kebab-case 格式（小写字母，单词间用连字符分隔）
- **文件命名**: 使用 kebab-case 格式
- **语义明确**: 目录和文件名能清晰表达其内容和用途

## 学习建议

建议按照以下顺序学习：

1. **01-fundamentals** - 掌握基础语法和概念
2. **02-ownership-borrowing** - 理解 Rust 的核心特性
3. **03-data-structures** - 学习数据结构和模式匹配
4. **04-advanced-concepts** - 深入高级特性
5. **05-concurrency-async** - 掌握并发和异步编程
6. **06-memory-management** - 理解内存管理机制
7. **07-performance-optimization** - 学习性能优化技巧
8. **08-practical-examples** - 实践应用开发

## 更新日期

最后更新：2024年1月