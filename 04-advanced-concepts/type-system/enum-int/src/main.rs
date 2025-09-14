//! Rust 枚举与整数转换全面教程
//!
//! 本教程深入探讨 Rust 中枚举与整数之间的转换机制，
//! 包括 discriminant、repr 属性、类型转换 trait 等核心概念。

use std::convert::{TryFrom, TryInto};
use std::mem::{discriminant, size_of, transmute};

fn main() {
    println!("=== Rust 枚举与整数转换全面教程 ===");
    println!();

    // 1. 基础概念和 discriminant
    basic_enum_concepts();
    println!();

    // 2. repr 属性详解
    repr_attributes_demo();
    println!();

    // 3. 基础类型转换
    basic_type_conversion();
    println!();

    // 4. 转换 trait 实现
    conversion_traits_demo();
    println!();

    // 5. unsafe transmute 和内存布局
    unsafe_transmute_demo();
    println!();

    // 6. 实际应用场景
    practical_scenarios();
    println!();

    // 7. 高级模式和技巧
    advanced_patterns();
    println!();

    // 8. 性能分析
    performance_analysis();
    println!();

    println!("=== 教程总结 ===");
    println!("本教程全面覆盖了 Rust 枚举与整数转换的各个方面：");
    println!("• Discriminant 和内存布局");
    println!("• repr 属性的各种用法");
    println!("• 类型转换 trait 的实现");
    println!("• unsafe 操作和性能优化");
    println!("• 实际应用场景和最佳实践");
}

// ============================================================================
// 1. 基础概念和 discriminant
// ============================================================================

/// 基础枚举类型
#[derive(Debug, Clone, Copy, PartialEq)]
enum BasicEnum {
    First,
    Second,
    Third,
}

/// 带有显式 discriminant 的枚举
#[derive(Debug, Clone, Copy, PartialEq)]
enum ExplicitDiscriminant {
    Alpha = 10,
    Beta = 20,
    Gamma, // 自动为 21
    Delta = 100,
    Epsilon, // 自动为 101
}

/// 混合类型枚举
#[derive(Debug, PartialEq)]
enum MixedEnum {
    Unit,
    Tuple(i32, String),
    Struct { x: i32, y: i32 },
}

fn basic_enum_concepts() {
    println!("1. 基础概念和 discriminant");
    println!("{}", "=".repeat(40));

    // 基础枚举的 discriminant
    let basic_variants = [BasicEnum::First, BasicEnum::Second, BasicEnum::Third];
    for (_i, variant) in basic_variants.iter().enumerate() {
        println!("BasicEnum::{:?} as u8 = {}", variant, *variant as u8);
        println!("  discriminant: {:?}", discriminant(variant));
    }

    println!();

    // 显式 discriminant
    let explicit_variants = [
        ExplicitDiscriminant::Alpha,
        ExplicitDiscriminant::Beta,
        ExplicitDiscriminant::Gamma,
        ExplicitDiscriminant::Delta,
        ExplicitDiscriminant::Epsilon,
    ];

    for variant in &explicit_variants {
        println!(
            "ExplicitDiscriminant::{:?} as u8 = {}",
            variant, *variant as u8
        );
    }

    println!();

    // 混合枚举的 discriminant
    let mixed_variants = [
        MixedEnum::Unit,
        MixedEnum::Tuple(42, "hello".to_string()),
        MixedEnum::Struct { x: 10, y: 20 },
    ];

    for variant in &mixed_variants {
        println!("MixedEnum discriminant: {:?}", discriminant(variant));
    }

    // 内存大小分析
    println!();
    println!("内存大小分析:");
    println!("BasicEnum size: {} bytes", size_of::<BasicEnum>());
    println!(
        "ExplicitDiscriminant size: {} bytes",
        size_of::<ExplicitDiscriminant>()
    );
    println!("MixedEnum size: {} bytes", size_of::<MixedEnum>());
}

// ============================================================================
// 2. repr 属性详解
// ============================================================================

