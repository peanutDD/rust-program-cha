# Rust 错误处理深度教程

🦀 **全面掌握 Rust 错误处理机制的完整指南**

## 📚 教程概述

本教程深入分析 Rust 错误处理的核心概念和最佳实践，涵盖从基础概念到高级技巧的完整知识体系。通过丰富的实际案例和代码演示，帮助开发者全面理解 Rust 独特的错误处理模型。

## 🎯 学习目标

- 深入理解 Rust 错误处理的设计哲学
- 掌握 `panic!` 机制和不可恢复错误的处理
- 熟练使用 `Result<T, E>` 类型处理可恢复错误
- 理解 `Option<T>` 类型在空值处理中的应用
- 掌握 `?` 操作符和错误传播机制
- 学会设计和实现自定义错误类型
- 掌握各种错误处理模式和组合器方法
- 了解高级错误处理技巧和最佳实践

## 🚀 快速开始

```bash
# 克隆项目
git clone <repository-url>
cd Error-Handling

# 运行教程
cargo run

# 运行测试
cargo test
```

## 📖 教程内容

### 1. 错误处理基础和设计哲学

**核心概念：**
- Rust 错误分类：不可恢复错误 vs 可恢复错误
- 显式错误处理原则
- 零成本抽象和类型安全
- 错误的组合性和可转换性

**设计哲学：**
```rust
// Rust 强制显式处理错误
let result = risky_operation();
match result {
    Ok(value) => println!("成功: {}", value),
    Err(error) => println!("错误: {}", error),
}
```

### 2. panic! 机制和不可恢复错误

**触发场景：**
- 显式调用 `panic!()`
- 数组越界访问
- 整数溢出（debug 模式）
- `unwrap()` 调用失败

**panic! 处理：**
```rust
// 捕获 panic!
let result = std::panic::catch_unwind(|| {
    panic!("这是一个被捕获的 panic!");
});

// 自定义 panic hook
std::panic::set_hook(Box::new(|panic_info| {
    println!("自定义处理: {:?}", panic_info);
}));
```

### 3. Result<T, E> 类型和可恢复错误

**基本用法：**
```rust
// 创建 Result
let success: Result<i32, &str> = Ok(42);
let failure: Result<i32, &str> = Err("错误信息");

// 模式匹配处理
match success {
    Ok(value) => println!("值: {}", value),
    Err(error) => println!("错误: {}", error),
}

// 使用 if let 简化
if let Ok(value) = success {
    println!("值: {}", value);
}
```

**实际应用：**
- 文件 I/O 操作
- 网络请求
- 数据解析
- 数据库操作

### 4. Option<T> 类型和空值处理

**核心方法：**
```rust
let value: Option<i32> = Some(42);
let empty: Option<i32> = None;

// 常用方法
value.is_some();           // true
value.is_none();           // false
value.unwrap_or(0);        // 42
empty.unwrap_or_else(|| 100); // 100
```

**应用场景：**
- HashMap 查找
- 字符串搜索
- 数组索引
- 配置项获取

### 5. 错误传播和 ? 操作符

**传统方式 vs ? 操作符：**
```rust
// 传统方式
fn read_file_traditional(filename: &str) -> Result<String, io::Error> {
    let file_result = File::open(filename);
    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    // ... 更多代码
}

// 使用 ? 操作符
fn read_file_modern(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

**链式调用：**
```rust
fn process_data(input: &str) -> Result<i32, Box<dyn Error>> {
    let contents = std::fs::read_to_string(input)?;
    let number = contents.trim().parse::<i32>()?;
    Ok(number * 2)
}
```

### 6. 自定义错误类型

**完整实现：**
```rust
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    InvalidInput(String),
}

// 实现 Display trait
impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "除零错误"),
            MathError::NegativeSquareRoot => write!(f, "负数开平方根错误"),
            MathError::InvalidInput(msg) => write!(f, "无效输入: {}", msg),
        }
    }
}

// 实现 Error trait
impl Error for MathError {}

// 实现错误转换
impl From<ParseIntError> for MathError {
    fn from(error: ParseIntError) -> Self {
        MathError::InvalidInput(format!("解析错误: {}", error))
    }
}
```

### 7. 错误处理模式和最佳实践

**核心方法对比：**

| 方法 | 用途 | 失败时行为 |
|------|------|------------|
| `unwrap()` | 确信不会失败 | panic! |
| `expect()` | 提供错误信息 | panic! with message |
| `unwrap_or()` | 提供默认值 | 返回默认值 |
| `unwrap_or_else()` | 计算默认值 | 执行闭包 |
| `map()` | 转换成功值 | 保持错误不变 |
| `map_err()` | 转换错误值 | 保持成功值不变 |
| `and_then()` | 链式操作 | 短路返回错误 |

**组合器应用：**
```rust
let result = input
    .parse::<i32>()
    .map_err(|_| "解析失败")
    .and_then(|num| {
        if num > 0 {
            Ok(num)
        } else {
            Err("数字必须为正数")
        }
    })
    .map(|num| num * num);
