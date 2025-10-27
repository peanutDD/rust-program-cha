use crate::models::{CreateUserRequest, UpdateUserRequest, UserQueryParams};
use crate::services::UserService;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use std::sync::Arc;

/// 用户处理器 - 处理HTTP请求并调用用户服务
#[derive(Clone, Debug)]
pub struct UserHandler {
    user_service: Arc<UserService>,
}

impl UserHandler {
    /// 创建新的用户处理器实例
    pub fn new(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }
}

/// 获取单个用户
async fn get_user(
    handler: web::Data<UserHandler>,
    user_id: web::Path<u32>,
    _req: HttpRequest,
) -> impl Responder {
    // 输入验证
    if *user_id == 0 {
        return HttpResponse::BadRequest().json(json!({"success": false, "message": "用户ID不能为0", "data": null, "code": 400}));
    }

    // 调用服务层获取用户
    match handler.user_service.get_user(*user_id).await {
        Ok(user) => HttpResponse::Ok().json(json!({"success": true, "message": "获取用户成功", "data": user, "code": 200})),
        Err(err) => {
                let status_code = err.status_code();
                HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)).json(json!({"success": false, "message": err.to_string(), "data": null, "code": status_code}))
            }
    }
}

/// 创建用户
async fn create_user(
    handler: web::Data<UserHandler>,
    user_request: web::Json<CreateUserRequest>,
    _req: HttpRequest,
) -> impl Responder {
    // 调用服务层创建用户
    match handler
        .user_service
        .create_user(user_request.into_inner())
        .await
    {
        Ok(user) => HttpResponse::Created().json(json!({"success": true, "message": "创建用户成功", "data": user, "code": 201})),
        Err(err) => {
                let status_code = err.status_code();
                HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)).json(json!({"success": false, "message": err.to_string(), "data": null, "code": status_code}))
            }
    }
}

/// 更新用户
async fn update_user(
    handler: web::Data<UserHandler>,
    user_id: web::Path<u32>,
    user_request: web::Json<UpdateUserRequest>,
    _req: HttpRequest,
) -> impl Responder {
    // 输入验证
    if *user_id == 0 {
        return HttpResponse::BadRequest().json(json!({"success": false, "message": "用户ID不能为0", "data": null, "code": 400}));
    }

    // 调用服务层更新用户
    match handler
        .user_service
        .update_user(*user_id, user_request.into_inner())
        .await
    {
        Ok(user) => HttpResponse::Ok().json(json!({"success": true, "message": "更新用户成功", "data": user, "code": 200})),
        Err(err) => {
                let status_code = err.status_code();
                HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)).json(json!({"success": false, "message": err.to_string(), "data": null, "code": status_code}))
            }
    }
}

/// 获取用户列表
async fn list_users(
    handler: web::Data<UserHandler>,
    query_params: web::Query<UserQueryParams>,
    _req: HttpRequest,
) -> impl Responder {
    // 调用服务层获取用户列表
    match handler
        .user_service
        .list_users(query_params.into_inner())
        .await
    {
        Ok(users) => HttpResponse::Ok().json(json!({"success": true, "message": "获取用户列表成功", "data": users, "code": 200})),
        Err(err) => {
                let status_code = err.status_code();
                HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)).json(json!({"success": false, "message": err.to_string(), "data": null, "code": status_code}))
            }
    }
}

/// 删除用户
async fn delete_user(
    handler: web::Data<UserHandler>,
    user_id: web::Path<u32>,
    _req: HttpRequest,
) -> impl Responder {
    // 输入验证
    if *user_id == 0 {
        return HttpResponse::BadRequest().json(json!({"success": false, "message": "用户ID不能为0", "data": null, "code": 400}));
    }

    // 调用服务层删除用户
    match handler.user_service.delete_user(*user_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
                let status_code = err.status_code();
                HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)).json(json!({"success": false, "message": err.to_string(), "data": null, "code": status_code}))
            }
    }
}

