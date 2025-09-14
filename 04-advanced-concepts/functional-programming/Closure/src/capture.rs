//! # 闭包的捕获机制
//!
//! 本模块详细介绍 Rust 闭包的捕获机制，包括：
//! - 不可变借用捕获
//! - 可变借用捕获
//! - 所有权转移捕获
//! - 捕获方式的自动推导
//! - move 关键字的使用

/// 演示闭包的捕获机制
pub fn demonstrate() {
    println!("\n🔍 2. 闭包的捕获机制");
    println!("{}", "-".repeat(40));

    immutable_borrow_capture();
    mutable_borrow_capture();
    ownership_transfer_capture();
    automatic_capture_inference();
    move_keyword_usage();
    capture_examples();
}

/// 演示不可变借用捕获
fn immutable_borrow_capture() {
    println!("\n📝 不可变借用捕获 (Immutable Borrow):");

    let x = 10;
    let y = 20;

    // 闭包通过不可变借用捕获环境变量
    let closure = || {
        println!("闭包内访问 x: {}, y: {}", x, y);
        x + y // 只读访问
    };

    println!("调用闭包前，x = {}, y = {}", x, y);
    let result = closure();
    println!("闭包返回值: {}", result);
    println!("调用闭包后，x = {}, y = {} (原变量未受影响)", x, y);

    // 可以多次调用闭包
    let result2 = closure();
    println!("再次调用闭包: {}", result2);

    // 原变量仍然可以使用
    println!("原变量仍可访问: x + y = {}", x + y);
}

/// 演示可变借用捕获
fn mutable_borrow_capture() {
    println!("\n📝 可变借用捕获 (Mutable Borrow):");

    let mut counter = 0;

    // 闭包需要修改捕获的变量，所以进行可变借用
    let mut increment = || {
        counter += 1;
        println!("计数器增加到: {}", counter);
        counter
    };

    // println!("初始计数器值: {}", counter); // 注释掉以避免借用冲突

    // 注意：当闭包进行可变借用时，原变量在闭包生命周期内不能使用
    // println!("这行代码会编译错误: {}", counter);  // 编译错误！

    let result1 = increment();
    let result2 = increment();
    let result3 = increment();

    println!("三次调用结果: {}, {}, {}", result1, result2, result3);

    // 闭包生命周期结束后，原变量可以再次使用
    drop(increment); // 显式结束闭包的生命周期
    println!("闭包结束后，counter = {}", counter);

    demonstrate_mutable_capture_with_multiple_vars();
}

/// 演示多个变量的可变借用捕获
fn demonstrate_mutable_capture_with_multiple_vars() {
    println!("\n📝 多变量可变借用捕获:");

    let mut a = 1;
    let mut b = 2;
    let c = 3; // 不可变变量

    let mut complex_closure = || {
        a *= 2; // 可变借用 a
        b += c; // 可变借用 b，不可变借用 c
        println!("闭包内: a = {}, b = {}, c = {}", a, b, c);
        (a, b, c)
    };

    let (new_a, new_b, new_c) = complex_closure();
    println!("第一次调用结果: ({}, {}, {})", new_a, new_b, new_c);

    let (new_a, new_b, new_c) = complex_closure();
    println!("第二次调用结果: ({}, {}, {})", new_a, new_b, new_c);
}

/// 演示所有权转移捕获
fn ownership_transfer_capture() {
    println!("\n📝 所有权转移捕获 (Move):");

    let name = String::from("Rust");
    let numbers = vec![1, 2, 3, 4, 5];

    // 使用 move 关键字强制所有权转移
    let closure = move || {
        println!("闭包拥有 name: {}", name);
        println!("闭包拥有 numbers: {:?}", numbers);
        format!("{} has {} numbers", name, numbers.len())
    };

    // 原变量已经被移动，不能再使用
    // println!("这行代码会编译错误: {}", name);     // 编译错误！
    // println!("这行代码会编译错误: {:?}", numbers); // 编译错误！

    let result = closure();
    println!("闭包结果: {}", result);

    // 可以多次调用闭包（如果闭包实现了相应的 trait）
    let result2 = closure();
    println!("再次调用: {}", result2);

    demonstrate_move_with_clone();
}

