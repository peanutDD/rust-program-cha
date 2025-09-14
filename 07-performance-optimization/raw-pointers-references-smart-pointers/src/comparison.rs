//! # 裸指针、引用和智能指针对比分析
//!
//! 本模块提供了裸指针、引用和智能指针的全面对比分析，
//! 包括安全性、性能、使用场景、内存管理等多个维度的比较。

use std::ptr;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::sync::Mutex;
use std::time::Instant;
use std::thread;
use std::borrow::Cow;

/// 运行所有对比分析示例
pub fn run_all_comparisons() {
    println!("\n🔍 裸指针、引用和智能指针全面对比分析");
    println!("{}\n", "=".repeat(60));
    
    // 基础特性对比
    basic_characteristics_comparison();
    
    // 安全性对比
    safety_comparison();
    
    // 性能对比
    performance_comparison();
    
    // 内存管理对比
    memory_management_comparison();
    
    // 使用场景对比
    use_case_comparison();
    
    // 语法和易用性对比
    syntax_usability_comparison();
    
    // 编译时检查对比
    compile_time_checks_comparison();
    
    // 运行时开销对比
    runtime_overhead_comparison();
    
    // 线程安全性对比
    thread_safety_comparison();
    
    // 选择指南
    selection_guide();
}

/// 基础特性对比
fn basic_characteristics_comparison() {
    println!("📊 1. 基础特性对比");
    println!("{}", "-".repeat(40));
    
    println!("\n📋 特性对比表:");
    println!("┌─────────────────┬─────────────┬─────────────┬─────────────┐");
    println!("│ 特性            │ 裸指针      │ 引用        │ 智能指针    │");
    println!("├─────────────────┼─────────────┼─────────────┼─────────────┤");
    println!("│ 内存安全        │ ❌ 不安全   │ ✅ 安全     │ ✅ 安全     │");
    println!("│ 空指针检查      │ ❌ 手动     │ ✅ 自动     │ ✅ 自动     │");
    println!("│ 生命周期管理    │ ❌ 手动     │ ✅ 编译时   │ ✅ 运行时   │");
    println!("│ 所有权语义      │ ❌ 无       │ ✅ 借用     │ ✅ 拥有     │");
    println!("│ 运行时开销      │ ✅ 零开销   │ ✅ 零开销   │ ⚠️ 有开销   │");
    println!("│ 编译时检查      │ ❌ 最少     │ ✅ 严格     │ ✅ 部分     │");
    println!("│ 灵活性          │ ✅ 最高     │ ⚠️ 受限    │ ⚠️ 中等     │");
    println!("│ 易用性          │ ❌ 困难     │ ✅ 简单     │ ✅ 简单     │");
    println!("└─────────────────┴─────────────┴─────────────┴─────────────┘");
    
    // 代码示例对比
    println!("\n💻 代码示例对比:");
    
    // 裸指针示例
    println!("\n🔸 裸指针 - 最大灵活性，最少安全保障:");
    unsafe {
        let x = 42;
        let raw_ptr = &x as *const i32;
        println!("  裸指针值: {}", *raw_ptr);
        
        // 可以进行指针算术
        let offset_ptr = raw_ptr.offset(0);
        println!("  偏移后值: {}", *offset_ptr);
    }
    
    // 引用示例
    println!("\n🔸 引用 - 编译时安全，零运行时开销:");
    let y = 42;
    let reference = &y;
    println!("  引用值: {}", *reference);
    // 编译器确保引用有效性
    
    // 智能指针示例
    println!("\n🔸 智能指针 - 运行时安全，自动内存管理:");
    let smart_ptr = Box::new(42);
    println!("  智能指针值: {}", *smart_ptr);
    // 自动释放内存
    
    println!("\n✅ 基础特性对比完成");
}

/// 安全性对比
fn safety_comparison() {
    println!("\n🛡️ 2. 安全性对比");
    println!("{}", "-".repeat(40));
    
    // 裸指针安全性问题
    println!("\n🔴 裸指针安全性问题:");
    demonstrate_raw_pointer_safety_issues();
    
    // 引用安全性保障
    println!("\n🟢 引用安全性保障:");
    demonstrate_reference_safety_guarantees();
    
    // 智能指针安全性特性
    println!("\n🟡 智能指针安全性特性:");
    demonstrate_smart_pointer_safety_features();
}

