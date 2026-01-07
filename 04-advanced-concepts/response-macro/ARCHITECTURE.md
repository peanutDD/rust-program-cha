# Response Macro 系列架构说明

## 📚 项目结构

response-macro 系列采用分层架构，适合渐进式学习：

```
response-macro-core/      # 核心库（基础类型和工具）
    ├── ApiError          # 统一的 API 错误响应类型
    ├── ResponseExt       # 响应扩展 trait
    └── 工具函数

response-macro/           # 过程宏库（proc-macro）
    ├── #[derive(Response)]  # 派生宏
    ├── #[response]          # 属性宏
    └── error!               # 过程宏

response-macro-example/   # 简单示例
    └── 展示基本用法和使用场景

response-macro-advanced/  # 高级应用示例
    ├── handlers/         # 请求处理器
    ├── services/         # 业务逻辑层
    └── models/           # 数据模型
```

## 🎯 学习路径

### 1. 核心库（response-macro-core）
**目标**：理解基础类型和工具

- 学习 `ApiError` 结构体
- 理解统一响应格式
- 掌握链式响应构建

**学习重点**：
- `ApiError` 的创建和使用方法
- 错误响应的标准化格式
- 状态码和消息处理

### 2. 过程宏库（response-macro）
**目标**：掌握宏的使用

- 学习 `#[derive(Response)]` 派生宏
- 使用 `#[response]` 属性宏简化函数
- 使用 `error!` 宏简化错误创建

**学习重点**：
- 宏的工作原理
- 如何自定义响应结构
- 自动错误处理机制

### 3. 简单示例（response-macro-example）
**目标**：实践基本用法

- 运行简单示例
- 理解基本工作流程
- 尝试修改和扩展

**学习重点**：
- API 端点的实现
- 错误处理模式
- 响应格式化

### 4. 高级应用（response-macro-advanced）
**目标**：构建完整应用

- 学习分层架构设计
- 理解服务层分离
- 掌握完整的错误处理流程

**学习重点**：
- 项目结构组织
- 业务逻辑分层
- 生产环境最佳实践

## 📖 依赖关系

```
response-macro-core (独立)
    ↑
response-macro (依赖 core)
    ↑
response-macro-example (依赖 macro)
response-macro-advanced (依赖 macro)
```

## 🚀 快速开始

### 1. 运行核心库示例
```bash
cd response-macro-core
cargo run
```

### 2. 运行简单示例
```bash
cd response-macro-example
cargo run
```

### 3. 运行高级应用
```bash
cd response-macro-advanced
cargo run
```

## 💡 使用建议

1. **初学者**：从 `response-macro-core` 开始，理解基础类型
2. **进阶者**：学习 `response-macro` 的宏系统
3. **实践者**：运行 `response-macro-example` 进行实践
4. **高级用户**：研究 `response-macro-advanced` 的完整架构

## 📚 相关文档

- [README.md](./README.md) - 宏库详细说明
- [EXAMPLE_USAGE.md](./EXAMPLE_USAGE.md) - 使用示例指南

