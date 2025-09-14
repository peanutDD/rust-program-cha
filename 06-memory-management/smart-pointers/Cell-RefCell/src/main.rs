//! # Rust Cell 与 RefCell 内部可变性深度教程
//!
//! 本教程基于 https://course.rs/advance/smart-pointer/cell-refcell.html 进行全面深入的分析
//! 涵盖内部可变性的所有相关知识点，包括原理、实现、应用场景和性能分析

use std::cell::{Cell, Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    println!("=== Rust Cell 与 RefCell 内部可变性深度教程 ===");
    println!();

    // 1. 内部可变性基础概念
    demonstrate_interior_mutability_concept();

    // 2. Cell<T> 详细分析
    demonstrate_cell_patterns();

    // 3. RefCell<T> 详细分析
    demonstrate_refcell_patterns();

    // 4. 运行时借用检查机制
    demonstrate_runtime_borrow_checking();

    // 5. Rc + RefCell 组合模式
    demonstrate_rc_refcell_combination();

    // 6. 实际应用场景
    demonstrate_practical_scenarios();

    // 7. 高级模式与技巧
    demonstrate_advanced_patterns();

    // 8. 性能分析与优化
    demonstrate_performance_analysis();

    println!("\n=== Cell 与 RefCell 教程总结 ===");
    println!("✅ 内部可变性：在不可变引用存在时修改数据");
    println!("✅ Cell<T>：Copy 类型的零成本内部可变性");
    println!("✅ RefCell<T>：引用类型的运行时借用检查");
    println!("✅ 运行时检查：将编译时错误推迟到运行时");
    println!("✅ Rc + RefCell：多所有者 + 内部可变性");
    println!("✅ 实际应用：观察者模式、缓存、配置管理");
    println!("✅ 性能权衡：灵活性 vs 运行时开销");
}

/// 1. 内部可变性基础概念演示
fn demonstrate_interior_mutability_concept() {
    println!("=== 1. 内部可变性基础概念 ===");

    // 1.1 传统借用规则的限制
    println!("\n1.1 传统借用规则的限制：");
    let x = 5;
    let r1 = &x;
    let r2 = &x;
    // let r3 = &mut x; // 编译错误：不能在不可变引用存在时创建可变引用
    println!("不可变值 x = {}, 引用 r1 = {}, r2 = {}", x, r1, r2);
    println!("❌ 无法在不可变引用存在时创建可变引用");

    // 1.2 内部可变性的定义和原理
    println!("\n1.2 内部可变性的定义：");
    println!("内部可变性是 Rust 中的一个设计模式，它允许你即使在有不可变引用时也可以改变数据");
    println!("这通常是借用规则所不允许的，但通过 unsafe 代码封装在安全的 API 中实现");

    // 1.3 UnsafeCell 的作用
    println!("\n1.3 UnsafeCell 的核心作用：");
    println!("• UnsafeCell<T> 是所有内部可变性的基础");
    println!("• 它告诉编译器：即使外部是不可变的，内部数据可能会改变");
    println!("• 编译器不会对 UnsafeCell 包装的数据进行不可变性优化");

    // 1.4 编译时 vs 运行时检查
    println!("\n1.4 编译时 vs 运行时检查：");
    println!("• 编译时检查：零运行时成本，但限制灵活性");
    println!("• 运行时检查：增加灵活性，但有性能开销");
    println!("• 内部可变性：将部分检查从编译时推迟到运行时");
}