/// 演示使用 clone 避免所有权问题
fn demonstrate_move_with_clone() {
    println!("\n📝 使用 clone 避免所有权问题:");

    let original_data = vec!["hello".to_string(), "world".to_string()];

    // 克隆数据给闭包使用
    let cloned_data = original_data.clone();
    let closure = move || {
        println!("闭包使用克隆的数据: {:?}", cloned_data);
        cloned_data.len()
    };

    // 原数据仍然可以使用
    println!("原数据仍可用: {:?}", original_data);

    let length = closure();
    println!("数据长度: {}", length);
}

/// 演示捕获方式的自动推导
fn automatic_capture_inference() {
    println!("\n📝 捕获方式的自动推导:");

    let x = 10;
    let mut y = 20;
    let z = String::from("hello");

    // Rust 会根据闭包内的使用方式自动推导捕获方式

    // 1. 只读访问 -> 不可变借用
    let read_only = || {
        println!("只读访问: x = {}", x);
    };
    read_only();
    println!("x 仍可访问: {}", x); // x 仍然可用

    // 2. 修改访问 -> 可变借用
    let mut modify = || {
        y += 1;
        println!("修改后: y = {}", y);
    };
    modify();
    // println!("这里不能访问 y: {}", y);  // 编译错误，因为 y 被可变借用

    // 3. 需要所有权的操作 -> 所有权转移
    let take_ownership = move || {
        let owned_z = z; // 获取所有权
        println!("拥有所有权: {}", owned_z);
        owned_z.len()
    };

    let length = take_ownership();
    println!("字符串长度: {}", length);
    // println!("z 已被移动: {}", z);  // 编译错误！

    demonstrate_capture_priority();
}

/// 演示捕获优先级
fn demonstrate_capture_priority() {
    println!("\n📝 捕获优先级演示:");

    let data = vec![1, 2, 3];

    // Rust 优先选择最小权限的捕获方式

    // 情况1: 只需要读取 -> 不可变借用
    let read_closure = || {
        println!("读取数据: {:?}", data);
        data.len() // 只读操作
    };

    let len = read_closure();
    println!("数据长度: {}", len);
    println!("原数据仍可用: {:?}", data); // 原数据仍然可用

    // 情况2: 需要消费数据 -> 所有权转移
    let consume_closure = move || {
        let mut owned_data = data; // 获取所有权
        owned_data.push(4);
        println!("修改后的数据: {:?}", owned_data);
        owned_data
    };

    let modified_data = consume_closure();
    println!("消费后的数据: {:?}", modified_data);
    // println!("原数据已被移动: {:?}", data);  // 编译错误！
}

/// 演示 move 关键字的使用
fn move_keyword_usage() {
    println!("\n📝 move 关键字的使用:");

    // 场景1: 闭包需要比捕获的变量活得更久
    let closure = {
        let local_var = 42;
        // 必须使用 move，否则闭包会尝试借用已经销毁的变量
        move || {
            println!("捕获的局部变量: {}", local_var);
            local_var * 2
        }
    }; // local_var 在这里被销毁

    let result = closure(); // 但闭包仍然可以使用
    println!("结果: {}", result);

    // 场景2: 在多线程中使用闭包
    demonstrate_move_in_threads();

    // 场景3: 强制所有权转移
    demonstrate_forced_move();
}

/// 演示在多线程中使用 move
fn demonstrate_move_in_threads() {
    println!("\n📝 多线程中的 move:");

    let data = vec![1, 2, 3, 4, 5];
    let name = "background_task".to_string();

    // 在新线程中使用闭包，必须使用 move
    let handle = std::thread::spawn(move || {
        println!("线程 {} 处理数据: {:?}", name, data);
        data.iter().sum::<i32>()
    });

    // 等待线程完成
    match handle.join() {
        Ok(sum) => println!("线程计算结果: {}", sum),
        Err(_) => println!("线程执行失败"),
    }

    // data 和 name 已经被移动到线程中，这里不能再使用
    // println!("数据: {:?}", data);  // 编译错误！
}

