//! # Rust 注释基础知识全面分析
//!
//! 本模块深入分析 Rust 中的基础注释类型、语法规则和使用场景。
//! 基于 https://course.rs/basic/comment.html 的内容进行全面扩展。

/// 演示 Rust 注释基础知识的主函数
///
/// 这个函数展示了 Rust 中所有类型的注释及其使用场景。
///
/// # 注释类型概览
///
/// Rust 支持以下几种注释类型：
/// 1. 行注释 (`//`)
/// 2. 块注释 (`/* */`)
/// 3. 文档注释 (`///` 和 `//!`)
/// 4. 属性注释 (`#[doc]`)
pub fn demonstrate_comment_basics() {
    println!("\n=== Rust 注释基础知识演示 ===");

    // 演示各种注释类型
    demonstrate_line_comments();
    demonstrate_block_comments();
    demonstrate_nested_comments();
    demonstrate_comment_conventions();
    demonstrate_comment_edge_cases();
}

/// 演示行注释的各种用法
///
/// 行注释是 Rust 中最常用的注释类型，使用 `//` 开头。
///
/// # 语法规则
///
/// - 以 `//` 开头，注释内容延续到行末
/// - 可以出现在代码行的任何位置
/// - 支持 Unicode 字符
/// - 不能跨行（每行都需要 `//`）
fn demonstrate_line_comments() {
    println!("\n--- 行注释演示 ---");

    // 1. 基本行注释
    // 这是一个简单的行注释
    let x = 42; // 行尾注释

    // 2. 多行注释（每行都用 //）
    // 这是第一行注释
    // 这是第二行注释
    // 这是第三行注释
    let y = x * 2;

    // 3. 缩进注释（保持代码对齐）
    if x > 0 {
        // 条件内的注释
        println!("x 是正数: {}", x);

        // 嵌套块中的注释
        if y > x {
            // 更深层的注释
            println!("y 大于 x: {} > {}", y, x);
        }
    }

    // 4. Unicode 支持
    // 这里支持中文注释 🦀
    // Здесь поддерживается русский язык
    // ここは日本語をサポートします
    let unicode_var = "Rust supports Unicode in comments! 🚀";

    // 5. 特殊字符和符号
    // TODO: 实现更多功能
    // FIXME: 修复这个 bug
    // NOTE: 重要提醒
    // HACK: 临时解决方案
    // XXX: 需要重构

    // 6. 代码注释（临时禁用代码）
    // let disabled_code = "这行代码被注释掉了";
    // println!("这行不会执行: {}", disabled_code);

    println!("行注释演示完成: x={}, y={}, unicode={}", x, y, unicode_var);
}

/// 演示块注释的各种用法
///
/// 块注释使用 `/* */` 包围，可以跨越多行。
///
/// # 语法规则
///
/// - 以 `/*` 开始，以 `*/` 结束
/// - 可以跨越多行
/// - 可以嵌套（Rust 特有功能）
/// - 可以出现在表达式中间
fn demonstrate_block_comments() {
    println!("\n--- 块注释演示 ---");

    /* 1. 基本块注释 */
    let a = 10;

    /*
     * 2. 多行块注释
     * 通常使用星号对齐
     * 提高可读性
     */
    let b = 20;

    /*
    3. 无星号的多行块注释
    这种风格也是可以的
    但可读性稍差
    */
    let c = 30;

    /* 4. 行内块注释 */
    let d = /* 表达式中的注释 */ a + b;

    // 5. 表达式内部的块注释
    let result = a /* 第一个操作数 */ + /* 运算符 */ b /* 第二个操作数 */;

    /* 6. 函数调用中的块注释 */
    println!(
        "计算结果: {} + {} = {}",
        a, /* 参数1 */ b, /* 参数2 */ result /* 结果 */
    );

    // 7. 复杂的块注释结构
    /*
     * ================================
     * 复杂数据结构说明
     * ================================
     *
     * 这里可以包含：
     * - 算法说明
     * - 数据结构图
     * - 性能分析
     * - 使用示例
     *
     * 时间复杂度: O(1)
     * 空间复杂度: O(1)
     * ================================
     */
    let complex_data = (a, b, c, d);

    println!(
        "块注释演示完成: a={}, b={}, c={}, d={}, result={}, complex={:?}",
        a, b, c, d, result, complex_data
    );
}

