# Rust 学习项目 (rust-program-cha)

系统性 Rust 编程语言学习项目，主题导向目录结构，从基础到高级特性的完整学习路径。

## 🚀 快速开始

```bash
# 克隆项目
git clone <repository-url>
cd rust-program-cha

# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 构建和运行
cargo build
cargo run --example <example-name>
```

## 📚 学习路径

### 推荐学习顺序

1. **[01-fundamentals](./01-fundamentals/)** - 基础概念
   - 语法、数据类型、函数、闭包

2. **[02-ownership-borrowing](./02-ownership-borrowing/)** - 所有权与借用
   - 所有权、借用、生命周期

3. **[03-data-structures](./03-data-structures/)** - 数据结构
   - 复合类型、方法、模式匹配

4. **[04-advanced-concepts](./04-advanced-concepts/)** - 高级概念
   - 泛型、Trait、宏、错误处理

5. **[05-concurrency-async](./05-concurrency-async/)** - 并发与异步
   - 多线程、异步编程

6. **[06-memory-management](./06-memory-management/)** - 内存管理
   - 智能指针、Unsafe Rust

7. **[07-performance-optimization](./07-performance-optimization/)** - 性能优化
   - 移动语义、优化技巧

8. **[08-practical-examples](./08-practical-examples/)** - 实际应用
   - 嵌入式开发示例

## 📖 文档资源

- **[项目结构说明](./docs/project-structure.md)**
- **[Rust 学习完整指南](./docs/rust-learning-complete-guide.md)**
- **[Rust 学习路线图](./docs/rust-learning-roadmap.md)**
- **[Rust 难点总结](./docs/rust-difficulty-summary.md)**
- **[API 学习路线图](./docs/api-learning-roadmap.md)**
- **[Rust 性能优化全面指南](./rust_performance_optimization_guide.md)**

## 🛠️ 工具和配置

- **[工具脚本](./tools/)** - 开发辅助工具
- **[Rust 工具链配置](./rust-toolchain.toml)** - 版本管理
- **[Cargo 配置](./Cargo.toml)** - 项目依赖
- **[Cargo 本地配置](./.cargo/config.toml)** - 开发环境
- **[Mise 版本管理](./docs/mise-setup-guide.md)** - 配置指南

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

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request 改进本项目！

## 📄 许可证

MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 🔗 相关资源

- [Rust 官方文档](https://doc.rust-lang.org/)
- [Rust 程序设计语言](https://doc.rust-lang.org/book/)
- [Rust 标准库文档](https://doc.rust-lang.org/std/)
- [Rust 参考手册](https://doc.rust-lang.org/reference/)

---

**开始你的 Rust 学习之旅！** 🦀