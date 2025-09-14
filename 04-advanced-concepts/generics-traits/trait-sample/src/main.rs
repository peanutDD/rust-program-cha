// Rust Trait 特征全面深入分析
// 基于 https://course.rs/basic/trait/trait.html 的完整知识点梳理
// 作者：AI Assistant
// 日期：2024

use std::clone::Clone;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::HashMap;
use std::convert::{From, Into, TryFrom, TryInto};
use std::default::Default;
use std::fmt::{Debug, Display};
use std::future::Future;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::iter::{IntoIterator, Iterator};
use std::marker::{Copy, PhantomData, Send, Sync};
use std::ops::{Add, Deref};
use std::pin::Pin;
use std::task::{Context, Poll};

// 删除重复的 main 函数，使用文件末尾的完整版本

// ==================== 1. 基础概念演示 ====================

/// Trait 是 Rust 中定义共享行为的抽象机制
/// 类似于其他语言中的接口（Interface），但功能更强大
fn basic_trait_concepts() {
    println!("\n--- 1. Trait 基础概念 ---");

    // Trait 解决的问题：
    // 1. 代码复用：多个类型可以实现相同的行为
    // 2. 抽象：定义通用的接口，不关心具体实现
    // 3. 多态：同一个接口可以有不同的实现

    println!("Trait 的核心作用：");
    println!("1. 定义共享行为的抽象接口");
    println!("2. 实现代码复用和多态");
    println!("3. 提供编译时的类型安全保证");
    println!("4. 支持泛型编程和约束");
}

// ==================== 2. Trait 定义和实现 ====================

/// 基础 trait 定义
trait Summary {
    /// 必须实现的方法
    fn summarize(&self) -> String;

    /// 带默认实现的方法
    fn summarize_author(&self) -> String {
        String::from("Unknown Author")
    }

    /// 调用其他方法的默认实现
    fn summarize_with_author(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

/// 新闻文章结构体
#[derive(Debug, Clone)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

/// 为 NewsArticle 实现 Summary trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

/// 微博结构体
#[derive(Debug, Clone)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

/// 为 Tweet 实现 Summary trait
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

/// 微博结构体（简化版）
#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}

/// 为 Weibo 实现 Summary trait（只实现必需方法）
impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo: {}", self.username, self.content)
    }
}

fn trait_definition_and_implementation() {
    println!("\n--- 2. Trait 定义和实现 ---");

    let article = NewsArticle {
        headline: "Rust 1.70 发布".to_string(),
        location: "全球".to_string(),
        author: "Rust Team".to_string(),
        content: "Rust 1.70 带来了许多新特性...".to_string(),
    };

    let tweet = Tweet {
        username: "rust_lang".to_string(),
        content: "Rust 1.70 is now available!".to_string(),
        reply: false,
        retweet: false,
    };

    let weibo = Weibo {
        username: "rust_china".to_string(),
        content: "Rust 学习资源分享".to_string(),
    };

    println!("新闻摘要: {}", article.summarize());
    println!("推特摘要: {}", tweet.summarize());
    println!("微博摘要: {}", weibo.summarize());

    println!("\n默认实现演示:");
    println!("新闻作者: {}", article.summarize_author());
    println!("推特作者: {}", tweet.summarize_author());
    println!("微博作者: {}", weibo.summarize_author()); // 使用默认实现

    println!("\n组合方法演示:");
    println!("新闻详情: {}", article.summarize_with_author());
    println!("推特详情: {}", tweet.summarize_with_author());
}

// ==================== 3. Trait 作为参数和返回值 ====================

/// 接受实现了 Summary trait 的参数
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/// 使用 trait bound 语法（等价于上面的写法）
fn notify_bound<T: Summary>(item: &T) {
    println!("Important update! {}", item.summarize());
}

/// 多个 trait 约束
fn notify_multiple(item: &(impl Summary + Display)) {
    println!("Multi-trait item: {}", item);
}

/// 使用 where 子句的复杂约束
fn notify_complex<T, U>(t: &T, u: &U) -> String
where
    T: Summary + Clone,
    U: Summary + Debug,
{
    format!("Complex notification: {} and {:?}", t.summarize(), u)
}

/// 返回实现了 trait 的类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    }
}

/// 条件性返回不同类型（需要使用 Box<dyn Trait>）
fn returns_summarizable_conditional(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(NewsArticle {
            headline: "Breaking News".to_string(),
            location: "World".to_string(),
            author: "Reporter".to_string(),
            content: "Something happened...".to_string(),
        })
    } else {
        Box::new(Tweet {
            username: "news_bot".to_string(),
            content: "Quick update".to_string(),
            reply: false,
            retweet: false,
        })
    }
}

fn trait_as_parameters_and_return_values() {
    println!("\n--- 3. Trait 作为参数和返回值 ---");

    let article = NewsArticle {
        headline: "Rust Trait 深度解析".to_string(),
        location: "技术社区".to_string(),
        author: "Rust 专家".to_string(),
        content: "深入理解 Rust 的 trait 系统...".to_string(),
    };

    let tweet = Tweet {
        username: "rust_learner".to_string(),
        content: "今天学习了 Rust trait，太强大了！".to_string(),
        reply: false,
        retweet: false,
    };

    // 使用 impl Trait 语法
    notify(&article);
    notify(&tweet);

    // 使用 trait bound 语法
    notify_bound(&article);
    notify_bound(&tweet);

    // 返回 trait 对象
    let returned_tweet = returns_summarizable();
    println!("返回的对象: {}", returned_tweet.summarize());

    // 条件性返回
    let conditional_news = returns_summarizable_conditional(true);
    let conditional_tweet = returns_summarizable_conditional(false);
    println!("条件返回 (news): {}", conditional_news.summarize());
    println!("条件返回 (tweet): {}", conditional_tweet.summarize());
}

