//! # Rust Sized Trait 和动态大小类型 (DST) 完全教程
//!
//! 本教程深入探讨 Rust 中的 Sized trait 和动态大小类型 (DST)，
//! 涵盖所有相关概念、使用场景和最佳实践。

use std::fmt::Display;
use std::mem;

fn main() {
    println!("=== Rust Sized Trait 和 DST 完全教程 ===");
    println!();

    // 1. 基础概念演示
    demonstrate_basic_concepts();

    // 2. Sized trait 详解
    demonstrate_sized_trait();

    // 3. DST 类型详解
    demonstrate_dst_types();

    // 4. ?Sized 约束
    demonstrate_unsized_constraints();

    // 5. 智能指针与 DST
    demonstrate_smart_pointers_with_dst();

    // 6. Trait 对象详解
    demonstrate_trait_objects();

    // 7. 切片操作
    demonstrate_slice_operations();

    // 8. 高级模式
    demonstrate_advanced_patterns();

    // 9. 性能分析
    demonstrate_performance_analysis();

    // 10. 实际应用场景
    demonstrate_practical_scenarios();

    println!("\n=== 教程总结 ===");
    println!("✅ Sized trait 是 Rust 类型系统的核心，标识编译时已知大小的类型");
    println!("✅ DST 类型包括 str、[T] 和 dyn Trait，只能通过引用或智能指针使用");
    println!("✅ ?Sized 约束允许函数接受 DST 类型，提供更大的灵活性");
    println!("✅ 理解这些概念对于编写高效、安全的 Rust 代码至关重要");
}

// ============================================================================
// 1. 基础概念演示
// ============================================================================

fn demonstrate_basic_concepts() {
    println!("1. 基础概念演示");
    println!("=================");

    // 1.1 固定大小类型 (Sized Types)
    println!("\n1.1 固定大小类型示例:");
    let number: i32 = 42;
    let tuple: (i32, f64) = (1, 2.0);
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("i32 大小: {} 字节", mem::size_of::<i32>());
    println!("(i32, f64) 大小: {} 字节", mem::size_of::<(i32, f64)>());
    println!("[i32; 5] 大小: {} 字节", mem::size_of::<[i32; 5]>());

    // 1.2 动态大小类型 (DST) 的限制
    println!("\n1.2 DST 类型的限制:");

    // 以下代码会编译错误，因为 str 是 DST
    // let s: str = "hello";  // ❌ 编译错误：str 没有已知大小

    // 正确的方式：通过引用使用 DST
    let s: &str = "hello"; // ✅ &str 有固定大小
    let slice: &[i32] = &[1, 2, 3]; // ✅ &[i32] 有固定大小

    println!("&str 大小: {} 字节", mem::size_of::<&str>());
    println!("&[i32] 大小: {} 字节", mem::size_of::<&[i32]>());

    // 1.3 胖指针 (Fat Pointers)
    println!("\n1.3 胖指针分析:");
    println!("普通引用 &i32 大小: {} 字节", mem::size_of::<&i32>());
    println!(
        "字符串切片 &str 大小: {} 字节 (指针 + 长度)",
        mem::size_of::<&str>()
    );
    println!(
        "数组切片 &[i32] 大小: {} 字节 (指针 + 长度)",
        mem::size_of::<&[i32]>()
    );

    // 胖指针包含额外的元数据
    let fat_ptr: &str = "Hello, Rust!";
    println!("字符串内容: {}", fat_ptr);
    println!("字符串长度: {} 字节", fat_ptr.len());
}

// ============================================================================
// 2. Sized Trait 详解
// ============================================================================

