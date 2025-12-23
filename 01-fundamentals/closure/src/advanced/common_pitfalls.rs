//! # 闭包常见陷阱和解决方案
//!
//! 本模块总结了使用闭包时的常见错误和最佳解决方案

/// 演示常见陷阱
pub fn demo_common_pitfalls() {
    println!("\n=== 闭包常见陷阱和解决方案 ===");

    pitfall_1_unexpected_move();
    pitfall_2_borrow_checker_confusion();
    pitfall_3_closure_size_bloat();
    pitfall_4_trait_object_limitations();
    pitfall_5_lifetime_confusion();
    pitfall_6_type_inference_failure();
}

/// 陷阱1：意外的移动语义
fn pitfall_1_unexpected_move() {
    println!("\n--- 陷阱1：意外的移动语义 ---");

    println!("❌ 常见错误：");
    println!("let data = vec![1, 2, 3];");
    println!("let closure = || {{");
    println!("    data  // 这会移动 data！");
    println!("}};");
    println!("println!(\"{{:?}}\", data);  // 错误：data 已被移动");

    println!("\n✅ 解决方案1：只借用需要的部分");
    {
        let data = vec![1, 2, 3];
        let closure = || {
            println!("数据长度: {}", data.len());  // 只借用，不移动
        };
        closure();
        println!("原始数据仍可用: {:?}", data);
    }

    println!("\n✅ 解决方案2：显式克隆");
    {
        let data = vec![1, 2, 3];
        let data_clone = data.clone();
        let closure = move || {
            println!("克隆的数据: {:?}", data_clone);
        };
        closure();
        println!("原始数据仍可用: {:?}", data);
    }

    println!("\n✅ 解决方案3：使用引用计数");
    {
        use std::rc::Rc;
        let data = Rc::new(vec![1, 2, 3]);
        let data_clone = Rc::clone(&data);
        let closure = move || {
            println!("共享数据: {:?}", data_clone);
        };
        closure();
        println!("原始引用仍可用: {:?}", data);
    }
}

/// 陷阱2：借用检查器混淆
fn pitfall_2_borrow_checker_confusion() {
    println!("\n--- 陷阱2：借用检查器混淆 ---");

    println!("❌ 常见错误：同时可变和不可变借用");
    println!("let mut count = 0;");
    println!("let increment = || {{ count += 1; }};");
    println!("println!(\"Count: {{}}\", count);  // 错误！");
    println!("increment();");

    println!("\n💡 问题分析：");
    println!("闭包 increment 创建了对 count 的可变借用");
    println!("println! 试图创建不可变借用");
    println!("Rust 不允许同时存在可变和不可变借用");

    println!("\n✅ 解决方案1：限制闭包作用域");
    {
        let mut count = 0;
        {
            let mut increment = || { count += 1; };
            increment();
            increment();
        } // 闭包在这里结束，释放可变借用
        
        println!("现在可以访问 count: {}", count);
    }

    println!("\n✅ 解决方案2：使用内部可变性");
    {
        use std::cell::RefCell;
        let count = RefCell::new(0);
        
        let increment = || {
            *count.borrow_mut() += 1;
        };
        
        increment();
        println!("Count: {}", count.borrow());
        increment();
        println!("Count: {}", count.borrow());
    }
}

/// 陷阱3：闭包大小膨胀
fn pitfall_3_closure_size_bloat() {
    println!("\n--- 陷阱3：闭包大小膨胀 ---");

    println!("❌ 问题：捕获大型数据结构");
    {
        let large_data = vec![0; 1000000];  // 1M 元素
        let small_value = 42;
        
        // 不好：捕获了整个大型数据结构
        let bad_closure = move || {
            println!("只需要: {}", small_value);
            // large_data 被移动但未使用
            let _ = large_data.len();
        };
        
        println!("闭包大小: {} bytes", std::mem::size_of_val(&bad_closure));
        bad_closure();
    }

    println!("\n✅ 解决方案：只捕获需要的部分");
    {
        let large_data = vec![0; 1000000];
        let small_value = 42;
        let data_len = large_data.len();  // 提取需要的信息
        
        // 好：只捕获必要的数据
        let good_closure = move || {
            println!("需要的值: {}, 数据长度: {}", small_value, data_len);
        };
        
        println!("闭包大小: {} bytes", std::mem::size_of_val(&good_closure));
        good_closure();
    }

    println!("\n💡 性能提示：");
    println!("- 闭包会按值捕获所有引用的变量");
    println!("- 大型数据结构会显著增加闭包大小");
    println!("- 考虑使用引用或只提取需要的字段");
}

