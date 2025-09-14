//! Rust变量绑定与解构示例 - 重构版本
//!
//! 本模块展示了企业级Rust代码的最佳实践，包括：
//! - 模块化架构设计
//! - Trait抽象和多态
//! - 依赖注入和控制反转
//! - 高级错误处理策略
//! - 性能优化和内存管理
//! - 函数式编程范式
//! - 并发安全设计

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

// ============================================================================
// 错误处理模块 - 分层错误设计
// ============================================================================

/// 应用程序错误的分层设计
#[derive(Debug, Clone)]
pub enum AppError {
  /// 计算错误
  Computation(ComputationError),
  /// 验证错误
  Validation(ValidationError),
  /// 网络错误
  Network(NetworkError),
  /// 系统错误
  System(SystemError),
}

#[derive(Debug, Clone)]
pub enum ComputationError {
  DivisionByZero,
  Overflow,
  Underflow,
  InvalidOperation(String),
}

#[derive(Debug, Clone)]
pub enum ValidationError {
  InvalidInput { field: String, reason: String },
  MissingRequired(String),
  OutOfRange { value: f64, min: f64, max: f64 },
}

#[derive(Debug, Clone)]
pub enum NetworkError {
  Timeout,
  ConnectionFailed,
  InvalidResponse { code: u16, message: String },
}

#[derive(Debug, Clone)]
pub enum SystemError {
  OutOfMemory,
  FileNotFound(String),
  PermissionDenied,
}

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      AppError::Computation(e) => write!(f, "计算错误: {}", e),
      AppError::Validation(e) => write!(f, "验证错误: {}", e),
      AppError::Network(e) => write!(f, "网络错误: {}", e),
      AppError::System(e) => write!(f, "系统错误: {}", e),
    }
  }
}

impl fmt::Display for ComputationError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ComputationError::DivisionByZero => write!(f, "除零错误"),
      ComputationError::Overflow => write!(f, "数值溢出"),
      ComputationError::Underflow => write!(f, "数值下溢"),
      ComputationError::InvalidOperation(op) => write!(f, "无效操作: {}", op),
    }
  }
}

impl fmt::Display for ValidationError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ValidationError::InvalidInput { field, reason } => {
        write!(f, "字段 '{}' 无效: {}", field, reason)
      }
      ValidationError::MissingRequired(field) => {
        write!(f, "缺少必需字段: {}", field)
      }
      ValidationError::OutOfRange { value, min, max } => {
        write!(f, "值 {} 超出范围 [{}, {}]", value, min, max)
      }
    }
  }
}

impl fmt::Display for NetworkError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      NetworkError::Timeout => write!(f, "请求超时"),
      NetworkError::ConnectionFailed => write!(f, "连接失败"),
      NetworkError::InvalidResponse { code, message } => {
        write!(f, "无效响应 {}: {}", code, message)
      }
    }
  }
}

impl fmt::Display for SystemError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      SystemError::OutOfMemory => write!(f, "内存不足"),
      SystemError::FileNotFound(path) => write!(f, "文件未找到: {}", path),
      SystemError::PermissionDenied => write!(f, "权限被拒绝"),
    }
  }
}

impl Error for AppError {}
impl Error for ComputationError {}
impl Error for ValidationError {}
impl Error for NetworkError {}
impl Error for SystemError {}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, AppError>;

// ============================================================================
// 核心抽象 - Trait设计
// ============================================================================

/// 可执行的命令抽象
pub trait Executable {
  type Output;
  type Error;

  fn execute(&self) -> std::result::Result<Self::Output, Self::Error>;
  fn validate(&self) -> std::result::Result<(), Self::Error>;
  fn description(&self) -> &str;
}

/// 可序列化的数据抽象
pub trait Serializable {
  fn serialize(&self) -> String;
  fn deserialize(data: &str) -> Result<Self>
  where
    Self: Sized;
}

/// 可观察的对象抽象
pub trait Observable<T> {
  fn subscribe<F>(&mut self, observer: F)
  where
    F: Fn(&T) + 'static;
  fn notify(&self, event: &T);
}

