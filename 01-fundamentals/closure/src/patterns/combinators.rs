//! # 组合子模式

/// 演示组合子
pub fn demo_combinators() {
    println!("\n=== 组合子模式 ===");

    // Option 组合子
    let numbers = vec![Some(1), None, Some(3), Some(4)];
    let processed: Vec<i32> = numbers
        .into_iter()
        .filter_map(|opt| opt.map(|x| x * 2))
        .collect();
    println!("处理后的数字: {:?}", processed);

    // Result 组合子
    demo_result_combinators();
}

fn demo_result_combinators() {
    let result = safe_divide(10.0, 2.0)
        .and_then(|x| safe_sqrt(x));

    match result {
        Ok(value) => println!("计算结果: {:.2}", value),
        Err(e) => println!("计算错误: {}", e),
    }
}

fn safe_divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b != 0.0 {
        Ok(a / b)
    } else {
        Err("除零错误")
    }
}

fn safe_sqrt(x: f64) -> Result<f64, &'static str> {
    if x >= 0.0 {
        Ok(x.sqrt())
    } else {
        Err("负数开方错误")
    }
}

