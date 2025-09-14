//! Rust 结构体练习题
//!
//! 本模块包含了从基础到高级的结构体练习题
//! 涵盖定义、方法、泛型、生命周期、trait 等各个方面

use std::collections::HashMap;
use std::fmt;

// ============================================================================
// 练习 1: 基础结构体定义和操作
// ============================================================================

/// 练习1: 定义一个表示人的结构体
/// 要求：包含姓名、年龄、身高、体重字段
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub height: f32, // 单位：米
    pub weight: f32, // 单位：公斤
}

impl Person {
    /// 创建新的人员实例
    pub fn new(name: String, age: u8, height: f32, weight: f32) -> Self {
        Person {
            name,
            age,
            height,
            weight,
        }
    }

    /// 计算BMI指数
    pub fn bmi(&self) -> f32 {
        self.weight / (self.height * self.height)
    }

    /// 判断BMI类别
    pub fn bmi_category(&self) -> &'static str {
        let bmi = self.bmi();
        match bmi {
            bmi if bmi < 18.5 => "偏瘦",
            bmi if bmi < 24.0 => "正常",
            bmi if bmi < 28.0 => "偏胖",
            _ => "肥胖",
        }
    }

    /// 增加年龄
    pub fn have_birthday(&mut self) {
        self.age += 1;
    }

    /// 更新体重
    pub fn update_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }
}

/// 练习1测试函数
pub fn exercise_1_person_struct() {
    println!("\n=== 练习1: 人员结构体 ===");

    let mut person = Person::new(String::from("张三"), 25, 1.75, 70.0);

    println!("人员信息: {:?}", person);
    println!("BMI: {:.2}", person.bmi());
    println!("BMI类别: {}", person.bmi_category());

    person.have_birthday();
    person.update_weight(68.0);

    println!("更新后: {:?}", person);
    println!("新BMI: {:.2} ({})", person.bmi(), person.bmi_category());
}

// ============================================================================
// 练习 2: 元组结构体和新类型模式
// ============================================================================