/// 2. Cell<T> 详细分析
fn demonstrate_cell_patterns() {
    println!("\n=== 2. Cell<T> 详细分析 ===");

    // 2.1 Cell 基本操作
    println!("\n2.1 Cell 基本操作：");
    let cell = Cell::new(42);
    println!("创建 Cell: {:?}", cell);
    println!("获取值: {}", cell.get());

    cell.set(100);
    println!("设置新值后: {}", cell.get());

    let old_value = cell.replace(200);
    println!("替换值，旧值: {}, 新值: {}", old_value, cell.get());

    // 2.2 Cell 的 Copy 限制
    println!("\n2.2 Cell 的 Copy 限制：");
    let int_cell = Cell::new(10);
    let bool_cell = Cell::new(true);
    let char_cell = Cell::new('A');

    println!("整数 Cell: {}", int_cell.get());
    println!("布尔 Cell: {}", bool_cell.get());
    println!("字符 Cell: {}", char_cell.get());

    // String 不实现 Copy，所以不能直接用于 Cell
    // let string_cell = Cell::new(String::from("hello")); // 编译错误
    println!("❌ String 等非 Copy 类型不能直接用于 Cell");

    // 2.3 Cell 的零成本抽象
    println!("\n2.3 Cell 的零成本抽象：");
    demonstrate_cell_zero_cost();

    // 2.4 Cell 的线程安全性
    println!("\n2.4 Cell 的线程安全性：");
    println!("❌ Cell 不是线程安全的，不能在多线程间共享");
    println!("✅ 如需线程安全，应使用 Mutex 或原子类型");
}

/// Cell 零成本抽象演示
fn demonstrate_cell_zero_cost() {
    let cell = Cell::new(0);
    let start = Instant::now();

    for i in 0..1_000_000 {
        cell.set(i);
        let _ = cell.get();
    }

    let duration = start.elapsed();
    println!("Cell 操作 100万次耗时: {:?}", duration);
    println!("✅ Cell 操作几乎零成本，直接内存读写");
}

/// 3. RefCell<T> 详细分析
fn demonstrate_refcell_patterns() {
    println!("\n=== 3. RefCell<T> 详细分析 ===");

    // 3.1 RefCell 基本操作
    println!("\n3.1 RefCell 基本操作：");
    let refcell = RefCell::new(vec![1, 2, 3]);

    // 不可变借用
    {
        let borrowed = refcell.borrow();
        println!("不可变借用: {:?}", *borrowed);
        println!("借用计数: 可以有多个不可变借用");
    }

    // 可变借用
    {
        let mut borrowed = refcell.borrow_mut();
        borrowed.push(4);
        println!("可变借用后: {:?}", *borrowed);
        println!("借用计数: 只能有一个可变借用");
    }

    // 3.2 RefCell 的运行时检查
    println!("\n3.2 RefCell 的运行时检查：");
    demonstrate_refcell_runtime_checks();

    // 3.3 RefCell 的 try_borrow 系列方法
    println!("\n3.3 RefCell 的 try_borrow 系列方法：");
    demonstrate_refcell_try_methods();

    // 3.4 RefCell 与生命周期
    println!("\n3.4 RefCell 与生命周期：");
    demonstrate_refcell_lifetimes();
}

/// RefCell 运行时检查演示
fn demonstrate_refcell_runtime_checks() {
    let refcell = RefCell::new(String::from("Hello"));

    // 正常情况：多个不可变借用
    {
        let borrow1 = refcell.borrow();
        let borrow2 = refcell.borrow();
        println!("✅ 多个不可变借用: '{}', '{}'", *borrow1, *borrow2);
    }

    // 正常情况：单个可变借用
    {
        let mut borrow = refcell.borrow_mut();
        borrow.push_str(", World!");
        println!("✅ 单个可变借用: '{}'", *borrow);
    }

    println!("❌ 如果同时存在可变和不可变借用，程序会 panic");
    println!("这将编译时错误推迟到了运行时");
}

/// RefCell try_borrow 方法演示
fn demonstrate_refcell_try_methods() {
    let refcell = RefCell::new(42);

    // 使用 try_borrow_mut 避免 panic
    let _borrow = refcell.borrow_mut();

    match refcell.try_borrow() {
        Ok(_) => println!("✅ 成功获取不可变借用"),
        Err(_) => println!("❌ 无法获取不可变借用：已存在可变借用"),
    }

    match refcell.try_borrow_mut() {
        Ok(_) => println!("✅ 成功获取可变借用"),
        Err(_) => println!("❌ 无法获取可变借用：已存在其他借用"),
    }

    drop(_borrow); // 显式释放借用

    match refcell.try_borrow() {
        Ok(borrow) => println!("✅ 释放后成功获取不可变借用: {}", *borrow),
        Err(_) => println!("❌ 仍无法获取借用"),
    }
}

