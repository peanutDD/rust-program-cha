//! # Rust 结构体自引用深度教程
//! 
//! 本教程深入探讨 Rust 中结构体自引用的各个方面，包括：
//! - 自引用问题的根本原因
//! - Pin 和 Unpin 机制
//! - unsafe 自引用实现
//! - 实际应用案例
//! - 性能与安全分析

use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::mem;
use std::time::Instant;

/// # 1. 自引用问题分析
/// 
/// 演示为什么 Rust 不允许直接的自引用结构体
fn demonstrate_self_reference_problem() {
    println!("\n=== 1. 自引用问题分析 ===");
    
    // 这种直接的自引用是不可能的，会导致编译错误
    // struct SelfRef<'a> {
    //     name: String,
    //     self_reference: &'a String, // 这里会产生生命周期冲突
    // }
    
    println!("❌ 直接自引用的问题：");
    println!("   1. 生命周期冲突：结构体不能引用自己的字段");
    println!("   2. 所有权规则：不能同时拥有和借用同一个值");
    println!("   3. 移动语义：结构体移动时引用会失效");
    
    // 演示移动问题
    demonstrate_move_problem();
}

/// 演示移动导致的引用失效问题
fn demonstrate_move_problem() {
    println!("\n📦 移动问题演示：");
    
    #[derive(Debug)]
    struct Container {
        data: String,
        // 如果这里有指向 data 的引用，移动时会失效
    }
    
    let container1 = Container {
        data: String::from("Hello"),
    };
    
    // 移动 container1 到 container2
    let container2 = container1;
    // container1 现在不可用，如果有自引用，引用会指向无效内存
    
    println!("   移动后 container2.data: {}", container2.data);
    println!("   如果有自引用，原引用现在指向无效内存！");
}

/// # 2. Pin 和 Unpin 机制深度解析
/// 
/// Pin<T> 是 Rust 解决自引用问题的核心机制
fn explore_pin_unpin_mechanism() {
    println!("\n=== 2. Pin 和 Unpin 机制 ===");
    
    println!("🔒 Pin<T> 的作用：");
    println!("   1. 防止值被移动到新的内存位置");
    println!("   2. 保证内存地址的稳定性");
    println!("   3. 为自引用提供安全保障");
    
    // 演示 Unpin trait
    demonstrate_unpin_trait();
    
    // 演示 PhantomPinned
    demonstrate_phantom_pinned();
}

/// 演示 Unpin trait 的作用
fn demonstrate_unpin_trait() {
    println!("\n🔓 Unpin trait 演示：");
    
    // 大多数类型都自动实现了 Unpin
    let mut x = 42i32;
    let pinned = Pin::new(&mut x);
    
    // 因为 i32 实现了 Unpin，所以可以安全地获取可变引用
    let unpinned = Pin::into_inner(pinned);
    *unpinned = 100;
    
    println!("   i32 实现了 Unpin，可以安全移动: {}", unpinned);
    
    // 演示哪些类型实现了 Unpin
    println!("   ✅ 实现 Unpin 的类型：基本类型、Vec、HashMap 等");
    println!("   ❌ 不实现 Unpin 的类型：PhantomPinned、某些 Future");
}

/// 演示 PhantomPinned 标记类型
fn demonstrate_phantom_pinned() {
    println!("\n👻 PhantomPinned 演示：");
    
    #[derive(Debug)]
    struct NotMovable {
        data: String,
        _marker: PhantomPinned, // 使类型变为 !Unpin
    }
    
    let not_movable = NotMovable {
        data: String::from("Cannot move me!"),
        _marker: PhantomPinned,
    };
    
    // 必须使用 Box::pin 来创建 Pin<Box<T>>
    let pinned = Box::pin(not_movable);
    
    println!("   PhantomPinned 使类型变为 !Unpin");
    println!("   数据: {}", pinned.data);
    
    // 以下代码会编译错误，因为类型是 !Unpin
    // let moved = Pin::into_inner(pinned); // 编译错误！
}

/// # 3. unsafe 自引用实现
/// 
/// 使用 unsafe 代码和裸指针实现真正的自引用结构体
fn implement_unsafe_self_reference() {
    println!("\n=== 3. unsafe 自引用实现 ===");
    
    // 实现一个自引用结构体
    demonstrate_unsafe_self_ref();
    
    // 使用 NonNull 指针的更安全版本
    demonstrate_nonnull_self_ref();
}

