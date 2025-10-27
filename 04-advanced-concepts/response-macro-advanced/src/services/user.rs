use crate::models::{User, AppError, CreateUserRequest, UpdateUserRequest, UserResponse, UserQueryParams, UserListResponse};
use crate::services::db::DatabaseService;
use std::sync::Arc;
use std::result::Result;

/// 用户服务 - 处理用户相关的业务逻辑
#[derive(Clone, Debug)]
pub struct UserService {
    db_service: Arc<DatabaseService>,
}

impl UserService {
    /// 创建新的用户服务实例
    pub fn new(db_service: Arc<DatabaseService>) -> Self {
        Self {
            db_service,
        }
    }
    
    /// 获取单个用户
    pub async fn get_user(&self, user_id: u32) -> Result<UserResponse, AppError> {
        // 从数据库获取用户
        let user = self.db_service.get_user_by_id(user_id)?;
        
        // 转换为响应DTO
        Ok(user.into())
    }
    
    /// 创建用户
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<UserResponse, AppError> {
        // 验证请求数据
        request.validate()
            .map_err(|msg| AppError::validation(&msg))?;
        
        // 检查权限（实际应用中可能需要更复杂的权限检查）
        // 这里简化处理，只允许创建普通用户和编辑员
        if request.role == crate::models::Role::Admin {
            return Err(AppError::permission_denied("不允许创建管理员账户"));
        }
        
        // 创建用户对象
        let new_user = User {
            id: 0, // ID将由数据库服务生成
            name: request.name,
            email: request.email,
            role: request.role,
            created_at: String::new(), // 将由数据库服务设置
            active: true,
        };
        
        // 保存到数据库
        let created_user = self.db_service.create_user(new_user)?;
        
        // 转换为响应DTO
        Ok(created_user.into())
    }
    
    /// 更新用户
    pub async fn update_user(&self, user_id: u32, request: UpdateUserRequest) -> Result<UserResponse, AppError> {
        // 验证请求数据
        request.validate()
            .map_err(|msg| AppError::validation(&msg))?;
        
        // 获取当前用户以进行权限检查
        let current_user = self.db_service.get_user_by_id(user_id)?;
        
        // 权限检查：不允许修改管理员角色
        if current_user.role == crate::models::Role::Admin {
            if request.role.is_some() || request.active.is_some() {
                return Err(AppError::permission_denied("不允许修改管理员账户"));
            }
        }
        
        // 更新用户
        let updated_user = self.db_service.update_user(user_id, |user| {
            // 应用更新字段
            if let Some(name) = request.name {
                user.name = name;
            }
            
            if let Some(email) = request.email {
                user.email = email;
            }
            
            if let Some(role) = request.role {
                // 再次确认不允许修改管理员角色
                if user.role == crate::models::Role::Admin {
                    return Err(AppError::permission_denied("不允许修改管理员角色"));
                }
                user.role = role;
            }
            
            if let Some(active) = request.active {
                // 再次确认不允许停用管理员账户
                if user.role == crate::models::Role::Admin && !active {
                    return Err(AppError::permission_denied("不允许停用管理员账户"));
                }
                user.active = active;
            }
            
            Ok(())
        })?;
        
        // 转换为响应DTO
        Ok(updated_user.into())
    }
    
    /// 获取用户列表
    pub async fn list_users(&self, params: UserQueryParams) -> Result<UserListResponse, AppError> {
        // 验证分页参数
        let valid_params = UserQueryParams {
            page: params.page.max(1), // 确保page至少为1
            page_size: params.page_size.min(100).max(1), // 限制page_size在1-100之间
            name: params.name,
            role: params.role,
            active: params.active,
        };
        
        // 获取用户列表
        let users = self.db_service.list_users(&valid_params)?;
        
        // 获取总数
        let total = self.db_service.count_users(&valid_params)?;
        
        // 转换为响应DTO
        let items: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();
        
        Ok(UserListResponse {
            items,
            total,
            page: valid_params.page,
            page_size: valid_params.page_size,
        })
    }
    
    /// 删除用户
    pub async fn delete_user(&self, user_id: u32) -> Result<(), AppError> {
        // 检查用户是否存在
        let user = self.db_service.get_user_by_id(user_id)?;
        
        // 权限检查：不允许删除管理员账户
        if user.role == crate::models::Role::Admin {
            return Err(AppError::permission_denied("不允许删除管理员账户"));
        }
        
        // 删除用户
        self.db_service.delete_user(user_id)?;
        
        Ok(())
    }
    
    /// 批量获取用户
    pub async fn get_users_by_ids(&self, ids: &[u32]) -> Result<Vec<UserResponse>, AppError> {
        // 验证参数
        if ids.is_empty() {
            return Err(AppError::validation("用户ID列表不能为空"));
        }
        
        if ids.len() > 100 {
            return Err(AppError::validation("一次最多查询100个用户"));
        }
        
        // 批量获取用户
        let users = self.db_service.get_users_by_ids(ids)?;
        
        // 转换为响应DTO
        let responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();
        
        Ok(responses)
    }
    
    /// 激活/停用用户
    pub async fn toggle_user_status(&self, user_id: u32, active: bool) -> Result<UserResponse, AppError> {
        // 检查用户是否存在
        let user = self.db_service.get_user_by_id(user_id)?;
        
        // 权限检查：不允许修改管理员账户状态
        if user.role == crate::models::Role::Admin {
            return Err(AppError::permission_denied("不允许修改管理员账户状态"));
        }
        
        // 如果状态没有变化，直接返回
        if user.active == active {
            return Ok(user.into());
        }
        
        // 更新用户状态
        let update_request = UpdateUserRequest {
            name: None,
            email: None,
            role: None,
            active: Some(active),
        };
        
        self.update_user(user_id, update_request).await
    }
    
    /// 检查邮箱是否已被使用
    pub async fn is_email_used(&self, email: &str) -> Result<bool, AppError> {
        // 这里我们通过尝试创建一个临时用户来检查邮箱是否已被使用
        // 在实际应用中，数据库服务应该提供一个更高效的直接检查方法
        let test_user = User {
            id: 0,
            name: "test".to_string(),
            email: email.to_string(),
            role: crate::models::Role::User,
            created_at: String::new(),
            active: true,
        };
        
        // 尝试创建用户，如果成功则立即删除
        match self.db_service.create_user(test_user.clone()) {
            Ok(created_user) => {
                // 清理创建的测试用户
                let _ = self.db_service.delete_user(created_user.id);
                Ok(false) // 邮箱未被使用
            },
            Err(err) => {
                match err {
                    AppError::Validation(_) => Ok(true), // 邮箱已被使用
                    _ => Err(err), // 其他错误
                }
            },
        }
    }
}

impl Default for UserService {
    fn default() -> Self {
        let db_service = Arc::new(DatabaseService::new());
        Self::new(db_service)
    }
}