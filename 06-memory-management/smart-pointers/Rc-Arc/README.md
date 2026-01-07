# Rust Rc 和 Arc 全面学习指南

> 基于 https://course.rs/smart-pointer/rc-arc.html 的深度分析

## 📚 项目概述

本项目全面深入地分析 Rust `Rc<T>` 和 `Arc<T>` 引用计数智能指针，帮助开发者理解 Rust 共享所有权的机制。

## 🎯 学习目标

通过本教程，你将学会：

- ✅ 理解引用计数的概念
- ✅ 掌握 Rc<T> 的使用场景
- ✅ 掌握 Arc<T> 的使用场景
- ✅ 理解单线程 vs 多线程共享
- ✅ 学会处理循环引用
- ✅ 理解性能特性

## 📖 核心知识点

### 1. Rc<T> - 单线程引用计数

**核心概念：**
- 引用计数智能指针
- 单线程使用
- 允许多个所有者

**基本用法：**
```rust
use std::rc::Rc;

let data = Rc::new(5);
let data1 = Rc::clone(&data);  // 克隆引用，增加计数
let data2 = Rc::clone(&data);

println!("引用计数: {}", Rc::strong_count(&data));
```

### 2. Arc<T> - 原子引用计数

**核心概念：**
- 线程安全的引用计数
- 使用原子操作
- 允许多线程共享

**基本用法：**
```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(5);
let mut handles = vec![];

for _ in 0..10 {
    let data = Arc::clone(&data);
    let handle = thread::spawn(move || {
        println!("Data: {}", data);
    });
    handles.push(handle);
}
```

### 3. 何时使用 Rc vs Arc

**使用 Rc：**
- 单线程环境
- 需要多个所有者
- 性能敏感场景

**使用 Arc：**
- 多线程环境
- 需要跨线程共享
- 性能开销可接受

### 4. Weak 引用

**避免循环引用：**
```rust
use std::rc::{Rc, Weak};

let strong = Rc::new(5);
let weak = Rc::downgrade(&strong);

match weak.upgrade() {
    Some(rc) => println!("{}", rc),
    None => println!("已被释放"),
}
```

### 5. 内部可变性

**配合 RefCell：**
```rust
use std::rc::Rc;
use std::cell::RefCell;

let data = Rc::new(RefCell::new(5));
*data.borrow_mut() += 1;
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
- 理解引用计数概念
- 掌握 Rc 的基本用法
- 学习 Arc 的基本用法

### 2. 进阶阶段
- 理解 Weak 引用
- 掌握循环引用处理
- 学习内部可变性

### 3. 高级阶段
- 优化性能
- 构建复杂数据结构
- 在实际项目中应用

## 💡 最佳实践

1. **优先使用所有权**：只在需要时才使用 Rc/Arc
2. **使用 Weak 避免循环**：防止内存泄漏
3. **注意性能开销**：引用计数有开销
4. **线程安全选择**：多线程用 Arc，单线程用 Rc
5. **配合内部可变性**：需要可变时使用 RefCell/Mutex

## 🔍 常见陷阱

1. **循环引用**：导致内存泄漏
2. **性能误解**：引用计数有开销
3. **线程安全混淆**：Rc 不能跨线程
4. **过度使用**：不必要的共享所有权

## 📚 相关资源

- [Rust Book - Rc](https://doc.rust-lang.org/book/ch15-04-rc.html)
- [Rc Documentation](https://doc.rust-lang.org/std/rc/struct.Rc.html)
- [Arc Documentation](https://doc.rust-lang.org/std/sync/struct.Arc.html)

## 🎯 总结

Rc 和 Arc 提供了 Rust 中的共享所有权机制，虽然有一定性能开销，但在需要多个所有者的场景中非常有用。

---

**Happy Reference Counting! 🦀**
