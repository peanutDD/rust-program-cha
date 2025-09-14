//! # Pin 和 Unpin 深度分析
//! 
//! 本项目全面深入分析 Rust 中的 Pin 和 Unpin 概念，涵盖：
//! - Pin 和 Unpin 的基础概念和设计原理
//! - 内存安全机制和自引用结构体问题
//! - Pin API 的详细使用方法
//! - 异步编程中的 Pin 应用
//! - 自引用结构体的安全实现
//! - Pin 投影和实际应用案例

use std::pin::Pin;
use std::marker::{PhantomPinned, Unpin};
use std::ptr::NonNull;
use std::future::Future;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};
use std::mem;
use std::boxed::Box;

#[tokio::main]
async fn main() {
    println!("🔒 Pin 和 Unpin 深度分析");
    println!("{}", "=".repeat(60));
    
    // 第一部分：Pin 和 Unpin 基础概念
    pin_unpin_basics().await;
    
    // 第二部分：内存安全机制
    memory_safety_analysis().await;
    
    // 演示用户提供的自引用结构体问题
    demonstrate_user_example().await;
    
    // 第三部分：Pin API 详解
    pin_api_deep_dive().await;
    
    // 第四部分：Unpin trait 深入
    unpin_trait_analysis().await;
    
    // 第五部分：异步上下文中的 Pin
    async_context_pin().await;
    
    // 第六部分：自引用结构体实现
    self_referential_structs().await;
    
    // 第七部分：Pin 投影
    pin_projection_demo().await;
    
    // 第八部分：实际应用案例
    practical_examples().await;
    
    println!("\n🎯 Pin 和 Unpin 分析完成！通过这些示例，你应该对 Pin 和 Unpin 有了深入理解。");
}

/// 第一部分：Pin 和 Unpin 基础概念
async fn pin_unpin_basics() {
    println!("\n📚 第一部分：Pin 和 Unpin 基础概念");
    println!("{}", "-".repeat(50));
    
    // 1. Pin 的定义和作用
    println!("\n1. Pin 的定义和核心作用：");
    println!("Pin<P> 是一个智能指针包装器，用于防止被包装的值在内存中移动");
    println!("主要解决自引用结构体的内存安全问题");
    
    // 2. Unpin trait 的含义
    println!("\n2. Unpin trait 的含义：");
    println!("Unpin 是一个 marker trait，表示类型可以安全地从 Pin 中移动");
    println!("大多数类型都自动实现了 Unpin");
    
    // 3. 为什么需要 Pin？
    println!("\n3. 为什么需要 Pin？");
    demonstrate_why_pin_needed();
    
    // 4. Pin 的设计哲学
    println!("\n4. Pin 的设计哲学：");
    println!("- 零成本抽象：Pin 在运行时没有额外开销");
    println!("- 编译时保证：通过类型系统确保内存安全");
    println!("- 最小化影响：只影响需要固定的类型");
    
    println!("✅ Pin 和 Unpin 基础概念演示完成");
}

/// 演示为什么需要 Pin
fn demonstrate_why_pin_needed() {
    println!("\n问题场景：自引用结构体");
    println!("```rust");
    println!("struct SelfReferential {{");
    println!("    data: String,");
    println!("    pointer: *const String, // 指向 data 字段");
    println!("}}");
    println!("```");
    
    println!("\n问题：如果结构体被移动，指针就会变成悬垂指针！");
    println!("Pin 通过防止移动来解决这个问题。");
}

/// 第二部分：内存安全机制
async fn memory_safety_analysis() {
    println!("\n🛡️ 第二部分：内存安全机制");
    println!("{}", "-".repeat(50));
    
    // 1. 内存移动的问题
    println!("\n1. 内存移动问题分析：");
    demonstrate_memory_move_problem();
    
    // 2. Pin 如何解决移动问题
    println!("\n2. Pin 如何解决移动问题：");
    demonstrate_pin_solution();
    
    // 3. 栈固定 vs 堆固定
    println!("\n3. 栈固定 vs 堆固定：");
    demonstrate_stack_vs_heap_pinning().await;
    
    // 4. 内存安全保证
    println!("\n4. 内存安全保证机制：");
    demonstrate_safety_guarantees();
    
    println!("✅ 内存安全机制分析完成");
}

/// 演示内存移动问题
fn demonstrate_memory_move_problem() {
    println!("内存移动问题示例：");
    println!("```rust");
    println!("let mut data = String::from(\"hello\");");
    println!("let ptr = &data as *const String;");
    println!("let moved_data = data; // data 被移动！");
    println!("// ptr 现在是悬垂指针");
    println!("```");
    
    println!("\n后果：");
    println!("- 悬垂指针导致未定义行为");
    println!("- 内存安全问题");
    println!("- 难以调试的运行时错误");
}