/// 演示裸指针安全性问题
fn demonstrate_raw_pointer_safety_issues() {
    println!("  ⚠️ 常见安全问题:");
    
    // 1. 悬垂指针
    println!("  1. 悬垂指针风险:");
    let dangling_ptr: *const i32;
    {
        let x = 42;
        dangling_ptr = &x as *const i32;
        // x 在这里被销毁
    }
    println!("     悬垂指针已创建，使用它会导致未定义行为");
    
    // 2. 空指针解引用
    println!("  2. 空指针解引用风险:");
    let null_ptr: *const i32 = ptr::null();
    println!("     空指针: {:?}", null_ptr);
    println!("     解引用空指针会导致程序崩溃");
    
    // 3. 缓冲区溢出
    println!("  3. 缓冲区溢出风险:");
    let arr = [1, 2, 3, 4, 5];
    let ptr = arr.as_ptr();
    unsafe {
        // 这可能访问无效内存
        println!("     数组边界内: {}", *ptr.offset(2));
        println!("     可能的缓冲区溢出: 编译器不会检查边界");
    }
    
    // 4. 数据竞争
    println!("  4. 数据竞争风险:");
    println!("     多线程环境下，裸指针无法防止数据竞争");
    println!("     需要手动同步机制");
}

/// 演示引用安全性保障
fn demonstrate_reference_safety_guarantees() {
    println!("  ✅ 安全性保障:");
    
    // 1. 借用检查器
    println!("  1. 借用检查器保障:");
    let x = 42;
    let r1 = &x;
    let r2 = &x;
    println!("     多个不可变引用: {} {}", r1, r2);
    println!("     编译器确保引用有效性");
    
    // 2. 生命周期检查
    println!("  2. 生命周期检查:");
    {
        let y = 100;
        let r = &y;
        println!("     引用在作用域内: {}", r);
    }
    println!("     编译器防止悬垂引用");
    
    // 3. 可变性控制
    println!("  3. 可变性控制:");
    let mut z = 200;
    {
        let r_mut = &mut z;
        *r_mut += 1;
        println!("     可变引用独占访问: {}", r_mut);
    }
    println!("     编译器防止数据竞争");
    
    // 4. 空引用不存在
    println!("  4. 空引用不存在:");
    println!("     Rust 引用永远不会为空");
    println!("     消除了空指针解引用的风险");
}

/// 演示智能指针安全性特性
fn demonstrate_smart_pointer_safety_features() {
    println!("  🔒 安全性特性:");
    
    // 1. 自动内存管理
    println!("  1. 自动内存管理:");
    {
        let boxed = Box::new(String::from("自动管理"));
        println!("     Box 自动释放: {}", boxed);
    } // 内存在这里自动释放
    println!("     无需手动释放，防止内存泄漏");
    
    // 2. 引用计数安全
    println!("  2. 引用计数安全:");
    let rc_data = Rc::new(vec![1, 2, 3]);
    let rc_clone = Rc::clone(&rc_data);
    println!("     Rc 引用计数: {}", Rc::strong_count(&rc_data));
    drop(rc_clone);
    println!("     自动引用计数管理，防止过早释放");
    
    // 3. 线程安全
    println!("  3. 线程安全:");
    let arc_data = Arc::new(Mutex::new(0));
    let arc_clone = Arc::clone(&arc_data);
    
    let handle = thread::spawn(move || {
        let mut data = arc_clone.lock().unwrap();
        *data += 1;
    });
    
    handle.join().unwrap();
    let final_value = arc_data.lock().unwrap();
    println!("     Arc<Mutex<T>> 提供线程安全: {}", *final_value);
    
    // 4. 内部可变性安全
    println!("  4. 内部可变性安全:");
    let cell_data = RefCell::new(42);
    {
        let borrowed = cell_data.borrow();
        println!("     RefCell 运行时借用检查: {}", *borrowed);
    }
    {
        let mut borrowed_mut = cell_data.borrow_mut();
        *borrowed_mut += 1;
        println!("     安全的内部可变性: {}", *borrowed_mut);
    }
}

