//! # 智能指针 (Smart Pointers) 深度解析
//!
//! 智能指针是包含指针和额外元数据的数据结构，提供了超越普通引用的功能。
//! 本模块深入探讨 Rust 中各种智能指针的实现、使用场景和内部机制。
//!
//! ## 主要智能指针类型
//!
//! - `Box<T>`: 堆分配的智能指针
//! - `Rc<T>`: 引用计数智能指针
//! - `Arc<T>`: 原子引用计数智能指针
//! - `RefCell<T>`: 内部可变性智能指针
//! - `Mutex<T>`: 互斥锁智能指针
//! - `RwLock<T>`: 读写锁智能指针
//! - `Weak<T>`: 弱引用智能指针
//! - `Cow<T>`: 写时克隆智能指针
//!
//! ## 核心特性
//!
//! 1. **自动内存管理**: 自动分配和释放内存
//! 2. **所有权语义**: 实现 Drop trait 进行清理
//! 3. **解引用**: 实现 Deref trait 提供透明访问
//! 4. **引用计数**: 某些类型支持共享所有权
//! 5. **线程安全**: 某些类型可在多线程间安全共享
//! 6. **内部可变性**: 在不可变引用下修改数据

use std::sync::{Arc, Mutex, RwLock, Weak};
use std::rc::{Rc, Weak as RcWeak};
use std::cell::{RefCell, Cell};
use std::borrow::Cow;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::ops::Deref;

/// 运行所有智能指针示例
pub fn run_smart_pointer_examples() {
    println!("🧠 智能指针深度解析");
    println!("{}", "=".repeat(50));

    box_smart_pointer();
    rc_smart_pointer();
    arc_smart_pointer();
    refcell_smart_pointer();
    mutex_smart_pointer();
    rwlock_smart_pointer();
    weak_references();
    cow_smart_pointer();
    custom_smart_pointers();
    smart_pointer_patterns();
    advanced_smart_pointer_techniques();
}

/// Box<T> 智能指针
fn box_smart_pointer() {
    println!("\n📌 1. Box<T> - 堆分配智能指针");
    println!("{}", "-".repeat(40));

    // 基本使用
    demonstrate_basic_box_usage();
    
    // 递归数据结构
    demonstrate_recursive_data_structures();
    
    // 大型数据的堆分配
    demonstrate_heap_allocation_for_large_data();
    
    // Box 的所有权转移
    demonstrate_box_ownership();
}

/// 演示 Box 的基本使用
fn demonstrate_basic_box_usage() {
    println!("\n📦 Box 基本使用:");
    
    // 在堆上分配单个值
    let boxed_int = Box::new(42);
    println!("堆上的整数: {}", boxed_int);
    println!("Box 地址: {:p}", &boxed_int);
    println!("值的地址: {:p}", &*boxed_int);
    
    // Box 实现了 Deref，可以像普通引用一样使用
    let value = *boxed_int;
    println!("解引用的值: {}", value);
    
    // 在堆上分配复杂数据
    let boxed_string = Box::new(String::from("Hello, Box!"));
    println!("堆上的字符串: {}", boxed_string);
    println!("字符串长度: {}", boxed_string.len());
    
    // Box 的大小
    println!("Box<i32> 大小: {} 字节", std::mem::size_of::<Box<i32>>());
    println!("i32 大小: {} 字节", std::mem::size_of::<i32>());
    println!("指针大小: {} 字节", std::mem::size_of::<*const i32>());
    
    println!("✅ Box 提供了堆分配和自动内存管理");
}

