//! # 裸指针 (Raw Pointers) 深度解析
//!
//! 裸指针是 Rust 中最底层的指针类型，提供了直接的内存访问能力。
//! 本模块深入探讨裸指针的特性、使用场景和安全考虑。
//!
//! ## 裸指针类型
//!
//! - `*const T`: 不可变裸指针
//! - `*mut T`: 可变裸指针
//!
//! ## 关键特性
//!
//! 1. **不受借用检查器约束**: 可以有多个可变裸指针指向同一内存
//! 2. **不保证指向有效内存**: 可能是悬垂指针或空指针
//! 3. **不自动清理**: 不实现 Drop trait
//! 4. **需要 unsafe 块**: 解引用裸指针需要在 unsafe 块中进行
//! 5. **可以为空**: 可以是 null 指针
//! 6. **可以进行指针运算**: 支持指针偏移等操作

use std::ptr;
use std::mem;
use std::alloc::{alloc, dealloc, Layout};
use std::slice;

/// 运行所有裸指针示例
pub fn run_raw_pointer_examples() {
    println!("🔍 裸指针深度解析");
    println!("{}", "=".repeat(50));

    basic_raw_pointer_usage();
    raw_pointer_creation_methods();
    raw_pointer_arithmetic();
    null_pointer_handling();
    raw_pointer_casting();
    memory_layout_exploration();
    unsafe_operations();
    raw_pointer_with_collections();
    advanced_raw_pointer_patterns();
    raw_pointer_pitfalls();
}

/// 基础裸指针使用
fn basic_raw_pointer_usage() {
    println!("\n📌 1. 基础裸指针使用");
    println!("{}", "-".repeat(30));

    // 从引用创建裸指针
    let x = 42;
    let const_ptr: *const i32 = &x;
    let mut y = 100;
    let mut_ptr: *mut i32 = &mut y;

    println!("原始值: x = {}, y = {}", x, y);
    println!("裸指针地址: const_ptr = {:p}, mut_ptr = {:p}", const_ptr, mut_ptr);

    // 解引用裸指针（需要 unsafe）
    unsafe {
        println!("通过裸指针读取: *const_ptr = {}, *mut_ptr = {}", *const_ptr, *mut_ptr);
        
        // 通过可变裸指针修改值
        *mut_ptr = 200;
        println!("修改后: y = {}, *mut_ptr = {}", y, *mut_ptr);
    }

    // 裸指针的特殊性质
    demonstrate_raw_pointer_properties();
}

/// 演示裸指针的特殊性质
fn demonstrate_raw_pointer_properties() {
    println!("\n🔬 裸指针特殊性质:");
    
    let mut value = 42;
    
    // 可以创建多个可变裸指针指向同一内存
    let ptr1: *mut i32 = &mut value;
    let ptr2: *mut i32 = &mut value; // 这在引用中是不允许的！
    
    println!("多个可变裸指针指向同一内存:");
    println!("ptr1 = {:p}, ptr2 = {:p}", ptr1, ptr2);
    
    unsafe {
        *ptr1 = 100;
        println!("通过 ptr1 修改: value = {}", value);
        
        *ptr2 = 200;
        println!("通过 ptr2 修改: value = {}", value);
    }
}

/// 裸指针创建方法
fn raw_pointer_creation_methods() {
    println!("\n📌 2. 裸指针创建方法");
    println!("{}", "-".repeat(30));

    // 方法1: 从引用创建
    let x = 42;
    let ptr_from_ref = &x as *const i32;
    println!("从引用创建: {:p}", ptr_from_ref);

    // 方法2: 从可变引用创建
    let mut y = 100;
    let ptr_from_mut_ref = &mut y as *mut i32;
    println!("从可变引用创建: {:p}", ptr_from_mut_ref);

    // 方法3: 使用 ptr::null() 创建空指针
    let null_ptr: *const i32 = ptr::null();
    let null_mut_ptr: *mut i32 = ptr::null_mut();
    println!("空指针: const = {:p}, mut = {:p}", null_ptr, null_mut_ptr);

    // 方法4: 从整数地址创建（极其危险！）
    let addr_ptr = 0x1000 as *const i32;
    println!("从地址创建: {:p}", addr_ptr);

    // 方法5: 使用 Box::into_raw() 从堆分配创建
    let boxed = Box::new(42);
    let heap_ptr = Box::into_raw(boxed);
    println!("从堆分配创建: {:p}", heap_ptr);
    
    // 记住释放内存
    unsafe {
        let _boxed_back = Box::from_raw(heap_ptr);
        // boxed_back 在这里被自动释放
    }

    // 方法6: 使用 std::alloc 手动分配
    demonstrate_manual_allocation();
}

