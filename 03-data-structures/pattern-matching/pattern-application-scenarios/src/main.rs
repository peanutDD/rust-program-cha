// Rust 模式匹配应用场景全面分析
// 基于 https://course.rs/basic/match-pattern/pattern-match.html
// 深入探讨模式匹配在各种场景下的应用

#[derive(Debug, Clone, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
    Hsv { h: u16, s: u8, v: u8 },
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    email: Option<String>,
}

fn main() {
    println!("=== Rust 模式匹配应用场景全面分析 ===");

    // 1. match 表达式的基本应用
    demonstrate_basic_match();

    // 2. if let 语法糖的应用场景
    demonstrate_if_let();

    // 3. while let 循环模式匹配
    demonstrate_while_let();

    // 4. for 循环中的模式匹配
    demonstrate_for_patterns();

    // 5. let 语句中的模式匹配
    demonstrate_let_patterns();

    // 6. 函数参数中的模式匹配
    demonstrate_function_parameters();

    // 7. 复杂数据结构的解构
    demonstrate_complex_destructuring();

    // 8. 守卫条件的使用
    demonstrate_match_guards();

    // 9. @ 绑定的应用
    demonstrate_at_bindings();

    // 10. 忽略值的模式
    demonstrate_ignoring_patterns();

    // 11. 范围模式匹配
    demonstrate_range_patterns();

    // 12. 引用和解引用模式
    demonstrate_ref_patterns();

    // 13. 嵌套模式匹配
    demonstrate_nested_patterns();

    // 14. 错误处理中的模式匹配
    demonstrate_error_handling();

    // 15. 实际应用场景
    demonstrate_real_world_scenarios();
}

// 1. match 表达式的基本应用
fn demonstrate_basic_match() {
    println!("\n1. === match 表达式基本应用 ===");

    let color = Color::Rgb(255, 0, 0);

    // 基本的枚举匹配
    let color_name = match color {
        Color::Red => "红色",
        Color::Green => "绿色",
        Color::Blue => "蓝色",
        Color::Rgb(r, g, b) => {
            println!("RGB 颜色: ({}, {}, {})", r, g, b);
            "自定义RGB颜色"
        }
        Color::Hsv { h, s, v } => {
            println!("HSV 颜色: h={}, s={}, v={}", h, s, v);
            "HSV颜色"
        }
    };

    println!("颜色名称: {}", color_name);

    // 数值匹配
    let number = 42;
    match number {
        0 => println!("零"),
        1..=10 => println!("1到10之间的数字: {}", number),
        11 | 12 | 13 => println!("11、12或13: {}", number),
        n if n > 100 => println!("大于100的数字: {}", n),
        _ => println!("其他数字: {}", number),
    }
}

// 2. if let 语法糖的应用场景
fn demonstrate_if_let() {
    println!("\n2. === if let 语法糖应用 ===");

    let some_value = Some(42);

    // 传统的 match 方式
    match some_value {
        Some(x) => println!("传统match: 值是 {}", x),
        None => {}
    }

    // 使用 if let 简化
    if let Some(x) = some_value {
        println!("if let: 值是 {}", x);
    }

    // 复杂枚举的 if let
    let message = Message::ChangeColor(Color::Red);
    if let Message::ChangeColor(Color::Red) = message {
        println!("收到红色变更消息");
    }

    // if let 与 else 结合
    let optional_word = Some("hello".to_string());
    if let Some(word) = optional_word {
        println!("单词是: {}", word);
    } else {
        println!("没有单词");
    }

    // if let 链式匹配
    let nested_option = Some(Some(42));
    if let Some(Some(n)) = nested_option {
        println!("嵌套Option的值: {}", n);
    }
}

// 3. while let 循环模式匹配
fn demonstrate_while_let() {
    println!("\n3. === while let 循环模式匹配 ===");

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("使用 while let 弹出栈元素:");
    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }

    // 迭代器与 while let
    let mut iter = vec![1, 2, 3, 4, 5].into_iter();
    println!("\n使用 while let 遍历迭代器:");
    while let Some(item) = iter.next() {
        if item > 3 {
            break;
        }
        println!("项目: {}", item);
    }

    // 复杂数据结构的 while let
    let mut messages = vec![
        Message::Write("Hello".to_string()),
        Message::Move { x: 10, y: 20 },
        Message::Quit,
    ];

    println!("\n处理消息队列:");
    while let Some(msg) = messages.pop() {
        match msg {
            Message::Write(text) => println!("写入消息: {}", text),
            Message::Move { x, y } => println!("移动到: ({}, {})", x, y),
            Message::Quit => {
                println!("退出消息");
                break;
            }
            _ => {}
        }
    }
}

