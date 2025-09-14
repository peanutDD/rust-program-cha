//! Rust Unsafe 五种超能力深度分析
//! 
//! 本教程全面深入地分析 Rust 中 unsafe 代码的五种超能力：
//! 1. 解引用原始指针 (Dereference raw pointers)
//! 2. 调用不安全函数或方法 (Call unsafe functions or methods)
//! 3. 访问或修改可变静态变量 (Access or modify mutable static variables)
//! 4. 实现不安全 trait (Implement unsafe traits)
//! 5. 访问 union 的字段 (Access fields of unions)
//!
//! 每种超能力都包含详细的理论分析、使用场景、风险评估和实际案例

use std::slice;
use std::mem;
use std::ptr;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

// ============================================================================
// 第一种超能力：解引用原始指针 (Dereference Raw Pointers)
// ============================================================================

/// 演示原始指针的基本概念和使用
fn demonstrate_raw_pointers() {
    println!("\n=== 第一种超能力：解引用原始指针 ===");
    
    // 1.1 原始指针的创建
    println!("\n1.1 原始指针的创建方式：");
    
    let mut num = 5;
    
    // 从引用创建原始指针（安全操作）
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    println!("不可变原始指针: {:p}", r1);
    println!("可变原始指针: {:p}", r2);
    
    // 从任意内存地址创建原始指针（危险操作）
    let address = 0x012345usize;
    let r3 = address as *const i32;
    println!("任意地址原始指针: {:p}", r3);
    
    // 1.2 原始指针的解引用（需要 unsafe）
    println!("\n1.2 原始指针解引用：");
    unsafe {
        println!("*r1 = {}", *r1);
        println!("*r2 = {}", *r2);
        
        // 修改值
        *r2 = 10;
        println!("修改后 num = {}", num);
    }
    
    // 1.3 原始指针的算术运算
    println!("\n1.3 指针算术运算：");
    let arr = [1, 2, 3, 4, 5];
    let ptr = arr.as_ptr();
    
    unsafe {
        for i in 0..5 {
            let element_ptr = ptr.add(i);
            println!("arr[{}] = {} (地址: {:p})", i, *element_ptr, element_ptr);
        }
    }
    
    // 1.4 空指针检查
    println!("\n1.4 空指针处理：");
    let null_ptr: *const i32 = ptr::null();
    if null_ptr.is_null() {
        println!("检测到空指针，避免解引用");
    }
    
    // 1.5 实际应用场景：手动内存管理
    demonstrate_manual_memory_management();
}

/// 演示手动内存管理的实际应用
fn demonstrate_manual_memory_management() {
    println!("\n1.5 实际应用：手动内存管理");
    
    use std::alloc::{alloc, dealloc, Layout};
    
    unsafe {
        // 分配内存
        let layout = Layout::new::<i32>();
        let ptr = alloc(layout) as *mut i32;
        
        if !ptr.is_null() {
            // 写入数据
            *ptr = 42;
            println!("手动分配的内存中的值: {}", *ptr);
            
            // 释放内存
            dealloc(ptr as *mut u8, layout);
            println!("内存已释放");
        } else {
            println!("内存分配失败");
        }
    }
}

// ============================================================================
// 第二种超能力：调用不安全函数或方法 (Call Unsafe Functions)
// ============================================================================

/// 不安全函数示例
unsafe fn dangerous_function() {
    println!("这是一个不安全函数，可能执行危险操作");
}

/// 演示不安全函数的定义和调用
fn demonstrate_unsafe_functions() {
    println!("\n=== 第二种超能力：调用不安全函数或方法 ===");
    
    // 2.1 调用自定义不安全函数
    println!("\n2.1 调用自定义不安全函数：");
    unsafe {
        dangerous_function();
    }
    
    // 2.2 调用标准库中的不安全函数
    println!("\n2.2 标准库不安全函数示例：");
    
    let mut vec = vec![1, 2, 3, 4, 5];
    let ptr = vec.as_mut_ptr();
    let len = vec.len();
    
    // 使用 slice::from_raw_parts 创建切片
    let slice = unsafe {
        slice::from_raw_parts(ptr, len)
    };
    
    println!("通过原始指针创建的切片: {:?}", slice);
    
    // 2.3 不安全方法的包装
    demonstrate_safe_abstraction();
    
    // 2.4 FFI (Foreign Function Interface)
    demonstrate_ffi();
}