/// 练习2: 定义温度类型（新类型模式）
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Celsius(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Fahrenheit(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Kelvin(pub f64);

impl Celsius {
    pub fn new(temp: f64) -> Self {
        Celsius(temp)
    }

    pub fn to_fahrenheit(self) -> Fahrenheit {
        Fahrenheit(self.0 * 9.0 / 5.0 + 32.0)
    }

    pub fn to_kelvin(self) -> Kelvin {
        Kelvin(self.0 + 273.15)
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

impl Fahrenheit {
    pub fn new(temp: f64) -> Self {
        Fahrenheit(temp)
    }

    pub fn to_celsius(self) -> Celsius {
        Celsius((self.0 - 32.0) * 5.0 / 9.0)
    }

    pub fn to_kelvin(self) -> Kelvin {
        self.to_celsius().to_kelvin()
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

impl Kelvin {
    pub fn new(temp: f64) -> Self {
        Kelvin(temp)
    }

    pub fn to_celsius(self) -> Celsius {
        Celsius(self.0 - 273.15)
    }

    pub fn to_fahrenheit(self) -> Fahrenheit {
        self.to_celsius().to_fahrenheit()
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

/// RGB颜色（元组结构体）
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGB(pub u8, pub u8, pub u8);

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RGB(r, g, b)
    }

    pub fn red(self) -> u8 {
        self.0
    }
    pub fn green(self) -> u8 {
        self.1
    }
    pub fn blue(self) -> u8 {
        self.2
    }

    /// 转换为十六进制字符串
    pub fn to_hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }

    /// 计算亮度
    pub fn brightness(self) -> f64 {
        (0.299 * self.0 as f64 + 0.587 * self.1 as f64 + 0.114 * self.2 as f64) / 255.0
    }

    /// 判断是否为深色
    pub fn is_dark(self) -> bool {
        self.brightness() < 0.5
    }
}

/// 练习2测试函数
pub fn exercise_2_tuple_structs() {
    println!("\n=== 练习2: 元组结构体和新类型 ===");

    // 温度转换
    let celsius = Celsius::new(25.0);
    let fahrenheit = celsius.to_fahrenheit();
    let kelvin = celsius.to_kelvin();

    println!("温度转换:");
    println!(
        "{:.1}°C = {:.1}°F = {:.1}K",
        celsius.value(),
        fahrenheit.value(),
        kelvin.value()
    );

    // 颜色操作
    let red = RGB::new(255, 0, 0);
    let dark_blue = RGB::new(0, 0, 139);
    let light_gray = RGB::new(211, 211, 211);

    println!("\n颜色分析:");
    for color in [red, dark_blue, light_gray] {
        println!(
            "{:?} -> {} (亮度: {:.2}, 深色: {})",
            color,
            color.to_hex(),
            color.brightness(),
            color.is_dark()
        );
    }
}

// ============================================================================
// 练习 3: 泛型结构体和约束
// ============================================================================

/// 练习3: 泛型容器结构体
#[derive(Debug, Clone)]
pub struct Container<T> {
    items: Vec<T>,
    capacity: usize,
}

impl<T> Container<T> {
    /// 创建新容器
    pub fn new(capacity: usize) -> Self {
        Container {
            items: Vec::new(),
            capacity,
        }
    }

    /// 添加项目
    pub fn add(&mut self, item: T) -> Result<(), &'static str> {
        if self.items.len() >= self.capacity {
            Err("容器已满")
        } else {
            self.items.push(item);
            Ok(())
        }
    }

    /// 移除项目
    pub fn remove(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// 获取项目数量
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// 检查是否已满
    pub fn is_full(&self) -> bool {
        self.items.len() >= self.capacity
    }

    /// 获取容量
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

// 为实现了 Clone 的类型添加额外方法
impl<T: Clone> Container<T> {
    /// 获取所有项目的副本
    pub fn get_all(&self) -> Vec<T> {
        self.items.clone()
    }
}

// 为实现了 PartialEq 的类型添加搜索功能
impl<T: PartialEq> Container<T> {
    /// 检查是否包含某项目
    pub fn contains(&self, item: &T) -> bool {
        self.items.contains(item)
    }

    /// 查找项目位置
    pub fn find(&self, item: &T) -> Option<usize> {
        self.items.iter().position(|x| x == item)
    }
}

// 为数值类型添加统计功能
impl Container<f64> {
    /// 计算平均值
    pub fn average(&self) -> Option<f64> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.iter().sum::<f64>() / self.items.len() as f64)
        }
    }

    /// 找到最大值
    pub fn max(&self) -> Option<f64> {
        self.items
            .iter()
            .copied()
            .fold(None, |acc, x| Some(acc.map_or(x, |a| a.max(x))))
    }

    /// 找到最小值
    pub fn min(&self) -> Option<f64> {
        self.items
            .iter()
            .copied()
            .fold(None, |acc, x| Some(acc.map_or(x, |a| a.min(x))))
    }
}

/// 练习3测试函数
pub fn exercise_3_generic_container() {
    println!("\n=== 练习3: 泛型容器 ===");

    // 字符串容器
    let mut string_container = Container::new(3);

    println!("字符串容器测试:");
    assert!(string_container.add("Hello".to_string()).is_ok());
    assert!(string_container.add("World".to_string()).is_ok());
    assert!(string_container.add("Rust".to_string()).is_ok());
    assert!(string_container.add("Full".to_string()).is_err());

    println!(
        "容器状态: 长度={}, 容量={}, 已满={}",
        string_container.len(),
        string_container.capacity(),
        string_container.is_full()
    );

    println!(
        "包含'Rust': {}",
        string_container.contains(&"Rust".to_string())
    );

    // 数值容器
    let mut number_container = Container::new(5);

    println!("\n数值容器测试:");
    for value in [1.5, 2.7, 3.2, 4.8, 1.1] {
        number_container.add(value).unwrap();
    }

    println!("数值统计:");
    println!("平均值: {:.2}", number_container.average().unwrap());
    println!("最大值: {:.2}", number_container.max().unwrap());
    println!("最小值: {:.2}", number_container.min().unwrap());
}

// ============================================================================
// 练习 4: 生命周期和借用
// ============================================================================

/// 练习4: 文本分析器（包含生命周期）
#[derive(Debug)]
pub struct TextAnalyzer<'a> {
    text: &'a str,
    words: Vec<&'a str>,
}

impl<'a> TextAnalyzer<'a> {
    /// 创建新的文本分析器
    pub fn new(text: &'a str) -> Self {
        let words: Vec<&str> = text.split_whitespace().collect();

        TextAnalyzer { text, words }
    }

    /// 获取原始文本
    pub fn text(&self) -> &str {
        self.text
    }

    /// 获取单词数量
    pub fn word_count(&self) -> usize {
        self.words.len()
    }

    /// 获取字符数量（不包括空格）
    pub fn char_count(&self) -> usize {
        self.text.chars().filter(|c| !c.is_whitespace()).count()
    }

    /// 获取最长的单词
    pub fn longest_word(&self) -> Option<&str> {
        self.words.iter().max_by_key(|word| word.len()).copied()
    }

    /// 获取最短的单词
    pub fn shortest_word(&self) -> Option<&str> {
        self.words.iter().min_by_key(|word| word.len()).copied()
    }

    /// 计算平均单词长度
    pub fn average_word_length(&self) -> f64 {
        if self.words.is_empty() {
            0.0
        } else {
            let total_length: usize = self.words.iter().map(|word| word.len()).sum();
            total_length as f64 / self.words.len() as f64
        }
    }

    /// 查找包含特定子字符串的单词
    pub fn words_containing(&self, substring: &str) -> Vec<&str> {
        self.words
            .iter()
            .filter(|word| word.contains(substring))
            .copied()
            .collect()
    }

    /// 获取单词频率统计
    pub fn word_frequency(&self) -> HashMap<&str, usize> {
        let mut frequency = HashMap::new();
        for &word in &self.words {
            *frequency.entry(word).or_insert(0) += 1;
        }
        frequency
    }
}

/// 练习4测试函数
pub fn exercise_4_text_analyzer() {
    println!("\n=== 练习4: 文本分析器（生命周期） ===");

    let text = "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety";

    let analyzer = TextAnalyzer::new(text);

    println!("原始文本: {}", analyzer.text());
    println!("单词数量: {}", analyzer.word_count());
    println!("字符数量: {}", analyzer.char_count());
    println!("最长单词: {:?}", analyzer.longest_word());
    println!("最短单词: {:?}", analyzer.shortest_word());
    println!("平均单词长度: {:.2}", analyzer.average_word_length());

    let rust_words = analyzer.words_containing("rust");
    println!("包含'rust'的单词: {:?}", rust_words);

    let programming_words = analyzer.words_containing("program");
    println!("包含'program'的单词: {:?}", programming_words);

    println!("\n单词频率统计:");
    let frequency = analyzer.word_frequency();
    for (word, count) in frequency.iter() {
        if *count > 1 {
            println!("{}: {} 次", word, count);
        }
    }
}

// ============================================================================
// 练习 5: Trait 实现和多态
// ============================================================================

/// 练习5: 几何图形 trait 和实现
pub trait Shape: std::fmt::Debug {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn name(&self) -> &'static str;

    // 默认实现
    fn describe(&self) -> String {
        format!(
            "{}: 面积={:.2}, 周长={:.2}",
            self.name(),
            self.area(),
            self.perimeter()
        )
    }
}

/// 可绘制的 trait
pub trait Drawable {
    fn draw(&self) -> String;
}

/// 可缩放的 trait
pub trait Scalable {
    fn scale(&mut self, factor: f64);
}

/// 矩形
#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn name(&self) -> &'static str {
        "矩形"
    }
}

