//! # Rust 泛型 (Generics) 全面深入分析
//!
//! 基于 https://course.rs/basic/trait/generic.html
//!
//! 本文档全面分析 Rust 中的泛型概念，包括：
//! 1. 泛型的基本概念和优势
//! 2. 泛型函数
//! 3. 泛型结构体
//! 4. 泛型枚举
//! 5. 泛型方法
//! 6. 泛型特征约束
//! 7. 多重约束和 where 子句
//! 8. 关联类型
//! 9. 生命周期参数
//! 10. const 泛型
//! 11. 泛型的性能和单态化
//! 12. 高级泛型技巧
//! 13. 实际应用场景

use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::ops::{Add, Mul, Sub};

fn main() {
    println!("=== Rust 泛型 (Generics) 全面深入分析 ===");

    // 1. 泛型函数演示
    generic_function_demo();

    // 2. 泛型结构体演示
    generic_struct_demo();

    // 3. 泛型枚举演示
    generic_enum_demo();

    // 4. 泛型方法演示
    generic_method_demo();

    // 5. 特征约束演示
    trait_bound_demo();

    // 6. 多重约束和 where 子句演示
    multiple_bounds_demo();

    // 7. 关联类型演示
    associated_type_demo();

    // 8. 生命周期参数演示
    lifetime_parameter_demo();

    // 9. const 泛型演示
    const_generic_demo();

    // 10. 高级泛型技巧演示
    advanced_generic_demo();

    // 11. 实际应用场景演示
    real_world_applications_demo();

    println!("\n=== 泛型分析完成 ===");
}

// 1. 泛型函数演示
fn generic_function_demo() {
    println!("\n--- 1. 泛型函数演示 ---");

    // 基本泛型函数
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    // 交换两个值的泛型函数
    fn swap<T>(a: &mut T, b: &mut T) {
        std::mem::swap(a, b);
    }

    // 比较两个值的泛型函数
    fn compare<T: PartialEq>(a: &T, b: &T) -> bool {
        a == b
    }

    // 打印任意类型的泛型函数
    fn print_type<T: Debug>(value: T) {
        println!("值: {:?}, 类型: {}", value, std::any::type_name::<T>());
    }

    // 多个泛型参数的函数
    fn combine<T, U>(first: T, second: U) -> (T, U) {
        (first, second)
    }

    // 使用泛型函数
    let numbers = vec![34, 50, 25, 100, 65];
    println!("最大数字: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("最大字符: {}", largest(&chars));

    let mut x = 5;
    let mut y = 10;
    println!("交换前: x = {}, y = {}", x, y);
    swap(&mut x, &mut y);
    println!("交换后: x = {}, y = {}", x, y);

    println!("5 == 5: {}", compare(&5, &5));
    println!("5 == 10: {}", compare(&5, &10));

    print_type(42);
    print_type("Hello");
    print_type(vec![1, 2, 3]);

    let combined = combine("Hello", 42);
    println!("组合结果: {:?}", combined);
}

// 2. 泛型结构体演示
fn generic_struct_demo() {
    println!("\n--- 2. 泛型结构体演示 ---");

    // 基本泛型结构体
    #[derive(Debug, Clone)]
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn new(x: T, y: T) -> Point<T> {
            Point { x, y }
        }

        fn x(&self) -> &T {
            &self.x
        }

        fn y(&self) -> &T {
            &self.y
        }
    }

    // 为特定类型实现方法
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    // 多个泛型参数的结构体
    #[derive(Debug)]
    struct Pair<T, U> {
        first: T,
        second: U,
    }

    impl<T, U> Pair<T, U> {
        fn new(first: T, second: U) -> Pair<T, U> {
            Pair { first, second }
        }

        fn first(&self) -> &T {
            &self.first
        }

        fn second(&self) -> &U {
            &self.second
        }

        // 混合泛型方法
        fn mixup<V, W>(self, other: Pair<V, W>) -> Pair<T, W> {
            Pair {
                first: self.first,
                second: other.second,
            }
        }
    }

    // 容器结构体
    #[derive(Debug)]
    struct Container<T> {
        items: Vec<T>,
    }

    impl<T> Container<T> {
        fn new() -> Container<T> {
            Container { items: Vec::new() }
        }

        fn add(&mut self, item: T) {
            self.items.push(item);
        }

        fn len(&self) -> usize {
            self.items.len()
        }

        fn get(&self, index: usize) -> Option<&T> {
            self.items.get(index)
        }
    }

    // 为实现了特定特征的类型实现方法
    impl<T: Display> Container<T> {
        fn print_all(&self) {
            for item in &self.items {
                println!("  - {}", item);
            }
        }
    }

    // 使用泛型结构体
    let int_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);

    println!("整数点: {:?}", int_point);
    println!("浮点数点: {:?}", float_point);
    println!(
        "浮点数点到原点距离: {:.2}",
        float_point.distance_from_origin()
    );

    let pair1 = Pair::new("Hello", 42);
    let pair2 = Pair::new(true, 3.14);

    println!("配对1: {:?}", pair1);
    println!("配对2: {:?}", pair2);

    let mixed = pair1.mixup(pair2);
    println!("混合后: {:?}", mixed);

    let mut string_container = Container::new();
    string_container.add("Apple".to_string());
    string_container.add("Banana".to_string());
    string_container.add("Cherry".to_string());

    println!("字符串容器长度: {}", string_container.len());
    println!("字符串容器内容:");
    string_container.print_all();
}