/// 性能对比
fn performance_comparison() {
    println!("\n⚡ 3. 性能对比");
    println!("{}", "-".repeat(40));
    
    const ITERATIONS: usize = 1_000_000;
    
    // 裸指针性能测试
    println!("\n🔸 裸指针性能测试:");
    let start = Instant::now();
    unsafe {
        let data = vec![1, 2, 3, 4, 5];
        let ptr = data.as_ptr();
        let mut sum = 0;
        for i in 0..ITERATIONS {
            sum += *ptr.offset((i % 5) as isize);
        }
        println!("  结果: {}, 耗时: {:?}", sum, start.elapsed());
    }
    
    // 引用性能测试
    println!("\n🔸 引用性能测试:");
    let start = Instant::now();
    let data = vec![1, 2, 3, 4, 5];
    let mut sum = 0;
    for i in 0..ITERATIONS {
        sum += data[i % 5];
    }
    println!("  结果: {}, 耗时: {:?}", sum, start.elapsed());
    
    // 智能指针性能测试
    println!("\n🔸 智能指针性能测试:");
    let start = Instant::now();
    let data = Rc::new(vec![1, 2, 3, 4, 5]);
    let mut sum = 0;
    for i in 0..ITERATIONS {
        sum += data[i % 5];
    }
    println!("  结果: {}, 耗时: {:?}", sum, start.elapsed());
    
    println!("\n📊 性能分析:");
    println!("  • 裸指针: 最快，零抽象开销");
    println!("  • 引用: 与裸指针相当，编译器优化");
    println!("  • 智能指针: 略慢，有额外的引用计数开销");
}

/// 内存管理对比
fn memory_management_comparison() {
    println!("\n🧠 4. 内存管理对比");
    println!("{}", "-".repeat(40));
    
    // 裸指针内存管理
    println!("\n🔸 裸指针内存管理:");
    demonstrate_raw_pointer_memory_management();
    
    // 引用内存管理
    println!("\n🔸 引用内存管理:");
    demonstrate_reference_memory_management();
    
    // 智能指针内存管理
    println!("\n🔸 智能指针内存管理:");
    demonstrate_smart_pointer_memory_management();
}

/// 演示裸指针内存管理
fn demonstrate_raw_pointer_memory_management() {
    println!("  📝 特点:");
    println!("  • 完全手动管理");
    println!("  • 需要显式分配和释放");
    println!("  • 容易出现内存泄漏和双重释放");
    
    println!("\n  💻 示例:");
    unsafe {
        // 模拟手动内存管理
        let layout = std::alloc::Layout::new::<i32>();
        let ptr = std::alloc::alloc(layout) as *mut i32;
        
        if !ptr.is_null() {
            *ptr = 42;
            println!("    分配并设置值: {}", *ptr);
            
            // 必须手动释放
            std::alloc::dealloc(ptr as *mut u8, layout);
            println!("    手动释放内存");
        }
    }
    
    println!("  ⚠️ 风险: 忘记释放导致内存泄漏，重复释放导致崩溃");
}

/// 演示引用内存管理
fn demonstrate_reference_memory_management() {
    println!("  📝 特点:");
    println!("  • 借用现有内存");
    println!("  • 不拥有内存，不负责释放");
    println!("  • 编译时生命周期检查");
    
    println!("\n  💻 示例:");
    {
        let data = String::from("Hello, World!");
        let reference = &data;
        println!("    借用数据: {}", reference);
        
        // 引用不能超出数据的生命周期
        let slice = &data[0..5];
        println!("    借用切片: {}", slice);
    } // data 在这里自动释放，引用也失效
    
    println!("  ✅ 优势: 零开销抽象，编译时安全保证");
}