/// 使用裸指针实现自引用
fn demonstrate_unsafe_self_ref() {
    println!("\n⚠️  裸指针自引用演示：");
    
    #[derive(Debug)]
    struct SelfReferential {
        data: String,
        self_ptr: *const String, // 裸指针指向自己的 data 字段
        _marker: PhantomPinned,
    }
    
    impl SelfReferential {
        fn new(content: &str) -> Pin<Box<Self>> {
            let mut boxed = Box::pin(SelfReferential {
                data: String::from(content),
                self_ptr: std::ptr::null(), // 初始化为空指针
                _marker: PhantomPinned,
            });
            
            // 获取 data 字段的地址
            let data_ptr: *const String = &boxed.data;
            
            // 使用 unsafe 设置自引用指针
            unsafe {
                let mut_ref = Pin::as_mut(&mut boxed);
                Pin::get_unchecked_mut(mut_ref).self_ptr = data_ptr;
            }
            
            boxed
        }
        
        fn data(&self) -> &str {
            &self.data
        }
        
        fn self_ref_data(&self) -> &str {
            unsafe {
                // 解引用自引用指针
                &*self.self_ptr
            }
        }
    }
    
    let self_ref = SelfReferential::new("Hello, Self-Reference!");
    
    println!("   原始数据: {}", self_ref.data());
    println!("   自引用数据: {}", self_ref.self_ref_data());
    println!("   地址相同: {}", 
             self_ref.data().as_ptr() == self_ref.self_ref_data().as_ptr());
}

/// 使用 NonNull 指针的更安全实现
fn demonstrate_nonnull_self_ref() {
    println!("\n🛡️  NonNull 自引用演示：");
    
    struct SaferSelfRef {
        data: String,
        self_ptr: Option<NonNull<String>>, // 使用 NonNull 确保非空
        _marker: PhantomPinned,
    }
    
    impl SaferSelfRef {
        fn new(content: &str) -> Pin<Box<Self>> {
            let mut boxed = Box::pin(SaferSelfRef {
                data: String::from(content),
                self_ptr: None,
                _marker: PhantomPinned,
            });
            
            // 获取 data 字段的 NonNull 指针
            let data_ptr = NonNull::from(&boxed.data);
            
            unsafe {
                let mut_ref = Pin::as_mut(&mut boxed);
                Pin::get_unchecked_mut(mut_ref).self_ptr = Some(data_ptr);
            }
            
            boxed
        }
        
        fn get_self_ref(&self) -> Option<&str> {
            self.self_ptr.map(|ptr| unsafe { ptr.as_ref() }.as_str())
        }
    }
    
    let safer_ref = SaferSelfRef::new("Safer Self-Reference");
    
    if let Some(self_data) = safer_ref.get_self_ref() {
        println!("   NonNull 自引用数据: {}", self_data);
    }
    
    println!("   NonNull 提供了额外的安全保证");
}

/// # 4. Pin 模式演示
/// 
/// 演示各种 Pin 相关的 API 使用模式
fn demonstrate_pinning_patterns() {
    println!("\n=== 4. Pin 模式演示 ===");
    
    demonstrate_box_pin();
    demonstrate_pin_projection();
    demonstrate_pin_utils();
}

/// 演示 Box::pin 的使用
fn demonstrate_box_pin() {
    println!("\n📦 Box::pin 演示：");
    
    #[derive(Debug)]
    struct PinnedData {
        value: i32,
        _marker: PhantomPinned,
    }
    
    // 使用 Box::pin 创建堆上的固定数据
    let pinned_box = Box::pin(PinnedData {
        value: 42,
        _marker: PhantomPinned,
    });
    
    println!("   Box::pin 创建堆上固定数据: {:?}", pinned_box.value);
    
    // 演示 Pin::as_ref 和 Pin::as_mut
    let pin_ref = Pin::as_ref(&pinned_box);
    println!("   Pin::as_ref 获取不可变引用: {}", pin_ref.value);
}