/// RefCell 生命周期演示
fn demonstrate_refcell_lifetimes() {
    let refcell = RefCell::new(vec![1, 2, 3]);

    // Ref 和 RefMut 的生命周期
    let len = {
        let borrow = refcell.borrow();
        borrow.len() // 可以返回从借用中计算的值
    }; // borrow 在这里被释放

    println!("向量长度: {}", len);

    // 可以在 borrow 释放后进行新的借用
    refcell.borrow_mut().push(4);
    println!("添加元素后: {:?}", refcell.borrow());
}

/// 4. 运行时借用检查机制
fn demonstrate_runtime_borrow_checking() {
    println!("\n=== 4. 运行时借用检查机制 ===");

    // 4.1 BorrowFlag 机制原理
    println!("\n4.1 BorrowFlag 机制原理：");
    println!("• RefCell 内部使用 isize 类型的 BorrowFlag");
    println!("• 正值：表示活跃的不可变借用数量");
    println!("• 负值：表示活跃的可变借用数量");
    println!("• 0：表示未被借用状态");

    // 4.2 借用状态追踪
    println!("\n4.2 借用状态追踪演示：");
    demonstrate_borrow_flag_tracking();

    // 4.3 Ref 和 RefMut 智能指针
    println!("\n4.3 Ref 和 RefMut 智能指针：");
    demonstrate_ref_refmut_pointers();

    // 4.4 借用冲突处理
    println!("\n4.4 借用冲突处理：");
    demonstrate_borrow_conflict_handling();
}

/// BorrowFlag 追踪演示
fn demonstrate_borrow_flag_tracking() {
    let refcell = RefCell::new(String::from("tracking"));

    println!("初始状态：BorrowFlag = 0 (未借用)");

    {
        let _borrow1 = refcell.borrow();
        println!("第一个不可变借用：BorrowFlag = 1");

        {
            let _borrow2 = refcell.borrow();
            println!("第二个不可变借用：BorrowFlag = 2");
        }
        println!("第二个借用释放：BorrowFlag = 1");
    }
    println!("所有不可变借用释放：BorrowFlag = 0");

    {
        let _mut_borrow = refcell.borrow_mut();
        println!("可变借用：BorrowFlag = -1");
    }
    println!("可变借用释放：BorrowFlag = 0");
}

/// Ref 和 RefMut 智能指针演示
fn demonstrate_ref_refmut_pointers() {
    let refcell = RefCell::new(vec![1, 2, 3]);

    // Ref<T> 智能指针
    let borrow: Ref<Vec<i32>> = refcell.borrow();
    println!("Ref<T> 解引用: {:?}", *borrow);
    println!("Ref<T> 可以像普通引用一样使用");
    drop(borrow);

    // RefMut<T> 智能指针
    let mut mut_borrow: RefMut<Vec<i32>> = refcell.borrow_mut();
    mut_borrow.push(4);
    println!("RefMut<T> 修改后: {:?}", *mut_borrow);
    println!("RefMut<T> 可以像可变引用一样使用");
    drop(mut_borrow);

    // 智能指针的自动释放
    println!("✅ Ref 和 RefMut 实现了 Drop trait，自动管理借用状态");
}

/// 借用冲突处理演示
fn demonstrate_borrow_conflict_handling() {
    let refcell = RefCell::new(42);

    // 使用 try_borrow 系列方法处理冲突
    let _mut_borrow = refcell.borrow_mut();

    // 尝试获取不可变借用
    match refcell.try_borrow() {
        Ok(_) => println!("✅ 成功获取不可变借用"),
        Err(e) => println!("❌ 借用冲突: {:?}", e),
    }

    // 尝试获取另一个可变借用
    match refcell.try_borrow_mut() {
        Ok(_) => println!("✅ 成功获取可变借用"),
        Err(e) => println!("❌ 借用冲突: {:?}", e),
    }

    drop(_mut_borrow);
    println!("✅ 释放借用后可以正常获取新借用");
}