impl Drawable for Rectangle {
    fn draw(&self) -> String {
        format!("绘制矩形: {}x{}", self.width, self.height)
    }
}

impl Scalable for Rectangle {
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

/// 圆形
#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Circle { radius }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    fn name(&self) -> &'static str {
        "圆形"
    }
}

impl Drawable for Circle {
    fn draw(&self) -> String {
        format!("绘制圆形: 半径={}", self.radius)
    }
}

impl Scalable for Circle {
    fn scale(&mut self, factor: f64) {
        self.radius *= factor;
    }
}

/// 三角形
#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Triangle {
    pub fn new(a: f64, b: f64, c: f64) -> Result<Self, &'static str> {
        // 检查三角形不等式
        if a + b > c && b + c > a && a + c > b {
            Ok(Triangle { a, b, c })
        } else {
            Err("无效的三角形边长")
        }
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        // 使用海伦公式
        let s = (self.a + self.b + self.c) / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }

    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }

    fn name(&self) -> &'static str {
        "三角形"
    }
}

impl Drawable for Triangle {
    fn draw(&self) -> String {
        format!("绘制三角形: 边长=({}, {}, {})", self.a, self.b, self.c)
    }
}

impl Scalable for Triangle {
    fn scale(&mut self, factor: f64) {
        self.a *= factor;
        self.b *= factor;
        self.c *= factor;
    }
}

