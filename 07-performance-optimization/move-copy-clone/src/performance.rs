//! # Performance Analysis and Benchmarks
//! 
//! 本模块提供 Move、Copy 和 Clone 的详细性能分析和基准测试。
//! 通过实际测量帮助开发者理解不同机制的性能特征和优化策略。

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::mem;

/// 性能测试结果
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub operation: String,
    pub duration: Duration,
    pub iterations: usize,
    pub avg_ns_per_op: f64,
    pub memory_usage: usize,
}

impl BenchmarkResult {
    pub fn new(operation: String, duration: Duration, iterations: usize, memory_usage: usize) -> Self {
        let avg_ns_per_op = duration.as_nanos() as f64 / iterations as f64;
        Self {
            operation,
            duration,
            iterations,
            avg_ns_per_op,
            memory_usage,
        }
    }
    
    pub fn print(&self) {
        println!(
            "📊 {}: {:.2}ns/op ({} ops, {:.2}ms total, {}B memory)",
            self.operation,
            self.avg_ns_per_op,
            self.iterations,
            self.duration.as_secs_f64() * 1000.0,
            self.memory_usage
        );
    }
}

/// 基准测试框架
struct Benchmark {
    name: String,
    iterations: usize,
}

impl Benchmark {
    fn new(name: &str, iterations: usize) -> Self {
        Self {
            name: name.to_string(),
            iterations,
        }
    }
    
    fn run<F>(&self, mut operation: F) -> BenchmarkResult
    where
        F: FnMut() -> usize,
    {
        // 预热
        for _ in 0..100 {
            operation();
        }
        
        let start = Instant::now();
        let mut total_memory = 0;
        
        for _ in 0..self.iterations {
            total_memory += operation();
        }
        
        let duration = start.elapsed();
        
        BenchmarkResult::new(
            self.name.clone(),
            duration,
            self.iterations,
            total_memory / self.iterations,
        )
    }
}

/// Move 语义性能分析
pub fn move_performance_analysis() {
    println!("\n🚀 Move 语义性能分析");
    println!("{}", "=".repeat(50));
    
    // 1. 不同大小数据的 Move 性能
    move_size_performance();
    
    // 2. Move vs Copy 性能对比
    move_vs_copy_performance();
    
    // 3. 智能指针 Move 性能
    smart_pointer_move_performance();
    
    // 4. 容器 Move 性能
    container_move_performance();
    
    // 5. 嵌套结构 Move 性能
    nested_structure_move_performance();
}

/// 不同大小数据的 Move 性能
fn move_size_performance() {
    println!("\n📏 不同大小数据的 Move 性能:");
    
    // 小型数据
    #[derive(Debug)]
    struct SmallData {
        value: i32,
    }
    
    // 中型数据
    #[derive(Debug)]
    struct MediumData {
        values: [i32; 100],
    }
    
    // 大型数据
    #[derive(Debug)]
    struct LargeData {
        buffer: Vec<u8>,
    }
    
    let iterations = 1_000_000;
    
    // 测试小型数据 Move
    let small_bench = Benchmark::new("Small Data Move", iterations);
    let small_result = small_bench.run(|| {
        let data = SmallData { value: 42 };
        let moved = data; // Move 发生
        mem::size_of_val(&moved)
    });
    small_result.print();
    
    // 测试中型数据 Move
    let medium_bench = Benchmark::new("Medium Data Move", iterations / 10);
    let medium_result = medium_bench.run(|| {
        let data = MediumData { values: [1; 100] };
        let moved = data; // Move 发生
        mem::size_of_val(&moved)
    });
    medium_result.print();
    
    // 测试大型数据 Move
    let large_bench = Benchmark::new("Large Data Move", iterations / 100);
    let large_result = large_bench.run(|| {
        let data = LargeData {
            buffer: vec![0; 10000],
        };
        let moved = data; // Move 发生
        mem::size_of_val(&moved) + moved.buffer.capacity() * mem::size_of::<u8>()
    });
    large_result.print();
    
    println!("\n💡 观察: Move 操作本身的开销与数据大小无关（零成本抽象）");
    println!("   实际开销来自于数据的创建和销毁，而不是 Move 本身");
}

