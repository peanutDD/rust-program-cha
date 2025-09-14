//! Rust Box<T> 智能指针全面教程
//!
//! 本教程深入分析 Box<T> 智能指针的所有相关知识点，包括：
//! - 堆栈内存管理
//! - 智能指针特性
//! - 递归类型处理
//! - 内存布局分析
//! - 实际应用场景
//! - 性能优化策略

use std::alloc::{Layout, alloc, dealloc};
use std::fmt::Debug;
use std::mem;
use std::ptr;

fn main() {
    println!("{}\n", "=".repeat(60));
    println!("🦀 Rust Box<T> 智能指针全面教程");
    println!("{}\n", "=".repeat(60));

    // 1. 基础概念和堆栈内存
    basic_concepts_and_memory();

    // 2. Box<T> 基础使用
    basic_box_usage();

    // 3. 所有权和移动语义
    ownership_and_move_semantics();

    // 4. 递归类型和数据结构
    recursive_types_and_structures();

    // 5. 内存管理和 Drop trait
    memory_management_and_drop();

    // 6. 解引用和智能指针特性
    deref_and_smart_pointer_traits();

    // 7. 实际应用场景
    practical_use_cases();

    // 8. 高级模式和技巧
    advanced_patterns_and_techniques();

    // 9. 性能分析和优化
    performance_analysis_and_optimization();

    println!("\n=== 教程总结 ===");
    println!("本教程全面覆盖了 Box<T> 智能指针的各个方面：");
    println!("• 堆栈内存管理和所有权机制");
    println!("• 递归类型和复杂数据结构");
    println!("• 智能指针特性和内存安全");
    println!("• 实际应用场景和最佳实践");
    println!("• 性能优化和内存布局分析");
}

/// 1. 基础概念和堆栈内存
fn basic_concepts_and_memory() {
    println!("1. 基础概念和堆栈内存");
    println!("{}", "=".repeat(40));

    // 栈内存分配
    let stack_value = 42i32;
    println!("栈上的值: {}, 地址: {:p}", stack_value, &stack_value);
    println!("栈上值的大小: {} bytes", mem::size_of_val(&stack_value));

    // 堆内存分配
    let heap_value = Box::new(42i32);
    println!(
        "堆上的值: {}, Box地址: {:p}, 堆数据地址: {:p}",
        heap_value,
        &heap_value,
        heap_value.as_ref()
    );
    println!("Box本身大小: {} bytes", mem::size_of_val(&heap_value));
    println!(
        "堆上数据大小: {} bytes",
        mem::size_of_val(heap_value.as_ref())
    );

    // 大型数据结构的对比
    let large_array_stack = [0u8; 1024]; // 栈上分配
    let large_array_heap = Box::new([0u8; 1024]); // 堆上分配

    println!("\n大型数据结构内存分析:");
    println!(
        "栈上数组地址: {:p}, 大小: {} bytes",
        &large_array_stack,
        mem::size_of_val(&large_array_stack)
    );
    println!(
        "堆上数组Box地址: {:p}, 堆数据地址: {:p}",
        &large_array_heap,
        large_array_heap.as_ref()
    );
    println!(
        "Box指针大小: {} bytes, 堆数据大小: {} bytes",
        mem::size_of_val(&large_array_heap),
        mem::size_of_val(large_array_heap.as_ref())
    );

    // 内存布局分析
    memory_layout_analysis();

    println!();
}

/// 内存布局分析
fn memory_layout_analysis() {
    println!("\n内存布局详细分析:");

    // 不同类型的 Box 大小分析
    let box_i32 = Box::new(42i32);
    let box_i64 = Box::new(42i64);
    let box_string = Box::new(String::from("Hello"));
    let box_vec = Box::new(vec![1, 2, 3, 4, 5]);

    println!("Box<i32> 指针大小: {} bytes", mem::size_of_val(&box_i32));
    println!("Box<i64> 指针大小: {} bytes", mem::size_of_val(&box_i64));
    println!(
        "Box<String> 指针大小: {} bytes",
        mem::size_of_val(&box_string)
    );
    println!(
        "Box<Vec<i32>> 指针大小: {} bytes",
        mem::size_of_val(&box_vec)
    );

    // 所有 Box 指针都是 8 bytes (64位系统)
    assert_eq!(mem::size_of::<Box<i32>>(), 8);
    assert_eq!(mem::size_of::<Box<String>>(), 8);
    assert_eq!(mem::size_of::<Box<Vec<i32>>>(), 8);

    println!("✓ 所有 Box<T> 指针在64位系统上都是8字节");
}

