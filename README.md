# Rust 学习项目 (rust-program-cha)

<p align="center">
  <a href="https://github.com/tyone/rust-program-cha"><img src="https://img.shields.io/github/stars/tyone/rust-program-cha?style=social" alt="GitHub stars"></a>
  <a href="https://github.com/tyone/rust-program-cha"><img src="https://img.shields.io/github/forks/tyone/rust-program-cha?style=social" alt="GitHub forks"></a>
  <a href="https://github.com/tyone/rust-program-cha"><img src="https://img.shields.io/github/issues/tyone/rust-program-cha" alt="GitHub issues"></a>
  <a href="https://github.com/tyone/rust-program-cha/blob/main/LICENSE"><img src="https://img.shields.io/github/license/tyone/rust-program-cha" alt="License"></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-stable-blue" alt="Rust stable"></a>
</p>

<p align="center">系统性 Rust 编程语言学习项目，采用主题导向目录结构，提供从基础到高级特性的完整学习路径和实践示例。</p>

## 🚀 快速开始

### 环境要求

- **操作系统**: Windows, macOS, Linux
- **Rust 版本**: 稳定版 (stable)
- **工具链**: 通过 rustup 管理

### 安装步骤

```bash
# 1. 克隆项目
git clone https://github.com/tyone/rust-program-cha.git
cd rust-program-cha

# 2. 安装 Rust (如果尚未安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 3. 验证安装
rustc --version
cargo --version

# 4. 构建和运行特定示例
cargo build
cargo run --example <example-name>

# 5. 运行特定子项目
cd 01-fundamentals/basic-concepts
cargo run
```

## 📚 学习路径

### 推荐学习顺序

1. **[01-fundamentals](./01-fundamentals/)** - 基础概念
   - 掌握 Rust 基础语法、数据类型系统、函数定义和闭包使用
   - 学习 Rust 的语句和表达式区别，构建基础编程能力

2. **[02-ownership-borrowing](./02-ownership-borrowing/)** - 所有权与借用
   - 理解 Rust 独特的所有权系统，这是 Rust 内存安全的核心
   - 掌握借用规则和生命周期概念，避免常见的内存错误

3. **[03-data-structures](./03-data-structures/)** - 数据结构
   - 学习 Rust 复合类型（结构体、枚举、元组）的使用
   - 掌握模式匹配和流程控制，编写更加优雅的代码

4. **[04-advanced-concepts](./04-advanced-concepts/)** - 高级概念
   - 深入学习泛型、Trait 系统、宏编程和错误处理
   - 掌握高级生命周期、类型系统和函数式编程特性

5. **[05-concurrency-async](./05-concurrency-async/)** - 并发与异步
   - 学习 Rust 多线程编程和消息传递模式
   - 掌握异步编程基础、Pin-Unpin 特性和多 Future 处理

6. **[06-memory-management](./06-memory-management/)** - 内存管理
   - 深入理解智能指针（Box、Rc、Arc、Cell、RefCell）
   - 了解 unsafe Rust 的使用场景和安全边界

7. **[07-performance-optimization](./07-performance-optimization/)** - 性能优化
   - 学习 Rust 性能优化技巧和最佳实践
   - 掌握移动语义、复制和克隆的性能影响

8. **[08-practical-examples](./08-practical-examples/)** - 实际应用
   - 通过实际项目学习 Rust 的应用
   - 了解 Rust 在嵌入式开发等领域的应用场景

## 📖 文档资源

- **[项目结构说明](./docs/project-structure.md)** - 详细说明项目组织和目录结构
- **[Rust 学习完整指南](./docs/rust-learning-complete-guide.md)** - 系统化的 Rust 学习方法论
- **[Rust 学习路线图](./docs/rust-learning-roadmap.md)** - 阶段性学习目标和时间规划
- **[Rust 难点总结](./docs/rust-difficulty-summary.md)** - 重点难点概念解析和常见问题
- **[API 学习路线图](./docs/api-learning-roadmap.md)** - Rust 生态系统 API 学习指南
- **[Rust 性能优化全面指南](./rust_performance_optimization_guide.md)** - 性能调优技巧和最佳实践