/// 演示手动内存分配
fn demonstrate_manual_allocation() {
    println!("\n🔧 手动内存分配:");
    
    unsafe {
        // 分配内存
        let layout = Layout::new::<i32>();
        let ptr = alloc(layout) as *mut i32;
        
        if !ptr.is_null() {
            // 初始化内存
            ptr::write(ptr, 42);
            println!("手动分配并初始化: *ptr = {}", *ptr);
            
            // 释放内存
            dealloc(ptr as *mut u8, layout);
            println!("内存已释放");
        } else {
            println!("内存分配失败");
        }
    }
}

/// 裸指针算术运算
fn raw_pointer_arithmetic() {
    println!("\n📌 3. 裸指针算术运算");
    println!("{}", "-".repeat(30));

    let array = [1, 2, 3, 4, 5];
    let ptr = array.as_ptr();

    println!("原始数组: {:?}", array);
    println!("数组首地址: {:p}", ptr);

    unsafe {
        // 指针偏移
        for i in 0..array.len() {
            let element_ptr = ptr.add(i);
            println!("array[{}] = {} (地址: {:p})", i, *element_ptr, element_ptr);
        }

        // 指针减法
        let end_ptr = ptr.add(array.len() - 1);
        let offset = end_ptr.offset_from(ptr);
        println!("指针偏移量: {}", offset);

        // 指针比较
        let mid_ptr = ptr.add(2);
        println!("指针比较: mid_ptr > ptr = {}", mid_ptr > ptr);
    }

    // 演示指针运算的危险性
    demonstrate_pointer_arithmetic_dangers();
}

/// 演示指针运算的危险性
fn demonstrate_pointer_arithmetic_dangers() {
    println!("\n⚠️ 指针运算危险性演示:");
    
    let array = [1, 2, 3];
    let ptr = array.as_ptr();
    
    unsafe {
        // 越界访问（未定义行为！）
        println!("正常访问: array[0] = {}", *ptr);
        println!("正常访问: array[2] = {}", *ptr.add(2));
        
        // 以下是危险的操作，可能导致未定义行为
        // println!("越界访问: {}", *ptr.add(10)); // 不要这样做！
        // println!("负偏移: {}", *ptr.sub(1));     // 不要这样做！
        
        println!("⚠️ 越界访问被注释掉以避免未定义行为");
    }
}

/// 空指针处理
fn null_pointer_handling() {
    println!("\n📌 4. 空指针处理");
    println!("{}", "-".repeat(30));

    // 创建空指针
    let null_ptr: *const i32 = ptr::null();
    let null_mut_ptr: *mut i32 = ptr::null_mut();

    // 检查空指针
    println!("空指针检查:");
    println!("null_ptr.is_null() = {}", null_ptr.is_null());
    println!("null_mut_ptr.is_null() = {}", null_mut_ptr.is_null());

    // 安全的空指针处理
    fn safe_dereference(ptr: *const i32) -> Option<i32> {
        if ptr.is_null() {
            None
        } else {
            unsafe { Some(*ptr) }
        }
    }

    let value = 42;
    let valid_ptr = &value as *const i32;
    
    println!("安全解引用结果:");
    println!("valid_ptr: {:?}", safe_dereference(valid_ptr));
    println!("null_ptr: {:?}", safe_dereference(null_ptr));

    // NonNull 类型
    demonstrate_non_null();
}

/// 演示 NonNull 类型
fn demonstrate_non_null() {
    use std::ptr::NonNull;
    
    println!("\n🔒 NonNull 类型:");
    
    let mut value = 42;
    
    // NonNull 保证非空
    if let Some(non_null) = NonNull::new(&mut value as *mut i32) {
        unsafe {
            println!("NonNull 值: {}", *non_null.as_ptr());
            *non_null.as_ptr() = 100;
            println!("修改后: {}", value);
        }
    }
    
    // 尝试从空指针创建 NonNull
    let null_ptr: *mut i32 = ptr::null_mut();
    match NonNull::new(null_ptr) {
        Some(_) => println!("NonNull 创建成功"),
        None => println!("NonNull 创建失败：指针为空"),
    }
}

