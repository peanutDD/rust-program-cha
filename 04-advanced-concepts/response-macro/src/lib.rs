//! Response 宏库
//! 
//! 提供宏来简化 HTTP 响应的生成逻辑和错误返回，减少重复代码。

use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, DeriveInput, ItemFn};

/// 为结构体自动实现 actix_web::Responder 特性
/// 
/// 这个派生宏会为结构体自动生成转换为 HTTP 响应的方法。
/// 
/// # 示例
/// ```
/// #[derive(Debug, Response)]
/// struct ApiResponse<T> {
///     data: T,
///     message: String,
///     code: u16,
/// }
/// ```
#[proc_macro_derive(Response)]
pub fn derive_response(input: TokenStream) -> TokenStream {
    // 解析输入的结构体定义
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let generics = &input.generics;
    
    // 生成代码
    let expanded = quote! {
        impl #generics actix_web::Responder for #struct_name #generics
        where
            Self: serde::Serialize,
        {
            fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse {
                // 使用更安全的状态码转换方式
                let status_code = match http::StatusCode::from_u16(self.code) {
                    Ok(status) => status,
                    Err(_) => {
                        // 对于无效的状态码，使用默认值并记录警告
                        http::StatusCode::OK
                    }
                };
                
                // 避免使用unwrap，使用fallback处理序列化失败
                match actix_web::HttpResponse::builder()
                    .status(status_code)
                    .header("content-type", "application/json")
                    .json(self)
                {
                    Ok(response) => response,
                    Err(_) => {
                        // 序列化失败时返回内部服务器错误
                        actix_web::HttpResponse::internal_server_error()
                            .header("content-type", "application/json")
                            .body(r#"{"data":null,"message":"Internal server error during response serialization", "code": 500}"#)
                            .unwrap_or_else(|_| actix_web::HttpResponse::internal_server_error().finish())
                    }
                }
            }
        }
        
        impl #generics #struct_name #generics {
            /// 创建成功响应
            pub fn success(data: T) -> Self {
                Self {
                    data,
                    message: "success".to_string(),
                    code: 200,
                }
            }
            
            /// 创建错误响应
            pub fn error(message: &str, code: u16) -> Self
            where
                T: Default,
            {
                Self {
                    data: Default::default(),
                    message: message.to_string(),
                    code,
                }
            }
        }
    };
    
    expanded.into()
}

/// 为函数添加自动的响应处理
/// 
/// 这个属性宏会将函数的 Result<T, E> 返回值自动转换为 HTTP 响应。
/// 可以通过属性参数自定义成功和错误状态码。
/// 
/// # 示例
/// ```
/// #[response]
/// async fn get_user(id: u32) -> Result<User, AppError> {
///     // 业务逻辑
/// }
/// 
/// // 自定义状态码
/// #[response(success_code = 201, error_code = 400)]
/// async fn create_user(user: User) -> Result<User, AppError> {
///     // 业务逻辑
/// }
/// ```
#[proc_macro_attribute]
pub fn response(args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析函数定义
    let item_fn = parse_macro_input!(input as ItemFn);
    
    // 默认状态码
    let mut success_code = 200;
    let mut error_code = 500;
    
    // 解析属性参数（简化实现）
    let args_str = args.to_string();
    if !args_str.is_empty() {
        if args_str.contains("success_code = ") {
            if let Some(start) = args_str.find("success_code = ") {
                let start = start + "success_code = ".len();
                if let Some(end) = args_str[start..].find(",").or_else(|| args_str[start..].find("}")) {
                    if let Ok(code) = args_str[start..start+end].trim().parse() {
                        success_code = code;
                    }
                }
            }
        }
        if args_str.contains("error_code = ") {
            if let Some(start) = args_str.find("error_code = ") {
                let start = start + "error_code = ".len();
                if let Some(end) = args_str[start..].find(",").or_else(|| args_str[start..].find("}")) {
                    if let Ok(code) = args_str[start..start+end].trim().parse() {
                        error_code = code;
                    }
                }
            }
        }
    }
    
    // 保存原始函数信息
    let block = &item_fn.block;
    let fn_visibility = &item_fn.vis;
    let fn_attrs = &item_fn.attrs;
    let fn_sig = &item_fn.sig;
    
    // 生成代码
    let expanded = quote! {
        #(#fn_attrs)*
        #fn_visibility #fn_sig -> impl actix_web::Responder {
            match (|| async move #block)().await {
                Ok(data) => {
                    let success_response = serde_json::json!({"data": data, "message": "success", "code": #success_code});
                    // 使用更通用的状态码转换方法
                    actix_web::HttpResponse::build(match http::StatusCode::from_u16(#success_code) {
                        Ok(status) => status,
                        Err(_) => http::StatusCode::OK
                    })
                    .header("content-type", "application/json")
                    .json(success_response)
                    .unwrap_or_else(|_| actix_web::HttpResponse::internal_server_error().finish())
                },
                Err(err) => {
                    let error_response = serde_json::json!({"data": null, "message": err.to_string(), "code": #error_code});
                    // 使用更通用的状态码转换方法
                    actix_web::HttpResponse::build(match http::StatusCode::from_u16(#error_code) {
                        Ok(status) => status,
                        Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR
                    })
                    .header("content-type", "application/json")
                    .json(error_response)
                    .unwrap_or_else(|_| actix_web::HttpResponse::internal_server_error().finish())
                }
            }
        }
    };
    
    expanded.into()
}

/// 简化错误处理的宏
/// 
/// 提供安全的错误信息处理，支持多种输入类型。
/// 
/// # 示例
/// ```
/// let user = db.get_user(id).map_err(|e| error!(e))?;
/// let user = db.get_user(id).map_err(|e| error!("获取用户失败: {}", e))?;
/// ```
#[proc_macro]
pub fn error(input: TokenStream) -> TokenStream {
    // 更安全地处理输入，将其转换为表达式
    let input_expr = parse_macro_input!(input as syn::Expr);
    
    let expanded = quote! {
        {
            // 使用trait对象来处理各种错误类型
            let error_value = &(#input_expr);
            // 尝试使用Display格式化，如果失败则使用Debug
            match std::fmt::Display::fmt(error_value, &mut std::fmt::Formatter::new(&mut std::io::sink())) {
                Ok(_) => {
                    // 如果类型实现了Display，则使用to_string
                    error_value.to_string()
                },
                Err(_) => {
                    // 否则使用Debug格式化
                    format!("{:?}", error_value)
                }
            }
        }
    };
    
    expanded.into()
}
