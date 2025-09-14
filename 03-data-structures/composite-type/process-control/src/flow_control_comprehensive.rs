//! # Rust 流程控制全面教程
//!
//! 基于 Rust 语言圣经流程控制章节的全面学习资源
//! 涵盖条件语句、循环语句、模式匹配等所有核心概念

/// # 1. 条件语句 (Conditional Statements)
///
/// Rust 提供了灵活的条件控制结构，包括 if、if-else、if-else if-else
pub fn conditional_statements_demo() {
    println!("=== 1. 条件语句演示 ===");

    // 1.1 基本 if 语句
    println!("\n1.1 基本 if 语句:");
    let number = 6;
    if number % 4 == 0 {
        println!("数字 {} 能被 4 整除", number);
    } else if number % 3 == 0 {
        println!("数字 {} 能被 3 整除", number);
    } else if number % 2 == 0 {
        println!("数字 {} 能被 2 整除", number);
    } else {
        println!("数字 {} 不能被 4、3、2 整除", number);
    }

    // 1.2 if 作为表达式
    println!("\n1.2 if 作为表达式:");
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("if 表达式的值: {}", number);

    // 1.3 复杂条件判断
    println!("\n1.3 复杂条件判断:");
    let age = 25;
    let has_license = true;
    let has_car = false;

    if age >= 18 && has_license {
        if has_car {
            println!("可以开车出行");
        } else {
            println!("可以租车或借车");
        }
    } else {
        println!("不能开车");
    }

    // 1.4 使用 if let 进行模式匹配
    println!("\n1.4 if let 模式匹配:");
    let some_value = Some(3);
    if let Some(x) = some_value {
        println!("解构出的值: {}", x);
    } else {
        println!("没有值");
    }

    // 1.5 多重条件组合
    println!("\n1.5 多重条件组合:");
    let score = 85;
    let grade = if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else if score >= 60 {
        "D"
    } else {
        "F"
    };
    println!("分数 {} 对应等级: {}", score, grade);
}

/// # 2. 循环语句 (Loop Statements)
///
/// Rust 提供三种循环结构：loop、while、for
pub fn loop_statements_demo() {
    println!("\n=== 2. 循环语句演示 ===");

    // 2.1 loop 无限循环
    println!("\n2.1 loop 无限循环:");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2; // loop 可以返回值
        }
        println!("loop 计数: {}", counter);
    };
    println!("loop 返回值: {}", result);

    // 2.2 while 条件循环
    println!("\n2.2 while 条件循环:");
    let mut number = 3;
    while number != 0 {
        println!("倒计时: {}", number);
        number -= 1;
    }
    println!("发射！");

    // 2.3 for 迭代循环
    println!("\n2.3 for 迭代循环:");

    // 遍历数组
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("数组元素: {}", element);
    }

    // 遍历范围
    println!("\n范围遍历:");
    for i in 1..=5 {
        println!("范围值: {}", i);
    }

    // 带索引遍历
    println!("\n带索引遍历:");
    for (index, value) in arr.iter().enumerate() {
        println!("索引 {}: 值 {}", index, value);
    }

    // 2.4 while let 模式匹配循环
    println!("\n2.4 while let 模式匹配循环:");
    let mut stack = vec![1, 2, 3, 4, 5];
    while let Some(top) = stack.pop() {
        println!("弹出元素: {}", top);
    }
}

/// # 3. 循环控制 (Loop Control)
///
/// break 和 continue 语句用于控制循环流程
pub fn loop_control_demo() {
    println!("\n=== 3. 循环控制演示 ===");

    // 3.1 break 跳出循环
    println!("\n3.1 break 跳出循环:");
    for i in 1..=10 {
        if i == 6 {
            println!("遇到 6，跳出循环");
            break;
        }
        println!("当前数字: {}", i);
    }

    // 3.2 continue 跳过当前迭代
    println!("\n3.2 continue 跳过当前迭代:");
    for i in 1..=10 {
        if i % 2 == 0 {
            continue; // 跳过偶数
        }
        println!("奇数: {}", i);
    }

    // 3.3 带标签的循环控制
    println!("\n3.3 带标签的循环控制:");
    'outer: for i in 1..=3 {
        'inner: for j in 1..=3 {
            if i == 2 && j == 2 {
                println!("在 ({}, {}) 处跳出外层循环", i, j);
                break 'outer;
            }
            println!("({}, {})", i, j);
        }
    }

    // 3.4 复杂的循环控制示例
    println!("\n3.4 复杂循环控制示例 - 查找质数:");
    'prime_search: for num in 2..=20 {
        for i in 2..num {
            if num % i == 0 {
                continue 'prime_search; // 不是质数，检查下一个数
            }
        }
        println!("{} 是质数", num);
    }
}