/// 裸指针类型转换
fn raw_pointer_casting() {
    println!("\n📌 5. 裸指针类型转换");
    println!("{}", "-".repeat(30));

    let value = 0x12345678u32;
    let ptr = &value as *const u32;

    println!("原始值: 0x{:08x}", value);
    println!("u32 指针: {:p}", ptr);

    // 类型转换
    let byte_ptr = ptr as *const u8;
    let void_ptr = ptr as *const ();
    let int_ptr = ptr as *const i32;

    unsafe {
        println!("作为 u8 读取: 0x{:02x}", *byte_ptr);
        println!("作为 i32 读取: 0x{:08x}", *int_ptr);
        
        // 读取所有字节
        let bytes = slice::from_raw_parts(byte_ptr, 4);
        println!("字节表示: {:?}", bytes);
    }

    // 演示对齐要求
    demonstrate_alignment_requirements();
}

/// 演示对齐要求
fn demonstrate_alignment_requirements() {
    println!("\n📐 对齐要求演示:");
    
    // 不同类型的对齐要求
    println!("对齐要求:");
    println!("u8:  {} 字节", mem::align_of::<u8>());
    println!("u16: {} 字节", mem::align_of::<u16>());
    println!("u32: {} 字节", mem::align_of::<u32>());
    println!("u64: {} 字节", mem::align_of::<u64>());
    
    let array = [0u8; 8];
    let ptr = array.as_ptr();
    
    // 检查指针对齐
    println!("\n指针对齐检查:");
    println!("ptr 地址: {:p}", ptr);
    println!("ptr 对 u16 对齐: {}", (ptr as usize) % mem::align_of::<u16>() == 0);
    println!("ptr 对 u32 对齐: {}", (ptr as usize) % mem::align_of::<u32>() == 0);
    println!("ptr 对 u64 对齐: {}", (ptr as usize) % mem::align_of::<u64>() == 0);
}

/// 内存布局探索
fn memory_layout_exploration() {
    println!("\n📌 6. 内存布局探索");
    println!("{}", "-".repeat(30));

    // 结构体内存布局
    #[repr(C)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 10, y: 20 };
    let point_ptr = &point as *const Point;
    let x_ptr = &point.x as *const i32;
    let y_ptr = &point.y as *const i32;

    println!("结构体内存布局:");
    println!("Point 地址: {:p}", point_ptr);
    println!("x 字段地址: {:p}", x_ptr);
    println!("y 字段地址: {:p}", y_ptr);
    println!("字段偏移: {} 字节", y_ptr as usize - x_ptr as usize);

    unsafe {
        // 通过指针访问字段
        let x_via_ptr = (point_ptr as *const i32).add(0);
        let y_via_ptr = (point_ptr as *const i32).add(1);
        
        println!("通过指针访问: x = {}, y = {}", *x_via_ptr, *y_via_ptr);
    }

    // 数组内存布局
    demonstrate_array_layout();
}

/// 演示数组内存布局
fn demonstrate_array_layout() {
    println!("\n📊 数组内存布局:");
    
    let array = [1i32, 2, 3, 4, 5];
    let array_ptr = array.as_ptr();
    
    println!("数组: {:?}", array);
    println!("数组地址: {:p}", array_ptr);
    
    for (i, &value) in array.iter().enumerate() {
        let element_ptr = unsafe { array_ptr.add(i) };
        println!("array[{}] = {} (地址: {:p}, 偏移: {} 字节)", 
                i, value, element_ptr, i * mem::size_of::<i32>());
    }
}

/// 不安全操作示例
fn unsafe_operations() {
    println!("\n📌 7. 不安全操作示例");
    println!("{}", "-".repeat(30));

    // 原始内存操作
    demonstrate_raw_memory_operations();
    
    // 类型双关
    demonstrate_type_punning();
    
    // 未初始化内存
    demonstrate_uninitialized_memory();
}

