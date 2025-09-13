//! # 最佳实践指南
//!
//! 本模块提供 Eq 和 PartialEq 在实际开发中的最佳实践、设计模式和常见陷阱的解决方案

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// 运行最佳实践演示
pub fn run_best_practices_guide() {
    println!("\n🔸 最佳实践指南:");
    
    trait_implementation_guidelines();
    hash_consistency_practices();
    performance_best_practices();
    error_handling_patterns();
    testing_strategies();
    documentation_practices();
    common_antipatterns();
    design_patterns();
}

/// Trait 实现指导原则
fn trait_implementation_guidelines() {
    println!("\n  📌 Trait 实现指导原则:");
    
    // 1. 优先使用 derive
    println!("\n    🔹 优先使用 derive:");
    
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct GoodExample {
        id: u32,
        name: String,
        active: bool,
    }
    
    println!("      ✅ 推荐：使用 #[derive(PartialEq, Eq)] 自动实现");
    println!("         - 自动生成正确的实现");
    println!("         - 减少手动错误");
    println!("         - 保持一致性");
    
    // 2. 手动实现的场景
    println!("\n    🔹 需要手动实现的场景:");
    
    #[derive(Debug, Clone)]
    struct CustomEquality {
        id: u32,
        name: String,
        case_sensitive: bool,
        metadata: HashMap<String, String>,  // 在比较中忽略
    }
    
    impl PartialEq for CustomEquality {
        fn eq(&self, other: &Self) -> bool {
            // 根据 case_sensitive 标志决定如何比较 name
            let names_equal = if self.case_sensitive {
                self.name == other.name
            } else {
                self.name.to_lowercase() == other.name.to_lowercase()
            };
            
            // 只比较 id, name 和 case_sensitive，忽略 metadata
            self.id == other.id && 
            names_equal && 
            self.case_sensitive == other.case_sensitive
        }
    }
    
    impl Eq for CustomEquality {}
    
    impl Hash for CustomEquality {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
            
            // 确保 hash 与 PartialEq 一致
            if self.case_sensitive {
                self.name.hash(state);
            } else {
                self.name.to_lowercase().hash(state);
            }
            
            self.case_sensitive.hash(state);
            // 不包含 metadata
        }
    }
    
    let item1 = CustomEquality {
        id: 1,
        name: "Test".to_string(),
        case_sensitive: false,
        metadata: HashMap::from([("key1".to_string(), "value1".to_string())]),
    };
    
    let item2 = CustomEquality {
        id: 1,
        name: "TEST".to_string(),  // 不同大小写
        case_sensitive: false,
        metadata: HashMap::from([("key2".to_string(), "value2".to_string())]),  // 不同 metadata
    };
    
    assert_eq!(item1, item2);
    println!("      ✅ 自定义实现：忽略大小写和 metadata 字段");
    
    // 验证 hash 一致性
    let mut hasher1 = DefaultHasher::new();
    item1.hash(&mut hasher1);
    let hash1 = hasher1.finish();
    
    let mut hasher2 = DefaultHasher::new();
    item2.hash(&mut hasher2);
    let hash2 = hasher2.finish();
    
    assert_eq!(hash1, hash2);
    println!("      ✅ Hash 一致性验证通过");
    
    // 3. 跨类型比较的最佳实践
    println!("\n    🔹 跨类型比较的最佳实践:");
    
    #[derive(Debug)]
    struct UserId(u32);
    
    #[derive(Debug)]
    struct UserIdString(String);
    
    // 实现双向比较，确保对称性
    impl PartialEq<UserIdString> for UserId {
        fn eq(&self, other: &UserIdString) -> bool {
            self.0.to_string() == other.0
        }
    }
    
    impl PartialEq<UserId> for UserIdString {
        fn eq(&self, other: &UserId) -> bool {
            self.0 == other.0.to_string()
        }
    }
    
    // 也要实现同类型比较
    impl PartialEq for UserId {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    
    impl PartialEq for UserIdString {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    
    let user_id = UserId(123);
    let user_id_str = UserIdString("123".to_string());
    
    assert_eq!(user_id, user_id_str);
    assert_eq!(user_id_str, user_id);  // 对称性
    
    println!("      ✅ 跨类型比较：确保对称性");
}

/// Hash 一致性最佳实践
fn hash_consistency_practices() {
    println!("\n  📌 Hash 一致性最佳实践:");
    
    // 1. 黄金法则：相等的对象必须有相同的 hash
    println!("\n    🔹 黄金法则：Eq 和 Hash 一致性:");
    
    #[derive(Debug, Clone)]
    struct ConsistentExample {
        important_field: String,
        ignored_field: String,
    }
    
    impl PartialEq for ConsistentExample {
        fn eq(&self, other: &Self) -> bool {
            // 只比较重要字段
            self.important_field == other.important_field
        }
    }
    
    impl Eq for ConsistentExample {}
    
    impl Hash for ConsistentExample {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // 只 hash 在 PartialEq 中使用的字段
            self.important_field.hash(state);
            // 不包含 ignored_field
        }
    }
    
    let item1 = ConsistentExample {
        important_field: "key".to_string(),
        ignored_field: "value1".to_string(),
    };
    
    let item2 = ConsistentExample {
        important_field: "key".to_string(),
        ignored_field: "value2".to_string(),  // 不同但被忽略
    };
    
    assert_eq!(item1, item2);
    
    // 验证 hash 一致性
    let mut hasher1 = DefaultHasher::new();
    item1.hash(&mut hasher1);
    let hash1 = hasher1.finish();
    
    let mut hasher2 = DefaultHasher::new();
    item2.hash(&mut hasher2);
    let hash2 = hasher2.finish();
    
    assert_eq!(hash1, hash2);
    println!("      ✅ 相等对象有相同 hash 值");
    
    // 2. 浮点数的特殊处理
    println!("\n    🔹 浮点数的特殊处理:");
    
    #[derive(Debug, Clone)]
    struct FloatContainer {
        value: f64,
        tolerance: f64,
    }
    
    impl PartialEq for FloatContainer {
        fn eq(&self, other: &Self) -> bool {
            // 使用容差比较
            (self.value - other.value).abs() < self.tolerance.min(other.tolerance)
        }
    }
    
    // 注意：由于使用容差比较，不能实现 Eq 和 Hash
    // 因为这违反了传递性和 hash 一致性
    
    let f1 = FloatContainer { value: 1.0, tolerance: 0.01 };
    let f2 = FloatContainer { value: 1.005, tolerance: 0.01 };
    
    assert_eq!(f1, f2);
    println!("      ✅ 浮点数容差比较（但不能实现 Eq/Hash）");
    
    // 3. 更好的浮点数处理方式
    println!("\n    🔹 更好的浮点数处理方式:");
    
    #[derive(Debug, Clone)]
    struct QuantizedFloat {
        // 将浮点数量化为整数以支持 Eq 和 Hash
        quantized_value: i64,  // 例如：存储 value * 1000 的整数部分
        scale: u32,
    }
    
    impl QuantizedFloat {
        fn new(value: f64, scale: u32) -> Self {
            let multiplier = 10_i64.pow(scale);
            Self {
                quantized_value: (value * multiplier as f64).round() as i64,
                scale,
            }
        }
        
        fn to_f64(&self) -> f64 {
            self.quantized_value as f64 / 10_f64.powi(self.scale as i32)
        }
    }
    
    impl PartialEq for QuantizedFloat {
        fn eq(&self, other: &Self) -> bool {
            self.quantized_value == other.quantized_value && self.scale == other.scale
        }
    }
    
    impl Eq for QuantizedFloat {}
    
    impl Hash for QuantizedFloat {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.quantized_value.hash(state);
            self.scale.hash(state);
        }
    }
    
    let qf1 = QuantizedFloat::new(1.234, 3);
    let qf2 = QuantizedFloat::new(1.234, 3);
    let qf3 = QuantizedFloat::new(1.235, 3);
    
    assert_eq!(qf1, qf2);
    assert_ne!(qf1, qf3);
    
    // 可以在 HashMap 中使用
    let mut map: HashMap<QuantizedFloat, &str> = HashMap::new();
    map.insert(qf1.clone(), "value1");
    assert_eq!(map.get(&qf2), Some(&"value1"));
    
    println!("      ✅ 量化浮点数：支持 Eq 和 Hash");
}

