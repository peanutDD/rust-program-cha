//! # 闭包捕获机制
//!
//! 本模块深入解析 Rust 闭包的捕获机制

/// 演示闭包的三种捕获方式
///
/// 1. 不可变借用捕获
/// 2. 可变借用捕获
/// 3. 获取所有权捕获
pub fn demo_capture_modes() {
    println!("\n=== 3. 闭包的捕获方式 ===");

    // 3.1 不可变借用捕获
    demo_immutable_borrow();

    // 3.2 可变借用捕获
    demo_mutable_borrow();

    // 3.3 获取所有权捕获
    demo_ownership_capture();
}

/// 不可变借用捕获示例
fn demo_immutable_borrow() {
    println!("\n--- 3.1 不可变借用捕获 ---");
    
    let name = String::from("Alice");

    let print_name = || println!("姓名: {}", name); // 不可变借用

    print_name();
    print_name(); // 可以多次调用

    // name 仍然可以使用
    println!("原始变量仍可用: {}", name);
}

/// 可变借用捕获示例
fn demo_mutable_borrow() {
    println!("\n--- 3.2 可变借用捕获 ---");
    
    let mut counter = 0;

    let mut increment = || {
        counter += 1; // 可变借用
        println!("计数器: {}", counter);
    };

    increment();
    increment();
    increment();

    // 闭包使用完毕后，counter 可以被访问
    println!("最终计数器值: {}", counter);
}

/// 获取所有权捕获示例
fn demo_ownership_capture() {
    println!("\n--- 3.3 获取所有权捕获 ---");
    
    let data = vec![1, 2, 3, 4, 5];

    // 这个闭包会消费 data（自动推导）
    let consume_data = || {
        println!("消费数据: {:?}", data);
        data.len()
    };

    let length = consume_data();
    println!("数据长度: {}", length);

    // data 已经被移动，无法再使用
    // println!("{:?}", data); // 编译错误
}

/// 演示编译器如何实现闭包
///
/// 编译器会为每个闭包生成一个匿名结构体
pub fn demo_compiler_implementation() {
    println!("\n=== 4. 闭包的编译器实现原理 ===");

    let x = 10;
    let y = 20;

    // 这个闭包会被编译器转换为一个包含 x 和 y 的结构体
    let closure = |z| {
        println!("闭包内部访问: x={}, y={}, z={}", x, y, z);
        x + y + z
    };

    println!("闭包调用结果: {}", closure(5));

    // 展示闭包的大小（包含捕获的变量）
    println!("闭包的内存大小: {} bytes", std::mem::size_of_val(&closure));

    // 对比：不捕获变量的闭包
    let no_capture = |a: i32| a * 2;
    println!(
        "无捕获闭包的大小: {} bytes",
        std::mem::size_of_val(&no_capture)
    );

    // 深入分析内存布局
    analyze_memory_layout();
}

/// 分析不同捕获方式的内存布局
fn analyze_memory_layout() {
    println!("\n--- 内存布局分析 ---");

    let small_var = 42u8; // 1 byte
    let medium_var = 1000u32; // 4 bytes
    let large_var = [0u64; 100]; // 800 bytes

    // 只捕获小变量
    let small_closure = |x: u8| small_var + x;
    println!(
        "只捕获 u8 的闭包大小: {} bytes",
        std::mem::size_of_val(&small_closure)
    );

    // 捕获中等变量
    let medium_closure = |x: u32| medium_var + x;
    println!(
        "只捕获 u32 的闭包大小: {} bytes",
        std::mem::size_of_val(&medium_closure)
    );

    // 捕获大变量（通过引用）
    let large_closure = |x: usize| large_var[x];
    println!(
        "捕获大数组引用的闭包大小: {} bytes",
        std::mem::size_of_val(&large_closure)
    );

    // 捕获大变量（通过 move）
    let moved_closure = move |x: usize| large_var[x];
    println!(
        "move 大数组的闭包大小: {} bytes",
        std::mem::size_of_val(&moved_closure)
    );
}

/// 演示编译器的捕获推导算法
pub fn demo_capture_inference() {
    println!("\n=== 5. 编译器捕获推导算法 ===");

    let mut counter = 0;
    let immutable_data = vec![1, 2, 3, 4, 5];

    // 编译器推导：只读访问 -> 不可变借用
    let read_only = || {
        println!("只读访问: {:?}", immutable_data);
        immutable_data.len()
    };

    // 编译器推导：修改访问 -> 可变借用
    let mut modify_access = || {
        counter += 1;
        println!("修改计数器: {}", counter);
    };

    println!("只读闭包结果: {}", read_only());
    modify_access();

    // 编译器推导：所有权转移 -> move 语义
    let ownership_transfer = move || {
        let local_data = immutable_data;
        local_data.into_iter().sum::<i32>()
    };

    println!("所有权转移闭包结果: {}", ownership_transfer());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutable_capture() {
        let x = 10;
        let closure = || x + 5;
        assert_eq!(closure(), 15);
        assert_eq!(x, 10); // x 仍可用
    }

    #[test]
    fn test_mutable_capture() {
        let mut count = 0;
        let mut increment = || {
            count += 1;
            count
        };
        assert_eq!(increment(), 1);
        assert_eq!(increment(), 2);
    }

    #[test]
    fn test_ownership_capture() {
        let data = vec![1, 2, 3];
        let len = data.len();
        let closure = move || data.len();
        assert_eq!(closure(), len);
        // data 已被移动，不能再使用
    }
}

