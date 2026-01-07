## Rust 所有权系统深度教程（ownership）

本子项目聚焦 Rust 的核心——**所有权系统**，配合 `OWNERSHIP_DEEP_ANALYSIS.md`、`OWNERSHIP_SUMMARY.md` 和多份示例/练习代码，形成一条从入门到深度分析的完整学习路径。

### 📁 项目结构

```text
ownership/
├── src/
│   ├── main.rs                         # 入口：串联各个示例/练习
│   ├── ownership_comprehensive_guide.rs# 所有权完整讲解与案例
│   ├── advanced_ownership_examples.rs  # 进阶/复杂场景示例
│   └── ownership_exercises.rs          # 练习题与思考题
├── OWNERSHIP_DEEP_ANALYSIS.md          # 深度分析：实现机制、边界与设计哲学
├── OWNERSHIP_SUMMARY.md                # 总结：速查表与关键要点
├── Cargo.toml                          # 项目配置
└── rustfmt.toml                        # 代码格式配置
```

### 🎯 学习目标

完成本项目后，你应当能够：

- 清晰解释 **所有权三大规则** 及其背后的编译期约束  
- 理解 **移动（move）/ 拷贝（copy）/ 克隆（clone）** 的区别与成本  
- 熟练分析：函数参数传递与返回值中的所有权流动  
- 识别并修复典型的所有权问题：重复释放、悬垂指针等（在 Rust 中如何被编译器禁止）  
- 在复杂结构（嵌套结构体、集合、闭包等）中追踪所有权  
- 将所有权分析迁移到实际业务设计中，而不只是“过编译”

### 🚀 快速开始

```bash
cd 02-ownership-borrowing/ownership

# 运行主示例（建议首选）
cargo run

# 运行测试（如有）
cargo test
```

> `main.rs` 负责**组织章节顺序**，按「基础 → 进阶 → 练习」的节奏输出各部分内容。

### 📚 学习路径建议

1. **从 `ownership_comprehensive_guide.rs` 开始**  
   - 跟随章节阅读与运行，理解所有权/移动/借用的基础语义  
2. **阅读 `OWNERSHIP_SUMMARY.md` 做一次总览与查缺补漏**  
3. **进入 `advanced_ownership_examples.rs` 观察复杂场景**  
   - 集合、闭包、返回 Iterator、嵌套结构体等  
4. **完成 `ownership_exercises.rs` 中的练习**  
   - 尝试只根据编译错误提示来修复  
5. 如需更深入的实现/设计层面理解，阅读 `OWNERSHIP_DEEP_ANALYSIS.md`

### ✅ 风格与规则对齐

本模块在实现时遵循本仓库的统一规范：

- **默认不可变**：优先使用 `let`，仅在必要时使用 `let mut`  
- **错误与空值使用 `Result` / `Option` 表达**，避免滥用 `panic!`  
- **中文说明 + 英文关键字**：注释与文档使用中文，API/类型名保持 Rust 原生风格  
- 尽量将示例写成 **可测试的函数**，而不是只在 `main` 中打印

### 🔗 配套模块

- `02-ownership-borrowing/reference-borrowing`：在本模块基础上，进一步聚焦 **引用与借用规则、生命周期**  
- `01-fundamentals/basic-concepts/docs/advanced-topics/ownership-memory`：所有权与内存布局的图示与延伸阅读  

如果你希望把“所有权 + 借用 + 生命周期”彻底打通，推荐按顺序学习：

1. `01-fundamentals/basic-concepts` 中的所有权入门  
2. 本模块 `ownership`  
3. `reference-borrowing`  
4. `04-advanced-concepts/advanced-lifetime` 与相关高级示例


