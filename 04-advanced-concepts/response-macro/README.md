# Response Macro

这是一个Rust过程宏库，用于简化API响应生成和错误处理，减少重复代码。

## 功能特性

1. **Response派生宏** - 为结构体自动实现Response特性，提供success和error方法
2. **response属性宏** - 为函数添加自动响应处理，自动转换Result为HTTP响应
3. **error!过程宏** - 简化错误创建和格式化

## 宏库结构

### 1. Response派生宏

```rust
#[derive(Debug, Serialize, Response)]
struct ApiResponse<T> {
    data: T,
    message: String,
    code: u16,
}
```

这个宏为结构体自动实现以下功能：
- `success(data)` 方法：创建成功响应，自动设置状态码为200
- `error(message, code)` 方法：创建错误响应，设置自定义错误信息和状态码
- `into_response()` 方法：将响应转换为HTTP响应

### 2. response属性宏

```rust
#[response(success_code = 200, error_code = 404)]
async fn get_user(db: web::Data<Database>, user_id: web::Path<u32>) -> Result<User, AppError> {
    db.get_user(*user_id)
}
```

这个宏自动将函数的Result<T, E>返回值转换为HTTP响应：
- 成功时：将T包装为JSON，设置指定的成功状态码
- 失败时：将错误信息包装为JSON，设置指定的错误状态码

### 3. error!过程宏

```rust
let user_id = params.get("user_id")
    .and_then(|v| v.as_u64())
    .ok_or_else(|| error!("无效的用户ID"))?;
```

这个宏简化了错误创建和格式化，支持格式化参数。

## 工作原理

### Response派生宏

1. 使用`syn`库解析结构体定义
2. 提取结构体的字段和泛型参数
3. 使用`quote`库生成实现代码，包括success、error和into_response方法

### response属性宏

1. 使用`syn`库解析函数定义和属性参数
2. 提取函数签名、返回类型和属性配置
3. 使用`quote`库生成新的函数实现，自动处理Result并转换为HTTP响应

### error!过程宏

1. 使用`syn`库解析宏调用和参数
2. 使用`quote`库生成错误创建代码，支持格式化字符串

## 安装和使用

将以下内容添加到你的Cargo.toml文件：

```toml
[dependencies]
response-macro = { path = "../response-macro" }
actix-web = "4.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

在代码中导入宏库：

```rust
use response_macro::{Response, response, error};
```

## 示例

### 定义响应结构体

```rust
#[derive(Debug, Serialize, Response)]
struct ApiResponse<T> {
    data: T,
    message: String,
    code: u16,
}
```

### 使用response属性宏

```rust
#[response(success_code = 200, error_code = 404)]
async fn get_user(db: web::Data<Database>, user_id: web::Path<u32>) -> Result<User, AppError> {
    db.get_user(*user_id)
}
```

### 手动使用ApiResponse

```rust
async fn get_all_users(db: web::Data<Database>) -> impl Responder {
    let users = match db.users.lock() {
        Ok(users) => users,
        Err(_) => {
            return Json(ApiResponse::error("数据库锁错误", 500));
        }
    };
    
    let users_vec: Vec<&User> = users.values().collect();
    Json(ApiResponse::success(users_vec))
}
```

## 优势

1. **减少样板代码**：自动处理响应生成和错误转换
2. **提高一致性**：确保所有API响应格式一致
3. **简化错误处理**：提供统一的错误创建和处理机制
4. **提高可读性**：让API处理函数更专注于业务逻辑

## 注意事项

1. 使用Response派生宏的结构体必须有data、message和code字段
2. response属性宏只适用于返回Result<T, E>的异步函数
3. 确保错误类型实现了std::fmt::Display和std::error::Error特性