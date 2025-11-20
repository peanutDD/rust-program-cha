# Response 宏库使用示例

本文件展示了如何在实际项目中使用我们的 Response 宏库，包含最新的功能实现和使用方法。

## 1. 创建一个新的独立项目

```bash
mkdir -p ~/response-macro-demo
cd ~/response-macro-demo
cargo new response-macro-lib --lib
cargo new response-macro-app
```

## 2. 配置 response-macro-lib

在 `response-macro-lib/Cargo.toml` 中：

```toml
[package]
name = "response-macro-lib"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
syn = "2.0"
quote = "1.0"
proc-macro2 = "1.0"
http = "0.2"
serde_json = "1.0"
actix-web = "4.0"
serde = { version = "1.0", features = ["derive"] }
```

## 3. 实现应用项目

在 `response-macro-app/Cargo.toml` 中：

```toml
[package]
name = "response-macro-app"
version = "0.1.0"
edition = "2021"

[dependencies]
response-macro-lib = { path = "../response-macro-lib" }
actix-web = "4.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

## 4. 主要功能和使用方法

### 4.1 ApiError 结构体

`ApiError` 是一个强大的错误响应结构体，支持丰富的链式方法和内容协商。

```rust
use response_macro_lib::ApiError;

// 创建基本错误响应
let error = ApiError::bad_request("参数错误");

// 添加详情信息
let detailed_error = ApiError::not_found("资源不存在")
    .with_details(Some("请检查资源ID是否正确"));

// 设置自定义状态码
let custom_error = ApiError::internal_error("服务器内部错误")
    .with_code(503);

// 添加追踪ID
let tracked_error = ApiError::forbidden("无权限访问")
    .with_trace_id(Some("trace-123456"));
```

### 4.2 快捷错误创建方法

```rust
// 常见错误类型的快捷方法
let bad_request = ApiError::bad_request("请求参数错误");
let unauthorized = ApiError::unauthorized("未授权访问");
let forbidden = ApiError::forbidden("禁止访问");
let not_found = ApiError::not_found("资源不存在");
let internal_error = ApiError::internal_error("服务器内部错误");
let service_unavailable = ApiError::service_unavailable("服务暂时不可用");
```

### 4.3 链式响应构建

使用链式方法构建复杂的响应：

```rust
use actix_web::{web, HttpResponse};
use response_macro_lib::{response, ApiError};

#[response(success_code = 200)]
async fn get_data() -> Result<serde_json::Value, ApiError> {
    let data = serde_json::json!({"name": "测试用户", "age": 30});
    Ok(data)
}

// 使用链式方法自定义响应
async fn get_custom_response() -> impl actix_web::Responder {
    // 创建成功响应并添加自定义头
    serde_json::json!({"status": "success", "data": {}})
        .with_header("X-Custom-Header", "custom-value")
        .with_status(201)
}

// 自定义错误响应
async fn handle_error() -> impl actix_web::Responder {
    ApiError::bad_request("参数验证失败")
        .with_details(Some("用户名不能为空"))
        .with_header("X-Error-Type", "validation")
}
```

### 4.4 内容协商功能

库支持基于 `Accept` 头的内容协商：

```rust
// 当客户端请求不同的内容类型时，自动返回相应格式
// curl -H "Accept: application/json" http://localhost:8080/api
// curl -H "Accept: text/plain" http://localhost:8080/api

// 强制指定内容类型
async fn get_xml_response() -> impl actix_web::Responder {
    let data = serde_json::json!({"message": "这是XML响应"});
    data.with_content_type("application/xml")
}
```

### 4.5 超时保护功能

`response` 宏内置了超时保护机制：

```rust
// 设置超时时间为5秒
#[response(success_code = 200, timeout = 5)]
async fn long_running_operation() -> Result<serde_json::Value, ApiError> {
    // 模拟耗时操作
    tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
    Ok(serde_json::json!({"result": "操作成功"}))
}

// 使用默认超时（3秒）
#[response]
async fn default_timeout_operation() -> Result<(), ApiError> {
    // 业务逻辑...
    Ok(())
}
```

## 5. 完整应用示例

在 `response-macro-app/src/main.rs` 中：

```rust
use actix_web::{web, App, HttpServer, middleware::Logger};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

// 导入宏库
use response_macro_lib::{response, ApiError};

// 定义用户结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// 内存数据库
struct Database {
    users: Mutex<HashMap<u32, User>>,
}

impl Database {
    fn new() -> Self {
        let mut users = HashMap::new();
        users.insert(1, User { id: 1, name: "张三".to_string(), email: "zhangsan@example.com".to_string() });
        users.insert(2, User { id: 2, name: "李四".to_string(), email: "lisi@example.com".to_string() });
        
        Self {
            users: Mutex::new(users),
        }
    }
    
    fn get_user(&self, id: u32) -> Result<User, ApiError> {
        let users = self.users.lock().map_err(|_| ApiError::internal_error("数据库锁错误"))?;
        users.get(&id)
            .cloned()
            .ok_or_else(|| ApiError::not_found(&format!("用户 ID {} 不存在", id)))
    }
    