/// 演示 Pin 的解决方案
fn demonstrate_pin_solution() {
    println!("Pin 的解决方案：");
    println!("```rust");
    println!("let data = Box::pin(String::from(\"hello\"));");
    println!("// data 现在被固定在内存中，无法移动");
    println!("let ptr = data.as_ref().get_ref() as *const String;");
    println!("// ptr 始终有效，因为 data 不会移动");
    println!("```");
    
    println!("\n优势：");
    println!("- 编译时防止移动");
    println!("- 保证指针有效性");
    println!("- 零运行时开销");
}

/// 演示栈固定 vs 堆固定
async fn demonstrate_stack_vs_heap_pinning() {
    println!("栈固定 vs 堆固定对比：");
    
    // 堆固定示例
    println!("\n堆固定（Box::pin）：");
    let heap_pinned = Box::pin(42);
    println!("值 {} 被固定在堆上", heap_pinned.as_ref().get_ref());
    
    // 栈固定示例（使用 pin! 宏）
    println!("\n栈固定（pin! 宏）：");
    tokio::pin! {
        let stack_pinned = 42;
    }
    println!("值 {} 被固定在栈上", stack_pinned.as_ref().get_ref());
    
    println!("\n区别：");
    println!("- 堆固定：分配在堆上，生命周期更灵活");
    println!("- 栈固定：分配在栈上，性能更好但生命周期受限");
}

/// 演示安全保证
fn demonstrate_safety_guarantees() {
    println!("Pin 的安全保证：");
    println!("\n1. 类型安全：");
    println!("   - Pin<P> 只能通过安全的方式创建");
    println!("   - 编译器防止不安全的操作");
    
    println!("\n2. 内存安全：");
    println!("   - 防止悬垂指针");
    println!("   - 保证引用有效性");
    
    println!("\n3. 线程安全：");
    println!("   - Pin 不影响 Send/Sync 特性");
    println!("   - 可以安全地在线程间传递");
}

// 用户提供的自引用结构体示例
struct Test {
    a: String,
    b: *const String, // 指向 a 的原始指针
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }
    
    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }
    
    fn a(&self) -> &str {
        &self.a
    }
    
    fn b(&self) -> &String {
        assert!(!self.b.is_null(), "Test::b called when not initialized!");
        unsafe { &*(self.b) }
    }
}

async fn demonstrate_user_example() {
    println!("\n🚨 用户示例：自引用结构体的内存安全问题");
    println!("{}", "-".repeat(50));
    
    println!("\n1. 问题演示 - 危险的内存交换：");
    
    // 这段代码展示了自引用结构体的问题
    unsafe {
        let mut test1 = Test::new("test1");
        test1.init();
        let mut test2 = Test::new("test2");
        test2.init();
        
        println!("交换前:");
        println!("   test1.a: {}, test1.b: {}", test1.a(), test1.b());
        println!("   test2.a: {}, test2.b: {}", test2.a(), test2.b());
        
        // 危险操作：交换两个自引用结构体
        std::mem::swap(&mut test1, &mut test2);
        
        println!("\n交换后 - 指针现在指向错误的内存位置！");
        println!("   test1.a: {}", test1.a());
        // test1.b() 现在指向 test2 原来的内存位置，这是悬垂指针！
        // println!("   test1.b: {}", test1.b()); // 这会导致未定义行为
        
        println!("   test2.a: {}", test2.a());
        // println!("   test2.b: {}", test2.b()); // 这也会导致未定义行为
    }
    
    println!("\n2. Pin 如何解决这个问题：");
    demonstrate_pin_solution_for_user_example().await;
}

// Pin 版本的安全实现
struct PinnedTest {
    a: String,
    b: *const String,
    _pin: PhantomPinned, // 标记为 !Unpin
}

impl PinnedTest {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let test = PinnedTest {
            a: String::from(txt),
            b: std::ptr::null(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(test);
        let self_ptr: *const String = &boxed.a;
        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).b = self_ptr;
        }
        boxed
    }
    
    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }
    
    fn b(self: Pin<&Self>) -> &String {
        assert!(!self.b.is_null(), "PinnedTest::b called when not initialized!");
        unsafe { &*(self.get_ref().b) }
    }
}

async fn demonstrate_pin_solution_for_user_example() {
    println!("\nPin 解决方案：");
    
    let test1 = PinnedTest::new("pinned_test1");
    let test2 = PinnedTest::new("pinned_test2");
    
    println!("   test1.a: {}, test1.b: {}", test1.as_ref().a(), test1.as_ref().b());
    println!("   test2.a: {}, test2.b: {}", test2.as_ref().a(), test2.as_ref().b());
    
    // 以下代码无法编译，因为 PinnedTest 是 !Unpin
    // std::mem::swap(&mut test1, &mut test2); // 编译错误！
    
    println!("\n✅ Pin 防止了危险的内存交换操作！");
    println!("   - PinnedTest 被标记为 !Unpin");
    println!("   - 编译器阻止 std::mem::swap");
    println!("   - 自引用指针始终有效");
    println!("   - 内存安全得到保证");
}

