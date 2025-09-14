/*
=== Rust 全模式列表深度分析 ===

本文档基于 https://course.rs/basic/match-pattern/all-patterns.html 内容，
提供 Rust 中所有模式匹配类型的全面分析和实际应用案例。

模式匹配是 Rust 最强大的特性之一，它允许我们以声明式的方式处理数据结构，
提供类型安全、性能优异且表达力强的代码编写方式。

目录：
1. 字面量模式 (Literal Patterns)
2. 变量模式 (Variable Patterns)
3. 通配符模式 (Wildcard Patterns)
4. 范围模式 (Range Patterns)
5. 解构模式 (Destructuring Patterns)
6. 引用模式 (Reference Patterns)
7. 守卫模式 (Guard Patterns)
8. 绑定模式 (Binding Patterns)
9. 多重模式 (Multiple Patterns)
10. 忽略模式 (Ignoring Patterns)
11. 嵌套模式 (Nested Patterns)
12. 高级模式技巧
13. 性能优化和最佳实践
14. 实际应用场景
*/

fn main() {
    println!("=== Rust 全模式列表深度分析 ===");

    // 1. 字面量模式演示
    literal_patterns_demo();

    // 2. 变量模式演示
    variable_patterns_demo();

    // 3. 通配符模式演示
    wildcard_patterns_demo();

    // 4. 范围模式演示
    range_patterns_demo();

    // 5. 解构模式演示
    destructuring_patterns_demo();

    // 6. 引用模式演示
    reference_patterns_demo();

    // 7. 守卫模式演示
    guard_patterns_demo();

    // 8. 绑定模式演示
    binding_patterns_demo();

    // 9. 多重模式演示
    multiple_patterns_demo();

    // 10. 忽略模式演示
    ignoring_patterns_demo();

    // 11. 嵌套模式演示
    nested_patterns_demo();

    // 12. 高级模式技巧演示
    advanced_patterns_demo();

    // 13. 性能优化演示
    performance_optimization_demo();

    // 14. 实际应用场景演示
    real_world_applications_demo();

    println!("\n=== 全模式列表分析完成 ===");
}

// 1. 字面量模式 (Literal Patterns)
// 直接匹配具体的值，包括数字、字符、字符串、布尔值等
fn literal_patterns_demo() {
    println!("\n--- 字面量模式演示 ---");

    // 数字字面量
    let number = 42;
    match number {
        0 => println!("零"),
        1 => println!("一"),
        42 => println!("生命、宇宙以及一切的答案"),
        _ => println!("其他数字: {}", number),
    }

    // 字符字面量
    let character = 'A';
    match character {
        'A' => println!("字母 A"),
        'B' => println!("字母 B"),
        'a'..='z' => println!("小写字母: {}", character),
        'A'..='Z' => println!("大写字母: {}", character),
        _ => println!("其他字符: {}", character),
    }

    // 字符串字面量
    let text = "hello";
    match text {
        "hello" => println!("问候语"),
        "world" => println!("世界"),
        "rust" => println!("Rust 语言"),
        _ => println!("其他文本: {}", text),
    }

    // 布尔字面量
    let flag = true;
    match flag {
        true => println!("真值"),
        false => println!("假值"),
    }

    // 浮点数字面量（需要特别注意精度问题）
    let pi = 3.14f64;
    #[allow(clippy::float_cmp)]
    match pi {
        x if (x - 3.14f64).abs() < f64::EPSILON => println!("π 的近似值"),
        x if (x - 2.71f64).abs() < f64::EPSILON => println!("e 的近似值"),
        _ => println!("其他浮点数: {}", pi),
    }
}

// 2. 变量模式 (Variable Patterns)
// 将匹配的值绑定到变量上，可以在匹配分支中使用
fn variable_patterns_demo() {
    println!("\n--- 变量模式演示 ---");

    // 基本变量绑定
    let value = Some(42);
    match value {
        Some(x) => println!("找到值: {}", x),
        None => println!("没有值"),
    }

    // 在元组中使用变量模式
    let point = (3, 4);
    match point {
        (x, y) => println!(
            "坐标点: ({}, {}), 距离原点: {:.2}",
            x,
            y,
            ((x * x + y * y) as f64).sqrt()
        ),
    }

    // 在结构体中使用变量模式
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    match person {
        Person { name, age } => {
            println!("姓名: {}, 年龄: {}", name, age);
            if age >= 18 {
                println!("{} 是成年人", name);
            } else {
                println!("{} 是未成年人", name);
            }
        }
    }

    // 变量模式的所有权转移
    let opt_string = Some(String::from("Rust"));
    match opt_string {
        Some(s) => {
            println!("字符串: {}", s);
            // s 的所有权已经转移到这里
        }
        None => println!("无字符串"),
    }
    // opt_string 在这里已经不能再使用
}

