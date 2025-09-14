//! # Rust 流程控制练习题
//!
//! 通过递进式练习掌握 Rust 流程控制的核心概念和实际应用

/// # 练习1: 条件语句基础
///
/// 掌握 if、if-else、if-else if-else 的使用
pub fn exercise1_conditional_basics() {
    println!("=== 练习1：条件语句基础 ===");

    // 1.1 成绩等级判定
    println!("\n1.1 成绩等级判定:");
    fn get_grade(score: u32) -> &'static str {
        if score >= 90 {
            "A"
        } else if score >= 80 {
            "B"
        } else if score >= 70 {
            "C"
        } else if score >= 60 {
            "D"
        } else {
            "F"
        }
    }

    let scores = [95, 87, 73, 65, 45];
    for score in scores {
        println!("分数 {} -> 等级 {}", score, get_grade(score));
    }

    // 1.2 年龄分组
    println!("\n1.2 年龄分组:");
    fn categorize_age(age: u32) -> String {
        match age {
            0..=2 => "婴儿".to_string(),
            3..=12 => "儿童".to_string(),
            13..=19 => "青少年".to_string(),
            20..=59 => "成年人".to_string(),
            60.. => "老年人".to_string(),
        }
    }

    let ages = [1, 8, 16, 35, 70];
    for age in ages {
        println!("年龄 {} -> 分组 {}", age, categorize_age(age));
    }

    // 1.3 BMI 计算和分类
    println!("\n1.3 BMI 计算和分类:");
    fn calculate_bmi_category(weight: f64, height: f64) -> String {
        let bmi = weight / (height * height);
        let category = if bmi < 18.5 {
            "偏瘦"
        } else if bmi < 24.0 {
            "正常"
        } else if bmi < 28.0 {
            "偏胖"
        } else {
            "肥胖"
        };
        format!("BMI: {:.1}, 分类: {}", bmi, category)
    }

    let people = [(65.0, 1.70), (80.0, 1.75), (55.0, 1.65)];
    for (weight, height) in people {
        println!(
            "体重{}kg, 身高{}m -> {}",
            weight,
            height,
            calculate_bmi_category(weight, height)
        );
    }

    // 1.4 复杂条件判断 - 贷款审批
    println!("\n1.4 贷款审批系统:");
    fn loan_approval(age: u32, income: u32, credit_score: u32, has_collateral: bool) -> String {
        if age < 18 || age > 65 {
            return "年龄不符合要求".to_string();
        }

        if income < 30000 {
            return "收入过低".to_string();
        }

        if credit_score < 600 {
            if has_collateral {
                "需要抵押物，可以考虑批准".to_string()
            } else {
                "信用分数过低，拒绝".to_string()
            }
        } else if credit_score < 700 {
            "信用良好，批准".to_string()
        } else {
            "信用优秀，快速批准".to_string()
        }
    }

    let applicants = [
        (25, 50000, 750, false),
        (30, 40000, 650, true),
        (45, 25000, 580, false),
        (35, 60000, 720, false),
    ];

    for (age, income, credit, collateral) in applicants {
        println!(
            "申请人(年龄:{}, 收入:{}, 信用:{}, 抵押:{}) -> {}",
            age,
            income,
            credit,
            collateral,
            loan_approval(age, income, credit, collateral)
        );
    }
}

