//! # Rust 弱引用与循环引用深度教程
//!
//! 本教程全面深入地分析 Rust 中的弱引用(Weak)与循环引用问题，
//! 涵盖循环引用的形成机制、内存泄漏原理、Weak 指针解决方案、
//! 实际应用场景以及最佳实践。
//!
//! ## 核心知识点
//! - 循环引用问题的本质与危害
//! - Rc<T> + RefCell<T> 组合的内存泄漏机制
//! - Weak<T> 弱引用的原理与应用
//! - 强引用计数 vs 弱引用计数
//! - 实际应用场景：树形结构、观察者模式、缓存系统
//! - Arc 与 Weak 的多线程应用
//! - 性能分析与最佳实践

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Rust 弱引用与循环引用深度教程 ===");
    println!();

    // 1. 循环引用问题分析
    demonstrate_circular_reference_problem();

    // 2. Weak 指针解决方案
    demonstrate_weak_pointer_solution();

    // 3. 引用计数机制演示
    demonstrate_reference_counting();

    // 4. 实际应用场景
    demonstrate_practical_applications();

    // 5. Arc 与 Weak 多线程应用
    demonstrate_arc_weak_threading();

    // 6. 性能分析与最佳实践
    demonstrate_performance_analysis();

    println!("\n=== 教程总结 ===");
    println!("✅ 循环引用问题：理解了 Rc<T> + RefCell<T> 组合可能导致的内存泄漏");
    println!("✅ Weak 指针解决方案：掌握了弱引用的原理和使用方法");
    println!("✅ 引用计数机制：了解了强引用和弱引用计数的管理");
    println!("✅ 实际应用场景：学会了在树形结构、观察者模式等场景中的应用");
    println!("✅ 多线程应用：掌握了 Arc 与 Weak 的线程安全使用");
    println!("✅ 性能优化：了解了内存开销和性能优化策略");
}

/// 1. 循环引用问题分析
///
/// 演示 Rc<T> + RefCell<T> 组合如何导致循环引用和内存泄漏
fn demonstrate_circular_reference_problem() {
    println!("1. === 循环引用问题分析 ===");

    // 1.1 简单循环引用示例
    println!("\n1.1 简单循环引用示例：");
    {
        #[derive(Debug)]
        struct Node {
            value: i32,
            next: RefCell<Option<Rc<Node>>>,
        }

        impl Drop for Node {
            fn drop(&mut self) {
                println!("  🗑️  Node {} 被释放", self.value);
            }
        }

        let node1 = Rc::new(Node {
            value: 1,
            next: RefCell::new(None),
        });

        let node2 = Rc::new(Node {
            value: 2,
            next: RefCell::new(None),
        });

        // 创建循环引用：node1 -> node2 -> node1
        *node1.next.borrow_mut() = Some(Rc::clone(&node2));
        *node2.next.borrow_mut() = Some(Rc::clone(&node1));

        println!("  Node1 强引用计数: {}", Rc::strong_count(&node1));
        println!("  Node2 强引用计数: {}", Rc::strong_count(&node2));
        println!("  ⚠️  创建了循环引用，节点不会被自动释放！");
    }
    println!("  📝 离开作用域后，由于循环引用，Node 对象没有被释放");

    // 1.2 链表循环引用详细分析
    println!("\n1.2 链表循环引用详细分析：");
    demonstrate_list_circular_reference();

    // 1.3 树形结构循环引用
    println!("\n1.3 树形结构循环引用：");
    demonstrate_tree_circular_reference();
}

/// 链表循环引用演示
fn demonstrate_list_circular_reference() {
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                List::Cons(_, item) => Some(item),
                List::Nil => None,
            }
        }
    }

    impl Drop for List {
        fn drop(&mut self) {
            match self {
                List::Cons(value, _) => println!("  🗑️  List::Cons({}) 被释放", value),
                List::Nil => println!("  🗑️  List::Nil 被释放"),
            }
        }
    }

    let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));
    println!("  a 初始引用计数: {}", Rc::strong_count(&a));

    let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));
    println!("  创建 b 后，a 引用计数: {}", Rc::strong_count(&a));
    println!("  b 初始引用计数: {}", Rc::strong_count(&b));

    // 创建循环引用
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("  创建循环引用后：");
    println!("    a 引用计数: {}", Rc::strong_count(&a));
    println!("    b 引用计数: {}", Rc::strong_count(&b));
    println!("  ⚠️  a -> b -> a 形成循环，内存泄漏！");
}

