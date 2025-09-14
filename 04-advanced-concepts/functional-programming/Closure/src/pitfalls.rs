//! # 闭包的常见陷阱和最佳实践
//!
//! 本模块详细介绍使用闭包时的常见陷阱，包括：
//! - 生命周期问题
//! - 借用检查器冲突
//! - 性能陷阱
//! - 类型推导问题
//! - 内存泄漏风险
//! - 最佳实践建议

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

/// 演示闭包的常见陷阱和最佳实践
pub fn demonstrate() {
    println!("\n⚠️ 9. 闭包的常见陷阱和最佳实践");
    println!("{}", "-".repeat(40));

    lifetime_pitfalls();
    borrow_checker_conflicts();
    performance_pitfalls();
    type_inference_issues();
    memory_leak_risks();
    best_practices();
}

/// 生命周期陷阱演示
fn lifetime_pitfalls() {
    println!("\n💀 生命周期陷阱:");

    // 陷阱1: 返回引用局部变量的闭包
    demonstrate_dangling_reference_pitfall();

    // 陷阱2: 闭包捕获临时值
    demonstrate_temporary_value_pitfall();

    // 陷阱3: 生命周期参数混乱
    demonstrate_lifetime_parameter_confusion();
}

/// 演示悬垂引用陷阱
fn demonstrate_dangling_reference_pitfall() {
    println!("\n🚫 悬垂引用陷阱:");

    // ❌ 错误示例 (编译失败)
    /*
    fn create_bad_closure() -> impl Fn() -> &'static str {
        let local_string = String::from("Hello");
        // 错误：试图返回对局部变量的引用
        move || &local_string  // 编译错误！
    }
    */

    println!("❌ 错误做法: 试图返回对局部变量的引用");
    println!("   这会导致悬垂引用，Rust 编译器会阻止这种情况");

    // ✅ 正确示例
    fn create_good_closure() -> impl Fn() -> String {
        let local_string = String::from("Hello");
        // 正确：移动所有权而不是借用
        move || local_string.clone()
    }

    let closure = create_good_closure();
    println!("✅ 正确做法: {}", closure());

    // 更好的做法：避免不必要的克隆
    fn create_better_closure() -> impl Fn() -> &'static str {
        // 使用静态字符串字面量
        move || "Hello"
    }

    let better_closure = create_better_closure();
    println!("✅ 更好做法: {}", better_closure());
}

/// 演示临时值陷阱
fn demonstrate_temporary_value_pitfall() {
    println!("\n⏰ 临时值陷阱:");

    // ❌ 潜在问题：捕获临时值的引用
    /*
    let temp_vec = vec![1, 2, 3];
    let bad_closure = || {
        // 如果 temp_vec 在闭包使用前被销毁，这里会有问题
        temp_vec.iter().sum::<i32>()
    };
    */

    println!("❌ 潜在问题: 捕获可能被提前销毁的值的引用");

    // ✅ 正确做法：移动所有权
    let temp_vec = vec![1, 2, 3];
    let good_closure = move || temp_vec.iter().sum::<i32>();

    println!("✅ 正确做法: 移动所有权，结果 = {}", good_closure());

    // 演示作用域问题
    demonstrate_scope_issues();
}

/// 演示作用域问题
fn demonstrate_scope_issues() {
    println!("\n📦 作用域问题演示:");

    let mut closures = Vec::new();

    // ❌ 错误模式 (编译失败)
    /*
    for i in 0..3 {
        let value = i * 10;
        // 错误：value 在循环结束时被销毁
        closures.push(|| println!("Value: {}", value));
    }
    */

    // ✅ 正确模式：移动捕获
    for i in 0..3 {
        let value = i * 10;
        closures.push(Box::new(move || println!("Value: {}", value)) as Box<dyn Fn()>);
    }

    println!("执行闭包:");
    for (i, closure) in closures.iter().enumerate() {
        print!("闭包 {}: ", i);
        closure();
    }
}

