//! # 作用域 (Scope) 深度分析
//!
//! 作用域是程序中变量和绑定可见性的范围。在Rust中，作用域决定了：
//! - 变量何时可以被访问
//! - 变量何时被销毁
//! - 内存何时被释放
//!
//! ## 作用域的类型
//!
//! 1. **词法作用域 (Lexical Scope)**: 由代码的结构决定
//! 2. **块作用域 (Block Scope)**: 由花括号 `{}` 定义
//! 3. **函数作用域 (Function Scope)**: 函数参数和局部变量的作用域
//! 4. **模块作用域 (Module Scope)**: 模块级别的作用域
//! 5. **全局作用域 (Global Scope)**: 整个程序的作用域

use std::collections::HashMap;

/// 运行所有作用域示例
pub fn run_scope_examples() {
    println!("\n=== 作用域 (Scope) 深度分析 ===");
    
    lexical_scope_examples();
    block_scope_examples();
    function_scope_examples();
    module_scope_examples();
    variable_shadowing_examples();
    scope_and_ownership_examples();
    scope_lifetime_interaction_examples();
}

/// 1. 词法作用域示例
fn lexical_scope_examples() {
    println!("\n🔍 1. 词法作用域 (Lexical Scope)");
    println!("词法作用域是由代码的静态结构决定的，编译时就能确定。");
    
    // 基本词法作用域
    {
        let outer_var = "外层变量";
        println!("外层作用域: {}", outer_var);
        
        {
            let inner_var = "内层变量";
            println!("内层作用域可以访问: {} 和 {}", outer_var, inner_var);
            
            {
                let deepest_var = "最深层变量";
                println!("最深层可以访问: {}, {}, {}", outer_var, inner_var, deepest_var);
            }
            // deepest_var 在这里不可访问
        }
        // inner_var 在这里不可访问
    }
    // outer_var 在这里不可访问
    
    // 词法作用域的嵌套规则
    demonstrate_lexical_nesting();
}

/// 演示词法作用域的嵌套规则
fn demonstrate_lexical_nesting() {
    println!("\n📚 词法作用域嵌套规则:");
    
    let level_1 = "第一层";
    
    {
        let level_2 = "第二层";
        println!("第二层可以访问: {}, {}", level_1, level_2);
        
        {
            let level_3 = "第三层";
            println!("第三层可以访问: {}, {}, {}", level_1, level_2, level_3);
            
            // 内层可以访问外层的所有变量
            let combined = format!("{} -> {} -> {}", level_1, level_2, level_3);
            println!("组合结果: {}", combined);
        }
    }
    
    println!("✅ 词法作用域遵循'内层可以访问外层，外层不能访问内层'的规则");
}

/// 2. 块作用域示例
fn block_scope_examples() {
    println!("\n🏗️  2. 块作用域 (Block Scope)");
    println!("块作用域由花括号 {{}} 定义，是Rust中最基本的作用域单位。");
    
    // 基本块作用域
    let x = 1;
    {
        let x = 2; // 这是一个新的变量，遮蔽了外层的x
        println!("块内的x: {}", x);
        
        let y = 3; // y只在这个块内有效
        println!("块内的y: {}", y);
    } // y在这里被销毁
    println!("块外的x: {}", x); // 这里的x是外层的x
    
    // 条件块作用域
    let condition = true;
    if condition {
        let if_var = "if块变量";
        println!("if块内: {}", if_var);
    }
    // if_var 在这里不可访问
    
    // 循环块作用域
    for i in 0..3 {
        let loop_var = format!("循环变量_{}", i);
        println!("循环块内: {}", loop_var);
    } // loop_var 在每次迭代结束时被销毁
    
    // match块作用域
    let value = Some(42);
    match value {
        Some(n) => {
            let match_var = format!("匹配到的值: {}", n);
            println!("match块内: {}", match_var);
        }
        None => {
            let none_var = "没有值";
            println!("None分支: {}", none_var);
        }
    }
    
    demonstrate_block_scope_memory();
}

