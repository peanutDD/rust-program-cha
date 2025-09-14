//! # Copy Trait 深度解析
//!
//! Copy trait 是 Rust 类型系统中的一个重要概念，它允许类型在赋值时进行按位复制而不是移动。
//! 本模块将深入解析 Copy trait 的工作原理、实现条件和最佳实践。

use std::marker::PhantomData;
use std::ptr::NonNull;

/// Copy trait 的基本概念演示
pub fn basic_copy_concepts() {
    println!("=== Copy Trait 基本概念 ===");
    
    // 1. 基本类型的 Copy 行为
    println!("\n1. 基本类型的 Copy 行为:");
    let x = 42;
    let y = x; // Copy 发生，x 仍然有效
    println!("x: {}, y: {}", x, y); // 两个变量都可以使用
    
    let a = 3.14;
    let b = a; // f64 实现了 Copy
    println!("a: {}, b: {}", a, b);
    
    let flag1 = true;
    let flag2 = flag1; // bool 实现了 Copy
    println!("flag1: {}, flag2: {}", flag1, flag2);
    
    // 2. 字符类型的 Copy
    println!("\n2. 字符类型的 Copy:");
    let ch1 = 'A';
    let ch2 = ch1; // char 实现了 Copy
    println!("ch1: {}, ch2: {}", ch1, ch2);
    
    // 3. 数组的 Copy（如果元素类型实现了 Copy）
    println!("\n3. 数组的 Copy:");
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = arr1; // [i32; 5] 实现了 Copy
    println!("arr1: {:?}, arr2: {:?}", arr1, arr2);
    
    // 4. 元组的 Copy（如果所有元素都实现了 Copy）
    println!("\n4. 元组的 Copy:");
    let tuple1 = (10, 20.5, 'X');
    let tuple2 = tuple1; // (i32, f64, char) 实现了 Copy
    println!("tuple1: {:?}, tuple2: {:?}", tuple1, tuple2);
}

/// Copy trait 的实现条件分析
pub fn copy_implementation_conditions() {
    println!("\n=== Copy Trait 实现条件分析 ===");
    
    // 1. 可以实现 Copy 的类型
    println!("\n1. 可以实现 Copy 的类型:");
    demonstrate_copyable_types();
    
    // 2. 不能实现 Copy 的类型
    println!("\n2. 不能实现 Copy 的类型:");
    demonstrate_non_copyable_types();
    
    // 3. 自定义类型的 Copy 实现
    println!("\n3. 自定义类型的 Copy 实现:");
    demonstrate_custom_copy_types();
}

/// 演示可以实现 Copy 的类型
fn demonstrate_copyable_types() {
    // 简单的值类型结构体
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1; // Copy 发生
    println!("p1: {:?}, p2: {:?}", p1, p2);
    
    // 包含 Copy 类型的结构体
    #[derive(Debug, Copy, Clone)]
    struct Color {
        r: u8,
        g: u8,
        b: u8,
        a: f32,
    }
    
    let color1 = Color { r: 255, g: 128, b: 64, a: 1.0 };
    let color2 = color1; // Copy 发生
    println!("color1: {:?}, color2: {:?}", color1, color2);
    
    // 枚举类型的 Copy
    #[derive(Debug, Copy, Clone)]
    enum Direction {
        North,
        South,
        East,
        West,
    }
    
    let dir1 = Direction::North;
    let dir2 = dir1; // Copy 发生
    println!("dir1: {:?}, dir2: {:?}", dir1, dir2);
    
    // 包含数据的枚举（所有变体都是 Copy 类型）
    #[derive(Debug, Copy, Clone)]
    enum Status {
        Active(u32),
        Inactive,
        Pending(bool),
    }
    
    let status1 = Status::Active(42);
    let status2 = status1; // Copy 发生
    println!("status1: {:?}, status2: {:?}", status1, status2);
}