// ==================== 4. Trait Bound 特征约束 ====================

/// 基础 trait bound
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/// 多个 trait bound
fn compare_and_display<T: PartialOrd + Display + Copy>(a: T, b: T) {
    if a > b {
        println!("{} is greater than {}", a, b);
    } else {
        println!("{} is less than or equal to {}", a, b);
    }
}

/// 使用 where 子句的复杂约束
fn complex_function<T, U, V>(t: T, u: U) -> V
where
    T: Display + Clone + Debug,
    U: Clone + Debug + PartialEq,
    V: From<T> + Default,
{
    println!("Processing: {} and {:?}", t, u);
    V::from(t)
}

/// 结构体的 trait bound
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

/// 条件性实现方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

/// 为满足条件的类型实现 trait
struct Wrapper<T>(T);

impl<T: Display> Display for Wrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Wrapped: {}", self.0)
    }
}

fn trait_bounds_demonstration() {
    println!("\n--- 4. Trait Bound 特征约束 ---");

    // 基础约束演示
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("最大的数字是 {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("最大的字符是 {}", result);

    // 多重约束演示
    compare_and_display(10, 20);
    compare_and_display('a', 'z');

    // 结构体约束演示
    let pair = Pair::new(10, 20);
    pair.cmp_display();

    let pair_str = Pair::new("hello", "world");
    pair_str.cmp_display();

    // 条件性实现演示
    let wrapped_int = Wrapper(42);
    println!("{}", wrapped_int);

    let wrapped_str = Wrapper("Hello, Rust!");
    println!("{}", wrapped_str);
}

// ==================== 5. 默认实现和关联类型 ====================

/// 带有关联类型的 trait
trait Iterator2 {
    type Item; // 关联类型

    fn next(&mut self) -> Option<Self::Item>;

    /// 带默认实现的方法
    fn count(self) -> usize
    where
        Self: Sized,
    {
        let mut count = 0;
        let mut iter = self;
        while iter.next().is_some() {
            count += 1;
        }
        count
    }
}

/// 自定义计数器
struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Counter {
        Counter { current: 0, max }
    }
}

impl Iterator2 for Counter {
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

/// 带有泛型参数的 trait
trait Add2<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

/// 点结构体
#[derive(Debug, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

/// 为 Point 实现 Add2 trait
impl Add2 for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// 为 Point 和 i32 实现 Add2 trait
impl Add2<i32> for Point {
    type Output = Point;

    fn add(self, scalar: i32) -> Point {
        Point {
            x: self.x + scalar,
            y: self.y + scalar,
        }
    }
}

/// 带有多个关联类型的复杂 trait
trait Graph {
    type Node;
    type Edge;

    fn nodes(&self) -> Vec<Self::Node>;
    fn edges(&self) -> Vec<Self::Edge>;

    /// 默认实现：计算节点数量
    fn node_count(&self) -> usize {
        self.nodes().len()
    }

    /// 默认实现：计算边数量
    fn edge_count(&self) -> usize {
        self.edges().len()
    }
}

/// 简单图实现
struct SimpleGraph {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>,
}

impl Graph for SimpleGraph {
    type Node = String;
    type Edge = (usize, usize);

    fn nodes(&self) -> Vec<Self::Node> {
        self.nodes.clone()
    }

    fn edges(&self) -> Vec<Self::Edge> {
        self.edges.clone()
    }
}

fn default_implementation_and_associated_types() {
    println!("\n--- 5. 默认实现和关联类型 ---");

    // 关联类型演示
    let mut counter = Counter::new(3);
    println!("计数器演示:");
    while let Some(value) = counter.next() {
        println!("计数: {}", value);
    }

    // 使用默认实现
    let counter2 = Counter::new(5);
    println!("计数器总数: {}", counter2.count());

    // 泛型参数演示
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1.add(p2);
    println!("点相加: {:?} + {:?} = {:?}", p1, p2, p3);

    let p4 = p1.add(5);
    println!("点加标量: {:?} + 5 = {:?}", p1, p4);

    // 复杂关联类型演示
    let graph = SimpleGraph {
        nodes: vec!["A".to_string(), "B".to_string(), "C".to_string()],
        edges: vec![(0, 1), (1, 2), (0, 2)],
    };

    println!("图节点数: {}", graph.node_count());
    println!("图边数: {}", graph.edge_count());
    println!("图节点: {:?}", graph.nodes());
    println!("图边: {:?}", graph.edges());
}

// ==================== 6. Derive 派生宏 ====================

/// 使用 derive 自动实现常用 trait
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

/// 自定义 Display 实现
impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}岁, {})", self.name, self.age, self.email)
    }
}

/// 枚举的 derive 演示
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Status {
    Active,
    Inactive,
    Pending,
}

/// 复杂结构体的 derive
#[derive(Debug, Clone, PartialEq)]
struct ComplexStruct {
    id: u64,
    name: String,
    status: Status,
    metadata: HashMap<String, String>,
}

/// 手动实现 PartialOrd 和 Ord
#[derive(Debug, Clone, PartialEq, Eq)]
struct Student {
    name: String,
    grade: u32,
}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Student {
    fn cmp(&self, other: &Self) -> Ordering {
        // 先按成绩排序，再按姓名排序
        self.grade
            .cmp(&other.grade)
            .then_with(|| self.name.cmp(&other.name))
    }
}

