//! # Rust 编程难点代码示例集合
//! 
//! 这个文件包含了 Rust 编程中所有主要难点的完整可运行示例
//! 配套《Rust 编程语言全面难点总结与最优解决方案》使用
//! 
//! ## 使用方法
//! ```bash
//! # 编译并运行所有示例
//! cargo run
//! ```

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::{self, Display, Debug};
use std::error::Error as StdError;

// ============================================================================
// 宏定义（必须在使用前定义）
// ============================================================================

macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

macro_rules! calculate_and_print {
    ( $( $expr:expr ),* ) => {
        $(
            println!("    计算 {} = {}", stringify!($expr), $expr);
        )*
    };
}

macro_rules! debug_print {
    ($msg:expr) => {
        #[cfg(debug_assertions)]
        println!("    [DEBUG] {}", $msg);
    };
}

fn main() {
    println!("🦀 Rust 编程难点代码示例集合 🦀");
    println!("{}", "=".repeat(50));
    
    // 1. 所有权系统难点
    println!("\n📦 1. 所有权系统难点示例");
    ownership_examples();
    
    // 2. 生命周期难点
    println!("\n⏰ 2. 生命周期难点示例");
    lifetime_examples();
    
    // 3. 借用检查器难点
    println!("\n🔍 3. 借用检查器难点示例");
    borrow_checker_examples();
    
    // 4. 智能指针难点
    println!("\n🧠 4. 智能指针难点示例");
    smart_pointer_examples();
    
    // 5. Trait 系统难点
    println!("\n🎭 5. Trait 系统难点示例");
    trait_system_examples();
    
    // 6. 泛型编程难点
    println!("\n🔧 6. 泛型编程难点示例");
    generic_programming_examples();
    
    // 7. 并发编程难点
    println!("\n🚀 7. 并发编程难点示例");
    concurrency_examples();
    
    // 8. 宏编程难点
    println!("\n🎪 8. 宏编程难点示例");
    macro_programming_examples();
    
    // 9. 错误处理难点
    println!("\n❌ 9. 错误处理难点示例");
    error_handling_examples();
    
    // 10. 内存管理难点
    println!("\n💾 10. 内存管理难点示例");
    memory_management_examples();
    
    println!("\n✅ 所有示例运行完成！");
}

// ============================================================================
// 1. 所有权系统难点
// ============================================================================

