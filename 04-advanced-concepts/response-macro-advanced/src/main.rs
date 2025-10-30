use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, middleware::Logger};
use actix_cors::Cors;
use log::{info};
use std::sync::Arc;
use std::io::Result as IoResult;
use serde_json;
use chrono::Utc;

mod models;
mod services;
mod handlers;
use models::error::AppError;
use services::{DatabaseService, UserService};
use handlers::UserHandler;

/// 健康检查端点
async fn health_check(_req: HttpRequest) -> impl Responder {
    // 直接构建HTTP响应
    HttpResponse::Ok()
        .content_type("application/j
        .json(serde_json::json!({
            "success": true,
            "message": "服务运行正常",
            "code": 200,
            "data": {"status": "healthy", "timestamp": chrono::Utc::now().to_rfc3339(), "service": "response-macro-advanced", "version": "1.0.0"}
        }))
}

/// 根路径处理
async fn root(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(serde_json::json!({
            "success": true,
            "message": "高级Response宏库示例API",
            "code": 200,
            "data": {
                "version": "1.0.0",
                "docs": "/api/docs",
                "health": "/api/health"
            }
        }))
}

// 错误处理将由response-macro自动处理

/// 设置应用程序路由和中间件
fn configure_app(app: &mut web::ServiceConfig) {
    // 初始化服务
    let db_service = Arc::new(DatabaseService::new());
    let user_service = Arc::new(UserService::new(db_service));
    let user_handler = Arc::new(UserHandler::new(user_service));
    
    // 注册API路由
    app
        // 基础路由
        .route("/", web::get().to(root))
        .route("/api/health", web::get().to(health_check))
        
        // 注册路由
        .configure(|cfg| {
            // 注册基本路由
            cfg.route("/health", web::get().to(health_check))
               .route("/", web::get().to(root));
            // 注册用户相关路由
            handlers::user::register_routes(cfg, user_handler.clone());
        })
        // 注册JSON错误处理
        .app_data(web::JsonConfig::default().error_handler(|err, _req| {
            actix_web::error::ErrorBadRequest(format!("请求体解析错误: {}", err))
        }))
        
        // 404处理
        .default_service(
            web::route().to(|_req: HttpRequest| async move {
                HttpResponse::NotFound()
                    .content_type("application/json")
                    .json(serde_json::json!({
                        "success": false,
                        "message": "请求的资源不存在",
                        "code": 404,
                        "data": null
                    }))
            })
        );
}

#[actix_web::main]
async fn main() -> IoResult<()> {
    // 初始化日志系统
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // 显示启动信息
    info!("正在启动高级Response宏库示例API服务器...");
    
    // 配置服务器
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        
        App::new()
            // 添加中间件
            .wrap(cors)
            // .wrap(Logger::default())
            
            // 自定义错误处理
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                AppError::validation(&format!("请求体解析错误: {}", err)).into()
            }))
            
            .app_data(web::QueryConfig::default().error_handler(|err, _| {
                AppError::validation(&format!("查询参数解析错误: {}", err)).into()
            }))
            
            .app_data(web::PathConfig::default().error_handler(|err, _| {
                AppError::validation(&format!("路径参数解析错误: {}", err)).into()
            }))
            
            // 配置路由
            .configure(configure_app)
    })
    .bind("127.0.0.1:8081")?
    .workers(num_cpus::get() * 2); // 使用CPU核心数的两倍作为工作进程数
    
    // 启动服务器
    info!("服务器已启动，监听地址: http://127.0.0.1:8081");
    info!("可用API端点:");
    info!("  GET    /                  - 根路径");
    info!("  GET    /api/health        - 健康检查");
    info!("  GET    /api/users         - 获取用户列表");
    info!("  POST   /api/users         - 创建新用户");
    info!("  GET    /api/users/{{id}}    - 获取指定用户");
    info!("  PUT    /api/users/{{id}}    - 更新用户信息");
    info!("  DELETE /api/users/{{id}}    - 删除用户");
    info!("  POST   /api/users/batch   - 批量获取用户");
    info!("  POST   /api/users/{{id}}/activate    - 激活用户");
    info!("  POST   /api/users/{{id}}/deactivate  - 停用用户");
    info!("  GET    /api/users/check-email?email=xxx - 检查邮箱是否可用");
    
    server.run().await
}