/// 第三部分：Pin API 详解
async fn pin_api_deep_dive() {
    println!("\n🔧 第三部分：Pin API 详解");
    println!("{}", "-".repeat(50));
    
    // 1. Pin::new 和 Pin::new_unchecked
    println!("\n1. Pin 创建方法：");
    demonstrate_pin_creation();
    
    // 2. Pin 的方法
    println!("\n2. Pin 的核心方法：");
    demonstrate_pin_methods().await;
    
    // 3. Pin 的解引用
    println!("\n3. Pin 的解引用机制：");
    demonstrate_pin_deref();
    
    // 4. Pin 的转换
    println!("\n4. Pin 的类型转换：");
    demonstrate_pin_conversion();
    
    println!("✅ Pin API 详解完成");
}

/// 演示 Pin 创建方法
fn demonstrate_pin_creation() {
    println!("Pin 的创建方法：");
    
    // Pin::new - 只能用于 Unpin 类型
    println!("\n1. Pin::new（仅适用于 Unpin 类型）：");
    let value = 42;
    let pinned = Pin::new(&value);
    println!("   创建了 Pin<&i32>: {}", pinned.get_ref());
    
    // Box::pin - 堆分配并固定
    println!("\n2. Box::pin（堆分配并固定）：");
    let boxed_pin = Box::pin(String::from("hello"));
    println!("   创建了 Pin<Box<String>>: {}", boxed_pin.as_ref().get_ref());
    
    // Pin::new_unchecked - 不安全方法
    println!("\n3. Pin::new_unchecked（不安全方法）：");
    println!("   unsafe {{ Pin::new_unchecked(&value) }}");
    println!("   ⚠️ 需要手动保证不会移动值");
}

/// 演示 Pin 的方法
async fn demonstrate_pin_methods() {
    println!("Pin 的核心方法：");
    
    let pinned_string = Box::pin(String::from("Hello, Pin!"));
    
    // as_ref
    println!("\n1. as_ref() - 获取 Pin<&T>：");
    let pin_ref = pinned_string.as_ref();
    println!("   Pin<&String>: {}", pin_ref.get_ref());
    
    // get_ref
    println!("\n2. get_ref() - 获取 &T（仅限 Unpin）：");
    println!("   &String: {}", pin_ref.get_ref());
    
    // as_ref for conversion
    println!("\n3. as_ref() - 转换为 Pin<&T>：");
    let another_pinned = Box::pin(42);
    let pin_ref = another_pinned.as_ref();
    println!("   Pin<&i32>: {}", pin_ref.get_ref());
    
    // map_unchecked
    println!("\n4. map_unchecked() - 不安全映射：");
    println!("   用于访问 Pin 内部的字段");
    println!("   ⚠️ 需要保证映射后的值也不会移动");
}

/// 演示 Pin 的解引用
fn demonstrate_pin_deref() {
    println!("Pin 的解引用机制：");
    
    let pinned = Box::pin(String::from("Hello"));
    
    println!("\n1. 通过 Deref trait：");
    println!("   Pin<Box<String>> 可以调用 String 的方法");
    println!("   长度: {}", pinned.len());
    
    println!("\n2. 限制：");
    println!("   - 不能获取 &mut T（除非 T: Unpin）");
    println!("   - 不能移动内部值");
    println!("   - 保护内存安全");
}

/// 演示 Pin 的转换
fn demonstrate_pin_conversion() {
    println!("Pin 的类型转换：");
    
    println!("\n1. Pin<Box<T>> -> Pin<&T>：");
    let boxed = Box::pin(String::from("conversion"));
    let pinned_ref = boxed.as_ref();
    println!("   转换成功: {}", pinned_ref.get_ref());
    
    println!("\n2. Pin<&mut T> -> Pin<&T>：");
    println!("   可以从可变引用转换为不可变引用");
    
    println!("\n3. 协变性：");
    println!("   Pin<P> 在 P 上是协变的");
    println!("   Pin<&'a T> 可以转换为 Pin<&'b T>（'a: 'b）");
}

/// 第四部分：Unpin trait 深入
async fn unpin_trait_analysis() {
    println!("\n🏷️ 第四部分：Unpin trait 深入");
    println!("{}", "-".repeat(50));
    
    // 1. Unpin 的定义
    println!("\n1. Unpin trait 定义：");
    demonstrate_unpin_definition();
    
    // 2. 自动实现 Unpin
    println!("\n2. 自动实现 Unpin：");
    demonstrate_auto_unpin();
    
    // 3. !Unpin 类型
    println!("\n3. !Unpin 类型：");
    demonstrate_not_unpin().await;
    
    // 4. PhantomPinned
    println!("\n4. PhantomPinned 的使用：");
    demonstrate_phantom_pinned();
    
    println!("✅ Unpin trait 分析完成");
}

/// 演示 Unpin 定义
fn demonstrate_unpin_definition() {
    println!("Unpin trait 的定义：");
    println!("```rust");
    println!("pub auto trait Unpin {{}}");
    println!("```");
    
    println!("\n特点：");
    println!("- auto trait：编译器自动实现");
    println!("- marker trait：没有方法，只是标记");
    println!("- 表示类型可以安全地从 Pin 中移动");
    
    println!("\n语义：");
    println!("- 实现 Unpin：可以安全移动");
    println!("- 不实现 Unpin：移动可能不安全");
}

