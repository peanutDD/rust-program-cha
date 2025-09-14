//! Rc 与 Arc 智能指针全面教程
//!
//! 本教程深入讲解 Rust 中的引用计数智能指针 Rc<T> 和 Arc<T>，
//! 涵盖共享所有权、引用计数机制、线程安全、弱引用等核心概念。

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    println!("=== Rc 与 Arc 智能指针教程 ===");

    // 1. 引用计数基础概念
    demonstrate_reference_counting();

    // 2. Rc<T> 单线程共享所有权
    demonstrate_rc_patterns();

    // 3. Arc<T> 多线程共享所有权
    demonstrate_arc_patterns();

    // 4. 弱引用与循环引用
    demonstrate_weak_references();

    // 5. 内部可变性结合
    demonstrate_interior_mutability();

    // 6. 实际应用场景
    demonstrate_practical_scenarios();

    // 7. 高级模式与技巧
    demonstrate_advanced_patterns();

    // 8. 性能分析与优化
    demonstrate_performance_analysis();

    println!("\n=== Rc 与 Arc 教程总结 ===");
    println!("✅ 引用计数确保共享数据的安全管理");
    println!("✅ Rc<T> 提供单线程环境下的共享所有权");
    println!("✅ Arc<T> 提供多线程环境下的原子引用计数");
    println!("✅ Weak<T> 解决循环引用问题");
    println!("✅ 结合 RefCell/Mutex 实现内部可变性");
}

/// 1. 引用计数基础概念演示
fn demonstrate_reference_counting() {
    println!("\n1. 引用计数基础概念");
    println!("  📊 引用计数机制演示");

    // 创建 Rc 智能指针
    let data = Rc::new(String::from("共享数据"));
    println!("    🔢 初始引用计数: {}", Rc::strong_count(&data));

    // 克隆增加引用计数
    let data_clone1 = Rc::clone(&data);
    println!("    🔢 克隆后引用计数: {}", Rc::strong_count(&data));

    let data_clone2 = data.clone(); // 等价于 Rc::clone(&data)
    println!("    🔢 再次克隆后引用计数: {}", Rc::strong_count(&data));

    // 验证数据共享
    println!("    📋 原始数据: {}", data);
    println!("    📋 克隆数据1: {}", data_clone1);
    println!("    📋 克隆数据2: {}", data_clone2);

    // 验证内存地址相同
    println!("    🏠 原始数据地址: {:p}", data.as_ptr());
    println!("    🏠 克隆数据1地址: {:p}", data_clone1.as_ptr());
    println!("    🏠 克隆数据2地址: {:p}", data_clone2.as_ptr());

    // 作用域结束时引用计数递减
    {
        let temp_clone = Rc::clone(&data);
        println!("    🔢 临时克隆后引用计数: {}", Rc::strong_count(&data));
    } // temp_clone 在此处被销毁

    println!("    🔢 临时克隆销毁后引用计数: {}", Rc::strong_count(&data));
}

/// 2. Rc<T> 单线程共享所有权模式
fn demonstrate_rc_patterns() {
    println!("\n2. Rc<T> 单线程共享所有权");

    // 2.1 链表节点共享
    demonstrate_shared_list();

    // 2.2 树结构共享
    demonstrate_shared_tree();

    // 2.3 图结构共享
    demonstrate_shared_graph();
}

/// 共享链表演示
fn demonstrate_shared_list() {
    println!("  🔗 共享链表演示");

    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};

    // 创建共享的尾部
    let shared_tail = Rc::new(Cons(15, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("    📋 共享尾部: {:?}", shared_tail);
    println!(
        "    🔢 共享尾部引用计数: {}",
        Rc::strong_count(&shared_tail)
    );

    // 创建两个不同的链表，共享同一个尾部
    let list_a = Cons(3, Rc::clone(&shared_tail));
    println!("    📋 链表A: {:?}", list_a);
    println!(
        "    🔢 共享尾部引用计数: {}",
        Rc::strong_count(&shared_tail)
    );

    let list_b = Cons(4, Rc::clone(&shared_tail));
    println!("    📋 链表B: {:?}", list_b);
    println!(
        "    🔢 共享尾部引用计数: {}",
        Rc::strong_count(&shared_tail)
    );
}

/// 共享树结构演示
fn demonstrate_shared_tree() {
    println!("  🌳 共享树结构演示");

    #[derive(Debug)]
    struct TreeNode {
        value: i32,
        children: Vec<Rc<TreeNode>>,
    }

    impl TreeNode {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(TreeNode {
                value,
                children: Vec::new(),
            })
        }

        fn add_child(&mut self, child: Rc<TreeNode>) {
            self.children.push(child);
        }
    }

    // 创建共享的叶子节点
    let shared_leaf = TreeNode::new(100);
    println!("    🍃 共享叶子节点: {:?}", shared_leaf.value);
    println!(
        "    🔢 叶子节点引用计数: {}",
        Rc::strong_count(&shared_leaf)
    );

    // 创建父节点，共享同一个叶子
    let mut parent1 = TreeNode::new(50);
    let mut parent2 = TreeNode::new(60);

    // 注意：这里需要使用 Rc::get_mut 或 RefCell 来修改
    // 为了演示，我们直接创建新的结构
    let parent1 = Rc::new(TreeNode {
        value: 50,
        children: vec![Rc::clone(&shared_leaf)],
    });

    let parent2 = Rc::new(TreeNode {
        value: 60,
        children: vec![Rc::clone(&shared_leaf)],
    });

    println!(
        "    🌿 父节点1: 值={}, 子节点数={}",
        parent1.value,
        parent1.children.len()
    );
    println!(
        "    🌿 父节点2: 值={}, 子节点数={}",
        parent2.value,
        parent2.children.len()
    );
    println!(
        "    🔢 叶子节点引用计数: {}",
        Rc::strong_count(&shared_leaf)
    );
}