/// 树形结构循环引用演示
fn demonstrate_tree_circular_reference() {
    #[derive(Debug)]
    struct TreeNode {
        value: String,
        parent: RefCell<Option<Rc<TreeNode>>>,
        children: RefCell<Vec<Rc<TreeNode>>>,
    }

    impl Drop for TreeNode {
        fn drop(&mut self) {
            println!("  🗑️  TreeNode '{}' 被释放", self.value);
        }
    }

    let root = Rc::new(TreeNode {
        value: "root".to_string(),
        parent: RefCell::new(None),
        children: RefCell::new(Vec::new()),
    });

    let child = Rc::new(TreeNode {
        value: "child".to_string(),
        parent: RefCell::new(Some(Rc::clone(&root))), // 子节点引用父节点
        children: RefCell::new(Vec::new()),
    });

    // 父节点引用子节点，形成循环引用
    root.children.borrow_mut().push(Rc::clone(&child));

    println!("  Root 引用计数: {}", Rc::strong_count(&root));
    println!("  Child 引用计数: {}", Rc::strong_count(&child));
    println!("  ⚠️  Parent -> Child -> Parent 形成循环引用！");
}

/// 2. Weak 指针解决方案
///
/// 演示如何使用 Weak<T> 解决循环引用问题
fn demonstrate_weak_pointer_solution() {
    println!("\n2. === Weak 指针解决方案 ===");

    // 2.1 基本 Weak 指针使用
    println!("\n2.1 基本 Weak 指针使用：");
    demonstrate_basic_weak_usage();

    // 2.2 树形结构的 Weak 解决方案
    println!("\n2.2 树形结构的 Weak 解决方案：");
    demonstrate_tree_weak_solution();

    // 2.3 双向链表的 Weak 实现
    println!("\n2.3 双向链表的 Weak 实现：");
    demonstrate_doubly_linked_list();
}

/// 基本 Weak 指针使用演示
fn demonstrate_basic_weak_usage() {
    let strong_ref = Rc::new("Hello, Weak!".to_string());
    println!("  强引用计数: {}", Rc::strong_count(&strong_ref));
    println!("  弱引用计数: {}", Rc::weak_count(&strong_ref));

    // 创建弱引用
    let weak_ref = Rc::downgrade(&strong_ref);
    println!("  创建弱引用后：");
    println!("    强引用计数: {}", Rc::strong_count(&strong_ref));
    println!("    弱引用计数: {}", Rc::weak_count(&strong_ref));

    // 尝试升级弱引用
    match weak_ref.upgrade() {
        Some(upgraded) => println!("  ✅ 弱引用升级成功: {}", upgraded),
        None => println!("  ❌ 弱引用升级失败，对象已被释放"),
    }

    // 释放强引用
    drop(strong_ref);

    // 再次尝试升级弱引用
    match weak_ref.upgrade() {
        Some(upgraded) => println!("  ✅ 弱引用升级成功: {}", upgraded),
        None => println!("  ❌ 弱引用升级失败，对象已被释放"),
    }
}

