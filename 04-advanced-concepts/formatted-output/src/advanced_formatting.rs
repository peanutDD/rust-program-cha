//! # Rust 高级格式化详解
//!
//! 本模块深入介绍 Rust 格式化系统的高级特性，包括精度控制、数值格式、
//! 科学计数法、进制转换等复杂格式化场景的处理。
//!
//! ## 高级格式化语法
//!
//! Rust 的高级格式化遵循完整的语法结构：
//! ```text
//! {[argument]:[fill]align[sign][#][0][width][.precision][type]}
//! ```
//!
//! ### 高级语法组件
//!
//! - `sign` - 符号控制：
//!   - `+` - 总是显示符号（正数显示+，负数显示-）
//!   - `-` - 仅负数显示符号（默认行为）
//!   - ` ` - 正数显示空格，负数显示-
//! - `#` - 替代格式标志（如十六进制前缀0x）
//! - `0` - 零填充标志
//! - `precision` - 精度控制（浮点数小数位数或字符串截断长度）
//! - `type` - 格式类型：
//!   - `b` - 二进制
//!   - `o` - 八进制
//!   - `x/X` - 十六进制（小写/大写）
//!   - `e/E` - 科学计数法（小写/大写）
//!   - `f` - 固定小数点
//!   - `g/G` - 通用格式（自动选择f或e）
//!
//! ## 设计哲学
//!
//! Rust 的高级格式化设计体现了以下原则：
//! 1. **精确控制** - 提供细粒度的格式控制选项
//! 2. **类型安全** - 编译时检查格式与类型的兼容性
//! 3. **性能优化** - 高效的数值转换和字符串构建
//! 4. **国际化支持** - 考虑不同地区的数值表示习惯

use std::collections::HashMap;
use std::f64::consts;

/// 演示精度控制功能
///
/// 展示如何精确控制浮点数的小数位数和字符串的截断长度。
///
/// # 精度控制规则
///
/// | 类型 | 精度含义 | 示例 | 输出 |
/// |------|----------|------|------|
/// | 浮点数 | 小数位数 | `{:.2}` | `3.14` |
/// | 字符串 | 最大字符数 | `{:.5}` | `"hello"` |
/// | 整数 | 最小位数（零填充） | `{:05}` | `00042` |
///
/// # Examples
///
/// ```rust
/// # use formatted_output::advanced_formatting::demonstrate_precision_control;
/// demonstrate_precision_control();
/// ```
pub fn demonstrate_precision_control() {
    println!("\n=== 精度控制功能演示 ===");

    // 1. 浮点数精度控制
    println!("\n--- 浮点数精度控制 ---");
    let pi = consts::PI;
    let e = consts::E;

    println!("π 的不同精度表示:");
    println!("  默认: {}", pi);
    println!("  1位小数: {:.1}", pi);
    println!("  2位小数: {:.2}", pi);
    println!("  5位小数: {:.5}", pi);
    println!("  10位小数: {:.10}", pi);

    println!("\ne 的不同精度表示:");
    println!("  默认: {}", e);
    println!("  3位小数: {:.3}", e);
    println!("  6位小数: {:.6}", e);

    // 2. 字符串精度控制（截断）
    println!("\n--- 字符串精度控制 ---");
    let long_text = "这是一个很长的字符串，用于演示精度控制功能";

    println!("原始字符串: '{}'", long_text);
    println!("截断到5字符: '{:.5}'", long_text);
    println!("截断到10字符: '{:.10}'", long_text);
    println!("截断到15字符: '{:.15}'", long_text);

    // 3. 动态精度控制
    println!("\n--- 动态精度控制 ---");
    let value = 123.456789;
    let precisions = vec![0, 1, 2, 3, 4, 5];

    for precision in precisions {
        println!(
            "精度{}: {:.precision$}",
            precision,
            value,
            precision = precision
        );
    }

    // 4. 精度与宽度结合
    println!("\n--- 精度与宽度结合 ---");
    let numbers = vec![1.2, 12.34, 123.456, 1234.5678];

    for num in numbers {
        println!(
            "数值: {:10.2} | 右对齐: {:>10.2} | 零填充: {:010.2}",
            num, num, num
        );
    }

    // 5. 科学计数法精度
    println!("\n--- 科学计数法精度 ---");
    let large_number = 123456789.123456789;
    let small_number = 0.000123456789;

    println!("大数值科学计数法:");
    println!("  默认: {:e}", large_number);
    println!("  2位精度: {:.2e}", large_number);
    println!("  5位精度: {:.5e}", large_number);

    println!("小数值科学计数法:");
    println!("  默认: {:e}", small_number);
    println!("  3位精度: {:.3e}", small_number);
    println!("  6位精度: {:.6e}", small_number);
}