```

### 8. 高级错误处理技巧

**错误上下文和错误链：**
```rust
fn level1() -> Result<i32, Box<dyn Error>> {
    level2().map_err(|e| format!("level1 中的错误: {}", e).into())
}

// 遍历错误链
let mut source = error.source();
while let Some(err) = source {
    println!("原因: {}", err);
    source = err.source();
}
```

**重试机制：**
```rust
fn retry_operation<F, T, E>(mut operation: F, max_attempts: u32) -> Result<T, E>
where
    F: FnMut(u32) -> Result<T, E>,
{
    for attempt in 1..=max_attempts {
        match operation(attempt) {
            Ok(result) => return Ok(result),
            Err(error) => {
                if attempt == max_attempts {
                    return Err(error);
                }
                // 等待后重试
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
    unreachable!()
}
```

**多线程错误处理：**
```rust
let handles: Vec<_> = (0..3)
    .map(|i| {
        thread::spawn(move || {
            // 可能失败的操作
            if i == 1 {
                Err(format!("线程 {} 发生错误", i))
            } else {
                Ok(format!("线程 {} 成功", i))
            }
        })
    })
    .collect();

for (i, handle) in handles.into_iter().enumerate() {
    match handle.join() {
        Ok(result) => match result {
            Ok(msg) => println!("线程 {}: {}", i, msg),
            Err(error) => println!("线程 {} 错误: {}", i, error),
        },
        Err(_) => println!("线程 {} panic!", i),
    }
}
```

## 🧪 测试用例

项目包含完整的测试套件，覆盖所有核心功能：

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_safe_divide
cargo test test_error_conversion
```

**测试示例：**
```rust
#[test]
fn test_safe_divide() {
    assert_eq!(safe_divide(10.0, 2.0).unwrap(), 5.0);
    assert!(safe_divide(10.0, 0.0).is_err());
}

#[test]
fn test_error_conversion() {
    let parse_error: ParseIntError = "abc".parse::<i32>().unwrap_err();
    let math_error: MathError = parse_error.into();
    assert!(matches!(math_error, MathError::InvalidInput(_)));
}

#[test]
#[should_panic(expected = "测试 panic!")]
fn test_panic() {
    panic!("测试 panic!");
}
```

## 💡 最佳实践

### 1. 错误类型选择
- **使用 `Result`** 处理可恢复的错误
- **使用 `Option`** 处理可能为空的值
- **使用 `panic!`** 处理程序逻辑错误

### 2. 错误信息设计
- 提供清晰、具体的错误描述
- 包含足够的上下文信息
- 考虑错误的可操作性

### 3. 错误传播策略
- 优先使用 `?` 操作符
- 在边界处进行错误转换
- 构建有意义的错误链

### 4. 性能考虑
- `Result` 和 `Option` 是零成本抽象
- 避免不必要的 `unwrap()` 调用
- 合理使用错误缓存

## 🔍 常见陷阱

### 1. 过度使用 `unwrap()`
```rust
// ❌ 危险：可能导致 panic!
let value = risky_operation().unwrap();

// ✅ 安全：显式处理错误
match risky_operation() {
    Ok(value) => process(value),
    Err(error) => handle_error(error),
}
```

### 2. 忽略错误信息
```rust
// ❌ 丢失错误信息
let _ = risky_operation();

// ✅ 适当处理错误
if let Err(error) = risky_operation() {
    log::error!("操作失败: {}", error);
}
```

### 3. 错误类型过于宽泛
```rust
// ❌ 信息不足
fn process() -> Result<String, Box<dyn Error>> { ... }

// ✅ 具体的错误类型
fn process() -> Result<String, ProcessError> { ... }
```

## 📚 扩展学习

### 推荐库
- **`anyhow`**: 简化错误处理的应用程序库
- **`thiserror`**: 简化自定义错误类型的库
- **`eyre`**: 提供更好错误报告的库
- **`color-eyre`**: 彩色错误报告

### 相关资源
- [Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust by Example - Error Handling](https://doc.rust-lang.org/rust-by-example/error.html)
- [The Rust Programming Language - Recoverable Errors](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

## 🎯 总结

Rust 的错误处理机制通过类型系统在编译时保证错误的正确处理，这是其内存安全保证的重要组成部分。掌握这些概念和模式，能够帮助你编写更加健壮和可靠的 Rust 程序。

**核心要点：**
- ✅ 使用 `Result<T, E>` 处理可恢复错误
- ✅ 使用 `Option<T>` 处理可能为空的值
- ✅ 使用 `?` 操作符简化错误传播
- ✅ 创建自定义错误类型提供更好的错误信息
- ✅ 使用组合器方法优雅地处理错误
- ✅ 在适当的时候使用 `panic!` 处理不可恢复错误
- ✅ 构建错误上下文和错误链提供更好的调试信息

通过本教程的学习，你将能够：
- 🎯 深入理解 Rust 错误处理的设计理念
- 🛠️ 熟练运用各种错误处理技巧和模式
- 🏗️ 设计健壮的错误处理架构
- 🔧 调试和解决复杂的错误处理问题
- 📈 编写更加可靠和可维护的 Rust 代码

---

**Happy Coding with Rust! 🦀**