/// 演示块作用域与内存管理
fn demonstrate_block_scope_memory() {
    println!("\n💾 块作用域与内存管理:");
    
    {
        let heap_data = vec![1, 2, 3, 4, 5];
        println!("创建堆数据: {:?}", heap_data);
        
        {
            let more_data = HashMap::from([
                ("key1", "value1"),
                ("key2", "value2"),
            ]);
            println!("创建更多堆数据: {:?}", more_data);
        } // more_data 在这里被自动释放
        
        println!("内层数据已释放，外层数据仍然存在: {:?}", heap_data);
    } // heap_data 在这里被自动释放
    
    println!("✅ 所有数据都已自动释放，无需手动管理内存");
}

/// 3. 函数作用域示例
fn function_scope_examples() {
    println!("\n🔧 3. 函数作用域 (Function Scope)");
    println!("函数作用域包括函数参数、局部变量和返回值的作用域。");
    
    // 函数参数作用域
    fn parameter_scope_demo(param: i32) -> i32 {
        println!("函数参数 param 的值: {}", param);
        let local_var = param * 2;
        println!("局部变量 local_var 的值: {}", local_var);
        local_var // 返回值
    }
    
    let result = parameter_scope_demo(10);
    println!("函数返回值: {}", result);
    // param 和 local_var 在函数外不可访问
    
    // 嵌套函数作用域
    fn outer_function() {
        let outer_local = "外层函数变量";
        
        fn inner_function() {
            let inner_local = "内层函数变量";
            println!("内层函数: {}", inner_local);
            // 注意: 内层函数不能直接访问外层函数的变量（除非通过参数传递）
        }
        
        inner_function();
        println!("外层函数: {}", outer_local);
    }
    
    outer_function();
    
    // 闭包与作用域
    demonstrate_closure_scope();
}

/// 演示闭包与作用域的关系
fn demonstrate_closure_scope() {
    println!("\n🔒 闭包与作用域:");
    
    let captured_var = "被捕获的变量";
    
    let closure = || {
        println!("闭包内访问: {}", captured_var);
    };
    
    closure();
    
    // 闭包可以捕获外层作用域的变量
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        println!("计数器: {}", counter);
    };
    
    increment();
    increment();
    increment();
    
    println!("✅ 闭包可以捕获并修改外层作用域的变量");
}

/// 4. 模块作用域示例
fn module_scope_examples() {
    println!("\n📦 4. 模块作用域 (Module Scope)");
    println!("模块作用域定义了模块内部和外部的可见性。");
    
    // 使用内部模块演示
    mod inner_module {
        pub const PUBLIC_CONST: &str = "公共常量";
        const PRIVATE_CONST: &str = "私有常量";
        
        pub fn public_function() {
            println!("公共函数可以访问私有常量: {}", PRIVATE_CONST);
        }
        
        fn private_function() {
            println!("私有函数: {}", PRIVATE_CONST);
        }
        
        pub fn call_private() {
            private_function();
        }
    }
    
    // 访问公共项
    println!("访问公共常量: {}", inner_module::PUBLIC_CONST);
    inner_module::public_function();
    inner_module::call_private();
    
    // 以下代码会编译错误，因为是私有的：
    // println!("{}", inner_module::PRIVATE_CONST);
    // inner_module::private_function();
    
    println!("✅ 模块作用域控制了代码的可见性和封装性");
}

/// 5. 变量遮蔽示例
fn variable_shadowing_examples() {
    println!("\n🎭 5. 变量遮蔽 (Variable Shadowing)");
    println!("变量遮蔽允许在内层作用域中重新定义同名变量。");
    
    let x = 5;
    println!("外层 x: {}", x);
    
    {
        let x = "字符串"; // 遮蔽了外层的x，类型也可以不同
        println!("内层 x (字符串): {}", x);
        
        {
            let x = vec![1, 2, 3]; // 再次遮蔽，类型又不同
            println!("最内层 x (向量): {:?}", x);
        }
        
        println!("回到内层 x: {}", x); // 这里的x是字符串
    }
    
    println!("回到外层 x: {}", x); // 这里的x是整数
    
    // 遮蔽与可变性
    let y = 10;
    let y = y + 5; // 遮蔽并重新绑定
    let y = format!("数字: {}", y); // 再次遮蔽并改变类型
    println!("最终的 y: {}", y);
    
    println!("✅ 变量遮蔽提供了灵活的变量重用机制");
}