/// 5. Rc + RefCell 组合模式
fn demonstrate_rc_refcell_combination() {
    println!("\n=== 5. Rc + RefCell 组合模式 ===");

    // 5.1 多所有者 + 内部可变性
    println!("\n5.1 多所有者 + 内部可变性：");
    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));
    let clone1 = Rc::clone(&shared_data);
    let clone2 = Rc::clone(&shared_data);

    // 通过不同的 Rc 修改共享数据
    clone1.borrow_mut().push(4);
    clone2.borrow_mut().push(5);

    println!("共享数据: {:?}", shared_data.borrow());
    println!("引用计数: {}", Rc::strong_count(&shared_data));

    // 5.2 树形数据结构
    println!("\n5.2 树形数据结构：");
    demonstrate_tree_structure();

    // 5.3 图形数据结构
    println!("\n5.3 图形数据结构：");
    demonstrate_graph_structure();
}

/// 树形数据结构演示
#[derive(Debug)]
struct TreeNode {
    value: i32,
    children: RefCell<Vec<Rc<TreeNode>>>,
}

impl TreeNode {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(TreeNode {
            value,
            children: RefCell::new(Vec::new()),
        })
    }

    fn add_child(&self, child: Rc<TreeNode>) {
        self.children.borrow_mut().push(child);
    }

    fn print_tree(&self, depth: usize) {
        println!("{}{}", "  ".repeat(depth), self.value);
        for child in self.children.borrow().iter() {
            child.print_tree(depth + 1);
        }
    }
}

fn demonstrate_tree_structure() {
    let root = TreeNode::new(1);
    let child1 = TreeNode::new(2);
    let child2 = TreeNode::new(3);
    let grandchild = TreeNode::new(4);

    root.add_child(child1.clone());
    root.add_child(child2.clone());
    child1.add_child(grandchild);

    println!("树形结构：");
    root.print_tree(0);
}

/// 图形数据结构演示
#[derive(Debug)]
struct GraphNode {
    id: usize,
    neighbors: RefCell<Vec<Rc<GraphNode>>>,
}

impl GraphNode {
    fn new(id: usize) -> Rc<Self> {
        Rc::new(GraphNode {
            id,
            neighbors: RefCell::new(Vec::new()),
        })
    }

    fn add_neighbor(&self, neighbor: Rc<GraphNode>) {
        self.neighbors.borrow_mut().push(neighbor);
    }

    fn print_neighbors(&self) {
        print!("节点 {} 的邻居: ", self.id);
        for neighbor in self.neighbors.borrow().iter() {
            print!("{} ", neighbor.id);
        }
        println!();
    }
}

fn demonstrate_graph_structure() {
    let node1 = GraphNode::new(1);
    let node2 = GraphNode::new(2);
    let node3 = GraphNode::new(3);

    // 创建双向连接
    node1.add_neighbor(node2.clone());
    node1.add_neighbor(node3.clone());
    node2.add_neighbor(node1.clone());
    node2.add_neighbor(node3.clone());
    node3.add_neighbor(node1.clone());
    node3.add_neighbor(node2.clone());

    println!("图形结构：");
    node1.print_neighbors();
    node2.print_neighbors();
    node3.print_neighbors();
}

/// 6. 实际应用场景
fn demonstrate_practical_scenarios() {
    println!("\n=== 6. 实际应用场景 ===");

    // 6.1 观察者模式
    println!("\n6.1 观察者模式：");
    demonstrate_observer_pattern();

    // 6.2 缓存系统
    println!("\n6.2 缓存系统：");
    demonstrate_cache_system();

    // 6.3 配置管理
    println!("\n6.3 配置管理：");
    demonstrate_config_management();

    // 6.4 状态机
    println!("\n6.4 状态机：");
    demonstrate_state_machine();
}

/// 观察者模式演示
trait Observer {
    fn update(&self, message: &str);
}

#[derive(Debug)]
struct ConcreteObserver {
    name: String,
}

impl ConcreteObserver {
    fn new(name: &str) -> Self {
        ConcreteObserver {
            name: name.to_string(),
        }
    }
}

impl Observer for ConcreteObserver {
    fn update(&self, message: &str) {
        println!("观察者 {} 收到消息: {}", self.name, message);
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
        self.observers.borrow_mut().push(observer);
    }

    fn notify(&self, message: &str) {
        for observer in self.observers.borrow().iter() {
            observer.update(message);
        }
    }
}