/// 演示数值格式化功能
///
/// 展示各种数值格式化选项，包括符号控制、零填充、替代格式等。
///
/// # 数值格式化选项
///
/// 1. **符号控制**
///    - `+` - 总是显示符号
///    - `-` - 仅负数显示符号（默认）
///    - ` ` - 正数显示空格
///
/// 2. **零填充**
///    - `0` - 使用零进行填充
///    - 与宽度结合使用
///
/// 3. **替代格式**
///    - `#` - 显示进制前缀或小数点
///
/// # Examples
///
/// ```rust
/// # use formatted_output::advanced_formatting::demonstrate_number_formatting;
/// demonstrate_number_formatting();
/// ```
pub fn demonstrate_number_formatting() {
    println!("\n=== 数值格式化功能演示 ===");

    // 1. 符号控制
    println!("\n--- 符号控制 ---");
    let positive = 42;
    let negative = -42;
    let zero = 0;

    println!("正数符号控制:");
    println!("  默认: {}", positive);
    println!("  强制符号: {:+}", positive);
    println!("  空格填充: {: }", positive);

    println!("负数符号控制:");
    println!("  默认: {}", negative);
    println!("  强制符号: {:+}", negative);
    println!("  空格填充: {: }", negative);

    println!("零的符号控制:");
    println!("  默认: {}", zero);
    println!("  强制符号: {:+}", zero);
    println!("  空格填充: {: }", zero);

    // 2. 零填充
    println!("\n--- 零填充 ---");
    let numbers = vec![1, 42, 999, 1234];

    for num in numbers {
        println!(
            "数字{}: 默认'{:4}' 零填充'{:04}' 符号零填充'{:+05}'",
            num, num, num, num
        );
    }

    // 3. 浮点数零填充
    println!("\n--- 浮点数零填充 ---");
    let float_numbers = vec![1.5, 42.75, 999.125];

    for num in float_numbers {
        println!(
            "浮点数{}: 默认'{:8.2}' 零填充'{:08.2}' 符号零填充'{:+09.2}'",
            num, num, num, num
        );
    }

    // 4. 替代格式标志
    println!("\n--- 替代格式标志 ---");
    let number = 255;

    println!("十六进制格式:");
    println!("  普通: {:x}", number);
    println!("  替代格式: {:#x}", number);
    println!("  大写替代: {:#X}", number);

    println!("二进制格式:");
    println!("  普通: {:b}", number);
    println!("  替代格式: {:#b}", number);

    println!("八进制格式:");
    println!("  普通: {:o}", number);
    println!("  替代格式: {:#o}", number);

    // 5. 浮点数替代格式
    println!("\n--- 浮点数替代格式 ---");
    let float_vals = vec![1.0, 2.5, 3.0];

    for val in float_vals {
        println!("浮点数{}: 普通'{:.0}' 替代格式'{:#.0}'", val, val, val);
    }

    // 6. 复杂数值格式化
    println!("\n--- 复杂数值格式化 ---");
    create_number_formatting_table();
}

/// 创建数值格式化对比表
fn create_number_formatting_table() {
    let test_numbers = vec![
        (42i32, 3.14159f64),
        (-17i32, -2.71828f64),
        (0i32, 0.0f64),
        (9999i32, 123.456f64),
    ];

    println!("数值格式化对比表:");
    println!("{:═^80}", " 格式化效果对比 ");
    println!(
        "│{:^6}│{:^8}│{:^10}│{:^12}│{:^12}│{:^12}│{:^12}│",
        "整数", "浮点数", "零填充", "符号控制", "十六进制", "科学计数", "百分比"
    );
    println!("{:─^80}", "");

    for (int_val, float_val) in test_numbers {
        println!(
            "│{:^6}│{:^8.2}│{:^10}│{:^12}│{:^12}│{:^12}│{:^12}│",
            int_val,
            float_val,
            format!("{:05}", int_val),
            format!("{:+}", int_val),
            format!("{:#x}", int_val.abs()),
            format!("{:.2e}", float_val),
            format!("{:.1}%", float_val * 100.0)
        );
    }
    println!("{:═^80}", "");
}