/// 性能最佳实践
fn performance_best_practices() {
    println!("\n  📌 性能最佳实践:");
    
    // 1. 早期退出策略
    println!("\n    🔹 早期退出策略:");
    
    #[derive(Debug, Clone)]
    struct OptimizedStruct {
        id: u64,           // 最可能不同的字段
        category: u8,      // 快速比较的字段
        name: String,      // 中等成本的字段
        data: Vec<u8>,     // 最昂贵的字段
    }
    
    impl PartialEq for OptimizedStruct {
        fn eq(&self, other: &Self) -> bool {
            // 按照从快到慢的顺序比较
            self.id == other.id &&
            self.category == other.category &&
            self.data.len() == other.data.len() &&  // 长度检查很快
            self.name == other.name &&
            self.data == other.data  // 最昂贵的比较放最后
        }
    }
    
    println!("      ✅ 按成本排序：ID → 类别 → 长度 → 名称 → 数据");
    
    // 2. 内联小函数
    println!("\n    🔹 内联优化:");
    
    #[derive(Debug, Clone)]
    struct InlinedStruct {
        x: i32,
        y: i32,
    }
    
    impl PartialEq for InlinedStruct {
        #[inline]  // 建议编译器内联
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y
        }
    }
    
    impl Eq for InlinedStruct {}
    
    impl Hash for InlinedStruct {
        #[inline]
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.x.hash(state);
            self.y.hash(state);
        }
    }
    
    println!("      ✅ 使用 #[inline] 优化小函数");
    
    // 3. 避免不必要的分配
    println!("\n    🔹 避免不必要的分配:");
    
    #[derive(Debug, Clone)]
    struct EfficientStringComparison {
        content: String,
        case_insensitive: bool,
    }
    
    impl PartialEq for EfficientStringComparison {
        fn eq(&self, other: &Self) -> bool {
            if self.case_insensitive != other.case_insensitive {
                return false;
            }
            
            if self.case_insensitive {
                // 避免创建新字符串，使用迭代器比较
                self.content.len() == other.content.len() &&
                self.content.chars().zip(other.content.chars())
                    .all(|(a, b)| a.to_lowercase().eq(b.to_lowercase()))
            } else {
                self.content == other.content
            }
        }
    }
    
    println!("      ✅ 使用迭代器避免字符串分配");
    
    // 4. 缓存昂贵的计算
    println!("\n    🔹 缓存昂贵的计算:");
    
    use std::cell::Cell;
    
    #[derive(Debug)]
    struct CachedHash {
        data: Vec<u8>,
        cached_hash: Cell<Option<u64>>,
    }
    
    impl CachedHash {
        fn new(data: Vec<u8>) -> Self {
            Self {
                data,
                cached_hash: Cell::new(None),
            }
        }
        
        fn compute_hash(&self) -> u64 {
            if let Some(hash) = self.cached_hash.get() {
                return hash;
            }
            
            let mut hasher = DefaultHasher::new();
            self.data.hash(&mut hasher);
            let hash = hasher.finish();
            self.cached_hash.set(Some(hash));
            hash
        }
    }
    
    impl PartialEq for CachedHash {
        fn eq(&self, other: &Self) -> bool {
            // 首先比较长度（快速）
            if self.data.len() != other.data.len() {
                return false;
            }
            
            // 然后比较 hash（如果已缓存则很快）
            if self.compute_hash() != other.compute_hash() {
                return false;
            }
            
            // 最后比较实际内容
            self.data == other.data
        }
    }
    
    println!("      ✅ 缓存 hash 值加速比较");
}

