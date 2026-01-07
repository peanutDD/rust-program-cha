# Response Macro 高级应用示例

> response-macro 库的完整项目架构示例

## 📚 项目概述

本项目展示了使用 `response-macro` 构建完整 RESTful API 的高级应用，包括：
- 分层架构设计（Handler/Service/Model）
- 完整的错误处理机制
- 数据库服务抽象
- 用户管理 API

## 🎯 学习目标

通过本示例，你将学会：

- ✅ 构建分层架构的 Rust Web 应用
- ✅ 实现完整的错误处理流程
- ✅ 设计可扩展的服务层
- ✅ 使用 response-macro 简化响应处理
- ✅ 构建生产级别的 API 项目

## 📖 项目结构

```
response-macro-advanced/
├── src/
│   ├── main.rs              # 应用入口和路由配置
│   ├── models/              # 数据模型层
│   │   ├── mod.rs
│   │   ├── user.rs          # 用户模型
│   │   └── error.rs         # 错误类型定义
│   ├── services/            # 业务逻辑层
│   │   ├── mod.rs
│   │   ├── db.rs            # 数据库服务抽象
│   │   └── user.rs          # 用户业务逻辑
│   └── handlers/            # 请求处理层
│       ├── mod.rs
│       └── user.rs          # 用户 API 处理器
└── Cargo.toml
```

## 🏗️ 架构设计

### 1. 分层架构

**Handler 层**：
- 处理 HTTP 请求
- 参数验证和解析
- 调用 Service 层
- 返回 HTTP 响应

**Service 层**：
- 业务逻辑实现
- 数据验证和处理
- 调用数据库服务
- 错误处理

**Model 层**：
- 数据模型定义
- 错误类型定义
- 序列化/反序列化

### 2. 错误处理

**统一错误类型：**
```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("用户不存在: {0}")]
    UserNotFound(u32),
    #[error("数据库错误: {0}")]
    DatabaseError(String),
}
```

**错误转换：**
- 自动转换为 HTTP 响应
- 统一的错误格式
- 详细的错误信息

### 3. 数据库抽象

**服务接口：**
```rust
pub trait DatabaseService: Send + Sync {
    fn get_user(&self, id: u32) -> Result<User, AppError>;
    fn create_user(&self, user: CreateUserRequest) -> Result<User, AppError>;
}
```

**实现：**
- 内存数据库实现
- 可扩展为真实数据库
- 线程安全的实现

## 🚀 快速开始

```bash
# 运行应用
cargo run

# 测试 API
curl http://127.0.0.1:8080/api/health
curl http://127.0.0.1:8080/api/users
curl http://127.0.0.1:8080/api/users/1
```

## 📖 学习路径

### 1. 理解架构
- 查看 `main.rs` 了解路由配置
- 理解分层设计理念
- 学习依赖注入模式

### 2. 学习实现
- 查看 `models/` 了解数据模型
- 查看 `services/` 了解业务逻辑
- 查看 `handlers/` 了解请求处理

### 3. 实践扩展
- 添加新的 API 端点
- 实现新的业务逻辑
- 扩展错误处理

## 💡 最佳实践

1. **分层设计**：清晰的分层有利于维护和测试
2. **错误处理**：统一的错误类型和转换机制
3. **服务抽象**：使用 trait 实现可测试和可扩展
4. **文档完善**：每个层都有清晰的职责说明

## 🔍 关键特性

- ✅ 完整的错误处理链
- ✅ 线程安全的数据访问
- ✅ 可扩展的服务架构
- ✅ 统一的响应格式
- ✅ 健康检查和监控端点

## 📚 相关资源

- [Response Macro 主文档](../response-macro/README.md)
- [Response Macro 架构说明](../response-macro/ARCHITECTURE.md)
- [简单示例](../response-macro-example/)

## 🎯 总结

这是一个生产级别的 API 项目示例，展示了如何使用 response-macro 构建完整、可维护的 Web 应用。通过学习这个示例，你可以掌握 Rust Web 开发的最佳实践。

---

**Happy Building! 🦀**