/// 共享图结构演示
fn demonstrate_shared_graph() {
    println!("  🕸️ 共享图结构演示");

    #[derive(Debug)]
    struct GraphNode {
        id: usize,
        value: String,
        neighbors: RefCell<Vec<Rc<GraphNode>>>,
    }

    impl GraphNode {
        fn new(id: usize, value: String) -> Rc<Self> {
            Rc::new(GraphNode {
                id,
                value,
                neighbors: RefCell::new(Vec::new()),
            })
        }

        fn add_neighbor(&self, neighbor: Rc<GraphNode>) {
            self.neighbors.borrow_mut().push(neighbor);
        }

        fn get_neighbors_count(&self) -> usize {
            self.neighbors.borrow().len()
        }
    }

    // 创建图节点
    let node_a = GraphNode::new(1, "节点A".to_string());
    let node_b = GraphNode::new(2, "节点B".to_string());
    let node_c = GraphNode::new(3, "节点C".to_string());

    // 建立连接关系
    node_a.add_neighbor(Rc::clone(&node_b));
    node_a.add_neighbor(Rc::clone(&node_c));
    node_b.add_neighbor(Rc::clone(&node_c));

    println!("    🔗 节点A邻居数: {}", node_a.get_neighbors_count());
    println!("    🔗 节点B邻居数: {}", node_b.get_neighbors_count());
    println!("    🔗 节点C邻居数: {}", node_c.get_neighbors_count());

    println!("    🔢 节点B引用计数: {}", Rc::strong_count(&node_b));
    println!("    🔢 节点C引用计数: {}", Rc::strong_count(&node_c));
}

/// 3. Arc<T> 多线程共享所有权模式
fn demonstrate_arc_patterns() {
    println!("\n3. Arc<T> 多线程共享所有权");

    // 3.1 基础多线程共享
    demonstrate_basic_arc();

    // 3.2 多线程计算
    demonstrate_multithreaded_computation();

    // 3.3 共享状态管理
    demonstrate_shared_state();
}

/// 基础 Arc 多线程共享演示
fn demonstrate_basic_arc() {
    println!("  🧵 基础多线程共享演示");

    let shared_data = Arc::new(vec![1, 2, 3, 4, 5]);
    println!("    📊 共享数据: {:?}", shared_data);
    println!("    🔢 初始引用计数: {}", Arc::strong_count(&shared_data));

    let mut handles = vec![];

    // 创建多个线程共享数据
    for i in 0..3 {
        let data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            println!("    🧵 线程{}: 访问共享数据 {:?}", i, data_clone);
            println!(
                "    🔢 线程{}中引用计数: {}",
                i,
                Arc::strong_count(&data_clone)
            );

            // 模拟一些工作
            thread::sleep(Duration::from_millis(100));

            let sum: i32 = data_clone.iter().sum();
            println!("    📊 线程{}: 数据和 = {}", i, sum);
        });
        handles.push(handle);
    }

    println!(
        "    🔢 创建线程后引用计数: {}",
        Arc::strong_count(&shared_data)
    );

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "    🔢 线程完成后引用计数: {}",
        Arc::strong_count(&shared_data)
    );
}

/// 多线程计算演示
fn demonstrate_multithreaded_computation() {
    println!("  🧮 多线程计算演示");

    let numbers = Arc::new((1..=1000).collect::<Vec<i32>>());
    let num_threads = 4;
    let chunk_size = numbers.len() / num_threads;

    println!("    📊 计算 1-1000 的和，使用 {} 个线程", num_threads);

    let mut handles = vec![];

    for i in 0..num_threads {
        let numbers_clone = Arc::clone(&numbers);
        let start = i * chunk_size;
        let end = if i == num_threads - 1 {
            numbers.len()
        } else {
            (i + 1) * chunk_size
        };

        let handle = thread::spawn(move || {
            let partial_sum: i32 = numbers_clone[start..end].iter().sum();
            println!(
                "    🧵 线程{}: 计算范围 {}-{}, 部分和 = {}",
                i,
                start + 1,
                end,
                partial_sum
            );
            partial_sum
        });

        handles.push(handle);
    }

    let mut total_sum = 0;
    for handle in handles {
        total_sum += handle.join().unwrap();
    }

    println!("    📊 总和: {}", total_sum);
    println!("    ✅ 验证: 1-1000的和应该是 {}", (1000 * 1001) / 2);
}