/// 演示智能指针内存管理
fn demonstrate_smart_pointer_memory_management() {
    println!("  📝 特点:");
    println!("  • 自动内存管理");
    println!("  • RAII 模式");
    println!("  • 引用计数或独占所有权");
    
    println!("\n  💻 示例:");
    
    // Box - 独占所有权
    {
        let boxed = Box::new(String::from("Box 管理"));
        println!("    Box 独占所有权: {}", boxed);
    } // 自动释放
    
    // Rc - 共享所有权
    {
        let rc_data = Rc::new(String::from("Rc 管理"));
        let rc_clone = Rc::clone(&rc_data);
        println!("    Rc 共享所有权: {} (引用计数: {})", 
                 rc_data, Rc::strong_count(&rc_data));
        drop(rc_clone);
        println!("    引用计数减少: {}", Rc::strong_count(&rc_data));
    } // 引用计数为0时自动释放
    
    println!("  ✅ 优势: 自动化内存管理，防止内存泄漏");
}

/// 使用场景对比
fn use_case_comparison() {
    println!("\n🎯 5. 使用场景对比");
    println!("{}", "-".repeat(40));
    
    println!("\n📋 适用场景分析:");
    
    // 裸指针使用场景
    println!("\n🔸 裸指针适用场景:");
    println!("  ✅ 系统级编程 (操作系统、驱动程序)");
    println!("  ✅ FFI (与 C/C++ 代码交互)");
    println!("  ✅ 性能关键代码 (零开销要求)");
    println!("  ✅ 底层数据结构实现");
    println!("  ✅ 内存映射和硬件访问");
    println!("  ❌ 一般应用程序开发");
    println!("  ❌ 初学者项目");
    
    // 引用使用场景
    println!("\n🔸 引用适用场景:");
    println!("  ✅ 函数参数传递");
    println!("  ✅ 数据借用和访问");
    println!("  ✅ 迭代器和集合操作");
    println!("  ✅ 字符串和切片操作");
    println!("  ✅ 一般应用程序开发");
    println!("  ✅ 性能敏感的安全代码");
    println!("  ❌ 需要所有权转移的场景");
    println!("  ❌ 复杂的生命周期管理");
    
    // 智能指针使用场景
    println!("\n🔸 智能指针适用场景:");
    println!("  ✅ 堆内存分配");
    println!("  ✅ 共享数据结构");
    println!("  ✅ 递归数据结构");
    println!("  ✅ 多线程编程");
    println!("  ✅ 复杂的所有权模式");
    println!("  ✅ 内部可变性需求");
    println!("  ❌ 简单的栈分配数据");
    println!("  ❌ 性能极度敏感的代码");
    
    // 实际场景示例
    println!("\n💡 实际场景示例:");
    demonstrate_real_world_scenarios();
}

/// 演示真实世界场景
fn demonstrate_real_world_scenarios() {
    // 场景1: 配置解析 - 引用最适合
    println!("\n  📄 场景1: 配置文件解析");
    fn parse_config(content: &str) -> Vec<&str> {
        content.lines().collect()
    }
    
    let config = "host=localhost\nport=8080\ndebug=true";
    let lines = parse_config(config);
    println!("    使用引用解析配置: {:?}", lines);
    println!("    ✅ 引用最适合: 零拷贝，安全借用");
    
    // 场景2: 数据共享 - 智能指针最适合
    println!("\n  🔗 场景2: 多组件数据共享");
    let shared_config = Rc::new(RefCell::new(std::collections::HashMap::new()));
    shared_config.borrow_mut().insert("theme".to_string(), "dark".to_string());
    
    let ui_component = Rc::clone(&shared_config);
    let network_component = Rc::clone(&shared_config);
    
    println!("    UI 组件访问配置: {:?}", ui_component.borrow().get("theme"));
    println!("    网络组件访问配置: {:?}", network_component.borrow().get("theme"));
    println!("    ✅ 智能指针最适合: 安全的共享可变状态");
    
    // 场景3: 系统调用 - 裸指针最适合
    println!("\n  ⚙️ 场景3: 系统级内存操作");
    unsafe {
        let mut buffer = vec![0u8; 1024];
        let ptr = buffer.as_mut_ptr();
        
        // 模拟系统调用
        ptr::write_bytes(ptr, 0xFF, 10);
        println!("    系统调用填充缓冲区: {:?}", &buffer[0..10]);
        println!("    ✅ 裸指针最适合: 直接内存操作，最大控制权");
    }
}