/// 2. Box<T> 基础使用
fn basic_box_usage() {
    println!("2. Box<T> 基础使用");
    println!("{}", "=".repeat(40));

    // 基本创建和使用
    let boxed_int = Box::new(5);
    println!("创建 Box<i32>: {}", boxed_int);

    // 解引用
    let value = *boxed_int;
    println!("解引用获取值: {}", value);

    // 可变 Box
    let mut boxed_mut = Box::new(10);
    *boxed_mut += 5;
    println!("修改后的值: {}", boxed_mut);

    // 复杂类型的 Box
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let person = Box::new(Person {
        name: "Alice".to_string(),
        age: 30,
    });
    println!("Box<Person>: {:?}", person);

    // 访问字段
    println!("姓名: {}, 年龄: {}", person.name, person.age);

    // Box 与 Vec 结合
    let boxed_vec = Box::new(vec![1, 2, 3, 4, 5]);
    println!("Box<Vec<i32>>: {:?}", boxed_vec);
    println!("向量长度: {}", boxed_vec.len());

    // 嵌套 Box
    let nested_box = Box::new(Box::new(42));
    println!("嵌套 Box: {}", nested_box);
    println!("双重解引用: {}", **nested_box);

    println!();
}

/// 3. 所有权和移动语义
fn ownership_and_move_semantics() {
    println!("3. 所有权和移动语义");
    println!("{}", "=".repeat(40));

    // 所有权转移
    let box1 = Box::new(String::from("Hello"));
    println!("box1 创建: {}", box1);

    let box2 = box1; // 所有权转移
    println!("box2 获得所有权: {}", box2);
    // println!("{}", box1); // 编译错误：box1 已被移动

    // 借用而不转移所有权
    let box3 = Box::new(vec![1, 2, 3]);
    let borrowed = &box3;
    println!("原始 box3: {:?}", box3);
    println!("借用的引用: {:?}", borrowed);

    // 可变借用
    let mut box4 = Box::new(vec![1, 2, 3]);
    {
        let borrowed_mut = &mut box4;
        borrowed_mut.push(4);
        println!("修改后的向量: {:?}", borrowed_mut);
    }
    println!("box4 最终状态: {:?}", box4);

    // 函数参数中的所有权
    fn take_ownership(boxed: Box<i32>) {
        println!("函数内部: {}", boxed);
        // boxed 在函数结束时被释放
    }

    fn borrow_box(boxed: &Box<i32>) {
        println!("借用的 Box: {}", boxed);
    }

    let box5 = Box::new(100);
    borrow_box(&box5); // 借用，不转移所有权
    println!("box5 仍然可用: {}", box5);

    take_ownership(box5); // 转移所有权
    // println!("{}", box5); // 编译错误：box5 已被移动

    // 返回 Box
    fn create_box() -> Box<String> {
        Box::new(String::from("Created in function"))
    }

    let returned_box = create_box();
    println!("从函数返回的 Box: {}", returned_box);

    println!();
}

