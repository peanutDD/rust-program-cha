//! # Edge Cases and Common Pitfalls
//! 
//! 本模块深入分析 Move、Copy 和 Clone 的边界情况、常见陷阱和解决方案。
//! 通过实际案例帮助开发者避免常见错误，提高代码质量。

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::sync::Mutex;
use std::mem;

/// 演示 Move 语义的边界情况和陷阱
pub fn move_edge_cases() {
    println!("\n🔥 Move 语义边界情况和陷阱分析");
    println!("{}", "=".repeat(50));
    
    // 1. 部分 Move 陷阱
    partial_move_pitfalls();
    
    // 2. 闭包捕获陷阱
    closure_capture_pitfalls();
    
    // 3. 模式匹配中的 Move
    pattern_matching_moves();
    
    // 4. 循环中的 Move 问题
    loop_move_issues();
    
    // 5. 条件 Move 的复杂性
    conditional_move_complexity();
}

/// 部分 Move 的陷阱
fn partial_move_pitfalls() {
    println!("\n📦 部分 Move 陷阱:");
    
    #[derive(Debug)]
    struct Container {
        data: Vec<i32>,
        metadata: String,
    }
    
    let container = Container {
        data: vec![1, 2, 3],
        metadata: "important".to_string(),
    };
    
    // 陷阱：部分 Move 后无法使用整个结构体
    let data = container.data; // Move data 字段
    // println!("{:?}", container); // 编译错误！
    
    // 但可以访问未被 Move 的字段
    println!("Metadata still accessible: {}", container.metadata);
    println!("Moved data: {:?}", data);
    
    // 解决方案1：Clone 需要的字段
    let container2 = Container {
        data: vec![4, 5, 6],
        metadata: "backup".to_string(),
    };
    
    let data_clone = container2.data.clone();
    println!("Original container: {:?}", container2);
    println!("Cloned data: {:?}", data_clone);
    
    // 解决方案2：使用引用
    let container3 = Container {
        data: vec![7, 8, 9],
        metadata: "reference".to_string(),
    };
    
    let data_ref = &container3.data;
    println!("Container via reference: {:?}", container3);
    println!("Data reference: {:?}", data_ref);
}

/// 闭包捕获的陷阱
fn closure_capture_pitfalls() {
    println!("\n🔒 闭包捕获陷阱:");
    
    let expensive_data = vec![1; 1000000];
    
    // 陷阱：意外的 Move 捕获
    let closure1 = move || {
        println!("Data length: {}", expensive_data.len()); // 整个 Vec 被 Move
    };
    
    // expensive_data 已经被 Move，无法再使用
    // println!("{}", expensive_data.len()); // 编译错误！
    
    closure1();
    
    // 解决方案1：只捕获需要的部分
    let expensive_data2 = vec![2; 1000000];
    let data_len = expensive_data2.len();
    
    let closure2 = move || {
        println!("Data length: {}", data_len); // 只 Move 长度值
    };
    
    println!("Original data still available: {}", expensive_data2.len());
    closure2();
    
    // 解决方案2：使用 Rc 共享所有权
    let expensive_data3 = Rc::new(vec![3; 1000000]);
    let data_clone = expensive_data3.clone();
    
    let closure3 = move || {
        println!("Shared data length: {}", data_clone.len());
    };
    
    println!("Original Rc still usable: {}", expensive_data3.len());
    closure3();
}

