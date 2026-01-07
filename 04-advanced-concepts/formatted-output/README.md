# Rust 格式化输出全面指南

> 基于 https://course.rs/basic/formatted-output.html 的深度分析

## 📚 项目概述

本项目全面深入地分析 Rust 格式化输出系统，从基础的 `println!` 到高级的格式化技巧，帮助开发者掌握 Rust 强大的格式化能力。

## 🎯 学习目标

通过本教程，你将学会：

- ✅ 掌握 `println!`, `print!`, `format!` 等格式化宏
- ✅ 理解格式化字符串的语法和占位符
- ✅ 学会使用格式化说明符控制输出格式
- ✅ 掌握自定义格式化实现
- ✅ 了解高级格式化技巧和最佳实践
- ✅ 学会调试输出和性能优化

## 📖 核心知识点

### 1. 基础格式化宏

**常用宏：**
```rust
println!("Hello, {}!", "World");           // 输出到标准输出，换行
print!("Hello, {}!", "World");             // 输出到标准输出，不换行
eprintln!("Error: {}", error);             // 输出到标准错误
format!("Hello, {}!", "World");            // 格式化字符串
write!(writer, "{}", value);               // 写入到 writer
writeln!(writer, "{}", value);             // 写入到 writer，换行
```

### 2. 格式化占位符

**基本语法：**
```rust
println!("{}", value);                     // 默认格式化
println!("{:?}", value);                   // Debug 格式化
println!("{:#?}", value);                  // 美化 Debug 格式化
println!("{0} {1} {0}", "A", "B");         // 位置参数
println!("{name} is {age}", name="Alice", age=30); // 命名参数
```

### 3. 格式化说明符

**宽度和精度：**
```rust
println!("{:5}", 42);                      // 最小宽度 5
println!("{:05}", 42);                     // 填充 0，宽度 5
println!("{:.2}", 3.14159);                // 小数点后 2 位
println!("{:8.2}", 3.14159);               // 宽度 8，精度 2
```

**对齐方式：**
```rust
println!("{:<10}", "left");                // 左对齐
println!("{:>10}", "right");               // 右对齐
println!("{:^10}", "center");              // 居中对齐
println!("{:=^10}", "center");             // 居中，用 = 填充
```

**数字格式化：**
```rust
println!("{:b}", 42);                      // 二进制
println!("{:o}", 42);                      // 八进制
println!("{:x}", 42);                      // 小写十六进制
println!("{:X}", 42);                      // 大写十六进制
println!("{:+}", 42);                      // 显示符号
```

### 4. 自定义格式化

**实现 Display trait：**
```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

**实现 Debug trait：**
```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
```

### 5. 高级格式化

**格式化参数：**
```rust
println!("{value:>width$.precision$}", 
         value=3.14159, width=10, precision=2);
```

**格式化宏变体：**
```rust
println!();                                // 只换行
println!("{}", format_args!("{}", value)); // 延迟格式化
```

## 🚀 快速开始

```bash
# 运行完整教程
cargo run

# 查看特定示例
cargo run -- --help
```

## 📖 学习路径

### 1. 基础阶段
- 掌握基本格式化宏
- 理解占位符语法
- 学习常用格式化说明符

### 2. 进阶阶段
- 掌握高级格式化选项
- 实现自定义格式化
- 优化格式化性能

### 3. 高级阶段
- 构建格式化 DSL
- 实现复杂的格式化逻辑
- 性能优化和最佳实践

## 💡 最佳实践

1. **选择合适的宏**：根据场景选择 `println!`, `format!` 等
2. **使用结构化输出**：JSON, YAML 等结构化格式
3. **性能考虑**：避免不必要的字符串分配
4. **错误处理**：格式化操作可能失败，注意处理
5. **国际化**：考虑多语言支持

## 🔍 常见陷阱

1. **格式字符串注入**：用户输入不应该直接用于格式字符串
2. **性能问题**：频繁的格式化操作可能影响性能
3. **类型不匹配**：占位符类型必须匹配参数类型
4. **精度丢失**：浮点数格式化可能丢失精度

## 📚 相关资源

- [Rust Book - Formatting](https://doc.rust-lang.org/std/fmt/)
- [std::fmt Documentation](https://doc.rust-lang.org/std/fmt/)
- [Formatting Mini-Language](https://doc.rust-lang.org/std/fmt/index.html#syntax)

## 🎯 总结

格式化输出是程序与用户交互的重要方式。掌握 Rust 的格式化系统可以编写出清晰、美观的输出，提升用户体验。

---

**Happy Formatting! 🦀**

