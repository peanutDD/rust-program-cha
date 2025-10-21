
# Rust高级特性详解：结构体、枚举与模式匹配

本文基于 `optimized_main.rs`文件中的代码，详细解析Rust语言中的高级结构体、枚举和模式匹配特性。

## 1. 高级结构体定义

### Point3D结构体

```rust
#[derive(Debug, Clone)]
struct Point3D {
  x: f64,
  y: f64,
  z: f64,
}
```

`Point3D`是一个三维坐标点结构体，包含三个 `f64`类型的字段：`x`、`y`和 `z`。通过派生 `Debug`和 `Clone`trait，实现了调试打印和克隆功能。

### Sphere结构体

```rust
#[derive(Debug)]
struct Sphere {
  center: Point3D,
  radius: f64,
  material: Material,
}
```

`Sphere`结构体展示了结构体的嵌套使用：

- 包含一个 `Point3D`类型的 `center`字段表示球心
- 包含一个 `f64`类型的 `radius`字段表示半径
- 包含一个 `Material`枚举类型的 `material`字段表示材质

## 2. 复杂枚举定义

### Material枚举

```rust
#[derive(Debug)]
enum Material {
  Metal {
    reflectivity: f64,
  },
  #[allow(dead_code)]
  Glass {
    transparency: f64,
  },
  #[allow(dead_code)]
  Plastic {
    color: (u8, u8, u8),
  },
}
```

`Material`枚举展示了Rust中带有关联数据的枚举变体：

- `Metal`变体包含一个 `reflectivity`关联字段
- `Glass`变体包含一个 `transparency`关联字段
- `Plastic`变体包含一个RGB颜色元组 `color`
- 使用 `#[allow(dead_code)]`属性来忽略未使用变体的警告

### Command和Axis枚举

```rust
#[derive(Debug, Clone)]
enum Command {
  Move { x: i32, y: i32, speed: f32 },
  Rotate { angle: f32, axis: Axis },
  Scale { factor: f32 },
  Batch(Vec<Command>),
}

#[derive(Debug, Clone)]
enum Axis {
  #[allow(dead_code)]
  X,
  #[allow(dead_code)]
  Y,
  Z,
}
```

这两个枚举展示了：

- `Command`枚举包含四种操作类型，其中 `Batch`变体展示了递归枚举定义（包含 `Vec<Command>`）
- `Axis`枚举是一个简单的无数据枚举，表示三个坐标轴
- 两个枚举都派生了 `Debug`和 `Clone`trait

## 3. 高级结构体解构

```rust
fn struct_destructuring_advanced() -> Result<()> {
  println!("=== 高级结构体解构 ===");

  let sphere = Sphere {
    center: Point3D { x: 1.0, y: 2.0, z: 3.0 },
    radius: 5.0,
    material: Material::Metal { reflectivity: 0.8 },
  };

  // 深度嵌套解构
  match sphere {
    Sphere {
      center: Point3D { x, y, z },
      radius,
      material: Material::Metal { reflectivity },
    } => {
      println!("金属球体: 中心({}, {}, {}), 半径: {}, 反射率: {}",
               x, y, z, radius, reflectivity);
    },
    // 其他匹配分支...
  }
  
  Ok(())
}
```

这段代码展示了Rust中强大的嵌套解构能力：

- 在 `match`表达式中同时解构 `Sphere`、`Point3D`和 `Material`枚举
- 直接访问嵌套结构体的字段，无需通过点号逐级访问
- 使用字段缩写语法（如 `radius`而不是 `radius: radius`）
- 使用 `..`省略不需要的字段

## 4. 枚举与模式匹配增强

### 枚举的方法实现

```rust
impl Command {
  fn execute(&self) -> Result<()> {
    match self {
      Command::Move { x, y, speed } => {
        if *speed <= 0.0 {
          return Err(AppError::InvalidInput("速度必须大于0".to_string()));
        }
        println!("移动到 ({}, {}), 速度: {}", x, y, speed);
      },
      Command::Batch(commands) => {
        println!("执行批量命令:");
        for cmd in commands {
          cmd.execute()?;
        }
      },
      // 其他命令处理...
    }
    Ok(())
  }
}
```

这段代码展示了：

- 为枚举实现方法（类似于为结构体实现方法）
- 在枚举方法中进行模式匹配
- 条件检查和错误处理
- 使用 `?`操作符进行错误传播
- 递归调用（在 `Batch`命令中调用其他命令的 `execute`方法）

### 枚举的批量处理

```rust
fn enum_pattern_matching_enhanced() -> Result<()> {
  println!("=== 增强的枚举与模式匹配 ===");

  let commands = vec![
    Command::Move { x: 10, y: 20, speed: 5.0 },
    Command::Rotate { angle: 90.0, axis: Axis::Z },
    Command::Scale { factor: 2.0 },
    Command::Batch(vec![
      Command::Move { x: 0, y: 0, speed: 1.0 },
      Command::Scale { factor: 0.5 },
    ]),
  ];

  for cmd in &commands {
    if let Err(e) = cmd.execute() {
      eprintln!("命令执行失败: {}", e);
    }
  }
  
  Ok(())
}
```

