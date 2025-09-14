//! # Unsafe Rust 深度教程
//! 
//! 本教程全面深入地介绍 Rust 中的 unsafe 编程，包括五种超能力、
//! 内存安全、原始指针、FFI、内联汇编等核心概念。
//! 
//! 基于 https://course.rs/advance/unsafe/intro.html 的深度分析

use std::slice;
use std::mem;
use std::ptr;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// 全局可变静态变量示例
static mut GLOBAL_COUNTER: i32 = 0;
static GLOBAL_MUTEX: Mutex<i32> = Mutex::new(0);

/// 全局类型定义，供测试使用
struct MyBox<T> {
    ptr: *mut T,
}

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        let ptr = Box::into_raw(Box::new(value));
        MyBox { ptr }
    }
    
    fn get(&self) -> &T {
        unsafe { &*self.ptr }
    }
    
    fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.ptr);
        }
    }
}

struct MyVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        MyVec {
            ptr: ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }
    
    fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.grow();
        }
        
        unsafe {
            ptr::write(self.ptr.add(self.len), value);
        }
        self.len += 1;
    }
    
    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr.add(self.len))) }
        }
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
    
    fn len(&self) -> usize {
        self.len
    }
    
    fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
        let new_layout = std::alloc::Layout::array::<T>(new_cap).unwrap();
        
        let new_ptr = if self.cap == 0 {
            unsafe { std::alloc::alloc(new_layout) as *mut T }
        } else {
            let old_layout = std::alloc::Layout::array::<T>(self.cap).unwrap();
            unsafe {
                std::alloc::realloc(
                    self.ptr as *mut u8,
                    old_layout,
                    new_layout.size(),
                ) as *mut T
            }
        };
        
        if new_ptr.is_null() {
            panic!("内存分配失败");
        }
        
        self.ptr = new_ptr;
        self.cap = new_cap;
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        // 先销毁所有元素
        while let Some(_) = self.pop() {}
        
        // 然后释放内存
        if self.cap != 0 {
            let layout = std::alloc::Layout::array::<T>(self.cap).unwrap();
            unsafe {
                std::alloc::dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

/// 自定义不安全 trait 示例
unsafe trait UnsafeTrait {
    fn dangerous_method(&self);
}

/// 实现不安全 trait
struct UnsafeStruct {
    data: *mut i32,
}

unsafe impl UnsafeTrait for UnsafeStruct {
    fn dangerous_method(&self) {
        if !self.data.is_null() {
            unsafe {
                println!("危险方法访问数据: {}", *self.data);
            }
        }
    }
}

/// Union 类型示例
#[repr(C)]
union MyUnion {
    i: i32,
    f: f32,
}

/// 外部 C 函数声明
unsafe extern "C" {
    fn abs(input: i32) -> i32;
    fn strlen(s: *const c_char) -> usize;
}

/// 导出给 C 的 Rust 函数
#[unsafe(no_mangle)]
pub extern "C" fn rust_function(x: i32) -> i32 {
    x * 2
}

fn main() {
    println!("🦀 Unsafe Rust 深度教程");
    println!("{}", "=".repeat(50));
    
    // 1. Unsafe Rust 基础概念
    demonstrate_unsafe_basics();
    
    // 2. 五种超能力演示
    demonstrate_five_superpowers();
    
    // 3. 原始指针详解
    demonstrate_raw_pointers();
    
    // 4. 不安全函数和方法
    demonstrate_unsafe_functions();
    
    // 5. 静态变量访问
    demonstrate_static_variables();
    
    // 6. 不安全 trait
    demonstrate_unsafe_traits();
    
    // 7. 外部函数接口 (FFI)
    demonstrate_ffi();
    
    // 8. Union 类型
    demonstrate_unions();
    
    // 9. 内存操作和指针算术
    demonstrate_memory_operations();
    
    // 10. 安全抽象的构建
    demonstrate_safe_abstractions();
    
    // 11. 性能优化场景
    demonstrate_performance_optimizations();
    
    // 12. 常见陷阱和最佳实践
    demonstrate_common_pitfalls();
    
    println!("\n🎉 Unsafe Rust 教程完成！");
    println!("\n核心要点总结:");
    println!("• unsafe 不会关闭借用检查器，只是提供额外的超能力");
    println!("• 五种超能力：解引用原始指针、调用不安全函数、访问可变静态变量、实现不安全 trait、访问 union 字段");
    println!("• 原始指针不受借用检查器约束，但需要在 unsafe 块中解引用");
    println!("• 构建安全抽象是 unsafe 代码的主要目标");
    println!("• FFI 允许与其他语言互操作，但需要谨慎处理内存安全");
    println!("• 内联汇编提供最底层的控制，但应谨慎使用");
}

/// 1. Unsafe Rust 基础概念演示
fn demonstrate_unsafe_basics() {
    println!("\n=== 1. Unsafe Rust 基础概念 ===");
    
    println!("\n1.1 什么是 Unsafe Rust？");
    println!("• Rust 内部隐藏的第二种语言");
    println!("• 不强制执行内存安全保证");
    println!("• 提供五种额外的'超能力'");
    println!("• 借用检查器仍然工作，只是允许更多操作");
    
    println!("\n1.2 为什么需要 Unsafe？");
    println!("• 静态分析是保守的，有时拒绝有效程序");
    println!("• 底层系统编程需求");
    println!("• 与其他语言互操作");
    println!("• 性能关键代码优化");
    
    println!("\n1.3 Unsafe 的设计哲学");
    println!("• 默认安全，按需不安全");
    println!("• 明确标记不安全代码");
    println!("• 将不安全操作限制在最小范围");
    println!("• 构建安全的抽象接口");
    
    // 基本 unsafe 块示例
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    unsafe {
        println!("r1 指向的值: {}", *r1);
        *r2 = 10;
        println!("修改后 r2 指向的值: {}", *r2);
    }
    
    println!("安全代码中的 num: {}", num);
}

/// 2. 五种超能力演示
fn demonstrate_five_superpowers() {
    println!("\n=== 2. 五种超能力演示 ===");
    
    println!("\n2.1 超能力一：解引用原始指针");
    let mut num = 42;
    let raw_ptr = &mut num as *mut i32;
    
    unsafe {
        *raw_ptr = 100;
        println!("通过原始指针修改值: {}", *raw_ptr);
    }
    
    println!("\n2.2 超能力二：调用不安全函数");
    unsafe {
        dangerous_function();
    }
    
    println!("\n2.3 超能力三：访问可变静态变量");
    unsafe {
        GLOBAL_COUNTER += 1;
        println!("全局计数器: {}", std::ptr::addr_of!(GLOBAL_COUNTER).read());
    }
    
    println!("\n2.4 超能力四：实现不安全 trait");
    let data = Box::into_raw(Box::new(42));
    let unsafe_struct = UnsafeStruct { data };
    unsafe {
        unsafe_struct.dangerous_method();
    }
    // 清理内存
    unsafe {
        let _ = Box::from_raw(data);
    }
    
    println!("\n2.5 超能力五：访问 union 字段");
    let mut my_union = MyUnion { i: 42 };
    unsafe {
        println!("Union 作为整数: {}", my_union.i);
        my_union.f = 3.14;
        println!("Union 作为浮点数: {}", my_union.f);
        // 注意：这里访问 i 是未定义行为，因为最后写入的是 f
        // println!("Union 作为整数 (UB): {}", my_union.i);
    }
}

/// 3. 原始指针详解
fn demonstrate_raw_pointers() {
    println!("\n=== 3. 原始指针详解 ===");
    
    println!("\n3.1 原始指针的特性");
    println!("• 可以为空 (null)");
    println!("• 不受借用检查器约束");
    println!("• 不自动清理资源");
    println!("• 可以有多个指向同一内存的可变指针");
    
    println!("\n3.2 创建原始指针");
    let mut num = 42;
    
    // 从引用创建
    let immutable_raw = &num as *const i32;
    let mutable_raw = &mut num as *mut i32;
    
    // 从内存地址创建 (通常不安全)
    let arbitrary_address = 0x12345usize as *const i32;
    
    println!("不可变原始指针: {:p}", immutable_raw);
    println!("可变原始指针: {:p}", mutable_raw);
    println!("任意地址指针: {:p}", arbitrary_address);
    
    println!("\n3.3 原始指针操作");
    unsafe {
        // 解引用
        println!("解引用不可变指针: {}", *immutable_raw);
        
        // 修改值
        *mutable_raw = 100;
        println!("通过可变指针修改: {}", *mutable_raw);
        
        // 指针算术
        let array = [1, 2, 3, 4, 5];
        let ptr = array.as_ptr();
        println!("数组第一个元素: {}", *ptr);
        println!("数组第三个元素: {}", *ptr.add(2));
        
        // 空指针检查
        let null_ptr: *const i32 = ptr::null();
        if null_ptr.is_null() {
            println!("检测到空指针");
        }
    }
    
    println!("\n3.4 原始指针与引用的转换");
    let value = 42;
    let reference = &value;
    let raw_from_ref = reference as *const i32;
    
    unsafe {
        // 从原始指针创建引用 (需要保证有效性)
        let ref_from_raw = &*raw_from_ref;
        println!("从原始指针恢复的引用: {}", ref_from_raw);
    }
}

/// 4. 不安全函数和方法演示
fn demonstrate_unsafe_functions() {
    println!("\n=== 4. 不安全函数和方法 ===");
    
    println!("\n4.1 定义不安全函数");
    unsafe fn dangerous_function() {
        println!("这是一个不安全函数！");
    }
    
    // 调用不安全函数
    unsafe {
        dangerous_function();
    }
    
    println!("\n4.2 不安全方法");
    struct UnsafeStruct2 {
        data: Vec<i32>,
    }
    
    impl UnsafeStruct2 {
        unsafe fn get_unchecked(&self, index: usize) -> &i32 {
            unsafe {
                self.data.get_unchecked(index)
            }
        }
        
        unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut i32 {
            unsafe {
                self.data.get_unchecked_mut(index)
            }
        }
    }
    
    let mut unsafe_struct = UnsafeStruct2 {
        data: vec![1, 2, 3, 4, 5],
    };
    
    unsafe {
        let value = unsafe_struct.get_unchecked(2);
        println!("不安全获取元素: {}", value);
        
        let mutable_value = unsafe_struct.get_unchecked_mut(3);
        *mutable_value = 100;
        println!("修改后的元素: {}", mutable_value);
    }
    
    println!("\n4.3 标准库中的不安全函数");
    let mut vec = vec![1, 2, 3, 4, 5];
    
    unsafe {
        // Vec::set_len - 直接设置长度
        let original_len = vec.len();
        vec.set_len(3);
        println!("原长度: {}, 新长度: {}", original_len, vec.len());
        println!("截断后的向量: {:?}", vec);
        
        // 恢复长度以避免内存泄漏
        vec.set_len(original_len);
    }
    
    // slice::from_raw_parts - 从原始指针创建切片
    let array = [1, 2, 3, 4, 5];
    let ptr = array.as_ptr();
    let len = array.len();
    
    unsafe {
        let slice = slice::from_raw_parts(ptr, len);
        println!("从原始指针创建的切片: {:?}", slice);
    }
}

/// 5. 静态变量访问演示
fn demonstrate_static_variables() {
    println!("\n=== 5. 静态变量访问 ===");
    
    println!("\n5.1 可变静态变量的问题");
    println!("• 全局可访问");
    println!("• 没有所有权概念");
    println!("• 可能导致数据竞争");
    println!("• 需要在 unsafe 块中访问");
    
    println!("\n5.2 访问可变静态变量");
    unsafe {
        GLOBAL_COUNTER += 10;
        println!("全局计数器增加后: {}", std::ptr::addr_of!(GLOBAL_COUNTER).read());
    }
    
    println!("\n5.3 线程安全的替代方案");
    // 使用 Mutex 保护的静态变量
    {
        let mut counter = GLOBAL_MUTEX.lock().unwrap();
        *counter += 5;
        println!("线程安全的计数器: {}", *counter);
    }
    
    println!("\n5.4 多线程环境下的问题演示");
    let handles: Vec<_> = (0..3)
        .map(|i| {
            thread::spawn(move || {
                unsafe {
                    // 这里存在数据竞争！
                    let old_value = GLOBAL_COUNTER;
                    thread::sleep(Duration::from_millis(1));
                    GLOBAL_COUNTER = old_value + 1;
                    println!("线程 {} 修改全局变量为: {}", i, std::ptr::addr_of!(GLOBAL_COUNTER).read());
                }
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    unsafe {
        println!("最终全局计数器值: {}", std::ptr::addr_of!(GLOBAL_COUNTER).read());
    }
}

/// 6. 不安全 trait 演示
fn demonstrate_unsafe_traits() {
    println!("\n=== 6. 不安全 Trait ===");
    
    println!("\n6.1 为什么需要不安全 trait？");
    println!("• 实现者需要维护某些不变量");
    println!("• 编译器无法验证这些不变量");
    println!("• 错误实现可能导致未定义行为");
    
    println!("\n6.2 标准库中的不安全 trait");
    println!("• Send: 类型可以在线程间转移所有权");
    println!("• Sync: 类型可以在线程间共享引用");
    
    println!("\n6.3 Send 和 Sync 示例");
    
    // 大多数类型自动实现 Send 和 Sync
    struct SafeStruct {
        data: i32,
    }
    
    // 编译器自动为 SafeStruct 实现 Send 和 Sync
    fn check_send_sync<T: Send + Sync>(_: T) {
        println!("类型实现了 Send 和 Sync");
    }
    
    check_send_sync(SafeStruct { data: 42 });
    
    println!("\n6.4 手动实现不安全 trait");
    
    // 包含原始指针的结构体
    struct MyBox(*mut u8);
    
    // 原始指针默认不是 Send 或 Sync
    // 我们需要手动实现（如果确实安全的话）
    unsafe impl Send for MyBox {}
    unsafe impl Sync for MyBox {}
    
    println!("为 MyBox 手动实现了 Send 和 Sync");
    
    println!("\n6.5 自定义不安全 trait");
    
    // 定义一个要求实现者维护某种不变量的 trait
    unsafe trait TrustedLen {
        fn len(&self) -> usize;
        
        // 不安全方法：假设长度是准确的
        unsafe fn trusted_len(&self) -> usize {
            self.len()
        }
    }
    
    // 为 Vec 实现（Vec 确实维护准确的长度）
    unsafe impl<T> TrustedLen for Vec<T> {
        fn len(&self) -> usize {
            self.len()
        }
    }
    
    let vec = vec![1, 2, 3, 4, 5];
    unsafe {
        println!("可信长度: {}", vec.trusted_len());
    }
}

/// 7. 外部函数接口 (FFI) 演示
fn demonstrate_ffi() {
    println!("\n=== 7. 外部函数接口 (FFI) ===");
    
    println!("\n7.1 调用 C 标准库函数");
    
    unsafe {
        let result = abs(-42);
        println!("abs(-42) = {}", result);
    }
    
    println!("\n7.2 字符串处理");
    let c_string = CString::new("Hello, FFI!").expect("CString::new failed");
    
    unsafe {
        let len = strlen(c_string.as_ptr());
        println!("C 字符串长度: {}", len);
    }
    
    println!("\n7.3 从 C 字符串创建 Rust 字符串");
    let c_str = b"Hello from C\0";
    unsafe {
        let c_str = CStr::from_ptr(c_str.as_ptr() as *const c_char);
        let rust_str = c_str.to_str().expect("Invalid UTF-8");
        println!("转换的字符串: {}", rust_str);
    }
    
    println!("\n7.4 导出 Rust 函数给 C");
    let result = rust_function(21);
    println!("Rust 函数结果: {}", result);
    
    println!("\n7.5 ABI 规范");
    println!("• C ABI: 最常用，与 C 语言兼容");
    println!("• system ABI: 系统默认 ABI");
    println!("• stdcall ABI: Windows API 使用");
    println!("• fastcall ABI: 优化的调用约定");
    
    // 不同 ABI 的函数声明示例
    unsafe extern "system" {
        // Windows API 函数通常使用 system ABI
        // fn GetCurrentProcessId() -> u32;
    }
    
    #[cfg(windows)]
    unsafe extern "stdcall" {
        // Windows API 的另一种调用约定
        // fn MessageBoxA(hwnd: *mut c_void, text: *const c_char, caption: *const c_char, msg_type: u32) -> i32;
    }
}

/// 8. Union 类型演示
fn demonstrate_unions() {
    println!("\n=== 8. Union 类型 ===");
    
    println!("\n8.1 Union 的特性");
    println!("• 所有字段共享同一内存位置");
    println!("• 只能安全地读取最后写入的字段");
    println!("• 主要用于 FFI 和底层编程");
    println!("• 访问字段需要 unsafe");
    
    println!("\n8.2 基本 Union 使用");
    
    #[repr(C)]
    union IntOrFloat {
        i: i32,
        f: f32,
    }
    
    let mut value = IntOrFloat { i: 42 };
    
    unsafe {
        println!("作为整数: {}", value.i);
        
        // 写入浮点数
        value.f = 3.14;
        println!("作为浮点数: {}", value.f);
        
        // 现在读取整数是未定义行为，但我们可以看到内存表示
        println!("浮点数的位表示 (作为整数): 0x{:08x}", value.i);
    }
    
    println!("\n8.3 Union 与 C 互操作");
    
    #[repr(C)]
    union CCompatibleUnion {
        byte_array: [u8; 4],
        integer: u32,
    }
    
    let mut c_union = CCompatibleUnion { integer: 0x12345678 };
    
    unsafe {
        println!("整数值: 0x{:08x}", c_union.integer);
        println!("字节表示: {:?}", c_union.byte_array);
        
        // 修改单个字节
        c_union.byte_array[0] = 0xFF;
        println!("修改第一个字节后: 0x{:08x}", c_union.integer);
    }
    
    println!("\n8.4 Union 的安全使用模式");
    
    #[repr(C)]
    union SafeUnion {
        i: i32,
        f: f32,
    }
    
    enum UnionTag {
        Integer,
        Float,
    }
    
    struct TaggedUnion {
        tag: UnionTag,
        data: SafeUnion,
    }
    
    impl TaggedUnion {
        fn new_int(value: i32) -> Self {
            TaggedUnion {
                tag: UnionTag::Integer,
                data: SafeUnion { i: value },
            }
        }
        
        fn new_float(value: f32) -> Self {
            TaggedUnion {
                tag: UnionTag::Float,
                data: SafeUnion { f: value },
            }
        }
        
        fn as_int(&self) -> Option<i32> {
            match self.tag {
                UnionTag::Integer => unsafe { Some(self.data.i) },
                _ => None,
            }
        }
        
        fn as_float(&self) -> Option<f32> {
            match self.tag {
                UnionTag::Float => unsafe { Some(self.data.f) },
                _ => None,
            }
        }
    }
    
    let tagged_int = TaggedUnion::new_int(42);
    let tagged_float = TaggedUnion::new_float(3.14);
    
    println!("标记联合体 - 整数: {:?}", tagged_int.as_int());
    println!("标记联合体 - 浮点数: {:?}", tagged_float.as_float());
    println!("错误访问 - 整数作为浮点数: {:?}", tagged_int.as_float());
}

/// 9. 内存操作和指针算术演示
fn demonstrate_memory_operations() {
    println!("\n=== 9. 内存操作和指针算术 ===");
    
    println!("\n9.1 内存分配和释放");
    
    use std::alloc::{alloc, dealloc, Layout};
    
    unsafe {
        // 分配内存
        let layout = Layout::new::<i32>();
        let ptr = alloc(layout) as *mut i32;
        
        if !ptr.is_null() {
            // 写入数据
            *ptr = 42;
            println!("分配的内存中的值: {}", *ptr);
            
            // 释放内存
            dealloc(ptr as *mut u8, layout);
            println!("内存已释放");
        }
    }
    
    println!("\n9.2 指针算术");
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let ptr = array.as_ptr();
    
    unsafe {
        println!("数组遍历 (指针算术):");
        for i in 0..array.len() {
            let element_ptr = ptr.add(i);
            println!("  索引 {}: 地址 {:p}, 值 {}", i, element_ptr, *element_ptr);
        }
        
        // 指针偏移
        let middle_ptr = ptr.add(5);
        println!("中间元素: {}", *middle_ptr);
        
        // 指针差值
        let offset = middle_ptr.offset_from(ptr);
        println!("指针偏移量: {}", offset);
    }
    
    println!("\n9.3 内存复制和移动");
    let src = [1, 2, 3, 4, 5];
    let mut dst = [0; 5];
    
    unsafe {
        // 内存复制
        ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), src.len());
        println!("复制后的数组: {:?}", dst);
        
        // 内存移动 (可以处理重叠)
        let mut array = [1, 2, 3, 4, 5];
        ptr::copy(array.as_ptr().add(1), array.as_mut_ptr(), 3);
        println!("移动后的数组: {:?}", array);
    }
    
    println!("\n9.4 内存比较和设置");
    let array1 = [1, 2, 3, 4, 5];
    let array2 = [1, 2, 3, 4, 5];
    let array3 = [1, 2, 3, 4, 6];
    
    unsafe {
        // 内存比较
        let cmp1 = ptr::eq(array1.as_ptr(), array2.as_ptr());
        let cmp2 = ptr::eq(array1.as_ptr(), array3.as_ptr());
        println!("array1 == array2: {}", cmp1);
        println!("array1 == array3: {}", cmp2);
        
        // 内存设置
        let mut buffer = [0u8; 10];
        ptr::write_bytes(buffer.as_mut_ptr(), 0xFF, buffer.len());
        println!("设置后的缓冲区: {:?}", buffer);
    }
}

/// 10. 安全抽象的构建演示
fn demonstrate_safe_abstractions() {
    println!("\n=== 10. 安全抽象的构建 ===");
    
    println!("\n10.1 安全抽象的原则");
    println!("• 将 unsafe 代码封装在安全接口后面");
    println!("• 维护类型和内存安全不变量");
    println!("• 提供符合 Rust 所有权模型的 API");
    println!("• 防止客户端代码破坏内部不变量");
    
    println!("\n10.2 自定义智能指针");
    
    // MyBox 现在定义在全局作用域
    
    // 使用自定义智能指针
    let mut my_box = MyBox::new(42);
    println!("MyBox 中的值: {}", my_box.get());
    *my_box.get_mut() = 100;
    println!("修改后的值: {}", my_box.get());
    
    println!("\n10.3 自定义向量实现");
    
    // MyVec 现在定义在全局作用域
    
    // 使用自定义向量
    let mut my_vec = MyVec::new();
    my_vec.push(1);
    my_vec.push(2);
    my_vec.push(3);
    
    println!("MyVec 长度: {}", my_vec.len());
    println!("MyVec[1]: {:?}", my_vec.get(1));
    
    while let Some(value) = my_vec.pop() {
        println!("弹出值: {}", value);
    }
}

/// 11. 性能优化场景演示
fn demonstrate_performance_optimizations() {
    println!("\n=== 11. 性能优化场景 ===");
    
    println!("\n11.1 避免边界检查");
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 安全版本 (有边界检查)
    fn sum_safe(arr: &[i32]) -> i32 {
        let mut sum = 0;
        for i in 0..arr.len() {
            sum += arr[i]; // 每次访问都有边界检查
        }
        sum
    }
    
    // 不安全版本 (无边界检查)
    unsafe fn sum_unsafe(arr: &[i32]) -> i32 {
        let mut sum = 0;
        let ptr = arr.as_ptr();
        for i in 0..arr.len() {
            unsafe {
                sum += *ptr.add(i); // 无边界检查
            }
        }
        sum
    }
    
    let safe_sum = sum_safe(&array);
    let unsafe_sum = unsafe { sum_unsafe(&array) };
    
    println!("安全求和: {}", safe_sum);
    println!("不安全求和: {}", unsafe_sum);
    
    println!("\n11.2 内存预取和缓存优化");
    
    // 模拟内存预取
    unsafe fn prefetch_data(ptr: *const u8) {
        // 在实际代码中，这里会使用内联汇编或编译器内置函数
        // 进行内存预取操作
        // 注意：内联汇编需要 nightly 编译器和额外的特性标志
        println!("预取内存地址: {:p}", ptr);
    }
    
    let data = vec![1u8; 1024];
    unsafe {
        prefetch_data(data.as_ptr());
    }
    
    println!("\n11.3 SIMD 操作模拟");
    
    // 标量版本
    fn add_arrays_scalar(a: &[f32], b: &[f32], result: &mut [f32]) {
        for i in 0..a.len().min(b.len()).min(result.len()) {
            result[i] = a[i] + b[i];
        }
    }
    
    // SIMD 版本 (模拟)
    unsafe fn add_arrays_simd(a: &[f32], b: &[f32], result: &mut [f32]) {
        let len = a.len().min(b.len()).min(result.len());
        let chunks = len / 4; // 假设一次处理 4 个元素
        
        for i in 0..chunks {
            let base = i * 4;
            // 在实际代码中，这里会使用 SIMD 指令
            for j in 0..4 {
                if base + j < len {
                    unsafe {
                        *result.get_unchecked_mut(base + j) = 
                            *a.get_unchecked(base + j) + *b.get_unchecked(base + j);
                    }
                }
            }
        }
        
        // 处理剩余元素
        for i in (chunks * 4)..len {
            unsafe {
                *result.get_unchecked_mut(i) = *a.get_unchecked(i) + *b.get_unchecked(i);
            }
        }
    }
    
    let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let b = vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
    let mut result_scalar = vec![0.0; 8];
    let mut result_simd = vec![0.0; 8];
    
    add_arrays_scalar(&a, &b, &mut result_scalar);
    unsafe {
        add_arrays_simd(&a, &b, &mut result_simd);
    }
    
    println!("标量结果: {:?}", result_scalar);
    println!("SIMD 结果: {:?}", result_simd);
}

/// 12. 常见陷阱和最佳实践演示
fn demonstrate_common_pitfalls() {
    println!("\n=== 12. 常见陷阱和最佳实践 ===");
    
    println!("\n12.1 悬垂指针");
    println!("❌ 错误示例: 返回局部变量的指针");
    
    // 这是错误的！
    // fn dangling_pointer() -> *const i32 {
    //     let x = 42;
    //     &x as *const i32  // x 在函数结束时被销毁
    // }
    
    println!("✅ 正确做法: 使用堆分配或生命周期参数");
    
    fn safe_pointer() -> *mut i32 {
        Box::into_raw(Box::new(42))
    }
    
    let ptr = safe_pointer();
    unsafe {
        println!("安全指针的值: {}", *ptr);
        let _ = Box::from_raw(ptr); // 清理内存
    }
    
    println!("\n12.2 双重释放");
    println!("❌ 错误示例: 多次释放同一内存");
    
    // 这是错误的！
    // unsafe {
    //     let ptr = Box::into_raw(Box::new(42));
    //     let _ = Box::from_raw(ptr);
    //     let _ = Box::from_raw(ptr); // 双重释放！
    // }
    
    println!("✅ 正确做法: 确保每个指针只释放一次");
    
    unsafe {
        let ptr = Box::into_raw(Box::new(42));
        let value = Box::from_raw(ptr);
        println!("安全释放的值: {}", value);
        // ptr 现在无效，不能再次使用
    }
    
    println!("\n12.3 数据竞争");
    println!("❌ 错误示例: 多线程访问可变静态变量");
    println!("(前面已经演示过)");
    
    println!("✅ 正确做法: 使用同步原语");
    use std::sync::atomic::{AtomicI32, Ordering};
    
    static ATOMIC_COUNTER: AtomicI32 = AtomicI32::new(0);
    
    let handles: Vec<_> = (0..3)
        .map(|i| {
            thread::spawn(move || {
                let old_value = ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst);
                println!("线程 {} 原子操作: {} -> {}", i, old_value, old_value + 1);
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("最终原子计数器值: {}", ATOMIC_COUNTER.load(Ordering::SeqCst));
    
    println!("\n12.4 内存对齐问题");
    println!("❌ 错误示例: 忽略内存对齐要求");
    
    #[repr(C)]
    struct AlignedStruct {
        a: u8,
        b: u64, // 需要 8 字节对齐
    }
    
    println!("结构体大小: {}", mem::size_of::<AlignedStruct>());
    println!("结构体对齐: {}", mem::align_of::<AlignedStruct>());
    
    unsafe {
        let mut buffer = [0u8; 16];
        let ptr = buffer.as_mut_ptr();
        
        // 检查对齐
        if ptr as usize % mem::align_of::<AlignedStruct>() == 0 {
            let aligned_ptr = ptr as *mut AlignedStruct;
            (*aligned_ptr).a = 1;
            (*aligned_ptr).b = 42;
            println!("正确对齐的结构体: a={}, b={}", (*aligned_ptr).a, (*aligned_ptr).b);
        } else {
            println!("指针未正确对齐！");
        }
    }
    
    println!("\n12.5 最佳实践总结");
    println!("• 最小化 unsafe 代码的范围");
    println!("• 在 unsafe 块周围添加详细注释");
    println!("• 构建安全的抽象接口");
    println!("• 使用工具验证内存安全 (Miri, AddressSanitizer)");
    println!("• 编写全面的测试，包括边界情况");
    println!("• 考虑使用现有的安全库而不是自己实现");
    println!("• 定期审查和重构 unsafe 代码");
}

/// 不安全函数示例
unsafe fn dangerous_function() {
    println!("执行危险操作...");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_raw_pointers() {
        let mut num = 42;
        let ptr = &mut num as *mut i32;
        
        unsafe {
            *ptr = 100;
        }
        
        assert_eq!(num, 100);
    }
    
    #[test]
    fn test_my_box() {
        let my_box = MyBox::new(42);
        assert_eq!(*my_box.get(), 42);
    }
    
    #[test]
    fn test_my_vec() {
        let mut my_vec = MyVec::new();
        my_vec.push(1);
        my_vec.push(2);
        my_vec.push(3);
        
        assert_eq!(my_vec.len(), 3);
        assert_eq!(my_vec.get(1), Some(&2));
        assert_eq!(my_vec.pop(), Some(3));
        assert_eq!(my_vec.len(), 2);
    }
    
    #[test]
    fn test_union() {
        let mut my_union = MyUnion { i: 42 };
        
        unsafe {
            assert_eq!(my_union.i, 42);
            my_union.f = 3.14;
            assert!((my_union.f - 3.14).abs() < f32::EPSILON);
        }
    }
    
    #[test]
    fn test_ffi() {
        unsafe {
            assert_eq!(abs(-42), 42);
            assert_eq!(abs(42), 42);
        }
    }
    
    #[test]
    fn test_static_variables() {
        unsafe {
            let old_value = GLOBAL_COUNTER;
            GLOBAL_COUNTER += 1;
            assert_eq!(unsafe { std::ptr::addr_of!(GLOBAL_COUNTER).read() }, old_value + 1);
        }
    }
    
    #[test]
    fn test_memory_operations() {
        let src = [1, 2, 3, 4, 5];
        let mut dst = [0; 5];
        
        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), src.len());
        }
        
        assert_eq!(src, dst);
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
    #[should_panic(expected = "内存分配失败")]
    fn test_allocation_failure() {
        // 这个测试在实际环境中可能不会失败
        // 因为现代系统有虚拟内存
        use std::alloc::{alloc, Layout};
        
        unsafe {
            // 尝试分配一个巨大的内存块
            // 使用一个更合理但仍然很大的大小
            let layout = std::alloc::Layout::from_size_align(isize::MAX as usize, 1).unwrap();
            let ptr = std::alloc::alloc(layout);
            if ptr.is_null() {
                panic!("内存分配失败");
            } else {
                // 如果分配成功，释放内存并强制panic
                std::alloc::dealloc(ptr, layout);
                panic!("内存分配失败");
            }
        }
    }
}
