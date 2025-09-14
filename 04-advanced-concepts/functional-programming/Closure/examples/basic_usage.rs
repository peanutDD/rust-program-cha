//! # 基础闭包使用示例
//!
//! 这个示例展示了闭包的基本用法，适合初学者快速上手。

use closure_tutorial::basics;

fn main() {
    println!("🚀 闭包基础使用示例");
    println!("{}", "=".repeat(40));

    // 1. 简单的闭包定义和调用
    println!("\n📝 1. 简单闭包");
    let add = |x, y| x + y;
    println!("add(2, 3) = {}", add(2, 3));

    // 2. 闭包捕获环境变量
    println!("\n📝 2. 捕获环境变量");
    let multiplier = 10;
    let multiply_by_ten = |x| x * multiplier;
    println!("multiply_by_ten(5) = {}", multiply_by_ten(5));

    // 3. 闭包作为函数参数
    println!("\n📝 3. 闭包作为参数");
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("原数组: {:?}", numbers);
    println!("翻倍后: {:?}", doubled);

    // 4. 使用 filter 过滤
    println!("\n📝 4. 过滤操作");
    let even_numbers: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    println!("偶数: {:?}", even_numbers);

    // 5. 链式操作
    println!("\n📝 5. 链式操作");
    let result: i32 = numbers.iter().filter(|&&x| x > 2).map(|x| x * x).sum();
    println!("大于2的数的平方和: {}", result);

    // 6. 运行库中的基础演示
    println!("\n📝 6. 库演示");
    basics::demonstrate();

    println!("\n✅ 基础示例完成！");
}

/// 演示不同类型的闭包
fn demonstrate_closure_types() {
    println!("\n🔍 闭包类型演示");

    // Fn - 可以多次调用，不可变借用
    let fn_closure = |x: i32| x + 1;
    println!("Fn 闭包: {} -> {}", 5, fn_closure(5));
    println!("Fn 闭包: {} -> {}", 10, fn_closure(10));

    // FnMut - 可以多次调用，可变借用
    let mut counter = 0;
    let mut fn_mut_closure = || {
        counter += 1;
        counter
    };
    println!("FnMut 闭包第1次调用: {}", fn_mut_closure());
    println!("FnMut 闭包第2次调用: {}", fn_mut_closure());

    // FnOnce - 只能调用一次，获取所有权
    let data = vec![1, 2, 3];
    let fn_once_closure = move || {
        println!("FnOnce 闭包获取了数据: {:?}", data);
        data.len()
    };
    println!("FnOnce 闭包返回长度: {}", fn_once_closure());
    // fn_once_closure(); // 这行会编译错误，因为已经被消费了
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_closure() {
        let add = |x, y| x + y;
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_capture_closure() {
        let multiplier = 10;
        let multiply = |x| x * multiplier;
        assert_eq!(multiply(5), 50);
    }

    #[test]
    fn test_iterator_with_closure() {
        let numbers = vec![1, 2, 3, 4, 5];
        let sum: i32 = numbers.iter().map(|x| x * 2).sum();
        assert_eq!(sum, 30); // (1+2+3+4+5) * 2 = 30
    }
}