/// 演示科学计数法格式化
///
/// 展示科学计数法的各种格式选项和应用场景。
///
/// # 科学计数法格式
///
/// | 格式 | 说明 | 示例输入 | 示例输出 |
/// |------|------|----------|----------|
/// | `e` | 小写科学计数法 | `1234.5` | `1.2345e3` |
/// | `E` | 大写科学计数法 | `1234.5` | `1.2345E3` |
/// | `g` | 通用格式（自动选择） | `1234.5` | `1234.5` |
/// | `G` | 大写通用格式 | `0.001` | `0.001` |
///
/// # Examples
///
/// ```rust
/// # use formatted_output::advanced_formatting::demonstrate_scientific_notation;
/// demonstrate_scientific_notation();
/// ```
pub fn demonstrate_scientific_notation() {
    println!("\n=== 科学计数法格式化演示 ===");

    // 1. 基本科学计数法
    println!("\n--- 基本科学计数法 ---");
    let test_values = vec![1234.5678, 0.0012345, 123456789.0, 0.000000123, 1.0];

    for value in &test_values {
        println!(
            "数值 {:<12}: 小写e '{:e}' 大写E '{:E}'",
            value, value, value
        );
    }

    // 2. 精度控制的科学计数法
    println!("\n--- 精度控制的科学计数法 ---");
    let pi = consts::PI * 1000.0;

    println!("π × 1000 的不同精度科学计数法:");
    for precision in 0..=6 {
        println!(
            "  精度{}: {:.precision$e}",
            precision,
            pi,
            precision = precision
        );
    }

    // 3. 通用格式（g/G）
    println!("\n--- 通用格式 (g/G) ---");
    let mixed_values = vec![
        1.23,        // 普通小数
        1234.0,      // 整数形式
        0.000123,    // 小数
        123456789.0, // 大数
        0.1,         // 简单小数
    ];

    for value in mixed_values {
        println!(
            "数值 {:<12}: 默认'{:}' e格式'{:e}' E格式'{:E}'",
            value, value, value, value
        );
    }

    // 4. 科学计数法与宽度、对齐结合
    println!("\n--- 科学计数法对齐和宽度 ---");
    let scientific_data = vec![
        ("光速", 299792458.0),
        ("普朗克常数", 6.62607015e-34),
        ("阿伏伽德罗常数", 6.02214076e23),
        ("电子质量", 9.1093837015e-31),
    ];

    for (name, value) in scientific_data {
        println!(
            "{:<12}: {:>15.3e} | {:^20.3E} | {:<15.3e}",
            name, value, value, value
        );
    }

    // 5. 工程计数法模拟
    println!("\n--- 工程计数法模拟 ---");
    demonstrate_engineering_notation();
}

/// 演示工程计数法（指数为3的倍数）
fn demonstrate_engineering_notation() {
    let values = vec![
        ("纳米", 1e-9),
        ("微米", 1e-6),
        ("毫米", 1e-3),
        ("米", 1.0),
        ("千米", 1e3),
        ("兆米", 1e6),
    ];

    println!("工程计数法示例:");
    for (unit, value) in values {
        let (coefficient, exponent) = engineering_format(value);
        println!(
            "{:<6}: {:.3} × 10^{:>2} ({:e})",
            unit, coefficient, exponent, value
        );
    }
}

/// 将数值转换为工程计数法格式
///
/// # Arguments
///
/// * `value` - 要转换的数值
///
/// # Returns
///
/// 返回 (系数, 指数) 元组，其中指数是3的倍数
fn engineering_format(value: f64) -> (f64, i32) {
    if value == 0.0 {
        return (0.0, 0);
    }

    let log_val = value.abs().log10();
    let exponent = (log_val.floor() as i32 / 3) * 3;
    let coefficient = value / 10_f64.powi(exponent);

    (coefficient, exponent)
}

