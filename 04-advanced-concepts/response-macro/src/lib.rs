//! Response 宏库
//! 
//! 提供宏来简化 Response<T> 的生成逻辑和错误返回，减少 match/Json()/StatusCode 的重复书写。

use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, AttributeArgs, ItemFn, Lit, Meta, NestedMeta};

/// 为结构体自动实现 Response 特性
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
        impl #generics Response for #struct_name #generics
        where
            Self: serde::Serialize,
        {
            fn into_response(self) -> HttpResponse {
                let status = match self.code {
                    200 => StatusCode::OK,
                    201 => StatusCode::CREATED,
                    400 => StatusCode::BAD_REQUEST,
                    401 => StatusCode::UNAUTHORIZED,
                    403 => StatusCode::FORBIDDEN,
                    404 => StatusCode::NOT_FOUND,
                    500 => StatusCode::INTERNAL_SERVER_ERROR,
                    _ => StatusCode::OK,
                };
                
                HttpResponse::builder()
                    .status(status)
                    .header("content-type", "application/json")
                    .body(Json(self).into_body())
                    .unwrap()
            }
        }
        
        impl #generics #struct_name #generics {
            /// 创建成功响应
            pub fn success(data: <#struct_name #generics as ResponseData>::Data) -> Self {
                Self {
                    data,
                    message: "success".to_string(),
                    code: 200,
                }
            }
            
            /// 创建错误响应
            pub fn error(message: &str, code: u16) -> Self {
                Self {
                    data: Default::default(),
                    message: message.to_string(),
                    code,
                }
            }
        }
        
        impl #generics ResponseData for #struct_name #generics {
            type Data = <Self as std::ops::Deref>::Target;
        }
    };
    
    expanded.into()
}

/// 为函数添加自动的响应处理
/// 
/// 这个属性宏会将函数的 Result<T, E> 返回值自动转换为 HTTP 响应。
/// 
/// # 示例
/// ```
/// #[response]
/// async fn get_user(id: u32) -> Result<User, AppError> {
///     // 业务逻辑
/// }
/// ```
#[proc_macro_attribute]
pub fn response(args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析属性参数
    let args = parse_macro_input!(args as AttributeArgs);
    
    // 解析函数定义
    let mut item_fn = parse_macro_input!(input as ItemFn);
    
    // 处理属性参数，提取自定义状态码等信息
    let mut success_code = 200;
    let mut error_code = 500;
    
    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
            if nv.path.is_ident("success_code") {
                if let Lit::Int(int) = nv.lit {
                    success_code = int.base10_parse().unwrap_or(200);
                }
            } else if nv.path.is_ident("error_code") {
                if let Lit::Int(int) = nv.lit {
                    error_code = int.base10_parse().unwrap_or(500);
                }
            }
        }
    }
    
    // 保存原始函数体
    let block = &item_fn.block;
    let fn_name = &item_fn.sig.ident;
    let fn_visibility = &item_fn.vis;
    let fn_sig = &item_fn.sig;
    let fn_attrs = &item_fn.attrs;
    
    // 生成新的函数体，自动处理 Result
    let expanded = quote! {
        #(#fn_attrs)*
        #fn_visibility #fn_sig {
            match (|| async move #block)().await {
                Ok(data) => {
                    let response = ApiResponse::success(data);
                    Json(response).into_response()
                },
                Err(err) => {
                    let error_response = ApiResponse::error(&err.to_string(), #error_code);
                    Json(error_response).into_response()
                }
            }
        }
    };
    
    expanded.into()
}

/// 简化错误处理的宏
/// 
/// # 示例
/// ```
/// let user = db.get_user(id).map_err(|e| error!("获取用户失败: {}", e))?;
/// ```
#[proc_macro]
pub fn error(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    
    let expanded = quote! {
        AppError::new(&format!(#input_str))
    };
    
    expanded.into()
}