/// 演示生命周期参数混乱
fn demonstrate_lifetime_parameter_confusion() {
    println!("\n🔄 生命周期参数混乱:");

    // 复杂的生命周期场景
    struct DataProcessor<'a> {
        data: &'a [i32],
    }

    impl<'a> DataProcessor<'a> {
        fn new(data: &'a [i32]) -> Self {
            DataProcessor { data }
        }

        // ❌ 容易混乱的生命周期
        fn create_processor_closure(&self) -> impl Fn(i32) -> Option<i32> + '_ {
            move |target| self.data.iter().find(|&&x| x == target).copied()
        }

        // ✅ 更清晰的做法：明确生命周期关系
        fn create_clear_closure(&self) -> Box<dyn Fn(i32) -> Option<i32> + '_> {
            let data = self.data;
            Box::new(move |target| data.iter().find(|&&x| x == target).copied())
        }
    }

    let data = vec![1, 2, 3, 4, 5];
    let processor = DataProcessor::new(&data);

    let finder = processor.create_clear_closure();
    println!("查找 3: {:?}", finder(3));
    println!("查找 6: {:?}", finder(6));

    println!("\n💡 生命周期最佳实践:");
    println!("- 明确标注生命周期参数");
    println!("- 优先使用 move 语义避免借用问题");
    println!("- 考虑使用 'static 生命周期的数据");
}

/// 借用检查器冲突演示
fn borrow_checker_conflicts() {
    println!("\n🔒 借用检查器冲突:");

    // 陷阱1: 同时可变和不可变借用
    demonstrate_multiple_borrow_conflict();

    // 陷阱2: 闭包中的可变借用冲突
    demonstrate_closure_mutable_borrow_conflict();

    // 陷阱3: 迭代器和闭包的借用冲突
    demonstrate_iterator_borrow_conflict();
}

/// 演示多重借用冲突
fn demonstrate_multiple_borrow_conflict() {
    println!("\n🚫 多重借用冲突:");

    let mut data = vec![1, 2, 3, 4, 5];

    // ❌ 错误模式 (编译失败)
    /*
    let immutable_closure = || {
        println!("Data length: {}", data.len());
    };

    let mutable_closure = || {
        data.push(6);  // 可变借用
    };

    immutable_closure();  // 不可变借用
    mutable_closure();    // 编译错误：同时存在可变和不可变借用
    */

    println!("❌ 错误: 同时进行可变和不可变借用");

    // ✅ 正确模式1: 分离作用域
    {
        let immutable_closure = || {
            println!("Data length: {}", data.len());
        };
        immutable_closure();
    } // 不可变借用结束

    {
        let mut mutable_closure = || {
            data.push(6);
        };
        mutable_closure();
    } // 可变借用结束

    println!("✅ 正确: 分离借用作用域");

    // ✅ 正确模式2: 使用 RefCell
    demonstrate_refcell_solution();
}

/// 演示 RefCell 解决方案
fn demonstrate_refcell_solution() {
    println!("\n🔧 RefCell 解决方案:");

    let data = Rc::new(RefCell::new(vec![1, 2, 3, 4, 5]));

    let data_clone1 = data.clone();
    let reader_closure = move || {
        let borrowed = data_clone1.borrow();
        println!("Data length: {}", borrowed.len());
    };

    let data_clone2 = data.clone();
    let writer_closure = move || {
        let mut borrowed = data_clone2.borrow_mut();
        borrowed.push(6);
        println!("Added element, new length: {}", borrowed.len());
    };

    reader_closure();
    writer_closure();
    reader_closure();

    println!("✅ RefCell 允许运行时借用检查");
}

/// 演示闭包中的可变借用冲突
fn demonstrate_closure_mutable_borrow_conflict() {
    println!("\n🔄 闭包可变借用冲突:");

    let _counter = 0;

    // ❌ 错误模式 (编译失败)
    /*
    let mut increment = || {
        counter += 1;  // 可变借用 counter
    };

    let read_counter = || {
        println!("Counter: {}", counter);  // 不可变借用 counter
    };

    increment();
    read_counter();  // 编译错误
    */

    println!("❌ 错误: 闭包中的借用冲突");

    // ✅ 正确模式1: 使用局部变量
    let mut local_counter = 0;
    {
        let mut increment = || {
            local_counter += 1;
        };
        increment();
        increment();
    }
    println!("✅ 正确: 局部计数器 = {}", local_counter);

    // ✅ 正确模式2: 使用 Cell
    use std::cell::Cell;

    let cell_counter = Cell::new(0);
    let increment = || {
        let current = cell_counter.get();
        cell_counter.set(current + 1);
    };

    let read_counter = || {
        println!("Cell counter: {}", cell_counter.get());
    };

    increment();
    read_counter();
    increment();
    read_counter();

    println!("✅ Cell 允许内部可变性");
}

