//! Rust 数值类型全面教程
//! 基于 https://course.rs/basic/base-type/numbers.html 的详细讲解
//! 包含所有知识点和具体案例

#![allow(unused_variables, dead_code)]

fn main() {
  println!("=== Rust 数值类型全面教程 ===");

  // 1. 整数类型基础
  integer_types_basics();

  // 2. 整数字面量表示法
  integer_literals();

  // 3. 浮点数类型
  floating_point_types();

  // 4. 数值运算
  numeric_operations();

  // 5. 类型转换
  type_conversion();

  // 6. 整数溢出处理
  integer_overflow_handling();

  // 7. 浮点数特殊值
  floating_point_special_values();

  // 8. 数值方法
  numeric_methods();

  // 9. 浮点数精度问题
  floating_point_precision_issues();

  // 10. 字节字面量
  byte_literals();

  // 11. 类型推导和注解
  type_inference_and_annotation();

  // 12. 数值类型的范围
  numeric_type_ranges();

  // 13. 性能考虑
  performance_considerations();

  // 14. 实际应用案例
  practical_examples();
}

/// 1. 整数类型基础
fn integer_types_basics() {
  println!("\n=== 1. 整数类型基础 ===");

  // 有符号整数类型
  let i8_val: i8 = -128; // 8位有符号整数，范围：-128 到 127
  let i16_val: i16 = -32768; // 16位有符号整数，范围：-32,768 到 32,767
  let i32_val: i32 = -2147483648; // 32位有符号整数（默认类型）
  let i64_val: i64 = -9223372036854775808; // 64位有符号整数
  let i128_val: i128 = -170141183460469231731687303715884105728; // 128位有符号整数
  let isize_val: isize = -1000; // 平台相关大小（32位或64位）

  // 无符号整数类型
  let u8_val: u8 = 255; // 8位无符号整数，范围：0 到 255
  let u16_val: u16 = 65535; // 16位无符号整数，范围：0 到 65,535
  let u32_val: u32 = 4294967295; // 32位无符号整数
  let u64_val: u64 = 18446744073709551615; // 64位无符号整数
  let u128_val: u128 = 340282366920938463463374607431768211455; // 128位无符号整数
  let usize_val: usize = 1000; // 平台相关大小（32位或64位）

  println!(
    "有符号整数: i8={}, i16={}, i32={}",
    i8_val, i16_val, i32_val
  );
  println!(
    "无符号整数: u8={}, u16={}, u32={}",
    u8_val, u16_val, u32_val
  );
  println!("平台相关: isize={}, usize={}", isize_val, usize_val);

  // 默认类型推导
  let default_int = 42; // 默认为 i32
  let default_float = 3.14; // 默认为 f64
  println!("默认类型: 整数={}, 浮点数={}", default_int, default_float);
}

/// 2. 整数字面量表示法
fn integer_literals() {
  println!("\n=== 2. 整数字面量表示法 ===");

  // 十进制
  let decimal = 98_222; // 使用下划线分隔提高可读性

  // 十六进制
  let hex = 0xff; // 255

  // 八进制
  let octal = 0o77; // 63

  // 二进制
  let binary = 0b1111_0000; // 240

  // 字节字面量（只能表示 u8 类型）
  let byte = b'A'; // 65 (ASCII码)

  // 类型后缀
  let typed_int = 42u32; // 明确指定为 u32 类型
  let typed_int2 = 42_u32; // 使用下划线分隔

  println!("十进制: {}", decimal);
  println!("十六进制: {} (0xff)", hex);
  println!("八进制: {} (0o77)", octal);
  println!("二进制: {} (0b1111_0000)", binary);
  println!("字节: {} (b'A')", byte);
  println!("类型后缀: {}", typed_int);
}

/// 3. 浮点数类型
fn floating_point_types() {
  println!("\n=== 3. 浮点数类型 ===");

  // f32 和 f64
  let f32_val: f32 = 3.14159; // 32位浮点数（单精度）
  let f64_val: f64 = 3.141592653589793; // 64位浮点数（双精度，默认）

  // 科学计数法
  let scientific1 = 1e6; // 1,000,000.0 (f64)
  let scientific2 = 2.5e-4_f32; // 0.00025 (f32)

  println!("f32: {}", f32_val);
  println!("f64: {}", f64_val);
  println!("科学计数法: {}, {}", scientific1, scientific2);

  // 浮点数的特殊性质
  println!("\n浮点数特殊性质:");
  println!("f32 精度: {} 位小数", f32::DIGITS);
  println!("f64 精度: {} 位小数", f64::DIGITS);
  println!("f32 最大值: {}", f32::MAX);
  println!("f64 最大值: {}", f64::MAX);
}

