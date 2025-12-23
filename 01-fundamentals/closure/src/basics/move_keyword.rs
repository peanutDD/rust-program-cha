//! # move 关键字
//!
//! 本模块详细介绍 move 关键字的用法和使用场景

use std::thread;

/// 演示 move 关键字的基本用法
///
/// `move` 关键字强制闭包获取被捕获变量的所有权
pub fn demo_move_keyword() {
    println!("\n=== 6. move 关键字 ===");

    // 6.1 不使用 move
    demo_without_move();

    // 6.2 使用 move
    demo_with_move();

    // 6.3 move 与 Copy 类型
    demo_move_with_copy();
}

/// 不使用 move 的示例
fn demo_without_move() {
    println!("\n--- 6.1 不使用 move ---");
    
    let x = vec![1, 2, 3];

    let closure = || println!("向量: {:?}", x); // 借用

    closure();
    println!("原始向量仍可用: {:?}", x);
}

/// 使用 move 的示例
fn demo_with_move() {
    println!("\n--- 6.2 使用 move ---");
    
    let x = vec![1, 2, 3];

    let closure = move || println!("向量: {:?}", x); // 移动所有权

    closure();
    // println!("{:?}", x); // 编译错误：x 已被移动
    println!("x 已被移动到闭包中，无法再使用");
}

/// move 与 Copy 类型的交互
fn demo_move_with_copy() {
    println!("\n--- 6.3 move 与 Copy 类型 ---");
    
    let x = 42; // i32 实现了 Copy

    let closure = move || println!("数字: {}", x); // 复制而非移动

    closure();
    println!("原始数字仍可用: {}", x); // Copy 类型不会被移动

    // 演示多个闭包捕获同一个 Copy 类型
    demonstrate_multiple_closures_with_copy();
}

/// 演示多个闭包捕获同一个 Copy 类型
fn demonstrate_multiple_closures_with_copy() {
    println!("\n多个闭包捕获 Copy 类型:");

    let number = 100;

    let closure1 = move || println!("闭包1: {}", number);
    let closure2 = move || println!("闭包2: {}", number);

    closure1();
    closure2();
    println!("原始值仍可用: {}", number);
}

/// 演示 move 在多线程中的应用
///
/// 在多线程环境中，move 关键字是必需的
pub fn demo_move_with_threads() {
    println!("\n=== 7. move 在多线程中的应用 ===");

    demo_thread_basic();
    demo_thread_multiple();
}

/// 基本的线程使用示例
fn demo_thread_basic() {
    println!("\n--- 7.1 基本线程使用 ---");
    
    let data = vec![1, 2, 3, 4, 5];

    let handle = thread::spawn(move || {
        println!("线程中的数据: {:?}", data);
        data.iter().sum::<i32>()
    });

    let sum = handle.join().unwrap();
    println!("线程计算结果: {}", sum);

    // data 已被移动到线程中，这里无法使用
    // println!("{:?}", data); // 编译错误
}

/// 多个线程的示例
fn demo_thread_multiple() {
    println!("\n--- 7.2 多个线程示例 ---");

    let data1 = vec![1, 2, 3];
    let data2 = vec![4, 5, 6];
    let data3 = vec![7, 8, 9];

    let handle1 = thread::spawn(move || data1.iter().sum::<i32>());
    let handle2 = thread::spawn(move || data2.iter().sum::<i32>());
    let handle3 = thread::spawn(move || data3.iter().sum::<i32>());

    let sum1 = handle1.join().unwrap();
    let sum2 = handle2.join().unwrap();
    let sum3 = handle3.join().unwrap();

    println!("线程1结果: {}", sum1);
    println!("线程2结果: {}", sum2);
    println!("线程3结果: {}", sum3);
    println!("总和: {}", sum1 + sum2 + sum3);
}

/// 演示何时需要 move，何时不需要
pub fn demo_when_to_use_move() {
    println!("\n=== 8. 何时使用 move ===");

    println!("\n需要使用 move 的情况:");
    println!("1. 在多线程中使用闭包");
    println!("2. 闭包的生命周期超过被捕获变量的作用域");
    println!("3. 需要转移所有权而非借用");

    println!("\n不需要使用 move 的情况:");
    println!("1. 只需要短期借用变量");
    println!("2. 变量是 Copy 类型且不关心所有权");
    println!("3. 闭包在变量作用域内使用");

    // 实际示例
    demonstrate_move_scenarios();
}

/// 演示不同的 move 使用场景
fn demonstrate_move_scenarios() {
    println!("\n--- move 使用场景演示 ---");

    // 场景1：闭包作为返回值（必须使用 move）
    let multiplier = create_multiplier(5);
    println!("场景1 - 返回闭包: {}", multiplier(10));

    // 场景2：短期使用（不需要 move）
    let data = vec![1, 2, 3];
    let sum = calculate_sum(&data, || data.iter().sum::<i32>());
    println!("场景2 - 短期借用: {}", sum);
    println!("data 仍可用: {:?}", data);
}

/// 创建乘法器闭包（返回值场景）
fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

/// 计算总和（短期借用场景）
fn calculate_sum<F>(data: &[i32], f: F) -> i32
where
    F: FnOnce() -> i32,
{
    let _ = data; // 使用 data 参数
    f()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_with_copy() {
        let x = 42;
        let closure = move || x + 1;
        assert_eq!(closure(), 43);
        assert_eq!(x, 42); // Copy 类型仍可用
    }

    #[test]
    fn test_move_with_vec() {
        let vec = vec![1, 2, 3];
        let len = vec.len();
        let closure = move || vec.len();
        assert_eq!(closure(), len);
        // vec 已被移动
    }

    #[test]
    fn test_thread_with_move() {
        let data = vec![1, 2, 3];
        let handle = thread::spawn(move || data.iter().sum::<i32>());
        let sum = handle.join().unwrap();
        assert_eq!(sum, 6);
    }
}