/// 演示 Pin 投影（Pin Projection）
fn demonstrate_pin_projection() {
    println!("\n🎯 Pin 投影演示：");
    
    #[derive(Debug)]
    struct Container {
        pinned_field: String,
        normal_field: i32,
        _marker: PhantomPinned,
    }
    
    impl Container {
        // 安全的字段投影
        fn project_pinned_field(self: Pin<&mut Self>) -> Pin<&mut String> {
            // 这是一个简化的投影，实际应用中可能需要更复杂的处理
            unsafe {
                let this = Pin::get_unchecked_mut(self);
                Pin::new_unchecked(&mut this.pinned_field)
            }
        }
        
        // 普通字段可以直接访问
        fn get_normal_field(self: Pin<&Self>) -> &i32 {
            &self.get_ref().normal_field
        }
    }
    
    let mut container = Box::pin(Container {
        pinned_field: String::from("Pinned"),
        normal_field: 100,
        _marker: PhantomPinned,
    });
    
    // 投影到固定字段
    let pinned_field = container.as_mut().project_pinned_field();
    // 可以安全地修改固定字段的内容（但不能移动字段本身）
    
    println!("   Pin 投影允许安全访问固定结构体的字段");
    println!("   普通字段值: {}", container.as_ref().get_normal_field());
}

/// 演示 Pin 相关工具函数
fn demonstrate_pin_utils() {
    println!("\n🔧 Pin 工具函数演示：");
    
    let mut data = String::from("Mutable data");
    
    // Pin::new - 用于 Unpin 类型
    let pinned = Pin::new(&mut data);
    println!("   Pin::new 用于 Unpin 类型");
    
    // Pin::into_inner - 只能用于 Unpin 类型
    let unpinned = Pin::into_inner(pinned);
    unpinned.push_str(" - modified");
    println!("   Pin::into_inner 结果: {}", unpinned);
    
    // 演示 Pin::new_unchecked (unsafe)
    unsafe {
        let unchecked_pin = Pin::new_unchecked(&mut data);
        println!("   Pin::new_unchecked 用于任何类型（unsafe）");
    }
}

/// # 5. 实际应用案例
/// 
/// 展示自引用在实际编程中的应用场景
fn build_practical_examples() {
    println!("\n=== 5. 实际应用案例 ===");
    
    demonstrate_self_ref_linked_list();
    demonstrate_intrusive_list();
    demonstrate_future_like_structure();
}

/// 自引用链表节点
fn demonstrate_self_ref_linked_list() {
    println!("\n🔗 自引用链表演示：");
    
    struct Node {
        data: i32,
        next: Option<NonNull<Node>>,
        prev: Option<NonNull<Node>>,
    }
    
    impl Node {
        fn new(data: i32) -> Pin<Box<Self>> {
            Box::pin(Node {
                data,
                next: None,
                prev: None,
            })
        }
        
        // 在实际实现中，这里需要更复杂的逻辑来维护链表不变性
        fn link_nodes(mut first: Pin<Box<Node>>, mut second: Pin<Box<Node>>) {
            unsafe {
                let first_ptr = NonNull::from(first.as_ref().get_ref());
                let second_ptr = NonNull::from(second.as_ref().get_ref());
                
                Pin::get_unchecked_mut(first.as_mut()).next = Some(second_ptr);
                Pin::get_unchecked_mut(second.as_mut()).prev = Some(first_ptr);
            }
        }
    }
    
    let node1 = Node::new(1);
    let node2 = Node::new(2);
    
    println!("   创建了两个链表节点: {} 和 {}", node1.data, node2.data);
    println!("   在实际应用中，需要仔细管理节点间的引用关系");
}

/// 侵入式数据结构演示
fn demonstrate_intrusive_list() {
    println!("\n🏗️  侵入式数据结构演示：");
    
    // 侵入式链表节点
    struct IntrusiveNode {
        data: String,
        next: Option<NonNull<IntrusiveNode>>,
        _marker: PhantomPinned,
    }
    
    impl IntrusiveNode {
        fn new(data: String) -> Pin<Box<Self>> {
            Box::pin(IntrusiveNode {
                data,
                next: None,
                _marker: PhantomPinned,
            })
        }
    }
    
    let node = IntrusiveNode::new(String::from("Intrusive Node"));
    
    println!("   侵入式数据结构将链接信息嵌入到数据中");
    println!("   节点数据: {}", node.data);
    println!("   这种模式在操作系统内核和高性能库中很常见");
}

/// 类似 Future 的自引用结构
fn demonstrate_future_like_structure() {
    println!("\n🔮 Future 类似结构演示：");
    
    // 模拟一个简单的 Future 结构
    struct SimpleFuture {
        state: String,
        waker_ptr: Option<NonNull<String>>, // 指向自己的 state
        _marker: PhantomPinned,
    }
    
    impl SimpleFuture {
        fn new(initial_state: String) -> Pin<Box<Self>> {
            let mut future = Box::pin(SimpleFuture {
                state: initial_state,
                waker_ptr: None,
                _marker: PhantomPinned,
            });
            
            // 设置自引用
            unsafe {
                let state_ptr = NonNull::from(&future.state);
                Pin::get_unchecked_mut(future.as_mut()).waker_ptr = Some(state_ptr);
            }
            
            future
        }
        
        fn poll(self: Pin<&mut Self>) -> &str {
            // 在真实的 Future 中，这里会检查任务是否完成
            &unsafe { self.get_unchecked_mut() }.state
        }
    }
    
    let mut future = SimpleFuture::new(String::from("Pending"));
    
    println!("   Future 类型经常需要自引用来存储状态");
    println!("   当前状态: {}", future.as_mut().poll());
    println!("   Pin 使得 async/await 成为可能");
}