/// 语法和易用性对比
fn syntax_usability_comparison() {
    println!("\n📝 6. 语法和易用性对比");
    println!("{}", "-".repeat(40));
    
    // 创建和初始化
    println!("\n🔧 创建和初始化:");
    
    // 裸指针
    println!("\n  🔸 裸指针:");
    let x = 42;
    let raw_ptr = &x as *const i32;
    println!("    let raw_ptr = &x as *const i32;");
    println!("    复杂度: ⭐⭐⭐ (需要类型转换)");
    
    // 引用
    println!("\n  🔸 引用:");
    let reference = &x;
    println!("    let reference = &x;");
    println!("    复杂度: ⭐ (最简单)");
    
    // 智能指针
    println!("\n  🔸 智能指针:");
    let smart_ptr = Box::new(42);
    println!("    let smart_ptr = Box::new(42);");
    println!("    复杂度: ⭐⭐ (需要构造函数)");
    
    // 访问数据
    println!("\n🔍 访问数据:");
    
    println!("\n  🔸 裸指针:");
    unsafe {
        println!("    unsafe {{ *raw_ptr }}");
        println!("    值: {}", *raw_ptr);
    }
    println!("    复杂度: ⭐⭐⭐ (需要 unsafe 块)");
    
    println!("\n  🔸 引用:");
    println!("    *reference");
    println!("    值: {}", *reference);
    println!("    复杂度: ⭐ (直接解引用)");
    
    println!("\n  🔸 智能指针:");
    println!("    *smart_ptr");
    println!("    值: {}", *smart_ptr);
    println!("    复杂度: ⭐ (自动解引用)");
    
    // 错误处理
    println!("\n❌ 错误处理:");
    
    println!("\n  🔸 裸指针:");
    println!("    手动检查空指针");
    println!("    if !ptr.is_null() {{ ... }}");
    println!("    复杂度: ⭐⭐⭐ (完全手动)");
    
    println!("\n  🔸 引用:");
    println!("    编译时保证非空");
    println!("    无需运行时检查");
    println!("    复杂度: ⭐ (自动保证)");
    
    println!("\n  🔸 智能指针:");
    println!("    自动处理，Option<T> 用于可能为空的情况");
    println!("    if let Some(value) = weak_ref.upgrade() {{ ... }}");
    println!("    复杂度: ⭐⭐ (部分自动)");
}

/// 编译时检查对比
fn compile_time_checks_comparison() {
    println!("\n🔍 7. 编译时检查对比");
    println!("{}", "-".repeat(40));
    
    println!("\n📊 编译时检查能力:");
    println!("┌─────────────────┬─────────────┬─────────────┬─────────────┐");
    println!("│ 检查项目        │ 裸指针      │ 引用        │ 智能指针    │");
    println!("├─────────────────┼─────────────┼─────────────┼─────────────┤");
    println!("│ 空指针检查      │ ❌ 无       │ ✅ 完全     │ ✅ 部分     │");
    println!("│ 生命周期检查    │ ❌ 无       │ ✅ 完全     │ ⚠️ 部分     │");
    println!("│ 借用规则检查    │ ❌ 无       │ ✅ 完全     │ ⚠️ 运行时   │");
    println!("│ 类型安全检查    │ ⚠️ 基础     │ ✅ 完全     │ ✅ 完全     │");
    println!("│ 内存安全检查    │ ❌ 无       │ ✅ 完全     │ ✅ 大部分   │");
    println!("│ 数据竞争检查    │ ❌ 无       │ ✅ 完全     │ ✅ 部分     │");
    println!("└─────────────────┴─────────────┴─────────────┴─────────────┘");
    
    println!("\n💡 检查示例:");
    
    // 生命周期检查示例
    println!("\n  🔸 生命周期检查:");
    println!("    // 引用 - 编译时错误");
    println!("    // let r;");
    println!("    // {{}}");
    println!("    //     let x = 5;");
    println!("    //     r = &x;  // 错误: x 生命周期不够长");
    println!("    // }}");
    println!("    // println!(\"{{}}\" , r);");
    
    println!("\n    // 裸指针 - 编译通过，运行时未定义行为");
    println!("    // let r: *const i32;");
    println!("    // {{}}");
    println!("    //     let x = 5;");
    println!("    //     r = &x as *const i32;  // 编译通过");
    println!("    // }}");
    println!("    // unsafe {{{{ println!(\"{{}}\" , *r); }}}}  // 未定义行为");
    
    // 借用规则检查示例
    println!("\n  🔸 借用规则检查:");
    println!("    // 引用 - 编译时错误");
    println!("    // let mut x = 5;");
    println!("    // let r1 = &x;");
    println!("    // let r2 = &mut x;  // 错误: 不能同时有不可变和可变引用");
    
    println!("\n    // RefCell - 运行时检查");
    let cell = RefCell::new(5);
    let _borrow1 = cell.borrow();
    // let _borrow2 = cell.borrow_mut(); // 运行时 panic
    println!("    RefCell 在运行时检查借用规则");
}

