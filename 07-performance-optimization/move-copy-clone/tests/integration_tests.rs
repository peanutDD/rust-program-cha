//! Move, Copy, Clone 集成测试
//! 
//! 这个文件包含了对所有模块功能的集成测试，确保代码的正确性和完整性。

use move_copy_clone::*;

#[test]
fn test_library_info() {
    // 测试库信息打印不会 panic
    print_library_info();
}

#[test]
fn test_quick_demo() {
    // 测试快速演示不会 panic
    quick_demo();
}

#[test]
fn test_run_all_examples() {
    // 测试运行所有示例不会 panic
    run_all_examples();
}

#[test]
fn test_move_semantics_module() {
    // 测试 Move 语义模块
    move_semantics::run_move_examples();
}

#[test]
fn test_copy_trait_module() {
    // 测试 Copy trait 模块
    copy_trait::run_copy_examples();
}

#[test]
fn test_clone_trait_module() {
    // 测试 Clone trait 模块
    clone_trait::run_clone_examples();
}

#[test]
fn test_comparison_module() {
    // 测试对比分析模块
    comparison::run_comparison_analysis();
}

#[test]
fn test_practical_examples_module() {
    // 测试实际应用案例模块
    practical_examples::run_all_examples();
}

#[test]
fn test_edge_cases_module() {
    // 测试边界情况模块
    edge_cases::run_all_edge_cases();
}

#[test]
fn test_performance_module() {
    // 测试性能分析模块
    performance::run_all_performance_analysis();
}

// 具体功能测试

#[test]
fn test_move_semantics() {
    // 测试基本的 Move 语义
    let s1 = String::from("hello");
    let s2 = s1; // s1 moved to s2
    assert_eq!(s2, "hello");
    // s1 不能再使用，这是编译时检查
}

#[test]
fn test_copy_trait() {
    // 测试 Copy trait
    let x = 5;
    let y = x; // x copied to y
    assert_eq!(x, 5);
    assert_eq!(y, 5);
    
    // 测试自定义 Copy 类型
    #[derive(Copy, Clone, PartialEq, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1; // p1 copied to p2
    assert_eq!(p1, Point { x: 1, y: 2 });
    assert_eq!(p2, Point { x: 1, y: 2 });
}

#[test]
fn test_clone_trait() {
    // 测试 Clone trait
    let v1 = vec![1, 2, 3];
    let v2 = v1.clone(); // v1 cloned to v2
    assert_eq!(v1, vec![1, 2, 3]);
    assert_eq!(v2, vec![1, 2, 3]);
    
    // 测试自定义 Clone 类型
    #[derive(Clone, PartialEq, Debug)]
    struct Data {
        values: Vec<i32>,
    }
    
    let d1 = Data { values: vec![1, 2, 3] };
    let d2 = d1.clone();
    assert_eq!(d1.values, vec![1, 2, 3]);
    assert_eq!(d2.values, vec![1, 2, 3]);
}

#[test]
fn test_smart_pointer_cloning() {
    use std::rc::Rc;
    use std::sync::Arc;
    
    // 测试 Rc 克隆
    let rc1 = Rc::new(vec![1, 2, 3]);
    let rc2 = rc1.clone();
    assert_eq!(*rc1, vec![1, 2, 3]);
    assert_eq!(*rc2, vec![1, 2, 3]);
    assert_eq!(Rc::strong_count(&rc1), 2);
    
    // 测试 Arc 克隆
    let arc1 = Arc::new(vec![1, 2, 3]);
    let arc2 = arc1.clone();
    assert_eq!(*arc1, vec![1, 2, 3]);
    assert_eq!(*arc2, vec![1, 2, 3]);
    assert_eq!(Arc::strong_count(&arc1), 2);
}

#[test]
fn test_partial_move() {
    // 测试部分移动
    struct Container {
        field1: String,
        field2: i32,
    }
    
    let container = Container {
        field1: String::from("hello"),
        field2: 42,
    };
    
    let moved_field = container.field1; // 部分移动
    let copied_field = container.field2; // Copy
    
    assert_eq!(moved_field, "hello");
    assert_eq!(copied_field, 42);
    // container.field1 不能再使用
    // container.field2 仍然可以使用
}