fn derive_traits_demonstration() {
    println!("\n--- 6. Derive 派生宏演示 ---");

    // Debug trait 演示
    let person1 = Person {
        name: "张三".to_string(),
        age: 25,
        email: "zhangsan@example.com".to_string(),
    };

    println!("Debug 输出: {:?}", person1);
    println!("Display 输出: {}", person1);

    // Clone trait 演示
    let person2 = person1.clone();
    println!("克隆对象: {:?}", person2);

    // PartialEq trait 演示
    println!("对象相等性: {}", person1 == person2);

    // Default trait 演示
    let default_person = Person::default();
    println!("默认对象: {:?}", default_person);

    // Hash trait 演示（用于 HashMap）
    let mut person_map = HashMap::new();
    person_map.insert(person1.clone(), "员工信息");
    println!("HashMap 查找: {:?}", person_map.get(&person1));

    // 枚举 derive 演示
    let status = Status::Active;
    println!("状态: {:?}", status);

    let status_copy = status; // Copy trait
    println!("复制状态: {:?}", status_copy);

    // 复杂结构体演示
    let mut metadata = HashMap::new();
    metadata.insert("department".to_string(), "IT".to_string());
    metadata.insert("location".to_string(), "北京".to_string());

    let complex = ComplexStruct {
        id: 1001,
        name: "复杂对象".to_string(),
        status: Status::Active,
        metadata,
    };

    println!("复杂对象: {:#?}", complex);

    // 自定义排序演示
    let mut students = vec![
        Student {
            name: "Alice".to_string(),
            grade: 85,
        },
        Student {
            name: "Bob".to_string(),
            grade: 92,
        },
        Student {
            name: "Charlie".to_string(),
            grade: 85,
        },
    ];

    students.sort();
    println!("排序后的学生: {:#?}", students);
}

// ==================== 7. Trait 对象和动态分发 ====================

/// 绘制 trait
trait Draw {
    fn draw(&self) -> String;

    /// 默认实现
    fn area(&self) -> f64 {
        0.0
    }
}

/// 按钮结构体
#[derive(Debug)]
struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) -> String {
        format!("绘制按钮: {} ({}x{})", self.label, self.width, self.height)
    }

    fn area(&self) -> f64 {
        (self.width * self.height) as f64
    }
}

/// 选择框结构体
#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) -> String {
        format!(
            "绘制选择框 ({}x{}) 选项: {:?}",
            self.width, self.height, self.options
        )
    }

    fn area(&self) -> f64 {
        (self.width * self.height) as f64
    }
}

/// 圆形结构体
#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Draw for Circle {
    fn draw(&self) -> String {
        format!("绘制圆形，半径: {}", self.radius)
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

/// 屏幕结构体，包含多种可绘制组件
struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn new() -> Self {
        Screen {
            components: Vec::new(),
        }
    }

    fn add_component(&mut self, component: Box<dyn Draw>) {
        self.components.push(component);
    }

    fn run(&self) {
        for component in self.components.iter() {
            println!("{}", component.draw());
            println!("面积: {:.2}", component.area());
        }
    }

    fn total_area(&self) -> f64 {
        self.components.iter().map(|c| c.area()).sum()
    }
}

/// 使用引用的 trait 对象
fn draw_component(drawable: &dyn Draw) {
    println!("组件绘制: {}", drawable.draw());
}

/// 返回 trait 对象
fn create_ui_component(component_type: &str) -> Box<dyn Draw> {
    match component_type {
        "button" => Box::new(Button {
            width: 100,
            height: 30,
            label: "确定".to_string(),
        }),
        "selectbox" => Box::new(SelectBox {
            width: 150,
            height: 25,
            options: vec!["选项1".to_string(), "选项2".to_string()],
        }),
        "circle" => Box::new(Circle { radius: 50.0 }),
        _ => Box::new(Button {
            width: 80,
            height: 25,
            label: "默认".to_string(),
        }),
    }
}

fn trait_objects_and_dynamic_dispatch() {
    println!("\n--- 7. Trait 对象和动态分发 ---");

    // 创建不同类型的组件
    let button = Button {
        width: 120,
        height: 40,
        label: "提交".to_string(),
    };

    let select_box = SelectBox {
        width: 200,
        height: 30,
        options: vec![
            "Rust".to_string(),
            "Python".to_string(),
            "JavaScript".to_string(),
        ],
    };

    let circle = Circle { radius: 25.0 };

    // 使用引用的 trait 对象
    draw_component(&button);
    draw_component(&select_box);
    draw_component(&circle);

    // 创建屏幕并添加组件
    let mut screen = Screen::new();
    screen.add_component(Box::new(button));
    screen.add_component(Box::new(select_box));
    screen.add_component(Box::new(circle));

    // 动态创建组件
    screen.add_component(create_ui_component("button"));
    screen.add_component(create_ui_component("selectbox"));
    screen.add_component(create_ui_component("circle"));

    println!("\n屏幕渲染:");
    screen.run();

    println!("\n总面积: {:.2}", screen.total_area());

    // 演示静态分发 vs 动态分发
    println!("\n--- 静态分发 vs 动态分发 ---");

    // 静态分发：编译时确定类型
    fn static_dispatch<T: Draw>(item: &T) {
        println!("静态分发: {}", item.draw());
    }

    // 动态分发：运行时确定类型
    fn dynamic_dispatch(item: &dyn Draw) {
        println!("动态分发: {}", item.draw());
    }

    let new_button = Button {
        width: 80,
        height: 25,
        label: "测试".to_string(),
    };

    static_dispatch(&new_button);
    dynamic_dispatch(&new_button);
}

// ==================== 8. 高级 Trait 特性 ====================

/// 关联常量
trait MathConstants {
    const PI: f64;
    const E: f64;

    fn circle_area(radius: f64) -> f64 {
        Self::PI * radius * radius
    }
}

struct StandardMath;