/// # 练习2: 循环语句掌握
///
/// 掌握 loop、while、for 三种循环的使用
pub fn exercise2_loop_mastery() {
    println!("\n=== 练习2：循环语句掌握 ===");

    // 2.1 斐波那契数列生成
    println!("\n2.1 斐波那契数列生成:");
    fn fibonacci_sequence(n: usize) -> Vec<u64> {
        let mut sequence = Vec::new();
        let mut a = 0;
        let mut b = 1;

        for _ in 0..n {
            sequence.push(a);
            let temp = a + b;
            a = b;
            b = temp;
        }
        sequence
    }

    let fib_10 = fibonacci_sequence(10);
    println!("前10个斐波那契数: {:?}", fib_10);

    // 2.2 质数检测和生成
    println!("\n2.2 质数检测和生成:");
    fn is_prime(n: u32) -> bool {
        if n < 2 {
            return false;
        }
        for i in 2..=(n as f64).sqrt() as u32 {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    fn generate_primes(limit: u32) -> Vec<u32> {
        let mut primes = Vec::new();
        for num in 2..=limit {
            if is_prime(num) {
                primes.push(num);
            }
        }
        primes
    }

    let primes_50 = generate_primes(50);
    println!("50以内的质数: {:?}", primes_50);

    // 2.3 数字游戏 - 猜数字模拟
    println!("\n2.3 猜数字游戏模拟:");
    fn guess_number_simulation(target: u32, max_attempts: u32) -> u32 {
        let mut attempts = 0;
        let mut low = 1;
        let mut high = 100;

        loop {
            attempts += 1;
            let guess = (low + high) / 2;

            println!("第{}次猜测: {}", attempts, guess);

            if guess == target {
                println!("猜中了！目标数字是 {}", target);
                break;
            } else if guess < target {
                println!("太小了");
                low = guess + 1;
            } else {
                println!("太大了");
                high = guess - 1;
            }

            if attempts >= max_attempts {
                println!("达到最大尝试次数");
                break;
            }
        }
        attempts
    }

    let target = 73;
    let attempts = guess_number_simulation(target, 10);
    println!("总共尝试了 {} 次\n", attempts);

    // 2.4 数据统计 - while let 应用
    println!("\n2.4 数据统计分析:");
    fn analyze_data(data: Vec<Option<i32>>) -> (i32, i32, i32, usize) {
        let mut sum = 0;
        let mut count = 0;
        let mut max = i32::MIN;
        let mut min = i32::MAX;
        let mut iter = data.into_iter();

        while let Some(value_opt) = iter.next() {
            if let Some(value) = value_opt {
                sum += value;
                count += 1;
                max = max.max(value);
                min = min.min(value);
            }
        }

        (sum, max, min, count)
    }

    let data = vec![Some(10), None, Some(20), Some(5), None, Some(30), Some(15)];
    let (sum, max, min, count) = analyze_data(data);
    println!("数据分析结果:");
    println!("  总和: {}", sum);
    println!("  最大值: {}", max);
    println!("  最小值: {}", min);
    println!("  有效数据个数: {}", count);
    println!("  平均值: {:.2}", sum as f64 / count as f64);
}

/// # 练习3: 循环控制技巧
///
/// 掌握 break、continue、标签循环的使用
pub fn exercise3_loop_control() {
    println!("\n=== 练习3：循环控制技巧 ===");

    // 3.1 数组搜索 - 多重条件
    println!("\n3.1 数组搜索 - 多重条件:");
    fn search_with_conditions(data: &[i32], target: i32, max_iterations: usize) -> Option<usize> {
        for (index, &value) in data.iter().enumerate() {
            if index >= max_iterations {
                println!("达到最大搜索次数限制");
                break;
            }

            if value < 0 {
                println!("跳过负数: {}", value);
                continue;
            }

            if value == target {
                return Some(index);
            }
        }
        None
    }

    let data = [1, -2, 3, 4, -5, 6, 7, 8, 9, 10];
    let target = 7;
    match search_with_conditions(&data, target, 8) {
        Some(index) => println!("找到目标值 {} 在索引 {}", target, index),
        None => println!("未找到目标值 {}", target),
    }

    // 3.2 矩阵操作 - 标签循环
    println!("\n3.2 矩阵操作 - 查找特定模式:");
    fn find_pattern_in_matrix(matrix: &[[i32; 4]; 4], pattern: &[i32]) -> Option<(usize, usize)> {
        'row_loop: for (row_idx, row) in matrix.iter().enumerate() {
            'col_loop: for col_idx in 0..=(row.len() - pattern.len()) {
                // 检查是否匹配模式
                for (i, &pattern_val) in pattern.iter().enumerate() {
                    if row[col_idx + i] != pattern_val {
                        continue 'col_loop;
                    }
                }
                return Some((row_idx, col_idx));
            }
        }
        None
    }

    let matrix = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 1, 2, 3], [4, 5, 6, 7]];

    let pattern = [1, 2, 3];
    match find_pattern_in_matrix(&matrix, &pattern) {
        Some((row, col)) => println!("找到模式 {:?} 在位置 ({}, {})", pattern, row, col),
        None => println!("未找到模式 {:?}", pattern),
    }

    // 3.3 数据清洗 - 复杂循环控制
    println!("\n3.3 数据清洗 - 复杂循环控制:");
    fn clean_data(data: Vec<f64>) -> Vec<f64> {
        let mut cleaned = Vec::new();
        let mut consecutive_outliers = 0;

        'data_loop: for (i, &value) in data.iter().enumerate() {
            // 跳过 NaN 和无穷大
            if value.is_nan() || value.is_infinite() {
                println!("跳过无效值在索引 {}: {}", i, value);
                continue;
            }

            // 检查是否为异常值（这里简单定义为绝对值大于100）
            if value.abs() > 100.0 {
                consecutive_outliers += 1;
                println!("发现异常值在索引 {}: {}", i, value);

                // 如果连续3个异常值，停止处理
                if consecutive_outliers >= 3 {
                    println!("连续异常值过多，停止数据清洗");
                    break 'data_loop;
                }
                continue;
            } else {
                consecutive_outliers = 0; // 重置计数器
            }

            cleaned.push(value);
        }

        cleaned
    }

    let raw_data = vec![1.0, 2.5, 150.0, 3.0, 4.5, 200.0, 300.0, 400.0, 5.0, 6.0];
    let cleaned_data = clean_data(raw_data.clone());
    println!("原始数据: {:?}", raw_data);
    println!("清洗后数据: {:?}", cleaned_data);

    // 3.4 游戏逻辑 - 多层循环控制
    println!("\n3.4 游戏逻辑 - 寻宝游戏:");
    fn treasure_hunt(map: &[[char; 5]; 5], start: (usize, usize)) -> Option<(usize, usize)> {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // 右、下、左、上
        let mut visited = [[false; 5]; 5];
        let mut queue = vec![start];

        'search: while let Some((x, y)) = queue.pop() {
            if visited[x][y] {
                continue;
            }

            visited[x][y] = true;
            println!("探索位置 ({}, {}): {}", x, y, map[x][y]);

            match map[x][y] {
                'T' => {
                    println!("找到宝藏！");
                    return Some((x, y));
                }
                'X' => {
                    println!("遇到障碍物，无法通过");
                    continue;
                }
                _ => {
                    // 探索相邻位置
                    for &(dx, dy) in &directions {
                        let new_x = x as i32 + dx;
                        let new_y = y as i32 + dy;

                        if new_x >= 0 && new_x < 5 && new_y >= 0 && new_y < 5 {
                            let new_x = new_x as usize;
                            let new_y = new_y as usize;

                            if !visited[new_x][new_y] {
                                queue.push((new_x, new_y));
                            }
                        }
                    }
                }
            }
        }

        None
    }

    let treasure_map = [
        ['.', '.', 'X', '.', '.'],
        ['.', 'X', '.', '.', '.'],
        ['X', '.', '.', 'X', '.'],
        ['.', '.', 'X', '.', 'T'],
        ['.', '.', '.', '.', '.'],
    ];

    match treasure_hunt(&treasure_map, (0, 0)) {
        Some((x, y)) => println!("宝藏位置: ({}, {})", x, y),
        None => println!("未找到宝藏"),
    }
}

