use std::sync::{Arc, RwLock};
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::models::{User, AppError, UserQueryParams};
use std::result::Result;
use std::convert::TryFrom;

/// 线程安全的数据库服务
#[derive(Clone, Debug)]
pub struct DatabaseService {
    users: Arc<RwLock<HashMap<u32, User>>>,
    email_index: Arc<RwLock<HashSet<String>>>, // 用于快速检查邮箱是否已存在
    next_id: Arc<RwLock<u32>>,
}

impl DatabaseService {
    /// 创建新的数据库实例
    pub fn new() -> Self {
        let users = Arc::new(RwLock::new(HashMap::new()));
        let email_index = Arc::new(RwLock::new(HashSet::new()));
        let next_id = Arc::new(RwLock::new(1));
        
        let db = Self {
            users,
            email_index,
            next_id,
        };
        
        // 初始化一些测试数据
        db.init_test_data().expect("Failed to initialize test data");
        
        db
    }
    
    /// 初始化测试数据
    fn init_test_data(&self) -> Result<(), AppError> {
        let test_users = vec![
            User {
                id: 1,
                name: "管理员".to_string(),
                email: "admin@example.com".to_string(),
                role: crate::models::Role::Admin,
                created_at: self.current_timestamp(),
                active: true,
            },
            User {
                id: 2,
                name: "编辑员".to_string(),
                email: "editor@example.com".to_string(),
                role: crate::models::Role::Editor,
                created_at: self.current_timestamp(),
                active: true,
            },
            User {
                id: 3,
                name: "普通用户".to_string(),
                email: "user@example.com".to_string(),
                role: crate::models::Role::User,
                created_at: self.current_timestamp(),
                active: true,
            },
        ];
        
        let mut users_write = self.users.write().map_err(|_| AppError::database("无法获取数据库写锁"))?;
        let mut email_index_write = self.email_index.write().map_err(|_| AppError::database("无法获取邮箱索引写锁"))?;
        let mut next_id_write = self.next_id.write().map_err(|_| AppError::database("无法获取ID生成器写锁"))?;
        
        for user in test_users {
            users_write.insert(user.id, user.clone());
            email_index_write.insert(user.email);
            *next_id_write = u32::max(*next_id_write, user.id + 1);
        }
        
        Ok(())
    }
    