/// 树形结构的 Weak 解决方案
fn demonstrate_tree_weak_solution() {
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>, // 使用 Weak 避免循环引用
        children: RefCell<Vec<Rc<Node>>>,
    }

    impl Drop for Node {
        fn drop(&mut self) {
            println!("  🗑️  Node {} 被正确释放", self.value);
        }
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "  叶子节点 - 强引用: {}, 弱引用: {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // 叶子节点的父节点使用弱引用指向分支节点
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("  创建分支后：");
        println!(
            "    分支节点 - 强引用: {}, 弱引用: {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        println!(
            "    叶子节点 - 强引用: {}, 弱引用: {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );

        // 验证父节点访问
        {
            let parent_weak = leaf.parent.borrow().clone();
            if let Some(parent) = parent_weak.upgrade() {
                println!("  ✅ 叶子节点可以访问父节点: {}", parent.value);
            }
        }
    }

    println!("  分支节点离开作用域后：");
    println!(
        "    叶子节点 - 强引用: {}, 弱引用: {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    // 尝试访问已释放的父节点
    {
        let parent_weak = leaf.parent.borrow().clone();
        match parent_weak.upgrade() {
            Some(parent) => println!("  父节点仍然存在: {}", parent.value),
            None => println!("  ✅ 父节点已被正确释放，无循环引用"),
        }
    }
}

/// 双向链表的 Weak 实现
fn demonstrate_doubly_linked_list() {
    #[derive(Debug)]
    struct ListNode {
        value: i32,
        next: RefCell<Option<Rc<ListNode>>>,
        prev: RefCell<Weak<ListNode>>, // 使用 Weak 避免循环引用
    }

    impl Drop for ListNode {
        fn drop(&mut self) {
            println!("  🗑️  ListNode {} 被正确释放", self.value);
        }
    }

    let node1 = Rc::new(ListNode {
        value: 1,
        next: RefCell::new(None),
        prev: RefCell::new(Weak::new()),
    });

    let node2 = Rc::new(ListNode {
        value: 2,
        next: RefCell::new(None),
        prev: RefCell::new(Weak::new()),
    });

    let node3 = Rc::new(ListNode {
        value: 3,
        next: RefCell::new(None),
        prev: RefCell::new(Weak::new()),
    });

    // 建立双向链接：node1 <-> node2 <-> node3
    *node1.next.borrow_mut() = Some(Rc::clone(&node2));
    *node2.prev.borrow_mut() = Rc::downgrade(&node1);

    *node2.next.borrow_mut() = Some(Rc::clone(&node3));
    *node3.prev.borrow_mut() = Rc::downgrade(&node2);

    println!("  双向链表构建完成：");
    println!("    Node1 强引用: {}", Rc::strong_count(&node1));
    println!("    Node2 强引用: {}", Rc::strong_count(&node2));
    println!("    Node3 强引用: {}", Rc::strong_count(&node3));

    // 验证双向访问
    if let Some(next) = node1.next.borrow().as_ref() {
        println!("  Node1 的下一个节点: {}", next.value);
    }

    if let Some(prev) = node2.prev.borrow().upgrade() {
        println!("  Node2 的上一个节点: {}", prev.value);
    }

    println!("  ✅ 双向链表使用 Weak 指针避免了循环引用");
}

/// 3. 引用计数机制演示
///
/// 详细演示强引用计数和弱引用计数的变化过程
fn demonstrate_reference_counting() {
    println!("\n3. === 引用计数机制演示 ===");

    #[derive(Debug)]
    struct Counter {
        id: u32,
    }

    impl Drop for Counter {
        fn drop(&mut self) {
            println!("  🗑️  Counter {} 被释放", self.id);
        }
    }

    let counter = Rc::new(Counter { id: 1 });
    println!("\n  初始状态：");
    println!("    强引用计数: {}", Rc::strong_count(&counter));
    println!("    弱引用计数: {}", Rc::weak_count(&counter));

    // 创建强引用
    let strong_clone1 = Rc::clone(&counter);
    let strong_clone2 = Rc::clone(&counter);
    println!("\n  创建两个强引用后：");
    println!("    强引用计数: {}", Rc::strong_count(&counter));
    println!("    弱引用计数: {}", Rc::weak_count(&counter));

    // 创建弱引用
    let weak1 = Rc::downgrade(&counter);
    let weak2 = Rc::downgrade(&counter);
    let weak3 = Rc::downgrade(&counter);
    println!("\n  创建三个弱引用后：");
    println!("    强引用计数: {}", Rc::strong_count(&counter));
    println!("    弱引用计数: {}", Rc::weak_count(&counter));

    // 释放强引用
    drop(strong_clone1);
    println!("\n  释放一个强引用后：");
    println!("    强引用计数: {}", Rc::strong_count(&counter));
    println!("    弱引用计数: {}", Rc::weak_count(&counter));

    drop(strong_clone2);
    println!("\n  释放第二个强引用后：");
    println!("    强引用计数: {}", Rc::strong_count(&counter));
    println!("    弱引用计数: {}", Rc::weak_count(&counter));

    // 测试弱引用升级
    println!("\n  测试弱引用升级：");
    for (i, weak_ref) in [&weak1, &weak2, &weak3].iter().enumerate() {
        match weak_ref.upgrade() {
            Some(upgraded) => println!("    弱引用 {} 升级成功: {:?}", i + 1, upgraded.id),
            None => println!("    弱引用 {} 升级失败，对象已释放", i + 1),
        }
    }

    // 释放最后一个强引用
    drop(counter);
    println!("\n  释放最后一个强引用后，测试弱引用升级：");
    for (i, weak_ref) in [&weak1, &weak2, &weak3].iter().enumerate() {
        match weak_ref.upgrade() {
            Some(upgraded) => println!("    弱引用 {} 升级成功: {:?}", i + 1, upgraded.id),
            None => println!("    弱引用 {} 升级失败，对象已释放", i + 1),
        }
    }
}

/// 4. 实际应用场景演示
///
/// 展示 Weak 指针在实际开发中的应用场景
fn demonstrate_practical_applications() {
    println!("\n4. === 实际应用场景演示 ===");

    // 4.1 观察者模式
    println!("\n4.1 观察者模式：");
    demonstrate_observer_pattern();

    // 4.2 缓存系统
    println!("\n4.3 缓存系统：");
    demonstrate_cache_system();

    // 4.4 DOM 树模拟
    println!("\n4.4 DOM 树模拟：");
    demonstrate_dom_tree();
}

/// 观察者模式相关定义
trait Observer {
    fn update(&self, message: &str);
}

struct Subject {
    observers: RefCell<Vec<Weak<dyn Observer>>>,
}

impl Subject {
    fn new() -> Self {
        Subject {
            observers: RefCell::new(Vec::new()),
        }
    }

    fn attach(&self, observer: Weak<dyn Observer>) {
        self.observers.borrow_mut().push(observer);
    }

    fn notify(&self, message: &str) {
        let mut observers = self.observers.borrow_mut();
        // 清理已失效的弱引用
        observers.retain(|weak_observer| {
            if let Some(observer) = weak_observer.upgrade() {
                observer.update(message);
                true
            } else {
                false // 移除已失效的观察者
            }
        });
    }
}

/// 观察者模式实现
fn demonstrate_observer_pattern() {
    struct ConcreteObserver {
        id: u32,
    }

    impl Observer for ConcreteObserver {
        fn update(&self, message: &str) {
            println!("    观察者 {} 收到消息: {}", self.id, message);
        }
    }

    impl Drop for ConcreteObserver {
        fn drop(&mut self) {
            println!("    🗑️  观察者 {} 被释放", self.id);
        }
    }

    let subject = Subject::new();

    // 创建观察者
    let observer1: Rc<dyn Observer> = Rc::new(ConcreteObserver { id: 1 });
    let observer2: Rc<dyn Observer> = Rc::new(ConcreteObserver { id: 2 });

    // 注册观察者（使用弱引用）
    subject.attach(Rc::downgrade(&observer1));
    subject.attach(Rc::downgrade(&observer2));

    // 通知所有观察者
    subject.notify("第一条消息");

    // 释放一个观察者
    drop(observer1);

    // 再次通知（已释放的观察者会被自动清理）
    subject.notify("第二条消息");

    println!("  ✅ 观察者模式使用 Weak 指针避免了循环引用");
}

/// 缓存系统实现
fn demonstrate_cache_system() {
    struct CacheEntry {
        key: String,
        value: String,
        parent_cache: Weak<RefCell<Cache>>,
    }

    impl Drop for CacheEntry {
        fn drop(&mut self) {
            println!("    🗑️  缓存条目 '{}' 被释放", self.key);
            // 从父缓存中移除自己
            if let Some(cache) = self.parent_cache.upgrade() {
                cache.borrow_mut().entries.remove(&self.key);
            }
        }
    }

    struct Cache {
        entries: HashMap<String, Rc<CacheEntry>>,
    }

    impl Cache {
        fn new() -> Rc<RefCell<Self>> {
            Rc::new(RefCell::new(Cache {
                entries: HashMap::new(),
            }))
        }

        fn insert(cache: &Rc<RefCell<Self>>, key: String, value: String) -> Rc<CacheEntry> {
            let entry = Rc::new(CacheEntry {
                key: key.clone(),
                value,
                parent_cache: Rc::downgrade(cache),
            });

            cache.borrow_mut().entries.insert(key, Rc::clone(&entry));
            entry
        }

        fn get(&self, key: &str) -> Option<Rc<CacheEntry>> {
            self.entries.get(key).cloned()
        }

        fn size(&self) -> usize {
            self.entries.len()
        }
    }

    let cache = Cache::new();

    // 插入缓存条目
    let entry1 = Cache::insert(&cache, "key1".to_string(), "value1".to_string());
    let entry2 = Cache::insert(&cache, "key2".to_string(), "value2".to_string());

    println!("    缓存大小: {}", cache.borrow().size());

    // 访问缓存
    if let Some(entry) = cache.borrow().get("key1") {
        println!("    获取缓存: {} = {}", entry.key, entry.value);
    }

    // 释放一个条目引用
    drop(entry1);
    println!("    释放 entry1 后缓存大小: {}", cache.borrow().size());

    println!("  ✅ 缓存系统使用 Weak 指针实现自动清理");
}

/// DOM 树模拟
fn demonstrate_dom_tree() {
    #[derive(Debug)]
    struct DOMNode {
        tag: String,
        parent: RefCell<Weak<DOMNode>>,
        children: RefCell<Vec<Rc<DOMNode>>>,
    }

    impl DOMNode {
        fn new(tag: &str) -> Rc<Self> {
            Rc::new(DOMNode {
                tag: tag.to_string(),
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(Vec::new()),
            })
        }

        fn append_child(parent: &Rc<DOMNode>, child: &Rc<DOMNode>) {
            parent.children.borrow_mut().push(Rc::clone(child));
            *child.parent.borrow_mut() = Rc::downgrade(parent);
        }

        fn get_path(&self) -> String {
            let mut path = vec![self.tag.clone()];
            let mut current_parent = self.parent.borrow().upgrade();

            while let Some(parent) = current_parent {
                path.push(parent.tag.clone());
                current_parent = parent.parent.borrow().upgrade();
            }

            path.reverse();
            path.join(" > ")
        }
    }

    impl Drop for DOMNode {
        fn drop(&mut self) {
            println!("    🗑️  DOM节点 '{}' 被释放", self.tag);
        }
    }

    // 构建 DOM 树
    let html = DOMNode::new("html");
    let body = DOMNode::new("body");
    let div = DOMNode::new("div");
    let p = DOMNode::new("p");

    DOMNode::append_child(&html, &body);
    DOMNode::append_child(&body, &div);
    DOMNode::append_child(&div, &p);

    println!("    DOM 树构建完成：");
    println!("    p 元素路径: {}", p.get_path());

    // 验证引用计数
    println!("    各节点强引用计数：");
    println!("      html: {}", Rc::strong_count(&html));
    println!("      body: {}", Rc::strong_count(&body));
    println!("      div: {}", Rc::strong_count(&div));
    println!("      p: {}", Rc::strong_count(&p));

    println!("  ✅ DOM 树使用 Weak 指针避免父子循环引用");
}

/// 5. Arc 与 Weak 多线程应用
///
/// 演示在多线程环境中使用 Arc 和 Weak
fn demonstrate_arc_weak_threading() {
    println!("\n5. === Arc 与 Weak 多线程应用 ===");

    use std::sync::{Arc, Weak as SyncWeak};

    #[derive(Debug)]
    struct SharedResource {
        id: u32,
        data: Mutex<String>,
    }

    impl Drop for SharedResource {
        fn drop(&mut self) {
            println!("  🗑️  SharedResource {} 被释放", self.id);
        }
    }

    let resource = Arc::new(SharedResource {
        id: 1,
        data: Mutex::new("初始数据".to_string()),
    });

    println!(
        "\n  创建共享资源，强引用计数: {}",
        Arc::strong_count(&resource)
    );

    // 创建弱引用用于线程间通信
    let weak_refs: Vec<SyncWeak<SharedResource>> =
        (0..3).map(|_| Arc::downgrade(&resource)).collect();

    println!(
        "  创建弱引用后，强引用计数: {}, 弱引用计数: {}",
        Arc::strong_count(&resource),
        Arc::weak_count(&resource)
    );

    // 启动工作线程
    let handles: Vec<_> = weak_refs
        .into_iter()
        .enumerate()
        .map(|(i, weak_ref)| {
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(100 * (i as u64 + 1)));

                match weak_ref.upgrade() {
                    Some(resource) => {
                        let mut data = resource.data.lock().unwrap();
                        *data = format!("{} + 线程{}", *data, i + 1);
                        println!("    线程 {} 更新数据: {}", i + 1, *data);
                    }
                    None => {
                        println!("    线程 {} 发现资源已被释放", i + 1);
                    }
                }
            })
        })
        .collect();

    // 主线程等待一段时间后释放强引用
    thread::sleep(Duration::from_millis(150));
    println!("\n  主线程释放强引用...");
    drop(resource);

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("  ✅ Arc 与 Weak 在多线程环境中安全使用");
}