/// # 练习4: 模式匹配进阶
///
/// 掌握 match、if let、while let 的高级用法
pub fn exercise4_pattern_matching() {
    println!("\n=== 练习4：模式匹配进阶 ===");

    // 4.1 复杂枚举处理
    println!("\n4.1 复杂枚举处理 - HTTP 状态码:");
    #[derive(Debug, Clone)]
    enum HttpStatus {
        Ok,
        NotFound,
        ServerError(u16),
        ClientError { code: u16, message: String },
        Redirect { location: String, permanent: bool },
    }

    fn handle_http_status(status: HttpStatus) -> String {
        match status {
            HttpStatus::Ok => "请求成功".to_string(),
            HttpStatus::NotFound => "页面未找到".to_string(),
            HttpStatus::ServerError(code) if code >= 500 => {
                format!("服务器错误: {}", code)
            }
            HttpStatus::ServerError(code) => {
                format!("未知服务器状态: {}", code)
            }
            HttpStatus::ClientError { code, message } if code == 400 => {
                format!("请求错误: {}", message)
            }
            HttpStatus::ClientError { code, message } => {
                format!("客户端错误 {}: {}", code, message)
            }
            HttpStatus::Redirect {
                location,
                permanent: true,
            } => {
                format!("永久重定向到: {}", location)
            }
            HttpStatus::Redirect {
                location,
                permanent: false,
            } => {
                format!("临时重定向到: {}", location)
            }
        }
    }

    let statuses = vec![
        HttpStatus::Ok,
        HttpStatus::NotFound,
        HttpStatus::ServerError(500),
        HttpStatus::ClientError {
            code: 400,
            message: "Bad Request".to_string(),
        },
        HttpStatus::Redirect {
            location: "https://example.com".to_string(),
            permanent: true,
        },
    ];

    for status in statuses {
        println!("{:?} -> {}", status, handle_http_status(status.clone()));
    }

    // 4.2 嵌套数据结构解构
    println!("\n4.2 嵌套数据结构解构:");
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        address: Option<Address>,
    }

    #[derive(Debug)]
    struct Address {
        street: String,
        city: String,
        country: String,
    }

    fn process_person(person: Person) -> String {
        match person {
            Person {
                name,
                age,
                address: Some(Address { city, country, .. }),
            } if age >= 18 => {
                format!("{} ({}岁) 来自 {}, {}", name, age, city, country)
            }
            Person {
                name,
                age,
                address: Some(_),
            } => {
                format!("{} ({}岁) 未成年，有地址", name, age)
            }
            Person {
                name,
                age,
                address: None,
            } => {
                format!("{} ({}岁) 无地址信息", name, age)
            }
        }
    }

    let people = vec![
        Person {
            name: "张三".to_string(),
            age: 25,
            address: Some(Address {
                street: "中山路123号".to_string(),
                city: "北京".to_string(),
                country: "中国".to_string(),
            }),
        },
        Person {
            name: "李四".to_string(),
            age: 16,
            address: Some(Address {
                street: "解放路456号".to_string(),
                city: "上海".to_string(),
                country: "中国".to_string(),
            }),
        },
        Person {
            name: "王五".to_string(),
            age: 30,
            address: None,
        },
    ];

    for person in people {
        println!("{}", process_person(person));
    }

    // 4.3 状态机实现 - 订单处理
    println!("\n4.3 状态机实现 - 订单处理:");
    #[derive(Debug, Clone)]
    enum OrderState {
        Pending,
        Confirmed { payment_method: String },
        Shipped { tracking_number: String },
        Delivered { delivery_date: String },
        Cancelled { reason: String },
    }

    #[derive(Debug)]
    struct Order {
        id: u32,
        state: OrderState,
    }

    impl Order {
        fn new(id: u32) -> Self {
            Order {
                id,
                state: OrderState::Pending,
            }
        }

        fn process_event(&mut self, event: OrderEvent) -> Result<String, String> {
            let new_state = match (&self.state, event) {
                (OrderState::Pending, OrderEvent::Confirm(payment)) => OrderState::Confirmed {
                    payment_method: payment,
                },
                (OrderState::Confirmed { .. }, OrderEvent::Ship(tracking)) => OrderState::Shipped {
                    tracking_number: tracking,
                },
                (OrderState::Shipped { .. }, OrderEvent::Deliver(date)) => OrderState::Delivered {
                    delivery_date: date,
                },
                (_, OrderEvent::Cancel(reason)) => OrderState::Cancelled { reason },
                _ => return Err("无效的状态转换".to_string()),
            };

            let message = match &new_state {
                OrderState::Confirmed { payment_method } => {
                    format!("订单已确认，支付方式: {}", payment_method)
                }
                OrderState::Shipped { tracking_number } => {
                    format!("订单已发货，追踪号: {}", tracking_number)
                }
                OrderState::Delivered { delivery_date } => {
                    format!("订单已送达，送达日期: {}", delivery_date)
                }
                OrderState::Cancelled { reason } => {
                    format!("订单已取消，原因: {}", reason)
                }
                _ => "状态更新".to_string(),
            };

            self.state = new_state;
            Ok(message)
        }
    }

    #[derive(Debug)]
    enum OrderEvent {
        Confirm(String),
        Ship(String),
        Deliver(String),
        Cancel(String),
    }

    let mut order = Order::new(12345);
    println!("初始订单状态: {:?}", order);

    let events = vec![
        OrderEvent::Confirm("信用卡".to_string()),
        OrderEvent::Ship("SF123456789".to_string()),
        OrderEvent::Deliver("2024-01-15".to_string()),
    ];

    for event in events {
        match order.process_event(event) {
            Ok(message) => println!("✅ {}", message),
            Err(error) => println!("❌ {}", error),
        }
        println!("当前状态: {:?}\n", order.state);
    }

    // 4.4 解析器实现 - 简单表达式
    println!("\n4.4 解析器实现 - 简单表达式:");
    #[derive(Debug, Clone)]
    enum Token {
        Number(i32),
        Plus,
        Minus,
        Multiply,
        Divide,
        LeftParen,
        RightParen,
    }

    fn evaluate_expression(tokens: Vec<Token>) -> Result<i32, String> {
        let mut stack = Vec::new();
        let mut operators = Vec::new();

        for token in tokens {
            match token {
                Token::Number(n) => stack.push(n),
                Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
                    // 简化的操作符处理
                    while let Some(op) = operators.pop() {
                        if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                            let result = match op {
                                Token::Plus => a + b,
                                Token::Minus => a - b,
                                Token::Multiply => a * b,
                                Token::Divide => {
                                    if b == 0 {
                                        return Err("除零错误".to_string());
                                    }
                                    a / b
                                }
                                _ => return Err("无效操作符".to_string()),
                            };
                            stack.push(result);
                        }
                    }
                    operators.push(token);
                }
                _ => {} // 忽略括号（简化实现）
            }
        }

        // 处理剩余操作符
        while let Some(op) = operators.pop() {
            if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                let result = match op {
                    Token::Plus => a + b,
                    Token::Minus => a - b,
                    Token::Multiply => a * b,
                    Token::Divide => {
                        if b == 0 {
                            return Err("除零错误".to_string());
                        }
                        a / b
                    }
                    _ => return Err("无效操作符".to_string()),
                };
                stack.push(result);
            }
        }

        stack.pop().ok_or_else(|| "表达式为空".to_string())
    }

    let expression = vec![
        Token::Number(10),
        Token::Plus,
        Token::Number(5),
        Token::Multiply,
        Token::Number(2),
    ];

    match evaluate_expression(expression) {
        Ok(result) => println!("表达式计算结果: {}", result),
        Err(error) => println!("计算错误: {}", error),
    }
}