/// # 4. 嵌套循环 (Nested Loops)
///
/// 演示各种嵌套循环的使用场景
pub fn nested_loops_demo() {
    println!("\n=== 4. 嵌套循环演示 ===");

    // 4.1 基本嵌套循环 - 乘法表
    println!("\n4.1 九九乘法表:");
    for i in 1..=9 {
        for j in 1..=i {
            print!("{} × {} = {}\t", j, i, i * j);
        }
        println!();
    }

    // 4.2 矩阵遍历
    println!("\n4.2 矩阵遍历:");
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, &value) in row.iter().enumerate() {
            println!("matrix[{}][{}] = {}", row_idx, col_idx, value);
        }
    }

    // 4.3 带标签的嵌套循环控制
    println!("\n4.3 带标签的嵌套循环 - 查找目标值:");
    let target = 5;
    'search: for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, &value) in row.iter().enumerate() {
            if value == target {
                println!("找到目标值 {} 在位置 ({}, {})", target, row_idx, col_idx);
                break 'search;
            }
        }
    }

    // 4.4 复杂嵌套循环 - 生成图案
    println!("\n4.4 生成星形图案:");
    let size = 5;
    for i in 0..size {
        // 打印空格
        for _ in 0..(size - i - 1) {
            print!(" ");
        }
        // 打印星号
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }
}

/// # 5. 模式匹配与流程控制 (Pattern Matching)
///
/// match 表达式和各种模式匹配技巧
pub fn pattern_matching_demo() {
    println!("\n=== 5. 模式匹配演示 ===");

    // 5.1 基本 match 表达式
    println!("\n5.1 基本 match 表达式:");
    let number = 3;
    match number {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        4 | 5 => println!("四或五"),
        6..=10 => println!("六到十"),
        _ => println!("其他数字"),
    }

    // 5.2 match 与 Option
    println!("\n5.2 match 与 Option:");
    let some_number = Some(42);
    match some_number {
        Some(x) if x > 40 => println!("大于 40 的数字: {}", x),
        Some(x) => println!("数字: {}", x),
        None => println!("没有数字"),
    }

    // 5.3 match 与 Result
    println!("\n5.3 match 与 Result:");
    let result: Result<i32, &str> = Ok(200);
    match result {
        Ok(code) if code == 200 => println!("请求成功"),
        Ok(code) => println!("请求完成，状态码: {}", code),
        Err(error) => println!("请求失败: {}", error),
    }

    // 5.4 解构结构体
    println!("\n5.4 解构结构体:");
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 0, y: 7 };
    match point {
        Point { x: 0, y } => println!("在 Y 轴上，y = {}", y),
        Point { x, y: 0 } => println!("在 X 轴上，x = {}", x),
        Point { x, y } => println!("在其他位置 ({}, {})", x, y),
    }

    // 5.5 解构枚举
    println!("\n5.5 解构枚举:");
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => println!("退出消息"),
        Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
        Message::Write(text) => println!("写入文本: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("改变颜色为 RGB({}, {}, {})", r, g, b);
        }
    }

    // 5.6 match 守卫
    println!("\n5.6 match 守卫:");
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("小于 5: {}", x),
        Some(x) => println!("大于等于 5: {}", x),
        None => println!("没有值"),
    }

    // 5.7 @ 绑定
    println!("\n5.7 @ 绑定:");
    let age = 25;
    match age {
        young @ 0..=17 => println!("年轻人，年龄: {}", young),
        adult @ 18..=64 => println!("成年人，年龄: {}", adult),
        senior @ 65.. => println!("老年人，年龄: {}", senior),
        _ => println!("无效年龄: {}", age),
    }
}

