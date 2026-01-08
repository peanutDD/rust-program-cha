//! Rust 结构体全面教程
//!
//! 本模块涵盖了 Rust 结构体的所有核心概念和高级特性
//! 包括：定义、实例化、方法、关联函数、生命周期、所有权等

use std::collections::HashMap;
use std::fmt;

// ============================================================================
// 1. 基础结构体定义和实例化
// ============================================================================

/// 基础用户结构体 - 演示结构体的基本定义
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
}

/// 演示结构体的基本操作
pub fn basic_struct_operations() {
    println!("\n=== 1. 基础结构体操作 ===");

    // 创建结构体实例
    let user1 = User {
        active: true,
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
    };

    println!("用户1: {:?}", user1);

    // 字段简写语法
    let username = String::from("bob");
    let email = String::from("bob@example.com");

    let user2 = User {
        username, // 等同于 username: username
        email,    // 等同于 email: email
        active: true,
        sign_in_count: 1,
    };

    println!("用户2: {:?}", user2);

    // 结构体更新语法
    let user3 = User {
        email: String::from("charlie@example.com"),
        ..user1 // 使用 user1 的其他字段值
    };

    println!("用户3: {:?}", user3);

    // 可变结构体
    let mut user4 = User {
        active: false,
        username: String::from("david"),
        email: String::from("david@example.com"),
        sign_in_count: 0,
    };

    user4.active = true;
    user4.sign_in_count += 1;
    println!("修改后的用户4: {:?}", user4);
}

// ============================================================================
// 2. 元组结构体
// ============================================================================

/// RGB 颜色 - 元组结构体示例
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);

/// 3D 点坐标 - 元组结构体示例
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D(pub f64, pub f64, pub f64);

/// 新类型模式 - 用于类型安全
#[derive(Debug, Clone, PartialEq)]
pub struct UserId(pub u64);

#[derive(Debug, Clone, PartialEq)]
pub struct Email(pub String);

/// 演示元组结构体的使用
pub fn tuple_struct_operations() {
    println!("\n=== 2. 元组结构体操作 ===");

    // 创建元组结构体实例
    let red = Color(255, 0, 0);
    let green = Color(0, 255, 0);
    let blue = Color(0, 0, 255);

    println!("红色: {:?}, 绿色: {:?}, 蓝色: {:?}", red, green, blue);

    // 访问元组结构体字段
    println!("红色的RGB值: ({}, {}, {})", red.0, red.1, red.2);

    // 3D 点操作
    let origin = Point3D(0.0, 0.0, 0.0);
    let point = Point3D(1.0, 2.0, 3.0);

    println!("原点: {:?}, 点: {:?}", origin, point);

    // 新类型模式 - 提供类型安全
    let user_id = UserId(12345);
    let email = Email(String::from("user@example.com"));

    println!("用户ID: {:?}, 邮箱: {:?}", user_id, email);

    // 解构元组结构体
    let Color(r, g, b) = red;
    println!("解构后的红色值: r={}, g={}, b={}", r, g, b);
}

// ============================================================================
// 3. 单元结构体
// ============================================================================

/// 单元结构体 - 没有字段的结构体
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Unit;

/// 标记类型 - 用于类型系统
#[derive(Debug)]
pub struct Marker;

/// 演示单元结构体的使用
pub fn unit_struct_operations() {
    println!("\n=== 3. 单元结构体操作 ===");

    let unit1 = Unit;
    let unit2 = Unit {}; // 另一种创建方式

    println!("单元结构体1: {:?}", unit1);
    println!("单元结构体2: {:?}", unit2);
    println!("两个单元结构体相等: {}", unit1 == unit2);

    let marker = Marker;
    println!("标记类型: {:?}", marker);
}

// ============================================================================
// 4. 结构体方法和关联函数
// ============================================================================

