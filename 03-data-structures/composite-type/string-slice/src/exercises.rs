// exercises.rs
// Rust 字符串与切片练习题
// 通过实际编程练习巩固学习成果

/// 字符串与切片练习模块
///
/// 本模块包含了一系列练习题，帮助你巩固字符串和切片的知识
/// 每个练习都有详细的说明和测试用例

pub fn run_all_exercises() {
    println!("🎯 === Rust 字符串与切片练习题 ===");
    println!("💪 通过这些练习巩固你的学习成果！\n");

    exercise_1_basic_slicing();
    exercise_2_string_manipulation();
    exercise_3_utf8_handling();
    exercise_4_string_parsing();
    exercise_5_performance_optimization();
    exercise_6_advanced_operations();

    println!("\n🎉 恭喜完成所有练习！你已经掌握了 Rust 字符串与切片的核心技能！");
}

/// 练习1: 基础切片操作
/// 目标: 掌握切片的基本语法和用法
fn exercise_1_basic_slicing() {
    println!("📝 练习1: 基础切片操作");

    // 任务1: 从字符串中提取特定部分
    let text = "Hello, Rust Programming!";

    // TODO: 提取 "Hello"
    let hello = &text[0..5];
    assert_eq!(hello, "Hello");
    println!("✅ 提取 'Hello': {}", hello);

    // TODO: 提取 "Rust"
    let rust = &text[7..11];
    assert_eq!(rust, "Rust");
    println!("✅ 提取 'Rust': {}", rust);

    // TODO: 提取 "Programming!"
    let programming = &text[12..];
    assert_eq!(programming, "Programming!");
    println!("✅ 提取 'Programming!': {}", programming);

    // 任务2: 数组切片操作
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // TODO: 提取前5个数字
    let first_five = &numbers[..5];
    assert_eq!(first_five, &[1, 2, 3, 4, 5]);
    println!("✅ 前5个数字: {:?}", first_five);

    // TODO: 提取中间3个数字 (索引3-5)
    let middle_three = &numbers[3..6];
    assert_eq!(middle_three, &[4, 5, 6]);
    println!("✅ 中间3个数字: {:?}", middle_three);

    println!("🎯 练习1完成！\n");
}

/// 练习2: 字符串操作
/// 目标: 掌握 String 和 &str 的各种操作方法
fn exercise_2_string_manipulation() {
    println!("📝 练习2: 字符串操作");

    // 任务1: 字符串构建
    let mut greeting = String::new();
    greeting.push_str("Hello");
    greeting.push(',');
    greeting.push(' ');
    greeting.push_str("World!");

    assert_eq!(greeting, "Hello, World!");
    println!("✅ 构建问候语: {}", greeting);

    // 任务2: 字符串连接的不同方式
    let first = "Rust";
    let second = "Programming";

    // 使用 format! 宏
    let combined1 = format!("{} {}", first, second);
    assert_eq!(combined1, "Rust Programming");
    println!("✅ format! 连接: {}", combined1);

    // 使用 + 操作符
    let first_owned = String::from(first);
    let combined2 = first_owned + " " + second;
    assert_eq!(combined2, "Rust Programming");
    println!("✅ + 操作符连接: {}", combined2);

    // 任务3: 字符串查找和替换
    let text = "The quick brown fox jumps over the lazy dog";

    // 查找单词位置
    if let Some(pos) = text.find("fox") {
        println!("✅ 找到 'fox' 在位置: {}", pos);
    }

    // 替换单词
    let new_text = text.replace("fox", "cat");
    assert_eq!(new_text, "The quick brown cat jumps over the lazy dog");
    println!("✅ 替换后: {}", new_text);

    // 任务4: 字符串分割
    let csv_data = "apple,banana,cherry,date";
    let fruits: Vec<&str> = csv_data.split(',').collect();
    assert_eq!(fruits, vec!["apple", "banana", "cherry", "date"]);
    println!("✅ 分割结果: {:?}", fruits);

    println!("🎯 练习2完成！\n");
}

/// 练习3: UTF-8 编码处理
/// 目标: 正确处理 Unicode 字符和 UTF-8 编码
fn exercise_3_utf8_handling() {
    println!("📝 练习3: UTF-8 编码处理");

    // 任务1: 多语言字符串处理
    let multilingual = "Hello 你好 🦀 Здравствуй";

    // 字节长度 vs 字符数量
    let byte_len = multilingual.len();
    let char_count = multilingual.chars().count();

    println!("✅ 字符串: {}", multilingual);
    println!("✅ 字节长度: {}", byte_len);
    println!("✅ 字符数量: {}", char_count);

    // 任务2: 安全的字符访问
    let chinese = "你好世界";

    // 遍历字符
    print!("✅ 逐字符遍历: ");
    for c in chinese.chars() {
        print!("{} ", c);
    }
    println!();

    // 获取第一个字符
    if let Some(first_char) = chinese.chars().next() {
        println!("✅ 第一个字符: {}", first_char);
    }

    // 任务3: 安全的切片操作
    let emoji_text = "🦀🔥⚡";

    // 每个 emoji 占用4个字节
    let first_emoji = &emoji_text[0..4];
    assert_eq!(first_emoji, "🦀");
    println!("✅ 第一个 emoji: {}", first_emoji);

    // 任务4: 字符边界检查
    let text = "café";

    // 检查是否在字符边界
    println!("✅ 字符边界检查:");
    for i in 0..=text.len() {
        if text.is_char_boundary(i) {
            println!("   位置 {} 是字符边界", i);
        }
    }

    println!("🎯 练习3完成！\n");
}

