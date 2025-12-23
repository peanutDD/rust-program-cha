# 闭包性能基准测试指南

## 概述

本项目使用 **Criterion** 框架进行性能基准测试，提供准确、可重复的性能测量。

## 快速开始

### 安装依赖

Criterion 已在 `Cargo.toml` 中配置，无需额外安装。

### 运行基准测试

```bash
# 运行所有基准测试
cargo bench

# 运行特定测试组
cargo bench closure_vs_function
cargo bench capture_modes
cargo bench dispatch

# 查看详细输出
cargo bench -- --verbose

# 保存基线（用于后续对比）
cargo bench -- --save-baseline my-baseline

# 与基线对比
cargo bench -- --baseline my-baseline
```

## 测试组说明

### 1. 闭包 vs 函数 (closure_vs_function)

**测试目的**：验证闭包的零成本抽象

```rust
// 测试普通函数
fn function(x: i32) -> i32 { x * 2 + 1 }

// 测试闭包
let closure = |x: i32| x * 2 + 1;
```

**预期结果**：性能应该相同或接近

### 2. 捕获模式 (capture_modes)

**测试目的**：对比不同捕获方式的性能开销

- `no_capture`: 不捕获变量
- `capture_one`: 捕获一个变量
- `capture_multiple`: 捕获多个变量

**预期结果**：
- 无捕获：最快
- 捕获一个/多个：略慢但差异很小

### 3. 静态分发 vs 动态分发 (dispatch)

**测试目的**：对比两种分发方式的性能差异

```rust
// 静态分发
fn call_static<F: Fn(i32) -> i32>(f: &F, x: i32) -> i32 { f(x) }

// 动态分发
fn call_dynamic(f: &dyn Fn(i32) -> i32, x: i32) -> i32 { f(x) }
```

**预期结果**：
- 静态分发：更快（可内联）
- 动态分发：较慢（虚表调用）

### 4. 迭代器与闭包 (iterator)

**测试目的**：验证函数式编程的性能

```rust
// 迭代器 + 闭包
numbers.iter().filter(|&&x| x % 2 == 0).map(|&x| x * 2).sum()

// 手写循环
for &x in &numbers {
    if x % 2 == 0 { result += x * 2; }
}
```

**预期结果**：性能应该相同（编译器优化）

### 5. 闭包大小影响 (closure_size)

**测试目的**：测试捕获不同大小数据的性能影响

- `size_0`: 不捕获（0 bytes）
- `size_i32`: 捕获 i32（4 bytes）
- `size_small_array`: 捕获小数组
- `size_large_ref`: 捕获大数组引用

**预期结果**：
- 大小对调用性能影响很小
- 主要影响栈空间使用

### 6. 闭包组合 (composition)

**测试目的**：测试函数组合的性能

```rust
single: add_one(x)
compose_two: double(add_one(x))
compose_three: square(double(add_one(x)))
```

**预期结果**：
- 编译器优化后差异很小
- 内联后接近零开销

### 7. 输入大小缩放 (input_size)

**测试目的**：测试不同输入大小的性能缩放

测试 10、100、1000、10000 个元素的处理性能

**预期结果**：线性缩放

### 8. Trait 性能 (trait_types)

**测试目的**：对比 Fn、FnMut、FnOnce 的性能

**预期结果**：
- Fn: 最快（无修改）
- FnMut: 略慢（需要可变访问）
- FnOnce: 最慢（需要重新创建）

## 性能优化建议

### 基于测试结果的优化策略

1. **优先使用静态分发**
   ```rust
   // ✅ 好：静态分发
   fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
   
   // ⚠️ 慢：动态分发
   fn apply(f: &dyn Fn(i32) -> i32, x: i32) -> i32 { f(x) }
   ```

2. **最小化捕获**
   ```rust
   let large_data = vec![0; 10000];
   
   // ❌ 捕获整个大数组
   let bad = move || large_data.len();
   
   // ✅ 只捕获长度
   let len = large_data.len();
   let good = move || len;
   ```

3. **使用迭代器**
   ```rust
   // ✅ 编译器会优化为高效循环
   numbers.iter().filter(|&&x| x > 0).map(|&x| x * 2).sum()
   ```

4. **选择合适的 Trait**
   ```rust
   // ✅ 优先使用 Fn
   fn process<F: Fn(i32) -> i32>(f: F) { ... }
   
   // 只在必要时使用 FnMut
   fn process_mut<F: FnMut(i32) -> i32>(mut f: F) { ... }
   ```

## 解读基准测试结果

### Criterion 输出说明

```
closure_vs_function/function
                        time:   [1.2345 µs 1.2456 µs 1.2567 µs]
                        change: [-2.34% -1.23% +0.12%] (p = 0.23 > 0.05)
                        No change in performance detected.
```

- **time**: 执行时间范围（最小值、中位数、最大值）
- **change**: 相对于上次运行的变化
- **p-value**: 统计显著性（p < 0.05 表示显著变化）

### 性能比较

查看生成的 HTML 报告：

```bash
# 运行测试后，打开报告
open target/criterion/report/index.html
```

报告包含：
- 📊 性能图表
- 📈 历史趋势
- 📉 性能对比
- 📋 详细统计

## 常见问题

### Q: 为什么结果不稳定？

**A**: 可能的原因：
1. CPU 频率缩放
2. 后台进程干扰
3. 热插拔效应

**解决方案**：
```bash
# 增加测试次数
cargo bench -- --sample-size 1000

# 增加预热时间
cargo bench -- --warm-up-time 10
```

### Q: 如何对比不同实现？

**A**: 使用基线对比：
```bash
# 保存当前版本为基线
cargo bench -- --save-baseline before

# 修改代码...

# 与基线对比
cargo bench -- --baseline before
```

### Q: 闭包真的零成本吗？

**A**: 是的！测试结果显示：
- 简单闭包与函数性能相同
- 迭代器 + 闭包与手写循环相同
- 编译器会内联和优化闭包

但注意：
- 动态分发有开销
- 捕获大量数据有内存开销
- 复杂闭包可能不会被内联

## 进阶用法

### 自定义基准测试

在 `benches/` 目录创建新文件：

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn my_benchmark(c: &mut Criterion) {
    c.bench_function("my_test", |b| {
        b.iter(|| {
            // 测试代码
            black_box(my_function(black_box(42)))
        });
    });
}

criterion_group!(benches, my_benchmark);
criterion_main!(benches);
```

### 参数化测试

```rust
use criterion::BenchmarkId;

fn parameterized(c: &mut Criterion) {
    let mut group = c.benchmark_group("sizes");
    
    for size in [10, 100, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &size| {
                b.iter(|| process(black_box(size)));
            }
        );
    }
    
    group.finish();
}
```

## 总结

通过基准测试，我们验证了：

✅ **闭包是零成本抽象**
✅ **编译器优化非常有效**
✅ **函数式编程不会牺牲性能**
✅ **静态分发优于动态分发**
✅ **迭代器与手写循环性能相同**

这些结果证明：在 Rust 中，你可以写出既优雅又高效的代码！