fn demonstrate_observer_pattern() {
    let subject = Subject::new();
    let observer1 = Rc::new(ConcreteObserver::new("Observer1"));
    let observer2 = Rc::new(ConcreteObserver::new("Observer2"));

    subject.add_observer(observer1);
    subject.add_observer(observer2);

    subject.notify("Hello from Subject!");
    subject.notify("Another message!");
}

/// 缓存系统演示
struct Cache {
    data: RefCell<HashMap<String, String>>,
    hit_count: Cell<usize>,
    miss_count: Cell<usize>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            data: RefCell::new(HashMap::new()),
            hit_count: Cell::new(0),
            miss_count: Cell::new(0),
        }
    }

    fn get(&self, key: &str) -> Option<String> {
        if let Some(value) = self.data.borrow().get(key) {
            self.hit_count.set(self.hit_count.get() + 1);
            Some(value.clone())
        } else {
            self.miss_count.set(self.miss_count.get() + 1);
            None
        }
    }

    fn set(&self, key: String, value: String) {
        self.data.borrow_mut().insert(key, value);
    }

    fn stats(&self) -> (usize, usize) {
        (self.hit_count.get(), self.miss_count.get())
    }
}

fn demonstrate_cache_system() {
    let cache = Cache::new();

    // 设置缓存
    cache.set("key1".to_string(), "value1".to_string());
    cache.set("key2".to_string(), "value2".to_string());

    // 访问缓存
    println!("获取 key1: {:?}", cache.get("key1"));
    println!("获取 key2: {:?}", cache.get("key2"));
    println!("获取 key3: {:?}", cache.get("key3"));

    let (hits, misses) = cache.stats();
    println!("缓存统计 - 命中: {}, 未命中: {}", hits, misses);
}

/// 配置管理演示
struct Config {
    settings: RefCell<HashMap<String, String>>,
    version: Cell<u32>,
}

impl Config {
    fn new() -> Self {
        Config {
            settings: RefCell::new(HashMap::new()),
            version: Cell::new(1),
        }
    }

    fn set(&self, key: &str, value: &str) {
        self.settings
            .borrow_mut()
            .insert(key.to_string(), value.to_string());
        self.version.set(self.version.get() + 1);
    }

    fn get(&self, key: &str) -> Option<String> {
        self.settings.borrow().get(key).cloned()
    }

    fn version(&self) -> u32 {
        self.version.get()
    }

    fn reload(&self, new_settings: HashMap<String, String>) {
        *self.settings.borrow_mut() = new_settings;
        self.version.set(self.version.get() + 1);
    }
}

fn demonstrate_config_management() {
    let config = Config::new();

    println!("初始版本: {}", config.version());

    config.set("database_url", "localhost:5432");
    config.set("max_connections", "100");

    println!("设置后版本: {}", config.version());
    println!("数据库URL: {:?}", config.get("database_url"));
    println!("最大连接数: {:?}", config.get("max_connections"));

    // 重新加载配置
    let mut new_settings = HashMap::new();
    new_settings.insert("database_url".to_string(), "remote:5432".to_string());
    new_settings.insert("max_connections".to_string(), "200".to_string());

    config.reload(new_settings);
    println!("重载后版本: {}", config.version());
    println!("新数据库URL: {:?}", config.get("database_url"));
}

/// 状态机演示
#[derive(Debug, Clone, PartialEq)]
enum State {
    Idle,
    Running,
    Paused,
    Stopped,
}

struct StateMachine {
    current_state: RefCell<State>,
    state_history: RefCell<Vec<State>>,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            current_state: RefCell::new(State::Idle),
            state_history: RefCell::new(vec![State::Idle]),
        }
    }

    fn transition_to(&self, new_state: State) -> Result<(), String> {
        let current = self.current_state.borrow().clone();

        let valid_transition = match (&current, &new_state) {
            (State::Idle, State::Running) => true,
            (State::Running, State::Paused) => true,
            (State::Running, State::Stopped) => true,
            (State::Paused, State::Running) => true,
            (State::Paused, State::Stopped) => true,
            (State::Stopped, State::Idle) => true,
            _ => false,
        };

        if valid_transition {
            *self.current_state.borrow_mut() = new_state.clone();
            self.state_history.borrow_mut().push(new_state);
            Ok(())
        } else {
            Err(format!("无效的状态转换: {:?} -> {:?}", current, new_state))
        }
    }

    fn current_state(&self) -> State {
        self.current_state.borrow().clone()
    }

    fn history(&self) -> Vec<State> {
        self.state_history.borrow().clone()
    }
}