/// 错误处理模式
fn error_handling_patterns() {
    println!("\n  📌 错误处理模式:");
    
    // 1. 处理可能失败的比较
    println!("\n    🔹 处理可能失败的比较:");
    
    #[derive(Debug, Clone)]
    struct SafeComparison {
        data: Vec<u8>,
    }
    
    impl SafeComparison {
        // 提供一个可能失败的比较方法
        fn try_eq(&self, other: &Self) -> Result<bool, &'static str> {
            if self.data.len() > 1_000_000 || other.data.len() > 1_000_000 {
                return Err("数据太大，比较可能很慢");
            }
            
            Ok(self.data == other.data)
        }
    }
    
    impl PartialEq for SafeComparison {
        fn eq(&self, other: &Self) -> bool {
            // 在 PartialEq 中处理错误情况
            match self.try_eq(other) {
                Ok(result) => result,
                Err(_) => {
                    // 对于过大的数据，只比较长度和前几个字节
                    self.data.len() == other.data.len() &&
                    self.data.get(..100) == other.data.get(..100)
                }
            }
        }
    }
    
    println!("      ✅ 为大数据提供降级比较策略");
    
    // 2. 处理无效状态
    println!("\n    🔹 处理无效状态:");
    
    #[derive(Debug, Clone)]
    struct ValidatedStruct {
        value: i32,
        is_valid: bool,
    }
    
    impl ValidatedStruct {
        fn new(value: i32) -> Self {
            Self {
                value,
                is_valid: value >= 0,  // 简单的验证规则
            }
        }
        
        fn is_valid(&self) -> bool {
            self.is_valid && self.value >= 0
        }
    }
    
    impl PartialEq for ValidatedStruct {
        fn eq(&self, other: &Self) -> bool {
            // 只有两个对象都有效时才进行比较
            match (self.is_valid(), other.is_valid()) {
                (true, true) => self.value == other.value,
                (false, false) => true,  // 两个无效对象被认为相等
                _ => false,  // 一个有效一个无效
            }
        }
    }
    
    let valid1 = ValidatedStruct::new(10);
    let valid2 = ValidatedStruct::new(10);
    let invalid1 = ValidatedStruct::new(-5);
    let invalid2 = ValidatedStruct::new(-3);
    
    assert_eq!(valid1, valid2);
    assert_eq!(invalid1, invalid2);  // 两个无效对象相等
    assert_ne!(valid1, invalid1);
    
    println!("      ✅ 处理有效/无效状态的比较");
}