// 3. 泛型枚举演示
fn generic_enum_demo() {
    println!("\n--- 3. 泛型枚举演示 ---");

    // 自定义 Result 类型
    #[derive(Debug)]
    enum MyResult<T, E> {
        Ok(T),
        Err(E),
    }

    impl<T, E> MyResult<T, E> {
        fn is_ok(&self) -> bool {
            matches!(self, MyResult::Ok(_))
        }

        fn is_err(&self) -> bool {
            matches!(self, MyResult::Err(_))
        }

        fn unwrap(self) -> T
        where
            E: Debug,
        {
            match self {
                MyResult::Ok(val) => val,
                MyResult::Err(err) => panic!("调用 unwrap 时出错: {:?}", err),
            }
        }

        fn unwrap_or(self, default: T) -> T {
            match self {
                MyResult::Ok(val) => val,
                MyResult::Err(_) => default,
            }
        }

        fn map<U, F>(self, f: F) -> MyResult<U, E>
        where
            F: FnOnce(T) -> U,
        {
            match self {
                MyResult::Ok(val) => MyResult::Ok(f(val)),
                MyResult::Err(err) => MyResult::Err(err),
            }
        }
    }

    // 自定义 Option 类型
    #[derive(Debug, Clone)]
    enum MyOption<T> {
        Some(T),
        None,
    }

    impl<T> MyOption<T> {
        fn is_some(&self) -> bool {
            matches!(self, MyOption::Some(_))
        }

        fn is_none(&self) -> bool {
            matches!(self, MyOption::None)
        }

        fn unwrap(self) -> T {
            match self {
                MyOption::Some(val) => val,
                MyOption::None => panic!("在 None 值上调用 unwrap"),
            }
        }

        fn unwrap_or(self, default: T) -> T {
            match self {
                MyOption::Some(val) => val,
                MyOption::None => default,
            }
        }

        fn map<U, F>(self, f: F) -> MyOption<U>
        where
            F: FnOnce(T) -> U,
        {
            match self {
                MyOption::Some(val) => MyOption::Some(f(val)),
                MyOption::None => MyOption::None,
            }
        }

        fn and_then<U, F>(self, f: F) -> MyOption<U>
        where
            F: FnOnce(T) -> MyOption<U>,
        {
            match self {
                MyOption::Some(val) => f(val),
                MyOption::None => MyOption::None,
            }
        }
    }

    // 树结构枚举
    #[derive(Debug)]
    enum Tree<T> {
        Leaf(T),
        Node {
            left: Box<Tree<T>>,
            right: Box<Tree<T>>,
        },
    }

    impl<T> Tree<T> {
        fn leaf(value: T) -> Tree<T> {
            Tree::Leaf(value)
        }

        fn node(left: Tree<T>, right: Tree<T>) -> Tree<T> {
            Tree::Node {
                left: Box::new(left),
                right: Box::new(right),
            }
        }
    }

    impl<T: Clone> Tree<T> {
        fn count_leaves(&self) -> usize {
            match self {
                Tree::Leaf(_) => 1,
                Tree::Node { left, right } => left.count_leaves() + right.count_leaves(),
            }
        }

        fn collect_leaves(&self) -> Vec<T> {
            match self {
                Tree::Leaf(value) => vec![value.clone()],
                Tree::Node { left, right } => {
                    let mut leaves = left.collect_leaves();
                    leaves.extend(right.collect_leaves());
                    leaves
                }
            }
        }
    }

    // 使用泛型枚举
    let success: MyResult<i32, &str> = MyResult::Ok(42);
    let failure: MyResult<i32, &str> = MyResult::Err("出错了");

    println!("成功结果: {:?}, 是否成功: {}", success, success.is_ok());
    println!("失败结果: {:?}, 是否失败: {}", failure, failure.is_err());

    let mapped: MyResult<i32, String> = MyResult::Ok(10).map(|x| x * 2);
    println!("映射结果: {:?}", mapped);

    let some_value = MyOption::Some("Hello");
    let none_value: MyOption<&str> = MyOption::None;

    println!("Some 值: {:?}", some_value);
    println!("None 值: {:?}", none_value);

    let chained = MyOption::Some(5).map(|x| x * 2).and_then(|x| {
        if x > 5 {
            MyOption::Some(x)
        } else {
            MyOption::None
        }
    });
    println!("链式操作结果: {:?}", chained);

    // 创建树结构
    let tree = Tree::node(
        Tree::node(Tree::leaf(1), Tree::leaf(2)),
        Tree::node(Tree::leaf(3), Tree::leaf(4)),
    );

    println!("树结构: {:?}", tree);
    println!("叶子节点数量: {}", tree.count_leaves());
    println!("所有叶子值: {:?}", tree.collect_leaves());
}