/// 演示迭代器借用冲突
fn demonstrate_iterator_borrow_conflict() {
    println!("\n🔁 迭代器借用冲突:");

    let mut data = vec![1, 2, 3, 4, 5];

    // ❌ 错误模式 (编译失败)
    /*
    let iter = data.iter();
    data.push(6);  // 可变借用
    for item in iter {  // 不可变借用仍在使用
        println!("{}", item);  // 编译错误
    }
    */

    println!("❌ 错误: 迭代器使用期间修改集合");

    // ✅ 正确模式1: 先收集再修改
    let items: Vec<_> = data.iter().cloned().collect();
    data.push(6);
    println!("✅ 正确: 收集的数据 {:?}", items);

    // ✅ 正确模式2: 分离操作
    {
        for item in &data {
            println!("Item: {}", item);
        }
    }
    data.push(7);
    println!("✅ 正确: 最终数据 {:?}", data);
}

/// 性能陷阱演示
fn performance_pitfalls() {
    println!("\n🐌 性能陷阱:");

    // 陷阱1: 不必要的克隆
    demonstrate_unnecessary_cloning();

    // 陷阱2: 大型捕获
    demonstrate_large_captures();

    // 陷阱3: 堆分配滥用
    demonstrate_heap_allocation_abuse();
}

/// 演示不必要的克隆
fn demonstrate_unnecessary_cloning() {
    println!("\n📋 不必要的克隆陷阱:");

    let large_string = "A".repeat(10000);

    // ❌ 低效：不必要的克隆
    let inefficient_closure = {
        let cloned_string = large_string.clone(); // 昂贵的克隆
        move || cloned_string.len()
    };

    // ✅ 高效：只捕获需要的部分
    let length = large_string.len();
    let efficient_closure = move || length;

    println!(
        "❌ 低效闭包大小: {} 字节",
        std::mem::size_of_val(&inefficient_closure)
    );
    println!(
        "✅ 高效闭包大小: {} 字节",
        std::mem::size_of_val(&efficient_closure)
    );

    // 性能测试
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        std::hint::black_box(inefficient_closure());
    }
    let inefficient_time = start.elapsed();

    let start = std::time::Instant::now();
    for _ in 0..1000 {
        std::hint::black_box(efficient_closure());
    }
    let efficient_time = start.elapsed();

    println!("❌ 低效执行时间: {:?}", inefficient_time);
    println!("✅ 高效执行时间: {:?}", efficient_time);
}

/// 演示大型捕获陷阱
fn demonstrate_large_captures() {
    println!("\n📦 大型捕获陷阱:");

    #[derive(Clone)]
    struct LargeStruct {
        data: Vec<u8>,
        metadata: HashMap<String, String>,
    }

    impl LargeStruct {
        fn new() -> Self {
            let mut metadata = HashMap::new();
            for i in 0..1000 {
                metadata.insert(format!("key_{}", i), format!("value_{}", i));
            }

            LargeStruct {
                data: vec![0u8; 10000],
                metadata,
            }
        }

        fn get_data_size(&self) -> usize {
            self.data.len()
        }
    }

    let large_struct = LargeStruct::new();

    // ❌ 低效：捕获整个大型结构
    let inefficient_closure = {
        let large_struct = large_struct.clone();
        move || large_struct.get_data_size()
    };

    // ✅ 高效：只捕获需要的数据
    let data_size = large_struct.get_data_size();
    let efficient_closure = move || data_size;

    println!(
        "❌ 低效闭包大小: ~{} KB",
        std::mem::size_of_val(&inefficient_closure) / 1024
    );
    println!(
        "✅ 高效闭包大小: {} 字节",
        std::mem::size_of_val(&efficient_closure)
    );

    println!("\n💡 大型捕获优化建议:");
    println!("- 只捕获真正需要的数据");
    println!("- 考虑预先计算值而不是捕获整个对象");
    println!("- 使用引用而非所有权转移（如果生命周期允许）");
}