/// 演示嵌套注释的特殊功能
///
/// Rust 支持嵌套块注释，这是很多语言不支持的特性。
///
/// # 嵌套规则
///
/// - 每个 `/*` 必须有对应的 `*/`
/// - 可以任意层级嵌套
/// - 常用于临时注释掉包含注释的代码块
fn demonstrate_nested_comments() {
    println!("\n--- 嵌套注释演示 ---");

    /*
     * 外层注释开始
     *
     * /* 内层注释开始
     *    这里是嵌套的注释内容
     *    /* 更深层的注释
     *       可以无限嵌套
     *    */
     *    内层注释结束
     * */
     *
     * 外层注释结束
     */
    let nested_demo = "嵌套注释演示";

    /*
    // 注释掉包含行注释的代码块
    let temp_var = 42; // 这是行注释
    println!("临时代码: {}", temp_var);

    /* 注释掉包含块注释的代码
       let another_var = "test";
    */
    */

    // 实际的嵌套注释使用场景
    let active_code = "这行代码是活跃的";

    /*
    // 临时禁用的功能模块
    fn disabled_function() {
        /* 这个函数暂时不用
           包含复杂的逻辑
        */
        println!("这个函数被注释掉了");
    }

    // 调用被禁用的函数
    disabled_function();
    */

    println!("嵌套注释演示完成: {}", nested_demo);
    println!("活跃代码: {}", active_code);
}

/// 演示注释的编码规范和最佳实践
///
/// 良好的注释习惯能显著提高代码的可维护性。
///
/// # 注释规范
///
/// 1. **内容规范**：注释应该解释"为什么"而不是"是什么"
/// 2. **格式规范**：保持一致的注释风格
/// 3. **位置规范**：注释应该放在合适的位置
/// 4. **更新规范**：代码变更时同步更新注释
fn demonstrate_comment_conventions() {
    println!("\n--- 注释规范演示 ---");

    // ✅ 好的注释：解释为什么这样做
    // 使用二分查找提高搜索效率，因为数组已排序
    let sorted_array = [1, 3, 5, 7, 9, 11, 13];

    // ❌ 不好的注释：只是重复代码
    // let sorted_array = [1, 3, 5, 7, 9, 11, 13]; // 创建一个数组

    // ✅ 好的注释：解释复杂的业务逻辑
    // 根据用户等级计算折扣：VIP用户享受20%折扣，普通用户10%
    let user_level = "VIP";
    let discount = match user_level {
        "VIP" => 0.2,      // VIP用户20%折扣
        "Premium" => 0.15, // 高级用户15%折扣
        _ => 0.1,          // 普通用户10%折扣
    };

    // ✅ 好的注释：解释算法选择的原因
    // 使用快速排序而不是冒泡排序，因为数据量较大（>1000项）
    // 时间复杂度从 O(n²) 降低到 O(n log n)
    let large_dataset_size = 5000;

    // ✅ 好的注释：标记临时解决方案
    // TODO: 重构这个函数，拆分为更小的函数
    // FIXME: 处理除零错误
    // HACK: 临时绕过API限制，等待官方修复
    let temp_solution = "临时方案";

    // ✅ 好的注释：解释魔法数字
    const MAX_RETRY_COUNT: u32 = 3; // 网络请求最大重试次数
    const TIMEOUT_MS: u64 = 5000; // 请求超时时间（毫秒）

    // ✅ 好的注释：解释复杂的正则表达式
    // 匹配邮箱格式：用户名@域名.后缀
    // ^[\w\.-]+@ : 用户名部分（字母数字下划线点横线）
    // [\w\.-]+    : 域名部分
    // \.[a-zA-Z]{2,}$ : 后缀部分（至少2个字母）
    let email_pattern = r"^[\w\.-]+@[\w\.-]+\.[a-zA-Z]{2,}$";

    println!("注释规范演示完成:");
    println!("  - 数组长度: {}", sorted_array.len());
    println!("  - 用户折扣: {}%", (discount * 100.0) as u32);
    println!("  - 数据集大小: {}", large_dataset_size);
    println!("  - 临时方案: {}", temp_solution);
    println!("  - 最大重试: {}", MAX_RETRY_COUNT);
    println!("  - 超时时间: {}ms", TIMEOUT_MS);
    println!("  - 邮箱模式: {}", email_pattern);
}