fn demonstrate_sized_trait() {
    println!("\n\n2. Sized Trait 详解");
    println!("===================");

    // 2.1 自动实现 Sized
    println!("\n2.1 自动实现 Sized 的类型:");

    struct Point {
        x: f64,
        y: f64,
    }

    enum Color {
        Red,
        Green,
        Blue,
    }

    println!("Point 大小: {} 字节", mem::size_of::<Point>());
    println!("Color 大小: {} 字节", mem::size_of::<Color>());
    println!("Vec<i32> 大小: {} 字节", mem::size_of::<Vec<i32>>());
    println!("String 大小: {} 字节", mem::size_of::<String>());

    // 2.2 泛型函数的默认 Sized 约束
    println!("\n2.2 泛型函数的 Sized 约束:");

    // 这个函数隐式地有 T: Sized 约束
    fn print_size<T>() {
        println!("类型 T 的大小: {} 字节", mem::size_of::<T>());
    }

    print_size::<i32>();
    print_size::<Point>();
    print_size::<Vec<String>>();

    // 2.3 显式 Sized 约束
    println!("\n2.3 显式 Sized 约束示例:");

    fn create_on_stack<T: Sized>(value: T) -> T {
        // 只有 Sized 类型才能在栈上创建
        value
    }

    let point = create_on_stack(Point { x: 1.0, y: 2.0 });
    println!("创建的点: ({}, {})", point.x, point.y);

    // 2.4 检查类型是否实现了 Sized
    println!("\n2.4 类型的 Sized 实现检查:");

    fn is_sized<T: Sized>() -> &'static str {
        "这个类型实现了 Sized"
    }

    println!("i32: {}", is_sized::<i32>());
    println!("String: {}", is_sized::<String>());
    // println!("str: {}", is_sized::<str>());  // ❌ 编译错误
}

// ============================================================================
// 3. DST 类型详解
// ============================================================================

fn demonstrate_dst_types() {
    println!("\n\n3. DST 类型详解");
    println!("===============");

    // 3.1 字符串切片 str
    println!("\n3.1 字符串切片 str:");

    let string_literal: &str = "Hello, DST!";
    let string_owned = String::from("Owned string");
    let string_slice: &str = &string_owned[0..5];

    println!("字符串字面量: {}", string_literal);
    println!("字符串切片: {}", string_slice);

    // str 的内存布局分析
    unsafe {
        let ptr = string_literal.as_ptr();
        let len = string_literal.len();
        println!("str 指针地址: {:p}", ptr);
        println!("str 长度: {} 字节", len);
    }

    // 3.2 数组切片 [T]
    println!("\n3.2 数组切片 [T]:");

    let array = [1, 2, 3, 4, 5];
    let slice: &[i32] = &array[1..4];
    let dynamic_slice: &[i32] = &array[..array.len()];

    println!("原数组: {:?}", array);
    println!("切片 [1..4]: {:?}", slice);
    println!("动态切片: {:?}", dynamic_slice);

    // [T] 的内存布局分析
    unsafe {
        let ptr = slice.as_ptr();
        let len = slice.len();
        println!("切片指针地址: {:p}", ptr);
        println!("切片长度: {} 元素", len);
    }

    // 3.3 Trait 对象 dyn Trait
    println!("\n3.3 Trait 对象 dyn Trait:");

    trait Drawable {
        fn draw(&self);
        fn area(&self) -> f64;
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

        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    impl Drawable for Rectangle {
        fn draw(&self) {
            println!("绘制矩形，宽: {}, 高: {}", self.width, self.height);
        }

        fn area(&self) -> f64 {
            self.width * self.height
        }
    }

    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };

    // 创建 trait 对象
    let shapes: Vec<&dyn Drawable> = vec![&circle, &rectangle];

    for shape in shapes {
        shape.draw();
        println!("面积: {:.2}", shape.area());
    }

    // dyn Trait 的内存布局
    println!(
        "\ndyn Drawable 大小: {} 字节 (指针 + vtable)",
        mem::size_of::<&dyn Drawable>()
    );
}

// ============================================================================
// 4. ?Sized 约束演示
// ============================================================================