/// 演示堆分配滥用
fn demonstrate_heap_allocation_abuse() {
    println!("\n🏗️ 堆分配滥用:");

    // ❌ 低效：过度使用 Box
    fn create_boxed_closures() -> Vec<Box<dyn Fn(i32) -> i32>> {
        let mut closures: Vec<Box<dyn Fn(i32) -> i32>> = Vec::new();
        for i in 0..3 {
            closures.push(Box::new(move |x| x + i));
        }
        closures
    }

    // ✅ 高效：使用泛型避免装箱
    fn process_with_closure<F>(data: &[i32], f: F) -> Vec<i32>
    where
        F: Fn(i32) -> i32,
    {
        data.iter().map(|&x| f(x)).collect()
    }

    let boxed_closures = create_boxed_closures();
    let data = vec![1, 2, 3, 4, 5];

    println!("❌ 装箱闭包数量: {}", boxed_closures.len());

    // 使用泛型闭包
    let add_10 = |x| x + 10;
    let result = process_with_closure(&data, add_10);
    println!("✅ 泛型闭包结果: {:?}", result);

    println!("\n💡 堆分配优化建议:");
    println!("- 优先使用泛型参数而非 trait 对象");
    println!("- 避免不必要的 Box 装箱");
    println!("- 考虑使用 impl Trait 语法");
}

/// 类型推导问题演示
fn type_inference_issues() {
    println!("\n🤔 类型推导问题:");

    // 陷阱1: 模糊的类型推导
    demonstrate_ambiguous_type_inference();

    // 陷阱2: 闭包类型不匹配
    demonstrate_closure_type_mismatch();

    // 陷阱3: 生命周期推导失败
    demonstrate_lifetime_inference_failure();
}

/// 演示模糊的类型推导
fn demonstrate_ambiguous_type_inference() {
    println!("\n❓ 模糊的类型推导:");

    // ❌ 可能导致推导失败的代码
    /*
    let closure = |x| x + 1;  // 类型不明确
    let result = closure(5.0);  // 这里才确定类型
    */

    println!("❌ 问题: 延迟的类型推导可能导致错误");

    // ✅ 明确的类型注解
    let typed_closure = |x: f64| x + 1.0;
    let result = typed_closure(5.0);
    println!("✅ 明确类型: {}", result);

    // ✅ 使用类型提示
    let inferred_closure = |x: f64| x + 1.0;
    let result: f64 = inferred_closure(5.0);
    println!("✅ 类型提示: {}", result);

    // 复杂情况下的类型推导
    demonstrate_complex_type_inference();
}

/// 演示复杂类型推导
fn demonstrate_complex_type_inference() {
    println!("\n🧩 复杂类型推导:");

    // 泛型闭包的类型推导问题
    fn process_data<T, F>(data: Vec<T>, processor: F) -> Vec<T>
    where
        F: Fn(T) -> T,
        T: Clone,
    {
        data.into_iter().map(processor).collect()
    }

    let numbers = vec![1, 2, 3, 4, 5];

    // ✅ 明确的闭包类型
    let result1 = process_data(numbers.clone(), |x: i32| x * 2);
    println!("明确类型结果: {:?}", result1);

    // ✅ 使用类型注解帮助推导
    let multiplier: fn(i32) -> i32 = |x| x * 3;
    let result2 = process_data(numbers, multiplier);
    println!("函数类型结果: {:?}", result2);
}