/// 演示注释的边界情况和特殊用法
///
/// 包括一些不常见但有用的注释技巧和需要注意的陷阱。
///
/// # 特殊情况
///
/// 1. 字符串中的注释符号
/// 2. 注释中的特殊字符
/// 3. 条件编译中的注释
/// 4. 宏中的注释处理
fn demonstrate_comment_edge_cases() {
    println!("\n--- 注释边界情况演示 ---");

    // 1. 字符串中包含注释符号（不会被当作注释）
    let comment_in_string = "这个字符串包含 // 和 /* */ 符号";
    let url = "https://example.com/path?param=value#fragment";

    // 2. 原始字符串中的注释符号
    let raw_string = r#"这是原始字符串 // 这不是注释 /* 这也不是 */"#;

    // 3. 多行字符串中的注释符号
    let multiline = "
        第一行 // 不是注释
        第二行 /* 不是注释 */
        第三行
    ";

    // 4. 注释中的特殊字符和转义
    // 反斜杠: \\ 引号: \" 制表符: \t 换行符: \n
    // 5. 注释中的 Unicode 和 Emoji
    // 🦀 Rust 螃蟹 🚀 火箭 ⚡ 闪电 🔥 火焰
    // 数学符号: ∑ ∏ ∫ ∆ ∇ ∞ ≈ ≠ ≤ ≥

    // 6. 条件编译中的注释
    #[cfg(debug_assertions)]
    {
        // 这个块只在调试模式下编译
        // 可以包含调试相关的注释
        println!("调试模式下的注释");
    }

    // 7. 注释中的代码示例（不会被执行）
    // 示例代码：
    // ```rust
    // let example = "这是注释中的代码示例";
    // println!("{}", example);
    // ```

    // 8. 注释的性能影响（编译时会被移除）
    // 大量的注释不会影响运行时性能
    /*
     * 这是一个非常长的注释块
     * 包含大量的文本内容
     * 但不会影响编译后的二进制文件大小
     * 因为注释在编译时会被完全移除
     */

    // 9. 注释与宏的交互
    macro_rules! commented_macro {
        () => {
            // 宏内部的注释
            println!("宏生成的代码");
        };
    }

    commented_macro!(); // 调用包含注释的宏

    println!("边界情况演示完成:");
    println!("  - 字符串中的符号: {}", comment_in_string);
    println!("  - URL: {}", url);
    println!("  - 原始字符串: {}", raw_string);
    println!("  - 多行字符串长度: {}", multiline.len());
}

// 模块级别的注释示例
// 这些注释描述整个模块的功能和设计

/*
 * =====================================
 * 模块总结
 * =====================================
 *
 * 本模块全面展示了 Rust 中注释的各种用法：
 *
 * 1. 行注释 (//) - 最常用的注释类型
 * 2. 块注释 (/* */) - 适合多行和行内注释
 * 3. 嵌套注释 - Rust 特有的强大功能
 * 4. 注释规范 - 提高代码可维护性
 * 5. 边界情况 - 特殊场景的处理
 *
 * 关键要点：
 * - 注释应该解释"为什么"而不是"是什么"
 * - 保持注释与代码同步更新
 * - 使用一致的注释风格
 * - 合理使用 TODO、FIXME 等标记
 *
 * =====================================
 */

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试注释基础功能
    ///
    /// 这个测试确保所有演示函数都能正常运行。
    #[test]
    fn test_comment_basics() {
        // 测试不应该 panic
        demonstrate_comment_basics();
    }

    /// 测试各个子功能
    #[test]
    fn test_individual_functions() {
        // 测试每个演示函数
        demonstrate_line_comments();
        demonstrate_block_comments();
        demonstrate_nested_comments();
        demonstrate_comment_conventions();
        demonstrate_comment_edge_cases();
    }
}