fn demonstrate_state_machine() {
    let sm = StateMachine::new();

    println!("初始状态: {:?}", sm.current_state());

    // 有效的状态转换
    sm.transition_to(State::Running).unwrap();
    println!("转换到运行状态: {:?}", sm.current_state());

    sm.transition_to(State::Paused).unwrap();
    println!("转换到暂停状态: {:?}", sm.current_state());

    sm.transition_to(State::Stopped).unwrap();
    println!("转换到停止状态: {:?}", sm.current_state());

    // 无效的状态转换
    if let Err(e) = sm.transition_to(State::Running) {
        println!("❌ {}", e);
    }

    println!("状态历史: {:?}", sm.history());
}

/// 7. 高级模式与技巧
fn demonstrate_advanced_patterns() {
    println!("\n=== 7. 高级模式与技巧 ===");

    // 7.1 条件借用
    println!("\n7.1 条件借用：");
    demonstrate_conditional_borrowing();

    // 7.2 借用分割
    println!("\n7.2 借用分割：");
    demonstrate_borrow_splitting();

    // 7.3 懒加载模式
    println!("\n7.3 懒加载模式：");
    demonstrate_lazy_loading();

    // 7.4 写时复制 (Copy-on-Write)
    println!("\n7.4 写时复制模式：");
    demonstrate_copy_on_write();
}

/// 条件借用演示
fn demonstrate_conditional_borrowing() {
    let data = RefCell::new(vec![1, 2, 3, 4, 5]);

    // 根据条件决定是否进行可变借用
    let should_modify = true;

    if should_modify {
        if let Ok(mut borrow) = data.try_borrow_mut() {
            borrow.push(6);
            println!("✅ 条件满足，成功修改: {:?}", *borrow);
        } else {
            println!("❌ 无法获取可变借用");
        }
    } else {
        let borrow = data.borrow();
        println!("只读访问: {:?}", *borrow);
    }
}

/// 借用分割演示
struct Container {
    data: RefCell<Vec<i32>>,
    metadata: RefCell<String>,
}

impl Container {
    fn new() -> Self {
        Container {
            data: RefCell::new(vec![1, 2, 3]),
            metadata: RefCell::new("initial".to_string()),
        }
    }

    fn update_both(&self) {
        // 可以同时借用不同的 RefCell
        let mut data = self.data.borrow_mut();
        let mut metadata = self.metadata.borrow_mut();

        data.push(4);
        *metadata = "updated".to_string();

        println!("同时更新数据和元数据: {:?}, {}", *data, *metadata);
    }
}

fn demonstrate_borrow_splitting() {
    let container = Container::new();
    container.update_both();
    println!("✅ 不同 RefCell 可以同时进行可变借用");
}

/// 懒加载模式演示
struct LazyData {
    expensive_computation: RefCell<Option<Vec<i32>>>,
}

impl LazyData {
    fn new() -> Self {
        LazyData {
            expensive_computation: RefCell::new(None),
        }
    }

    fn get_data(&self) -> Vec<i32> {
        // 检查是否已经计算过
        if self.expensive_computation.borrow().is_none() {
            println!("执行昂贵的计算...");
            let result = (1..=1000).collect::<Vec<i32>>();
            *self.expensive_computation.borrow_mut() = Some(result);
        }

        self.expensive_computation
            .borrow()
            .as_ref()
            .unwrap()
            .clone()
    }
}

fn demonstrate_lazy_loading() {
    let lazy = LazyData::new();

    println!("第一次访问：");
    let data1 = lazy.get_data();
    println!("数据长度: {}", data1.len());

    println!("第二次访问：");
    let data2 = lazy.get_data();
    println!("数据长度: {} (无需重新计算)", data2.len());
}

/// 写时复制模式演示
struct CowData {
    data: RefCell<Vec<i32>>,
    is_shared: Cell<bool>,
}