/// 模式匹配中的 Move
fn pattern_matching_moves() {
    println!("\n🎯 模式匹配中的 Move 陷阱:");
    
    #[derive(Debug, Clone)]
    enum Message {
        Text(String),
        Data(Vec<u8>),
        Mixed { text: String, data: Vec<u8> },
    }
    
    let msg = Message::Mixed {
        text: "Hello".to_string(),
        data: vec![1, 2, 3],
    };
    
    // 陷阱：模式匹配中的意外 Move
    match msg {
        Message::Mixed { text, data } => {
            println!("Text: {}, Data: {:?}", text, data);
            // text 和 data 都被 Move 了
        }
        _ => {}
    }
    
    // msg 已经被部分 Move，无法再使用
    // println!("{:?}", msg); // 编译错误！
    
    // 解决方案1：使用引用模式
    let msg2 = Message::Mixed {
        text: "World".to_string(),
        data: vec![4, 5, 6],
    };
    
    match &msg2 {
        Message::Mixed { text, data } => {
            println!("Borrowed - Text: {}, Data: {:?}", text, data);
        }
        _ => {}
    }
    
    println!("msg2 still usable: {:?}", msg2);
    
    // 解决方案2：Clone 在匹配中
    let msg3 = Message::Text("Clone me".to_string());
    
    match msg3.clone() {
        Message::Text(text) => {
            println!("Cloned text: {}", text);
        }
        _ => {}
    }
    
    println!("Original msg3: {:?}", msg3);
}

/// 循环中的 Move 问题
fn loop_move_issues() {
    println!("\n🔄 循环中的 Move 问题:");
    
    let data = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    
    // 陷阱：在循环中意外 Move
    for item in data {
        println!("Processing: {}", item); // item 被 Move
        // 无法在循环中再次使用 item
    }
    
    // data 已经被消费，无法再使用
    // println!("{:?}", data); // 编译错误！
    
    // 解决方案1：使用引用迭代
    let data2 = vec!["x".to_string(), "y".to_string(), "z".to_string()];
    
    for item in &data2 {
        println!("Borrowed processing: {}", item);
    }
    
    println!("data2 still available: {:?}", data2);
    
    // 解决方案2：使用 Clone 迭代
    let data3 = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    
    for item in data3.clone() {
        println!("Cloned processing: {}", item);
    }
    
    println!("Original data3: {:?}", data3);
    
    // 解决方案3：使用索引访问
    let data4 = vec!["i".to_string(), "j".to_string(), "k".to_string()];
    
    for i in 0..data4.len() {
        println!("Indexed processing: {}", data4[i]);
    }
    
    println!("data4 preserved: {:?}", data4);
}

/// 条件 Move 的复杂性
fn conditional_move_complexity() {
    println!("\n🤔 条件 Move 的复杂性:");
    
    let data = vec![1, 2, 3, 4, 5];
    let condition = true;
    
    // 陷阱：条件性 Move 导致的复杂性
    let result = if condition {
        Some(data) // data 在这个分支被 Move
    } else {
        None // data 在这个分支没有被 Move
    };
    
    // 无论哪个分支，data 都不能再使用
    // println!("{:?}", data); // 编译错误！
    
    println!("Conditional result: {:?}", result);
    
    // 解决方案：明确的所有权策略
    let data2 = vec![6, 7, 8, 9, 10];
    let condition2 = false;
    
    let result2 = if condition2 {
        Some(data2.clone()) // 明确 Clone
    } else {
        None
    };
    
    println!("Original data2: {:?}", data2);
    println!("Conditional result2: {:?}", result2);
}

/// 演示 Copy trait 的边界情况和陷阱
pub fn copy_edge_cases() {
    println!("\n📋 Copy trait 边界情况和陷阱分析");
    println!("{}", "=".repeat(50));
    
    // 1. Copy 和 Drop 的冲突
    copy_drop_conflict();
    
    // 2. 大型 Copy 类型的性能陷阱
    large_copy_performance();
    
    // 3. Copy 语义的误解
    copy_semantics_misunderstanding();
    
    // 4. 泛型中的 Copy 约束
    generic_copy_constraints();
    
    // 5. Copy 类型的内存布局陷阱
    copy_memory_layout_pitfalls();
}