/// 演示递归数据结构
fn demonstrate_recursive_data_structures() {
    println!("\n🌳 递归数据结构:");
    
    // 链表节点
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    use List::{Cons, Nil};
    
    // 创建链表
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("链表: {:?}", list);
    
    // 二叉树
    #[derive(Debug)]
    struct TreeNode {
        value: i32,
        left: Option<Box<TreeNode>>,
        right: Option<Box<TreeNode>>,
    }
    
    impl TreeNode {
        fn new(value: i32) -> Self {
            TreeNode {
                value,
                left: None,
                right: None,
            }
        }
        
        fn insert(&mut self, value: i32) {
            if value < self.value {
                match &mut self.left {
                    Some(left) => left.insert(value),
                    None => self.left = Some(Box::new(TreeNode::new(value))),
                }
            } else {
                match &mut self.right {
                    Some(right) => right.insert(value),
                    None => self.right = Some(Box::new(TreeNode::new(value))),
                }
            }
        }
        
        fn inorder_traversal(&self) {
            if let Some(left) = &self.left {
                left.inorder_traversal();
            }
            print!("{} ", self.value);
            if let Some(right) = &self.right {
                right.inorder_traversal();
            }
        }
    }
    
    let mut tree = TreeNode::new(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(9);
    
    print!("二叉搜索树中序遍历: ");
    tree.inorder_traversal();
    println!();
    
    println!("✅ Box 使递归数据结构成为可能");
}

/// 演示大型数据的堆分配
fn demonstrate_heap_allocation_for_large_data() {
    println!("\n📊 大型数据的堆分配:");
    
    // 大型数组
    const LARGE_SIZE: usize = 1_000_000;
    
    // 如果在栈上分配会导致栈溢出
    let large_array = Box::new([0u8; LARGE_SIZE]);
    println!("大型数组已在堆上分配，大小: {} 字节", large_array.len());
    
    // 大型结构体
    #[derive(Debug)]
    struct LargeStruct {
        data: [u64; 10000],
        metadata: String,
    }
    
    let large_struct = Box::new(LargeStruct {
        data: [42; 10000],
        metadata: String::from("Large data structure"),
    });
    
    println!("大型结构体元数据: {}", large_struct.metadata);
    println!("结构体大小: {} 字节", std::mem::size_of::<LargeStruct>());
    
    println!("✅ Box 避免了栈溢出，适合大型数据");
}

/// 演示 Box 的所有权转移
fn demonstrate_box_ownership() {
    println!("\n🏠 Box 所有权转移:");
    
    let boxed_data = Box::new(vec![1, 2, 3, 4, 5]);
    println!("原始 Box: {:?}", boxed_data);
    
    // 所有权转移
    let moved_box = boxed_data;
    println!("转移后的 Box: {:?}", moved_box);
    // println!("{:?}", boxed_data);  // 错误！boxed_data 已被移动
    
    // 函数参数中的所有权转移
    fn take_ownership(b: Box<Vec<i32>>) {
        println!("函数中的 Box: {:?}", b);
    } // b 在这里被销毁
    
    let another_box = Box::new(vec![6, 7, 8]);
    take_ownership(another_box);
    // println!("{:?}", another_box);  // 错误！another_box 已被移动
    
    // 返回 Box
    fn create_box() -> Box<String> {
        Box::new(String::from("Created in function"))
    }
    
    let returned_box = create_box();
    println!("返回的 Box: {}", returned_box);
    
    println!("✅ Box 遵循 Rust 的所有权规则");
}

/// Rc<T> 智能指针
fn rc_smart_pointer() {
    println!("\n📌 2. Rc<T> - 引用计数智能指针");
    println!("{}", "-".repeat(40));

    // 基本使用
    demonstrate_basic_rc_usage();
    
    // 共享所有权
    demonstrate_shared_ownership();
    
    // 引用计数
    demonstrate_reference_counting();
    
    // Rc 与循环引用
    demonstrate_rc_cycles();
}

/// 演示 Rc 的基本使用
fn demonstrate_basic_rc_usage() {
    println!("\n🔄 Rc 基本使用:");
    
    let data = Rc::new(String::from("Shared data"));
    println!("原始 Rc: {}", data);
    println!("引用计数: {}", Rc::strong_count(&data));
    
    // 克隆 Rc（增加引用计数）
    let data_clone1 = Rc::clone(&data);
    let data_clone2 = data.clone();  // 等价写法
    
    println!("克隆后引用计数: {}", Rc::strong_count(&data));
    println!("所有引用都指向同一数据:");
    println!("  data: {}", data);
    println!("  clone1: {}", data_clone1);
    println!("  clone2: {}", data_clone2);
    
    // 检查是否指向同一内存位置
    println!("内存地址相同: {}", Rc::ptr_eq(&data, &data_clone1));
    
    println!("✅ Rc 允许多个所有者共享同一数据");
}

/// 演示共享所有权
fn demonstrate_shared_ownership() {
    println!("\n🤝 共享所有权:");
    
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: Vec<Rc<Node>>,
    }
    
    impl Node {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(Node {
                value,
                children: Vec::new(),
            })
        }
    }
    
    let root = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);
    let shared_child = Node::new(4);
    
    // 创建共享结构
    let mut root_mut = Rc::try_unwrap(root).unwrap();
    
    // 先修改 child1，再添加到 root
    let mut child1_mut = Rc::try_unwrap(child1).unwrap();
    child1_mut.children.push(shared_child.clone());
    let child1 = Rc::new(child1_mut);
    
    // 添加子节点到根节点
    root_mut.children.push(child1.clone());
    root_mut.children.push(child2.clone());
    root_mut.children.push(shared_child.clone());
    
    let root = Rc::new(root_mut);
    
    println!("共享子节点的引用计数: {}", Rc::strong_count(&shared_child));
    println!("✅ Rc 实现了树形结构中的节点共享");
}

/// 演示引用计数
fn demonstrate_reference_counting() {
    println!("\n🔢 引用计数机制:");
    
    let data = Rc::new(vec![1, 2, 3, 4, 5]);
    println!("初始引用计数: {}", Rc::strong_count(&data));
    
    {
        let _clone1 = data.clone();
        println!("第一次克隆后: {}", Rc::strong_count(&data));
        
        {
            let _clone2 = data.clone();
            let _clone3 = data.clone();
            println!("多次克隆后: {}", Rc::strong_count(&data));
        } // clone2 和 clone3 在这里被销毁
        
        println!("内部作用域结束后: {}", Rc::strong_count(&data));
    } // clone1 在这里被销毁
    
    println!("最终引用计数: {}", Rc::strong_count(&data));
    
    // 当引用计数降为 0 时，数据被自动释放
    drop(data);
    println!("✅ 引用计数自动管理内存释放");
}

/// 演示 Rc 与循环引用
fn demonstrate_rc_cycles() {
    println!("\n🔄 Rc 循环引用问题:");
    
    #[derive(Debug)]
    struct CyclicNode {
        value: i32,
        parent: Option<RcWeak<CyclicNode>>,
        children: Vec<Rc<CyclicNode>>,
    }
    
    impl CyclicNode {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(CyclicNode {
                value,
                parent: None,
                children: Vec::new(),
            })
        }
    }
    
    // 创建可能的循环引用
    let parent = CyclicNode::new(1);
    let child = CyclicNode::new(2);
    
    // 这里我们使用 Weak 引用来避免循环
    println!("父节点引用计数: {}", Rc::strong_count(&parent));
    println!("子节点引用计数: {}", Rc::strong_count(&child));
    
    println!("⚠️ 直接的 Rc 循环引用会导致内存泄漏");
    println!("✅ 使用 Weak 引用可以打破循环");
}

/// Arc<T> 智能指针
fn arc_smart_pointer() {
    println!("\n📌 3. Arc<T> - 原子引用计数智能指针");
    println!("{}", "-".repeat(40));

    // 基本使用
    demonstrate_basic_arc_usage();
    
    // 多线程共享
    demonstrate_multithreaded_sharing();
    
    // Arc 性能考虑
    demonstrate_arc_performance();
}

/// 演示 Arc 的基本使用
fn demonstrate_basic_arc_usage() {
    println!("\n⚛️ Arc 基本使用:");
    
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    println!("原始 Arc: {:?}", data);
    println!("强引用计数: {}", Arc::strong_count(&data));
    
    let data_clone = Arc::clone(&data);
    println!("克隆后强引用计数: {}", Arc::strong_count(&data));
    
    // Arc 是线程安全的
    println!("Arc 是 Send: {}", std::any::type_name::<Arc<Vec<i32>>>().contains("Send"));
    println!("Arc 是 Sync: {}", std::any::type_name::<Arc<Vec<i32>>>().contains("Sync"));
    
    println!("✅ Arc 提供线程安全的引用计数");
}