/// 测试策略
fn testing_strategies() {
    println!("\n  📌 测试策略:");
    
    // 1. 等价关系测试
    println!("\n    🔹 等价关系测试模板:");
    
    fn test_equivalence_relation<T: PartialEq + Clone + std::fmt::Debug>(items: &[T]) {
        // 测试自反性
        for item in items {
            assert_eq!(item, item, "自反性失败");
        }
        
        // 测试对称性
        for i in 0..items.len() {
            for j in 0..items.len() {
                let eq_ij = items[i] == items[j];
                let eq_ji = items[j] == items[i];
                assert_eq!(eq_ij, eq_ji, "对称性失败: {} vs {}", i, j);
            }
        }
        
        // 测试传递性
        for i in 0..items.len() {
            for j in 0..items.len() {
                for k in 0..items.len() {
                    if items[i] == items[j] && items[j] == items[k] {
                        assert_eq!(items[i], items[k], "传递性失败: {} -> {} -> {}", i, j, k);
                    }
                }
            }
        }
    }
    
    // 示例测试
    let test_data = vec![1, 1, 2, 3, 2];
    test_equivalence_relation(&test_data);
    
    println!("      ✅ 等价关系测试模板");
    
    // 2. Hash 一致性测试
    println!("\n    🔹 Hash 一致性测试模板:");
    
    fn test_hash_consistency<T: PartialEq + Hash + Clone>(items: &[T]) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;
        
        for i in 0..items.len() {
            for j in 0..items.len() {
                if items[i] == items[j] {
                    let mut hasher1 = DefaultHasher::new();
                    items[i].hash(&mut hasher1);
                    let hash1 = hasher1.finish();
                    
                    let mut hasher2 = DefaultHasher::new();
                    items[j].hash(&mut hasher2);
                    let hash2 = hasher2.finish();
                    
                    assert_eq!(hash1, hash2, "Hash 不一致: {} vs {}", i, j);
                }
            }
        }
    }
    
    #[derive(PartialEq, Eq, Hash, Clone)]
    struct TestStruct(i32);
    
    let hash_test_data = vec![TestStruct(1), TestStruct(1), TestStruct(2)];
    test_hash_consistency(&hash_test_data);
    
    println!("      ✅ Hash 一致性测试模板");
    
    // 3. 边界情况测试
    println!("\n    🔹 边界情况测试清单:");
    println!("      • 空值/默认值测试");
    println!("      • 极值测试 (最大/最小值)");
    println!("      • 特殊值测试 (NaN, 无穷大等)");
    println!("      • 大数据量测试");
    println!("      • Unicode/特殊字符测试");
    println!("      • 内存压力测试");
}