// 4. 泛型方法演示
fn generic_method_demo() {
    println!("\n--- 4. 泛型方法演示 ---");

    struct Calculator;

    impl Calculator {
        // 泛型方法 - 加法
        fn add<T>(&self, a: T, b: T) -> T
        where
            T: Add<Output = T>,
        {
            a + b
        }

        // 泛型方法 - 比较
        fn max<T>(&self, a: T, b: T) -> T
        where
            T: PartialOrd,
        {
            if a > b { a } else { b }
        }

        // 泛型方法 - 转换
        fn convert<T, U>(&self, value: T) -> U
        where
            T: Into<U>,
        {
            value.into()
        }

        // 泛型方法 - 处理集合
        fn process_collection<T, F, U>(&self, items: Vec<T>, f: F) -> Vec<U>
        where
            F: Fn(T) -> U,
        {
            items.into_iter().map(f).collect()
        }
    }

    // 带有泛型方法的结构体
    #[derive(Debug)]
    struct DataProcessor<T> {
        data: Vec<T>,
    }

    impl<T> DataProcessor<T> {
        fn new() -> DataProcessor<T> {
            DataProcessor { data: Vec::new() }
        }

        fn add(&mut self, item: T) {
            self.data.push(item);
        }

        // 泛型方法 - 映射到新类型
        fn map<U, F>(&self, f: F) -> DataProcessor<U>
        where
            F: Fn(&T) -> U,
        {
            let mapped_data = self.data.iter().map(f).collect();
            DataProcessor { data: mapped_data }
        }

        // 泛型方法 - 过滤
        fn filter<F>(&self, predicate: F) -> DataProcessor<T>
        where
            F: Fn(&T) -> bool,
            T: Clone,
        {
            let filtered_data = self
                .data
                .iter()
                .filter(|&x| predicate(x))
                .cloned()
                .collect();
            DataProcessor {
                data: filtered_data,
            }
        }

        // 泛型方法 - 折叠
        fn fold<U, F>(&self, init: U, f: F) -> U
        where
            F: Fn(U, &T) -> U,
        {
            self.data.iter().fold(init, f)
        }
    }

    impl<T: Display> DataProcessor<T> {
        fn print_all(&self) {
            for item in &self.data {
                println!("  数据项: {}", item);
            }
        }
    }

    // 使用泛型方法
    let calc = Calculator;

    println!("5 + 3 = {}", calc.add(5, 3));
    println!("2.5 + 1.5 = {}", calc.add(2.5, 1.5));
    println!("max(10, 20) = {}", calc.max(10, 20));

    let converted: f64 = calc.convert(42i32);
    println!("转换 42i32 到 f64: {}", converted);

    let numbers = vec![1, 2, 3, 4, 5];
    let doubled = calc.process_collection(numbers, |x| x * 2);
    println!("翻倍结果: {:?}", doubled);

    let mut processor = DataProcessor::new();
    processor.add(1);
    processor.add(2);
    processor.add(3);
    processor.add(4);
    processor.add(5);

    println!("原始数据:");
    processor.print_all();

    let squared = processor.map(|&x| x * x);
    println!("平方后的数据:");
    squared.print_all();

    let even_only = processor.filter(|&x| x % 2 == 0);
    println!("偶数数据:");
    even_only.print_all();

    let sum = processor.fold(0, |acc, &x| acc + x);
    println!("数据总和: {}", sum);
}

// 5. 特征约束演示
fn trait_bound_demo() {
    println!("\n--- 5. 特征约束演示 ---");

    // 基本特征约束
    fn print_and_compare<T>(a: &T, b: &T)
    where
        T: Debug + PartialEq,
    {
        println!("a = {:?}, b = {:?}", a, b);
        println!("a == b: {}", a == b);
    }

    // 数值运算特征约束
    fn calculate<T>(a: T, b: T) -> T
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Debug,
    {
        let result = (a + b) * (a - b);
        println!("({:?} + {:?}) * ({:?} - {:?}) = {:?}", a, b, a, b, result);
        result
    }

    // 集合操作特征约束
    fn find_max<T>(slice: &[T]) -> Option<&T>
    where
        T: Ord,
    {
        slice.iter().max()
    }

    // 克隆特征约束
    fn duplicate<T>(value: T) -> (T, T)
    where
        T: Clone,
    {
        (value.clone(), value)
    }

    // 默认值特征约束
    fn create_with_default<T>() -> T
    where
        T: Default,
    {
        T::default()
    }

    // 迭代器特征约束
    fn process_iterator<I, T>(iter: I) -> Vec<T>
    where
        I: Iterator<Item = T>,
        T: Clone + Debug,
    {
        let items: Vec<T> = iter.collect();
        println!("处理的项目: {:?}", items);
        items
    }

    // 自定义特征
    trait Summarizable {
        fn summary(&self) -> String;
    }

    #[derive(Debug)]
    struct Article {
        title: String,
        content: String,
    }

    impl Summarizable for Article {
        fn summary(&self) -> String {
            format!("文章: {} ({}字符)", self.title, self.content.len())
        }
    }

    #[derive(Debug)]
    struct Tweet {
        username: String,
        content: String,
    }

    impl Summarizable for Tweet {
        fn summary(&self) -> String {
            format!("@{}: {}", self.username, self.content)
        }
    }

    // 使用自定义特征约束
    fn notify<T>(item: &T)
    where
        T: Summarizable + Debug,
    {
        println!("通知: {}", item.summary());
        println!("详细信息: {:?}", item);
    }

    // 使用特征约束
    print_and_compare(&5, &10);
    print_and_compare(&"hello", &"world");

    calculate(10, 3);
    calculate(5.5, 2.5);

    let numbers = [3, 7, 1, 9, 2];
    if let Some(max) = find_max(&numbers) {
        println!("最大值: {}", max);
    }

    let (original, copy) = duplicate("Hello".to_string());
    println!("原始: {}, 复制: {}", original, copy);

    let default_vec: Vec<i32> = create_with_default();
    let default_string: String = create_with_default();
    println!(
        "默认向量: {:?}, 默认字符串: {:?}",
        default_vec, default_string
    );

    let range_items = process_iterator(1..=5);
    println!("范围项目: {:?}", range_items);

    let article = Article {
        title: "Rust 泛型".to_string(),
        content: "这是一篇关于 Rust 泛型的文章...".to_string(),
    };

    let tweet = Tweet {
        username: "rustacean".to_string(),
        content: "学习 Rust 泛型真有趣！".to_string(),
    };

    notify(&article);
    notify(&tweet);
}