/// 陷阱4：trait object 的限制
fn pitfall_4_trait_object_limitations() {
    println!("\n--- 陷阱4：Trait Object 的限制 ---");

    println!("❌ 问题：返回不同类型的闭包");
    println!("fn create_closure(flag: bool) -> impl Fn(i32) -> i32 {{");
    println!("    if flag {{");
    println!("        |x| x + 1  // 类型 A");
    println!("    }} else {{");
    println!("        |x| x * 2  // 类型 B - 编译错误！");
    println!("    }}");
    println!("}}");

    println!("\n✅ 解决方案1：使用 Box<dyn Fn>");
    {
        fn create_closure(flag: bool) -> Box<dyn Fn(i32) -> i32> {
            if flag {
                Box::new(|x| x + 1)
            } else {
                Box::new(|x| x * 2)
            }
        }

        let closure = create_closure(true);
        println!("结果: {}", closure(5));
    }

    println!("\n✅ 解决方案2：使用枚举");
    {
        enum Operation {
            Add,
            Multiply,
        }

        fn create_closure(op: Operation) -> impl Fn(i32) -> i32 {
            move |x| match op {
                Operation::Add => x + 1,
                Operation::Multiply => x * 2,
            }
        }

        let closure = create_closure(Operation::Add);
        println!("结果: {}", closure(5));
    }

    println!("\n💡 性能对比：");
    println!("- Box<dyn Fn>: 动态分发，有运行时开销");
    println!("- 枚举方式: 静态分发，编译时优化");
}

/// 陷阱5：生命周期混淆
fn pitfall_5_lifetime_confusion() {
    println!("\n--- 陷阱5：生命周期混淆 ---");

    println!("❌ 常见错误：返回捕获局部变量的闭包");
    println!("fn bad_closure() -> impl Fn() -> &str {{");
    println!("    let s = String::from(\"hello\");");
    println!("    || &s  // 错误：s 的生命周期不够长");
    println!("}}");

    println!("\n✅ 解决方案1：返回所有权");
    {
        fn good_closure() -> impl Fn() -> String {
            let s = String::from("hello");
            move || s.clone()
        }

        let closure = good_closure();
        println!("结果: {}", closure());
    }

    println!("\n✅ 解决方案2：使用静态生命周期");
    {
        fn good_closure() -> impl Fn() -> &'static str {
            || "hello"
        }

        let closure = good_closure();
        println!("结果: {}", closure());
    }

    println!("\n✅ 解决方案3：使用 Rc 或 Arc");
    {
        use std::rc::Rc;
        
        fn good_closure() -> impl Fn() -> String {
            let s = Rc::new(String::from("hello"));
            move || (*s).clone()
        }

        let closure = good_closure();
        println!("结果: {}", closure());
    }
}

/// 陷阱6：类型推导失败
fn pitfall_6_type_inference_failure() {
    println!("\n--- 陷阱6：类型推导失败 ---");

    println!("❌ 问题：闭包类型推导冲突");
    println!("let closure = |x| x;");
    println!("println!(\"{{}}\", closure(5));");
    println!("println!(\"{{}}\", closure(\"hello\"));  // 错误：类型已确定为整数");

    println!("\n✅ 解决方案1：使用泛型函数");
    {
        fn identity<T>(x: T) -> T {
            x
        }

        println!("整数: {}", identity(5));
        println!("字符串: {}", identity("hello"));
    }

    println!("\n✅ 解决方案2：创建多个闭包");
    {
        let int_closure = |x: i32| x;
        let str_closure = |x: &str| x.to_string();

        println!("整数: {}", int_closure(5));
        println!("字符串: {}", str_closure("hello"));
    }

    println!("\n💡 类型推导规则：");
    println!("- 闭包的类型在第一次使用时确定");
    println!("- 一旦确定，就不能改变");
    println!("- 需要多态行为时，使用泛型函数或 trait");
}