/// # 6. 生命周期挑战分析
/// 
/// 深入分析自引用中的生命周期问题
fn analyze_lifetime_challenges() {
    println!("\n=== 6. 生命周期挑战分析 ===");
    
    demonstrate_lifetime_conflicts();
    demonstrate_nll_limitations();
    demonstrate_borrow_checker_issues();
}

/// 演示生命周期冲突
fn demonstrate_lifetime_conflicts() {
    println!("\n⏰ 生命周期冲突演示：");
    
    // 这种结构无法编译，因为生命周期冲突
    // struct BadSelfRef<'a> {
    //     data: String,
    //     reference: &'a String, // 'a 从哪里来？
    // }
    
    println!("   ❌ 问题：'a 生命周期参数无法满足");
    println!("   - 结构体不能引用自己的字段");
    println!("   - 生命周期参数需要外部提供");
    println!("   - 移动时引用会失效");
    
    // 展示正确的方法
    println!("\n   ✅ 解决方案：使用 Pin + unsafe 指针");
}

/// 演示 NLL (Non-Lexical Lifetimes) 的限制
fn demonstrate_nll_limitations() {
    println!("\n🔍 NLL 规则限制演示：");
    
    struct Container {
        data: Vec<String>,
    }
    
    impl Container {
        // 即使有 NLL，这种模式仍然有问题
        fn get_self_ref(&self) -> Option<&String> {
            // 不能返回指向自己字段的引用用于自引用
            self.data.first()
        }
    }
    
    let container = Container {
        data: vec![String::from("test")],
    };
    
    if let Some(reference) = container.get_self_ref() {
        println!("   NLL 改善了借用检查，但不能解决自引用问题");
        println!("   引用内容: {}", reference);
    }
    
    println!("   NLL 主要解决了借用作用域问题，不是自引用问题");
}

/// 演示借用检查器的问题
fn demonstrate_borrow_checker_issues() {
    println!("\n🔒 借用检查器问题演示：");
    
    struct DataHolder {
        data: String,
    }
    
    impl DataHolder {
        // 这个方法展示了借用检查器的限制
        fn try_self_reference(&mut self) -> &String {
            // 不能同时可变借用和不可变借用
            &self.data
        }
    }
    
    let mut holder = DataHolder {
        data: String::from("Borrow checker test"),
    };
    
    let reference = holder.try_self_reference();
    println!("   借用检查器防止数据竞争");
    println!("   但也阻止了某些安全的自引用模式");
    println!("   引用: {}", reference);
    
    // holder.data.push_str(" modified"); // 这会导致编译错误
}

/// # 7. 替代解决方案
/// 
/// 探索除了 Pin 之外的其他自引用解决方案
fn explore_alternative_solutions() {
    println!("\n=== 7. 替代解决方案 ===");
    
    demonstrate_rc_refcell_solution();
    demonstrate_arc_mutex_solution();
    demonstrate_index_based_solution();
    demonstrate_external_storage_solution();
}

/// 使用 Rc<RefCell<T>> 的解决方案
fn demonstrate_rc_refcell_solution() {
    println!("\n🔄 Rc<RefCell<T>> 解决方案：");
    
    #[derive(Debug)]
    struct Node {
        data: i32,
        next: Option<Rc<RefCell<Node>>>,
    }
    
    impl Node {
        fn new(data: i32) -> Rc<RefCell<Self>> {
            Rc::new(RefCell::new(Node {
                data,
                next: None,
            }))
        }
        
        fn set_next(&self, next: Rc<RefCell<Node>>) {
            // 这里可以创建循环引用，需要小心内存泄漏
        }
    }
    
    let node1 = Node::new(1);
    let node2 = Node::new(2);
    
    println!("   Rc<RefCell<T>> 允许共享可变性");
    println!("   节点1数据: {}", node1.borrow().data);
    println!("   节点2数据: {}", node2.borrow().data);
    println!("   ⚠️  注意：可能导致循环引用和内存泄漏");
}

