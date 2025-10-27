use serde::{Deserialize, Serialize};
use std::fmt;

/// 用户结构体 - 领域模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub role: Role,
    pub created_at: String, // 在实际应用中，应该使用chrono::DateTime
    pub active: bool,
}

/// 用户角色枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Role {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "editor")]
    Editor,
    #[serde(rename = "user")]
    User,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Admin => write!(f, "admin"),
            Role::Editor => write!(f, "editor"),
            Role::User => write!(f, "user"),
        }
    }
}

/// 用户创建请求DTO
#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub role: Role,
}

/// 用户更新请求DTO
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<Role>,
    pub active: Option<bool>,
}

/// 用户响应DTO
#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub role: Role,
    pub created_at: String,
    pub active: bool,
}

/// 用户列表响应DTO
#[derive(Debug, Clone, Serialize)]
pub struct UserListResponse {
    pub items: Vec<UserResponse>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}

/// 用户查询参数
#[derive(Debug, Clone, Deserialize)]
pub struct UserQueryParams {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
    pub name: Option<String>,
    pub role: Option<Role>,
    pub active: Option<bool>,
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    10
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            role: user.role,
            created_at: user.created_at,
            active: user.active,
        }
    }
}

impl CreateUserRequest {
    /// 验证请求数据
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("用户名不能为空".to_string());
        }
        
        if self.name.len() < 2 || self.name.len() > 50 {
            return Err("用户名长度必须在2-50个字符之间".to_string());
        }
        
        // 简单的邮箱格式验证
        if !validate_email(&self.email) {
            return Err("邮箱格式无效".to_string());
        }
        
        Ok(())
    }
}

impl UpdateUserRequest {
    /// 验证请求数据
    pub fn validate(&self) -> Result<(), String> {
        if let Some(name) = &self.name {
            if name.trim().is_empty() {
                return Err("用户名不能为空".to_string());
            }
            
            if name.len() < 2 || name.len() > 50 {
                return Err("用户名长度必须在2-50个字符之间".to_string());
            }
        }
        
        if let Some(email) = &self.email {
            if !validate_email(email) {
                return Err("邮箱格式无效".to_string());
            }
        }
        
        Ok(())
    }
}

/// 简单的邮箱验证函数
fn validate_email(email: &str) -> bool {
    // 这个验证比较基础，实际生产环境中可能需要更复杂的正则表达式
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    
    if parts[0].is_empty() || parts[1].is_empty() {
        return false;
    }
    
    parts[1].contains('.')
}