impl CowData {
    fn new(data: Vec<i32>) -> Self {
        CowData {
            data: RefCell::new(data),
            is_shared: Cell::new(false),
        }
    }

    fn mark_shared(&self) {
        self.is_shared.set(true);
    }

    fn get_data(&self) -> Vec<i32> {
        self.data.borrow().clone()
    }

    fn modify_data(&self, new_value: i32) {
        if self.is_shared.get() {
            println!("数据被共享，执行写时复制");
            let mut current = self.data.borrow().clone();
            current.push(new_value);
            *self.data.borrow_mut() = current;
            self.is_shared.set(false);
        } else {
            println!("数据未共享，直接修改");
            self.data.borrow_mut().push(new_value);
        }
    }
}

fn demonstrate_copy_on_write() {
    let cow = CowData::new(vec![1, 2, 3]);

    println!("初始数据: {:?}", cow.get_data());

    cow.modify_data(4);
    println!("修改后: {:?}", cow.get_data());

    cow.mark_shared();
    cow.modify_data(5);
    println!("写时复制后: {:?}", cow.get_data());
}

/// 8. 性能分析与优化
fn demonstrate_performance_analysis() {
    println!("\n=== 8. 性能分析与优化 ===");

    // 8.1 Cell vs RefCell 性能对比
    println!("\n8.1 Cell vs RefCell 性能对比：");
    compare_cell_refcell_performance();

    // 8.2 运行时检查开销
    println!("\n8.2 运行时检查开销：");
    analyze_runtime_overhead();

    // 8.3 内存使用分析
    println!("\n8.3 内存使用分析：");
    analyze_memory_usage();

    // 8.4 最佳实践建议
    println!("\n8.4 最佳实践建议：");
    provide_best_practices();
}

/// Cell vs RefCell 性能对比
fn compare_cell_refcell_performance() {
    const ITERATIONS: usize = 1_000_000;

    // Cell 性能测试
    let cell = Cell::new(0);
    let start = Instant::now();
    for i in 0..ITERATIONS {
        cell.set(i);
        let _ = cell.get();
    }
    let cell_duration = start.elapsed();

    // RefCell 性能测试
    let refcell = RefCell::new(0);
    let start = Instant::now();
    for i in 0..ITERATIONS {
        *refcell.borrow_mut() = i;
        let _ = *refcell.borrow();
    }
    let refcell_duration = start.elapsed();

    println!("Cell {} 次操作耗时: {:?}", ITERATIONS, cell_duration);
    println!("RefCell {} 次操作耗时: {:?}", ITERATIONS, refcell_duration);
    println!(
        "RefCell 比 Cell 慢 {:.2}x",
        refcell_duration.as_nanos() as f64 / cell_duration.as_nanos() as f64
    );
}

/// 运行时检查开销分析
fn analyze_runtime_overhead() {
    let refcell = RefCell::new(vec![1, 2, 3]);
    const ITERATIONS: usize = 100_000;

    // 测试 borrow 开销
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _borrow = refcell.borrow();
        // 借用会自动释放
    }
    let borrow_duration = start.elapsed();

    // 测试 try_borrow 开销
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _borrow = refcell.try_borrow().unwrap();
    }
    let try_borrow_duration = start.elapsed();

    println!("borrow {} 次耗时: {:?}", ITERATIONS, borrow_duration);
    println!(
        "try_borrow {} 次耗时: {:?}",
        ITERATIONS, try_borrow_duration
    );
    println!(
        "平均每次 borrow 耗时: {:?}",
        borrow_duration / ITERATIONS as u32
    );
}

/// 内存使用分析
fn analyze_memory_usage() {
    use std::mem;

    println!("基本类型大小：");
    println!("i32: {} bytes", mem::size_of::<i32>());
    println!("Cell<i32>: {} bytes", mem::size_of::<Cell<i32>>());
    println!("RefCell<i32>: {} bytes", mem::size_of::<RefCell<i32>>());

    println!("\n复合类型大小：");
    println!("Vec<i32>: {} bytes", mem::size_of::<Vec<i32>>());
    println!("Cell<Vec<i32>>: 不可能，Vec 不实现 Copy");
    println!(
        "RefCell<Vec<i32>>: {} bytes",
        mem::size_of::<RefCell<Vec<i32>>>()
    );

    println!("\n智能指针大小：");
    println!("Rc<i32>: {} bytes", mem::size_of::<Rc<i32>>());
    println!(
        "Rc<RefCell<i32>>: {} bytes",
        mem::size_of::<Rc<RefCell<i32>>>()
    );

    // 内存开销分析
    let overhead = mem::size_of::<RefCell<i32>>() - mem::size_of::<i32>();
    println!("\nRefCell 内存开销: {} bytes (用于借用计数)", overhead);
}