impl MathConstants for StandardMath {
    const PI: f64 = 3.14159265359;
    const E: f64 = 2.71828182846;
}

struct PreciseMath;

impl MathConstants for PreciseMath {
    const PI: f64 = std::f64::consts::PI;
    const E: f64 = std::f64::consts::E;
}

/// 高阶 trait：trait 作为 trait 的约束
trait Clone2: Sized {
    fn clone(&self) -> Self;
}

trait CloneAndDebug: Clone2 + Debug {
    fn clone_and_debug(&self) -> Self {
        let cloned = self.clone();
        println!("克隆对象: {:?}", cloned);
        cloned
    }
}

#[derive(Debug)]
struct MyStruct {
    value: i32,
}

impl Clone2 for MyStruct {
    fn clone(&self) -> Self {
        MyStruct { value: self.value }
    }
}

impl CloneAndDebug for MyStruct {}

/// 关联类型 vs 泛型参数
// 使用关联类型：每个实现只能有一个具体类型
trait Iterator3 {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// 使用泛型参数：可以为同一个类型实现多次
trait From2<T> {
    fn from(value: T) -> Self;
}

/// 同一个类型可以实现多个 From2<T>
struct MyNumber(i32);

impl From2<i32> for MyNumber {
    fn from(value: i32) -> Self {
        MyNumber(value)
    }
}

impl From2<f64> for MyNumber {
    fn from(value: f64) -> Self {
        MyNumber(value as i32)
    }
}

impl From2<String> for MyNumber {
    fn from(value: String) -> Self {
        MyNumber(value.parse().unwrap_or(0))
    }
}

/// 生命周期参数的 trait
trait Borrowable<'a> {
    type Borrowed: 'a;

    fn borrow(&'a self) -> Self::Borrowed;
}

struct Container<T> {
    value: T,
}

impl<'a, T: 'a> Borrowable<'a> for Container<T> {
    type Borrowed = &'a T;

    fn borrow(&'a self) -> Self::Borrowed {
        &self.value
    }
}

/// 高阶生命周期 trait bound (HRTB)
fn higher_ranked_trait_bound<F>(f: F)
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let result = f("hello");
    println!("HRTB 结果: {}", result);
}

/// 关联类型的复杂约束
trait ComplexTrait {
    type Input;
    type Output;
    type Error;

    fn process(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
}

struct StringProcessor;

impl ComplexTrait for StringProcessor {
    type Input = String;
    type Output = usize;
    type Error = &'static str;

    fn process(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        if input.is_empty() {
            Err("输入为空")
        } else {
            Ok(input.len())
        }
    }
}

fn advanced_trait_features() {
    println!("\n--- 8. 高级 Trait 特性 ---");

    // 关联常量演示
    println!("标准数学常量:");
    println!("PI = {}", StandardMath::PI);
    println!("E = {}", StandardMath::E);
    println!("圆面积 (r=5) = {}", StandardMath::circle_area(5.0));

    println!("\n精确数学常量:");
    println!("PI = {}", PreciseMath::PI);
    println!("E = {}", PreciseMath::E);
    println!("圆面积 (r=5) = {}", PreciseMath::circle_area(5.0));

    // 高阶 trait 演示
    let my_struct = MyStruct { value: 42 };
    let cloned = my_struct.clone_and_debug();
    println!("原始值: {:?}", my_struct);

    // 多重 From2 实现演示
    let num1 = <MyNumber as From2<i32>>::from(42i32);
    let num2 = <MyNumber as From2<f64>>::from(3.14f64);
    let num3 = <MyNumber as From2<String>>::from("123".to_string());

    println!("从不同类型转换: {}, {}, {}", num1.0, num2.0, num3.0);

    // 生命周期 trait 演示
    let container = Container {
        value: "Hello, Rust!",
    };
    let borrowed = container.borrow();
    println!("借用的值: {}", borrowed);

    // 高阶生命周期 trait bound
    higher_ranked_trait_bound(|s| s);

    // 复杂 trait 演示
    let processor = StringProcessor;
    match processor.process("Hello, World!".to_string()) {
        Ok(length) => println!("字符串长度: {}", length),
        Err(e) => println!("处理错误: {}", e),
    }

    match processor.process(String::new()) {
        Ok(length) => println!("字符串长度: {}", length),
        Err(e) => println!("处理错误: {}", e),
    }
}

// ==================== 9. 标准库常用 Trait ====================

/// 演示标准库中的重要 trait
fn standard_library_traits() {
    println!("\n--- 9. 标准库常用 Trait ---");

    // 1. Iterator trait
    println!("\n1. Iterator Trait:");
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("原数组: {:?}", numbers);
    println!("翻倍后: {:?}", doubled);

    let sum: i32 = numbers.iter().sum();
    println!("求和: {}", sum);

    // 2. From/Into trait
    println!("\n2. From/Into Trait:");
    let s = String::from("hello");
    let bytes: Vec<u8> = s.into();
    println!("字符串转字节: {:?}", bytes);

    let num: i32 = "42".parse().unwrap();
    println!("字符串解析为数字: {}", num);

    // 3. TryFrom/TryInto trait
    println!("\n3. TryFrom/TryInto Trait:");
    use std::convert::TryInto;

    let big_number: i64 = 1000;
    let small_number: Result<i32, _> = big_number.try_into();
    match small_number {
        Ok(n) => println!("转换成功: {}", n),
        Err(e) => println!("转换失败: {:?}", e),
    }

    // 4. Display 和 Debug trait
    println!("\n4. Display 和 Debug Trait:");
    #[derive(Debug)]
    struct Point3D {
        x: f64,
        y: f64,
        z: f64,
    }

    impl Display for Point3D {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {}, {})", self.x, self.y, self.z)
        }
    }

    let point = Point3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // 5. PartialEq 和 Eq trait
    println!("\n5. PartialEq 和 Eq Trait:");
    #[derive(Debug, PartialEq)]
    struct FloatPoint {
        x: f64,
        y: f64,
    }

    let p1 = FloatPoint { x: 1.0, y: 2.0 };
    let p2 = FloatPoint { x: 1.0, y: 2.0 };
    let p3 = FloatPoint { x: 1.1, y: 2.0 };

    println!("p1 == p2: {}", p1 == p2);
    println!("p1 == p3: {}", p1 == p3);

    // 6. Hash trait
    println!("\n6. Hash Trait:");
    use std::collections::HashSet;

    #[derive(Debug, Hash, PartialEq, Eq)]
    struct User {
        id: u32,
        name: String,
    }

    let mut users = HashSet::new();
    users.insert(User {
        id: 1,
        name: "Alice".to_string(),
    });
    users.insert(User {
        id: 2,
        name: "Bob".to_string(),
    });
    users.insert(User {
        id: 1,
        name: "Alice".to_string(),
    }); // 重复，不会插入

    println!("用户集合: {:?}", users);
    println!("用户数量: {}", users.len());

    // 7. Default trait
    println!("\n7. Default Trait:");
    #[derive(Debug, Default)]
    struct Config {
        debug: bool,
        max_connections: u32,
        timeout: u64,
    }

    let default_config = Config::default();
    let custom_config = Config {
        debug: true,
        max_connections: 100,
        ..Default::default()
    };

    println!("默认配置: {:?}", default_config);
    println!("自定义配置: {:?}", custom_config);

    // 8. Drop trait
    println!("\n8. Drop Trait:");
    struct Resource {
        name: String,
    }

    impl Drop for Resource {
        fn drop(&mut self) {
            println!("释放资源: {}", self.name);
        }
    }

    {
        let _resource = Resource {
            name: "数据库连接".to_string(),
        };
        println!("资源使用中...");
    } // resource 在这里被 drop

    println!("资源已释放");

    // 9. Send 和 Sync trait
    println!("\n9. Send 和 Sync Trait:");
    fn is_send<T: Send>(_: T) {
        println!("类型实现了 Send trait");
    }

    fn is_sync<T: Sync>(_: &T) {
        println!("类型实现了 Sync trait");
    }

    let data = vec![1, 2, 3, 4, 5];
    is_send(data.clone());
    is_sync(&data);

    // 10. Deref trait
    println!("\n10. Deref Trait:");
    use std::ops::Deref;

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    println!("x = {}", x);
    println!("*y = {}", *y); // 解引用

    let m = MyBox::new(String::from("Rust"));
    let len = m.len(); // 自动解引用
    println!("字符串长度: {}", len);
}