/// 运行时开销对比
fn runtime_overhead_comparison() {
    println!("\n⚡ 8. 运行时开销对比");
    println!("{}", "-".repeat(40));
    
    println!("\n📊 开销分析:");
    
    // 内存开销
    println!("\n🧠 内存开销:");
    println!("  🔸 裸指针: {} 字节 (指针大小)", std::mem::size_of::<*const i32>());
    println!("  🔸 引用: {} 字节 (与裸指针相同)", std::mem::size_of::<&i32>());
    println!("  🔸 Box<T>: {} 字节 (指针大小)", std::mem::size_of::<Box<i32>>());
    println!("  🔸 Rc<T>: {} 字节 (指针 + 引用计数)", std::mem::size_of::<Rc<i32>>());
    println!("  🔸 Arc<T>: {} 字节 (指针 + 原子引用计数)", std::mem::size_of::<Arc<i32>>());
    
    // CPU 开销
    println!("\n⚙️ CPU 开销:");
    println!("  🔸 裸指针:");
    println!("    • 解引用: 0 开销");
    println!("    • 算术运算: 0 开销");
    println!("    • 比较: 0 开销");
    
    println!("  🔸 引用:");
    println!("    • 解引用: 0 开销 (编译器优化)");
    println!("    • 借用检查: 0 开销 (编译时)");
    println!("    • 生命周期检查: 0 开销 (编译时)");
    
    println!("  🔸 智能指针:");
    println!("    • Box<T> 解引用: 0 开销");
    println!("    • Rc<T> 克隆: 原子操作开销");
    println!("    • Arc<T> 克隆: 原子操作开销 (更高)");
    println!("    • RefCell<T> 借用: 运行时检查开销");
    
    // 开销测试
    println!("\n🧪 开销测试:");
    test_overhead_comparison();
}

/// 测试开销对比
fn test_overhead_comparison() {
    const ITERATIONS: usize = 10_000_000;
    
    // 裸指针开销测试
    let data = 42;
    let raw_ptr = &data as *const i32;
    
    let start = Instant::now();
    unsafe {
        for _ in 0..ITERATIONS {
            let _ = *raw_ptr;
        }
    }
    let raw_time = start.elapsed();
    println!("  裸指针解引用 {} 次: {:?}", ITERATIONS, raw_time);
    
    // 引用开销测试
    let reference = &data;
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = *reference;
    }
    let ref_time = start.elapsed();
    println!("  引用解引用 {} 次: {:?}", ITERATIONS, ref_time);
    
    // Box 开销测试
    let boxed = Box::new(42);
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = *boxed;
    }
    let box_time = start.elapsed();
    println!("  Box 解引用 {} 次: {:?}", ITERATIONS, box_time);
    
    // Rc 克隆开销测试
    let rc_data = Rc::new(42);
    let start = Instant::now();
    for _ in 0..ITERATIONS / 1000 { // 减少迭代次数，因为克隆开销较大
        let _clone = Rc::clone(&rc_data);
    }
    let rc_time = start.elapsed();
    println!("  Rc 克隆 {} 次: {:?}", ITERATIONS / 1000, rc_time);
}