/// 演示自动实现 Unpin
fn demonstrate_auto_unpin() {
    println!("自动实现 Unpin 的类型：");
    
    // 基本类型
    println!("\n1. 基本类型：");
    let num = 42;
    let pinned_num = Pin::new(&num);
    println!("   i32 实现了 Unpin: {}", pinned_num.get_ref());
    
    // 标准库类型
    println!("\n2. 标准库类型：");
    let string = String::from("auto unpin");
    let pinned_string = Pin::new(&string);
    println!("   String 实现了 Unpin: {}", pinned_string.get_ref());
    
    // 结构体
    #[derive(Debug)]
    struct AutoUnpin {
        value: i32,
    }
    
    println!("\n3. 普通结构体：");
    let auto_unpin = AutoUnpin { value: 42 };
    let pinned_struct = Pin::new(&auto_unpin);
    println!("   AutoUnpin 自动实现了 Unpin: {:?}", pinned_struct.get_ref());
    
    println!("\n规则：");
    println!("- 所有字段都实现 Unpin -> 结构体自动实现 Unpin");
    println!("- 任何字段不实现 Unpin -> 结构体不实现 Unpin");
}

/// 演示 !Unpin 类型
async fn demonstrate_not_unpin() {
    println!("!Unpin 类型示例：");
    
    // 自引用结构体
    println!("\n1. 自引用结构体：");
    println!("```rust");
    println!("struct SelfRef {{");
    println!("    data: String,");
    println!("    pointer: *const String,");
    println!("    _pin: PhantomPinned, // 阻止自动实现 Unpin");
    println!("}}");
    println!("```");
    
    // Future 类型
    println!("\n2. 某些 Future 类型：");
    println!("   async fn 生成的 Future 可能是 !Unpin");
    
    // 演示 !Unpin Future
    let future = create_not_unpin_future();
    tokio::pin!(future);
    let result = future.await;
    println!("   !Unpin Future 执行结果: {}", result);
    
    println!("\n特点：");
    println!("- 不能使用 Pin::new");
    println!("- 必须使用 Box::pin 或 pin! 宏");
    println!("- 需要特殊处理");
}

/// 创建一个 !Unpin 的 Future
async fn create_not_unpin_future() -> i32 {
    // 这个 Future 包含自引用，因此是 !Unpin
    let data = String::from("future data");
    let _ptr = &data as *const String;
    42
}

/// 演示 PhantomPinned
fn demonstrate_phantom_pinned() {
    println!("PhantomPinned 的使用：");
    
    // 定义包含 PhantomPinned 的结构体
    struct NotUnpin {
        data: String,
        _pin: PhantomPinned,
    }
    
    println!("\n1. 阻止自动实现 Unpin：");
    println!("```rust");
    println!("struct NotUnpin {{");
    println!("    data: String,");
    println!("    _pin: PhantomPinned, // 关键！");
    println!("}}");
    println!("```");
    
    println!("\n2. 效果：");
    println!("   - NotUnpin 不会自动实现 Unpin");
    println!("   - 必须使用 Box::pin 或其他方式固定");
    
    // 创建实例
    let not_unpin = NotUnpin {
        data: String::from("pinned data"),
        _pin: PhantomPinned,
    };
    
    let pinned = Box::pin(not_unpin);
    println!("   成功创建 Pin<Box<NotUnpin>>: {}", pinned.data);
    
    println!("\n3. 用途：");
    println!("   - 自引用结构体");
    println!("   - 需要固定语义的类型");
    println!("   - 异步状态机");
}

/// 第五部分：异步上下文中的 Pin
async fn async_context_pin() {
    println!("\n⚡ 第五部分：异步上下文中的 Pin");
    println!("{}", "-".repeat(50));
    
    // 1. Future trait 和 Pin
    println!("\n1. Future trait 和 Pin：");
    demonstrate_future_pin();
    
    // 2. async/await 和状态机
    println!("\n2. async/await 和状态机：");
    demonstrate_async_state_machine().await;
    
    // 3. 自定义 Future 实现
    println!("\n3. 自定义 Future 实现：");
    demonstrate_custom_future().await;
    
    // 4. Pin 在异步运行时中的作用
    println!("\n4. Pin 在异步运行时中的作用：");
    demonstrate_runtime_pin().await;
    
    println!("✅ 异步上下文中的 Pin 分析完成");
}

/// 演示 Future trait 和 Pin
fn demonstrate_future_pin() {
    println!("Future trait 的定义：");
    println!("```rust");
    println!("trait Future {{");
    println!("    type Output;");
    println!("    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;");
    println!("}}");
    println!("```");
    
    println!("\n关键点：");
    println!("- poll 方法接收 Pin<&mut Self>");
    println!("- 保证 Future 在轮询期间不会移动");
    println!("- 支持自引用的异步状态机");
    
    println!("\n为什么需要 Pin？");
    println!("- async fn 可能生成自引用的状态机");
    println!("- 跨 await 点的局部变量可能相互引用");
    println!("- 移动会导致悬垂指针");
}