/// 4. 递归类型和数据结构
fn recursive_types_and_structures() {
    println!("4. 递归类型和数据结构");
    println!("{}", "=".repeat(40));

    // 链表实现
    #[derive(Debug)]
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil,
    }

    impl<T> List<T> {
        fn new() -> Self {
            List::Nil
        }

        fn prepend(self, elem: T) -> Self {
            List::Cons(elem, Box::new(self))
        }

        fn len(&self) -> usize {
            match self {
                List::Cons(_, tail) => 1 + tail.len(),
                List::Nil => 0,
            }
        }

        fn stringify(&self) -> String
        where
            T: std::fmt::Display,
        {
            match self {
                List::Cons(head, tail) => {
                    format!("{}, {}", head, tail.stringify())
                }
                List::Nil => "Nil".to_string(),
            }
        }
    }

    let list = List::new().prepend(1).prepend(2).prepend(3);

    println!("链表: {}", list.stringify());
    println!("链表长度: {}", list.len());

    // 二叉树实现
    #[derive(Debug)]
    struct TreeNode<T> {
        value: T,
        left: Option<Box<TreeNode<T>>>,
        right: Option<Box<TreeNode<T>>>,
    }

    impl<T> TreeNode<T> {
        fn new(value: T) -> Self {
            TreeNode {
                value,
                left: None,
                right: None,
            }
        }

        fn insert_left(&mut self, value: T) {
            self.left = Some(Box::new(TreeNode::new(value)));
        }

        fn insert_right(&mut self, value: T) {
            self.right = Some(Box::new(TreeNode::new(value)));
        }

        fn height(&self) -> usize {
            let left_height = self.left.as_ref().map_or(0, |node| node.height());
            let right_height = self.right.as_ref().map_or(0, |node| node.height());
            1 + left_height.max(right_height)
        }
    }

    let mut root = TreeNode::new("root");
    root.insert_left("left");
    root.insert_right("right");

    if let Some(ref mut left) = root.left {
        left.insert_left("left-left");
        left.insert_right("left-right");
    }

    println!("二叉树根节点: {}", root.value);
    println!("树的高度: {}", root.height());

    // 图结构（简化版）
    #[derive(Debug)]
    struct GraphNode {
        id: usize,
        data: String,
        neighbors: Vec<Box<GraphNode>>,
    }

    impl GraphNode {
        fn new(id: usize, data: String) -> Self {
            GraphNode {
                id,
                data,
                neighbors: Vec::new(),
            }
        }

        fn add_neighbor(&mut self, neighbor: GraphNode) {
            self.neighbors.push(Box::new(neighbor));
        }
    }

    let mut node1 = GraphNode::new(1, "Node 1".to_string());
    let node2 = GraphNode::new(2, "Node 2".to_string());
    let node3 = GraphNode::new(3, "Node 3".to_string());

    node1.add_neighbor(node2);
    node1.add_neighbor(node3);

    println!("图节点 {} 有 {} 个邻居", node1.id, node1.neighbors.len());

    println!();
}

/// 5. 内存管理和 Drop trait
fn memory_management_and_drop() {
    println!("5. 内存管理和 Drop trait");
    println!("{}", "=".repeat(40));

    // 自定义 Drop 实现
    struct CustomDrop {
        name: String,
    }

    impl Drop for CustomDrop {
        fn drop(&mut self) {
            println!("🗑️  释放资源: {}", self.name);
        }
    }

    {
        let _resource1 = Box::new(CustomDrop {
            name: "Resource 1".to_string(),
        });
        let _resource2 = Box::new(CustomDrop {
            name: "Resource 2".to_string(),
        });
        println!("资源已创建");
    } // 作用域结束，资源自动释放

    println!("作用域结束后继续执行");

    // RAII 模式演示
    struct FileHandle {
        filename: String,
    }

    impl FileHandle {
        fn new(filename: &str) -> Self {
            println!("📂 打开文件: {}", filename);
            FileHandle {
                filename: filename.to_string(),
            }
        }
    }

    impl Drop for FileHandle {
        fn drop(&mut self) {
            println!("📂 关闭文件: {}", self.filename);
        }
    }

    {
        let _file = Box::new(FileHandle::new("data.txt"));
        println!("文件操作中...");
    } // 文件自动关闭

    // 内存泄漏检测
    fn demonstrate_memory_leak_prevention() {
        // 循环引用会导致内存泄漏，但 Box 本身不会
        let box1 = Box::new(vec![1, 2, 3, 4, 5]);
        println!("创建大向量，内存地址: {:p}", box1.as_ref());
        // box1 在函数结束时自动释放
    }

    demonstrate_memory_leak_prevention();
    println!("内存已正确释放");

    // 手动释放演示
    let boxed_data = Box::new(String::from("手动释放的数据"));
    println!("数据: {}", boxed_data);
    drop(boxed_data); // 显式释放
    // println!("{}", boxed_data); // 编译错误：已被释放

    println!();
}