/// Copy 和 Drop 的冲突
fn copy_drop_conflict() {
    println!("\n⚠️  Copy 和 Drop 冲突:");
    
    // 正确：可以实现 Copy 的类型
    #[derive(Copy, Clone, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1; // Copy 发生
    println!("p1: {:?}, p2: {:?}", p1, p2); // 两个都可用
    
    // 错误示例：不能同时实现 Copy 和 Drop
    /*
    #[derive(Copy, Clone)]
    struct Resource {
        id: i32,
    }
    
    impl Drop for Resource {
        fn drop(&mut self) {
            println!("Dropping resource {}", self.id);
        }
    }
    */
    
    println!("✅ Copy 类型不能实现 Drop trait");
    println!("   原因：Copy 语义要求类型可以安全地按位复制");
    println!("   而 Drop 意味着需要特殊的清理逻辑");
}

/// 大型 Copy 类型的性能陷阱
fn large_copy_performance() {
    println!("\n🐌 大型 Copy 类型性能陷阱:");
    
    // 陷阱：大型数组的 Copy 开销
    #[derive(Copy, Clone, Debug)]
    struct LargeArray {
        data: [i32; 1000], // 4KB 的数据
    }
    
    let large = LargeArray { data: [42; 1000] };
    
    // 每次赋值都会复制 4KB 数据
    let copy1 = large; // 4KB 复制
    let copy2 = large; // 又一次 4KB 复制
    let copy3 = large; // 再一次 4KB 复制
    
    println!("创建了多个大型数组的副本，每个 4KB");
    println!("copy1[0]: {}, copy2[0]: {}, copy3[0]: {}", 
             copy1.data[0], copy2.data[0], copy3.data[0]);
    
    // 更好的方案：使用引用或智能指针
    let large_ref = &large;
    let another_ref = &large;
    
    println!("使用引用避免不必要的复制");
    println!("ref1[0]: {}, ref2[0]: {}", large_ref.data[0], another_ref.data[0]);
    
    // 或者使用 Rc 共享所有权
    let shared = Rc::new(LargeArray { data: [99; 1000] });
    let shared_clone1 = shared.clone(); // 只复制引用计数
    let shared_clone2 = shared.clone(); // 只复制引用计数
    
    println!("使用 Rc 共享大型数据，避免深拷贝");
    println!("shared[0]: {}, clone1[0]: {}, clone2[0]: {}", 
             shared.data[0], shared_clone1.data[0], shared_clone2.data[0]);
}

/// Copy 语义的误解
fn copy_semantics_misunderstanding() {
    println!("\n🤷 Copy 语义常见误解:");
    
    // 误解1：认为所有简单类型都是 Copy
    let s1 = "hello".to_string(); // String 不是 Copy
    let s2 = s1; // Move 发生，不是 Copy
    // println!("{}", s1); // 编译错误！
    
    println!("❌ 误解：String 是 Copy 类型");
    println!("✅ 事实：String 是 Move 类型，因为它管理堆内存");
    
    // 误解2：认为实现了 Clone 就自动是 Copy
    #[derive(Clone, Debug)]
    struct NotCopy {
        data: Vec<i32>,
    }
    
    let nc1 = NotCopy { data: vec![1, 2, 3] };
    let nc2 = nc1.clone(); // 显式 Clone
    // let nc3 = nc1; // Move 发生，不是 Copy
    
    println!("❌ 误解：实现 Clone 就是 Copy");
    println!("✅ 事实：Copy 是 Clone 的子 trait，但需要额外约束");
    println!("nc2: {:?}", nc2);
    
    // 正确的 Copy 类型
    #[derive(Copy, Clone, Debug)]
    struct IsCopy {
        x: i32,
        y: i32,
    }
    
    let ic1 = IsCopy { x: 1, y: 2 };
    let ic2 = ic1; // Copy 发生
    let ic3 = ic1; // 仍然可以使用 ic1
    
    println!("✅ 正确：同时实现 Copy 和 Clone");
    println!("ic1: {:?}, ic2: {:?}, ic3: {:?}", ic1, ic2, ic3);
}

