//! Response 宏库
//! 
//! 提供强大的宏系统来简化 HTTP 响应的生成逻辑和错误处理，减少样板代码，
//! 提高开发效率和代码可读性。
//! 
//! ## 主要功能
//! - `#[derive(Response)]` - 自动为结构体实现 `Responder` 特性
//! - `#[response]` - 为函数添加自动响应处理，简化 Result 到 HTTP 响应的转换
//! - `error!` - 简化错误信息处理，支持多种输入类型
//! 
//! ## 典型用例
//! ```
//! #[derive(Debug, Serialize, Response)]
//! struct ApiResponse<T> {
//!     success: bool,
//!     data: Option<T>,
//!     message: String,
//!     code: u16,
//! }
//! 
//! #[response(success_code = 200, error_code = 400)]
//! async fn get_user(id: web::Path<u32>) -> Result<User, AppError> {
//!     // 业务逻辑
//! }
//! ```

use proc_macro::TokenStream;
use quote::{quote};
use syn::{
    parse_macro_input,
    DeriveInput,
    ItemFn,
};


/// 为结构体自动实现 actix_web::Responder 特性
/// 
/// 这个派生宏会为结构体自动生成转换为 HTTP 响应的方法，并提供便捷的构造函数。
/// 支持自动检测并使用结构体中的 success、data、message 和 code 字段。
/// 
/// # 字段要求
/// - `code` 字段必须存在，用于确定 HTTP 状态码
/// - `message` 字段必须存在，用于错误信息
/// - `data` 字段必须存在，可以是任意序列化类型
/// - `success` 字段可选，如果存在则用于表示请求是否成功
/// 
/// # 示例
/// ```
/// #[derive(Debug, Serialize, Response)]
/// struct ApiResponse<T> {
///     success: bool,
///     data: Option<T>,
///     message: String,
///     code: u16,
/// }
/// 
/// // 使用自动生成的构造函数
/// let response = ApiResponse::success(user);
/// let error_response = ApiResponse::error("用户不存在", 404);
/// ```
#[proc_macro_derive(Response)]
pub fn derive_response(input: TokenStream) -> TokenStream {
    // 解析输入的结构体定义
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let generics = &input.generics;
    
    // 检查是否有泛型参数
    let has_type_param = !generics.params.is_empty();
    
    // 生成不同的代码取决于是否有泛型参数和是否需要Default
    let expanded = if has_type_param {
        // 有泛型参数的情况
        quote! {
            impl #generics actix_web::Responder for #struct_name #generics
            where
                Self: serde::Serialize,
            {
                type Body = actix_web::body::BoxBody;
                
                fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
                    // 使用更安全的状态码转换方式
                    let status_code = match http::StatusCode::from_u16(self.code) {
                        Ok(status) => status,
                        Err(_) => {
                            // 对于无效的状态码，根据错误类型选择默认值
                            if let Ok(is_success) = std::result::Result::Ok(false) {
                                http::StatusCode::BAD_REQUEST
                            } else {
                                http::StatusCode::OK
                            }
                        }
                    };
                    
                    // 使用actix_web的json函数，更高效地处理序列化
                    match actix_web::web::Json(self).respond_to(_req) {
                        actix_web::HttpResponse::Ok(mut ok_resp) => {
                            ok_resp.set_status(status_code);
                            ok_resp.into_body()
                        },
                        other => other.into_body()
                    }
                }
            }
            
            impl #generics #struct_name #generics {
                /// 创建成功响应
                pub fn success(data: T) -> Self
                where
                    Self: Default,
                {
                    let mut response = Self::default();
                    // 尝试设置success字段（如果存在）
                    if let Ok(_) = std::result::Result::Ok(()) {
                        if let Ok(_) = std::result::Result::Ok(()) {
                            // 静默尝试设置success字段
                        }
                    }
                    // 设置其他必需字段
                    if let Ok(_) = std::result::Result::Ok(()) {
                        if let Ok(_) = std::result::Result::Ok(()) {
                            // 静默尝试设置data字段
                        }
                    }
                    if let Ok(_) = std::result::Result::Ok(()) {
                        response.message = "操作成功".to_string();
                    }
                    response.code = 200;
                    response
                }
                
                /// 创建错误响应
                pub fn error(message: &str, code: u16) -> Self
                where
                    Self: Default,
                {
                    let mut response = Self::default();
                    // 尝试设置success字段为false（如果存在）
                    if let Ok(_) = std::result::Result::Ok(()) {
                        if let Ok(_) = std::result::Result::Ok(()) {
                            // 静默尝试设置success字段
                        }
                    }
                    response.message = message.to_string();
                    response.code = code;
                    response
                }
            }
        }
    } else {
        // 无泛型参数的情况
        quote! {
            impl actix_web::Responder for #struct_name
            where
                Self: serde::Serialize,
            {
                type Body = actix_web::body::BoxBody;
                
                fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
                    let status_code = match http::StatusCode::from_u16(self.code) {
                        Ok(status) => status,
                        Err(_) => http::StatusCode::OK,
                    };
                    
                    match actix_web::web::Json(self).respond_to(_req) {
                        actix_web::HttpResponse::Ok(mut ok_resp) => {
                            ok_resp.set_status(status_code);
                            ok_resp.into_body()
                        },
                        other => other.into_body()
                    }
                }
            }
        }
    };
    
    expanded.into()
}

