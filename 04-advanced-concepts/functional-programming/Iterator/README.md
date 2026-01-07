# Rust 迭代器 (Iterator) 全面学习指南

> 基于 https://course.rs/advance/functional-programing/iterator.html 的深度分析

## 📚 项目概述

本项目全面深入地分析 Rust 迭代器系统，从基础概念到高级应用，帮助开发者掌握 Rust 函数式编程的核心工具。

## 🎯 学习目标

通过本教程，你将学会：

- ✅ 理解迭代器的核心概念和设计原理
- ✅ 掌握迭代器的创建方式
- ✅ 熟练使用迭代器适配器 (Iterator Adaptors)
- ✅ 掌握消费者适配器 (Consumer Adaptors)
- ✅ 学会实现自定义迭代器
- ✅ 理解迭代器的性能特性
- ✅ 掌握迭代器在实际项目中的应用

## 📖 核心知识点

### 1. 迭代器基础

**核心概念：**
- 迭代器是惰性的（Lazy）
- 迭代器是零成本抽象
- 迭代器实现了 `Iterator` trait

**基本用法：**
```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();

for val in v1_iter {
    println!("{}", val);
}
```

### 2. Iterator Trait

**定义：**
```rust
pub trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
    
    // 默认实现的方法...
}
```

**核心方法：**
- `next()` - 获取下一个元素
- `map()` - 转换元素
- `filter()` - 过滤元素
- `collect()` - 收集到集合
- `fold()` - 折叠操作

### 3. 迭代器适配器

**常用适配器：**
```rust
let doubled: Vec<i32> = vec![1, 2, 3]
    .iter()
    .map(|x| x * 2)
    .filter(|&x| x > 2)
    .collect();
```

**适配器类型：**
- 转换类：`map`, `flat_map`, `filter_map`
- 过滤类：`filter`, `take`, `skip`
- 组合类：`zip`, `chain`, `enumerate`

### 4. 消费者适配器

**常用消费者：**
```rust
let sum: i32 = vec![1, 2, 3].iter().sum();
let max = vec![1, 2, 3].iter().max();
let count = vec![1, 2, 3].iter().count();
```

**消费者类型：**
- 聚合：`sum`, `product`, `max`, `min`
- 查找：`find`, `position`, `rposition`
- 收集：`collect`, `partition`, `unzip`

### 5. 自定义迭代器

**实现 Iterator trait：**
```rust
struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

### 6. 性能特性

**零成本抽象：**
- 编译时优化
- 内联展开
- 消除中间分配

**性能对比：**
- 迭代器 vs 循环：性能相当或更好
- 链式调用：编译器优化为单次循环

## 🚀 快速开始

```bash
# 运行完整教程
cargo run

# 运行测试
cargo test
```

## 📖 学习路径

### 1. 基础阶段
- 理解迭代器概念
- 掌握基本用法
- 学习常用适配器

### 2. 进阶阶段
- 实现自定义迭代器
- 掌握高级适配器
- 理解性能特性

### 3. 高级阶段
- 构建复杂迭代器链
- 优化迭代器性能
- 在实际项目中应用

## 💡 最佳实践

1. **优先使用迭代器**：比手动循环更安全、更清晰
2. **利用惰性求值**：迭代器链不会立即执行
3. **注意所有权**：`iter()` vs `into_iter()` vs `iter_mut()`
4. **性能考虑**：编译器会优化迭代器代码
5. **可读性优先**：清晰的迭代器链比性能微优化更重要

## 🔍 常见陷阱

1. **忘记消费**：迭代器是惰性的，需要消费才能执行
2. **所有权问题**：注意 `iter()` 和 `into_iter()` 的区别
3. **多次使用**：迭代器消费后不能再次使用
4. **性能误解**：迭代器并不比循环慢

## 📚 相关资源

- [Rust Book - Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Iterator Documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [Functional Programming in Rust](https://rust-lang.github.io/rfcs/0216-impl-trait.html)

## 🎯 总结

迭代器是 Rust 函数式编程的核心，掌握迭代器可以编写出更安全、更清晰、更高效的代码。

---

**Happy Iterating! 🦀**

