# Rust panic! 和 Result 对比学习指南

> 基于 https://course.rs/basic/result-error/panic.html 的深度分析

## 📚 项目概述

本项目全面深入地分析 Rust 中 `panic!` 和 `Result` 的区别和使用场景，帮助开发者理解何时使用 panic，何时使用 Result。

## 🎯 学习目标

通过本教程，你将学会：

- ✅ 理解 panic! 和 Result 的本质区别
- ✅ 掌握何时使用 panic!
- ✅ 掌握何时使用 Result
- ✅ 理解不可恢复错误 vs 可恢复错误
- ✅ 学会在实际项目中做出正确选择

## 📖 核心知识点

### 1. panic! 机制

**触发场景：**
- 显式调用 `panic!()`
- 数组越界访问
- 整数溢出（debug 模式）
- `unwrap()` 调用失败

**基本用法：**
```rust
if value < 0 {
    panic!("Value must be non-negative, got {}", value);
}
```

### 2. Result 类型

**使用场景：**
- 可恢复的错误
- 需要调用者处理的错误
- 网络请求、文件 I/O 等

**基本用法：**
```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
```

### 3. 错误分类

**不可恢复错误（使用 panic!）：**
- 程序逻辑错误
- 违反不变量
- 无法恢复的状态

**可恢复错误（使用 Result）：**
- 用户输入错误
- 网络请求失败
- 文件不存在

### 4. 选择指南

**使用 panic! 的情况：**
```rust
// 1. 测试代码
#[test]
fn test_function() {
    assert_eq!(2 + 2, 4);
}

// 2. 程序逻辑错误
fn get_element(arr: &[i32], index: usize) -> i32 {
    if index >= arr.len() {
        panic!("Index out of bounds");
    }
    arr[index]
}

// 3. 不变量违反
fn set_age(age: u32) {
    if age > 150 {
        panic!("Invalid age: {}", age);
    }
}
```

**使用 Result 的情况：**
```rust
// 1. 用户输入验证
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse()
}

// 2. 文件操作
fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

// 3. 网络请求
fn fetch_data(url: &str) -> Result<String, NetworkError> {
    // 实现
}
```

### 5. 错误处理策略

**防御性编程：**
```rust
// 使用 Option 处理可能为空的值
fn get_first<T>(vec: &[T]) -> Option<&T> {
    vec.first()
}

// 使用 Result 处理可能失败的操作
fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
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
- 理解 panic! 和 Result 的区别
- 掌握基本用法
- 学习错误分类

### 2. 进阶阶段
- 掌握选择策略
- 学习错误处理模式
- 理解防御性编程

### 3. 高级阶段
- 构建错误处理系统
- 优化错误处理流程
- 在实际项目中应用

## 💡 最佳实践

1. **默认使用 Result**：除非确定是程序错误
2. **panic! 用于测试**：测试代码中可以使用 panic!
3. **提供清晰错误信息**：错误信息应该帮助调试
4. **避免在生产代码中使用 unwrap**：除非确定不会失败
5. **构建错误类型层次**：使用自定义错误类型

## 🔍 常见陷阱

1. **过度使用 panic!**：应该使用 Result 处理可恢复错误
2. **忽略错误**：使用 `_` 忽略错误是不好的实践
3. **错误信息不足**：提供清晰的错误上下文
4. **混淆错误类型**：不理解何时使用 panic! vs Result

## 📚 相关资源

- [Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Unrecoverable Errors with panic!](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html)
- [Recoverable Errors with Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

## 🎯 总结

理解 panic! 和 Result 的区别是 Rust 错误处理的基础。正确选择错误处理方式可以编写出更健壮、更可靠的程序。

---

**Happy Error Handling! 🦀**