/// 使用不同 repr 属性的枚举
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum ReprU8 {
    A = 1,
    B = 2,
    C = 255,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum ReprU16 {
    Small = 1,
    Medium = 256,
    Large = 65535,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum ReprI32 {
    Negative = -100,
    Zero = 0,
    Positive = 100,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum ReprC {
    First,
    Second,
    Third,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum ReprCU8 {
    Alpha = 10,
    Beta = 20,
    Gamma = 30,
}

fn repr_attributes_demo() {
    println!("2. repr 属性详解");
    println!("{}", "=".repeat(40));

    // repr(u8) 示例
    println!("repr(u8) 枚举:");
    let u8_variants = [ReprU8::A, ReprU8::B, ReprU8::C];
    for variant in &u8_variants {
        println!("  {:?} as u8 = {}", variant, *variant as u8);
    }
    println!("  ReprU8 size: {} bytes", size_of::<ReprU8>());

    println!();

    // repr(u16) 示例
    println!("repr(u16) 枚举:");
    let u16_variants = [ReprU16::Small, ReprU16::Medium, ReprU16::Large];
    for variant in &u16_variants {
        println!("  {:?} as u16 = {}", variant, *variant as u16);
    }
    println!("  ReprU16 size: {} bytes", size_of::<ReprU16>());

    println!();

    // repr(i32) 示例
    println!("repr(i32) 枚举:");
    let i32_variants = [ReprI32::Negative, ReprI32::Zero, ReprI32::Positive];
    for variant in &i32_variants {
        println!("  {:?} as i32 = {}", variant, *variant as i32);
    }
    println!("  ReprI32 size: {} bytes", size_of::<ReprI32>());

    println!();

    // repr(C) 示例
    println!("repr(C) 枚举:");
    let c_variants = [ReprC::First, ReprC::Second, ReprC::Third];
    for variant in &c_variants {
        println!("  {:?} as isize = {}", variant, *variant as isize);
    }
    println!("  ReprC size: {} bytes", size_of::<ReprC>());

    println!();

    // repr(C, u8) 示例
    println!("repr(C, u8) 枚举:");
    let cu8_variants = [ReprCU8::Alpha, ReprCU8::Beta, ReprCU8::Gamma];
    for variant in &cu8_variants {
        println!("  {:?} as u8 = {}", variant, *variant as u8);
    }
    println!("  ReprCU8 size: {} bytes", size_of::<ReprCU8>());
}

// ============================================================================
// 3. 基础类型转换
// ============================================================================

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    Red = 1,
    Green = 2,
    Blue = 3,
}

#[repr(i16)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North = -1,
    South = 1,
    East = -2,
    West = 2,
}

fn basic_type_conversion() {
    println!("3. 基础类型转换");
    println!("{}", "=".repeat(40));

    // 枚举到整数的转换
    println!("枚举到整数的转换:");
    let colors = [Color::Red, Color::Green, Color::Blue];
    for color in &colors {
        println!("  {:?} as u8 = {}", color, *color as u8);
        println!("  {:?} as u16 = {}", color, *color as u16);
        println!("  {:?} as i32 = {}", color, *color as i32);
    }

    println!();

    // 有符号枚举的转换
    println!("有符号枚举的转换:");
    let directions = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    for direction in &directions {
        println!("  {:?} as i16 = {}", direction, *direction as i16);
        println!("  {:?} as i32 = {}", direction, *direction as i32);
    }

    println!();

    // 类型转换的限制
    println!("类型转换的限制:");

    // 只有无字段枚举可以直接转换
    let color = Color::Red;
    let color_value = color as u8;
    println!("  Color::Red as u8 = {}", color_value);

    // 尝试从整数创建枚举（需要手动实现）
    match color_value {
        1 => println!("  {} 对应 Color::Red", color_value),
        2 => println!("  {} 对应 Color::Green", color_value),
        3 => println!("  {} 对应 Color::Blue", color_value),
        _ => println!("  {} 不是有效的颜色值", color_value),
    }
}

// ============================================================================
// 4. 转换 trait 实现
// ============================================================================

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum Status {
    Pending = 0,
    Running = 1,
    Completed = 2,
    Failed = 3,
}

// 实现 From trait（从枚举到整数）
impl From<Status> for u8 {
    fn from(status: Status) -> Self {
        status as u8
    }
}

// 实现 TryFrom trait（从整数到枚举）
impl TryFrom<u8> for Status {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Status::Pending),
            1 => Ok(Status::Running),
            2 => Ok(Status::Completed),
            3 => Ok(Status::Failed),
            _ => Err("Invalid status value"),
        }
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum ErrorCode {
    Success = 0,
    InvalidInput = 1001,
    NetworkError = 2001,
    DatabaseError = 3001,
    UnknownError = 9999,
}

// 为 ErrorCode 实现双向转换
impl From<ErrorCode> for u16 {
    fn from(code: ErrorCode) -> Self {
        code as u16
    }
}

impl TryFrom<u16> for ErrorCode {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ErrorCode::Success),
            1001 => Ok(ErrorCode::InvalidInput),
            2001 => Ok(ErrorCode::NetworkError),
            3001 => Ok(ErrorCode::DatabaseError),
            9999 => Ok(ErrorCode::UnknownError),
            _ => Err(format!("Unknown error code: {}", value)),
        }
    }
}