/// 演示 async/await 状态机
async fn demonstrate_async_state_machine() {
    println!("async/await 状态机示例：");
    
    println!("\n原始 async 函数：");
    println!("```rust");
    println!("async fn example() -> i32 {{");
    println!("    let x = 1;");
    println!("    let y = async_operation().await;");
    println!("    x + y");
    println!("}}");
    println!("```");
    
    println!("\n编译器生成的状态机（简化）：");
    println!("```rust");
    println!("enum ExampleFuture {{");
    println!("    Start {{ x: i32 }},");
    println!("    Waiting {{ x: i32, future: SomeFuture }},");
    println!("    Done,");
    println!("}}");
    println!("```");
    
    // 实际执行异步操作
    let result = example_async_function().await;
    println!("\n实际执行结果: {}", result);
    
    println!("\n状态机特点：");
    println!("- 可能包含自引用");
    println!("- 需要 Pin 保证内存安全");
    println!("- 零成本抽象");
}

/// 示例异步函数
async fn example_async_function() -> i32 {
    let x = 1;
    let _y = tokio::time::sleep(Duration::from_millis(1)).await;
    x + 41 // 返回 42
}

/// 演示自定义 Future 实现
async fn demonstrate_custom_future() {
    println!("自定义 Future 实现：");
    
    // 使用自定义 Future
    let custom_future = CustomFuture::new(Duration::from_millis(100));
    let result = custom_future.await;
    println!("\n自定义 Future 执行结果: {}", result);
    
    println!("\n实现要点：");
    println!("- poll 方法接收 Pin<&mut Self>");
    println!("- 可以安全地存储自引用");
    println!("- 与异步运行时集成");
}

/// 自定义 Future 实现
struct CustomFuture {
    start_time: Option<Instant>,
    duration: Duration,
    _pin: PhantomPinned, // 使其成为 !Unpin
}

impl CustomFuture {
    fn new(duration: Duration) -> Self {
        Self {
            start_time: None,
            duration,
            _pin: PhantomPinned,
        }
    }
}

impl Future for CustomFuture {
    type Output = String;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        
        let start_time = this.start_time.get_or_insert_with(Instant::now);
        
        if start_time.elapsed() >= this.duration {
            Poll::Ready("Custom Future completed!".to_string())
        } else {
            // 注册 waker 以便稍后唤醒
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// 演示 Pin 在异步运行时中的作用
async fn demonstrate_runtime_pin() {
    println!("Pin 在异步运行时中的作用：");
    
    println!("\n1. 任务调度：");
    println!("   - 运行时需要在堆上存储 Future");
    println!("   - Pin<Box<dyn Future>> 确保 Future 不会移动");
    
    println!("\n2. 跨线程执行：");
    println!("   - Future 可能在不同线程间迁移");
    println!("   - Pin 保证内存布局稳定");
    
    println!("\n3. 生命周期管理：");
    println!("   - Pin 确保异步任务的内存安全");
    println!("   - 防止悬垂指针和内存泄漏");
    
    // 演示任务创建
    let task = tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(10)).await;
        "Task completed"
    });
    
    let result = task.await.unwrap();
    println!("\n任务执行结果: {}", result);
    
    println!("\n运行时保证：");
    println!("- 内存安全");
    println!("- 性能优化");
    println!("- 并发正确性");
}

/// 第六部分：自引用结构体实现
async fn self_referential_structs() {
    println!("\n🔗 第六部分：自引用结构体实现");
    println!("{}", "-".repeat(50));
    
    // 1. 自引用结构体的挑战
    println!("\n1. 自引用结构体的挑战：");
    demonstrate_self_ref_challenges();
    
    // 2. 安全的自引用实现
    println!("\n2. 安全的自引用实现：");
    demonstrate_safe_self_ref();
    
    // 3. 构造和初始化
    println!("\n3. 构造和初始化：");
    demonstrate_self_ref_construction().await;
    
    // 4. 生命周期管理
    println!("\n4. 生命周期管理：");
    demonstrate_lifetime_management();
    
    println!("✅ 自引用结构体实现完成");
}

/// 演示自引用结构体的挑战
fn demonstrate_self_ref_challenges() {
    println!("自引用结构体的挑战：");
    
    println!("\n问题示例：");
    println!("```rust");
    println!("struct SelfRef {{");
    println!("    data: String,");
    println!("    ptr: *const String, // 指向 data");
    println!("}}");
    println!("```");
    
    println!("\n挑战：");
    println!("1. 构造困难：");
    println!("   - 无法在构造时获取 self 引用");
    println!("   - 需要两阶段初始化");
    
    println!("\n2. 移动问题：");
    println!("   - 移动后指针失效");
    println!("   - 内存安全风险");
    
    println!("\n3. 生命周期复杂：");
    println!("   - 内部引用的生命周期管理");
    println!("   - 借用检查器限制");
}