// 4. for 循环中的模式匹配
fn demonstrate_for_patterns() {
    println!("\n4. === for 循环中的模式匹配 ===");

    // 元组解构
    let points = vec![(1, 2), (3, 4), (5, 6)];
    println!("解构元组:");
    for (x, y) in points {
        println!("点: ({}, {})", x, y);
    }

    // 枚举索引和值
    let colors = vec![Color::Red, Color::Green, Color::Blue];
    println!("\n枚举索引和值:");
    for (index, color) in colors.iter().enumerate() {
        println!("索引 {}: {:?}", index, color);
    }

    // HashMap 的键值对解构
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert("Alice".to_string(), 95);
    scores.insert("Bob".to_string(), 87);

    println!("\nHashMap 键值对:");
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // 复杂结构的解构
    let users = vec![
        User {
            name: "Alice".to_string(),
            age: 30,
            email: Some("alice@example.com".to_string()),
        },
        User {
            name: "Bob".to_string(),
            age: 25,
            email: None,
        },
    ];

    println!("\n用户信息解构:");
    for User { name, age, email } in users {
        match email {
            Some(email_addr) => println!("{} ({}岁): {}", name, age, email_addr),
            None => println!("{} ({}岁): 无邮箱", name, age),
        }
    }
}

// 5. let 语句中的模式匹配
fn demonstrate_let_patterns() {
    println!("\n5. === let 语句中的模式匹配 ===");

    // 元组解构
    let (x, y, z) = (1, 2, 3);
    println!("元组解构: x={}, y={}, z={}", x, y, z);

    // 数组解构
    let [first, second, third] = [1, 2, 3];
    println!(
        "数组解构: first={}, second={}, third={}",
        first, second, third
    );

    // 结构体解构
    let point = Point { x: 10, y: 20 };
    let Point { x, y } = point;
    println!("结构体解构: x={}, y={}", x, y);

    // 重命名字段
    let Point { x: px, y: py } = Point { x: 5, y: 15 };
    println!("重命名字段: px={}, py={}", px, py);

    // 嵌套解构
    let ((a, b), Point { x, y }) = ((1, 2), Point { x: 3, y: 4 });
    println!("嵌套解构: a={}, b={}, x={}, y={}", a, b, x, y);

    // 忽略部分值
    let (first, _, third) = (1, 2, 3);
    println!("忽略中间值: first={}, third={}", first, third);

    // 剩余模式
    let numbers = [1, 2, 3, 4, 5];
    let [head, tail @ ..] = numbers;
    println!("头部: {}, 尾部: {:?}", head, tail);
}

// 6. 函数参数中的模式匹配
fn demonstrate_function_parameters() {
    println!("\n6. === 函数参数中的模式匹配 ===");

    // 元组参数解构
    fn print_coordinates((x, y): (i32, i32)) {
        println!("坐标: ({}, {})", x, y);
    }

    print_coordinates((10, 20));

    // 结构体参数解构
    fn print_point(Point { x, y }: Point) {
        println!("点的坐标: x={}, y={}", x, y);
    }

    print_point(Point { x: 5, y: 10 });

    // 引用解构
    fn print_point_ref(&Point { x, y }: &Point) {
        println!("引用点的坐标: x={}, y={}", x, y);
    }

    let point = Point { x: 15, y: 25 };
    print_point_ref(&point);

    // 复杂参数解构
    fn process_user_data((name, User { age, email, .. }): (String, User)) {
        println!("处理用户: {}, 年龄: {}", name, age);
        if let Some(email_addr) = email {
            println!("邮箱: {}", email_addr);
        }
    }

    let user = User {
        name: "Charlie".to_string(),
        age: 28,
        email: Some("charlie@example.com".to_string()),
    };

    process_user_data(("用户Charlie".to_string(), user));
}

// 7. 复杂数据结构的解构
fn demonstrate_complex_destructuring() {
    println!("\n7. === 复杂数据结构解构 ===");

    // 嵌套枚举解构
    let message = Message::ChangeColor(Color::Rgb(255, 128, 0));

    match message {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("更改为RGB颜色: ({}, {}, {})", r, g, b);
        }
        Message::ChangeColor(Color::Hsv { h, s, v }) => {
            println!("更改为HSV颜色: h={}, s={}, v={}", h, s, v);
        }
        Message::Move { x, y } => {
            println!("移动到: ({}, {})", x, y);
        }
        _ => println!("其他消息"),
    }

    // Option 嵌套解构
    let nested_option: Option<Option<i32>> = Some(Some(42));
    match nested_option {
        Some(Some(n)) => println!("嵌套Some值: {}", n),
        Some(None) => println!("外层Some，内层None"),
        None => println!("外层None"),
    }

    // Result 与 Option 组合
    let result_option: Result<Option<i32>, String> = Ok(Some(100));
    match result_option {
        Ok(Some(value)) => println!("成功获取值: {}", value),
        Ok(None) => println!("成功但无值"),
        Err(e) => println!("错误: {}", e),
    }

    // 向量解构
    let vec = vec![1, 2, 3, 4, 5];
    match vec.as_slice() {
        [] => println!("空向量"),
        [x] => println!("单元素向量: {}", x),
        [x, y] => println!("双元素向量: {}, {}", x, y),
        [first, .., last] => println!("多元素向量，首: {}, 尾: {}", first, last),
    }
}