/// 演示多线程共享
fn demonstrate_multithreaded_sharing() {
    println!("\n🧵 多线程共享:");
    
    let shared_data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];
    
    // 创建多个线程共享数据
    for i in 0..3 {
        let data = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            println!("线程 {} 看到的数据: {:?}", i, data);
            println!("线程 {} 中的引用计数: {}", i, Arc::strong_count(&data));
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("主线程中的最终引用计数: {}", Arc::strong_count(&shared_data));
    println!("✅ Arc 实现了跨线程的安全数据共享");
}

/// 演示 Arc 性能考虑
fn demonstrate_arc_performance() {
    println!("\n⚡ Arc 性能考虑:");
    
    // Arc 的原子操作开销
    let start = std::time::Instant::now();
    let arc_data = Arc::new(42);
    
    for _ in 0..1000 {
        let _clone = Arc::clone(&arc_data);
    }
    
    let arc_time = start.elapsed();
    
    // 对比 Rc 的性能
    let start = std::time::Instant::now();
    let rc_data = Rc::new(42);
    
    for _ in 0..1000 {
        let _clone = Rc::clone(&rc_data);
    }
    
    let rc_time = start.elapsed();
    
    println!("Arc 克隆时间: {:?}", arc_time);
    println!("Rc 克隆时间: {:?}", rc_time);
    println!("Arc 开销比 Rc 高约 {:.2}x", arc_time.as_nanos() as f64 / rc_time.as_nanos() as f64);
    
    println!("✅ Arc 的原子操作有额外开销，但提供线程安全");
}

/// RefCell<T> 智能指针
fn refcell_smart_pointer() {
    println!("\n📌 4. RefCell<T> - 内部可变性智能指针");
    println!("{}", "-".repeat(40));

    // 基本使用
    demonstrate_basic_refcell_usage();
    
    // 运行时借用检查
    demonstrate_runtime_borrow_checking();
    
    // RefCell 与 Rc 结合
    demonstrate_refcell_with_rc();
    
    // 内部可变性模式
    demonstrate_interior_mutability_patterns();
}

/// 演示 RefCell 的基本使用
fn demonstrate_basic_refcell_usage() {
    println!("\n🔬 RefCell 基本使用:");
    
    let data = RefCell::new(vec![1, 2, 3]);
    
    // 不可变借用
    {
        let borrowed = data.borrow();
        println!("不可变借用: {:?}", *borrowed);
        println!("借用计数: {}", data.try_borrow().map(|_| "成功").unwrap_or("失败"));
    } // 借用在这里结束
    
    // 可变借用
    {
        let mut borrowed_mut = data.borrow_mut();
        borrowed_mut.push(4);
        println!("可变借用后: {:?}", *borrowed_mut);
    } // 可变借用在这里结束
    
    // 再次不可变借用
    let final_borrow = data.borrow();
    println!("最终状态: {:?}", *final_borrow);
    
    println!("✅ RefCell 提供运行时借用检查");
}

/// 演示运行时借用检查
fn demonstrate_runtime_borrow_checking() {
    println!("\n⚠️ 运行时借用检查:");
    
    let data = RefCell::new(42);
    
    // 安全的借用模式
    let borrow1 = data.try_borrow();
    match borrow1 {
        Ok(val) => {
            println!("成功借用: {}", *val);
            
            // 尝试同时进行可变借用
            let borrow_mut = data.try_borrow_mut();
            match borrow_mut {
                Ok(_) => println!("意外：可变借用成功"),
                Err(e) => println!("预期：可变借用失败 - {:?}", e),
            }
        }
        Err(e) => println!("借用失败: {:?}", e),
    }
    
    // 演示 panic 情况（注释掉以避免程序崩溃）
    /*
    let _borrow = data.borrow();
    let _borrow_mut = data.borrow_mut();  // 这会 panic!
    */
    
    println!("✅ RefCell 在运行时强制执行借用规则");
}

/// 演示 RefCell 与 Rc 结合
fn demonstrate_refcell_with_rc() {
    println!("\n🤝 RefCell 与 Rc 结合:");
    
    #[derive(Debug)]
    struct MutableNode {
        value: i32,
        children: Vec<Rc<RefCell<MutableNode>>>,
    }
    
    impl MutableNode {
        fn new(value: i32) -> Rc<RefCell<Self>> {
            Rc::new(RefCell::new(MutableNode {
                value,
                children: Vec::new(),
            }))
        }
        
        fn add_child(parent: &Rc<RefCell<Self>>, child: Rc<RefCell<Self>>) {
            parent.borrow_mut().children.push(child);
        }
        
        fn set_value(node: &Rc<RefCell<Self>>, value: i32) {
            node.borrow_mut().value = value;
        }
        
        fn print_tree(node: &Rc<RefCell<Self>>, depth: usize) {
            let indent = "  ".repeat(depth);
            let borrowed = node.borrow();
            println!("{}节点值: {}", indent, borrowed.value);
            
            for child in &borrowed.children {
                Self::print_tree(child, depth + 1);
            }
        }
    }
    
    let root = MutableNode::new(1);
    let child1 = MutableNode::new(2);
    let child2 = MutableNode::new(3);
    
    MutableNode::add_child(&root, child1.clone());
    MutableNode::add_child(&root, child2.clone());
    
    println!("初始树结构:");
    MutableNode::print_tree(&root, 0);
    
    // 修改节点值
    MutableNode::set_value(&child1, 20);
    MutableNode::set_value(&child2, 30);
    
    println!("\n修改后的树结构:");
    MutableNode::print_tree(&root, 0);
    
    println!("✅ Rc<RefCell<T>> 实现了共享的可变数据结构");
}

/// 演示内部可变性模式
fn demonstrate_interior_mutability_patterns() {
    println!("\n🎭 内部可变性模式:");
    
    // 缓存模式
    struct Cache {
        data: RefCell<HashMap<String, String>>,
    }
    
    impl Cache {
        fn new() -> Self {
            Cache {
                data: RefCell::new(HashMap::new()),
            }
        }
        
        fn get(&self, key: &str) -> Option<String> {
            self.data.borrow().get(key).cloned()
        }
        
        fn set(&self, key: String, value: String) {
            self.data.borrow_mut().insert(key, value);
        }
        
        fn len(&self) -> usize {
            self.data.borrow().len()
        }
    }
    
    let cache = Cache::new();
    println!("缓存初始大小: {}", cache.len());
    
    cache.set("key1".to_string(), "value1".to_string());
    cache.set("key2".to_string(), "value2".to_string());
    
    println!("缓存大小: {}", cache.len());
    println!("获取 key1: {:?}", cache.get("key1"));
    println!("获取 key3: {:?}", cache.get("key3"));
    
    println!("✅ 内部可变性允许在不可变引用下修改数据");
}

