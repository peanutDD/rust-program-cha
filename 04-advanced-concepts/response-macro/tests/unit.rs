use response_macro_core::ApiError;
use serde_json::json;
use std::error::Error;

// 测试ApiError的基本构造函数
#[test]
fn test_api_error_constructors() {
    // 测试new构造函数
    let error = ApiError::new(404, "资源不存在");
    assert!(!error.success);
    assert_eq!(error.code, 404);
    assert_eq!(error.message, "资源不存在");
    
    // 测试success构造函数
    let success = ApiError::success("操作成功", 200);
    assert!(success.success);
    assert_eq!(success.code, 200);
    assert_eq!(success.message, "操作成功");
    
    // 测试success_with_data构造函数
    let data = json!({
        "id": 1,
        "name": "测试数据"
    });
    let success_with_data = ApiError::success_with_data(data.clone(), 201);
    assert!(success_with_data.success);
    assert_eq!(success_with_data.code, 201);
    assert!(success_with_data.data.is_some());
    assert_eq!(success_with_data.data.unwrap(), data);
}

// 测试ApiError的快捷方法
#[test]
fn test_api_error_shortcut_methods() {
    // 测试客户端错误方法
    let bad_request = ApiError::bad_request("参数错误");
    assert_eq!(bad_request.code, 400);
    
    let unauthorized = ApiError::unauthorized("未授权");
    assert_eq!(unauthorized.code, 401);
    
    let forbidden = ApiError::forbidden("禁止访问");
    assert_eq!(forbidden.code, 403);
    
    let not_found = ApiError::not_found("资源未找到");
    assert_eq!(not_found.code, 404);
    
    // 测试服务器错误方法
    let internal_error = ApiError::internal_error("服务器错误");
    assert_eq!(internal_error.code, 500);
    
    let service_unavailable = ApiError::service_unavailable("服务不可用");
    assert_eq!(service_unavailable.code, 503);
}

// 测试ApiError的实用方法
#[test]
fn test_api_error_utils() {
    let success = ApiError::success("成功", 200);
    assert!(success.is_success());
    assert_eq!(success.get_message(), "成功");
    assert_eq!(success.get_code(), 200);
    
    let error = ApiError::bad_request("失败");
    assert!(!error.is_success());
    assert_eq!(error.get_message(), "失败");
    assert_eq!(error.get_code(), 400);
}

// 测试ApiError的错误转换
#[test]
fn test_api_error_from_error() {
    // 自定义错误类型
    #[derive(Debug)]
    struct TestError {
        message: String,
    }
    
    impl std::fmt::Display for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.message)
        }
    }
    
    impl Error for TestError {
        fn source(&self) -> Option<&( dyn Error + 'static )> {
            None
        }
    }
    
    let test_error = TestError { message: "测试错误".to_string() };
    let api_error = ApiError::from_error(400, test_error);
    
    assert!(!api_error.success);
    assert_eq!(api_error.code, 400);
    assert_eq!(api_error.message, "测试错误");
}

// 测试ApiError的格式化输出
#[test]
fn test_api_error_display() {
    let success = ApiError::success("操作成功", 200);
    let success_str = format!("{}", success);
    assert!(success_str.contains("SuccessResponse"));
    assert!(success_str.contains("200"));
    assert!(success_str.contains("操作成功"));
    
    let error = ApiError::bad_request("参数错误")
        .with_details(Some("id不能为空"));
    let error_str = format!("{}", error);
    assert!(error_str.contains("ApiError"));
    assert!(error_str.contains("400"));
    assert!(error_str.contains("参数错误"));
    assert!(error_str.contains("Details"));
}

// 测试ApiError的时间戳功能
#[test]
fn test_api_error_timestamp() {
    let error = ApiError::new(400, "错误");
    assert!(error.timestamp > 0);
    
    // 确保时间戳是合理的（不是未来时间）
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    assert!(error.timestamp <= now + 10); // 允许10秒误差
}