/// 文档化最佳实践
fn documentation_practices() {
    println!("\n  📌 文档化最佳实践:");
    
    println!("\n    🔹 文档化特殊比较语义:");
    
    /// 用户账户信息
    /// 
    /// # 相等性语义
    /// 
    /// 两个 `UserAccount` 被认为相等当且仅当：
    /// - `user_id` 相同
    /// - `email` 相同（忽略大小写）
    /// 
    /// 注意：`last_login` 和 `metadata` 字段不参与相等性比较。
    /// 
    /// # 示例
    /// 
    /// ```
    /// let user1 = UserAccount {
    ///     user_id: 123,
    ///     email: "user@example.com".to_string(),
    ///     last_login: Some(SystemTime::now()),
    ///     metadata: HashMap::new(),
    /// };
    /// 
    /// let user2 = UserAccount {
    ///     user_id: 123,
    ///     email: "USER@EXAMPLE.COM".to_string(),  // 不同大小写
    ///     last_login: None,  // 不同的登录时间
    ///     metadata: HashMap::from([("key".to_string(), "value".to_string())]),
    /// };
    /// 
    /// assert_eq!(user1, user2);  // 相等！
    /// ```
    #[derive(Debug, Clone)]
    struct UserAccount {
        user_id: u64,
        email: String,
        last_login: Option<std::time::SystemTime>,
        metadata: HashMap<String, String>,
    }
    
    impl PartialEq for UserAccount {
        fn eq(&self, other: &Self) -> bool {
            self.user_id == other.user_id &&
            self.email.to_lowercase() == other.email.to_lowercase()
        }
    }
    
    println!("      ✅ 详细文档化相等性语义和示例");
    
    println!("\n    🔹 性能特征文档化:");
    
    /// 大型数据容器
    /// 
    /// # 性能特征
    /// 
    /// - **比较复杂度**: O(n)，其中 n 是数据长度
    /// - **Hash 复杂度**: O(1) (使用缓存)
    /// - **内存使用**: 每个实例额外使用 8 字节缓存 hash 值
    /// 
    /// # 优化策略
    /// 
    /// - 使用长度检查进行早期退出
    /// - 缓存 hash 值避免重复计算
    /// - 在集合中使用时性能最佳
    #[derive(Debug)]
    struct LargeDataContainer {
        data: Vec<u8>,
        cached_hash: std::cell::Cell<Option<u64>>,
    }
    
    println!("      ✅ 文档化性能特征和优化策略");
}

