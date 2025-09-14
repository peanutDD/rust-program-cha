//! # Rust 特征对象 (Trait Object) 深度分析
//!
//! 基于 https://course.rs/basic/trait/trait-object.html 的全面分析
//! 涵盖特征对象的所有核心概念、机制和实际应用

use std::any::Any;
use std::fmt::{Debug, Display};
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

fn main() {
    println!("=== Rust 特征对象 (Trait Object) 深度分析 ===");
    println!();

    // 1. 特征对象基础概念
    trait_object_basics();

    // 2. 动态分发机制
    dynamic_dispatch_analysis();

    // 3. 特征对象语法
    trait_object_syntax();

    // 4. 对象安全性
    object_safety_rules();

    // 5. 虚函数表机制
    vtable_mechanism();

    // 6. 性能分析
    performance_comparison();

    // 7. 实际应用场景
    practical_applications();

    // 8. 高级特性
    advanced_features();

    // 9. 常见陷阱
    common_pitfalls();

    // 10. 最佳实践
    best_practices();

    println!("\n=== 特征对象分析完成 ===");
}

/// 1. 特征对象基础概念
fn trait_object_basics() {
    println!("\n1. === 特征对象基础概念 ===");

    // 1.1 什么是特征对象
    println!("\n1.1 特征对象定义:");
    println!("特征对象是一种实现动态分发的机制，允许在运行时确定调用哪个具体类型的方法");
    println!("语法: dyn Trait，表示实现了 Trait 的某种类型的实例");

    // 1.2 静态分发 vs 动态分发对比
    println!("\n1.2 静态分发 vs 动态分发:");

    // 静态分发示例
    trait Draw {
        fn draw(&self);
    }

    struct Circle {
        radius: f64,
    }

    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Draw for Circle {
        fn draw(&self) {
            println!("绘制圆形，半径: {}", self.radius);
        }
    }

    impl Draw for Rectangle {
        fn draw(&self) {
            println!("绘制矩形，宽: {}, 高: {}", self.width, self.height);
        }
    }

    // 静态分发：编译时确定类型
    fn draw_static<T: Draw>(shape: &T) {
        shape.draw();
    }

    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 10.0,
        height: 8.0,
    };

    println!("静态分发示例:");
    draw_static(&circle);
    draw_static(&rectangle);

    // 动态分发：运行时确定类型
    fn draw_dynamic(shape: &dyn Draw) {
        shape.draw();
    }

    println!("\n动态分发示例:");
    draw_dynamic(&circle);
    draw_dynamic(&rectangle);

    // 1.3 特征对象的内存布局
    println!("\n1.3 特征对象内存布局:");
    println!("特征对象由两部分组成:");
    println!("- 数据指针: 指向具体类型的实例");
    println!("- 虚函数表指针: 指向该类型的方法实现");

    let trait_obj: &dyn Draw = &circle;
    println!(
        "特征对象大小: {} 字节 (两个指针)",
        std::mem::size_of_val(&trait_obj)
    );
    println!("具体类型大小: {} 字节", std::mem::size_of_val(&circle));
}

/// 2. 动态分发机制详细分析
fn dynamic_dispatch_analysis() {
    println!("\n\n2. === 动态分发机制详细分析 ===");

    // 2.1 动态分发的工作原理
    println!("\n2.1 动态分发工作原理:");
    println!("1. 编译器为每个实现了特征的类型生成虚函数表(vtable)");
    println!("2. 特征对象包含指向数据和vtable的指针");
    println!("3. 方法调用通过vtable间接进行");

    trait Animal {
        fn name(&self) -> &str;
        fn make_sound(&self);
        fn info(&self) {
            println!("{} 发出声音:", self.name());
            self.make_sound();
        }
    }

    struct Dog {
        name: String,
        breed: String,
    }

    struct Cat {
        name: String,
        age: u8,
    }

    impl Animal for Dog {
        fn name(&self) -> &str {
            &self.name
        }

        fn make_sound(&self) {
            println!("汪汪! ({}品种)", self.breed);
        }
    }

    impl Animal for Cat {
        fn name(&self) -> &str {
            &self.name
        }

        fn make_sound(&self) {
            println!("喵喵! ({}岁)", self.age);
        }
    }

    // 2.2 动态分发的实际应用
    println!("\n2.2 动态分发实际应用:");

    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog {
            name: "旺财".to_string(),
            breed: "金毛".to_string(),
        }),
        Box::new(Cat {
            name: "咪咪".to_string(),
            age: 3,
        }),
        Box::new(Dog {
            name: "小黑".to_string(),
            breed: "拉布拉多".to_string(),
        }),
    ];

    println!("动物园里的动物们:");
    for animal in &animals {
        animal.info();
    }

    // 2.3 动态分发的优势
    println!("\n2.3 动态分发的优势:");
    println!("- 运行时多态性: 可以处理不同类型的对象");
    println!("- 代码复用: 同一套代码处理多种类型");
    println!("- 灵活性: 可以在运行时添加新类型");

    // 2.4 动态分发的代价
    println!("\n2.4 动态分发的代价:");
    println!("- 运行时开销: 间接函数调用");
    println!("- 内存开销: 额外的指针存储");
    println!("- 编译器优化限制: 无法内联优化");
}

