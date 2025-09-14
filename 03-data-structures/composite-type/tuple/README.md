# 🦀 Rust 元组学习教程

一个全面的 Rust 元组（Tuple）学习项目，包含详细的教程、实践练习和实际应用案例。

## 📚 项目简介

本项目基于 [Rust 语言圣经](https://course.rs/basic/compound-type/tuple.html) 的元组章节，提供了：

- 📖 **详细教程**：从基础概念到高级应用的完整讲解
- 🏃 **实践练习**：10个不同难度的练习题，涵盖各种应用场景
- 🎯 **实际案例**：几何计算器、状态机、配置管理等实用示例
- 🧪 **单元测试**：确保代码质量和正确性

## 🚀 快速开始

### 环境要求

- Rust 1.70.0 或更高版本
- Cargo（随 Rust 安装）

### 运行项目

```bash
# 克隆或进入项目目录
cd tuple

# 运行主程序（包含所有教程和练习）
cargo run

# 运行测试
cargo test

# 查看文档
cargo doc --open
```

## 📁 项目结构

```
tuple/
├── src/
│   ├── main.rs                    # 主程序入口
│   ├── tuple_comprehensive.rs     # 详细教程和示例
│   └── exercises.rs               # 练习题集
├── Cargo.toml                     # 项目配置
└── README.md                      # 项目说明
```

## 📖 学习内容

### 1. 基础概念
- 元组的定义和语法
- 元组的类型系统
- 单元元组和空元组
- 内存布局和性能特点

### 2. 基本操作
- 创建和初始化元组
- 通过索引访问元素
- 元组解构和模式匹配
- 元组的比较和排序

### 3. 高级应用
- 函数参数和返回值
- 嵌套元组和复杂结构
- 错误处理和选项类型
- 迭代器和集合操作

### 4. 实际项目
- 几何图形计算器
- 简单状态机实现
- 配置管理系统
- 性能优化技巧

## 🏃 练习题目

1. **基础操作** - 元组创建、访问、解构
2. **类型和嵌套** - 复杂类型定义和嵌套结构
3. **模式匹配** - 条件处理和模式匹配
4. **函数应用** - 参数传递和返回值
5. **实用工具** - 工具函数编写
6. **错误处理** - 安全编程实践
7. **迭代器操作** - 集合处理和数据变换
8. **状态机** - 状态管理和配置
9. **性能优化** - 内存布局和性能考虑
10. **综合项目** - 几何计算器实现

## 🎯 学习目标

完成本教程后，你将能够：

- ✅ 熟练使用元组进行数据组织
- ✅ 理解元组的内存布局和性能特点
- ✅ 在实际项目中合理选择数据结构
- ✅ 编写高质量、高性能的 Rust 代码
- ✅ 掌握函数式编程的基本概念

## 🔧 代码示例

### 基础用法

```rust
// 创建元组
let person = ("Alice", 25, 175.5);

// 访问元素
println!("姓名: {}", person.0);
println!("年龄: {}", person.1);
println!("身高: {:.1}cm", person.2);

// 解构赋值
let (name, age, height) = person;
println!("{} 今年 {} 岁，身高 {:.1}cm", name, age, height);
```

### 函数应用

```rust
// 返回多个值
fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}

let (quotient, remainder) = divide_with_remainder(17, 5);
println!("17 ÷ 5 = {} 余 {}", quotient, remainder);
```

### 模式匹配

```rust
// 坐标象限判断
fn quadrant(point: (i32, i32)) -> &'static str {
    match point {
        (0, 0) => "原点",
        (x, y) if x > 0 && y > 0 => "第一象限",
        (x, y) if x < 0 && y > 0 => "第二象限",
        (x, y) if x < 0 && y < 0 => "第三象限",
        (x, y) if x > 0 && y < 0 => "第四象限",
        (0, _) => "x轴",
        (_, 0) => "y轴",
    }
}
```

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request！

1. Fork 本项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🔗 相关资源

- [Rust 官方文档](https://doc.rust-lang.org/book/)
- [Rust 语言圣经](https://course.rs/)
- [Rust 练习题](https://github.com/rust-lang/rustlings)
- [Rust 标准库文档](https://doc.rust-lang.org/std/)

## 📞 联系方式

如有问题或建议，请通过以下方式联系：

- 📧 Email: rust-learning@example.com
- 💬 GitHub Issues: [提交问题](https://github.com/rust-learning/tuple-tutorial/issues)
- 🌐 官网: https://rust-learning.example.com

---

**Happy Coding! 🦀**

*让我们一起探索 Rust 的美妙世界！*