/// 演示不能实现 Copy 的类型
fn demonstrate_non_copyable_types() {
    println!("以下类型不能实现 Copy:");
    
    // 1. 包含 String 的结构体
    #[derive(Debug, Clone)] // 不能 derive Copy
    struct Person {
        name: String, // String 不实现 Copy
        age: u32,
    }
    
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };
    let person2 = person1.clone(); // 必须显式 clone
    // let person3 = person1; // 这会导致 move
    println!("person2: {:?}", person2);
    
    // 2. 包含 Vec 的结构体
    #[derive(Debug, Clone)]
    struct Container {
        items: Vec<i32>, // Vec 不实现 Copy
    }
    
    let container1 = Container {
        items: vec![1, 2, 3],
    };
    let container2 = container1.clone();
    println!("container2: {:?}", container2);
    
    // 3. 包含 Box 的结构体
    #[derive(Debug, Clone)]
    struct BoxedData {
        data: Box<i32>, // Box 不实现 Copy
    }
    
    let boxed1 = BoxedData {
        data: Box::new(42),
    };
    let boxed2 = boxed1.clone();
    println!("boxed2: {:?}", boxed2);
    
    println!("这些类型涉及堆内存分配，不能简单按位复制");
}

/// 演示自定义类型的 Copy 实现
fn demonstrate_custom_copy_types() {
    // 1. 手动实现 Copy 和 Clone
    #[derive(Debug)]
    struct CustomPoint {
        x: f64,
        y: f64,
    }
    
    impl Copy for CustomPoint {}
    
    impl Clone for CustomPoint {
        fn clone(&self) -> Self {
            println!("CustomPoint 正在被克隆");
            *self // Copy 语义
        }
    }
    
    let cp1 = CustomPoint { x: 1.0, y: 2.0 };
    let cp2 = cp1; // Copy 发生，不会调用 clone
    let cp3 = cp1.clone(); // 显式调用 clone
    println!("cp1: {:?}, cp2: {:?}, cp3: {:?}", cp1, cp2, cp3);
    
    // 2. 包含 PhantomData 的类型
    #[derive(Debug, Copy, Clone)]
    struct TypedId<T> {
        id: u64,
        _phantom: PhantomData<T>,
    }
    
    impl<T> TypedId<T> {
        fn new(id: u64) -> Self {
            TypedId {
                id,
                _phantom: PhantomData,
            }
        }
    }
    
    let user_id1: TypedId<String> = TypedId::new(123);
    let user_id2 = user_id1.clone(); // Clone 发生，因为 TypedId<String> 不实现 Copy
    println!("user_id1: {:?}, user_id2: {:?}", user_id1, user_id2);
    
    // 3. 零大小类型（ZST）
    #[derive(Debug, Copy, Clone)]
    struct Unit;
    
    let unit1 = Unit;
    let unit2 = unit1; // Copy 发生
    println!("unit1: {:?}, unit2: {:?}", unit1, unit2);
    println!("零大小类型的 Copy 是零成本的");
}

/// Copy trait 的内存模型分析
pub fn memory_model_analysis() {
    println!("\n=== Copy Trait 内存模型分析 ===");
    
    // 1. Copy 的内存布局
    println!("\n1. Copy 的内存布局:");
    analyze_copy_memory_layout();
    
    // 2. Copy vs Move 的内存行为
    println!("\n2. Copy vs Move 的内存行为:");
    compare_copy_vs_move_memory();
    
    // 3. 栈上 Copy 的性能
    println!("\n3. 栈上 Copy 的性能:");
    analyze_stack_copy_performance();
}

/// 分析 Copy 的内存布局
fn analyze_copy_memory_layout() {
    #[derive(Debug, Copy, Clone)]
    struct Data {
        a: u32,
        b: u64,
        c: u8,
    }
    
    let data1 = Data { a: 10, b: 20, c: 30 };
    println!("data1 地址: {:p}", &data1);
    println!("data1.a 地址: {:p}", &data1.a);
    println!("data1.b 地址: {:p}", &data1.b);
    println!("data1.c 地址: {:p}", &data1.c);
    
    let data2 = data1; // Copy 发生
    println!("\ndata2 地址: {:p}", &data2);
    println!("data2.a 地址: {:p}", &data2.a);
    println!("data2.b 地址: {:p}", &data2.b);
    println!("data2.c 地址: {:p}", &data2.c);
    
    println!("\nCopy 创建了完全独立的内存副本");
    println!("data1 和 data2 在不同的内存地址");
    
    // 验证独立性
    println!("\n验证数据独立性:");
    println!("data1: {:?}", data1);
    println!("data2: {:?}", data2);
    println!("修改其中一个不会影响另一个");
}