/// Move vs Copy 性能对比
fn move_vs_copy_performance() {
    println!("\n⚖️  Move vs Copy 性能对比:");
    
    // Copy 类型
    #[derive(Copy, Clone, Debug)]
    struct CopyableData {
        x: i32,
        y: i32,
        z: i32,
    }
    
    // Move 类型
    #[derive(Debug)]
    struct MovableData {
        data: String,
    }
    
    let iterations = 1_000_000;
    
    // 测试 Copy 性能
    let copy_bench = Benchmark::new("Copy Operation", iterations);
    let copy_result = copy_bench.run(|| {
        let original = CopyableData { x: 1, y: 2, z: 3 };
        let copied = original; // Copy 发生
        let _another = original; // 可以继续使用原始值
        mem::size_of_val(&copied)
    });
    copy_result.print();
    
    // 测试 Move 性能
    let move_bench = Benchmark::new("Move Operation", iterations);
    let move_result = move_bench.run(|| {
        let original = MovableData {
            data: "test".to_string(),
        };
        let moved = original; // Move 发生
        mem::size_of_val(&moved) + moved.data.capacity()
    });
    move_result.print();
    
    // 性能比较
    let copy_speed = copy_result.avg_ns_per_op;
    let move_speed = move_result.avg_ns_per_op;
    
    if copy_speed < move_speed {
        println!("\n🏆 Copy 比 Move 快 {:.2}x", move_speed / copy_speed);
    } else {
        println!("\n🏆 Move 比 Copy 快 {:.2}x", copy_speed / move_speed);
    }
    
    println!("\n💡 分析:");
    println!("   • Copy 适用于小型、简单的数据类型");
    println!("   • Move 适用于复杂、大型或拥有资源的类型");
    println!("   • 选择取决于数据特性，而不仅仅是性能");
}

/// 智能指针 Move 性能
fn smart_pointer_move_performance() {
    println!("\n🧠 智能指针 Move 性能:");
    
    let iterations = 100_000;
    let data_size = 10000;
    
    // Box Move 性能
    let box_bench = Benchmark::new("Box Move", iterations);
    let box_result = box_bench.run(|| {
        let boxed = Box::new(vec![0; data_size]);
        let moved = boxed; // Move Box
        mem::size_of_val(&moved) + moved.len() * mem::size_of::<i32>()
    });
    box_result.print();
    
    // Rc Move 性能
    let rc_bench = Benchmark::new("Rc Move", iterations);
    let rc_result = rc_bench.run(|| {
        let rc_data = Rc::new(vec![0; data_size]);
        let moved = rc_data; // Move Rc
        mem::size_of_val(&moved) + moved.len() * mem::size_of::<i32>()
    });
    rc_result.print();
    
    // Arc Move 性能
    let arc_bench = Benchmark::new("Arc Move", iterations);
    let arc_result = arc_bench.run(|| {
        let arc_data = Arc::new(vec![0; data_size]);
        let moved = arc_data; // Move Arc
        mem::size_of_val(&moved) + moved.len() * mem::size_of::<i32>()
    });
    arc_result.print();
    
    println!("\n💡 智能指针 Move 分析:");
    println!("   • Box Move: 转移堆数据的唯一所有权");
    println!("   • Rc Move: 转移引用计数指针（数据共享）");
    println!("   • Arc Move: 转移原子引用计数指针（线程安全）");
    println!("   • 所有智能指针的 Move 都是零成本的");
}

/// 容器 Move 性能
fn container_move_performance() {
    println!("\n📦 容器 Move 性能:");
    
    let iterations = 10_000;
    let container_size = 1000;
    
    // Vec Move 性能
    let vec_bench = Benchmark::new("Vec Move", iterations);
    let vec_result = vec_bench.run(|| {
        let vec_data = (0..container_size).collect::<Vec<i32>>();
        let moved = vec_data; // Move Vec
        mem::size_of_val(&moved) + moved.capacity() * mem::size_of::<i32>()
    });
    vec_result.print();
    
    // HashMap Move 性能
    let map_bench = Benchmark::new("HashMap Move", iterations);
    let map_result = map_bench.run(|| {
        let mut map: HashMap<i32, String> = HashMap::new();
        for i in 0..container_size {
            map.insert(i, format!("value_{}", i));
        }
        let moved = map; // Move HashMap
        mem::size_of_val(&moved) + moved.capacity() * (mem::size_of::<i32>() + mem::size_of::<String>())
    });
    map_result.print();
    
    // String Move 性能
    let string_bench = Benchmark::new("String Move", iterations);
    let string_result = string_bench.run(|| {
        let string_data = "x".repeat(container_size as usize);
        let moved = string_data; // Move String
        mem::size_of_val(&moved) + moved.capacity()
    });
    string_result.print();
    
    println!("\n💡 容器 Move 特点:");
    println!("   • 所有容器的 Move 都是 O(1) 操作");
    println!("   • 只转移元数据（指针、长度、容量）");
    println!("   • 实际数据保持在原位置不动");
}

