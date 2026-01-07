//! Core types and utilities for response-macro

use actix_web::http::StatusCode;
use serde::{Serialize, Deserialize, ser::SerializeMap};
use serde_json::json;
use std::fmt;
use std::error::Error;
use rand::random;

// 确保所有必要的类型都被导入
use actix_web::http::header::{HeaderValue, HeaderName};

// 辅助函数，用于安全地创建HeaderName
fn create_header_name(name: &str) -> Option<HeaderName> {
    HeaderName::try_from(name).ok()
}

// 辅助函数，用于安全地创建HeaderValue
fn create_header_value(value: &str) -> Option<HeaderValue> {
    HeaderValue::from_str(value).ok()
}

/// ContentFormat 枚举，表示支持的内容协商格式
#[derive(Debug, Clone, Copy)]
pub enum ContentFormat {
    Json,
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
        // 使用正确的方式序列化结构体
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("success", &self.success)?;
        map.serialize_entry("message", &self.message)?;
        map.serialize_entry("code", &self.code)?;
        if let Some(details) = &self.details {
            map.serialize_entry("details", details)?;
        }
        if let Some(data) = &self.data {
            map.serialize_entry("data", data)?;
        }
        if let Some(trace_id) = &self.trace_id {
            map.serialize_entry("trace_id", trace_id)?;
        }
        map.serialize_entry("timestamp", &self.timestamp)?;
        map.end()
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
    
    pub fn with_details(mut self, details: Option<&str>) -> Self {
        self.details = details.map(|d| serde_json::Value::String(d.to_string()));
        self
    }
    
    pub fn with_data<T: serde::Serialize>(mut self, data: Option<T>) -> Self {
        self.data = data.and_then(|d| serde_json::to_value(d).ok());
        self
    }
    
    pub fn with_trace(mut self, trace_id: Option<&str>) -> Self {
        let tid = match trace_id {
            Some(id) => id.to_string(),
            None => format!("{:x}", random::<u32>()),
        };
        self.trace_id = Some(tid);
        self
    }
    
    pub fn success(message: &str, code: u16) -> Self {
        let mut err = Self::new(code, message);
        err.success = true;
        err
    }

    pub fn success_with_data<T: serde::Serialize>(data: T, code: u16) -> Self {
        let mut err = Self::success("success", code);
        err.data = serde_json::to_value(data).ok();
        err
    }
    
    pub fn from_error<E: std::fmt::Display>(code: u16, error: E) -> Self {
        Self::new(code, &error.to_string())
            .with_details(Some(&error.to_string()))
            .with_trace(None)
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

    pub fn is_success(&self) -> bool {
        self.success
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn get_code(&self) -> u16 {
        self.code
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.success {
            // 对于成功响应，使用 SuccessResponse 作为前缀，便于与错误区分
            write!(
                f,
                "SuccessResponse: {}, code: {}, trace_id: {}",
                self.message,
                self.code,
                self.trace_id.as_deref().unwrap_or("unknown")
            )
        } else {
            // 对于失败响应，保持 ApiError 前缀，强调错误语义
            write!(
                f,
                "ApiError: {}, code: {}, trace_id: {}",
                self.message,
                self.code,
                self.trace_id.as_deref().unwrap_or("unknown")
            )?;

            // 如果存在details字段，在Display中显式展示，便于调试与测试断言
            if let Some(details) = &self.details {
                write!(f, ", Details: {}", details)?;
            }

            Ok(())
        }
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

    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        // 简单存储为字符串，在Responder实现中处理转换
        self.headers.push((name.into(), value.into()));
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

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        // 简单存储为字符串，在Responder实现中处理转换
        self.headers.push((key.to_string(), value.to_string()));
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

// 为ContentFormat实现Display trait
impl std::fmt::Display for ContentFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentFormat::Json => write!(f, "application/json"),
        }
    }
}

// 实现 ApiError 的 Responder 特性
impl actix_web::Responder for ApiError {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _request: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        // 确保状态码转换正确
        let status = match StatusCode::from_u16(self.code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR, // 无效状态码默认使用500
        };

        // 直接使用JSON格式，确保响应体完整
        let message = self.message.clone(); // 避免临时值被丢弃
        let details = self.details.clone();
        let trace_id = self.trace_id.clone();
        let data = self.data.clone();
        
        let response_body = json!({
            "success": self.success,
            "message": message,
            "code": self.code,
            "details": details,
            "data": data,
            "trace_id": trace_id,
            "timestamp": self.timestamp
        });
        