/// 比较 Copy 和 Move 的内存行为
fn compare_copy_vs_move_memory() {
    // Copy 类型
    #[derive(Debug, Copy, Clone)]
    struct CopyType {
        value: i32,
    }
    
    // Move 类型
    #[derive(Debug)]
    struct MoveType {
        value: String,
    }
    
    println!("Copy 类型的行为:");
    let copy1 = CopyType { value: 42 };
    let copy2 = copy1; // Copy 发生
    println!("copy1: {:?}, copy2: {:?}", copy1, copy2);
    println!("两个变量都可以继续使用");
    
    println!("\nMove 类型的行为:");
    let move1 = MoveType { value: String::from("Hello") };
    let move2 = move1; // Move 发生
    println!("move2: {:?}", move2);
    // println!("move1: {:?}", move1); // 编译错误！
    println!("原变量不能再使用");
}

/// 分析栈上 Copy 的性能
fn analyze_stack_copy_performance() {
    use std::time::Instant;
    
    #[derive(Copy, Clone)]
    struct LargeStackData {
        data: [u64; 1000], // 8KB 的栈数据
    }
    
    impl LargeStackData {
        fn new() -> Self {
            LargeStackData {
                data: [42; 1000],
            }
        }
    }
    
    let large_data = LargeStackData::new();
    
    // 测量 Copy 操作的时间
    let start = Instant::now();
    let _copied_data = large_data; // Copy 发生
    let copy_duration = start.elapsed();
    
    println!("大型栈数据 Copy 耗时: {:?}", copy_duration);
    
    // 多次 Copy 测试
    let start = Instant::now();
    for _ in 0..1000 {
        let _temp = large_data; // 每次都 Copy
    }
    let multiple_copy_duration = start.elapsed();
    
    println!("1000 次 Copy 操作耗时: {:?}", multiple_copy_duration);
    println!("平均每次 Copy 耗时: {:?}", multiple_copy_duration / 1000);
    
    println!("\n注意：Copy 的成本与数据大小成正比");
    println!("对于大型数据，Copy 可能比 Move 更昂贵");
}

/// Copy trait 的高级应用
pub fn advanced_copy_patterns() {
    println!("\n=== Copy Trait 高级应用 ===");
    
    // 1. Copy 在泛型中的应用
    println!("\n1. Copy 在泛型中的应用:");
    demonstrate_copy_in_generics();
    
    // 2. Copy 和生命周期
    println!("\n2. Copy 和生命周期:");
    demonstrate_copy_and_lifetimes();
    
    // 3. Copy 的条件实现
    println!("\n3. Copy 的条件实现:");
    demonstrate_conditional_copy();
    
    // 4. Copy 和 unsafe 代码
    println!("\n4. Copy 和 unsafe 代码:");
    demonstrate_copy_with_unsafe();
}

/// 演示 Copy 在泛型中的应用
fn demonstrate_copy_in_generics() {
    // 要求 T 实现 Copy 的泛型函数
    fn duplicate<T: Copy>(value: T) -> (T, T) {
        (value, value) // 可以多次使用 value
    }
    
    let num = 42;
    let (a, b) = duplicate(num);
    println!("原值: {}, 复制值: ({}, {})", num, a, b);
    
    let point = Point2D { x: 1.0, y: 2.0 };
    let (p1, p2) = duplicate(point);
    println!("原点: {:?}, 复制点: ({:?}, {:?})", point, p1, p2);
    
    // 泛型结构体中的 Copy 约束
    #[derive(Debug)]
    struct Pair<T: Copy> {
        first: T,
        second: T,
    }
    
    impl<T: Copy> Pair<T> {
        fn new(value: T) -> Self {
            Pair {
                first: value,
                second: value, // 可以多次使用 value
            }
        }
        
        fn swap(&mut self) {
            let temp = self.first; // Copy 发生
            self.first = self.second;
            self.second = temp;
        }
    }
    
    let mut pair = Pair::new(100);
    println!("交换前: {:?}", pair);
    pair.swap();
    println!("交换后: {:?}", pair);
}