/// 嵌套结构 Move 性能
fn nested_structure_move_performance() {
    println!("\n🏗️ 嵌套结构 Move 性能:");
    
    #[derive(Debug)]
    struct NestedData {
        id: u32,
        name: String,
        values: Vec<i32>,
        metadata: HashMap<String, String>,
    }
    
    let iterations = 1_000;
    
    // 嵌套结构 Move 性能
    let nested_bench = Benchmark::new("Nested Structure Move", iterations);
    let nested_result = nested_bench.run(|| {
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "test".to_string());
        metadata.insert("version".to_string(), "1.0".to_string());
        
        let nested = NestedData {
            id: 42,
            name: "test_data".to_string(),
            values: (0..100).collect(),
            metadata,
        };
        
        let moved = nested; // Move 整个嵌套结构
        
        // 估算内存使用
        mem::size_of_val(&moved) +
        moved.name.capacity() +
        moved.values.capacity() * mem::size_of::<i32>() +
        moved.metadata.capacity() * (mem::size_of::<String>() * 2)
    });
    nested_result.print();
    
    println!("\n💡 嵌套结构 Move 分析:");
    println!("   • 复杂度仍然是 O(1)");
    println!("   • 只移动顶层结构的元数据");
    println!("   • 所有嵌套的堆数据保持原位");
    println!("   • Move 语义递归应用到所有字段");
}

/// Copy trait 性能分析
pub fn copy_performance_analysis() {
    println!("\n📋 Copy trait 性能分析");
    println!("{}", "=".repeat(50));
    
    // 1. 不同大小 Copy 类型的性能
    copy_size_performance();
    
    // 2. Copy vs Clone 性能对比
    copy_vs_clone_performance();
    
    // 3. 数组 Copy 性能
    array_copy_performance();
    
    // 4. 循环中的 Copy 性能
    loop_copy_performance();
    
    // 5. Copy 类型的内存访问模式
    copy_memory_access_patterns();
}

/// 不同大小 Copy 类型的性能
fn copy_size_performance() {
    println!("\n📏 不同大小 Copy 类型性能:");
    
    let iterations = 1_000_000;
    
    // 1 字节 Copy
    let byte_bench = Benchmark::new("1 Byte Copy", iterations);
    let byte_result = byte_bench.run(|| {
        let original: u8 = 42;
        let copied = original;
        let _another = original;
        mem::size_of_val(&copied)
    });
    byte_result.print();
    
    // 4 字节 Copy
    let int_bench = Benchmark::new("4 Bytes Copy", iterations);
    let int_result = int_bench.run(|| {
        let original: i32 = 42;
        let copied = original;
        let _another = original;
        mem::size_of_val(&copied)
    });
    int_result.print();
    
    // 8 字节 Copy
    let long_bench = Benchmark::new("8 Bytes Copy", iterations);
    let long_result = long_bench.run(|| {
        let original: i64 = 42;
        let copied = original;
        let _another = original;
        mem::size_of_val(&copied)
    });
    long_result.print();
    
    // 16 字节 Copy
    #[derive(Copy, Clone)]
    struct Bytes16 {
        data: [u8; 16],
    }
    
    let bytes16_bench = Benchmark::new("16 Bytes Copy", iterations);
    let bytes16_result = bytes16_bench.run(|| {
        let original = Bytes16 { data: [42; 16] };
        let copied = original;
        let _another = original;
        mem::size_of_val(&copied)
    });
    bytes16_result.print();
    
    // 64 字节 Copy
    #[derive(Copy, Clone)]
    struct Bytes64 {
        data: [u8; 64],
    }
    
    let bytes64_bench = Benchmark::new("64 Bytes Copy", iterations / 10);
    let bytes64_result = bytes64_bench.run(|| {
        let original = Bytes64 { data: [42; 64] };
        let copied = original;
        let _another = original;
        mem::size_of_val(&copied)
    });
    bytes64_result.print();
    
    // 256 字节 Copy
    #[derive(Copy, Clone)]
    struct Bytes256 {
        data: [u8; 256],
    }
    
    let bytes256_bench = Benchmark::new("256 Bytes Copy", iterations / 100);
    let bytes256_result = bytes256_bench.run(|| {
        let original = Bytes256 { data: [42; 256] };
        let copied = original;
        let _another = original;
        mem::size_of_val(&copied)
    });
    bytes256_result.print();
    
    println!("\n💡 Copy 大小性能分析:");
    println!("   • 小型数据（≤8字节）: 通常在寄存器中操作，极快");
    println!("   • 中型数据（16-64字节）: 可能需要多次内存操作");
    println!("   • 大型数据（≥256字节）: 明显的内存复制开销");
    println!("   • 考虑使用引用而不是大型 Copy 类型");
}