// ==================== 10. 实际应用场景 ====================

/// 1. 插件系统
trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn execute(&self, input: &str) -> String;

    fn info(&self) -> String {
        format!("{} v{}", self.name(), self.version())
    }
}

struct LoggerPlugin;
struct ValidatorPlugin;
struct FormatterPlugin;

impl Plugin for LoggerPlugin {
    fn name(&self) -> &str {
        "Logger"
    }
    fn version(&self) -> &str {
        "1.0.0"
    }

    fn execute(&self, input: &str) -> String {
        let log_entry = format!("[LOG] {}", input);
        println!("{}", log_entry);
        log_entry
    }
}

impl Plugin for ValidatorPlugin {
    fn name(&self) -> &str {
        "Validator"
    }
    fn version(&self) -> &str {
        "2.1.0"
    }

    fn execute(&self, input: &str) -> String {
        if input.len() > 0 && input.len() < 100 {
            format!("✓ 验证通过: {}", input)
        } else {
            format!("✗ 验证失败: 输入长度不符合要求")
        }
    }
}

impl Plugin for FormatterPlugin {
    fn name(&self) -> &str {
        "Formatter"
    }
    fn version(&self) -> &str {
        "1.5.2"
    }

    fn execute(&self, input: &str) -> String {
        input.to_uppercase()
    }
}

struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    fn new() -> Self {
        PluginManager {
            plugins: Vec::new(),
        }
    }

    fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        println!("注册插件: {}", plugin.info());
        self.plugins.push(plugin);
    }

    fn execute_all(&self, input: &str) -> Vec<String> {
        self.plugins
            .iter()
            .map(|plugin| plugin.execute(input))
            .collect()
    }

    fn list_plugins(&self) {
        println!("已注册的插件:");
        for plugin in &self.plugins {
            println!("  - {}", plugin.info());
        }
    }
}

/// 2. 序列化系统
trait Serializable {
    fn serialize(&self) -> String;
    fn deserialize(data: &str) -> Result<Self, String>
    where
        Self: Sized;
}

#[derive(Debug, Clone)]
struct User2 {
    id: u32,
    name: String,
    email: String,
}

impl Serializable for User2 {
    fn serialize(&self) -> String {
        format!("{}|{}|{}", self.id, self.name, self.email)
    }

    fn deserialize(data: &str) -> Result<Self, String> {
        let parts: Vec<&str> = data.split('|').collect();
        if parts.len() != 3 {
            return Err("Invalid format".to_string());
        }

        let id = parts[0].parse().map_err(|_| "Invalid id".to_string())?;
        let name = parts[1].to_string();
        let email = parts[2].to_string();

        Ok(User2 { id, name, email })
    }
}

#[derive(Debug, Clone)]
struct Product {
    id: u32,
    name: String,
    price: f64,
}

impl Serializable for Product {
    fn serialize(&self) -> String {
        format!("{}:{}:{}", self.id, self.name, self.price)
    }