/// 6. 性能分析与最佳实践
///
/// 分析内存开销和性能特征
fn demonstrate_performance_analysis() {
    println!("\n6. === 性能分析与最佳实践 ===");

    // 6.1 内存开销分析
    println!("\n6.1 内存开销分析：");
    analyze_memory_overhead();

    // 6.2 性能对比测试
    println!("\n6.2 性能对比测试：");
    performance_comparison();

    // 6.3 最佳实践建议
    println!("\n6.3 最佳实践建议：");
    best_practices_guide();
}

/// 内存开销分析
fn analyze_memory_overhead() {
    use std::mem;

    #[derive(Debug)]
    struct TestData {
        value: u64,
    }

    let data = TestData { value: 42 };

    println!("    原始数据大小: {} 字节", mem::size_of::<TestData>());
    println!(
        "    Rc<TestData> 大小: {} 字节",
        mem::size_of::<Rc<TestData>>()
    );
    println!(
        "    Weak<TestData> 大小: {} 字节",
        mem::size_of::<Weak<TestData>>()
    );
    println!(
        "    Arc<TestData> 大小: {} 字节",
        mem::size_of::<Arc<TestData>>()
    );

    let rc_data = Rc::new(data);
    let weak_data = Rc::downgrade(&rc_data);

    println!("\n    引用计数开销：");
    println!("      强引用计数: {}", Rc::strong_count(&rc_data));
    println!("      弱引用计数: {}", Rc::weak_count(&rc_data));

    println!("\n  📊 内存开销总结：");
    println!("    - Rc/Arc 本身只是一个指针，开销很小");
    println!("    - 实际开销在于引用计数的存储和原子操作");
    println!("    - Weak 指针不增加强引用计数，但会增加弱引用计数");
}

