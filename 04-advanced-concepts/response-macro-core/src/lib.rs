//! Core types and utilities for response-macro

use actix_web::{http::StatusCode, HttpRequest, HttpResponse, body::BoxBody};
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::fmt;
use std::error::Error;
use rand::random;

// 确保所有必要的类型都被导入
use std::convert::TryFrom;
use actix_web::http::header::{HeaderValue, HeaderName};

// 为HeaderName实现TryFrom扩展，提供更安全的转换
impl TryFrom<String> for HeaderName {
    type Error = actix_web::http::header::InvalidHeaderValue;
    
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

/// ContentFormat 枚举，表示支持的内容协商格式
#[derive(Debug, Clone, Copy)]
pub enum ContentFormat {
    Json,
    Text,
}

/// ApiError 结构体，用于表示 API 错误响应
#[derive(Debug, Clone, Deserialize)]
pub struct ApiError {
    pub success: bool,
    pub message: String,
    pub code: u16,
    pub details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    pub timestamp: u64
}

impl serde::Serialize for ApiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("ApiError", 7)?;
        state.serialize_field("success", &self.success)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("message", &self.message)?;
        state.serialize_field("details", &self.details)?;
        state.serialize_field("data", &self.data)?;
        state.serialize_field("trace_id", &self.trace_id)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.end()
    }
}

// 添加JsonSchema支持，用于API文档生成和数据验证
#[cfg(feature = "schema")]
impl schemars::JsonSchema for ApiError {
    fn schema_name() -> String {
        "ApiError".to_string()
    }
    
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let mut schema = gen.subschema_for::<Self>();
        if let Some(obj) = schema.as_object_mut() {
            // 增强schema描述
            obj.description = Some("API响应统一格式，支持成功和错误响应。".to_string());
        }
        schema
    }
}

impl ApiError {
    pub fn new(code: u16, message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            code,
            details: None,
            data: None,
            trace_id: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |d| d.as_secs()),
        }
    }
    
    pub fn with_details(mut self, details: Option<String>) -> Self {
        self.details = details.map(|d| serde_json::Value::String(d));
        self
    }
    
    pub fn with_data<T: serde::Serialize>(mut self, data: Option<T>) -> Self {
        self.data = data.map(|d| serde_json::to_value(d).ok()).flatten();
        self
    }
    
    pub fn with_trace(mut self) -> Self {
        let trace_id = format!("{:x}", random::<u32>());
        self.trace_id = Some(trace_id);
        self
    }
    
    pub fn from_error<E: std::fmt::Display>(code: u16, error: E) -> Self {
        Self::new(code, &error.to_string())
            .with_details(Some(error.to_string()))
            .with_trace()
    }
    
    pub fn from_status_code(code: u16) -> Self {
        match code {
            400 => Self::bad_request("请求参数错误"),
            401 => Self::unauthorized("未授权访问"),
            403 => Self::forbidden("拒绝访问"),
            404 => Self::not_found("资源不存在"),
            500 => Self::internal_error("服务器内部错误"),
            _ => Self::new(code, "未知错误"),
        }
    }
    
    pub fn bad_request(message: &str) -> Self {
        Self::new(400, message)
    }
    
    pub fn unauthorized(message: &str) -> Self {
        Self::new(401, message)
    }
    
    pub fn forbidden(message: &str) -> Self {
        Self::new(403, message)
    }
    
    pub fn not_found(message: &str) -> Self {
        Self::new(404, message)
    }
    
    pub fn internal_error(message: &str) -> Self {
        Self::new(500, message)
    }
    
    pub fn service_unavailable(message: &str) -> Self {
        Self::new(503, message)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ApiError: {}, code: {}, trace_id: {}", 
               self.message, self.code, 
               self.trace_id.as_deref().unwrap_or("unknown"))
    }
}

impl Error for ApiError {}

// 链式响应构建结构体
#[derive(Debug)]
pub struct WithHeader<T> {
    inner: T,
    headers: Vec<(String, String)>,
}

impl<T> WithHeader<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            headers: Vec::new(),
        }
    }

    pub fn with_header(mut self, name: &str, value: &str) -> Self {
        // 简单存储为字符串，在Responder实现中处理转换
        self.headers.push((name.to_string(), value.to_string()));
        self
    }

    pub fn with_status(self, status: u16) -> WithStatus<T> {
        WithStatus::new(self.inner, status, self.headers)
    }

    pub fn with_content_type(self, content_type: &str) -> WithContentType<T> {
        WithContentType::new(self.inner, content_type, self.headers)
    }
}

#[derive(Debug)]
pub struct WithStatus<T> {
    inner: T,
    status: u16,
    headers: Vec<(String, String)>,
}

impl<T> WithStatus<T> {
    pub fn new(inner: T, status: u16, headers: Vec<(String, String)>) -> Self {
        Self {
            inner,
            status,
            headers,
        }
    }

    pub fn with_header(mut self, name: &str, value: &str) -> Self {
        self.headers.push((name.to_string(), value.to_string()));
        self
    }

    pub fn with_content_type(self, content_type: &str) -> WithContentType<T> {
        WithContentType::new(self.inner, content_type, self.headers)
    }
}

#[derive(Debug)]
pub struct WithContentType<T> {
    inner: T,
    content_type: String,
    headers: Vec<(String, String)>,
}

