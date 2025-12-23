# Rust 学习项目 (rust-program-cha)

<div align="center">
  <img src="https://www.rust-lang.org/static/images/rust-logo-blk.svg" alt="Rust Logo" width="200" height="200" />

<div style="margin-top: 20px;">
    <a href="https://github.com/tyone/rust-program-cha"><img src="https://img.shields.io/github/stars/tyone/rust-program-cha?style=social" alt="GitHub stars" /></a>
    <a href="https://github.com/tyone/rust-program-cha"><img src="https://img.shields.io/github/forks/tyone/rust-program-cha?style=social" alt="GitHub forks" /></a>
    <a href="https://github.com/tyone/rust-program-cha"><img src="https://img.shields.io/github/issues/tyone/rust-program-cha" alt="GitHub issues" /></a>
    <a href="https://github.com/tyone/rust-program-cha/blob/main/LICENSE"><img src="https://img.shields.io/github/license/tyone/rust-program-cha" alt="License" /></a>
    <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-stable-blue" alt="Rust stable" /></a>
  </div>
</div>

<p align="center">
  <strong>🚀 一站式 Rust 学习与实践平台，从零基础到高级开发的完整路径</strong><br>
  <span>💡 系统化主题导向学习 | 🛠️ 丰富实战示例 | 📚 深度技术解析 | 🎯 专注内存安全与性能优化</span>
</p>

## 📝 项目简介

这是一个全面、系统的 Rust 编程语言学习项目，专为想要深入掌握 Rust 核心概念和实战技能的开发者设计。项目采用渐进式学习路径，从基础语法到高级特性，从内存安全模型到异步并发编程，提供了完整的知识体系和实践指导。

### ✨ 核心优势

- **🚀 渐进式学习路径**：精心设计的从入门到精通的学习曲线
- **🔒 深入内存安全**：详解 Rust 独特的所有权、借用和生命周期系统
- **⚡ 性能优化指南**：提供全面的性能调优技巧和最佳实践
- **🔧 实战驱动**：每个概念都配有可运行的示例代码和练习
- **📚 详尽文档**：深入解析 Rust 难点概念和高级特性
- **🌐 社区支持**：活跃的学习社区和持续更新的内容

## 🚀 快速开始

### 🔧 环境要求

- **操作系统**: Windows, macOS, Linux
- **Rust 版本**: 稳定版 (stable)
- **工具链**: 通过 rustup 管理

### 📋 安装步骤

```bash
# 1. 克隆项目
git clone https://github.com/tyone/rust-program-cha.git
cd rust-program-cha

# 2. 安装 Rust (如果尚未安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 3. 验证安装
rustc --version  # 验证 Rust 版本
cargo --version  # 验证 Cargo 版本

# 4. 构建和运行特定示例
cargo build           # 构建项目
cargo run --example <example-name>  # 运行特定示例

# 5. 运行特定子项目
cd 01-fundamentals/basic-concepts
cargo run
```

### 🎮 快速学习路线

```
🏁 起点 → 📚 基础概念(01-fundamentals) → 🔐 所有权与借用(02-ownership-borrowing) → 📊 数据结构(03-data-structures)
      → 🚀 高级概念(04-advanced-concepts) → ⚡ 并发与异步(05-concurrency-async) → 🔧 内存管理(06-memory-management)
      → 📈 性能优化(07-performance-optimization) → 🏆 精通 Rust
```

## 📁 详细项目结构与学习内容

### 1. 基础概念 (01-fundamentals/)

Rust 学习的起点，涵盖编程语言的基础要素。

- **basic-concepts**: 变量、可变性、基本语法
- **basic-type**: 数值、浮点数、字符等基本类型
- **function**: 函数定义、参数传递与返回值
- **closure**: 闭包概念与函数式编程基础
- **statements-expressions**: 语句与表达式的区别

### 2. 所有权与借用 (02-ownership-borrowing/)

理解 Rust 内存安全模型的核心。

- **ownership**: 所有权三大规则、移动语义
- **reference-borrowing**: 引用、借用规则、切片引用

### 3. 数据结构 (03-data-structures/)

Rust 的复合类型与模式匹配机制。

- **composite-type**:
    - **structure**: 结构体定义与方法
    - **enumeration**: 枚举类型与变体
    - **array/tuple**: 数组与元组
    - **string-slice**: 字符串与切片操作
- **pattern-matching**: 模式匹配基础与进阶、解构 Option