    fn add_user(&self, user: User) -> Result<User, ApiError> {
        let mut users = self.users.lock().map_err(|_| ApiError::internal_error("数据库锁错误"))?;
        
        if users.contains_key(&user.id) {
            return Err(ApiError::bad_request(&format!("用户 ID {} 已存在", user.id)));
        }
        
        users.insert(user.id, user.clone());
        Ok(user)
    }
    
    fn get_all_users(&self) -> Result<Vec<User>, ApiError> {
        let users = self.users.lock().map_err(|_| ApiError::internal_error("数据库锁错误"))?;
        Ok(users.values().cloned().collect())
    }
}

// 使用response宏简化错误处理和响应生成
#[response(success_code = 200, error_code = 404)]
async fn get_user(db: web::Data<Database>, user_id: web::Path<u32>) -> Result<User, ApiError> {
    db.get_user(*user_id)
}

// 使用response宏，并自定义成功状态码
#[response(success_code = 201)]
async fn create_user(db: web::Data<Database>, user: web::Json<User>) -> Result<User, ApiError> {
    db.add_user(user.into_inner())
}

// 使用链式方法创建响应
async fn get_all_users(db: web::Data<Database>) -> impl actix_web::Responder {
    match db.get_all_users() {
        Ok(users) => {
            // 使用链式方法添加自定义头和状态码
            users
                .with_header("X-User-Count", &users.len().to_string())
                .with_header("Cache-Control", "no-cache")
                .with_status(200)
        },
        Err(err) => {
            // 错误响应也可以链式添加头信息
            err.with_header("X-Error-Detail", "数据库操作失败")
        }
    }
}

// 演示内容协商
async fn negotiate_content() -> impl actix_web::Responder {
    // 这个响应会根据客户端的Accept头自动调整格式
    serde_json::json!({"message": "这是一个支持内容协商的响应"})
}

// 演示超时功能
#[response(timeout = 3)]
async fn slow_operation() -> Result<serde_json::Value, ApiError> {
    // 模拟耗时操作，如果超过3秒会自动返回超时错误
    tokio::time::sleep(Duration::from_secs(4)).await;
    Ok(serde_json::json!({"result": "操作完成"}))
}

// 自定义错误处理示例
#[response]
async fn validate_input(param: web::Query<HashMap<String, String>>) -> Result<(), ApiError> {
    if let Some(value) = param.get("required") {
        if value.is_empty() {
            return Err(ApiError::bad_request("required参数不能为空")
                .with_details(Some("请提供有效的required参数值"))
                .with_trace_id(Some("validation-123")));
        }
    } else {
        return Err(ApiError::bad_request("缺少required参数"));
    }
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("启动服务器...");
    
    // 创建数据库实例
    let db = web::Data::new(Database::new());
    
    // 启动服务器
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .wrap(Logger::default())
            .route("/users", web::get().to(get_all_users))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users", web::post().to(create_user))
            .route("/negotiate", web::get().to(negotiate_content))
            .route("/slow", web::get().to(slow_operation))
            .route("/validate", web::get().to(validate_input))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## 6. 如何测试

### 6.1 测试基本功能

```bash
# 启动服务器
cd ~/response-macro-demo/response-macro-app
cargo run

# 测试成功响应
curl http://localhost:8080/users

# 测试特定用户查询
curl http://localhost:8080/users/1

# 测试不存在的用户（错误响应）
curl http://localhost:8080/users/999

# 测试创建用户
curl -X POST http://localhost:8080/users \
  -H "Content-Type: application/json" \
  -d '{"id": 3, "name": "王五", "email": "wangwu@example.com"}'
```

### 6.2 测试内容协商

```bash
# 请求JSON格式
curl -H "Accept: application/json" http://localhost:8080/negotiate

# 请求文本格式
curl -H "Accept: text/plain" http://localhost:8080/negotiate
```

### 6.3 测试超时功能

```bash
# 测试超时功能（会返回504错误）
curl http://localhost:8080/slow
```

### 6.4 测试参数验证

```bash
# 缺少参数
curl http://localhost:8080/validate

# 参数为空
curl http://localhost:8080/validate?required=

# 参数有效
curl http://localhost:8080/validate?required=test
```

## 7. 核心功能说明

### 7.1 ApiError 结构体

`ApiError` 是一个实现了 `actix_web::Responder` 和 `std::error::Error` 的错误结构体，支持：
- 统一的错误响应格式
- 丰富的错误创建快捷方法
- 链式方法添加详情、状态码和追踪ID
- 自动内容协商（JSON/Text）

### 7.2 response 宏

`response` 属性宏可以：
- 自动处理函数返回的 `Result<T, E>`
- 根据 `success_code` 和 `error_code` 自定义状态码
- 通过 `timeout` 参数设置超时保护
- 自动序列化成功响应和错误响应

### 7.3 链式响应构建

通过 `with_header`、`with_status` 和 `with_content_type` 方法：
- 灵活添加自定义HTTP头
- 修改响应状态码
- 自定义内容类型

### 7.4 内容协商

库自动支持基于 `Accept` 头的内容协商：
- 优先根据客户端请求的格式返回内容
- 支持JSON和Text格式
- 可以通过 `with_content_type` 强制指定格式

### 7.5 超时保护

`response` 宏内置的超时保护：
- 防止长时间运行的操作阻塞请求处理
- 可以自定义超时时间
- 超时后自动返回504 Gateway Timeout错误