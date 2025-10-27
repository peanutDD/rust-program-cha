use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result, Error, middleware::Logger, web::Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

// 导入我们的宏库和必要的类型
use response_macro::{Response, response, error};
use response_macro::{ResponseData};

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

// 定义ResponseData特性，用于提取响应数据
pub trait ResponseData {
    type Data;
}

// 定义API响应结构体，并使用我们的Response宏
#[derive(Debug, Serialize, Response)]
struct ApiResponse<T> {
    data: T,
    message: String,
    code: u16,
}

// 为ApiResponse实现ResponseData特性，用于提取响应数据类型
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

// 手动使用ApiResponse和error!宏
async fn get_all_users(db: web::Data<Database>) -> impl Responder {
    let users = match db.users.lock() {
        Ok(users) => users,
        Err(_) => {
            return Json(ApiResponse::error("数据库锁错误", 500));
        }
    };
    
    let users_vec: Vec<&User> = users.values().collect();
    // 使用ApiResponse的success方法创建成功响应
    Json(ApiResponse::success(users_vec))
}

// 使用error!宏简化错误创建和格式化
#[response(success_code = 200)]
async fn complex_operation(db: web::Data<Database>, params: web::Json<serde_json::Value>) -> Result<User, AppError> {
    // 一些复杂操作，使用error!宏简化错误处理
    let user_id = params.get("user_id")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| AppError::new("无效的用户ID"))?;
    
    let operation = params.get("operation")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::new("操作类型不能为空"))?;
    
    // 模拟业务逻辑
    if operation != "read" {
        return Err(AppError::new(&format!("不支持的操作类型: {}", operation)));
    }
    
    // 获取用户并返回
    let user = db.get_user(user_id as u32)
        .map_err(|e| AppError::new(&format!("操作失败: {}", e)))?;
    
    Ok(user)
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
            .route("/operation", web::post().to(complex_operation))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