        match serde_json::to_string(&response_body) {
            Ok(body) => {
                // 使用链式调用构建响应
                let mut response = actix_web::HttpResponse::build(status);
                response.content_type("application/json");
                // 始终添加跟踪ID到响应头
                if let Some(trace) = trace_id {
                    response.append_header(("X-Trace-Id", trace));
                }
                response.body(body)
            },
            Err(e) => {
                // 序列化失败时的降级处理
                let details = format!("Serialization error: {}", e);
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_or(0, |d| d.as_secs());
                
                let fallback_body = format!("{{\"success\": false, \"message\": \"Failed to process error response\", \"code\": 500, \"details\": \"{}\", \"timestamp\": {}}}", details, timestamp);
                
                // 使用链式调用构建响应
                let mut response = actix_web::HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR);
                response.content_type("application/json");
                response.body(fallback_body)
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
                // 使用链式调用构建响应
                let mut response = actix_web::HttpResponse::Ok();
                response.content_type("application/json");
                
                // 安全地添加所有头信息
                for (name_str, value_str) in self.headers {
                    // 使用辅助函数安全创建头信息
                    let name_clone = name_str.clone();
                    let value_clone = value_str.clone();
                    if let (Some(header_name), Some(header_value)) = (create_header_name(&name_clone), create_header_value(&value_clone)) {
                        response.append_header((header_name, header_value));
                    }
                }
                
                response.body(body)
            },
            Err(e) => {
                // 使用JSON格式的错误响应
                let error_response = json!({"success": false, "message": "Serialization error", "details": e.to_string(), "code": 500, "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())});
                let error_body = serde_json::to_string(&error_response).unwrap_or_else(|_| "{\"success\": false, \"message\": \"Failed to process error response\"}".to_string());
                
                let mut response = actix_web::HttpResponse::InternalServerError();
                response.content_type("application/json");
                response.body(error_body)
            }
        }
    }
}

// 实现 WithStatus 的 Responder 特性
impl<T: Serialize> actix_web::Responder for WithStatus<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _request: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::OK);
        
        // 直接使用JSON格式，简化实现
        match serde_json::to_string(&self.inner) {
            Ok(body) => {
                // 使用链式调用构建响应
                let mut response = actix_web::HttpResponse::build(status);
                response.content_type("application/json");
                
                // 安全地添加所有头信息
                for (name_str, value_str) in self.headers {
                    // 使用辅助函数安全创建头信息
                    let name_clone = name_str.clone();
                    let value_clone = value_str.clone();
                    if let (Some(header_name), Some(header_value)) = (create_header_name(&name_clone), create_header_value(&value_clone)) {
                        response.append_header((header_name, header_value));
                    }
                }
                
                response.body(body)
            },
            Err(e) => {
                // 使用JSON格式的错误响应
                let error_response = json!({"success": false, "message": "Serialization error", "details": e.to_string(), "code": 500, "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())});
                let error_body = serde_json::to_string(&error_response).unwrap_or_else(|_| "{\"success\": false, \"message\": \"Failed to process error response\"}".to_string());
                
                let mut response = actix_web::HttpResponse::InternalServerError();
                response.content_type("application/json");
                response.body(error_body)
            }
        }
    }
}

// 实现 WithContentType 的 Responder 特性
impl<T: Serialize> actix_web::Responder for WithContentType<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _request: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match serde_json::to_string(&self.inner) {
            Ok(body) => {
                // 使用链式调用构建响应
                let content_type_value = self.content_type.clone();
                let mut response = actix_web::HttpResponse::Ok();
                response.content_type(content_type_value);
                
                // 安全地添加所有头信息
                for (name_str, value_str) in self.headers {
                    // 使用辅助函数安全创建头信息
                    let name_clone = name_str.clone();
                    let value_clone = value_str.clone();
                    if let (Some(header_name), Some(header_value)) = (create_header_name(&name_clone), create_header_value(&value_clone)) {
                        response.append_header((header_name, header_value));
                    }
                }
                
                response.body(body)
            },
            Err(e) => {
                // 使用JSON格式的错误响应，即使是content_type错误
                let error_response = json!({
                    "success": false,
                    "message": "Serialization error",
                    "details": e.to_string(),
                    "code": 500,
                    "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
                });
                let error_body = serde_json::to_string(&error_response).unwrap_or_else(|_| "{\"success\": false, \"message\": \"Failed to process error response\"}".to_string());
                
                let mut response = actix_web::HttpResponse::InternalServerError();
                response.content_type("application/json");
                response.body(error_body)
            }
        }
    }
}
