//! Rust线程安全基于Send和Sync Trait的深度教程
//! 
//! 本教程全面深入地探讨Rust中的Send和Sync trait，这是Rust并发安全的核心机制。
//! Send和Sync是标记trait（marker traits），用于指示类型在多线程环境中的安全性。

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use std::cell::{Cell, RefCell, UnsafeCell};
use std::rc::Rc;
use std::marker::PhantomData;
use std::ptr::NonNull;

fn main() {
    println!("=== Rust线程安全基于Send和Sync Trait的深度教程 ===");
    
    // 1. Send和Sync trait基础
    send_sync_basics_demo();
    
    // 2. Send trait深入分析
    send_trait_deep_dive_demo();
    
    // 3. Sync trait深入分析
    sync_trait_deep_dive_demo();
    
    // 4. 标准库类型分析
    standard_types_analysis_demo();
    
    // 5. 内部可变性与线程安全
    interior_mutability_demo();
    
    // 6. unsafe实现Send和Sync
    unsafe_impl_send_sync_demo();
    
    // 7. 高级模式
    advanced_patterns_demo();
    
    // 8. 实际应用案例
    real_world_examples_demo();
    
    println!("\n=== 教程完成 ===");
}

/// 1. Send和Sync trait基础演示
fn send_sync_basics_demo() {
    println!("\n=== 1. Send和Sync Trait基础 ===");
    
    // Send和Sync的定义
    println!("\n1.1 Send和Sync的定义：");
    println!("- Send: 类型可以安全地在线程间移动（转移所有权）");
    println!("- Sync: 类型可以安全地在线程间共享（通过不可变引用）");
    println!("- T: Sync 等价于 &T: Send");
    
    // 标记trait特性
    println!("\n1.2 标记Trait特性：");
    println!("- 没有方法需要实现");
    println!("- 由编译器自动派生");
    println!("- 是unsafe trait，手动实现需要unsafe");
    
    // 自动派生规则
    println!("\n1.3 自动派生规则：");
    println!("- 如果类型的所有字段都是Send，则类型自动实现Send");
    println!("- 如果类型的所有字段都是Sync，则类型自动实现Sync");
    
    // 基本类型的Send/Sync状态
    demonstrate_basic_types_send_sync();
}

/// 演示基本类型的Send/Sync状态
fn demonstrate_basic_types_send_sync() {
    println!("\n1.4 基本类型的Send/Sync状态：");
    
    // 原始类型都是Send + Sync
    let number: i32 = 42;
    let text: &str = "hello";
    
    // 可以在线程间移动和共享
    let handle = thread::spawn(move || {
        println!("线程中的数字: {}", number);
        println!("线程中的文本: {}", text);
    });
    
    handle.join().unwrap();
    
    println!("✓ 原始类型（i32, &str等）都是Send + Sync");
}

/// 2. Send trait深入分析演示
fn send_trait_deep_dive_demo() {
    println!("\n=== 2. Send Trait深入分析 ===");
    
    // Send的含义
    println!("\n2.1 Send的含义：");
    println!("- 类型T是Send意味着T的所有权可以安全地转移到另一个线程");
    println!("- 包括T本身和&mut T都可以跨线程移动");
    
    // Send的典型例子
    demonstrate_send_examples();
    
    // 不是Send的例子
    demonstrate_non_send_examples();
    
    // Send的实际应用
    demonstrate_send_usage();
}

/// 演示Send的典型例子
fn demonstrate_send_examples() {
    println!("\n2.2 Send的典型例子：");
    
    // Vec<T>是Send（当T是Send时）
    let data = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        println!("线程中的向量: {:?}", data);
        data.len()
    });
    
    let len = handle.join().unwrap();
    println!("✓ Vec<i32>是Send，可以移动到其他线程，长度: {}", len);
    
    // String是Send
    let message = String::from("Hello from another thread!");
    let handle = thread::spawn(move || {
        println!("线程中的消息: {}", message);
    });
    
    handle.join().unwrap();
    println!("✓ String是Send，可以移动到其他线程");
}