// 6. 多重约束和 where 子句演示
fn multiple_bounds_demo() {
    println!("\n--- 6. 多重约束和 where 子句演示 ---");

    // 复杂的多重约束
    fn complex_function<T, U, V>(t: T, u: U, v: V) -> String
    where
        T: Display + Debug + Clone,
        U: Debug + PartialEq<V>,
        V: Debug + Copy,
    {
        let t_clone = t.clone();
        let comparison = u == v;

        format!(
            "T: {} (调试: {:?}), U == V: {} (U: {:?}, V: {:?}), T克隆: {:?}",
            t, t, comparison, u, v, t_clone
        )
    }

    // 条件实现
    struct Wrapper<T> {
        value: T,
    }

    impl<T> Wrapper<T> {
        fn new(value: T) -> Wrapper<T> {
            Wrapper { value }
        }
    }

    // 只为实现了 Display 的类型实现方法
    impl<T: Display> Wrapper<T> {
        fn print(&self) {
            println!("包装的值: {}", self.value);
        }
    }

    // 只为实现了 Debug 的类型实现方法
    impl<T: Debug> Wrapper<T> {
        fn debug_print(&self) {
            println!("调试包装的值: {:?}", self.value);
        }
    }

    // 只为实现了 Clone 的类型实现方法
    impl<T: Clone> Wrapper<T> {
        fn duplicate(&self) -> Wrapper<T> {
            Wrapper {
                value: self.value.clone(),
            }
        }
    }

    // 复合约束
    impl<T> Wrapper<T>
    where
        T: Display + Debug + Clone + PartialEq,
    {
        fn full_analysis(&self, other: &Wrapper<T>) -> String {
            format!(
                "值1: {} ({:?}), 值2: {} ({:?}), 相等: {}",
                self.value,
                self.value,
                other.value,
                other.value,
                self.value == other.value
            )
        }
    }

    // 泛型函数与复杂约束
    fn advanced_processing<T, F, R>(items: Vec<T>, processor: F) -> Vec<R>
    where
        T: Debug + Clone,
        F: Fn(T) -> R,
        R: Debug,
    {
        println!("处理项目: {:?}", items);
        let results: Vec<R> = items.into_iter().map(processor).collect();
        println!("处理结果: {:?}", results);
        results
    }

    // 使用多重约束
    let result = complex_function("Hello".to_string(), 42, 42);
    println!("{}", result);

    let wrapper1 = Wrapper::new("World");
    wrapper1.print();
    wrapper1.debug_print();

    let wrapper2 = wrapper1.duplicate();
    println!("分析结果: {}", wrapper1.full_analysis(&wrapper2));

    let numbers = vec![1, 2, 3, 4, 5];
    advanced_processing(numbers, |x| x * x);

    let strings = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    advanced_processing(strings, |s| s.len());
}

// 7. 关联类型演示
fn associated_type_demo() {
    println!("\n--- 7. 关联类型演示 ---");

    // 定义带有关联类型的特征
    trait Iterator2 {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;

        // 默认实现使用关联类型
        fn collect_all(mut self) -> Vec<Self::Item>
        where
            Self: Sized,
        {
            let mut items = Vec::new();
            while let Some(item) = self.next() {
                items.push(item);
            }
            items
        }
    }

    // 数字范围迭代器
    struct Range {
        current: usize,
        end: usize,
    }

    impl Range {
        fn new(start: usize, end: usize) -> Range {
            Range {
                current: start,
                end,
            }
        }
    }

    impl Iterator2 for Range {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end {
                let current = self.current;
                self.current += 1;
                Some(current)
            } else {
                None
            }
        }
    }

    // 字符串分割迭代器
    struct StringSplitter {
        text: String,
        delimiter: char,
        position: usize,
    }

    impl StringSplitter {
        fn new(text: String, delimiter: char) -> StringSplitter {
            StringSplitter {
                text,
                delimiter,
                position: 0,
            }
        }
    }

    impl Iterator2 for StringSplitter {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            if self.position >= self.text.len() {
                return None;
            }

            let start = self.position;
            let remaining = &self.text[start..];

            if let Some(delimiter_pos) = remaining.find(self.delimiter) {
                let end = start + delimiter_pos;
                self.position = end + 1;
                Some(self.text[start..end].to_string())
            } else {
                self.position = self.text.len();
                Some(remaining.to_string())
            }
        }
    }

    // 转换特征
    trait Convert {
        type Output;

        fn convert(self) -> Self::Output;
    }

    // 为不同类型实现转换
    impl Convert for i32 {
        type Output = String;

        fn convert(self) -> Self::Output {
            self.to_string()
        }
    }

    impl Convert for String {
        type Output = usize;

        fn convert(self) -> Self::Output {
            self.len()
        }
    }

    // 使用关联类型的泛型函数
    fn process_convertible<T>(value: T) -> T::Output
    where
        T: Convert,
    {
        value.convert()
    }

    // 集合特征
    trait Collection {
        type Item;

        fn add(&mut self, item: Self::Item);
        fn len(&self) -> usize;
        fn get(&self, index: usize) -> Option<&Self::Item>;
    }

    // 简单向量实现
    struct SimpleVec<T> {
        items: Vec<T>,
    }

    impl<T> SimpleVec<T> {
        fn new() -> SimpleVec<T> {
            SimpleVec { items: Vec::new() }
        }
    }

    impl<T> Collection for SimpleVec<T> {
        type Item = T;

        fn add(&mut self, item: Self::Item) {
            self.items.push(item);
        }

        fn len(&self) -> usize {
            self.items.len()
        }

        fn get(&self, index: usize) -> Option<&Self::Item> {
            self.items.get(index)
        }
    }

    // 使用关联类型
    let mut range = Range::new(1, 5);
    println!("范围迭代:");
    while let Some(num) = range.next() {
        println!("  {}", num);
    }

    let range2 = Range::new(10, 15);
    let collected = range2.collect_all();
    println!("收集的范围: {:?}", collected);

    let mut splitter = StringSplitter::new("hello,world,rust".to_string(), ',');
    println!("字符串分割:");
    while let Some(part) = splitter.next() {
        println!("  {}", part);
    }

    let converted_int = process_convertible(42);
    let converted_string = process_convertible("Hello".to_string());
    println!(
        "转换结果: {} -> {}, {} -> {}",
        42, converted_int, "Hello", converted_string
    );

    let mut collection = SimpleVec::new();
    collection.add("Apple");
    collection.add("Banana");
    collection.add("Cherry");

    println!("集合长度: {}", collection.len());
    if let Some(item) = collection.get(1) {
        println!("索引1的项目: {}", item);
    }
}