fn conversion_traits_demo() {
    println!("4. 转换 trait 实现");
    println!("{}", "=".repeat(40));

    // From trait 使用
    println!("From trait 使用:");
    let status = Status::Running;
    let status_u8: u8 = status.into(); // 使用 Into trait
    let status_u8_2 = u8::from(status); // 使用 From trait
    println!("  Status::Running -> u8: {} (via Into)", status_u8);
    println!("  Status::Running -> u8: {} (via From)", status_u8_2);

    println!();

    // TryFrom trait 使用
    println!("TryFrom trait 使用:");
    let values = [0u8, 1, 2, 3, 4, 255];
    for value in &values {
        match Status::try_from(*value) {
            Ok(status) => println!("  {} -> {:?}", value, status),
            Err(e) => println!("  {} -> Error: {}", value, e),
        }
    }

    println!();

    // TryInto trait 使用
    println!("TryInto trait 使用:");
    for value in &values {
        let result: Result<Status, _> = (*value).try_into();
        match result {
            Ok(status) => println!("  {} -> {:?} (via TryInto)", value, status),
            Err(e) => println!("  {} -> Error: {} (via TryInto)", value, e),
        }
    }

    println!();

    // ErrorCode 转换示例
    println!("ErrorCode 转换示例:");
    let error_codes = [
        ErrorCode::Success,
        ErrorCode::InvalidInput,
        ErrorCode::NetworkError,
    ];
    for code in &error_codes {
        let value: u16 = (*code).into();
        let back: Result<ErrorCode, _> = value.try_into();
        println!("  {:?} -> {} -> {:?}", code, value, back.unwrap());
    }

    // 无效值测试
    let invalid_values = [500u16, 1500, 2500];
    for value in &invalid_values {
        match ErrorCode::try_from(*value) {
            Ok(code) => println!("  {} -> {:?}", value, code),
            Err(e) => println!("  {} -> {}", value, e),
        }
    }
}