// 3. 通配符模式 (Wildcard Patterns)
// 使用 _ 匹配任何值但不绑定到变量
fn wildcard_patterns_demo() {
    println!("\n--- 通配符模式演示 ---");

    // 基本通配符使用
    let number = 100;
    match number {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        _ => println!("其他数字"), // 匹配所有其他情况
    }

    // 在元组中使用通配符
    let tuple = (1, 2, 3, 4);
    match tuple {
        (1, _, _, 4) => println!("第一个是1，最后一个是4"),
        (_, 2, 3, _) => println!("中间两个是2和3"),
        _ => println!("其他组合"),
    }

    // 在结构体中使用通配符
    #[derive(Debug)]
    struct Config {
        debug: bool,
        version: String,
        port: u16,
    }

    let config = Config {
        debug: true,
        version: "1.0.0".to_string(),
        port: 8080,
    };

    match config {
        Config { debug: true, .. } => println!("调试模式已启用"),
        Config { port: 80, .. } => println!("使用标准 HTTP 端口"),
        Config { port: 443, .. } => println!("使用标准 HTTPS 端口"),
        _ => println!("其他配置"),
    }

    // 忽略函数返回值
    fn get_result() -> Result<i32, &'static str> {
        Ok(42)
    }

    match get_result() {
        Ok(value) => println!("成功: {}", value),
        Err(_) => println!("发生错误，但我们不关心具体错误"),
    }
}

// 4. 范围模式 (Range Patterns)
// 匹配一个值的范围，使用 ..= 语法
fn range_patterns_demo() {
    println!("\n--- 范围模式演示 ---");

    // 数字范围
    let score = 85;
    let grade = match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        0..=59 => "F",
        _ => "无效分数",
    };
    println!("分数 {} 对应等级: {}", score, grade);

    // 字符范围
    let ch = 'm';
    match ch {
        'a'..='z' => println!("小写字母: {}", ch),
        'A'..='Z' => println!("大写字母: {}", ch),
        '0'..='9' => println!("数字字符: {}", ch),
        _ => println!("其他字符: {}", ch),
    }

    // 年龄分组
    let age = 25;
    let category = match age {
        0..=12 => "儿童",
        13..=19 => "青少年",
        20..=59 => "成年人",
        60..=120 => "老年人",
        _ => "无效年龄",
    };
    println!("年龄 {} 属于: {}", age, category);

    // HTTP 状态码分类
    let status_code = 404;
    let status_type = match status_code {
        100..=199 => "信息响应",
        200..=299 => "成功响应",
        300..=399 => "重定向",
        400..=499 => "客户端错误",
        500..=599 => "服务器错误",
        _ => "未知状态码",
    };
    println!("状态码 {} 类型: {}", status_code, status_type);

    // 温度范围判断
    let temperature = 25;
    match temperature {
        i32::MIN..=-10 => println!("极寒: {}°C", temperature),
        -9..=0 => println!("寒冷: {}°C", temperature),
        1..=15 => println!("凉爽: {}°C", temperature),
        16..=25 => println!("舒适: {}°C", temperature),
        26..=35 => println!("温暖: {}°C", temperature),
        36..=i32::MAX => println!("炎热: {}°C", temperature),
    }
}

// 5. 解构模式 (Destructuring Patterns)
// 解构复合数据类型，如元组、数组、结构体、枚举等
fn destructuring_patterns_demo() {
    println!("\n--- 解构模式演示 ---");

    // 元组解构
    let point_3d = (1, 2, 3);
    match point_3d {
        (x, y, z) => println!("3D 坐标: ({}, {}, {})", x, y, z),
    }

    // 数组解构
    let array = [1, 2, 3, 4, 5];
    match array {
        [first, second, .., last] => {
            println!("第一个: {}, 第二个: {}, 最后一个: {}", first, second, last);
        }
    }

    // 切片解构
    let slice = &[1, 2, 3, 4][..];
    match slice {
        [] => println!("空切片"),
        [x] => println!("单元素切片: [{}]", x),
        [x, y] => println!("双元素切片: [{}, {}]", x, y),
        [first, .., last] => println!("多元素切片，首: {}, 尾: {}", first, last),
    }

    // 结构体解构
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect = Rectangle {
        width: 10,
        height: 20,
    };
    match rect {
        Rectangle { width, height } => {
            println!("矩形尺寸: {}x{}", width, height);
            println!("面积: {}", width * height);
            println!("周长: {}", 2 * (width + height));
        }
    }

    // 部分结构体解构
    #[derive(Debug)]
    struct User {
        id: u32,
        name: String,
        email: String,
        active: bool,
    }

    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        active: true,
    };

    match user {
        User {
            name, active: true, ..
        } => {
            println!("活跃用户: {}", name);
        }
        User {
            name,
            active: false,
            ..
        } => {
            println!("非活跃用户: {}", name);
        }
    }

    // 枚举解构
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write("Hello".to_string()),
        Message::ChangeColor(255, 0, 0),
    ];

    for msg in messages {
        match msg {
            Message::Quit => println!("退出消息"),
            Message::Move { x, y } => println!("移动到坐标: ({}, {})", x, y),
            Message::Write(text) => println!("写入文本: {}", text),
            Message::ChangeColor(r, g, b) => println!("改变颜色为 RGB({}, {}, {})", r, g, b),
        }
    }
}