/// 性能对比测试
fn performance_comparison() {
    use std::time::Instant;

    const ITERATIONS: usize = 1_000_000;

    // 测试 Rc clone 性能
    let rc_data = Rc::new(42u64);
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _clone = Rc::clone(&rc_data);
    }
    let rc_duration = start.elapsed();

    // 测试 Weak upgrade 性能
    let weak_data = Rc::downgrade(&rc_data);
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _upgraded = weak_data.upgrade();
    }
    let weak_duration = start.elapsed();

    println!("    {} 次操作性能对比：", ITERATIONS);
    println!("      Rc::clone: {:?}", rc_duration);
    println!("      Weak::upgrade: {:?}", weak_duration);
    println!(
        "      性能比率: {:.2}x",
        weak_duration.as_nanos() as f64 / rc_duration.as_nanos() as f64
    );

    println!("\n  📊 性能特征：");
    println!("    - Rc::clone 只是增加引用计数，非常快");
    println!("    - Weak::upgrade 需要检查对象是否存在，稍慢");
    println!("    - 在循环引用场景中，Weak 的额外开销是值得的");
}

/// 最佳实践建议
fn best_practices_guide() {
    println!("  🎯 使用场景选择：");
    println!("    ✅ 使用 Rc<T> 的场景：");
    println!("      - 需要多个所有者共享数据");
    println!("      - 数据结构是 DAG（有向无环图）");
    println!("      - 单线程环境");

    println!("\n    ✅ 使用 Weak<T> 的场景：");
    println!("      - 父子关系（子节点引用父节点）");
    println!("      - 观察者模式");
    println!("      - 缓存系统");
    println!("      - 任何可能形成循环的场景");

    println!("\n    ✅ 使用 Arc<T> 的场景：");
    println!("      - 多线程环境下的共享数据");
    println!("      - 跨线程传递数据");

    println!("\n  ⚠️  注意事项：");
    println!("    - 避免 Rc<RefCell<T>> 的循环引用");
    println!("    - 及时清理失效的 Weak 引用");
    println!("    - 在性能敏感场景中谨慎使用 Weak::upgrade");
    println!("    - 优先考虑生命周期和借用检查器的解决方案");

    println!("\n  🔧 调试技巧：");
    println!("    - 使用 strong_count() 和 weak_count() 监控引用");
    println!("    - 实现 Drop trait 来跟踪对象释放");
    println!("    - 使用工具如 Valgrind 检测内存泄漏");
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weak_reference_upgrade() {
        let strong = Rc::new("test".to_string());
        let weak = Rc::downgrade(&strong);

        assert!(weak.upgrade().is_some());
        drop(strong);
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn test_reference_counting() {
        let data = Rc::new(42);
        assert_eq!(Rc::strong_count(&data), 1);
        assert_eq!(Rc::weak_count(&data), 0);

        let weak = Rc::downgrade(&data);
        assert_eq!(Rc::strong_count(&data), 1);
        assert_eq!(Rc::weak_count(&data), 1);

        let clone = Rc::clone(&data);
        assert_eq!(Rc::strong_count(&data), 2);
        assert_eq!(Rc::weak_count(&data), 1);
    }

    #[test]
    fn test_tree_structure_no_leak() {
        #[derive(Debug)]
        struct Node {
            value: i32,
            parent: RefCell<Weak<Node>>,
            children: RefCell<Vec<Rc<Node>>>,
        }

        let parent = Rc::new(Node {
            value: 1,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        });

        let child = Rc::new(Node {
            value: 2,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        });

        // 建立父子关系
        parent.children.borrow_mut().push(Rc::clone(&child));
        *child.parent.borrow_mut() = Rc::downgrade(&parent);

        // 验证引用计数
        assert_eq!(Rc::strong_count(&parent), 1);
        assert_eq!(Rc::strong_count(&child), 2); // parent.children + child
        assert_eq!(Rc::weak_count(&parent), 1); // child.parent

        // 验证可以访问父节点
        assert!(child.parent.borrow().upgrade().is_some());
    }

    #[test]
    fn test_observer_pattern() {
        use std::sync::{Arc, Mutex};

        struct TestObserver {
            messages: Arc<Mutex<Vec<String>>>,
        }

        impl Observer for TestObserver {
            fn update(&self, message: &str) {
                self.messages.lock().unwrap().push(message.to_string());
            }
        }

        let messages = Arc::new(Mutex::new(Vec::new()));
        let observer: Rc<dyn Observer> = Rc::new(TestObserver {
            messages: Arc::clone(&messages),
        });

        let subject = Subject::new();
        subject.attach(Rc::downgrade(&observer));

        subject.notify("test message");

        let msgs = messages.lock().unwrap();
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0], "test message");
    }

    #[test]
    fn test_circular_reference_prevention() {
        #[derive(Debug)]
        struct Node {
            value: i32,
            next: RefCell<Option<Rc<Node>>>,
            prev: RefCell<Weak<Node>>,
        }

        let node1 = Rc::new(Node {
            value: 1,
            next: RefCell::new(None),
            prev: RefCell::new(Weak::new()),
        });

        let node2 = Rc::new(Node {
            value: 2,
            next: RefCell::new(None),
            prev: RefCell::new(Weak::new()),
        });

        // 建立双向链接
        *node1.next.borrow_mut() = Some(Rc::clone(&node2));
        *node2.prev.borrow_mut() = Rc::downgrade(&node1);

        // 验证没有循环引用
        assert_eq!(Rc::strong_count(&node1), 1);
        assert_eq!(Rc::strong_count(&node2), 2);
        assert_eq!(Rc::weak_count(&node1), 1);

        // 验证可以正常访问
        assert!(node1.next.borrow().is_some());
        assert!(node2.prev.borrow().upgrade().is_some());
    }
}