/// # 6. 实际应用案例 (Practical Examples)
///
/// 流程控制在实际编程中的应用
pub fn practical_examples() {
    println!("\n=== 6. 实际应用案例 ===");

    // 6.1 用户输入验证
    println!("\n6.1 用户输入验证:");
    fn validate_age(age: i32) -> String {
        match age {
            0..=0 => "年龄必须大于 0".to_string(),
            1..=17 => "未成年人".to_string(),
            18..=65 => "成年人".to_string(),
            66..=120 => "老年人".to_string(),
            _ => "无效年龄".to_string(),
        }
    }

    let ages = [0, 15, 25, 70, 150];
    for age in ages {
        println!("年龄 {}: {}", age, validate_age(age));
    }

    // 6.2 状态机实现
    println!("\n6.2 状态机实现 - 交通灯:");
    #[derive(Debug, PartialEq)]
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    impl TrafficLight {
        fn next(&self) -> TrafficLight {
            match self {
                TrafficLight::Red => TrafficLight::Green,
                TrafficLight::Yellow => TrafficLight::Red,
                TrafficLight::Green => TrafficLight::Yellow,
            }
        }

        fn duration(&self) -> u32 {
            match self {
                TrafficLight::Red => 30,
                TrafficLight::Yellow => 5,
                TrafficLight::Green => 25,
            }
        }
    }

    let mut light = TrafficLight::Red;
    for cycle in 1..=6 {
        println!(
            "周期 {}: {:?} 灯，持续 {} 秒",
            cycle,
            light,
            light.duration()
        );
        light = light.next();
    }

    // 6.3 数据处理流水线
    println!("\n6.3 数据处理流水线:");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut processed = Vec::new();

    for num in numbers {
        // 多重条件处理
        let result = if num % 2 == 0 {
            if num % 4 == 0 {
                num * 3 // 4的倍数乘以3
            } else {
                num * 2 // 其他偶数乘以2
            }
        } else {
            if num > 5 {
                num + 10 // 大于5的奇数加10
            } else {
                num // 其他奇数保持不变
            }
        };
        processed.push(result);
    }

    println!("处理结果: {:?}", processed);

    // 6.4 错误处理流程
    println!("\n6.4 错误处理流程:");
    fn divide_numbers(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("除数不能为零".to_string())
        } else if a.is_nan() || b.is_nan() {
            Err("输入包含 NaN".to_string())
        } else if a.is_infinite() || b.is_infinite() {
            Err("输入包含无穷大".to_string())
        } else {
            Ok(a / b)
        }
    }

    let test_cases = [
        (10.0, 2.0),
        (5.0, 0.0),
        (f64::NAN, 1.0),
        (f64::INFINITY, 1.0),
    ];

    for (a, b) in test_cases {
        match divide_numbers(a, b) {
            Ok(result) => println!("{} ÷ {} = {}", a, b, result),
            Err(error) => println!("{} ÷ {} 错误: {}", a, b, error),
        }
    }
}

/// # 7. 性能优化技巧 (Performance Tips)
///
/// 流程控制的性能优化建议
pub fn performance_tips() {
    println!("\n=== 7. 性能优化技巧 ===");

    // 7.1 避免不必要的分支
    println!("\n7.1 分支优化示例:");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 低效的方式
    let mut sum_even_inefficient = 0;
    for num in &numbers {
        if num % 2 == 0 {
            sum_even_inefficient += num;
        }
    }

    // 高效的方式 - 使用迭代器
    let sum_even_efficient: i32 = numbers.iter().filter(|&&x| x % 2 == 0).sum();

    println!("低效方式结果: {}", sum_even_inefficient);
    println!("高效方式结果: {}", sum_even_efficient);

    // 7.2 循环展开示例
    println!("\n7.2 循环展开概念:");
    let data = [1, 2, 3, 4, 5, 6, 7, 8];

    // 普通循环
    let mut sum1 = 0;
    for &item in &data {
        sum1 += item;
    }

    // 手动展开（仅作演示，实际中编译器会优化）
    let mut sum2 = 0;
    let mut i = 0;
    while i < data.len() {
        if i + 3 < data.len() {
            sum2 += data[i] + data[i + 1] + data[i + 2] + data[i + 3];
            i += 4;
        } else {
            sum2 += data[i];
            i += 1;
        }
    }

    println!("普通循环结果: {}", sum1);
    println!("展开循环结果: {}", sum2);

    // 7.3 提前返回优化
    println!("\n7.3 提前返回优化:");
    fn find_first_negative(numbers: &[i32]) -> Option<usize> {
        for (index, &num) in numbers.iter().enumerate() {
            if num < 0 {
                return Some(index); // 提前返回，避免不必要的迭代
            }
        }
        None
    }

    let test_data = [1, 2, 3, -4, 5, 6];
    match find_first_negative(&test_data) {
        Some(index) => println!("第一个负数在索引 {} 处", index),
        None => println!("没有找到负数"),
    }
}

/// 运行所有流程控制示例
pub fn run_all_examples() {
    println!("🚀 Rust 流程控制全面教程");
    println!("{}", "=".repeat(50));

    conditional_statements_demo();
    loop_statements_demo();
    loop_control_demo();
    nested_loops_demo();
    pattern_matching_demo();
    practical_examples();
    performance_tips();

    println!("\n{}", "=".repeat(50));
    println!("✅ 所有流程控制示例运行完成！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conditional_logic() {
        let result = if true { 42 } else { 0 };
        assert_eq!(result, 42);
    }

    #[test]
    fn test_loop_break_value() {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 3 {
                break counter * 10;
            }
        };
        assert_eq!(result, 30);
    }

    #[test]
    fn test_pattern_matching() {
        let value = Some(42);
        let result = match value {
            Some(x) if x > 40 => "大",
            Some(_) => "小",
            None => "无",
        };
        assert_eq!(result, "大");
    }

    #[test]
    fn test_nested_loop_control() {
        let mut found = false;
        'outer: for i in 1..=3 {
            for j in 1..=3 {
                if i == 2 && j == 2 {
                    found = true;
                    break 'outer;
                }
            }
        }
        assert!(found);
    }

    #[test]
    fn test_while_let() {
        let mut stack = vec![1, 2, 3];
        let mut sum = 0;
        while let Some(value) = stack.pop() {
            sum += value;
        }
        assert_eq!(sum, 6);
    }
}
