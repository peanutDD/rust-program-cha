//! # Trait 层次关系
//!
//! Fn > FnMut > FnOnce 的继承关系

/// 演示 Trait 的层次关系
pub fn demo_trait_hierarchy() {
    println!("\n=== Trait 层次关系 ===");
    
    println!("继承关系: Fn : FnMut : FnOnce");
    println!("即：实现 Fn 的闭包也实现 FnMut 和 FnOnce\n");

    // Fn 闭包
    let immutable_env = 42;
    let fn_closure = || immutable_env * 2;

    // FnMut 闭包
    let mut mutable_env = 0;
    let mut fn_mut_closure = || {
        mutable_env += 1;
        mutable_env
    };

    // FnOnce 闭包
    let owned_data = vec![1, 2, 3];
    let fn_once_closure = move || owned_data.into_iter().sum::<i32>();

    // 展示 trait 的包含关系
    call_fn_trait(&fn_closure);
    call_fn_mut_trait(&mut fn_mut_closure);
    call_fn_once_trait(fn_once_closure);

    // Fn 可以当作 FnMut 和 FnOnce 使用
    println!("\nFn 可以当作 FnMut 使用:");
    call_fn_mut_trait(&mut || immutable_env * 3);
    
    println!("\nFn 可以当作 FnOnce 使用:");
    call_fn_once_trait(|| immutable_env * 4);
}

fn call_fn_trait<F>(f: &F) -> i32
where
    F: Fn() -> i32,
{
    let result = f();
    println!("调用 Fn trait: {}", result);
    result
}

fn call_fn_mut_trait<F>(f: &mut F) -> i32
where
    F: FnMut() -> i32,
{
    let result = f();
    println!("调用 FnMut trait: {}", result);
    result
}

fn call_fn_once_trait<F>(f: F) -> i32
where
    F: FnOnce() -> i32,
{
    let result = f();
    println!("调用 FnOnce trait: {}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_hierarchy() {
        let x = 10;
        let fn_closure = || x;
        
        // Fn 可以用作 FnMut
        let mut fn_as_fn_mut = fn_closure;
        assert_eq!(call_fn_mut_trait(&mut fn_as_fn_mut), 10);
        
        // Fn 可以用作 FnOnce
        assert_eq!(call_fn_once_trait(fn_closure), 10);
    }
}

