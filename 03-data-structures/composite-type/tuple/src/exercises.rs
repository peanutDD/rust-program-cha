//! # Rust 元组练习题集
//!
//! 本文件包含了从基础到高级的元组练习题
//! 涵盖元组的各种用法和应用场景

/// # 练习1：基础元组操作
///
/// 完成基础的元组创建、访问和解构操作
pub fn exercise_1_basic_operations() {
    println!("=== 练习1：基础元组操作 ===");

    // TODO: 创建一个包含你的姓名(String)、年龄(u32)和身高(f64)的元组
    let personal_info = (String::from("张三"), 25u32, 175.5f64);

    // TODO: 使用索引访问打印每个字段
    println!("姓名: {}", personal_info.0);
    println!("年龄: {}", personal_info.1);
    println!("身高: {:.1}cm", personal_info.2);

    // TODO: 使用解构赋值重新获取这些值
    let (name, age, height) = personal_info;
    println!(
        "解构后 - 姓名: {}, 年龄: {}, 身高: {:.1}cm",
        name, age, height
    );

    // TODO: 创建一个单元素元组（注意语法）
    let single = (42,);
    println!("单元素元组: {:?}", single);

    // TODO: 创建空元组
    let unit = ();
    println!("空元组: {:?}", unit);
}

/// # 练习2：元组类型和嵌套
///
/// 练习复杂的元组类型定义和嵌套结构
pub fn exercise_2_types_and_nesting() {
    println!("\n=== 练习2：元组类型和嵌套 ===");

    // TODO: 定义类型别名
    type Point2D = (f64, f64);
    type Point3D = (f64, f64, f64);
    type Line2D = (Point2D, Point2D);
    type Triangle2D = (Point2D, Point2D, Point2D);

    // TODO: 创建几个点
    let origin: Point2D = (0.0, 0.0);
    let point_a: Point2D = (3.0, 4.0);
    let point_b: Point2D = (6.0, 8.0);

    // TODO: 创建一条线段
    let line: Line2D = (origin, point_a);
    println!("线段: 从 {:?} 到 {:?}", line.0, line.1);

    // TODO: 创建一个三角形
    let triangle: Triangle2D = (origin, point_a, point_b);
    println!("三角形顶点: {:?}", triangle);

    // TODO: 解构三角形并计算周长
    let ((x1, y1), (x2, y2), (x3, y3)) = triangle;
    let side1 = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
    let side2 = ((x3 - x2).powi(2) + (y3 - y2).powi(2)).sqrt();
    let side3 = ((x1 - x3).powi(2) + (y1 - y3).powi(2)).sqrt();
    let perimeter = side1 + side2 + side3;
    println!("三角形周长: {:.2}", perimeter);

    // TODO: 创建3D点和复杂嵌套结构
    let point_3d: Point3D = (1.0, 2.0, 3.0);
    let complex_data = (
        "几何数据",
        triangle,
        point_3d,
        ("元数据", ("创建时间", "2024-01-01"), ("作者", "Rust学习者")),
    );

    println!("复杂数据结构: {:?}", complex_data);
}