/// 最佳实践建议
fn provide_best_practices() {
    println!("✅ 最佳实践建议：");
    println!("1. 优先使用编译时借用检查，只在必要时使用内部可变性");
    println!("2. 对于 Copy 类型，优先使用 Cell 而不是 RefCell");
    println!("3. 使用 try_borrow 系列方法避免 panic");
    println!("4. 尽量缩短借用的生命周期，及时释放借用");
    println!("5. 在多线程环境中使用 Arc<Mutex<T>> 而不是 Rc<RefCell<T>>");
    println!("6. 考虑使用 OnceCell 或 LazyCell 进行懒初始化");
    println!("7. 避免在性能关键路径上频繁使用 RefCell");
    println!("8. 使用借用分割技术避免不必要的借用冲突");
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_basic_operations() {
        let cell = Cell::new(42);
        assert_eq!(cell.get(), 42);

        cell.set(100);
        assert_eq!(cell.get(), 100);

        let old = cell.replace(200);
        assert_eq!(old, 100);
        assert_eq!(cell.get(), 200);
    }

    #[test]
    fn test_refcell_basic_operations() {
        let refcell = RefCell::new(vec![1, 2, 3]);

        // 不可变借用
        {
            let borrow = refcell.borrow();
            assert_eq!(*borrow, vec![1, 2, 3]);
        }

        // 可变借用
        {
            let mut borrow = refcell.borrow_mut();
            borrow.push(4);
            assert_eq!(*borrow, vec![1, 2, 3, 4]);
        }
    }

    #[test]
    fn test_refcell_try_borrow() {
        let refcell = RefCell::new(42);

        let _borrow = refcell.borrow_mut();

        // 应该失败，因为已经有可变借用
        assert!(refcell.try_borrow().is_err());
        assert!(refcell.try_borrow_mut().is_err());
    }

    #[test]
    fn test_rc_refcell_combination() {
        let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
        let clone1 = Rc::clone(&shared);
        let clone2 = Rc::clone(&shared);

        clone1.borrow_mut().push(4);
        clone2.borrow_mut().push(5);

        assert_eq!(*shared.borrow(), vec![1, 2, 3, 4, 5]);
        assert_eq!(Rc::strong_count(&shared), 3);
    }

    #[test]
    fn test_tree_structure() {
        let root = TreeNode::new(1);
        let child = TreeNode::new(2);

        root.add_child(child.clone());

        assert_eq!(root.children.borrow().len(), 1);
        assert_eq!(root.children.borrow()[0].value, 2);
    }

    #[test]
    fn test_cache_system() {
        let cache = Cache::new();

        cache.set("key1".to_string(), "value1".to_string());

        assert_eq!(cache.get("key1"), Some("value1".to_string()));
        assert_eq!(cache.get("key2"), None);

        let (hits, misses) = cache.stats();
        assert_eq!(hits, 1);
        assert_eq!(misses, 1);
    }

    #[test]
    fn test_state_machine() {
        let sm = StateMachine::new();

        assert_eq!(sm.current_state(), State::Idle);

        sm.transition_to(State::Running).unwrap();
        assert_eq!(sm.current_state(), State::Running);

        // 无效转换应该失败
        assert!(sm.transition_to(State::Idle).is_err());
    }

    #[test]
    fn test_lazy_loading() {
        let lazy = LazyData::new();

        // 第一次访问应该触发计算
        let data1 = lazy.get_data();
        assert_eq!(data1.len(), 1000);

        // 第二次访问应该使用缓存
        let data2 = lazy.get_data();
        assert_eq!(data2.len(), 1000);
    }
}