/// 3. 特征对象语法详解
fn trait_object_syntax() {
    println!("\n\n3. === 特征对象语法详解 ===");

    // 3.1 基本语法形式
    println!("\n3.1 基本语法形式:");

    trait Drawable {
        fn draw(&self);
        fn area(&self) -> f64;
    }

    struct Square {
        side: f64,
    }

    impl Drawable for Square {
        fn draw(&self) {
            println!("绘制边长为 {} 的正方形", self.side);
        }

        fn area(&self) -> f64 {
            self.side * self.side
        }
    }

    let square = Square { side: 4.0 };

    // 不同的特征对象语法
    println!("不同的特征对象语法:");

    // 1. 引用形式
    let drawable_ref: &dyn Drawable = &square;
    println!("1. 引用形式: &dyn Drawable");
    drawable_ref.draw();

    // 2. Box 智能指针
    let drawable_box: Box<dyn Drawable> = Box::new(Square { side: 6.0 });
    println!("\n2. Box 智能指针: Box<dyn Drawable>");
    drawable_box.draw();

    // 3. Rc 智能指针
    let drawable_rc: Rc<dyn Drawable> = Rc::new(Square { side: 8.0 });
    println!("\n3. Rc 智能指针: Rc<dyn Drawable>");
    drawable_rc.draw();

    // 4. Arc 智能指针 (线程安全)
    let drawable_arc: Arc<dyn Drawable> = Arc::new(Square { side: 10.0 });
    println!("\n4. Arc 智能指针: Arc<dyn Drawable>");
    drawable_arc.draw();

    // 3.2 特征对象作为参数
    println!("\n3.2 特征对象作为参数:");

    fn process_drawable(drawable: &dyn Drawable) {
        drawable.draw();
        println!("面积: {}", drawable.area());
    }

    fn process_drawable_box(drawable: Box<dyn Drawable>) {
        drawable.draw();
        println!("面积: {}", drawable.area());
    }

    process_drawable(&square);
    process_drawable_box(Box::new(Square { side: 12.0 }));

    // 3.3 特征对象作为返回值
    println!("\n3.3 特征对象作为返回值:");

    fn create_drawable(size: f64) -> Box<dyn Drawable> {
        Box::new(Square { side: size })
    }

    let created = create_drawable(15.0);
    created.draw();

    // 3.4 特征对象集合
    println!("\n3.4 特征对象集合:");

    let drawables: Vec<Box<dyn Drawable>> = vec![
        Box::new(Square { side: 2.0 }),
        Box::new(Square { side: 4.0 }),
        Box::new(Square { side: 6.0 }),
    ];

    println!("绘制所有图形:");
    for (i, drawable) in drawables.iter().enumerate() {
        print!("图形 {}: ", i + 1);
        drawable.draw();
    }
}

/// 4. 对象安全性规则
fn object_safety_rules() {
    println!("\n\n4. === 对象安全性规则 ===");

    println!("\n4.1 对象安全性概念:");
    println!("并非所有特征都可以作为特征对象使用，必须满足对象安全性规则");

    // 4.2 对象安全的规则
    println!("\n4.2 对象安全规则:");
    println!("1. 方法的返回类型不能是 Self");
    println!("2. 方法不能有泛型参数");
    println!("3. 除了接收者之外，方法不能有其他 Self 类型的参数");

    // 4.3 对象安全的特征示例
    println!("\n4.3 对象安全的特征:");

    trait ObjectSafe {
        fn method1(&self);
        fn method2(&mut self);
        fn method3(&self, x: i32) -> String;
    }

    struct SafeImpl;

    impl ObjectSafe for SafeImpl {
        fn method1(&self) {
            println!("对象安全的方法1");
        }

        fn method2(&mut self) {
            println!("对象安全的方法2");
        }

        fn method3(&self, x: i32) -> String {
            format!("参数: {}", x)
        }
    }

    let safe_obj: Box<dyn ObjectSafe> = Box::new(SafeImpl);
    safe_obj.method1();

    // 4.4 对象不安全的特征示例
    println!("\n4.4 对象不安全的特征 (注释掉的代码):");

    trait ObjectUnsafe {
        // 返回 Self - 不安全
        fn clone_self(&self) -> Self
        where
            Self: Sized; // 必须添加 Sized 约束

        // 泛型方法 - 不安全
        fn generic_method<T>(&self, value: T)
        where
            Self: Sized; // 必须添加 Sized 约束

        // Self 作为参数 - 不安全
        fn compare(&self, other: Self) -> bool
        where
            Self: Sized; // 必须添加 Sized 约束

        // 对象安全的方法
        fn safe_method(&self);
    }

    struct UnsafeImpl {
        value: i32,
    }

    impl ObjectUnsafe for UnsafeImpl {
        fn clone_self(&self) -> Self {
            UnsafeImpl { value: self.value }
        }

        fn generic_method<T>(&self, _value: T) {
            println!("泛型方法");
        }

        fn compare(&self, other: Self) -> bool {
            self.value == other.value
        }

        fn safe_method(&self) {
            println!("对象安全的方法，值: {}", self.value);
        }
    }

    // 只能调用对象安全的方法
    let unsafe_impl = UnsafeImpl { value: 42 };

    // 这样可以工作，因为只使用对象安全的方法
    trait ObjectSafeSubset {
        fn safe_method(&self);
    }

    impl ObjectSafeSubset for UnsafeImpl {
        fn safe_method(&self) {
            println!("子集特征的安全方法，值: {}", self.value);
        }
    }

    let safe_subset: Box<dyn ObjectSafeSubset> = Box::new(unsafe_impl);
    safe_subset.safe_method();

    // 4.5 使用 where Self: Sized 约束
    println!("\n4.5 使用 where Self: Sized 约束:");
    println!("通过添加 Sized 约束，可以在特征中包含不安全的方法");
    println!("这些方法在特征对象中不可用，但在具体类型中可用");
}