/// 泛型中的 Copy 约束
fn generic_copy_constraints() {
    println!("\n🔧 泛型中的 Copy 约束陷阱:");
    
    // 函数要求 T: Copy
    fn process_copy<T: Copy + std::fmt::Debug>(value: T) -> T {
        let copy1 = value;
        let copy2 = value; // 因为 T: Copy，这是安全的
        println!("Processing: {:?}", copy1);
        copy2
    }
    
    // 可以用于 Copy 类型
    let num = 42i32;
    let result = process_copy(num);
    println!("Result: {}", result);
    
    // 不能用于非 Copy 类型
    let text = "hello".to_string();
    // let result2 = process_copy(text); // 编译错误！
    
    println!("❌ String 不满足 Copy 约束");
    
    // 更灵活的设计：使用 Clone 约束
    fn process_clone<T: Clone + std::fmt::Debug>(value: T) -> T {
        let cloned = value.clone();
        println!("Processing clone: {:?}", cloned);
        value // 返回原始值
    }
    
    let text2 = "world".to_string();
    let result3 = process_clone(text2);
    println!("Clone result: {}", result3);
    
    println!("✅ Clone 约束更灵活，适用于更多类型");
}

/// Copy 类型的内存布局陷阱
fn copy_memory_layout_pitfalls() {
    println!("\n🧠 Copy 类型内存布局陷阱:");
    
    // 陷阱：包含非 Copy 字段的结构体不能是 Copy
    /*
    #[derive(Copy, Clone)] // 编译错误！
    struct Mixed {
        id: i32,        // Copy
        name: String,   // 非 Copy
    }
    */
    
    println!("❌ 包含非 Copy 字段的结构体不能实现 Copy");
    
    // 正确：所有字段都是 Copy
    #[derive(Copy, Clone, Debug)]
    struct AllCopy {
        id: i32,
        value: f64,
        flag: bool,
    }
    
    let ac = AllCopy { id: 1, value: 3.14, flag: true };
    let ac_copy = ac;
    println!("✅ 所有字段都是 Copy: {:?}", ac_copy);
    
    // 陷阱：数组的 Copy 行为
    let small_array: [i32; 3] = [1, 2, 3];
    let array_copy = small_array; // Copy 发生
    println!("Small array copy: {:?}", array_copy);
    
    // 大数组仍然是 Copy，但性能开销大
    let big_array: [i32; 1000] = [42; 1000];
    let big_copy = big_array; // 4KB 的 Copy！
    println!("Big array copied (first element): {}", big_copy[0]);
    
    println!("⚠️  大数组的 Copy 开销很大，考虑使用引用");
}

/// 演示 Clone trait 的边界情况和陷阱
pub fn clone_edge_cases() {
    println!("\n🔄 Clone trait 边界情况和陷阱分析");
    println!("{}", "=".repeat(50));
    
    // 1. 深拷贝 vs 浅拷贝的混淆
    deep_vs_shallow_clone();
    
    // 2. Clone 的性能陷阱
    clone_performance_pitfalls();
    
    // 3. 循环引用的 Clone 问题
    circular_reference_clone();
    
    // 4. 异步上下文中的 Clone
    async_clone_issues();
    
    // 5. Clone 的内存泄漏风险
    clone_memory_leak_risks();
}