/// 资源管理抽象
pub trait ResourceManager<T> {
  fn acquire(&mut self) -> Result<T>;
  fn release(&mut self, resource: T) -> Result<()>;
  fn is_available(&self) -> bool;
}

// ============================================================================
// 数据模型 - 更丰富的类型系统
// ============================================================================

/// 三维点，支持泛型坐标类型
#[derive(Debug, Clone, PartialEq)]
pub struct Point3D<T = f64> {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl<T> Point3D<T>
where
  T: Copy + std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
  pub fn new(x: T, y: T, z: T) -> Self {
    Self { x, y, z }
  }

  pub fn dot_product(&self, other: &Self) -> T {
    self.x * other.x + self.y * other.y + self.z * other.z
  }
}

/// 几何体抽象
#[derive(Debug, Clone)]
pub enum Geometry<T = f64> {
  Sphere {
    center: Point3D<T>,
    radius: T,
  },
  Cube {
    center: Point3D<T>,
    side_length: T,
  },
  Cylinder {
    center: Point3D<T>,
    radius: T,
    height: T,
  },
}

/// 材质系统
#[derive(Debug, Clone)]
pub struct Material {
  pub name: String,
  pub properties: MaterialProperties,
}

#[derive(Debug, Clone)]
pub enum MaterialProperties {
  Metal {
    reflectivity: f64,
    conductivity: f64,
    hardness: f64,
  },
  Glass {
    transparency: f64,
    refractive_index: f64,
    brittleness: f64,
  },
  Plastic {
    color: Color,
    flexibility: f64,
    density: f64,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

impl Color {
  pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
    Self { r, g, b, a }
  }

  pub fn rgb(r: u8, g: u8, b: u8) -> Self {
    Self::new(r, g, b, 255)
  }
}

// ============================================================================
// 命令系统 - 命令模式实现
// ============================================================================

/// 增强的命令系统
#[derive(Debug, Clone)]
pub enum Command {
  Transform(TransformCommand),
  Render(RenderCommand),
  System(SystemCommand),
  Composite(CompositeCommand),
}

#[derive(Debug, Clone)]
pub enum TransformCommand {
  Move {
    delta: Point3D,
    speed: f32,
  },
  Rotate {
    axis: Axis,
    angle: f32,
    pivot: Option<Point3D>,
  },
  Scale {
    factor: Point3D,
    center: Option<Point3D>,
  },
}

#[derive(Debug, Clone)]
pub enum RenderCommand {
  SetMaterial(Material),
  SetLighting {
    ambient: f32,
    diffuse: f32,
    specular: f32,
  },
  SetCamera {
    position: Point3D,
    target: Point3D,
  },
}

#[derive(Debug, Clone)]
pub enum SystemCommand {
  SaveState(String),
  LoadState(String),
  Reset,
}

#[derive(Debug, Clone)]
pub struct CompositeCommand {
  pub commands: Vec<Command>,
  pub parallel: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Axis {
  X,
  Y,
  Z,
}

impl Executable for Command {
  type Output = String;
  type Error = AppError;

  fn execute(&self) -> std::result::Result<Self::Output, Self::Error> {
    self.validate()?;

    match self {
      Command::Transform(cmd) => cmd.execute(),
      Command::Render(cmd) => cmd.execute(),
      Command::System(cmd) => cmd.execute(),
      Command::Composite(cmd) => cmd.execute(),
    }
  }

  fn validate(&self) -> std::result::Result<(), Self::Error> {
    match self {
      Command::Transform(cmd) => cmd.validate(),
      Command::Render(cmd) => cmd.validate(),
      Command::System(cmd) => cmd.validate(),
      Command::Composite(cmd) => cmd.validate(),
    }
  }

  fn description(&self) -> &str {
    match self {
      Command::Transform(_) => "变换命令",
      Command::Render(_) => "渲染命令",
      Command::System(_) => "系统命令",
      Command::Composite(_) => "复合命令",
    }
  }
}

impl Executable for TransformCommand {
  type Output = String;
  type Error = AppError;