/// 5. 虚函数表机制深入分析
fn vtable_mechanism() {
    println!("\n\n5. === 虚函数表机制深入分析 ===");

    // 5.1 虚函数表概念
    println!("\n5.1 虚函数表 (vtable) 概念:");
    println!("虚函数表是一个函数指针数组，存储了类型的方法实现");

    trait VTableDemo {
        fn method_a(&self);
        fn method_b(&self) -> i32;
        fn method_c(&self, x: f64);
    }

    struct TypeA {
        value: i32,
    }

    struct TypeB {
        name: String,
    }

    impl VTableDemo for TypeA {
        fn method_a(&self) {
            println!("TypeA::method_a, value: {}", self.value);
        }

        fn method_b(&self) -> i32 {
            self.value * 2
        }

        fn method_c(&self, x: f64) {
            println!("TypeA::method_c, value: {}, x: {}", self.value, x);
        }
    }

    impl VTableDemo for TypeB {
        fn method_a(&self) {
            println!("TypeB::method_a, name: {}", self.name);
        }

        fn method_b(&self) -> i32 {
            self.name.len() as i32
        }

        fn method_c(&self, x: f64) {
            println!("TypeB::method_c, name: {}, x: {}", self.name, x);
        }
    }

    // 5.2 虚函数表的工作过程
    println!("\n5.2 虚函数表工作过程演示:");

    let type_a = TypeA { value: 100 };
    let type_b = TypeB {
        name: "Hello".to_string(),
    };

    // 创建特征对象
    let objects: Vec<Box<dyn VTableDemo>> = vec![Box::new(type_a), Box::new(type_b)];

    println!("通过特征对象调用方法 (使用虚函数表):");
    for (i, obj) in objects.iter().enumerate() {
        println!("对象 {}:", i + 1);
        obj.method_a();
        println!("  method_b 返回: {}", obj.method_b());
        obj.method_c(3.14);
        println!();
    }

    // 5.3 虚函数表的内存布局
    println!("5.3 虚函数表内存布局分析:");
    println!("每个特征对象包含:");
    println!("- 数据指针: 8 字节 (64位系统)");
    println!("- vtable指针: 8 字节 (64位系统)");
    println!("总计: 16 字节");

    let obj_ref: &dyn VTableDemo = &TypeA { value: 42 };
    println!("特征对象引用大小: {} 字节", std::mem::size_of_val(&obj_ref));

    // 5.4 虚函数表的性能影响
    println!("\n5.4 虚函数表性能影响:");
    println!("- 间接调用开销: 需要通过指针查找函数");
    println!("- 缓存局部性: 可能影响CPU缓存效率");
    println!("- 内联限制: 编译器无法内联虚函数调用");

    // 5.5 Drop trait 的特殊处理
    println!("\n5.5 Drop trait 在虚函数表中的特殊处理:");

    struct DropDemo {
        name: String,
    }

    impl Drop for DropDemo {
        fn drop(&mut self) {
            println!("销毁 DropDemo: {}", self.name);
        }
    }

    impl VTableDemo for DropDemo {
        fn method_a(&self) {
            println!("DropDemo::method_a, name: {}", self.name);
        }

        fn method_b(&self) -> i32 {
            42
        }

        fn method_c(&self, x: f64) {
            println!("DropDemo::method_c, name: {}, x: {}", self.name, x);
        }
    }

    {
        let drop_obj: Box<dyn VTableDemo> = Box::new(DropDemo {
            name: "测试对象".to_string(),
        });
        drop_obj.method_a();
        println!("drop_obj 即将离开作用域...");
    } // drop_obj 在这里被销毁，会调用 Drop::drop

    println!("Drop 已完成");
}