/// 6. 解引用和智能指针特性
fn deref_and_smart_pointer_traits() {
    println!("6. 解引用和智能指针特性");
    println!("{}", "=".repeat(40));

    // Deref trait 演示
    let boxed_string = Box::new(String::from("Hello, Box!"));

    // 自动解引用
    println!("字符串长度: {}", boxed_string.len()); // 调用 String::len
    println!("转为大写: {}", boxed_string.to_uppercase()); // 调用 String::to_uppercase

    // 显式解引用
    let string_ref: &String = &*boxed_string;
    println!("显式解引用: {}", string_ref);

    // 解引用强制转换 (Deref Coercion)
    fn print_str(s: &str) {
        println!("打印字符串: {}", s);
    }

    print_str(&boxed_string); // Box<String> -> &String -> &str

    // 多层解引用
    let nested_box = Box::new(Box::new(Box::new(42)));
    println!("三层嵌套 Box: {}", ***nested_box);

    // 智能指针比较
    let box1 = Box::new(42);
    let box2 = Box::new(42);
    let box3 = Box::new(43);

    println!("box1 == box2: {}", box1 == box2); // 比较值，不是指针
    println!("box1 == box3: {}", box1 == box3);

    // 指针地址比较
    println!("box1 地址: {:p}", box1.as_ref());
    println!("box2 地址: {:p}", box2.as_ref());
    println!("地址相同: {}", ptr::eq(box1.as_ref(), box2.as_ref()));

    // AsRef 和 AsMut trait
    let boxed_vec = Box::new(vec![1, 2, 3]);
    let vec_ref: &Vec<i32> = boxed_vec.as_ref();
    println!("AsRef 获取引用: {:?}", vec_ref);

    let mut boxed_vec_mut = Box::new(vec![1, 2, 3]);
    {
        let vec_mut: &mut Vec<i32> = boxed_vec_mut.as_mut();
        vec_mut.push(4);
    }
    println!("AsMut 修改后: {:?}", boxed_vec_mut);

    println!();
}