#[test]
fn test_closure_capture() {
    // 测试闭包捕获
    let data = vec![1, 2, 3];
    
    // 通过引用捕获
    let closure1 = || {
        println!("Data length: {}", data.len());
    };
    closure1();
    
    // data 仍然可用
    assert_eq!(data.len(), 3);
    
    // 通过移动捕获
    let closure2 = move || {
        println!("Data: {:?}", data);
    };
    closure2();
    
    // data 已被移动到闭包中，不能再使用
}

#[test]
fn test_option_take() {
    // 测试 Option::take()
    let mut opt = Some(String::from("hello"));
    
    let taken = opt.take();
    assert_eq!(taken, Some(String::from("hello")));
    assert_eq!(opt, None);
}

#[test]
fn test_cow_optimization() {
    use std::borrow::Cow;
    
    // 测试 Cow 优化
    fn process_text(input: Cow<str>) -> Cow<str> {
        if input.contains("modify") {
            Cow::Owned(input.replace("modify", "changed"))
        } else {
            input
        }
    }
    
    // 不需要修改的情况
    let text1 = "hello world";
    let result1 = process_text(Cow::Borrowed(text1));
    assert_eq!(result1, "hello world");
    
    // 需要修改的情况
    let text2 = "please modify this";
    let result2 = process_text(Cow::Borrowed(text2));
    assert_eq!(result2, "please changed this");
}

#[test]
fn test_performance_characteristics() {
    use std::time::Instant;
    
    // 简单的性能特征测试
    let large_vec = vec![1; 10000];
    
    // 测试 Move（应该很快）
    let start = Instant::now();
    let moved_vec = large_vec;
    let move_duration = start.elapsed();
    
    // 测试 Clone（相对较慢）
    let start = Instant::now();
    let cloned_vec = moved_vec.clone();
    let clone_duration = start.elapsed();
    
    // Move 应该比 Clone 快得多
    println!("Move duration: {:?}", move_duration);
    println!("Clone duration: {:?}", clone_duration);
    
    // 验证数据正确性
    assert_eq!(moved_vec.len(), 10000);
    assert_eq!(cloned_vec.len(), 10000);
}

#[test]
fn test_memory_safety() {
    // 测试内存安全特性
    
    // 防止悬垂指针
    fn create_and_return() -> String {
        let s = String::from("hello");
        s // 返回时移动，不会产生悬垂指针
    }
    
    let result = create_and_return();
    assert_eq!(result, "hello");
    
    // 防止双重释放
    let s1 = String::from("hello");
    let s2 = s1; // s1 移动到 s2
    // s1 不能再使用，防止双重释放
    assert_eq!(s2, "hello");
}

#[test]
fn test_type_system_enforcement() {
    // 测试类型系统强制执行的规则
    
    // Copy 类型不能同时实现 Drop
    #[derive(Copy, Clone)]
    struct CopyType {
        value: i32,
    }
    
    let ct1 = CopyType { value: 42 };
    let ct2 = ct1; // Copy
    assert_eq!(ct1.value, 42);
    assert_eq!(ct2.value, 42);
    
    // 非 Copy 类型的移动语义
    struct NonCopyType {
        data: Vec<i32>,
    }
    
    let nct1 = NonCopyType { data: vec![1, 2, 3] };
    let nct2 = nct1; // Move
    assert_eq!(nct2.data, vec![1, 2, 3]);
    // nct1 不能再使用
}

#[test]
fn test_generic_constraints() {
    // 测试泛型约束
    
    fn copy_and_return<T: Copy>(value: T) -> (T, T) {
        (value, value) // 两次使用，需要 Copy
    }
    
    fn clone_and_return<T: Clone>(value: T) -> (T, T) {
        let cloned = value.clone();
        (value, cloned)
    }
    
    // 测试 Copy 约束
    let (a, b) = copy_and_return(42);
    assert_eq!(a, 42);
    assert_eq!(b, 42);
    
    // 测试 Clone 约束
    let vec = vec![1, 2, 3];
    let (original, cloned) = clone_and_return(vec);
    assert_eq!(original, vec![1, 2, 3]);
    assert_eq!(cloned, vec![1, 2, 3]);
}