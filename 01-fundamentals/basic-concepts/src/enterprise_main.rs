//! 企业级Rust应用架构示例
//!
//! 这个文件展示了如何将学习代码重构为生产级别的Rust应用，
//! 包含了现代软件工程的最佳实践。

use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

// ============================================================================
// 错误处理系统 - 分层错误设计
// ============================================================================

/// 应用程序顶层错误类型
#[derive(Debug, Clone)]
pub enum AppError {
  Computation(ComputationError),
  Validation(ValidationError),
  Network(NetworkError),
  System(SystemError),
}

/// 计算相关错误
#[derive(Debug, Clone)]
pub enum ComputationError {
  DivisionByZero,
  Overflow,
  InvalidOperation(String),
}

/// 验证相关错误
#[derive(Debug, Clone)]
pub enum ValidationError {
  InvalidInput { field: String, reason: String },
  MissingRequired(String),
  OutOfRange { value: f64, min: f64, max: f64 },
}

/// 网络相关错误
#[derive(Debug, Clone)]
pub enum NetworkError {
  ConnectionFailed,
  Timeout,
  InvalidResponse { code: u16, message: String },
}

/// 系统相关错误
#[derive(Debug, Clone)]
pub enum SystemError {
  FileNotFound(String),
  PermissionDenied,
  ResourceExhausted,
}

/// 统一的Result类型
pub type Result<T> = std::result::Result<T, AppError>;

// 错误转换实现
impl From<ComputationError> for AppError {
  fn from(err: ComputationError) -> Self {
    AppError::Computation(err)
  }
}

impl From<ValidationError> for AppError {
  fn from(err: ValidationError) -> Self {
    AppError::Validation(err)
  }
}

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      AppError::Computation(e) => write!(f, "计算错误: {:?}", e),
      AppError::Validation(e) => write!(f, "验证错误: {:?}", e),
      AppError::Network(e) => write!(f, "网络错误: {:?}", e),
      AppError::System(e) => write!(f, "系统错误: {:?}", e),
    }
  }
}

impl std::error::Error for AppError {}

// ============================================================================
// 核心抽象层 - Trait系统
// ============================================================================

/// 可执行的命令抽象
pub trait Executable {
  type Output;
  type Error;

  fn execute(&self) -> std::result::Result<Self::Output, Self::Error>;
  fn validate(&self) -> std::result::Result<(), Self::Error>;
  fn description(&self) -> &str;
}

/// 可序列化的对象抽象
pub trait Serializable {
  fn serialize(&self) -> Result<String>;
  fn deserialize(data: &str) -> Result<Self>
  where
    Self: Sized;
}

/// 可观察的对象抽象
pub trait Observable<T> {
  fn subscribe<F>(&mut self, observer: F)
  where
    F: Fn(&T) + 'static;
  fn notify(&self, data: &T);
}

/// 资源管理抽象
pub trait ResourceManager<T> {
  fn acquire(&mut self) -> Result<T>;
  fn release(&mut self, resource: T) -> Result<()>;
  fn is_available(&self) -> bool;
  fn capacity(&self) -> usize;
}

// ============================================================================
// 数据模型 - 泛型和类型安全
// ============================================================================

/// 泛型3D点
#[derive(Debug, Clone, PartialEq)]
pub struct Point3D<T = f64> {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl<T> Point3D<T> {
  pub fn new(x: T, y: T, z: T) -> Self {
    Self { x, y, z }
  }
}

impl<T> Point3D<T>
where
  T: Copy + std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
  /// 计算点积
  pub fn dot_product(&self, other: &Self) -> T {
    self.x * other.x + self.y * other.y + self.z * other.z
  }
}