/// Copy vs Clone 性能对比
fn copy_vs_clone_performance() {
    println!("\n⚖️  Copy vs Clone 性能对比:");
    
    #[derive(Copy, Clone, Debug)]
    struct Point {
        x: f64,
        y: f64,
    }
    
    #[derive(Clone, Debug)]
    struct ComplexData {
        values: Vec<i32>,
        name: String,
    }
    
    let iterations = 100_000;
    
    // Copy 性能测试
    let copy_bench = Benchmark::new("Copy Trait", iterations);
    let copy_result = copy_bench.run(|| {
        let point = Point { x: 1.0, y: 2.0 };
        let copied = point; // 隐式 Copy
        let _another = point; // 原始值仍可用
        mem::size_of_val(&copied)
    });
    copy_result.print();
    
    // Clone 性能测试（简单数据）
    let simple_clone_bench = Benchmark::new("Clone Trait (Simple)", iterations);
    let simple_clone_result = simple_clone_bench.run(|| {
        let point = Point { x: 1.0, y: 2.0 };
        let cloned = point.clone(); // 显式 Clone
        mem::size_of_val(&cloned)
    });
    simple_clone_result.print();
    
    // Clone 性能测试（复杂数据）
    let complex_clone_bench = Benchmark::new("Clone Trait (Complex)", iterations / 10);
    let complex_clone_result = complex_clone_bench.run(|| {
        let data = ComplexData {
            values: vec![1, 2, 3, 4, 5],
            name: "test".to_string(),
        };
        let cloned = data.clone(); // 深拷贝
        mem::size_of_val(&cloned) + cloned.values.capacity() * mem::size_of::<i32>() + cloned.name.capacity()
    });
    complex_clone_result.print();
    
    println!("\n💡 Copy vs Clone 分析:");
    println!("   • Copy: 按位复制，极快，适用于简单类型");
    println!("   • Clone (简单): 与 Copy 性能相近，但有函数调用开销");
    println!("   • Clone (复杂): 需要深拷贝，性能取决于数据复杂度");
}

/// 数组 Copy 性能
fn array_copy_performance() {
    println!("\n🔢 数组 Copy 性能:");
    
    let iterations = 100_000;
    
    // 小数组 Copy
    let small_array_bench = Benchmark::new("Small Array [i32; 4]", iterations);
    let small_array_result = small_array_bench.run(|| {
        let arr: [i32; 4] = [1, 2, 3, 4];
        let copied = arr;
        let _another = arr;
        mem::size_of_val(&copied)
    });
    small_array_result.print();
    
    // 中等数组 Copy
    let medium_array_bench = Benchmark::new("Medium Array [i32; 100]", iterations / 10);
    let medium_array_result = medium_array_bench.run(|| {
        let arr: [i32; 100] = [42; 100];
        let copied = arr;
        let _another = arr;
        mem::size_of_val(&copied)
    });
    medium_array_result.print();
    
    // 大数组 Copy
    let large_array_bench = Benchmark::new("Large Array [i32; 1000]", iterations / 100);
    let large_array_result = large_array_bench.run(|| {
        let arr: [i32; 1000] = [42; 1000];
        let copied = arr;
        let _another = arr;
        mem::size_of_val(&copied)
    });
    large_array_result.print();
    
    println!("\n💡 数组 Copy 性能特点:");
    println!("   • 数组 Copy 开销与元素数量成正比");
    println!("   • 编译器可能优化小数组的 Copy");
    println!("   • 大数组 Copy 可能导致缓存未命中");
    println!("   • 考虑使用切片引用 &[T] 代替大数组");
}

/// 循环中的 Copy 性能
fn loop_copy_performance() {
    println!("\n🔄 循环中的 Copy 性能:");
    
    #[derive(Copy, Clone, Debug)]
    struct Data {
        value: i64,
        flag: bool,
    }
    
    let iterations = 10_000;
    let loop_count = 100;
    
    // 循环中频繁 Copy
    let loop_copy_bench = Benchmark::new("Loop Copy", iterations);
    let loop_copy_result = loop_copy_bench.run(|| {
        let original = Data { value: 42, flag: true };
        let mut sum = 0;
        
        for _ in 0..loop_count {
            let copied = original; // 每次循环都 Copy
            sum += copied.value;
        }
        
        mem::size_of::<Data>() * loop_count
    });
    loop_copy_result.print();
    
    // 循环中使用引用
    let loop_ref_bench = Benchmark::new("Loop Reference", iterations);
    let loop_ref_result = loop_ref_bench.run(|| {
        let original = Data { value: 42, flag: true };
        let mut sum = 0;
        
        for _ in 0..loop_count {
            let referenced = &original; // 使用引用
            sum += referenced.value;
        }
        
        mem::size_of::<&Data>() * loop_count
    });
    loop_ref_result.print();
    
    println!("\n💡 循环中的 Copy 优化:");
    println!("   • 避免在热循环中不必要的 Copy");
    println!("   • 优先使用引用访问数据");
    println!("   • 编译器可能优化掉一些 Copy 操作");
}