/// 常见反模式
fn common_antipatterns() {
    println!("\n  📌 常见反模式和解决方案:");
    
    // 1. Hash 和 Eq 不一致
    println!("\n    🔹 反模式：Hash 和 Eq 不一致");
    
    // 错误示例（已在前面展示）
    println!("      ❌ 错误：PartialEq 忽略某字段，但 Hash 包含该字段");
    println!("      ✅ 正确：确保 Hash 只包含 PartialEq 使用的字段");
    
    // 2. 浮点数直接实现 Eq
    println!("\n    🔹 反模式：浮点数直接实现 Eq");
    
    println!("      ❌ 错误：为包含浮点数的类型实现 Eq");
    println!("      ✅ 正确：只实现 PartialEq，或使用量化/有序浮点数");
    
    // 3. 昂贵的比较操作
    println!("\n    🔹 反模式：昂贵的比较操作");
    
    #[derive(Debug, Clone)]
    struct ExpensiveComparison {
        id: u32,
        large_data: Vec<String>,
    }
    
    // 错误的实现
    impl PartialEq for ExpensiveComparison {
        fn eq(&self, other: &Self) -> bool {
            // 错误：总是比较大数据，即使 ID 不同
            self.large_data == other.large_data && self.id == other.id
        }
    }
    
    println!("      ❌ 错误：先比较昂贵的字段");
    println!("      ✅ 正确：先比较便宜的字段，使用早期退出");
    
    // 4. 忽略对称性
    println!("\n    🔹 反模式：忽略对称性");
    
    println!("      ❌ 错误：只实现 A == B，忘记实现 B == A");
    println!("      ✅ 正确：跨类型比较时确保双向实现");
    
    // 5. 不测试边界情况
    println!("\n    🔹 反模式：不测试边界情况");
    
    println!("      ❌ 错误：只测试正常情况");
    println!("      ✅ 正确：测试空值、极值、特殊值等边界情况");
}

/// 设计模式
fn design_patterns() {
    println!("\n  📌 设计模式:");
    
    // 1. 新类型模式
    println!("\n    🔹 新类型模式 (Newtype Pattern):");
    
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct EmailAddress(String);
    
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct PhoneNumber(String);
    
    impl EmailAddress {
        fn new(email: String) -> Result<Self, &'static str> {
            if email.contains('@') {
                Ok(EmailAddress(email.to_lowercase()))
            } else {
                Err("无效的邮箱地址")
            }
        }
    }
    
    // 不能意外地比较邮箱和电话号码
    let _email = EmailAddress::new("user@example.com".to_string()).unwrap();
    let _phone = PhoneNumber("123-456-7890".to_string());
    
    // assert_eq!(email, phone);  // 编译错误！类型不匹配
    
    println!("      ✅ 新类型防止意外的跨类型比较");
    
    // 2. 构建器模式与相等性
    println!("\n    🔹 构建器模式与相等性:");
    
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Config {
        host: String,
        port: u16,
        ssl: bool,
        timeout: u32,
    }
    
    impl Config {
        fn builder() -> ConfigBuilder {
            ConfigBuilder::default()
        }
    }
    
    #[derive(Default)]
    struct ConfigBuilder {
        host: Option<String>,
        port: Option<u16>,
        ssl: Option<bool>,
        timeout: Option<u32>,
    }
    
    impl ConfigBuilder {
        fn host(mut self, host: impl Into<String>) -> Self {
            self.host = Some(host.into());
            self
        }
        
        fn port(mut self, port: u16) -> Self {
            self.port = Some(port);
            self
        }
        
        fn ssl(mut self, ssl: bool) -> Self {
            self.ssl = Some(ssl);
            self
        }
        
        fn timeout(mut self, timeout: u32) -> Self {
            self.timeout = Some(timeout);
            self
        }
        
        fn build(self) -> Config {
            Config {
                host: self.host.unwrap_or_else(|| "localhost".to_string()),
                port: self.port.unwrap_or(8080),
                ssl: self.ssl.unwrap_or(false),
                timeout: self.timeout.unwrap_or(30),
            }
        }
    }
    
    let config1 = Config::builder()
        .host("example.com")
        .port(443)
        .ssl(true)
        .build();
    
    let config2 = Config::builder()
        .host("example.com")
        .ssl(true)
        .port(443)
        .build();
    
    assert_eq!(config1, config2);  // 构建顺序不影响相等性
    
    println!("      ✅ 构建器模式：构建顺序不影响相等性");
    
    // 3. 状态模式与相等性
    println!("\n    🔹 状态模式与相等性:");
    
    #[derive(Debug, Clone, PartialEq, Eq)]
    enum ConnectionState {
        Disconnected,
        Connecting { attempt: u32 },
        Connected { session_id: String },
        Error { message: String },
    }
    
    #[derive(Debug, Clone)]
    struct Connection {
        id: u32,
        state: ConnectionState,
    }
    
    impl PartialEq for Connection {
        fn eq(&self, other: &Self) -> bool {
            // 连接相等性基于 ID 和状态
            self.id == other.id && self.state == other.state
        }
    }
    
    let conn1 = Connection {
        id: 1,
        state: ConnectionState::Connected { session_id: "abc123".to_string() },
    };
    
    let conn2 = Connection {
        id: 1,
        state: ConnectionState::Connected { session_id: "abc123".to_string() },
    };
    
    let conn3 = Connection {
        id: 1,
        state: ConnectionState::Connecting { attempt: 1 },
    };
    
    assert_eq!(conn1, conn2);
    assert_ne!(conn1, conn3);
    
    println!("      ✅ 状态模式：状态变化影响相等性");
}