#[derive(Debug, Copy, Clone)]
struct Point2D {
    x: f64,
    y: f64,
}

#[derive(Debug, Copy, Clone)]
struct Event {
    id: u32,
    timestamp: u64,
    event_type: u8,
}

/// 演示 Copy 和生命周期
fn demonstrate_copy_and_lifetimes() {
    // Copy 类型不需要考虑生命周期
    fn process_copy_data<T: Copy>(data: T) -> T {
        // 可以直接返回，不需要考虑借用
        data
    }
    
    let value = 42;
    let result = process_copy_data(value);
    println!("处理结果: {}", result);
    println!("原值仍可用: {}", value);
    
    // 对比：非 Copy 类型需要考虑生命周期
    fn process_borrowed_data(data: &str) -> &str {
        data // 返回借用，需要考虑生命周期
    }
    
    let text = "Hello, World!";
    let borrowed = process_borrowed_data(text);
    println!("借用的文本: {}", borrowed);
}

/// 演示 Copy 的条件实现
fn demonstrate_conditional_copy() {
    // 只有当 T 实现 Copy 时，Wrapper<T> 才实现 Copy
    #[derive(Debug)]
    struct Wrapper<T> {
        value: T,
    }
    
    impl<T: Copy> Copy for Wrapper<T> {}
    
    impl<T: Clone> Clone for Wrapper<T> {
        fn clone(&self) -> Self {
            Wrapper {
                value: self.value.clone(),
            }
        }
    }
    
    // Copy 类型的 Wrapper
    let wrapper1 = Wrapper { value: 42 };
    let wrapper2 = wrapper1; // Copy 发生
    println!("wrapper1: {:?}, wrapper2: {:?}", wrapper1, wrapper2);
    
    // 非 Copy 类型的 Wrapper
    let wrapper3 = Wrapper { value: String::from("Hello") };
    let wrapper4 = wrapper3.clone(); // 必须显式 clone
    // let wrapper5 = wrapper3; // 这会导致 move
    println!("wrapper4: {:?}", wrapper4);
}

/// 演示 Copy 和 unsafe 代码
fn demonstrate_copy_with_unsafe() {
    // 使用 unsafe 实现自定义的 Copy 行为
    #[derive(Debug)]
    struct UnsafeCopy {
        ptr: NonNull<i32>,
    }
    
    unsafe impl Send for UnsafeCopy {}
    unsafe impl Sync for UnsafeCopy {}
    
    // 注意：UnsafeCopy 不能实现 Copy，因为它包含需要特殊处理的资源
    // impl Copy for UnsafeCopy {} // 这会导致编译错误
    
    impl Clone for UnsafeCopy {
        fn clone(&self) -> Self {
            // 注意：这里只是复制指针，不复制指向的数据
            // 在实际使用中需要非常小心
            UnsafeCopy { ptr: self.ptr }
        }
    }
    
    impl Drop for UnsafeCopy {
        fn drop(&mut self) {
            // 注意：Copy 类型不应该实现 Drop
            // 这里只是为了演示，实际代码中应该避免
            println!("UnsafeCopy 被销毁");
        }
    }
    
    // 创建一个堆上的值
    let boxed_value = Box::new(42);
    let raw_ptr = Box::into_raw(boxed_value);
    
    unsafe {
        let unsafe_copy1 = UnsafeCopy {
            ptr: NonNull::new(Box::into_raw(Box::new(42))).unwrap(),
        };
        
        let unsafe_copy2 = unsafe_copy1.clone(); // Clone 发生，因为 UnsafeCopy 不能实现 Copy
        
        println!("unsafe_copy2: {:?}", unsafe_copy2);
        println!("unsafe_copy2: {:?}", unsafe_copy2);
        
        // 清理内存
        let _ = Box::from_raw(raw_ptr);
    }
    
    println!("注意：在 unsafe 代码中使用 Copy 需要格外小心");
}

