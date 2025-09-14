//! # 安全性分析模块
//!
//! 本模块深入分析裸指针、引用和智能指针的安全性特征，
//! 展示常见的安全陷阱和最佳实践，帮助开发者编写安全的 Rust 代码。

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

/// 运行所有安全性分析
pub fn run_all_safety_analysis() {
    println!("\n🔒 安全性分析和常见陷阱");
    println!("{}", "=".repeat(60));
    
    // 引用安全性分析
    reference_safety_analysis();
    
    // 智能指针安全性分析
    smart_pointer_safety_analysis();
    
    // 裸指针安全性分析
    raw_pointer_safety_analysis();
    
    // 内存安全陷阱
    memory_safety_pitfalls();
    
    // 并发安全分析
    concurrency_safety_analysis();
    
    // 生命周期安全分析
    lifetime_safety_analysis();
    
    // 安全编程最佳实践
    safety_best_practices();
}

/// 引用安全性分析
fn reference_safety_analysis() {
    println!("\n📚 1. 引用安全性分析");
    println!("{}", "-".repeat(40));
    
    // 借用检查器保护
    println!("\n🛡️ 借用检查器保护:");
    borrow_checker_protection();
    
    // 生命周期安全
    println!("\n⏰ 生命周期安全:");
    lifetime_safety();
    
    // 可变性控制
    println!("\n🔄 可变性控制:");
    mutability_control();
}

/// 借用检查器保护示例
fn borrow_checker_protection() {
    println!("  借用检查器在编译时防止以下问题:");
    
    // 示例1：防止悬垂引用
    println!("\n  ❌ 悬垂引用 (编译时错误):");
    println!("    ```rust");
    println!("    fn dangling_reference() -> &i32 {{}}");
    println!("        let x = 42;");
    println!("        &x  // 错误：返回对局部变量的引用");
    println!("    }}");
    println!("    ```");
    
    // 正确的做法
    fn safe_reference() -> i32 {
        let x = 42;
        x  // 返回值而不是引用
    }
    
    let result = safe_reference();
    println!("  ✅ 安全做法: 返回值 = {}", result);
    
    // 示例2：防止数据竞争
    println!("\n  ❌ 数据竞争 (编译时错误):");
    println!("    ```rust");
    println!("    let mut data = vec![1, 2, 3];");
    println!("    let r1 = &data;");
    println!("    let r2 = &mut data;  // 错误：不能同时有可变和不可变引用");
    println!("    ```");
    
    // 正确的做法
    let mut data = vec![1, 2, 3];
    {
        let r1 = &data;
        println!("  ✅ 不可变引用: {:?}", r1);
    } // r1 的作用域结束
    {
        let r2 = &mut data;
        r2.push(4);
        println!("  ✅ 可变引用: {:?}", r2);
    }
    
    println!("  ✅ 借用检查器确保内存安全和线程安全");
}

/// 生命周期安全示例
fn lifetime_safety() {
    // 生命周期确保引用有效性
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("  最长字符串: {}", result);
    } // string2 在这里被销毁，但 result 在其作用域内使用是安全的
    
    // 结构体生命周期
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("  结构体引用: {:?}", excerpt);
    println!("  ✅ 生命周期注解确保引用在结构体存在期间有效");
}

/// 可变性控制示例
fn mutability_control() {
    // 内部可变性的安全使用
    let data = RefCell::new(vec![1, 2, 3]);
    
    // 安全的借用
    {
        let borrowed = data.borrow();
        println!("  不可变借用: {:?}", *borrowed);
    } // 借用在这里结束
    
    {
        let mut borrowed = data.borrow_mut();
        borrowed.push(4);
        println!("  可变借用后: {:?}", *borrowed);
    } // 可变借用在这里结束
    
    // 演示运行时借用检查
    println!("\n  🔍 运行时借用检查:");
    let borrow1 = data.borrow();
    
    // 尝试同时进行可变借用会导致 panic
    println!("  当前借用计数: {}", borrow1.len());
    
    // 这里我们不能调用 borrow_mut()，因为已经有不可变借用
    // let borrow2 = data.borrow_mut(); // 这会导致 panic
    
    drop(borrow1); // 显式释放借用
    
    let mut borrow2 = data.borrow_mut();
    borrow2.push(5);
    println!("  释放后可变借用: {:?}", *borrow2);
    
    println!("  ✅ RefCell 提供运行时借用检查，确保借用规则");
}