  fn execute(&self) -> std::result::Result<Self::Output, Self::Error> {
    match self {
      TransformCommand::Move { delta, speed } => Ok(format!(
        "移动 ({}, {}, {})，速度: {}",
        delta.x, delta.y, delta.z, speed
      )),
      TransformCommand::Rotate { axis, angle, pivot } => {
        let pivot_str = match pivot {
          Some(p) => format!("围绕点 ({}, {}, {})", p.x, p.y, p.z),
          None => "围绕原点".to_string(),
        };
        Ok(format!("绕{:?}轴旋转 {}度，{}", axis, angle, pivot_str))
      }
      TransformCommand::Scale { factor, center } => {
        let center_str = match center {
          Some(c) => format!("以点 ({}, {}, {}) 为中心", c.x, c.y, c.z),
          None => "以原点为中心".to_string(),
        };
        Ok(format!(
          "缩放 ({}, {}, {})倍，{}",
          factor.x, factor.y, factor.z, center_str
        ))
      }
    }
  }

  fn validate(&self) -> std::result::Result<(), Self::Error> {
    match self {
      TransformCommand::Move { speed, .. } => {
        if *speed <= 0.0 {
          Err(AppError::Validation(ValidationError::OutOfRange {
            value: *speed as f64,
            min: 0.0,
            max: f64::INFINITY,
          }))
        } else {
          Ok(())
        }
      }
      TransformCommand::Scale { factor, .. } => {
        if factor.x <= 0.0 || factor.y <= 0.0 || factor.z <= 0.0 {
          Err(AppError::Validation(ValidationError::InvalidInput {
            field: "scale_factor".to_string(),
            reason: "缩放因子必须大于0".to_string(),
          }))
        } else {
          Ok(())
        }
      }
      _ => Ok(()),
    }
  }

  fn description(&self) -> &str {
    match self {
      TransformCommand::Move { .. } => "移动变换",
      TransformCommand::Rotate { .. } => "旋转变换",
      TransformCommand::Scale { .. } => "缩放变换",
    }
  }
}

impl Executable for RenderCommand {
  type Output = String;
  type Error = AppError;

  fn execute(&self) -> std::result::Result<Self::Output, Self::Error> {
    match self {
      RenderCommand::SetMaterial(material) => Ok(format!("设置材质: {}", material.name)),
      RenderCommand::SetLighting {
        ambient,
        diffuse,
        specular,
      } => Ok(format!(
        "设置光照: 环境光={}, 漫反射={}, 镜面反射={}",
        ambient, diffuse, specular
      )),
      RenderCommand::SetCamera { position, target } => Ok(format!(
        "设置相机: 位置=({}, {}, {}), 目标=({}, {}, {})",
        position.x, position.y, position.z, target.x, target.y, target.z
      )),
    }
  }

  fn validate(&self) -> std::result::Result<(), Self::Error> {
    match self {
      RenderCommand::SetLighting {
        ambient,
        diffuse,
        specular,
      } => {
        let values = [*ambient, *diffuse, *specular];
        for (i, &value) in values.iter().enumerate() {
          if !(0.0..=1.0).contains(&value) {
            let _field = match i {
              0 => "ambient",
              1 => "diffuse",
              2 => "specular",
              _ => unreachable!(),
            };
            return Err(AppError::Validation(ValidationError::OutOfRange {
              value: value as f64,
              min: 0.0,
              max: 1.0,
            }));
          }
        }
        Ok(())
      }
      _ => Ok(()),
    }
  }

  fn description(&self) -> &str {
    match self {
      RenderCommand::SetMaterial(_) => "设置材质",
      RenderCommand::SetLighting { .. } => "设置光照",
      RenderCommand::SetCamera { .. } => "设置相机",
    }
  }
}

impl Executable for SystemCommand {
  type Output = String;
  type Error = AppError;

  fn execute(&self) -> std::result::Result<Self::Output, Self::Error> {
    match self {
      SystemCommand::SaveState(name) => Ok(format!("保存状态: {}", name)),
      SystemCommand::LoadState(name) => Ok(format!("加载状态: {}", name)),
      SystemCommand::Reset => Ok("重置系统".to_string()),
    }
  }