// 8. 生命周期参数演示
fn lifetime_parameter_demo() {
    println!("\n--- 8. 生命周期参数演示 ---");

    // 带生命周期的泛型结构体
    #[derive(Debug)]
    struct Holder<'a, T> {
        value: &'a T,
    }

    impl<'a, T> Holder<'a, T> {
        fn new(value: &'a T) -> Holder<'a, T> {
            Holder { value }
        }

        fn get(&self) -> &T {
            self.value
        }
    }

    impl<'a, T: Display> Holder<'a, T> {
        fn print(&self) {
            println!("持有的值: {}", self.value);
        }
    }

    // 多个生命周期参数
    #[derive(Debug)]
    struct DoubleRef<'a, 'b, T, U> {
        first: &'a T,
        second: &'b U,
    }

    impl<'a, 'b, T, U> DoubleRef<'a, 'b, T, U> {
        fn new(first: &'a T, second: &'b U) -> DoubleRef<'a, 'b, T, U> {
            DoubleRef { first, second }
        }

        fn get_first(&self) -> &T {
            self.first
        }

        fn get_second(&self) -> &U {
            self.second
        }
    }

    // 生命周期约束的泛型函数
    fn longest<'a, T>(x: &'a T, y: &'a T) -> &'a T
    where
        T: PartialOrd,
    {
        if x > y { x } else { y }
    }

    // 复杂生命周期约束
    fn complex_lifetime<'a, 'b, T>(x: &'a T, y: &'b T) -> &'a T
    where
        'b: 'a, // 'b 必须比 'a 活得更久
        T: Clone + Debug,
    {
        println!("比较 {:?} 和 {:?}", x, y);
        x
    }

    // 带有生命周期的特征
    trait Borrowable<'a> {
        type Borrowed;

        fn borrow(&'a self) -> Self::Borrowed;
    }

    struct Container<T> {
        items: Vec<T>,
    }

    impl<T> Container<T> {
        fn new() -> Container<T> {
            Container { items: Vec::new() }
        }

        fn add(&mut self, item: T) {
            self.items.push(item);
        }
    }

    impl<'a, T: 'a> Borrowable<'a> for Container<T> {
        type Borrowed = &'a [T];

        fn borrow(&'a self) -> Self::Borrowed {
            &self.items
        }
    }

    // 使用生命周期参数
    let value = 42;
    let holder = Holder::new(&value);
    println!("持有者: {:?}", holder);
    holder.print();

    let first = "Hello";
    let second = 123;
    let double_ref = DoubleRef::new(&first, &second);
    println!("双重引用: {:?}", double_ref);
    println!(
        "第一个: {}, 第二个: {}",
        double_ref.get_first(),
        double_ref.get_second()
    );

    let x = 10;
    let y = 20;
    let max = longest(&x, &y);
    println!("最大值: {}", max);

    let a = "short";
    let b = "longer string";
    let result = complex_lifetime(&a, &b);
    println!("复杂生命周期结果: {}", result);

    let mut container = Container::new();
    container.add(1);
    container.add(2);
    container.add(3);

    let borrowed = container.borrow();
    println!("借用的切片: {:?}", borrowed);
}

// 9. const 泛型演示
fn const_generic_demo() {
    println!("\n--- 9. const 泛型演示 ---");

    // 固定大小数组
    #[derive(Debug, Clone)]
    struct FixedArray<T, const N: usize> {
        data: [T; N],
    }

    impl<T, const N: usize> FixedArray<T, N>
    where
        T: Default + Copy,
    {
        fn new() -> FixedArray<T, N> {
            FixedArray {
                data: [T::default(); N],
            }
        }

        fn set(&mut self, index: usize, value: T) -> Result<(), &'static str> {
            if index < N {
                self.data[index] = value;
                Ok(())
            } else {
                Err("索引超出范围")
            }
        }

        fn get(&self, index: usize) -> Option<&T> {
            self.data.get(index)
        }

        fn len(&self) -> usize {
            N
        }

        fn iter(&self) -> std::slice::Iter<'_, T> {
            self.data.iter()
        }
    }

    impl<T: Display, const N: usize> FixedArray<T, N> {
        fn print_all(&self) {
            print!("[");
            for (i, item) in self.data.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{}", item);
            }
            println!("]")
        }
    }

    // 矩阵结构
    #[derive(Debug)]
    struct Matrix<T, const ROWS: usize, const COLS: usize> {
        data: [[T; COLS]; ROWS],
    }

    impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS>
    where
        T: Default + Copy,
    {
        fn new() -> Matrix<T, ROWS, COLS> {
            Matrix {
                data: [[T::default(); COLS]; ROWS],
            }
        }

        fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), &'static str> {
            if row < ROWS && col < COLS {
                self.data[row][col] = value;
                Ok(())
            } else {
                Err("索引超出范围")
            }
        }

        fn get(&self, row: usize, col: usize) -> Option<&T> {
            if row < ROWS && col < COLS {
                Some(&self.data[row][col])
            } else {
                None
            }
        }

        fn dimensions(&self) -> (usize, usize) {
            (ROWS, COLS)
        }
    }

    impl<T: Display, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
        fn print(&self) {
            for row in &self.data {
                print!("[");
                for (i, item) in row.iter().enumerate() {
                    if i > 0 {
                        print!(", ");
                    }
                    print!("{:3}", item);
                }
                println!("]")
            }
        }
    }

    // 向量运算
    impl<const N: usize> FixedArray<f64, N> {
        fn dot_product(&self, other: &FixedArray<f64, N>) -> f64 {
            self.data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a * b)
                .sum()
        }

        fn magnitude(&self) -> f64 {
            self.data.iter().map(|x| x * x).sum::<f64>().sqrt()
        }
    }

    // 编译时计算
    const fn factorial(n: usize) -> usize {
        match n {
            0 | 1 => 1,
            _ => n * factorial(n - 1),
        }
    }

    // 简化的阶乘数组，避免const泛型复杂操作
    struct FactorialArray<const N: usize> {
        data: Vec<usize>,
    }

    impl<const N: usize> FactorialArray<N> {
        fn new() -> FactorialArray<N> {
            let size = factorial(N);
            FactorialArray {
                data: vec![0; size],
            }
        }

        fn size(&self) -> usize {
            self.data.len()
        }
    }

    // 使用 const 泛型
    let mut arr5: FixedArray<i32, 5> = FixedArray::new();
    arr5.set(0, 10).unwrap();
    arr5.set(1, 20).unwrap();
    arr5.set(2, 30).unwrap();

    println!("固定数组 (长度 {}): {:?}", arr5.len(), arr5);
    arr5.print_all();

    let mut arr3: FixedArray<f64, 3> = FixedArray::new();
    arr3.set(0, 1.0).unwrap();
    arr3.set(1, 2.0).unwrap();
    arr3.set(2, 3.0).unwrap();

    let mut arr3_2: FixedArray<f64, 3> = FixedArray::new();
    arr3_2.set(0, 4.0).unwrap();
    arr3_2.set(1, 5.0).unwrap();
    arr3_2.set(2, 6.0).unwrap();

    println!("向量1: {:?}", arr3);
    println!("向量2: {:?}", arr3_2);
    println!("点积: {}", arr3.dot_product(&arr3_2));
    println!("向量1模长: {:.2}", arr3.magnitude());

    let mut matrix: Matrix<i32, 3, 3> = Matrix::new();
    matrix.set(0, 0, 1).unwrap();
    matrix.set(0, 1, 2).unwrap();
    matrix.set(0, 2, 3).unwrap();
    matrix.set(1, 0, 4).unwrap();
    matrix.set(1, 1, 5).unwrap();
    matrix.set(1, 2, 6).unwrap();
    matrix.set(2, 0, 7).unwrap();
    matrix.set(2, 1, 8).unwrap();
    matrix.set(2, 2, 9).unwrap();

    println!("矩阵 {:?}:", matrix.dimensions());
    matrix.print();

    let fact_arr: FactorialArray<4> = FactorialArray::new();
    println!("阶乘数组大小 (4! = {}): {}", factorial(4), fact_arr.size());
}