    /// 获取当前时间戳字符串
    fn current_timestamp(&self) -> String {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs().to_string())
            .unwrap_or_else(|_| "0".to_string())
    }
    
    /// 生成下一个用户ID
    fn generate_id(&self) -> Result<u32, AppError> {
        let mut next_id = self.next_id.write().map_err(|_| AppError::database("无法获取ID生成器写锁"))?;
        let id = *next_id;
        *next_id += 1;
        Ok(id)
    }
    
    /// 根据ID获取用户
    pub fn get_user_by_id(&self, id: u32) -> Result<User, AppError> {
        // 使用读锁提高并发性能
        let users = self.users.read().map_err(|_| AppError::database("无法获取数据库读锁"))?;
        
        users.get(&id)
            .cloned()
            .ok_or_else(|| AppError::not_found(&format!("用户ID {} 不存在", id)))
    }
    
    /// 创建新用户
    pub fn create_user(&self, mut user: User) -> Result<User, AppError> {
        // 检查邮箱是否已存在
        {
            let email_index = self.email_index.read().map_err(|_| AppError::database("无法获取邮箱索引读锁"))?;
            if email_index.contains(&user.email) {
                return Err(AppError::validation(&format!("邮箱 {} 已被使用", user.email)));
            }
        }
        
        // 生成用户ID
        user.id = self.generate_id()?;
        user.created_at = self.current_timestamp();
        user.active = true;
        
        // 写入用户数据
        let mut users = self.users.write().map_err(|_| AppError::database("无法获取数据库写锁"))?;
        let mut email_index = self.email_index.write().map_err(|_| AppError::database("无法获取邮箱索引写锁"))?;
        
        users.insert(user.id, user.clone());
        email_index.insert(user.email.clone());
        
        Ok(user)
    }
    
    /// 更新用户
    pub fn update_user(&self, id: u32, update_fn: impl FnOnce(&mut User) -> Result<(), AppError>) -> Result<User, AppError> {
        // 检查用户是否存在
        let mut users = self.users.write().map_err(|_| AppError::database("无法获取数据库写锁"))?;
        
        let user = users.get_mut(&id)
            .ok_or_else(|| AppError::not_found(&format!("用户ID {} 不存在", id)))?;
        
        // 保存原始邮箱用于索引更新
        let original_email = user.email.clone();
        
        // 应用更新函数
        update_fn(user)?;
        
        // 如果邮箱发生变化，更新索引
        if original_email != user.email {
            let mut email_index = self.email_index.write().map_err(|_| AppError::database("无法获取邮箱索引写锁"))?;
            
            // 检查新邮箱是否已被使用
            if email_index.contains(&user.email) {
                // 恢复原始邮箱
                user.email = original_email;
                return Err(AppError::validation(&format!("邮箱 {} 已被使用", user.email)));
            }
            
            // 更新索引
            email_index.remove(&original_email);
            email_index.insert(user.email.clone());
        }
        
        Ok(user.clone())
    }
    
    /// 列出用户
    pub fn list_users(&self, params: &UserQueryParams) -> Result<Vec<User>, AppError> {
        // 计算分页偏移量
        let offset = (params.page.saturating_sub(1)) * params.page_size;
        let limit = params.page_size;
        
        // 使用读锁进行查询
        let users = self.users.read().map_err(|_| AppError::database("无法获取数据库读锁"))?;
        
        // 过滤并排序用户
        let mut filtered_users: Vec<_> = users.values()
            .filter(|user| {
                // 根据查询参数过滤
                if let Some(ref name) = params.name {
                    if !user.name.contains(name) {
                        return false;
                    }
                }
                
                if let Some(ref role) = params.role {
                    if &user.role != role {
                        return false;
                    }
                }
                
                if let Some(active) = params.active {
                    if user.active != active {
                        return false;
                    }
                }
                
                true
            })
            .collect();
        
        // 按ID排序（实际应用中可能需要更复杂的排序逻辑）
        filtered_users.sort_by_key(|user| user.id);
        
        // 应用分页
        let paginated_users = filtered_users
            .into_iter()
            .skip(offset as usize)
            .take(limit as usize)
            .cloned()
            .collect();
        
        Ok(paginated_users)
    }
    
    /// 获取用户总数
    pub fn count_users(&self, params: &UserQueryParams) -> Result<u64, AppError> {
        // 使用读锁进行计数
        let users = self.users.read().map_err(|_| AppError::database("无法获取数据库读锁"))?;
        
        // 过滤并计数
        let count = users.values()
            .filter(|user| {
                // 根据查询参数过滤
                if let Some(ref name) = params.name {
                    if !user.name.contains(name) {
                        return false;
                    }
                }
                
                if let Some(ref role) = params.role {
                    if &user.role != role {
                        return false;
                    }
                }
                
                if let Some(active) = params.active {
                    if user.active != active {
                        return false;
                    }
                }
                
                true
            })
            .count() as u64;
        
        Ok(count)
    }
    
    /// 删除用户
    pub fn delete_user(&self, id: u32) -> Result<(), AppError> {
        // 获取数据库锁
        let mut users = self.users.write().map_err(|_| AppError::database("无法获取数据库写锁"))?;
        
        // 检查用户是否存在
        let user = users.remove(&id)
            .ok_or_else(|| AppError::not_found(&format!("用户ID {} 不存在", id)))?;
        
        // 从邮箱索引中删除
        let mut email_index = self.email_index.write().map_err(|_| AppError::database("无法获取邮箱索引写锁"))?;
        email_index.remove(&user.email);
        
        Ok(())
    }
    
    /// 批量获取用户
    pub fn get_users_by_ids(&self, ids: &[u32]) -> Result<Vec<User>, AppError> {
        // 使用读锁提高并发性能
        let users = self.users.read().map_err(|_| AppError::database("无法获取数据库读锁"))?;
        
        let mut result = Vec::with_capacity(ids.len());
        
        for id in ids {
            if let Some(user) = users.get(id) {
                result.push(user.clone());
            }
        }
        
        Ok(result)
    }
}

impl Default for DatabaseService {
    fn default() -> Self {
        Self::new()
    }
}