/// 智能指针安全性分析
fn smart_pointer_safety_analysis() {
    println!("\n🧠 2. 智能指针安全性分析");
    println!("{}", "-".repeat(40));
    
    // 引用计数安全
    println!("\n🔢 引用计数安全:");
    reference_counting_safety();
    
    // 循环引用问题
    println!("\n🔄 循环引用问题:");
    circular_reference_problem();
    
    // 内部可变性安全
    println!("\n🔒 内部可变性安全:");
    interior_mutability_safety();
}

/// 引用计数安全示例
fn reference_counting_safety() {
    // Rc 的安全使用
    let data = Rc::new(vec![1, 2, 3, 4, 5]);
    println!("  初始引用计数: {}", Rc::strong_count(&data));
    
    let data_clone1 = Rc::clone(&data);
    let data_clone2 = Rc::clone(&data);
    println!("  克隆后引用计数: {}", Rc::strong_count(&data));
    
    // 作用域控制引用计数
    {
        let data_clone3 = Rc::clone(&data);
        println!("  作用域内引用计数: {}", Rc::strong_count(&data));
    } // data_clone3 在这里被销毁
    
    println!("  作用域外引用计数: {}", Rc::strong_count(&data));
    
    // 显式释放引用
    drop(data_clone1);
    drop(data_clone2);
    println!("  释放后引用计数: {}", Rc::strong_count(&data));
    
    println!("  ✅ Rc 自动管理引用计数，防止内存泄漏");
}

/// 循环引用问题示例
fn circular_reference_problem() {
    use std::rc::Weak;
    
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
    
    impl Node {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(Node {
                value,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(Vec::new()),
            })
        }
        
        fn add_child(parent: &Rc<Node>, child: Rc<Node>) {
            child.parent.borrow_mut().clone_from(&Rc::downgrade(parent));
            parent.children.borrow_mut().push(child);
        }
    }
    
    // 创建父子节点
    let parent = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);
    
    println!("  创建节点 - 父节点引用计数: {}", Rc::strong_count(&parent));
    
    // 建立父子关系
    Node::add_child(&parent, child1);
    Node::add_child(&parent, child2);
    
    println!("  添加子节点后 - 父节点引用计数: {}", Rc::strong_count(&parent));
    
    // 访问子节点的父节点
    if let Some(child) = parent.children.borrow().first() {
        if let Some(parent_ref) = child.parent.borrow().upgrade() {
            println!("  子节点的父节点值: {}", parent_ref.value);
        }
    }
    
    println!("  ✅ 使用 Weak 引用打破循环引用，防止内存泄漏");
    
    // 演示循环引用的危险（注释掉的代码）
    println!("\n  ❌ 危险的循环引用模式:");
    println!("    ```rust");
    println!("    // 如果父节点和子节点都使用 Rc 互相引用");
    println!("    // 会导致循环引用，内存永远不会被释放");
    println!("    struct BadNode {{}}");
    println!("        parent: RefCell<Option<Rc<BadNode>>>,  // 强引用");
    println!("        children: RefCell<Vec<Rc<BadNode>>>,   // 强引用");
    println!("    }}");
    println!("    ```");
}