/// 演示原始内存操作
fn demonstrate_raw_memory_operations() {
    println!("\n🔧 原始内存操作:");
    
    let mut buffer = [0u8; 16];
    let ptr = buffer.as_mut_ptr();
    
    unsafe {
        // 写入数据
        ptr::write(ptr as *mut u32, 0x12345678);
        ptr::write(ptr.add(4) as *mut u32, 0x9abcdef0);
        
        // 读取数据
        let value1 = ptr::read(ptr as *const u32);
        let value2 = ptr::read(ptr.add(4) as *const u32);
        
        println!("写入的值: 0x{:08x}, 0x{:08x}", value1, value2);
        println!("缓冲区内容: {:?}", buffer);
        
        // 内存复制
        let mut dest = [0u8; 8];
        ptr::copy_nonoverlapping(ptr, dest.as_mut_ptr(), 8);
        println!("复制的数据: {:?}", dest);
        
        // 内存比较
        let is_equal = ptr::eq(ptr, buffer.as_ptr());
        println!("指针相等: {}", is_equal);
    }
}

/// 演示类型双关
fn demonstrate_type_punning() {
    println!("\n🎭 类型双关:");
    
    let float_value = 3.14159f32;
    let float_ptr = &float_value as *const f32;
    let int_ptr = float_ptr as *const u32;
    
    unsafe {
        let float_bits = *int_ptr;
        println!("浮点数: {}", float_value);
        println!("位表示: 0x{:08x}", float_bits);
        println!("符号位: {}", (float_bits >> 31) & 1);
        println!("指数: {}", (float_bits >> 23) & 0xff);
        println!("尾数: 0x{:06x}", float_bits & 0x7fffff);
    }
}

/// 演示未初始化内存
fn demonstrate_uninitialized_memory() {
    println!("\n🚫 未初始化内存:");
    
    unsafe {
        // 分配未初始化内存
        let layout = Layout::array::<i32>(5).unwrap();
        let ptr = alloc(layout) as *mut i32;
        
        if !ptr.is_null() {
            println!("分配了 {} 字节的未初始化内存", layout.size());
            
            // 读取未初始化内存（未定义行为！）
            // println!("未初始化值: {}", *ptr); // 危险！
            
            // 正确做法：先初始化再使用
            for i in 0..5 {
                ptr::write(ptr.add(i), i as i32 * 10);
            }
            
            println!("初始化后的值:");
            for i in 0..5 {
                println!("  [{}] = {}", i, *ptr.add(i));
            }
            
            // 释放内存
            dealloc(ptr as *mut u8, layout);
            println!("内存已释放");
        }
    }
}

/// 裸指针与集合类型
fn raw_pointer_with_collections() {
    println!("\n📌 8. 裸指针与集合类型");
    println!("{}", "-".repeat(30));

    // Vec 的内部指针
    let mut vec = vec![1, 2, 3, 4, 5];
    let vec_ptr = vec.as_ptr();
    let vec_mut_ptr = vec.as_mut_ptr();
    
    println!("Vec 内容: {:?}", vec);
    println!("Vec 数据指针: {:p}", vec_ptr);
    println!("Vec 容量: {}, 长度: {}", vec.capacity(), vec.len());
    
    unsafe {
        // 通过裸指针修改 Vec 内容
        *vec_mut_ptr.add(2) = 100;
        println!("修改后: {:?}", vec);
        
        // 从裸指针创建切片
        let slice = slice::from_raw_parts(vec_ptr, vec.len());
        println!("从指针创建的切片: {:?}", slice);
    }
    
    // 字符串的内部指针
    demonstrate_string_pointers();
}

/// 演示字符串指针
fn demonstrate_string_pointers() {
    println!("\n📝 字符串指针:");
    
    let s = String::from("Hello, Rust!");
    let str_ptr = s.as_ptr();
    let str_len = s.len();
    
    println!("字符串: \"{}\"", s);
    println!("字符串指针: {:p}", str_ptr);
    println!("字符串长度: {} 字节", str_len);
    
    unsafe {
        // 从裸指针重建字符串切片
        let reconstructed = slice::from_raw_parts(str_ptr, str_len);
        let str_slice = std::str::from_utf8_unchecked(reconstructed);
        println!("重建的字符串: \"{}\"", str_slice);
        
        // 逐字节访问
        println!("字节内容:");
        for i in 0..str_len {
            let byte = *str_ptr.add(i);
            println!("  [{}] = 0x{:02x} ('{}')", i, byte, byte as char);
        }
    }
}

