//! # Rust 元组(Tuple)全面学习指南
//!
//! 本文件包含了 Rust 元组的所有核心概念、用法和最佳实践
//! 基于 course.rs 官方教程内容，提供详细的代码示例和精确讲解

/// # 1. 元组基础概念
///
/// 元组是 Rust 中的复合数据类型，可以将多个不同类型的值组合在一起
/// 特点：
/// - 固定长度：一旦声明，长度不能改变
/// - 异构类型：可以包含不同类型的元素
/// - 有序：元素有固定的顺序
/// - 索引访问：通过 .0, .1, .2 等方式访问元素
pub fn tuple_basics() {
    println!("=== 元组基础概念 ===");

    // 1.1 基本语法 - 显式类型注解
    let tuple_with_types: (i32, f64, u8) = (500, 6.4, 1);
    println!("显式类型元组: {:?}", tuple_with_types);

    // 1.2 基本语法 - 类型推导
    let tuple_inferred = (500, 6.4, 1);
    println!("类型推导元组: {:?}", tuple_inferred);

    // 1.3 混合类型元组
    let mixed_tuple = (42, "hello", 3.14, true, 'R');
    println!("混合类型元组: {:?}", mixed_tuple);

    // 1.4 单元素元组（需要逗号）
    let single_element = (42,); // 注意逗号
    println!("单元素元组: {:?}", single_element);

    // 1.5 空元组（单元类型）
    let unit = ();
    println!("空元组/单元类型: {:?}", unit);
}

/// # 2. 元组访问方式
///
/// Rust 提供了多种访问元组元素的方式
pub fn tuple_access() {
    println!("\n=== 元组访问方式 ===");

    let coordinates = (10, 20, 30);

    // 2.1 索引访问（从0开始）
    println!("x坐标: {}", coordinates.0);
    println!("y坐标: {}", coordinates.1);
    println!("z坐标: {}", coordinates.2);

    // 2.2 解构赋值（推荐方式）
    let (x, y, z) = coordinates;
    println!("解构后 - x: {}, y: {}, z: {}", x, y, z);

    // 2.3 部分解构（使用下划线忽略不需要的元素）
    let (first, _, third) = coordinates;
    println!("部分解构 - first: {}, third: {}", first, third);

    // 2.4 嵌套元组访问
    let nested = ((1, 2), (3, 4));
    println!("嵌套元组第一个元组的第二个元素: {}", nested.0 .1);

    // 2.5 复杂嵌套解构
    let ((a, b), (c, d)) = nested;
    println!("嵌套解构 - a: {}, b: {}, c: {}, d: {}", a, b, c, d);
}

/// # 3. 元组的模式匹配
///
/// 元组在模式匹配中的强大应用
pub fn tuple_pattern_matching() {
    println!("\n=== 元组模式匹配 ===");

    // 3.1 基本模式匹配
    let point = (3, 4);
    match point {
        (0, 0) => println!("原点"),
        (0, y) => println!("在y轴上，y = {}", y),
        (x, 0) => println!("在x轴上，x = {}", x),
        (x, y) => println!("点({}, {})", x, y),
    }

    // 3.2 范围匹配
    let score = (85, 'A');
    match score {
        (90..=100, grade) => println!("优秀: {}分，等级{}", score.0, grade),
        (80..=89, grade) => println!("良好: {}分，等级{}", score.0, grade),
        (60..=79, grade) => println!("及格: {}分，等级{}", score.0, grade),
        (_, grade) => println!("不及格，等级{}", grade),
    }

    // 3.3 守卫条件
    let data = (5, 10);
    match data {
        (x, y) if x + y > 10 => println!("和大于10: {} + {} = {}", x, y, x + y),
        (x, y) if x == y => println!("相等: {} = {}", x, y),
        (x, y) => println!("其他情况: ({}, {})", x, y),
    }

    // 3.4 if let 模式匹配
    let maybe_point = Some((1, 2));
    if let Some((x, y)) = maybe_point {
        println!("if let 解构: 点({}, {})", x, y);
    }
}