## 🛠️ 工具和配置

- **[工具脚本](./tools/)** - 开发辅助工具和实用脚本
- **[Rust 工具链配置](./rust-toolchain.toml)** - 项目使用的 Rust 版本规范
- **[Cargo 配置](./Cargo.toml)** - 主项目依赖和元数据
- **[Cargo 本地配置](./.cargo/config.toml)** - 开发环境和构建优化配置
- **[Mise 版本管理](./docs/mise-setup-guide.md)** - 多工具版本管理配置指南

## 📁 详细项目结构

### 核心学习模块

#### 1. 基础概念 (01-fundamentals/)
```
01-fundamentals/
├── basic-concepts/          # 基础语法概念
├── basic-type/              # 基本数据类型
├── characters-booleans-unit-type/ # 字符、布尔值和单元类型
├── closure/                 # 闭包函数
├── function/                # 函数定义和使用
└── statements-expressions/  # 语句和表达式
```

#### 2. 所有权与借用 (02-ownership-borrowing/)
```
02-ownership-borrowing/
├── ownership/               # 所有权概念
└── reference-borrowing/     # 引用与借用规则
```

#### 3. 数据结构 (03-data-structures/)
```
03-data-structures/
├── composite-type/          # 复合类型
│   ├── array/               # 数组
│   ├── enumeration/         # 枚举类型
│   ├── match-iflet/         # 模式匹配
│   ├── process-control/     # 流程控制
│   ├── string-slice/        # 字符串和切片
│   ├── structure/           # 结构体
│   └── tuple/               # 元组
├── method/                  # 方法实现
└── pattern-matching/        # 模式匹配进阶
    ├── deconstructing-option/ # Option解构
    └── full-pattern-list/  # 完整模式列表
```

#### 4. 高级概念 (04-advanced-concepts/)
```
04-advanced-concepts/
├── advanced-lifetime/       # 高级生命周期
├── collections/             # 集合类型
│   ├── dynamic-array-vector/ # 动态数组Vector
│   └── kv-storage-hashMap/  # 键值存储HashMap
├── comments-documentation/  # 注释和文档
├── crate-package/           # 包和模块
├── deep-trait/              # Trait深入理解
├── error-handling/          # 错误处理
├── formatted-output/        # 格式化输出
├── functional-programming/  # 函数式编程
├── generics-traits/         # 泛型和Trait
├── global-variables/        # 全局变量
├── lifetime/                # 生命周期
├── macro-programming/       # 宏编程
├── response-macro/          # 响应宏
├── response-macro-advanced/ # 高级响应宏
├── response-macro-example/  # 响应宏示例
├── returnValues-errorHandling/ # 返回值与错误处理
└── type-system/             # 类型系统
```

#### 5. 并发与异步 (05-concurrency-async/)
```
05-concurrency-async/
├── async-programming/       # 异步编程
│   ├── Async/               # 异步基础
│   ├── Pin-Unpin/           # Pin与Unpin特性
│   └── multipleFutures/     # 多Future处理
└── multithreading/          # 多线程编程
    └── message-passing/     # 消息传递模式
```

#### 6. 内存管理 (06-memory-management/)
```
06-memory-management/
├── cyclic-references/       # 循环引用
├── smart-pointers/          # 智能指针
│   ├── Box-T/               # Box<T>指针
│   ├── Cell-RefCell/        # Cell和RefCell
│   ├── Deref/               # Deref特性
│   ├── Drop/                # Drop特性
│   └── Rc-Arc/              # Rc和Arc引用计数
└── unsafe-rust/             # 不安全Rust
    ├── inline-assembly/     # 内联汇编
    ├── unsafe-rust/         # 不安全代码
    └── unsafe-superpowers/  # 不安全特性
```

#### 7. 性能优化 (07-performance-optimization/)
```
07-performance-optimization/
├── eq-partial-eq/           # 相等性比较
├── move-copy-clone/         # 移动、复制和克隆
├── scope-lifetime-nll/      # 作用域、生命周期和NLL
└── slices-slice-references/ # 切片和切片引用
```

