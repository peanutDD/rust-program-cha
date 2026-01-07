# Rust Result 类型全面学习指南

> 基于 https://course.rs/basic/result-error/result.html 的深度分析

## 📚 项目概述

本项目全面深入地分析 Rust `Result<T, E>` 类型，从基础概念到高级应用，帮助开发者掌握 Rust 错误处理的核心机制。

## 🎯 学习目标

通过本教程，你将学会：

- ✅ 理解 Result 类型的设计哲学
- ✅ 掌握 Result 的基本用法
- ✅ 学会使用 `?` 操作符传播错误
- ✅ 掌握 Result 的组合器方法
- ✅ 理解错误转换和链式处理
- ✅ 学会在实际项目中应用 Result

## 📖 核心知识点

### 1. Result 基础

**类型定义：**
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

**基本用法：**
```rust
let result: Result<i32, &str> = Ok(42);
let error: Result<i32, &str> = Err("Something went wrong");
```

### 2. 模式匹配

**match 表达式：**
```rust
match result {
    Ok(value) => println!("Success: {}", value),
    Err(error) => println!("Error: {}", error),
}
```

**if let 语法：**
```rust
if let Ok(value) = result {
    println!("Value: {}", value);
}
```

### 3. ? 操作符

**错误传播：**
```rust
fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

**等价写法：**
```rust
fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    // ...
}
```

### 4. 组合器方法

**map 和 map_err：**
```rust
let result = Ok(5);
let doubled = result.map(|x| x * 2); // Ok(10)

let error = Err("error");
let mapped = error.map_err(|e| format!("Error: {}", e));
```

**and_then：**
```rust
fn parse_and_validate(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|e| format!("Parse error: {}", e))
        .and_then(|n| {
            if n > 0 {
                Ok(n)
            } else {
                Err("Number must be positive".to_string())
            }
        })
}
```

**unwrap 系列：**
```rust
let value = result.unwrap();              // panic on error
let value = result.expect("message");     // panic with message
let value = result.unwrap_or(0);          // default value
let value = result.unwrap_or_else(|| 0);  // compute default
```

### 5. 错误转换

**From trait：**
```rust
impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> Self {
        MyError::ParseError(err.to_string())
    }
}
```

**使用：**
```rust
let result: Result<i32, MyError> = "42".parse()?;
```

### 6. 链式处理

**复杂错误处理：**
```rust
fn process_data(input: &str) -> Result<i32, Box<dyn Error>> {
    let file = File::open(input)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let number: i32 = line.trim().parse()?;
    Ok(number * 2)
}
```

## 🚀 快速开始

```bash
# 运行完整教程
cargo run

# 运行测试
cargo test
```

## 📖 学习路径

### 1. 基础阶段
- 理解 Result 类型
- 掌握基本用法
- 学习模式匹配

### 2. 进阶阶段
- 掌握 `?` 操作符
- 学习组合器方法
- 理解错误转换

### 3. 高级阶段
- 构建错误处理系统
- 优化错误处理流程
- 在实际项目中应用

## 💡 最佳实践

1. **优先使用 Result**：处理可恢复错误
2. **使用 ? 操作符**：简化错误传播
3. **提供有意义的错误**：错误信息应该清晰
4. **避免过度使用 unwrap**：只在确定不会失败时使用
5. **构建错误类型层次**：使用自定义错误类型

## 🔍 常见陷阱

1. **过度使用 unwrap**：可能导致程序 panic
2. **忽略错误**：使用 `_` 忽略错误是不好的实践
3. **错误信息不足**：提供清晰的错误上下文
4. **错误类型过于宽泛**：使用具体的错误类型

## 📚 相关资源

- [Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Result Documentation](https://doc.rust-lang.org/std/result/)
- [Error Handling Guide](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

## 🎯 总结

Result 类型是 Rust 错误处理的核心，通过类型系统在编译时保证错误的正确处理，这是 Rust 内存安全保证的重要组成部分。

---

**Happy Error Handling! 🦀**