/// 演示如何创建安全的抽象
fn demonstrate_safe_abstraction() {
    println!("\n2.3 创建安全的抽象：");
    
    fn split_at_mut_safe<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();
        
        assert!(mid <= len);
        
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
    
    let mut arr = [1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut_safe(&mut arr, 3);
    
    println!("左半部分: {:?}", left);
    println!("右半部分: {:?}", right);
    
    left[0] = 10;
    right[0] = 20;
    
    println!("修改后的数组: {:?}", arr);
}

/// 演示 FFI (外部函数接口)
fn demonstrate_ffi() {
    println!("\n2.4 FFI 外部函数接口：");
    
    // 声明外部 C 函数
    unsafe extern "C" {
        fn abs(input: i32) -> i32;
        fn strlen(s: *const i8) -> usize;
    }
    
    unsafe {
        println!("C abs(-3) = {}", abs(-3));
        
        // 注意：这里使用字符串字面量，它以 null 结尾
        let c_string = b"Hello, World!\0";
        let length = strlen(c_string.as_ptr() as *const i8);
        println!("C strlen = {}", length);
    }
}

// ============================================================================
// 第三种超能力：访问或修改可变静态变量 (Mutable Static Variables)
// ============================================================================

static mut GLOBAL_COUNTER: i32 = 0;
static GLOBAL_NAME: &str = "Rust Unsafe Tutorial";

// 使用 Mutex 的线程安全替代方案
static SAFE_COUNTER: Mutex<i32> = Mutex::new(0);

/// 演示静态变量的访问和修改
fn demonstrate_static_variables() {
    println!("\n=== 第三种超能力：访问或修改可变静态变量 ===");
    
    // 3.1 访问不可变静态变量（安全）
    println!("\n3.1 访问不可变静态变量：");
    println!("程序名称: {}", GLOBAL_NAME);
    
    // 3.2 访问和修改可变静态变量（不安全）
    println!("\n3.2 可变静态变量操作：");
    
    unsafe {
        let current_value = std::ptr::read_volatile(std::ptr::addr_of!(GLOBAL_COUNTER));
        println!("当前计数器值: {}", current_value);
        std::ptr::write_volatile(std::ptr::addr_of_mut!(GLOBAL_COUNTER), current_value + 1);
        let new_value = std::ptr::read_volatile(std::ptr::addr_of!(GLOBAL_COUNTER));
        println!("递增后计数器值: {}", new_value);
    }
    
    // 3.3 静态变量的问题和风险
    demonstrate_static_variable_risks();
    
    // 3.4 线程安全的替代方案
    demonstrate_thread_safe_alternatives();
}

/// 演示静态变量在多线程环境中的风险
fn demonstrate_static_variable_risks() {
    println!("\n3.3 静态变量的风险演示：");
    
    println!("⚠️  可变静态变量在多线程环境中存在数据竞争风险");
    println!("⚠️  编译器无法保证线程安全性");
    println!("⚠️  可能导致未定义行为");
    
    // 模拟多线程访问（实际项目中应避免）
    let handles: Vec<_> = (0..3).map(|i| {
        thread::spawn(move || {
            unsafe {
                let old_value = std::ptr::read_volatile(std::ptr::addr_of!(GLOBAL_COUNTER));
                thread::sleep(Duration::from_millis(1)); // 模拟竞争条件
                std::ptr::write_volatile(std::ptr::addr_of_mut!(GLOBAL_COUNTER), old_value + 1);
                let new_value = std::ptr::read_volatile(std::ptr::addr_of!(GLOBAL_COUNTER));
                println!("线程 {} 设置计数器为: {}", i, new_value);
            }
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    unsafe {
        let final_value = std::ptr::read_volatile(std::ptr::addr_of!(GLOBAL_COUNTER));
        println!("最终计数器值: {} (可能不是预期的值)", final_value);
    }
}

/// 演示线程安全的替代方案
fn demonstrate_thread_safe_alternatives() {
    println!("\n3.4 线程安全的替代方案：");
    
    // 使用 Mutex 保护的静态变量
    let handles: Vec<_> = (0..3).map(|i| {
        thread::spawn(move || {
            let mut counter = SAFE_COUNTER.lock().unwrap();
            *counter += 1;
            println!("线程 {} 安全地递增计数器到: {}", i, *counter);
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_value = *SAFE_COUNTER.lock().unwrap();
    println!("最终安全计数器值: {}", final_value);
}

// ============================================================================
// 第四种超能力：实现不安全 trait (Implement Unsafe Traits)
// ============================================================================

/// 自定义不安全 trait
unsafe trait UnsafeTrait {
    fn unsafe_method(&self);
}

/// 实现不安全 trait 的结构体
struct MyStruct {
    data: i32,
}

// 实现不安全 trait 需要 unsafe 关键字
unsafe impl UnsafeTrait for MyStruct {
    fn unsafe_method(&self) {
        println!("执行不安全方法，数据: {}", self.data);
    }
}

/// 演示不安全 trait 的实现和使用
fn demonstrate_unsafe_traits() {
    println!("\n=== 第四种超能力：实现不安全 trait ===");
    
    // 4.1 自定义不安全 trait
    println!("\n4.1 自定义不安全 trait：");
    
    let my_struct = MyStruct { data: 42 };
    my_struct.unsafe_method();
    
    // 4.2 标准库中的不安全 trait
    demonstrate_send_sync_traits();
    
    // 4.3 实际应用场景
    demonstrate_unsafe_trait_applications();
}

/// 演示 Send 和 Sync trait
fn demonstrate_send_sync_traits() {
    println!("\n4.2 标准库不安全 trait - Send 和 Sync：");
    
    // Send: 类型可以在线程间转移所有权
    // Sync: 类型可以在线程间安全地共享引用
    
    println!("Send trait: 允许类型在线程间转移所有权");
    println!("Sync trait: 允许类型在线程间安全地共享引用");
    
    // 大多数类型自动实现 Send 和 Sync
    let data = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        println!("在新线程中处理数据: {:?}", data);
    });
    
    handle.join().unwrap();
    
    // 演示不满足 Send 的类型
    demonstrate_non_send_type();
}

/// 演示不满足 Send 的类型
fn demonstrate_non_send_type() {
    use std::rc::Rc;
    
    println!("\n4.2.1 不满足 Send 的类型示例：");
    
    let rc_data = Rc::new(vec![1, 2, 3]);
    println!("Rc<T> 不实现 Send，因为它使用非原子引用计数");
    
    // 以下代码会编译错误，因为 Rc 不是 Send
    // let handle = thread::spawn(move || {
    //     println!("数据: {:?}", rc_data);
    // });
    
    // 正确的做法是使用 Arc
    use std::sync::Arc;
    let arc_data = Arc::new(vec![1, 2, 3]);
    let arc_clone = Arc::clone(&arc_data);
    
    let handle = thread::spawn(move || {
        println!("Arc 数据: {:?}", arc_clone);
    });
    
    handle.join().unwrap();
    println!("原始 Arc 数据: {:?}", arc_data);
}

/// 演示不安全 trait 的实际应用
fn demonstrate_unsafe_trait_applications() {
    println!("\n4.3 不安全 trait 的实际应用：");
    
    // 4.3.1 自定义智能指针
    demonstrate_custom_smart_pointer();
    
    // 4.3.2 零成本抽象
    demonstrate_zero_cost_abstraction();
}

/// 自定义智能指针示例
struct MyBox<T> {
    ptr: *mut T,
}

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        let boxed = Box::new(value);
        let ptr = Box::into_raw(boxed);
        MyBox { ptr }
    }
    
    fn get(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.ptr);
        }
    }
}

// 为自定义智能指针实现 Send 和 Sync
unsafe impl<T: Send> Send for MyBox<T> {}
unsafe impl<T: Sync> Sync for MyBox<T> {}

fn demonstrate_custom_smart_pointer() {
    println!("\n4.3.1 自定义智能指针：");
    
    let my_box = MyBox::new(42);
    println!("MyBox 中的值: {}", my_box.get());
    
    // 可以在线程间传递
    let handle = thread::spawn(move || {
        println!("线程中的 MyBox 值: {}", my_box.get());
    });
    
    handle.join().unwrap();
}

/// 零成本抽象示例
fn demonstrate_zero_cost_abstraction() {
    println!("\n4.3.2 零成本抽象：");
    
    // 使用不安全代码实现高性能的向量操作
    fn unsafe_vector_sum(data: &[i32]) -> i64 {
        let mut sum = 0i64;
        let ptr = data.as_ptr();
        let len = data.len();
        
        unsafe {
            for i in 0..len {
                sum += *ptr.add(i) as i64;
            }
        }
        
        sum
    }
    
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum = unsafe_vector_sum(&data);
    println!("向量和: {}", sum);
    
    // 对比安全版本
    let safe_sum: i64 = data.iter().map(|&x| x as i64).sum();
    println!("安全版本和: {}", safe_sum);
    assert_eq!(sum, safe_sum);
}

// ============================================================================
// 第五种超能力：访问 union 的字段 (Access Union Fields)
// ============================================================================

/// 定义 union 类型
#[repr(C)]
union MyUnion {
    i: i32,
    f: f32,
    bytes: [u8; 4],
}

/// 演示 union 的使用
fn demonstrate_unions() {
    println!("\n=== 第五种超能力：访问 union 的字段 ===");
    
    // 5.1 基本 union 操作
    println!("\n5.1 基本 union 操作：");
    
    let mut my_union = MyUnion { i: 0x12345678 };
    
    unsafe {
        println!("作为 i32: 0x{:08x}", my_union.i);
        println!("作为 f32: {}", my_union.f);
        println!("作为字节数组: {:?}", my_union.bytes);
    }
    
    // 5.2 union 的内存布局
    demonstrate_union_memory_layout();
    
    // 5.3 实际应用场景
    demonstrate_union_applications();
    
    // 5.4 union 的安全使用模式
    demonstrate_safe_union_patterns();
}

/// 演示 union 的内存布局
fn demonstrate_union_memory_layout() {
    println!("\n5.2 union 内存布局分析：");
    
    let union_size = mem::size_of::<MyUnion>();
    let union_align = mem::align_of::<MyUnion>();
    
    println!("MyUnion 大小: {} 字节", union_size);
    println!("MyUnion 对齐: {} 字节", union_align);
    
    // 演示不同字段的内存表示
    let mut my_union = MyUnion { i: 0 };
    
    unsafe {
        my_union.i = 0x41424344; // ASCII "ABCD" 的十六进制表示
        println!("\n设置 i32 值为 0x41424344:");
        println!("  作为 i32: 0x{:08x}", my_union.i);
        println!("  作为 f32: {}", my_union.f);
        println!("  作为字节: {:?}", my_union.bytes);
        
        // 字节序演示
        let bytes = my_union.bytes;
        println!("  字节序分析:");
        for (i, byte) in bytes.iter().enumerate() {
            println!("    bytes[{}] = 0x{:02x} ('{}')", i, byte, *byte as char);
        }
    }
}

/// union 的实际应用场景
fn demonstrate_union_applications() {
    println!("\n5.3 union 实际应用场景：");
    
    // 5.3.1 类型双关 (Type Punning)
    demonstrate_type_punning();
    
    // 5.3.2 C 互操作
    demonstrate_c_interop();
    
    // 5.3.3 内存优化
    demonstrate_memory_optimization();
}

/// 类型双关示例
fn demonstrate_type_punning() {
    println!("\n5.3.1 类型双关 (Type Punning)：");
    
    #[repr(C)]
    union FloatBits {
        f: f32,
        bits: u32,
    }
    
    let pi = std::f32::consts::PI;
    let float_union = FloatBits { f: pi };
    
    unsafe {
        println!("π 的值: {}", float_union.f);
        println!("π 的位表示: 0x{:08x}", float_union.bits);
        println!("π 的二进制: {:032b}", float_union.bits);
        
        // 分析 IEEE 754 格式
        let bits = float_union.bits;
        let sign = (bits >> 31) & 1;
        let exponent = (bits >> 23) & 0xFF;
        let mantissa = bits & 0x7FFFFF;
        
        println!("IEEE 754 分析:");
        println!("  符号位: {}", sign);
        println!("  指数: {} (偏移后: {})", exponent, exponent as i32 - 127);
        println!("  尾数: 0x{:06x}", mantissa);
    }
}

/// C 互操作示例
fn demonstrate_c_interop() {
    println!("\n5.3.2 C 互操作：");
    
    // 模拟 C 语言中的 union
    #[repr(C)]
    union CStyleUnion {
        integer: i32,
        floating: f32,
        character: [i8; 4],
    }
    
    let c_union = CStyleUnion { integer: 0x48656C6C }; // "Hell" in ASCII
    
    unsafe {
        println!("C 风格 union:");
        println!("  作为整数: 0x{:08x}", c_union.integer);
        println!("  作为浮点: {}", c_union.floating);
        print!("  作为字符: ");
        for &ch in &c_union.character {
            if ch != 0 {
                print!("{}", ch as u8 as char);
            }
        }
        println!();
    }
}

/// 内存优化示例
fn demonstrate_memory_optimization() {
    println!("\n5.3.3 内存优化：");
    
    // 使用 union 实现变体类型
    #[repr(C)]
    union VariantData {
        int_val: i64,
        float_val: f64,
        ptr_val: *const u8,
    }
    
    #[repr(C)]
    struct Variant {
        tag: u8,
        data: VariantData,
    }
    
    impl Variant {
        fn new_int(value: i64) -> Self {
            Variant {
                tag: 0,
                data: VariantData { int_val: value },
            }
        }
        
        fn new_float(value: f64) -> Self {
            Variant {
                tag: 1,
                data: VariantData { float_val: value },
            }
        }
        
        fn as_int(&self) -> Option<i64> {
            if self.tag == 0 {
                Some(unsafe { self.data.int_val })
            } else {
                None
            }
        }
        
        fn as_float(&self) -> Option<f64> {
            if self.tag == 1 {
                Some(unsafe { self.data.float_val })
            } else {
                None
            }
        }
    }
    
    let int_variant = Variant::new_int(42);
    let float_variant = Variant::new_float(3.14);
    
    println!("变体类型大小: {} 字节", mem::size_of::<Variant>());
    println!("整数变体: {:?}", int_variant.as_int());
    println!("浮点变体: {:?}", float_variant.as_float());
}

/// union 的安全使用模式
fn demonstrate_safe_union_patterns() {
    println!("\n5.4 union 的安全使用模式：");
    
    // 5.4.1 标记联合 (Tagged Union)
    demonstrate_tagged_union();
    
    // 5.4.2 初始化模式
    demonstrate_union_initialization();
}

/// 标记联合模式
fn demonstrate_tagged_union() {
    println!("\n5.4.1 标记联合模式：");
    
    #[repr(C)]
    union Data {
        i: i32,
        f: f32,
        b: bool,
    }
    
    #[repr(C)]
    struct TaggedUnion {
        tag: u8,
        data: Data,
    }
    
    const TAG_INT: u8 = 0;
    const TAG_FLOAT: u8 = 1;
    const TAG_BOOL: u8 = 2;
    
    impl TaggedUnion {
        fn new_int(value: i32) -> Self {
            TaggedUnion {
                tag: TAG_INT,
                data: Data { i: value },
            }
        }
        
        fn new_float(value: f32) -> Self {
            TaggedUnion {
                tag: TAG_FLOAT,
                data: Data { f: value },
            }
        }
        
        fn new_bool(value: bool) -> Self {
            TaggedUnion {
                tag: TAG_BOOL,
                data: Data { b: value },
            }
        }
        
        fn get_value(&self) -> String {
            unsafe {
                match self.tag {
                    TAG_INT => format!("整数: {}", self.data.i),
                    TAG_FLOAT => format!("浮点: {}", self.data.f),
                    TAG_BOOL => format!("布尔: {}", self.data.b),
                    _ => "未知类型".to_string(),
                }
            }
        }
    }
    
    let values = vec![
        TaggedUnion::new_int(42),
        TaggedUnion::new_float(3.14),
        TaggedUnion::new_bool(true),
    ];
    
    for (i, value) in values.iter().enumerate() {
        println!("值 {}: {}", i, value.get_value());
    }
}

/// union 初始化模式
fn demonstrate_union_initialization() {
    println!("\n5.4.2 union 初始化模式：");
    
    #[repr(C)]
    union InitUnion {
        i: i32,
        f: f32,
    }
    
    // 正确的初始化方式
    let union1 = InitUnion { i: 42 };
    let union2 = InitUnion { f: 3.14 };
    
    unsafe {
        println!("union1 (初始化为 i32): {}", union1.i);
        println!("union2 (初始化为 f32): {}", union2.f);
    }
    
    // 演示未初始化的危险
    println!("\n⚠️  union 使用注意事项：");
    println!("1. 必须明确知道当前活跃的字段");
    println!("2. 访问非活跃字段可能导致未定义行为");
    println!("3. 建议使用标记联合模式");
    println!("4. 考虑使用 Rust 的 enum 作为更安全的替代");
}

// ============================================================================
// 综合应用和最佳实践
// ============================================================================

/// 演示 unsafe 代码的最佳实践
fn demonstrate_best_practices() {
    println!("\n=== Unsafe Rust 最佳实践 ===");
    
    // 6.1 最小化 unsafe 代码范围
    demonstrate_minimize_unsafe_scope();
    
    // 6.2 文档化不变量
    demonstrate_document_invariants();
    
    // 6.3 测试和验证
    demonstrate_testing_unsafe_code();
    
    // 6.4 常见陷阱和避免方法
    demonstrate_common_pitfalls();
}

/// 最小化 unsafe 代码范围
fn demonstrate_minimize_unsafe_scope() {
    println!("\n6.1 最小化 unsafe 代码范围：");
    
    // ❌ 不好的做法：整个函数都是 unsafe
    // unsafe fn bad_example() {
    //     let ptr = std::ptr::null_mut::<i32>();
    //     let value = 42;
    //     // 大量安全代码...
    //     *ptr = value; // 只有这一行需要 unsafe
    // }
    
    // ✅ 好的做法：只在必要时使用 unsafe
    fn good_example() {
        let mut value = 42;
        let ptr = &mut value as *mut i32;
        
        // 大量安全代码...
        println!("准备写入值");
        
        // 只在必要时使用 unsafe
        unsafe {
            *ptr = 100;
        }
        
        println!("值已更新为: {}", value);
    }
    
    good_example();
    
    println!("✅ 将 unsafe 代码限制在最小必要范围内");
    println!("✅ 在 unsafe 块周围添加详细注释");
    println!("✅ 将 unsafe 操作封装在安全的接口中");
}

/// 文档化不变量
fn demonstrate_document_invariants() {
    println!("\n6.2 文档化不变量：");
    
    /// 安全地从原始指针创建切片
    /// 
    /// # 安全性
    /// 
    /// 调用者必须确保：
    /// - `ptr` 指向有效的内存区域
    /// - 内存区域至少包含 `len` 个 `T` 类型的元素
    /// - 内存区域在返回的切片生命周期内保持有效
    /// - 没有其他可变引用指向同一内存区域
    unsafe fn slice_from_raw_parts_documented<T>(ptr: *const T, len: usize) -> &'static [T] {
        // SAFETY: 调用者保证了上述不变量
        slice::from_raw_parts(ptr, len)
    }
    
    let data = [1, 2, 3, 4, 5];
    let ptr = data.as_ptr();
    
    let slice = unsafe {
        // SAFETY: 
        // - ptr 指向有效的栈分配数组
        // - 数组包含 5 个元素
        // - 数组在函数结束前保持有效
        // - 没有可变引用
        slice_from_raw_parts_documented(ptr, data.len())
    };
    
    println!("文档化的切片: {:?}", slice);
    
    println!("\n📝 文档化要点：");
    println!("1. 明确说明安全性要求");
    println!("2. 在调用点添加 SAFETY 注释");
    println!("3. 解释为什么满足安全性要求");
    println!("4. 记录所有不变量和前置条件");
}

/// 测试 unsafe 代码
fn demonstrate_testing_unsafe_code() {
    println!("\n6.3 测试和验证 unsafe 代码：");
    
    // 示例：测试自定义向量实现
    struct UnsafeVec<T> {
        ptr: *mut T,
        len: usize,
        cap: usize,
    }
    
    impl<T> UnsafeVec<T> {
        fn new() -> Self {
            UnsafeVec {
                ptr: std::ptr::NonNull::dangling().as_ptr(),
                len: 0,
                cap: 0,
            }
        }
        
        fn push(&mut self, value: T) {
            if self.len == self.cap {
                self.grow();
            }
            
            unsafe {
                std::ptr::write(self.ptr.add(self.len), value);
            }
            
            self.len += 1;
        }
        
        fn get(&self, index: usize) -> Option<&T> {
            if index < self.len {
                unsafe {
                    Some(&*self.ptr.add(index))
                }
            } else {
                None
            }
        }
        
        fn grow(&mut self) {
            let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
            
            let layout = std::alloc::Layout::array::<T>(new_cap).unwrap();
            
            let new_ptr = if self.cap == 0 {
                unsafe { std::alloc::alloc(layout) as *mut T }
            } else {
                let old_layout = std::alloc::Layout::array::<T>(self.cap).unwrap();
                unsafe {
                    std::alloc::realloc(self.ptr as *mut u8, old_layout, layout.size()) as *mut T
                }
            };
            
            if new_ptr.is_null() {
                panic!("内存分配失败");
            }
            
            self.ptr = new_ptr;
            self.cap = new_cap;
        }
    }
    
    impl<T> Drop for UnsafeVec<T> {
        fn drop(&mut self) {
            if self.cap > 0 {
                unsafe {
                    for i in 0..self.len {
                        std::ptr::drop_in_place(self.ptr.add(i));
                    }
                    
                    let layout = std::alloc::Layout::array::<T>(self.cap).unwrap();
                    std::alloc::dealloc(self.ptr as *mut u8, layout);
                }
            }
        }
    }
    
    // 测试自定义向量
    let mut vec = UnsafeVec::new();
    
    // 基本功能测试
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    assert_eq!(vec.get(0), Some(&1));
    assert_eq!(vec.get(1), Some(&2));
    assert_eq!(vec.get(2), Some(&3));
    assert_eq!(vec.get(3), None);
    
    println!("✅ 基本功能测试通过");
    
    // 容量扩展测试
    for i in 4..=10 {
        vec.push(i);
    }
    
    for i in 1..=10 {
        assert_eq!(vec.get(i - 1), Some(&i));
    }
    
    println!("✅ 容量扩展测试通过");
    
    println!("\n🧪 测试策略：");
    println!("1. 单元测试覆盖所有边界情况");
    println!("2. 使用 Miri 检测未定义行为");
    println!("3. 压力测试和模糊测试");
    println!("4. 内存泄漏检测");
    println!("5. 并发测试（如果适用）");
}

/// 常见陷阱和避免方法
fn demonstrate_common_pitfalls() {
    println!("\n6.4 常见陷阱和避免方法：");
    
    // 6.4.1 悬垂指针
    println!("\n6.4.1 悬垂指针：");
    
    // ❌ 危险：返回局部变量的指针
    // fn dangling_pointer() -> *const i32 {
    //     let x = 42;
    //     &x as *const i32  // x 在函数结束时被销毁
    // }
    
    // ✅ 安全：使用堆分配或生命周期参数
    fn safe_pointer() -> Box<i32> {
        Box::new(42)
    }
    
    let safe_ptr = safe_pointer();
    println!("安全指针的值: {}", *safe_ptr);
    
    // 6.4.2 双重释放
    println!("\n6.4.2 双重释放：");
    
    // ❌ 危险：多次释放同一内存
    // unsafe {
    //     let ptr = std::alloc::alloc(Layout::new::<i32>()) as *mut i32;
    //     std::alloc::dealloc(ptr as *mut u8, Layout::new::<i32>());
    //     std::alloc::dealloc(ptr as *mut u8, Layout::new::<i32>()); // 双重释放！
    // }
    
    // ✅ 安全：确保每个指针只释放一次
    unsafe {
        let layout = std::alloc::Layout::new::<i32>();
        let ptr = std::alloc::alloc(layout) as *mut i32;
        if !ptr.is_null() {
            *ptr = 42;
            println!("安全释放的值: {}", *ptr);
            std::alloc::dealloc(ptr as *mut u8, layout);
            // ptr 现在无效，不再使用
        }
    }
    
    // 6.4.3 数据竞争
    println!("\n6.4.3 数据竞争：");
    
    // ❌ 危险：多线程访问可变静态变量
    // 前面已经演示过
    
    // ✅ 安全：使用同步原语
    use std::sync::atomic::{AtomicI32, Ordering};
    
    static ATOMIC_COUNTER: AtomicI32 = AtomicI32::new(0);
    
    let handles: Vec<_> = (0..3).map(|i| {
        thread::spawn(move || {
            let old_value = ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst);
            println!("线程 {} 原子操作: {} -> {}", i, old_value, old_value + 1);
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终原子计数器值: {}", ATOMIC_COUNTER.load(Ordering::SeqCst));
    
    // 6.4.4 内存对齐问题
    println!("\n6.4.4 内存对齐问题：");
    
    #[repr(C)]
    struct AlignedStruct {
        a: u8,
        b: u64,
    }
    
    println!("结构体大小: {}", mem::size_of::<AlignedStruct>());
    println!("结构体对齐: {}", mem::align_of::<AlignedStruct>());
    
    // ✅ 正确处理对齐
    let aligned = AlignedStruct { a: 1, b: 42 };
    println!("正确对齐的结构体: a={}, b={}", aligned.a, aligned.b);
    
    println!("\n⚠️  避免陷阱的建议：");
    println!("1. 最小化 unsafe 代码的范围");
    println!("2. 在 unsafe 块周围添加详细注释");
    println!("3. 构建安全的抽象接口");
    println!("4. 使用工具验证内存安全 (Miri, AddressSanitizer)");
    println!("5. 编写全面的测试，包括边界情况");
    println!("6. 考虑使用现有的安全库而不是自己实现");
    println!("7. 定期审查和重构 unsafe 代码");
}

// ============================================================================
// 主函数和测试
// ============================================================================

fn main() {
    println!("🦀 Rust Unsafe 五种超能力深度分析");
    println!("=====================================\n");
    
    println!("Unsafe Rust 提供了五种超能力，让程序员能够：");
    println!("1. 解引用原始指针");
    println!("2. 调用不安全函数或方法");
    println!("3. 访问或修改可变静态变量");
    println!("4. 实现不安全 trait");
    println!("5. 访问 union 的字段");
    
    println!("\n⚠️  重要提醒：");
    println!("- unsafe 不会关闭借用检查器");
    println!("- unsafe 只是提供额外的超能力");
    println!("- 程序员需要手动确保内存安全");
    println!("- 应该将 unsafe 代码封装在安全的抽象中");
    
    // 演示所有五种超能力
    demonstrate_raw_pointers();
    demonstrate_unsafe_functions();
    demonstrate_static_variables();
    demonstrate_unsafe_traits();
    demonstrate_unions();
    
    // 最佳实践
    demonstrate_best_practices();
    
    println!("\n🎉 Unsafe Rust 五种超能力分析完成！");
    
    println!("\n核心要点总结:");
    println!("• unsafe 不会关闭借用检查器，只是提供额外的超能力");
    println!("• 五种超能力：解引用原始指针、调用不安全函数、访问可变静态变量、实现不安全 trait、访问 union 字段");
    println!("• 原始指针不受借用检查器约束，但需要在 unsafe 块中解引用");
    println!("• 构建安全抽象是 unsafe 代码的主要目标");
    println!("• FFI 允许与其他语言互操作，但需要谨慎处理内存安全");
    println!("• Send 和 Sync 是重要的不安全 trait，用于线程安全");
    println!("• union 提供了类型双关和 C 互操作的能力");
    println!("• 最小化 unsafe 范围，添加详细文档，编写全面测试");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_raw_pointers() {
        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        
        unsafe {
            assert_eq!(*r1, 5);
            *r2 = 10;
            assert_eq!(*r2, 10);
        }
    }
    
    #[test]
    fn test_unsafe_functions() {
        let data = vec![1, 2, 3, 4, 5];
        let slice = unsafe {
            slice::from_raw_parts(data.as_ptr(), data.len())
        };
        assert_eq!(slice, &[1, 2, 3, 4, 5]);
    }
    
    #[test]
    fn test_static_variables() {
        unsafe {
            std::ptr::write_volatile(std::ptr::addr_of_mut!(GLOBAL_COUNTER), 100);
            let value = std::ptr::read_volatile(std::ptr::addr_of!(GLOBAL_COUNTER));
            assert_eq!(value, 100);
        }
    }
    
    #[test]
    fn test_unsafe_traits() {
        let my_struct = MyStruct { data: 42 };
        my_struct.unsafe_method(); // 应该正常执行
    }
    
    #[test]
    fn test_unions() {
        let mut my_union = MyUnion { i: 0x12345678 };
        
        unsafe {
            assert_eq!(my_union.i, 0x12345678);
            
            // 测试字节表示
            let bytes = my_union.bytes;
            assert_eq!(bytes.len(), 4);
        }
    }
    
    #[test]
    fn test_custom_smart_pointer() {
        let my_box = MyBox::new(42);
        assert_eq!(*my_box.get(), 42);
    }
}