#### 8. 实际应用 (08-practical-examples/)
```
08-practical-examples/
└── embedded-development/    # 嵌入式开发示例
```

### 文档和工具
```
rust-program-cha/
├── docs/                    # 文档资源
│   ├── api-learning-roadmap.md       # API学习路线图
│   ├── mise-setup-guide.md           # Mise配置指南
│   ├── project-structure.md          # 项目结构说明
│   ├── rust-difficulty-examples/     # Rust难点示例
│   ├── rust-difficulty-summary.md    # Rust难点总结
│   ├── rust-learning-complete-guide.md # Rust完整学习指南
│   ├── rust-learning-roadmap.md      # Rust学习路线图
│   └── system_design_topic_map.md    # 系统设计主题图
├── tools/                   # 工具脚本
│   └── restart-rust-analyzer.sh     # 重启Rust Analyzer脚本
├── .cargo/                  # Cargo配置
├── .gitignore               # Git忽略文件
├── .tool-versions           # 工具版本配置
├── Cargo.toml               # 主Cargo配置
├── rust-toolchain.toml      # Rust工具链配置
├── rust_performance_optimization_guide.md # 性能优化指南
└── README.md                # 项目说明文档
```

## 🎯 学习目标

- ✅ Rust 基础语法和核心概念
- ✅ 所有权系统和内存安全
- ✅ 高级类型系统和泛型编程
- ✅ 并发和异步编程
- ✅ 性能优化技巧
- ✅ 实际项目开发经验

## 🌟 项目特点

- **系统化学习路径** - 从基础到高级的渐进式学习体系
- **实践驱动** - 每个概念都配有实际示例和练习
- **详细文档** - 丰富的文档资源和学习指南
- **模块化设计** - 主题明确的目录结构，易于导航和学习
- **最新工具链** - 基于最新稳定版 Rust 工具链构建

## 🤝 贡献指南

我们欢迎社区贡献来改进和扩展这个学习项目！

### 贡献方式

1. **报告问题** - 通过 GitHub Issues 报告错误或提出改进建议
2. **提交代码** - 遵循以下步骤：
   - Fork 本仓库
   - 创建功能分支 (`git checkout -b feature/amazing-feature`)
   - 提交更改 (`git commit -m 'Add some amazing feature'`)
   - 推送到分支 (`git push origin feature/amazing-feature`)
   - 打开 Pull Request

### 代码规范

- 遵循 Rust 官方风格指南
- 确保所有代码通过 `cargo fmt` 和 `cargo clippy` 检查
- 为新功能添加适当的文档注释
- 包含测试用例验证功能正确性

## 📄 许可证

本项目采用 **MIT 许可证** - 详见 [LICENSE](LICENSE) 文件。

## 🔗 相关资源

### 官方文档

- [Rust 官方文档](https://doc.rust-lang.org/) - 全面的 Rust 语言参考
- [Rust 程序设计语言](https://doc.rust-lang.org/book/) - 官方入门书籍
- [Rust 标准库文档](https://doc.rust-lang.org/std/) - 标准库 API 参考
- [Rust 参考手册](https://doc.rust-lang.org/reference/) - 语言技术细节

### 在线学习资源

- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - 通过示例学习 Rust
- [Rustlings](https://github.com/rust-lang/rustlings) - 小练习集合
- [Rust 进阶指南](https://doc.rust-lang.org/nightly/book/second-edition/) - 深度主题讲解

### 社区资源

- [Rust 论坛](https://users.rust-lang.org/) - 官方社区论坛
- [Rust Discord](https://discord.gg/rust-lang) - 实时讨论社区
- [Rust 中文社区](https://rustcc.cn/) - 中文资源和讨论

---

**开始你的 Rust 学习之旅！** 🦀

<p align="center">
  <i>"Rust: 一门赋予每个人构建可靠且高效软件能力的语言。"</i>
</p>