// 6. 引用模式 (Reference Patterns)
// 处理引用和借用的模式匹配
fn reference_patterns_demo() {
    println!("\n--- 引用模式演示 ---");

    // 基本引用模式
    let x = 5;
    let y = &x;

    match y {
        &val => println!("通过引用获取值: {}", val),
    }

    // Option 引用模式
    let opt = Some(String::from("Rust"));
    match &opt {
        Some(s) => println!("字符串引用: {}", s),
        None => println!("无值"),
    }
    // opt 仍然可用
    println!("opt 仍然拥有: {:?}", opt);

    // 向量引用模式
    let vec = vec![1, 2, 3, 4, 5];
    match vec.as_slice() {
        [] => println!("空向量"),
        [first] => println!("单元素向量: {}", first),
        [first, rest @ ..] => {
            println!("第一个元素: {}", first);
            println!("剩余元素: {:?}", rest);
        }
    }

    // 结构体引用模式
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 0, y: 0 };
    match &point {
        Point { x: 0, y: 0 } => println!("原点"),
        Point { x: 0, y } => println!("在 Y 轴上，y = {}", y),
        Point { x, y: 0 } => println!("在 X 轴上，x = {}", x),
        Point { x, y } => println!("其他点: ({}, {})", x, y),
    }

    // 嵌套引用模式
    let nested = Some(&42);
    match nested {
        Some(&value) => println!("嵌套引用的值: {}", value),
        None => println!("无值"),
    }

    // 可变引用模式
    let mut data = vec![1, 2, 3];
    match &mut data {
        vec if vec.len() > 2 => {
            vec.push(4);
            println!("添加元素后: {:?}", vec);
        }
        vec => println!("短向量: {:?}", vec),
    }
}

// 7. 守卫模式 (Guard Patterns)
// 在模式后添加额外的条件判断
fn guard_patterns_demo() {
    println!("\n--- 守卫模式演示 ---");

    // 基本守卫使用
    let number = 4;
    match number {
        x if x < 0 => println!("负数: {}", x),
        x if x == 0 => println!("零"),
        x if x > 0 && x < 10 => println!("个位正数: {}", x),
        x if x >= 10 && x < 100 => println!("两位数: {}", x),
        _ => println!("大数: {}", number),
    }

    // 在 Option 中使用守卫
    let opt_num = Some(42);
    match opt_num {
        Some(x) if x > 50 => println!("大于50的数: {}", x),
        Some(x) if x > 0 => println!("正数: {}", x),
        Some(0) => println!("零"),
        Some(x) => println!("负数: {}", x), // 处理所有剩余的 Some 情况
        None => println!("无值"),
    }

    // 复杂守卫条件
    #[derive(Debug)]
    struct Student {
        name: String,
        age: u8,
        grade: f32,
    }

    let student = Student {
        name: "Bob".to_string(),
        age: 20,
        grade: 85.5,
    };

    match student {
        Student { grade, age, .. } if grade >= 90.0 && age < 25 => {
            println!("年轻的优秀学生，成绩: {}", grade);
        }
        Student { grade, .. } if grade >= 80.0 => {
            println!("良好学生，成绩: {}", grade);
        }
        Student { grade, .. } if grade >= 60.0 => {
            println!("及格学生，成绩: {}", grade);
        }
        Student { name, grade, .. } => {
            println!("学生 {} 需要努力，当前成绩: {}", name, grade);
        }
    }

    // 外部变量在守卫中的使用
    let threshold = 100;
    let values = vec![50, 150, 75, 200];

    for value in values {
        match value {
            x if x > threshold => println!("{} 超过阈值 {}", x, threshold),
            x => println!("{} 在阈值范围内", x),
        }
    }

    // 函数调用在守卫中
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }

    let num = 8;
    match num {
        x if is_even(x) => println!("{} 是偶数", x),
        x => println!("{} 是奇数", x),
    }
}

// 8. 绑定模式 (Binding Patterns)
// 使用 @ 操作符同时进行模式匹配和变量绑定
fn binding_patterns_demo() {
    println!("\n--- 绑定模式演示 ---");

    // 基本 @ 绑定
    let number = 42;
    match number {
        n @ 1..=50 => println!("小数字: {}", n),
        n @ 51..=100 => println!("中等数字: {}", n),
        n => println!("大数字: {}", n),
    }

    // 在枚举中使用 @ 绑定
    #[derive(Debug)]
    enum Status {
        Active(u32),
        Inactive,
        Pending(String),
    }

    let status = Status::Active(42);
    match status {
        Status::Active(id @ 1..=100) => {
            println!("活跃状态，ID 在正常范围: {}", id);
        }
        Status::Active(id) => {
            println!("活跃状态，ID 超出范围: {}", id);
        }
        Status::Pending(msg @ _) if msg.len() > 10 => {
            println!("长待处理消息: {}", msg);
        }
        Status::Pending(msg) => {
            println!("短待处理消息: {}", msg);
        }
        Status::Inactive => println!("非活跃状态"),
    }

    // 结构体中的 @ 绑定
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 5, y: 10 };
    match point {
        Point {
            x: x_val @ 0..=5,
            y: y_val @ 0..=10,
        } => {
            println!("点在第一象限的小范围内: ({}, {})", x_val, y_val);
        }
        Point { x, y } => {
            println!("其他位置的点: ({}, {})", x, y);
        }
    }

    // 嵌套 @ 绑定
    let nested_data = Some((1, 2, 3));
    match nested_data {
        Some(tuple @ (x, y, z)) if x + y + z > 5 => {
            println!("元组 {:?} 的和大于5", tuple);
        }
        Some((x, y, z)) => {
            println!("元组 ({}, {}, {}) 的和不大于5", x, y, z);
        }
        None => println!("无数据"),
    }

    // 向量中的 @ 绑定
    let vec = vec![1, 2, 3, 4, 5];
    match vec.as_slice() {
        [] => println!("空向量"),
        [single] => println!("单元素向量: {}", single),
        [first, middle @ .., last] if middle.len() > 2 => {
            println!("长向量 - 首: {}, 中间: {:?}, 尾: {}", first, middle, last);
        }
        [first, rest @ ..] => {
            println!("向量 - 首: {}, 其余: {:?}", first, rest);
        }
    }
}

