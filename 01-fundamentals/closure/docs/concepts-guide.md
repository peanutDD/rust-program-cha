# Rust 闭包概念指南

## 什么是闭包？

闭包（Closure）是一种可以捕获其环境中变量的匿名函数。在 Rust 中，闭包有以下特点：

- **匿名性**：闭包不需要名称
- **捕获环境**：可以访问外部作用域的变量
- **类型推导**：编译器自动推导参数和返回值类型
- **零成本抽象**：编译后性能与手写代码相同

## 基本语法

```rust
// 最简单的闭包
let simple = || println!("Hello!");

// 带参数的闭包
let add = |x, y| x + y;

// 带类型注解的闭包
let multiply: fn(i32, i32) -> i32 = |x, y| x * y;

// 多行闭包
let complex = |x| {
    let temp = x * 2;
    temp + 1
};
```

## 捕获机制

Rust 闭包有三种捕获方式：

### 1. 不可变借用（Immutable Borrow）

```rust
let name = String::from("Alice");
let print_name = || println!("{}", name);
print_name();
println!("{}", name); // name 仍然可用
```

### 2. 可变借用（Mutable Borrow）

```rust
let mut count = 0;
let mut increment = || {
    count += 1;
    count
};
increment();
```

### 3. 获取所有权（Take Ownership）

```rust
let data = vec![1, 2, 3];
let consume = move || data.len();
// data 已被移动
```

## move 关键字

`move` 关键字强制闭包获取捕获变量的所有权：

```rust
let data = vec![1, 2, 3];
let closure = move || {
    println!("{:?}", data);
};
// data 已被移动，无法再使用
```

## 使用场景

- 迭代器操作（map、filter、fold 等）
- 错误处理（map_err、and_then 等）
- 异步编程（Future、async/await）
- 回调函数
- 策略模式
- 延迟计算

## 学习路径

1. ✅ 掌握基本语法和类型推导
2. ✅ 理解三种捕获方式
3. ✅ 学习 move 关键字的使用
4. → 深入学习 Fn/FnMut/FnOnce trait
5. → 探索高级模式和实际应用
6. → 优化性能和遵循最佳实践

## 常见错误

### 1. 混淆捕获方式

```rust
// ❌ 错误：尝试修改不可变捕获的变量
let x = 10;
let closure = || {
    x += 1; // 编译错误
};
```

### 2. 不必要的 move

```rust
// ❌ 不必要：只需要借用
let data = vec![1, 2, 3];
let closure = move || println!("{:?}", data);
// data 被移动了，但其实只借用就够了

// ✅ 正确：使用借用
let data = vec![1, 2, 3];
let closure = || println!("{:?}", data);
println!("{:?}", data); // data 仍然可用
```

### 3. 生命周期问题

```rust
// ❌ 错误：返回的闭包引用了局部变量
fn create_closure() -> impl Fn() -> i32 {
    let x = 10;
    || x // 编译错误：x 的生命周期不够长
}

// ✅ 正确：使用 move
fn create_closure() -> impl Fn() -> i32 {
    let x = 10;
    move || x // OK：x 被移动到闭包中
}
```

## 最佳实践

1. **优先使用 Fn trait**，除非需要修改或消费变量
2. **保持闭包简洁**，复杂逻辑提取为函数
3. **只捕获需要的变量**，避免不必要的捕获
4. **合理使用 move**，只在真正需要时使用
5. **注意生命周期**，特别是返回闭包时

## 下一步

- 阅读 [Trait 系统指南](./traits-guide.md)
- 学习 [高级模式](./patterns-guide.md)
- 完成 exercises/ 目录下的练习
- 查看 examples/ 目录下的实际示例

