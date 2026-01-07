# Rust match 和 if let 全面学习指南

> 基于 https://course.rs/basic/match-pattern/match.html 的深度分析

## 📚 项目概述

本项目全面深入地分析 Rust `match` 和 `if let` 模式匹配，从基础概念到高级应用，帮助开发者掌握 Rust 强大的模式匹配系统。

## 🎯 学习目标

通过本教程，你将学会：

- ✅ 理解 match 表达式的语法和用法
- ✅ 掌握 if let 的简洁语法
- ✅ 理解模式匹配的各种模式
- ✅ 掌握守卫条件 (guards) 的使用
- ✅ 学会在函数、循环中使用模式匹配
- ✅ 理解模式匹配与所有权的交互

## 📖 核心知识点

### 1. match 表达式

**基本语法：**
```rust
match value {
    pattern1 => expression1,
    pattern2 => expression2,
    _ => default_expression,
}
```

**必须穷尽：**
```rust
match number {
    1 => println!("One"),
    2 => println!("Two"),
    _ => println!("Other"),  // 必须处理所有情况
}
```

### 2. if let 语法

**简洁的模式匹配：**
```rust
if let Some(value) = option {
    println!("Got value: {}", value);
}
```

**等价于：**
```rust
match option {
    Some(value) => println!("Got value: {}", value),
    _ => {},
}
```

### 3. while let

**循环中的模式匹配：**
```rust
let mut stack = Vec::new();
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

### 4. 模式类型

**基本模式：**
```rust
match x {
    1 | 2 => println!("One or two"),
    3..=5 => println!("Three to five"),
    _ => println!("Other"),
}
```

**解构模式：**
```rust
match point {
    (0, 0) => println!("Origin"),
    (0, y) => println!("On y-axis: {}", y),
    (x, 0) => println!("On x-axis: {}", x),
    (x, y) => println!("({}, {})", x, y),
}
```

**守卫条件：**
```rust
match number {
    n if n < 0 => println!("Negative"),
    n if n == 0 => println!("Zero"),
    n => println!("Positive: {}", n),
}
```

### 5. @ 绑定

**绑定值到变量：**
```rust
match value {
    n @ 1..=10 => println!("Small: {}", n),
    n @ 11..=100 => println!("Medium: {}", n),
    n => println!("Large: {}", n),
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
- 理解 match 表达式
- 掌握 if let 语法
- 学习基本模式

### 2. 进阶阶段
- 掌握解构模式
- 学习守卫条件
- 理解 @ 绑定

### 3. 高级阶段
- 构建复杂模式
- 优化模式匹配
- 在实际项目中应用

## 💡 最佳实践

1. **使用 match 处理所有情况**：确保穷尽所有模式
2. **使用 if let 简化单分支**：更简洁的语法
3. **使用守卫条件**：添加额外的匹配条件
4. **注意所有权**：模式匹配可能移动值
5. **性能考虑**：编译器会优化模式匹配

## 🔍 常见陷阱

1. **忘记处理所有情况**：match 必须穷尽
2. **所有权问题**：模式匹配可能移动值
3. **守卫条件错误**：守卫条件的逻辑错误
4. **模式顺序**：模式顺序影响匹配结果

## 📚 相关资源

- [Rust Book - Pattern Matching](https://doc.rust-lang.org/book/ch06-02-match.html)
- [Rust Book - if let](https://doc.rust-lang.org/book/ch06-03-if-let.html)
- [Pattern Syntax](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html)

## 🎯 总结

match 和 if let 是 Rust 模式匹配的核心工具，掌握它们可以编写出更清晰、更安全的代码。

---

**Happy Pattern Matching! 🦀**