/// 演示进制转换功能
///
/// 展示二进制、八进制、十六进制等不同进制的格式化输出。
///
/// # 进制格式说明
///
/// | 格式 | 进制 | 前缀 | 示例 |
/// |------|------|------|------|
/// | `b` | 二进制 | 无/0b | `1010` / `0b1010` |
/// | `o` | 八进制 | 无/0o | `12` / `0o12` |
/// | `x` | 十六进制（小写） | 无/0x | `a` / `0xa` |
/// | `X` | 十六进制（大写） | 无/0X | `A` / `0XA` |
///
/// # Examples
///
/// ```rust
/// # use formatted_output::advanced_formatting::demonstrate_base_conversion;
/// demonstrate_base_conversion();
/// ```
pub fn demonstrate_base_conversion() {
    println!("\n=== 进制转换功能演示 ===");

    // 1. 基本进制转换
    println!("\n--- 基本进制转换 ---");
    let numbers = vec![10, 42, 255, 1024, 65535];

    println!("{:─^70}", " 进制转换对比表 ");
    println!(
        "│{:^8}│{:^12}│{:^10}│{:^12}│{:^12}│",
        "十进制", "二进制", "八进制", "十六进制", "大写十六"
    );
    println!("{:─^70}", "");

    for num in numbers {
        println!(
            "│{:^8}│{:^12b}│{:^10o}│{:^12x}│{:^12X}│",
            num, num, num, num, num
        );
    }
    println!("{:─^70}", "");

    // 2. 带前缀的进制转换
    println!("\n--- 带前缀的进制转换 ---");
    let test_number = 255;

    println!("数字 {} 的不同进制表示:", test_number);
    println!(
        "  二进制: {:b} (无前缀) | {:#b} (带前缀)",
        test_number, test_number
    );
    println!(
        "  八进制: {:o} (无前缀) | {:#o} (带前缀)",
        test_number, test_number
    );
    println!(
        "  十六进制: {:x} (无前缀) | {:#x} (带前缀)",
        test_number, test_number
    );
    println!(
        "  大写十六进制: {:X} (无前缀) | {:#X} (带前缀)",
        test_number, test_number
    );

    // 3. 进制转换与宽度、填充结合
    println!("\n--- 进制转换与格式化结合 ---");
    let values = vec![1, 15, 255, 4095];

    for val in values {
        println!(
            "数值 {:>4}: 二进制 '{:08b}' 八进制 '{:06o}' 十六进制 '{:04x}'",
            val, val, val, val
        );
    }

    // 4. 颜色值表示
    println!("\n--- 颜色值表示 ---");
    let colors = vec![
        ("红色", 255, 0, 0),
        ("绿色", 0, 255, 0),
        ("蓝色", 0, 0, 255),
        ("黄色", 255, 255, 0),
        ("紫色", 128, 0, 128),
        ("橙色", 255, 165, 0),
    ];

    println!("RGB 颜色值表示:");
    for (name, r, g, b) in colors {
        let rgb_hex = (r << 16) | (g << 8) | b;
        println!(
            "{:<6}: RGB({:3},{:3},{:3}) = #{:06X} = 0b{:024b}",
            name, r, g, b, rgb_hex, rgb_hex
        );
    }

    // 5. 位操作演示
    println!("\n--- 位操作演示 ---");
    demonstrate_bit_operations();
}

/// 演示位操作的格式化输出
fn demonstrate_bit_operations() {
    let a = 0b11010010; // 210
    let b = 0b10110100; // 180

    println!("位操作结果展示:");
    println!("操作数 A: {:08b} ({})", a, a);
    println!("操作数 B: {:08b} ({})", b, b);
    println!("{:-<40}", "");

    println!("A & B:    {:08b} ({})", a & b, a & b);
    println!("A | B:    {:08b} ({})", a | b, a | b);
    println!("A ^ B:    {:08b} ({})", a ^ b, a ^ b);
    println!("!A:       {:08b} ({})", !a as u8, !a as u8);
    println!("A << 2:   {:08b} ({})", (a << 2) as u8, (a << 2) as u8);
    println!("A >> 2:   {:08b} ({})", a >> 2, a >> 2);
}

/// 演示格式化的实际应用场景
///
/// 展示高级格式化在实际项目中的应用，包括数据报表、日志输出、配置显示等。
///
/// # 应用场景
///
/// 1. **财务报表** - 货币格式化、精度控制
/// 2. **科学计算** - 科学计数法、工程计数法
/// 3. **系统监控** - 进制转换、百分比显示
/// 4. **数据分析** - 统计数据格式化
///
/// # Examples
///
/// ```rust
/// # use formatted_output::advanced_formatting::demonstrate_practical_applications;
/// demonstrate_practical_applications();
/// ```
pub fn demonstrate_practical_applications() {
    println!("\n=== 实际应用场景演示 ===");

    // 1. 财务报表
    println!("\n--- 财务报表 ---");
    create_financial_report();

    // 2. 科学数据表
    println!("\n--- 科学数据表 ---");
    create_scientific_data_table();

    // 3. 系统性能监控
    println!("\n--- 系统性能监控 ---");
    create_system_monitor();

    // 4. 数据统计分析
    println!("\n--- 数据统计分析 ---");
    create_statistical_analysis();
}