fn demonstrate_unsized_constraints() {
    println!("\n\n4. ?Sized 约束演示");
    println!("==================");

    // 4.1 接受 DST 类型的函数
    println!("\n4.1 使用 ?Sized 约束的函数:");

    // 不使用 ?Sized 的函数（默认 T: Sized）
    fn print_sized<T: Display>(value: &T) {
        println!("Sized 值: {}", value);
    }

    // 使用 ?Sized 的函数，可以接受 DST
    fn print_maybe_unsized<T: Display + ?Sized>(value: &T) {
        println!("可能 Unsized 的值: {}", value);
    }

    let number = 42;
    let string = String::from("Hello");
    let str_slice: &str = "World";

    // 两个函数都可以处理 Sized 类型
    print_sized(&number);
    print_sized(&string);
    print_maybe_unsized(&number);
    print_maybe_unsized(&string);

    // 只有 ?Sized 函数可以处理 DST
    // print_sized(str_slice);  // ❌ 编译错误
    print_maybe_unsized(str_slice); // ✅ 正确

    // 4.2 泛型结构体中的 ?Sized
    println!("\n4.2 泛型结构体中的 ?Sized:");

    struct Container<T: ?Sized> {
        data: Box<T>,
    }

    impl<T: ?Sized> Container<T> {
        fn new(data: Box<T>) -> Self {
            Container { data }
        }

        fn get(&self) -> &T {
            &self.data
        }
    }

    // 可以存储 DST
    let str_container = Container::new(Box::<str>::from("Hello, DST!"));
    let slice_container = Container::new(vec![1, 2, 3, 4, 5].into_boxed_slice());

    println!("字符串容器: {}", str_container.get());
    println!("切片容器: {:?}", slice_container.get());

    // 4.3 Trait 定义中的 ?Sized
    println!("\n4.3 Trait 定义中的 ?Sized:");

    trait MyTrait {
        // 默认情况下，Self: ?Sized
        fn process(&self);
    }

    impl MyTrait for str {
        fn process(&self) {
            println!("处理字符串: {}", self);
        }
    }

    impl MyTrait for [i32] {
        fn process(&self) {
            println!("处理切片: {:?}", self);
        }
    }

    let s: &str = "test";
    let slice: &[i32] = &[1, 2, 3];

    s.process();
    slice.process();
}

// ============================================================================
// 5. 智能指针与 DST
// ============================================================================

fn demonstrate_smart_pointers_with_dst() {
    println!("\n\n5. 智能指针与 DST");
    println!("==================");

    // 5.1 Box<T> 与 DST
    println!("\n5.1 Box<T> 与 DST:");

    // Box 可以存储 DST
    let boxed_str: Box<str> = Box::from("Boxed string");
    let boxed_slice: Box<[i32]> = vec![1, 2, 3, 4].into_boxed_slice();

    println!("Box<str>: {}", boxed_str);
    println!("Box<[i32]>: {:?}", boxed_slice);

    // Box<str> 和 Box<[T]> 的大小
    println!("Box<str> 大小: {} 字节", mem::size_of::<Box<str>>());
    println!("Box<[i32]> 大小: {} 字节", mem::size_of::<Box<[i32]>>());

    // 5.2 Rc<T> 与 DST
    println!("\n5.2 Rc<T> 与 DST:");

    use std::rc::Rc;

    let rc_str: Rc<str> = Rc::from("Reference counted string");
    let rc_str_clone = rc_str.clone();

    println!("Rc<str>: {}", rc_str);
    println!("Rc<str> 克隆: {}", rc_str_clone);
    println!("引用计数: {}", Rc::strong_count(&rc_str));

    // 5.3 Arc<T> 与 DST（线程安全）
    println!("\n5.3 Arc<T> 与 DST:");

    use std::sync::Arc;
    use std::thread;

    let arc_str: Arc<str> = Arc::from("Thread-safe string");
    let arc_str_clone = arc_str.clone();

    let handle = thread::spawn(move || {
        println!("线程中的 Arc<str>: {}", arc_str_clone);
    });

    println!("主线程中的 Arc<str>: {}", arc_str);
    handle.join().unwrap();
}

// ============================================================================
// 6. Trait 对象详解
// ============================================================================