/// 使用 Arc<Mutex<T>> 的线程安全解决方案
fn demonstrate_arc_mutex_solution() {
    println!("\n🔐 Arc<Mutex<T>> 线程安全解决方案：");
    
    #[derive(Debug)]
    struct ThreadSafeNode {
        data: i32,
        next: Option<Arc<Mutex<ThreadSafeNode>>>,
    }
    
    impl ThreadSafeNode {
        fn new(data: i32) -> Arc<Mutex<Self>> {
            Arc::new(Mutex::new(ThreadSafeNode {
                data,
                next: None,
            }))
        }
    }
    
    let node = ThreadSafeNode::new(42);
    
    if let Ok(guard) = node.lock() {
        println!("   Arc<Mutex<T>> 提供线程安全的共享可变性");
        println!("   节点数据: {}", guard.data);
    }
    
    println!("   适用于多线程环境");
    println!("   性能开销：原子操作 + 锁");
}

/// 基于索引的解决方案
fn demonstrate_index_based_solution() {
    println!("\n📇 基于索引的解决方案：");
    
    #[derive(Debug)]
    struct IndexedNode {
        data: i32,
        next_index: Option<usize>, // 使用索引而不是指针
    }
    
    struct IndexedList {
        nodes: Vec<IndexedNode>,
    }
    
    impl IndexedList {
        fn new() -> Self {
            IndexedList {
                nodes: Vec::new(),
            }
        }
        
        fn add_node(&mut self, data: i32) -> usize {
            let index = self.nodes.len();
            self.nodes.push(IndexedNode {
                data,
                next_index: None,
            });
            index
        }
        
        fn link_nodes(&mut self, from: usize, to: usize) {
            if from < self.nodes.len() {
                self.nodes[from].next_index = Some(to);
            }
        }
        
        fn get_node(&self, index: usize) -> Option<&IndexedNode> {
            self.nodes.get(index)
        }
    }
    
    let mut list = IndexedList::new();
    let idx1 = list.add_node(10);
    let idx2 = list.add_node(20);
    list.link_nodes(idx1, idx2);
    
    if let Some(node) = list.get_node(idx1) {
        println!("   使用索引避免了指针和生命周期问题");
        println!("   节点数据: {}", node.data);
        if let Some(next_idx) = node.next_index {
            if let Some(next_node) = list.get_node(next_idx) {
                println!("   下一个节点数据: {}", next_node.data);
            }
        }
    }
    
    println!("   优点：简单、安全、无生命周期问题");
    println!("   缺点：需要集中存储、索引可能失效");
}

/// 外部存储模式
fn demonstrate_external_storage_solution() {
    println!("\n🏪 外部存储解决方案：");
    
    use std::collections::HashMap;
    
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct NodeId(usize);
    
    #[derive(Debug)]
    struct ExternalNode {
        data: i32,
        connections: Vec<NodeId>,
    }
    
    struct Graph {
        nodes: HashMap<NodeId, ExternalNode>,
        next_id: usize,
    }
    
    impl Graph {
        fn new() -> Self {
            Graph {
                nodes: HashMap::new(),
                next_id: 0,
            }
        }
        
        fn add_node(&mut self, data: i32) -> NodeId {
            let id = NodeId(self.next_id);
            self.next_id += 1;
            
            self.nodes.insert(id, ExternalNode {
                data,
                connections: Vec::new(),
            });
            
            id
        }
        
        fn connect_nodes(&mut self, from: NodeId, to: NodeId) {
            if let Some(node) = self.nodes.get_mut(&from) {
                node.connections.push(to);
            }
        }
        
        fn get_node(&self, id: NodeId) -> Option<&ExternalNode> {
            self.nodes.get(&id)
        }
    }
    
    let mut graph = Graph::new();
    let node1 = graph.add_node(100);
    let node2 = graph.add_node(200);
    graph.connect_nodes(node1, node2);
    
    if let Some(node) = graph.get_node(node1) {
        println!("   外部存储将数据和关系分离");
        println!("   节点数据: {}", node.data);
        println!("   连接数: {}", node.connections.len());
    }
    
    println!("   优点：灵活、可扩展、类型安全");
    println!("   缺点：间接访问、可能的性能开销");
}

/// # 8. 性能与安全分析
/// 
/// 分析不同自引用解决方案的性能和安全特性
fn performance_safety_analysis() {
    println!("\n=== 8. 性能与安全分析 ===");
    
    analyze_pin_performance();
    analyze_memory_layout();
    analyze_safety_guarantees();
    provide_best_practices();
}