/// # 4. 元组作为函数参数和返回值
///
/// 元组在函数中的应用场景
pub fn tuple_in_functions() {
    println!("\n=== 元组在函数中的应用 ===");

    // 4.1 元组作为参数
    fn print_point(point: (i32, i32)) {
        println!("点的坐标: ({}, {})", point.0, point.1);
    }

    let point = (5, 10);
    print_point(point);

    // 4.2 元组作为返回值（返回多个值）
    fn calculate_stats(numbers: &[i32]) -> (i32, i32, f64) {
        let sum: i32 = numbers.iter().sum();
        let max = *numbers.iter().max().unwrap_or(&0);
        let avg = if numbers.is_empty() {
            0.0
        } else {
            sum as f64 / numbers.len() as f64
        };
        (sum, max, avg)
    }

    let data = [1, 5, 3, 9, 2];
    let (sum, max, avg) = calculate_stats(&data);
    println!(
        "统计结果 - 总和: {}, 最大值: {}, 平均值: {:.2}",
        sum, max, avg
    );

    // 4.3 复杂函数示例：坐标变换
    fn transform_point(point: (f64, f64), scale: f64, offset: (f64, f64)) -> (f64, f64) {
        let (x, y) = point;
        let (dx, dy) = offset;
        (x * scale + dx, y * scale + dy)
    }

    let original = (2.0, 3.0);
    let transformed = transform_point(original, 2.0, (1.0, 1.0));
    println!("坐标变换: {:?} -> {:?}", original, transformed);
}

/// # 5. 嵌套元组和复杂结构
///
/// 处理复杂的嵌套元组结构
pub fn nested_tuples() {
    println!("\n=== 嵌套元组和复杂结构 ===");

    // 5.1 二维坐标系统
    type Point2D = (f64, f64);
    type Line = (Point2D, Point2D);
    type Triangle = (Point2D, Point2D, Point2D);

    let triangle: Triangle = ((0.0, 0.0), (3.0, 0.0), (1.5, 2.6));
    println!("三角形顶点: {:?}", triangle);

    // 5.2 解构复杂嵌套结构
    let ((x1, y1), (x2, y2), (x3, y3)) = triangle;
    println!(
        "顶点坐标: A({}, {}), B({}, {}), C({}, {})",
        x1, y1, x2, y2, x3, y3
    );

    // 5.3 三维嵌套
    let matrix_3d = (((1, 2), (3, 4)), ((5, 6), (7, 8)));
    println!("3D矩阵: {:?}", matrix_3d);

    // 访问深层嵌套元素
    println!("矩阵[0][1][0] = {}", matrix_3d.0 .1 .0);

    // 5.4 混合类型嵌套
    let complex_data = (
        "用户信息",
        ("张三", 25),
        ("联系方式", ("email@example.com", "13800138000")),
        ("地址", ("北京市", "朝阳区", 100000)),
    );

    let (
        title,
        (name, age),
        (contact_label, (email, phone)),
        (addr_label, (city, district, zipcode)),
    ) = complex_data;
    println!("{}:", title);
    println!("  姓名: {}, 年龄: {}", name, age);
    println!("  {}: 邮箱={}, 电话={}", contact_label, email, phone);
    println!("  {}: {}{}，邮编{}", addr_label, city, district, zipcode);
}