// ============================================================================
// 5. unsafe transmute 和内存布局
// ============================================================================

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum NetworkProtocol {
    TCP = 0x06,
    UDP = 0x11,
    ICMP = 0x01,
    HTTP = 0x50,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct PacketHeader {
    protocol: NetworkProtocol,
    length: u16,
    checksum: u16,
}

fn unsafe_transmute_demo() {
    println!("5. unsafe transmute 和内存布局");
    println!("{}", "=".repeat(40));

    // 基本 transmute 示例
    println!("基本 transmute 示例:");
    let protocol = NetworkProtocol::TCP;
    let protocol_value = protocol as u32;
    println!("  NetworkProtocol::TCP as u32 = 0x{:02X}", protocol_value);

    // 使用 transmute 进行转换（需要相同大小）
    unsafe {
        let transmuted: u32 = transmute(protocol);
        println!("  transmute(NetworkProtocol::TCP) = 0x{:02X}", transmuted);

        // 反向转换（危险！需要确保值有效）
        let back: NetworkProtocol = transmute(0x06u32);
        println!("  transmute(0x06u32) = {:?}", back);
    }

    println!();

    // 内存布局分析
    println!("内存布局分析:");
    println!(
        "  NetworkProtocol size: {} bytes",
        size_of::<NetworkProtocol>()
    );
    println!("  u32 size: {} bytes", size_of::<u32>());
    println!("  PacketHeader size: {} bytes", size_of::<PacketHeader>());

    let header = PacketHeader {
        protocol: NetworkProtocol::HTTP,
        length: 1024,
        checksum: 0xABCD,
    };

    println!("  PacketHeader: {:?}", header);

    // 将结构体转换为字节数组
    unsafe {
        let bytes: [u8; size_of::<PacketHeader>()] = transmute(header);
        println!("  Header as bytes: {:02X?}", bytes);

        // 从字节数组恢复结构体
        let restored: PacketHeader = transmute(bytes);
        println!("  Restored header: {:?}", restored);
    }

    println!();

    // transmute 的限制和注意事项
    println!("transmute 的限制和注意事项:");
    println!("  • 只能在相同大小的类型之间转换");
    println!("  • 不进行任何有效性检查");
    println!("  • 可能导致未定义行为");
    println!("  • 应该优先使用安全的转换方法");

    // 安全替代方案
    println!();
    println!("安全替代方案:");
    let protocol_safe = NetworkProtocol::UDP;
    let value_safe = protocol_safe as u32;
    println!(
        "  使用 as 转换: {:?} -> 0x{:02X}",
        protocol_safe, value_safe
    );

    // 使用 to_ne_bytes 和 from_ne_bytes
    let bytes_safe = value_safe.to_ne_bytes();
    let restored_safe = u32::from_ne_bytes(bytes_safe);
    println!(
        "  使用字节转换: 0x{:02X} -> {:02X?} -> 0x{:02X}",
        value_safe, bytes_safe, restored_safe
    );
}

// ============================================================================
// 6. 实际应用场景
// ============================================================================

// FFI 场景：与 C 库交互
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum CLibraryError {
    Success = 0,
    InvalidParameter = -1,
    OutOfMemory = -2,
    NetworkTimeout = -3,
    FileNotFound = -4,
}

impl From<CLibraryError> for i32 {
    fn from(error: CLibraryError) -> Self {
        error as i32
    }
}

impl TryFrom<i32> for CLibraryError {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CLibraryError::Success),
            -1 => Ok(CLibraryError::InvalidParameter),
            -2 => Ok(CLibraryError::OutOfMemory),
            -3 => Ok(CLibraryError::NetworkTimeout),
            -4 => Ok(CLibraryError::FileNotFound),
            _ => Err("Unknown C library error code"),
        }
    }
}

// 协议解析场景
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum HttpMethod {
    GET = 1,
    POST = 2,
    PUT = 3,
    DELETE = 4,
    PATCH = 5,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum HttpStatusCode {
    Ok = 200,
    Created = 201,
    BadRequest = 400,
    Unauthorized = 401,
    NotFound = 404,
    InternalServerError = 500,
}

// 状态机场景
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum ConnectionState {
    Disconnected = 0,
    Connecting = 1,
    Connected = 2,
    Authenticating = 3,
    Authenticated = 4,
    Error = 255,
}

impl ConnectionState {
    fn transition(&self, event: ConnectionEvent) -> Result<ConnectionState, &'static str> {
        use ConnectionEvent::*;
        use ConnectionState::*;

        match (self, event) {
            (Disconnected, Connect) => Ok(Connecting),
            (Connecting, ConnectionEstablished) => Ok(Connected),
            (Connected, Authenticate) => Ok(Authenticating),
            (Authenticating, AuthenticationSuccess) => Ok(Authenticated),
            (_, Disconnect) => Ok(Disconnected),
            (_, ErrorOccurred) => Ok(Error),
            _ => Err("Invalid state transition"),
        }
    }

    fn can_send_data(&self) -> bool {
        matches!(self, ConnectionState::Authenticated)
    }
}

#[derive(Debug, Clone, Copy)]
enum ConnectionEvent {
    Connect,
    Disconnect,
    ConnectionEstablished,
    Authenticate,
    AuthenticationSuccess,
    ErrorOccurred,
}