fn ownership_examples() {
    // 示例1：移动语义
    let s1 = String::from("hello");
    let s2 = s1; // s1 被移动到 s2
    // println!("{}", s1); // 这会编译错误
    println!("    移动后的字符串: {}", s2);
    
    // 示例2：借用
    let s3 = String::from("world");
    let len = calculate_length(&s3);
    println!("    字符串 '{}' 的长度是 {}", s3, len);
    
    // 示例3：可变借用
    let mut s4 = String::from("hello");
    change_string(&mut s4);
    println!("    修改后的字符串: {}", s4);
    
    // 示例4：多个不可变借用
    let s5 = String::from("rust");
    let r1 = &s5;
    let r2 = &s5;
    println!("    多个不可变借用: {} 和 {}", r1, r2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(", world");
}

// ============================================================================
// 2. 生命周期难点
// ============================================================================

fn lifetime_examples() {
    // 示例1：函数中的生命周期
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("    最长的字符串是: {}", result);
    
    // 示例2：结构体中的生命周期
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("    重要摘录: {}", i.part);
    
    // 示例3：方法中的生命周期
    let s = "Hello world";
    let word = first_word(s);
    println!("    第一个单词: {}", word);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// ============================================================================
// 3. 借用检查器难点
// ============================================================================

fn borrow_checker_examples() {
    // 示例1：可变和不可变借用冲突
    let mut data = vec![1, 2, 3, 4, 5];
    
    // 正确的方式：分开进行借用
    {
        let first = &data[0];
        println!("    第一个元素: {}", first);
    }
    
    data.push(6);
    println!("    添加元素后: {:?}", data);
    
    // 示例2：迭代器和修改
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("    翻倍后: {:?}", doubled);
    
    // 示例3：结构体字段借用
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    let name_ref = &person.name;
    println!("    人员姓名: {}", name_ref);
}

struct Person {
    name: String,
    age: u32,
}

// ============================================================================
// 4. 智能指针难点
// ============================================================================

fn smart_pointer_examples() {
    // 示例1：Box<T> - 堆分配
    let b = Box::new(5);
    println!("    Box 中的值: {}", b);
    
    // 示例2：递归类型
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("    链表创建成功");
    
    // 示例3：Rc<T> - 引用计数
    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("    a 的引用计数: {}", Rc::strong_count(&a));
    
    let _b = Cons2(3, Rc::clone(&a));
    println!("    创建 b 后，a 的引用计数: {}", Rc::strong_count(&a));
    
    {
        let _c = Cons2(4, Rc::clone(&a));
        println!("    创建 c 后，a 的引用计数: {}", Rc::strong_count(&a));
    }
    
    println!("    c 离开作用域后，a 的引用计数: {}", Rc::strong_count(&a));
    
    // 示例4：RefCell<T> - 内部可变性
    let data = Rc::new(RefCell::new(5));
    
    let a = Rc::clone(&data);
    let b = Rc::clone(&data);
    
    *a.borrow_mut() += 10;
    *b.borrow_mut() += 1;
    
    println!("    RefCell 中的值: {}", data.borrow());
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

enum List2 {
    Cons2(i32, Rc<List2>),
    Nil2,
}

use List2::{Cons2, Nil2};

// ============================================================================
// 5. Trait 系统难点
// ============================================================================

fn trait_system_examples() {
    // 示例1：基本 trait 实现
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    println!("    推文摘要: {}", tweet.summarize());
    
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    
    println!("    文章摘要: {}", article.summarize());
    
    // 示例2：trait 作为参数
    notify(&tweet);
    notify(&article);
    
    // 示例3：条件 trait 实现
    let pair = Pair::new(10, 20);
    pair.cmp_display();
    
    // 示例4：自定义迭代器
    let counter = Counter::new();
    for i in counter.take(5) {
        println!("    计数器值: {}", i);
    }
    
    // 示例5：运算符重载
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2;
    println!("    点相加结果: {:?}", p3);
}

trait Summary {
    fn summarize_author(&self) -> String;
    
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn notify(item: &impl Summary) {
    println!("    通知: {}", item.summarize());
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("    最大值是 x = {}", self.x);
        } else {
            println!("    最大值是 y = {}", self.y);
        }
    }
}

struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    fn new() -> Counter {
        Counter { current: 0, max: 5 }
    }
}

impl Iterator for Counter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// ============================================================================
// 6. 泛型编程难点
// ============================================================================

fn generic_programming_examples() {
    // 示例1：泛型函数
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("    最大的数字是: {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("    最大的字符是: {}", result);
    
    // 示例2：泛型结构体
    let integer = GenericPoint { x: 5, y: 10 };
    let float = GenericPoint { x: 1.0, y: 4.0 };
    
    println!("    整数点: x = {}, y = {}", integer.x(), integer.y);
    println!("    浮点数点: x = {}, y = {}", float.x(), float.y);
    
    // 示例3：混合类型点
    let mixed = MixedPoint { x: 5, y: 4.0 };
    println!("    混合点: x = {}, y = {}", mixed.x, mixed.y);
    
    // 示例4：特定类型的方法
    let p = GenericPoint { x: 1.0, y: 3.0 };
    println!("    距离原点: {}", p.distance_from_origin());
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

struct GenericPoint<T> {
    x: T,
    y: T,
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T> GenericPoint<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl GenericPoint<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// ============================================================================
// 7. 并发编程难点
// ============================================================================

fn concurrency_examples() {
    // 示例1：基本线程
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("    子线程: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..5 {
        println!("    主线程: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();
    
    // 示例2：移动闭包
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("    线程中的向量: {:?}", v);
    });
    handle.join().unwrap();
    
    // 示例3：共享状态 - Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("    计数器结果: {}", *counter.lock().unwrap());
}

// ============================================================================
// 8. 宏编程难点
// ============================================================================

fn macro_programming_examples() {
    // 示例1：自定义 vec! 宏
    let v = my_vec![1, 2, 3];
    println!("    自定义向量: {:?}", v);
    
    // 示例2：计算宏
    calculate_and_print!(2 + 3, 4 * 5, 10 - 2);
    
    // 示例3：条件编译宏
    debug_print!("这是一个调试信息");
    
    println!("    宏编程示例完成");
}

// ============================================================================
// 9. 错误处理难点
// ============================================================================

fn error_handling_examples() {
    // 示例1：Result 类型
    match divide(10.0, 2.0) {
        Ok(result) => println!("    除法结果: {}", result),
        Err(e) => println!("    除法错误: {}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("    除法结果: {}", result),
        Err(e) => println!("    除法错误: {}", e),
    }
    
    // 示例2：? 操作符
    match read_and_parse() {
        Ok(number) => println!("    解析的数字: {}", number),
        Err(e) => println!("    解析错误: {}", e),
    }
    
    // 示例3：自定义错误类型
    match validate_age(15) {
        Ok(age) => println!("    有效年龄: {}", age),
        Err(e) => println!("    年龄验证错误: {}", e),
    }
    
    match validate_age(25) {
        Ok(age) => println!("    有效年龄: {}", age),
        Err(e) => println!("    年龄验证错误: {}", e),
    }
    
    // 示例4：错误链
    match process_file("nonexistent.txt") {
        Ok(content) => println!("    文件内容: {}", content),
        Err(e) => {
            println!("    文件处理错误: {}", e);
            if let Some(source) = e.source() {
                println!("    原因: {}", source);
            }
        }
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}

fn read_and_parse() -> Result<i32, Box<dyn StdError>> {
    let s = "42";
    let number: i32 = s.parse()?;
    Ok(number)
}

#[derive(Debug)]
enum ValidationError {
    TooYoung,
    TooOld,
    InvalidAge,
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::TooYoung => write!(f, "年龄太小"),
            ValidationError::TooOld => write!(f, "年龄太大"),
            ValidationError::InvalidAge => write!(f, "无效年龄"),
        }
    }
}

impl StdError for ValidationError {}

fn validate_age(age: u32) -> Result<u32, ValidationError> {
    if age < 18 {
        Err(ValidationError::TooYoung)
    } else if age > 100 {
        Err(ValidationError::TooOld)
    } else {
        Ok(age)
    }
}

#[derive(Debug)]
struct FileProcessError {
    message: String,
    source: Option<Box<dyn StdError>>,
}

impl Display for FileProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "文件处理错误: {}", self.message)
    }
}

impl StdError for FileProcessError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

fn process_file(filename: &str) -> Result<String, FileProcessError> {
    Err(FileProcessError {
        message: format!("无法处理文件: {}", filename),
        source: None,
    })
}

// ============================================================================
// 10. 内存管理难点
// ============================================================================

fn memory_management_examples() {
    // 示例1：栈 vs 堆
    let stack_array = [1, 2, 3, 4, 5]; // 栈上分配
    let heap_vector = vec![1, 2, 3, 4, 5]; // 堆上分配
    
    println!("    栈数组: {:?}", stack_array);
    println!("    堆向量: {:?}", heap_vector);
    
    // 示例2：Drop trait
    {
        let _custom_drop = CustomDrop::new("临时资源");
        println!("    创建了临时资源");
    } // custom_drop 在这里被 drop
    
    // 示例3：内存泄漏预防
    let _leak_prevention = LeakPrevention::new();
    println!("    创建了防泄漏资源");
    
    // 示例4：内存对齐
    println!("    AlignedStruct 大小: {}", std::mem::size_of::<AlignedStruct>());
    println!("    AlignedStruct 对齐: {}", std::mem::align_of::<AlignedStruct>());
}

struct CustomDrop {
    name: String,
}

impl CustomDrop {
    fn new(name: &str) -> Self {
        println!("    创建资源: {}", name);
        CustomDrop {
            name: name.to_string(),
        }
    }
}

impl Drop for CustomDrop {
    fn drop(&mut self) {
        println!("    清理资源: {}", self.name);
    }
}

struct LeakPrevention {
    data: Vec<i32>,
}

impl LeakPrevention {
    fn new() -> Self {
        LeakPrevention {
            data: vec![1, 2, 3, 4, 5],
        }
    }
}

impl Drop for LeakPrevention {
    fn drop(&mut self) {
        println!("    防泄漏资源被正确清理，数据长度: {}", self.data.len());
    }
}

#[repr(C)]
struct AlignedStruct {
    a: u8,
    b: u32,
    c: u16,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ownership() {
        let s = String::from("test");
        let len = calculate_length(&s);
        assert_eq!(len, 4);
        assert_eq!(s, "test"); // s 仍然有效
    }
    
    #[test]
    fn test_generic_function() {
        let numbers = vec![1, 2, 3, 4, 5];
        let max = largest(&numbers);
        assert_eq!(max, 5);
    }
    
    #[test]
    fn test_error_handling() {
        assert!(divide(10.0, 2.0).is_ok());
        assert!(divide(10.0, 0.0).is_err());
        
        assert!(validate_age(25).is_ok());
        assert!(validate_age(15).is_err());
    }
    
    #[test]
    fn test_macro() {
        let v = my_vec![1, 2, 3];
        assert_eq!(v, vec![1, 2, 3]);
    }
}