// 8. 守卫条件的使用
fn demonstrate_match_guards() {
    println!("\n8. === 守卫条件使用 ===");

    let number = Some(4);

    match number {
        Some(x) if x < 5 => println!("小于5的数字: {}", x),
        Some(x) if x == 5 => println!("等于5的数字: {}", x),
        Some(x) if x > 5 => println!("大于5的数字: {}", x),
        Some(x) => println!("其他数字: {}", x),
        None => println!("没有数字"),
    }

    // 复杂守卫条件
    let point = Point { x: 10, y: 20 };
    match point {
        Point { x, y } if x == y => println!("x和y相等: {}", x),
        Point { x, y } if x > y => println!("x大于y: {} > {}", x, y),
        Point { x, y } if x < y => println!("x小于y: {} < {}", x, y),
        Point { x, y } => println!("其他情况: ({}, {})", x, y),
    }

    // 多重条件守卫
    let age = 25;
    let has_license = true;

    match age {
        age if age >= 18 && has_license => println!("可以开车"),
        age if age >= 18 => println!("成年但无驾照"),
        _ => println!("未成年"),
    }

    // 枚举与守卫结合
    let color = Color::Rgb(255, 0, 0);
    match color {
        Color::Rgb(r, g, b) if r > 200 => println!("高红色值: {}", r),
        Color::Rgb(r, g, b) if g > 200 => println!("高绿色值: {}", g),
        Color::Rgb(r, g, b) if b > 200 => println!("高蓝色值: {}", b),
        Color::Rgb(r, g, b) => println!("普通RGB: ({}, {}, {})", r, g, b),
        _ => println!("非RGB颜色"),
    }
}

// 9. @ 绑定的应用
fn demonstrate_at_bindings() {
    println!("\n9. === @ 绑定应用 ===");

    let number = 15;

    match number {
        n @ 1..=10 => println!("1到10之间的数字: {}", n),
        n @ 11..=20 => println!("11到20之间的数字: {}", n),
        n => println!("其他数字: {}", n),
    }

    // 复杂结构的 @ 绑定
    let message = Message::Move { x: 10, y: 20 };
    match message {
        msg @ Message::Move { x, y } if x > 5 => {
            println!("移动消息 {:?}，x > 5", msg);
        }
        Message::Move { x, y } => println!("普通移动: ({}, {})", x, y),
        _ => println!("其他消息"),
    }

    // Option 与 @ 绑定
    let some_number = Some(42);
    match some_number {
        opt @ Some(n) if n > 40 => {
            println!("大数字选项: {:?}", opt);
        }
        Some(n) => println!("小数字: {}", n),
        None => println!("无值"),
    }

    // 嵌套 @ 绑定
    let nested = Some(Point { x: 5, y: 10 });
    match nested {
        Some(p @ Point { x, y }) if x < 10 => {
            println!("小x值的点: {:?}", p);
        }
        Some(Point { x, y }) => println!("大x值的点: ({}, {})", x, y),
        None => println!("无点"),
    }
}

// 10. 忽略值的模式
fn demonstrate_ignoring_patterns() {
    println!("\n10. === 忽略值的模式 ===");

    // 使用 _ 忽略整个值
    let some_value = Some(42);
    match some_value {
        Some(_) => println!("有值，但不关心具体值"),
        None => println!("无值"),
    }

    // 忽略结构体的部分字段
    let point = Point { x: 10, y: 20 };
    match point {
        Point { x, y: _ } => println!("只关心x值: {}", x),
    }

    // 使用 .. 忽略剩余字段
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        email: Some("alice@example.com".to_string()),
    };

    match user {
        User { name, .. } => println!("用户名: {}", name),
    }

    // 忽略元组的部分元素
    let tuple = (1, 2, 3, 4, 5);
    match tuple {
        (first, _, _, _, last) => println!("首: {}, 尾: {}", first, last),
    }

    // 忽略函数参数
    fn process_point(Point { x, y: _ }: Point) {
        println!("处理点的x坐标: {}", x);
    }

    process_point(Point { x: 100, y: 200 });

    // 使用 _name 保留绑定但表示不使用
    let _unused_variable = 42;
    println!("这个变量不会产生未使用警告");
}

// 11. 范围模式匹配
fn demonstrate_range_patterns() {
    println!("\n11. === 范围模式匹配 ===");

    let number = 42;

    match number {
        1..=10 => println!("1到10"),
        11..=50 => println!("11到50: {}", number),
        51..=100 => println!("51到100"),
        _ => println!("其他范围"),
    }

    // 字符范围
    let character = 'g';
    match character {
        'a'..='f' => println!("前半部分字母: {}", character),
        'g'..='z' => println!("后半部分字母: {}", character),
        _ => println!("非小写字母"),
    }

    // 多个范围组合
    let score = 85;
    let grade = match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    };
    println!("分数 {} 对应等级: {}", score, grade);

    // 范围与守卫结合
    let value = 25;
    match value {
        n @ 1..=50 if n % 2 == 0 => println!("1-50间的偶数: {}", n),
        n @ 1..=50 => println!("1-50间的奇数: {}", n),
        n => println!("其他数字: {}", n),
    }
}