fn practical_scenarios() {
    println!("6. 实际应用场景");
    println!("{}", "=".repeat(40));

    // FFI 场景演示
    println!("FFI 场景演示:");
    let c_errors = [
        CLibraryError::Success,
        CLibraryError::InvalidParameter,
        CLibraryError::OutOfMemory,
    ];

    for error in &c_errors {
        let code: i32 = (*error).into();
        let back: Result<CLibraryError, _> = code.try_into();
        println!("  {:?} <-> {} <-> {:?}", error, code, back.unwrap());
    }

    // 模拟 C 函数返回值
    fn simulate_c_function() -> i32 {
        -2 // OutOfMemory
    }

    let c_result = simulate_c_function();
    match CLibraryError::try_from(c_result) {
        Ok(error) => println!("  C 函数返回: {:?}", error),
        Err(e) => println!("  未知 C 错误码: {} ({})", c_result, e),
    }

    println!();

    // 协议解析场景
    println!("协议解析场景:");

    // 模拟解析 HTTP 请求
    fn parse_http_method(byte: u8) -> Option<HttpMethod> {
        match byte {
            1 => Some(HttpMethod::GET),
            2 => Some(HttpMethod::POST),
            3 => Some(HttpMethod::PUT),
            4 => Some(HttpMethod::DELETE),
            5 => Some(HttpMethod::PATCH),
            _ => None,
        }
    }

    let method_bytes = [1u8, 2, 3, 6, 255];
    for byte in &method_bytes {
        match parse_http_method(*byte) {
            Some(method) => println!("  字节 {} -> {:?}", byte, method),
            None => println!("  字节 {} -> 未知 HTTP 方法", byte),
        }
    }

    // HTTP 状态码处理
    let status_codes = [200u16, 404, 500, 999];
    for code in &status_codes {
        let status = match *code {
            200 => Some(HttpStatusCode::Ok),
            201 => Some(HttpStatusCode::Created),
            400 => Some(HttpStatusCode::BadRequest),
            401 => Some(HttpStatusCode::Unauthorized),
            404 => Some(HttpStatusCode::NotFound),
            500 => Some(HttpStatusCode::InternalServerError),
            _ => None,
        };

        match status {
            Some(s) => println!("  状态码 {} -> {:?}", code, s),
            None => println!("  状态码 {} -> 未知状态", code),
        }
    }

    println!();

    // 状态机场景
    println!("状态机场景:");
    let mut state = ConnectionState::Disconnected;
    let events = [
        ConnectionEvent::Connect,
        ConnectionEvent::ConnectionEstablished,
        ConnectionEvent::Authenticate,
        ConnectionEvent::AuthenticationSuccess,
    ];

    println!("  初始状态: {:?} (值: {})", state, state as u8);

    for event in &events {
        match state.transition(*event) {
            Ok(new_state) => {
                println!(
                    "  {:?} + {:?} -> {:?} (值: {})",
                    state, event, new_state, new_state as u8
                );
                state = new_state;
                println!("    可以发送数据: {}", state.can_send_data());
            }
            Err(e) => {
                println!("  {:?} + {:?} -> 错误: {}", state, event, e);
            }
        }
    }

    // 错误处理
    println!();
    println!("  错误处理演示:");
    match state.transition(ConnectionEvent::ErrorOccurred) {
        Ok(error_state) => {
            println!(
                "  发生错误 -> {:?} (值: {})",
                error_state, error_state as u8
            );
        }
        Err(e) => println!("  状态转换失败: {}", e),
    }
}

// ============================================================================
// 7. 高级模式和技巧
// ============================================================================

// 使用宏生成转换代码
macro_rules! impl_enum_conversion {
    ($enum_name:ident, $int_type:ty, $error_type:ty, {
        $($variant:ident = $value:expr),* $(,)?
    }) => {
        impl From<$enum_name> for $int_type {
            fn from(e: $enum_name) -> Self {
                e as $int_type
            }
        }

        impl TryFrom<$int_type> for $enum_name {
            type Error = $error_type;

            fn try_from(value: $int_type) -> Result<Self, Self::Error> {
                match value {
                    $($value => Ok($enum_name::$variant),)*
                    _ => Err("Invalid enum value".into()),
                }
            }
        }
    };
}

// 使用宏定义枚举和转换
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

impl_enum_conversion!(Priority, u8, String, {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
});