/// 深拷贝 vs 浅拷贝的混淆
fn deep_vs_shallow_clone() {
    println!("\n🏊 深拷贝 vs 浅拷贝混淆:");
    
    // Rc 的 Clone 是浅拷贝
    let data = Rc::new(vec![1, 2, 3, 4, 5]);
    let shallow_clone = data.clone(); // 只复制引用计数
    
    println!("Rc 引用计数: {}", Rc::strong_count(&data));
    println!("原始数据: {:?}", data);
    println!("浅拷贝数据: {:?}", shallow_clone);
    
    // 修改会影响所有引用（如果可变的话）
    drop(shallow_clone);
    println!("删除浅拷贝后引用计数: {}", Rc::strong_count(&data));
    
    // Vec 的 Clone 是深拷贝
    let original_vec = vec![1, 2, 3, 4, 5];
    let deep_clone = original_vec.clone(); // 复制所有元素
    
    println!("\n原始 Vec: {:?}", original_vec);
    println!("深拷贝 Vec: {:?}", deep_clone);
    
    // 证明是独立的副本
    let mut mutable_clone = original_vec.clone();
    mutable_clone.push(6);
    
    println!("修改后的克隆: {:?}", mutable_clone);
    println!("原始 Vec 未变: {:?}", original_vec);
    
    // 嵌套结构的 Clone 复杂性
    #[derive(Clone, Debug)]
    struct Container {
        shared: Rc<Vec<i32>>,
        owned: Vec<i32>,
    }
    
    let container = Container {
        shared: Rc::new(vec![1, 2, 3]),
        owned: vec![4, 5, 6],
    };
    
    let container_clone = container.clone();
    
    println!("\n容器 Clone 的复杂性:");
    println!("shared 字段引用计数: {}", Rc::strong_count(&container.shared));
    println!("原始容器: {:?}", container);
    println!("克隆容器: {:?}", container_clone);
}

/// Clone 的性能陷阱
fn clone_performance_pitfalls() {
    println!("\n⚡ Clone 性能陷阱:");
    
    // 陷阱1：在循环中不必要的 Clone
    let data = vec![1, 2, 3, 4, 5];
    let mut results = Vec::new();
    
    // 低效：每次迭代都 Clone
    for i in 0..3 {
        let cloned_data = data.clone(); // 不必要的 Clone
        results.push(cloned_data.len() + i);
    }
    
    println!("低效方式结果: {:?}", results);
    
    // 高效：使用引用
    let mut efficient_results = Vec::new();
    for i in 0..3 {
        efficient_results.push(data.len() + i); // 直接使用引用
    }
    
    println!("高效方式结果: {:?}", efficient_results);
    
    // 陷阱2：链式 Clone 调用
    let original = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    
    // 低效：多次 Clone
    let step1 = original.clone();
    let step2 = step1.clone();
    let final_result = step2.clone();
    
    println!("链式 Clone 结果: {:?}", final_result);
    
    // 更好：直接 Clone 一次
    let direct_clone = original.clone();
    println!("直接 Clone 结果: {:?}", direct_clone);
    
    // 陷阱3：大型数据结构的意外 Clone
    let large_map: HashMap<String, Vec<i32>> = (0..1000)
        .map(|i| (format!("key_{}", i), vec![i; 100]))
        .collect();
    
    println!("大型 HashMap 创建完成，包含 {} 个条目", large_map.len());
    
    // 意外的昂贵 Clone
    let expensive_clone = large_map.clone();
    println!("昂贵的 Clone 完成，大小: {}", expensive_clone.len());
    
    // 更好的选择：使用 Rc 共享
    let shared_map = Rc::new(large_map);
    let cheap_clone = shared_map.clone(); // 只复制引用计数
    
    println!("便宜的 Rc Clone，引用计数: {}", Rc::strong_count(&shared_map));
}

/// 循环引用的 Clone 问题
fn circular_reference_clone() {
    println!("\n🔄 循环引用 Clone 问题:");
    
    use std::cell::RefCell;
    
    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Option<Rc<RefCell<Node>>>,
    }
    
    // 创建循环引用
    let node1 = Rc::new(RefCell::new(Node {
        value: 1,
        next: None,
    }));
    
    let node2 = Rc::new(RefCell::new(Node {
        value: 2,
        next: Some(node1.clone()),
    }));
    
    // 创建循环
    node1.borrow_mut().next = Some(node2.clone());
    
    println!("创建了循环引用结构");
    println!("node1 引用计数: {}", Rc::strong_count(&node1));
    println!("node2 引用计数: {}", Rc::strong_count(&node2));
    
    // Clone 循环引用结构的问题
    // 注意：这里不能简单地 derive Clone，因为会导致无限递归
    
    // 手动实现安全的 Clone
    impl Clone for Node {
        fn clone(&self) -> Self {
            Node {
                value: self.value,
                next: None, // 打破循环，避免无限递归
            }
        }
    }
    
    let cloned_node = node1.borrow().clone();
    println!("安全的 Clone（打破循环）: {:?}", cloned_node);
    
    // 使用 Weak 引用避免循环
    use std::rc::Weak;
    
    #[derive(Debug)]
    struct SafeNode {
        value: i32,
        next: Option<Rc<RefCell<SafeNode>>>,
        parent: Option<Weak<RefCell<SafeNode>>>, // 使用 Weak 避免循环
    }
    
    println!("\n✅ 使用 Weak 引用可以安全地避免循环引用问题");
}

