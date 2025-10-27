# Response 宏库使用示例

本文件展示了如何在实际项目中使用我们的 Response 宏库，即使在当前工作区配置下无法直接构建。

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
syn = { version = "2.0", features = ["full", "extra-traits"] }
quote = "1.0"
proc-macro2 = "1.0"
http = "0.2"
serde_json = "1.0"
actix-web = "4.0"
```

## 3. 实现宏库

在 `response-macro-lib/src/lib.rs` 中：

```rust
//! Response 宏库
//! 
//! 提供宏来简化 Response<T> 的生成逻辑和错误返回，减少 match/Json()/StatusCode 的重复书写。

use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, DeriveInput, AttributeArgs, ItemFn, Lit, Meta, NestedMeta};

/// Response 特性，用于将结构体转换为 HTTP 响应
pub trait Response {
    fn into_response(self) -> actix_web::HttpResponse;
}

/// ResponseData 特性，用于提取响应数据类型
pub trait ResponseData {
    type Data;
}

/// 为结构体自动实现 Response 特性
#[proc_macro_derive(Response)]
pub fn derive_response(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let generics = &input.generics;
    
    let expanded = quote! {
        impl #generics Response for #struct_name #generics
        where
            Self: serde::Serialize,
        {
            fn into_response(self) -> actix_web::HttpResponse {
                let status = match self.code {
                    200 => http::StatusCode::OK,
                    201 => http::StatusCode::CREATED,
                    400 => http::StatusCode::BAD_REQUEST,
                    401 => http::StatusCode::UNAUTHORIZED,
                    403 => http::StatusCode::FORBIDDEN,
                    404 => http::StatusCode::NOT_FOUND,
                    500 => http::StatusCode::INTERNAL_SERVER_ERROR,
                    _ => http::StatusCode::OK,
                };
                
                actix_web::HttpResponse::builder()
                    .status(status)
                    .header("content-type", "application/json")
                    .body(actix_web::web::Json(self).into_body())
                    .unwrap()
            }
        }
        
        impl #generics #struct_name #generics {
            /// 创建成功响应
            pub fn success(data: <#struct_name #generics as ResponseData>::Data) -> Self {
                Self {
                    data,
                    message: "success".to_string(),
                    code: 200,
                }
            }
            
            /// 创建错误响应
            pub fn error(message: &str, code: u16) -> Self
            where
                <Self as ResponseData>::Data: Default,
            {
                Self {
                    data: Default::default(),
                    message: message.to_string(),
                    code,
                }
            }
        }
    };
    
    expanded.into()
}

/// 为函数添加自动的响应处理
#[proc_macro_attribute]
pub fn response(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let item_fn = parse_macro_input!(input as ItemFn);
    
    let mut success_code = 200;
    let mut error_code = 500;
    
    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
            if nv.path.is_ident("success_code") {
                if let Lit::Int(int) = nv.lit {
                    success_code = int.base10_parse().unwrap_or(200);
                }
            } else if nv.path.is_ident("error_code") {
                if let Lit::Int(int) = nv.lit {
                    error_code = int.base10_parse().unwrap_or(500);
                }
            }
        }
    }
    
    let block = &item_fn.block;
    let fn_name = &item_fn.sig.ident;
    let fn_visibility = &item_fn.vis;
    let fn_inputs = &item_fn.sig.inputs;
    let fn_attrs = &item_fn.attrs;
    
    let expanded = quote! {
        #(#fn_attrs)*
        #fn_visibility async fn #fn_name(#(#fn_inputs),*) -> impl actix_web::Responder {
            match (|| async move #block)().await {
                Ok(data) => {
                    let success_response = serde_json::json!({
                        "data": data,
                        "message": "success",
                        "code": #success_code
                    });
                    actix_web::HttpResponse::build(match #success_code {
                        200 => http::StatusCode::OK,
                        201 => http::StatusCode::CREATED,
                        _ => http::StatusCode::OK
                    })
                    .header("content-type", "application/json")
                    .json(success_response)
                },
                Err(err) => {
                    let error_response = serde_json::json!(
                        {
                            "data": null,
                            "message": err.to_string(),
                            "code": #error_code
                        }
                    );
                    actix_web::HttpResponse::build(match #error_code {
                        400 => http::StatusCode::BAD_REQUEST,
                        401 => http::StatusCode::UNAUTHORIZED,
                        403 => http::StatusCode::FORBIDDEN,
                        404 => http::StatusCode::NOT_FOUND,
                        500 => http::StatusCode::INTERNAL_SERVER_ERROR,
                        _ => http::StatusCode::INTERNAL_SERVER_ERROR
                    })
                    .header("content-type", "application/json")
                    .json(error_response)
                }
            }
        }
    };
    
    expanded.into()
}