/// # 练习5: 实际应用综合
///
/// 综合运用所有流程控制技巧解决实际问题
pub fn exercise5_comprehensive_application() {
    println!("\n=== 练习5：实际应用综合 ===");

    // 5.1 文本处理器
    println!("\n5.1 文本处理器:");
    fn process_text(text: &str) -> (usize, usize, usize, Vec<String>) {
        let mut word_count = 0;
        let mut line_count = 0;
        let mut char_count = 0;
        let mut words = Vec::new();

        for line in text.lines() {
            line_count += 1;
            char_count += line.len();

            for word in line.split_whitespace() {
                // 清理标点符号
                let clean_word: String = word
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .collect::<String>()
                    .to_lowercase();

                if !clean_word.is_empty() {
                    word_count += 1;
                    words.push(clean_word);
                }
            }
        }

        (word_count, line_count, char_count, words)
    }

    let sample_text = "Hello, World!\nThis is a sample text.\nIt has multiple lines and words.";
    let (words, lines, chars, word_list) = process_text(sample_text);

    println!("文本统计:");
    println!("  单词数: {}", words);
    println!("  行数: {}", lines);
    println!("  字符数: {}", chars);
    println!("  单词列表: {:?}", word_list);

    // 5.2 数据验证器
    println!("\n5.2 数据验证器:");
    #[derive(Debug)]
    struct ValidationResult {
        is_valid: bool,
        errors: Vec<String>,
    }

    fn validate_user_data(name: &str, email: &str, age: u32, password: &str) -> ValidationResult {
        let mut errors = Vec::new();

        // 验证姓名
        if name.is_empty() {
            errors.push("姓名不能为空".to_string());
        } else if name.len() < 2 {
            errors.push("姓名至少需要2个字符".to_string());
        }

        // 验证邮箱
        if !email.contains('@') || !email.contains('.') {
            errors.push("邮箱格式无效".to_string());
        }

        // 验证年龄
        match age {
            0..=12 => errors.push("年龄过小".to_string()),
            13..=17 => errors.push("需要监护人同意".to_string()),
            18..=120 => {} // 有效年龄
            _ => errors.push("年龄无效".to_string()),
        }

        // 验证密码
        let mut has_upper = false;
        let mut has_lower = false;
        let mut has_digit = false;
        let mut has_special = false;

        if password.len() < 8 {
            errors.push("密码至少需要8个字符".to_string());
        } else {
            for ch in password.chars() {
                match ch {
                    'A'..='Z' => has_upper = true,
                    'a'..='z' => has_lower = true,
                    '0'..='9' => has_digit = true,
                    '!'..='/' | ':'..='@' | '['..='`' | '{'..='~' => has_special = true,
                    _ => {}
                }
            }

            if !has_upper {
                errors.push("密码需要包含大写字母".to_string());
            }
            if !has_lower {
                errors.push("密码需要包含小写字母".to_string());
            }
            if !has_digit {
                errors.push("密码需要包含数字".to_string());
            }
            if !has_special {
                errors.push("密码需要包含特殊字符".to_string());
            }
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
        }
    }

    let test_users = [
        ("张三", "zhangsan@example.com", 25, "Password123!"),
        ("", "invalid-email", 15, "weak"),
        ("李四", "lisi@test.com", 30, "NoSpecial123"),
    ];

    for (name, email, age, password) in test_users {
        let result = validate_user_data(name, email, age, password);
        println!("用户 '{}' 验证结果:", name);
        if result.is_valid {
            println!("  ✅ 验证通过");
        } else {
            println!("  ❌ 验证失败:");
            for error in result.errors {
                println!("    - {}", error);
            }
        }
        println!();
    }

    // 5.3 简单调度器
    println!("\n5.3 任务调度器:");
    #[derive(Debug, Clone)]
    enum TaskStatus {
        Pending,
        Running,
        Completed,
        Failed(String),
    }

    #[derive(Debug, Clone)]
    struct Task {
        id: u32,
        name: String,
        priority: u32,
        status: TaskStatus,
        dependencies: Vec<u32>,
    }

    struct TaskScheduler {
        tasks: Vec<Task>,
        completed_tasks: Vec<u32>,
    }

    impl TaskScheduler {
        fn new() -> Self {
            TaskScheduler {
                tasks: Vec::new(),
                completed_tasks: Vec::new(),
            }
        }

        fn add_task(&mut self, task: Task) {
            self.tasks.push(task);
        }

        fn run_scheduler(&mut self) {
            let mut max_iterations = 100; // 防止无限循环

            'scheduler_loop: while max_iterations > 0 {
                max_iterations -= 1;
                let mut made_progress = false;

                // 查找可以执行的任务
                for task in &mut self.tasks {
                    if let TaskStatus::Pending = task.status {
                        // 检查依赖是否完成
                        let dependencies_met = task
                            .dependencies
                            .iter()
                            .all(|&dep_id| self.completed_tasks.contains(&dep_id));

                        if dependencies_met {
                            println!("开始执行任务: {} (优先级: {})", task.name, task.priority);
                            task.status = TaskStatus::Running;

                            // 模拟任务执行
                            if task.name.contains("fail") {
                                task.status = TaskStatus::Failed("模拟失败".to_string());
                                println!("  ❌ 任务失败: {}", task.name);
                            } else {
                                task.status = TaskStatus::Completed;
                                self.completed_tasks.push(task.id);
                                println!("  ✅ 任务完成: {}", task.name);
                            }

                            made_progress = true;
                        }
                    }
                }

                // 检查是否所有任务都完成或失败
                let all_done = self.tasks.iter().all(|task| {
                    matches!(task.status, TaskStatus::Completed | TaskStatus::Failed(_))
                });

                if all_done {
                    println!("所有任务处理完成");
                    break 'scheduler_loop;
                }

                if !made_progress {
                    println!("检测到循环依赖或无法执行的任务");
                    break 'scheduler_loop;
                }
            }

            if max_iterations == 0 {
                println!("调度器达到最大迭代次数");
            }
        }

        fn print_status(&self) {
            println!("\n任务状态报告:");
            for task in &self.tasks {
                println!("  任务 {}: {} - {:?}", task.id, task.name, task.status);
            }
        }
    }

    let mut scheduler = TaskScheduler::new();

    // 添加任务
    scheduler.add_task(Task {
        id: 1,
        name: "初始化系统".to_string(),
        priority: 1,
        status: TaskStatus::Pending,
        dependencies: vec![],
    });

    scheduler.add_task(Task {
        id: 2,
        name: "加载配置".to_string(),
        priority: 2,
        status: TaskStatus::Pending,
        dependencies: vec![1],
    });

    scheduler.add_task(Task {
        id: 3,
        name: "启动服务".to_string(),
        priority: 3,
        status: TaskStatus::Pending,
        dependencies: vec![1, 2],
    });

    scheduler.add_task(Task {
        id: 4,
        name: "fail_task".to_string(), // 这个任务会失败
        priority: 4,
        status: TaskStatus::Pending,
        dependencies: vec![3],
    });

    scheduler.run_scheduler();
    scheduler.print_status();
}

