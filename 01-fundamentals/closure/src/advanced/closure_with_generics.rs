//! # 闭包与泛型的深度交互
//!
//! 本模块深入讲解闭包如何与 Rust 的泛型系统协同工作

/// 演示闭包与泛型的交互
pub fn demo_generics_interaction() {
    println!("\n=== 闭包与泛型的深度交互 ===");

    demo_generic_closures();
    demo_closure_as_generic_parameter();
    demo_multiple_trait_bounds();
    demo_higher_ranked_trait_bounds();
}

/// 泛型闭包
fn demo_generic_closures() {
    println!("\n--- 1. 泛型闭包 ---");

    // 闭包本身不能直接声明为泛型，但可以通过上下文推导
    let identity = |x| x;
    
    println!("整数: {}", identity(42));
    // 第一次调用后类型已确定，不能再用于其他类型
    // println!("{}", identity("hello")); // 错误

    // 解决方案：使用泛型函数包装闭包
    fn apply_generic<T, F>(value: T, f: F) -> T
    where
        F: Fn(T) -> T,
    {
        f(value)
    }

    println!("泛型应用整数: {}", apply_generic(42, |x| x * 2));
    println!("泛型应用字符串: {}", apply_generic("hello", |s| s));
}

/// 闭包作为泛型参数
fn demo_closure_as_generic_parameter() {
    println!("\n--- 2. 闭包作为泛型参数 ---");

    // 定义接受任意闭包的泛型函数
    fn twice<F, T>(f: F, x: T) -> T
    where
        F: Fn(T) -> T,
        T: Copy,
    {
        f(f(x))
    }

    println!("应用两次 +1: {}", twice(|x| x + 1, 5));
    println!("应用两次 *2: {}", twice(|x| x * 2, 3));

    // 更复杂：不同输入输出类型
    fn transform<F, A, B>(f: F, value: A) -> B
    where
        F: Fn(A) -> B,
    {
        f(value)
    }

    let result = transform(|x: i32| x.to_string(), 42);
    println!("转换结果: {}", result);
}

/// 多重 trait 约束
fn demo_multiple_trait_bounds() {
    println!("\n--- 3. 多重 Trait 约束 ---");

    // 闭包需要同时满足多个 trait
    #[allow(dead_code)] // 示例代码，演示泛型约束
    fn process<F, T>(f: F, value: T) -> T
    where
        F: Fn(T) -> T + Clone,
        T: Clone,
    {
        let f2 = f.clone();
        f2(value)
    }

    // 注意：大多数闭包不实现 Clone
    // 需要手动实现或使用 Copy 类型

    println!("💡 闭包的 trait 实现:");
    println!("- 闭包默认不实现 Clone");
    println!("- 如果捕获的所有变量都是 Copy，闭包才是 Copy");
    println!("- 可以使用 move 将变量移入闭包");
}

/// 高阶 trait 约束（HRTB）
fn demo_higher_ranked_trait_bounds() {
    println!("\n--- 4. 高阶 Trait 约束 (HRTB) ---");

    // HRTB 允许闭包接受任意生命周期的引用
    fn apply_to_ref<F>(f: F, value: &i32) -> i32
    where
        F: for<'a> Fn(&'a i32) -> i32,
    {
        f(value)
    }

    let closure = |x: &i32| *x * 2;
    println!("HRTB 结果: {}", apply_to_ref(closure, &21));

    // 实际应用：处理任意生命周期的字符串
    fn process_str<F>(f: F, s1: &str, s2: &str) -> String
    where
        F: Fn(&str) -> String,
    {
        format!("{} {}", f(s1), f(s2))
    }

    let trim = |s: &str| s.trim().to_string();
    println!("处理字符串: {}", process_str(trim, "  hello  ", "  world  "));
}

/// 实战案例：构建泛型 Pipeline
pub fn demo_generic_pipeline() {
    println!("\n=== 实战：泛型数据处理管道 ===");

    struct Pipeline<T> {
        data: Vec<T>,
    }

    impl<T> Pipeline<T> {
        fn new(data: Vec<T>) -> Self {
            Pipeline { data }
        }

        fn map<U, F>(self, f: F) -> Pipeline<U>
        where
            F: Fn(T) -> U,
        {
            Pipeline {
                data: self.data.into_iter().map(f).collect(),
            }
        }

        fn filter<F>(self, f: F) -> Pipeline<T>
        where
            F: Fn(&T) -> bool,
        {
            Pipeline {
                data: self.data.into_iter().filter(|x| f(x)).collect(),
            }
        }

        fn collect(self) -> Vec<T> {
            self.data
        }
    }

    let numbers = vec![1, 2, 3, 4, 5];
    let result = Pipeline::new(numbers)
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .map(|x| x.to_string())
        .collect();

    println!("管道结果: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_closure() {
        fn apply<T, F>(x: T, f: F) -> T
        where
            F: Fn(T) -> T,
        {
            f(x)
        }

        assert_eq!(apply(5, |x| x * 2), 10);
    }

    #[test]
    fn test_hrtb() {
        fn apply<F>(f: F, x: &i32) -> i32
        where
            F: for<'a> Fn(&'a i32) -> i32,
        {
            f(x)
        }

        assert_eq!(apply(|x| *x * 2, &5), 10);
    }
}