// 9. 多重模式 (Multiple Patterns)
// 使用 | 操作符匹配多个模式
fn multiple_patterns_demo() {
    println!("\n--- 多重模式演示 ---");

    // 基本多重模式
    let number = 3;
    match number {
        1 | 2 | 3 => println!("小数字: {}", number),
        4 | 5 | 6 => println!("中等数字: {}", number),
        7 | 8 | 9 => println!("大数字: {}", number),
        _ => println!("其他数字: {}", number),
    }

    // 字符多重模式
    let character = 'e';
    match character {
        'a' | 'e' | 'i' | 'o' | 'u' => println!("元音字母: {}", character),
        'y' => println!("有时是元音: {}", character),
        _ => println!("辅音字母: {}", character),
    }

    // 枚举多重模式
    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let direction = Direction::North;
    match direction {
        Direction::North | Direction::South => println!("垂直方向: {:?}", direction),
        Direction::East | Direction::West => println!("水平方向: {:?}", direction),
    }

    // HTTP 方法分组
    #[derive(Debug)]
    enum HttpMethod {
        Get,
        Post,
        Put,
        Delete,
        Head,
        Options,
    }

    let method = HttpMethod::Post;
    match method {
        HttpMethod::Get | HttpMethod::Head => println!("只读操作: {:?}", method),
        HttpMethod::Post | HttpMethod::Put => println!("写入操作: {:?}", method),
        HttpMethod::Delete => println!("删除操作: {:?}", method),
        HttpMethod::Options => println!("选项查询: {:?}", method),
    }

    // 结合范围的多重模式
    let score = 85;
    match score {
        90..=100 | 85..=89 => println!("优秀成绩: {}", score),
        70..=84 => println!("良好成绩: {}", score),
        60..=69 => println!("及格成绩: {}", score),
        0..=59 => println!("不及格: {}", score),
        _ => println!("无效分数: {}", score),
    }

    // 复杂多重模式
    #[derive(Debug)]
    enum Event {
        KeyPress(char),
        MouseClick { x: i32, y: i32 },
        Scroll(i32),
    }

    let events = vec![
        Event::KeyPress('a'),
        Event::MouseClick { x: 100, y: 200 },
        Event::KeyPress('q'),
        Event::Scroll(-5),
    ];

    for event in events {
        match event {
            Event::KeyPress('q') | Event::KeyPress('Q') => {
                println!("退出键按下");
            }
            Event::KeyPress(c) if c.is_alphabetic() => {
                println!("字母键按下: {}", c);
            }
            Event::MouseClick { x, y } => {
                println!("鼠标点击事件: 坐标({}, {})", x, y);
            }
            Event::Scroll(delta) => {
                println!("滚动事件: 滚动量{}", delta);
            }
            _ => println!("其他事件: {:?}", event),
        }
    }
}

// 10. 忽略模式 (Ignoring Patterns)
// 使用 _ 和 .. 忽略不需要的值
fn ignoring_patterns_demo() {
    println!("\n--- 忽略模式演示 ---");

    // 忽略元组中的部分值
    let tuple = (1, 2, 3, 4, 5);
    match tuple {
        (first, _, _, _, last) => {
            println!("只关心第一个和最后一个: {} 和 {}", first, last);
        }
    }

    // 使用 .. 忽略剩余值
    let tuple_long = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    match tuple_long {
        (first, second, ..) => {
            println!("前两个值: {} 和 {}", first, second);
        }
    }

    match tuple_long {
        (.., second_last, last) => {
            println!("后两个值: {} 和 {}", second_last, last);
        }
    }

    match tuple_long {
        (first, .., last) => {
            println!("首尾值: {} 和 {}", first, last);
        }
    }

    // 结构体中忽略字段
    #[derive(Debug)]
    struct Config {
        name: String,
        version: String,
        debug: bool,
        port: u16,
        host: String,
    }

    let config = Config {
        name: "MyApp".to_string(),
        version: "1.0.0".to_string(),
        debug: true,
        port: 8080,
        host: "localhost".to_string(),
    };

    match config {
        Config { name, debug, .. } => {
            println!("应用: {}, 调试模式: {}", name, debug);
        }
    }

    // 函数参数中忽略
    fn process_point((x, _): (i32, i32)) -> i32 {
        x * 2 // 只使用 x 坐标
    }

    let point = (10, 20);
    let result = process_point(point);
    println!("处理点 {:?} 的结果: {}", point, result);

    // 忽略 Result 的错误
    fn might_fail() -> Result<i32, &'static str> {
        Ok(42)
    }

    match might_fail() {
        Ok(value) => println!("成功值: {}", value),
        Err(_) => println!("发生了某种错误"),
    }

    // 忽略向量中的元素
    let vec = vec![1, 2, 3, 4, 5, 6];
    match vec.as_slice() {
        [first, _, third, .., last] => {
            println!("第1个: {}, 第3个: {}, 最后: {}", first, third, last);
        }
        _ => println!("不匹配的向量"),
    }

    // 在循环中忽略索引
    let items = vec!["apple", "banana", "cherry"];
    for (_, item) in items.iter().enumerate() {
        println!("水果: {}", item);
    }
}