/// Copy 类型的内存访问模式
fn copy_memory_access_patterns() {
    println!("\n🧠 Copy 类型内存访问模式:");
    
    #[derive(Copy, Clone)]
    struct CacheLineFriendly {
        data: [u8; 64], // 一个缓存行大小
    }
    
    #[derive(Copy, Clone)]
    struct CacheLineUnfriendly {
        data: [u8; 65], // 跨越缓存行边界
    }
    
    let iterations = 10_000;
    
    // 缓存友好的 Copy
    let friendly_bench = Benchmark::new("Cache-Friendly Copy", iterations);
    let friendly_result = friendly_bench.run(|| {
        let data = CacheLineFriendly { data: [42; 64] };
        let copied = data;
        mem::size_of_val(&copied)
    });
    friendly_result.print();
    
    // 缓存不友好的 Copy
    let unfriendly_bench = Benchmark::new("Cache-Unfriendly Copy", iterations);
    let unfriendly_result = unfriendly_bench.run(|| {
        let data = CacheLineUnfriendly { data: [42; 65] };
        let copied = data;
        mem::size_of_val(&copied)
    });
    unfriendly_result.print();
    
    println!("\n💡 内存访问模式优化:");
    println!("   • 考虑数据结构的缓存行对齐");
    println!("   • 避免跨缓存行边界的小结构");
    println!("   • 使用 #[repr(align(64))] 进行缓存行对齐");
}

/// Clone trait 性能分析
pub fn clone_performance_analysis() {
    println!("\n🔄 Clone trait 性能分析");
    println!("{}", "=".repeat(50));
    
    // 1. 不同数据结构的 Clone 性能
    clone_data_structure_performance();
    
    // 2. 深度嵌套结构的 Clone 性能
    nested_clone_performance();
    
    // 3. 智能指针 Clone 性能
    smart_pointer_clone_performance();
    
    // 4. Clone vs 手动复制性能
    clone_vs_manual_copy_performance();
    
    // 5. 条件 Clone 优化
    conditional_clone_optimization();
}

/// 不同数据结构的 Clone 性能
fn clone_data_structure_performance() {
    println!("\n📊 不同数据结构 Clone 性能:");
    
    let iterations = 1_000;
    let size = 1000;
    
    // Vec Clone 性能
    let vec_bench = Benchmark::new("Vec<i32> Clone", iterations);
    let vec_result = vec_bench.run(|| {
        let vec_data: Vec<i32> = (0..size).collect();
        let cloned = vec_data.clone();
        mem::size_of_val(&cloned) + cloned.capacity() * mem::size_of::<i32>()
    });
    vec_result.print();
    
    // HashMap Clone 性能
    let map_bench = Benchmark::new("HashMap Clone", iterations);
    let map_result = map_bench.run(|| {
        let mut map: HashMap<i32, String> = HashMap::new();
        for i in 0..size {
            map.insert(i, format!("value_{}", i));
        }
        let cloned = map.clone();
        mem::size_of_val(&cloned) + cloned.capacity() * (mem::size_of::<i32>() + mem::size_of::<String>())
    });
    map_result.print();
    
    // String Clone 性能
    let string_bench = Benchmark::new("String Clone", iterations);
    let string_result = string_bench.run(|| {
        let string_data = "x".repeat(size as usize);
        let cloned = string_data.clone();
        mem::size_of_val(&cloned) + cloned.capacity()
    });
    string_result.print();
    
    // Vec<String> Clone 性能
    let vec_string_bench = Benchmark::new("Vec<String> Clone", iterations / 10);
    let vec_string_result = vec_string_bench.run(|| {
        let vec_strings: Vec<String> = (0..size)
            .map(|i| format!("string_{}", i))
            .collect();
        let cloned = vec_strings.clone();
        
        let string_memory: usize = cloned.iter()
            .map(|s| s.capacity())
            .sum();
        
        mem::size_of_val(&cloned) + 
        cloned.capacity() * mem::size_of::<String>() + 
        string_memory
    });
    vec_string_result.print();
    
    println!("\n💡 数据结构 Clone 分析:");
    println!("   • Vec<T>: O(n) 复制所有元素");
    println!("   • HashMap<K,V>: O(n) 复制所有键值对");
    println!("   • String: O(n) 复制字符数据");
    println!("   • Vec<String>: O(n*m) n个字符串，每个平均m字符");
}

/// 深度嵌套结构的 Clone 性能
fn nested_clone_performance() {
    println!("\n🏗️ 嵌套结构 Clone 性能:");
    
    #[derive(Clone, Debug)]
    struct Level3 {
        data: Vec<i32>,
    }
    
    #[derive(Clone, Debug)]
    struct Level2 {
        items: Vec<Level3>,
        name: String,
    }
    
    #[derive(Clone, Debug)]
    struct Level1 {
        levels: Vec<Level2>,
        metadata: HashMap<String, String>,
    }
    
    let iterations = 100;
    
    // 创建深度嵌套结构
    let nested_bench = Benchmark::new("Deep Nested Clone", iterations);
    let nested_result = nested_bench.run(|| {
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "nested".to_string());
        
        let level1 = Level1 {
            levels: (0..10)
                .map(|i| Level2 {
                    items: (0..10)
                        .map(|j| Level3 {
                            data: vec![i * 10 + j; 10],
                        })
                        .collect(),
                    name: format!("level2_{}", i),
                })
                .collect(),
            metadata,
        };
        
        let cloned = level1.clone();
        
        // 估算内存使用
        let mut total_memory = mem::size_of_val(&cloned);
        
        for level2 in &cloned.levels {
            total_memory += mem::size_of_val(level2);
            total_memory += level2.name.capacity();
            
            for level3 in &level2.items {
                total_memory += mem::size_of_val(level3);
                total_memory += level3.data.capacity() * mem::size_of::<i32>();
            }
        }
        
        total_memory
    });
    nested_result.print();
    
    println!("\n💡 嵌套结构 Clone 特点:");
    println!("   • Clone 递归应用到所有字段");
    println!("   • 深度嵌套导致指数级内存分配");
    println!("   • 考虑使用 Rc/Arc 共享深层数据");
}