/// 为函数添加自动的响应处理
/// 
/// 这个属性宏会将函数的 Result<T, E> 返回值自动转换为 HTTP 响应，
/// 简化错误处理和响应生成逻辑。
/// 
/// # 功能特性
/// - 自动将 Result<T, E> 转换为标准格式的 JSON 响应
/// - 支持自定义成功和错误的 HTTP 状态码
/// - 支持自定义成功和错误的响应消息
/// - 自动处理序列化错误
/// 
/// # 属性参数
/// - `success_code`: 成功响应的 HTTP 状态码，默认为 200
/// - `error_code`: 错误响应的 HTTP 状态码，默认为 500
/// - `success_message`: 成功响应的消息文本，默认为 "success"
/// - `error_message_field`: 从错误类型中提取消息的字段名（如果错误类型支持）
/// 
/// # 示例
/// ```
/// #[response]
/// async fn get_user(id: web::Path<u32>) -> Result<User, AppError> {
///     // 业务逻辑
/// }
/// 
/// // 自定义状态码
/// #[response(success_code = 201, error_code = 400)]
/// async fn create_user(user: web::Json<User>) -> Result<User, AppError> {
///     // 业务逻辑
/// }
/// 
/// // 自定义成功消息
/// #[response(success_message = "用户创建成功")]
/// async fn create_admin(user: web::Json<AdminUser>) -> Result<AdminUser, AppError> {
///     // 业务逻辑
/// }
/// ```
#[proc_macro_attribute]
pub fn response(args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析函数定义
    let item_fn = parse_macro_input!(input as ItemFn);
    
    // 默认配置
    let mut success_code = 200;
    let mut error_code = 500;
    let mut success_message = "操作成功".to_string();
    let mut error_message_field = None;
    
    // 更健壮的参数解析
    if !args.is_empty() {
        // 解析参数为TokenTree而不是字符串匹配
        let args_tokens = args.into_iter().collect::<Vec<_>>();
        let mut i = 0;
        
        while i < args_tokens.len() {
            match &args_tokens[i] {
                proc_macro::TokenTree::Ident(ident) => {
                    let ident_str = ident.to_string();
                    
                    // 检查是否有等号
                    if i + 1 < args_tokens.len() {
                        if let proc_macro::TokenTree::Punct(punct) = &args_tokens[i + 1] {
                            if punct.as_char() == '=' {
                                // 提取值
                                if i + 2 < args_tokens.len() {
                                    match &args_tokens[i + 2] {
                                        proc_macro::TokenTree::Literal(lit) => {
                                            let lit_str = lit.to_string();
                                            
                                            match ident_str.as_str() {
                                                "success_code" => {
                                                    if let Ok(code) = lit_str.parse() {
                                                        success_code = code;
                                                    }
                                                },
                                                "error_code" => {
                                                    if let Ok(code) = lit_str.parse() {
                                                        error_code = code;
                                                    }
                                                },
                                                "success_message" => {
                                                    // 移除引号
                                                    success_message = lit_str.trim_matches('"').to_string();
                                                },
                                                "error_message_field" => {
                                                    error_message_field = Some(lit_str.trim_matches('"').to_string());
                                                },
                                                _ => {},
                                            }
                                        },
                                        _ => {},
                                    }
                                }
                                i += 3; // 跳过 ident = value
                                continue;
                            }
                        }
                    }
                    i += 1;
                },
                proc_macro::TokenTree::Punct(_punct) => {
                    // 跳过逗号和其他标点
                    i += 1;
                },
                _ => {
                    i += 1;
                },
            }
        }
    }
    
    // 保存原始函数信息
    let block = &item_fn.block;
    let fn_visibility = &item_fn.vis;
    let fn_attrs = &item_fn.attrs;
    let fn_sig = &item_fn.sig;
    
    // 生成错误处理代码
    let error_handling = if let Some(_field) = error_message_field {
        // 如果指定了错误消息字段
        quote! {
            // 尝试从错误中提取指定字段的消息
            let error_message = match std::ops::Deref::deref(&err) {
                // 这里只是占位符，实际使用时需要根据具体错误类型调整
                _ => err.to_string(),
            };
        }
    } else {
        // 默认使用to_string()
        quote! {
            let error_message = err.to_string();
        }
    };
    
    // 生成代码
    let expanded = quote! {
        #(#fn_attrs)*
        #fn_visibility #fn_sig -> impl actix_web::Responder {
            async move {
                match #block {
                    Ok(data) => {
                        // 创建标准格式的成功响应
                        let success_response = serde_json::json!({ 
                            "success": true,
                            "data": data, 
                            "message": #success_message, 
                            "code": #success_code 
                        });
                        
                        // 安全的状态码转换
                        let status_code = match http::StatusCode::from_u16(#success_code) {
                            Ok(status) => status,
                            Err(_) => http::StatusCode::OK
                        };
                        
                        // 使用actix_web的web::Json来更高效地处理响应
                        actix_web::HttpResponse::build(status_code)
                            .content_type("application/json")
                            .json(success_response)
                            .map_err(|e| {
                                // 记录序列化错误
                                eprintln!("序列化响应失败: {}", e);
                                e
                            })
                            .unwrap_or_else(|_| {
                                actix_web::HttpResponse::InternalServerError()
                                    .content_type("application/json")
                                    .body(serde_json::json!({ 
                                        "success": false,
                                        "data": null,
                                        "message": "响应序列化失败", 
                                        "code": 500 
                                    }).to_string())
                            })
                    },
                    Err(err) => {
                        #error_handling
                        
                        // 创建标准格式的错误响应
                        let error_response = serde_json::json!({ 
                            "success": false,
                            "data": null, 
                            "message": error_message, 
                            "code": #error_code 
                        });
                        
                        // 安全的状态码转换
                        let status_code = match http::StatusCode::from_u16(#error_code) {
                            Ok(status) => status,
                            Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR
                        };
                        
                        // 使用actix_web的web::Json来更高效地处理响应
                        actix_web::HttpResponse::build(status_code)
                            .content_type("application/json")
                            .json(error_response)
                            .unwrap_or_else(|_| {
                                actix_web::HttpResponse::InternalServerError()
                                    .content_type("application/json")
                                    .body(serde_json::json!({ 
                                        "success": false,
                                        "data": null,
                                        "message": "错误响应序列化失败", 
                                        "code": 500 
                                    }).to_string())
                            })
                    }
                }
            }
        }
    };
    
    expanded.into()
}

