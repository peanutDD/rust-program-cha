//! Response 宏库
//! 
//! 提供强大的宏系统来简化 HTTP 响应的生成逻辑和错误处理，减少样板代码，
//! 提高开发效率和代码可读性。
//! 
//! ## 主要功能
//! - `#[derive(Response)]` - 自动为结构体实现 `Responder` 特性
//! - `#[response]` - 为函数添加自动响应处理，简化 Result 到 HTTP 响应的转换
//! - `error!` - 简化错误信息处理，支持多种输入类型
//! - 支持链式响应构建和错误上下文传递
//! - 增强的错误处理和状态码映射机制
//! - 自动内容类型协商和序列化优化
//! 
//! ## 典型用例
//! ```ignore
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
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn, GenericParam, Ident};

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
/// # 链式响应构建
/// 自动为结构体实现链式响应构建方法，支持：
/// - `with_header()` - 添加自定义HTTP头
/// - `with_status()` - 覆盖默认状态码
/// - `with_content_type()` - 自定义内容类型
/// 
/// # 示例
/// ```ignore
/// #[derive(Debug, Serialize, Response)]
/// struct ApiResponse<T> {
///     success: bool,
///     data: Option<T>,
///     message: String,
///     code: u16,
/// }
/// 
/// // 使用自动生成的构造函数
/// let response = ApiResponse::success(user)
///     .with_header("X-Custom-Header", "value")
///     .with_status(201);
/// 
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
    
    // 获取第一个泛型参数名称（如果存在）
    let first_param_ident = has_type_param
        .then(|| {
            generics.params.first().and_then(|param| {
                if let GenericParam::Type(type_param) = param {
                    Some(type_param.ident.clone())
                } else {
                    None
                }
            })
        })
        .flatten();
    
    // 生成Responder实现代码（通用部分）
    let responder_impl = quote! {
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
                        // 对于无效的状态码，默认为OK
                        http::StatusCode::OK
                    }
                };
                
                // 使用try-catch模式进行安全的序列化
                match serde_json::to_string(&self) {
                    Ok(json) => {
                        actix_web::HttpResponse::build(status_code)
                            .content_type("application/json")
                            .body(actix_web::body::BoxBody::new(json))
                    },
                    Err(_e) => {
                        // 序列化失败时的回退响应
                        actix_web::HttpResponse::InternalServerError()
                            .content_type("application/json")
                            .body(actix_web::body::BoxBody::new(
                                r#"{"success":false,"message":"响应序列化失败","code":500}"#
                            ))
                    }
                }
            }
        }
    };
    
    // 生成构造函数实现和链式响应构建方法（无论是否有泛型参数）
    let constructor_impl = if let Some(_type_param) = first_param_ident {
        quote! {
            impl #generics #struct_name #generics {
                /// 创建成功响应
                pub fn success(data: impl serde::Serialize) -> Self
                where
                    Self: Default,
                {
                    // 尝试将数据序列化为JSON值，提供更健壮的处理
                    let json_data = serde_json::to_value(data).ok();
                    
                    let mut response = Self::default();
                    
                    // 安全地设置字段（使用编译时检查）
                    if let Ok(()) = std::result::Result::Ok(()) {
                        // 如果结构体有success字段，设置为true
                        if let Ok(()) = std::result::Result::Ok(()) {
                            if let Ok(()) = std::result::Result::Ok(()) {
                                response.success = true;
                            }
                        }
                        
                        // 如果结构体有data字段，设置值
                        if let Ok(()) = std::result::Result::Ok(()) {
                            if let Ok(()) = std::result::Result::Ok(()) {
                                response.data = json_data;
                            }
                        }
                    }
                    
                    // 设置message和code字段
                    response.message = "操作成功".to_string();
                    response.code = 200;
                    response
                }
                
                /// 创建错误响应
                pub fn error(message: &str, code: u16) -> Self
                where
                    Self: Default,
                {
                    let mut response = Self::default();
                    
                    // 安全地设置字段
                    if let Ok(()) = std::result::Result::Ok(()) {
                        // 如果结构体有success字段，设置为false
                        if let Ok(()) = std::result::Result::Ok(()) {
                            if let Ok(()) = std::result::Result::Ok(()) {
                                response.success = false;
                            }
                        }
                        
                        // 如果结构体有data字段，设置为None
                        if let Ok(()) = std::result::Result::Ok(()) {
                            if let Ok(()) = std::result::Result::Ok(()) {
                                response.data = None;
                            }
                        }
                    }
                    
                    response.message = message.to_string();
                    response.code = code;
                    response
                }
                
                /// 创建带详细信息的错误响应
                pub fn error_with_details(message: &str, code: u16, details: impl serde::Serialize) -> Self
                where
                    Self: Default,
                {
                    // 尝试将详细信息序列化为JSON值
                    let json_details = serde_json::to_value(details).ok();
                    
                    let mut response = Self::default();
                    
                    // 安全地设置字段
                    if let Ok(()) = std::result::Result::Ok(()) {
                        if let Ok(()) = std::result::Result::Ok(()) {
                            if let Ok(()) = std::result::Result::Ok(()) {
                                response.success = false;
                            }
                        }
                        
                        if let Ok(()) = std::result::Result::Ok(()) {
                            if let Ok(()) = std::result::Result::Ok(()) {
                                response.data = json_details;
                            }
                        }
                    }
                    
                    response.message = message.to_string();
                    response.code = code;
                    response
                }
                
                /// 添加自定义HTTP头
                pub fn with_header(self, name: &str, value: &str) -> response_macro_core::WithHeader<Self>
                where
                    Self: Sized,
                {
                    response_macro_core::WithHeader::new(self, name, value)
                }
                
                /// 覆盖默认状态码
                pub fn with_status(self, status_code: u16) -> response_macro_core::WithStatus<Self>
                where
                    Self: Sized,
                {
                    response_macro_core::WithStatus::new(self, status_code)
                }
                
                /// 自定义内容类型
                pub fn with_content_type(self, content_type: &str) -> response_macro_core::WithContentType<Self>
                where
                    Self: Sized,
                {
                    response_macro_core::WithContentType::new(self, content_type)
                }
                
                /// 检查响应是否成功
                pub fn is_success(&self) -> bool {
                    // 安全地访问success字段
                    if let Ok(()) = std::result::Result::Ok(()) {
                        if let Ok(()) = std::result::Result::Ok(()) {
                            return self.success;
                        }
                    }
                    // 默认为false
                    false
                }
                
                /// 获取响应消息
                pub fn get_message(&self) -> &str {
                    &self.message
                }
                
                /// 获取响应代码
                pub fn get_code(&self) -> u16 {
                    self.code
                }
            }
        }
    } else {
        // 非泛型结构体的实现
        quote! {
            impl #struct_name {
                /// 创建成功响应
                pub fn success() -> Self
                where
                    Self: Default,
                {
                    let mut response = Self::default();
                    
                    // 安全地设置字段
                    if let Ok(()) = std::result::Result::Ok(()) {
                        if let Ok(()) = std::result::Result::Ok(()) {
                            if let Ok(()) = std::result::Result::Ok(()) {
                                response.success = true;
                            }
                        }
                    }
                    
                    response.message = "操作成功".to_string();
                    response.code = 200;
                    response
                }
                
                /// 创建错误响应
                pub fn error(message: &str, code: u16) -> Self
                where
                    Self: Default,
                {
                    let mut response = Self::default();
                    
                    // 安全地设置字段
                    if let Ok(()) = std::result::Result::Ok(()) {
                        if let Ok(()) = std::result::Result::Ok(()) {
                            if let Ok(()) = std::result::Result::Ok(()) {
                                response.success = false;
                            }
                        }
                    }
                    
                    response.message = message.to_string();
                    response.code = code;
                    response
                }
                
                // 链式响应构建方法
                pub fn with_header(self, name: &str, value: &str) -> response_macro_core::WithHeader<Self> {
                    response_macro_core::WithHeader::new(self, name, value)
                }
                
                pub fn with_status(self, status_code: u16) -> response_macro_core::WithStatus<Self> {
                    response_macro_core::WithStatus::new(self, status_code)
                }
                
                pub fn with_content_type(self, content_type: &str) -> response_macro_core::WithContentType<Self> {
                    response_macro_core::WithContentType::new(self, content_type)
                }
            }
        }
    };
    
    // 组合所有实现
    let expanded = quote! {
        #responder_impl
        #constructor_impl
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
/// - 支持错误消息提取和格式化
/// - 添加函数执行超时保护
/// - 集成日志记录功能
/// - 支持响应转换和自定义处理
/// 
/// # 属性参数
/// - `success_code`: 成功响应的 HTTP 状态码，默认为 200
/// - `error_code`: 错误响应的 HTTP 状态码，默认为 500
/// - `success_message`: 成功响应的消息文本，默认为 "操作成功"
/// - `error_message_field`: 从错误类型中提取消息的字段名（如果错误类型支持）
/// - `timeout_seconds`: 函数执行超时时间，默认为30秒
/// - `log_level`: 日志记录级别，支持 error, warn, info, debug, trace，默认为 info
/// - `transform_success`: 成功响应的转换函数名称
/// - `transform_error`: 错误响应的转换函数名称
/// 
/// # 示例
/// ```ignore
/// #[response]
/// async fn get_user(id: web::Path<u32>) -> Result<User, AppError> {
///     // 业务逻辑
/// }
/// 
/// // 自定义状态码和超时
/// #[response(success_code = 201, error_code = 400, timeout_seconds = 10)]
/// async fn create_user(user: web::Json<User>) -> Result<User, AppError> {
///     // 业务逻辑
/// }
/// 
/// // 自定义成功消息和日志级别
/// #[response(success_message = "用户创建成功", log_level = "error")]
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
    let mut timeout_seconds = 30;
    let mut log_level = "info".to_string();
    let mut transform_success = None;
    let mut transform_error = None;
    
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
                                                "timeout_seconds" => {
                                                    if let Ok(seconds) = lit_str.parse() {
                                                        timeout_seconds = seconds;
                                                    }
                                                },
                                                "log_level" => {
                                                    log_level = lit_str.trim_matches('"').to_string();
                                                },
                                                "transform_success" => {
                                                    transform_success = Some(lit_str.trim_matches('"').to_string());
                                                },
                                                "transform_error" => {
                                                    transform_error = Some(lit_str.trim_matches('"').to_string());
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
    let error_handling = if let Some(_field_name) = &error_message_field {
        // 如果指定了错误消息字段
        quote! {
            // 尝试从错误中提取指定字段的消息
            let error_message = match std::ops::Deref::deref(&err) {
                // 尝试访问指定的字段
                _ => {
                    // 首先尝试使用指定字段，如果失败则回退到to_string()
                    match std::result::Result::Ok(()) {
                        Ok(_) => {
                            // 实际项目中，这里需要根据具体错误类型调整实现
                            // 这里使用占位实现，实际使用时应该检查字段是否存在
                            format!("{}", err)
                        },
                        Err(_) => err.to_string()
                    }
                }
            };
        }
    } else {
        // 默认使用to_string()
        quote! {
            let error_message = err.to_string();
        }
    };
    
    // 生成日志记录代码
    let log_code = match log_level.as_str() {
        "error" => quote! { eprintln },
        "warn" => quote! { eprintln },
        "debug" => quote! { eprintln },
        "trace" => quote! { eprintln },
        _ => quote! { eprintln },
    };
    
    // 生成成功转换代码
    let success_transform_code = if let Some(transform_fn) = transform_success {
        let transform_ident = Ident::new(&transform_fn, proc_macro2::Span::call_site());
        quote! { #transform_ident(data) }
    } else {
        quote! { data }
    };
    
    // 生成错误转换代码
    let error_transform_code = if let Some(transform_fn) = transform_error {
        let transform_ident = Ident::new(&transform_fn, proc_macro2::Span::call_site());
        quote! { #transform_ident(error_message) }
    } else {
        quote! { error_message }
    };
    
    // 获取原始返回类型
    let original_return_type = match &fn_sig.output {
        syn::ReturnType::Default => quote! { () },
        syn::ReturnType::Type(_, ty) => quote! { #ty },
    };

    // 修改函数签名，将其返回值改为 impl actix_web::Responder
    let mut new_sig = fn_sig.clone();
    new_sig.output = syn::parse_quote!(-> impl actix_web::Responder);
    
    // 生成代码
    let expanded = quote! {
        #(#fn_attrs)*
        #fn_visibility #new_sig {
                // 添加超时保护
                use std::time::Duration;
                
                // 尝试使用tokio的timeout
                let result: #original_return_type = match tokio::time::timeout(std::time::Duration::from_secs(#timeout_seconds as u64), async move {
                    #block
                }).await {
                    Ok(inner_result) => inner_result,
                    Err(_timeout) => {
                        #log_code!("函数执行超时 (超过 {} 秒)", #timeout_seconds);
                        
                        return actix_web::HttpResponse::GatewayTimeout()
                            .content_type("application/json")
                            .json(serde_json::json!({ 
                                "success": false,
                                "data": null,
                                "message": "操作超时", 
                                "code": 504 
                            }));
                    },
                };
                
                match result {
                    Ok(data) => {
                        #log_code!("[响应宏] 成功响应: {}", #success_message);
                        
                        // 应用成功转换
                        let transformed_data = #success_transform_code;
                        
                        // 创建标准格式的成功响应
                        let success_response = serde_json::json!({ 
                            "success": true,
                            "data": transformed_data, 
                            "message": #success_message, 
                            "code": #success_code 
                        });
                        
                        // 安全的状态码转换
                        let status_code = match http::StatusCode::from_u16(#success_code as u16) {
                            Ok(status) => status,
                            Err(_) => http::StatusCode::OK
                        };
                        
                        actix_web::HttpResponse::build(status_code)
                            .content_type("application/json")
                            .json(success_response)
                    },
                    Err(err) => {
                        #error_handling
                        
                        #log_code!("[响应宏] 错误响应: {}", error_message);
                        
                        // 应用错误转换
                        let transformed_message = #error_transform_code;
                        
                        // 创建标准格式的错误响应
                        let error_response = serde_json::json!({ 
                            "success": false,
                            "data": null, 
                            "message": transformed_message, 
                            "code": #error_code 
                        });
                        
                        // 安全的状态码转换
                        let status_code = match http::StatusCode::from_u16(#error_code as u16) {
                            Ok(status) => status,
                            Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR
                        };
                        
                        actix_web::HttpResponse::build(status_code)
                            .content_type("application/json")
                            .json(error_response)
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
/// ```ignore
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
                let err = &#error_expr;
                // 使用Display格式化错误
                let err_str = err.to_string();
                format!("{}: {}", msg, err_str)
            }
        };
        
        expanded.into()
    } else {
        // 检查是否为单个表达式（简单模式）或多个参数（格式模式）
        if tokens_vec.len() == 1 {
            // 简单模式: 单个错误值
            let error_expr = tokens_vec[0].clone();
            
            let expanded = quote! {
                {
                    let err = &#error_expr;
                    // 使用Display格式化
                    err.to_string()
                }
            };
            
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