/// 智能指针 Clone 性能
fn smart_pointer_clone_performance() {
    println!("\n🧠 智能指针 Clone 性能:");
    
    let iterations = 100_000;
    let data_size = 1000;
    
    // Rc Clone 性能
    let rc_bench = Benchmark::new("Rc Clone", iterations);
    let rc_result = rc_bench.run(|| {
        let rc_data = Rc::new(vec![0; data_size]);
        let cloned = rc_data.clone(); // 只增加引用计数
        mem::size_of_val(&cloned)
    });
    rc_result.print();
    
    // Arc Clone 性能
    let arc_bench = Benchmark::new("Arc Clone", iterations);
    let arc_result = arc_bench.run(|| {
        let arc_data = Arc::new(vec![0; data_size]);
        let cloned = arc_data.clone(); // 原子操作增加引用计数
        mem::size_of_val(&cloned)
    });
    arc_result.print();
    
    // Box Clone 性能（需要实现 Clone）
    #[derive(Clone)]
    struct BoxedData {
        inner: Box<Vec<i32>>,
    }
    
    let box_bench = Benchmark::new("Box Clone", iterations / 100);
    let box_result = box_bench.run(|| {
        let boxed = BoxedData {
            inner: Box::new(vec![0; data_size]),
        };
        let cloned = boxed.clone(); // 深拷贝 Box 内容
        mem::size_of_val(&cloned) + cloned.inner.capacity() * mem::size_of::<i32>()
    });
    box_result.print();
    
    println!("\n💡 智能指针 Clone 对比:");
    println!("   • Rc Clone: 只增加引用计数，O(1)");
    println!("   • Arc Clone: 原子操作增加引用计数，略慢于 Rc");
    println!("   • Box Clone: 深拷贝内容，O(n)");
    println!("   • 共享数据时优先使用 Rc/Arc");
}

/// Clone vs 手动复制性能
fn clone_vs_manual_copy_performance() {
    println!("\n🔧 Clone vs 手动复制性能:");
    
    #[derive(Clone, Debug)]
    struct Data {
        values: Vec<i32>,
        name: String,
        flag: bool,
    }
    
    let iterations = 10_000;
    
    // 使用 Clone trait
    let clone_bench = Benchmark::new("Clone Trait", iterations);
    let clone_result = clone_bench.run(|| {
        let data = Data {
            values: vec![1, 2, 3, 4, 5],
            name: "test".to_string(),
            flag: true,
        };
        let cloned = data.clone();
        mem::size_of_val(&cloned) + cloned.values.capacity() * mem::size_of::<i32>() + cloned.name.capacity()
    });
    clone_result.print();
    
    // 手动复制
    let manual_bench = Benchmark::new("Manual Copy", iterations);
    let manual_result = manual_bench.run(|| {
        let data = Data {
            values: vec![1, 2, 3, 4, 5],
            name: "test".to_string(),
            flag: true,
        };
        
        let manual_copy = Data {
            values: data.values.clone(),
            name: data.name.clone(),
            flag: data.flag,
        };
        
        mem::size_of_val(&manual_copy) + 
        manual_copy.values.capacity() * mem::size_of::<i32>() + 
        manual_copy.name.capacity()
    });
    manual_result.print();
    
    println!("\n💡 Clone vs 手动复制:");
    println!("   • Clone trait: 编译器优化，类型安全");
    println!("   • 手动复制: 更多控制，可能更高效");
    println!("   • 性能差异通常很小");
    println!("   • 优先使用 Clone trait 保证正确性");
}