/// 内部可变性安全示例
fn interior_mutability_safety() {
    // Cell 的安全使用（Copy 类型）
    let cell_data = Cell::new(42);
    println!("  Cell 初始值: {}", cell_data.get());
    
    cell_data.set(100);
    println!("  Cell 更新值: {}", cell_data.get());
    
    // RefCell 的安全使用（非 Copy 类型）
    let refcell_data = RefCell::new(vec![1, 2, 3]);
    
    // 安全的借用模式
    {
        let borrowed = refcell_data.borrow();
        println!("  RefCell 读取: {:?}", *borrowed);
    }
    
    {
        let mut borrowed = refcell_data.borrow_mut();
        borrowed.push(4);
        println!("  RefCell 修改: {:?}", *borrowed);
    }
    
    // 演示借用检查
    println!("\n  🔍 RefCell 借用检查:");
    
    // 安全的嵌套借用
    fn safe_nested_borrow(data: &RefCell<Vec<i32>>) {
        let borrowed = data.borrow();
        println!("    嵌套借用长度: {}", borrowed.len());
        // 不在这里进行可变借用
    }
    
    safe_nested_borrow(&refcell_data);
    
    // 危险的模式（会导致 panic）
    println!("\n  ❌ 危险的借用模式:");
    println!("    ```rust");
    println!("    let borrow1 = data.borrow();");
    println!("    let borrow2 = data.borrow_mut(); // panic: already borrowed");
    println!("    ```");
    
    println!("  ✅ 内部可变性提供运行时安全检查");
}

/// 裸指针安全性分析
fn raw_pointer_safety_analysis() {
    println!("\n⚠️ 3. 裸指针安全性分析");
    println!("{}", "-".repeat(40));
    
    // 裸指针的危险性
    println!("\n💀 裸指针的危险性:");
    raw_pointer_dangers();
    
    // 安全的裸指针使用
    println!("\n🛡️ 安全的裸指针使用:");
    safe_raw_pointer_usage();
    
    // 裸指针最佳实践
    println!("\n📋 裸指针最佳实践:");
    raw_pointer_best_practices();
}

/// 裸指针危险性示例
fn raw_pointer_dangers() {
    println!("  裸指针可能导致的问题:");
    
    // 1. 悬垂指针
    println!("\n  1. 悬垂指针:");
    let dangling_ptr: *const i32;
    {
        let x = 42;
        dangling_ptr = &x as *const i32;
        // x 在这里被销毁
    }
    // 使用 dangling_ptr 是未定义行为
    println!("    悬垂指针地址: {:p} (不要解引用！)", dangling_ptr);
    
    // 2. 空指针解引用
    println!("\n  2. 空指针解引用:");
    let null_ptr: *const i32 = std::ptr::null();
    println!("    空指针: {:p}", null_ptr);
    // unsafe { println!("{}", *null_ptr); } // 这会导致段错误
    
    // 3. 缓冲区溢出
    println!("\n  3. 缓冲区溢出:");
    let array = [1, 2, 3, 4, 5];
    let ptr = array.as_ptr();
    unsafe {
        // 安全的访问
        println!("    安全访问 array[2]: {}", *ptr.add(2));
        
        // 危险的访问（越界）
        // println!("危险访问 array[10]: {}", *ptr.add(10)); // 未定义行为
    }
    
    // 4. 数据竞争
    println!("\n  4. 数据竞争:");
    println!("    裸指针可以绕过借用检查器，导致数据竞争");
    println!("    多个线程同时修改同一内存位置是未定义行为");
    
    println!("  ⚠️ 这些问题在 unsafe 块中不会被编译器检查");
}