    fn deserialize(data: &str) -> Result<Self, String> {
        let parts: Vec<&str> = data.split(':').collect();
        if parts.len() != 3 {
            return Err("Invalid format".to_string());
        }

        let id = parts[0].parse().map_err(|_| "Invalid id".to_string())?;
        let name = parts[1].to_string();
        let price = parts[2].parse().map_err(|_| "Invalid price".to_string())?;

        Ok(Product { id, name, price })
    }
}

/// 3. 策略模式
trait SortStrategy<T> {
    fn sort(&self, data: &mut [T]);
    fn name(&self) -> &str;
}

struct BubbleSort;
struct QuickSort;
struct MergeSort;

impl<T: PartialOrd> SortStrategy<T> for BubbleSort {
    fn sort(&self, data: &mut [T]) {
        let len = data.len();
        for i in 0..len {
            for j in 0..len - 1 - i {
                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                }
            }
        }
    }

    fn name(&self) -> &str {
        "冒泡排序"
    }
}

impl<T: PartialOrd> SortStrategy<T> for QuickSort {
    fn sort(&self, data: &mut [T]) {
        if data.len() <= 1 {
            return;
        }
        // 简化的快速排序实现
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    fn name(&self) -> &str {
        "快速排序"
    }
}

impl<T: PartialOrd> SortStrategy<T> for MergeSort {
    fn sort(&self, data: &mut [T]) {
        // 简化的归并排序实现
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    fn name(&self) -> &str {
        "归并排序"
    }
}

struct Sorter<T> {
    strategy: Box<dyn SortStrategy<T>>,
}

impl<T> Sorter<T> {
    fn new(strategy: Box<dyn SortStrategy<T>>) -> Self {
        Sorter { strategy }
    }

    fn set_strategy(&mut self, strategy: Box<dyn SortStrategy<T>>) {
        self.strategy = strategy;
    }

    fn sort(&self, data: &mut [T]) {
        println!("使用 {} 进行排序", self.strategy.name());
        self.strategy.sort(data);
    }
}

/// 4. 观察者模式
trait Observer {
    fn update(&self, message: &str);
    fn id(&self) -> &str;
}

trait Subject {
    fn attach(&mut self, observer: Box<dyn Observer>);
    fn detach(&mut self, observer_id: &str);
    fn notify(&self, message: &str);
}

struct EmailObserver {
    id: String,
    email: String,
}

impl EmailObserver {
    fn new(id: String, email: String) -> Self {
        EmailObserver { id, email }
    }
}

impl Observer for EmailObserver {
    fn update(&self, message: &str) {
        println!("📧 发送邮件到 {}: {}", self.email, message);
    }

    fn id(&self) -> &str {
        &self.id
    }
}

struct SMSObserver {
    id: String,
    phone: String,
}

impl SMSObserver {
    fn new(id: String, phone: String) -> Self {
        SMSObserver { id, phone }
    }
}

impl Observer for SMSObserver {
    fn update(&self, message: &str) {
        println!("📱 发送短信到 {}: {}", self.phone, message);
    }

    fn id(&self) -> &str {
        &self.id
    }
}

struct NewsPublisher {
    observers: Vec<Box<dyn Observer>>,
}

impl NewsPublisher {
    fn new() -> Self {
        NewsPublisher {
            observers: Vec::new(),
        }
    }

    fn publish_news(&self, news: &str) {
        println!("📰 发布新闻: {}", news);
        self.notify(news);
    }
}

impl Subject for NewsPublisher {
    fn attach(&mut self, observer: Box<dyn Observer>) {
        println!("➕ 添加观察者: {}", observer.id());
        self.observers.push(observer);
    }

    fn detach(&mut self, observer_id: &str) {
        self.observers.retain(|obs| obs.id() != observer_id);
        println!("➖ 移除观察者: {}", observer_id);
    }