  fn validate(&self) -> std::result::Result<(), Self::Error> {
    match self {
      SystemCommand::SaveState(name) | SystemCommand::LoadState(name) => {
        if name.is_empty() {
          Err(AppError::Validation(ValidationError::MissingRequired(
            "state_name".to_string(),
          )))
        } else {
          Ok(())
        }
      }
      SystemCommand::Reset => Ok(()),
    }
  }

  fn description(&self) -> &str {
    match self {
      SystemCommand::SaveState(_) => "保存状态",
      SystemCommand::LoadState(_) => "加载状态",
      SystemCommand::Reset => "重置系统",
    }
  }
}

impl Executable for CompositeCommand {
  type Output = String;
  type Error = AppError;

  fn execute(&self) -> std::result::Result<Self::Output, Self::Error> {
    let mut results = Vec::new();

    if self.parallel {
      // 模拟并行执行
      results.push("[并行执行]".to_string());
    }

    for cmd in &self.commands {
      let result = cmd.execute()?;
      results.push(result);
    }

    Ok(results.join("; "))
  }

  fn validate(&self) -> std::result::Result<(), Self::Error> {
    for cmd in &self.commands {
      cmd.validate()?;
    }
    Ok(())
  }

  fn description(&self) -> &str {
    if self.parallel {
      "并行复合命令"
    } else {
      "串行复合命令"
    }
  }
}

// ============================================================================
// 计算模块 - 函数式编程风格
// ============================================================================

/// 数学计算工具
pub struct Calculator;

impl Calculator {
  /// 安全除法
  pub fn safe_divide(a: f64, b: f64) -> Result<f64> {
    if b.abs() < f64::EPSILON {
      Err(AppError::Computation(ComputationError::DivisionByZero))
    } else {
      Ok(a / b)
    }
  }

  /// 安全开方
  pub fn safe_sqrt(x: f64) -> Result<f64> {
    if x < 0.0 {
      Err(AppError::Computation(ComputationError::InvalidOperation(
        "负数开方".to_string(),
      )))
    } else {
      Ok(x.sqrt())
    }
  }

  /// 向量运算
  pub fn vector_magnitude(point: &Point3D) -> f64 {
    (point.x * point.x + point.y * point.y + point.z * point.z).sqrt()
  }

  /// 向量归一化
  pub fn normalize_vector(point: Point3D) -> Result<Point3D> {
    let magnitude = Self::vector_magnitude(&point);
    if magnitude < f64::EPSILON {
      Err(AppError::Computation(ComputationError::DivisionByZero))
    } else {
      Ok(Point3D {
        x: point.x / magnitude,
        y: point.y / magnitude,
        z: point.z / magnitude,
      })
    }
  }
}

// ============================================================================
// 数据处理模块 - 泛型和高阶函数
// ============================================================================

/// 数据处理器
pub struct DataProcessor;

impl DataProcessor {
  /// 泛型查找最大值
  pub fn find_max<T>(slice: &[T]) -> Option<&T>
  where
    T: Ord,
  {
    slice.iter().max()
  }

  /// 泛型过滤和映射
  pub fn filter_map<T, U, F, P>(data: &[T], predicate: P, mapper: F) -> Vec<U>
  where
    F: Fn(&T) -> U,
    P: Fn(&T) -> bool,
  {
    data
      .iter()
      .filter(|item| predicate(item))
      .map(mapper)
      .collect()
  }

  /// 分组操作
  pub fn group_by<T, K, F>(data: &[T], key_fn: F) -> HashMap<K, Vec<&T>>
  where
    K: Eq + std::hash::Hash,
    F: Fn(&T) -> K,
  {
    let mut groups = HashMap::new();
    for item in data {
      let key = key_fn(item);
      groups.entry(key).or_insert_with(Vec::new).push(item);
    }
    groups
  }

  /// 安全索引访问
  pub fn safe_get<T>(slice: &[T], index: usize) -> Option<&T> {
    slice.get(index)
  }