/// 演示安全的自引用实现
fn demonstrate_safe_self_ref() {
    println!("安全的自引用实现策略：");
    
    println!("\n1. 使用 Pin + PhantomPinned：");
    println!("```rust");
    println!("struct SafeSelfRef {{");
    println!("    data: String,");
    println!("    ptr: Option<NonNull<String>>,");
    println!("    _pin: PhantomPinned,");
    println!("}}");
    println!("```");
    
    println!("\n2. 两阶段构造：");
    println!("   - 第一阶段：创建基本结构");
    println!("   - 第二阶段：建立内部引用");
    
    println!("\n3. Pin 保护：");
    println!("   - 防止移动");
    println!("   - 保证指针有效性");
    
    // 实际创建安全的自引用结构体
    let safe_self_ref = SafeSelfRef::new("Hello, self-reference!".to_string());
    println!("\n创建成功: {}", safe_self_ref.get_data());
}

/// 安全的自引用结构体
struct SafeSelfRef {
    data: String,
    ptr: Option<NonNull<String>>,
    _pin: PhantomPinned,
}

impl SafeSelfRef {
    fn new(data: String) -> Pin<Box<Self>> {
        let mut boxed = Box::pin(SafeSelfRef {
            data,
            ptr: None,
            _pin: PhantomPinned,
        });
        
        // 安全地建立自引用
        let ptr = NonNull::from(&boxed.data);
        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).ptr = Some(ptr);
        }
        
        boxed
    }
    
    fn get_data(&self) -> &str {
        &self.data
    }
    
    fn get_ptr_data(&self) -> Option<&str> {
        self.ptr.map(|ptr| unsafe { ptr.as_ref() }.as_str())
    }
}

/// 演示自引用结构体的构造
async fn demonstrate_self_ref_construction() {
    println!("自引用结构体的构造过程：");
    
    println!("\n1. 创建 SafeSelfRef：");
    let self_ref = SafeSelfRef::new("Construction example".to_string());
    
    println!("   原始数据: {}", self_ref.get_data());
    if let Some(ptr_data) = self_ref.get_ptr_data() {
        println!("   指针数据: {}", ptr_data);
        println!("   ✅ 自引用建立成功！");
    }
    
    println!("\n2. 构造步骤：");
    println!("   a) 创建 Box::pin 包装的结构体");
    println!("   b) 获取内部数据的指针");
    println!("   c) 安全地设置自引用指针");
    println!("   d) 返回固定的结构体");
    
    println!("\n3. 安全保证：");
    println!("   - Pin 防止移动");
    println!("   - NonNull 保证指针有效");
    println!("   - PhantomPinned 标记 !Unpin");
}

/// 演示生命周期管理
fn demonstrate_lifetime_management() {
    println!("自引用结构体的生命周期管理：");
    
    println!("\n1. 内存布局：");
    println!("   - 数据和指针在同一内存块中");
    println!("   - Pin 保证内存位置不变");
    
    println!("\n2. 生命周期规则：");
    println!("   - 结构体生命周期 >= 内部引用生命周期");
    println!("   - Pin 确保这一点");
    
    println!("\n3. 清理过程：");
    println!("   - Drop 时自动清理");
    println!("   - 无需手动管理指针");
    
    println!("\n4. 最佳实践：");
    println!("   - 使用 Pin<Box<T>> 进行堆分配");
    println!("   - 通过构造函数建立引用");
    println!("   - 避免手动操作指针");
}

/// 第七部分：Pin 投影
async fn pin_projection_demo() {
    println!("\n📍 第七部分：Pin 投影");
    println!("{}", "-".repeat(50));
    
    // 1. Pin 投影的概念
    println!("\n1. Pin 投影的概念：");
    demonstrate_pin_projection_concept();
    
    // 2. 手动 Pin 投影
    println!("\n2. 手动 Pin 投影：");
    demonstrate_manual_projection();
    
    // 3. pin-project 库
    println!("\n3. pin-project 库的使用：");
    demonstrate_pin_project_crate();
    
    // 4. 结构化 Pin
    println!("\n4. 结构化 Pin：");
    demonstrate_structural_pinning().await;
    
    println!("✅ Pin 投影演示完成");
}

/// 演示 Pin 投影概念
fn demonstrate_pin_projection_concept() {
    println!("Pin 投影的概念：");
    
    println!("\n定义：");
    println!("Pin 投影是从 Pin<&mut Struct> 安全地获取 Pin<&mut Field> 的过程");
    
    println!("\n问题：");
    println!("```rust");
    println!("struct Container {{");
    println!("    field1: SomeType,");
    println!("    field2: AnotherType,");
    println!("}}");
    println!("\n// 如何从 Pin<&mut Container> 获取 Pin<&mut field1>？");
    println!("```");
    
    println!("\n挑战：");
    println!("- 不能简单地解引用 Pin");
    println!("- 需要保证字段也被正确固定");
    println!("- 必须维护 Pin 的不变性");
    
    println!("\n解决方案：");
    println!("- 结构化固定（Structural Pinning）");
    println!("- Pin 投影宏");
    println!("- 手动实现投影函数");
}