/// 高级裸指针模式
fn advanced_raw_pointer_patterns() {
    println!("\n📌 9. 高级裸指针模式");
    println!("{}", "-".repeat(30));

    // 自引用结构
    demonstrate_self_referential_struct();
    
    // 指针链表
    demonstrate_pointer_linked_list();
    
    // 函数指针
    demonstrate_function_pointers();
}

/// 演示自引用结构
fn demonstrate_self_referential_struct() {
    println!("\n🔄 自引用结构:");
    
    struct SelfRef {
        data: i32,
        self_ptr: *const i32,
    }
    
    let mut self_ref = SelfRef {
        data: 42,
        self_ptr: ptr::null(),
    };
    
    // 设置自引用
    self_ref.self_ptr = &self_ref.data;
    
    println!("数据: {}", self_ref.data);
    println!("数据地址: {:p}", &self_ref.data);
    println!("自引用指针: {:p}", self_ref.self_ptr);
    
    unsafe {
        if !self_ref.self_ptr.is_null() {
            println!("通过自引用访问: {}", *self_ref.self_ptr);
        }
    }
}

/// 演示指针链表
fn demonstrate_pointer_linked_list() {
    println!("\n🔗 指针链表:");
    
    struct Node {
        data: i32,
        next: *mut Node,
    }
    
    // 创建节点
    let mut node1 = Box::new(Node { data: 1, next: ptr::null_mut() });
    let mut node2 = Box::new(Node { data: 2, next: ptr::null_mut() });
    let mut node3 = Box::new(Node { data: 3, next: ptr::null_mut() });
    
    // 建立链接
    node1.next = &mut *node2 as *mut Node;
    node2.next = &mut *node3 as *mut Node;
    
    // 遍历链表
    let mut current = &*node1 as *const Node;
    let mut count = 0;
    
    unsafe {
        while !current.is_null() && count < 10 { // 防止无限循环
            println!("节点 {}: 数据 = {}, 地址 = {:p}", count + 1, (*current).data, current);
            current = (*current).next;
            count += 1;
        }
    }
    
    // Box 会自动清理内存
}

/// 演示函数指针
fn demonstrate_function_pointers() {
    println!("\n🎯 函数指针:");
    
    fn add(a: i32, b: i32) -> i32 { a + b }
    fn multiply(a: i32, b: i32) -> i32 { a * b }
    
    // 函数指针数组
    let operations: [fn(i32, i32) -> i32; 2] = [add, multiply];
    
    for (i, op) in operations.iter().enumerate() {
        let result = op(5, 3);
        println!("操作 {}: 5 op 3 = {}", i, result);
        println!("函数地址: {:p}", *op as *const ());
    }
    
    // 原始函数指针
    let raw_fn_ptr = add as *const ();
    println!("原始函数指针: {:p}", raw_fn_ptr);
    
    // 通过原始指针调用函数（极其危险！）
    unsafe {
        let fn_ptr: fn(i32, i32) -> i32 = std::mem::transmute(raw_fn_ptr);
        let result = fn_ptr(10, 20);
        println!("通过原始指针调用: 10 + 20 = {}", result);
    }
}

/// 裸指针常见陷阱
fn raw_pointer_pitfalls() {
    println!("\n📌 10. 裸指针常见陷阱");
    println!("{}", "-".repeat(30));

    // 悬垂指针
    demonstrate_dangling_pointer();
    
    // 双重释放
    demonstrate_double_free_prevention();
    
    // 内存泄漏
    demonstrate_memory_leak_prevention();
    
    // 数据竞争
    demonstrate_data_race_issues();
}

/// 演示悬垂指针
fn demonstrate_dangling_pointer() {
    println!("\n💀 悬垂指针陷阱:");
    
    let dangling_ptr: *const i32;
    
    {
        let temp_value = 42;
        dangling_ptr = &temp_value; // 危险！temp_value 即将被销毁
        println!("临时值: {}", temp_value);
    } // temp_value 在这里被销毁
    
    // 此时 dangling_ptr 指向已释放的内存
    println!("悬垂指针地址: {:p}", dangling_ptr);
    // unsafe { println!("悬垂指针值: {}", *dangling_ptr); } // 未定义行为！
    
    println!("⚠️ 解引用悬垂指针会导致未定义行为，已注释掉");
}