/// 批量获取用户
async fn get_users_by_ids(
    handler: web::Data<UserHandler>,
    user_ids: web::Json<Vec<u32>>,
    _req: HttpRequest,
) -> impl Responder {
    // 调用服务层批量获取用户
    match handler.user_service.get_users_by_ids(&user_ids).await {
        Ok(users) => HttpResponse::Ok().json(json!({"success": true, "message": "批量获取用户成功", "data": users, "code": 200})),
        Err(err) => {
                let status_code = err.status_code();
                HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)).json(json!({"success": false, "message": err.to_string(), "data": null, "code": status_code}))
            }
    }
}

/// 激活用户
async fn activate_user(
    handler: web::Data<UserHandler>,
    user_id: web::Path<u32>,
    _req: HttpRequest,
) -> impl Responder {
    // 输入验证
    if *user_id == 0 {
        return HttpResponse::BadRequest().json(json!({"success": false, "message": "用户ID不能为0", "data": null, "code": 400}));
    }

    // 调用服务层激活用户
    match handler
        .user_service
        .toggle_user_status(*user_id, true)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(json!({"success": true, "message": "用户激活成功", "data": user, "code": 200})),
        Err(err) => {
                let status_code = err.status_code();
                HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)).json(json!({"success": false, "message": err.to_string(), "data": null, "code": status_code}))
            }
    }
}

/// 停用用户
async fn deactivate_user(
    handler: web::Data<UserHandler>,
    user_id: web::Path<u32>,
    _req: HttpRequest,
) -> impl Responder {
    // 输入验证
    if *user_id == 0 {
        return HttpResponse::BadRequest().json(json!({"success": false, "message": "用户ID不能为0", "data": null, "code": 400}));
    }

    // 调用服务层停用用户
    match handler
        .user_service
        .toggle_user_status(*user_id, false)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(json!({"success": true, "message": "用户停用成功", "data": user, "code": 200})),
        Err(err) => {
                let status_code = err.status_code();
                HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)).json(json!({"success": false, "message": err.to_string(), "data": null, "code": status_code}))
            }
    }
}

use serde::{Deserialize, Serialize};

/// 邮箱检查请求参数
#[derive(Debug, Deserialize)]
struct EmailCheckQuery {
    email: String
}

/// 检查邮箱是否已被使用
async fn check_email(
    handler: web::Data<UserHandler>,
    query: web::Query<EmailCheckQuery>,
    _req: HttpRequest,
) -> impl Responder {
    // 获取并验证邮箱参数
    let email_value = query.email.trim().to_string();

    if email_value.is_empty() {
        return HttpResponse::BadRequest().json(json!({"success": false, "message": "邮箱不能为空", "data": null, "code": 400}));
    }

    // 验证邮箱格式（简单验证）
    if !email_value.contains('@') || !email_value.contains('.') {
        return HttpResponse::BadRequest().json(json!({"success": false, "message": "邮箱格式无效", "data": null, "code": 400}));
    }

    // 调用服务层检查邮箱
    match handler.user_service.is_email_used(&email_value).await {
        Ok(is_used) => HttpResponse::Ok().json(json!({"success": true, "message": "邮箱检查成功", "data": {"is_used": is_used, "email": email_value}, "code": 200})),
        Err(err) => {
                let status_code = err.status_code();
                HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)).json(json!({"success": false, "message": err.to_string(), "data": null, "code": status_code}))
            }
    }
}

/// 注册路由
pub fn register_routes(cfg: &mut web::ServiceConfig, user_handler: Arc<UserHandler>) {
    let handler_data = web::Data::new(user_handler);

    cfg.app_data(handler_data.clone()).service(
        web::scope("/api/users")
            // 基本CRUD操作
            .route("", web::get().to(list_users))
            .route("", web::post().to(create_user))
            .route("/{id}", web::get().to(get_user))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user))
            // 批量操作
            .route("/batch", web::post().to(get_users_by_ids))
            // 状态管理
            .route("/{id}/activate", web::post().to(activate_user))
            .route("/{id}/deactivate", web::post().to(deactivate_user))
            // 辅助功能
            .route("/check-email", web::get().to(check_email)),
    );
}