fn demonstrate_trait_objects() {
    println!("\n\n6. Trait 对象详解");
    println!("=================");

    // 6.1 对象安全的 Trait
    println!("\n6.1 对象安全的 Trait:");

    trait ObjectSafe {
        fn method(&self);
        fn another_method(&self, x: i32) -> String;
    }

    struct TypeA;
    struct TypeB;

    impl ObjectSafe for TypeA {
        fn method(&self) {
            println!("TypeA 的方法");
        }

        fn another_method(&self, x: i32) -> String {
            format!("TypeA 处理: {}", x)
        }
    }

    impl ObjectSafe for TypeB {
        fn method(&self) {
            println!("TypeB 的方法");
        }

        fn another_method(&self, x: i32) -> String {
            format!("TypeB 处理: {}", x * 2)
        }
    }

    // 创建 trait 对象
    let objects: Vec<Box<dyn ObjectSafe>> = vec![Box::new(TypeA), Box::new(TypeB)];

    for (i, obj) in objects.iter().enumerate() {
        println!("对象 {}:", i);
        obj.method();
        println!("结果: {}", obj.another_method(10));
    }

    // 6.2 虚函数表 (vtable) 分析
    println!("\n6.2 虚函数表分析:");

    let type_a = TypeA;
    let trait_obj: &dyn ObjectSafe = &type_a;

    // trait 对象包含数据指针和 vtable 指针
    unsafe {
        let raw_parts: (*const (), *const ()) = std::mem::transmute(trait_obj);
        println!("数据指针: {:p}", raw_parts.0);
        println!("vtable 指针: {:p}", raw_parts.1);
    }

    // 6.3 动态分发 vs 静态分发
    println!("\n6.3 动态分发 vs 静态分发:");

    // 静态分发（单态化）
    fn static_dispatch<T: ObjectSafe>(obj: &T) {
        obj.method(); // 编译时确定调用哪个方法
    }

    // 动态分发
    fn dynamic_dispatch(obj: &dyn ObjectSafe) {
        obj.method(); // 运行时通过 vtable 确定调用哪个方法
    }

    let type_a = TypeA;
    let type_b = TypeB;

    println!("静态分发:");
    static_dispatch(&type_a);
    static_dispatch(&type_b);

    println!("动态分发:");
    dynamic_dispatch(&type_a);
    dynamic_dispatch(&type_b);
}

// ============================================================================
// 7. 切片操作详解
// ============================================================================

fn demonstrate_slice_operations() {
    println!("\n\n7. 切片操作详解");
    println!("===============");

    // 7.1 基本切片操作
    println!("\n7.1 基本切片操作:");

    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let full_slice = &array[..];
    let partial_slice = &array[2..7];
    let from_start = &array[..5];
    let to_end = &array[5..];

    println!("原数组: {:?}", array);
    println!("完整切片: {:?}", full_slice);
    println!("部分切片 [2..7]: {:?}", partial_slice);
    println!("从开始 [..5]: {:?}", from_start);
    println!("到结尾 [5..]: {:?}", to_end);

    // 7.2 可变切片
    println!("\n7.2 可变切片操作:");

    let mut mutable_array = [1, 2, 3, 4, 5];
    println!("修改前: {:?}", mutable_array);

    {
        let mutable_slice = &mut mutable_array[1..4];
        for item in mutable_slice.iter_mut() {
            *item *= 2;
        }
    }

    println!("修改后: {:?}", mutable_array);

    // 7.3 字符串切片
    println!("\n7.3 字符串切片操作:");

    let text = "Hello, 世界! 🦀";

    // 注意：字符串切片必须在 UTF-8 字符边界上
    let hello = &text[0..5];
    let world_start = text.find('世').unwrap();
    let world = &text[world_start..world_start + 6]; // "世界" 占 6 字节

    println!("原文本: {}", text);
    println!("Hello 部分: {}", hello);
    println!("世界 部分: {}", world);

    // 安全的字符串切片
    if let Some(comma_pos) = text.find(',') {
        let before_comma = &text[..comma_pos];
        println!("逗号前: {}", before_comma);
    }

    // 7.4 切片模式匹配
    println!("\n7.4 切片模式匹配:");

    fn analyze_slice(slice: &[i32]) {
        match slice {
            [] => println!("空切片"),
            [x] => println!("单元素切片: {}", x),
            [x, y] => println!("双元素切片: {}, {}", x, y),
            [first, .., last] => println!("多元素切片，首: {}, 尾: {}", first, last),
        }
    }

    analyze_slice(&[]);
    analyze_slice(&[42]);
    analyze_slice(&[1, 2]);
    analyze_slice(&[1, 2, 3, 4, 5]);
}