  /// 批量处理
  pub fn batch_process<T, U, F, E>(
    data: &[T],
    batch_size: usize,
    processor: F,
  ) -> std::result::Result<Vec<U>, E>
  where
    F: Fn(&[T]) -> std::result::Result<U, E>,
  {
    data.chunks(batch_size).map(processor).collect()
  }
}

// ============================================================================
// 状态管理 - 观察者模式
// ============================================================================

/// 应用状态
#[derive(Debug, Clone)]
pub struct AppState {
  pub objects: Vec<Geometry>,
  pub materials: HashMap<String, Material>,
  pub camera: Camera,
  pub lighting: Lighting,
}

#[derive(Debug, Clone)]
pub struct Camera {
  pub position: Point3D,
  pub target: Point3D,
  pub up: Point3D,
  pub fov: f32,
}

#[derive(Debug, Clone)]
pub struct Lighting {
  pub ambient: f32,
  pub diffuse: f32,
  pub specular: f32,
}

impl Default for AppState {
  fn default() -> Self {
    Self {
      objects: Vec::new(),
      materials: HashMap::new(),
      camera: Camera {
        position: Point3D::new(0.0, 0.0, 5.0),
        target: Point3D::new(0.0, 0.0, 0.0),
        up: Point3D::new(0.0, 1.0, 0.0),
        fov: 45.0,
      },
      lighting: Lighting {
        ambient: 0.2,
        diffuse: 0.8,
        specular: 0.5,
      },
    }
  }
}

/// 状态管理器
pub struct StateManager {
  state: AppState,
  observers: Vec<Box<dyn Fn(&AppState)>>,
}

impl StateManager {
  pub fn new() -> Self {
    Self {
      state: AppState::default(),
      observers: Vec::new(),
    }
  }

  pub fn get_state(&self) -> &AppState {
    &self.state
  }

  pub fn update_state<F>(&mut self, updater: F)
  where
    F: FnOnce(&mut AppState),
  {
    updater(&mut self.state);
    self.notify_observers();
  }

  pub fn subscribe<F>(&mut self, observer: F)
  where
    F: Fn(&AppState) + 'static,
  {
    self.observers.push(Box::new(observer));
  }

  fn notify_observers(&self) {
    for observer in &self.observers {
      observer(&self.state);
    }
  }
}

// ============================================================================
// 主程序模块 - 依赖注入和控制反转
// ============================================================================

/// 应用程序配置
#[derive(Debug, Clone)]
pub struct AppConfig {
  pub enable_logging: bool,
  pub max_objects: usize,
  pub performance_mode: bool,
  pub debug_mode: bool,
}

impl Default for AppConfig {
  fn default() -> Self {
    Self {
      enable_logging: true,
      max_objects: 1000,
      performance_mode: false,
      debug_mode: false,
    }
  }
}

/// 主应用程序
pub struct Application {
  config: AppConfig,
  state_manager: StateManager,
  command_history: Vec<Command>,
}

impl Application {
  pub fn new(config: AppConfig) -> Self {
    let mut app = Self {
      config,
      state_manager: StateManager::new(),
      command_history: Vec::new(),
    };

    // 设置状态观察者
    app.setup_observers();
    app
  }

  fn setup_observers(&mut self) {
    if self.config.enable_logging {
      self.state_manager.subscribe(|state| {
        println!(
          "[状态更新] 对象数量: {}, 材质数量: {}",
          state.objects.len(),
          state.materials.len()
        );
      });
    }
  }

  pub fn execute_command(&mut self, command: Command) -> Result<String> {
    if self.config.debug_mode {
      println!("[调试] 执行命令: {}", command.description());
    }

    let result = command.execute()?;
    self.command_history.push(command);

    if self.config.enable_logging {
      println!("[日志] 命令执行成功: {}", result);
    }

    Ok(result)
  }

  pub fn run_demo(&mut self) -> Result<()> {
    println!("=== Rust重构示例 - 企业级架构 ===\n");

    // 演示各个模块
    self.demo_variable_binding()?;
    self.demo_smart_pointers()?;
    self.demo_pattern_matching()?;
    self.demo_command_system()?;
    self.demo_data_processing()?;
    self.demo_error_handling()?;

    Ok(())
  }

