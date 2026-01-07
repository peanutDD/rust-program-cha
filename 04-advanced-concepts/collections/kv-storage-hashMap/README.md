# Rust HashMap 键值存储全面学习指南

> 基于 https://course.rs/basic/compound-type/hashmap.html 的深度分析

## 📚 项目概述

本项目全面深入地分析 Rust `HashMap<K, V>` 键值存储，从基础概念到高级应用，帮助开发者掌握 Rust 的哈希表实现。

## 🎯 学习目标

通过本教程，你将学会：

- ✅ 理解 HashMap 的实现原理
- ✅ 掌握 HashMap 的创建和初始化
- ✅ 熟练使用 HashMap 的基本操作
- ✅ 理解哈希函数和冲突处理
- ✅ 掌握 HashMap 的迭代方法
- ✅ 学会在实际项目中应用 HashMap

## 📖 核心知识点

### 1. HashMap 基础

**类型定义：**
```rust
use std::collections::HashMap;

let mut map: HashMap<String, i32> = HashMap::new();
```

**核心特性：**
- 键值对存储
- O(1) 平均时间复杂度
- 键必须实现 `Hash` 和 `Eq` trait

### 2. 创建和初始化

**多种创建方式：**
```rust
let mut map = HashMap::new();
map.insert("key".to_string(), 42);

let map: HashMap<_, _> = [
    ("key1", 1),
    ("key2", 2),
].iter().cloned().collect();
```

### 3. 基本操作

**插入和更新：**
```rust
map.insert("key".to_string(), 42);
map.insert("key".to_string(), 100);  // 覆盖

// 只在不存在时插入
map.entry("key".to_string()).or_insert(42);
```

**访问元素：**
```rust
let value = map.get("key");           // 返回 Option<&V>
let value = map.get_mut("key");      // 返回 Option<&mut V>
let value = &map["key"];              // 可能 panic
```

**删除元素：**
```rust
map.remove("key");                    // 删除并返回值
map.clear();                          // 清空
```

### 4. Entry API

**强大的 Entry API：**
```rust
// 只在不存在时插入
map.entry("key".to_string())
    .or_insert(0);

// 更新已存在的值
map.entry("key".to_string())
    .and_modify(|v| *v += 1)
    .or_insert(1);
```

### 5. 迭代

**多种迭代方式：**
```rust
for (key, value) in &map {
    println!("{}: {}", key, value);
}

for key in map.keys() {
    println!("{}", key);
}

for value in map.values() {
    println!("{}", value);
}
```

### 6. 哈希函数

**自定义哈希函数：**
```rust
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

let mut hasher = DefaultHasher::new();
key.hash(&mut hasher);
let hash = hasher.finish();
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
- 理解 HashMap 概念
- 掌握基本操作
- 学习 Entry API

### 2. 进阶阶段
- 掌握迭代方法
- 理解哈希函数
- 学习性能优化

### 3. 高级阶段
- 自定义哈希函数
- 优化性能
- 在实际项目中应用

## 💡 最佳实践

1. **使用 Entry API**：更安全和高效
2. **预分配容量**：如果知道大小，使用 `with_capacity`
3. **注意键的类型**：键必须实现 `Hash` 和 `Eq`
4. **性能考虑**：理解哈希冲突的影响
5. **所有权管理**：注意键值对的所有权

## 🔍 常见陷阱

1. **键不存在**：使用 `[]` 可能 panic
2. **借用冲突**：同时持有可变和不可变引用
3. **哈希冲突**：理解性能影响
4. **所有权问题**：插入后键的所有权转移

## 📚 相关资源

- [Rust Book - Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
- [HashMap Documentation](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [Hash Trait](https://doc.rust-lang.org/std/hash/trait.Hash.html)

## 🎯 总结

HashMap 是 Rust 中强大的键值存储工具，掌握 HashMap 的使用可以高效地处理键值对数据。

---

**Happy Hashing! 🦀**