/// 演示强制所有权转移
fn demonstrate_forced_move() {
    println!("\n📝 强制所有权转移:");

    let x = 10;
    let y = String::from("hello");

    // 即使闭包只需要借用，也可以强制使用 move
    let forced_move = move || {
        println!("强制移动: x = {}, y = {}", x, y);
        format!("{}-{}", x, y)
    };

    let result = forced_move();
    println!("结果: {}", result);

    // x 是 Copy 类型，所以仍然可以使用
    println!("x 仍可用 (Copy 类型): {}", x);

    // y 是非 Copy 类型，已经被移动
    // println!("y 已被移动: {}", y);  // 编译错误！
}

/// 综合捕获示例
fn capture_examples() {
    println!("\n📝 综合捕获示例:");

    let mut counter = 0;
    let multiplier = 3;
    let message = String::from("计算结果");

    // 复杂的捕获场景
    let mut complex_closure = || {
        counter += 1; // 可变借用 counter
        let result = counter * multiplier; // 不可变借用 multiplier
        println!("{}: 第{}次调用，结果 = {}", message, counter, result); // 不可变借用 message
        result
    };

    // 注意：由于 counter 被可变借用，在闭包生命周期内不能直接访问

    let results: Vec<i32> = (0..3).map(|_| complex_closure()).collect();
    println!("所有结果: {:?}", results);

    // 演示捕获列表的使用
    demonstrate_capture_list();
}

/// 演示不同类型的捕获组合
fn demonstrate_capture_list() {
    println!("\n📝 不同类型捕获的组合:");

    let readonly = 10;
    let mutable = 20;
    let owned = String::from("owned_data");
    let copyable = 42;

    // 创建一个复杂的闭包，展示不同的捕获方式
    let mixed_capture = move || {
        // readonly: 不可变借用 (但由于 move，实际是复制)
        // mutable: 可变借用 (但由于 move，实际是移动)
        // owned: 所有权转移
        // copyable: 复制 (Copy 类型)

        println!("只读数据: {}", readonly);
        // mutable += 1;  // 注意：由于使用了 move，这里的 mutable 是移动后的副本
        println!("可变数据: {}", mutable);
        println!("拥有的数据: {}", owned);
        println!("可复制数据: {}", copyable);

        format!("混合捕获: {}-{}-{}-{}", readonly, mutable, owned, copyable)
    };

    let result = mixed_capture();
    println!("混合捕获结果: {}", result);

    // 检查哪些变量仍然可用
    println!("readonly 仍可用: {}", readonly); // Copy 类型，仍可用
    println!("copyable 仍可用: {}", copyable); // Copy 类型，仍可用
                                               // println!("mutable 已移动: {}", mutable);   // 已移动，编译错误
                                               // println!("owned 已移动: {}", owned);       // 已移动，编译错误
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutable_capture() {
        let x = 5;
        let closure = || x * 2;
        assert_eq!(closure(), 10);
        assert_eq!(x, 5); // 原变量未受影响
    }

    #[test]
    fn test_mutable_capture() {
        let mut counter = 0;
        {
            let mut increment = || {
                counter += 1;
                counter
            };
            assert_eq!(increment(), 1);
            assert_eq!(increment(), 2);
        }
        assert_eq!(counter, 2);
    }

    #[test]
    fn test_move_capture() {
        let data = vec![1, 2, 3];
        let closure = move || data.len();
        assert_eq!(closure(), 3);
        // data 已经被移动，不能再使用
    }

    #[test]
    fn test_capture_priority() {
        let x = 10;
        let read_only = || x; // 最小权限：不可变借用
        assert_eq!(read_only(), 10);
        assert_eq!(x, 10); // 原变量仍可用
    }
}