/// 图形集合
#[derive(Debug)]
pub struct ShapeCollection {
    shapes: Vec<Box<dyn Shape>>,
}

impl ShapeCollection {
    pub fn new() -> Self {
        ShapeCollection { shapes: Vec::new() }
    }

    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }

    pub fn total_area(&self) -> f64 {
        self.shapes.iter().map(|shape| shape.area()).sum()
    }

    pub fn total_perimeter(&self) -> f64 {
        self.shapes.iter().map(|shape| shape.perimeter()).sum()
    }

    pub fn describe_all(&self) -> Vec<String> {
        self.shapes.iter().map(|shape| shape.describe()).collect()
    }
}

/// 练习5测试函数
pub fn exercise_5_shapes_and_traits() {
    println!("\n=== 练习5: 几何图形和 Trait ===");

    // 创建各种图形
    let mut rect = Rectangle::new(5.0, 3.0);
    let mut circle = Circle::new(2.0);
    let mut triangle = Triangle::new(3.0, 4.0, 5.0).unwrap();

    println!("原始图形:");
    println!("{}", rect.describe());
    println!("{}", circle.describe());
    println!("{}", triangle.describe());

    // 绘制图形
    println!("\n绘制图形:");
    println!("{}", rect.draw());
    println!("{}", circle.draw());
    println!("{}", triangle.draw());

    // 缩放图形
    println!("\n缩放图形 (2倍):");
    rect.scale(2.0);
    circle.scale(2.0);
    triangle.scale(2.0);

    println!("{}", rect.describe());
    println!("{}", circle.describe());
    println!("{}", triangle.describe());

    // 使用图形集合
    let mut collection = ShapeCollection::new();
    collection.add_shape(Box::new(Rectangle::new(10.0, 5.0)));
    collection.add_shape(Box::new(Circle::new(3.0)));
    collection.add_shape(Box::new(Triangle::new(6.0, 8.0, 10.0).unwrap()));

    println!("\n图形集合统计:");
    println!("总面积: {:.2}", collection.total_area());
    println!("总周长: {:.2}", collection.total_perimeter());

    println!("\n所有图形描述:");
    for description in collection.describe_all() {
        println!("- {}", description);
    }
}

// ============================================================================
// 练习 6: 复杂应用 - 学生管理系统
// ============================================================================

/// 练习6: 学生管理系统
#[derive(Debug, Clone, PartialEq)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub grades: HashMap<String, f64>,
}

#[derive(Debug)]
pub struct Course {
    pub code: String,
    pub name: String,
    pub credits: u8,
}

#[derive(Debug)]
pub struct StudentManagementSystem {
    students: HashMap<u32, Student>,
    courses: HashMap<String, Course>,
    enrollments: HashMap<u32, Vec<String>>, // 学生ID -> 课程代码列表
}

impl Student {
    pub fn new(id: u32, name: String, age: u8) -> Self {
        Student {
            id,
            name,
            age,
            grades: HashMap::new(),
        }
    }

    pub fn add_grade(&mut self, course: String, grade: f64) {
        self.grades.insert(course, grade);
    }

    pub fn gpa(&self) -> f64 {
        if self.grades.is_empty() {
            0.0
        } else {
            self.grades.values().sum::<f64>() / self.grades.len() as f64
        }
    }

    pub fn course_count(&self) -> usize {
        self.grades.len()
    }

    pub fn highest_grade(&self) -> Option<f64> {
        self.grades
            .values()
            .copied()
            .fold(None, |acc, grade| Some(acc.map_or(grade, |a| a.max(grade))))
    }

    pub fn lowest_grade(&self) -> Option<f64> {
        self.grades
            .values()
            .copied()
            .fold(None, |acc, grade| Some(acc.map_or(grade, |a| a.min(grade))))
    }
}

impl Course {
    pub fn new(code: String, name: String, credits: u8) -> Self {
        Course {
            code,
            name,
            credits,
        }
    }
}