/// 6. 性能对比分析
fn performance_comparison() {
    println!("\n\n6. === 性能对比分析 ===");

    // 6.1 静态分发 vs 动态分发性能测试
    println!("\n6.1 静态分发 vs 动态分发性能测试:");

    trait Compute {
        fn compute(&self, x: i32) -> i32;
    }

    struct FastCompute;
    struct SlowCompute;

    impl Compute for FastCompute {
        fn compute(&self, x: i32) -> i32 {
            x * 2 + 1
        }
    }

    impl Compute for SlowCompute {
        fn compute(&self, x: i32) -> i32 {
            (0..10).fold(x, |acc, _| acc + 1)
        }
    }

    // 静态分发版本
    fn static_dispatch_test<T: Compute>(computer: &T, iterations: usize) -> u128 {
        let start = Instant::now();
        let mut sum: u64 = 0;
        for i in 0..iterations {
            sum = sum.wrapping_add(computer.compute(i as i32) as u64);
        }
        let duration = start.elapsed().as_nanos();
        println!("静态分发结果: {}, 耗时: {} 纳秒", sum, duration);
        duration
    }

    // 动态分发版本
    fn dynamic_dispatch_test(computer: &dyn Compute, iterations: usize) -> u128 {
        let start = Instant::now();
        let mut sum: u64 = 0;
        for i in 0..iterations {
            sum = sum.wrapping_add(computer.compute(i as i32) as u64);
        }
        let duration = start.elapsed().as_nanos();
        println!("动态分发结果: {}, 耗时: {} 纳秒", sum, duration);
        duration
    }

    let fast_compute = FastCompute;
    let iterations = 100_000;

    println!("测试迭代次数: {}", iterations);

    let static_time = static_dispatch_test(&fast_compute, iterations);
    let dynamic_time = dynamic_dispatch_test(&fast_compute, iterations);

    let overhead = if dynamic_time > static_time {
        ((dynamic_time - static_time) as f64 / static_time as f64) * 100.0
    } else {
        0.0
    };

    println!("动态分发开销: {:.2}%", overhead);

    // 6.2 内存使用对比
    println!("\n6.2 内存使用对比:");

    let concrete_obj = FastCompute;
    let trait_obj: &dyn Compute = &concrete_obj;
    let boxed_trait_obj: Box<dyn Compute> = Box::new(FastCompute);

    println!(
        "具体类型大小: {} 字节",
        std::mem::size_of_val(&concrete_obj)
    );
    println!(
        "特征对象引用大小: {} 字节",
        std::mem::size_of_val(&trait_obj)
    );
    println!(
        "Box<dyn Trait> 大小: {} 字节",
        std::mem::size_of::<Box<dyn Compute>>()
    );

    // 6.3 编译器优化差异
    println!("\n6.3 编译器优化差异:");
    println!("静态分发优势:");
    println!("- 内联优化: 编译器可以内联函数调用");
    println!("- 死代码消除: 未使用的方法可以被移除");
    println!("- 常量折叠: 编译时计算常量表达式");

    println!("\n动态分发限制:");
    println!("- 无法内联: 运行时才知道具体实现");
    println!("- 间接调用: 需要通过函数指针调用");
    println!("- 缓存不友好: 可能导致分支预测失败");

    // 6.4 选择建议
    println!("\n6.4 性能选择建议:");
    println!("使用静态分发当:");
    println!("- 性能是关键因素");
    println!("- 编译时已知所有类型");
    println!("- 代码大小不是问题");

    println!("\n使用动态分发当:");
    println!("- 需要运行时多态");
    println!("- 类型在编译时未知");
    println!("- 代码复用比性能更重要");
}