// 11. 嵌套模式 (Nested Patterns)
// 在复杂数据结构中进行深层模式匹配
fn nested_patterns_demo() {
    println!("\n--- 嵌套模式演示 ---");

    // 嵌套元组
    let nested_tuple = ((1, 2), (3, 4));
    match nested_tuple {
        ((a, b), (c, d)) => {
            println!("嵌套元组: ({}, {}) 和 ({}, {})", a, b, c, d);
            println!("对角线和: {} 和 {}", a + d, b + c);
        }
    }

    // 嵌套 Option
    let nested_option = Some(Some(42));
    match nested_option {
        Some(Some(value)) => println!("双重包装的值: {}", value),
        Some(None) => println!("外层有值，内层无值"),
        None => println!("外层无值"),
    }

    // 嵌套 Result 和 Option
    let complex_result: Result<Option<i32>, &str> = Ok(Some(100));
    match complex_result {
        Ok(Some(value)) => println!("成功获取值: {}", value),
        Ok(None) => println!("成功但无值"),
        Err(error) => println!("错误: {}", error),
    }

    // 复杂嵌套结构
    #[derive(Debug)]
    struct Address {
        street: String,
        city: String,
        country: String,
    }

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        address: Option<Address>,
    }

    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        address: Some(Address {
            street: "123 Main St".to_string(),
            city: "Beijing".to_string(),
            country: "China".to_string(),
        }),
    };

    match person {
        Person {
            name,
            age,
            address: Some(Address { city, country, .. }),
        } => {
            println!("{} ({} 岁) 住在 {}, {}", name, age, city, country);
        }
        Person {
            name,
            age,
            address: None,
        } => {
            println!("{} ({} 岁) 地址未知", name, age);
        }
    }

    // 嵌套向量
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    for (i, row) in matrix.iter().enumerate() {
        match row.as_slice() {
            [a, b, c] => println!("第 {} 行: [{}, {}, {}]", i + 1, a, b, c),
            _ => println!("第 {} 行格式不正确", i + 1),
        }
    }

    // 嵌套枚举
    #[derive(Debug)]
    enum Shape {
        Circle(f64),
        Rectangle(f64, f64),
        Triangle(f64, f64, f64),
    }

    #[derive(Debug)]
    enum Color {
        Red,
        Green,
        Blue,
        RGB(u8, u8, u8),
    }

    #[derive(Debug)]
    struct ColoredShape {
        shape: Shape,
        color: Color,
    }

    let colored_shape = ColoredShape {
        shape: Shape::Circle(5.0),
        color: Color::RGB(255, 0, 0),
    };

    match colored_shape {
        ColoredShape {
            shape: Shape::Circle(radius),
            color: Color::RGB(r, g, b),
        } => {
            println!("红色圆形，半径: {}, RGB({}, {}, {})", radius, r, g, b);
        }
        ColoredShape {
            shape: Shape::Rectangle(w, h),
            color,
        } => {
            println!("矩形 {}x{}，颜色: {:?}", w, h, color);
        }
        ColoredShape { shape, color } => {
            println!("其他形状: {:?}，颜色: {:?}", shape, color);
        }
    }
}