    fn notify(&self, message: &str) {
        for observer in &self.observers {
            observer.update(message);
        }
    }
}

fn practical_applications() {
    println!("\n--- 10. 实际应用场景 ---");

    // 1. 插件系统演示
    println!("\n=== 插件系统演示 ===");
    let mut plugin_manager = PluginManager::new();

    plugin_manager.register_plugin(Box::new(LoggerPlugin));
    plugin_manager.register_plugin(Box::new(ValidatorPlugin));
    plugin_manager.register_plugin(Box::new(FormatterPlugin));

    plugin_manager.list_plugins();

    let input = "Hello, Rust Traits!";
    let results = plugin_manager.execute_all(input);

    println!("\n处理结果:");
    for (i, result) in results.iter().enumerate() {
        println!("  {}. {}", i + 1, result);
    }

    // 2. 序列化系统演示
    println!("\n=== 序列化系统演示 ===");
    let user = User2 {
        id: 1001,
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
    };

    let product = Product {
        id: 2001,
        name: "Rust 编程书籍".to_string(),
        price: 89.99,
    };

    // 序列化
    let user_data = user.serialize();
    let product_data = product.serialize();

    println!("用户序列化: {}", user_data);
    println!("产品序列化: {}", product_data);

    // 反序列化
    match User2::deserialize(&user_data) {
        Ok(deserialized_user) => println!("用户反序列化: {:?}", deserialized_user),
        Err(e) => println!("用户反序列化失败: {}", e),
    }

    match Product::deserialize(&product_data) {
        Ok(deserialized_product) => println!("产品反序列化: {:?}", deserialized_product),
        Err(e) => println!("产品反序列化失败: {}", e),
    }

    // 3. 策略模式演示
    println!("\n=== 策略模式演示 ===");
    let mut data1 = vec![64, 34, 25, 12, 22, 11, 90];
    let mut data2 = data1.clone();
    let mut data3 = data1.clone();

    println!("原始数据: {:?}", data1);

    let mut sorter = Sorter::new(Box::new(BubbleSort));
    sorter.sort(&mut data1);
    println!("冒泡排序结果: {:?}", data1);

    sorter.set_strategy(Box::new(QuickSort));
    sorter.sort(&mut data2);
    println!("快速排序结果: {:?}", data2);

    sorter.set_strategy(Box::new(MergeSort));
    sorter.sort(&mut data3);
    println!("归并排序结果: {:?}", data3);

    // 4. 观察者模式演示
    println!("\n=== 观察者模式演示 ===");
    let mut publisher = NewsPublisher::new();

    let email_observer = EmailObserver::new("email_1".to_string(), "user@example.com".to_string());

    let sms_observer = SMSObserver::new("sms_1".to_string(), "138-0000-0000".to_string());

    publisher.attach(Box::new(email_observer));
    publisher.attach(Box::new(sms_observer));

    publisher.publish_news("Rust 1.75 正式发布！");

    // 移除观察者
    publisher.detach("email_1");

    publisher.publish_news("Rust 异步编程新特性介绍");
}

// ==================== 总结和最佳实践 ====================

/// Trait 设计原则和最佳实践
fn trait_best_practices() {
    println!("\n=== Trait 设计原则和最佳实践 ===");

    println!("\n1. 单一职责原则:");
    println!("   - 每个 trait 应该只负责一个明确的功能");
    println!("   - 避免创建过于庞大的 trait");

    println!("\n2. 接口隔离原则:");
    println!("   - 客户端不应该依赖它不需要的方法");
    println!("   - 将大的 trait 拆分为多个小的 trait");

    println!("\n3. 依赖倒置原则:");
    println!("   - 高层模块不应该依赖低层模块，都应该依赖抽象");
    println!("   - 使用 trait 作为抽象层");

    println!("\n4. 性能考虑:");
    println!("   - 静态分发 vs 动态分发的选择");
    println!("   - 避免不必要的 trait 对象");
    println!("   - 合理使用泛型和 trait bound");

    println!("\n5. 可扩展性:");
    println!("   - 提供默认实现以保持向后兼容");
    println!("   - 使用关联类型而非泛型参数（当适用时）");
    println!("   - 考虑未来的扩展需求");
}

/// 常见陷阱和解决方案
fn common_pitfalls_and_solutions() {
    println!("\n=== 常见陷阱和解决方案 ===");

    println!("\n1. 孤儿规则 (Orphan Rule):");
    println!("   问题: 不能为外部类型实现外部 trait");
    println!("   解决: 使用 newtype 模式包装外部类型");

    // 示例：为 Vec<i32> 实现自定义 trait
    struct MyVec(Vec<i32>);

    trait Summable {
        fn sum(&self) -> i32;
    }

    impl Summable for MyVec {
        fn sum(&self) -> i32 {
            self.0.iter().sum()
        }
    }

    let my_vec = MyVec(vec![1, 2, 3, 4, 5]);
    println!("   示例: MyVec 求和 = {}", my_vec.sum());

    println!("\n2. Trait 对象安全性:");
    println!("   问题: 不是所有 trait 都可以作为 trait 对象");
    println!("   解决: 确保 trait 是对象安全的");

    // 对象安全的 trait
    trait ObjectSafe {
        fn method(&self) -> String;
    }

    // 不是对象安全的 trait（有泛型方法）
    trait NotObjectSafe {
        fn generic_method<T>(&self, value: T) -> T;
    }

    println!("   - 避免泛型方法");
    println!("   - 避免返回 Self 的方法");
    println!("   - 避免关联函数（除了接收 &self 的方法）");

    println!("\n3. 生命周期复杂性:");
    println!("   问题: trait 中的生命周期参数可能很复杂");
    println!("   解决: 使用生命周期省略规则，必要时显式标注");

    println!("\n4. 性能开销:");
    println!("   问题: trait 对象有运行时开销");
    println!("   解决: 在性能关键路径上使用静态分发");
}

/// Trait 与其他语言特性的比较
fn trait_comparison_with_other_languages() {
    println!("\n=== Trait 与其他语言特性的比较 ===");

    println!("\n1. Rust Trait vs Java Interface:");
    println!("   相似点:");
    println!("   - 都定义了类型必须实现的方法");
    println!("   - 都支持多重实现");
    println!("   ");
    println!("   不同点:");
    println!("   - Rust trait 支持默认实现");
    println!("   - Rust trait 支持关联类型和常量");
    println!("   - Rust trait 可以为已存在的类型实现");
    println!("   - Rust trait 有更强的类型安全保证");

    println!("\n2. Rust Trait vs C++ Concepts:");
    println!("   相似点:");
    println!("   - 都用于约束泛型参数");
    println!("   - 都在编译时检查");
    println!("   ");
    println!("   不同点:");
    println!("   - Rust trait 需要显式实现");
    println!("   - Rust trait 支持动态分发");
    println!("   - Rust trait 有更丰富的功能");

    println!("\n3. Rust Trait vs Haskell Type Classes:");
    println!("   相似点:");
    println!("   - 都支持 ad-hoc 多态");
    println!("   - 都有类似的语法结构");
    println!("   ");
    println!("   不同点:");
    println!("   - Rust 更注重零成本抽象");
    println!("   - Rust 有所有权系统的约束");
}

/// 未来发展趋势
fn future_developments() {
    println!("\n=== Trait 系统的未来发展 ===");

    println!("\n1. 异步 Trait:");
    println!("   - async fn 在 trait 中的支持正在改进");
    println!("   - 异步 trait 对象的支持");

    println!("\n2. 高阶类型 (Higher-Kinded Types):");
    println!("   - 可能在未来版本中支持");
    println!("   - 将使 trait 系统更加强大");

    println!("\n3. 特化 (Specialization):");
    println!("   - 允许为特定类型提供更优化的实现");
    println!("   - 目前仍在实验阶段");

    println!("\n4. 常量泛型的扩展:");
    println!("   - 更灵活的常量泛型支持");
    println!("   - 与 trait 系统的更好集成");
}

/// 性能分析和优化
fn performance_analysis() {
    println!("\n=== 性能分析和优化 ===");

    println!("\n1. 静态分发 (Static Dispatch):");
    println!("   优点: 零运行时开销，编译器可以内联优化");
    println!("   缺点: 代码膨胀，编译时间增加");

    println!("\n2. 动态分发 (Dynamic Dispatch):");
    println!("   优点: 代码大小小，运行时灵活性");
    println!("   缺点: 虚函数调用开销，无法内联");

    println!("\n3. 选择建议:");
    println!("   - 性能关键路径: 使用静态分发");
    println!("   - 需要运行时多态: 使用动态分发");
    println!("   - 库设计: 提供两种选择");

    // 性能测试示例
    use std::time::Instant;

    trait Compute {
        fn compute(&self, x: i32) -> i32;
    }

    struct FastCompute;
    impl Compute for FastCompute {
        fn compute(&self, x: i32) -> i32 {
            x * x + x + 1
        }
    }

    // 静态分发测试
    fn static_dispatch_test<T: Compute>(computer: &T, iterations: usize) -> u128 {
        let start = Instant::now();
        let mut sum = 0;
        for i in 0..iterations {
            sum += computer.compute(i as i32);
        }
        let duration = start.elapsed().as_nanos();
        println!("   静态分发结果: sum = {}, 耗时: {} ns", sum, duration);
        duration
    }

    // 动态分发测试
    fn dynamic_dispatch_test(computer: &dyn Compute, iterations: usize) -> u128 {
        let start = Instant::now();
        let mut sum = 0;
        for i in 0..iterations {
            sum += computer.compute(i as i32);
        }
        let duration = start.elapsed().as_nanos();
        println!("   动态分发结果: sum = {}, 耗时: {} ns", sum, duration);
        duration
    }

    let computer = FastCompute;
    let iterations = 1000;

    println!("\n4. 性能测试 ({} 次迭代):", iterations);
    let static_time = static_dispatch_test(&computer, iterations);
    let dynamic_time = dynamic_dispatch_test(&computer, iterations);

    if static_time > 0 {
        let ratio = dynamic_time as f64 / static_time as f64;
        println!("   动态分发 / 静态分发 = {:.2}x", ratio);
    }
}

/// 实际项目中的应用建议
fn practical_project_advice() {
    println!("\n=== 实际项目中的应用建议 ===");

    println!("\n1. API 设计:");
    println!("   - 优先使用 impl Trait 作为参数");
    println!("   - 返回值使用 impl Trait（单一类型）或 Box<dyn Trait>（多类型）");
    println!("   - 合理使用 trait bound 约束泛型");

    println!("\n2. 错误处理:");
    println!("   - 实现 std::error::Error trait");
    println!("   - 使用 thiserror 或 anyhow 简化错误处理");

    println!("\n3. 序列化/反序列化:");
    println!("   - 使用 serde 的 Serialize/Deserialize trait");
    println!("   - 自定义序列化逻辑时实现相应 trait");

    println!("\n4. 异步编程:");
    println!("   - 实现 Future trait（通常通过 async/await）");
    println!("   - 使用 Stream trait 处理异步数据流");

    println!("\n5. 测试:");
    println!("   - 使用 trait 创建可测试的抽象");
    println!("   - 通过 trait 实现 mock 对象");

    println!("\n6. 插件架构:");
    println!("   - 定义插件 trait 接口");
    println!("   - 使用动态加载和 trait 对象");
}

// 在 main 函数中添加新的演示
fn main() {
    println!("=== Rust Trait 特征全面深入分析 ===");

    // 1. 基础概念演示
    basic_trait_concepts();

    // 2. trait 定义和实现
    trait_definition_and_implementation();

    // 3. trait 作为参数和返回值
    trait_as_parameters_and_return_values();

    // 4. trait bound 特征约束
    trait_bounds_demonstration();

    // 5. 默认实现和关联类型
    default_implementation_and_associated_types();

    // 6. derive 派生宏
    derive_traits_demonstration();

    // 7. trait 对象和动态分发
    trait_objects_and_dynamic_dispatch();

    // 8. 高级 trait 特性
    advanced_trait_features();

    // 9. 标准库常用 trait
    standard_library_traits();

    // 10. 实际应用场景
    practical_applications();

    // 11. 最佳实践和设计原则
    trait_best_practices();

    // 12. 常见陷阱和解决方案
    common_pitfalls_and_solutions();

    // 13. 与其他语言特性的比较
    trait_comparison_with_other_languages();

    // 14. 未来发展趋势
    future_developments();

    // 15. 性能分析和优化
    performance_analysis();

    // 16. 实际项目应用建议
    practical_project_advice();

    println!("\n=== Trait 分析完成 ===");
    println!("\n📚 总结:");
    println!("本分析涵盖了 Rust Trait 系统的所有重要方面：");
    println!("✅ 基础概念和语法");
    println!("✅ 高级特性和技巧");
    println!("✅ 实际应用场景");
    println!("✅ 性能考虑和优化");
    println!("✅ 最佳实践和设计模式");
    println!("✅ 与其他语言的比较");
    println!("✅ 未来发展方向");
    println!("\n🎯 Trait 是 Rust 最强大的特性之一，掌握它对于编写高质量的 Rust 代码至关重要！");
}