// ============================================================================
// 8. 高级模式
// ============================================================================

fn demonstrate_advanced_patterns() {
    println!("\n\n8. 高级模式");
    println!("============");

    // 8.1 自定义 DST
    println!("\n8.1 自定义 DST 类型:");

    // 创建一个自定义的 DST：变长结构体
    #[repr(C)]
    struct VariableLengthStruct {
        len: usize,
        data: [u8], // 不定长字段必须是最后一个
    }

    impl VariableLengthStruct {
        fn new(data: &[u8]) -> Box<Self> {
            let len = data.len();
            let layout = std::alloc::Layout::from_size_align(
                mem::size_of::<usize>() + len,
                mem::align_of::<usize>(),
            )
            .unwrap();

            unsafe {
                let ptr = std::alloc::alloc(layout);
                let header_ptr = ptr as *mut usize;
                header_ptr.write(len);

                let data_ptr = ptr.add(mem::size_of::<usize>());
                std::ptr::copy_nonoverlapping(data.as_ptr(), data_ptr, len);

                let fat_ptr = std::ptr::slice_from_raw_parts_mut(ptr as *mut (), len) as *mut Self;
                Box::from_raw(fat_ptr)
            }
        }

        fn data(&self) -> &[u8] {
            &self.data
        }

        fn len(&self) -> usize {
            self.len
        }
    }

    let custom_dst = VariableLengthStruct::new(b"Hello, Custom DST!");
    println!("自定义 DST 长度: {}", custom_dst.len());
    println!(
        "自定义 DST 数据: {:?}",
        std::str::from_utf8(custom_dst.data()).unwrap()
    );

    // 8.2 零成本抽象
    println!("\n8.2 零成本抽象示例:");

    trait ZeroCostTrait {
        fn process(&self) -> usize;
    }

    impl ZeroCostTrait for str {
        fn process(&self) -> usize {
            self.len()
        }
    }

    impl ZeroCostTrait for [u8] {
        fn process(&self) -> usize {
            self.len()
        }
    }

    fn zero_cost_function<T: ZeroCostTrait + ?Sized>(data: &T) -> usize {
        data.process() // 编译时单态化，无运行时开销
    }

    let text: &str = "Zero cost abstraction";
    let bytes: &[u8] = b"byte array";

    println!("文本处理结果: {}", zero_cost_function(text));
    println!("字节处理结果: {}", zero_cost_function(bytes));

    // 8.3 内存布局分析
    println!("\n8.3 内存布局分析:");

    fn analyze_memory_layout<T: ?Sized>(name: &str, value: &T) {
        let ptr = value as *const T;
        let size = mem::size_of_val(value);
        let align = mem::align_of_val(value);

        println!(
            "{} - 地址: {:p}, 大小: {} 字节, 对齐: {} 字节",
            name, ptr, size, align
        );
    }

    let number = 42i32;
    let text = "Hello";
    let array = [1, 2, 3, 4, 5];
    let slice = &array[1..4];

    analyze_memory_layout("i32", &number);
    analyze_memory_layout("&str", &text);
    analyze_memory_layout("[i32; 5]", &array);
    analyze_memory_layout("&[i32]", &slice);
}

// ============================================================================
// 9. 性能分析
// ============================================================================

