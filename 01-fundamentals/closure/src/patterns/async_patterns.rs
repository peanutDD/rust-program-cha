//! # 异步编程模式

/// 演示回调模式
pub fn demo_callback_patterns() {
    println!("\n=== 回调模式 ===");

    execute_with_callbacks(
        || {
            if true {
                Ok("操作成功")
            } else {
                Err("操作失败")
            }
        },
        |result| println!("成功回调: {}", result),
        |error| println!("失败回调: {}", error),
    );
}

fn execute_with_callbacks<T, E, Op, OnSuccess, OnError>(
    operation: Op,
    on_success: OnSuccess,
    on_error: OnError,
) where
    Op: FnOnce() -> Result<T, E>,
    OnSuccess: FnOnce(T),
    OnError: FnOnce(E),
{
    match operation() {
        Ok(result) => on_success(result),
        Err(error) => on_error(error),
    }
}