/// 异步上下文中的 Clone
fn async_clone_issues() {
    println!("\n🚀 异步上下文中的 Clone 问题:");
    
    // 模拟异步场景中的 Clone 需求
    let shared_data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));
    
    // 在多个异步任务中共享数据
    let handles: Vec<_> = (0..3)
        .map(|i| {
            let data_clone = shared_data.clone(); // Arc 的浅拷贝
            std::thread::spawn(move || {
                let mut guard = data_clone.lock().unwrap();
                guard.push(i + 10);
                println!("线程 {} 修改了数据", i);
            })
        })
        .collect();
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终数据: {:?}", shared_data.lock().unwrap());
    
    // Clone 在异步中的性能考虑
    let expensive_data = Arc::new(vec![0; 1000000]);
    
    // 好的做法：Clone Arc（便宜）
    let cheap_clone = expensive_data.clone();
    println!("Arc clone 引用计数: {}", Arc::strong_count(&expensive_data));
    
    // 坏的做法：Clone 内部数据（昂贵）
    let expensive_clone = (*expensive_data).clone();
    println!("昂贵的数据 clone 大小: {}", expensive_clone.len());
    
    println!("\n💡 在异步编程中，优先使用 Arc::clone 而不是数据本身的 clone");
}

/// Clone 的内存泄漏风险
fn clone_memory_leak_risks() {
    println!("\n💧 Clone 内存泄漏风险:");
    
    // 风险1：过度 Clone 导致内存使用过多
    let mut clones = Vec::new();
    let original = vec![0; 100000]; // 400KB 数据
    
    // 创建多个 Clone
    for i in 0..10 {
        let clone = original.clone(); // 每个 Clone 400KB
        clones.push(clone);
        println!("创建第 {} 个 clone", i + 1);
    }
    
    println!("总共使用内存约: {} MB", (clones.len() * 400) / 1024);
    
    // 更好的方案：使用 Rc 共享
    let shared_original = Rc::new(vec![0; 100000]);
    let mut shared_clones = Vec::new();
    
    for i in 0..10 {
        let shared_clone = shared_original.clone(); // 只复制引用
        shared_clones.push(shared_clone);
    }
    
    println!("使用 Rc 共享，引用计数: {}", Rc::strong_count(&shared_original));
    println!("内存使用仅约 400KB（共享）");
    
    // 风险2：忘记释放 Clone 的数据
    {
        let temp_data = vec![1; 50000];
        let temp_clone = temp_data.clone();
        
        // 在作用域结束时，两个 Vec 都会被释放
        println!("临时数据大小: {}", temp_clone.len());
    } // temp_data 和 temp_clone 在这里被释放
    
    println!("✅ 临时数据已自动释放");
    
    // 风险3：循环引用导致的内存泄漏
    println!("\n⚠️  循环引用可能导致内存泄漏，使用 Weak 引用来打破循环");
    println!("   定期检查引用计数，确保资源能够正确释放");
}

