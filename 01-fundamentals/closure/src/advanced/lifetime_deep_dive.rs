//! # 闭包与生命周期深度解析
//!
//! 生命周期是 Rust 闭包中最容易出错的部分，本模块深入讲解

/// 演示闭包的生命周期问题
pub fn demo_lifetime_issues() {
    println!("\n=== 闭包与生命周期深度解析 ===");

    // 1. 返回闭包的生命周期问题
    demo_returning_closures();

    // 2. 闭包中的引用生命周期
    demo_closure_with_references();

    // 3. 高阶函数中的生命周期
    demo_higher_order_lifetime();

    // 4. 复杂场景：嵌套闭包的生命周期
    demo_nested_closure_lifetime();
}

/// 返回闭包的生命周期问题
fn demo_returning_closures() {
    println!("\n--- 1. 返回闭包的生命周期 ---");

    // ❌ 错误示例：尝试返回引用局部变量的闭包
    // fn bad_closure() -> impl Fn() -> i32 {
    //     let x = 10;
    //     || x  // 错误：x 的生命周期不够长
    // }

    // ✅ 正确：使用 move 获取所有权
    fn good_closure_move() -> impl Fn() -> i32 {
        let x = 10;
        move || x  // OK：x 被移动到闭包中
    }

    let closure = good_closure_move();
    println!("使用 move 的闭包结果: {}", closure());

    // ✅ 正确：返回不捕获变量的闭包
    fn good_closure_no_capture() -> impl Fn(i32) -> i32 {
        |x| x * 2  // OK：不捕获外部变量
    }

    let closure = good_closure_no_capture();
    println!("不捕获变量的闭包结果: {}", closure(5));

    // 深入理解：为什么需要 move？
    explain_why_move_needed();
}

fn explain_why_move_needed() {
    println!("\n💡 为什么需要 move？");
    println!("当闭包的生命周期超过被捕获变量的作用域时，必须使用 move");
    println!("move 将变量的所有权转移到闭包中，闭包拥有了这些数据");
    
    // 示例：对比借用和 move
    {
        let data = vec![1, 2, 3];
        
        // 借用版本：闭包生命周期不能超过 data
        let borrow_closure = || data.len();
        println!("借用版本: {}", borrow_closure());
        // borrow_closure 的生命周期必须在 data 之前结束
    }

    {
        let data = vec![1, 2, 3];
        
        // move 版本：闭包拥有数据，可以超过原作用域
        let move_closure = move || data.len();
        // data 已被移动，但 move_closure 可以继续存在
        println!("move 版本: {}", move_closure());
    }
}

/// 闭包中的引用生命周期
fn demo_closure_with_references() {
    println!("\n--- 2. 闭包中的引用生命周期 ---");

    // 场景1：闭包捕获引用
    demo_capturing_references();

    // 场景2：闭包返回引用
    demo_returning_references();

    // 场景3：生命周期省略规则
    demo_lifetime_elision();
}

fn demo_capturing_references() {
    println!("\n场景1：闭包捕获引用");

    let data = vec![1, 2, 3, 4, 5];
    
    // 闭包捕获 data 的引用
    let get_first = || data.first();
    
    println!("第一个元素: {:?}", get_first());
    
    // data 仍然可用
    println!("原始数据: {:?}", data);

    // 使用显式生命周期标注理解
    explain_reference_lifetime();
}

fn explain_reference_lifetime() {
    println!("\n💡 引用生命周期分析:");
    
    let data = String::from("Hello");
    
    // 编译器实际看到的（简化版）：
    // struct Closure<'a> {
    //     data: &'a String,
    // }
    //
    // impl<'a> Fn() -> usize for Closure<'a> {
    //     fn call(&self) -> usize {
    //         self.data.len()
    //     }
    // }
    
    let closure = || data.len();
    println!("闭包捕获引用，生命周期与 data 相同");
    println!("结果: {}", closure());
}

fn demo_returning_references() {
    println!("\n场景2：闭包返回引用");

    let data = vec![1, 2, 3, 4, 5];
    
    // 闭包返回引用
    let get_item = |index: usize| -> Option<&i32> {
        data.get(index)
    };
    
    if let Some(item) = get_item(2) {
        println!("索引2的元素: {}", item);
    }

    // 复杂场景：闭包既捕获引用又返回引用
    demo_capture_and_return_ref();
}

fn demo_capture_and_return_ref() {
    println!("\n💡 捕获引用并返回引用:");

    let data = vec!["Hello", "World", "Rust"];
    
    // 这个闭包捕获 data 的引用，并返回其中元素的引用
    let get_longest = || -> &str {
        data.iter()
            .max_by_key(|s| s.len())
            .map(|s| *s)
            .unwrap_or("")
    };
    
    println!("最长的字符串: {}", get_longest());
}

fn demo_lifetime_elision() {
    println!("\n场景3：生命周期省略规则");

    // 在闭包中，生命周期省略规则同样适用
    let data = vec![1, 2, 3];
    
    // 编译器自动推导生命周期 - 返回值而非引用
    let closure = |x: &i32| *x;
    
    if let Some(first) = data.first() {
        let result = closure(first);
        println!("结果: {}", result);
    }

    // 演示函数中的生命周期省略
    fn get_first(data: &[i32]) -> Option<&i32> {
        data.first()
    }

    if let Some(first) = get_first(&data) {
        println!("第一个元素: {}", first);
    }

    println!("\n💡 闭包的生命周期省略规则与函数相同");
}