/// 演示闭包类型不匹配
fn demonstrate_closure_type_mismatch() {
    println!("\n🔄 闭包类型不匹配:");

    // ❌ 类型不匹配的问题
    /*
    let mut closures = Vec::new();
    closures.push(|x: i32| x + 1);      // Fn(i32) -> i32
    closures.push(|x: i32| x * 2);      // 不同的闭包类型！
    */

    println!("❌ 问题: 每个闭包都有唯一的类型");

    // ✅ 解决方案1: 使用函数指针
    let mut fn_pointers: Vec<fn(i32) -> i32> = Vec::new();
    fn_pointers.push(|x| x + 1);
    fn_pointers.push(|x| x * 2);

    println!("✅ 函数指针解决方案:");
    for (i, f) in fn_pointers.iter().enumerate() {
        println!("  函数 {}: f(5) = {}", i, f(5));
    }

    // ✅ 解决方案2: 使用 trait 对象
    let mut trait_objects: Vec<Box<dyn Fn(i32) -> i32>> = Vec::new();
    trait_objects.push(Box::new(|x| x + 1));
    trait_objects.push(Box::new(|x| x * 2));

    println!("✅ Trait 对象解决方案:");
    for (i, f) in trait_objects.iter().enumerate() {
        println!("  闭包 {}: f(5) = {}", i, f(5));
    }
}

/// 演示生命周期推导失败
fn demonstrate_lifetime_inference_failure() {
    println!("\n⏳ 生命周期推导失败:");

    // 复杂的生命周期场景
    struct Container<'a> {
        data: &'a str,
    }

    impl<'a> Container<'a> {
        fn create_closure(&self) -> impl Fn() -> &'a str + '_ {
            move || self.data
        }

        // ❌ 可能的生命周期问题
        /*
        fn create_problematic_closure(&self) -> impl Fn() -> &str {
            // 缺少生命周期标注，可能导致推导失败
            move || self.data
        }
        */
    }

    let data = "Hello, World!";
    let container = Container { data };
    let closure = container.create_closure();

    println!("✅ 正确的生命周期标注: {}", closure());

    println!("\n💡 生命周期推导建议:");
    println!("- 明确标注复杂的生命周期关系");
    println!("- 使用 '_ 让编译器推导简单情况");
    println!("- 考虑使用 'static 生命周期避免复杂性");
}

/// 内存泄漏风险演示
fn memory_leak_risks() {
    println!("\n💧 内存泄漏风险:");

    // 风险1: 循环引用
    demonstrate_circular_reference_risk();

    // 风险2: 长生命周期捕获
    demonstrate_long_lived_capture_risk();

    // 风险3: 事件监听器泄漏
    demonstrate_event_listener_leak();
}

/// 演示循环引用风险
fn demonstrate_circular_reference_risk() {
    println!("\n🔄 循环引用风险:");

    // ❌ 潜在的循环引用
    /*
    let parent = Rc::new(RefCell::new(None));
    let child = Rc::new(RefCell::new(Some(parent.clone())));
    *parent.borrow_mut() = Some(child.clone());
    // 循环引用：parent -> child -> parent
    */

    println!("❌ 风险: Rc 循环引用导致内存泄漏");

    // ✅ 使用 Weak 引用打破循环
    use std::rc::Weak;

    struct Parent {
        children: RefCell<Vec<Rc<Child>>>,
    }

    struct Child {
        parent: Weak<Parent>, // 使用 Weak 引用
    }

    let parent = Rc::new(Parent {
        children: RefCell::new(Vec::new()),
    });

    let child = Rc::new(Child {
        parent: Rc::downgrade(&parent),
    });

    parent.children.borrow_mut().push(child);

    println!("✅ 解决方案: 使用 Weak 引用打破循环");

    // 闭包中的循环引用风险
    demonstrate_closure_circular_reference();
}

/// 演示闭包中的循环引用
fn demonstrate_closure_circular_reference() {
    println!("\n🔗 闭包循环引用:");

    // ❌ 闭包中的潜在循环引用
    let data = Rc::new(RefCell::new(vec![1, 2, 3]));

    // 创建一个持有 data 引用的闭包
    let data_clone = data.clone();
    let processor = move |new_value: i32| {
        data_clone.borrow_mut().push(new_value);
    };

    // 如果将 processor 存储在 data 相关的结构中，可能形成循环

    processor(4);
    println!("✅ 谨慎处理: 数据 = {:?}", data.borrow());

    println!("\n💡 循环引用预防:");
    println!("- 使用 Weak 引用打破强引用循环");
    println!("- 避免在长生命周期对象中存储闭包");
    println!("- 定期清理不需要的闭包引用");
}