// 12. 高级模式技巧
// 展示一些高级的模式匹配技巧和最佳实践
fn advanced_patterns_demo() {
    println!("\n--- 高级模式技巧演示 ---");

    // 模式匹配中的类型转换
    let value: Box<dyn std::any::Any> = Box::new(42i32);
    match value.downcast::<i32>() {
        Ok(int_val) => println!("整数值: {}", int_val),
        Err(_) => println!("不是整数类型"),
    }

    // 使用模式匹配进行错误处理链
    fn parse_and_double(s: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let num: i32 = s.parse()?;
        Ok(num * 2)
    }

    let inputs = vec!["42", "not_a_number", "100"];
    for input in inputs {
        match parse_and_double(input) {
            Ok(result) => println!("'{}' -> {}", input, result),
            Err(e) => println!("'{}' 解析失败: {}", input, e),
        }
    }

    // 模式匹配与迭代器结合
    let pairs = vec![(1, 2), (3, 4), (5, 6)];
    let sum: i32 = pairs.iter().map(|(a, b)| a + b).sum();
    println!("配对和的总和: {}", sum);

    // 条件编译中的模式匹配
    #[cfg(target_os = "macos")]
    let os_info = "macOS";
    #[cfg(target_os = "linux")]
    let os_info = "Linux";
    #[cfg(target_os = "windows")]
    let os_info = "Windows";
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    let os_info = "Unknown";

    println!("当前操作系统: {}", os_info);

    // 使用模式匹配实现状态机
    #[derive(Debug, Clone)]
    enum State {
        Idle,
        Processing { progress: u8 },
        Complete { result: String },
        Error { message: String },
    }

    fn transition_state(current: State, input: &str) -> State {
        match (current, input) {
            (State::Idle, "start") => State::Processing { progress: 0 },
            (State::Processing { progress }, "update") if progress < 100 => State::Processing {
                progress: progress + 10,
            },
            (State::Processing { progress: 100 }, _) => State::Complete {
                result: "任务完成".to_string(),
            },
            (State::Processing { .. }, "error") => State::Error {
                message: "处理过程中出错".to_string(),
            },
            (state, _) => state, // 保持当前状态
        }
    }

    let mut state = State::Idle;
    let commands = vec![
        "start", "update", "update", "update", "update", "update", "update", "update", "update",
        "update", "update",
    ];

    for cmd in commands {
        state = transition_state(state.clone(), cmd);
        println!("命令 '{}' -> 状态: {:?}", cmd, state);
    }

    // 模式匹配优化：避免克隆
    #[derive(Debug)]
    struct LargeData {
        id: u32,
        data: Vec<u8>,
    }

    let large_data = LargeData {
        id: 1,
        data: vec![0; 1000], // 大数据
    };

    // 使用引用避免移动
    match &large_data {
        LargeData { id, data } if data.len() > 500 => {
            println!("大数据对象 ID: {}, 大小: {} 字节", id, data.len());
        }
        LargeData { id, .. } => {
            println!("小数据对象 ID: {}", id);
        }
    }

    // large_data 仍然可用
    println!("数据对象仍然可用: ID = {}", large_data.id);
}

// 13. 性能优化和最佳实践
fn performance_optimization_demo() {
    println!("\n--- 性能优化演示 ---");

    // 1. 避免不必要的克隆
    let strings = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];

    // 好的做法：使用引用
    for s in &strings {
        match s.as_str() {
            "hello" => println!("问候"),
            "world" => println!("世界"),
            "rust" => println!("Rust 语言"),
            _ => println!("其他: {}", s),
        }
    }

    // 2. 使用 if let 简化简单匹配
    let opt = Some(42);

    // 简洁的写法
    if let Some(value) = opt {
        println!("值: {}", value);
    }

    // 3. 使用 while let 处理迭代器
    let mut iter = vec![1, 2, 3, 4, 5].into_iter();
    while let Some(value) = iter.next() {
        if value > 3 {
            println!("大于3的值: {}", value);
        }
    }

    // 4. 模式匹配的编译时优化
    // Rust 编译器会将模式匹配优化为跳转表或决策树
    let number = 42;
    let category = match number {
        0..=10 => "小",
        11..=50 => "中",
        51..=100 => "大",
        _ => "超大",
    };
    println!("数字 {} 的类别: {}", number, category);

    // 5. 使用 matches! 宏进行简单检查
    let value = Some(42);
    if matches!(value, Some(x) if x > 40) {
        println!("值大于40");
    }

    // 6. 避免深层嵌套，使用早期返回
    fn process_nested_option(opt: Option<Option<i32>>) -> String {
        match opt {
            Some(Some(value)) if value > 0 => format!("正数: {}", value),
            Some(Some(0)) => "零".to_string(),
            Some(Some(value)) => format!("负数: {}", value), // 处理所有剩余的 Some(Some(_)) 情况
            Some(None) => "内层为空".to_string(),
            None => "外层为空".to_string(),
        }
    }

    let test_cases = vec![
        Some(Some(42)),
        Some(Some(-10)),
        Some(Some(0)),
        Some(None),
        None,
    ];

    for case in test_cases {
        println!("处理 {:?}: {}", case, process_nested_option(case));
    }

    // 7. 使用解构减少字段访问
    #[derive(Debug)]
    struct Point3D {
        x: f64,
        y: f64,
        z: f64,
    }

    let point = Point3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    // 一次性解构所有需要的字段
    let Point3D { x, y, z } = point;
    let distance = (x * x + y * y + z * z).sqrt();
    println!("点 ({}, {}, {}) 到原点的距离: {:.2}", x, y, z, distance);
}