/// 4. 数值运算
fn numeric_operations() {
  println!("\n=== 4. 数值运算 ===");

  let a = 10;
  let b = 3;

  // 基本算术运算
  println!("加法: {} + {} = {}", a, b, a + b);
  println!("减法: {} - {} = {}", a, b, a - b);
  println!("乘法: {} * {} = {}", a, b, a * b);
  println!("除法: {} / {} = {}", a, b, a / b); // 整数除法
  println!("取余: {} % {} = {}", a, b, a % b);

  // 浮点数运算
  let x = 10.0;
  let y = 3.0;
  println!("浮点除法: {} / {} = {}", x, y, x / y);

  // 复合赋值运算符
  let mut sum = 5;
  sum += 10; // 等同于 sum = sum + 10
  println!("复合赋值: sum += 10 结果为 {}", sum);

  // 注意：Rust 不支持 ++ 和 -- 运算符
  // sum++; // 编译错误
  sum += 1; // 正确的自增方式
  println!("自增: sum += 1 结果为 {}", sum);
}

/// 5. 类型转换
fn type_conversion() {
  println!("\n=== 5. 类型转换 ===");

  // 使用 as 进行显式转换
  let a = 3.1_f32;
  let b = 100_i8;
  let c = 'A';

  println!("f32 转 i8: {} as i8 = {}", a, a as i8);
  println!("i8 转 i32: {} as i32 = {}", b, b as i32);
  println!("char 转 u8: '{}' as u8 = {}", c, c as u8); // ASCII 码

  // 类型转换的注意事项
  let large_num = 300_i32;
  let truncated = large_num as i8; // 会发生截断
  println!("截断示例: {} as i8 = {} (发生截断)", large_num, truncated);

  // 安全的类型转换
  use std::convert::TryInto;

  let result: Result<i8, _> = large_num.try_into();
  match result {
    Ok(val) => println!("安全转换成功: {}", val),
    Err(_) => println!("安全转换失败: {} 超出 i8 范围", large_num),
  }

  // 类型转换的最佳实践
  let small = 100_i16;
  let big = 200_i32;

  // 将小类型转换为大类型（安全）
  if (small as i32) < big {
    println!("比较: {} < {}", small, big);
  }
}

/// 6. 整数溢出处理
fn integer_overflow_handling() {
  println!("\n=== 6. 整数溢出处理 ===");

  let i = 100_i8;

  // checked_* 系列：返回 Option，溢出时返回 None
  println!("checked_add: {:?}", i.checked_add(i)); // Some(200)
  println!("checked_add 溢出: {:?}", i.checked_add(50)); // None

  // saturating_* 系列：溢出时返回类型的最大/最小值
  println!("saturating_add: {}", i.saturating_add(i)); // 127 (i8::MAX)
  println!("saturating_add 溢出: {}", i.saturating_add(50)); // 127

  // wrapping_* 系列：溢出时环绕（截断高位）
  println!("wrapping_add: {}", i.wrapping_add(i)); // -56 (环绕)
  println!("wrapping_add 溢出: {}", i.wrapping_add(50)); // -106

  // overflowing_* 系列：返回结果和是否溢出的布尔值
  let (result, overflowed) = i.overflowing_add(i);
  println!("overflowing_add: 结果={}, 是否溢出={}", result, overflowed);

  // 使用 Wrapping 类型进行环绕运算
  use std::num::Wrapping;

  let big = Wrapping(std::u32::MAX);
  let sum = big + Wrapping(2_u32);
  println!("Wrapping 运算: {} + 2 = {}", big.0, sum.0); // 环绕到 1
}