/// 共享状态管理演示
fn demonstrate_shared_state() {
    println!("  🏪 共享状态管理演示");

    #[derive(Debug)]
    struct Counter {
        value: Mutex<i32>,
        name: String,
    }

    impl Counter {
        fn new(name: String) -> Self {
            Counter {
                value: Mutex::new(0),
                name,
            }
        }

        fn increment(&self) {
            let mut value = self.value.lock().unwrap();
            *value += 1;
            println!("    📈 {} 计数器增加到: {}", self.name, *value);
        }

        fn get_value(&self) -> i32 {
            *self.value.lock().unwrap()
        }
    }

    let shared_counter = Arc::new(Counter::new("共享计数器".to_string()));
    let mut handles = vec![];

    // 多个线程同时修改共享状态
    for i in 0..5 {
        let counter_clone = Arc::clone(&shared_counter);
        let handle = thread::spawn(move || {
            for j in 0..3 {
                counter_clone.increment();
                thread::sleep(Duration::from_millis(50));
            }
            println!("    🧵 线程{} 完成工作", i);
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("    📊 最终计数器值: {}", shared_counter.get_value());
}

/// 4. 弱引用与循环引用演示
fn demonstrate_weak_references() {
    println!("\n4. 弱引用与循环引用");

    // 4.1 循环引用问题
    demonstrate_circular_reference_problem();

    // 4.2 弱引用解决方案
    demonstrate_weak_reference_solution();

    // 4.3 父子关系模式
    demonstrate_parent_child_pattern();
}

/// 循环引用问题演示
fn demonstrate_circular_reference_problem() {
    println!("  ⚠️ 循环引用问题演示");

    #[derive(Debug)]
    struct Node {
        value: i32,
        next: RefCell<Option<Rc<Node>>>,
    }

    impl Node {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(Node {
                value,
                next: RefCell::new(None),
            })
        }
    }

    impl Drop for Node {
        fn drop(&mut self) {
            println!("    🗑️ 节点 {} 被销毁", self.value);
        }
    }

    // 创建两个节点
    let node_a = Node::new(1);
    let node_b = Node::new(2);

    println!("    🔢 节点A引用计数: {}", Rc::strong_count(&node_a));
    println!("    🔢 节点B引用计数: {}", Rc::strong_count(&node_b));

    // 创建循环引用
    *node_a.next.borrow_mut() = Some(Rc::clone(&node_b));
    *node_b.next.borrow_mut() = Some(Rc::clone(&node_a));

    println!(
        "    🔢 循环引用后节点A引用计数: {}",
        Rc::strong_count(&node_a)
    );
    println!(
        "    🔢 循环引用后节点B引用计数: {}",
        Rc::strong_count(&node_b)
    );

    println!("    ⚠️ 注意：这些节点不会被自动销毁，造成内存泄漏！");

    // 手动打破循环引用
    *node_a.next.borrow_mut() = None;
    *node_b.next.borrow_mut() = None;

    println!("    ✅ 手动打破循环引用");
}

/// 弱引用解决方案演示
fn demonstrate_weak_reference_solution() {
    println!("  💪 弱引用解决方案演示");

    #[derive(Debug)]
    struct SafeNode {
        value: i32,
        next: RefCell<Option<Rc<SafeNode>>>,
        prev: RefCell<Option<Weak<SafeNode>>>, // 使用弱引用
    }

    impl SafeNode {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(SafeNode {
                value,
                next: RefCell::new(None),
                prev: RefCell::new(None),
            })
        }
    }

    impl Drop for SafeNode {
        fn drop(&mut self) {
            println!("    🗑️ 安全节点 {} 被销毁", self.value);
        }
    }

    impl Clone for SafeNode {
        fn clone(&self) -> Self {
            SafeNode {
                value: self.value,
                next: RefCell::new(None),
                prev: RefCell::new(None),
            }
        }
    }

    // 创建安全的双向链表
    let node_a = SafeNode::new(10);
    let node_b = SafeNode::new(20);

    println!("    🔢 安全节点A强引用计数: {}", Rc::strong_count(&node_a));
    println!("    🔢 安全节点A弱引用计数: {}", Rc::weak_count(&node_a));

    // 建立连接（使用弱引用避免循环）
    *node_a.next.borrow_mut() = Some(Rc::clone(&node_b));
    *node_b.prev.borrow_mut() = Some(Rc::downgrade(&node_a));

    println!(
        "    🔢 连接后节点A强引用计数: {}",
        Rc::strong_count(&node_a)
    );
    println!("    🔢 连接后节点A弱引用计数: {}", Rc::weak_count(&node_a));
    println!(
        "    🔢 连接后节点B强引用计数: {}",
        Rc::strong_count(&node_b)
    );

    // 验证弱引用可以升级
    if let Some(prev_node) = node_b.prev.borrow().as_ref() {
        if let Some(strong_ref) = prev_node.upgrade() {
            println!("    ✅ 弱引用成功升级，前驱节点值: {}", strong_ref.value);
        } else {
            println!("    ❌ 弱引用升级失败，前驱节点已被销毁");
        }
    }

    println!("    ✅ 作用域结束时节点会被自动销毁，无内存泄漏");
}

/// 父子关系模式演示
fn demonstrate_parent_child_pattern() {
    println!("  👨‍👧‍👦 父子关系模式演示");

    #[derive(Debug)]
    struct Parent {
        name: String,
        children: RefCell<Vec<Rc<Child>>>,
    }

    #[derive(Debug)]
    struct Child {
        name: String,
        parent: RefCell<Option<Weak<Parent>>>, // 子节点持有父节点的弱引用
    }

    impl Parent {
        fn new(name: String) -> Rc<Self> {
            Rc::new(Parent {
                name,
                children: RefCell::new(Vec::new()),
            })
        }

        fn add_child(self: &Rc<Self>, name: String) -> Rc<Child> {
            let child = Rc::new(Child {
                name,
                parent: RefCell::new(Some(Rc::downgrade(self))),
            });

            self.children.borrow_mut().push(Rc::clone(&child));
            child
        }

        fn get_children_count(&self) -> usize {
            self.children.borrow().len()
        }
    }

    impl Child {
        fn get_parent_name(&self) -> Option<String> {
            self.parent
                .borrow()
                .as_ref()
                .and_then(|parent_weak| parent_weak.upgrade())
                .map(|parent| parent.name.clone())
        }
    }

    impl Drop for Parent {
        fn drop(&mut self) {
            println!("    🗑️ 父节点 '{}' 被销毁", self.name);
        }
    }

    impl Drop for Child {
        fn drop(&mut self) {
            println!("    🗑️ 子节点 '{}' 被销毁", self.name);
        }
    }

    // 创建父子关系
    let parent = Parent::new("父亲".to_string());
    println!("    👨 创建父节点: {}", parent.name);

    let child1 = parent.add_child("孩子1".to_string());
    let child2 = parent.add_child("孩子2".to_string());

    println!("    👧 添加子节点: {}", child1.name);
    println!("    👦 添加子节点: {}", child2.name);

    println!("    📊 父节点子女数量: {}", parent.get_children_count());
    println!("    🔢 父节点强引用计数: {}", Rc::strong_count(&parent));
    println!("    🔢 父节点弱引用计数: {}", Rc::weak_count(&parent));

    // 子节点访问父节点
    if let Some(parent_name) = child1.get_parent_name() {
        println!("    👧 {} 的父亲是: {}", child1.name, parent_name);
    }

    if let Some(parent_name) = child2.get_parent_name() {
        println!("    👦 {} 的父亲是: {}", child2.name, parent_name);
    }

    println!("    ✅ 父子关系正确建立，无循环引用问题");
}

/// 5. 内部可变性结合演示
fn demonstrate_interior_mutability() {
    println!("\n5. 内部可变性结合");

    // 5.1 Rc + RefCell 模式
    demonstrate_rc_refcell();

    // 5.2 Arc + Mutex 模式
    demonstrate_arc_mutex();
}

/// Rc + RefCell 模式演示
fn demonstrate_rc_refcell() {
    println!("  🔄 Rc + RefCell 模式演示");

    #[derive(Debug)]
    struct SharedData {
        values: RefCell<Vec<i32>>,
        name: String,
    }

    impl SharedData {
        fn new(name: String) -> Rc<Self> {
            Rc::new(SharedData {
                values: RefCell::new(Vec::new()),
                name,
            })
        }

        fn add_value(&self, value: i32) {
            self.values.borrow_mut().push(value);
            println!("    ➕ {} 添加值: {}", self.name, value);
        }

        fn get_sum(&self) -> i32 {
            self.values.borrow().iter().sum()
        }

        fn get_values(&self) -> Vec<i32> {
            self.values.borrow().clone()
        }
    }

    let shared_data = SharedData::new("共享数据".to_string());
    let data_ref1 = Rc::clone(&shared_data);
    let data_ref2 = Rc::clone(&shared_data);

    println!("    🔢 初始引用计数: {}", Rc::strong_count(&shared_data));

    // 通过不同的引用修改数据
    shared_data.add_value(10);
    data_ref1.add_value(20);
    data_ref2.add_value(30);

    println!("    📊 最终数据: {:?}", shared_data.get_values());
    println!("    📊 数据总和: {}", shared_data.get_sum());

    // 验证所有引用看到的是同一份数据
    println!("    ✅ 引用1看到的总和: {}", data_ref1.get_sum());
    println!("    ✅ 引用2看到的总和: {}", data_ref2.get_sum());
}

/// Arc + Mutex 模式演示
fn demonstrate_arc_mutex() {
    println!("  🔒 Arc + Mutex 模式演示");

    let shared_data = Arc::new(Mutex::new(Vec::<i32>::new()));
    let mut handles = vec![];

    // 多个线程同时修改共享数据
    for i in 0..3 {
        let data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            for j in 0..5 {
                let value = i * 10 + j;
                {
                    let mut data = data_clone.lock().unwrap();
                    data.push(value);
                    println!("    🧵 线程{} 添加值: {}", i, value);
                }
                thread::sleep(Duration::from_millis(10));
            }
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 查看最终结果
    let final_data = shared_data.lock().unwrap();
    println!("    📊 最终数据: {:?}", *final_data);
    println!("    📊 数据长度: {}", final_data.len());
    println!("    📊 数据总和: {}", final_data.iter().sum::<i32>());
}

/// 6. 实际应用场景演示
fn demonstrate_practical_scenarios() {
    println!("\n6. 实际应用场景");

    // 6.1 缓存系统
    demonstrate_cache_system();

    // 6.2 观察者模式
    demonstrate_observer_pattern();

    // 6.3 配置管理
    demonstrate_configuration_management();
}

/// 缓存系统演示
fn demonstrate_cache_system() {
    println!("  💾 缓存系统演示");

    #[derive(Debug, Clone)]
    struct CacheEntry {
        key: String,
        value: String,
        access_count: RefCell<usize>,
    }

    impl CacheEntry {
        fn new(key: String, value: String) -> Self {
            CacheEntry {
                key,
                value,
                access_count: RefCell::new(0),
            }
        }

        fn access(&self) -> String {
            *self.access_count.borrow_mut() += 1;
            self.value.clone()
        }

        fn get_access_count(&self) -> usize {
            *self.access_count.borrow()
        }
    }

    type Cache = HashMap<String, Rc<CacheEntry>>;

    let mut cache: Cache = HashMap::new();

    // 添加缓存条目
    let entry1 = Rc::new(CacheEntry::new("user:1".to_string(), "Alice".to_string()));
    let entry2 = Rc::new(CacheEntry::new("user:2".to_string(), "Bob".to_string()));

    cache.insert("user:1".to_string(), Rc::clone(&entry1));
    cache.insert("user:2".to_string(), Rc::clone(&entry2));

    println!("    📝 缓存条目1引用计数: {}", Rc::strong_count(&entry1));
    println!("    📝 缓存条目2引用计数: {}", Rc::strong_count(&entry2));

    // 模拟多个客户端访问缓存
    let clients = vec![
        Rc::clone(&entry1),
        Rc::clone(&entry1),
        Rc::clone(&entry2),
        Rc::clone(&entry1),
    ];

    for (i, client_entry) in clients.iter().enumerate() {
        let value = client_entry.access();
        println!(
            "    👤 客户端{} 访问: {} = {}",
            i + 1,
            client_entry.key,
            value
        );
    }

    println!("    📊 用户1访问次数: {}", entry1.get_access_count());
    println!("    📊 用户2访问次数: {}", entry2.get_access_count());
    println!("    🔢 条目1最终引用计数: {}", Rc::strong_count(&entry1));
    println!("    🔢 条目2最终引用计数: {}", Rc::strong_count(&entry2));
}

/// 观察者模式演示
fn demonstrate_observer_pattern() {
    println!("  👁️ 观察者模式演示");

    trait Observer {
        fn notify(&self, message: &str);
        fn get_name(&self) -> &str;
    }

    #[derive(Debug)]
    struct ConcreteObserver {
        name: String,
        message_count: RefCell<usize>,
    }

    impl ConcreteObserver {
        fn new(name: String) -> Self {
            ConcreteObserver {
                name,
                message_count: RefCell::new(0),
            }
        }

        fn get_message_count(&self) -> usize {
            *self.message_count.borrow()
        }
    }

    impl Observer for ConcreteObserver {
        fn notify(&self, message: &str) {
            *self.message_count.borrow_mut() += 1;
            println!("    📢 {} 收到消息: {}", self.name, message);
        }

        fn get_name(&self) -> &str {
            &self.name
        }
    }

    struct Subject {
        observers: RefCell<Vec<Rc<dyn Observer>>>,
    }

    impl Subject {
        fn new() -> Self {
            Subject {
                observers: RefCell::new(Vec::new()),
            }
        }

        fn add_observer(&self, observer: Rc<dyn Observer>) {
            let name = observer.get_name();
            self.observers.borrow_mut().push(observer.clone());
            println!("    ➕ 添加观察者: {}", name);
        }

        fn notify_all(&self, message: &str) {
            println!("    📡 广播消息: {}", message);
            for observer in self.observers.borrow().iter() {
                observer.notify(message);
            }
        }

        fn get_observer_count(&self) -> usize {
            self.observers.borrow().len()
        }
    }

    // 创建主题和观察者
    let subject = Subject::new();
    let observer1 = Rc::new(ConcreteObserver::new("观察者1".to_string()));
    let observer2 = Rc::new(ConcreteObserver::new("观察者2".to_string()));
    let observer3 = Rc::new(ConcreteObserver::new("观察者3".to_string()));

    // 注册观察者
    subject.add_observer(observer1.clone() as Rc<dyn Observer>);
    subject.add_observer(observer2.clone() as Rc<dyn Observer>);
    subject.add_observer(observer3.clone() as Rc<dyn Observer>);

    println!("    📊 观察者数量: {}", subject.get_observer_count());

    // 发送通知
    subject.notify_all("系统启动");
    subject.notify_all("数据更新");
    subject.notify_all("系统关闭");

    // 查看观察者接收情况
    println!(
        "    📊 观察者1接收消息数: {}",
        observer1.get_message_count()
    );
    println!(
        "    📊 观察者2接收消息数: {}",
        observer2.get_message_count()
    );
    println!(
        "    📊 观察者3接收消息数: {}",
        observer3.get_message_count()
    );
}

/// 配置管理演示
fn demonstrate_configuration_management() {
    println!("  ⚙️ 配置管理演示");

    #[derive(Debug, Clone)]
    struct Configuration {
        database_url: String,
        api_key: String,
        max_connections: i32,
        debug_mode: bool,
    }

    impl Configuration {
        fn new() -> Self {
            Configuration {
                database_url: "localhost:5432".to_string(),
                api_key: "secret-key-123".to_string(),
                max_connections: 100,
                debug_mode: false,
            }
        }
    }

    // 全局配置实例
    let global_config = Arc::new(Configuration::new());
    println!("    ⚙️ 创建全局配置: {:?}", global_config);

    // 模拟多个服务组件共享配置
    let services = vec![
        ("数据库服务", Arc::clone(&global_config)),
        ("API服务", Arc::clone(&global_config)),
        ("缓存服务", Arc::clone(&global_config)),
        ("日志服务", Arc::clone(&global_config)),
    ];

    println!("    🔢 配置引用计数: {}", Arc::strong_count(&global_config));

    // 各服务使用配置
    for (service_name, config) in services {
        println!("    🔧 {} 使用配置:", service_name);
        println!("      📊 数据库URL: {}", config.database_url);
        println!("      🔑 API密钥: {}***", &config.api_key[..6]);
        println!("      🔗 最大连接数: {}", config.max_connections);
        println!("      🐛 调试模式: {}", config.debug_mode);
    }

    println!("    ✅ 所有服务共享同一配置实例");
}

/// 7. 高级模式与技巧演示
fn demonstrate_advanced_patterns() {
    println!("\n7. 高级模式与技巧");

    // 7.1 类型转换
    demonstrate_type_conversion();

    // 7.2 条件克隆
    demonstrate_conditional_cloning();

    // 7.3 引用计数优化
    demonstrate_reference_counting_optimization();
}

/// 类型转换演示
fn demonstrate_type_conversion() {
    println!("  🔄 类型转换演示");

    // Rc 到 Arc 的转换（需要通过数据克隆）
    let rc_data = Rc::new(vec![1, 2, 3, 4, 5]);
    println!("    📊 Rc数据: {:?}", rc_data);
    println!("    🔢 Rc引用计数: {}", Rc::strong_count(&rc_data));

    // 转换为 Arc（克隆数据）
    let arc_data = Arc::new((*rc_data).clone());
    println!("    📊 Arc数据: {:?}", arc_data);
    println!("    🔢 Arc引用计数: {}", Arc::strong_count(&arc_data));

    // 使用 try_unwrap 获取所有权
    let rc_data2 = Rc::new(String::from("独占数据"));
    println!("    📝 尝试解包前引用计数: {}", Rc::strong_count(&rc_data2));

    match Rc::try_unwrap(rc_data2) {
        Ok(data) => {
            println!("    ✅ 成功解包获得所有权: {}", data);
        }
        Err(rc) => {
            println!("    ❌ 解包失败，引用计数: {}", Rc::strong_count(&rc));
        }
    }

    // 演示失败的情况
    let rc_data3 = Rc::new(String::from("共享数据"));
    let _rc_clone = Rc::clone(&rc_data3);

    match Rc::try_unwrap(rc_data3) {
        Ok(data) => {
            println!("    ✅ 成功解包: {}", data);
        }
        Err(rc) => {
            println!(
                "    ❌ 解包失败，仍有其他引用，引用计数: {}",
                Rc::strong_count(&rc)
            );
        }
    }
}

/// 条件克隆演示
fn demonstrate_conditional_cloning() {
    println!("  🎯 条件克隆演示");

    #[derive(Debug)]
    struct ExpensiveData {
        id: usize,
        data: Vec<i32>,
    }

    impl ExpensiveData {
        fn new(id: usize, size: usize) -> Self {
            ExpensiveData {
                id,
                data: (0..size).map(|i| i as i32).collect(),
            }
        }
    }

    impl Clone for ExpensiveData {
        fn clone(&self) -> Self {
            println!("    💰 执行昂贵的克隆操作，ID: {}", self.id);
            ExpensiveData {
                id: self.id,
                data: self.data.clone(),
            }
        }
    }

    let expensive_data = Rc::new(ExpensiveData::new(1, 1000));
    println!("    📊 创建昂贵数据，ID: {}", expensive_data.id);

    // 智能克隆：只有在需要时才克隆
    let should_clone = false;

    let data_ref = if should_clone {
        println!("    🔄 条件为真，执行克隆");
        Rc::new((*expensive_data).clone())
    } else {
        println!("    🔗 条件为假，共享引用");
        Rc::clone(&expensive_data)
    };

    println!(
        "    🔢 原始数据引用计数: {}",
        Rc::strong_count(&expensive_data)
    );
    println!("    🔢 数据引用引用计数: {}", Rc::strong_count(&data_ref));

    // 使用 Rc::ptr_eq 检查是否指向同一数据
    if Rc::ptr_eq(&expensive_data, &data_ref) {
        println!("    ✅ 两个引用指向同一数据");
    } else {
        println!("    ❌ 两个引用指向不同数据");
    }
}

/// 引用计数优化演示
fn demonstrate_reference_counting_optimization() {
    println!("  ⚡ 引用计数优化演示");

    // 批量操作优化
    let data = Rc::new(vec![1, 2, 3, 4, 5]);
    let mut refs = Vec::new();

    let start_time = Instant::now();

    // 创建大量引用
    for i in 0..10000 {
        refs.push(Rc::clone(&data));
        if i % 2000 == 0 {
            println!(
                "    📈 创建 {} 个引用，当前引用计数: {}",
                i + 1,
                Rc::strong_count(&data)
            );
        }
    }

    let creation_time = start_time.elapsed();
    println!("    ⏱️ 创建 10000 个引用耗时: {:?}", creation_time);

    // 批量释放
    let release_start = Instant::now();
    refs.clear();
    let release_time = release_start.elapsed();

    println!("    ⏱️ 释放 10000 个引用耗时: {:?}", release_time);
    println!("    🔢 最终引用计数: {}", Rc::strong_count(&data));

    // 内存使用分析
    let single_ref_size = std::mem::size_of::<Rc<Vec<i32>>>();
    let data_size = std::mem::size_of::<Vec<i32>>() + data.len() * std::mem::size_of::<i32>();

    println!("    📊 单个Rc引用大小: {} 字节", single_ref_size);
    println!("    📊 实际数据大小: {} 字节", data_size);
    println!("    📊 10000个引用总大小: {} 字节", single_ref_size * 10000);
    println!(
        "    💡 内存效率: 共享一份数据，节省 {} 字节",
        data_size * 9999
    );
}

/// 8. 性能分析与优化演示
fn demonstrate_performance_analysis() {
    println!("\n8. 性能分析与优化");

    // 8.1 Rc vs Arc 性能对比
    demonstrate_rc_vs_arc_performance();

    // 8.2 引用计数开销分析
    demonstrate_reference_counting_overhead();

    // 8.3 内存使用分析
    demonstrate_memory_usage_analysis();
}

/// Rc vs Arc 性能对比
fn demonstrate_rc_vs_arc_performance() {
    println!("  🏁 Rc vs Arc 性能对比");

    let test_data = vec![1, 2, 3, 4, 5];
    let iterations = 100000;

    // Rc 性能测试
    let rc_data = Rc::new(test_data.clone());
    let start_time = Instant::now();

    let mut rc_refs = Vec::new();
    for _ in 0..iterations {
        rc_refs.push(Rc::clone(&rc_data));
    }

    let rc_time = start_time.elapsed();
    println!("    ⏱️ Rc 创建 {} 个引用耗时: {:?}", iterations, rc_time);

    // Arc 性能测试
    let arc_data = Arc::new(test_data);
    let start_time = Instant::now();

    let mut arc_refs = Vec::new();
    for _ in 0..iterations {
        arc_refs.push(Arc::clone(&arc_data));
    }

    let arc_time = start_time.elapsed();
    println!("    ⏱️ Arc 创建 {} 个引用耗时: {:?}", iterations, arc_time);

    // 性能比较
    let ratio = arc_time.as_nanos() as f64 / rc_time.as_nanos() as f64;
    println!("    📊 Arc/Rc 性能比率: {:.2}x", ratio);

    if ratio > 1.0 {
        println!(
            "    💡 Arc 由于原子操作开销，比 Rc 慢 {:.1}%",
            (ratio - 1.0) * 100.0
        );
    } else {
        println!("    💡 性能差异在误差范围内");
    }

    // 清理测试数据
    rc_refs.clear();
    arc_refs.clear();

    println!("    🔢 Rc 最终引用计数: {}", Rc::strong_count(&rc_data));
    println!("    🔢 Arc 最终引用计数: {}", Arc::strong_count(&arc_data));
}

/// 引用计数开销分析
fn demonstrate_reference_counting_overhead() {
    println!("  💰 引用计数开销分析");

    #[derive(Clone)]
    struct TestData {
        values: Vec<i32>,
    }

    impl TestData {
        fn new(size: usize) -> Self {
            TestData {
                values: (0..size as i32).collect(),
            }
        }
    }

    let data_size = 1000;
    let clone_count = 1000;

    // 直接克隆开销
    let test_data = TestData::new(data_size);
    let start_time = Instant::now();

    let mut direct_clones = Vec::new();
    for _ in 0..clone_count {
        direct_clones.push(test_data.clone());
    }

    let direct_clone_time = start_time.elapsed();
    println!(
        "    ⏱️ 直接克隆 {} 次耗时: {:?}",
        clone_count, direct_clone_time
    );

    // Rc 引用开销
    let rc_data = Rc::new(TestData::new(data_size));
    let start_time = Instant::now();

    let mut rc_clones = Vec::new();
    for _ in 0..clone_count {
        rc_clones.push(Rc::clone(&rc_data));
    }

    let rc_clone_time = start_time.elapsed();
    println!("    ⏱️ Rc 引用 {} 次耗时: {:?}", clone_count, rc_clone_time);

    // 性能提升计算
    let speedup = direct_clone_time.as_nanos() as f64 / rc_clone_time.as_nanos() as f64;
    println!("    📊 Rc 相比直接克隆快 {:.1}x", speedup);

    // 内存使用比较
    let direct_memory = std::mem::size_of::<TestData>() * clone_count
        + data_size * std::mem::size_of::<i32>() * clone_count;
    let rc_memory =
        std::mem::size_of::<Rc<TestData>>() * clone_count + data_size * std::mem::size_of::<i32>();

    println!("    📊 直接克隆内存使用: {} 字节", direct_memory);
    println!("    📊 Rc 引用内存使用: {} 字节", rc_memory);
    println!(
        "    💡 内存节省: {} 字节 ({:.1}%)",
        direct_memory - rc_memory,
        (1.0 - rc_memory as f64 / direct_memory as f64) * 100.0
    );
}

/// 内存使用分析
fn demonstrate_memory_usage_analysis() {
    println!("  📊 内存使用分析");

    // 分析不同大小数据的内存效率
    let sizes = vec![10, 100, 1000, 10000];

    for size in sizes {
        let data = Rc::new((0..size as i32).collect::<Vec<i32>>());
        let ref_count = 100;

        let mut refs = Vec::new();
        for _ in 0..ref_count {
            refs.push(Rc::clone(&data));
        }

        let data_size = size * std::mem::size_of::<i32>();
        let ref_size = std::mem::size_of::<Rc<Vec<i32>>>() * ref_count;
        let total_size = data_size + ref_size;

        let direct_clone_size = (data_size + std::mem::size_of::<Vec<i32>>()) * ref_count;
        let memory_efficiency = (1.0 - total_size as f64 / direct_clone_size as f64) * 100.0;

        println!("    📏 数据大小: {} 元素", size);
        println!("      💾 实际数据: {} 字节", data_size);
        println!("      🔗 引用开销: {} 字节", ref_size);
        println!("      📊 总内存: {} 字节", total_size);
        println!("      💡 内存效率: {:.1}% 节省", memory_efficiency);
        println!();
    }

    // 弱引用内存分析
    let strong_data = Rc::new(String::from("强引用数据"));
    let weak_ref = Rc::downgrade(&strong_data);

    println!("    🔍 引用类型内存分析:");
    println!(
        "      💪 强引用大小: {} 字节",
        std::mem::size_of::<Rc<String>>()
    );
    println!(
        "      💨 弱引用大小: {} 字节",
        std::mem::size_of::<Weak<String>>()
    );
    println!(
        "      📊 字符串数据大小: {} 字节",
        std::mem::size_of::<String>()
    );

    println!("    🔢 强引用计数: {}", Rc::strong_count(&strong_data));
    println!("    🔢 弱引用计数: {}", Rc::weak_count(&strong_data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc_basic_functionality() {
        let data = Rc::new(42);
        assert_eq!(Rc::strong_count(&data), 1);

        let data_clone = Rc::clone(&data);
        assert_eq!(Rc::strong_count(&data), 2);
        assert_eq!(*data, *data_clone);

        drop(data_clone);
        assert_eq!(Rc::strong_count(&data), 1);
    }

    #[test]
    fn test_arc_thread_safety() {
        let data = Arc::new(vec![1, 2, 3, 4, 5]);
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            assert_eq!(*data_clone, vec![1, 2, 3, 4, 5]);
            Arc::strong_count(&data_clone)
        });

        let count = handle.join().unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_weak_reference() {
        let strong = Rc::new(String::from("test"));
        let weak = Rc::downgrade(&strong);

        assert_eq!(Rc::strong_count(&strong), 1);
        assert_eq!(Rc::weak_count(&strong), 1);

        assert!(weak.upgrade().is_some());

        drop(strong);
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn test_rc_refcell_interior_mutability() {
        let data = Rc::new(RefCell::new(vec![1, 2, 3]));
        let data_clone = Rc::clone(&data);

        data.borrow_mut().push(4);
        assert_eq!(*data_clone.borrow(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_arc_mutex_thread_safety() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 10);
    }

    #[test]
    fn test_try_unwrap() {
        let data = Rc::new(String::from("test"));

        // 成功解包的情况
        match Rc::try_unwrap(data) {
            Ok(string) => assert_eq!(string, "test"),
            Err(_) => panic!("Should be able to unwrap"),
        }

        // 失败解包的情况
        let data2 = Rc::new(String::from("test2"));
        let _clone = Rc::clone(&data2);

        match Rc::try_unwrap(data2) {
            Ok(_) => panic!("Should not be able to unwrap"),
            Err(rc) => assert_eq!(Rc::strong_count(&rc), 2),
        }
    }

    #[test]
    fn test_circular_reference_prevention() {
        use std::rc::Weak;

        #[derive(Debug)]
        struct Node {
            value: i32,
            parent: RefCell<Option<Weak<Node>>>,
            children: RefCell<Vec<Rc<Node>>>,
        }

        impl Node {
            fn new(value: i32) -> Rc<Self> {
                Rc::new(Node {
                    value,
                    parent: RefCell::new(None),
                    children: RefCell::new(Vec::new()),
                })
            }
        }

        let parent = Node::new(1);
        let child = Node::new(2);

        // 建立父子关系
        *child.parent.borrow_mut() = Some(Rc::downgrade(&parent));
        parent.children.borrow_mut().push(Rc::clone(&child));

        assert_eq!(Rc::strong_count(&parent), 1);
        assert_eq!(Rc::weak_count(&parent), 1);
        assert_eq!(Rc::strong_count(&child), 2); // parent.children + child
    }

    #[test]
    fn test_performance_characteristics() {
        let data = Rc::new(vec![1, 2, 3, 4, 5]);
        let mut refs = Vec::new();

        // 测试大量引用的创建
        for _ in 0..1000 {
            refs.push(Rc::clone(&data));
        }

        assert_eq!(Rc::strong_count(&data), 1001);

        // 测试引用的释放
        refs.clear();
        assert_eq!(Rc::strong_count(&data), 1);
    }
}