// 10. 高级泛型技巧演示
fn advanced_generic_demo() {
    println!("\n--- 10. 高级泛型技巧演示 ---");

    // PhantomData 使用
    struct PhantomContainer<T> {
        _phantom: PhantomData<T>,
        data: Vec<u8>,
    }

    impl<T> PhantomContainer<T> {
        fn new() -> PhantomContainer<T> {
            PhantomContainer {
                _phantom: PhantomData,
                data: Vec::new(),
            }
        }

        fn add_bytes(&mut self, bytes: &[u8]) {
            self.data.extend_from_slice(bytes);
        }

        fn type_name(&self) -> &'static str {
            std::any::type_name::<T>()
        }
    }

    // 类型状态模式
    struct Locked;
    struct Unlocked;

    struct Vault<State> {
        contents: String,
        _state: PhantomData<State>,
    }

    impl Vault<Locked> {
        fn new(contents: String) -> Vault<Locked> {
            Vault {
                contents,
                _state: PhantomData,
            }
        }

        fn unlock(self, password: &str) -> Result<Vault<Unlocked>, Vault<Locked>> {
            if password == "secret" {
                Ok(Vault {
                    contents: self.contents,
                    _state: PhantomData,
                })
            } else {
                Err(self)
            }
        }
    }

    impl Vault<Unlocked> {
        fn read(&self) -> &str {
            &self.contents
        }

        fn write(&mut self, new_contents: String) {
            self.contents = new_contents;
        }

        fn lock(self) -> Vault<Locked> {
            Vault {
                contents: self.contents,
                _state: PhantomData,
            }
        }
    }

    // 高阶类型
    trait Functor<A> {
        type Wrapped<B>;

        fn map<B, F>(self, f: F) -> Self::Wrapped<B>
        where
            F: FnOnce(A) -> B;
    }

    impl<A> Functor<A> for Option<A> {
        type Wrapped<B> = Option<B>;

        fn map<B, F>(self, f: F) -> Self::Wrapped<B>
        where
            F: FnOnce(A) -> B,
        {
            self.map(f)
        }
    }

    impl<A> Functor<A> for Vec<A> {
        type Wrapped<B> = Vec<B>;

        fn map<B, F>(self, f: F) -> Self::Wrapped<B>
        where
            F: FnOnce(A) -> B,
        {
            vec![f(self.into_iter().next().unwrap())]
        }
    }

    // 类型级编程
    trait TypeLevel {}

    struct True;
    struct False;

    impl TypeLevel for True {}
    impl TypeLevel for False {}

    trait And<Rhs> {
        type Output: TypeLevel;
    }

    impl And<True> for True {
        type Output = True;
    }

    impl And<False> for True {
        type Output = False;
    }

    impl And<True> for False {
        type Output = False;
    }

    impl And<False> for False {
        type Output = False;
    }

    // 条件编译泛型
    trait ConditionalImpl<T> {
        fn conditional_method(&self);
    }

    struct ConditionalStruct<T> {
        value: T,
    }

    impl<T> ConditionalStruct<T> {
        fn new(value: T) -> ConditionalStruct<T> {
            ConditionalStruct { value }
        }
    }

    impl<T: Display> ConditionalImpl<T> for ConditionalStruct<T> {
        fn conditional_method(&self) {
            println!("可显示类型: {}", self.value);
        }
    }

    impl<T: Debug> ConditionalStruct<T> {
        fn debug_method(&self) {
            println!("可调试类型: {:?}", self.value);
        }
    }

    // 使用高级泛型技巧
    let mut phantom_int: PhantomContainer<i32> = PhantomContainer::new();
    phantom_int.add_bytes(&[1, 2, 3, 4]);
    println!("幻影容器类型: {}", phantom_int.type_name());

    let mut phantom_string: PhantomContainer<String> = PhantomContainer::new();
    phantom_string.add_bytes(&[65, 66, 67]);
    println!("幻影容器类型: {}", phantom_string.type_name());

    // 类型状态模式
    let locked_vault = Vault::new("秘密信息".to_string());
    println!("保险库已创建（锁定状态）");

    match locked_vault.unlock("wrong") {
        Ok(_) => println!("解锁成功"),
        Err(vault) => {
            println!("解锁失败，重试...");
            match vault.unlock("secret") {
                Ok(mut unlocked) => {
                    println!("解锁成功！内容: {}", unlocked.read());
                    unlocked.write("新的秘密信息".to_string());
                    println!("更新后内容: {}", unlocked.read());
                    let _locked_again = unlocked.lock();
                    println!("保险库已重新锁定");
                }
                Err(_) => println!("解锁失败"),
            }
        }
    }

    // 高阶类型使用
    let option_result = Some(42).map(|x| x * 2);
    println!("Option 映射结果: {:?}", option_result);

    let vec_result = vec![1, 2, 3].map(|x| x.to_string());
    println!("Vec 映射结果: {:?}", vec_result);

    // 条件实现
    let display_struct = ConditionalStruct::new("Hello");
    display_struct.conditional_method();
    display_struct.debug_method();

    let debug_struct = ConditionalStruct::new(vec![1, 2, 3]);
    debug_struct.debug_method();
    // debug_struct.conditional_method(); // 这行会编译错误，因为 Vec<i32> 没有实现 Display
}