impl StudentManagementSystem {
    pub fn new() -> Self {
        StudentManagementSystem {
            students: HashMap::new(),
            courses: HashMap::new(),
            enrollments: HashMap::new(),
        }
    }

    pub fn add_student(&mut self, student: Student) {
        let student_id = student.id;
        self.students.insert(student_id, student);
        self.enrollments.insert(student_id, Vec::new());
    }

    pub fn add_course(&mut self, course: Course) {
        self.courses.insert(course.code.clone(), course);
    }

    pub fn enroll_student(&mut self, student_id: u32, course_code: String) -> Result<(), String> {
        if !self.students.contains_key(&student_id) {
            return Err("学生不存在".to_string());
        }

        if !self.courses.contains_key(&course_code) {
            return Err("课程不存在".to_string());
        }

        if let Some(enrollments) = self.enrollments.get_mut(&student_id) {
            if !enrollments.contains(&course_code) {
                enrollments.push(course_code);
            }
        }

        Ok(())
    }

    pub fn add_grade(
        &mut self,
        student_id: u32,
        course_code: String,
        grade: f64,
    ) -> Result<(), String> {
        if let Some(student) = self.students.get_mut(&student_id) {
            if self.courses.contains_key(&course_code) {
                student.add_grade(course_code, grade);
                Ok(())
            } else {
                Err("课程不存在".to_string())
            }
        } else {
            Err("学生不存在".to_string())
        }
    }

    pub fn get_student(&self, student_id: u32) -> Option<&Student> {
        self.students.get(&student_id)
    }

    pub fn get_course(&self, course_code: &str) -> Option<&Course> {
        self.courses.get(course_code)
    }

    pub fn students_by_gpa(&self, min_gpa: f64) -> Vec<&Student> {
        self.students
            .values()
            .filter(|student| student.gpa() >= min_gpa)
            .collect()
    }

    pub fn top_students(&self, count: usize) -> Vec<&Student> {
        let mut students: Vec<&Student> = self.students.values().collect();
        students.sort_by(|a, b| b.gpa().partial_cmp(&a.gpa()).unwrap());
        students.into_iter().take(count).collect()
    }

    pub fn course_statistics(&self, course_code: &str) -> Option<(f64, f64, f64, usize)> {
        let grades: Vec<f64> = self
            .students
            .values()
            .filter_map(|student| student.grades.get(course_code))
            .copied()
            .collect();

        if grades.is_empty() {
            None
        } else {
            let average = grades.iter().sum::<f64>() / grades.len() as f64;
            let max = grades.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let min = grades.iter().copied().fold(f64::INFINITY, f64::min);
            let count = grades.len();

            Some((average, max, min, count))
        }
    }

    pub fn student_count(&self) -> usize {
        self.students.len()
    }

    pub fn course_count(&self) -> usize {
        self.courses.len()
    }
}

