# Rust 学习项目 (rust-program-cha)

这是一个系统性的 Rust 编程语言学习项目，采用主题导向的目录结构，涵盖从基础概念到高级特性的完整学习路径。

## 🚀 快速开始

```bash
# 克隆项目
git clone <repository-url>
cd rust-program-cha

# 安装 Rust（如果尚未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 构建项目
cargo build

# 运行示例
cargo run --example <example-name>
```

## 📚 学习路径

### 🎯 推荐学习顺序

1. **[01-fundamentals](./01-fundamentals/)** - 基础概念
   - 基本语法、数据类型、函数、闭包

2. **[02-ownership-borrowing](./02-ownership-borrowing/)** - 所有权与借用
   - Rust 的核心特性：所有权、借用、生命周期

3. **[03-data-structures](./03-data-structures/)** - 数据结构
   - 复合类型、方法、模式匹配

4. **[04-advanced-concepts](./04-advanced-concepts/)** - 高级概念
   - 泛型、Trait、宏、错误处理、类型系统

5. **[05-concurrency-async](./05-concurrency-async/)** - 并发与异步
   - 多线程、异步编程、并发原语

6. **[06-memory-management](./06-memory-management/)** - 内存管理
   - 智能指针、Unsafe Rust、循环引用

7. **[07-performance-optimization](./07-performance-optimization/)** - 性能优化
   - 移动语义、生命周期优化、性能分析

8. **[08-practical-examples](./08-practical-examples/)** - 实际应用
   - 嵌入式开发、实际项目示例

## 📖 文档资源

- **[项目结构说明](./docs/project-structure.md)** - 详细的目录结构和命名规范
- **[Rust 学习完整指南](./docs/rust-learning-complete-guide.md)** - 全面的学习指南
- **[Rust 学习路线图](./docs/rust-learning-roadmap.md)** - 学习路径规划
- **[Rust 难点总结](./docs/rust-difficulty-summary.md)** - 常见难点和解决方案
- **[API 学习路线图](./docs/api-learning-roadmap.md)** - API 使用指南
- **[Rust 性能优化全面指南](./rust_performance_optimization_guide.md)** - 全面的性能优化技术总结

## 🛠️ 工具和配置

- **[工具脚本](./tools/)** - 开发辅助工具
- **[Rust 工具链配置](./rust-toolchain.toml)** - 项目工具链版本
- **[Cargo 配置](./Cargo.toml)** - 项目依赖和配置
- **[Cargo 本地配置](./.cargo/config.toml)** - 本地开发环境配置

## 📁 项目结构概览

```
rust-program-cha/
├── 01-fundamentals/           # 基础概念
│   ├── basic-concepts/        # 基础语法概念
│   ├── basic-type/            # 基本数据类型
│   ├── characters-booleans-unit-type/ # 字符、布尔和单元类型
│   ├── closure/               # 闭包
│   ├── function/              # 函数
│   └── statements-expressions/ # 语句与表达式
├── 02-ownership-borrowing/    # 所有权与借用
│   ├── ownership/             # 所有权
│   └── reference-borrowing/   # 引用与借用
├── 03-data-structures/        # 数据结构
│   ├── composite-type/        # 复合类型
│   ├── method/                # 方法
│   └── pattern-matching/      # 模式匹配
├── 04-advanced-concepts/      # 高级概念
│   ├── advanced-lifetime/     # 高级生命周期
│   ├── collections/           # 集合类型
│   ├── comments-documentation/ # 注释与文档
│   ├── crate-package/         # 包与模块
│   ├── deep-trait/            # Trait深入
│   ├── error-handling/        # 错误处理
│   ├── formatted-output/      # 格式化输出
│   ├── functional-programming/ # 函数式编程
│   ├── generics-traits/       # 泛型与Trait
│   ├── global-variables/      # 全局变量
│   ├── lifetime/              # 生命周期
│   ├── macro-programming/     # 宏编程
│   ├── returnValues-errorHandling/ # 返回值与错误处理
│   └── type-system/           # 类型系统
├── 05-concurrency-async/      # 并发与异步
│   ├── async-programming/     # 异步编程
│   └── multithreading/        # 多线程
├── 06-memory-management/      # 内存管理
│   ├── cyclic-references/     # 循环引用
│   ├── smart-pointers/        # 智能指针
│   └── unsafe-rust/           # 不安全Rust
├── 07-performance-optimization/ # 性能优化
│   ├── eq-partial-eq/         # 相等性比较
│   ├── move-copy-clone/       # 移动、复制与克隆
│   ├── raw-pointers-references-smart-pointers/ # 指针比较
│   ├── scope-lifetime-nll/    # 作用域与生命周期
│   └── slices-slice-references/ # 切片与切片引用
├── 08-practical-examples/     # 实际应用
│   └── embedded-development/  # 嵌入式开发
├── docs/                      # 文档资源
├── tools/                     # 工具脚本
├── .cargo/                    # Cargo配置
├── Cargo.toml                 # 项目依赖配置
├── rust-toolchain.toml        # Rust工具链配置
├── rust_performance_optimization_guide.md # Rust性能优化全面指南
```

## 🎯 学习目标

通过本项目，你将掌握：

- ✅ Rust 基础语法和核心概念
- ✅ 所有权系统和内存安全
- ✅ 高级类型系统和泛型编程
- ✅ 并发和异步编程模式
- ✅ 性能优化技巧
- ✅ 实际项目开发经验

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request 来改进这个学习项目！

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🔗 相关资源

- [Rust 官方文档](https://doc.rust-lang.org/)
- [Rust 程序设计语言](https://doc.rust-lang.org/book/)
- [Rust 标准库文档](https://doc.rust-lang.org/std/)
- [Rust 参考手册](https://doc.rust-lang.org/reference/)

---

**开始你的 Rust 学习之旅吧！** 🦀