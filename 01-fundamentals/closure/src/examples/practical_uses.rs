//! # 实用场景

/// 演示实用场景
pub fn demo_practical_uses() {
    println!("\n=== 实用场景 ===");

    // 策略模式
    demo_strategy_pattern();
}

fn demo_strategy_pattern() {
    println!("\n--- 策略模式 ---");

    struct Calculator {
        operation: Box<dyn Fn(f64, f64) -> f64>,
    }

    impl Calculator {
        fn new<F>(op: F) -> Self
        where
            F: Fn(f64, f64) -> f64 + 'static,
        {
            Calculator {
                operation: Box::new(op),
            }
        }

        fn calculate(&self, a: f64, b: f64) -> f64 {
            (self.operation)(a, b)
        }
    }

    let adder = Calculator::new(|a, b| a + b);
    let multiplier = Calculator::new(|a, b| a * b);

    println!("5 + 3 = {}", adder.calculate(5.0, 3.0));
    println!("5 * 3 = {}", multiplier.calculate(5.0, 3.0));
}