/// 安全的裸指针使用示例
fn safe_raw_pointer_usage() {
    // 1. 空指针检查
    unsafe fn safe_deref(ptr: *const i32) -> Option<i32> {
        if ptr.is_null() {
            None
        } else {
            Some(*ptr)
        }
    }
    
    let value = 42;
    let ptr = &value as *const i32;
    let null_ptr: *const i32 = std::ptr::null();
    
    unsafe {
        println!("  安全解引用有效指针: {:?}", safe_deref(ptr));
        println!("  安全解引用空指针: {:?}", safe_deref(null_ptr));
    }
    
    // 2. 边界检查
    fn safe_array_access(ptr: *const i32, len: usize, index: usize) -> Option<i32> {
        if index < len {
            unsafe { Some(*ptr.add(index)) }
        } else {
            None
        }
    }
    
    let array = [10, 20, 30, 40, 50];
    let ptr = array.as_ptr();
    
    println!("  安全数组访问 [2]: {:?}", safe_array_access(ptr, array.len(), 2));
    println!("  安全数组访问 [10]: {:?}", safe_array_access(ptr, array.len(), 10));
    
    // 3. 生命周期管理
    struct SafePtr<'a> {
        ptr: *const i32,
        _marker: std::marker::PhantomData<&'a i32>,
    }
    
    impl<'a> SafePtr<'a> {
        fn new(reference: &'a i32) -> Self {
            SafePtr {
                ptr: reference as *const i32,
                _marker: std::marker::PhantomData,
            }
        }
        
        fn get(&self) -> i32 {
            unsafe { *self.ptr }
        }
    }
    
    let value = 100;
    let safe_ptr = SafePtr::new(&value);
    println!("  生命周期安全的指针: {}", safe_ptr.get());
    
    println!("  ✅ 通过检查和封装可以安全使用裸指针");
}

/// 裸指针最佳实践
fn raw_pointer_best_practices() {
    println!("  裸指针使用最佳实践:");
    println!("  1. 总是检查空指针");
    println!("  2. 验证指针有效性和边界");
    println!("  3. 使用生命周期参数约束指针");
    println!("  4. 封装 unsafe 操作在安全接口中");
    println!("  5. 最小化 unsafe 块的范围");
    println!("  6. 详细文档化安全不变量");
    println!("  7. 使用工具检测内存错误 (Miri, Valgrind)");
    
    // 示例：安全的裸指针封装
    struct SafeBuffer {
        ptr: *mut u8,
        len: usize,
        cap: usize,
    }
    
    impl SafeBuffer {
        fn new(capacity: usize) -> Self {
            let layout = std::alloc::Layout::array::<u8>(capacity).unwrap();
            let ptr = unsafe { std::alloc::alloc(layout) };
            
            if ptr.is_null() {
                panic!("内存分配失败");
            }
            
            SafeBuffer {
                ptr,
                len: 0,
                cap: capacity,
            }
        }
        
        fn push(&mut self, value: u8) -> Result<(), &'static str> {
            if self.len >= self.cap {
                return Err("缓冲区已满");
            }
            
            unsafe {
                *self.ptr.add(self.len) = value;
            }
            self.len += 1;
            Ok(())
        }
        
        fn get(&self, index: usize) -> Option<u8> {
            if index < self.len {
                unsafe { Some(*self.ptr.add(index)) }
            } else {
                None
            }
        }
        
        fn len(&self) -> usize {
            self.len
        }
    }
    
    impl Drop for SafeBuffer {
        fn drop(&mut self) {
            if !self.ptr.is_null() {
                let layout = std::alloc::Layout::array::<u8>(self.cap).unwrap();
                unsafe {
                    std::alloc::dealloc(self.ptr, layout);
                }
            }
        }
    }
    
    // 使用安全封装
    let mut buffer = SafeBuffer::new(10);
    
    for i in 0..5 {
        buffer.push(i * 10).unwrap();
    }
    
    println!("  安全缓冲区长度: {}", buffer.len());
    println!("  安全缓冲区 [2]: {:?}", buffer.get(2));
    println!("  安全缓冲区 [10]: {:?}", buffer.get(10));
    
    println!("  ✅ 安全封装隐藏了 unsafe 细节");
}

/// 内存安全陷阱
fn memory_safety_pitfalls() {
    println!("\n🕳️ 4. 内存安全陷阱");
    println!("{}", "-".repeat(40));
    
    // 双重释放
    println!("\n💥 双重释放:");
    double_free_prevention();
    
    // 使用后释放
    println!("\n🔄 使用后释放:");
    use_after_free_prevention();
    
    // 内存泄漏
    println!("\n💧 内存泄漏:");
    memory_leak_prevention();
}