/// # 6. 元组的实用工具函数
///
/// 常用的元组操作工具函数
pub fn tuple_utilities() {
    println!("\n=== 元组实用工具函数 ===");

    // 6.1 交换元组元素
    fn swap<T, U>(tuple: (T, U)) -> (U, T) {
        let (a, b) = tuple;
        (b, a)
    }

    let original = ("hello", 42);
    let swapped = swap(original);
    println!("交换前: {:?}, 交换后: {:?}", ("hello", 42), swapped);

    // 6.2 元组映射
    fn map_tuple<T, U, F1, F2>(tuple: (T, T), f1: F1, f2: F2) -> (U, U)
    where
        F1: Fn(T) -> U,
        F2: Fn(T) -> U,
        T: Copy,
    {
        (f1(tuple.0), f2(tuple.1))
    }

    let numbers = (3, 4);
    let results = map_tuple(numbers, |x| x * x, |x| x * 2);
    println!("映射结果: {:?} -> {:?}", numbers, results);

    // 6.3 元组比较
    fn compare_tuples(t1: (i32, i32), t2: (i32, i32)) -> std::cmp::Ordering {
        match t1.0.cmp(&t2.0) {
            std::cmp::Ordering::Equal => t1.1.cmp(&t2.1),
            other => other,
        }
    }

    let tuple1 = (1, 5);
    let tuple2 = (1, 3);
    println!(
        "比较 {:?} 和 {:?}: {:?}",
        tuple1,
        tuple2,
        compare_tuples(tuple1, tuple2)
    );

    // 6.4 元组距离计算
    fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
        let (x1, y1) = p1;
        let (x2, y2) = p2;
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }

    let point1 = (0.0, 0.0);
    let point2 = (3.0, 4.0);
    println!(
        "点 {:?} 到 {:?} 的距离: {:.2}",
        point1,
        point2,
        distance(point1, point2)
    );
}

/// # 7. 元组的限制和注意事项
///
/// 了解元组的限制和使用注意事项
pub fn tuple_limitations() {
    println!("\n=== 元组的限制和注意事项 ===");

    // 7.1 长度限制 - 过长的元组无法打印
    // 注意：超过12个元素的元组无法使用 Debug trait
    let long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("12元素元组: {:?}", long_tuple);

    // 以下代码会编译错误，因为13个元素的元组没有实现 Debug
    // let too_long = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("{:?}", too_long); // 编译错误！

    // 7.2 类型安全性
    let _coordinates = (10, 20);
    // 以下代码会编译错误，因为类型不匹配
    // let (x, y, z) = coordinates; // 编译错误：元组只有2个元素

    // 7.3 可变性
    let mut mutable_tuple = (1, 2, 3);
    // 不能直接修改元组的元素
    // mutable_tuple.0 = 10; // 编译错误！

    // 但可以重新赋值整个元组
    mutable_tuple = (10, 20, 30);
    println!("可变元组重新赋值后: {:?}", mutable_tuple);
    println!("重新赋值后的元组: {:?}", mutable_tuple);

    // 7.4 所有权和借用
    let tuple_with_string = (String::from("hello"), 42);
    let (s, n) = tuple_with_string;
    // tuple_with_string 的所有权已经移动，不能再使用
    // println!("{:?}", tuple_with_string); // 编译错误！
    println!("解构后: {}, {}", s, n);

    // 使用引用避免所有权移动
    let tuple_with_string2 = (String::from("world"), 84);
    let (ref s2, n2) = tuple_with_string2;
    println!("使用引用: {}, {}", s2, n2);
    println!("原元组仍可用: {:?}", tuple_with_string2);
}

/// # 8. 元组 vs 其他数据结构
///
/// 比较元组与其他数据结构的优缺点
pub fn tuple_vs_others() {
    println!("\n=== 元组 vs 其他数据结构 ===");

    // 8.1 元组 vs 数组
    println!("元组 vs 数组:");
    let tuple_mixed = (1, "hello", 3.14); // 不同类型
    let array_same = [1, 2, 3]; // 相同类型
    println!("  元组（混合类型）: {:?}", tuple_mixed);
    println!("  数组（相同类型）: {:?}", array_same);

    // 8.2 元组 vs 结构体
    println!("\n元组 vs 结构体:");

    // 元组：简洁但缺乏语义
    let point_tuple = (10, 20);
    println!("  元组表示点: {:?} (缺乏语义)", point_tuple);

    // 结构体：更清晰的语义
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point_struct = Point { x: 10, y: 20 };
    println!("  结构体表示点: {:?} (语义清晰)", point_struct);

    // 8.3 元组结构体：两者的结合
    #[derive(Debug)]
    struct Color(u8, u8, u8); // RGB颜色

    let red = Color(255, 0, 0);
    println!("  元组结构体: {:?}", red);
    println!("  访问元素: R={}, G={}, B={}", red.0, red.1, red.2);
}