/// 矩形结构体 - 演示方法和关联函数
#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// 关联函数 - 创建新的矩形（构造函数）
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    /// 关联函数 - 创建正方形
    pub fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }

    /// 方法 - 计算面积
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /// 方法 - 计算周长
    pub fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    /// 方法 - 检查是否为正方形
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }

    /// 方法 - 检查是否能容纳另一个矩形
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /// 可变方法 - 缩放矩形
    pub fn scale(&mut self, factor: f64) {
        self.width = (self.width as f64 * factor) as u32;
        self.height = (self.height as f64 * factor) as u32;
    }

    /// 消费方法 - 转换为面积值
    pub fn into_area(self) -> u32 {
        self.area()
    }
}

/// 演示结构体方法和关联函数
pub fn struct_methods_demo() {
    println!("\n=== 4. 结构体方法和关联函数 ===");

    // 使用关联函数创建实例
    let rect1 = Rectangle::new(30, 50);
    let square = Rectangle::square(25);

    println!("矩形1: {:?}", rect1);
    println!("正方形: {:?}", square);

    // 调用方法
    println!("矩形1面积: {}", rect1.area());
    println!("矩形1周长: {}", rect1.perimeter());
    println!("矩形1是正方形: {}", rect1.is_square());
    println!("正方形是正方形: {}", square.is_square());

    // 方法链调用
    let rect2 = Rectangle::new(10, 40);
    println!("矩形1能容纳矩形2: {}", rect1.can_hold(&rect2));
    println!("矩形2能容纳矩形1: {}", rect2.can_hold(&rect1));

    // 可变方法
    let mut rect3 = Rectangle::new(20, 30);
    println!("缩放前: {:?}", rect3);
    rect3.scale(1.5);
    println!("缩放后: {:?}", rect3);

    // 消费方法
    let rect4 = Rectangle::new(15, 25);
    let area = rect4.into_area(); // rect4 被消费
    println!("转换为面积值: {}", area);
    // println!("{:?}", rect4); // 编译错误：rect4 已被移动
}

// ============================================================================
// 5. 泛型结构体
// ============================================================================

/// 泛型点结构体
#[derive(Debug, Clone, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    /// 创建新点
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T: Copy> Point<T> {
    /// 获取 x 坐标（需要 Copy trait）
    #[allow(dead_code)] // 示例代码，供学习者参考
    pub fn x(&self) -> T {
        self.x
    }

    /// 获取 y 坐标（需要 Copy trait）
    #[allow(dead_code)] // 示例代码，供学习者参考
    pub fn y(&self) -> T {
        self.y
    }
}

impl Point<f64> {
    /// 计算到原点的距离（仅适用于 f64）
    pub fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

/// 混合泛型结构体
#[derive(Debug, Clone, PartialEq)]
pub struct MixedPoint<T, U> {
    pub x: T,
    pub y: U,
}

impl<T, U> MixedPoint<T, U> {
    /// 创建混合点
    pub fn new(x: T, y: U) -> Self {
        MixedPoint { x, y }
    }

    /// 混合方法 - 返回不同类型的点
    pub fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

/// 演示泛型结构体
pub fn generic_struct_demo() {
    println!("\n=== 5. 泛型结构体 ===");

    // 不同类型的点
    let integer_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);
    let char_point = Point::new('x', 'y');

    println!("整数点: {:?}", integer_point);
    println!("浮点数点: {:?}", float_point);
    println!("字符点: {:?}", char_point);

    // 使用特定实现的方法
    println!(
        "浮点数点到原点距离: {:.2}",
        float_point.distance_from_origin()
    );

    // 混合类型点
    let mixed1 = MixedPoint::new(5, 4.0);
    let mixed2 = MixedPoint::new("Hello", 'c');

    println!("混合点1: {:?}", mixed1);
    println!("混合点2: {:?}", mixed2);

    // 混合操作
    let mixed3 = mixed1.mixup(mixed2);
    println!("混合后的点: {:?}", mixed3);
}

// ============================================================================
// 6. 结构体与所有权
// ============================================================================

/// 拥有所有权的结构体
#[derive(Debug, Clone)]
pub struct OwnedData {
    pub name: String,
    pub data: Vec<i32>,
}

/// 借用数据的结构体（需要生命周期）
#[derive(Debug)]
pub struct BorrowedData<'a> {
    pub name: &'a str,
    pub data: &'a [i32],
}

