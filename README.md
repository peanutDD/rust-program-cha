# Rust 学习项目 (rust-program-cha)

<p align="center">
  <a href="https://github.com/tyone/rust-program-cha"><img src="https://img.shields.io/github/stars/tyone/rust-program-cha?style=social" alt="GitHub stars"></a>
  <a href="https://github.com/tyone/rust-program-cha"><img src="https://img.shields.io/github/forks/tyone/rust-program-cha?style=social" alt="GitHub forks"></a>
  <a href="https://github.com/tyone/rust-program-cha"><img src="https://img.shields.io/github/issues/tyone/rust-program-cha" alt="GitHub issues"></a>
  <a href="https://github.com/tyone/rust-program-cha/blob/main/LICENSE"><img src="https://img.shields.io/github/license/tyone/rust-program-cha" alt="License"></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-stable-blue" alt="Rust stable"></a>
</p>

<p align="center">系统性 Rust 编程语言学习项目，采用主题导向目录结构，提供从基础到高级特性的完整学习路径和实践示例。包含最新的Rust特性解析、内存安全模型、unsafe Rust使用指南以及性能优化技巧。</p>

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
   - 掌握 unsafe Rust 的使用场景、安全边界和未定义行为
   - 学习内存布局、循环引用处理和零成本抽象

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

**模块说明：**

基础概念模块是Rust学习的起点，涵盖了编程语言的基础要素，包括语法规则、基本数据类型、函数定义和闭包使用等核心概念。

**重点子模块：**

##### basic-concepts
包含Rust最基础的语法概念和核心知识点，为后续学习奠定基础。

##### closure
详细讲解Rust闭包的概念、语法和使用场景，展示函数式编程的基本特性。

##### function
系统介绍Rust函数的定义、参数传递、返回值和函数签名等概念。

#### 2. 所有权与借用 (02-ownership-borrowing/)
```
02-ownership-borrowing/
├── ownership/               # 所有权概念
└── reference-borrowing/     # 引用与借用规则
```

**模块说明：**

所有权与借用模块是理解Rust内存安全模型的关键，介绍了Rust独特的所有权系统、借用规则和生命周期概念，这是Rust区别于其他语言的核心特性。

**重点子模块：**

##### ownership
深入讲解Rust所有权的三大规则、移动语义和作用域概念，这是Rust内存安全的基础。

##### reference-borrowing
详细介绍Rust的引用和借用规则，包括可变借用、不可变借用以及借用检查器的工作原理。

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

**模块说明：**

数据结构模块介绍Rust的复合数据类型和模式匹配功能，包括结构体、枚举、数组、字符串和切片等，以及如何通过模式匹配优雅地处理这些数据类型。

**重点子模块：**

##### composite-type
讲解Rust的各种复合数据类型，是构建复杂数据模型的基础。

##### pattern-matching
深入介绍Rust强大的模式匹配功能，包括解构、匹配守卫和多种匹配模式。