/// 通用的边界情况和最佳实践
pub fn general_edge_cases_and_best_practices() {
    println!("\n🎯 通用边界情况和最佳实践");
    println!("{}", "=".repeat(50));
    
    // 1. 类型选择的决策树
    type_selection_decision_tree();
    
    // 2. 性能优化策略
    performance_optimization_strategies();
    
    // 3. 内存安全最佳实践
    memory_safety_best_practices();
    
    // 4. 调试和诊断技巧
    debugging_and_diagnostic_tips();
    
    // 5. 常见反模式和解决方案
    common_antipatterns_and_solutions();
}

/// 类型选择的决策树
fn type_selection_decision_tree() {
    println!("\n🌳 类型选择决策树:");
    
    println!("\n📋 选择指南:");
    println!("1. 数据大小 <= 几个字节 && 不需要 Drop → 考虑 Copy");
    println!("2. 需要共享所有权 → 使用 Rc/Arc + Clone");
    println!("3. 需要深拷贝但不频繁 → 实现 Clone");
    println!("4. 大型数据频繁传递 → 使用 Move 或引用");
    println!("5. 跨线程共享 → Arc<T> 或 Arc<Mutex<T>>");
    
    // 示例决策过程
    #[derive(Copy, Clone, Debug)]
    struct SmallData {
        id: u32,
        flag: bool,
    } // 5 字节，选择 Copy
    
    #[derive(Clone, Debug)]
    struct MediumData {
        name: String,
        values: Vec<i32>,
    } // 可变大小，选择 Clone
    
    struct LargeData {
        buffer: Vec<u8>,
    } // 大型数据，使用 Move 或引用
    
    let small = SmallData { id: 1, flag: true };
    let small_copy = small; // Copy
    println!("小数据使用 Copy: {:?}", small_copy);
    
    let medium = MediumData {
        name: "test".to_string(),
        values: vec![1, 2, 3],
    };
    let medium_clone = medium.clone(); // Clone
    println!("中等数据使用 Clone: {:?}", medium_clone);
    
    let large = LargeData {
        buffer: vec![0; 1000000],
    };
    // 大数据使用 Move 或引用，避免不必要的复制
    let large_moved = large; // Move
    println!("大数据使用 Move，缓冲区大小: {}", large_moved.buffer.len());
}

/// 性能优化策略
fn performance_optimization_strategies() {
    println!("\n⚡ 性能优化策略:");
    
    println!("\n🚀 优化技巧:");
    println!("1. 避免在热路径上进行 Clone");
    println!("2. 使用 Cow<T> 进行写时复制优化");
    println!("3. 考虑使用 &str 而不是 String");
    println!("4. 在可能的情况下使用 &[T] 而不是 Vec<T>");
    println!("5. 利用 Rc/Arc 避免不必要的深拷贝");
    
    use std::borrow::Cow;
    
    // Cow 示例
    fn process_text(input: Cow<str>) -> Cow<str> {
        if input.contains("bad") {
            Cow::Owned(input.replace("bad", "good")) // 需要修改时才分配
        } else {
            input // 不需要修改时直接返回
        }
    }
    
    let text1 = "hello world";
    let result1 = process_text(Cow::Borrowed(text1));
    println!("无需修改: {:?}", result1);
    
    let text2 = "bad example";
    let result2 = process_text(Cow::Borrowed(text2));
    println!("需要修改: {:?}", result2);
    
    // 内存池模式
    println!("\n🏊 考虑使用对象池减少分配/释放开销");
    println!("   特别是在游戏开发或高性能服务中");
}

/// 内存安全最佳实践
fn memory_safety_best_practices() {
    println!("\n🛡️ 内存安全最佳实践:");
    
    println!("\n🔒 安全原则:");
    println!("1. 优先使用栈分配而不是堆分配");
    println!("2. 使用 RAII 模式管理资源");
    println!("3. 避免裸指针，使用智能指针");
    println!("4. 注意生命周期和借用检查器的提示");
    println!("5. 使用 Weak 引用打破循环引用");
    
    // RAII 示例
    struct Resource {
        name: String,
    }
    
    impl Resource {
        fn new(name: &str) -> Self {
            println!("获取资源: {}", name);
            Resource {
                name: name.to_string(),
            }
        }
    }
    
    impl Drop for Resource {
        fn drop(&mut self) {
            println!("释放资源: {}", self.name);
        }
    }
    
    {
        let _resource = Resource::new("database_connection");
        // 资源在作用域结束时自动释放
    } // Drop 在这里被调用
    
    println!("✅ RAII 确保资源自动管理");
}

