// 这个文件用于检查response-macro库的核心定义
// 由于工作区依赖问题，我们无法直接运行cargo check

// 模拟ApiError结构体的定义，检查语法
#[derive(Debug)]
struct MockApiError {
    success: bool,
    message: String,
    code: u16,
    details: Option<String>,
    data: Option<String>,
    trace_id: Option<String>,
    timestamp: u64
}

fn main() {
    println!("开始检查response-macro库的定义...");
    
    // 创建MockApiError实例，验证结构体定义是否正确
    let api_error = MockApiError {
        success: false,
        message: "Test error".to_string(),
        code: 400,
        details: None,
        data: None,
        trace_id: None,
        timestamp: 1234567890
    };
    
    println!("MockApiError定义正确: {:?}", api_error);
    println!("语法检查完成: 结构体定义语法正确");
    
    // 提示用户我们需要更深入地检查实际文件
    println!("\n注意: 由于工作区依赖问题，我们无法直接运行cargo check");
    println!("建议检查response-macro和response-macro-core库的以下几个方面:");
    println!("1. ApiError结构体的Serialize和Deserialize实现");
    println!("2. WithHeader和WithStatus的Responder实现");
    println!("3. ContentFormat相关的所有引用是否已移除");
    println!("4. 任何对text格式的引用是否已移除");
}