/// 演示不是Send的例子
fn demonstrate_non_send_examples() {
    println!("\n2.3 不是Send的例子：");
    
    // Rc<T>不是Send
    let rc_data = Rc::new(vec![1, 2, 3]);
    println!("✗ Rc<T>不是Send，因为引用计数不是线程安全的");
    
    // 以下代码会编译错误：
    // let handle = thread::spawn(move || {
    //     println!("Rc data: {:?}", rc_data);
    // });
    
    // MutexGuard不是Send
    println!("✗ MutexGuard不是Send，因为必须在获取锁的线程中释放");
    
    // 原始指针不是Send
    let raw_ptr: *const i32 = &42;
    println!("✗ 原始指针(*const T, *mut T)不是Send，因为没有安全保证");
    
    // 避免编译器警告
    drop(rc_data);
    drop(raw_ptr);
}

/// 演示Send的实际应用
fn demonstrate_send_usage() {
    println!("\n2.4 Send的实际应用：");
    
    // 使用Arc代替Rc来实现跨线程共享
    let arc_data = Arc::new(vec![1, 2, 3, 4, 5]);
    let arc_clone = Arc::clone(&arc_data);
    
    let handle = thread::spawn(move || {
        println!("线程中的Arc数据: {:?}", arc_clone);
        arc_clone.len()
    });
    
    let len = handle.join().unwrap();
    println!("✓ Arc<Vec<i32>>是Send，可以在线程间共享，长度: {}", len);
    println!("主线程中的Arc数据: {:?}", arc_data);
}

/// 3. Sync trait深入分析演示
fn sync_trait_deep_dive_demo() {
    println!("\n=== 3. Sync Trait深入分析 ===");
    
    // Sync的含义
    println!("\n3.1 Sync的含义：");
    println!("- 类型T是Sync意味着&T可以安全地在多个线程间共享");
    println!("- T: Sync 等价于 &T: Send");
    println!("- 多个线程可以同时持有&T而不会造成数据竞争");
    
    // Sync的典型例子
    demonstrate_sync_examples();
    
    // 不是Sync的例子
    demonstrate_non_sync_examples();
    
    // Send和Sync的关系
    demonstrate_send_sync_relationship();
}