/// 双重释放预防
fn double_free_prevention() {
    println!("  Rust 如何防止双重释放:");
    
    // Box 的移动语义防止双重释放
    let box1 = Box::new(42);
    let box2 = box1; // box1 被移动，不能再使用
    
    // println!("{}", box1); // 编译错误：使用已移动的值
    println!("  Box 值: {}", box2);
    
    // 显式释放
    drop(box2);
    // drop(box2); // 编译错误：使用已移动的值
    
    println!("  ✅ 移动语义确保每个值只被释放一次");
    
    // 危险的裸指针模式（注释掉）
    println!("\n  ❌ 危险的双重释放模式:");
    println!("    ```rust");
    println!("    let ptr = Box::into_raw(Box::new(42));");
    println!("    unsafe {{}}");
    println!("        drop(Box::from_raw(ptr));  // 第一次释放");
    println!("        drop(Box::from_raw(ptr));  // 双重释放！");
    println!("    }}");
    println!("    ```");
}

/// 使用后释放预防
fn use_after_free_prevention() {
    println!("  Rust 如何防止使用后释放:");
    
    // 作用域确保安全
    let _reference: Option<&String> = None;
    {
        let value = String::from("Hello");
        // _reference = Some(&value); // 编译错误：借用的值生命周期不够长
    }
    
    // 正确的做法：使用拥有所有权的类型
    let owned_value;
    {
        let value = String::from("Hello");
        owned_value = value; // 移动所有权
    }
    println!("  拥有的值: {}", owned_value);
    
    // 智能指针的安全性
    let rc_value;
    {
        let inner_rc = Rc::new(String::from("Shared"));
        rc_value = Rc::clone(&inner_rc);
    } // inner_rc 被销毁，但数据仍然存在
    
    println!("  共享值: {}", rc_value);
    println!("  引用计数: {}", Rc::strong_count(&rc_value));
    
    println!("  ✅ 所有权系统防止使用已释放的内存");
}

/// 内存泄漏预防
fn memory_leak_prevention() {
    println!("  Rust 如何预防内存泄漏:");
    
    // RAII 自动清理
    {
        let _vec = vec![1, 2, 3, 4, 5];
        let _string = String::from("This will be cleaned up");
        let _box = Box::new([0; 1000]);
    } // 所有资源在这里自动释放
    
    println!("  ✅ RAII 确保资源自动释放");
    
    // 循环引用的解决
    println!("\n  🔄 循环引用解决方案:");
    
    #[derive(Debug)]
    struct Parent {
        children: RefCell<Vec<Rc<Child>>>,
    }
    
    #[derive(Debug)]
    struct Child {
        parent: Weak<Parent>, // 使用 Weak 打破循环
    }
    
    let parent = Rc::new(Parent {
        children: RefCell::new(Vec::new()),
    });
    
    let child = Rc::new(Child {
        parent: Rc::downgrade(&parent),
    });
    
    parent.children.borrow_mut().push(child);
    
    println!("  父节点引用计数: {}", Rc::strong_count(&parent));
    
    // 当 parent 离开作用域时，所有内存都会被正确释放
    println!("  ✅ Weak 引用打破循环，防止内存泄漏");
    
    // 手动内存泄漏（故意的）
    println!("\n  🚨 故意的内存泄漏:");
    let leaked_box = Box::new(42);
    let leaked_ptr = Box::into_raw(leaked_box);
    println!("  泄漏的指针: {:p}", leaked_ptr);
    
    // 可以通过 Box::from_raw 恢复并释放
    unsafe {
        let recovered_box = Box::from_raw(leaked_ptr);
        println!("  恢复的值: {}", recovered_box);
    } // 在这里正确释放
    
    println!("  ✅ 即使手动管理内存，也可以安全恢复");
}

/// 并发安全分析
fn concurrency_safety_analysis() {
    println!("\n🧵 5. 并发安全分析");
    println!("{}", "-".repeat(40));
    
    // Send 和 Sync trait
    println!("\n📤 Send 和 Sync trait:");
    send_sync_analysis();
    
    // 数据竞争预防
    println!("\n🏃 数据竞争预防:");
    data_race_prevention();
    
    // 死锁预防
    println!("\n🔒 死锁预防:");
    deadlock_prevention();
}