// 12. 引用和解引用模式
fn demonstrate_ref_patterns() {
    println!("\n12. === 引用和解引用模式 ===");

    let point = Point { x: 10, y: 20 };
    let point_ref = &point;

    // 匹配引用
    match point_ref {
        &Point { x, y } => println!("解引用匹配: ({}, {})", x, y),
    }

    // 使用 ref 创建引用
    match point {
        Point { ref x, ref y } => {
            println!("创建引用: x={}, y={}", x, y);
            // x 和 y 是 &i32 类型
        }
    }

    // ref mut 创建可变引用
    let mut mutable_point = Point { x: 5, y: 10 };
    match mutable_point {
        Point { ref mut x, ref y } => {
            *x += 10;
            println!("修改后的x: {}, y: {}", x, y);
        }
    }

    // 向量引用模式
    let vec = vec![1, 2, 3, 4];
    match vec.as_slice() {
        [] => println!("空向量"),
        [first, rest @ ..] => {
            println!("第一个元素: {}", first);
            println!("剩余元素: {:?}", rest);
        }
    }

    // Option 引用模式
    let opt_string = Some("hello".to_string());
    match &opt_string {
        Some(s) => println!("字符串引用: {}", s),
        None => println!("无值"),
    }

    // 保持所有权的同时进行匹配
    match &opt_string {
        Some(s) => println!("仍然拥有字符串: {}", s),
        None => println!("无值"),
    }
    println!("opt_string 仍然可用: {:?}", opt_string);
}

// 13. 嵌套模式匹配
fn demonstrate_nested_patterns() {
    println!("\n13. === 嵌套模式匹配 ===");

    // 深度嵌套的 Option
    let deeply_nested = Some(Some(Some(42)));
    match deeply_nested {
        Some(Some(Some(n))) => println!("三层嵌套的值: {}", n),
        Some(Some(None)) => println!("第三层为None"),
        Some(None) => println!("第二层为None"),
        None => println!("第一层为None"),
    }

    // 复杂结构嵌套
    #[derive(Debug)]
    struct Company {
        name: String,
        employees: Vec<User>,
    }

    let company = Company {
        name: "TechCorp".to_string(),
        employees: vec![
            User {
                name: "Alice".to_string(),
                age: 30,
                email: Some("alice@techcorp.com".to_string()),
            },
            User {
                name: "Bob".to_string(),
                age: 25,
                email: None,
            },
        ],
    };

    // 嵌套结构匹配
    match company {
        Company {
            name,
            employees: ref emps,
        } if emps.len() > 0 => {
            println!("公司 {} 有 {} 名员工", name, emps.len());

            // 进一步匹配第一个员工
            match emps.first() {
                Some(User {
                    name: emp_name,
                    email: Some(email),
                    ..
                }) => {
                    println!("第一个员工: {}, 邮箱: {}", emp_name, email);
                }
                Some(User {
                    name: emp_name,
                    email: None,
                    ..
                }) => {
                    println!("第一个员工: {}, 无邮箱", emp_name);
                }
                None => println!("没有员工"),
            }
        }
        Company { name, .. } => println!("公司 {} 没有员工", name),
    }

    // 元组嵌套匹配
    let nested_tuple = ((1, 2), (3, (4, 5)));
    match nested_tuple {
        ((a, b), (c, (d, e))) => {
            println!("嵌套元组: a={}, b={}, c={}, d={}, e={}", a, b, c, d, e);
        }
    }

    // 枚举嵌套匹配
    let nested_message = Message::ChangeColor(Color::Hsv {
        h: 180,
        s: 100,
        v: 50,
    });
    match nested_message {
        Message::ChangeColor(Color::Hsv { h, s, v }) if h > 120 => {
            println!("冷色调HSV: h={}, s={}, v={}", h, s, v);
        }
        Message::ChangeColor(Color::Hsv { h, s, v }) => {
            println!("暖色调HSV: h={}, s={}, v={}", h, s, v);
        }
        _ => println!("其他消息"),
    }
}

// 14. 错误处理中的模式匹配
fn demonstrate_error_handling() {
    println!("\n14. === 错误处理中的模式匹配 ===");

    // 自定义错误类型
    #[derive(Debug)]
    enum CustomError {
        NetworkError(String),
        ParseError { line: u32, column: u32 },
        ValidationError,
    }

    fn risky_operation(input: i32) -> Result<String, CustomError> {
        match input {
            0 => Err(CustomError::ValidationError),
            1..=10 => Ok(format!("处理成功: {}", input)),
            11..=20 => Err(CustomError::ParseError {
                line: 1,
                column: input as u32,
            }),
            _ => Err(CustomError::NetworkError("连接超时".to_string())),
        }
    }

    // 使用模式匹配处理不同类型的错误
    for i in [0, 5, 15, 25] {
        match risky_operation(i) {
            Ok(result) => println!("成功: {}", result),
            Err(CustomError::ValidationError) => {
                println!("输入 {} 验证失败", i);
            }
            Err(CustomError::ParseError { line, column }) => {
                println!("输入 {} 解析错误，位置: 行{}, 列{}", i, line, column);
            }
            Err(CustomError::NetworkError(msg)) => {
                println!("输入 {} 网络错误: {}", i, msg);
            }
        }
    }

    // 链式错误处理
    fn parse_and_double(s: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let num: i32 = s.parse()?;
        Ok(num * 2)
    }

    let inputs = ["42", "abc", "100"];
    for input in inputs {
        match parse_and_double(input) {
            Ok(result) => println!("\"{}\" 解析并翻倍: {}", input, result),
            Err(e) => println!("\"{}\" 处理失败: {}", input, e),
        }
    }

    // Option 与 Result 组合
    fn find_and_parse(vec: &[&str], index: usize) -> Result<Option<i32>, std::num::ParseIntError> {
        match vec.get(index) {
            Some(s) => Ok(Some(s.parse()?)),
            None => Ok(None),
        }
    }

    let data = ["10", "20", "abc", "40"];
    for i in 0..5 {
        match find_and_parse(&data, i) {
            Ok(Some(num)) => println!("索引 {} 找到数字: {}", i, num),
            Ok(None) => println!("索引 {} 超出范围", i),
            Err(e) => println!("索引 {} 解析错误: {}", i, e),
        }
    }
}