/// # 练习3：模式匹配和条件处理
///
/// 练习在模式匹配中使用元组
pub fn exercise_3_pattern_matching() {
    println!("\n=== 练习3：模式匹配和条件处理 ===");

    // TODO: 实现一个函数，根据坐标判断象限
    fn determine_quadrant(point: (i32, i32)) -> &'static str {
        match point {
            (0, 0) => "原点",
            (0, _) => "x轴",
            (_, 0) => "y轴",
            (x, y) if x > 0 && y > 0 => "第一象限",
            (x, y) if x < 0 && y > 0 => "第二象限",
            (x, y) if x < 0 && y < 0 => "第三象限",
            (x, y) if x > 0 && y < 0 => "第四象限",
            _ => "未知位置",
        }
    }

    let points = vec![(0, 0), (3, 4), (-2, 5), (-3, -4), (2, -3), (0, 5), (3, 0)];
    for point in points {
        println!("点 {:?} 位于: {}", point, determine_quadrant(point));
    }

    // TODO: 实现成绩等级判断
    fn grade_analysis(score_info: (u32, &str)) -> String {
        match score_info {
            (score, subject) if score >= 90 => format!("{}: {}分 - 优秀", subject, score),
            (score, subject) if score >= 80 => format!("{}: {}分 - 良好", subject, score),
            (score, subject) if score >= 70 => format!("{}: {}分 - 中等", subject, score),
            (score, subject) if score >= 60 => format!("{}: {}分 - 及格", subject, score),
            (score, subject) => format!("{}: {}分 - 不及格", subject, score),
        }
    }

    let grades = vec![(95, "数学"), (82, "英语"), (76, "物理"), (58, "化学")];

    for grade in grades {
        println!("{}", grade_analysis(grade));
    }

    // TODO: 使用 if let 进行选择性匹配
    let maybe_coordinate = Some((10, 20));
    if let Some((x, y)) = maybe_coordinate {
        println!(
            "坐标存在: ({}, {}), 距离原点: {:.2}",
            x,
            y,
            ((x * x + y * y) as f64).sqrt()
        );
    }
}

/// # 练习4：函数参数和返回值
///
/// 练习在函数中使用元组
pub fn exercise_4_functions() {
    println!("\n=== 练习4：函数参数和返回值 ===");

    // TODO: 实现一个函数，接受两个点，返回中点和距离
    fn point_analysis(p1: (f64, f64), p2: (f64, f64)) -> ((f64, f64), f64) {
        let midpoint = ((p1.0 + p2.0) / 2.0, (p1.1 + p2.1) / 2.0);
        let distance = ((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2)).sqrt();
        (midpoint, distance)
    }

    let point1 = (1.0, 2.0);
    let point2 = (4.0, 6.0);
    let (midpoint, distance) = point_analysis(point1, point2);
    println!(
        "点 {:?} 和 {:?} 的中点: {:?}, 距离: {:.2}",
        point1, point2, midpoint, distance
    );

    // TODO: 实现统计函数，返回多个统计值
    fn statistics(numbers: &[i32]) -> (i32, i32, i32, i32) {
        if numbers.is_empty() {
            return (0, 0, 0, 0);
        }

        let sum: i32 = numbers.iter().sum();
        let max = *numbers.iter().max().unwrap();
        let min = *numbers.iter().min().unwrap();
        let avg = sum / numbers.len() as i32;

        (sum, max, avg, min)
    }

    let data = [5, 2, 8, 1, 9, 3];
    let (sum, max, avg, min) = statistics(&data);
    println!(
        "数据 {:?} 的统计: 总和={}, 最大={}, 平均={}, 最小={}",
        data, sum, max, avg, min
    );

    // TODO: 实现矩形计算函数
    fn rectangle_info(top_left: (f64, f64), bottom_right: (f64, f64)) -> (f64, f64, f64) {
        let width = bottom_right.0 - top_left.0;
        let height = top_left.1 - bottom_right.1;
        let area = width * height;
        let perimeter = 2.0 * (width + height);

        (area, perimeter, width * height)
    }

    let rect_tl = (1.0, 5.0);
    let rect_br = (4.0, 2.0);
    let (area, perimeter, _) = rectangle_info(rect_tl, rect_br);
    println!(
        "矩形 {:?} 到 {:?}: 面积={:.1}, 周长={:.1}",
        rect_tl, rect_br, area, perimeter
    );
}