/// Send 和 Sync 分析
fn send_sync_analysis() {
    use std::marker::{Send, Sync};
    
    // 检查类型的 Send 和 Sync 实现
    fn check_send_sync<T: Send + Sync>(_: T) {
        println!("  类型实现了 Send + Sync");
    }
    
    fn check_send<T: Send>(_: T) {
        println!("  类型实现了 Send");
    }
    
    // 基础类型
    check_send_sync(42i32);
    check_send_sync(String::from("hello"));
    check_send_sync(Vec::<i32>::new());
    
    // 智能指针
    check_send_sync(Box::new(42));
    check_send_sync(Arc::new(42));
    
    // Rc 不是 Send + Sync
    let rc = Rc::new(42);
    // check_send_sync(rc); // 编译错误
    println!("  Rc 不实现 Send + Sync (单线程专用)");
    
    // RefCell 不是 Sync
    let refcell = RefCell::new(42);
    check_send(refcell);
    // check_send_sync(RefCell::new(42)); // 编译错误
    println!("  RefCell 实现 Send 但不实现 Sync");
    
    // Mutex 是 Send + Sync
    check_send_sync(Mutex::new(42));
    println!("  Mutex 实现 Send + Sync (线程安全)");
    
    println!("  ✅ 类型系统确保线程安全");
}

/// 数据竞争预防
fn data_race_prevention() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    // 原子操作防止数据竞争
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for _ in 0..3 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("  原子计数器最终值: {}", counter.load(Ordering::Relaxed));
    
    // Mutex 保护共享数据
    let shared_data = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];
    
    for i in 0..3 {
        let data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            let mut data = data_clone.lock().unwrap();
            data.push(i);
            thread::sleep(Duration::from_millis(1)); // 模拟工作
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_data = shared_data.lock().unwrap();
    println!("  Mutex 保护的数据: {:?}", *final_data);
    
    println!("  ✅ 同步原语防止数据竞争");
}