// 15. 实际应用场景
fn demonstrate_real_world_scenarios() {
    println!("\n15. === 实际应用场景 ===");

    // 场景1: 状态机实现
    demonstrate_state_machine();

    // 场景2: 配置解析
    demonstrate_config_parsing();

    // 场景3: 事件处理系统
    demonstrate_event_system();

    // 场景4: 数据验证
    demonstrate_data_validation();

    // 场景5: 协议解析
    demonstrate_protocol_parsing();
}

// 状态机实现
fn demonstrate_state_machine() {
    println!("\n--- 状态机实现 ---");

    #[derive(Debug, Clone)]
    enum State {
        Idle,
        Loading { progress: u8 },
        Success { data: String },
        Error { message: String, retry_count: u8 },
    }

    #[derive(Debug)]
    enum Event {
        StartLoading,
        UpdateProgress(u8),
        LoadSuccess(String),
        LoadError(String),
        Retry,
        Reset,
    }

    fn handle_state_transition(current_state: State, event: Event) -> State {
        match (current_state, event) {
            (State::Idle, Event::StartLoading) => {
                println!("开始加载...");
                State::Loading { progress: 0 }
            }
            (State::Loading { .. }, Event::UpdateProgress(progress)) => {
                println!("加载进度: {}%", progress);
                State::Loading { progress }
            }
            (State::Loading { .. }, Event::LoadSuccess(data)) => {
                println!("加载成功!");
                State::Success { data }
            }
            (State::Loading { .. }, Event::LoadError(message)) => {
                println!("加载失败: {}", message);
                State::Error {
                    message,
                    retry_count: 0,
                }
            }
            (
                State::Error {
                    message,
                    retry_count,
                },
                Event::Retry,
            ) if retry_count < 3 => {
                println!("重试第 {} 次", retry_count + 1);
                State::Loading { progress: 0 }
            }
            (State::Error { retry_count, .. }, Event::Retry) if retry_count >= 3 => {
                println!("重试次数已达上限");
                State::Error {
                    message: "重试次数超限".to_string(),
                    retry_count,
                }
            }
            (_, Event::Reset) => {
                println!("重置状态");
                State::Idle
            }
            (state, event) => {
                println!("无效的状态转换: {:?} + {:?}", state, event);
                state
            }
        }
    }

    // 模拟状态机运行
    let mut state = State::Idle;
    let events = vec![
        Event::StartLoading,
        Event::UpdateProgress(25),
        Event::UpdateProgress(50),
        Event::LoadError("网络错误".to_string()),
        Event::Retry,
        Event::UpdateProgress(75),
        Event::LoadSuccess("数据加载完成".to_string()),
        Event::Reset,
    ];

    for event in events {
        state = handle_state_transition(state, event);
        println!("当前状态: {:?}\n", state);
    }
}