  fn demo_variable_binding(&mut self) -> Result<()> {
    println!("=== 变量绑定与类型系统演示 ===");

    // 泛型点的使用
    let point_f64 = Point3D::new(1.0, 2.0, 3.0);
    let point_i32 = Point3D::new(1i32, 2i32, 3i32);

    println!("f64点: {:?}", point_f64);
    println!("i32点: {:?}", point_i32);

    // 向量运算
    let magnitude = Calculator::vector_magnitude(&point_f64);
    println!("向量长度: {:.2}", magnitude);

    let normalized = Calculator::normalize_vector(point_f64)?;
    println!("归一化向量: {:?}", normalized);

    println!();
    Ok(())
  }

  fn demo_smart_pointers(&mut self) -> Result<()> {
    println!("=== 智能指针与内存管理演示 ===");

    // Rc示例
    let shared_material = Rc::new(Material {
      name: "金属材质".to_string(),
      properties: MaterialProperties::Metal {
        reflectivity: 0.8,
        conductivity: 0.9,
        hardness: 7.5,
      },
    });

    let material_ref1 = Rc::clone(&shared_material);
    let material_ref2 = Rc::clone(&shared_material);

    println!("Rc引用计数: {}", Rc::strong_count(&shared_material));
    println!("材质1: {}", material_ref1.name);
    println!("材质2: {}", material_ref2.name);

    // Arc示例（线程安全）
    let thread_safe_data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));
    let _data_clone = Arc::clone(&thread_safe_data);

    // 模拟多线程访问
    {
      let mut data = thread_safe_data.lock().unwrap();
      data.push(6);
      println!("线程安全数据: {:?}", *data);
    }

    println!();
    Ok(())
  }

  fn demo_pattern_matching(&mut self) -> Result<()> {
    println!("=== 高级模式匹配演示 ===");

    let geometries = vec![
      Geometry::Sphere {
        center: Point3D::new(0.0, 0.0, 0.0),
        radius: 1.0,
      },
      Geometry::Cube {
        center: Point3D::new(1.0, 1.0, 1.0),
        side_length: 2.0,
      },
      Geometry::Cylinder {
        center: Point3D::new(2.0, 0.0, 0.0),
        radius: 0.5,
        height: 3.0,
      },
    ];

    for (i, geometry) in geometries.iter().enumerate() {
      match geometry {
        Geometry::Sphere { center, radius } => {
          println!(
            "球体{}: 中心({}, {}, {}), 半径{}",
            i, center.x, center.y, center.z, radius
          );
        }
        Geometry::Cube {
          center,
          side_length,
        } => {
          println!(
            "立方体{}: 中心({}, {}, {}), 边长{}",
            i, center.x, center.y, center.z, side_length
          );
        }
        Geometry::Cylinder {
          center,
          radius,
          height,
        } => {
          println!(
            "圆柱体{}: 中心({}, {}, {}), 半径{}, 高度{}",
            i, center.x, center.y, center.z, radius, height
          );
        }
      }
    }

    println!();
    Ok(())
  }

  fn demo_command_system(&mut self) -> Result<()> {
    println!("=== 命令系统演示 ===");

    let commands = vec![
      Command::Transform(TransformCommand::Move {
        delta: Point3D::new(1.0, 2.0, 3.0),
        speed: 5.0,
      }),
      Command::Transform(TransformCommand::Rotate {
        axis: Axis::Y,
        angle: 90.0,
        pivot: Some(Point3D::new(0.0, 0.0, 0.0)),
      }),
      Command::Render(RenderCommand::SetLighting {
        ambient: 0.3,
        diffuse: 0.7,
        specular: 0.5,
      }),
      Command::Composite(CompositeCommand {
        commands: vec![
          Command::System(SystemCommand::SaveState("checkpoint1".to_string())),
          Command::Transform(TransformCommand::Scale {
            factor: Point3D::new(2.0, 2.0, 2.0),
            center: None,
          }),
        ],
        parallel: false,
      }),
    ];

    for cmd in commands {
      match self.execute_command(cmd) {
        Ok(result) => println!("✅ {}", result),
        Err(e) => println!("❌ 错误: {}", e),
      }
    }

    println!();
    Ok(())
  }