##### method
讲解如何为结构体和枚举实现方法，封装数据和行为。

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
├── response-macro-advanced/ # 高级响应宏（含RESTful API示例）
├── response-macro-example/  # 响应宏示例
├── returnValues-errorHandling/ # 返回值与错误处理
└── type-system/             # 类型系统
```

**模块说明：**

高级概念模块深入讲解Rust的核心高级特性，包括泛型、trait、生命周期、宏编程和错误处理等。

**重点子模块：**

##### response-macro-advanced

一个基于Actix-web的RESTful API服务示例，展示了如何构建具有完善错误处理、分层架构和响应格式化的后端应用。

**主要功能：**
- 完整的用户管理CRUD操作
- 批量用户查询功能
- 用户状态管理（激活/停用）
- 邮箱唯一性检查
- 基于角色的权限控制
- 统一的错误处理和响应格式

**技术栈：**
- Actix-web：异步Web框架
- Serde：序列化/反序列化
- Thiserror：错误处理
- Arc：线程安全的引用计数

**项目架构：**
- `models/`：数据模型和DTO定义
- `handlers/`：HTTP请求处理器
- `services/`：业务逻辑层
- `main.rs`：应用程序入口和路由配置

**API端点：**
- `GET /api/users`：获取用户列表（支持分页和筛选）
- `POST /api/users`：创建新用户
- `GET /api/users/{id}`：获取单个用户信息
- `PUT /api/users/{id}`：更新用户信息
- `DELETE /api/users/{id}`：删除用户
- `POST /api/users/batch`：批量获取用户
- `POST /api/users/{id}/activate`：激活用户
- `POST /api/users/{id}/deactivate`：停用用户
- `GET /api/users/check-email`：检查邮箱是否已被使用

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

**模块说明：**

并发与异步模块介绍Rust的并行编程能力，包括多线程编程和异步编程两种范式，展示了Rust在高性能并发系统开发中的优势。

**重点子模块：**

##### async-programming
讲解Rust的异步编程模型，包括async/await语法、Future trait、Pin-Unpin特性以及多Future组合等高级特性。

##### multithreading
介绍Rust的多线程编程，重点讲解基于消息传递的并发模式，避免共享状态带来的并发问题。

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

**模块说明：**

内存管理模块深入讲解Rust的内存管理机制，包括智能指针、内存布局、循环引用处理以及unsafe Rust的使用场景，帮助开发者更深入地理解Rust的内存安全模型。

**重点子模块：**

##### smart-pointers
详细介绍Rust的各种智能指针类型，包括Box<T>、Rc/Arc、Cell/RefCell等，以及它们的适用场景、内存语义和使用方法。

##### unsafe-rust
深入讲解unsafe Rust的概念、语法和使用场景，包括五种unsafe超能力（原始指针解引用、调用unsafe函数、访问可变静态变量、实现unsafe trait、访问union字段）。详细说明未定义行为的危险性和避免方法，以及如何安全地使用unsafe代码构建抽象接口。

##### cyclic-references
分析Rust中循环引用的问题及解决方案，包括使用Weak指针和设计模式等方法。

#### 7. 性能优化 (07-performance-optimization/)
```
07-performance-optimization/
├── eq-partial-eq/           # 相等性比较
├── move-copy-clone/         # 移动、复制和克隆
├── scope-lifetime-nll/      # 作用域、生命周期和NLL
└── slices-slice-references/ # 切片和切片引用
```

**模块说明：**

性能优化模块聚焦于Rust程序的性能调优技巧，包括内存布局优化、移动语义、借用规则优化等内容，帮助开发者编写高性能的Rust应用。

**重点子模块：**

##### move-copy-clone
深入分析Rust的移动语义、复制和克隆操作的性能影响，学习如何优化数据传递效率。

##### scope-lifetime-nll
讲解Rust的作用域、生命周期和非词法生命周期(NLL)特性，以及如何利用这些特性优化代码性能。

##### slices-slice-references
介绍切片和切片引用的高效使用方法，避免不必要的数据复制。

#### 8. 实际应用 (08-practical-examples/)
```
08-practical-examples/
└── embedded-development/    # 嵌入式开发示例
```

**模块说明：**

实际应用模块展示了Rust在不同领域的应用案例，包括嵌入式开发等，帮助开发者了解Rust的实际应用场景和最佳实践。

**重点子模块：**

##### embedded-development
展示Rust在嵌入式系统开发中的应用，包括硬件访问、内存管理和实时性能等方面的示例。

### 文档和工具

#### 文档资源

```
docs/
├── api-learning-roadmap.md       # API学习路线图
├── mise-setup-guide.md           # Mise配置指南
├── project-structure.md          # 项目结构说明
├── rust-difficulty-examples/     # Rust难点示例代码
├── rust-difficulty-summary.md    # Rust难点概念总结
├── rust-learning-complete-guide.md # Rust完整学习指南
├── rust-learning-roadmap.md      # Rust学习路线图
└── system_design_topic_map.md    # 系统设计主题图
```

#### 工具和配置

```
rust-program-cha/
├── tools/                   # 工具脚本
│   └── restart-rust-analyzer.sh     # 重启Rust Analyzer脚本
├── .cargo/                  # Cargo配置目录
├── .gitignore               # Git忽略配置
├── .tool-versions           # 工具版本规范
├── Cargo.toml               # 主项目Cargo配置
├── rust-toolchain.toml      # Rust工具链版本规范
├── rust_performance_optimization_guide.md # 性能优化专项指南
└── README.md                # 项目主说明文档
```

## 🎯 学习目标

### 基础能力
- **语言基础**：掌握Rust语法规则、基本数据类型、控制流和函数定义
- **核心概念**：深入理解所有权、借用、生命周期等Rust独特的内存管理机制
- **工具链使用**：熟练使用Cargo、rustfmt、Clippy等Rust开发工具链

### 进阶能力
- **并发与异步**：掌握Rust的并发模型、线程安全和现代异步编程模式
- **系统编程**：学习使用Rust进行底层系统级编程和硬件交互
- **Web开发**：使用Rust开发高性能的Web应用和API服务
- **内存安全**：深入理解Rust的内存安全保证和未定义行为的边界

### 质量保证
- **代码质量**：编写安全、高效、可维护的生产级Rust代码
- **性能优化**：掌握Rust性能调优和内存优化技术
- **实战经验**：通过完成各种难度的项目，积累实际开发经验

## 🌟 项目特点

### 教学体系
- **系统化学习路径** - 从基础到高级的渐进式学习体系
- **实践驱动** - 每个概念都配有实际示例和练习

### 内容组织
- **详细文档** - 丰富的文档资源和学习指南
- **模块化设计** - 主题明确的目录结构，易于导航和学习

### 技术支持
- **最新工具链** - 基于最新稳定版 Rust 工具链构建
- **规范标准** - 遵循Rust官方推荐的编码规范和最佳实践

### 代码质量
- **注释详尽** - 代码中包含详细注释，便于理解核心概念
- **可维护性** - 注重代码组织和结构，便于维护和扩展

### 学习资源
- **难点解析** - 深入剖析Rust学习中的常见难点，特别是unsafe Rust和内存管理相关概念
- **示例项目** - 提供各种难度的实际项目案例
- **未定义行为检测** - 介绍如何使用Miri等工具检测未定义行为

## 🤝 贡献指南

我们欢迎社区贡献来改进和扩展这个学习项目！

### 如何贡献
1. **报告问题** - 通过 GitHub Issues 报告错误或提出改进建议
2. **提交代码** - 遵循以下步骤：
   - Fork 本仓库
   - 创建功能分支 (`git checkout -b feature/amazing-feature`)
   - 提交更改 (`git commit -m 'Add some amazing feature'`)
   - 推送到分支 (`git push origin feature/amazing-feature`)
   - 打开 Pull Request

### 代码规范
- 遵循 Rust 官方的编码规范
- 使用`rustfmt`格式化代码
- 运行`clippy`检查潜在问题
- 确保所有测试通过

### 提交信息
- 提交信息应清晰描述变更内容
- 建议使用语义化提交信息格式
- 对于bug修复，引用相关issue编号

### 问题反馈
- 在GitHub上创建issue描述问题
- 提供详细的复现步骤和预期行为
- 尽可能包含相关的环境信息和日志

## 📄 许可证

该项目采用以下开源许可证：

- **主项目代码**：MIT License
- **文档资源**：CC BY-NC-SA 4.0

### MIT License

MIT许可证是一种宽松的开源软件许可证，允许您：
- 自由使用、复制、修改和分发软件
- 在私有项目中使用
- 商业使用，但需保留原始许可证声明

### 许可证详情

完整的许可证文本可在项目根目录的LICENSE文件中找到。使用本项目代码时，请确保遵守相应的许可证条款。

## 🔗 相关资源

### 官方文档
- [Rust官方文档](https://doc.rust-lang.org/)
- [Rust编程语言中文版](https://kaisery.github.io/trpl-zh-cn/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)

### 学习社区
- [Rust中文社区](https://rustcc.cn/)
- [Rust编程学院](https://time.geekbang.org/course/intro/100060501)
- [Stack Overflow - Rust标签](https://stackoverflow.com/questions/tagged/rust)

### 视频教程
- [Rust官方入门视频](https://www.youtube.com/c/rustlang)
- [Rust实战教程](https://www.bilibili.com/video/BV1hp4y1k7SV)

### 工具推荐
- [Rust Playground](https://play.rust-lang.org/) - 在线Rust编程环境
- [VS Code + Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) - 推荐的开发环境
- [Rustfmt](https://github.com/rust-lang/rustfmt) - 代码格式化工具
- [Clippy](https://github.com/rust-lang/rust-clippy) - 代码质量检查工具
- [Miri](https://github.com/rust-lang/miri) - 未定义行为检测器和Rust解释器

---

**开始你的 Rust 学习之旅！** 🦀

<p align="center">
  <i>"Rust: 一门赋予每个人构建可靠且高效软件能力的语言。"</i>
  <br>
  <i>安全、并发、高效 — 选择 Rust，选择未来！</i>
</p>