/// 分析 Pin 的性能特性
fn analyze_pin_performance() {
    println!("\n⚡ Pin 性能分析：");
    
    let iterations = 10_000u64;
    
    // 测试普通结构体访问
    let start = Instant::now();
    let mut sum = 0u64;
    for i in 0..iterations {
        sum = sum.wrapping_add(i);
    }
    let normal_time = start.elapsed();
    
    // 测试通过 Pin 访问（模拟）
    let start = Instant::now();
    let mut pinned_sum = 0u64;
    for i in 0..iterations {
        pinned_sum = pinned_sum.wrapping_add(i); // 实际上 Pin 本身没有运行时开销
    }
    let pinned_time = start.elapsed();
    
    println!("   普通访问时间: {:?}", normal_time);
    println!("   Pin 访问时间: {:?}", pinned_time);
    println!("   Pin 本身几乎没有运行时开销");
    println!("   主要开销来自 unsafe 操作和间接访问");
    
    // 内存使用分析
    println!("\n💾 内存使用分析：");
    println!("   Pin<Box<T>>: sizeof(Box<T>) = 8 bytes (指针大小)");
    println!("   PhantomPinned: 0 bytes (零大小类型)");
    println!("   自引用指针: 8 bytes per pointer");
}

/// 分析内存布局影响
fn analyze_memory_layout() {
    println!("\n🏗️  内存布局分析：");
    
    #[derive(Debug)]
    struct NormalStruct {
        data: String,
        number: i32,
    }
    
    #[derive(Debug)]
    struct PinnedStruct {
        data: String,
        number: i32,
        self_ptr: *const String,
        _marker: PhantomPinned,
    }
    
    println!("   普通结构体大小: {} bytes", mem::size_of::<NormalStruct>());
    println!("   Pin 结构体大小: {} bytes", mem::size_of::<PinnedStruct>());
    println!("   额外开销: 自引用指针 + PhantomPinned");
    
    // 对齐分析
    println!("\n📐 内存对齐：");
    println!("   String 对齐: {} bytes", mem::align_of::<String>());
    println!("   指针对齐: {} bytes", mem::align_of::<*const String>());
    println!("   PhantomPinned 对齐: {} bytes", mem::align_of::<PhantomPinned>());
}

/// 分析安全保证
fn analyze_safety_guarantees() {
    println!("\n🛡️  安全保证分析：");
    
    println!("   Pin 提供的保证：");
    println!("   ✅ 防止意外移动");
    println!("   ✅ 内存地址稳定性");
    println!("   ✅ 编译时检查");
    
    println!("\n   unsafe 代码的责任：");
    println!("   ⚠️  正确初始化自引用指针");
    println!("   ⚠️  确保指针有效性");
    println!("   ⚠️  避免数据竞争");
    
    println!("\n   常见陷阱：");
    println!("   ❌ 在 Pin 之前设置自引用");
    println!("   ❌ 忘记使用 PhantomPinned");
    println!("   ❌ 不正确的 Pin 投影");
}

/// 提供最佳实践指南
fn provide_best_practices() {
    println!("\n📋 最佳实践指南：");
    
    println!("\n   1. 设计原则：");
    println!("      • 优先考虑非自引用设计");
    println!("      • 使用索引或外部存储");
    println!("      • 只在必要时使用 Pin");
    
    println!("\n   2. 实现指南：");
    println!("      • 总是使用 PhantomPinned");
    println!("      • 在 Pin 后设置自引用");
    println!("      • 提供安全的访问接口");
    
    println!("\n   3. 测试策略：");
    println!("      • 使用 Miri 检测 unsafe 代码");
    println!("      • 测试移动场景");
    println!("      • 验证内存安全性");
    
    println!("\n   4. 性能优化：");
    println!("      • 最小化 unsafe 操作");
    println!("      • 考虑缓存友好性");
    println!("      • 测量实际性能影响");
}

/// # 9. 高级主题和扩展
/// 
/// 探讨更高级的自引用相关主题
fn explore_advanced_topics() {
    println!("\n=== 9. 高级主题和扩展 ===");
    
    demonstrate_async_self_reference();
    demonstrate_drop_and_cleanup();
    demonstrate_testing_strategies();
}

