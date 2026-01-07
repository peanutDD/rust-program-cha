# Response Macro 简单示例

> response-macro 库的基础使用示例

## 📚 项目概述

本项目展示了 `response-macro` 库的基本用法，包括：
- ApiResponse 结构体的使用
- error! 宏的使用
- 基本的 API 端点实现

## 🎯 学习目标

通过本示例，你将学会：

- ✅ 使用 response-macro 简化 API 响应
- ✅ 实现基本的 RESTful API 端点
- ✅ 处理错误和成功响应
- ✅ 理解 response-macro 的基本工作流程

## 🚀 快速开始

```bash
# 运行示例
cargo run

# 测试 API
curl http://127.0.0.1:8080/users
curl http://127.0.0.1:8080/users/1
```

## 📖 示例内容

### 1. 基本响应结构

```rust
#[derive(Debug, Serialize)]
struct ApiResponse<T: Serialize> {
    success: bool,
    message: String,
    data: Option<T>,
    code: u16,
}
```

### 2. 使用 error! 宏

```rust
use response_macro::error;

let user_id = params.get("user_id")
    .and_then(|v| v.as_u64())
    .ok_or_else(|| error!("无效的用户ID"))?;
```

### 3. API 端点实现

- `GET /users` - 获取所有用户
- `GET /users/{id}` - 获取指定用户
- `POST /users` - 创建新用户

## 💡 学习建议

这是一个简单的示例项目，适合：
1. 了解 response-macro 的基本用法
2. 学习如何构建简单的 RESTful API
3. 理解响应格式的统一处理

**进阶学习**：
- 查看 `response-macro/` 了解宏的实现
- 查看 `response-macro-advanced/` 学习完整项目架构
- 查看 `response-macro-core/` 了解核心类型

## 📚 相关资源

- [Response Macro 主文档](../response-macro/README.md)
- [Response Macro 架构说明](../response-macro/ARCHITECTURE.md)
- [高级应用示例](../response-macro-advanced/)

---

**Happy Coding! 🦀**