/// 7. 实际应用场景
fn practical_applications() {
    println!("\n\n7. === 实际应用场景 ===");

    // 7.1 插件系统
    println!("\n7.1 插件系统:");

    trait Plugin {
        fn name(&self) -> &str;
        fn version(&self) -> &str;
        fn execute(&self, input: &str) -> String;
    }

    struct TextProcessor {
        name: String,
        version: String,
    }

    struct ImageFilter {
        name: String,
        version: String,
    }

    impl Plugin for TextProcessor {
        fn name(&self) -> &str {
            &self.name
        }

        fn version(&self) -> &str {
            &self.version
        }

        fn execute(&self, input: &str) -> String {
            format!("[文本处理] {}", input.to_uppercase())
        }
    }

    impl Plugin for ImageFilter {
        fn name(&self) -> &str {
            &self.name
        }

        fn version(&self) -> &str {
            &self.version
        }

        fn execute(&self, input: &str) -> String {
            format!("[图像滤镜] 对 {} 应用模糊效果", input)
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
            println!("注册插件: {} v{}", plugin.name(), plugin.version());
            self.plugins.push(plugin);
        }

        fn execute_all(&self, input: &str) {
            println!("\n执行所有插件，输入: {}", input);
            for plugin in &self.plugins {
                let result = plugin.execute(input);
                println!("{}: {}", plugin.name(), result);
            }
        }
    }

    let mut manager = PluginManager::new();

    manager.register_plugin(Box::new(TextProcessor {
        name: "文本处理器".to_string(),
        version: "1.0.0".to_string(),
    }));

    manager.register_plugin(Box::new(ImageFilter {
        name: "图像滤镜".to_string(),
        version: "2.1.0".to_string(),
    }));

    manager.execute_all("hello world");

    // 7.2 状态机模式
    println!("\n\n7.2 状态机模式:");

    trait State {
        fn handle(&self, context: &mut Context);
        fn name(&self) -> &str;
    }

    struct Context {
        state: Box<dyn State>,
        data: String,
    }

    impl Context {
        fn new(initial_state: Box<dyn State>) -> Self {
            Context {
                state: initial_state,
                data: String::new(),
            }
        }

        fn set_state(&mut self, state: Box<dyn State>) {
            let old_name = self.state.name().to_string();
            let new_name = state.name().to_string();
            println!("状态转换: {} -> {}", old_name, new_name);
            self.state = state;
        }

        fn handle(&mut self) {
            // 临时取出状态以避免借用冲突
            let current_state = std::mem::replace(&mut self.state, Box::new(IdleState));
            current_state.handle(self);
        }

        fn add_data(&mut self, data: &str) {
            self.data.push_str(data);
        }

        fn get_data(&self) -> &str {
            &self.data
        }
    }

    struct IdleState;
    struct ProcessingState;
    struct CompletedState;

    impl State for IdleState {
        fn handle(&self, context: &mut Context) {
            println!("空闲状态: 开始处理");
            context.add_data("开始处理 -> ");
            context.set_state(Box::new(ProcessingState));
        }

        fn name(&self) -> &str {
            "空闲"
        }
    }

    impl State for ProcessingState {
        fn handle(&self, context: &mut Context) {
            println!("处理状态: 正在处理数据");
            context.add_data("处理中 -> ");
            context.set_state(Box::new(CompletedState));
        }

        fn name(&self) -> &str {
            "处理中"
        }
    }

    impl State for CompletedState {
        fn handle(&self, context: &mut Context) {
            println!("完成状态: 处理完成");
            context.add_data("完成");
            println!("处理流程: {}", context.get_data());
        }

        fn name(&self) -> &str {
            "完成"
        }
    }

    let mut context = Context::new(Box::new(IdleState));

    println!("状态机演示:");
    context.handle(); // 空闲 -> 处理中
    context.handle(); // 处理中 -> 完成
    context.handle(); // 完成状态处理

    // 7.3 策略模式
    println!("\n\n7.3 策略模式:");

    trait SortStrategy {
        fn sort(&self, data: &mut Vec<i32>);
        fn name(&self) -> &str;
    }

    struct BubbleSort;
    struct QuickSort;
    struct MergeSort;

    impl SortStrategy for BubbleSort {
        fn sort(&self, data: &mut Vec<i32>) {
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

    impl SortStrategy for QuickSort {
        fn sort(&self, data: &mut Vec<i32>) {
            if data.len() <= 1 {
                return;
            }
            data.sort(); // 使用标准库的快速排序实现
        }

        fn name(&self) -> &str {
            "快速排序"
        }
    }

    impl SortStrategy for MergeSort {
        fn sort(&self, data: &mut Vec<i32>) {
            if data.len() <= 1 {
                return;
            }
            // 简化的归并排序实现
            data.sort();
        }

        fn name(&self) -> &str {
            "归并排序"
        }
    }

    struct Sorter {
        strategy: Box<dyn SortStrategy>,
    }

    impl Sorter {
        fn new(strategy: Box<dyn SortStrategy>) -> Self {
            Sorter { strategy }
        }

        fn set_strategy(&mut self, strategy: Box<dyn SortStrategy>) {
            self.strategy = strategy;
        }

        fn sort(&self, data: &mut Vec<i32>) {
            println!("使用 {} 排序: {:?}", self.strategy.name(), data);
            self.strategy.sort(data);
            println!("排序结果: {:?}", data);
        }
    }

    let mut sorter = Sorter::new(Box::new(BubbleSort));
    let mut data1 = vec![64, 34, 25, 12, 22, 11, 90];
    sorter.sort(&mut data1);

    sorter.set_strategy(Box::new(QuickSort));
    let mut data2 = vec![64, 34, 25, 12, 22, 11, 90];
    sorter.sort(&mut data2);
}

/// 8. 高级特性
fn advanced_features() {
    println!("\n\n8. === 高级特性 ===");

    // 8.1 特征对象与生命周期
    println!("\n8.1 特征对象与生命周期:");

    trait Processor {
        fn process(&self, data: &str) -> String;
    }

    struct StringProcessor<'a> {
        prefix: &'a str,
    }

    impl<'a> Processor for StringProcessor<'a> {
        fn process(&self, data: &str) -> String {
            format!("{}: {}", self.prefix, data)
        }
    }

    // 生命周期参数在特征对象中的使用
    fn process_with_lifetime<'a>(processor: &'a dyn Processor, data: &str) -> String {
        processor.process(data)
    }

    let prefix = "处理";
    let processor = StringProcessor { prefix };
    let result = process_with_lifetime(&processor, "数据");
    println!("生命周期处理结果: {}", result);

    // 8.2 特征对象与 Send + Sync
    println!("\n8.2 特征对象与 Send + Sync:");

    trait ThreadSafeProcessor: Send + Sync {
        fn process_safely(&self, data: i32) -> i32;
    }

    struct SafeProcessor {
        multiplier: i32,
    }

    impl ThreadSafeProcessor for SafeProcessor {
        fn process_safely(&self, data: i32) -> i32 {
            data * self.multiplier
        }
    }

    // 线程安全的特征对象
    let processor = Arc::new(SafeProcessor { multiplier: 2 });
    let processor_clone = processor.clone();

    let handle = thread::spawn(move || {
        let trait_obj: Arc<dyn ThreadSafeProcessor> = processor_clone;
        trait_obj.process_safely(42)
    });

    let result = handle.join().unwrap();
    println!("线程安全处理结果: {}", result);

    // 8.3 特征对象与 Any trait
    println!("\n8.3 特征对象与 Any trait:");

    trait ProcessorAny: Any {
        fn process_any(&self) -> String;
        fn as_any(&self) -> &dyn Any;
    }

    struct IntProcessor {
        value: i32,
    }

    struct StringProcessorAny {
        value: String,
    }

    impl ProcessorAny for IntProcessor {
        fn process_any(&self) -> String {
            format!("整数处理: {}", self.value)
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ProcessorAny for StringProcessorAny {
        fn process_any(&self) -> String {
            format!("字符串处理: {}", self.value)
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    let processors: Vec<Box<dyn ProcessorAny>> = vec![
        Box::new(IntProcessor { value: 100 }),
        Box::new(StringProcessorAny {
            value: "Hello".to_string(),
        }),
    ];

    println!("使用 Any trait 进行类型转换:");
    for processor in &processors {
        println!("{}", processor.process_any());

        // 尝试向下转型
        if let Some(int_proc) = processor.as_any().downcast_ref::<IntProcessor>() {
            println!("  -> 转换为 IntProcessor: {}", int_proc.value);
        } else if let Some(str_proc) = processor.as_any().downcast_ref::<StringProcessorAny>() {
            println!("  -> 转换为 StringProcessorAny: {}", str_proc.value);
        }
    }

    // 8.4 多特征对象
    println!("\n8.4 多特征对象:");

    trait Drawable2 {
        fn draw2(&self);
    }

    trait Clickable {
        fn click(&self);
    }

    struct Button {
        label: String,
    }

    impl Drawable2 for Button {
        fn draw2(&self) {
            println!("绘制按钮: {}", self.label);
        }
    }

    impl Clickable for Button {
        fn click(&self) {
            println!("点击按钮: {}", self.label);
        }
    }

    // 创建组合特征
    trait UIElement: Drawable2 + Clickable {}

    impl UIElement for Button {}

    // 组合多个特征
    fn handle_ui_element(element: &dyn UIElement) {
        element.draw2();
        element.click();
    }

    let button = Button {
        label: "确定".to_string(),
    };

    handle_ui_element(&button);

    // 使用 Box 存储多特征对象
    let ui_elements: Vec<Box<dyn UIElement>> = vec![
        Box::new(Button {
            label: "取消".to_string(),
        }),
        Box::new(Button {
            label: "保存".to_string(),
        }),
    ];

    println!("\n处理多个UI元素:");
    for element in &ui_elements {
        element.draw2();
        element.click();
        println!();
    }
}

/// 9. 常见陷阱和解决方案
fn common_pitfalls() {
    println!("\n\n9. === 常见陷阱和解决方案 ===");

    // 9.1 对象安全性陷阱
    println!("\n9.1 对象安全性陷阱:");

    // 错误示例：返回 Self
    trait BadClone {
        // 这会导致对象不安全
        fn bad_clone(&self) -> Self
        where
            Self: Sized;

        // 正确的方式：返回 Box
        fn good_clone(&self) -> Box<dyn BadClone>;
    }

    struct CloneableStruct {
        value: i32,
    }

    impl BadClone for CloneableStruct {
        fn bad_clone(&self) -> Self {
            CloneableStruct { value: self.value }
        }

        fn good_clone(&self) -> Box<dyn BadClone> {
            Box::new(CloneableStruct { value: self.value })
        }
    }

    let original = CloneableStruct { value: 42 };

    // 这样可以工作
    let cloned = original.bad_clone();
    println!("直接克隆: {}", cloned.value);

    // 通过特征对象克隆
    let trait_obj: &dyn BadClone = &original;
    let boxed_clone = trait_obj.good_clone();
    println!("特征对象克隆成功");

    // 9.2 生命周期陷阱
    println!("\n9.2 生命周期陷阱:");

    trait DataProcessor {
        fn process_data(&self, data: &str) -> String;
    }

    struct SimpleProcessor;

    impl DataProcessor for SimpleProcessor {
        fn process_data(&self, data: &str) -> String {
            // 返回处理后的数据
            format!("处理后的: {}", data)
        }
    }

    // 正确的使用方式
    fn use_processor_correctly() {
        let processor = SimpleProcessor;
        let data = "test data";
        let trait_obj: &dyn DataProcessor = &processor;
        let result = trait_obj.process_data(data);
        println!("处理结果: {}", result);
    }

    use_processor_correctly();

    // 9.3 性能陷阱
    println!("\n9.3 性能陷阱:");

    trait ExpensiveOperation {
        fn expensive_calc(&self, n: usize) -> usize;
    }

    struct Calculator;

    impl ExpensiveOperation for Calculator {
        fn expensive_calc(&self, n: usize) -> usize {
            (0..n).sum()
        }
    }

    // 错误：频繁的动态分发
    fn inefficient_loop(ops: &[Box<dyn ExpensiveOperation>]) {
        let start = Instant::now();
        for _ in 0..1000 {
            for op in ops {
                op.expensive_calc(100);
            }
        }
        println!("低效循环耗时: {:?}", start.elapsed());
    }

    // 正确：减少动态分发次数
    fn efficient_approach(ops: &[Box<dyn ExpensiveOperation>]) {
        let start = Instant::now();
        for op in ops {
            for _ in 0..1000 {
                op.expensive_calc(100);
            }
        }
        println!("高效方法耗时: {:?}", start.elapsed());
    }

    let operations: Vec<Box<dyn ExpensiveOperation>> =
        vec![Box::new(Calculator), Box::new(Calculator)];

    inefficient_loop(&operations);
    efficient_approach(&operations);

    // 9.4 内存泄漏陷阱
    println!("\n9.4 内存泄漏陷阱:");

    trait Node {
        fn get_id(&self) -> usize;
    }

    struct TreeNode {
        id: usize,
        // 注意：这里可能导致循环引用
        children: Vec<Rc<dyn Node>>,
    }

    impl Node for TreeNode {
        fn get_id(&self) -> usize {
            self.id
        }
    }

    // 正确的方式：使用 Weak 引用避免循环
    use std::rc::Weak;

    struct SafeTreeNode {
        id: usize,
        children: Vec<Rc<dyn Node>>,
        parent: Option<Weak<dyn Node>>,
    }

    impl Node for SafeTreeNode {
        fn get_id(&self) -> usize {
            self.id
        }
    }

    let root = Rc::new(TreeNode {
        id: 1,
        children: vec![],
    });

    println!("创建了安全的树节点，ID: {}", root.get_id());

    // 9.5 类型擦除陷阱
    println!("\n9.5 类型擦除陷阱:");

    trait Identifiable {
        fn get_type_name(&self) -> &'static str;
    }

    struct TypeA2;
    struct TypeB2;

    impl Identifiable for TypeA2 {
        fn get_type_name(&self) -> &'static str {
            "TypeA2"
        }
    }

    impl Identifiable for TypeB2 {
        fn get_type_name(&self) -> &'static str {
            "TypeB2"
        }
    }

    // 类型信息丢失
    let objects: Vec<Box<dyn Identifiable>> = vec![Box::new(TypeA2), Box::new(TypeB2)];

    println!("类型擦除后的对象:");
    for obj in &objects {
        println!("类型: {}", obj.get_type_name());
        // 无法直接转换回原始类型，除非使用 Any trait
    }
}

/// 10. 最佳实践和设计指南
fn best_practices() {
    println!("\n\n10. === 最佳实践和设计指南 ===");

    // 10.1 何时使用特征对象
    println!("\n10.1 何时使用特征对象:");
    println!("✅ 适合使用特征对象的场景:");
    println!("- 需要在运行时处理不同类型的对象");
    println!("- 实现插件系统或扩展机制");
    println!("- 构建状态机或策略模式");
    println!("- 创建异构集合");

    println!("\n❌ 不适合使用特征对象的场景:");
    println!("- 性能是关键因素且类型在编译时已知");
    println!("- 需要频繁调用简单方法");
    println!("- 特征包含泛型方法或返回 Self");

    // 10.2 设计对象安全的特征
    println!("\n10.2 设计对象安全的特征:");

    trait WellDesignedTrait {
        // ✅ 好的设计：简单的方法
        fn process(&self);

        // ✅ 好的设计：返回具体类型
        fn get_result(&self) -> String;

        // ✅ 好的设计：接受引用参数
        fn configure(&mut self, config: &str);

        // ✅ 好的设计：使用 Sized 约束排除不安全方法
        fn clone_self(&self) -> Self
        where
            Self: Sized + Clone,
        {
            self.clone()
        }

        // ✅ 好的设计：提供对象安全的替代方法
        fn clone_boxed(&self) -> Box<dyn WellDesignedTrait>;
    }

    #[derive(Clone)]
    struct GoodImplementation {
        data: String,
    }

    impl WellDesignedTrait for GoodImplementation {
        fn process(&self) {
            println!("处理数据: {}", self.data);
        }

        fn get_result(&self) -> String {
            format!("结果: {}", self.data)
        }

        fn configure(&mut self, config: &str) {
            self.data = config.to_string();
        }

        fn clone_boxed(&self) -> Box<dyn WellDesignedTrait> {
            Box::new(self.clone())
        }
    }

    let mut impl_obj = GoodImplementation {
        data: "初始数据".to_string(),
    };

    // 演示良好设计的使用
    let trait_obj: &mut dyn WellDesignedTrait = &mut impl_obj;
    trait_obj.configure("新配置");
    trait_obj.process();
    println!("{}", trait_obj.get_result());

    // 10.3 性能优化技巧
    println!("\n10.3 性能优化技巧:");

    // 技巧1：使用枚举代替特征对象（当类型数量有限时）
    enum ProcessorType {
        Fast(FastProcessor),
        Slow(SlowProcessor),
    }

    struct FastProcessor {
        multiplier: i32,
    }

    struct SlowProcessor {
        operations: Vec<String>,
    }

    impl ProcessorType {
        fn process(&self, input: i32) -> i32 {
            match self {
                ProcessorType::Fast(p) => input * p.multiplier,
                ProcessorType::Slow(p) => {
                    // 模拟复杂处理
                    input + p.operations.len() as i32
                }
            }
        }
    }

    println!("使用枚举代替特征对象可以获得更好的性能");

    let processors = vec![
        ProcessorType::Fast(FastProcessor { multiplier: 2 }),
        ProcessorType::Slow(SlowProcessor {
            operations: vec!["op1".to_string(), "op2".to_string()],
        }),
    ];

    for (i, processor) in processors.iter().enumerate() {
        let result = processor.process(10);
        println!("处理器 {} 结果: {}", i + 1, result);
    }

    // 技巧2：批量处理减少虚函数调用
    trait BatchProcessor {
        fn process_batch(&self, items: &[i32]) -> Vec<i32>;
    }

    struct EfficientProcessor;

    impl BatchProcessor for EfficientProcessor {
        fn process_batch(&self, items: &[i32]) -> Vec<i32> {
            items.iter().map(|x| x * 2).collect()
        }
    }

    let processor: Box<dyn BatchProcessor> = Box::new(EfficientProcessor);
    let data = vec![1, 2, 3, 4, 5];
    let results = processor.process_batch(&data);
    println!("批量处理结果: {:?}", results);

    // 10.4 错误处理最佳实践
    println!("\n10.4 错误处理最佳实践:");

    trait SafeProcessor {
        type Error;
        fn safe_process(&self, input: &str) -> Result<String, Self::Error>;
    }

    #[derive(Debug)]
    enum ProcessError {
        InvalidInput,
        ProcessingFailed,
    }

    struct ValidatingProcessor;

    impl SafeProcessor for ValidatingProcessor {
        type Error = ProcessError;

        fn safe_process(&self, input: &str) -> Result<String, Self::Error> {
            if input.is_empty() {
                Err(ProcessError::InvalidInput)
            } else if input.len() > 100 {
                Err(ProcessError::ProcessingFailed)
            } else {
                Ok(format!("处理完成: {}", input))
            }
        }
    }

    let processor = ValidatingProcessor;

    match processor.safe_process("测试数据") {
        Ok(result) => println!("成功: {}", result),
        Err(e) => println!("错误: {:?}", e),
    }

    match processor.safe_process("") {
        Ok(result) => println!("成功: {}", result),
        Err(e) => println!("错误: {:?}", e),
    }

    // 10.5 文档和测试建议
    println!("\n10.5 文档和测试建议:");
    println!("📝 文档建议:");
    println!("- 明确说明特征是否对象安全");
    println!("- 提供使用示例和性能注意事项");
    println!("- 说明生命周期要求");

    println!("\n🧪 测试建议:");
    println!("- 测试不同实现的多态行为");
    println!("- 验证对象安全性");
    println!("- 进行性能基准测试");

    // 10.6 总结
    println!("\n10.6 特征对象使用总结:");
    println!("\n🎯 核心原则:");
    println!("1. 优先考虑静态分发，必要时使用动态分发");
    println!("2. 设计对象安全的特征接口");
    println!("3. 注意性能影响，适当优化");
    println!("4. 合理处理生命周期和所有权");
    println!("5. 提供清晰的文档和示例");

    println!("\n🚀 高级技巧:");
    println!("- 使用枚举替代简单的特征对象");
    println!("- 批量处理减少虚函数调用开销");
    println!("- 结合 Any trait 实现类型恢复");
    println!("- 使用多特征约束增强表达能力");
    println!("- 合理使用 Send + Sync 实现线程安全");
}

// 补充：特征对象的内存布局演示
#[allow(dead_code)]
fn memory_layout_demo() {
    println!("\n\n=== 特征对象内存布局演示 ===");

    trait Demo {
        fn demo_method(&self);
    }

    struct DemoStruct {
        value: i32,
    }

    impl Demo for DemoStruct {
        fn demo_method(&self) {
            println!("Demo method called with value: {}", self.value);
        }
    }

    let demo_obj = DemoStruct { value: 42 };
    let trait_obj: &dyn Demo = &demo_obj;

    println!("具体类型大小: {} 字节", std::mem::size_of::<DemoStruct>());
    println!("特征对象大小: {} 字节", std::mem::size_of_val(&trait_obj));

    trait_obj.demo_method();
}