/// 调试和诊断技巧
fn debugging_and_diagnostic_tips() {
    println!("\n🔍 调试和诊断技巧:");
    
    println!("\n🛠️ 调试工具:");
    println!("1. 使用 #[derive(Debug)] 进行调试输出");
    println!("2. 利用 std::mem::size_of 检查类型大小");
    println!("3. 使用 Rc::strong_count 监控引用计数");
    println!("4. 通过 Drop trait 追踪对象生命周期");
    println!("5. 使用 cargo expand 查看宏展开结果");
    
    // 大小检查
    println!("\n📏 类型大小分析:");
    println!("i32 大小: {} 字节", mem::size_of::<i32>());
    println!("String 大小: {} 字节", mem::size_of::<String>());
    println!("Vec<i32> 大小: {} 字节", mem::size_of::<Vec<i32>>());
    println!("Rc<i32> 大小: {} 字节", mem::size_of::<Rc<i32>>());
    println!("Arc<i32> 大小: {} 字节", mem::size_of::<Arc<i32>>());
    
    // 引用计数监控
    let data = Rc::new(42);
    println!("\n📊 引用计数监控:");
    println!("初始引用计数: {}", Rc::strong_count(&data));
    
    let clone1 = data.clone();
    println!("第一次 clone 后: {}", Rc::strong_count(&data));
    
    let clone2 = data.clone();
    println!("第二次 clone 后: {}", Rc::strong_count(&data));
    
    drop(clone1);
    println!("drop clone1 后: {}", Rc::strong_count(&data));
    
    drop(clone2);
    println!("drop clone2 后: {}", Rc::strong_count(&data));
}

/// 常见反模式和解决方案
fn common_antipatterns_and_solutions() {
    println!("\n❌ 常见反模式和解决方案:");
    
    println!("\n🚫 反模式 1: 过度使用 Clone");
    println!("   问题: 在不需要所有权的地方使用 Clone");
    println!("   解决: 使用引用或 Cow<T>");
    
    println!("\n🚫 反模式 2: 忽略 Copy 的性能影响");
    println!("   问题: 大型 Copy 类型导致性能问题");
    println!("   解决: 使用引用或重新设计数据结构");
    
    println!("\n🚫 反模式 3: 不必要的 Arc/Rc 包装");
    println!("   问题: 在单线程场景中使用 Arc");
    println!("   解决: 单线程使用 Rc，多线程使用 Arc");
    
    println!("\n🚫 反模式 4: 循环引用导致内存泄漏");
    println!("   问题: 强引用形成循环");
    println!("   解决: 使用 Weak 引用打破循环");
    
    println!("\n🚫 反模式 5: 在错误的地方使用 Move");
    println!("   问题: 过早 Move 导致后续无法使用");
    println!("   解决: 仔细设计所有权转移时机");
    
    println!("\n✅ 最佳实践总结:");
    println!("   • 理解每种机制的适用场景");
    println!("   • 优先考虑性能和内存使用");
    println!("   • 利用编译器的帮助和提示");
    println!("   • 定期进行代码审查和性能分析");
    println!("   • 保持代码简洁和可维护性");
}

/// 运行所有边界情况分析
pub fn run_all_edge_cases() {
    move_edge_cases();
    copy_edge_cases();
    clone_edge_cases();
    general_edge_cases_and_best_practices();
    
    println!("\n🎉 边界情况和陷阱分析完成！");
    println!("通过理解这些边界情况，你可以更好地使用 Rust 的内存管理机制。");
}