/// # 9. 元组的高级应用场景
///
/// 实际开发中元组的高级应用
pub fn advanced_tuple_usage() {
    println!("\n=== 元组的高级应用场景 ===");

    // 9.1 错误处理中的应用
    fn divide_with_remainder(dividend: i32, divisor: i32) -> Result<(i32, i32), &'static str> {
        if divisor == 0 {
            Err("除数不能为零")
        } else {
            Ok((dividend / divisor, dividend % divisor))
        }
    }

    match divide_with_remainder(17, 5) {
        Ok((quotient, remainder)) => {
            println!("17 ÷ 5 = {} 余 {}", quotient, remainder);
        }
        Err(e) => println!("错误: {}", e),
    }

    // 9.2 迭代器中的应用
    let data = vec![("Alice", 25), ("Bob", 30), ("Charlie", 35)];

    // 过滤和映射
    let adults: Vec<_> = data
        .iter()
        .filter(|(_, age)| *age >= 30)
        .map(|(name, age)| format!("{} ({}岁)", name, age))
        .collect();

    println!("30岁以上的人: {:?}", adults);

    // 9.3 配置和选项
    type DatabaseConfig = (String, u16, String, String); // (host, port, username, password)

    fn connect_database(config: DatabaseConfig) -> Result<String, String> {
        let (host, port, username, _password) = config;
        // 模拟连接
        Ok(format!("已连接到 {}:{} 用户: {}", host, port, username))
    }

    let db_config = (
        "localhost".to_string(),
        5432,
        "admin".to_string(),
        "secret".to_string(),
    );
    match connect_database(db_config) {
        Ok(msg) => println!("数据库连接: {}", msg),
        Err(e) => println!("连接失败: {}", e),
    }

    // 9.4 状态机实现
    #[derive(Debug, PartialEq, Clone)]
    enum State {
        Start,
        Processing,
        End,
    }

    #[derive(Debug, Clone)]
    enum Event {
        Begin,
        Process,
        Finish,
    }

    fn state_transition(current: State, event: Event) -> (State, String) {
        match (current, event) {
            (State::Start, Event::Begin) => (State::Processing, "开始处理".to_string()),
            (State::Processing, Event::Process) => (State::Processing, "继续处理".to_string()),
            (State::Processing, Event::Finish) => (State::End, "处理完成".to_string()),
            (state, event) => (
                state.clone(),
                format!("无效转换: {:?} -> {:?}", state, event),
            ),
        }
    }

    let mut current_state = State::Start;
    let events = vec![Event::Begin, Event::Process, Event::Finish];

    for event in events {
        let (new_state, message) = state_transition(current_state, event);
        println!("状态转换: {:?} -> {}", new_state, message);
        current_state = new_state;
    }
}

/// # 10. 元组的性能考虑
///
/// 元组的内存布局和性能特点
pub fn tuple_performance() {
    println!("\n=== 元组的性能考虑 ===");

    // 10.1 内存布局
    println!("内存大小比较:");
    println!("  (i32, i32): {} bytes", std::mem::size_of::<(i32, i32)>());
    println!("  (i32, i64): {} bytes", std::mem::size_of::<(i32, i64)>());
    println!(
        "  (i32, u8, i64): {} bytes",
        std::mem::size_of::<(i32, u8, i64)>()
    );
    println!(
        "  (u8, i32, i64): {} bytes",
        std::mem::size_of::<(u8, i32, i64)>()
    );

    // 10.2 对齐和填充
    println!("\n内存对齐影响:");
    #[repr(C)]
    struct StructVersion {
        a: u8,
        b: i32,
        c: i64,
    }

    println!(
        "  结构体版本: {} bytes",
        std::mem::size_of::<StructVersion>()
    );
    println!(
        "  元组版本: {} bytes",
        std::mem::size_of::<(u8, i32, i64)>()
    );

    // 10.3 零成本抽象
    let tuple_data = (1, 2, 3, 4, 5);
    let sum1 = tuple_data.0 + tuple_data.1 + tuple_data.2 + tuple_data.3 + tuple_data.4;

    let (a, b, c, d, e) = tuple_data;
    let sum2 = a + b + c + d + e;

    println!("\n零成本抽象验证:");
    println!("  直接访问求和: {}", sum1);
    println!("  解构后求和: {}", sum2);
    println!("  两种方式性能相同（零成本抽象）");
}