/// 死锁预防
fn deadlock_prevention() {
    println!("  死锁预防策略:");
    
    // 策略1：锁排序
    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));
    
    let m1_clone = Arc::clone(&mutex1);
    let m2_clone = Arc::clone(&mutex2);
    
    // 线程1：按顺序获取锁
    let handle1 = thread::spawn(move || {
        let _guard1 = m1_clone.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let _guard2 = m2_clone.lock().unwrap();
        println!("  线程1: 获取了两个锁");
    });
    
    let m1_clone = Arc::clone(&mutex1);
    let m2_clone = Arc::clone(&mutex2);
    
    // 线程2：相同顺序获取锁
    let handle2 = thread::spawn(move || {
        let _guard1 = m1_clone.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let _guard2 = m2_clone.lock().unwrap();
        println!("  线程2: 获取了两个锁");
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    
    println!("  ✅ 锁排序防止死锁");
    
    // 策略2：超时锁
    println!("\n  🕐 超时锁策略:");
    let mutex = Arc::new(Mutex::new(0));
    let mutex_clone = Arc::clone(&mutex);
    
    let handle = thread::spawn(move || {
        let _guard = mutex_clone.lock().unwrap();
        thread::sleep(Duration::from_millis(100)); // 持有锁一段时间
    });
    
    thread::sleep(Duration::from_millis(10)); // 确保另一个线程先获取锁
    
    // 尝试获取锁（在实际应用中可以使用 try_lock 或超时机制）
    match mutex.try_lock() {
        Ok(_guard) => println!("  成功获取锁"),
        Err(_) => println!("  锁被占用，避免阻塞"),
    }
    
    handle.join().unwrap();
    
    println!("  ✅ 非阻塞锁操作防止死锁");
}

/// 生命周期安全分析
fn lifetime_safety_analysis() {
    println!("\n⏰ 6. 生命周期安全分析");
    println!("{}", "-".repeat(40));
    
    // 生命周期省略
    println!("\n📝 生命周期省略规则:");
    lifetime_elision_rules();
    
    // 复杂生命周期
    println!("\n🔗 复杂生命周期关系:");
    complex_lifetime_relationships();
    
    // 静态生命周期
    println!("\n♾️ 静态生命周期:");
    static_lifetime_analysis();
}

/// 生命周期省略规则
fn lifetime_elision_rules() {
    // 规则1：每个引用参数都有自己的生命周期
    fn rule1(s: &str) -> usize { // 等价于 fn rule1<'a>(s: &'a str) -> usize
        s.len()
    }
    
    // 规则2：如果只有一个输入生命周期，它被赋给所有输出生命周期
    fn rule2(s: &str) -> &str { // 等价于 fn rule2<'a>(s: &'a str) -> &'a str
        s
    }
    
    // 规则3：如果有多个输入生命周期，但其中一个是 &self 或 &mut self，
    // self 的生命周期被赋给所有输出生命周期
    struct LifetimeExample {
        data: String,
    }
    
    impl LifetimeExample {
        fn get_data(&self) -> &str { // 等价于 fn get_data<'a>(&'a self) -> &'a str
            &self.data
        }
    }
    
    let example = LifetimeExample {
        data: String::from("test data"),
    };
    
    println!("  规则1 - 字符串长度: {}", rule1("hello"));
    println!("  规则2 - 返回输入: {}", rule2("world"));
    println!("  规则3 - 方法返回: {}", example.get_data());
    
    println!("  ✅ 生命周期省略简化了常见模式");
}

/// 复杂生命周期关系
fn complex_lifetime_relationships() {
    // 多个生命周期参数
    fn longest_with_context<'a, 'b>(
        x: &'a str,
        y: &'a str,
        context: &'b str,
    ) -> (&'a str, &'b str) {
        let result = if x.len() > y.len() { x } else { y };
        (result, context)
    }
    
    let string1 = String::from("long string");
    let string2 = String::from("short");
    let context = String::from("context");
    
    let (longest, ctx) = longest_with_context(&string1, &string2, &context);
    println!("  最长字符串: {}, 上下文: {}", longest, ctx);
    
    // 生命周期约束
    fn with_lifetime_bound<'a, T>(x: &'a T) -> &'a T
    where
        T: std::fmt::Display + 'a,
    {
        println!("  显示值: {}", x);
        x
    }
    
    let number = 42;
    let result = with_lifetime_bound(&number);
    println!("  约束结果: {}", result);
    
    println!("  ✅ 复杂生命周期关系确保引用安全");
}

/// 静态生命周期分析
fn static_lifetime_analysis() {
    // 字符串字面量有 'static 生命周期
    let static_str: &'static str = "This lives for the entire program";
    println!("  静态字符串: {}", static_str);
    
    // 静态变量
    static GLOBAL_DATA: &str = "Global static data";
    println!("  全局静态数据: {}", GLOBAL_DATA);
    
    // 泄漏内存获得 'static 引用
    let leaked_string = Box::leak(Box::new(String::from("Leaked string")));
    println!("  泄漏的字符串: {}", leaked_string);
    
    // 函数返回 'static 引用
    fn get_static_str() -> &'static str {
        "Static from function"
    }
    
    println!("  函数返回静态: {}", get_static_str());
    
    // 'static 约束
    fn needs_static<T: 'static>(x: T) -> T {
        x
    }
    
    let owned_string = String::from("Owned");
    let result = needs_static(owned_string);
    println!("  静态约束结果: {}", result);
    
    println!("  ✅ 'static 生命周期确保数据在程序整个生命周期内有效");
}