// 配置解析
fn demonstrate_config_parsing() {
    println!("--- 配置解析 ---");

    #[derive(Debug)]
    enum ConfigValue {
        String(String),
        Integer(i64),
        Float(f64),
        Boolean(bool),
        Array(Vec<ConfigValue>),
        Object(std::collections::HashMap<String, ConfigValue>),
    }

    fn parse_config_value(input: &str) -> Result<ConfigValue, String> {
        let trimmed = input.trim();

        match trimmed {
            "true" => Ok(ConfigValue::Boolean(true)),
            "false" => Ok(ConfigValue::Boolean(false)),
            s if s.starts_with('"') && s.ends_with('"') => {
                Ok(ConfigValue::String(s[1..s.len() - 1].to_string()))
            }
            s if s.contains('.') => s
                .parse::<f64>()
                .map(ConfigValue::Float)
                .map_err(|_| format!("无效的浮点数: {}", s)),
            s => s
                .parse::<i64>()
                .map(ConfigValue::Integer)
                .map_err(|_| format!("无效的整数: {}", s)),
        }
    }

    fn get_config_as_string(config: &ConfigValue, key: &str) -> Option<String> {
        match config {
            ConfigValue::Object(map) => match map.get(key)? {
                ConfigValue::String(s) => Some(s.clone()),
                _ => None,
            },
            _ => None,
        }
    }

    // 模拟配置解析
    let config_inputs = [
        ("debug", "true"),
        ("port", "8080"),
        ("timeout", "30.5"),
        ("name", "\"MyApp\""),
        ("invalid", "not_a_number"),
    ];

    let mut config_map = std::collections::HashMap::new();

    for (key, value) in config_inputs {
        match parse_config_value(value) {
            Ok(parsed_value) => {
                println!("解析配置 {}: {:?}", key, parsed_value);
                config_map.insert(key.to_string(), parsed_value);
            }
            Err(e) => println!("配置解析错误 {}: {}", key, e),
        }
    }

    let config = ConfigValue::Object(config_map);

    // 使用配置
    match &config {
        ConfigValue::Object(map) => {
            for (key, value) in map {
                match value {
                    ConfigValue::Boolean(b) => println!("布尔配置 {}: {}", key, b),
                    ConfigValue::Integer(i) => println!("整数配置 {}: {}", key, i),
                    ConfigValue::Float(f) => println!("浮点配置 {}: {}", key, f),
                    ConfigValue::String(s) => println!("字符串配置 {}: {}", key, s),
                    _ => println!("复杂配置 {}: {:?}", key, value),
                }
            }
        }
        _ => println!("配置格式错误"),
    }
    println!();
}

// 事件处理系统
fn demonstrate_event_system() {
    println!("--- 事件处理系统 ---");

    #[derive(Debug, Clone)]
    enum UIEvent {
        Click { x: i32, y: i32, button: MouseButton },
        KeyPress { key: char, modifiers: Vec<Modifier> },
        Scroll { delta_x: i32, delta_y: i32 },
        Resize { width: u32, height: u32 },
        Focus { element_id: String },
        Custom { event_type: String, data: String },
    }

    #[derive(Debug, Clone)]
    enum MouseButton {
        Left,
        Right,
        Middle,
    }

    #[derive(Debug, Clone, PartialEq)]
    enum Modifier {
        Ctrl,
        Alt,
        Shift,
        Meta,
    }

    fn handle_ui_event(event: UIEvent) {
        match event {
            UIEvent::Click {
                x,
                y,
                button: MouseButton::Left,
            } if x > 100 && y > 100 => {
                println!("左键点击在区域内: ({}, {})", x, y);
            }
            UIEvent::Click { x, y, button } => {
                println!("点击事件: {:?} 按钮在 ({}, {})", button, x, y);
            }
            UIEvent::KeyPress {
                key: 'q',
                modifiers,
            } if modifiers.contains(&Modifier::Ctrl) => {
                println!("Ctrl+Q 退出快捷键");
            }
            UIEvent::KeyPress { key, modifiers } if !modifiers.is_empty() => {
                println!("组合键: {:?} + {}", modifiers, key);
            }
            UIEvent::KeyPress { key, .. } => {
                println!("按键: {}", key);
            }
            UIEvent::Scroll {
                delta_x: 0,
                delta_y,
            } => {
                println!("垂直滚动: {}", delta_y);
            }
            UIEvent::Scroll {
                delta_x,
                delta_y: 0,
            } => {
                println!("水平滚动: {}", delta_x);
            }
            UIEvent::Scroll { delta_x, delta_y } => {
                println!("对角滚动: ({}, {})", delta_x, delta_y);
            }
            UIEvent::Resize { width, height } if width * height > 1000000 => {
                println!("大窗口调整: {}x{}", width, height);
            }
            UIEvent::Resize { width, height } => {
                println!("窗口调整: {}x{}", width, height);
            }
            UIEvent::Focus { element_id } => {
                println!("元素获得焦点: {}", element_id);
            }
            UIEvent::Custom { event_type, data } => {
                println!("自定义事件 {}: {}", event_type, data);
            }
        }
    }

    // 模拟事件处理
    let events = vec![
        UIEvent::Click {
            x: 150,
            y: 200,
            button: MouseButton::Left,
        },
        UIEvent::Click {
            x: 50,
            y: 50,
            button: MouseButton::Right,
        },
        UIEvent::KeyPress {
            key: 'q',
            modifiers: vec![Modifier::Ctrl],
        },
        UIEvent::KeyPress {
            key: 'a',
            modifiers: vec![Modifier::Ctrl, Modifier::Shift],
        },
        UIEvent::KeyPress {
            key: 'x',
            modifiers: vec![],
        },
        UIEvent::Scroll {
            delta_x: 0,
            delta_y: -10,
        },
        UIEvent::Scroll {
            delta_x: 5,
            delta_y: 0,
        },
        UIEvent::Resize {
            width: 1920,
            height: 1080,
        },
        UIEvent::Focus {
            element_id: "input-field".to_string(),
        },
        UIEvent::Custom {
            event_type: "user-action".to_string(),
            data: "button-clicked".to_string(),
        },
    ];

    for event in events {
        handle_ui_event(event);
    }
    println!();
}