/// # 11. 元组的最佳实践
///
/// 使用元组的最佳实践和建议
pub fn tuple_best_practices() {
    println!("\n=== 元组的最佳实践 ===");

    println!("1. 适用场景:");
    println!("   ✓ 临时组合少量相关数据");
    println!("   ✓ 函数返回多个值");
    println!("   ✓ 模式匹配和解构");
    println!("   ✓ 简单的坐标或配对数据");

    println!("\n2. 避免使用场景:");
    println!("   ✗ 超过3-4个元素的复杂数据");
    println!("   ✗ 需要清晰语义的业务数据");
    println!("   ✗ 频繁访问特定字段的数据");
    println!("   ✗ 需要添加方法的数据结构");

    println!("\n3. 命名建议:");

    // 好的做法：使用类型别名增加可读性
    type Point2D = (f64, f64);
    type RGB = (u8, u8, u8);
    type UserInfo = (String, u32, String); // (name, age, email)

    let point: Point2D = (3.0, 4.0);
    let color: RGB = (255, 128, 0);
    let user: UserInfo = ("Alice".to_string(), 25, "alice@example.com".to_string());

    println!("   类型别名示例:");
    println!("     Point2D: {:?}", point);
    println!("     RGB: {:?}", color);
    println!("     UserInfo: {:?}", user);

    println!("\n4. 解构模式:");

    // 推荐：立即解构以提高可读性
    let user_data = ("Bob".to_string(), 30, "Engineer".to_string());
    let (name, age, profession) = user_data;
    println!("   解构后使用: {} 是一位 {} 岁的 {}", name, age, profession);

    // 避免：频繁使用索引访问
    let _user_data2 = ("Charlie".to_string(), 35, "Designer".to_string());
    // 不推荐：println!("{} 是一位 {} 岁的 {}", user_data2.0, user_data2.1, user_data2.2);

    println!("\n5. 错误处理模式:");

    // 好的模式：结合 Result 使用
    fn parse_coordinate(input: &str) -> Result<(f64, f64), &'static str> {
        let parts: Vec<&str> = input.split(',').collect();
        if parts.len() != 2 {
            return Err("格式错误：需要两个坐标值");
        }

        let x = parts[0]
            .trim()
            .parse::<f64>()
            .map_err(|_| "X坐标解析失败")?;
        let y = parts[1]
            .trim()
            .parse::<f64>()
            .map_err(|_| "Y坐标解析失败")?;

        Ok((x, y))
    }

    match parse_coordinate("3.14, 2.71") {
        Ok((x, y)) => println!("   解析成功: 坐标({}, {})", x, y),
        Err(e) => println!("   解析失败: {}", e),
    }
}