/// 简化错误处理的宏
#[proc_macro]
pub fn error(input: TokenStream) -> TokenStream {
    let tokens = input.into_iter().collect::<Vec<_>>();
    let input_str = tokens.first().map(|t| t.to_string()).unwrap_or_default();
    let args = &tokens[1..];
    
    let expanded = quote! {
        {{
            let error_message = format!(#input_str #(, #args)*);
            AppError::new(&error_message)
        }}
    };
    
    expanded.into()
}
```

## 4. 配置应用项目

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
```

## 5. 实现应用示例

在 `response-macro-app/src/main.rs` 中：

```rust
use actix_web::{web, App, HttpServer, Responder, middleware::Logger};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

// 导入我们的宏库
use response_macro_lib::{Response, response, error, ResponseData};

// 定义错误类型
#[derive(Debug)]
struct AppError {
    message: String,
}

impl AppError {
    fn new(message: &str) -> Self {
        Self { message: message.to_string() }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppError {}

// 定义API响应结构体，并使用我们的Response宏
#[derive(Debug, Serialize, Response)]
struct ApiResponse<T> {
    data: T,
    message: String,
    code: u16,
}

// 为ApiResponse实现ResponseData特性
impl<T> ResponseData for ApiResponse<T> {
    type Data = T;
}

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
    
    fn get_user(&self, id: u32) -> Result<User, AppError> {
        let users = self.users.lock().map_err(|_| AppError::new("数据库锁错误"))?;
        users.get(&id)
            .cloned()
            .ok_or_else(|| AppError::new(&format!("用户 {} 不存在", id)))
    }
    
    fn add_user(&self, user: User) -> Result<User, AppError> {
        let mut users = self.users.lock().map_err(|_| AppError::new("数据库锁错误"))?;
        
        if users.contains_key(&user.id) {
            return Err(AppError::new(&format!("用户 ID {} 已存在", user.id)));
        }
        
        users.insert(user.id, user.clone());
        Ok(user)
    }
}

// 使用response宏简化错误处理和响应生成
#[response(success_code = 200, error_code = 404)]
async fn get_user(db: web::Data<Database>, user_id: web::Path<u32>) -> Result<User, AppError> {
    // 直接返回Result，宏会自动处理错误和转换为响应
    db.get_user(*user_id)
}

// 使用response宏，并自定义成功状态码
#[response(success_code = 201)]
async fn create_user(db: web::Data<Database>, user: web::Json<User>) -> Result<User, AppError> {
    // 直接返回Result，宏会自动处理错误和转换为响应
    db.add_user(user.into_inner())
}

// 使用ApiResponse直接创建响应
async fn get_all_users(db: web::Data<Database>) -> impl Responder {
    let users = match db.users.lock() {
        Ok(users) => {
            let users_vec: Vec<&User> = users.values().collect();
            // 使用ApiResponse的success方法创建成功响应
            actix_web::web::Json(ApiResponse::success(users_vec))
        },
        Err(_) => {
            // 使用ApiResponse的error方法创建错误响应
            actix_web::web::Json(ApiResponse::error("数据库锁错误", 500))
        }
    };
    users
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
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## 6. 如何构建和运行

在独立的目录中：

```bash
cd ~/response-macro-demo

# 构建宏库
cd response-macro-lib
cargo build

# 构建并运行应用
cd ../response-macro-app
cargo run
```

## 7. 宏库的优势

使用这些宏可以显著减少样板代码：

### 不使用宏时的代码：

```rust
async fn get_user(db: web::Data<Database>, user_id: web::Path<u32>) -> impl Responder {
    match db.get_user(*user_id) {
        Ok(user) => {
            let response = ApiResponse {
                data: user,
                message: "success".to_string(),
                code: 200,
            };
            HttpResponse::Ok()
                .header("content-type", "application/json")
                .json(response)
        },
        Err(err) => {
            let error_response = ApiResponse {
                data: None,
                message: err.to_string(),
                code: 404,
            };
            HttpResponse::NotFound()
                .header("content-type", "application/json")
                .json(error_response)
        }
    }
}
```

### 使用宏后的代码：

```rust
#[response(success_code = 200, error_code = 404)]
async fn get_user(db: web::Data<Database>, user_id: web::Path<u32>) -> Result<User, AppError> {
    db.get_user(*user_id)
}
```

## 8. 宏的工作原理简要说明

1. **Response派生宏**：
   - 解析结构体定义，提取名称和泛型参数
   - 生成Response特性实现，处理状态码和HTTP响应转换
   - 添加success和error方法用于创建响应

2. **response属性宏**：
   - 解析函数定义和属性参数
   - 提取成功和错误状态码
   - 生成新的函数体，自动处理Result并转换为HTTP响应

3. **error!过程宏**：
   - 解析格式化字符串和参数
   - 生成错误创建代码，支持字符串格式化

这些宏共同工作，大大简化了API响应处理和错误管理，让开发者可以更专注于业务逻辑的实现。