/// 线程安全性对比
fn thread_safety_comparison() {
    println!("\n🧵 9. 线程安全性对比");
    println!("{}", "-".repeat(40));
    
    println!("\n🔒 线程安全性分析:");
    
    // 裸指针线程安全性
    println!("\n  🔸 裸指针:");
    println!("    ❌ 无线程安全保证");
    println!("    ❌ 可能导致数据竞争");
    println!("    ❌ 需要手动同步");
    println!("    ⚠️ 适用于单线程或专家级同步");
    
    // 引用线程安全性
    println!("\n  🔸 引用:");
    println!("    ✅ 编译时防止数据竞争");
    println!("    ✅ 借用检查器保证安全");
    println!("    ❌ 不能在线程间传递可变引用");
    println!("    ✅ 不可变引用可以安全共享");
    
    // 智能指针线程安全性
    println!("\n  🔸 智能指针:");
    println!("    📦 Box<T>: 独占所有权，可以安全移动到其他线程");
    println!("    🔄 Rc<T>: 单线程引用计数，不是线程安全的");
    println!("    🌐 Arc<T>: 多线程引用计数，线程安全");
    println!("    🔒 Mutex<T>: 提供互斥访问");
    println!("    📚 RwLock<T>: 读写锁，允许多个读者或一个写者");
    
    // 线程安全示例
    println!("\n💡 线程安全示例:");
    demonstrate_thread_safety_examples();
}