/// # 12. 元组相关的常见错误和解决方案
///
/// 总结使用元组时的常见错误和解决方法
pub fn common_tuple_errors() {
    println!("\n=== 常见错误和解决方案 ===");

    println!("1. 单元素元组忘记逗号:");
    // 错误：let not_tuple = (42);     // 这只是一个带括号的整数
    let correct_tuple = (42,); // 正确的单元素元组
    println!("   正确的单元素元组: {:?}", correct_tuple);

    println!("\n2. 解构时元素数量不匹配:");
    let data = (1, 2, 3);
    // 错误：let (a, b) = data;        // 编译错误：元素数量不匹配
    let (a, b, c) = data; // 正确
    let (x, _, z) = data; // 或者使用下划线忽略
    println!("   正确解构: a={}, b={}, c={}", a, b, c);
    println!("   部分解构: x={}, z={}", x, z);

    println!("\n3. 过长元组的调试问题:");
    let long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("   12元素元组可以打印: {:?}", long_tuple);
    println!("   超过12元素的元组无法使用 {{:?}} 打印");

    println!("\n4. 所有权移动问题:");
    let tuple_with_string = (String::from("hello"), 42);

    // 解决方案1：使用引用
    let (ref s1, n1) = tuple_with_string;
    println!("   使用引用: {}, {}", s1, n1);
    println!("   原元组仍可用: {:?}", tuple_with_string);

    // 解决方案2：克隆
    let tuple_clone = tuple_with_string.clone();
    let (s2, n2) = tuple_clone;
    println!("   克隆后解构: {}, {}", s2, n2);

    println!("\n5. 类型推导问题:");
    // 有时需要显式类型注解
    let mixed: (i32, &str, f64) = (42, "hello", 3.14);
    println!("   显式类型注解: {:?}", mixed);

    println!("\n6. 嵌套访问的可读性问题:");
    let nested = ((1, 2), (3, (4, 5)));
    // 不推荐：深层索引访问
    println!("   深层访问（不推荐）: {}", nested.1 .1 .0);

    // 推荐：逐步解构
    let ((a, b), (c, (d, e))) = nested;
    println!(
        "   逐步解构（推荐）: a={}, b={}, c={}, d={}, e={}",
        a, b, c, d, e
    );
}

/// # 主函数：运行所有示例
pub fn run_all_examples() {
    println!("🦀 Rust 元组(Tuple)全面学习指南");
    println!("=====================================\n");

    tuple_basics();
    tuple_access();
    tuple_pattern_matching();
    tuple_in_functions();
    nested_tuples();
    tuple_utilities();
    tuple_limitations();
    tuple_vs_others();
    advanced_tuple_usage();
    tuple_performance();
    tuple_best_practices();
    common_tuple_errors();

    println!("\n=====================================");
    println!("🎉 元组学习指南完成！");
    println!("\n📚 关键要点总结:");
    println!("• 元组是固定长度的异构数据类型");
    println!("• 使用 .0, .1, .2 索引访问或解构赋值");
    println!("• 适合临时组合数据和函数返回多值");
    println!("• 超过3-4个元素时考虑使用结构体");
    println!("• 注意所有权移动和借用规则");
    println!("• 善用模式匹配和类型别名提高可读性");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_basics() {
        let tuple: (i32, f64, &str) = (42, 3.14, "hello");
        assert_eq!(tuple.0, 42);
        assert_eq!(tuple.1, 3.14);
        assert_eq!(tuple.2, "hello");
    }

    #[test]
    fn test_tuple_destructuring() {
        let tuple = (1, 2, 3);
        let (a, b, c) = tuple;
        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(c, 3);
    }

    #[test]
    fn test_nested_tuples() {
        let nested = ((1, 2), (3, 4));
        assert_eq!(nested.0 .0, 1);
        assert_eq!(nested.1 .1, 4);

        let ((a, b), (c, d)) = nested;
        assert_eq!(a + b + c + d, 10);
    }

    #[test]
    fn test_tuple_as_return_value() {
        fn get_name_age() -> (&'static str, u32) {
            ("Alice", 25)
        }

        let (name, age) = get_name_age();
        assert_eq!(name, "Alice");
        assert_eq!(age, 25);
    }

    #[test]
    fn test_tuple_pattern_matching() {
        let point = (0, 5);
        let result = match point {
            (0, 0) => "origin",
            (0, _) => "y-axis",
            (_, 0) => "x-axis",
            _ => "other",
        };
        assert_eq!(result, "y-axis");
    }
}