/// 最佳实践总结
pub fn best_practices_summary() {
    println!("\n🔸 最佳实践总结:");
    
    println!("\n  📋 核心原则:");
    println!("    1. 优先使用 #[derive(PartialEq, Eq)] 自动实现");
    println!("    2. 确保 Hash 和 Eq 的一致性");
    println!("    3. 为浮点数类型只实现 PartialEq");
    println!("    4. 使用早期退出优化性能");
    println!("    5. 跨类型比较时确保对称性");
    
    println!("\n  🛠️ 实现技巧:");
    println!("    • 按成本排序比较字段（便宜的先比较）");
    println!("    • 使用 #[inline] 优化小函数");
    println!("    • 缓存昂贵的计算结果");
    println!("    • 避免不必要的内存分配");
    println!("    • 处理无效状态和错误情况");
    
    println!("\n  🧪 测试策略:");
    println!("    • 测试等价关系性质（自反性、对称性、传递性）");
    println!("    • 验证 Hash 一致性");
    println!("    • 测试边界情况和特殊值");
    println!("    • 性能基准测试");
    println!("    • 内存使用分析");
    
    println!("\n  📚 文档化:");
    println!("    • 明确说明相等性语义");
    println!("    • 文档化性能特征");
    println!("    • 提供使用示例");
    println!("    • 说明特殊情况的处理");
    
    println!("\n  ⚠️ 常见陷阱:");
    println!("    • Hash 和 Eq 不一致");
    println!("    • 浮点数实现 Eq");
    println!("    • 昂贵的比较操作");
    println!("    • 忽略对称性和传递性");
    println!("    • 不测试边界情况");
    
    println!("\n  🎯 设计模式:");
    println!("    • 新类型模式防止意外比较");
    println!("    • 构建器模式与相等性");
    println!("    • 状态模式的相等性设计");
    println!("    • 量化浮点数支持 Eq");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_basic_equality() {
        // 简单的相等性测试
        assert_eq!(1, 1);
        assert_eq!("hello", "hello");
        assert_ne!(1, 2);
    }
    
    #[test]
    fn test_string_equality() {
        let s1 = String::from("test");
        let s2 = String::from("test");
        assert_eq!(s1, s2);
    }
}