fn demonstrate_performance_analysis() {
    println!("\n\n9. 性能分析");
    println!("============");

    // 9.1 编译时 vs 运行时大小确定
    println!("\n9.1 编译时 vs 运行时大小确定:");

    // 编译时已知大小
    struct CompileTimeKnown {
        a: i32,
        b: f64,
        c: [u8; 16],
    }

    println!(
        "编译时已知大小的结构体: {} 字节",
        mem::size_of::<CompileTimeKnown>()
    );

    // 运行时确定大小
    let runtime_slice: &[i32] = &[1, 2, 3, 4, 5][..3];
    println!(
        "运行时确定大小的切片: {} 字节",
        mem::size_of_val(runtime_slice)
    );

    // 9.2 内存开销对比
    println!("\n9.2 内存开销对比:");

    // 普通引用 vs 胖指针
    let number = 42i32;
    let number_ref: &i32 = &number;
    let str_ref: &str = "hello";
    let slice_ref: &[i32] = &[1, 2, 3];

    println!("普通引用 &i32: {} 字节", mem::size_of_val(&number_ref));
    println!("字符串引用 &str: {} 字节", mem::size_of_val(&str_ref));
    println!("切片引用 &[i32]: {} 字节", mem::size_of_val(&slice_ref));

    // 9.3 动态分发开销
    println!("\n9.3 动态分发开销分析:");

    trait Benchmark {
        fn compute(&self) -> i32;
    }

    struct FastType;
    struct SlowType;

    impl Benchmark for FastType {
        fn compute(&self) -> i32 {
            42 // 简单计算
        }
    }

    impl Benchmark for SlowType {
        fn compute(&self) -> i32 {
            (0..1000).sum() // 复杂计算
        }
    }

    // 静态分发（无额外开销）
    fn static_benchmark<T: Benchmark>(obj: &T) -> i32 {
        obj.compute()
    }

    // 动态分发（有 vtable 查找开销）
    fn dynamic_benchmark(obj: &dyn Benchmark) -> i32 {
        obj.compute()
    }

    let fast = FastType;
    let slow = SlowType;

    println!("静态分发结果: {}", static_benchmark(&fast));
    println!("动态分发结果: {}", dynamic_benchmark(&fast));

    // 9.4 内存对齐影响
    println!("\n9.4 内存对齐影响:");

    #[repr(C)]
    struct Aligned {
        a: u8,  // 1 字节
        b: u32, // 4 字节，需要对齐
        c: u8,  // 1 字节
    }

    #[repr(C, packed)]
    struct Packed {
        a: u8,  // 1 字节
        b: u32, // 4 字节，紧密排列
        c: u8,  // 1 字节
    }

    println!("对齐结构体大小: {} 字节", mem::size_of::<Aligned>());
    println!("紧密结构体大小: {} 字节", mem::size_of::<Packed>());
}

// ============================================================================
// 10. 实际应用场景
// ============================================================================