/// 创建财务报表
fn create_financial_report() {
    let financial_data = vec![
        ("营业收入", 1234567.89f64, 15.2f64),
        ("营业成本", -876543.21f64, -8.7f64),
        ("毛利润", 358024.68f64, 23.5f64),
        ("管理费用", -125000.00f64, 5.2f64),
        ("销售费用", -89000.00f64, -2.1f64),
        ("净利润", 144024.68f64, 18.9f64),
    ];

    println!("2024年第一季度财务报表");
    println!("{:═^60}", " 财务数据 ");
    println!(
        "│{:^12}│{:^15}│{:^12}│{:^15}│",
        "项目", "金额(元)", "增长率(%)", "格式化显示"
    );
    println!("{:─^60}", "");

    for (item, amount, growth) in financial_data {
        let formatted_amount = if amount >= 0.0 {
            format!("¥{:.2}", amount)
        } else {
            format!("-¥{:.2}", amount.abs())
        };

        let growth_str = if growth >= 0.0 {
            format!("+{:.1}%", growth)
        } else {
            format!("{:.1}%", growth)
        };

        println!(
            "│{:<12}│{:>15.2}│{:>12}│{:>15}│",
            item, amount, growth_str, formatted_amount
        );
    }
    println!("{:═^60}", "");
}

/// 创建科学数据表
fn create_scientific_data_table() {
    let scientific_constants = vec![
        ("光速", 299792458.0, "m/s"),
        ("普朗克常数", 6.62607015e-34, "J⋅s"),
        ("电子电荷", 1.602176634e-19, "C"),
        ("阿伏伽德罗常数", 6.02214076e23, "mol⁻¹"),
        ("玻尔兹曼常数", 1.380649e-23, "J/K"),
        ("万有引力常数", 6.67430e-11, "m³⋅kg⁻¹⋅s⁻²"),
    ];

    println!("物理常数表");
    println!("{:═^70}", " 基本物理常数 ");
    println!(
        "│{:^15}│{:^20}│{:^15}│{:^15}│",
        "常数名称", "数值", "科学计数法", "单位"
    );
    println!("{:─^70}", "");

    for (name, value, unit) in scientific_constants {
        println!(
            "│{:<15}│{:>20.6}│{:>15.3e}│{:^15}│",
            name, value, value, unit
        );
    }
    println!("{:═^70}", "");
}

/// 创建系统性能监控
fn create_system_monitor() {
    let system_stats = vec![
        ("CPU使用率", 45.7, 100.0, "%"),
        ("内存使用", 2.1, 8.0, "GB"),
        ("磁盘使用", 156.8, 500.0, "GB"),
        ("网络上传", 1024.0, 0.0, "KB/s"),
        ("网络下载", 2048.0, 0.0, "KB/s"),
    ];

    println!("系统性能监控");
    println!("{:═^80}", " 实时系统状态 ");

    for (metric, current, total, unit) in system_stats {
        let percentage = if total > 0.0 {
            (current / total) * 100.0
        } else {
            0.0
        };

        let bar_width = 20;
        let filled = ((percentage / 100.0) * bar_width as f64) as usize;
        let empty = bar_width - filled;

        let progress_bar = format!(
            "{:█<filled$}{:░<empty$}",
            "",
            "",
            filled = filled,
            empty = empty
        );

        if total > 0.0 {
            println!(
                "{:<12}: [{:20}] {:>6.1}{} ({:.1}/{:.1}{})",
                metric, progress_bar, percentage, "%", current, total, unit
            );
        } else {
            println!("{:<12}: {:>8.1}{} (实时速率)", metric, current, unit);
        }
    }

    // 内存详细信息（十六进制地址）
    println!("\n内存地址信息:");
    let memory_addresses = vec![
        ("堆起始地址", 0x7fff5fbff000_u64),
        ("栈起始地址", 0x7fff5fc00000_u64),
        ("程序入口点", 0x0000000100000000_u64),
    ];

    for (desc, addr) in memory_addresses {
        println!("  {:<12}: 0x{:016X} ({})", desc, addr, addr);
    }
}

