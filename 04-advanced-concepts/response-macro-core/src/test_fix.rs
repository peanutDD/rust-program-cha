use crate::{ApiError, WithHeader, ContentFormat};
use actix_web::http::StatusCode;
use serde_json;

// 测试ApiError的基本功能
fn test_api_error() {
    let error = ApiError::bad_request("Invalid input");
    assert_eq!(error.status_code, StatusCode::BAD_REQUEST.as_u16());
    assert_eq!(error.message, "Invalid input");
    assert!(error.timestamp.is_some());
}

// 测试WithHeader的基本功能
fn test_with_header() {
    let data = serde_json::json!({ "key": "value" });
    let with_header = WithHeader::new(data)
        .with_header("X-Custom-Header", "custom-value");
    assert_eq!(with_header.headers.len(), 1);
    assert_eq!(with_header.headers[0], ("X-Custom-Header".to_string(), "custom-value".to_string()));
}

// 测试ContentFormat枚举
fn test_content_format() {
    let format = ContentFormat::Json;
    assert_eq!(format.to_string(), "application/json");
}

fn main() {
    println!("Testing response-macro-core fixes...");
    test_api_error();
    test_with_header();
    test_content_format();
    println!("All tests passed!");
}