/// 演示手动 Pin 投影
fn demonstrate_manual_projection() {
    println!("手动 Pin 投影实现：");
    
    // 定义需要投影的结构体
    struct ProjectionExample {
        pinned_field: String,
        unpinned_field: i32,
        _pin: PhantomPinned,
    }
    
    impl ProjectionExample {
        // 安全的 Pin 投影方法
        fn project(self: Pin<&mut Self>) -> (Pin<&mut String>, &mut i32) {
            unsafe {
                let this = self.get_unchecked_mut();
                (
                    Pin::new_unchecked(&mut this.pinned_field),
                    &mut this.unpinned_field,
                )
            }
        }
    }
    
    println!("\n实现要点：");
    println!("1. 使用 get_unchecked_mut 获取可变引用");
    println!("2. 对需要固定的字段使用 Pin::new_unchecked");
    println!("3. 对不需要固定的字段直接返回可变引用");
    
    println!("\n安全性要求：");
    println!("- 必须保证投影后的 Pin 仍然有效");
    println!("- 不能破坏 Pin 的不变性");
    println!("- 需要仔细考虑字段的固定语义");
    
    // 创建示例
    let mut example = Box::pin(ProjectionExample {
        pinned_field: String::from("pinned"),
        unpinned_field: 42,
        _pin: PhantomPinned,
    });
    
    let (pinned_str, unpinned_int) = example.as_mut().project();
    println!("\n投影结果:");
    println!("   固定字段: {}", &*pinned_str);
    println!("   非固定字段: {}", unpinned_int);
}

/// 演示 pin-project 库的概念
fn demonstrate_pin_project_crate() {
    println!("pin-project 库的使用：");
    
    println!("\n1. 基本用法：");
    println!("```rust");
    println!("use pin_project::pin_project;");
    println!("\n#[pin_project]");
    println!("struct Struct<T, U> {{");
    println!("    #[pin]");
    println!("    pinned: T,");
    println!("    unpinned: U,");
    println!("}}");
    println!("```");
    
    println!("\n2. 生成的代码：");
    println!("   - 自动生成 project 方法");
    println!("   - 处理 Pin 投影的复杂性");
    println!("   - 保证内存安全");
    
    println!("\n3. 优势：");
    println!("   - 减少样板代码");
    println!("   - 自动处理安全性");
    println!("   - 编译时检查");
    
    println!("\n4. 使用场景：");
    println!("   - 复杂的异步状态机");
    println!("   - 自引用数据结构");
    println!("   - 需要字段级 Pin 控制的场景");
}

/// 演示结构化固定
async fn demonstrate_structural_pinning() {
    println!("结构化固定（Structural Pinning）：");
    
    println!("\n概念：");
    println!("结构化固定意味着如果结构体被固定，其某些字段也被固定");
    
    println!("\n规则：");
    println!("1. 如果字段类型是 !Unpin，则必须结构化固定");
    println!("2. 如果字段类型是 Unpin，可以选择是否结构化固定");
    println!("3. 一旦选择结构化固定，必须一致维护");
    
    // 演示结构化固定的 Future
    let structural_future = StructuralFuture::new();
    let result = structural_future.await;
    println!("\n结构化固定 Future 结果: {}", result);
    
    println!("\n实现考虑：");
    println!("- Drop 实现必须考虑固定语义");
    println!("- 移动构造函数需要特殊处理");
    println!("- 字段访问需要通过投影");
}

/// 结构化固定的 Future 示例
struct StructuralFuture {
    state: i32,
    _pin: PhantomPinned,
}

impl StructuralFuture {
    fn new() -> Self {
        Self {
            state: 0,
            _pin: PhantomPinned,
        }
    }
}

impl Future for StructuralFuture {
    type Output = i32;
    
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        this.state += 1;
        
        if this.state >= 3 {
            Poll::Ready(this.state)
        } else {
            Poll::Pending
        }
    }
}

/// 第八部分：实际应用案例
async fn practical_examples() {
    println!("\n🚀 第八部分：实际应用案例");
    println!("{}", "-".repeat(50));
    
    // 1. 异步 I/O 中的 Pin
    println!("\n1. 异步 I/O 中的 Pin：");
    demonstrate_async_io_pin().await;
    
    // 2. Future 组合中的 Pin
    println!("\n2. Future 组合中的 Pin：");
    demonstrate_future_combinators().await;
    
    // 3. 自定义异步类型
    println!("\n3. 自定义异步类型：");
    demonstrate_custom_async_types().await;
    
    // 4. 性能优化案例
    println!("\n4. 性能优化案例：");
    demonstrate_performance_optimization().await;
    
    println!("✅ 实际应用案例演示完成");
}