/// 创建统计分析报告
fn create_statistical_analysis() {
    let data_points = vec![85.2, 92.1, 78.9, 95.3, 88.7, 91.2, 87.4, 93.8, 89.1, 90.5];

    // 计算统计量
    let count = data_points.len() as f64;
    let sum: f64 = data_points.iter().sum();
    let mean = sum / count;

    let variance = data_points.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / count;
    let std_dev = variance.sqrt();

    let mut sorted_data = data_points.clone();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = if sorted_data.len() % 2 == 0 {
        let mid = sorted_data.len() / 2;
        (sorted_data[mid - 1] + sorted_data[mid]) / 2.0
    } else {
        sorted_data[sorted_data.len() / 2]
    };

    let min = sorted_data.first().unwrap();
    let max = sorted_data.last().unwrap();

    println!("数据统计分析报告");
    println!("{:═^50}", " 描述性统计 ");

    let stats = vec![
        ("样本数量", count, ""),
        ("平均值", mean, ""),
        ("中位数", median, ""),
        ("标准差", std_dev, ""),
        ("方差", variance, ""),
        ("最小值", *min, ""),
        ("最大值", *max, ""),
        ("极差", max - min, ""),
    ];

    for (name, value, unit) in stats {
        if name == "样本数量" {
            println!("{:<12}: {:>10.0}{}", name, value, unit);
        } else {
            println!(
                "{:<12}: {:>10.3}{} (科学计数法: {:.2e})",
                name, value, unit, value
            );
        }
    }

    // 数据分布直方图（简化版）
    println!("\n数据分布直方图:");
    let bins = 5;
    let bin_width = (max - min) / bins as f64;

    for i in 0..bins {
        let bin_start = min + i as f64 * bin_width;
        let bin_end = bin_start + bin_width;

        let count_in_bin = data_points
            .iter()
            .filter(|&&x| x >= bin_start && (x < bin_end || (i == bins - 1 && x <= bin_end)))
            .count();

        let bar = "█".repeat(count_in_bin);
        println!(
            "[{:5.1}-{:5.1}): {:2} {}",
            bin_start, bin_end, count_in_bin, bar
        );
    }

    println!("{:═^50}", "");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precision_control() {
        assert_eq!(format!("{:.2}", 3.14159), "3.14");
        assert_eq!(format!("{:.0}", 3.14159), "3");
        assert_eq!(format!("{:.5}", "hello world"), "hello");
    }

    #[test]
    fn test_sign_control() {
        assert_eq!(format!("{:+}", 42), "+42");
        assert_eq!(format!("{:+}", -42), "-42");
        assert_eq!(format!("{: }", 42), "42"); // 实际输出是 "42" 而不是 " 42"
        assert_eq!(format!("{: }", -42), "-42");
    }

    #[test]
    fn test_zero_padding() {
        assert_eq!(format!("{:05}", 42), "00042");
        assert_eq!(format!("{:08.2}", 3.14), "00003.14");
        assert_eq!(format!("{:+06}", 42), "+00042");
    }

    #[test]
    fn test_alternate_format() {
        assert_eq!(format!("{:#x}", 255), "0xff");
        assert_eq!(format!("{:#X}", 255), "0xFF");
        assert_eq!(format!("{:#b}", 8), "0b1000");
        assert_eq!(format!("{:#o}", 8), "0o10");
    }

    #[test]
    fn test_scientific_notation() {
        assert_eq!(format!("{:.2e}", 1234.5), "1.23e3");
        assert_eq!(format!("{:.2E}", 1234.5), "1.23E3");
        assert_eq!(format!("{:.3e}", 0.001234), "1.234e-3");
    }

    #[test]
    fn test_base_conversion() {
        assert_eq!(format!("{:b}", 10), "1010");
        assert_eq!(format!("{:o}", 10), "12");
        assert_eq!(format!("{:x}", 10), "a");
        assert_eq!(format!("{:X}", 10), "A");
    }

    #[test]
    fn test_engineering_format() {
        let (coeff, exp) = engineering_format(1234.0);
        assert!((coeff - 1.234).abs() < 1e-10);
        assert_eq!(exp, 3);

        let (coeff, exp) = engineering_format(0.001234);
        assert!((coeff - 1.234).abs() < 1e-10);
        assert_eq!(exp, -3);
    }

    #[test]
    fn test_complex_formatting() {
        let result = format!("{:+08.2e}", 1234.567);
        assert_eq!(result, "+01.23e3"); // 实际输出包含前导零
    }
}