### 4. 高级概念 (04-advanced-concepts/)

深入掌握 Rust 的高级特性与抽象能力。

- **generics-traits**: 泛型编程与 Trait 系统
- **error-handling**: Result/Option 错误处理模式
- **lifetime**: 生命周期注解与高级生命周期
- **macro-programming**: 声明宏与过程宏
- **response-macro 生态**:
    - **response-macro**: 自定义过程宏实现，用于简化 API 响应构建
    - **response-macro-core**: 核心类型定义与 trait
    - **response-macro-advanced**: 包含 RESTful API 示例的高级应用
- **deep-trait**: 深入理解 Trait 对象、关联类型等
- **functional-programming**: 迭代器与高阶函数

### 5. 并发与异步 (05-concurrency-async/)

掌握 Rust 的并行编程能力。

- **multithreading**:
    - 线程创建与管理
    - **message-passing**: 通道(Channel)通信模式
    - **Lock-Condvar-Semaphore**: 互斥锁与同步原语
    - **Thread-safety**: Send 与 Sync trait
- **async-programming**:
    - **Async/Await**: 异步编程语法
    - **Pin-Unpin**: Pin 机制详解
    - **multipleFutures**: 多 Future 组合与执行

### 6. 内存管理 (06-memory-management/)

深入底层内存机制与不安全代码。

- **smart-pointers**: Box、Rc、Arc、Cell、RefCell 详解
- **cyclic-references**: 循环引用问题与 Weak 指针
- **unsafe-rust**: Unsafe 代码块、原始指针与 FFI

### 7. 性能优化 (07-performance-optimization/)

编写高性能 Rust 代码的技巧。

- **move-copy-clone**: 移动、复制与克隆的性能权衡
- **scope-lifetime-nll**: 作用域优化与 NLL (Non-Lexical Lifetimes)
- **slices-slice-references**: 高效利用切片减少内存复制
- **eq-partial-eq**: 高效的相等性比较实现

## 📖 文档资源

- **[项目结构说明](./docs/project-structure.md)** - 详细说明项目组织和目录结构
- **[Rust 学习完整指南](./docs/rust-learning-complete-guide.md)** - 系统化的 Rust 学习方法论
- **[Rust 学习路线图](./docs/rust-learning-roadmap.md)** - 阶段性学习目标和时间规划
- **[Rust 难点总结](./docs/rust-difficulty-summary.md)** - 重点难点概念解析和常见问题
- **[API 学习路线图](./docs/api-learning-roadmap.md)** - Rust 生态系统 API 学习指南
- **[Mise 配置指南](./docs/mise-setup-guide.md)** - 多工具版本管理配置

## 🛠️ 工具和配置

- **[工具脚本](./tools/)** - 开发辅助工具和实用脚本
- **[Rust 工具链配置](./rust-toolchain.toml)** - 项目使用的 Rust 版本规范
- **[Cargo 配置](./Cargo.toml)** - 主项目依赖和元数据

## 🎯 学习目标

### 基础能力
- **语言基础**：掌握 Rust 语法规则、基本数据类型、控制流和函数定义
- **核心概念**：深入理解所有权、借用、生命周期等 Rust 独特的内存管理机制

### 进阶能力
- **并发与异步**：掌握 Rust 的并发模型、线程安全和现代异步编程模式
- **系统编程**：学习使用 Rust 进行底层系统级编程和硬件交互
- **Web 开发**：使用 Rust 开发高性能的 Web 应用和 API 服务

### 质量保证
- **代码质量**：编写安全、高效、可维护的生产级 Rust 代码
- **性能优化**：掌握 Rust 性能调优和内存优化技术

## 🤝 贡献指南

我们欢迎社区贡献来改进和扩展这个学习项目！

1. **报告问题** - 通过 GitHub Issues 报告错误或提出改进建议
2. **提交代码** - Fork 本仓库 -> 创建分支 -> 提交更改 -> 推送 -> 提交 PR

### 代码规范
- 遵循 Rust 官方编码规范
- 使用 `rustfmt` 格式化代码
- 运行 `clippy` 检查潜在问题

## 📄 许可证

本项目采用 [MIT License](LICENSE) 许可证。文档资源采用 CC BY-NC-SA 4.0 协议。

---

<p align="center">
  <i>"Rust: 一门赋予每个人构建可靠且高效软件能力的语言。"</i>
  <br>
  <i>安全、并发、高效 — 选择 Rust，选择未来！</i>
</p>