/// 6. 作用域与所有权的交互
fn scope_and_ownership_examples() {
    println!("\n🔐 6. 作用域与所有权的交互");
    println!("作用域决定了所有权的转移和借用的有效性。");
    
    // 所有权转移
    {
        let data = String::from("拥有的数据");
        println!("创建数据: {}", data);
        
        {
            let moved_data = data; // 所有权转移到内层作用域
            println!("数据移动到内层: {}", moved_data);
            // data 在这里不再可用
        } // moved_data 在这里被销毁
        
        // println!("{}", data); // 这会编译错误，因为所有权已转移
    }
    
    // 借用与作用域
    {
        let original = String::from("原始数据");
        
        {
            let borrowed = &original; // 借用
            println!("借用的数据: {}", borrowed);
        } // 借用在这里结束
        
        println!("原始数据仍然可用: {}", original);
    }
    
    // 可变借用与作用域
    {
        let mut mutable_data = vec![1, 2, 3];
        
        {
            let mutable_ref = &mut mutable_data;
            mutable_ref.push(4);
            println!("可变借用修改后: {:?}", mutable_ref);
        } // 可变借用在这里结束
        
        println!("原始数据: {:?}", mutable_data);
    }
    
    println!("✅ 作用域确保了所有权和借用规则的正确执行");
}

/// 7. 作用域与生命周期的交互
fn scope_lifetime_interaction_examples() {
    println!("\n⏰ 7. 作用域与生命周期的交互");
    println!("作用域为生命周期提供了基础，但生命周期可以更精确地控制引用的有效性。");
    
    // 基本的作用域-生命周期关系
    {
        let long_lived = String::from("长生命周期数据");
        
        {
            let short_lived = String::from("短生命周期数据");
            let long_ref = &long_lived;   // 引用长生命周期数据
            let short_ref = &short_lived; // 引用短生命周期数据
            
            println!("长引用: {}", long_ref);
            println!("短引用: {}", short_ref);
        } // short_lived 和 short_ref 在这里结束
        
        // long_ref 在这里不再可用，即使 long_lived 还存在
        println!("长生命周期数据仍存在: {}", long_lived);
    }
    
    // 函数返回引用的作用域问题
    demonstrate_return_reference_scope();
    
    println!("✅ 理解作用域是掌握生命周期的基础");
}

/// 演示函数返回引用的作用域问题
fn demonstrate_return_reference_scope() {
    println!("\n🔄 函数返回引用的作用域:");
    
    // 正确的方式：返回拥有的数据
    fn create_string() -> String {
        String::from("创建的字符串")
    }
    
    let owned = create_string();
    println!("拥有的字符串: {}", owned);
    
    // 正确的方式：接受引用参数并返回引用
    fn get_first_word(s: &str) -> &str {
        s.split_whitespace().next().unwrap_or("")
    }
    
    let sentence = "Hello Rust World";
    let first_word = get_first_word(&sentence);
    println!("第一个单词: {}", first_word);
    
    // 错误的方式（注释掉，因为不会编译）：
    /*
    fn create_reference() -> &str {
        let local = String::from("局部字符串");
        &local // 错误！返回了对局部变量的引用
    }
    */
    
    println!("✅ 函数不能返回对局部变量的引用，因为局部变量在函数结束时被销毁");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_examples() {
        // 测试所有作用域示例是否能正常运行
        run_scope_examples();
    }

    #[test]
    fn test_variable_shadowing() {
        let x = 5;
        {
            let x = "shadow";
            assert_eq!(x, "shadow");
        }
        assert_eq!(x, 5);
    }

    #[test]
    fn test_block_scope_isolation() {
        let outer = 10;
        {
            let inner = 20;
            assert_eq!(outer + inner, 30);
        }
        // inner 在这里不可访问
        assert_eq!(outer, 10);
    }
}