/// # 练习5：实用工具函数
///
/// 实现一些实用的元组操作工具
pub fn exercise_5_utilities() {
    println!("\n=== 练习5：实用工具函数 ===");

    // TODO: 实现元组交换函数
    fn swap<T, U>(tuple: (T, U)) -> (U, T) {
        (tuple.1, tuple.0)
    }

    let original = ("hello", 42);
    let swapped = swap(original);
    println!("交换: {:?} -> {:?}", ("hello", 42), swapped);

    // TODO: 实现元组映射函数
    fn map_both<T, U, F>(tuple: (T, T), f: F) -> (U, U)
    where
        F: Fn(T) -> U,
        T: Copy,
    {
        (f(tuple.0), f(tuple.1))
    }

    let numbers = (3, 7);
    let squared = map_both(numbers, |x| x * x);
    let doubled = map_both(numbers, |x| x * 2);
    println!(
        "原始: {:?}, 平方: {:?}, 双倍: {:?}",
        numbers, squared, doubled
    );

    // TODO: 实现元组比较函数
    fn compare_tuples(t1: (i32, i32), t2: (i32, i32)) -> std::cmp::Ordering {
        match t1.0.cmp(&t2.0) {
            std::cmp::Ordering::Equal => t1.1.cmp(&t2.1),
            other => other,
        }
    }

    let pairs = vec![(1, 3), (2, 1), (1, 5), (2, 2)];
    let mut sorted_pairs = pairs.clone();
    sorted_pairs.sort_by(|&a, &b| compare_tuples(a, b));
    println!("排序前: {:?}", pairs);
    println!("排序后: {:?}", sorted_pairs);

    // TODO: 实现向量运算
    fn vector_add(v1: (f64, f64), v2: (f64, f64)) -> (f64, f64) {
        (v1.0 + v2.0, v1.1 + v2.1)
    }

    fn vector_dot_product(v1: (f64, f64), v2: (f64, f64)) -> f64 {
        v1.0 * v2.0 + v1.1 * v2.1
    }

    fn vector_magnitude(v: (f64, f64)) -> f64 {
        (v.0 * v.0 + v.1 * v.1).sqrt()
    }

    let vec1 = (3.0, 4.0);
    let vec2 = (1.0, 2.0);
    let sum = vector_add(vec1, vec2);
    let dot = vector_dot_product(vec1, vec2);
    let mag1 = vector_magnitude(vec1);

    println!("向量 {:?} + {:?} = {:?}", vec1, vec2, sum);
    println!("向量 {:?} · {:?} = {:.2}", vec1, vec2, dot);
    println!("向量 {:?} 的模长: {:.2}", vec1, mag1);
}

/// # 练习6：错误处理和选项类型
///
/// 练习在错误处理中使用元组
pub fn exercise_6_error_handling() {
    println!("\n=== 练习6：错误处理和选项类型 ===");

    // TODO: 实现安全的除法函数
    fn safe_divide(dividend: f64, divisor: f64) -> Result<(f64, f64), &'static str> {
        if divisor == 0.0 {
            Err("除数不能为零")
        } else {
            let quotient = dividend / divisor;
            let remainder = dividend % divisor;
            Ok((quotient, remainder))
        }
    }

    let test_cases = vec![(10.0, 3.0), (15.0, 0.0), (7.5, 2.5)];
    for (a, b) in test_cases {
        match safe_divide(a, b) {
            Ok((q, r)) => println!("{} ÷ {} = {:.2} 余 {:.2}", a, b, q, r),
            Err(e) => println!("{} ÷ {} 错误: {}", a, b, e),
        }
    }

    // TODO: 实现坐标解析函数
    fn parse_coordinate(input: &str) -> Option<(i32, i32)> {
        let parts: Vec<&str> = input.split(',').collect();
        if parts.len() != 2 {
            return None;
        }

        let x = parts[0].trim().parse().ok()?;
        let y = parts[1].trim().parse().ok()?;

        Some((x, y))
    }

    let coordinate_strings = vec!["10,20", "3, 4", "invalid", "1,2,3", "-5, 7"];
    for coord_str in coordinate_strings {
        match parse_coordinate(coord_str) {
            Some((x, y)) => println!("解析 '{}' -> 坐标({}, {})", coord_str, x, y),
            None => println!("解析 '{}' 失败", coord_str),
        }
    }

    // TODO: 实现批量处理函数
    fn process_coordinates(coords: Vec<&str>) -> (Vec<(i32, i32)>, Vec<String>) {
        let mut valid = Vec::new();
        let mut errors = Vec::new();

        for coord_str in coords {
            match parse_coordinate(coord_str) {
                Some(coord) => valid.push(coord),
                None => errors.push(format!("无效坐标: '{}'", coord_str)),
            }
        }

        (valid, errors)
    }

    let input_coords = vec!["1,2", "3,4", "invalid", "5,6", "bad,data"];
    let (valid_coords, error_msgs) = process_coordinates(input_coords);
    println!("有效坐标: {:?}", valid_coords);
    println!("错误信息: {:?}", error_msgs);
}