/// 演示长生命周期捕获风险
fn demonstrate_long_lived_capture_risk() {
    println!("\n⏰ 长生命周期捕获风险:");

    // ❌ 风险：捕获大量数据的长生命周期闭包
    let large_data = vec![0u8; 1_000_000]; // 1MB 数据

    // ✅ 解决方案：只捕获需要的数据
    let data_length = large_data.len();

    // 这个闭包会持有整个 large_data 的所有权
    let long_lived_closure = move || {
        large_data.len() // 只需要长度，但持有了整个向量
    };

    // 如果这个闭包被长期持有，会导致内存浪费

    println!("❌ 风险: 长生命周期闭包持有大量数据");
    println!(
        "   闭包大小: ~{} MB",
        std::mem::size_of_val(&long_lived_closure) / 1_000_000
    );
    let efficient_closure = move || data_length;

    println!("✅ 解决方案: 只捕获需要的数据");
    println!(
        "   闭包大小: {} 字节",
        std::mem::size_of_val(&efficient_closure)
    );

    // 模拟长期持有
    std::mem::forget(long_lived_closure); // 演示目的，实际不要这样做
    println!("   高效闭包结果: {}", efficient_closure());
}

/// 演示事件监听器泄漏
fn demonstrate_event_listener_leak() {
    println!("\n📡 事件监听器泄漏:");

    struct EventEmitter {
        listeners: RefCell<Vec<Box<dyn Fn(&str)>>>,
    }

    impl EventEmitter {
        fn new() -> Self {
            EventEmitter {
                listeners: RefCell::new(Vec::new()),
            }
        }

        fn add_listener<F>(&self, listener: F)
        where
            F: Fn(&str) + 'static,
        {
            self.listeners.borrow_mut().push(Box::new(listener));
        }

        fn emit(&self, event: &str) {
            for listener in self.listeners.borrow().iter() {
                listener(event);
            }
        }

        fn clear_listeners(&self) {
            self.listeners.borrow_mut().clear();
        }

        fn listener_count(&self) -> usize {
            self.listeners.borrow().len()
        }
    }

    let emitter = EventEmitter::new();

    // 添加监听器
    let large_context = vec![0u8; 10000]; // 模拟大型上下文

    emitter.add_listener(move |event| {
        println!("监听器1: {} (上下文大小: {})", event, large_context.len());
    });

    emitter.add_listener(|event| {
        println!("监听器2: {}", event);
    });

    println!("添加监听器后数量: {}", emitter.listener_count());

    emitter.emit("测试事件");

    // ✅ 重要：清理监听器避免内存泄漏
    emitter.clear_listeners();
    println!("清理后监听器数量: {}", emitter.listener_count());

    println!("\n💡 事件监听器最佳实践:");
    println!("- 提供移除监听器的机制");
    println!("- 在对象销毁时清理所有监听器");
    println!("- 避免在监听器中捕获大型数据");
}

/// 最佳实践建议
fn best_practices() {
    println!("\n✅ 最佳实践建议:");

    println!("\n🎯 设计原则:");
    println!("1. 最小化捕获：只捕获真正需要的数据");
    println!("2. 明确生命周期：使用适当的生命周期标注");
    println!("3. 避免循环引用：使用 Weak 引用打破循环");
    println!("4. 性能优先：优先使用泛型而非 trait 对象");

    println!("\n🔧 实现技巧:");
    println!("1. 使用 move 语义避免借用问题");
    println!("2. 预先计算值而不是捕获整个对象");
    println!("3. 使用 Cell/RefCell 处理内部可变性");
    println!("4. 明确标注复杂的类型和生命周期");

    println!("\n🚨 常见错误:");
    println!("1. 返回引用局部变量的闭包");
    println!("2. 同时进行可变和不可变借用");
    println!("3. 在长生命周期闭包中捕获大型数据");
    println!("4. 创建循环引用导致内存泄漏");

    println!("\n🎨 代码风格:");
    println!("1. 使用有意义的变量名");
    println!("2. 保持闭包简洁和专注");
    println!("3. 添加适当的注释说明复杂逻辑");
    println!("4. 考虑将复杂闭包重构为函数");

    // 演示最佳实践示例
    demonstrate_best_practice_examples();
}