/// 7. 浮点数特殊值
fn floating_point_special_values() {
  println!("\n=== 7. 浮点数特殊值 ===");

  // NaN (Not a Number)
  let nan = f64::NAN;
  let sqrt_negative = (-1.0_f64).sqrt();

  println!("NaN 值: {}", nan);
  println!("负数开方: {}", sqrt_negative);
  println!("NaN 检测: is_nan() = {}", nan.is_nan());

  // 无穷大
  let infinity = f64::INFINITY;
  let neg_infinity = f64::NEG_INFINITY;
  let div_by_zero = 1.0 / 0.0;

  println!("正无穷: {}", infinity);
  println!("负无穷: {}", neg_infinity);
  println!("除零: {}", div_by_zero);
  println!("无穷检测: is_infinite() = {}", infinity.is_infinite());

  // 有限数检测
  println!("有限数检测: is_finite() = {}", 3.14_f64.is_finite());
  println!("正常数检测: is_normal() = {}", 3.14_f64.is_normal());

  // NaN 的特殊性质
  println!("\nNaN 特殊性质:");
  println!("NaN == NaN: {}", nan == nan); // false!
  println!("NaN != NaN: {}", nan != nan); // true!

  // 安全的浮点数比较
  let a = 0.1_f64 + 0.2_f64;
  let b = 0.3_f64;
  let epsilon = f64::EPSILON;

  println!("\n浮点数比较:");
  println!("0.1 + 0.2 = {}", a);
  println!("0.3 = {}", b);
  println!("直接比较: {} == {} = {}", a, b, a == b);
  println!(
    "安全比较: |{} - {}| < {} = {}",
    a,
    b,
    epsilon,
    (a - b).abs() < epsilon
  );
}

/// 8. 数值方法
fn numeric_methods() {
  println!("\n=== 8. 数值方法 ===");

  // 整数方法
  let num = 45_i32;
  println!("数值: {}", num);
  println!("绝对值: {}", num.abs());
  println!("幂运算: {}^2 = {}", num, num.pow(2));
  println!("二进制中1的个数: {}", num.count_ones());
  println!("前导零个数: {}", num.leading_zeros());
  println!("尾随零个数: {}", num.trailing_zeros());

  // 浮点数方法
  let float_num = 13.14_f32;
  println!("\n浮点数: {}", float_num);
  println!("向上取整: {}", float_num.ceil());
  println!("向下取整: {}", float_num.floor());
  println!("四舍五入: {}", float_num.round());
  println!("截断小数: {}", float_num.trunc());
  println!("小数部分: {}", float_num.fract());
  println!("绝对值: {}", float_num.abs());
  println!("平方根: {}", float_num.sqrt());
  println!("立方根: {}", float_num.cbrt());

  // 三角函数
  let angle = std::f64::consts::PI / 4.0; // 45度
  println!("\n三角函数 (π/4):");
  println!("sin: {}", angle.sin());
  println!("cos: {}", angle.cos());
  println!("tan: {}", angle.tan());

  // 对数和指数函数
  let e_val = std::f64::consts::E;
  println!("\n对数和指数:");
  println!("e^2: {}", 2.0_f64.exp());
  println!("ln(e): {}", e_val.ln());
  println!("log10(100): {}", 100.0_f64.log10());
  println!("log2(8): {}", 8.0_f64.log2());
}