/// 运行所有练习
pub fn run_all_exercises() {
    println!("🎯 Rust 流程控制练习题");
    println!("{}", "=".repeat(50));

    exercise1_conditional_basics();
    exercise2_loop_mastery();
    exercise3_loop_control();
    exercise4_pattern_matching();
    exercise5_comprehensive_application();

    println!("\n{}", "=".repeat(50));
    println!("✅ 所有流程控制练习完成！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grade_calculation() {
        fn get_grade(score: u32) -> &'static str {
            if score >= 90 {
                "A"
            } else if score >= 80 {
                "B"
            } else if score >= 70 {
                "C"
            } else if score >= 60 {
                "D"
            } else {
                "F"
            }
        }

        assert_eq!(get_grade(95), "A");
        assert_eq!(get_grade(85), "B");
        assert_eq!(get_grade(75), "C");
        assert_eq!(get_grade(65), "D");
        assert_eq!(get_grade(55), "F");
    }

    #[test]
    fn test_fibonacci() {
        fn fibonacci_sequence(n: usize) -> Vec<u64> {
            let mut sequence = Vec::new();
            let mut a = 0;
            let mut b = 1;

            for _ in 0..n {
                sequence.push(a);
                let temp = a + b;
                a = b;
                b = temp;
            }
            sequence
        }

        let fib_5 = fibonacci_sequence(5);
        assert_eq!(fib_5, vec![0, 1, 1, 2, 3]);
    }

    #[test]
    fn test_prime_detection() {
        fn is_prime(n: u32) -> bool {
            if n < 2 {
                return false;
            }
            for i in 2..=(n as f64).sqrt() as u32 {
                if n % i == 0 {
                    return false;
                }
            }
            true
        }

        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(!is_prime(4));
        assert!(!is_prime(6));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
    }

    #[test]
    fn test_pattern_matching() {
        fn categorize_number(n: i32) -> &'static str {
            match n {
                x if x < 0 => "负数",
                0 => "零",
                1..=10 => "小正数",
                11..=100 => "中等正数",
                _ => "大正数",
            }
        }

        assert_eq!(categorize_number(-5), "负数");
        assert_eq!(categorize_number(0), "零");
        assert_eq!(categorize_number(5), "小正数");
        assert_eq!(categorize_number(50), "中等正数");
        assert_eq!(categorize_number(500), "大正数");
    }

    #[test]
    fn test_loop_control() {
        fn sum_until_negative(numbers: &[i32]) -> i32 {
            let mut sum = 0;
            for &num in numbers {
                if num < 0 {
                    break;
                }
                sum += num;
            }
            sum
        }

        let numbers = [1, 2, 3, -4, 5, 6];
        assert_eq!(sum_until_negative(&numbers), 6);
    }
}