/// 演示最佳实践示例
fn demonstrate_best_practice_examples() {
    println!("\n🌟 最佳实践示例:");

    // 示例1: 配置驱动的处理器
    #[derive(Clone)]
    struct ProcessorConfig {
        multiplier: f64,
        offset: f64,
        max_value: f64,
    }

    fn create_processor(config: ProcessorConfig) -> impl Fn(f64) -> Option<f64> {
        move |input| {
            let result = input * config.multiplier + config.offset;
            if result <= config.max_value {
                Some(result)
            } else {
                None
            }
        }
    }

    let config = ProcessorConfig {
        multiplier: 2.0,
        offset: 1.0,
        max_value: 100.0,
    };

    let processor = create_processor(config);

    println!("配置驱动处理器:");
    for input in [10.0, 30.0, 60.0] {
        match processor(input) {
            Some(result) => println!("  {} -> {}", input, result),
            None => println!("  {} -> 超出范围", input),
        }
    }

    // 示例2: 资源安全的闭包
    demonstrate_resource_safe_closure();
}

/// 演示资源安全的闭包
fn demonstrate_resource_safe_closure() {
    println!("\n🛡️ 资源安全的闭包:");

    use std::sync::Arc;

    struct ResourceManager {
        resources: Arc<Mutex<Vec<String>>>,
    }

    impl ResourceManager {
        fn new() -> Self {
            ResourceManager {
                resources: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn create_safe_processor(&self) -> impl Fn(String) -> Result<String, String> {
            let resources = self.resources.clone();

            move |input| {
                // 安全的资源访问
                match resources.lock() {
                    Ok(mut guard) => {
                        guard.push(input.clone());
                        Ok(format!("处理: {} (资源数: {})", input, guard.len()))
                    }
                    Err(_) => Err("资源锁定失败".to_string()),
                }
            }
        }

        fn get_resource_count(&self) -> usize {
            self.resources.lock().unwrap().len()
        }
    }

    let manager = ResourceManager::new();
    let processor = manager.create_safe_processor();

    println!("资源安全处理:");
    for item in ["任务1", "任务2", "任务3"] {
        match processor(item.to_string()) {
            Ok(result) => println!("  ✅ {}", result),
            Err(error) => println!("  ❌ {}", error),
        }
    }

    println!("最终资源数量: {}", manager.get_resource_count());

    println!("\n💡 这个示例展示了:");
    println!("- 使用 Arc<Mutex<>> 进行线程安全的资源共享");
    println!("- 错误处理和资源管理的结合");
    println!("- 闭包中的安全资源访问模式");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifetime_safety() {
        fn create_safe_closure() -> impl Fn() -> String {
            let local_data = String::from("test");
            move || local_data.clone()
        }

        let closure = create_safe_closure();
        assert_eq!(closure(), "test");
    }

    #[test]
    fn test_borrow_checker_compliance() {
        let mut data = vec![1, 2, 3];

        // 分离借用作用域
        {
            let reader = || data.len();
            assert_eq!(reader(), 3);
        }

        {
            let mut writer = || data.push(4);
            writer();
        }

        assert_eq!(data.len(), 4);
    }

    #[test]
    fn test_performance_optimization() {
        let large_string = "A".repeat(1000);
        let length = large_string.len();

        // 只捕获需要的数据
        let efficient_closure = move || length;

        assert_eq!(efficient_closure(), 1000);
        assert_eq!(
            std::mem::size_of_val(&efficient_closure),
            std::mem::size_of::<usize>()
        );
    }

    #[test]
    fn test_type_safety() {
        let typed_closure: fn(i32) -> i32 = |x| x * 2;
        assert_eq!(typed_closure(5), 10);

        let boxed_closure: Box<dyn Fn(i32) -> i32> = Box::new(|x| x + 1);
        assert_eq!(boxed_closure(5), 6);
    }

    #[test]
    fn test_resource_management() {
        use std::sync::{Arc, Mutex};

        let counter = Arc::new(Mutex::new(0));
        let counter_clone = counter.clone();

        let increment = move || {
            let mut guard = counter_clone.lock().unwrap();
            *guard += 1;
            *guard
        };

        assert_eq!(increment(), 1);
        assert_eq!(increment(), 2);
        assert_eq!(*counter.lock().unwrap(), 2);
    }
}