// 数据验证
fn demonstrate_data_validation() {
    println!("--- 数据验证 ---");

    #[derive(Debug)]
    enum ValidationError {
        TooShort { field: String, min_length: usize },
        TooLong { field: String, max_length: usize },
        InvalidFormat { field: String, expected: String },
        OutOfRange { field: String, min: i32, max: i32 },
        Required { field: String },
    }

    #[derive(Debug)]
    struct UserInput {
        username: Option<String>,
        email: Option<String>,
        age: Option<i32>,
        password: Option<String>,
    }

    fn validate_user_input(input: UserInput) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // 验证用户名
        match input.username {
            None => errors.push(ValidationError::Required {
                field: "username".to_string(),
            }),
            Some(ref username) if username.len() < 3 => {
                errors.push(ValidationError::TooShort {
                    field: "username".to_string(),
                    min_length: 3,
                });
            }
            Some(ref username) if username.len() > 20 => {
                errors.push(ValidationError::TooLong {
                    field: "username".to_string(),
                    max_length: 20,
                });
            }
            Some(ref username) if !username.chars().all(|c| c.is_alphanumeric() || c == '_') => {
                errors.push(ValidationError::InvalidFormat {
                    field: "username".to_string(),
                    expected: "字母数字和下划线".to_string(),
                });
            }
            _ => {} // 验证通过
        }

        // 验证邮箱
        match input.email {
            None => errors.push(ValidationError::Required {
                field: "email".to_string(),
            }),
            Some(ref email) if !email.contains('@') || !email.contains('.') => {
                errors.push(ValidationError::InvalidFormat {
                    field: "email".to_string(),
                    expected: "有效的邮箱格式".to_string(),
                });
            }
            _ => {}
        }

        // 验证年龄
        match input.age {
            None => errors.push(ValidationError::Required {
                field: "age".to_string(),
            }),
            Some(age) if age < 13 || age > 120 => {
                errors.push(ValidationError::OutOfRange {
                    field: "age".to_string(),
                    min: 13,
                    max: 120,
                });
            }
            _ => {}
        }

        // 验证密码
        match input.password {
            None => errors.push(ValidationError::Required {
                field: "password".to_string(),
            }),
            Some(ref password) if password.len() < 8 => {
                errors.push(ValidationError::TooShort {
                    field: "password".to_string(),
                    min_length: 8,
                });
            }
            _ => {}
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    // 测试数据验证
    let test_inputs = vec![
        UserInput {
            username: Some("ab".to_string()),
            email: Some("invalid-email".to_string()),
            age: Some(10),
            password: Some("123".to_string()),
        },
        UserInput {
            username: Some("valid_user".to_string()),
            email: Some("user@example.com".to_string()),
            age: Some(25),
            password: Some("secure_password".to_string()),
        },
        UserInput {
            username: None,
            email: None,
            age: None,
            password: None,
        },
    ];

    for (i, input) in test_inputs.into_iter().enumerate() {
        println!("验证输入 {}:", i + 1);
        match validate_user_input(input) {
            Ok(()) => println!("✓ 验证通过"),
            Err(errors) => {
                println!("✗ 验证失败:");
                for error in errors {
                    match error {
                        ValidationError::Required { field } => {
                            println!("  - {} 是必填字段", field);
                        }
                        ValidationError::TooShort { field, min_length } => {
                            println!("  - {} 太短，最少需要 {} 个字符", field, min_length);
                        }
                        ValidationError::TooLong { field, max_length } => {
                            println!("  - {} 太长，最多允许 {} 个字符", field, max_length);
                        }
                        ValidationError::InvalidFormat { field, expected } => {
                            println!("  - {} 格式无效，期望: {}", field, expected);
                        }
                        ValidationError::OutOfRange { field, min, max } => {
                            println!("  - {} 超出范围，应在 {} 到 {} 之间", field, min, max);
                        }
                    }
                }
            }
        }
        println!();
    }
}

// 协议解析
fn demonstrate_protocol_parsing() {
    println!("--- 协议解析 ---");

    #[derive(Debug)]
    enum HttpMethod {
        GET,
        POST,
        PUT,
        DELETE,
        PATCH,
    }

    #[derive(Debug)]
    struct HttpRequest {
        method: HttpMethod,
        path: String,
        version: String,
        headers: std::collections::HashMap<String, String>,
        body: Option<String>,
    }

    #[derive(Debug)]
    enum ParseError {
        InvalidMethod(String),
        InvalidFormat,
        MissingPath,
        InvalidHeader(String),
    }

    fn parse_http_method(method_str: &str) -> Result<HttpMethod, ParseError> {
        match method_str.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "PATCH" => Ok(HttpMethod::PATCH),
            _ => Err(ParseError::InvalidMethod(method_str.to_string())),
        }
    }

    fn parse_http_request(request_str: &str) -> Result<HttpRequest, ParseError> {
        let lines: Vec<&str> = request_str.lines().collect();

        if lines.is_empty() {
            return Err(ParseError::InvalidFormat);
        }

        // 解析请求行
        let request_line_parts: Vec<&str> = lines[0].split_whitespace().collect();
        if request_line_parts.len() != 3 {
            return Err(ParseError::InvalidFormat);
        }

        let method = parse_http_method(request_line_parts[0])?;
        let path = request_line_parts[1].to_string();
        let version = request_line_parts[2].to_string();

        if path.is_empty() {
            return Err(ParseError::MissingPath);
        }

        // 解析头部
        let mut headers = std::collections::HashMap::new();
        let mut body_start = lines.len();

        for (i, line) in lines.iter().enumerate().skip(1) {
            if line.is_empty() {
                body_start = i + 1;
                break;
            }

            match line.split_once(':') {
                Some((key, value)) => {
                    headers.insert(key.trim().to_lowercase(), value.trim().to_string());
                }
                None => return Err(ParseError::InvalidHeader(line.to_string())),
            }
        }

        // 解析请求体
        let body = if body_start < lines.len() {
            Some(lines[body_start..].join("\n"))
        } else {
            None
        };

        Ok(HttpRequest {
            method,
            path,
            version,
            headers,
            body,
        })
    }

    fn handle_http_request(request: HttpRequest) {
        match request {
            HttpRequest {
                method: HttpMethod::GET,
                path,
                ..
            } if path == "/" => {
                println!("处理首页GET请求");
            }
            HttpRequest {
                method: HttpMethod::GET,
                path,
                ..
            } if path.starts_with("/api/") => {
                println!("处理API GET请求: {}", path);
            }
            HttpRequest {
                method: HttpMethod::POST,
                path,
                body: Some(body),
                ..
            } if path == "/api/users" => {
                println!("创建用户请求，数据: {}", body);
            }
            HttpRequest {
                method: HttpMethod::PUT,
                path,
                body: Some(body),
                ..
            } if path.starts_with("/api/users/") => {
                let user_id = &path["/api/users/".len()..];
                println!("更新用户 {} 的数据: {}", user_id, body);
            }
            HttpRequest {
                method: HttpMethod::DELETE,
                path,
                ..
            } if path.starts_with("/api/users/") => {
                let user_id = &path["/api/users/".len()..];
                println!("删除用户: {}", user_id);
            }
            HttpRequest {
                method,
                path,
                headers,
                ..
            } if headers.contains_key("authorization") => {
                println!("认证请求: {:?} {}", method, path);
            }
            HttpRequest { method, path, .. } => {
                println!("未处理的请求: {:?} {}", method, path);
            }
        }
    }

    // 测试HTTP请求解析
    let test_requests = vec![
        "GET / HTTP/1.1\nHost: example.com\nUser-Agent: Mozilla/5.0\n\n",
        "POST /api/users HTTP/1.1\nContent-Type: application/json\nContent-Length: 25\n\n{\"name\": \"John Doe\"}",
        "PUT /api/users/123 HTTP/1.1\nAuthorization: Bearer token123\nContent-Type: application/json\n\n{\"age\": 30}",
        "DELETE /api/users/456 HTTP/1.1\nAuthorization: Bearer token456\n\n",
        "INVALID REQUEST",
    ];

    for (i, request_str) in test_requests.iter().enumerate() {
        println!("解析请求 {}:", i + 1);
        match parse_http_request(request_str) {
            Ok(request) => {
                println!("✓ 解析成功: {:?}", request);
                handle_http_request(request);
            }
            Err(error) => {
                println!("✗ 解析失败: {:?}", error);
            }
        }
        println!();
    }

    println!("=== 模式匹配应用场景分析完成 ===");
}