/// 练习6测试函数
pub fn exercise_6_student_management() {
    println!("\n=== 练习6: 学生管理系统 ===");

    let mut system = StudentManagementSystem::new();

    // 添加课程
    system.add_course(Course::new(
        "CS101".to_string(),
        "计算机科学导论".to_string(),
        3,
    ));
    system.add_course(Course::new(
        "MATH201".to_string(),
        "高等数学".to_string(),
        4,
    ));
    system.add_course(Course::new("ENG101".to_string(), "大学英语".to_string(), 2));

    // 添加学生
    system.add_student(Student::new(1001, "张三".to_string(), 20));
    system.add_student(Student::new(1002, "李四".to_string(), 19));
    system.add_student(Student::new(1003, "王五".to_string(), 21));

    println!(
        "系统状态: {} 名学生, {} 门课程",
        system.student_count(),
        system.course_count()
    );

    // 学生选课
    system.enroll_student(1001, "CS101".to_string()).unwrap();
    system.enroll_student(1001, "MATH201".to_string()).unwrap();
    system.enroll_student(1002, "CS101".to_string()).unwrap();
    system.enroll_student(1002, "ENG101".to_string()).unwrap();
    system.enroll_student(1003, "MATH201".to_string()).unwrap();
    system.enroll_student(1003, "ENG101".to_string()).unwrap();

    // 添加成绩
    system.add_grade(1001, "CS101".to_string(), 85.0).unwrap();
    system.add_grade(1001, "MATH201".to_string(), 92.0).unwrap();
    system.add_grade(1002, "CS101".to_string(), 78.0).unwrap();
    system.add_grade(1002, "ENG101".to_string(), 88.0).unwrap();
    system.add_grade(1003, "MATH201".to_string(), 95.0).unwrap();
    system.add_grade(1003, "ENG101".to_string(), 82.0).unwrap();

    // 显示学生信息
    println!("\n学生成绩信息:");
    for id in [1001, 1002, 1003] {
        if let Some(student) = system.get_student(id) {
            println!(
                "{} (ID: {}): GPA={:.2}, 最高分={:.1}, 最低分={:.1}",
                student.name,
                student.id,
                student.gpa(),
                student.highest_grade().unwrap_or(0.0),
                student.lowest_grade().unwrap_or(0.0)
            );
        }
    }

    // 显示优秀学生
    println!("\nGPA >= 85 的学生:");
    for student in system.students_by_gpa(85.0) {
        println!("- {} (GPA: {:.2})", student.name, student.gpa());
    }

    // 显示前2名学生
    println!("\n前2名学生:");
    for (i, student) in system.top_students(2).iter().enumerate() {
        println!("{}. {} (GPA: {:.2})", i + 1, student.name, student.gpa());
    }

    // 课程统计
    println!("\n课程统计:");
    for course_code in ["CS101", "MATH201", "ENG101"] {
        if let Some((avg, max, min, count)) = system.course_statistics(course_code) {
            println!(
                "{}: 平均分={:.1}, 最高分={:.1}, 最低分={:.1}, 学生数={}",
                course_code, avg, max, min, count
            );
        }
    }
}

// ============================================================================
// 运行所有练习
// ============================================================================

/// 运行所有练习
pub fn run_all_exercises() {
    println!("🎯 Rust 结构体练习题");
    println!("====================");

    exercise_1_person_struct();
    exercise_2_tuple_structs();
    exercise_3_generic_container();
    exercise_4_text_analyzer();
    exercise_5_shapes_and_traits();
    exercise_6_student_management();

    println!("\n✅ 所有练习完成！");
}

// ============================================================================
// 测试模块
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_bmi() {
        let person = Person::new("Test".to_string(), 25, 1.75, 70.0);
        assert!((person.bmi() - 22.86).abs() < 0.01);
        assert_eq!(person.bmi_category(), "正常");
    }

    #[test]
    fn test_temperature_conversion() {
        let celsius = Celsius::new(0.0);
        let fahrenheit = celsius.to_fahrenheit();
        assert_eq!(fahrenheit.value(), 32.0);

        let kelvin = celsius.to_kelvin();
        assert_eq!(kelvin.value(), 273.15);
    }

    #[test]
    fn test_container_operations() {
        let mut container = Container::new(2);
        assert!(container.add("item1".to_string()).is_ok());
        assert!(container.add("item2".to_string()).is_ok());
        assert!(container.add("item3".to_string()).is_err());

        assert!(container.is_full());
        assert_eq!(container.len(), 2);
    }

    #[test]
    fn test_text_analyzer() {
        let analyzer = TextAnalyzer::new("hello world hello");
        assert_eq!(analyzer.word_count(), 3);
        assert_eq!(analyzer.longest_word(), Some("hello"));

        let frequency = analyzer.word_frequency();
        assert_eq!(frequency.get("hello"), Some(&2));
        assert_eq!(frequency.get("world"), Some(&1));
    }

    #[test]
    fn test_shapes() {
        let rect = Rectangle::new(4.0, 5.0);
        assert_eq!(rect.area(), 20.0);
        assert_eq!(rect.perimeter(), 18.0);

        let circle = Circle::new(1.0);
        assert!((circle.area() - std::f64::consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_student_system() {
        let mut system = StudentManagementSystem::new();
        let student = Student::new(1, "Test".to_string(), 20);

        system.add_student(student);
        system.add_course(Course::new(
            "TEST".to_string(),
            "Test Course".to_string(),
            3,
        ));

        assert!(system.enroll_student(1, "TEST".to_string()).is_ok());
        assert!(system.add_grade(1, "TEST".to_string(), 85.0).is_ok());

        let student = system.get_student(1).unwrap();
        assert_eq!(student.gpa(), 85.0);
    }
}