/// 演示Sync的典型例子
fn demonstrate_sync_examples() {
    println!("\n3.2 Sync的典型例子：");
    
    // 原始类型是Sync
    let shared_number = Arc::new(42i32);
    let mut handles = vec![];
    
    for i in 0..3 {
        let number_clone = Arc::clone(&shared_number);
        let handle = thread::spawn(move || {
            println!("线程{}: 共享的数字是 {}", i, number_clone);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("✓ i32是Sync，可以通过Arc在多个线程间安全共享");
    
    // Mutex<T>是Sync（当T是Send时）
    let shared_counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..3 {
        let counter_clone = Arc::clone(&shared_counter);
        let handle = thread::spawn(move || {
            let mut count = counter_clone.lock().unwrap();
            *count += 1;
            println!("线程{}: 计数器值 {}", i, *count);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("✓ Mutex<i32>是Sync，可以在多个线程间安全共享可变状态");
}

/// 演示不是Sync的例子
fn demonstrate_non_sync_examples() {
    println!("\n3.3 不是Sync的例子：");
    
    // Cell<T>不是Sync
    println!("✗ Cell<T>不是Sync，因为它提供非线程安全的内部可变性");
    
    // RefCell<T>不是Sync
    println!("✗ RefCell<T>不是Sync，因为它的借用检查不是线程安全的");
    
    // Rc<T>不是Sync
    println!("✗ Rc<T>不是Sync，因为引用计数操作不是原子的");
    
    // UnsafeCell<T>不是Sync
    println!("✗ UnsafeCell<T>不是Sync，因为它允许通过共享引用进行可变访问");
    
    // 演示为什么Cell不是Sync
    let cell = Cell::new(42);
    println!("Cell值: {}", cell.get());
    cell.set(100);
    println!("Cell新值: {}", cell.get());
    println!("如果Cell是Sync，多个线程同时调用set()会造成数据竞争");
}

/// 演示Send和Sync的关系
fn demonstrate_send_sync_relationship() {
    println!("\n3.4 Send和Sync的关系：");
    
    println!("关系矩阵：");
    println!("- Send + Sync: 大多数类型（i32, String, Vec<T>, Arc<T>等）");
    println!("- Send + !Sync: mpsc::Receiver<T>");
    println!("- !Send + Sync: MutexGuard<T>（在某些平台上）");
    println!("- !Send + !Sync: Rc<T>, Cell<T>, RefCell<T>");
    
    // 实际验证
    verify_send_sync_combinations();
}

/// 验证Send和Sync的组合
fn verify_send_sync_combinations() {
    use std::sync::mpsc;
    
    println!("\n验证不同类型的Send/Sync状态：");
    
    // 创建一个channel来演示Receiver的特性
    let (tx, rx) = mpsc::channel::<i32>();
    
    // Sender是Send + !Sync
    let handle = thread::spawn(move || {
        tx.send(42).unwrap();
        println!("✓ Sender<T>是Send，可以移动到其他线程");
    });
    
    handle.join().unwrap();
    
    // Receiver是Send + !Sync
    let handle = thread::spawn(move || {
        let value = rx.recv().unwrap();
        println!("✓ Receiver<T>是Send，接收到值: {}", value);
    });
    
    handle.join().unwrap();
    println!("✗ Receiver<T>不是Sync，不能在多个线程间共享");
}

/// 4. 标准库类型分析演示
fn standard_types_analysis_demo() {
    println!("\n=== 4. 标准库类型分析 ===");
    
    // 原始类型
    analyze_primitive_types();
    
    // 智能指针
    analyze_smart_pointers();
    
    // 集合类型
    analyze_collection_types();
    
    // 同步原语
    analyze_sync_primitives();
}

/// 分析原始类型
fn analyze_primitive_types() {
    println!("\n4.1 原始类型分析：");
    
    println!("Send + Sync类型：");
    println!("- 数值类型: i8, i16, i32, i64, i128, isize");
    println!("- 数值类型: u8, u16, u32, u64, u128, usize");
    println!("- 浮点类型: f32, f64");
    println!("- 布尔类型: bool");
    println!("- 字符类型: char");
    println!("- 单元类型: ()");
    println!("- 函数指针: fn()");
    
    // 演示原始类型的线程安全性
    let numbers = vec![1i32, 2, 3, 4, 5];
    let sum = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for num in numbers {
        let sum_clone = Arc::clone(&sum);
        let handle = thread::spawn(move || {
            let mut total = sum_clone.lock().unwrap();
            *total += num;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("✓ 原始类型线程安全验证，总和: {}", *sum.lock().unwrap());
}

/// 分析智能指针
fn analyze_smart_pointers() {
    println!("\n4.2 智能指针分析：");
    
    println!("Box<T>:");
    println!("- Send: T是Send时");
    println!("- Sync: T是Sync时");
    
    println!("Rc<T>:");
    println!("- Send: 否（引用计数不是线程安全的）");
    println!("- Sync: 否（引用计数不是线程安全的）");
    
    println!("Arc<T>:");
    println!("- Send: T是Send + Sync时");
    println!("- Sync: T是Send + Sync时");
    
    // 演示智能指针的使用
    demonstrate_smart_pointer_usage();
}

/// 演示智能指针的使用
fn demonstrate_smart_pointer_usage() {
    println!("\n智能指针使用演示：");
    
    // Box<T>的使用
    let boxed_data = Box::new(vec![1, 2, 3, 4, 5]);
    let handle = thread::spawn(move || {
        println!("Box中的数据: {:?}", boxed_data);
        boxed_data.len()
    });
    
    let len = handle.join().unwrap();
    println!("✓ Box<Vec<i32>>可以移动到其他线程，长度: {}", len);
    
    // Arc<T>的使用
    let arc_data = Arc::new(String::from("共享字符串"));
    let mut handles = vec![];
    
    for i in 0..3 {
        let data_clone = Arc::clone(&arc_data);
        let handle = thread::spawn(move || {
            println!("线程{}: {}", i, data_clone);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("✓ Arc<String>可以在多个线程间共享");
}

/// 分析集合类型
fn analyze_collection_types() {
    println!("\n4.3 集合类型分析：");
    
    println!("Vec<T>:");
    println!("- Send: T是Send时");
    println!("- Sync: T是Sync时");
    
    println!("HashMap<K, V>:");
    println!("- Send: K和V都是Send时");
    println!("- Sync: K和V都是Sync时");
    
    println!("BTreeMap<K, V>:");
    println!("- Send: K和V都是Send时");
    println!("- Sync: K和V都是Sync时");
    
    // 演示集合类型的线程安全性
    demonstrate_collection_thread_safety();
}

/// 演示集合类型的线程安全性
fn demonstrate_collection_thread_safety() {
    use std::collections::HashMap;
    
    println!("\n集合类型线程安全演示：");
    
    // 共享HashMap
    let shared_map = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];
    
    for i in 0..3 {
        let map_clone = Arc::clone(&shared_map);
        let handle = thread::spawn(move || {
            let mut map = map_clone.lock().unwrap();
            map.insert(format!("key{}", i), i * 10);
            println!("线程{}插入了 key{} -> {}", i, i, i * 10);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_map = shared_map.lock().unwrap();
    println!("✓ 最终HashMap: {:?}", *final_map);
}

/// 分析同步原语
fn analyze_sync_primitives() {
    println!("\n4.4 同步原语分析：");
    
    println!("Mutex<T>:");
    println!("- Send: T是Send时");
    println!("- Sync: T是Send时（注意不需要T是Sync）");
    
    println!("RwLock<T>:");
    println!("- Send: T是Send时");
    println!("- Sync: T是Send + Sync时");
    
    println!("MutexGuard<T>:");
    println!("- Send: 否（必须在获取锁的线程中释放）");
    println!("- Sync: 否（同上）");
    
    // 演示同步原语的特性
    demonstrate_sync_primitives_characteristics();
}

/// 演示同步原语的特性
fn demonstrate_sync_primitives_characteristics() {
    println!("\n同步原语特性演示：");
    
    // Mutex的使用
    let mutex_data = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..5 {
        let data_clone = Arc::clone(&mutex_data);
        let handle = thread::spawn(move || {
            let mut value = data_clone.lock().unwrap();
            *value += i;
            println!("线程{}更新值为: {}", i, *value);
            // MutexGuard在这里自动释放
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("✓ Mutex最终值: {}", *mutex_data.lock().unwrap());
    
    // RwLock的使用
    let rwlock_data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];
    
    // 多个读者
    for i in 0..3 {
        let data_clone = Arc::clone(&rwlock_data);
        let handle = thread::spawn(move || {
            let data = data_clone.read().unwrap();
            println!("读者{}: {:?}", i, *data);
        });
        handles.push(handle);
    }
    
    // 一个写者
    let data_clone = Arc::clone(&rwlock_data);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10)); // 确保读者先执行
        let mut data = data_clone.write().unwrap();
        data.push(4);
        println!("写者添加元素，新数据: {:?}", *data);
    });
    handles.push(handle);
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("✓ RwLock允许多个读者或一个写者");
}

/// 5. 内部可变性与线程安全演示
fn interior_mutability_demo() {
    println!("\n=== 5. 内部可变性与线程安全 ===");
    
    // UnsafeCell - 内部可变性的基础
    demonstrate_unsafe_cell();
    
    // Cell - 单线程内部可变性
    demonstrate_cell();
    
    // RefCell - 运行时借用检查
    demonstrate_refcell();
    
    // Mutex - 线程安全的内部可变性
    demonstrate_mutex_interior_mutability();
}

/// 演示UnsafeCell
fn demonstrate_unsafe_cell() {
    println!("\n5.1 UnsafeCell - 内部可变性的基础：");
    
    let cell = UnsafeCell::new(42);
    
    unsafe {
        let ptr = cell.get();
        println!("UnsafeCell值: {}", *ptr);
        *ptr = 100;
        println!("UnsafeCell新值: {}", *ptr);
    }
    
    println!("✓ UnsafeCell允许通过不可变引用进行可变访问");
    println!("✗ UnsafeCell不是Sync，因为没有同步保证");
}

/// 演示Cell
fn demonstrate_cell() {
    println!("\n5.2 Cell - 单线程内部可变性：");
    
    let cell = Cell::new(42);
    println!("Cell初始值: {}", cell.get());
    
    cell.set(100);
    println!("Cell新值: {}", cell.get());
    
    // Cell的replace方法
    let old_value = cell.replace(200);
    println!("Cell替换值: {} -> {}", old_value, cell.get());
    
    println!("✓ Cell提供Copy类型的内部可变性");
    println!("✗ Cell不是Sync，只能在单线程中使用");
}

/// 演示RefCell
fn demonstrate_refcell() {
    println!("\n5.3 RefCell - 运行时借用检查：");
    
    let refcell = RefCell::new(vec![1, 2, 3]);
    
    // 不可变借用
    {
        let borrowed = refcell.borrow();
        println!("RefCell不可变借用: {:?}", *borrowed);
    }
    
    // 可变借用
    {
        let mut borrowed = refcell.borrow_mut();
        borrowed.push(4);
        println!("RefCell可变借用后: {:?}", *borrowed);
    }
    
    // try_borrow演示
    let borrow1 = refcell.try_borrow();
    let borrow2 = refcell.try_borrow();
    
    match (borrow1, borrow2) {
        (Ok(b1), Ok(b2)) => {
            println!("✓ RefCell允许多个不可变借用: {:?}, {:?}", *b1, *b2);
        }
        _ => println!("✗ RefCell借用失败"),
    }
    
    println!("✓ RefCell提供运行时借用检查");
    println!("✗ RefCell不是Sync，只能在单线程中使用");
}

/// 演示Mutex的内部可变性
fn demonstrate_mutex_interior_mutability() {
    println!("\n5.4 Mutex - 线程安全的内部可变性：");
    
    let mutex_data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let mut handles = vec![];
    
    for i in 0..3 {
        let data_clone = Arc::clone(&mutex_data);
        let handle = thread::spawn(move || {
            let mut data = data_clone.lock().unwrap();
            data.push(i + 10);
            println!("线程{}添加元素，当前数据: {:?}", i, *data);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("✓ Mutex提供线程安全的内部可变性");
    println!("✓ Mutex<T>是Sync（当T是Send时）");
}

/// 6. unsafe实现Send和Sync演示
fn unsafe_impl_send_sync_demo() {
    println!("\n=== 6. Unsafe实现Send和Sync ===");
    
    // 何时需要unsafe实现
    println!("\n6.1 何时需要unsafe实现：");
    println!("- 包含原始指针的类型");
    println!("- 使用UnsafeCell但确保线程安全的类型");
    println!("- 与外部系统交互的类型");
    println!("- 需要特殊同步保证的类型");
    
    // 演示自定义类型的Send/Sync实现
    demonstrate_custom_send_sync();
    
    // 演示负实现
    demonstrate_negative_impls();
    
    // 安全性考虑
    discuss_safety_considerations();
}

/// 自定义Send/Sync类型
struct MyBox<T> {
    ptr: NonNull<T>,
    _marker: PhantomData<T>,
}

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        let boxed = Box::new(value);
        let ptr = NonNull::new(Box::into_raw(boxed)).unwrap();
        MyBox {
            ptr,
            _marker: PhantomData,
        }
    }
    
    fn get(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.ptr.as_ptr());
        }
    }
}

// 安全地实现Send和Sync
unsafe impl<T: Send> Send for MyBox<T> {}
unsafe impl<T: Sync> Sync for MyBox<T> {}

/// 演示自定义Send/Sync实现
fn demonstrate_custom_send_sync() {
    println!("\n6.2 自定义Send/Sync实现演示：");
    
    let my_box = MyBox::new(42);
    println!("MyBox值: {}", my_box.get());
    
    // 测试Send
    let my_box = MyBox::new(String::from("Hello"));
    let handle = thread::spawn(move || {
        println!("线程中的MyBox: {}", my_box.get());
    });
    
    handle.join().unwrap();
    println!("✓ MyBox<T>实现了Send（当T: Send时）");
    
    // 测试Sync
    let shared_box = Arc::new(MyBox::new(vec![1, 2, 3]));
    let mut handles = vec![];
    
    for i in 0..3 {
        let box_clone = Arc::clone(&shared_box);
        let handle = thread::spawn(move || {
            println!("线程{}访问MyBox: {:?}", i, box_clone.get());
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("✓ MyBox<T>实现了Sync（当T: Sync时）");
}

/// 不是线程安全的类型（演示用）
struct NotThreadSafe {
    _data: *mut u8,
}

impl NotThreadSafe {
    fn new() -> Self {
        NotThreadSafe {
            _data: std::ptr::null_mut(),
        }
    }
}

// 负实现（需要nightly Rust）
// impl !Send for NotThreadSafe {}
// impl !Sync for NotThreadSafe {}

/// 演示负实现
fn demonstrate_negative_impls() {
    println!("\n6.3 负实现演示：");
    
    let not_safe = NotThreadSafe::new();
    println!("✓ 创建了NotThreadSafe实例");
    
    // 以下代码在启用负实现时会编译错误：
    // let handle = thread::spawn(move || {
    //     println!("NotThreadSafe: {:?}", not_safe);
    // });
    
    println!("负实现用于明确标记类型不是Send或Sync");
    println!("需要nightly Rust和#![feature(negative_impls)]");
    
    drop(not_safe);
}

/// 讨论安全性考虑
fn discuss_safety_considerations() {
    println!("\n6.4 安全性考虑：");
    
    println!("实现Send的安全要求：");
    println!("- 类型的所有权可以安全地转移到其他线程");
    println!("- 不能包含线程本地数据");
    println!("- 析构函数可以在任何线程中运行");
    
    println!("\n实现Sync的安全要求：");
    println!("- 多个线程可以同时持有&T而不造成数据竞争");
    println!("- 所有通过&T的操作都必须是线程安全的");
    println!("- 内部可变性必须使用同步原语保护");
    
    println!("\n常见错误：");
    println!("- 错误地为包含非线程安全数据的类型实现Send/Sync");
    println!("- 忽略内部可变性的同步需求");
    println!("- 不考虑析构函数的线程安全性");
}

/// 7. 高级模式演示
fn advanced_patterns_demo() {
    println!("\n=== 7. 高级模式 ===");
    
    // PhantomData的使用
    demonstrate_phantom_data();
    
    // 条件实现
    demonstrate_conditional_impls();
    
    // Trait bounds的使用
    demonstrate_trait_bounds();
    
    // 类型系统技巧
    demonstrate_type_system_tricks();
}

/// PhantomData演示
struct MyContainer<T> {
    data: *mut u8,
    len: usize,
    _marker: PhantomData<T>,
}

impl<T> MyContainer<T> {
    fn new() -> Self {
        MyContainer {
            data: std::ptr::null_mut(),
            len: 0,
            _marker: PhantomData,
        }
    }
}

// PhantomData确保Send/Sync的正确派生
unsafe impl<T: Send> Send for MyContainer<T> {}
unsafe impl<T: Sync> Sync for MyContainer<T> {}

/// 演示PhantomData
fn demonstrate_phantom_data() {
    println!("\n7.1 PhantomData演示：");
    
    let container: MyContainer<i32> = MyContainer::new();
    println!("✓ 创建了MyContainer<i32>");
    
    println!("PhantomData的作用：");
    println!("- 标记类型参数的所有权");
    println!("- 影响Send/Sync的自动派生");
    println!("- 提供类型安全保证");
    
    drop(container);
}

/// 演示条件实现
fn demonstrate_conditional_impls() {
    println!("\n7.2 条件实现演示：");
    
    println!("标准库中的条件实现示例：");
    println!("- Arc<T>: Send + Sync when T: Send + Sync");
    println!("- Mutex<T>: Send + Sync when T: Send");
    println!("- RwLock<T>: Send when T: Send, Sync when T: Send + Sync");
    
    // 演示Arc的条件实现
    let arc_i32 = Arc::new(42i32);
    let handle = thread::spawn(move || {
        println!("Arc<i32>在线程中: {}", arc_i32);
    });
    handle.join().unwrap();
    
    println!("✓ Arc<i32>是Send + Sync，因为i32是Send + Sync");
    
    // 如果T不是Send + Sync，Arc<T>也不会是
    let rc = Rc::new(42);
    // let arc_rc = Arc::new(rc); // 这会编译错误，因为Rc不是Send + Sync
    drop(rc);
    println!("✗ Arc<Rc<T>>不是Send + Sync，因为Rc<T>不是Send + Sync");
}

/// 演示trait bounds
fn demonstrate_trait_bounds() {
    println!("\n7.3 Trait Bounds演示：");
    
    // 需要Send的函数
    fn spawn_with_data<T: Send + 'static>(data: T) -> thread::JoinHandle<()> 
    where
        T: std::fmt::Debug,
    {
        thread::spawn(move || {
            println!("线程中的数据: {:?}", data);
        })
    }
    
    // 需要Send + Sync的函数
    fn share_data<T: Send + Sync + 'static>(data: T) -> Arc<T> {
        Arc::new(data)
    }
    
    // 使用示例
    let handle = spawn_with_data(String::from("Hello"));
    handle.join().unwrap();
    println!("✓ spawn_with_data要求T: Send");
    
    let shared = share_data(42i32);
    let mut handles = vec![];
    
    for i in 0..3 {
        let data_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            println!("线程{}访问共享数据: {}", i, data_clone);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("✓ share_data要求T: Send + Sync");
}

/// 演示类型系统技巧
fn demonstrate_type_system_tricks() {
    println!("\n7.4 类型系统技巧：");
    
    // 使用类型状态模式
    struct Locked;
    struct Unlocked;
    
    struct StateMachine<State> {
        data: i32,
        _state: PhantomData<State>,
    }
    
    impl StateMachine<Unlocked> {
        fn new(data: i32) -> Self {
            StateMachine {
                data,
                _state: PhantomData,
            }
        }
        
        fn lock(self) -> StateMachine<Locked> {
            StateMachine {
                data: self.data,
                _state: PhantomData,
            }
        }
    }
    
    impl StateMachine<Locked> {
        fn unlock(self) -> StateMachine<Unlocked> {
            StateMachine {
                data: self.data,
                _state: PhantomData,
            }
        }
        
        fn get_data(&self) -> i32 {
            self.data
        }
    }
    
    // 只有Locked状态的StateMachine可以访问数据
    unsafe impl<State> Send for StateMachine<State> {}
    unsafe impl Send for Locked {}
    unsafe impl Send for Unlocked {}
    
    let machine = StateMachine::new(42);
    let locked = machine.lock();
    println!("✓ 状态机数据: {}", locked.get_data());
    let _unlocked = locked.unlock();
    
    println!("类型状态模式确保编译时状态安全");
}

/// 8. 实际应用案例演示
fn real_world_examples_demo() {
    println!("\n=== 8. 实际应用案例 ===");
    
    // 线程安全的计数器
    demonstrate_thread_safe_counter();
    
    // 生产者消费者模式
    demonstrate_producer_consumer();
    
    // 线程池
    demonstrate_thread_pool_concept();
    
    // 缓存系统
    demonstrate_cache_system();
}

/// 线程安全的计数器
struct ThreadSafeCounter {
    count: Arc<Mutex<u64>>,
}

impl ThreadSafeCounter {
    fn new() -> Self {
        ThreadSafeCounter {
            count: Arc::new(Mutex::new(0)),
        }
    }
    
    fn increment(&self) -> u64 {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        *count
    }
    
    fn get(&self) -> u64 {
        *self.count.lock().unwrap()
    }
}

// ThreadSafeCounter自动实现Send + Sync

/// 演示线程安全计数器
fn demonstrate_thread_safe_counter() {
    println!("\n8.1 线程安全计数器：");
    
    let counter = Arc::new(ThreadSafeCounter::new());
    let mut handles = vec![];
    
    for i in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let count = counter_clone.increment();
                if count % 10 == 0 {
                    println!("线程{}达到计数: {}", i, count);
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("✓ 最终计数: {}", counter.get());
}

/// 演示生产者消费者模式
fn demonstrate_producer_consumer() {
    use std::sync::mpsc;
    
    println!("\n8.2 生产者消费者模式：");
    
    let (tx, rx) = mpsc::channel::<String>();
    let rx = Arc::new(Mutex::new(rx));
    
    // 生产者
    let tx_clone = tx.clone();
    let producer = thread::spawn(move || {
        for i in 0..10 {
            tx_clone.send(format!("消息{}", i)).unwrap();
            println!("生产者发送: 消息{}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // 消费者
    let mut consumers = vec![];
    for i in 0..3 {
        let rx_clone = Arc::clone(&rx);
        let consumer = thread::spawn(move || {
            loop {
                let message = {
                    let receiver = rx_clone.lock().unwrap();
                    receiver.try_recv()
                };
                
                match message {
                    Ok(msg) => println!("消费者{}接收: {}", i, msg),
                    Err(_) => {
                        thread::sleep(Duration::from_millis(50));
                        continue;
                    }
                }
            }
        });
        consumers.push(consumer);
    }
    
    producer.join().unwrap();
    drop(tx); // 关闭发送端
    
    thread::sleep(Duration::from_millis(500)); // 等待消费者处理完
    
    println!("✓ 生产者消费者模式演示完成");
}

/// 演示线程池概念
fn demonstrate_thread_pool_concept() {
    println!("\n8.3 线程池概念演示：");
    
    use std::sync::mpsc;
    
    type Job = Box<dyn FnOnce() + Send + 'static>;
    
    struct SimpleThreadPool {
        sender: mpsc::Sender<Job>,
        _handles: Vec<thread::JoinHandle<()>>,
    }
    
    impl SimpleThreadPool {
        fn new(size: usize) -> Self {
            let (sender, receiver) = mpsc::channel::<Job>();
            let receiver = Arc::new(Mutex::new(receiver));
            let mut handles = Vec::with_capacity(size);
            
            for i in 0..size {
                let receiver = Arc::clone(&receiver);
                let handle = thread::spawn(move || {
                    loop {
                        let job = {
                            let receiver = receiver.lock().unwrap();
                            receiver.recv()
                        };
                        
                        match job {
                            Ok(job) => {
                                println!("工作线程{}执行任务", i);
                                job();
                            }
                            Err(_) => break,
                        }
                    }
                });
                handles.push(handle);
            }
            
            SimpleThreadPool {
                sender,
                _handles: handles,
            }
        }
        
        fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
    }
    
    let pool = SimpleThreadPool::new(3);
    
    for i in 0..5 {
        pool.execute(move || {
            println!("执行任务{}", i);
            thread::sleep(Duration::from_millis(100));
        });
    }
    
    thread::sleep(Duration::from_millis(1000));
    println!("✓ 线程池概念演示完成");
}

/// 线程安全的缓存系统
struct ThreadSafeCache<K, V> 
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    cache: Arc<RwLock<std::collections::HashMap<K, V>>>,
}

impl<K, V> ThreadSafeCache<K, V>
where
    K: std::hash::Hash + Eq + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    fn new() -> Self {
        ThreadSafeCache {
            cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }
    
    fn get(&self, key: &K) -> Option<V> {
        let cache = self.cache.read().unwrap();
        cache.get(key).cloned()
    }
    
    fn insert(&self, key: K, value: V) {
        let mut cache = self.cache.write().unwrap();
        cache.insert(key, value);
    }
    
    fn len(&self) -> usize {
        let cache = self.cache.read().unwrap();
        cache.len()
    }
}

// ThreadSafeCache自动实现Send + Sync

/// 演示缓存系统
fn demonstrate_cache_system() {
    println!("\n8.4 线程安全缓存系统：");
    
    let cache = Arc::new(ThreadSafeCache::new());
    let mut handles = vec![];
    
    // 写入线程
    for i in 0..3 {
        let cache_clone = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            for j in 0..5 {
                let key = format!("key_{}_{}", i, j);
                let value = format!("value_{}_{}", i, j);
                cache_clone.insert(key.clone(), value.clone());
                println!("线程{}插入: {} -> {}", i, key, value);
            }
        });
        handles.push(handle);
    }
    
    // 读取线程
    for i in 0..2 {
        let cache_clone = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100)); // 等待一些数据被插入
            for j in 0..3 {
                let key = format!("key_0_{}", j);
                if let Some(value) = cache_clone.get(&key) {
                    println!("读取线程{}获取: {} -> {}", i, key, value);
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("✓ 缓存系统最终大小: {}", cache.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_send_sync_basics() {
        // 测试基本类型是Send + Sync
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        
        assert_send::<i32>();
        assert_sync::<i32>();
        assert_send::<String>();
        assert_sync::<String>();
    }
    
    #[test]
    fn test_arc_mutex() {
        let data = Arc::new(Mutex::new(0));
        let data_clone = Arc::clone(&data);
        
        let handle = thread::spawn(move || {
            let mut value = data_clone.lock().unwrap();
            *value += 1;
        });
        
        handle.join().unwrap();
        assert_eq!(*data.lock().unwrap(), 1);
    }
    
    #[test]
    fn test_thread_safe_counter() {
        let counter = Arc::new(ThreadSafeCounter::new());
        let mut handles = vec![];
        
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..10 {
                    counter_clone.increment();
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(counter.get(), 100);
    }
}