/// 混合所有权的结构体
#[derive(Debug)]
pub struct MixedOwnership<'a> {
    pub owned_name: String,
    pub borrowed_name: &'a str,
    pub owned_data: Vec<i32>,
    pub borrowed_data: &'a [i32],
}

/// 演示结构体与所有权
pub fn ownership_demo() {
    println!("\n=== 6. 结构体与所有权 ===");

    // 拥有所有权的结构体
    let owned = OwnedData {
        name: String::from("拥有的数据"),
        data: vec![1, 2, 3, 4, 5],
    };

    println!("拥有所有权的数据: {:?}", owned);
    println!("数据名称: {}, 数据长度: {}", owned.name, owned.data.len());

    // 借用数据的结构体
    let name = "借用的数据";
    let data = [10, 20, 30, 40, 50];

    let borrowed = BorrowedData {
        name: &name,
        data: &data,
    };

    println!("借用的数据: {:?}", borrowed);
    println!("数据名称: {}, 数据长度: {}", borrowed.name, borrowed.data.len());
    println!("数据名称: {}, 数据长度: {}", borrowed.name, borrowed.data.len());

    // 混合所有权
    let mixed = MixedOwnership {
        owned_name: String::from("拥有的名称"),
        borrowed_name: &name,
        owned_data: vec![100, 200, 300],
        borrowed_data: &data,
    };

    println!("混合所有权数据: {:?}", mixed);
    println!(
        "混合数据: 拥有名称={}, 借用名称={}, 拥有数据长度={}, 借用数据长度={}",
        mixed.owned_name,
        mixed.borrowed_name,
        mixed.owned_data.len(),
        mixed.borrowed_data.len()
    );

    // 演示移动语义
    let owned2 = owned; // owned 被移动到 owned2
    println!("移动后的数据: {:?}", owned2);
    // println!("{:?}", owned); // 编译错误：owned 已被移动

    // 克隆避免移动
    let owned3 = owned2.clone();
    println!("克隆的数据: {:?}", owned3);
    println!("原始数据仍可用: {:?}", owned2);
}

// ============================================================================
// 7. 结构体与 Trait
// ============================================================================

/// 可显示的 trait
pub trait Display {
    fn display(&self) -> String;
}

/// 可计算的 trait
pub trait Calculable {
    fn calculate(&self) -> f64;
}

/// 学生结构体
#[derive(Debug, Clone, PartialEq)]
pub struct Student {
    pub name: String,
    pub age: u8,
    pub grades: Vec<f64>,
}

impl Student {
    pub fn new(name: String, age: u8) -> Self {
        Student {
            name,
            age,
            grades: Vec::new(),
        }
    }

    pub fn add_grade(&mut self, grade: f64) {
        self.grades.push(grade);
    }
}

impl Display for Student {
    fn display(&self) -> String {
        format!(
            "学生: {}, 年龄: {}, 成绩数: {}",
            self.name,
            self.age,
            self.grades.len()
        )
    }
}

impl Calculable for Student {
    fn calculate(&self) -> f64 {
        if self.grades.is_empty() {
            0.0
        } else {
            self.grades.iter().sum::<f64>() / self.grades.len() as f64
        }
    }
}

/// 圆形结构体
#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Circle { radius }
    }
}

impl Display for Circle {
    fn display(&self) -> String {
        format!("圆形，半径: {:.2}", self.radius)
    }
}

