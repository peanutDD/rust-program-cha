use thiserror::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use actix_web::{http::StatusCode, ResponseError};

/// 应用程序错误类型枚举
#[derive(Debug, Error)]
pub enum AppError {
    /// 数据库错误
    #[error("数据库错误: {0}")]
    Database(String),
    
    /// 资源未找到错误
    #[error("资源未找到: {0}")]
    NotFound(String),
    
    /// 验证错误
    #[error("验证错误: {0}")]
    Validation(String),
    
    /// 权限错误
    #[error("权限不足: {0}")]
    PermissionDenied(String),
    
    /// 内部服务器错误
    #[error("内部服务器错误: {0}")]
    Internal(String),
    
    /// 业务规则错误
    #[error("业务规则错误: {0}")]
    BusinessRule(String),
    
    /// 并发错误
    #[error("并发错误: {0}")]
    Concurrency(String),
}

impl AppError {
    /// 创建数据库错误
    pub fn database(message: &str) -> Self {
        AppError::Database(message.to_string())
    }
    
    /// 创建资源未找到错误
    pub fn not_found(message: &str) -> Self {
        AppError::NotFound(message.to_string())
    }
    
    /// 创建验证错误
    pub fn validation(message: &str) -> Self {
        AppError::Validation(message.to_string())
    }
    
    /// 创建权限错误
    pub fn permission_denied(message: &str) -> Self {
        AppError::PermissionDenied(message.to_string())
    }
    
    /// 创建内部服务器错误
    pub fn internal(message: &str) -> Self {
        AppError::Internal(message.to_string())
    }
    
    /// 创建业务规则错误
    pub fn business_rule(message: &str) -> Self {
        AppError::BusinessRule(message.to_string())
    }
    
    /// 创建并发错误
    pub fn concurrency(message: &str) -> Self {
        AppError::Concurrency(message.to_string())
    }
    
    /// 获取对应的HTTP状态码
    pub fn status_code(&self) -> u16 {
        match self {
            AppError::NotFound(_) => 404,
            AppError::Validation(_) => 400,
            AppError::PermissionDenied(_) => 403,
            AppError::BusinessRule(_) => 422,
            AppError::Concurrency(_) => 409,
            _ => 500, // 其他所有错误默认为500
        }
    }
}

/// 实现ResponseError trait，使AppError可以作为actix-web的错误处理
impl ResponseError for AppError {
    fn error_response(&self) -> actix_web::HttpResponse {
        let status_code = StatusCode::from_u16(self.status_code())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        
        let api_error = ApiError::new(&self.to_string(), self.status_code());
        
        actix_web::HttpResponse::build(status_code)
            .content_type("application/json")
            .json(api_error)
    }
    
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status_code())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

use serde::Serialize;

/// 通用的API响应结构
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiError {
    /// 是否成功
    pub success: bool,
    /// 消息
    pub message: String,
    /// 错误码
    pub code: u16,
    /// 数据
    pub data: Option<serde_json::Value>,
}

impl ApiError {
    /// 创建新的API错误
    pub fn new(message: &str, code: u16) -> Self {
        ApiError {
            success: false,
            message: message.to_string(),
            code,
            data: None,
        }
    }
    
    /// 创建带数据的API错误
    pub fn with_data(message: &str, code: u16, data: serde_json::Value) -> Self {
        ApiError {
            success: false,
            message: message.to_string(),
            code,
            data: Some(data),
        }
    }
}

/// 从AppError转换为ApiError
impl From<AppError> for ApiError {
    fn from(error: AppError) -> Self {
        ApiError::new(&error.to_string(), error.status_code())
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "ApiError: message={}, code={}", self.message, self.code)
    }
}