/// 进阶陷阱：闭包与迭代器的交互
pub fn demo_advanced_pitfalls() {
    println!("\n=== 进阶陷阱 ===");

    // 陷阱7：迭代器中的闭包捕获
    pitfall_7_iterator_capture();

    // 陷阱8：递归闭包
    pitfall_8_recursive_closure();
}

fn pitfall_7_iterator_capture() {
    println!("\n--- 陷阱7：迭代器中的闭包捕获 ---");

    println!("❌ 问题：在循环中创建捕获可变变量的闭包");
    println!("let mut closures = Vec::new();");
    println!("for i in 0..3 {{");
    println!("    closures.push(|| println!(\"{{}}\", i));  // 问题！");
    println!("}}");

    println!("\n✅ 解决方案：使用 move 捕获副本");
    {
        let mut closures: Vec<Box<dyn Fn()>> = Vec::new();
        for i in 0..3 {
            closures.push(Box::new(move || println!("值: {}", i)));
        }

        println!("执行闭包:");
        for closure in closures {
            closure();
        }
    }
}

fn pitfall_8_recursive_closure() {
    println!("\n--- 陷阱8：递归闭包 ---");

    println!("❌ 问题：闭包不能直接递归调用自己");
    println!("let factorial = |n| {{");
    println!("    if n == 0 {{ 1 }} else {{ n * factorial(n-1) }}  // 错误！");
    println!("}};");

    println!("\n✅ 解决方案1：使用函数");
    {
        fn factorial(n: u32) -> u32 {
            if n == 0 { 1 } else { n * factorial(n - 1) }
        }

        println!("5! = {}", factorial(5));
    }

    println!("\n✅ 解决方案2：使用 Rc 和 RefCell（高级）");
    {
        use std::rc::Rc;
        use std::cell::RefCell;

        type FactorialFn = Rc<RefCell<Option<Box<dyn Fn(u32) -> u32>>>>;

        fn make_factorial() -> impl Fn(u32) -> u32 {
            let factorial: FactorialFn = Rc::new(RefCell::new(None));
            let factorial_clone = factorial.clone();

            *factorial.borrow_mut() = Some(Box::new(move |n: u32| {
                if n == 0 {
                    1
                } else {
                    let f = factorial_clone.borrow();
                    let f = f.as_ref().unwrap();
                    n * f(n - 1)
                }
            }));

            let factorial_final = factorial.clone();
            move |n| {
                let f = factorial_final.borrow();
                f.as_ref().unwrap()(n)
            }
        }

        let factorial = make_factorial();
        println!("5! = {}", factorial(5));
    }
}

/// 最佳实践总结
pub fn demo_best_practices_summary() {
    println!("\n=== 避免陷阱的最佳实践 ===");

    println!("\n📋 检查清单:");
    println!("1. ✅ 明确是否需要 move");
    println!("2. ✅ 注意闭包的生命周期");
    println!("3. ✅ 避免捕获不必要的大型数据");
    println!("4. ✅ 理解借用检查器的规则");
    println!("5. ✅ 选择合适的 Fn trait");
    println!("6. ✅ 在需要多态时使用 trait object");
    println!("7. ✅ 为复杂逻辑创建独立函数");

    println!("\n💡 调试技巧:");
    println!("- 使用 println! 检查闭包何时被调用");
    println!("- 使用 std::mem::size_of_val 检查闭包大小");
    println!("- 阅读编译器错误信息，通常很有帮助");
    println!("- 从简单示例开始，逐步增加复杂度");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avoiding_move() {
        let data = vec![1, 2, 3];
        let closure = || data.len();
        assert_eq!(closure(), 3);
        assert_eq!(data.len(), 3); // data 仍可用
    }

    #[test]
    fn test_interior_mutability() {
        use std::cell::RefCell;
        let count = RefCell::new(0);
        
        let increment = || {
            *count.borrow_mut() += 1;
        };
        
        increment();
        assert_eq!(*count.borrow(), 1);
    }

    #[test]
    fn test_minimal_capture() {
        let large_data = vec![0; 1000];
        let len = large_data.len();
        
        let closure = move || len;  // 只捕获 len，不捕获 large_data
        assert_eq!(closure(), 1000);
    }
}