/// # 练习7：迭代器和集合操作
///
/// 练习在迭代器中使用元组
pub fn exercise_7_iterators() {
    println!("\n=== 练习7：迭代器和集合操作 ===");

    // TODO: 处理学生成绩数据
    let students = vec![
        ("Alice", 85, "Math"),
        ("Bob", 92, "Physics"),
        ("Charlie", 78, "Math"),
        ("Diana", 96, "Physics"),
        ("Eve", 88, "Math"),
    ];

    // 过滤高分学生
    let high_scorers: Vec<_> = students
        .iter()
        .filter(|(_, score, _)| *score >= 90)
        .collect();
    println!("高分学生 (>=90): {:?}", high_scorers);

    // 按科目分组计算平均分
    let math_scores: Vec<_> = students
        .iter()
        .filter(|(_, _, subject)| *subject == "Math")
        .map(|(_, score, _)| *score)
        .collect();

    let physics_scores: Vec<_> = students
        .iter()
        .filter(|(_, _, subject)| *subject == "Physics")
        .map(|(_, score, _)| *score)
        .collect();

    let math_avg = if !math_scores.is_empty() {
        math_scores.iter().sum::<u32>() as f64 / math_scores.len() as f64
    } else {
        0.0
    };
    let physics_avg = if !physics_scores.is_empty() {
        physics_scores.iter().sum::<u32>() as f64 / physics_scores.len() as f64
    } else {
        0.0
    };

    println!("数学平均分: {:.1}", math_avg);
    println!("物理平均分: {:.1}", physics_avg);

    // TODO: 坐标变换
    let points = vec![(1, 2), (3, 4), (5, 6)];

    // 平移变换
    let translated: Vec<_> = points.iter().map(|(x, y)| (x + 10, y + 20)).collect();
    println!("平移后: {:?}", translated);

    // 缩放变换
    let scaled: Vec<_> = points.iter().map(|(x, y)| (x * 2, y * 2)).collect();
    println!("缩放后: {:?}", scaled);

    // 计算距离
    let distances: Vec<_> = points
        .iter()
        .map(|(x, y)| ((*x as f64).powi(2) + (*y as f64).powi(2)).sqrt())
        .collect();
    println!("到原点距离: {:?}", distances);

    // TODO: 配对操作
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![25, 30, 35];

    let paired: Vec<_> = names.iter().zip(ages.iter()).collect();
    println!("配对结果: {:?}", paired);

    // 枚举索引
    let indexed: Vec<_> = names.iter().enumerate().collect();
    println!("带索引: {:?}", indexed);
}

