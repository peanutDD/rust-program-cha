//! # 基础概念练习
//!
//! 完成这些练习来掌握闭包的基础知识

/// 练习 1: 创建一个简单的闭包
///
/// 任务：创建一个闭包，接受两个整数并返回它们的和
#[allow(dead_code)]
fn exercise_1() {
    // TODO: 在这里创建闭包
    let add = |x, y| x + y;
    
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(10, 20), 30);
    println!("✅ 练习 1 完成！");
}

/// 练习 2: 捕获环境变量
///
/// 任务：创建一个闭包，捕获外部变量并在闭包中使用
#[allow(dead_code)]
fn exercise_2() {
    let multiplier = 3;
    
    // TODO: 创建一个闭包，使用 multiplier
    let multiply_by_three = |x| x * multiplier;
    
    assert_eq!(multiply_by_three(2), 6);
    assert_eq!(multiply_by_three(5), 15);
    println!("✅ 练习 2 完成！");
}

/// 练习 3: 使用 move 关键字
///
/// 任务：创建一个使用 move 的闭包
#[allow(dead_code)]
fn exercise_3() {
    let data = vec![1, 2, 3, 4, 5];
    let len = data.len();
    
    // TODO: 使用 move 关键字创建闭包
    let get_len = move || data.len();
    
    assert_eq!(get_len(), len);
    // data 已被移动，不能再使用
    println!("✅ 练习 3 完成！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise_1() {
        exercise_1();
    }

    #[test]
    fn test_exercise_2() {
        exercise_2();
    }

    #[test]
    fn test_exercise_3() {
        exercise_3();
    }
}

fn main() {
    println!("=== 基础概念练习 ===\n");
    exercise_1();
    exercise_2();
    exercise_3();
    println!("\n🎉 所有练习完成！");
}

