# Rust 语句与表达式详解

本项目基于 [Rust语言圣经](https://course.rs/basic/base-type/statement-expression.html) 的内容，提供了关于 Rust 语句与表达式的全面、详细的讲解和示例。

## 📚 内容概览

### 核心概念
- **语句 (Statement)**: 执行操作但不返回值的指令
- **表达式 (Expression)**: 计算并产生值的代码
- **表达式语句**: 表达式后加分号变成的语句

### 详细内容

#### 1. 语句类型
- **声明语句**: `let` 变量绑定、函数定义、结构体定义等
- **表达式语句**: 表达式 + 分号

#### 2. 表达式类型
- 字面量表达式 (数字、字符串、布尔值等)
- 元组表达式
- 结构体表达式
- 块表达式 `{}`
- 范围表达式 `1..5`
- if 表达式
- match 表达式
- 数组表达式
- 运算符表达式

#### 3. 实际应用
- 配置初始化
- 数据处理管道
- 错误处理
- 条件逻辑

#### 4. 常见错误
- 试图将语句赋值给变量
- 混淆表达式和语句的返回值
- if 表达式分支类型不匹配

## 🚀 运行示例

### 运行完整演示
```bash
cargo run
```

### 运行测试
```bash
cargo test
```

## 📖 学习路径

1. **基础概念**: 理解语句与表达式的基本区别
2. **语句详解**: 学习声明语句和表达式语句
3. **表达式详解**: 掌握各种表达式类型
4. **实际应用**: 通过实例理解在实际编程中的应用
5. **常见错误**: 避免常见的陷阱和错误
6. **高级用法**: 学习复杂的嵌套和组合用法

## 🎯 关键要点

### 语句 vs 表达式
```rust
// 语句 - 不返回值
let x = 5;  // 声明语句
5;          // 表达式语句

// 表达式 - 返回值
5           // 字面量表达式，返回 5
{ 5 }       // 块表达式，返回 5
if true { 5 } else { 10 }  // if 表达式，返回 5
```

### 块表达式的关键
```rust
// 返回值
let value = {
    let a = 5;
    let b = 10;
    a + b  // 没有分号，这是表达式
}; // value = 15

// 返回 ()
let unit = {
    let a = 5;
    let b = 10;
    a + b; // 有分号，这是语句
}; // unit = ()
```

### 函数返回值
```rust
// 返回表达式的值
fn add(a: i32, b: i32) -> i32 {
    a + b  // 表达式，作为返回值
}

// 返回 ()
fn print_sum(a: i32, b: i32) {
    println!("{}", a + b); // 语句
}
```

## 🔍 深入理解

### 为什么这很重要？
1. **函数式编程**: Rust 是基于表达式的语言
2. **返回值**: 理解何时返回值，何时返回 `()`
3. **代码简洁性**: 表达式让代码更简洁
4. **错误避免**: 避免常见的语法错误

### 与其他语言的区别
- **C/C++**: 主要基于语句
- **JavaScript**: 混合模式
- **Rust**: 基于表达式，语句是特殊情况

## 📝 练习建议

1. 尝试将语句改写为表达式
2. 练习不同类型的块表达式
3. 理解 if、match 作为表达式的用法
4. 实践复杂的嵌套表达式

## 🔗 参考资料

- [Rust 语言圣经 - 语句与表达式](https://course.rs/basic/base-type/statement-expression.html) <mcreference link="https://course.rs/basic/base-type/statement-expression.html" index="1">1</mcreference>
- [Rust Reference - Statements and expressions](https://doc.rust-lang.org/reference/statements-and-expressions.html)
- [Rust By Practice - 语句与表达式](https://practice.rs/basic-types/statements-expressions.html) <mcreference link="https://learnku.com/docs/practice/yu-ju-yu-biao-da-shi/13819" index="2">2</mcreference>

## 💡 提示

- 记住：表达式返回值，语句不返回值
- 分号是语句和表达式的关键区别
- 块表达式的最后一行决定返回值
- if、match 等控制流都是表达式
- 理解这些概念对掌握 Rust 至关重要

---

**Happy Coding! 🦀**