// 零成本抽象示例
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
struct TypedId<T> {
    id: u32,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> TypedId<T> {
    fn new(id: u32) -> Self {
        Self {
            id,
            _phantom: std::marker::PhantomData,
        }
    }

    fn value(&self) -> u32 {
        self.id
    }
}

// 类型安全的 ID 系统
#[derive(Debug)]
struct User;
#[derive(Debug)]
struct Product;
#[derive(Debug)]
struct Order;

type UserId = TypedId<User>;
type ProductId = TypedId<Product>;
type OrderId = TypedId<Order>;

// 高级枚举模式：带有关联数据的状态
#[derive(Debug, Clone)]
enum TaskState {
    Pending { created_at: u64 },
    Running { started_at: u64, progress: f32 },
    Completed { finished_at: u64, result: String },
    Failed { failed_at: u64, error: String },
}

impl TaskState {
    fn discriminant_value(&self) -> u8 {
        match self {
            TaskState::Pending { .. } => 0,
            TaskState::Running { .. } => 1,
            TaskState::Completed { .. } => 2,
            TaskState::Failed { .. } => 3,
        }
    }

    fn timestamp(&self) -> u64 {
        match self {
            TaskState::Pending { created_at } => *created_at,
            TaskState::Running { started_at, .. } => *started_at,
            TaskState::Completed { finished_at, .. } => *finished_at,
            TaskState::Failed { failed_at, .. } => *failed_at,
        }
    }
}

fn advanced_patterns() {
    println!("7. 高级模式和技巧");
    println!("{}", "=".repeat(40));

    // 宏生成的转换
    println!("宏生成的转换:");
    let priorities = [
        Priority::Low,
        Priority::Medium,
        Priority::High,
        Priority::Critical,
    ];
    for priority in &priorities {
        let value: u8 = (*priority).into();
        let back: Result<Priority, _> = value.try_into();
        println!("  {:?} <-> {} <-> {:?}", priority, value, back.unwrap());
    }

    // 测试无效值
    let invalid_values = [0u8, 5, 255];
    for value in &invalid_values {
        match Priority::try_from(*value) {
            Ok(p) => println!("  {} -> {:?}", value, p),
            Err(e) => println!("  {} -> 错误: {}", value, e),
        }
    }

    println!();

    // 零成本抽象
    println!("零成本抽象 - 类型安全的 ID:");
    let user_id = UserId::new(12345);
    let product_id = ProductId::new(67890);
    let order_id = OrderId::new(11111);

    println!("  UserId: {:?} (值: {})", user_id, user_id.value());
    println!("  ProductId: {:?} (值: {})", product_id, product_id.value());
    println!("  OrderId: {:?} (值: {})", order_id, order_id.value());

    // 编译时类型安全
    // user_id == product_id; // 编译错误！不同类型无法比较

    println!("  TypedId<User> size: {} bytes", size_of::<UserId>());
    println!("  u32 size: {} bytes", size_of::<u32>());
    println!("  零成本抽象：TypedId 和 u32 大小相同！");

    println!();

    // 复杂枚举状态管理
    println!("复杂枚举状态管理:");
    let task_states = vec![
        TaskState::Pending { created_at: 1000 },
        TaskState::Running {
            started_at: 1100,
            progress: 0.5,
        },
        TaskState::Completed {
            finished_at: 1200,
            result: "Success".to_string(),
        },
        TaskState::Failed {
            failed_at: 1150,
            error: "Network timeout".to_string(),
        },
    ];

    for (i, state) in task_states.iter().enumerate() {
        println!("  任务 {}: {:?}", i + 1, state);
        println!("    判别值: {}", state.discriminant_value());
        println!("    时间戳: {}", state.timestamp());
        println!("    内存大小: {} bytes", size_of_val(state));
    }

    println!();

    // 枚举大小优化
    println!("枚举大小优化:");

    #[repr(u8)]
    #[derive(Debug)]
    enum OptimizedEnum {
        Small = 1,
        Medium = 2,
        Large = 3,
    }

    #[derive(Debug)]
    enum UnoptimizedEnum {
        Small,
        Medium,
        Large,
    }

    println!(
        "  OptimizedEnum (repr(u8)) size: {} bytes",
        size_of::<OptimizedEnum>()
    );
    println!(
        "  UnoptimizedEnum size: {} bytes",
        size_of::<UnoptimizedEnum>()
    );

    // Option 优化
    println!(
        "  Option<OptimizedEnum> size: {} bytes",
        size_of::<Option<OptimizedEnum>>()
    );
    println!(
        "  Option<UnoptimizedEnum> size: {} bytes",
        size_of::<Option<UnoptimizedEnum>>()
    );
}

// ============================================================================
// 8. 性能分析
// ============================================================================

use std::time::Instant;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum BenchmarkEnum {
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    F = 6,
    G = 7,
    H = 8,
    I = 9,
    J = 10,
}

impl From<BenchmarkEnum> for u8 {
    fn from(e: BenchmarkEnum) -> Self {
        e as u8
    }
}

impl TryFrom<u8> for BenchmarkEnum {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(BenchmarkEnum::A),
            2 => Ok(BenchmarkEnum::B),
            3 => Ok(BenchmarkEnum::C),
            4 => Ok(BenchmarkEnum::D),
            5 => Ok(BenchmarkEnum::E),
            6 => Ok(BenchmarkEnum::F),
            7 => Ok(BenchmarkEnum::G),
            8 => Ok(BenchmarkEnum::H),
            9 => Ok(BenchmarkEnum::I),
            10 => Ok(BenchmarkEnum::J),
            _ => Err(()),
        }
    }
}

