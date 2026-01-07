# Rust Vector 动态数组全面学习指南

> 基于 https://course.rs/basic/compound-type/vector.html 的深度分析

## 📚 项目概述

本项目全面深入地分析 Rust `Vec<T>` 动态数组，从基础概念到高级应用，帮助开发者掌握 Rust 最常用的集合类型。

## 🎯 学习目标

通过本教程，你将学会：

- ✅ 理解 Vector 的内存布局和实现原理
- ✅ 掌握 Vector 的创建和初始化方法
- ✅ 熟练使用 Vector 的基本操作
- ✅ 理解 Vector 的容量管理
- ✅ 掌握 Vector 的迭代方法
- ✅ 学会在实际项目中应用 Vector

## 📖 核心知识点

### 1. Vector 基础

**类型定义：**
```rust
let mut v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];
```

**内存布局：**
- 堆上分配
- 三个字段：指针、长度、容量
- 自动扩容

### 2. 创建和初始化

**多种创建方式：**
```rust
let v1 = Vec::new();                    // 空 Vector
let v2 = vec![1, 2, 3];                 // 字面量
let v3 = vec![0; 10];                   // 重复值
let v4 = (0..10).collect::<Vec<i32>>();  // 从迭代器
```

### 3. 基本操作

**添加元素：**
```rust
let mut v = Vec::new();
v.push(1);
v.push(2);
v.insert(0, 0);  // 在索引 0 插入
```

**访问元素：**
```rust
let first = &v[0];           // 索引访问（可能 panic）
let first = v.get(0);        // 安全访问（返回 Option）
```

**删除元素：**
```rust
v.pop();                     // 移除最后一个
v.remove(0);                // 移除指定索引
v.clear();                   // 清空
```

### 4. 容量管理

**容量 vs 长度：**
```rust
let mut v = Vec::with_capacity(10);
println!("Capacity: {}, Length: {}", v.capacity(), v.len());
```

**扩容策略：**
- 容量不足时自动扩容
- 通常扩容为当前容量的 2 倍
- 可以使用 `with_capacity` 预分配

### 5. 迭代

**多种迭代方式：**
```rust
for item in &v {            // 不可变引用
    println!("{}", item);
}

for item in &mut v {        // 可变引用
    *item *= 2;
}

for item in v {             // 获取所有权
    println!("{}", item);
}
```

### 6. 切片操作

**转换为切片：**
```rust
let slice: &[i32] = &v[1..3];
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
- 理解 Vector 概念
- 掌握基本操作
- 学习容量管理

### 2. 进阶阶段
- 掌握迭代方法
- 理解内存布局
- 学习性能优化

### 3. 高级阶段
- 构建复杂数据结构
- 优化性能
- 在实际项目中应用

## 💡 最佳实践

1. **预分配容量**：如果知道大小，使用 `with_capacity`
2. **使用 get 方法**：避免索引越界 panic
3. **优先使用迭代器**：比手动循环更安全
4. **注意所有权**：理解 `&`, `&mut`, 和移动的区别
5. **性能考虑**：理解扩容的开销

## 🔍 常见陷阱

1. **索引越界**：使用 `[]` 可能 panic
2. **借用冲突**：同时持有可变和不可变引用
3. **容量浪费**：没有预分配导致多次扩容
4. **所有权问题**：移动后无法使用

## 📚 相关资源

- [Rust Book - Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [Vec Documentation](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [Vector Performance](https://doc.rust-lang.org/std/vec/struct.Vec.html#performance)

## 🎯 总结

Vector 是 Rust 最常用的集合类型，掌握 Vector 的使用是 Rust 编程的基础。

---

**Happy Vectoring! 🦀**