impl Calculable for Circle {
    fn calculate(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

/// 演示结构体与 Trait
pub fn trait_demo() {
    println!("\n=== 7. 结构体与 Trait ===");

    // 创建学生
    let mut student = Student::new(String::from("张三"), 20);
    student.add_grade(85.0);
    student.add_grade(92.0);
    student.add_grade(78.0);

    println!("{}", student.display());
    println!("平均成绩: {:.2}", student.calculate());

    // 创建圆形
    let circle = Circle::new(5.0);
    println!("{}", circle.display());
    println!("面积: {:.2}", circle.calculate());

    // 使用 trait 对象
    let displayables: Vec<&dyn Display> = vec![&student, &circle];

    println!("\n所有可显示的对象:");
    for item in displayables {
        println!("- {}", item.display());
    }

    // 使用泛型函数
    fn print_calculation<T: Display + Calculable>(item: &T) {
        println!("{} -> 计算结果: {:.2}", item.display(), item.calculate());
    }

    println!("\n泛型函数调用:");
    print_calculation(&student);
    print_calculation(&circle);
}

// ============================================================================
// 8. 高级特性：自定义 Debug 和 Display
// ============================================================================

/// 产品结构体
#[derive(Clone, PartialEq)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub price: f64,
    pub category: String,
}

impl Product {
    pub fn new(id: u64, name: String, price: f64, category: String) -> Self {
        Product {
            id,
            name,
            price,
            category,
        }
    }
}

// 自定义 Debug 实现
impl fmt::Debug for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Product")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("price", &format_args!("${:.2}", self.price))
            .field("category", &self.category)
            .finish()
    }
}

// 自定义 Display 实现
impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({}) - ${:.2} [{}]",
            self.name, self.id, self.price, self.category
        )
    }
}

/// 演示自定义格式化
pub fn custom_formatting_demo() {
    println!("\n=== 8. 自定义格式化 ===");

    let product = Product::new(
        1001,
        String::from("Rust 编程书籍"),
        59.99,
        String::from("图书"),
    );

    println!("Display 格式: {}", product);
    println!("Debug 格式: {:?}", product);
    println!("美化 Debug 格式: {:#?}", product);
}

// ============================================================================
// 9. 实际应用案例：图书管理系统
// ============================================================================

/// 图书结构体
#[derive(Debug, Clone, PartialEq)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub year: u16,
    pub available: bool,
}

/// 图书馆结构体
#[derive(Debug)]
pub struct Library {
    pub name: String,
    pub books: HashMap<String, Book>,
    pub borrowed_books: HashMap<String, String>, // ISBN -> 借阅者
}

impl Book {
    pub fn new(isbn: String, title: String, author: String, year: u16) -> Self {
        Book {
            isbn,
            title,
            author,
            year,
            available: true,
        }
    }
}

impl Library {
    pub fn new(name: String) -> Self {
        Library {
            name,
            books: HashMap::new(),
            borrowed_books: HashMap::new(),
        }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.insert(book.isbn.clone(), book);
    }

    // ✅ 优化：使用 &'static str 作为错误消息
    pub fn borrow_book(&mut self, isbn: &str, borrower: String) -> Result<(), &'static str> {
        match self.books.get_mut(isbn) {
            Some(book) if book.available => {
                book.available = false;
                self.borrowed_books.insert(isbn.to_string(), borrower);
                Ok(())
            }
            Some(_) => Err("图书已被借出"),
            None => Err("图书不存在"),
        }
    }

    // ✅ 优化：使用 &'static str 作为错误消息
    pub fn return_book(&mut self, isbn: &str) -> Result<(), &'static str> {
        match self.books.get_mut(isbn) {
            Some(book) if !book.available => {
                book.available = true;
                self.borrowed_books.remove(isbn);
                Ok(())
            }
            Some(_) => Err("图书未被借出"),
            None => Err("图书不存在"),
        }
    }

    pub fn search_by_author(&self, author: &str) -> Vec<&Book> {
        self.books
            .values()
            .filter(|book| book.author.contains(author))
            .collect()
    }

    pub fn available_books(&self) -> Vec<&Book> {
        self.books.values().filter(|book| book.available).collect()
    }

    pub fn borrowed_books_info(&self) -> Vec<(String, String, String)> {
        self.borrowed_books
            .iter()
            .filter_map(|(isbn, borrower)| {
                self.books
                    .get(isbn)
                    .map(|book| (book.title.clone(), borrower.clone(), isbn.clone()))
            })
            .collect()
    }
}

