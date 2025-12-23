//! # 函数式编程模式

/// 演示高阶函数
pub fn demo_higher_order_functions() {
    println!("\n=== 高阶函数模式 ===");

    // 组合函数
    let add_one = |x: i32| x + 1;
    let double = |x: i32| x * 2;
    
    let composed = compose(add_one, double);
    println!("组合函数 (5+1)*2 = {}", composed(5));

    // 柯里化
    let add_curried = curry_add();
    let add_5 = add_curried(5);
    println!("柯里化 5 + 3 = {}", add_5(3));
}

/// 函数组合
fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

/// 柯里化加法
fn curry_add() -> impl Fn(i32) -> Box<dyn Fn(i32) -> i32> {
    |x| Box::new(move |y| x + y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose() {
        let add_one = |x| x + 1;
        let double = |x| x * 2;
        let composed = compose(add_one, double);
        assert_eq!(composed(5), 12);
    }
}