/// 演示异步 I/O 中的 Pin
async fn demonstrate_async_io_pin() {
    println!("异步 I/O 中的 Pin 应用：");
    
    println!("\n场景：异步读取操作");
    println!("```rust");
    println!("async fn read_file(path: &str) -> Result<String, Error> {{");
    println!("    let mut file = File::open(path).await?;");
    println!("    let mut contents = String::new();");
    println!("    file.read_to_string(&mut contents).await?;");
    println!("    Ok(contents)");
    println!("}}");
    println!("```");
    
    println!("\nPin 的作用：");
    println!("- 异步读取操作可能跨多个 await 点");
    println!("- 缓冲区需要保持固定位置");
    println!("- Pin 确保内存安全");
    
    // 模拟异步 I/O 操作
    let io_result = simulate_async_io().await;
    println!("\n模拟 I/O 操作结果: {}", io_result);
    
    println!("\n实际应用：");
    println!("- tokio::fs 文件操作");
    println!("- tokio::net 网络操作");
    println!("- 自定义异步 I/O 驱动");
}

/// 模拟异步 I/O 操作
async fn simulate_async_io() -> String {
    // 模拟异步读取
    tokio::time::sleep(Duration::from_millis(10)).await;
    "Async I/O completed".to_string()
}

/// 演示 Future 组合中的 Pin
async fn demonstrate_future_combinators() {
    println!("Future 组合中的 Pin：");
    
    println!("\n1. join 操作：");
    let future1 = async { "Result 1" };
    let future2 = async { "Result 2" };
    
    let (result1, result2) = tokio::join!(future1, future2);
    println!("   Join 结果: {} + {}", result1, result2);
    
    println!("\n2. select 操作：");
    let fast_future = async {
        tokio::time::sleep(Duration::from_millis(10)).await;
        "Fast"
    };
    let slow_future = async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        "Slow"
    };
    
    tokio::select! {
        result = fast_future => println!("   Select 结果: {}", result),
        result = slow_future => println!("   Select 结果: {}", result),
    }
    
    println!("\nPin 在组合中的作用：");
    println!("- 确保组合的 Future 不会移动");
    println!("- 支持复杂的异步控制流");
    println!("- 保证内存安全");
}

/// 演示自定义异步类型
async fn demonstrate_custom_async_types() {
    println!("自定义异步类型：");
    
    // 使用自定义异步迭代器
    println!("\n1. 异步迭代器：");
    let mut async_iter = AsyncCounter::new(3);
    
    while let Some(value) = async_iter.next().await {
        println!("   异步迭代值: {}", value);
    }
    
    // 使用异步流
    println!("\n2. 异步流：");
    let stream = AsyncStream::new(vec![1, 2, 3]);
    tokio::pin!(stream);
    
    while let Some(item) = stream.as_mut().next().await {
        println!("   流项目: {}", item);
    }
    
    println!("\n自定义类型的 Pin 要求：");
    println!("- 实现适当的 Future/Stream trait");
    println!("- 处理 Pin 语义");
    println!("- 考虑自引用情况");
}

/// 自定义异步计数器
struct AsyncCounter {
    current: usize,
    max: usize,
}

impl AsyncCounter {
    fn new(max: usize) -> Self {
        Self { current: 0, max }
    }
    
    async fn next(&mut self) -> Option<usize> {
        if self.current < self.max {
            let value = self.current;
            self.current += 1;
            tokio::time::sleep(Duration::from_millis(1)).await;
            Some(value)
        } else {
            None
        }
    }
}

/// 自定义异步流
struct AsyncStream {
    items: Vec<i32>,
    index: usize,
}

impl AsyncStream {
    fn new(items: Vec<i32>) -> Self {
        Self { items, index: 0 }
    }
    
    async fn next(self: Pin<&mut Self>) -> Option<i32> {
        let this = unsafe { self.get_unchecked_mut() };
        
        if this.index < this.items.len() {
            let item = this.items[this.index];
            this.index += 1;
            tokio::time::sleep(Duration::from_millis(1)).await;
            Some(item)
        } else {
            None
        }
    }
}

/// 演示性能优化案例
async fn demonstrate_performance_optimization() {
    println!("性能优化案例：");
    
    println!("\n1. 零成本抽象：");
    println!("   Pin 在编译时被优化掉，运行时无开销");
    
    // 性能测试
    let start = Instant::now();
    
    // 使用 Pin 的异步操作
    let pinned_future = Box::pin(async {
        for _ in 0..1000 {
            tokio::task::yield_now().await;
        }
        "Pinned operation completed"
    });
    
    let result = pinned_future.await;
    let duration = start.elapsed();
    
    println!("\n性能测试结果:");
    println!("   操作: {}", result);
    println!("   耗时: {:?}", duration);
    
    println!("\n2. 内存优化：");
    println!("   - 避免不必要的内存拷贝");
    println!("   - 就地操作支持");
    println!("   - 缓存友好的内存布局");
    
    println!("\n3. 编译器优化：");
    println!("   - 内联优化");
    println!("   - 死代码消除");
    println!("   - 循环展开");
    
    println!("\n最佳实践：");
    println!("- 合理使用 Pin，避免过度使用");
    println!("- 优先使用栈固定而非堆固定");
    println!("- 利用编译器优化");
    println!("- 测量实际性能影响");
}