impl Point3D<f64> {
  /// 计算向量长度
  pub fn magnitude(&self) -> f64 {
    (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
  }

  /// 归一化向量
  pub fn normalize(&self) -> Result<Self> {
    let mag = self.magnitude();
    if mag == 0.0 {
      return Err(ComputationError::DivisionByZero.into());
    }
    Ok(Point3D::new(self.x / mag, self.y / mag, self.z / mag))
  }
}

/// 几何体枚举
#[derive(Debug, Clone)]
pub enum Geometry<T = f64> {
  Point(Point3D<T>),
  Line {
    start: Point3D<T>,
    end: Point3D<T>,
  },
  Triangle {
    a: Point3D<T>,
    b: Point3D<T>,
    c: Point3D<T>,
  },
  Sphere {
    center: Point3D<T>,
    radius: T,
  },
}

/// 材质属性
#[derive(Debug, Clone)]
pub enum MaterialProperties {
  Metal {
    reflectivity: f64,
    roughness: f64,
  },
  Glass {
    transparency: f64,
    refractive_index: f64,
  },
  Plastic {
    color: Color,
    shininess: f64,
  },
  Emissive {
    color: Color,
    intensity: f64,
  },
}

/// 颜色定义
#[derive(Debug, Clone, PartialEq)]
pub struct Color {
  pub r: f64,
  pub g: f64,
  pub b: f64,
  pub a: f64,
}

impl Color {
  pub fn new(r: f64, g: f64, b: f64, a: f64) -> Result<Self> {
    for &value in &[r, g, b, a] {
      if !(0.0..=1.0).contains(&value) {
        return Err(
          ValidationError::OutOfRange {
            value,
            min: 0.0,
            max: 1.0,
          }
          .into(),
        );
      }
    }
    Ok(Self { r, g, b, a })
  }

  pub const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
  };
  pub const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
  };
  pub const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
  };
}

// ============================================================================
// 命令系统 - 命令模式实现
// ============================================================================

/// 坐标轴枚举
#[derive(Debug, Clone, Copy)]
pub enum Axis {
  X,
  Y,
  Z,
}

/// 分层命令系统
#[derive(Debug, Clone)]
pub enum Command {
  Transform(TransformCommand),
  Render(RenderCommand),
  System(SystemCommand),
  Composite(CompositeCommand),
}

/// 变换命令
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

/// 渲染命令
#[derive(Debug, Clone)]
pub enum RenderCommand {
  SetMaterial(MaterialProperties),
  SetLighting {
    ambient: f64,
    diffuse: f64,
    specular: f64,
  },
  Render {
    geometry: Geometry,
    material: MaterialProperties,
  },
}

/// 系统命令
#[derive(Debug, Clone)]
pub enum SystemCommand {
  SaveState(String),
  LoadState(String),
  Reset,
  Exit,
}

/// 复合命令
#[derive(Debug, Clone)]
pub struct CompositeCommand {
  pub commands: Vec<Command>,
  pub parallel: bool,
  pub rollback_on_error: bool,
}

// 命令验证和执行
impl Executable for Command {
  type Output = String;
  type Error = AppError;

  fn execute(&self) -> Result<Self::Output> {
    self.validate()?;

    match self {
      Command::Transform(cmd) => match cmd {
        TransformCommand::Move { delta, speed } => Ok(format!(
          "移动到 ({}, {}, {})，速度: {}",
          delta.x, delta.y, delta.z, speed
        )),
        TransformCommand::Rotate { axis, angle, pivot } => {
          let pivot_str = match pivot {
            Some(p) => format!("围绕点 ({}, {}, {})", p.x, p.y, p.z),
            None => "围绕原点".to_string(),
          };
          Ok(format!("绕 {:?} 轴旋转 {} 度，{}", axis, angle, pivot_str))
        }
        TransformCommand::Scale { factor, center } => {
          let center_str = match center {
            Some(c) => format!("以 ({}, {}, {}) 为中心", c.x, c.y, c.z),
            None => "以原点为中心".to_string(),
          };
          Ok(format!(
            "缩放 ({}, {}, {})，{}",
            factor.x, factor.y, factor.z, center_str
          ))
        }
      },
      Command::Render(cmd) => match cmd {
        RenderCommand::SetMaterial(material) => Ok(format!("设置材质: {:?}", material)),
        RenderCommand::SetLighting {
          ambient,
          diffuse,
          specular,
        } => Ok(format!(
          "设置光照: 环境光={}, 漫反射={}, 镜面反射={}",
          ambient, diffuse, specular
        )),
        RenderCommand::Render { geometry, material } => {
          Ok(format!("渲染几何体: {:?}，材质: {:?}", geometry, material))
        }
      },
      Command::System(cmd) => match cmd {
        SystemCommand::SaveState(name) => Ok(format!("保存状态: {}", name)),
        SystemCommand::LoadState(name) => Ok(format!("加载状态: {}", name)),
        SystemCommand::Reset => Ok("重置系统".to_string()),
        SystemCommand::Exit => Ok("退出应用".to_string()),
      },
      Command::Composite(cmd) => {
        let mut results = Vec::new();
        for command in &cmd.commands {
          match command.execute() {
            Ok(result) => results.push(result),
            Err(e) if cmd.rollback_on_error => {
              return Err(e);
            }
            Err(e) => {
              results.push(format!("错误: {}", e));
            }
          }
        }
        Ok(format!("复合命令执行完成: [{}]", results.join(", ")))
      }
    }
  }