/// 高阶函数中的生命周期
fn demo_higher_order_lifetime() {
    println!("\n--- 3. 高阶函数中的生命周期 ---");

    // 场景：接受闭包作为参数时的生命周期
    fn process_with_closure<'a, F>(data: &'a str, f: F) -> &'a str
    where
        F: Fn(&'a str) -> &'a str,
    {
        f(data)
    }

    let text = String::from("Hello, Rust!");
    let result = process_with_closure(&text, |s| {
        s.split(',').next().unwrap_or("")
    });
    
    println!("处理结果: {}", result);

    // 更复杂的例子
    demo_complex_higher_order();
}

fn demo_complex_higher_order() {
    println!("\n💡 复杂的高阶函数生命周期:");

    // 闭包捕获多个引用，每个都有不同的生命周期
    fn process_two<'a, 'b, F>(s1: &'a str, s2: &'b str, f: F) -> String
    where
        F: Fn(&'a str, &'b str) -> String,
    {
        f(s1, s2)
    }

    let s1 = String::from("Hello");
    let s2 = String::from("World");
    
    let result = process_two(&s1, &s2, |a, b| {
        format!("{} {}", a, b)
    });
    
    println!("组合结果: {}", result);
}

/// 嵌套闭包的生命周期
fn demo_nested_closure_lifetime() {
    println!("\n--- 4. 嵌套闭包的生命周期 ---");

    let outer_data = vec![1, 2, 3, 4, 5];
    
    // 外层闭包
    let outer_closure = || {
        let inner_data = vec![10, 20, 30];
        
        // 内层闭包捕获两层作用域的数据
        let inner_closure = || {
            println!("外层数据: {:?}", outer_data);
            println!("内层数据: {:?}", inner_data);
        };
        
        inner_closure();
    };
    
    outer_closure();

    // 更复杂的嵌套场景
    demo_complex_nested();
}

fn demo_complex_nested() {
    println!("\n💡 复杂嵌套闭包分析:");

    let data = String::from("Rust Programming");
    
    // 返回一个闭包工厂
    let create_processor = move || {
        // 这个闭包捕获了 data
        move |prefix: &str| {
            format!("{}: {}", prefix, data)
        }
    };
    
    let processor = create_processor();
    println!("结果: {}", processor("Language"));
    
    // 注意：data 已被第一个 move 移动，无法再使用
    // println!("{}", data); // 错误
}

/// 实战案例：设计带生命周期的闭包 API
pub fn demo_lifetime_api_design() {
    println!("\n=== 实战：设计带生命周期的闭包 API ===");

    // 案例1：数据处理管道
    struct Pipeline<'a> {
        data: &'a [i32],
    }

    impl<'a> Pipeline<'a> {
        fn new(data: &'a [i32]) -> Self {
            Pipeline { data }
        }

        fn map<F>(&self, f: F) -> Vec<i32>
        where
            F: Fn(&i32) -> i32,
        {
            self.data.iter().map(f).collect()
        }

        fn filter<F>(&self, f: F) -> Vec<&'a i32>
        where
            F: Fn(&&i32) -> bool,
        {
            self.data.iter().filter(f).collect()
        }
    }

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let pipeline = Pipeline::new(&numbers);

    let doubled = pipeline.map(|&x| x * 2);
    println!("翻倍结果: {:?}", doubled);

    let evens = pipeline.filter(|&&x| x % 2 == 0);
    println!("偶数: {:?}", evens);
}

/// 常见生命周期错误和解决方案
pub fn demo_lifetime_errors() {
    println!("\n=== 常见生命周期错误 ===");

    // 错误1：悬垂引用
    println!("\n❌ 错误1：悬垂引用");
    println!("fn bad() -> &str {{");
    println!("    let s = String::from(\"hello\");");
    println!("    &s  // 错误：返回对局部变量的引用");
    println!("}}");

    println!("\n✅ 解决方案1：返回所有权");
    fn good1() -> String {
        String::from("hello")
    }
    println!("返回值: {}", good1());

    println!("\n✅ 解决方案2：使用静态生命周期");
    fn good2() -> &'static str {
        "hello"
    }
    println!("返回值: {}", good2());

    // 错误2：闭包捕获引用后数据被移动
    println!("\n❌ 错误2：闭包捕获引用后数据被移动");
    println!("let data = vec![1, 2, 3];");
    println!("let closure = || data.len();");
    println!("drop(data);  // 错误：data 被移动，但闭包仍持有引用");

    println!("\n✅ 解决方案：使用 move 获取所有权");
    {
        let data = vec![1, 2, 3];
        let closure = move || data.len();
        // data 已被移动到闭包中
        println!("闭包结果: {}", closure());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifetime_move() {
        fn create_closure() -> impl Fn() -> i32 {
            let x = 10;
            move || x
        }
        
        let closure = create_closure();
        assert_eq!(closure(), 10);
    }

    #[test]
    fn test_capturing_reference() {
        let data = vec![1, 2, 3];
        let closure = || data.len();
        assert_eq!(closure(), 3);
        assert_eq!(data.len(), 3); // data 仍可用
    }

    #[test]
    fn test_higher_order_lifetime() {
        fn process<'a, F>(s: &'a str, f: F) -> &'a str
        where
            F: Fn(&'a str) -> &'a str,
        {
            f(s)
        }

        let text = "Hello, World!";
        let result = process(text, |s| s.split(',').next().unwrap());
        assert_eq!(result, "Hello");
    }
}