// 14. 实际应用场景
fn real_world_applications_demo() {
    println!("\n--- 实际应用场景演示 ---");

    // 1. JSON 解析和处理
    #[derive(Debug)]
    enum JsonValue {
        Null,
        Bool(bool),
        Number(f64),
        String(String),
        Array(Vec<JsonValue>),
        Object(std::collections::HashMap<String, JsonValue>),
    }

    fn process_json_value(value: &JsonValue) -> String {
        match value {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::String(s) => format!("\"{}\"", s),
            JsonValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(process_json_value).collect();
                format!("[{}]", items.join(", "))
            }
            JsonValue::Object(obj) => {
                let pairs: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, process_json_value(v)))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
        }
    }

    let json_data = JsonValue::Object({
        let mut map = std::collections::HashMap::new();
        map.insert("name".to_string(), JsonValue::String("Alice".to_string()));
        map.insert("age".to_string(), JsonValue::Number(30.0));
        map.insert("active".to_string(), JsonValue::Bool(true));
        map
    });

    println!("JSON 处理结果: {}", process_json_value(&json_data));

    // 2. 命令行参数解析
    #[derive(Debug)]
    enum Command {
        Help,
        Version,
        Run { file: String, verbose: bool },
        Test { pattern: Option<String> },
    }

    fn parse_command(args: &[String]) -> Result<Command, String> {
        match args {
            [] | [_] => Ok(Command::Help),
            [_, cmd] if cmd == "help" || cmd == "-h" || cmd == "--help" => Ok(Command::Help),
            [_, cmd] if cmd == "version" || cmd == "-v" || cmd == "--version" => {
                Ok(Command::Version)
            }
            [_, cmd, file] if cmd == "run" => Ok(Command::Run {
                file: file.clone(),
                verbose: false,
            }),
            [_, cmd, file, flag] if cmd == "run" && (flag == "-v" || flag == "--verbose") => {
                Ok(Command::Run {
                    file: file.clone(),
                    verbose: true,
                })
            }
            [_, cmd] if cmd == "test" => Ok(Command::Test { pattern: None }),
            [_, cmd, pattern] if cmd == "test" => Ok(Command::Test {
                pattern: Some(pattern.clone()),
            }),
            _ => Err("无效的命令行参数".to_string()),
        }
    }

    let test_args = vec![
        vec!["program".to_string()],
        vec!["program".to_string(), "help".to_string()],
        vec!["program".to_string(), "version".to_string()],
        vec![
            "program".to_string(),
            "run".to_string(),
            "main.rs".to_string(),
        ],
        vec![
            "program".to_string(),
            "test".to_string(),
            "unit".to_string(),
        ],
    ];

    for args in test_args {
        match parse_command(&args) {
            Ok(cmd) => println!("解析命令: {:?}", cmd),
            Err(e) => println!("解析错误: {}", e),
        }
    }

    // 3. 网络协议处理
    #[derive(Debug)]
    enum NetworkMessage {
        Connect { client_id: u32 },
        Disconnect { client_id: u32 },
        Data { client_id: u32, payload: Vec<u8> },
        Heartbeat { client_id: u32, timestamp: u64 },
        Error { code: u16, message: String },
    }

    fn handle_network_message(msg: NetworkMessage) {
        match msg {
            NetworkMessage::Connect { client_id } => {
                println!("客户端 {} 已连接", client_id);
            }
            NetworkMessage::Disconnect { client_id } => {
                println!("客户端 {} 已断开连接", client_id);
            }
            NetworkMessage::Data { client_id, payload } if payload.len() > 1024 => {
                println!("客户端 {} 发送大数据包: {} 字节", client_id, payload.len());
            }
            NetworkMessage::Data { client_id, payload } => {
                println!("客户端 {} 发送数据: {} 字节", client_id, payload.len());
            }
            NetworkMessage::Heartbeat {
                client_id,
                timestamp,
            } => {
                println!("客户端 {} 心跳: {}", client_id, timestamp);
            }
            NetworkMessage::Error { code, message } => {
                println!("网络错误 {}: {}", code, message);
            }
        }
    }

    let messages = vec![
        NetworkMessage::Connect { client_id: 1 },
        NetworkMessage::Data {
            client_id: 1,
            payload: vec![0; 512],
        },
        NetworkMessage::Heartbeat {
            client_id: 1,
            timestamp: 1234567890,
        },
        NetworkMessage::Error {
            code: 404,
            message: "Not Found".to_string(),
        },
        NetworkMessage::Disconnect { client_id: 1 },
    ];

    for msg in messages {
        handle_network_message(msg);
    }

    // 4. 配置文件解析
    #[derive(Debug)]
    enum ConfigValue {
        String(String),
        Integer(i64),
        Float(f64),
        Boolean(bool),
        Array(Vec<ConfigValue>),
        Section(std::collections::HashMap<String, ConfigValue>),
    }

    fn validate_config(
        config: &std::collections::HashMap<String, ConfigValue>,
    ) -> Result<(), String> {
        match (config.get("database"), config.get("server")) {
            (Some(ConfigValue::Section(db)), Some(ConfigValue::Section(server))) => {
                // 验证数据库配置
                match (db.get("host"), db.get("port")) {
                    (Some(ConfigValue::String(_)), Some(ConfigValue::Integer(port)))
                        if *port > 0 && *port < 65536 =>
                    {
                        println!("数据库配置有效");
                    }
                    _ => return Err("数据库配置无效".to_string()),
                }

                // 验证服务器配置
                match server.get("port") {
                    Some(ConfigValue::Integer(port)) if *port > 1024 && *port < 65536 => {
                        println!("服务器配置有效");
                    }
                    _ => return Err("服务器端口配置无效".to_string()),
                }

                Ok(())
            }
            _ => Err("缺少必要的配置节".to_string()),
        }
    }

    let mut config = std::collections::HashMap::new();

    let mut db_config = std::collections::HashMap::new();
    db_config.insert(
        "host".to_string(),
        ConfigValue::String("localhost".to_string()),
    );
    db_config.insert("port".to_string(), ConfigValue::Integer(5432));
    config.insert("database".to_string(), ConfigValue::Section(db_config));

    let mut server_config = std::collections::HashMap::new();
    server_config.insert("port".to_string(), ConfigValue::Integer(8080));
    config.insert("server".to_string(), ConfigValue::Section(server_config));

    match validate_config(&config) {
        Ok(()) => println!("配置验证通过"),
        Err(e) => println!("配置验证失败: {}", e),
    }

    // 5. 编译器/解释器中的 AST 处理
    #[derive(Debug)]
    enum Expr {
        Number(f64),
        Variable(String),
        Add(Box<Expr>, Box<Expr>),
        Multiply(Box<Expr>, Box<Expr>),
        Call { name: String, args: Vec<Expr> },
    }

    fn evaluate_expr(
        expr: &Expr,
        vars: &std::collections::HashMap<String, f64>,
    ) -> Result<f64, String> {
        match expr {
            Expr::Number(n) => Ok(*n),
            Expr::Variable(name) => vars
                .get(name)
                .copied()
                .ok_or_else(|| format!("未定义的变量: {}", name)),
            Expr::Add(left, right) => {
                let l = evaluate_expr(left, vars)?;
                let r = evaluate_expr(right, vars)?;
                Ok(l + r)
            }
            Expr::Multiply(left, right) => {
                let l = evaluate_expr(left, vars)?;
                let r = evaluate_expr(right, vars)?;
                Ok(l * r)
            }
            Expr::Call { name, args } => match (name.as_str(), args.as_slice()) {
                ("sqrt", [arg]) => {
                    let val = evaluate_expr(arg, vars)?;
                    if val >= 0.0 {
                        Ok(val.sqrt())
                    } else {
                        Err("sqrt 参数不能为负数".to_string())
                    }
                }
                ("pow", [base, exp]) => {
                    let b = evaluate_expr(base, vars)?;
                    let e = evaluate_expr(exp, vars)?;
                    Ok(b.powf(e))
                }
                _ => Err(format!("未知函数: {}", name)),
            },
        }
    }

    let mut variables = std::collections::HashMap::new();
    variables.insert("x".to_string(), 10.0);
    variables.insert("y".to_string(), 5.0);

    // 创建表达式: x + y * 2
    let expr = Expr::Add(
        Box::new(Expr::Variable("x".to_string())),
        Box::new(Expr::Multiply(
            Box::new(Expr::Variable("y".to_string())),
            Box::new(Expr::Number(2.0)),
        )),
    );

    match evaluate_expr(&expr, &variables) {
        Ok(result) => println!("表达式 'x + y * 2' 的结果: {}", result),
        Err(e) => println!("表达式求值错误: {}", e),
    }

    // 创建函数调用表达式: sqrt(x * x + y * y)
    let distance_expr = Expr::Call {
        name: "sqrt".to_string(),
        args: vec![Expr::Add(
            Box::new(Expr::Multiply(
                Box::new(Expr::Variable("x".to_string())),
                Box::new(Expr::Variable("x".to_string())),
            )),
            Box::new(Expr::Multiply(
                Box::new(Expr::Variable("y".to_string())),
                Box::new(Expr::Variable("y".to_string())),
            )),
        )],
    };

    match evaluate_expr(&distance_expr, &variables) {
        Ok(result) => println!("距离计算 'sqrt(x*x + y*y)' 的结果: {:.2}", result),
        Err(e) => println!("距离计算错误: {}", e),
    }

    println!("\n=== 总结 ===");
    println!("本文档展示了 Rust 中所有主要的模式匹配类型：");
    println!("1. 字面量模式 - 直接匹配具体值");
    println!("2. 变量模式 - 绑定匹配的值到变量");
    println!("3. 通配符模式 - 使用 _ 匹配任何值");
    println!("4. 范围模式 - 匹配值的范围");
    println!("5. 解构模式 - 解构复合数据类型");
    println!("6. 引用模式 - 处理引用和借用");
    println!("7. 守卫模式 - 添加额外条件判断");
    println!("8. 绑定模式 - 使用 @ 同时匹配和绑定");
    println!("9. 多重模式 - 使用 | 匹配多个模式");
    println!("10. 忽略模式 - 使用 _ 和 .. 忽略值");
    println!("11. 嵌套模式 - 深层模式匹配");
    println!("12. 高级技巧 - 类型转换、状态机等");
    println!("13. 性能优化 - 最佳实践和优化技巧");
    println!("14. 实际应用 - JSON解析、命令行、网络协议等");
    println!("\n模式匹配是 Rust 最强大的特性之一，掌握这些模式能够：");
    println!("- 编写更安全、更表达力强的代码");
    println!("- 充分利用 Rust 的类型系统");
    println!("- 实现复杂的控制流逻辑");
    println!("- 提高代码的可读性和维护性");
}