fn performance_analysis() {
    println!("8. 性能分析");
    println!("{}", "=".repeat(40));

    const ITERATIONS: usize = 1_000_000;
    let test_enum = BenchmarkEnum::E;
    let test_value = 5u8;

    // 枚举到整数转换性能
    println!("枚举到整数转换性能测试 ({} 次迭代):", ITERATIONS);

    // 使用 as 转换
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _: u8 = test_enum as u8;
    }
    let as_duration = start.elapsed();
    println!("  as 转换: {:?}", as_duration);

    // 使用 Into trait
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _: u8 = test_enum.into();
    }
    let into_duration = start.elapsed();
    println!("  Into trait: {:?}", into_duration);

    // 使用 From trait
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = u8::from(test_enum);
    }
    let from_duration = start.elapsed();
    println!("  From trait: {:?}", from_duration);

    println!();

    // 整数到枚举转换性能
    println!("整数到枚举转换性能测试 ({} 次迭代):", ITERATIONS);

    // 使用 TryFrom trait
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = BenchmarkEnum::try_from(test_value).unwrap();
    }
    let try_from_duration = start.elapsed();
    println!("  TryFrom trait: {:?}", try_from_duration);

    // 使用 match 表达式
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = match test_value {
            1 => BenchmarkEnum::A,
            2 => BenchmarkEnum::B,
            3 => BenchmarkEnum::C,
            4 => BenchmarkEnum::D,
            5 => BenchmarkEnum::E,
            6 => BenchmarkEnum::F,
            7 => BenchmarkEnum::G,
            8 => BenchmarkEnum::H,
            9 => BenchmarkEnum::I,
            10 => BenchmarkEnum::J,
            _ => panic!("Invalid value"),
        };
    }
    let match_duration = start.elapsed();
    println!("  match 表达式: {:?}", match_duration);

    // 使用 unsafe transmute（仅用于演示，实际不推荐）
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        unsafe {
            let _: BenchmarkEnum = transmute(test_value);
        }
    }
    let transmute_duration = start.elapsed();
    println!("  unsafe transmute: {:?}", transmute_duration);

    println!();

    // 内存使用分析
    println!("内存使用分析:");

    #[repr(u8)]
    enum SmallEnum {
        A,
        B,
        C,
    }

    #[repr(u32)]
    enum LargeEnum {
        A,
        B,
        C,
    }

    enum DefaultEnum {
        A,
        B,
        C,
    }

    println!("  SmallEnum (repr(u8)): {} bytes", size_of::<SmallEnum>());
    println!("  LargeEnum (repr(u32)): {} bytes", size_of::<LargeEnum>());
    println!("  DefaultEnum: {} bytes", size_of::<DefaultEnum>());

    // 数组中的内存使用
    const ARRAY_SIZE: usize = 1000;
    println!(
        "  SmallEnum[{}]: {} bytes",
        ARRAY_SIZE,
        size_of::<[SmallEnum; ARRAY_SIZE]>()
    );
    println!(
        "  LargeEnum[{}]: {} bytes",
        ARRAY_SIZE,
        size_of::<[LargeEnum; ARRAY_SIZE]>()
    );
    println!(
        "  DefaultEnum[{}]: {} bytes",
        ARRAY_SIZE,
        size_of::<[DefaultEnum; ARRAY_SIZE]>()
    );

    println!();

    // 编译时优化分析
    println!("编译时优化分析:");
    println!("  • as 转换通常被优化为零成本操作");
    println!("  • From/Into trait 在 release 模式下通常内联");
    println!("  • TryFrom 包含错误检查，有轻微开销");
    println!("  • match 表达式可能被优化为跳转表");
    println!("  • transmute 是零成本的，但不安全");
    println!("  • repr 属性影响内存布局和大小");

    // 性能建议
    println!();
    println!("性能建议:");
    println!("  1. 对于简单转换，使用 as 操作符");
    println!("  2. 需要错误处理时，使用 TryFrom/TryInto");
    println!("  3. 选择合适的 repr 类型以优化内存使用");
    println!("  4. 在 release 模式下测试性能");
    println!("  5. 避免不必要的 unsafe 操作");
}