/// Mutex<T> 智能指针
fn mutex_smart_pointer() {
    println!("\n📌 5. Mutex<T> - 互斥锁智能指针");
    println!("{}", "-".repeat(40));

    // 基本使用
    demonstrate_basic_mutex_usage();
    
    // 多线程互斥
    demonstrate_multithreaded_mutex();
    
    // 死锁预防
    demonstrate_deadlock_prevention();
}

/// 演示 Mutex 的基本使用
fn demonstrate_basic_mutex_usage() {
    println!("\n🔒 Mutex 基本使用:");
    
    let data = Mutex::new(42);
    
    // 获取锁并修改数据
    {
        let mut locked_data = data.lock().unwrap();
        println!("原始值: {}", *locked_data);
        *locked_data = 100;
        println!("修改后: {}", *locked_data);
    } // 锁在这里自动释放
    
    // 再次获取锁
    let locked_data = data.lock().unwrap();
    println!("最终值: {}", *locked_data);
    
    println!("✅ Mutex 提供了互斥访问保护");
}

/// 演示多线程互斥
fn demonstrate_multithreaded_mutex() {
    println!("\n🧵 多线程互斥:");
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    // 创建多个线程同时修改计数器
    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for j in 0..10 {
                let mut num = counter.lock().unwrap();
                *num += 1;
                println!("线程 {} 第 {} 次: 计数器 = {}", i, j + 1, *num);
                thread::sleep(Duration::from_millis(10));
            }
        });
        handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_count = counter.lock().unwrap();
    println!("最终计数: {}", *final_count);
    
    println!("✅ Mutex 确保了多线程环境下的数据一致性");
}

/// 演示死锁预防
fn demonstrate_deadlock_prevention() {
    println!("\n💀 死锁预防:");
    
    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));
    
    let mutex1_clone = Arc::clone(&mutex1);
    let mutex2_clone = Arc::clone(&mutex2);
    
    // 线程1：按顺序获取锁
    let handle1 = thread::spawn(move || {
        let _lock1 = mutex1_clone.lock().unwrap();
        println!("线程1获得锁1");
        thread::sleep(Duration::from_millis(50));
        
        let _lock2 = mutex2_clone.lock().unwrap();
        println!("线程1获得锁2");
    });
    
    let mutex1_clone2 = Arc::clone(&mutex1);
    let mutex2_clone2 = Arc::clone(&mutex2);
    
    // 线程2：按相同顺序获取锁（避免死锁）
    let handle2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        let _lock1 = mutex1_clone2.lock().unwrap();
        println!("线程2获得锁1");
        
        let _lock2 = mutex2_clone2.lock().unwrap();
        println!("线程2获得锁2");
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    println!("✅ 通过一致的锁获取顺序避免死锁");
}

/// RwLock<T> 智能指针
fn rwlock_smart_pointer() {
    println!("\n📌 6. RwLock<T> - 读写锁智能指针");
    println!("{}", "-".repeat(40));

    // 基本使用
    demonstrate_basic_rwlock_usage();
    
    // 多读者单写者
    demonstrate_multiple_readers_single_writer();
    
    // 性能比较
    demonstrate_rwlock_performance();
}

/// 演示 RwLock 的基本使用
fn demonstrate_basic_rwlock_usage() {
    println!("\n📖 RwLock 基本使用:");
    
    let data = RwLock::new(vec![1, 2, 3, 4, 5]);
    
    // 读取数据
    {
        let read_guard = data.read().unwrap();
        println!("读取数据: {:?}", *read_guard);
        println!("数据长度: {}", read_guard.len());
    } // 读锁自动释放
    
    // 写入数据
    {
        let mut write_guard = data.write().unwrap();
        write_guard.push(6);
        println!("写入后: {:?}", *write_guard);
    } // 写锁自动释放
    
    // 再次读取
    let read_guard = data.read().unwrap();
    println!("最终数据: {:?}", *read_guard);
    
    println!("✅ RwLock 支持多读者或单写者模式");
}

/// 演示多读者单写者
fn demonstrate_multiple_readers_single_writer() {
    println!("\n👥 多读者单写者:");
    
    let data = Arc::new(RwLock::new(0));
    let mut handles = vec![];
    
    // 创建多个读者线程
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            for j in 0..5 {
                let read_guard = data.read().unwrap();
                println!("读者 {} 第 {} 次读取: {}", i, j + 1, *read_guard);
                thread::sleep(Duration::from_millis(100));
            }
        });
        handles.push(handle);
    }
    
    // 创建一个写者线程
    let data_writer = Arc::clone(&data);
    let writer_handle = thread::spawn(move || {
        for i in 1..=3 {
            thread::sleep(Duration::from_millis(200));
            let mut write_guard = data_writer.write().unwrap();
            *write_guard = i * 10;
            println!("写者更新数据为: {}", *write_guard);
        }
    });
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    writer_handle.join().unwrap();
    
    let final_value = data.read().unwrap();
    println!("最终值: {}", *final_value);
    
    println!("✅ RwLock 允许并发读取，独占写入");
}