/// Copy trait 的性能分析
pub fn performance_analysis() {
    println!("\n=== Copy Trait 性能分析 ===");
    
    // 1. 不同大小类型的 Copy 性能
    println!("\n1. 不同大小类型的 Copy 性能:");
    benchmark_copy_sizes();
    
    // 2. Copy vs Clone 性能对比
    println!("\n2. Copy vs Clone 性能对比:");
    compare_copy_vs_clone_performance();
    
    // 3. Copy 在循环中的性能
    println!("\n3. Copy 在循环中的性能:");
    analyze_copy_in_loops();
}

/// 基准测试不同大小类型的 Copy 性能
fn benchmark_copy_sizes() {
    use std::time::Instant;
    
    // 小型数据
    #[derive(Copy, Clone)]
    struct Small {
        data: u32,
    }
    
    // 中型数据
    #[derive(Copy, Clone)]
    struct Medium {
        data: [u32; 16], // 64 bytes
    }
    
    // 大型数据
    #[derive(Copy, Clone)]
    struct Large {
        data: [u32; 256], // 1KB
    }
    
    let small = Small { data: 42 };
    let medium = Medium { data: [42; 16] };
    let large = Large { data: [42; 256] };
    
    const ITERATIONS: usize = 1_000_000;
    
    // 测试小型数据 Copy
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _temp = small;
    }
    let small_duration = start.elapsed();
    
    // 测试中型数据 Copy
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _temp = medium;
    }
    let medium_duration = start.elapsed();
    
    // 测试大型数据 Copy
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _temp = large;
    }
    let large_duration = start.elapsed();
    
    println!("小型数据 (4B) Copy {} 次耗时: {:?}", ITERATIONS, small_duration);
    println!("中型数据 (64B) Copy {} 次耗时: {:?}", ITERATIONS, medium_duration);
    println!("大型数据 (1KB) Copy {} 次耗时: {:?}", ITERATIONS, large_duration);
    
    println!("\n性能比例:");
    println!("中型/小型: {:.2}x", medium_duration.as_nanos() as f64 / small_duration.as_nanos() as f64);
    println!("大型/小型: {:.2}x", large_duration.as_nanos() as f64 / small_duration.as_nanos() as f64);
}

/// 比较 Copy 和 Clone 的性能
fn compare_copy_vs_clone_performance() {
    use std::time::Instant;
    
    #[derive(Copy, Clone)]
    struct Data {
        values: [i32; 100],
    }
    
    let data = Data { values: [42; 100] };
    const ITERATIONS: usize = 100_000;
    
    // 测试 Copy 性能
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _temp = data; // 隐式 Copy
    }
    let copy_duration = start.elapsed();
    
    // 测试显式 Clone 性能
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _temp = data.clone(); // 显式 Clone
    }
    let clone_duration = start.elapsed();
    
    println!("Copy {} 次耗时: {:?}", ITERATIONS, copy_duration);
    println!("Clone {} 次耗时: {:?}", ITERATIONS, clone_duration);
    println!("性能比例 (Clone/Copy): {:.2}x", 
             clone_duration.as_nanos() as f64 / copy_duration.as_nanos() as f64);
    
    println!("\n注意：对于 Copy 类型，Clone 通常会调用 Copy");
    println!("性能差异主要来自函数调用开销");
}

/// 分析 Copy 在循环中的性能
fn analyze_copy_in_loops() {
    use std::time::Instant;
    
    #[derive(Copy, Clone)]
    struct Point3D {
        x: f64,
        y: f64,
        z: f64,
    }
    
    let points: Vec<Point3D> = (0..10000)
        .map(|i| Point3D {
            x: i as f64,
            y: (i * 2) as f64,
            z: (i * 3) as f64,
        })
        .collect();
    
    // 方法1：直接 Copy
    let start = Instant::now();
    let mut sum1 = 0.0;
    for point in &points {
        let p = *point; // Copy 发生
        sum1 += p.x + p.y + p.z;
    }
    let copy_duration = start.elapsed();
    
    // 方法2：使用引用
    let start = Instant::now();
    let mut sum2 = 0.0;
    for point in &points {
        sum2 += point.x + point.y + point.z; // 直接访问，无 Copy
    }
    let reference_duration = start.elapsed();
    
    println!("使用 Copy 的循环耗时: {:?}", copy_duration);
    println!("使用引用的循环耗时: {:?}", reference_duration);
    println!("结果验证: sum1={:.2}, sum2={:.2}", sum1, sum2);
    
    if copy_duration > reference_duration {
        println!("\n在循环中，引用访问比 Copy 更高效");
    } else {
        println!("\n编译器优化使得两种方式性能相近");
    }
}