// ============================================================================
// 测试模块
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_enum_conversion() {
        assert_eq!(BasicEnum::First as u8, 0);
        assert_eq!(BasicEnum::Second as u8, 1);
        assert_eq!(BasicEnum::Third as u8, 2);
    }

    #[test]
    fn test_explicit_discriminant() {
        assert_eq!(ExplicitDiscriminant::Alpha as u8, 10);
        assert_eq!(ExplicitDiscriminant::Beta as u8, 20);
        assert_eq!(ExplicitDiscriminant::Gamma as u8, 21);
        assert_eq!(ExplicitDiscriminant::Delta as u8, 100);
        assert_eq!(ExplicitDiscriminant::Epsilon as u8, 101);
    }

    #[test]
    fn test_repr_attributes() {
        assert_eq!(size_of::<ReprU8>(), 1);
        assert_eq!(size_of::<ReprU16>(), 2);
        assert_eq!(size_of::<ReprI32>(), 4);
    }

    #[test]
    fn test_conversion_traits() {
        let status = Status::Running;
        let value: u8 = status.into();
        assert_eq!(value, 1);

        let back = Status::try_from(value).unwrap();
        assert_eq!(back, status);

        assert!(Status::try_from(255u8).is_err());
    }

    #[test]
    fn test_macro_generated_conversion() {
        let priority = Priority::High;
        let value: u8 = priority.into();
        assert_eq!(value, 3);

        let back: Priority = value.try_into().unwrap();
        assert_eq!(back, priority);
    }

    #[test]
    fn test_zero_cost_abstraction() {
        let user_id = UserId::new(12345);
        assert_eq!(user_id.value(), 12345);
        assert_eq!(size_of::<UserId>(), size_of::<u32>());
    }

    #[test]
    fn test_task_state_discriminant() {
        let pending = TaskState::Pending { created_at: 1000 };
        let running = TaskState::Running {
            started_at: 1100,
            progress: 0.5,
        };
        let completed = TaskState::Completed {
            finished_at: 1200,
            result: "Success".to_string(),
        };
        let failed = TaskState::Failed {
            failed_at: 1150,
            error: "Error".to_string(),
        };

        assert_eq!(pending.discriminant_value(), 0);
        assert_eq!(running.discriminant_value(), 1);
        assert_eq!(completed.discriminant_value(), 2);
        assert_eq!(failed.discriminant_value(), 3);
    }

    #[test]
    fn test_connection_state_machine() {
        let mut state = ConnectionState::Disconnected;

        state = state.transition(ConnectionEvent::Connect).unwrap();
        assert_eq!(state, ConnectionState::Connecting);

        state = state
            .transition(ConnectionEvent::ConnectionEstablished)
            .unwrap();
        assert_eq!(state, ConnectionState::Connected);

        assert!(!state.can_send_data());

        state = state.transition(ConnectionEvent::Authenticate).unwrap();
        assert_eq!(state, ConnectionState::Authenticating);

        state = state
            .transition(ConnectionEvent::AuthenticationSuccess)
            .unwrap();
        assert_eq!(state, ConnectionState::Authenticated);

        assert!(state.can_send_data());
    }

    #[test]
    fn test_unsafe_transmute() {
        let protocol = NetworkProtocol::TCP;
        let value = protocol as u32;
        assert_eq!(value, 0x06);

        unsafe {
            let transmuted: u32 = transmute(protocol);
            assert_eq!(transmuted, 0x06);

            let back: NetworkProtocol = transmute(0x06u32);
            assert_eq!(back, NetworkProtocol::TCP);
        }
    }
}