/// 演示图书管理系统
pub fn library_system_demo() {
    println!("\n=== 9. 图书管理系统案例 ===");

    let mut library = Library::new(String::from("市立图书馆"));

    // 添加图书
    library.add_book(Book::new(
        "978-1-59327-828-5".to_string(),
        "Rust 程序设计语言".to_string(),
        "Steve Klabnik".to_string(),
        2018,
    ));

    library.add_book(Book::new(
        "978-0-13-110362-7".to_string(),
        "C++ Primer".to_string(),
        "Stanley Lippman".to_string(),
        2012,
    ));

    library.add_book(Book::new(
        "978-0-321-56384-2".to_string(),
        "Effective Modern C++".to_string(),
        "Scott Meyers".to_string(),
        2014,
    ));

    println!("图书馆: {}", library.name);
    println!("总图书数: {}", library.books.len());

    // 显示可借图书
    println!("\n可借图书:");
    for book in library.available_books() {
        println!("- {} by {} ({})", book.title, book.author, book.year);
    }

    // 借书
    match library.borrow_book("978-1-59327-828-5", "张三".to_string()) {
        Ok(()) => println!("\n张三成功借阅了 Rust 程序设计语言"),
        Err(e) => println!("借书失败: {}", e),
    }

    match library.borrow_book("978-0-13-110362-7", "李四".to_string()) {
        Ok(()) => println!("李四成功借阅了 C++ Primer"),
        Err(e) => println!("借书失败: {}", e),
    }

    // 显示借阅信息
    println!("\n当前借阅情况:");
    for (title, borrower, isbn) in library.borrowed_books_info() {
        println!("- {} 被 {} 借阅 (ISBN: {})", title, borrower, isbn);
    }

    // 按作者搜索
    println!("\n搜索作者包含 'Scott' 的图书:");
    for book in library.search_by_author("Scott") {
        println!("- {} (可借: {})", book.title, book.available);
    }

    // 还书
    match library.return_book("978-1-59327-828-5") {
        Ok(()) => println!("\nRust 程序设计语言 已归还"),
        Err(e) => println!("还书失败: {}", e),
    }

    println!("\n归还后可借图书数: {}", library.available_books().len());
}

// ============================================================================
// 10. 运行所有示例
// ============================================================================

/// 运行所有结构体示例
pub fn run_all_examples() {
    println!("🦀 Rust 结构体全面教程");
    println!("========================");

    basic_struct_operations();
    tuple_struct_operations();
    unit_struct_operations();
    struct_methods_demo();
    generic_struct_demo();
    ownership_demo();
    trait_demo();
    custom_formatting_demo();
    library_system_demo();

    println!("\n✅ 所有结构体示例运行完成！");
}

// ============================================================================
// 测试模块
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User {
            active: true,
            username: String::from("test"),
            email: String::from("test@example.com"),
            sign_in_count: 1,
        };

        assert_eq!(user.username, "test");
        assert!(user.active);
    }

    #[test]
    fn test_rectangle_methods() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.area(), 200);
        assert_eq!(rect.perimeter(), 60);
        assert!(!rect.is_square());

        let square = Rectangle::square(15);
        assert!(square.is_square());
    }

    #[test]
    fn test_generic_point() {
        let int_point = Point::new(5, 10);
        let float_point = Point::new(3.0, 4.0);

        assert_eq!(int_point.x(), 5);
        assert_eq!(float_point.distance_from_origin(), 5.0);
    }

    #[test]
    fn test_library_operations() {
        let mut library = Library::new(String::from("测试图书馆"));
        let book = Book::new(
            "123".to_string(),
            "测试书籍".to_string(),
            "测试作者".to_string(),
            2023,
        );

        library.add_book(book);
        assert_eq!(library.books.len(), 1);

        assert!(library.borrow_book("123", "测试用户".to_string()).is_ok());
        assert!(library.borrow_book("123", "另一用户".to_string()).is_err());

        assert!(library.return_book("123").is_ok());
        assert!(library.return_book("123").is_err());
    }

    #[test]
    fn test_color_operations() {
        let red = Color(255, 0, 0);
        let Color(r, g, b) = red;

        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
    }

    #[test]
    fn test_student_traits() {
        let mut student = Student::new(String::from("测试学生"), 20);
        student.add_grade(90.0);
        student.add_grade(80.0);

        assert_eq!(student.calculate(), 85.0);
        assert!(student.display().contains("测试学生"));
    }
}