/// 7. 实际应用场景
fn practical_use_cases() {
    println!("7. 实际应用场景");
    println!("{}", "=".repeat(40));

    // 场景1: 大对象存储
    struct LargeObject {
        data: [u8; 1024 * 1024], // 1MB 数据
        metadata: String,
    }

    impl LargeObject {
        fn new(metadata: String) -> Self {
            LargeObject {
                data: [0; 1024 * 1024],
                metadata,
            }
        }
    }

    // 使用 Box 避免栈溢出
    let large_obj = Box::new(LargeObject::new("大对象".to_string()));
    println!("大对象元数据: {}", large_obj.metadata);
    println!(
        "大对象大小: {} MB",
        mem::size_of_val(large_obj.as_ref()) / (1024 * 1024)
    );

    // 场景2: 动态分发和 trait 对象
    trait Drawable {
        fn draw(&self);
    }

    struct Circle {
        radius: f64,
    }
    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Drawable for Circle {
        fn draw(&self) {
            println!("绘制圆形，半径: {}", self.radius);
        }
    }

    impl Drawable for Rectangle {
        fn draw(&self) {
            println!("绘制矩形，{}x{}", self.width, self.height);
        }
    }

    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle {
            width: 10.0,
            height: 20.0,
        }),
        Box::new(Circle { radius: 3.0 }),
    ];

    println!("\n绘制所有图形:");
    for shape in &shapes {
        shape.draw();
    }

    // 场景3: 配置和状态管理
    #[derive(Debug)]
    struct AppConfig {
        database_url: String,
        api_key: String,
        max_connections: u32,
        features: Vec<String>,
    }

    struct Application {
        config: Box<AppConfig>,
        state: Box<dyn std::any::Any>,
    }

    impl Application {
        fn new(config: AppConfig) -> Self {
            Application {
                config: Box::new(config),
                state: Box::new(String::from("初始状态")),
            }
        }

        fn get_config(&self) -> &AppConfig {
            &self.config
        }
    }

    let config = AppConfig {
        database_url: "postgresql://localhost/mydb".to_string(),
        api_key: "secret-key".to_string(),
        max_connections: 100,
        features: vec!["auth".to_string(), "logging".to_string()],
    };

    let app = Application::new(config);
    println!("\n应用配置: {:?}", app.get_config());

    // 场景4: 缓存和延迟初始化
    struct LazyValue<T> {
        value: Option<Box<T>>,
        initializer: Box<dyn Fn() -> T>,
    }

    impl<T> LazyValue<T> {
        fn new<F>(initializer: F) -> Self
        where
            F: Fn() -> T + 'static,
        {
            LazyValue {
                value: None,
                initializer: Box::new(initializer),
            }
        }

        fn get(&mut self) -> &T {
            if self.value.is_none() {
                let value = (self.initializer)();
                self.value = Some(Box::new(value));
            }
            self.value.as_ref().unwrap()
        }
    }

    let mut lazy_string = LazyValue::new(|| {
        println!("初始化昂贵的计算...");
        "计算结果".to_string()
    });

    println!("\n第一次访问:");
    println!("值: {}", lazy_string.get());
    println!("第二次访问:");
    println!("值: {}", lazy_string.get());

    println!();
}

/// 8. 高级模式和技巧
fn advanced_patterns_and_techniques() {
    println!("8. 高级模式和技巧");
    println!("{}", "=".repeat(40));

    // 模式1: Box 与其他智能指针结合
    use std::cell::RefCell;
    use std::rc::Rc;

    // Box + Rc 实现共享所有权
    let shared_data = Rc::new(Box::new(vec![1, 2, 3, 4, 5]));
    let clone1 = Rc::clone(&shared_data);
    let clone2 = Rc::clone(&shared_data);

    println!("共享数据引用计数: {}", Rc::strong_count(&shared_data));
    println!("clone1: {:?}", clone1);
    println!("clone2: {:?}", clone2);

    // Box + RefCell 实现内部可变性
    let mutable_box = Box::new(RefCell::new(vec![1, 2, 3]));
    {
        let mut borrowed = mutable_box.borrow_mut();
        borrowed.push(4);
        borrowed.push(5);
    }
    println!("内部可变的 Box: {:?}", mutable_box.borrow());

    // 模式2: 自定义智能指针
    struct MyBox<T> {
        data: Box<T>,
        metadata: String,
    }

    impl<T> MyBox<T> {
        fn new(value: T, metadata: String) -> Self {
            MyBox {
                data: Box::new(value),
                metadata,
            }
        }

        fn get_metadata(&self) -> &str {
            &self.metadata
        }
    }

    impl<T> std::ops::Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    impl<T> std::ops::DerefMut for MyBox<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.data
        }
    }

    let mut my_box = MyBox::new(42, "自定义智能指针".to_string());
    println!("自定义 Box 值: {}", *my_box);
    println!("元数据: {}", my_box.get_metadata());

    *my_box += 8;
    println!("修改后的值: {}", *my_box);

    // 模式3: Box 与泛型和生命周期
    fn process_boxed_data<T: Debug + Clone>(data: Box<T>) -> Box<T> {
        println!("处理数据: {:?}", data);
        Box::new((*data).clone())
    }

    let original = Box::new(String::from("原始数据"));
    let processed = process_boxed_data(original);
    println!("处理后的数据: {}", processed);

    // 模式4: 条件 Box 分配
    fn maybe_box<T>(value: T, should_box: bool) -> Either<T, Box<T>> {
        if should_box {
            Either::Right(Box::new(value))
        } else {
            Either::Left(value)
        }
    }

    enum Either<L, R> {
        Left(L),
        Right(R),
    }

    impl<L: Debug, R: Debug> Debug for Either<L, R> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Either::Left(l) => write!(f, "Left({:?})", l),
                Either::Right(r) => write!(f, "Right({:?})", r),
            }
        }
    }

    let stack_value = maybe_box(42, false);
    let heap_value = maybe_box(42, true);

    println!("栈分配: {:?}", stack_value);
    println!("堆分配: {:?}", heap_value);

    println!();
}

