# Rust 数组 (Array) 全面学习项目

> 基于 [Rust 语言圣经 - 数组章节](https://course.rs/basic/compound-type/array.html) 的全面学习资源

## 📖 项目简介

本项目是一个全面的 Rust 数组学习资源，涵盖了从基础概念到高级应用的所有知识点。通过系统的教程和递进式的练习，帮助你深入理解和掌握 Rust 数组的核心特性。

## 🏗️ 项目结构

```
array/
├── src/
│   ├── main.rs                 # 主程序入口
│   ├── array_comprehensive.rs  # 全面的数组教程
│   └── exercises.rs            # 递进式练习题
├── Cargo.toml                  # 项目配置
└── README.md                   # 项目文档
```

## 🎯 学习内容

### 📚 核心知识点

#### 1. 数组基础概念
- **固定长度**: 编译时确定大小，类型为 `[T; N]`
- **同类型元素**: 所有元素必须是相同类型
- **栈上分配**: 高性能的内存访问
- **零成本抽象**: 运行时性能接近 C 语言

#### 2. 数组声明和初始化
```rust
// 基本声明
let arr1: [i32; 5] = [1, 2, 3, 4, 5];
let arr2 = [1, 2, 3, 4, 5]; // 类型推断

// 重复值初始化
let arr3 = [0; 10]; // 10个0
let arr4 = [42; 5]; // 5个42

// 不同类型数组
let float_arr: [f64; 3] = [1.1, 2.2, 3.3];
let char_arr: [char; 4] = ['a', 'b', 'c', 'd'];
let str_arr: [&str; 3] = ["hello", "world", "rust"];
```

#### 3. 数组访问和安全性
```rust
let arr = [1, 2, 3, 4, 5];

// 基本索引访问
println!("{}", arr[0]); // 1

// 安全访问
match arr.get(2) {
    Some(value) => println!("值: {}", value),
    None => println!("索引越界"),
}

// 边界检查
// arr[10]; // 运行时 panic
```

#### 4. 数组遍历
```rust
let arr = [1, 2, 3, 4, 5];

// 索引遍历
for i in 0..arr.len() {
    println!("arr[{}] = {}", i, arr[i]);
}

// 值遍历
for value in arr {
    println!("值: {}", value);
}

// 引用遍历
for value in &arr {
    println!("引用值: {}", value);
}

// 迭代器遍历
for (index, value) in arr.iter().enumerate() {
    println!("arr[{}] = {}", index, value);
}
```

#### 5. 多维数组
```rust
// 二维数组
let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
println!("{}", matrix[0][1]); // 2

// 三维数组
let cube: [[[i32; 2]; 2]; 2] = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]];

// 遍历二维数组
for (i, row) in matrix.iter().enumerate() {
    for (j, &value) in row.iter().enumerate() {
        println!("matrix[{}][{}] = {}", i, j, value);
    }
}
```

#### 6. 数组切片
```rust
let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// 基本切片
let slice = &arr[2..5];        // [3, 4, 5]
let from_start = &arr[..3];    // [1, 2, 3]
let to_end = &arr[7..];        // [8, 9, 10]
let full_slice = &arr[..];     // 完整切片

// 窗口和分块
for window in arr.windows(3) {
    println!("窗口: {:?}", window);
}

for chunk in arr.chunks(3) {
    println!("块: {:?}", chunk);
}
```

#### 7. 数组方法
```rust
let mut arr = [3, 1, 4, 1, 5, 9, 2, 6];

// 查找
let contains_5 = arr.contains(&5);
let position = arr.iter().position(|&x| x == 5);

// 排序
arr.sort();                    // 升序
arr.sort_by(|a, b| b.cmp(a)); // 降序
arr.sort_unstable();           // 不稳定排序（更快）

// 其他操作
arr.reverse();                 // 反转
arr.fill(0);                   // 填充
arr.swap(0, 1);               // 交换元素

// 聚合操作
let sum: i32 = arr.iter().sum();
let max = arr.iter().max();
let min = arr.iter().min();
```

### 💪 练习题概览

#### 练习1: 数组基础操作
- 数组创建和初始化
- 基本统计计算（和、平均值、最值）
- 元素查找和计数
- 数组变换操作

#### 练习2: 搜索和排序算法
- 线性搜索和二分搜索
- 选择排序、插入排序
- 冒泡排序实现
- 算法性能比较

#### 练习3: 多维数组操作
- 矩阵创建和访问
- 矩阵加法和乘法
- 矩阵转置
- 矩阵的迹计算

#### 练习4: 高级切片操作
- 滑动窗口算法
- 数组分块处理
- 数组旋转
- 最大子数组和（Kadane算法）
- 数组去重

#### 练习5: 经典算法实现
- 快速排序
- 归并排序
- 堆排序
- 计数排序
- 算法复杂度分析

#### 练习6: 实际应用场景
- 学生成绩管理系统
- 图像处理和滤镜
- 时间序列数据分析
- 库存管理系统

#### 练习7: 性能优化
- 缓存友好的数组访问
- 内存对齐和布局
- SIMD 向量化操作
- 性能测试和分析

#### 练习8: 综合应用
- 数据分析工具开发
- 销售数据统计分析
- 趋势预测算法
- 可视化图表生成

## 🚀 快速开始

### 环境要求
- Rust 1.70.0 或更高版本
- Cargo 包管理器

### 运行项目

```bash
# 克隆或进入项目目录
cd array

# 运行完整教程和练习
cargo run

# 运行测试
cargo test

# 查看文档
cargo doc --open
```

### 学习模式

项目提供三种学习模式：

1. **📚 教程模式**: 系统学习所有数组知识点
2. **💪 练习模式**: 通过实践巩固数组技能
3. **🚀 综合模式**: 教程 + 练习完整体验

## 🎯 学习目标

完成本项目学习后，你将能够：

- ✅ 深入理解 Rust 数组的内存模型和性能特性
- ✅ 熟练使用各种数组操作和方法
- ✅ 掌握多维数组和矩阵运算
- ✅ 灵活运用数组切片进行数据处理
- ✅ 实现经典的搜索和排序算法
- ✅ 应用数组解决实际编程问题
- ✅ 优化数组操作的性能
- ✅ 理解数组与其他集合类型的区别和适用场景

## 🔍 核心特性

### 数组 vs 向量 vs 切片

| 特性 | 数组 `[T; N]` | 向量 `Vec<T>` | 切片 `&[T]` |
|------|---------------|---------------|-------------|
| 大小 | 编译时固定 | 运行时动态 | 运行时已知 |
| 内存 | 栈上分配 | 堆上分配 | 引用/视图 |
| 性能 | 最高 | 较高 | 高 |
| 灵活性 | 最低 | 最高 | 中等 |
| 所有权 | 拥有数据 | 拥有数据 | 借用数据 |

### 内存安全保证

- **编译时检查**: 数组大小在编译时确定
- **运行时边界检查**: 防止缓冲区溢出
- **所有权系统**: 防止数据竞争和内存泄漏
- **类型安全**: 强类型系统防止类型错误

## 📚 学习建议

### 循序渐进
1. **基础概念**: 先理解数组的基本特性
2. **基本操作**: 掌握创建、访问、遍历
3. **高级特性**: 学习切片、多维数组
4. **实际应用**: 通过练习巩固知识
5. **性能优化**: 了解性能特性和优化技巧

### 实践为主
- 每学完一个概念就动手实践
- 尝试修改示例代码
- 完成所有练习题
- 思考实际应用场景

### 对比学习
- 比较数组与向量的异同
- 理解何时使用数组，何时使用向量
- 掌握它们之间的转换方法

## 🔗 相关资源

### 官方文档
- [Rust 语言圣经 - 数组](https://course.rs/basic/compound-type/array.html)
- [Rust By Example - 数组和切片](https://rustwiki.org/zh-CN/rust-by-example/primitives/array.html)
- [Rust 标准库 - std::array](https://doc.rust-lang.org/std/array/)
- [The Rust Programming Language - Arrays](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type)

### 进阶学习
- [Rust 性能优化指南](https://nnethercote.github.io/perf-book/)
- [Rust 算法和数据结构](https://github.com/TheAlgorithms/Rust)
- [Rust 并发编程](https://course.rs/advance/concurrency/intro.html)

## 🤝 贡献

欢迎提交 Issue 和 Pull Request 来改进这个学习项目！

## 📄 许可证

本项目采用 MIT 许可证。

---

🎉 **开始你的 Rust 数组学习之旅吧！**