// 总结和最佳实践
/*
模式匹配最佳实践总结：

1. **优先使用 match 而不是 if-else 链**
   - match 确保穷尽性检查
   - 编译器会检查是否遗漏了某些情况
   - 代码更清晰、更易维护

2. **合理使用 if let 和 while let**
   - 当只关心一种模式时使用 if let
   - 循环处理 Option 或 Result 时使用 while let
   - 避免不必要的 match 嵌套

3. **善用守卫条件**
   - 在模式匹配中添加额外的逻辑判断
   - 使代码更具表达力
   - 避免在匹配臂中使用复杂的 if 语句

4. **使用 @ 绑定捕获值**
   - 在需要同时匹配模式和使用值时使用
   - 特别适用于范围匹配和复杂条件

5. **合理忽略不需要的值**
   - 使用 _ 忽略单个值
   - 使用 .. 忽略多个值
   - 使用 _name 保留绑定但避免未使用警告

6. **深度解构复杂数据**
    - 一次性解构嵌套结构
    - 减少中间变量的使用

 7. **性能考虑**
    - 模式匹配在编译时优化
    - 避免不必要的克隆和移动
    - 使用引用模式保持所有权

 8. **错误处理集成**
    - 结合 Result 和 Option 进行错误处理
    - 使用 ? 操作符简化错误传播
    - 自定义错误类型提供更好的错误信息

 9. **实际应用建议**
    - 状态机：清晰表达状态转换逻辑
    - 解析器：处理复杂的输入格式
    - 事件系统：分发和处理不同类型的事件
    - 配置管理：类型安全的配置解析
    - 数据验证：全面的输入验证逻辑

模式匹配是 Rust 最强大的特性之一，掌握它能让代码更安全、更清晰、更易维护。
*/