/// 9. 性能分析和优化
fn performance_analysis_and_optimization() {
    println!("9. 性能分析和优化");
    println!("{}", "=".repeat(40));

    use std::time::Instant;

    // 性能测试1: 栈 vs 堆分配
    let iterations = 1_000_000;

    // 栈分配性能测试
    let start = Instant::now();
    for _ in 0..iterations {
        let _stack_value = [0u8; 64]; // 64字节栈分配
    }
    let stack_duration = start.elapsed();

    // 堆分配性能测试
    let start = Instant::now();
    for _ in 0..iterations {
        let _heap_value = Box::new([0u8; 64]); // 64字节堆分配
    }
    let heap_duration = start.elapsed();

    println!("性能对比 ({} 次迭代):", iterations);
    println!("栈分配时间: {:?}", stack_duration);
    println!("堆分配时间: {:?}", heap_duration);
    println!(
        "堆分配开销: {:.2}x",
        heap_duration.as_nanos() as f64 / stack_duration.as_nanos() as f64
    );

    // 性能测试2: 不同大小对象的分配
    fn benchmark_allocation<T: Default + Clone>(name: &str, value: T, iterations: usize) {
        // 栈分配
        let start = Instant::now();
        for _ in 0..iterations {
            let _stack = value.clone();
        }
        let stack_time = start.elapsed();

        // 堆分配
        let start = Instant::now();
        for _ in 0..iterations {
            let _heap = Box::new(value.clone());
        }
        let heap_time = start.elapsed();

        println!(
            "{} - 栈: {:?}, 堆: {:?}, 比率: {:.2}x",
            name,
            stack_time,
            heap_time,
            heap_time.as_nanos() as f64 / stack_time.as_nanos() as f64
        );
    }

    println!("\n不同大小对象分配性能:");
    benchmark_allocation("小对象 (4B)", 0i32, 100_000);
    benchmark_allocation("中对象 (1KB)", vec![0u8; 1024], 10_000);
    benchmark_allocation("大对象 (1MB)", vec![0u8; 1024 * 1024], 1_000);

    // 内存使用分析
    println!("\n内存使用分析:");

    // 不同类型的内存占用
    println!("基本类型大小:");
    println!("  i32: {} bytes", mem::size_of::<i32>());
    println!("  Box<i32>: {} bytes", mem::size_of::<Box<i32>>());
    println!("  String: {} bytes", mem::size_of::<String>());
    println!("  Box<String>: {} bytes", mem::size_of::<Box<String>>());
    println!("  Vec<i32>: {} bytes", mem::size_of::<Vec<i32>>());
    println!("  Box<Vec<i32>>: {} bytes", mem::size_of::<Box<Vec<i32>>>());

    // 数组大小对比
    println!("\n数组内存占用:");
    println!(
        "  [i32; 1000] 栈数组: {} bytes",
        mem::size_of::<[i32; 1000]>()
    );
    println!(
        "  Box<[i32; 1000]> 堆数组: {} bytes (Box本身)",
        mem::size_of::<Box<[i32; 1000]>>()
    );
    println!(
        "  Vec<i32> 动态数组: {} bytes (Vec本身)",
        mem::size_of::<Vec<i32>>()
    );

    // 优化建议
    println!("\n性能优化建议:");
    println!("1. 小对象 (<= 栈大小限制) 优先使用栈分配");
    println!("2. 大对象或递归类型使用 Box 堆分配");
    println!("3. 频繁分配/释放的场景考虑对象池");
    println!("4. 使用 Vec 而不是 Box<[T]> 来处理动态数组");
    println!("5. 避免不必要的 Box 嵌套");
    println!("6. 在 release 模式下进行性能测试");

    // 内存对齐分析
    println!("\n内存对齐分析:");

    #[repr(C)]
    struct AlignedStruct {
        a: u8,  // 1 byte
        b: u32, // 4 bytes
        c: u8,  // 1 byte
    }

    #[repr(C, packed)]
    struct PackedStruct {
        a: u8,  // 1 byte
        b: u32, // 4 bytes
        c: u8,  // 1 byte
    }

    println!("对齐结构体大小: {} bytes", mem::size_of::<AlignedStruct>());
    println!("紧凑结构体大小: {} bytes", mem::size_of::<PackedStruct>());
    println!(
        "Box<对齐结构体>: {} bytes",
        mem::size_of::<Box<AlignedStruct>>()
    );
    println!(
        "Box<紧凑结构体>: {} bytes",
        mem::size_of::<Box<PackedStruct>>()
    );

    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_box_operations() {
        let boxed = Box::new(42);
        assert_eq!(*boxed, 42);

        let mut boxed_mut = Box::new(10);
        *boxed_mut += 5;
        assert_eq!(*boxed_mut, 15);
    }

    #[test]
    fn test_box_ownership() {
        let box1 = Box::new(String::from("test"));
        let box2 = box1; // 移动
        assert_eq!(*box2, "test");
        // box1 不再可用
    }

    #[test]
    fn test_recursive_list() {
        #[derive(Debug, PartialEq)]
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }

        let list = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );

        match list {
            List::Cons(val, _) => assert_eq!(val, 1),
            List::Nil => panic!("Expected Cons"),
        }
    }

    #[test]
    fn test_box_deref() {
        let boxed_string = Box::new(String::from("hello"));
        assert_eq!(boxed_string.len(), 5);
        assert_eq!(boxed_string.to_uppercase(), "HELLO");
    }

    #[test]
    fn test_box_comparison() {
        let box1 = Box::new(42);
        let box2 = Box::new(42);
        let box3 = Box::new(43);

        assert_eq!(box1, box2); // 值相等
        assert_ne!(box1, box3); // 值不等

        // 指针地址不同
        assert_ne!(box1.as_ref() as *const i32, box2.as_ref() as *const i32);
    }

    #[test]
    fn test_box_with_trait_objects() {
        trait TestTrait {
            fn test_method(&self) -> i32;
        }

        struct TestStruct(i32);

        impl TestTrait for TestStruct {
            fn test_method(&self) -> i32 {
                self.0
            }
        }

        let boxed_trait: Box<dyn TestTrait> = Box::new(TestStruct(42));
        assert_eq!(boxed_trait.test_method(), 42);
    }

    #[test]
    fn test_memory_layout() {
        // 所有 Box 指针都是 8 bytes (64位系统)
        assert_eq!(mem::size_of::<Box<i32>>(), 8);
        assert_eq!(mem::size_of::<Box<String>>(), 8);
        assert_eq!(mem::size_of::<Box<Vec<i32>>>(), 8);
        assert_eq!(mem::size_of::<Box<[u8; 1024]>>(), 8);
    }

    #[test]
    fn test_box_drop() {
        use std::sync::{Arc, Mutex};

        let counter = Arc::new(Mutex::new(0));

        struct DropCounter {
            counter: Arc<Mutex<i32>>,
        }

        impl Drop for DropCounter {
            fn drop(&mut self) {
                let mut count = self.counter.lock().unwrap();
                *count += 1;
            }
        }

        {
            let _boxed = Box::new(DropCounter {
                counter: Arc::clone(&counter),
            });
        } // Drop 在这里被调用

        assert_eq!(*counter.lock().unwrap(), 1);
    }
}