/// 练习4: 字符串解析
/// 目标: 掌握字符串与其他类型的转换
fn exercise_4_string_parsing() {
    println!("📝 练习4: 字符串解析");

    // 任务1: 数字解析
    let number_strings = vec!["42", "3.14", "100", "invalid"];

    for s in number_strings {
        match s.parse::<i32>() {
            Ok(num) => println!("✅ '{}' 解析为整数: {}", s, num),
            Err(_) => {
                // 尝试解析为浮点数
                match s.parse::<f64>() {
                    Ok(num) => println!("✅ '{}' 解析为浮点数: {}", s, num),
                    Err(_) => println!("❌ '{}' 无法解析为数字", s),
                }
            }
        }
    }

    // 任务2: 布尔值解析
    let bool_strings = vec!["true", "false", "1", "0", "yes"];

    for s in bool_strings {
        match s.parse::<bool>() {
            Ok(b) => println!("✅ '{}' 解析为布尔值: {}", s, b),
            Err(_) => {
                // 自定义布尔值解析
                let custom_bool = match s {
                    "1" | "yes" | "Yes" | "YES" => Some(true),
                    "0" | "no" | "No" | "NO" => Some(false),
                    _ => None,
                };

                match custom_bool {
                    Some(b) => println!("✅ '{}' 自定义解析为布尔值: {}", s, b),
                    None => println!("❌ '{}' 无法解析为布尔值", s),
                }
            }
        }
    }

    // 任务3: CSV 解析
    let csv_line = "John,25,Engineer,true";
    let fields: Vec<&str> = csv_line.split(',').collect();

    if fields.len() == 4 {
        let name = fields[0];
        let age: Result<u32, _> = fields[1].parse();
        let job = fields[2];
        let active: Result<bool, _> = fields[3].parse();

        match (age, active) {
            (Ok(age), Ok(active)) => {
                println!("✅ CSV 解析成功:");
                println!("   姓名: {}", name);
                println!("   年龄: {}", age);
                println!("   职业: {}", job);
                println!("   活跃: {}", active);
            }
            _ => println!("❌ CSV 解析失败"),
        }
    }

    println!("🎯 练习4完成！\n");
}

/// 练习5: 性能优化
/// 目标: 学习字符串操作的性能最佳实践
fn exercise_5_performance_optimization() {
    println!("📝 练习5: 性能优化");

    // 任务1: 预分配容量
    let mut efficient_string = String::with_capacity(100);

    for i in 0..10 {
        efficient_string.push_str(&format!("Item {} ", i));
    }

    println!("✅ 预分配容量的字符串: {}", efficient_string);
    println!("✅ 最终容量: {}", efficient_string.capacity());

    // 任务2: 避免不必要的分配
    fn format_message_efficient(name: &str, age: u32, city: &str) -> String {
        format!(
            "Hello, {}! You are {} years old and live in {}.",
            name, age, city
        )
    }

    let message = format_message_efficient("Alice", 30, "Tokyo");
    println!("✅ 高效格式化: {}", message);

    // 任务3: 字符串比较优化
    let strings = vec!["apple", "banana", "cherry", "date"];
    let target = "cherry";

    // 使用 &str 比较（高效）
    if let Some(pos) = strings.iter().position(|&s| s == target) {
        println!("✅ 找到 '{}' 在位置: {}", target, pos);
    }

    // 任务4: 内存使用优化
    let mut large_string = String::with_capacity(1000);
    large_string.push_str("Small content");

    println!(
        "✅ 收缩前 - 长度: {}, 容量: {}",
        large_string.len(),
        large_string.capacity()
    );

    large_string.shrink_to_fit();
    println!(
        "✅ 收缩后 - 长度: {}, 容量: {}",
        large_string.len(),
        large_string.capacity()
    );

    println!("🎯 练习5完成！\n");
}