/// 演示线程安全示例
fn demonstrate_thread_safety_examples() {
    // Arc + Mutex 示例
    println!("\n  🔒 Arc<Mutex<T>> 示例:");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..3 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("    线程 {} 增加计数器", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("    最终计数: {}", *counter.lock().unwrap());
    
    // 引用的线程限制示例
    println!("\n  📝 引用的线程限制:");
    let data = vec![1, 2, 3, 4, 5];
    let data_ref = &data;
    
    // 这会编译失败，因为引用不能跨线程传递
    // thread::spawn(move || {
    //     println!("数据: {:?}", data_ref);
    // });
    
    println!("    引用不能直接在线程间传递 (编译时保护)");
    
    // 使用 Arc 共享不可变数据
    let shared_data = Arc::new(data);
    let shared_clone = Arc::clone(&shared_data);
    
    let handle = thread::spawn(move || {
        println!("    共享数据: {:?}", shared_clone);
    });
    
    handle.join().unwrap();
    println!("    Arc 允许安全的跨线程数据共享");
}

/// 选择指南
fn selection_guide() {
    println!("\n🎯 10. 选择指南");
    println!("{}", "-".repeat(40));
    
    println!("\n🤔 如何选择合适的指针类型?");
    
    // 决策树
    println!("\n🌳 决策树:");
    println!("\n1️⃣ 你需要与 C/C++ 代码交互吗?");
    println!("   ✅ 是 → 使用裸指针 (*const T, *mut T)");
    println!("   ❌ 否 → 继续下一步");
    
    println!("\n2️⃣ 你需要拥有数据的所有权吗?");
    println!("   ❌ 否 → 使用引用 (&T, &mut T)");
    println!("   ✅ 是 → 继续下一步");
    
    println!("\n3️⃣ 数据需要在堆上分配吗?");
    println!("   ❌ 否 → 直接拥有数据 (T)");
    println!("   ✅ 是 → 继续下一步");
    
    println!("\n4️⃣ 数据需要被多个所有者共享吗?");
    println!("   ❌ 否 → 使用 Box<T>");
    println!("   ✅ 是 → 继续下一步");
    
    println!("\n5️⃣ 需要在多线程环境中使用吗?");
    println!("   ❌ 否 → 使用 Rc<T>");
    println!("   ✅ 是 → 使用 Arc<T>");
    
    println!("\n6️⃣ 需要内部可变性吗?");
    println!("   单线程: Rc<RefCell<T>>");
    println!("   多线程: Arc<Mutex<T>> 或 Arc<RwLock<T>>");
    
    // 常见模式
    println!("\n📋 常见模式:");
    
    println!("\n🔸 函数参数:");
    println!("   • 只读访问: &T");
    println!("   • 可写访问: &mut T");
    println!("   • 获取所有权: T");
    
    println!("\n🔸 数据结构:");
    println!("   • 简单字段: T");
    println!("   • 可选字段: Option<T>");
    println!("   • 大型数据: Box<T>");
    println!("   • 共享数据: Rc<T> 或 Arc<T>");
    
    println!("\n🔸 集合元素:");
    println!("   • 值类型: Vec<T>");
    println!("   • 引用类型: Vec<&T> (需要生命周期)");
    println!("   • 智能指针: Vec<Box<T>> 或 Vec<Rc<T>>");
    
    println!("\n🔸 错误处理:");
    println!("   • 可能失败: Result<T, E>");
    println!("   • 可能为空: Option<T>");
    println!("   • 弱引用: Weak<T>");
    
    // 性能考虑
    println!("\n⚡ 性能考虑:");
    println!("\n🏆 最快到最慢:");
    println!("   1. 栈上的值 (T)");
    println!("   2. 引用 (&T, &mut T)");
    println!("   3. 裸指针 (*const T, *mut T)");
    println!("   4. Box<T>");
    println!("   5. Rc<T>");
    println!("   6. Arc<T>");
    println!("   7. RefCell<T>, Mutex<T>, RwLock<T>");
    
    // 安全性考虑
    println!("\n🛡️ 安全性考虑:");
    println!("\n🔒 最安全到最不安全:");
    println!("   1. 引用 (&T, &mut T) - 编译时保证");
    println!("   2. 智能指针 (Box, Rc, Arc) - 运行时安全");
    println!("   3. 内部可变性 (RefCell, Mutex) - 运行时检查");
    println!("   4. 裸指针 (*const T, *mut T) - 无安全保证");
    
    println!("\n✅ 选择指南总结完成");
}

/// 快速对比演示
pub fn quick_comparison_demo() {
    println!("\n🚀 快速对比演示");
    println!("{}", "=".repeat(40));
    
    let data = 42;
    
    // 裸指针
    println!("\n🔸 裸指针:");
    let raw_ptr = &data as *const i32;
    unsafe {
        println!("  值: {}, 地址: {:p}", *raw_ptr, raw_ptr);
    }
    println!("  特点: 最大灵活性，需要 unsafe，无安全保证");
    
    // 引用
    println!("\n🔸 引用:");
    let reference = &data;
    println!("  值: {}, 地址: {:p}", *reference, reference);
    println!("  特点: 编译时安全，零开销，借用语义");
    
    // 智能指针
    println!("\n🔸 智能指针:");
    let smart_ptr = Box::new(42);
    println!("  值: {}, 地址: {:p}", *smart_ptr, &*smart_ptr);
    println!("  特点: 自动内存管理，所有权语义，运行时开销");
    
    println!("\n🎯 选择建议:");
    println!("  • 一般开发: 优先使用引用");
    println!("  • 堆分配: 使用 Box<T>");
    println!("  • 共享数据: 使用 Rc<T> 或 Arc<T>");
    println!("  • 系统编程: 谨慎使用裸指针");
    
    println!("\n✅ 快速对比演示完成");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointer_sizes() {
        assert_eq!(std::mem::size_of::<*const i32>(), std::mem::size_of::<&i32>());
        assert_eq!(std::mem::size_of::<Box<i32>>(), std::mem::size_of::<*const i32>());
    }

    #[test]
    fn test_reference_safety() {
        let x = 42;
        let r1 = &x;
        let r2 = &x;
        assert_eq!(*r1, *r2);
    }

    #[test]
    fn test_smart_pointer_functionality() {
        let boxed = Box::new(String::from("test"));
        assert_eq!(*boxed, "test");
        
        let rc = Rc::new(42);
        let rc_clone = Rc::clone(&rc);
        assert_eq!(Rc::strong_count(&rc), 2);
        assert_eq!(*rc, *rc_clone);
    }

    #[test]
    fn test_cow_behavior() {
        let original = "Hello";
        let mut cow = Cow::Borrowed(original);
        
        // 借用状态
        assert!(matches!(cow, Cow::Borrowed(_)));
        
        // 修改后变为拥有
        cow.to_mut().push_str(", World!");
        assert!(matches!(cow, Cow::Owned(_)));
        assert_eq!(cow.as_ref(), "Hello, World!");
    }
}