impl<T> WithContentType<T> {
    pub fn new(inner: T, content_type: &str, headers: Vec<(String, String)>) -> Self {
        Self {
            inner,
            content_type: content_type.to_string(),
            headers,
        }
    }

    pub fn with_header(mut self, name: &str, value: &str) -> Self {
        self.headers.push((name.to_string(), value.to_string()));
        self
    }
}

// ResponseExt 扩展特性
pub trait ResponseExt: Sized {
    fn with_header(self, name: &str, value: &str) -> WithHeader<Self>;
    fn with_status(self, status: u16) -> WithStatus<Self>;
    fn with_content_type(self, content_type: &str) -> WithContentType<Self>;
}

impl<T: Serialize> ResponseExt for T {
    fn with_header(self, name: &str, value: &str) -> WithHeader<Self> {
        WithHeader::new(self).with_header(name, value)
    }

    fn with_status(self, status: u16) -> WithStatus<Self> {
        WithStatus::new(self, status, Vec::new())
    }

    fn with_content_type(self, content_type: &str) -> WithContentType<Self> {
        WithContentType::new(self, content_type, Vec::new())
    }
}

// 内容协商辅助函数
pub fn negotiate_content_type(_request: &actix_web::HttpRequest) -> ContentFormat {
    // 简化实现，始终返回JSON格式
    ContentFormat::Json
}

// 实现 ApiError 的 Responder 特性
impl actix_web::Responder for ApiError {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, request: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        // 确保状态码转换正确
        let status = match StatusCode::from_u16(self.code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR, // 无效状态码默认使用500
        };

        // 直接使用JSON格式，确保响应体完整
        let response_body = json!({
            "success": self.success,
            "message": self.message,
            "code": self.code,
            "details": self.details,
            "data": self.data,
            "trace_id": self.trace_id,
            "timestamp": self.timestamp
        });
        
        match serde_json::to_string(&response_body) {
            Ok(body) => {
                let mut response = actix_web::HttpResponse::build(status)
                    .content_type("application/json");
                // 始终添加跟踪ID到响应头
                if let Some(ref trace) = self.trace_id {
                    response = response.append_header(("X-Trace-Id", trace.clone()));
                }
                response.body(body)
            },
            Err(e) => {
                // 序列化失败时的降级处理
                let fallback_body = json!({
                    "success": false,
                    "message": "Failed to process error response",
                    "code": 500,
                    "details": format!("Serialization error: {}", e),
                    "timestamp": std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map_or(0, |d| d.as_secs())
                }).to_string();
                
                actix_web::HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .content_type("application/json")
                    .body(fallback_body)
            }
        }
    }
}

// 实现 WithHeader 的 Responder 特性
impl<T: Serialize> actix_web::Responder for WithHeader<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _request: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match serde_json::to_string(&self.inner) {
            Ok(body) => {
                let mut response = actix_web::HttpResponse::Ok()
                    .content_type("application/json");
                
                // 安全地添加所有头信息
                for (name, value) in self.headers {
                    // 使用HeaderName和HeaderValue确保类型安全
                    if let Ok(header_name) = HeaderName::try_from(name.as_str()) {
                        if let Ok(header_value) = HeaderValue::from_str(&value) {
                            response = response.append_header((header_name, header_value));
                        }
                    }
                }
                
                response.body(body)
            },
            Err(e) => {
                let error_message = format!("Serialization error: {}", e);
                actix_web::HttpResponse::InternalServerError()
                    .content_type("text/plain")
                    .body(error_message)
            }
        }
    }
}

// 实现 WithStatus 的 Responder 特性
impl<T: Serialize> actix_web::Responder for WithStatus<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, request: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let content_format = negotiate_content_type(request);
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::OK);
        
        match content_format {
            ContentFormat::Json => {
                match serde_json::to_string(&self.inner) {
                    Ok(body) => {
                        let mut response = actix_web::HttpResponse::build(status)
                            .content_type("application/json");
                        for (name, value) in self.headers {
                            response = response.append_header((name, value));
                        }
                        response.body(body)
                    },
                    Err(e) => {
                        let error_message = format!("Serialization error: {}", e);
                        actix_web::HttpResponse::InternalServerError()
                            .content_type("text/plain")
                            .body(error_message)
                    }
                }
            },
            ContentFormat::Text => {
                match serde_json::to_string(&self.inner) {
                    Ok(json_str) => {
                        let mut response = actix_web::HttpResponse::build(status)
                            .content_type("text/plain");
                        for (name, value) in self.headers {
                            response = response.append_header((name, value));
                        }
                        response.body(json_str)
                    },
                    Err(e) => {
                        actix_web::HttpResponse::InternalServerError()
                            .content_type("text/plain")
                            .body(e.to_string())
                    }
                }
            }
        }
    }
}

// 实现 WithContentType 的 Responder 特性
impl<T: Serialize> actix_web::Responder for WithContentType<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, request: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match serde_json::to_string(&self.inner) {
            Ok(body) => {
                let mut response = actix_web::HttpResponse::Ok()
                    .content_type(&self.content_type);
                for (name, value) in self.headers {
                    response = response.append_header((name, value));
                }
                response.body(body)
            },
            Err(e) => {
                actix_web::HttpResponse::InternalServerError()
                    .content_type("text/plain")
                    .body(format!("Serialization error: {}", e))
            }
        }
    }
}