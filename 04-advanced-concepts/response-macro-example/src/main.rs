use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, middleware::Logger, http::StatusCode, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};
use env_logger;

// 导入我们的宏库功能
use response_macro::error;

// 定义用户ID生成器，用于自动生成唯一ID
static NEXT_USER_ID: AtomicU32 = AtomicU32::new(3); // 从3开始，因为已有ID 1和2的用户

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

// 定义ApiResponse结构体并手动实现Responder特性
#[derive(Debug, Serialize)]
struct ApiResponse<T: Serialize> {
    success: bool,
    message: String,
    data: Option<T>,
    code: u16,
}

// 手动实现Responder特性
impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = match serde_json::to_string(&self) {
            Ok(body) => body,
            Err(e) => {
                // 序列化错误返回500状态码和错误信息
                return HttpResponse::InternalServerError()
                    .content_type("application/json")
                    .body(format!("{{\"success\":false,\"message\":\"序列化错误: {}\"}}", e));
            }
        };
        
        // 根据code字段确定HTTP状态码
        let status_code = match StatusCode::from_u16(self.code) {
            Ok(code) => code,
            Err(_) => StatusCode::OK,
        };

        HttpResponse::build(status_code)
            .content_type("application/json")
            .body(body)
    }
}

// 为ApiResponse实现静态构造方法
impl<T: Serialize> ApiResponse<T> {
    // 成功响应构造方法
    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            message: "操作成功".to_string(),
            data: Some(data),
            code: 200,
        }
    }
    
    // 错误响应构造方法
    pub fn error(message: &str, code: u16) -> Self {
        ApiResponse {
            success: false,
            message: message.to_string(),
            data: None,
            code,
        }
    }
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
        // 改进数据库锁错误处理
        let users = self.users.lock()
            .map_err(|e| AppError::new(&format!("数据库锁错误: {:?}", e)))?;
        
        users.get(&id)
            .cloned()
            .ok_or_else(|| AppError::new(&format!("用户 {} 不存在", id)))
    }
    
    fn add_user(&self, mut user: User) -> Result<User, AppError> {
        let mut users = self.users.lock()
            .map_err(|e| AppError::new(&format!("数据库锁错误: {:?}", e)))?;
        
        // 如果用户没有指定ID或ID已存在，生成新的唯一ID
        if user.id == 0 || users.contains_key(&user.id) {
            user.id = Self::generate_user_id();
        }
        
        users.insert(user.id, user.clone());
        Ok(user)
    }
    
    fn get_all_users(&self) -> Result<Vec<User>, AppError> {
        let users = self.users.lock()
            .map_err(|e| AppError::new(&format!("数据库锁错误: {:?}", e)))?;
        
        Ok(users.values().cloned().collect())
    }
    
    // 生成唯一的用户ID
    fn generate_user_id() -> u32 {
        NEXT_USER_ID.fetch_add(1, Ordering::Relaxed)
    }
}

// 使用error!宏处理错误，简化响应处理
async fn get_user(db: web::Data<Database>, user_id: web::Path<u32>) -> impl Responder {
    // 添加输入验证
    if *user_id == 0 {
        return ApiResponse::error("用户ID不能为0", 400);
    }
    
    // 使用error!宏处理错误，提供更丰富的上下文信息
    match db.get_user(*user_id) {
        Ok(user) => ApiResponse::success(user),
        Err(e) => ApiResponse::error(&format!("{}", error!("获取用户失败: {}", e)), 404)
    }
}

// 使用error!宏处理错误，简化响应处理
async fn create_user(db: web::Data<Database>, user: web::Json<User>) -> impl Responder {
    // 增强参数验证
    let new_user = user.into_inner();
    
    // 验证用户名
    if new_user.name.trim().is_empty() {
        return ApiResponse::error("用户名不能为空", 400);
    }
    
    // 简单的邮箱格式验证
    if !new_user.email.contains('@') || !new_user.email.contains('.') {
        return ApiResponse::error("邮箱格式无效", 400);
    }
    
    // 使用error!宏处理数据库操作错误，提供更丰富的上下文信息
    match db.add_user(new_user) {
        Ok(user) => {
            let mut response = ApiResponse::success(user);
            response.message = "用户创建成功".to_string();
            response.code = 201;
            response
        },
        Err(e) => ApiResponse::error(&format!("{}", error!("创建用户失败: {}", e)), 400)
    }
}

// 使用error!宏处理错误，简化响应处理
async fn get_all_users(db: web::Data<Database>) -> impl Responder {
    // 使用专门的get_all_users方法，改进错误处理
    match db.get_all_users() {
        Ok(users) => {
            let mut response = ApiResponse::success(users);
            response.message = "获取用户列表成功".to_string();
            response
        },
        Err(e) => ApiResponse::error(&format!("{}", error!("获取用户列表失败: {}", e)), 500)
    }
}

// 使用error!宏简化错误创建和格式化
async fn complex_operation(db: web::Data<Database>, params: web::Json<serde_json::Value>) -> impl Responder {
    // 验证必要参数
    let user_id = match params.get("user_id").and_then(|v| v.as_u64()) {
        Some(id) => id,
        None => return ApiResponse::error("无效的用户ID", 400)
    };
    
    // 验证user_id范围
    if user_id == 0 || user_id > u32::MAX as u64 {
        return ApiResponse::error("用户ID超出有效范围", 400);
    }
    
    let operation = match params.get("operation").and_then(|v| v.as_str()) {
        Some(op) => op,
        None => return ApiResponse::error("操作类型不能为空", 400)
    };
    
    // 模拟业务逻辑，支持多种操作类型
    match operation {
        "read" => {
            // 使用error!宏提供更丰富的上下文信息
            match db.get_user(user_id as u32) {
                Ok(user) => ApiResponse::success(user),
                Err(e) => ApiResponse::error(&format!("{}", error!("读取操作失败，用户ID: {}: {}", user_id, e)), 404)
            }
        },
        "validate" => {
            // 使用error!宏提供更丰富的上下文信息
            match db.get_user(user_id as u32) {
                Ok(user) => ApiResponse::success(user),
                Err(e) => ApiResponse::error(&format!("{}", error!("验证失败，用户ID: {}: {}", user_id, e)), 404)
            }
        },
        _ => {
            // 使用error!宏格式化错误信息
            ApiResponse::error(&format!("{}", error!("不支持的操作类型: {}", operation)), 400)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 设置更好的日志记录
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();
    
    println!("启动服务器...");
    println!("服务地址: http://127.0.0.1:8080");
    println!("API端点:");
    println!("  GET    /users          - 获取所有用户");
    println!("  GET    /users/{{id}}     - 获取指定用户");
    println!("  POST   /users          - 创建新用户");
    println!("  POST   /operation      - 执行复杂操作");
    
    // 创建数据库实例
    let db = web::Data::new(Database::new());
    
    // 启动服务器
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            // 使用增强的日志中间件
            .wrap(Logger::new("%r %s %b %Dms"))
            // 添加错误处理
            .default_service(
                web::route().to(|_req: HttpRequest| async move {
                    HttpResponse::build(StatusCode::NOT_FOUND)
                        .content_type("application/json")
                        .body(r#"{"data":null,"message":"接口不存在","code":404}"#)
                })
            )
            .route("/users", web::get().to(get_all_users))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users", web::post().to(create_user))
            .route("/operation", web::post().to(complex_operation))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