/// 异步编程中的自引用
fn demonstrate_async_self_reference() {
    println!("\n🔮 异步编程中的自引用：");
    
    // 模拟 async 块生成的 Future 结构
    struct AsyncSelfRef {
        state: String,
        future_data: Option<NonNull<String>>,
        _marker: PhantomPinned,
    }
    
    impl AsyncSelfRef {
        fn new() -> Pin<Box<Self>> {
            let mut future = Box::pin(AsyncSelfRef {
                state: String::from("async state"),
                future_data: None,
                _marker: PhantomPinned,
            });
            
            unsafe {
                let state_ptr = NonNull::from(&future.state);
                Pin::get_unchecked_mut(future.as_mut()).future_data = Some(state_ptr);
            }
            
            future
        }
    }
    
    let async_future = AsyncSelfRef::new();
    
    println!("   async/await 依赖 Pin 来处理自引用 Future");
    println!("   状态: {}", async_future.state);
    println!("   Pin 使得复杂的异步状态机成为可能");
}

/// Drop 和清理策略
fn demonstrate_drop_and_cleanup() {
    println!("\n🗑️  Drop 和清理策略：");
    
    struct SelfRefWithDrop {
        data: String,
        self_ptr: Option<NonNull<String>>,
        _marker: PhantomPinned,
    }
    
    impl Drop for SelfRefWithDrop {
        fn drop(&mut self) {
            println!("   正在清理自引用结构体: {}", self.data);
            // 自引用指针会自动失效，无需特殊处理
            self.self_ptr = None;
        }
    }
    
    impl SelfRefWithDrop {
        fn new(data: String) -> Pin<Box<Self>> {
            let mut boxed = Box::pin(SelfRefWithDrop {
                data,
                self_ptr: None,
                _marker: PhantomPinned,
            });
            
            unsafe {
                let data_ptr = NonNull::from(&boxed.data);
                Pin::get_unchecked_mut(boxed.as_mut()).self_ptr = Some(data_ptr);
            }
            
            boxed
        }
    }
    
    {
        let _self_ref = SelfRefWithDrop::new(String::from("Will be dropped"));
        println!("   创建了带 Drop 的自引用结构体");
    } // Drop 在这里被调用
    
    println!("   Drop 实现应该清理资源，但自引用指针会自动失效");
}

/// 测试策略演示
fn demonstrate_testing_strategies() {
    println!("\n🧪 测试策略演示：");
    
    println!("   1. 单元测试：");
    println!("      • 测试基本功能");
    println!("      • 验证自引用正确性");
    println!("      • 检查边界条件");
    
    println!("\n   2. 内存安全测试：");
    println!("      • 使用 Miri 检测未定义行为");
    println!("      • Valgrind 内存检查");
    println!("      • AddressSanitizer 检测");
    
    println!("\n   3. 并发测试：");
    println!("      • 多线程访问测试");
    println!("      • 数据竞争检测");
    println!("      • 死锁检测");
    
    // 简单的测试示例
    test_self_reference_correctness();
}

/// 测试自引用正确性
fn test_self_reference_correctness() {
    println!("\n   🔍 自引用正确性测试：");
    
    struct TestStruct {
        data: String,
        self_ref: Option<NonNull<String>>,
        _marker: PhantomPinned,
    }
    
    impl TestStruct {
        fn new(content: &str) -> Pin<Box<Self>> {
            let mut boxed = Box::pin(TestStruct {
                data: String::from(content),
                self_ref: None,
                _marker: PhantomPinned,
            });
            
            unsafe {
                let data_ptr = NonNull::from(&boxed.data);
                Pin::get_unchecked_mut(boxed.as_mut()).self_ref = Some(data_ptr);
            }
            
            boxed
        }
        
        fn verify_self_reference(&self) -> bool {
            if let Some(ptr) = self.self_ref {
                unsafe {
                    let self_data = ptr.as_ref();
                    std::ptr::eq(self_data, &self.data)
                }
            } else {
                false
            }
        }
    }
    
    let test_instance = TestStruct::new("Test Data");
    let is_valid = test_instance.verify_self_reference();
    
    println!("      自引用验证结果: {}", if is_valid { "✅ 通过" } else { "❌ 失败" });
    println!("      数据内容: {}", test_instance.data);
    
    assert!(is_valid, "自引用应该指向正确的内存地址");
}