/// 演示 RwLock 性能
fn demonstrate_rwlock_performance() {
    println!("\n⚡ RwLock vs Mutex 性能:");
    
    const ITERATIONS: usize = 1000;
    
    // RwLock 读取性能
    let rwlock_data = Arc::new(RwLock::new(42));
    let start = std::time::Instant::now();
    
    let mut handles = vec![];
    for _ in 0..4 {
        let data = Arc::clone(&rwlock_data);
        let handle = thread::spawn(move || {
            for _ in 0..ITERATIONS {
                let _guard = data.read().unwrap();
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let rwlock_time = start.elapsed();
    
    // Mutex 读取性能
    let mutex_data = Arc::new(Mutex::new(42));
    let start = std::time::Instant::now();
    
    let mut handles = vec![];
    for _ in 0..4 {
        let data = Arc::clone(&mutex_data);
        let handle = thread::spawn(move || {
            for _ in 0..ITERATIONS {
                let _guard = data.lock().unwrap();
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let mutex_time = start.elapsed();
    
    println!("RwLock 并发读取时间: {:?}", rwlock_time);
    println!("Mutex 串行访问时间: {:?}", mutex_time);
    println!("RwLock 性能提升: {:.2}x", mutex_time.as_nanos() as f64 / rwlock_time.as_nanos() as f64);
    
    println!("✅ RwLock 在读多写少的场景下性能更好");
}

/// 弱引用
fn weak_references() {
    println!("\n📌 7. Weak<T> - 弱引用智能指针");
    println!("{}", "-".repeat(40));

    // 基本使用
    demonstrate_basic_weak_usage();
    
    // 打破循环引用
    demonstrate_breaking_cycles();
    
    // 观察者模式
    demonstrate_observer_pattern();
}

/// 演示 Weak 的基本使用
fn demonstrate_basic_weak_usage() {
    println!("\n🔗 Weak 基本使用:");
    
    let strong_ref = Rc::new(String::from("Strong reference"));
    println!("强引用计数: {}", Rc::strong_count(&strong_ref));
    
    // 创建弱引用
    let weak_ref = Rc::downgrade(&strong_ref);
    println!("强引用计数: {}, 弱引用计数: {}", 
             Rc::strong_count(&strong_ref), 
             Rc::weak_count(&strong_ref));
    
    // 通过弱引用访问数据
    match weak_ref.upgrade() {
        Some(strong) => println!("通过弱引用访问: {}", strong),
        None => println!("强引用已被释放"),
    }
    
    // 释放强引用
    drop(strong_ref);
    
    // 尝试再次通过弱引用访问
    match weak_ref.upgrade() {
        Some(strong) => println!("通过弱引用访问: {}", strong),
        None => println!("强引用已被释放，弱引用无效"),
    }
    
    println!("✅ Weak 引用不影响对象的生命周期");
}

/// 演示打破循环引用
fn demonstrate_breaking_cycles() {
    println!("\n🔄 打破循环引用:");
    
    #[derive(Debug)]
    struct Parent {
        children: Vec<Rc<Child>>,
    }
    
    #[derive(Debug)]
    struct Child {
        parent: RcWeak<Parent>,
        name: String,
    }
    
    impl Parent {
        fn new() -> Rc<Self> {
            Rc::new(Parent {
                children: Vec::new(),
            })
        }
        
        fn add_child(parent: &Rc<Self>, name: String) -> Rc<Child> {
            let child = Rc::new(Child {
                parent: Rc::downgrade(parent),
                name,
            });
            
            // 这里需要使用 unsafe 或其他方法来修改 parent
            // 为了演示，我们简化处理
            println!("添加子节点: {}", child.name);
            child
        }
    }
    
    let parent = Parent::new();
    println!("父节点强引用计数: {}", Rc::strong_count(&parent));
    
    let child1 = Parent::add_child(&parent, "Child 1".to_string());
    let child2 = Parent::add_child(&parent, "Child 2".to_string());
    
    println!("父节点强引用计数: {}", Rc::strong_count(&parent));
    println!("父节点弱引用计数: {}", Rc::weak_count(&parent));
    
    // 子节点可以访问父节点
    if let Some(parent_ref) = child1.parent.upgrade() {
        println!("子节点 {} 访问父节点成功", child1.name);
    }
    
    drop(parent);
    
    // 父节点被释放后，子节点无法访问
    if let Some(_) = child1.parent.upgrade() {
        println!("子节点仍能访问父节点");
    } else {
        println!("父节点已释放，子节点无法访问");
    }
    
    println!("✅ Weak 引用成功打破了循环引用");
}

/// 演示观察者模式
fn demonstrate_observer_pattern() {
    println!("\n👁️ 观察者模式:");
    
    struct Subject {
        observers: Vec<RcWeak<RefCell<Observer>>>,
        state: i32,
    }
    
    struct Observer {
        id: usize,
    }
    
    impl Subject {
        fn new() -> Self {
            Subject {
                observers: Vec::new(),
                state: 0,
            }
        }
        
        fn add_observer(&mut self, observer: RcWeak<RefCell<Observer>>) {
            self.observers.push(observer);
        }
        
        fn set_state(&mut self, state: i32) {
            self.state = state;
            self.notify_observers();
        }
        
        fn notify_observers(&mut self) {
            // 清理无效的弱引用
            self.observers.retain(|weak_ref| {
                if let Some(observer) = weak_ref.upgrade() {
                    let obs = observer.borrow();
                    println!("通知观察者 {}: 状态变为 {}", obs.id, self.state);
                    true
                } else {
                    false  // 移除无效的弱引用
                }
            });
        }
    }
    
    let mut subject = Subject::new();
    
    // 创建观察者
    let observer1 = Rc::new(RefCell::new(Observer { id: 1 }));
    let observer2 = Rc::new(RefCell::new(Observer { id: 2 }));
    
    // 注册观察者
    subject.add_observer(Rc::downgrade(&observer1));
    subject.add_observer(Rc::downgrade(&observer2));
    
    // 改变状态
    subject.set_state(10);
    subject.set_state(20);
    
    // 释放一个观察者
    drop(observer1);
    
    subject.set_state(30);
    
    println!("✅ Weak 引用实现了松耦合的观察者模式");
}

/// Cow<T> 智能指针
fn cow_smart_pointer() {
    println!("\n📌 8. Cow<T> - 写时克隆智能指针");
    println!("{}", "-".repeat(40));

    // 基本使用
    demonstrate_basic_cow_usage();
    
    // 写时克隆机制
    demonstrate_copy_on_write();
    
    // 性能优化
    demonstrate_cow_performance();
}

/// 演示 Cow 的基本使用
fn demonstrate_basic_cow_usage() {
    println!("\n🐄 Cow 基本使用:");
    
    // 从借用创建 Cow
    let original = "Hello, World!";
    let cow_borrowed: Cow<str> = Cow::Borrowed(original);
    println!("借用的 Cow: {}", cow_borrowed);
    
    // 从拥有的数据创建 Cow
    let cow_owned: Cow<str> = Cow::Owned(String::from("Owned string"));
    println!("拥有的 Cow: {}", cow_owned);
    
    // 检查 Cow 的状态
    match &cow_borrowed {
        Cow::Borrowed(s) => println!("这是借用的字符串: {}", s),
        Cow::Owned(s) => println!("这是拥有的字符串: {}", s),
    }
    
    match &cow_owned {
        Cow::Borrowed(s) => println!("这是借用的字符串: {}", s),
        Cow::Owned(s) => println!("这是拥有的字符串: {}", s),
    }
    
    println!("✅ Cow 可以表示借用或拥有的数据");
}

/// 演示写时克隆机制
fn demonstrate_copy_on_write() {
    println!("\n✍️ 写时克隆机制:");
    
    fn process_string(mut input: Cow<str>) -> Cow<str> {
        if input.contains("Hello") {
            // 需要修改，触发写时克隆
            input.to_mut().push_str(", Rust!");
        }
        input
    }
    
    // 情况1：不需要修改
    let original1 = "Good morning";
    let result1 = process_string(Cow::Borrowed(original1));
    println!("不需要修改的情况:");
    println!("  原始: {}", original1);
    println!("  结果: {}", result1);
    match result1 {
        Cow::Borrowed(_) => println!("  仍然是借用"),
        Cow::Owned(_) => println!("  变成了拥有"),
    }
    
    // 情况2：需要修改
    let original2 = "Hello";
    let result2 = process_string(Cow::Borrowed(original2));
    println!("\n需要修改的情况:");
    println!("  原始: {}", original2);
    println!("  结果: {}", result2);
    match result2 {
        Cow::Borrowed(_) => println!("  仍然是借用"),
        Cow::Owned(_) => println!("  变成了拥有"),
    }
    
    println!("✅ Cow 只在需要修改时才进行克隆");
}

/// 演示 Cow 性能优化
fn demonstrate_cow_performance() {
    println!("\n⚡ Cow 性能优化:");
    
    fn process_with_cow(input: &str) -> Cow<str> {
        if input.len() > 10 {
            // 需要修改，创建新的字符串
            Cow::Owned(format!("Long: {}", input))
        } else {
            // 不需要修改，直接借用
            Cow::Borrowed(input)
        }
    }
    
    fn process_with_string(input: &str) -> String {
        if input.len() > 10 {
            format!("Long: {}", input)
        } else {
            input.to_string()  // 总是克隆
        }
    }
    
    let short_strings = vec!["Hi", "Hello", "Good", "Nice", "Cool"];
    let long_strings = vec!["This is a very long string", "Another long string here"];
    
    // 测试短字符串（不需要修改）
    let start = std::time::Instant::now();
    for _ in 0..10000 {
        for s in &short_strings {
            let _result = process_with_cow(s);
        }
    }
    let cow_time = start.elapsed();
    
    let start = std::time::Instant::now();
    for _ in 0..10000 {
        for s in &short_strings {
            let _result = process_with_string(s);
        }
    }
    let string_time = start.elapsed();
    
    println!("短字符串处理:");
    println!("  Cow 时间: {:?}", cow_time);
    println!("  String 时间: {:?}", string_time);
    println!("  Cow 性能提升: {:.2}x", string_time.as_nanos() as f64 / cow_time.as_nanos() as f64);
    
    println!("✅ Cow 在不需要修改时避免了不必要的克隆");
}

/// 自定义智能指针
fn custom_smart_pointers() {
    println!("\n📌 9. 自定义智能指针");
    println!("{}", "-".repeat(40));

    // 简单的智能指针
    demonstrate_simple_smart_pointer();
    
    // 带有自定义行为的智能指针
    demonstrate_custom_behavior_smart_pointer();
    
    // 引用计数智能指针
    demonstrate_custom_reference_counting();
}

/// 演示简单的智能指针
fn demonstrate_simple_smart_pointer() {
    use std::ops::Deref;
    
    println!("\n🔧 简单智能指针:");
    
    struct MyBox<T> {
        data: T,
    }
    
    impl<T> MyBox<T> {
        fn new(data: T) -> Self {
            MyBox { data }
        }
    }
    
    impl<T> Deref for MyBox<T> {
        type Target = T;
        
        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }
    
    impl<T> Drop for MyBox<T> {
        fn drop(&mut self) {
            println!("释放 MyBox");
        }
    }
    
    let my_box = MyBox::new(String::from("Custom smart pointer"));
    println!("通过智能指针访问: {}", *my_box);
    println!("调用方法: {}", my_box.len());
    
    // my_box 在这里被自动释放
    println!("✅ 自定义智能指针实现了 Deref 和 Drop");
}

/// 演示带有自定义行为的智能指针
fn demonstrate_custom_behavior_smart_pointer() {
    use std::ops::{Deref, DerefMut};
    
    println!("\n🎭 自定义行为智能指针:");
    
    struct LoggingBox<T> {
        data: T,
        access_count: Cell<usize>,
    }
    
    impl<T> LoggingBox<T> {
        fn new(data: T) -> Self {
            println!("创建 LoggingBox");
            LoggingBox {
                data,
                access_count: Cell::new(0),
            }
        }
        
        fn access_count(&self) -> usize {
            self.access_count.get()
        }
    }
    
    impl<T> Deref for LoggingBox<T> {
        type Target = T;
        
        fn deref(&self) -> &Self::Target {
            let count = self.access_count.get();
            self.access_count.set(count + 1);
            println!("访问数据，第 {} 次", count + 1);
            &self.data
        }
    }
    
    impl<T> Drop for LoggingBox<T> {
        fn drop(&mut self) {
            println!("释放 LoggingBox，总共访问了 {} 次", self.access_count.get());
        }
    }
    
    let logging_box = LoggingBox::new(vec![1, 2, 3, 4, 5]);
    
    println!("第一次访问: {:?}", *logging_box);
    println!("第二次访问长度: {}", logging_box.len());
    println!("第三次访问第一个元素: {}", logging_box[0]);
    
    println!("当前访问次数: {}", logging_box.access_count());
    
    println!("✅ 自定义智能指针可以添加额外的行为");
}

/// 演示自定义引用计数
fn demonstrate_custom_reference_counting() {
    use std::cell::Cell;
    use std::ptr::NonNull;
    
    println!("\n🔢 自定义引用计数:");
    
    struct RcData<T> {
        data: T,
        ref_count: Cell<usize>,
    }
    
    struct SimpleRc<T> {
        ptr: NonNull<RcData<T>>,
    }
    
    impl<T> SimpleRc<T> {
        fn new(data: T) -> Self {
            let boxed = Box::new(RcData {
                data,
                ref_count: Cell::new(1),
            });
            SimpleRc {
                ptr: unsafe { NonNull::new_unchecked(Box::into_raw(boxed)) },
            }
        }
        
        fn strong_count(&self) -> usize {
            unsafe { self.ptr.as_ref().ref_count.get() }
        }
    }
    
    impl<T> Clone for SimpleRc<T> {
        fn clone(&self) -> Self {
            unsafe {
                let data = self.ptr.as_ref();
                let count = data.ref_count.get();
                data.ref_count.set(count + 1);
                println!("克隆 SimpleRc，引用计数: {} -> {}", count, count + 1);
            }
            SimpleRc { ptr: self.ptr }
        }
    }
    
    impl<T> Deref for SimpleRc<T> {
        type Target = T;
        
        fn deref(&self) -> &T {
            unsafe { &self.ptr.as_ref().data }
        }
    }
    
    impl<T> Drop for SimpleRc<T> {
        fn drop(&mut self) {
            unsafe {
                let data = self.ptr.as_ref();
                let count = data.ref_count.get();
                println!("释放 SimpleRc，引用计数: {} -> {}", count, count - 1);
                
                if count == 1 {
                    // 最后一个引用，释放数据
                    println!("释放底层数据");
                    let _ = Box::from_raw(self.ptr.as_ptr());
                } else {
                    data.ref_count.set(count - 1);
                }
            }
        }
    }
    
    let rc1 = SimpleRc::new(String::from("Custom Rc"));
    println!("创建后引用计数: {}", rc1.strong_count());
    
    {
        let rc2 = rc1.clone();
        let rc3 = rc1.clone();
        
        println!("克隆后引用计数: {}", rc1.strong_count());
        println!("所有引用的数据: {}, {}, {}", *rc1, *rc2, *rc3);
    } // rc2 和 rc3 在这里被释放
    
    println!("作用域结束后引用计数: {}", rc1.strong_count());
    
    println!("✅ 自定义引用计数智能指针工作正常");
}

/// 智能指针模式
fn smart_pointer_patterns() {
    println!("\n📌 10. 智能指针模式");
    println!("{}", "-".repeat(40));

    // RAII 模式
    demonstrate_raii_pattern();
    
    // 构建器模式
    demonstrate_builder_pattern();
    
    // 状态模式
    demonstrate_state_pattern();
}

/// 演示 RAII 模式
fn demonstrate_raii_pattern() {
    println!("\n🏗️ RAII 模式:");
    
    struct FileHandler {
        filename: String,
    }
    
    impl FileHandler {
        fn new(filename: String) -> Self {
            println!("打开文件: {}", filename);
            FileHandler { filename }
        }
        
        fn write(&self, content: &str) {
            println!("写入文件 {}: {}", self.filename, content);
        }
    }
    
    impl Drop for FileHandler {
        fn drop(&mut self) {
            println!("关闭文件: {}", self.filename);
        }
    }
    
    {
        let file = Box::new(FileHandler::new("data.txt".to_string()));
        file.write("Hello, RAII!");
    } // 文件在这里自动关闭
    
    println!("✅ RAII 确保资源自动清理");
}

/// 演示构建器模式
fn demonstrate_builder_pattern() {
    println!("\n🔨 构建器模式:");
    
    struct Config {
        host: String,
        port: u16,
        timeout: Duration,
        retries: u32,
    }
    
    struct ConfigBuilder {
        host: Option<String>,
        port: Option<u16>,
        timeout: Option<Duration>,
        retries: Option<u32>,
    }
    
    impl ConfigBuilder {
        fn new() -> Self {
            ConfigBuilder {
                host: None,
                port: None,
                timeout: None,
                retries: None,
            }
        }
        
        fn host(mut self, host: String) -> Self {
            self.host = Some(host);
            self
        }
        
        fn port(mut self, port: u16) -> Self {
            self.port = Some(port);
            self
        }
        
        fn timeout(mut self, timeout: Duration) -> Self {
            self.timeout = Some(timeout);
            self
        }
        
        fn retries(mut self, retries: u32) -> Self {
            self.retries = Some(retries);
            self
        }
        
        fn build(self) -> Box<Config> {
            Box::new(Config {
                host: self.host.unwrap_or_else(|| "localhost".to_string()),
                port: self.port.unwrap_or(8080),
                timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
                retries: self.retries.unwrap_or(3),
            })
        }
    }
    
    let config = ConfigBuilder::new()
        .host("example.com".to_string())
        .port(443)
        .timeout(Duration::from_secs(60))
        .build();
    
    println!("配置: {}:{}, 超时: {:?}, 重试: {}", 
             config.host, config.port, config.timeout, config.retries);
    
    println!("✅ 构建器模式提供了灵活的对象构造");
}

/// 演示状态模式
fn demonstrate_state_pattern() {
    println!("\n🎭 状态模式:");
    
    trait State {
        fn handle(self: Box<Self>) -> Box<dyn State>;
        fn description(&self) -> &str;
    }
    
    struct IdleState;
    struct RunningState;
    struct StoppedState;
    
    impl State for IdleState {
        fn handle(self: Box<Self>) -> Box<dyn State> {
            println!("从空闲状态转换到运行状态");
            Box::new(RunningState)
        }
        
        fn description(&self) -> &str {
            "空闲"
        }
    }
    
    impl State for RunningState {
        fn handle(self: Box<Self>) -> Box<dyn State> {
            println!("从运行状态转换到停止状态");
            Box::new(StoppedState)
        }
        
        fn description(&self) -> &str {
            "运行中"
        }
    }
    
    impl State for StoppedState {
        fn handle(self: Box<Self>) -> Box<dyn State> {
            println!("从停止状态转换到空闲状态");
            Box::new(IdleState)
        }
        
        fn description(&self) -> &str {
            "已停止"
        }
    }
    
    struct StateMachine {
        state: Box<dyn State>,
    }
    
    impl StateMachine {
        fn new() -> Self {
            StateMachine {
                state: Box::new(IdleState),
            }
        }
        
        fn transition(&mut self) {
            println!("当前状态: {}", self.state.description());
            let old_state = std::mem::replace(&mut self.state, Box::new(IdleState));
            self.state = old_state.handle();
            println!("新状态: {}", self.state.description());
        }
    }
    
    let mut machine = StateMachine::new();
    machine.transition();
    machine.transition();
    machine.transition();
    
    println!("✅ 状态模式通过智能指针实现了动态状态转换");
}

/// 高级智能指针技术
fn advanced_smart_pointer_techniques() {
    println!("\n📌 11. 高级智能指针技术");
    println!("{}", "-".repeat(40));

    // 类型擦除
    demonstrate_type_erasure();
    
    // 智能指针组合
    demonstrate_smart_pointer_composition();
    
    // 自定义分配器
    demonstrate_custom_allocator_concept();
}

/// 演示类型擦除
fn demonstrate_type_erasure() {
    println!("\n🎭 类型擦除:");
    
    trait Drawable {
        fn draw(&self);
    }
    
    struct Circle { radius: f64 }
    struct Rectangle { width: f64, height: f64 }
    
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
    
    // 使用 Box 进行类型擦除
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 10.0, height: 8.0 }),
        Box::new(Circle { radius: 3.0 }),
    ];
    
    println!("绘制所有形状:");
    for shape in shapes {
        shape.draw();
    }
    
    println!("✅ Box<dyn Trait> 实现了类型擦除");
}

/// 演示智能指针组合
fn demonstrate_smart_pointer_composition() {
    println!("\n🔗 智能指针组合:");
    
    // Arc<Mutex<T>> - 线程安全的共享可变数据
    let shared_counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..3 {
        let counter = Arc::clone(&shared_counter);
        let handle = thread::spawn(move || {
            for j in 0..5 {
                let mut num = counter.lock().unwrap();
                *num += 1;
                println!("线程 {} 第 {} 次增加，当前值: {}", i, j + 1, *num);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_value = shared_counter.lock().unwrap();
    println!("最终计数: {}", *final_value);
    
    // Rc<RefCell<T>> - 单线程的共享可变数据
    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));
    let data_clone = Rc::clone(&shared_data);
    
    {
        let mut data = shared_data.borrow_mut();
        data.push(4);
    }
    
    {
        let mut data = data_clone.borrow_mut();
        data.push(5);
    }
    
    println!("共享数据: {:?}", shared_data.borrow());
    
    println!("✅ 智能指针组合提供了强大的内存管理能力");
}

/// 演示自定义分配器概念
fn demonstrate_custom_allocator_concept() {
    println!("\n🏭 自定义分配器概念:");
    
    // 模拟自定义分配器的概念
    struct PoolAllocator {
        allocated: Cell<usize>,
        deallocated: Cell<usize>,
    }
    
    impl PoolAllocator {
        fn new() -> Self {
            PoolAllocator {
                allocated: Cell::new(0),
                deallocated: Cell::new(0),
            }
        }
        
        fn allocate<T>(&self, value: T) -> PoolBox<T> {
            let count = self.allocated.get();
            self.allocated.set(count + 1);
            println!("池分配器分配内存，第 {} 次分配", count + 1);
            PoolBox {
                data: Box::new(value),
                allocator: self,
            }
        }
        
        fn deallocate(&self) {
            let count = self.deallocated.get();
            self.deallocated.set(count + 1);
            println!("池分配器释放内存，第 {} 次释放", count + 1);
        }
        
        fn stats(&self) {
            println!("分配器统计: 分配 {} 次，释放 {} 次", 
                     self.allocated.get(), self.deallocated.get());
        }
    }
    
    struct PoolBox<'a, T> {
        data: Box<T>,
        allocator: &'a PoolAllocator,
    }
    
    impl<T> std::ops::Deref for PoolBox<'_, T> {
        type Target = T;
        
        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }
    
    impl<T> Drop for PoolBox<'_, T> {
        fn drop(&mut self) {
            self.allocator.deallocate();
        }
    }
    
    let allocator = PoolAllocator::new();
    
    {
        let box1 = allocator.allocate(String::from("Hello"));
        let box2 = allocator.allocate(vec![1, 2, 3, 4, 5]);
        
        println!("Box1: {}", *box1);
        println!("Box2: {:?}", *box2);
        
        allocator.stats();
    } // box1 和 box2 在这里被释放
    
    allocator.stats();
    
    println!("✅ 自定义分配器可以提供特殊的内存管理策略");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_basic_usage() {
        let boxed_value = Box::new(42);
        assert_eq!(*boxed_value, 42);
    }

    #[test]
    fn test_rc_reference_counting() {
        let data = Rc::new(String::from("test"));
        assert_eq!(Rc::strong_count(&data), 1);
        
        let clone = Rc::clone(&data);
        assert_eq!(Rc::strong_count(&data), 2);
        
        drop(clone);
        assert_eq!(Rc::strong_count(&data), 1);
    }

    #[test]
    fn test_refcell_borrowing() {
        let cell = RefCell::new(42);
        
        {
            let borrowed = cell.borrow();
            assert_eq!(*borrowed, 42);
        }
        
        {
            let mut borrowed_mut = cell.borrow_mut();
            *borrowed_mut = 100;
        }
        
        let borrowed = cell.borrow();
        assert_eq!(*borrowed, 100);
    }

    #[test]
    fn test_weak_references() {
        let strong = Rc::new(String::from("test"));
        let weak = Rc::downgrade(&strong);
        
        assert!(weak.upgrade().is_some());
        
        drop(strong);
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn test_cow_clone_on_write() {
        let original = "Hello";
        let mut cow = Cow::Borrowed(original);
        
        // 不修改时仍然是借用
        match cow {
            Cow::Borrowed(_) => {},
            Cow::Owned(_) => panic!("Should be borrowed"),
        }
        
        // 修改时变成拥有
        cow.to_mut().push_str(", World!");
        match cow {
            Cow::Borrowed(_) => panic!("Should be owned"),
            Cow::Owned(ref s) => assert_eq!(s, "Hello, World!"),
        }
    }
}