/// 条件 Clone 优化
fn conditional_clone_optimization() {
    println!("\n🎯 条件 Clone 优化:");
    
    let iterations = 10_000;
    let data = vec![1, 2, 3, 4, 5];
    
    // 总是 Clone
    let always_clone_bench = Benchmark::new("Always Clone", iterations);
    let always_clone_result = always_clone_bench.run(|| {
        let cloned = data.clone(); // 总是执行 Clone
        mem::size_of_val(&cloned) + cloned.capacity() * mem::size_of::<i32>()
    });
    always_clone_result.print();
    
    // 条件 Clone
    let conditional_clone_bench = Benchmark::new("Conditional Clone", iterations);
    let conditional_clone_result = conditional_clone_bench.run(|| {
        let condition = true; // 模拟条件
        if condition {
            let cloned = data.clone();
            mem::size_of_val(&cloned) + cloned.capacity() * mem::size_of::<i32>()
        } else {
            0
        }
    });
    conditional_clone_result.print();
    
    // 使用 Cow 优化
    use std::borrow::Cow;
    
    let cow_bench = Benchmark::new("Cow Optimization", iterations);
    let cow_result = cow_bench.run(|| {
        let cow_data: Cow<[i32]> = Cow::Borrowed(&data);
        let condition = false; // 大部分时候不需要修改
        
        let result = if condition {
            let mut owned = cow_data.into_owned();
            owned.push(6);
            owned
        } else {
            cow_data.into_owned() // 只在需要时 Clone
        };
        
        mem::size_of_val(&result) + result.capacity() * mem::size_of::<i32>()
    });
    cow_result.print();
    
    println!("\n💡 条件 Clone 优化策略:");
    println!("   • 避免不必要的 Clone 操作");
    println!("   • 使用 Cow<T> 实现写时复制");
    println!("   • 延迟 Clone 直到真正需要时");
    println!("   • 考虑使用引用计数共享数据");
}

/// 综合性能对比分析
pub fn comprehensive_performance_comparison() {
    println!("\n🏆 综合性能对比分析");
    println!("{}", "=".repeat(50));
    
    // 1. 三种机制的直接对比
    direct_comparison();
    
    // 2. 不同场景下的最佳选择
    scenario_based_recommendations();
    
    // 3. 内存使用分析
    memory_usage_analysis();
    
    // 4. 性能优化建议
    performance_optimization_recommendations();
}

/// 三种机制的直接对比
fn direct_comparison() {
    println!("\n⚖️  Move vs Copy vs Clone 直接对比:");
    
    let iterations = 100_000;
    
    // 测试数据
    #[derive(Copy, Clone, Debug)]
    struct SmallCopyable {
        x: i32,
        y: i32,
    }
    
    #[derive(Clone, Debug)]
    struct LargeCloneable {
        data: Vec<i32>,
    }
    
    // Move 性能（小数据）
    let small_move_bench = Benchmark::new("Small Data Move", iterations);
    let small_move_result = small_move_bench.run(|| {
        let data = SmallCopyable { x: 1, y: 2 };
        let moved = data; // Move
        mem::size_of_val(&moved)
    });
    small_move_result.print();
    
    // Copy 性能（小数据）
    let small_copy_bench = Benchmark::new("Small Data Copy", iterations);
    let small_copy_result = small_copy_bench.run(|| {
        let data = SmallCopyable { x: 1, y: 2 };
        let copied = data; // Copy
        let _another = data; // 仍可使用
        mem::size_of_val(&copied)
    });
    small_copy_result.print();
    
    // Clone 性能（小数据）
    let small_clone_bench = Benchmark::new("Small Data Clone", iterations);
    let small_clone_result = small_clone_bench.run(|| {
        let data = SmallCopyable { x: 1, y: 2 };
        let cloned = data.clone(); // Clone
        mem::size_of_val(&cloned)
    });
    small_clone_result.print();
    
    // Move 性能（大数据）
    let large_move_bench = Benchmark::new("Large Data Move", iterations / 10);
    let large_move_result = large_move_bench.run(|| {
        let data = LargeCloneable {
            data: vec![1, 2, 3, 4, 5],
        };
        let moved = data; // Move
        mem::size_of_val(&moved) + moved.data.capacity() * mem::size_of::<i32>()
    });
    large_move_result.print();
    
    // Clone 性能（大数据）
    let large_clone_bench = Benchmark::new("Large Data Clone", iterations / 100);
    let large_clone_result = large_clone_bench.run(|| {
        let data = LargeCloneable {
            data: vec![1, 2, 3, 4, 5],
        };
        let cloned = data.clone(); // Clone
        mem::size_of_val(&cloned) + cloned.data.capacity() * mem::size_of::<i32>()
    });
    large_clone_result.print();
    
    println!("\n📊 性能对比总结:");
    println!("   • 小数据: Move ≈ Copy > Clone");
    println!("   • 大数据: Move >> Clone (Copy 不适用)");
    println!("   • Move 始终是零成本抽象");
    println!("   • Copy 适用于小型简单类型");
    println!("   • Clone 开销与数据复杂度成正比");
}