/// # 练习8：状态机和配置
///
/// 使用元组实现状态机和配置管理
pub fn exercise_8_state_machine() {
    println!("\n=== 练习8：状态机和配置 ===");

    // TODO: 实现简单的状态机
    #[derive(Debug, Clone, PartialEq)]
    enum State {
        Idle,
        Running,
        Paused,
        Stopped,
    }

    #[derive(Debug, Clone)]
    enum Event {
        Start,
        Pause,
        Resume,
        Stop,
        Reset,
    }

    fn state_transition(current: State, event: Event) -> (State, String) {
        match (current, event) {
            (State::Idle, Event::Start) => (State::Running, "开始运行".to_string()),
            (State::Running, Event::Pause) => (State::Paused, "暂停".to_string()),
            (State::Paused, Event::Resume) => (State::Running, "恢复运行".to_string()),
            (State::Running, Event::Stop) => (State::Stopped, "停止".to_string()),
            (State::Paused, Event::Stop) => (State::Stopped, "停止".to_string()),
            (State::Stopped, Event::Reset) => (State::Idle, "重置".to_string()),
            (state, event) => (state, format!("无效转换: {:?}", event)),
        }
    }

    let mut current_state = State::Idle;
    let events = vec![
        Event::Start,
        Event::Pause,
        Event::Resume,
        Event::Stop,
        Event::Reset,
    ];

    println!("初始状态: {:?}", current_state);
    for event in events {
        let (new_state, message) = state_transition(current_state.clone(), event.clone());
        println!("事件 {:?} -> 状态 {:?}: {}", event, new_state, message);
        current_state = new_state;
    }

    // TODO: 配置管理
    type DatabaseConfig = (String, u16, String, bool); // (host, port, database, ssl)
    type ServerConfig = (String, u16, u32, bool); // (bind_addr, port, max_connections, debug)
    type AppConfig = (DatabaseConfig, ServerConfig, String); // (db_config, server_config, log_level)

    fn create_config() -> AppConfig {
        let db_config = ("localhost".to_string(), 5432, "myapp".to_string(), true);

        let server_config = ("0.0.0.0".to_string(), 8080, 1000, false);

        (db_config, server_config, "info".to_string())
    }

    fn print_config(config: &AppConfig) {
        let ((db_host, db_port, db_name, db_ssl), (srv_addr, srv_port, max_conn, debug), log_level) =
            config;

        println!("应用配置:");
        println!(
            "  数据库: {}:{}/{} (SSL: {})",
            db_host, db_port, db_name, db_ssl
        );
        println!(
            "  服务器: {}:{} (最大连接: {}, 调试: {})",
            srv_addr, srv_port, max_conn, debug
        );
        println!("  日志级别: {}", log_level);
    }

    let config = create_config();
    print_config(&config);
}

/// # 练习9：性能和内存优化
///
/// 了解元组的性能特点
pub fn exercise_9_performance() {
    println!("\n=== 练习9：性能和内存优化 ===");

    // TODO: 内存大小比较
    println!("内存大小比较:");
    println!("  (u8, u8): {} bytes", std::mem::size_of::<(u8, u8)>());
    println!("  (u8, u16): {} bytes", std::mem::size_of::<(u8, u16)>());
    println!("  (u8, u32): {} bytes", std::mem::size_of::<(u8, u32)>());
    println!("  (u8, u64): {} bytes", std::mem::size_of::<(u8, u64)>());
    println!(
        "  (u16, u8, u32): {} bytes",
        std::mem::size_of::<(u16, u8, u32)>()
    );

    // TODO: 对齐优化示例
    #[repr(C)]
    struct UnoptimizedStruct {
        a: u8,
        b: u64,
        c: u8,
    }

    #[repr(C)]
    struct OptimizedStruct {
        b: u64,
        a: u8,
        c: u8,
    }

    println!("\n结构体大小比较:");
    println!(
        "  未优化结构体: {} bytes",
        std::mem::size_of::<UnoptimizedStruct>()
    );
    println!(
        "  优化后结构体: {} bytes",
        std::mem::size_of::<OptimizedStruct>()
    );
    println!(
        "  等效元组 (u8, u64, u8): {} bytes",
        std::mem::size_of::<(u8, u64, u8)>()
    );
    println!(
        "  优化元组 (u64, u8, u8): {} bytes",
        std::mem::size_of::<(u64, u8, u8)>()
    );

    // TODO: 零成本抽象验证
    let data = (1, 2, 3, 4, 5);

    // 方法1：直接访问
    let sum1 = data.0 + data.1 + data.2 + data.3 + data.4;

    // 方法2：解构访问
    let (a, b, c, d, e) = data;
    let sum2 = a + b + c + d + e;

    println!("\n零成本抽象验证:");
    println!("  直接访问求和: {}", sum1);
    println!("  解构访问求和: {}", sum2);
    println!("  结果相同，性能相同（零成本抽象）");

    // TODO: 大量数据处理性能测试
    let points: Vec<(i32, i32)> = (0..1000).map(|i| (i, i * 2)).collect();

    // 使用解构的方式计算
    let sum_destructured: i64 = points.iter().map(|(x, y)| (*x as i64) + (*y as i64)).sum();

    // 使用索引的方式计算
    let sum_indexed: i64 = points
        .iter()
        .map(|point| (point.0 as i64) + (point.1 as i64))
        .sum();

    println!("\n大量数据处理:");
    println!("  解构方式求和: {}", sum_destructured);
    println!("  索引方式求和: {}", sum_indexed);
    println!("  两种方式性能相同");
}