/// 9. 浮点数精度问题
fn floating_point_precision_issues() {
  println!("\n=== 9. 浮点数精度问题 ===");

  // 经典的 0.1 + 0.2 问题
  let a = 0.1;
  let b = 0.2;
  let c = 0.3;
  let sum = a + b;

  println!("0.1 + 0.2 = {}", sum);
  println!("0.3 = {}", c);
  println!("0.1 + 0.2 == 0.3: {}", sum == c);

  // 显示二进制表示
  let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
  let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

  println!("\nf32 二进制表示:");
  println!("0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
  println!("0.3:       {:x}", abc.2.to_bits());
  println!("相等: {}", abc.0 + abc.1 == abc.2);

  println!("\nf64 二进制表示:");
  println!("0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
  println!("0.3:       {:x}", xyz.2.to_bits());
  println!("相等: {}", xyz.0 + xyz.1 == xyz.2);

  // 安全的浮点数比较方法
  fn float_equal(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
  }

  let epsilon = 1e-10;
  println!("\n安全比较 (epsilon = {}):", epsilon);
  println!(
    "float_equal(0.1 + 0.2, 0.3): {}",
    float_equal(0.1 + 0.2, 0.3, epsilon)
  );

  // 使用 f64::EPSILON
  println!(
    "使用 f64::EPSILON: {}",
    (0.1_f64 + 0.2_f64 - 0.3_f64).abs() < f64::EPSILON * 4.0
  );
}

/// 10. 字节字面量
fn byte_literals() {
  println!("\n=== 10. 字节字面量 ===");

  // 字节字面量
  let byte_a = b'A'; // 65 (ASCII)
  let byte_newline = b'\n'; // 10
  let byte_quote = b'\''; // 39
  let byte_backslash = b'\\'; // 92
  let byte_hex = b'\x1b'; // 27 (ESC)

  println!("字节 'A': {}", byte_a);
  println!("字节 '\\n': {}", byte_newline);
  println!("字节 '\\'': {}", byte_quote);
  println!("字节 '\\\\': {}", byte_backslash);
  println!("字节 '\\x1b': {}", byte_hex);

  // 字节字符串
  let byte_string = b"Hello";
  println!("字节字符串: {:?}", byte_string);

  // 字节字面量的类型
  let a: u8 = b'A';
  let b = a - 65;
  println!("字节运算: {} - 65 = {}", a, b);
}

/// 11. 类型推导和注解
fn type_inference_and_annotation() {
  println!("\n=== 11. 类型推导和注解 ===");

  // 类型推导
  let inferred_int = 42; // 推导为 i32
  let inferred_float = 3.14; // 推导为 f64

  // 显式类型注解
  let explicit_int: i64 = 42;
  let explicit_float: f32 = 3.14;

  // 通过使用推导类型
  let mut vec = Vec::new(); // 此时类型未知
  vec.push(42); // 现在推导为 Vec<i32>

  // 类型后缀
  let suffix_int = 42u64;
  let suffix_float = 3.14f32;

  println!("推导类型: int={}, float={}", inferred_int, inferred_float);
  println!("显式类型: int={}, float={}", explicit_int, explicit_float);
  println!("后缀类型: int={}, float={}", suffix_int, suffix_float);

  // 需要类型注解的情况
  let parsed: i32 = "42".parse().expect("解析失败");
  // 或者
  let parsed2 = "42".parse::<i32>().expect("解析失败");

  println!("解析结果: {}, {}", parsed, parsed2);
}

/// 12. 数值类型的范围
fn numeric_type_ranges() {
  println!("\n=== 12. 数值类型的范围 ===");

  // 整数类型范围
  println!("整数类型范围:");
  println!("i8:   {} 到 {}", i8::MIN, i8::MAX);
  println!("i16:  {} 到 {}", i16::MIN, i16::MAX);
  println!("i32:  {} 到 {}", i32::MIN, i32::MAX);
  println!("i64:  {} 到 {}", i64::MIN, i64::MAX);
  println!("i128: {} 到 {}", i128::MIN, i128::MAX);

  println!("\nu8:   {} 到 {}", u8::MIN, u8::MAX);
  println!("u16:  {} 到 {}", u16::MIN, u16::MAX);
  println!("u32:  {} 到 {}", u32::MIN, u32::MAX);
  println!("u64:  {} 到 {}", u64::MIN, u64::MAX);
  println!("u128: {} 到 {}", u128::MIN, u128::MAX);

  // 浮点数类型范围
  println!("\n浮点数类型范围:");
  println!("f32: {} 到 {}", f32::MIN, f32::MAX);
  println!("f64: {} 到 {}", f64::MIN, f64::MAX);

  // 浮点数精度
  println!("\n浮点数精度:");
  println!("f32 EPSILON: {}", f32::EPSILON);
  println!("f64 EPSILON: {}", f64::EPSILON);
  println!("f32 有效数字: {} 位", f32::DIGITS);
  println!("f64 有效数字: {} 位", f64::DIGITS);
}

/// 13. 性能考虑
fn performance_considerations() {
  println!("\n=== 13. 性能考虑 ===");

  // 类型选择的性能影响
  println!("类型选择建议:");
  println!("- 默认使用 i32 和 f64（现代CPU优化良好）");
  println!("- 内存敏感场景使用较小类型（i8, i16, f32）");
  println!("- 数组索引使用 usize");
  println!("- 需要大范围时使用 i64, u64, i128, u128");

  // 运算性能
  let start = std::time::Instant::now();

  // 整数运算（通常很快）
  let mut sum_int = 0i64;
  for i in 0..1_000_000 {
    sum_int += i;
  }

  let int_duration = start.elapsed();
  println!("\n整数运算耗时: {:?}", int_duration);

  let start = std::time::Instant::now();

  // 浮点数运算（可能稍慢）
  let mut sum_float = 0.0f64;
  for i in 0..1_000_000 {
    sum_float += i as f64;
  }

  let float_duration = start.elapsed();
  println!("浮点数运算耗时: {:?}", float_duration);

  // 避免不必要的类型转换
  println!("\n性能提示:");
  println!("- 避免频繁的类型转换");
  println!("- 在循环中使用合适的类型");
  println!("- 考虑使用 SIMD 指令（在适当情况下）");
}

/// 14. 实际应用案例
fn practical_examples() {
  println!("\n=== 14. 实际应用案例 ===");

  // 案例1：安全的数组索引
  let arr = [1, 2, 3, 4, 5];
  let index = 2usize;

  if index < arr.len() {
    println!("数组元素 arr[{}] = {}", index, arr[index]);
  }

  // 案例2：金融计算（避免浮点数精度问题）
  fn calculate_interest(principal: u64, rate_basis_points: u32, days: u32) -> u64 {
    // 使用整数计算避免浮点数精度问题
    // rate_basis_points: 利率的万分之一（如 250 表示 2.5%）
    principal * rate_basis_points as u64 * days as u64 / (10000 * 365)
  }

  let principal = 100000; // 10万分（以分为单位）
  let rate = 250; // 2.5% 年利率
  let days = 30; // 30天
  let interest = calculate_interest(principal, rate, days);
  println!(
    "利息计算: 本金{}分, 利率{}bp, {}天 = {}分利息",
    principal, rate, days, interest
  );

  // 案例3：颜色值处理
  fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
  }

  let color_hex = rgb_to_hex(255, 128, 64);
  println!("RGB(255, 128, 64) = {}", color_hex);

  // 案例4：文件大小计算
  fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
      size /= 1024.0;
      unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
  }

  let file_sizes = [1024, 1048576, 1073741824, 1099511627776];
  for &size in &file_sizes {
    println!("文件大小: {} 字节 = {}", size, format_file_size(size));
  }

  // 案例5：温度转换
  fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
  }

  fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
  }

  let temp_c = 25.0;
  let temp_f = celsius_to_fahrenheit(temp_c);
  println!("温度转换: {}°C = {:.1}°F", temp_c, temp_f);
  println!(
    "反向转换: {:.1}°F = {:.1}°C",
    temp_f,
    fahrenheit_to_celsius(temp_f)
  );

  // 案例6：位操作示例
  fn set_bit(num: u32, bit_pos: u32) -> u32 {
    num | (1 << bit_pos)
  }

  fn clear_bit(num: u32, bit_pos: u32) -> u32 {
    num & !(1 << bit_pos)
  }

  fn toggle_bit(num: u32, bit_pos: u32) -> u32 {
    num ^ (1 << bit_pos)
  }

  let mut flags = 0b1010u32;
  println!("\n位操作示例:");
  println!("初始值: {:04b}", flags);
  flags = set_bit(flags, 0);
  println!("设置位0: {:04b}", flags);
  flags = clear_bit(flags, 3);
  println!("清除位3: {:04b}", flags);
  flags = toggle_bit(flags, 2);
  println!("切换位2: {:04b}", flags);
}

// 补充：编译时常量和运行时检查
const MAX_USERS: u32 = 1000;
const PI: f64 = 3.141592653589793;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_overflow_handling() {
    let max_i8 = i8::MAX;
    assert_eq!(max_i8.checked_add(1), None);
    assert_eq!(max_i8.saturating_add(1), i8::MAX);
    assert_eq!(max_i8.wrapping_add(1), i8::MIN);
  }

  #[test]
  fn test_float_precision() {
    let a = 0.1_f64 + 0.2_f64;
    let b = 0.3_f64;
    assert!((a - b).abs() < f64::EPSILON * 4.0);
  }

  #[test]
  fn test_type_conversion() {
    let small: i16 = 100;
    let big: i32 = small as i32;
    assert_eq!(big, 100);

    let truncated = 300i32 as i8;
    assert_eq!(truncated, 44); // 300 - 256 = 44
  }
}