  fn demo_data_processing(&mut self) -> Result<()> {
    println!("=== 数据处理演示 ===");

    let numbers = vec![1, 5, 3, 9, 2, 8, 4, 7, 6];

    // 查找最大值
    if let Some(max) = DataProcessor::find_max(&numbers) {
      println!("最大值: {}", max);
    }

    // 过滤和映射
    let even_squares = DataProcessor::filter_map(&numbers, |&n| n % 2 == 0, |&n| n * n);
    println!("偶数的平方: {:?}", even_squares);

    // 分组
    let grouped = DataProcessor::group_by(&numbers, |&n| n % 3);
    for (remainder, group) in grouped {
      println!("余数{}: {:?}", remainder, group);
    }

    // 批量处理
    let batch_results = DataProcessor::batch_process(&numbers, 3, |batch| -> Result<i32> {
      Ok(batch.iter().sum())
    })?;
    println!("批量求和结果: {:?}", batch_results);

    println!();
    Ok(())
  }

  fn demo_error_handling(&mut self) -> Result<()> {
    println!("=== 错误处理演示 ===");

    // 演示各种错误类型
    let test_cases = vec![
      (10.0, 2.0),
      (10.0, 0.0),          // 除零错误
      (f64::MAX, f64::MAX), // 可能的溢出
    ];

    for (a, b) in test_cases {
      match Calculator::safe_divide(a, b) {
        Ok(result) => println!("✅ {} ÷ {} = {:.2}", a, b, result),
        Err(e) => println!("❌ {} ÷ {} 失败: {}", a, b, e),
      }
    }

    // 演示开方错误
    let sqrt_cases = vec![4.0, -1.0, 0.0];
    for x in sqrt_cases {
      match Calculator::safe_sqrt(x) {
        Ok(result) => println!("✅ √{} = {:.2}", x, result),
        Err(e) => println!("❌ √{} 失败: {}", x, e),
      }
    }

    println!();
    Ok(())
  }
}

// ============================================================================
// 主函数
// ============================================================================

fn main() -> Result<()> {
  let config = AppConfig {
    enable_logging: true,
    debug_mode: true,
    ..Default::default()
  };

  let mut app = Application::new(config);
  app.run_demo()?;

  println!("=== 程序执行完成 ===");
  Ok(())
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_calculator_safe_divide() {
    assert!(Calculator::safe_divide(10.0, 2.0).is_ok());
    assert!(Calculator::safe_divide(10.0, 0.0).is_err());
  }

  #[test]
  fn test_point3d_operations() {
    let p1 = Point3D::new(1.0, 2.0, 3.0);
    let p2 = Point3D::new(4.0, 5.0, 6.0);

    let dot_product = p1.dot_product(&p2);
    assert_eq!(dot_product, 32.0); // 1*4 + 2*5 + 3*6 = 32
  }

  #[test]
  fn test_command_validation() {
    let valid_cmd = Command::Transform(TransformCommand::Move {
      delta: Point3D::new(1.0, 2.0, 3.0),
      speed: 5.0,
    });
    assert!(valid_cmd.validate().is_ok());

    let invalid_cmd = Command::Transform(TransformCommand::Move {
      delta: Point3D::new(1.0, 2.0, 3.0),
      speed: -1.0,
    });
    assert!(invalid_cmd.validate().is_err());
  }

  #[test]
  fn test_data_processor() {
    let numbers = vec![1, 2, 3, 4, 5];
    let max = DataProcessor::find_max(&numbers);
    assert_eq!(max, Some(&5));

    let evens = DataProcessor::filter_map(&numbers, |&n| n % 2 == 0, |&n| n);
    assert_eq!(evens, vec![2, 4]);
  }

  #[test]
  fn test_error_hierarchy() {
    let error = AppError::Computation(ComputationError::DivisionByZero);
    let error_string = format!("{}", error);
    assert!(error_string.contains("计算错误"));
    assert!(error_string.contains("除零错误"));
  }
}