/// # 练习10：综合应用项目
///
/// 综合运用元组知识的实际项目
pub fn exercise_10_comprehensive_project() {
    println!("\n=== 练习10：综合应用项目 - 简单图形计算器 ===");

    // TODO: 定义几何图形类型
    type Point = (f64, f64);
    type Circle = (Point, f64); // (center, radius)
    type Rectangle = (Point, Point); // (top_left, bottom_right)
    type Triangle = (Point, Point, Point); // (vertex1, vertex2, vertex3)

    // TODO: 实现几何计算函数
    fn distance(p1: Point, p2: Point) -> f64 {
        let (x1, y1) = p1;
        let (x2, y2) = p2;
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }

    fn circle_area(circle: Circle) -> f64 {
        let (_, radius) = circle;
        std::f64::consts::PI * radius * radius
    }

    fn circle_circumference(circle: Circle) -> f64 {
        let (_, radius) = circle;
        2.0 * std::f64::consts::PI * radius
    }

    fn rectangle_area(rect: Rectangle) -> f64 {
        let ((x1, y1), (x2, y2)) = rect;
        (x2 - x1).abs() * (y1 - y2).abs()
    }

    fn rectangle_perimeter(rect: Rectangle) -> f64 {
        let ((x1, y1), (x2, y2)) = rect;
        2.0 * ((x2 - x1).abs() + (y1 - y2).abs())
    }

    fn triangle_area(triangle: Triangle) -> f64 {
        let ((x1, y1), (x2, y2), (x3, y3)) = triangle;
        // 使用叉积公式
        0.5 * ((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).abs())
    }

    fn triangle_perimeter(triangle: Triangle) -> f64 {
        let (p1, p2, p3) = triangle;
        distance(p1, p2) + distance(p2, p3) + distance(p3, p1)
    }

    // TODO: 创建几何图形并计算
    println!("\n🔵 圆形计算:");
    let circle: Circle = ((0.0, 0.0), 5.0);
    println!("  圆心: {:?}, 半径: {}", circle.0, circle.1);
    println!("  面积: {:.2}", circle_area(circle));
    println!("  周长: {:.2}", circle_circumference(circle));

    println!("\n🔲 矩形计算:");
    let rectangle: Rectangle = ((1.0, 4.0), (6.0, 1.0));
    println!("  左上角: {:?}, 右下角: {:?}", rectangle.0, rectangle.1);
    println!("  面积: {:.2}", rectangle_area(rectangle));
    println!("  周长: {:.2}", rectangle_perimeter(rectangle));

    println!("\n🔺 三角形计算:");
    let triangle: Triangle = ((0.0, 0.0), (4.0, 0.0), (2.0, 3.0));
    println!(
        "  顶点: {:?}, {:?}, {:?}",
        triangle.0, triangle.1, triangle.2
    );
    println!("  面积: {:.2}", triangle_area(triangle));
    println!("  周长: {:.2}", triangle_perimeter(triangle));

    // TODO: 批量处理和统计
    let shapes_data = vec![
        ("圆1", circle_area(((0.0, 0.0), 3.0))),
        ("圆2", circle_area(((1.0, 1.0), 2.0))),
        ("矩形1", rectangle_area(((0.0, 0.0), (3.0, 4.0)))),
        ("矩形2", rectangle_area(((1.0, 1.0), (5.0, 3.0)))),
        (
            "三角形1",
            triangle_area(((0.0, 0.0), (3.0, 0.0), (1.5, 2.0))),
        ),
    ];

    println!("\n📊 图形面积统计:");
    let total_area: f64 = shapes_data.iter().map(|(_, area)| area).sum();
    let max_area = shapes_data
        .iter()
        .map(|(_, area)| area)
        .fold(0.0f64, |a, &b| a.max(b));
    let min_area = shapes_data
        .iter()
        .map(|(_, area)| area)
        .fold(f64::INFINITY, |a, &b| a.min(b));

    for (name, area) in &shapes_data {
        println!("  {}: {:.2}", name, area);
    }

    println!("\n📈 统计结果:");
    println!("  总面积: {:.2}", total_area);
    println!("  平均面积: {:.2}", total_area / shapes_data.len() as f64);
    println!("  最大面积: {:.2}", max_area);
    println!("  最小面积: {:.2}", min_area);

    // TODO: 碰撞检测示例
    fn point_in_circle(point: Point, circle: Circle) -> bool {
        let (center, radius) = circle;
        distance(point, center) <= radius
    }

    fn point_in_rectangle(point: Point, rect: Rectangle) -> bool {
        let (x, y) = point;
        let ((x1, y1), (x2, y2)) = rect;
        let (min_x, max_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (min_y, max_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        x >= min_x && x <= max_x && y >= min_y && y <= max_y
    }

    println!("\n🎯 碰撞检测:");
    let test_points = vec![(2.0, 2.0), (0.0, 0.0), (10.0, 10.0)];
    let test_circle = ((0.0, 0.0), 3.0);
    let test_rect = ((1.0, 1.0), (5.0, 5.0));

    for point in test_points {
        let in_circle = point_in_circle(point, test_circle);
        let in_rect = point_in_rectangle(point, test_rect);
        println!(
            "  点 {:?}: 在圆内={}, 在矩形内={}",
            point, in_circle, in_rect
        );
    }
}

/// # 运行所有练习
pub fn run_all_exercises() {
    println!("🦀 Rust 元组练习题集");
    println!("========================\n");

    exercise_1_basic_operations();
    exercise_2_types_and_nesting();
    exercise_3_pattern_matching();
    exercise_4_functions();
    exercise_5_utilities();
    exercise_6_error_handling();
    exercise_7_iterators();
    exercise_8_state_machine();
    exercise_9_performance();
    exercise_10_comprehensive_project();

    println!("\n========================");
    println!("🎉 所有练习完成！");
    println!("\n💡 练习要点总结:");
    println!("• 掌握了元组的基本操作和语法");
    println!("• 学会了复杂嵌套结构的处理");
    println!("• 熟练运用模式匹配和解构");
    println!("• 理解了元组在函数中的应用");
    println!("• 掌握了实用工具函数的编写");
    println!("• 学会了错误处理和选项类型");
    println!("• 熟悉了迭代器和集合操作");
    println!("• 了解了状态机和配置管理");
    println!("• 理解了性能和内存优化");
    println!("• 完成了综合应用项目");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let tuple = (1, 2, 3);
        let (a, b, c) = tuple;
        assert_eq!(a + b + c, 6);
    }

    #[test]
    fn test_nested_tuples() {
        let nested = ((1, 2), (3, 4));
        assert_eq!(nested.0 .0 + nested.1 .1, 5);
    }

    #[test]
    fn test_pattern_matching() {
        let point = (0, 5);
        let result = match point {
            (0, _) => "y-axis",
            (_, 0) => "x-axis",
            _ => "other",
        };
        assert_eq!(result, "y-axis");
    }

    #[test]
    fn test_utility_functions() {
        fn swap<T, U>(tuple: (T, U)) -> (U, T) {
            (tuple.1, tuple.0)
        }

        let original = (1, "hello");
        let swapped = swap(original);
        assert_eq!(swapped, ("hello", 1));
    }

    #[test]
    fn test_error_handling() {
        fn safe_divide(a: f64, b: f64) -> Result<f64, &'static str> {
            if b == 0.0 {
                Err("Division by zero")
            } else {
                Ok(a / b)
            }
        }

        assert!(safe_divide(10.0, 2.0).is_ok());
        assert!(safe_divide(10.0, 0.0).is_err());
    }
}