fn demonstrate_practical_scenarios() {
    println!("\n\n10. 实际应用场景");
    println!("=================");

    // 10.1 通用容器设计
    println!("\n10.1 通用容器设计:");

    struct FlexibleContainer<T: ?Sized> {
        data: Box<T>,
        metadata: String,
    }

    impl<T: ?Sized> FlexibleContainer<T> {
        fn new(data: Box<T>, metadata: String) -> Self {
            FlexibleContainer { data, metadata }
        }

        fn get_data(&self) -> &T {
            &self.data
        }

        fn get_metadata(&self) -> &str {
            &self.metadata
        }
    }

    // 可以存储各种类型，包括 DST
    let string_container = FlexibleContainer::new(
        Box::<str>::from("Hello, Container!"),
        "字符串数据".to_string(),
    );

    let slice_container = FlexibleContainer::new(
        vec![1, 2, 3, 4, 5].into_boxed_slice(),
        "数组数据".to_string(),
    );

    println!(
        "容器1 - 元数据: {}, 数据: {}",
        string_container.get_metadata(),
        string_container.get_data()
    );
    println!(
        "容器2 - 元数据: {}, 数据: {:?}",
        slice_container.get_metadata(),
        slice_container.get_data()
    );

    // 10.2 插件系统
    println!("\n10.2 插件系统设计:");

    trait Plugin {
        fn name(&self) -> &str;
        fn execute(&self, input: &str) -> String;
    }

    struct UppercasePlugin;
    struct ReversePlugin;

    impl Plugin for UppercasePlugin {
        fn name(&self) -> &str {
            "大写转换插件"
        }

        fn execute(&self, input: &str) -> String {
            input.to_uppercase()
        }
    }

    impl Plugin for ReversePlugin {
        fn name(&self) -> &str {
            "字符串反转插件"
        }

        fn execute(&self, input: &str) -> String {
            input.chars().rev().collect()
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
            self.plugins.push(plugin);
        }

        fn execute_all(&self, input: &str) -> Vec<String> {
            self.plugins
                .iter()
                .map(|plugin| {
                    println!("执行插件: {}", plugin.name());
                    plugin.execute(input)
                })
                .collect()
        }
    }

    let mut manager = PluginManager::new();
    manager.register_plugin(Box::new(UppercasePlugin));
    manager.register_plugin(Box::new(ReversePlugin));

    let input = "Hello, Plugin System!";
    let results = manager.execute_all(input);

    println!("输入: {}", input);
    for (i, result) in results.iter().enumerate() {
        println!("插件 {} 结果: {}", i + 1, result);
    }

    // 10.3 序列化框架
    println!("\n10.3 序列化框架示例:");

    trait Serializable {
        fn serialize(&self) -> Vec<u8>;
    }

    impl Serializable for str {
        fn serialize(&self) -> Vec<u8> {
            let mut result = Vec::new();
            result.extend_from_slice(&(self.len() as u32).to_le_bytes());
            result.extend_from_slice(self.as_bytes());
            result
        }
    }

    impl Serializable for [i32] {
        fn serialize(&self) -> Vec<u8> {
            let mut result = Vec::new();
            result.extend_from_slice(&(self.len() as u32).to_le_bytes());
            for &item in self {
                result.extend_from_slice(&item.to_le_bytes());
            }
            result
        }
    }

    fn serialize_data<T: Serializable + ?Sized>(data: &T) -> Vec<u8> {
        data.serialize()
    }

    let text: &str = "序列化测试";
    let numbers: &[i32] = &[1, 2, 3, 4, 5];

    let serialized_text = serialize_data(text);
    let serialized_numbers = serialize_data(numbers);

    println!("文本序列化: {:?}", serialized_text);
    println!("数字序列化: {:?}", serialized_numbers);
}

// ============================================================================
// 测试模块
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sized_trait() {
        // 测试 Sized 类型
        assert_eq!(mem::size_of::<i32>(), 4);
        assert_eq!(mem::size_of::<String>(), 24); // 在 64 位系统上

        // 测试胖指针
        assert_eq!(mem::size_of::<&str>(), 16); // 指针 + 长度
        assert_eq!(mem::size_of::<&[i32]>(), 16); // 指针 + 长度
    }

    #[test]
    fn test_dst_operations() {
        // 测试字符串切片
        let s = "Hello, World!";
        assert_eq!(s.len(), 13);
        assert_eq!(&s[0..5], "Hello");

        // 测试数组切片
        let arr = [1, 2, 3, 4, 5];
        let slice = &arr[1..4];
        assert_eq!(slice, &[2, 3, 4]);
    }

    #[test]
    fn test_unsized_constraints() {
        fn accepts_unsized<T: Display + ?Sized>(value: &T) -> String {
            format!("{}", value)
        }

        // 测试 Sized 类型
        assert_eq!(accepts_unsized(&42), "42");

        // 测试 DST 类型
        assert_eq!(accepts_unsized("hello"), "hello");
    }

    #[test]
    fn test_smart_pointers_with_dst() {
        // 测试 Box<str>
        let boxed_str: Box<str> = Box::from("boxed");
        assert_eq!(&*boxed_str, "boxed");

        // 测试 Box<[i32]>
        let boxed_slice: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();
        assert_eq!(&*boxed_slice, &[1, 2, 3]);
    }

    #[test]
    fn test_trait_objects() {
        trait TestTrait {
            fn test_method(&self) -> i32;
        }

        struct TestStruct(i32);

        impl TestTrait for TestStruct {
            fn test_method(&self) -> i32 {
                self.0
            }
        }

        let obj: Box<dyn TestTrait> = Box::new(TestStruct(42));
        assert_eq!(obj.test_method(), 42);
    }
}
