use serde::{Deserialize, Serialize};
use response_macro::response;
use response_macro_core::ApiError;
use std::time::Duration;
use tokio::time::sleep;

// 测试结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
struct TestUser {
    id: u32,
    name: String,
    email: String,
}

// 自定义错误类型
#[derive(Debug)]
struct AppError {
    message: String,
    code: u16,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppError {}

// 测试ApiError基本功能
#[tokio::test]
async fn test_api_error_basic() {
    // 测试成功响应
    let success = ApiError::success("操作成功", 200);
    assert!(success.success);
    assert_eq!(success.code, 200);
    assert_eq!(success.message, "操作成功");
    
    // 测试错误响应
    let error = ApiError::bad_request("请求参数错误");
    assert!(!error.success);
    assert_eq!(error.code, 400);
    assert_eq!(error.message, "请求参数错误");
}

// 测试ApiError链式调用
#[tokio::test]
async fn test_api_error_chain() {
    let error = ApiError::internal_error("服务器错误")
        .with_details(Some("数据库连接失败"))
        .with_trace(Some("trace-123"));
    
    assert!(!error.success);
    assert_eq!(error.code, 500);
    assert_eq!(error.message, "服务器错误");
    assert!(error.details.is_some());
    assert!(error.trace_id.is_some());
    assert_eq!(error.trace_id.unwrap(), "trace-123");
}

// 测试ApiError数据字段
#[tokio::test]
async fn test_api_error_data() {
    let user = TestUser {
        id: 1,
        name: "测试用户".to_string(),
        email: "test@example.com".to_string(),
    };
    
    let user_value = serde_json::to_value(user.clone()).unwrap();
    let success = ApiError::success_with_data(user_value, 200);
    
    assert!(success.success);
    assert!(success.data.is_some());
    
    // 验证数据内容
    let data = success.data.unwrap();
    assert_eq!(data["id"].as_u64().unwrap(), 1);
    assert_eq!(data["name"].as_str().unwrap(), "测试用户");
}

// 测试ApiError从状态码创建
#[tokio::test]
async fn test_api_error_from_status_code() {
    let not_found = ApiError::from_status_code(404);
    assert!(!not_found.success);
    assert_eq!(not_found.code, 404);
    assert_eq!(not_found.message, "资源不存在");
    
    let internal_error = ApiError::from_status_code(500);
    assert!(!internal_error.success);
    assert_eq!(internal_error.code, 500);
    assert_eq!(internal_error.message, "服务器内部错误");
}

// 测试response宏的基本功能
#[response]
async fn test_response_basic() -> Result<TestUser, AppError> {
    let user = TestUser {
        id: 1,
        name: "测试用户".to_string(),
        email: "test@example.com".to_string(),
    };
    Ok(user)
}

// 测试response宏的错误处理
#[response(error_code = 400)]
async fn test_response_error() -> Result<TestUser, AppError> {
    Err(AppError {
        message: "测试错误".to_string(),
        code: 400,
    })
}

// 测试response宏的超时功能
#[response(timeout_seconds = 1)]
async fn test_response_timeout() -> Result<TestUser, AppError> {
    // 睡眠2秒，触发超时
    sleep(Duration::from_secs(2)).await;
    Ok(TestUser {
        id: 1,
        name: "超时用户".to_string(),
        email: "timeout@example.com".to_string(),
    })
}

// 测试自定义转换函数
fn transform_success(user: TestUser) -> serde_json::Value {
    serde_json::json!({"transformed_id": user.id, "transformed_name": user.name.to_uppercase()})
}

fn transform_error(message: String) -> String {
    format!("转换后的错误: {}", message)
}

// 测试响应转换功能
#[response(transform_success = "transform_success", transform_error = "transform_error")]
async fn test_response_transform(should_fail: bool) -> Result<TestUser, AppError> {
    if should_fail {
        Err(AppError { message: "需要转换的错误".to_string(), code: 400 })
    } else {
        Ok(TestUser { id: 1, name: "test".to_string(), email: "test@example.com".to_string() })
    }
}