/// 主函数 - 运行所有演示
fn main() {
    println!("🦀 Rust 结构体自引用深度教程");
    println!("=====================================\n");
    
    // 1. 基础问题分析
    demonstrate_self_reference_problem();
    
    // 2. Pin/Unpin 机制
    explore_pin_unpin_mechanism();
    
    // 3. unsafe 实现
    implement_unsafe_self_reference();
    
    // 4. Pin 模式
    demonstrate_pinning_patterns();
    
    // 5. 实际应用
    build_practical_examples();
    
    // 6. 生命周期挑战
    analyze_lifetime_challenges();
    
    // 7. 替代方案
    explore_alternative_solutions();
    
    // 8. 性能安全分析
    performance_safety_analysis();
    
    // 9. 高级主题
    explore_advanced_topics();
    
    println!("\n🎯 教程总结");
    println!("=============\n");
    println!("本教程涵盖了 Rust 结构体自引用的所有重要方面：");
    println!("• 自引用问题的根本原因和挑战");
    println!("• Pin 和 Unpin 机制的深入理解");
    println!("• unsafe 代码实现自引用的技巧");
    println!("• 实际应用场景和最佳实践");
    println!("• 性能考虑和安全保证");
    println!("• 替代解决方案的权衡");
    println!("\n记住：自引用是高级特性，应谨慎使用！");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_phantom_pinned_prevents_move() {
        struct Unmovable {
            _marker: PhantomPinned,
        }
        
        let unmovable = Unmovable {
            _marker: PhantomPinned,
        };
        
        let pinned = Box::pin(unmovable);
        
        // 以下代码应该编译失败（在实际测试中）
        // let moved = Pin::into_inner(pinned); // 编译错误
        
        // 验证类型确实是 !Unpin
        fn assert_not_unpin<T: ?Sized>(_: &T) where T: Unpin {
            // 如果 T 实现了 Unpin，这个函数会编译成功
        }
        
        // assert_not_unpin(&*pinned); // 这应该编译失败
    }
    
    #[test]
    fn test_self_reference_basic() {
        struct SelfRef {
            data: String,
            self_ptr: *const String,
            _marker: PhantomPinned,
        }
        
        impl SelfRef {
            fn new(content: &str) -> Pin<Box<Self>> {
                let mut boxed = Box::pin(SelfRef {
                    data: String::from(content),
                    self_ptr: std::ptr::null(),
                    _marker: PhantomPinned,
                });
                
                let data_ptr: *const String = &boxed.data;
                unsafe {
                    let mut_ref = Pin::as_mut(&mut boxed);
                    Pin::get_unchecked_mut(mut_ref).self_ptr = data_ptr;
                }
                
                boxed
            }
            
            fn get_data(&self) -> &str {
                &self.data
            }
            
            fn get_self_ref_data(&self) -> &str {
                unsafe { &*self.self_ptr }
            }
        }
        
        let self_ref = SelfRef::new("test data");
        
        assert_eq!(self_ref.get_data(), "test data");
        assert_eq!(self_ref.get_self_ref_data(), "test data");
        assert_eq!(
            self_ref.get_data().as_ptr(),
            self_ref.get_self_ref_data().as_ptr()
        );
    }
    
    #[test]
    fn test_pin_projection() {
        struct Container {
            field1: String,
            field2: i32,
            _marker: PhantomPinned,
        }
        
        impl Container {
            fn new(s: String, i: i32) -> Pin<Box<Self>> {
                Box::pin(Container {
                    field1: s,
                    field2: i,
                    _marker: PhantomPinned,
                })
            }
            
            fn get_field2(self: Pin<&Self>) -> &i32 {
                &self.get_ref().field2
            }
        }
        
        let container = Container::new(String::from("test"), 42);
        assert_eq!(*container.as_ref().get_field2(), 42);
    }
    
    #[test]
    fn test_alternative_solutions() {
        // 测试基于索引的解决方案
        struct IndexNode {
            data: i32,
            next: Option<usize>,
        }
        
        struct IndexList {
            nodes: Vec<IndexNode>,
        }
        
        impl IndexList {
            fn new() -> Self {
                IndexList { nodes: Vec::new() }
            }
            
            fn add(&mut self, data: i32) -> usize {
                let index = self.nodes.len();
                self.nodes.push(IndexNode { data, next: None });
                index
            }
            
            fn link(&mut self, from: usize, to: usize) {
                if from < self.nodes.len() {
                    self.nodes[from].next = Some(to);
                }
            }
        }
        
        let mut list = IndexList::new();
        let idx1 = list.add(10);
        let idx2 = list.add(20);
        list.link(idx1, idx2);
        
        assert_eq!(list.nodes[idx1].data, 10);
        assert_eq!(list.nodes[idx1].next, Some(idx2));
        assert_eq!(list.nodes[idx2].data, 20);
    }
}