  fn validate(&self) -> Result<()> {
    match self {
      Command::Transform(TransformCommand::Move { speed, .. }) => {
        if *speed < 0.0 {
          return Err(
            ValidationError::InvalidInput {
              field: "speed".to_string(),
              reason: "速度不能为负数".to_string(),
            }
            .into(),
          );
        }
      }
      Command::Transform(TransformCommand::Scale { factor, .. }) => {
        if factor.x <= 0.0 || factor.y <= 0.0 || factor.z <= 0.0 {
          return Err(
            ValidationError::InvalidInput {
              field: "scale_factor".to_string(),
              reason: "缩放因子必须为正数".to_string(),
            }
            .into(),
          );
        }
      }
      Command::Render(RenderCommand::SetLighting {
        ambient,
        diffuse,
        specular,
      }) => {
        for (value, _name) in [
          (*ambient, "ambient"),
          (*diffuse, "diffuse"),
          (*specular, "specular"),
        ] {
          if !(0.0..=1.0).contains(&value) {
            return Err(
              ValidationError::OutOfRange {
                value,
                min: 0.0,
                max: 1.0,
              }
              .into(),
            );
          }
        }
      }
      Command::Composite(cmd) => {
        for command in &cmd.commands {
          command.validate()?;
        }
      }
      _ => {} // 其他命令暂时不需要特殊验证
    }
    Ok(())
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

// ============================================================================
// 计算模块 - 函数式编程风格
// ============================================================================

/// 计算器模块
pub struct Calculator;

impl Calculator {
  /// 安全除法
  pub fn safe_divide(a: f64, b: f64) -> Result<f64> {
    if b == 0.0 {
      Err(ComputationError::DivisionByZero.into())
    } else {
      Ok(a / b)
    }
  }

  /// 安全开方
  pub fn safe_sqrt(x: f64) -> Result<f64> {
    if x < 0.0 {
      Err(ComputationError::InvalidOperation("负数不能开平方根".to_string()).into())
    } else {
      Ok(x.sqrt())
    }
  }

  /// 链式计算
  pub fn chain_operations<T, F>(initial: T, operations: Vec<F>) -> Result<T>
  where
    F: Fn(T) -> Result<T>,
  {
    operations.into_iter().try_fold(initial, |acc, op| op(acc))
  }
}

/// 函数式计算模块
pub struct FunctionalComputation;

impl FunctionalComputation {
  /// 高阶函数：应用函数到向量的每个元素
  pub fn map<T, U, F>(data: &[T], f: F) -> Vec<U>
  where
    F: Fn(&T) -> U,
  {
    data.iter().map(f).collect()
  }

  /// 过滤和映射组合
  pub fn filter_map<T, U, P, F>(data: &[T], predicate: P, mapper: F) -> Vec<U>
  where
    P: Fn(&T) -> bool,
    F: Fn(&T) -> U,
  {
    data
      .iter()
      .filter(|item| predicate(item))
      .map(mapper)
      .collect()
  }

  /// 折叠操作
  pub fn fold<T, U, F>(data: &[T], initial: U, f: F) -> U
  where
    F: Fn(U, &T) -> U,
  {
    data.iter().fold(initial, f)
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
}

// ============================================================================
// 数据处理模块 - 泛型数据操作
// ============================================================================

/// 泛型数据处理器
pub struct DataProcessor;

impl DataProcessor {
  /// 查找最大值
  pub fn find_max<T: Ord + Copy>(slice: &[T]) -> Option<T> {
    slice.iter().max().copied()
  }

  /// 查找最小值
  pub fn find_min<T: Ord + Copy>(slice: &[T]) -> Option<T> {
    slice.iter().min().copied()
  }

  /// 计算平均值
  pub fn average<T>(slice: &[T]) -> Option<f64>
  where
    T: Copy + Into<f64>,
  {
    if slice.is_empty() {
      None
    } else {
      let sum: f64 = slice.iter().map(|&x| x.into()).sum();
      Some(sum / slice.len() as f64)
    }
  }

  /// 数据分页
  pub fn paginate<T>(data: &[T], page: usize, page_size: usize) -> Result<&[T]> {
    if page_size == 0 {
      return Err(
        ValidationError::InvalidInput {
          field: "page_size".to_string(),
          reason: "页面大小不能为0".to_string(),
        }
        .into(),
      );
    }

    let start = page * page_size;
    if start >= data.len() {
      Ok(&[])
    } else {
      let end = std::cmp::min(start + page_size, data.len());
      Ok(&data[start..end])
    }
  }

  /// 数据去重
  pub fn deduplicate<T: Clone + PartialEq>(data: &[T]) -> Vec<T> {
    let mut result = Vec::new();
    for item in data {
      if !result.contains(item) {
        result.push(item.clone());
      }
    }
    result
  }
}

// ============================================================================
// 状态管理系统 - 观察者模式
// ============================================================================

/// 应用程序状态
#[derive(Debug, Clone)]
pub struct AppState {
  pub current_scene: String,
  pub object_count: usize,
  pub performance_mode: bool,
  pub last_command: Option<String>,
  pub error_count: usize,
}

impl Default for AppState {
  fn default() -> Self {
    Self {
      current_scene: "默认场景".to_string(),
      object_count: 0,
      performance_mode: false,
      last_command: None,
      error_count: 0,
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
// 应用程序配置
// ============================================================================

/// 应用程序配置
#[derive(Debug, Clone)]
pub struct AppConfig {
  pub enable_logging: bool,
  pub max_objects: usize,
  pub performance_mode: bool,
  pub debug_mode: bool,
  pub auto_save_interval: Option<u64>, // 秒
}

impl Default for AppConfig {
  fn default() -> Self {
    Self {
      enable_logging: true,
      max_objects: 1000,
      performance_mode: false,
      debug_mode: false,
      auto_save_interval: Some(300), // 5分钟
    }
  }
}

// ============================================================================
// 主应用程序结构
// ============================================================================

/// 主应用程序
pub struct Application {
  #[allow(dead_code)]
  config: AppConfig,
  state_manager: StateManager,
  command_history: Vec<Command>,
  #[allow(dead_code)]
  resource_pool: HashMap<String, Rc<dyn std::any::Any>>,
}

impl Application {
  /// 创建新的应用程序实例
  pub fn new(config: AppConfig) -> Self {
    let mut state_manager = StateManager::new();

    // 订阅状态变化
    if config.enable_logging {
      state_manager.subscribe(|state| {
        println!(
          "[状态变化] 场景: {}, 对象数: {}, 错误数: {}",
          state.current_scene, state.object_count, state.error_count
        );
      });
    }

    Self {
      config,
      state_manager,
      command_history: Vec::new(),
      resource_pool: HashMap::new(),
    }
  }

  /// 执行命令
  pub fn execute_command(&mut self, command: Command) -> Result<String> {
    let result = command.execute();

    // 更新状态
    self.state_manager.update_state(|state| match &result {
      Ok(msg) => {
        state.last_command = Some(msg.clone());
      }
      Err(_) => {
        state.error_count += 1;
      }
    });

    // 记录命令历史
    self.command_history.push(command);

    // 限制历史记录大小
    if self.command_history.len() > 100 {
      self.command_history.remove(0);
    }

    result
  }

  /// 获取命令历史
  pub fn get_command_history(&self) -> &[Command] {
    &self.command_history
  }

  /// 获取应用状态
  pub fn get_state(&self) -> &AppState {
    self.state_manager.get_state()
  }

  /// 运行演示
  pub fn run_demo(&mut self) -> Result<()> {
    println!("🚀 启动企业级Rust应用演示\n");

    // 演示命令执行
    let demo_commands = vec![
      Command::Transform(TransformCommand::Move {
        delta: Point3D::new(1.0, 2.0, 3.0),
        speed: 5.0,
      }),
      Command::Transform(TransformCommand::Rotate {
        axis: Axis::Y,
        angle: 45.0,
        pivot: Some(Point3D::new(0.0, 0.0, 0.0)),
      }),
      Command::Render(RenderCommand::SetMaterial(MaterialProperties::Metal {
        reflectivity: 0.8,
        roughness: 0.2,
      })),
      Command::System(SystemCommand::SaveState("demo_state".to_string())),
    ];

    for (i, command) in demo_commands.into_iter().enumerate() {
      println!("📋 执行命令 {}: {}", i + 1, command.description());
      match self.execute_command(command) {
        Ok(result) => println!("✅ 成功: {}", result),
        Err(e) => println!("❌ 失败: {}", e),
      }
      println!();
    }

    // 演示数据处理
    self.demo_data_processing()?;

    // 演示计算功能
    self.demo_calculations()?;

    println!("🎉 演示完成！");
    Ok(())
  }

  /// 演示数据处理功能
  fn demo_data_processing(&self) -> Result<()> {
    println!("📊 数据处理演示:");

    let numbers = vec![1, 5, 3, 9, 2, 8, 4, 7, 6];

    // 查找最大值和最小值
    if let (Some(max), Some(min)) = (
      DataProcessor::find_max(&numbers),
      DataProcessor::find_min(&numbers),
    ) {
      println!("   最大值: {}, 最小值: {}", max, min);
    }

    // 计算平均值
    if let Some(avg) = DataProcessor::average(&numbers) {
      println!("   平均值: {:.2}", avg);
    }

    // 过滤和映射
    let even_squares: Vec<i32> =
      FunctionalComputation::filter_map(&numbers, |&x| x % 2 == 0, |&x| x * x);
    println!("   偶数的平方: {:?}", even_squares);

    // 分组
    let grouped = FunctionalComputation::group_by(&numbers, |&x| x % 3);
    println!("   按余数分组: {:?}", grouped);

    println!();
    Ok(())
  }

  /// 演示计算功能
  fn demo_calculations(&self) -> Result<()> {
    println!("🧮 计算功能演示:");

    // 安全除法
    match Calculator::safe_divide(10.0, 3.0) {
      Ok(result) => println!("   10 ÷ 3 = {:.2}", result),
      Err(e) => println!("   除法错误: {}", e),
    }

    // 向量操作
    let v1 = Point3D::new(1.0, 2.0, 3.0);
    let v2 = Point3D::new(4.0, 5.0, 6.0);
    let dot_product = v1.dot_product(&v2);
    println!("   向量点积: {}", dot_product);

    // 向量归一化
    match v1.normalize() {
      Ok(normalized) => println!(
        "   归一化向量: ({:.2}, {:.2}, {:.2})",
        normalized.x, normalized.y, normalized.z
      ),
      Err(e) => println!("   归一化错误: {}", e),
    }

    // 链式计算
    let chain_result = Calculator::chain_operations(
      5.0,
      vec![|x| Ok(x * 2.0), |x| Calculator::safe_sqrt(x), |x| {
        Ok(x + 1.0)
      }],
    );

    match chain_result {
      Ok(result) => println!("   链式计算结果: {:.2}", result),
      Err(e) => println!("   链式计算错误: {}", e),
    }

    println!();
    Ok(())
  }
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
    let p1 = Point3D::new(3.0, 4.0, 0.0);
    assert_eq!(p1.magnitude(), 5.0);

    let p2 = Point3D::new(1.0, 0.0, 0.0);
    let normalized = p2.normalize().unwrap();
    assert_eq!(normalized.magnitude(), 1.0);
  }

  #[test]
  fn test_command_validation() {
    let invalid_cmd = Command::Transform(TransformCommand::Move {
      delta: Point3D::new(1.0, 2.0, 3.0),
      speed: -1.0,
    });
    assert!(invalid_cmd.validate().is_err());

    let valid_cmd = Command::Transform(TransformCommand::Move {
      delta: Point3D::new(1.0, 2.0, 3.0),
      speed: 5.0,
    });
    assert!(valid_cmd.validate().is_ok());
  }

  #[test]
  fn test_data_processor() {
    let data = vec![1, 5, 3, 9, 2];
    assert_eq!(DataProcessor::find_max(&data), Some(9));
    assert_eq!(DataProcessor::find_min(&data), Some(1));

    let avg = DataProcessor::average(&data).unwrap();
    assert!((avg - 4.0).abs() < 0.001);
  }

  #[test]
  fn test_color_validation() {
    assert!(Color::new(0.5, 0.5, 0.5, 1.0).is_ok());
    assert!(Color::new(1.5, 0.5, 0.5, 1.0).is_err());
    assert!(Color::new(-0.1, 0.5, 0.5, 1.0).is_err());
  }

  #[test]
  fn test_functional_operations() {
    let numbers = vec![1, 2, 3, 4, 5];

    let squares: Vec<i32> = FunctionalComputation::map(&numbers, |&x| x * x);
    assert_eq!(squares, vec![1, 4, 9, 16, 25]);

    let even_squares: Vec<i32> =
      FunctionalComputation::filter_map(&numbers, |&x| x % 2 == 0, |&x| x * x);
    assert_eq!(even_squares, vec![4, 16]);
  }
}

// ============================================================================
// 主函数
// ============================================================================

fn main() -> Result<()> {
  // 创建应用配置
  let config = AppConfig {
    enable_logging: true,
    debug_mode: true,
    ..Default::default()
  };

  // 创建并运行应用
  let mut app = Application::new(config);
  app.run_demo()?;

  Ok(())
}