/// 不同场景下的最佳选择
fn scenario_based_recommendations() {
    println!("\n🎯 场景化性能建议:");
    
    println!("\n🔢 数值计算场景:");
    println!("   • 推荐: Copy (i32, f64, Point, Complex)");
    println!("   • 原因: 小型数据，频繁传递，栈操作极快");
    
    println!("\n📊 数据处理场景:");
    println!("   • 推荐: Move + 引用 (Vec, HashMap, String)");
    println!("   • 原因: 避免大数据复制，保持所有权清晰");
    
    println!("\n🌐 Web 服务场景:");
    println!("   • 推荐: Arc + Clone (共享配置和状态)");
    println!("   • 原因: 多线程安全，浅拷贝高效");
    
    println!("\n🎮 游戏开发场景:");
    println!("   • 推荐: Copy (位置、向量) + Move (资源)");
    println!("   • 原因: 高频数学运算 + 资源管理");
    
    println!("\n🔧 系统编程场景:");
    println!("   • 推荐: Move + 智能指针");
    println!("   • 原因: 精确的资源控制和生命周期管理");
}

/// 内存使用分析
fn memory_usage_analysis() {
    println!("\n🧠 内存使用分析:");
    
    // 类型大小分析
    println!("\n📏 基本类型大小:");
    println!("   • i32: {} 字节", mem::size_of::<i32>());
    println!("   • String: {} 字节 (+ 堆内存)", mem::size_of::<String>());
    println!("   • Vec<i32>: {} 字节 (+ 堆内存)", mem::size_of::<Vec<i32>>());
    println!("   • Box<i32>: {} 字节 (+ 堆内存)", mem::size_of::<Box<i32>>());
    println!("   • Rc<i32>: {} 字节 (+ 堆内存)", mem::size_of::<Rc<i32>>());
    println!("   • Arc<i32>: {} 字节 (+ 堆内存)", mem::size_of::<Arc<i32>>());
    
    // 内存开销对比
    let data = vec![1, 2, 3, 4, 5];
    let rc_data = Rc::new(data.clone());
    let arc_data = Arc::new(data.clone());
    
    println!("\n💾 内存开销对比 (5个i32的Vec):");
    println!("   • 直接拥有: {} 字节", 
             mem::size_of_val(&data) + data.capacity() * mem::size_of::<i32>());
    println!("   • Rc包装: {} 字节 (+ 引用计数开销)", 
             mem::size_of_val(&rc_data) + rc_data.capacity() * mem::size_of::<i32>());
    println!("   • Arc包装: {} 字节 (+ 原子引用计数开销)", 
             mem::size_of_val(&arc_data) + arc_data.capacity() * mem::size_of::<i32>());
    
    println!("\n🔄 Clone 内存影响:");
    println!("   • Vec Clone: 完全复制，2x 内存使用");
    println!("   • Rc Clone: 只复制指针，共享数据");
    println!("   • Arc Clone: 原子操作，共享数据");
}

/// 性能优化建议
fn performance_optimization_recommendations() {
    println!("\n⚡ 性能优化建议:");
    
    println!("\n🎯 选择策略:");
    println!("   1. 小型数据 (≤16字节) → Copy");
    println!("   2. 大型数据，单一所有者 → Move");
    println!("   3. 需要共享，不常修改 → Rc/Arc + Clone");
    println!("   4. 需要深拷贝，不频繁 → Clone");
    println!("   5. 条件性需要 → Cow<T>");
    
    println!("\n🚀 优化技巧:");
    println!("   • 避免在热路径上 Clone 大数据");
    println!("   • 使用引用传递而不是值传递");
    println!("   • 考虑数据结构的缓存友好性");
    println!("   • 利用编译器优化 (内联、消除)");
    println!("   • 使用 #[inline] 标记小函数");
    
    println!("\n🔍 性能分析工具:");
    println!("   • cargo bench - 基准测试");
    println!("   • perf - 系统级性能分析");
    println!("   • valgrind - 内存使用分析");
    println!("   • flamegraph - 性能火焰图");
    
    println!("\n📊 监控指标:");
    println!("   • 操作延迟 (ns/op)");
    println!("   • 内存使用量 (bytes)");
    println!("   • 分配次数 (allocs/op)");
    println!("   • 缓存命中率");
    println!("   • CPU 使用率");
}

/// 运行所有性能分析
pub fn run_all_performance_analysis() {
    move_performance_analysis();
    copy_performance_analysis();
    clone_performance_analysis();
    comprehensive_performance_comparison();
    
    println!("\n🎉 性能分析完成！");
    println!("\n📈 关键洞察:");
    println!("   • Move 是零成本抽象，适用于所有场景");
    println!("   • Copy 适用于小型、简单的值类型");
    println!("   • Clone 提供灵活性，但要注意性能开销");
    println!("   • 智能指针提供共享所有权的高效方案");
    println!("   • 选择合适的机制比微优化更重要");
    
    println!("\n🎯 最佳实践:");
    println!("   1. 理解数据的生命周期和使用模式");
    println!("   2. 优先考虑 Move 语义的设计");
    println!("   3. 谨慎使用 Clone，特别是大数据");
    println!("   4. 利用类型系统防止性能陷阱");
    println!("   5. 定期进行性能测试和分析");
}