/// 演示双重释放预防
fn demonstrate_double_free_prevention() {
    println!("\n🔒 双重释放预防:");
    
    let boxed = Box::new(42);
    let raw_ptr = Box::into_raw(boxed);
    
    println!("堆分配的值地址: {:p}", raw_ptr);
    
    unsafe {
        println!("值: {}", *raw_ptr);
        
        // 正确的释放方式
        let _boxed_back = Box::from_raw(raw_ptr);
        // boxed_back 在这里自动释放
        
        // 错误：再次释放会导致双重释放
        // let _another_box = Box::from_raw(raw_ptr); // 危险！
    }
    
    println!("✅ 内存已安全释放一次");
}

/// 演示内存泄漏预防
fn demonstrate_memory_leak_prevention() {
    println!("\n🚰 内存泄漏预防:");
    
    // 正确的内存管理
    {
        let boxed = Box::new(vec![1, 2, 3, 4, 5]);
        let raw_ptr = Box::into_raw(boxed);
        
        unsafe {
            println!("堆上的向量: {:?}", *raw_ptr);
            
            // 重要：必须重新包装成 Box 以确保释放
            let _boxed_back = Box::from_raw(raw_ptr);
            // 内存在这里自动释放
        }
    }
    
    println!("✅ 内存已正确释放，无泄漏");
    
    // 演示泄漏检测
    demonstrate_leak_detection();
}

/// 演示泄漏检测
fn demonstrate_leak_detection() {
    println!("\n🔍 内存泄漏检测:");
    
    // 故意泄漏内存（仅用于演示）
    let leaked_box = Box::new("This will be leaked".to_string());
    let leaked_ptr = Box::into_raw(leaked_box);
    
    println!("故意泄漏的字符串地址: {:p}", leaked_ptr);
    
    // 在实际代码中，应该这样修复：
    // unsafe {
    //     let _recovered = Box::from_raw(leaked_ptr);
    // }
    
    println!("⚠️ 演示目的故意泄漏内存，实际代码中应避免");
    
    // 使用 std::mem::forget 也会导致泄漏
    let another_box = Box::new(42);
    std::mem::forget(another_box); // 故意泄漏
    println!("⚠️ 使用 mem::forget 也会导致内存泄漏");
}

/// 演示数据竞争问题
fn demonstrate_data_race_issues() {
    println!("\n🏃 数据竞争问题:");
    
    let mut shared_data = 0;
    let ptr1 = &mut shared_data as *mut i32;
    let ptr2 = &mut shared_data as *mut i32;
    
    println!("共享数据地址: {:p}", &shared_data);
    println!("指针1: {:p}, 指针2: {:p}", ptr1, ptr2);
    
    // 在单线程中，这是安全的
    unsafe {
        *ptr1 = 10;
        *ptr2 = 20;
        println!("最终值: {}", shared_data);
    }
    
    println!("⚠️ 在多线程环境中，同时使用多个可变裸指针会导致数据竞争");
    println!("💡 解决方案: 使用 Mutex, RwLock, 或原子操作");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_raw_pointer() {
        let x = 42;
        let ptr = &x as *const i32;
        unsafe {
            assert_eq!(*ptr, 42);
        }
    }

    #[test]
    fn test_null_pointer() {
        let null_ptr: *const i32 = ptr::null();
        assert!(null_ptr.is_null());
    }

    #[test]
    fn test_pointer_arithmetic() {
        let array = [1, 2, 3, 4, 5];
        let ptr = array.as_ptr();
        
        unsafe {
            assert_eq!(*ptr, 1);
            assert_eq!(*ptr.add(2), 3);
            assert_eq!(*ptr.add(4), 5);
        }
    }

    #[test]
    fn test_pointer_casting() {
        let value = 0x12345678u32;
        let u32_ptr = &value as *const u32;
        let u8_ptr = u32_ptr as *const u8;
        
        unsafe {
            assert_eq!(*u32_ptr, 0x12345678);
            // 小端序系统中，第一个字节是最低位
            assert_eq!(*u8_ptr, 0x78);
        }
    }

    #[test]
    fn test_box_raw_conversion() {
        let boxed = Box::new(42);
        let raw_ptr = Box::into_raw(boxed);
        
        unsafe {
            assert_eq!(*raw_ptr, 42);
            let _boxed_back = Box::from_raw(raw_ptr);
        }
    }
}