// 11. 实际应用场景演示
fn real_world_applications_demo() {
    println!("\n--- 11. 实际应用场景演示 ---");

    // 场景1: 数据库连接池
    trait DatabaseConnection {
        fn execute(&self, query: &str) -> String;
        fn close(&self);
    }

    struct MySQLConnection {
        id: usize,
    }

    impl DatabaseConnection for MySQLConnection {
        fn execute(&self, query: &str) -> String {
            format!("MySQL[{}] 执行: {}", self.id, query)
        }

        fn close(&self) {
            println!("MySQL[{}] 连接已关闭", self.id);
        }
    }

    struct PostgreSQLConnection {
        id: usize,
    }

    impl DatabaseConnection for PostgreSQLConnection {
        fn execute(&self, query: &str) -> String {
            format!("PostgreSQL[{}] 执行: {}", self.id, query)
        }

        fn close(&self) {
            println!("PostgreSQL[{}] 连接已关闭", self.id);
        }
    }

    struct ConnectionPool<T: DatabaseConnection> {
        connections: Vec<T>,
        available: Vec<bool>,
    }

    impl<T: DatabaseConnection> ConnectionPool<T> {
        fn new(connections: Vec<T>) -> ConnectionPool<T> {
            let available = vec![true; connections.len()];
            ConnectionPool {
                connections,
                available,
            }
        }

        fn get_connection(&mut self) -> Option<&T> {
            for (i, &available) in self.available.iter().enumerate() {
                if available {
                    self.available[i] = false;
                    return Some(&self.connections[i]);
                }
            }
            None
        }

        fn release_connection(&mut self, index: usize) {
            if index < self.available.len() {
                self.available[index] = true;
            }
        }

        fn available_count(&self) -> usize {
            self.available.iter().filter(|&&x| x).count()
        }
    }

    // 场景2: 序列化框架
    trait Serializable {
        fn serialize(&self) -> String;
    }

    trait Deserializable: Sized {
        fn deserialize(data: &str) -> Result<Self, String>;
    }

    #[derive(Debug, Clone)]
    struct User {
        id: u32,
        name: String,
        email: String,
    }

    impl Serializable for User {
        fn serialize(&self) -> String {
            format!("{}|{}|{}", self.id, self.name, self.email)
        }
    }

    impl Deserializable for User {
        fn deserialize(data: &str) -> Result<Self, String> {
            let parts: Vec<&str> = data.split('|').collect();
            if parts.len() != 3 {
                return Err("无效的用户数据格式".to_string());
            }

            let id = parts[0]
                .parse::<u32>()
                .map_err(|_| "无效的ID".to_string())?;
            let name = parts[1].to_string();
            let email = parts[2].to_string();

            Ok(User { id, name, email })
        }
    }

    // 泛型序列化器
    struct Serializer<T> {
        _phantom: PhantomData<T>,
    }

    impl<T> Serializer<T>
    where
        T: Serializable + Deserializable,
    {
        fn new() -> Serializer<T> {
            Serializer {
                _phantom: PhantomData,
            }
        }

        fn serialize_batch(&self, items: &[T]) -> Vec<String> {
            items.iter().map(|item| item.serialize()).collect()
        }

        fn deserialize_batch(&self, data: &[String]) -> Result<Vec<T>, String> {
            data.iter()
                .map(|s| T::deserialize(s))
                .collect::<Result<Vec<T>, String>>()
        }
    }

    // 场景3: 缓存系统
    trait Cache<K, V> {
        fn get(&self, key: &K) -> Option<&V>;
        fn set(&mut self, key: K, value: V);
        fn remove(&mut self, key: &K) -> Option<V>;
        fn clear(&mut self);
        fn len(&self) -> usize;
    }

    struct LRUCache<K, V> {
        capacity: usize,
        data: HashMap<K, V>,
        access_order: Vec<K>,
    }

    impl<K, V> LRUCache<K, V>
    where
        K: Clone + Eq + std::hash::Hash,
    {
        fn new(capacity: usize) -> LRUCache<K, V> {
            LRUCache {
                capacity,
                data: HashMap::new(),
                access_order: Vec::new(),
            }
        }

        fn update_access(&mut self, key: &K) {
            if let Some(pos) = self.access_order.iter().position(|k| k == key) {
                let key = self.access_order.remove(pos);
                self.access_order.push(key);
            }
        }

        fn evict_if_needed(&mut self) {
            while self.data.len() >= self.capacity && !self.access_order.is_empty() {
                let oldest_key = self.access_order.remove(0);
                self.data.remove(&oldest_key);
            }
        }
    }

    impl<K, V> Cache<K, V> for LRUCache<K, V>
    where
        K: Clone + Eq + std::hash::Hash,
    {
        fn get(&self, key: &K) -> Option<&V> {
            self.data.get(key)
        }

        fn set(&mut self, key: K, value: V) {
            self.evict_if_needed();

            if self.data.contains_key(&key) {
                self.update_access(&key);
            } else {
                self.access_order.push(key.clone());
            }

            self.data.insert(key, value);
        }

        fn remove(&mut self, key: &K) -> Option<V> {
            if let Some(pos) = self.access_order.iter().position(|k| k == key) {
                self.access_order.remove(pos);
            }
            self.data.remove(key)
        }

        fn clear(&mut self) {
            self.data.clear();
            self.access_order.clear();
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }

    // 场景4: 事件系统
    trait Event: Debug + Clone {}

    #[derive(Debug, Clone)]
    struct UserLoginEvent {
        user_id: u32,
        timestamp: u64,
    }

    impl Event for UserLoginEvent {}

    #[derive(Debug, Clone)]
    struct OrderCreatedEvent {
        order_id: u32,
        user_id: u32,
        amount: f64,
    }

    impl Event for OrderCreatedEvent {}

    trait EventHandler<E: Event> {
        fn handle(&self, event: &E);
    }

    struct EventBus<E: Event> {
        handlers: Vec<Box<dyn EventHandler<E>>>,
        _phantom: PhantomData<E>,
    }

    impl<E: Event> EventBus<E> {
        fn new() -> EventBus<E> {
            EventBus {
                handlers: Vec::new(),
                _phantom: PhantomData,
            }
        }

        fn subscribe(&mut self, handler: Box<dyn EventHandler<E>>) {
            self.handlers.push(handler);
        }

        fn publish(&self, event: &E) {
            for handler in &self.handlers {
                handler.handle(event);
            }
        }
    }

    struct LoginHandler;

    impl EventHandler<UserLoginEvent> for LoginHandler {
        fn handle(&self, event: &UserLoginEvent) {
            println!(
                "用户登录处理器: 用户 {} 在 {} 登录",
                event.user_id, event.timestamp
            );
        }
    }

    struct OrderHandler;

    impl EventHandler<OrderCreatedEvent> for OrderHandler {
        fn handle(&self, event: &OrderCreatedEvent) {
            println!(
                "订单处理器: 用户 {} 创建了金额为 {} 的订单 {}",
                event.user_id, event.amount, event.order_id
            );
        }
    }

    // 使用实际应用场景

    // 数据库连接池演示
    let mysql_connections = vec![
        MySQLConnection { id: 1 },
        MySQLConnection { id: 2 },
        MySQLConnection { id: 3 },
    ];

    let mut mysql_pool = ConnectionPool::new(mysql_connections);
    println!("MySQL 连接池可用连接数: {}", mysql_pool.available_count());

    if let Some(conn) = mysql_pool.get_connection() {
        let result = conn.execute("SELECT * FROM users");
        println!("{}", result);
    }

    println!("使用连接后可用连接数: {}", mysql_pool.available_count());
    mysql_pool.release_connection(0);
    println!("释放连接后可用连接数: {}", mysql_pool.available_count());

    // 序列化演示
    let user = User {
        id: 1,
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
    };

    let serializer = Serializer::<User>::new();
    let serialized = user.serialize();
    println!("序列化用户: {}", serialized);

    match User::deserialize(&serialized) {
        Ok(deserialized_user) => {
            println!("反序列化用户: {:?}", deserialized_user);
        }
        Err(e) => println!("反序列化失败: {}", e),
    }

    let users = vec![
        user.clone(),
        User {
            id: 2,
            name: "李四".to_string(),
            email: "lisi@example.com".to_string(),
        },
    ];

    let batch_serialized = serializer.serialize_batch(&users);
    println!("批量序列化: {:?}", batch_serialized);

    // 缓存系统演示
    let mut cache: LRUCache<String, i32> = LRUCache::new(3);

    cache.set("key1".to_string(), 100);
    cache.set("key2".to_string(), 200);
    cache.set("key3".to_string(), 300);

    println!("缓存大小: {}", cache.len());

    if let Some(value) = cache.get(&"key1".to_string()) {
        println!("获取 key1: {}", value);
    }

    cache.set("key4".to_string(), 400); // 这会触发 LRU 淘汰
    println!("添加 key4 后缓存大小: {}", cache.len());

    // 事件系统演示
    let mut login_bus = EventBus::<UserLoginEvent>::new();
    login_bus.subscribe(Box::new(LoginHandler));

    let login_event = UserLoginEvent {
        user_id: 123,
        timestamp: 1640995200,
    };

    println!("发布登录事件:");
    login_bus.publish(&login_event);

    let mut order_bus = EventBus::<OrderCreatedEvent>::new();
    order_bus.subscribe(Box::new(OrderHandler));

    let order_event = OrderCreatedEvent {
        order_id: 456,
        user_id: 123,
        amount: 99.99,
    };

    println!("发布订单事件:");
    order_bus.publish(&order_event);

    println!("\n=== 泛型性能和单态化说明 ===");
    println!("Rust 的泛型是零成本抽象：");
    println!("1. 编译时单态化 - 为每个具体类型生成专门的代码");
    println!("2. 没有运行时开销 - 泛型代码性能等同于手写的具体类型代码");
    println!("3. 代码复用 - 一份泛型代码可以处理多种类型");
    println!("4. 类型安全 - 编译时检查确保类型正确性");

    println!("\n=== 泛型最佳实践 ===");
    println!("1. 合理使用特征约束，避免过度约束");
    println!("2. 优先使用关联类型而不是泛型参数（当只有一种可能的类型时）");
    println!("3. 使用 where 子句提高代码可读性");
    println!("4. 考虑使用 PhantomData 进行类型状态管理");
    println!("5. 合理使用 const 泛型处理编译时常量");
    println!("6. 避免过度泛型化，保持代码简洁");
}