/// Copy trait 的最佳实践
pub fn best_practices() {
    println!("\n=== Copy Trait 最佳实践 ===");
    
    println!("\n1. 何时实现 Copy:");
    println!("   - 类型的所有字段都实现了 Copy");
    println!("   - 类型不涉及堆内存分配");
    println!("   - 类型的复制成本较低");
    println!("   - 类型的语义适合按值传递");
    
    println!("\n2. Copy 的优势:");
    println!("   - 简化所有权管理");
    println!("   - 避免借用检查器限制");
    println!("   - 提高代码可读性");
    println!("   - 减少生命周期复杂性");
    
    println!("\n3. Copy 的注意事项:");
    println!("   - 大型数据的 Copy 成本较高");
    println!("   - Copy 类型不能实现 Drop");
    println!("   - 需要同时实现 Clone");
    println!("   - 语义上应该是'值'而不是'资源'");
    
    println!("\n4. 设计建议:");
    println!("   - 优先考虑小型、简单的值类型");
    println!("   - 避免在 Copy 类型中包含大型数组");
    println!("   - 考虑使用引用而不是 Copy 大型数据");
    println!("   - 在性能敏感的代码中测量 Copy 成本");
    
    // 实际示例
    demonstrate_best_practice_examples();
}

/// 演示最佳实践示例
fn demonstrate_best_practice_examples() {
    println!("\n=== 最佳实践示例 ===");
    
    // 1. 好的 Copy 类型设计
    println!("\n1. 好的 Copy 类型设计:");
    
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct UserId(u64);
    
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Timestamp(u64);
    
    // 使用全局定义的 Event 结构体
    
    let event = Event {
        id: 12345,
        timestamp: 1640995200,
        event_type: 1,
    };
    
    // 可以轻松传递和复制
    process_event(event);
    store_event(event);
    log_event(event);
    
    println!("原始事件仍可用: {:?}", event);
    
    // 2. 避免的 Copy 类型设计
    println!("\n2. 应该避免的设计（仅作演示）:");
    
    // 这种设计不好：包含大型数组
    #[derive(Copy, Clone)]
    struct BadLargeStruct {
        data: [u8; 4096], // 4KB 数据
        metadata: u32,
    }
    
    let bad_struct = BadLargeStruct {
        data: [0; 4096],
        metadata: 42,
    };
    
    println!("大型结构体大小: {} bytes", std::mem::size_of::<BadLargeStruct>());
    println!("每次 Copy 都会复制 4KB 数据，成本较高");
    
    // 更好的设计：使用引用或 Box
    #[derive(Debug, Clone)]
    struct BetterLargeStruct {
        data: Box<[u8; 4096]>, // 堆分配
        metadata: u32,
    }
    
    let better_struct = BetterLargeStruct {
        data: Box::new([0; 4096]),
        metadata: 42,
    };
    
    println!("改进后结构体大小: {} bytes", std::mem::size_of::<BetterLargeStruct>());
    println!("Clone 只复制指针和元数据，更高效");
}

fn process_event(event: Event) {
    println!("处理事件: {:?}", event);
}

fn store_event(event: Event) {
    println!("存储事件: {:?}", event);
}

fn log_event(event: Event) {
    println!("记录事件: {:?}", event);
}

/// 运行所有 Copy trait 示例
pub fn run_copy_examples() {
    basic_copy_concepts();
    copy_implementation_conditions();
    memory_model_analysis();
    advanced_copy_patterns();
    performance_analysis();
    best_practices();
}