/// 安全编程最佳实践
fn safety_best_practices() {
    println!("\n📋 7. 安全编程最佳实践");
    println!("{}", "-".repeat(40));
    
    println!("\n🎯 核心原则:");
    println!("  1. 优先使用安全抽象");
    println!("     • 引用 > 智能指针 > 裸指针");
    println!("     • Vec/String > 手动内存管理");
    println!("     • Mutex/RwLock > 裸指针 + 同步");
    
    println!("\n  2. 最小化 unsafe 代码");
    println!("     • 将 unsafe 封装在安全接口中");
    println!("     • 详细文档化安全不变量");
    println!("     • 使用 #[deny(unsafe_code)] 限制使用");
    
    println!("\n  3. 利用类型系统");
    println!("     • 使用 newtype 模式增强类型安全");
    println!("     • 利用生命周期参数表达约束");
    println!("     • 使用 PhantomData 携带类型信息");
    
    println!("\n  4. 错误处理");
    println!("     • 使用 Result 而不是 panic");
    println!("     • 优雅处理资源清理");
    println!("     • 使用 ? 操作符简化错误传播");
    
    println!("\n  5. 测试和验证");
    println!("     • 编写全面的单元测试");
    println!("     • 使用 Miri 检测未定义行为");
    println!("     • 使用 Clippy 检查常见问题");
    println!("     • 使用 AddressSanitizer 检测内存错误");
    
    // 示例：安全的 API 设计
    println!("\n🔧 安全 API 设计示例:");
    
    // 使用 newtype 增强类型安全
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct UserId(u64);
    
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct PostId(u64);
    
    // 类型安全的函数
    fn get_user_posts(user_id: UserId) -> Vec<PostId> {
        // 模拟数据库查询
        vec![PostId(1), PostId(2), PostId(3)]
    }
    
    let user = UserId(123);
    let posts = get_user_posts(user);
    println!("  用户 {:?} 的帖子: {:?}", user, posts);
    
    // 这会导致编译错误，防止类型混淆
    // let posts = get_user_posts(PostId(456)); // 编译错误
    
    // 使用 Result 进行错误处理
    fn safe_divide(a: f64, b: f64) -> Result<f64, &'static str> {
        if b == 0.0 {
            Err("除零错误")
        } else {
            Ok(a / b)
        }
    }
    
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("  除法结果: {}", result),
        Err(error) => println!("  除法错误: {}", error),
    }
    
    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("  除法结果: {}", result),
        Err(error) => println!("  除法错误: {}", error),
    }
    
    println!("\n🎉 总结:");
    println!("  • Rust 的安全性来自于编译时检查和运行时保证");
    println!("  • 引用提供编译时安全，智能指针提供运行时安全");
    println!("  • 裸指针需要程序员确保安全，应谨慎使用");
    println!("  • 遵循最佳实践可以最大化代码安全性");
    println!("  • 类型系统是你的朋友，充分利用它！");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_reference_safety() {
        let data = vec![1, 2, 3];
        let reference = &data;
        
        // 引用应该指向有效数据
        assert_eq!(reference.len(), 3);
        assert_eq!(reference[1], 2);
    }
    
    #[test]
    fn test_smart_pointer_safety() {
        let rc_data = Rc::new(vec![1, 2, 3]);
        let rc_clone = Rc::clone(&rc_data);
        
        // 引用计数应该正确
        assert_eq!(Rc::strong_count(&rc_data), 2);
        
        drop(rc_clone);
        assert_eq!(Rc::strong_count(&rc_data), 1);
    }
    
    #[test]
    fn test_interior_mutability() {
        let cell = RefCell::new(42);
        
        // 应该能够安全地借用和修改
        {
            let value = cell.borrow();
            assert_eq!(*value, 42);
        }
        
        {
            let mut value = cell.borrow_mut();
            *value = 100;
        }
        
        assert_eq!(*cell.borrow(), 100);
    }
    
    #[test]
    #[should_panic(expected = "already borrowed")]
    fn test_refcell_panic() {
        let cell = RefCell::new(42);
        let _borrow1 = cell.borrow();
        let _borrow2 = cell.borrow_mut(); // 这应该导致 panic
    }
    
    #[test]
    fn test_weak_reference() {
        let strong = Rc::new(42);
        let weak = Rc::downgrade(&strong);
        
        // 弱引用应该能够升级
        assert!(weak.upgrade().is_some());
        
        drop(strong);
        
        // 强引用被释放后，弱引用应该无法升级
        assert!(weak.upgrade().is_none());
    }
}