/// 练习6: 高级操作
/// 目标: 实现复杂的字符串处理功能
fn exercise_6_advanced_operations() {
    println!("📝 练习6: 高级操作");

    // 任务1: 实现字符串反转（Unicode 安全）
    fn reverse_unicode_safe(s: &str) -> String {
        s.chars().rev().collect()
    }

    let original = "Hello, 世界! 🦀";
    let reversed = reverse_unicode_safe(original);
    println!("✅ 原字符串: {}", original);
    println!("✅ 反转后: {}", reversed);

    // 任务2: 实现单词首字母大写
    fn title_case(s: &str) -> String {
        s.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    let text = "hello rust programming";
    let titled = title_case(text);
    assert_eq!(titled, "Hello Rust Programming");
    println!("✅ 标题格式: {}", titled);

    // 任务3: 实现字符串压缩（简单的游程编码）
    fn simple_compress(s: &str) -> String {
        if s.is_empty() {
            return String::new();
        }

        let mut result = String::new();
        let mut chars = s.chars().peekable();

        while let Some(current) = chars.next() {
            let mut count = 1;

            while let Some(&next) = chars.peek() {
                if next == current {
                    count += 1;
                    chars.next();
                } else {
                    break;
                }
            }

            if count > 1 {
                result.push_str(&format!("{}{}", current, count));
            } else {
                result.push(current);
            }
        }

        result
    }

    let text = "aaabbccccdd";
    let compressed = simple_compress(text);
    println!("✅ 原文本: {}", text);
    println!("✅ 压缩后: {}", compressed);

    // 任务4: 实现简单的模板替换
    fn simple_template(template: &str, replacements: &[(String, String)]) -> String {
        let mut result = template.to_string();

        for (placeholder, value) in replacements {
            result = result.replace(placeholder, value);
        }

        result
    }

    let template = "Hello, {{name}}! Welcome to {{place}}.";
    let replacements = vec![
        ("{{name}}".to_string(), "Alice".to_string()),
        ("{{place}}".to_string(), "Rust World".to_string()),
    ];

    let result = simple_template(template, &replacements);
    println!("✅ 模板: {}", template);
    println!("✅ 替换后: {}", result);

    // 任务5: 实现字符串相似度计算（简单版本）
    fn simple_similarity(s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            return 1.0;
        }

        let len1 = s1.chars().count();
        let len2 = s2.chars().count();

        if len1 == 0 || len2 == 0 {
            return 0.0;
        }

        let common_chars = s1.chars().filter(|c| s2.contains(*c)).count();

        common_chars as f64 / (len1.max(len2) as f64)
    }

    let str1 = "hello";
    let str2 = "hallo";
    let similarity = simple_similarity(str1, str2);
    println!("✅ '{}' 和 '{}' 的相似度: {:.2}", str1, str2, similarity);

    println!("🎯 练习6完成！\n");
}

/// 挑战练习：实现一个简单的文本分析器
pub fn challenge_text_analyzer() {
    println!("🏆 === 挑战练习：文本分析器 ===");

    let text = r#"
        Rust is a systems programming language that runs blazingly fast,
        prevents segfaults, and guarantees thread safety. It accomplishes
        these goals without requiring a garbage collector or runtime.
        Rust is developed by Mozilla and has a growing community of
        contributors. The language emphasizes safety, speed, and concurrency.
    "#;

    // 分析文本
    let analyzer = TextAnalyzer::new(text);
    analyzer.print_analysis();
}

/// 文本分析器结构体
struct TextAnalyzer {
    text: String,
}

impl TextAnalyzer {
    fn new(text: &str) -> Self {
        Self {
            text: text.trim().to_string(),
        }
    }

    fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }

    fn char_count(&self) -> usize {
        self.text.chars().count()
    }

    fn line_count(&self) -> usize {
        self.text.lines().count()
    }

    fn most_common_word(&self) -> Option<(String, usize)> {
        use std::collections::HashMap;

        let mut word_counts = HashMap::new();

        for word in self.text.split_whitespace() {
            let clean_word = word
                .trim_matches(|c: char| !c.is_alphabetic())
                .to_lowercase();

            if !clean_word.is_empty() {
                *word_counts.entry(clean_word).or_insert(0) += 1;
            }
        }

        word_counts.into_iter().max_by_key(|(_, count)| *count)
    }

    fn average_word_length(&self) -> f64 {
        let words: Vec<&str> = self.text.split_whitespace().collect();
        if words.is_empty() {
            return 0.0;
        }

        let total_chars: usize = words.iter().map(|word| word.chars().count()).sum();

        total_chars as f64 / words.len() as f64
    }

    fn print_analysis(&self) {
        println!("📊 文本分析结果:");
        println!("   📝 总字符数: {}", self.char_count());
        println!("   📄 总行数: {}", self.line_count());
        println!("   🔤 总单词数: {}", self.word_count());
        println!("   📏 平均单词长度: {:.2}", self.average_word_length());

        if let Some((word, count)) = self.most_common_word() {
            println!("   🏆 最常见单词: '{}' (出现 {} 次)", word, count);
        }

        println!("\n📈 详细统计:");

        // 单词长度分布
        let mut length_counts = std::collections::HashMap::new();
        for word in self.text.split_whitespace() {
            let len = word
                .trim_matches(|c: char| !c.is_alphabetic())
                .chars()
                .count();
            if len > 0 {
                *length_counts.entry(len).or_insert(0) += 1;
            }
        }

        println!("   📊 单词长度分布:");
        let mut lengths: Vec<_> = length_counts.into_iter().collect();
        lengths.sort_by_key(|(len, _)| *len);

        for (len, count) in lengths {
            println!("      {} 字符: {} 个单词", len, count);
        }
    }
}