这段代码展示了：

- 创建复杂的枚举实例集合
- 使用 `if let`表达式处理可能的错误
- 批量执行命令并处理错误

## 5. 高级解构模式

```rust
fn advanced_destructuring_patterns() -> Result<()> {
  println!("=== 高级解构模式 ===");

  // 范围模式
  let numbers = vec![1, 5, 10, 15, 25, 50, 100];
  for &num in &numbers {
    match num {
      1..=10 => println!("{} 是小数", num),
      11..=50 => println!("{} 是中数", num),
      51..=100 => println!("{} 是大数", num),
      _ => println!("{} 超出范围", num),
    }
  }

  // 守卫模式与@绑定
  let point = Point3D { x: 3.0, y: 4.0, z: 0.0 };
  match point {
    Point3D { x, y, z: 0.0 } if x.powi(2) + y.powi(2) == 25.0 => {
      println!("在XY平面上的单位圆上的点: ({}, {})", x, y);
    },
    Point3D { z, .. } if z > 0.0 => {
      println!("在Z轴正方向的点");
    },
    p @ Point3D { x, y, z } if x == y && y == z => {
      println!("对角线上的点: {:?}", p);
    },
    _ => println!("普通点"),
  }
  
  Ok(())
}
```

这段代码展示了Rust中几种高级模式匹配技术：

- **范围模式**：使用 `1..=10`这样的语法匹配范围
- **守卫模式**：使用 `if`条件进一步过滤匹配
- **@绑定**：使用 `p @`语法同时绑定整个值和其字段
- **字段模式**：针对特定字段值进行匹配

## 6. 泛型与实际应用示例

### ApiResponse泛型结构体

```rust
#[derive(Debug)]
struct ApiResponse<T> {
  data: Option<T>,
  status: u16,
  message: String,
}

#[derive(Debug)]
struct User {
  #[allow(dead_code)]
  id: u64,
  name: String,
  email: String,
}
```

`ApiResponse<T>`是一个泛型结构体，展示了Rust中的泛型使用：

- 接受类型参数 `T`
- 包含一个 `Option<T>`类型的 `data`字段
- 结合 `User`结构体展示了泛型的实际应用

### 实用的API响应处理

```rust
fn practical_examples_optimized() -> Result<()> {
  println!("=== 优化的实际应用示例 ===");

  // 使用泛型的API响应处理
  let responses = vec![
    ApiResponse {
      data: Some(User { id: 1, name: "张三".to_string(), email: "zhangsan@example.com".to_string() }),
      status: 200,
      message: "成功".to_string(),
    },
    ApiResponse::<User> {
      data: None,
      status: 404,
      message: "用户未找到".to_string(),
    },
  ];

  for response in responses {
    match response {
      ApiResponse { data: Some(user), status: 200, .. } => {
        println!("✅ 用户加载成功: {} ({})", user.name, user.email);
      },
      ApiResponse { status: 404, message, .. } => {
        println!("❌ 资源未找到: {}", message);
      },
      ApiResponse { status, message, .. } => {
        println!("⚠️ 其他状态 {}: {}", status, message);
      }
    }
  }
  
  // 其他示例...
  
  Ok(())
}
```

这段代码展示了：

- 创建不同状态的API响应
- 使用显式类型注解 `ApiResponse::<User>`
- 使用复杂的模式匹配处理不同的API响应情况
- 结合 `Option<T>`处理可能存在的数据

## 7. 高效的错误处理和集合操作

```rust
// 高效的Option和Result链式操作
let numbers = vec!["42", "not_a_number", "100"];
let parsed_numbers: Vec<i32> = numbers.iter().filter_map(|s| s.parse().ok()).collect();
println!("解析的数字: {:?}", parsed_numbers);

// 使用?操作符的错误传播
fn safe_divide(a: f64, b: f64) -> Result<f64> {
  if b == 0.0 {
    Err(AppError::DivisionByZero)
  } else {
    Ok(a / b)
  }
}

match safe_divide(10.0, 3.0) {
  Ok(result) => println!("除法结果: {:.2}", result),
  Err(e) => println!("错误: {}", e),
}
```

这段代码展示了Rust中的几个强大特性：

- **函数式编程风格**：使用 `filter_map`和 `collect`进行集合操作
- **`ok()`方法**：将 `Result<T, E>`转换为 `Option<T>`
- **错误处理**：自定义错误类型和安全的除法实现
- **`?`操作符**：简化错误传播

## 总结

Rust的结构体、枚举和模式匹配提供了强大而灵活的类型系统功能：

1. **结构体**：支持嵌套定义、方法实现和多种派生trait
2. **枚举**：可以包含关联数据，支持递归定义，也可以实现方法
3. **模式匹配**：提供了深度嵌套解构、范围匹配、守卫条件和@绑定等高级功能
4. **泛型**：允许编写通用且类型安全的代码
5. **错误处理**：通过 `Result`和 `Option`类型提供了优雅的错误处理机制

这些特性共同构成了Rust强大的类型系统，使得代码更加安全、高效和可维护。