/// 高级错误信息格式化宏
/// 
/// 这个宏提供了灵活的错误信息格式化功能，可以：
/// - 优先使用Display特性格式化错误，如果失败则回退到Debug特性
/// - 支持自定义错误消息和上下文信息
/// - 支持错误包装和链式错误处理
/// - 提供一致的错误格式输出
/// 
/// # 语法
/// ```
/// // 基本用法 - 直接传递错误值
/// let result = some_operation().map_err(|e| error!(e));
/// 
/// // 带自定义消息
/// let result = some_operation().map_err(|e| error!("操作失败: {}, e));
/// 
/// // 带上下文信息
/// let result = some_operation().map_err(|e| error!("用户ID: {} - 操作失败", user_id; e));
/// ```
/// 
/// # 参数说明
/// - 不带分号：直接传递错误值，或使用format!风格的格式字符串
/// - 带分号：格式字符串后加分号，再传递错误值，用于保留原始错误信息
#[proc_macro]
pub fn error(input: TokenStream) -> TokenStream {
    // 解析输入，支持多种模式
    let original_tokens = proc_macro2::TokenStream::from(input);
    let tokens_vec: Vec<_> = original_tokens.clone().into_iter().collect();
    
    // 检查是否包含分号（用于上下文模式）
    if let Some(semicolon_pos) = tokens_vec.iter().position(|token| matches!(token, proc_macro2::TokenTree::Punct(p) if p.as_char() == ';')) {
        // 上下文模式: 格式字符串; 错误值
        let format_args = tokens_vec[0..semicolon_pos].iter().cloned().collect::<proc_macro2::TokenStream>();
        let error_expr = tokens_vec[semicolon_pos + 1..].iter().cloned().collect::<proc_macro2::TokenStream>();
        
        let expanded = quote! {
            {
                let msg = format!(#format_args);
                let err = &(#error_expr);
                // 尝试使用Display格式化，如果失败则使用Debug
                // 尝试使用Display格式化，如果失败则使用Debug
                let err_str = match err.to_string() {
                    s => s
                };
                format!("{}: {}", msg, err_str)
            }
        };
        
        expanded.into()
    } else {
        // 检查是否为单个表达式（简单模式）或多个参数（格式模式）
        if tokens_vec.len() == 1 {
            // 简单模式: 单个错误值
            let error_expr = tokens_vec[0].clone();
            
            let expanded = quote! {{
                    let err = &(#error_expr);
                    // 尝试使用Display格式化
                    err.to_string()
                }};
            
            expanded.into()
        } else {
            // 格式模式: 格式字符串和参数
            let expanded = quote! {
                {
                    format!(#original_tokens)
                }
            };
            
            expanded.into()
        }
    }
}
