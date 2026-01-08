//! 🦀 Rust Vector 动态数组深度分析
//! =====================================
//!
//! 本文档全面深入地分析 Rust 中的 Vector 类型，涵盖所有相关知识点
//! 包括基础概念、内存管理、性能优化、高级特性和实际应用案例

use std::collections::HashMap;
use std::fmt::Debug;
use std::time::Instant;

fn main() {
    println!("🦀 Rust Vector 动态数组深度分析");
    println!("=====================================");

    // 执行所有分析模块
    vector_basics_analysis();
    vector_creation_analysis();
    vector_operations_analysis();
    vector_iteration_analysis();
    vector_memory_analysis();
    vector_slicing_analysis();
    vector_sorting_analysis();
    vector_advanced_features();
    vector_performance_analysis();
    vector_real_world_examples();

    println!("\n=====================================");
    println!("🎉 Vector 深度分析完成！");
    println!("\n📚 本分析涵盖了以下核心知识点:");
    println!("   • Vector 基础概念和内存布局");
    println!("   • Vector 创建方法和最佳实践");
    println!("   • Vector 基本操作的深入分析");
    println!("   • Vector 迭代方法和性能对比");
    println!("   • Vector 内存管理和容量优化");
    println!("   • Vector 切片操作和借用机制");
    println!("   • Vector 排序算法和自定义比较");
    println!("   • Vector 高级特性和实用方法");
    println!("   • Vector 性能优化策略");
    println!("   • Vector 在实际项目中的应用");
    println!("\n💡 掌握这些知识将大大提升你的 Rust 编程能力！");
}

// ============================================================================
// 1. Vector 基础概念和内存布局分析
// ============================================================================

fn vector_basics_analysis() {
    println!("\n=== 1. Vector 基础概念和内存布局分析 ===");

    // Vector 的基本定义和特性
    println!("\n📖 Vector 基础概念:");
    println!("   • Vector<T> 是 Rust 标准库中的动态数组类型");
    println!("   • 存储在堆上，可以动态增长和缩减");
    println!("   • 元素在内存中连续存储，支持随机访问");
    println!("   • 拥有所有权，遵循 Rust 的所有权规则");

    // Vector 的内存布局
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("\n🧠 Vector 内存布局分析:");
    println!("   Vector 包含三个字段:");
    println!("   • ptr: 指向堆上数据的指针");
    println!("   • len: 当前元素数量 = {}", vec.len());
    println!("   • cap: 当前容量 = {}", vec.capacity());

    // 展示内存地址
    println!("\n📍 内存地址信息:");
    println!("   Vector 本身在栈上的地址: {:p}", &vec);
    println!("   Vector 数据在堆上的地址: {:p}", vec.as_ptr());

    // Vector 与数组的对比
    println!("\n⚖️ Vector vs 数组对比:");
    let array = [1, 2, 3, 4, 5]; // 固定大小，栈上分配
    println!(
        "   数组大小: {} bytes (栈上)",
        std::mem::size_of_val(&array)
    );
    println!(
        "   Vector 结构大小: {} bytes (栈上)",
        std::mem::size_of_val(&vec)
    );
    println!(
        "   Vector 数据大小: {} bytes (堆上)",
        vec.len() * std::mem::size_of::<i32>()
    );

    // Vector 的类型系统
    demonstrate_vector_types();
}

fn demonstrate_vector_types() {
    println!("\n🔍 Vector 类型系统演示:");

    // 不同类型的 Vector
    let int_vec: Vec<i32> = vec![1, 2, 3];
    let string_vec: Vec<String> = vec!["hello".to_string(), "world".to_string()];
    let bool_vec: Vec<bool> = vec![true, false, true];

    println!("   i32 Vector: {:?}", int_vec);
    println!("   String Vector: {:?}", string_vec);
    println!("   bool Vector: {:?}", bool_vec);

    // 嵌套 Vector
    let nested_vec: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];
    println!("   嵌套 Vector: {:?}", nested_vec);

    // 自定义类型的 Vector
    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    let people: Vec<Person> = vec![
        Person {
            name: "Alice".to_string(),
            age: 30,
        },
        Person {
            name: "Bob".to_string(),
            age: 25,
        },
    ];
    println!("   自定义类型 Vector: {:?}", people);
}

// ============================================================================
// 2. Vector 创建方法的全面分析和最佳实践
// ============================================================================

fn vector_creation_analysis() {
    println!("\n=== 2. Vector 创建方法的全面分析和最佳实践 ===");

    // 方法1: 使用 Vec::new() 创建空 Vector
    println!("\n📝 创建方法1: Vec::new()");
    let mut empty_vec: Vec<i32> = Vec::new();
    println!(
        "   空 Vector: {:?}, 长度: {}, 容量: {}",
        empty_vec,
        empty_vec.len(),
        empty_vec.capacity()
    );
    empty_vec.push(1);
    println!(
        "   添加元素后: {:?}, 长度: {}, 容量: {}",
        empty_vec,
        empty_vec.len(),
        empty_vec.capacity()
    );

    // 方法2: 使用 vec! 宏
    println!("\n📝 创建方法2: vec! 宏");
    let macro_vec = vec![1, 2, 3, 4, 5];
    println!("   使用 vec![1,2,3,4,5]: {:?}", macro_vec);

    let repeat_vec = vec![0; 5]; // 创建5个0
    println!("   使用 vec![0; 5]: {:?}", repeat_vec);

    let string_vec = vec!["a", "b", "c"];
    println!("   字符串 Vector: {:?}", string_vec);

    // 方法3: 使用 Vec::with_capacity() 预分配容量
    println!("\n📝 创建方法3: Vec::with_capacity()");
    let mut capacity_vec: Vec<i32> = Vec::with_capacity(10);
    println!(
        "   预分配容量10: 长度: {}, 容量: {}",
        capacity_vec.len(),
        capacity_vec.capacity()
    );

    for i in 0..5 {
        capacity_vec.push(i);
    }
    println!(
        "   添加5个元素后: {:?}, 长度: {}, 容量: {}",
        capacity_vec,
        capacity_vec.len(),
        capacity_vec.capacity()
    );

    // 方法4: 从其他集合创建
    println!("\n📝 创建方法4: 从其他集合创建");

    // 从数组创建
    let array = [1, 2, 3, 4, 5];
    let from_array: Vec<i32> = array.to_vec();
    println!("   从数组创建: {:?}", from_array);

    // 从切片创建
    let slice = &[10, 20, 30];
    let from_slice: Vec<i32> = slice.to_vec();
    println!("   从切片创建: {:?}", from_slice);

    // 从迭代器创建
    let from_iter: Vec<i32> = (1..=5).collect();
    println!("   从迭代器创建: {:?}", from_iter);

    // 从字符串创建字符 Vector
    let chars: Vec<char> = "Hello".chars().collect();
    println!("   从字符串创建字符Vector: {:?}", chars);

    // 方法5: 使用 Vec::from()
    println!("\n📝 创建方法5: Vec::from()");
    let from_array2 = Vec::from([1, 2, 3, 4]);
    println!("   Vec::from([1,2,3,4]): {:?}", from_array2);

    // 最佳实践建议
    println!("\n💡 创建 Vector 的最佳实践:");
    println!("   1. 如果知道大概大小，使用 Vec::with_capacity() 预分配");
    println!("   2. 对于小的固定数据，使用 vec! 宏最简洁");
    println!("   3. 从其他集合转换时，优先使用 collect() 方法");
    println!("   4. 避免频繁的重新分配，提前估算容量");

    demonstrate_creation_performance();
}

fn demonstrate_creation_performance() {
    println!("\n⚡ Vector 创建性能对比:");

    let size = 100_000;

    // 测试1: 不预分配容量
    let start = Instant::now();
    let mut vec1 = Vec::new();
    for i in 0..size {
        vec1.push(i);
    }
    let duration1 = start.elapsed();

    // 测试2: 预分配容量
    let start = Instant::now();
    let mut vec2 = Vec::with_capacity(size);
    for i in 0..size {
        vec2.push(i);
    }
    let duration2 = start.elapsed();

    // 测试3: 使用 collect
    let start = Instant::now();
    let _vec3: Vec<i32> = (0..size).map(|i| i as i32).collect();
    let duration3 = start.elapsed();

    println!("   不预分配容量: {:?}", duration1);
    println!("   预分配容量: {:?}", duration2);
    println!("   使用 collect: {:?}", duration3);
    println!(
        "   性能提升: {:.2}x",
        duration1.as_nanos() as f64 / duration2.as_nanos() as f64
    );
}

// ============================================================================
// 3. Vector 基本操作：增删改查的深入分析
// ============================================================================

fn vector_operations_analysis() {
    println!("\n=== 3. Vector 基本操作：增删改查的深入分析 ===");

    let mut vec = vec![1, 2, 3, 4, 5];
    println!("\n初始 Vector: {:?}", vec);

    // ========== 增加操作 ==========
    println!("\n➕ 增加操作:");

    // push: 在末尾添加元素
    vec.push(6);
    println!("   push(6) 后: {:?}", vec);

    // insert: 在指定位置插入元素
    vec.insert(0, 0); // 在索引0处插入0
    println!("   insert(0, 0) 后: {:?}", vec);

    vec.insert(3, 99); // 在索引3处插入99
    println!("   insert(3, 99) 后: {:?}", vec);

    // append: 将另一个 Vector 的所有元素移动到当前 Vector
    let mut other_vec = vec![100, 200];
    vec.append(&mut other_vec);
    println!("   append([100, 200]) 后: {:?}", vec);
    println!("   被移动的 Vector: {:?}", other_vec); // 现在为空

    // extend: 扩展 Vector
    vec.extend([300, 400]);
    println!("   extend([300, 400]) 后: {:?}", vec);

    vec.extend(vec![500, 600]);
    println!("   extend(vec![500, 600]) 后: {:?}", vec);

    // ========== 删除操作 ==========
    println!("\n➖ 删除操作:");

    // pop: 移除并返回最后一个元素
    if let Some(last) = vec.pop() {
        println!("   pop() 返回: {}, Vector: {:?}", last, vec);
    }

    // remove: 移除指定索引的元素
    let removed = vec.remove(1); // 移除索引1的元素
    println!("   remove(1) 返回: {}, Vector: {:?}", removed, vec);

    // swap_remove: 快速移除（与最后一个元素交换后移除）
    let swap_removed = vec.swap_remove(2);
    println!(
        "   swap_remove(2) 返回: {}, Vector: {:?}",
        swap_removed, vec
    );

    // truncate: 截断到指定长度
    vec.truncate(5);
    println!("   truncate(5) 后: {:?}", vec);

    // clear: 清空所有元素
    let mut temp_vec = vec![1, 2, 3];
    temp_vec.clear();
    println!(
        "   clear() 后: {:?}, 容量: {}",
        temp_vec,
        temp_vec.capacity()
    );

    // ========== 修改操作 ==========
    println!("\n✏️ 修改操作:");

    // 直接索引修改
    vec[0] = 999;
    println!("   vec[0] = 999 后: {:?}", vec);

    // 使用 get_mut 安全修改
    if let Some(element) = vec.get_mut(1) {
        *element = 888;
        println!("   get_mut(1) 修改后: {:?}", vec);
    }

    // swap: 交换两个位置的元素
    vec.swap(0, 2);
    println!("   swap(0, 2) 后: {:?}", vec);

    // reverse: 反转 Vector
    vec.reverse();
    println!("   reverse() 后: {:?}", vec);

    // ========== 查询操作 ==========
    println!("\n🔍 查询操作:");

    // 长度和容量
    println!("   长度: {}, 容量: {}", vec.len(), vec.capacity());
    println!("   是否为空: {}", vec.is_empty());

    // 索引访问
    println!("   vec[0]: {}", vec[0]);

    // 安全访问
    match vec.get(10) {
        Some(value) => println!("   vec.get(10): {}", value),
        None => println!("   vec.get(10): 索引超出范围"),
    }

    // first 和 last
    println!("   first(): {:?}", vec.first());
    println!("   last(): {:?}", vec.last());

    // contains: 检查是否包含某个值
    println!("   contains(&100): {}", vec.contains(&100));
    println!("   contains(&999): {}", vec.contains(&999));

    // 查找元素位置
    println!(
        "   position of 100: {:?}",
        vec.iter().position(|&x| x == 100)
    );

    demonstrate_operation_complexity();
}

fn demonstrate_operation_complexity() {
    println!("\n⏱️ Vector 操作时间复杂度分析:");
    println!("   • push(): O(1) 摊销时间复杂度");
    println!("   • pop(): O(1)");
    println!("   • insert(i, x): O(n) - 需要移动后续元素");
    println!("   • remove(i): O(n) - 需要移动后续元素");
    println!("   • swap_remove(i): O(1) - 不保持顺序");
    println!("   • get(i): O(1) - 随机访问");
    println!("   • contains(): O(n) - 线性搜索");

    // 实际性能测试
    let mut vec = Vec::with_capacity(100_000);
    for i in 0..100_000 {
        vec.push(i);
    }

    // 测试 push 性能
    let start = Instant::now();
    for _ in 0..10_000 {
        vec.push(0);
    }
    println!("   10,000 次 push 操作耗时: {:?}", start.elapsed());

    // 测试随机访问性能
    let start = Instant::now();
    let mut _sum = 0;
    for i in 0..10_000 {
        _sum += vec[i % vec.len()];
    }
    println!("   10,000 次随机访问耗时: {:?}", start.elapsed());
}

// ============================================================================
// 4. Vector 迭代方法和性能对比分析
// ============================================================================

fn vector_iteration_analysis() {
    println!("\n=== 4. Vector 迭代方法和性能对比分析 ===");

    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // ========== 基本迭代方法 ==========
    println!("\n🔄 基本迭代方法:");

    // 方法1: for 循环（推荐）
    print!("   for 循环: ");
    for item in &vec {
        print!("{} ", item);
    }
    println!();

    // 方法2: 索引循环
    print!("   索引循环: ");
    for i in 0..vec.len() {
        print!("{} ", vec[i]);
    }
    println!();

    // 方法3: while 循环
    print!("   while 循环: ");
    let mut i = 0;
    while i < vec.len() {
        print!("{} ", vec[i]);
        i += 1;
    }
    println!();

    // ========== 迭代器方法 ==========
    println!("\n🔄 迭代器方法:");

    // iter(): 创建不可变引用的迭代器
    print!("   iter(): ");
    for item in vec.iter() {
        print!("{} ", item);
    }
    println!();

    // ✅ 优化：如果需要消费，直接使用 into_iter，无需克隆
    // 演示：如果需要保留原 Vec，先克隆
    let vec_copy = vec.clone();
    print!("   into_iter() (克隆后): ");
    for item in vec_copy.into_iter() {
        print!("{} ", item);
    }
    println!();
    println!("   原 Vec 仍可用: {:?}", vec);

    // ✅ 优化：使用引用迭代修改，避免克隆
    let mut vec_mut = vec.clone();
    vec_mut.iter_mut().for_each(|x| *x *= 2); // 移除不必要的 mut
    println!("   iter_mut() 修改后: {:?}", vec_mut);
    println!("   原 Vec 仍可用: {:?}", vec);

    // ========== 高级迭代器操作 ==========
    println!("\n🚀 高级迭代器操作:");

    // map: 转换每个元素
    let doubled: Vec<i32> = vec.iter().map(|x| x * 2).collect();
    println!("   map(|x| x * 2): {:?}", doubled);

    // filter: 过滤元素
    let evens: Vec<&i32> = vec.iter().filter(|&&x| x % 2 == 0).collect();
    println!("   filter(偶数): {:?}", evens);

    // fold: 累积操作
    let sum = vec.iter().fold(0, |acc, x| acc + x);
    println!("   fold(求和): {}", sum);

    // reduce: 归约操作
    let max = vec.iter().reduce(|a, b| if a > b { a } else { b });
    println!("   reduce(最大值): {:?}", max);

    // enumerate: 带索引迭代
    println!("   enumerate():");
    for (index, value) in vec.iter().enumerate() {
        println!("     索引 {}: 值 {}", index, value);
    }

    // zip: 组合两个迭代器
    let vec2 = vec![10, 20, 30, 40, 50];
    let zipped: Vec<(i32, i32)> = vec.iter().zip(vec2.iter()).map(|(a, b)| (*a, *b)).collect();
    println!("   zip 组合: {:?}", zipped);

    // chain: 连接迭代器
    let chained: Vec<i32> = vec.iter().chain(vec2.iter()).cloned().collect();
    println!("   chain 连接: {:?}", chained);

    // take 和 skip
    let first_three: Vec<&i32> = vec.iter().take(3).collect();
    let skip_three: Vec<&i32> = vec.iter().skip(3).collect();
    println!("   take(3): {:?}", first_three);
    println!("   skip(3): {:?}", skip_three);

    demonstrate_iteration_performance();
}

fn demonstrate_iteration_performance() {
    println!("\n⚡ 迭代性能对比分析:");

    let vec: Vec<i32> = (0..100_000).collect();

    // 测试1: for 循环
    let start = Instant::now();
    let mut sum1 = 0i64;
    for item in &vec {
        sum1 += *item as i64;
    }
    let duration1 = start.elapsed();

    // 测试2: 索引循环
    let start = Instant::now();
    let mut sum2 = 0i64;
    for i in 0..vec.len() {
        sum2 += vec[i] as i64;
    }
    let duration2 = start.elapsed();

    // 测试3: 迭代器 fold
    let start = Instant::now();
    let sum3 = vec.iter().fold(0i64, |acc, x| acc + (*x as i64));
    let duration3 = start.elapsed();

    // 测试4: 迭代器 sum
    let start = Instant::now();
    let sum4: i64 = vec.iter().map(|&x| x as i64).sum();
    let duration4 = start.elapsed();

    println!("   for 循环: {:?} (sum: {})", duration1, sum1);
    println!("   索引循环: {:?} (sum: {})", duration2, sum2);
    println!("   iter().fold(): {:?} (sum: {})", duration3, sum3);
    println!("   iter().sum(): {:?} (sum: {})", duration4, sum4);

    println!("\n💡 迭代器优势:");
    println!("   • 零成本抽象：编译器优化后性能相当");
    println!("   • 函数式编程风格，代码更简洁");
    println!("   • 链式调用，操作组合更灵活");
    println!("   • 惰性求值，只在需要时计算");
}

// ============================================================================
// 5. Vector 内存管理和容量优化策略
// ============================================================================

fn vector_memory_analysis() {
    println!("\n=== 5. Vector 内存管理和容量优化策略 ===");

    // ========== 容量管理基础 ==========
    println!("\n📊 容量管理基础:");

    let mut vec = Vec::new();
    println!(
        "   初始状态 - 长度: {}, 容量: {}",
        vec.len(),
        vec.capacity()
    );

    // 观察容量增长模式
    for i in 1..=10 {
        vec.push(i);
        println!(
            "   添加元素 {} - 长度: {}, 容量: {}",
            i,
            vec.len(),
            vec.capacity()
        );
    }

    // ========== 容量控制方法 ==========
    println!("\n🎛️ 容量控制方法:");

    // reserve: 预留额外容量
    let mut vec2 = vec![1, 2, 3];
    println!(
        "   reserve 前 - 长度: {}, 容量: {}",
        vec2.len(),
        vec2.capacity()
    );
    vec2.reserve(10); // 预留至少10个额外位置
    println!(
        "   reserve(10) 后 - 长度: {}, 容量: {}",
        vec2.len(),
        vec2.capacity()
    );

    // reserve_exact: 精确预留容量
    let mut vec3 = vec![1, 2, 3];
    vec3.reserve_exact(5); // 精确预留5个额外位置
    println!(
        "   reserve_exact(5) - 长度: {}, 容量: {}",
        vec3.len(),
        vec3.capacity()
    );

    // shrink_to_fit: 缩减容量到实际大小
    let mut vec4 = Vec::with_capacity(100);
    vec4.extend(1..=10);
    println!(
        "   shrink_to_fit 前 - 长度: {}, 容量: {}",
        vec4.len(),
        vec4.capacity()
    );
    vec4.shrink_to_fit();
    println!(
        "   shrink_to_fit 后 - 长度: {}, 容量: {}",
        vec4.len(),
        vec4.capacity()
    );

    // shrink_to: 缩减到指定容量
    let mut vec5 = Vec::with_capacity(100);
    vec5.extend(1..=10);
    vec5.shrink_to(15);
    println!(
        "   shrink_to(15) - 长度: {}, 容量: {}",
        vec5.len(),
        vec5.capacity()
    );

    // ========== 内存使用分析 ==========
    println!("\n🧮 内存使用分析:");

    let vec_i32: Vec<i32> = vec![1, 2, 3, 4, 5];
    let vec_string: Vec<String> = vec!["hello".to_string(), "world".to_string()];

    println!("   i32 Vector:");
    println!("     栈上大小: {} bytes", std::mem::size_of_val(&vec_i32));
    println!(
        "     堆上大小: {} bytes",
        vec_i32.capacity() * std::mem::size_of::<i32>()
    );

    println!("   String Vector:");
    println!(
        "     栈上大小: {} bytes",
        std::mem::size_of_val(&vec_string)
    );
    println!(
        "     堆上大小(Vector): {} bytes",
        vec_string.capacity() * std::mem::size_of::<String>()
    );

    // 计算 String 内容的堆内存
    let string_heap_size: usize = vec_string.iter().map(|s| s.capacity()).sum();
    println!("     堆上大小(String内容): {} bytes", string_heap_size);

    demonstrate_memory_optimization();
}

fn demonstrate_memory_optimization() {
    println!("\n🚀 内存优化策略演示:");

    // 策略1: 预分配容量
    println!("\n策略1: 预分配容量");
    let start = Instant::now();
    let mut vec_no_prealloc = Vec::new();
    for i in 0..10000 {
        vec_no_prealloc.push(i);
    }
    let duration_no_prealloc = start.elapsed();

    let start = Instant::now();
    let mut vec_prealloc = Vec::with_capacity(10000);
    for i in 0..10000 {
        vec_prealloc.push(i);
    }
    let duration_prealloc = start.elapsed();

    println!("   不预分配: {:?}", duration_no_prealloc);
    println!("   预分配: {:?}", duration_prealloc);
    println!(
        "   性能提升: {:.2}x",
        duration_no_prealloc.as_nanos() as f64 / duration_prealloc.as_nanos() as f64
    );

    // 策略2: 及时释放内存
    println!("\n策略2: 及时释放内存");
    let mut large_vec = Vec::with_capacity(1_000_000);
    large_vec.extend(0..1_000_000);
    println!("   大Vector容量: {}", large_vec.capacity());

    // 只保留前100个元素
    large_vec.truncate(100);
    println!("   truncate后容量: {}", large_vec.capacity());

    large_vec.shrink_to_fit();
    println!("   shrink_to_fit后容量: {}", large_vec.capacity());

    // 策略3: 选择合适的数据结构
    println!("\n策略3: 数据结构选择");

    // 对于大量插入删除操作，考虑使用 VecDeque
    use std::collections::VecDeque;
    let mut vec_deque = VecDeque::new();
    vec_deque.push_front(1);
    vec_deque.push_back(2);
    println!("   VecDeque适合频繁首尾操作: {:?}", vec_deque);

    println!("\n💡 内存优化最佳实践:");
    println!("   1. 预估容量，使用 with_capacity() 预分配");
    println!("   2. 及时释放不需要的内存，使用 shrink_to_fit()");
    println!("   3. 避免频繁的重新分配");
    println!("   4. 考虑使用 Box<[T]> 替代不再增长的 Vector");
    println!("   5. 对于大对象，考虑存储指针而非对象本身");
}

// ============================================================================
// 6. Vector 切片操作和借用机制
// ============================================================================

fn vector_slicing_analysis() {
    println!("\n=== 6. Vector 切片操作和借用机制 ===");

    let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("\n原始 Vector: {:?}", vec);

    // ========== 基本切片操作 ==========
    println!("\n✂️ 基本切片操作:");

    // 获取切片
    let slice_all = &vec[..]; // 全部元素
    let slice_range = &vec[2..7]; // 索引2到6
    let slice_from = &vec[3..]; // 从索引3到末尾
    let slice_to = &vec[..5]; // 从开始到索引4

    println!("   &vec[..]: {:?}", slice_all);
    println!("   &vec[2..7]: {:?}", slice_range);
    println!("   &vec[3..]: {:?}", slice_from);
    println!("   &vec[..5]: {:?}", slice_to);

    // 使用 get 方法安全获取切片
    if let Some(safe_slice) = vec.get(2..7) {
        println!("   vec.get(2..7): {:?}", safe_slice);
    }

    // 超出范围的安全处理
    match vec.get(5..20) {
        Some(slice) => println!("   vec.get(5..20): {:?}", slice),
        None => println!("   vec.get(5..20): 范围超出边界"),
    }

    // ========== 可变切片操作 ==========
    println!("\n✏️ 可变切片操作:");

    let mut mut_vec = vec![1, 2, 3, 4, 5];
    println!("   原始可变Vector: {:?}", mut_vec);

    // 获取可变切片并修改
    {
        let mut_slice = &mut mut_vec[1..4];
        mut_slice[0] = 20;
        mut_slice[2] = 40;
        println!("   修改切片后: {:?}", mut_slice);
    }
    println!("   Vector变化: {:?}", mut_vec);

    // 使用 get_mut 安全获取可变切片
    if let Some(safe_mut_slice) = mut_vec.get_mut(0..3) {
        safe_mut_slice.reverse();
        println!("   反转前3个元素后: {:?}", mut_vec);
    }

    // ========== 切片方法演示 ==========
    println!("\n🔧 切片方法演示:");

    let data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    let slice = &data[..];

    // 查找操作
    println!("   contains(&5): {}", slice.contains(&5));
    println!("   starts_with(&[3, 1]): {}", slice.starts_with(&[3, 1]));
    println!("   ends_with(&[5, 3]): {}", slice.ends_with(&[5, 3]));

    // 位置查找
    println!(
        "   iter().position(|&x| x == 5): {:?}",
        slice.iter().position(|&x| x == 5)
    );
    println!(
        "   iter().rposition(|&x| x == 5): {:?}",
        slice.iter().rposition(|&x| x == 5)
    );

    // 分割操作
    let parts: Vec<&[i32]> = slice.split(|&x| x == 1).collect();
    println!("   split(|&x| x == 1): {:?}", parts);

    let chunks: Vec<&[i32]> = slice.chunks(3).collect();
    println!("   chunks(3): {:?}", chunks);

    let windows: Vec<&[i32]> = slice.windows(3).collect();
    println!("   windows(3): {:?}", windows);

    demonstrate_borrowing_rules();
}

fn demonstrate_borrowing_rules() {
    println!("\n📋 借用规则演示:");

    let mut vec = vec![1, 2, 3, 4, 5];

    // 规则1: 多个不可变借用可以同时存在
    {
        let slice1 = &vec[0..2];
        let slice2 = &vec[2..4];
        println!("   多个不可变借用: {:?}, {:?}", slice1, slice2);
    } // 借用在此结束

    // 规则2: 可变借用是独占的
    {
        let mut_slice = &mut vec[1..4];
        mut_slice[0] = 20;
        println!("   可变借用修改: {:?}", mut_slice);
        // 在可变借用存在期间，不能有其他借用
    } // 可变借用在此结束

    println!("   修改后的Vector: {:?}", vec);

    // 规则3: 借用检查器确保内存安全
    println!("\n🛡️ 借用检查器保护:");
    println!("   • 防止悬垂指针");
    println!("   • 防止数据竞争");
    println!("   • 确保内存安全");

    // 演示切片的零成本抽象
    println!("\n💰 切片的零成本抽象:");
    let large_vec: Vec<i32> = (0..1000).collect();

    // 传递切片而不是整个Vector
    fn process_slice(data: &[i32]) -> i32 {
        data.iter().sum()
    }

    let sum1 = process_slice(&large_vec[0..100]);
    let sum2 = process_slice(&large_vec[500..600]);

    println!("   处理切片[0..100]的和: {}", sum1);
    println!("   处理切片[500..600]的和: {}", sum2);
    println!("   切片传递无额外开销，只传递指针和长度");
}

// ============================================================================
// 7. Vector 排序算法和自定义比较器
// ============================================================================

fn vector_sorting_analysis() {
    println!("\n=== 7. Vector 排序算法和自定义比较器 ===");

    // ========== 基本排序方法 ==========
    println!("\n📊 基本排序方法:");

    // sort: 原地排序（稳定排序）
    let mut vec1 = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("   排序前: {:?}", vec1);
    vec1.sort();
    println!("   sort() 后: {:?}", vec1);

    // sort_unstable: 不稳定排序（更快）
    let mut vec2 = vec![3, 1, 4, 1, 5, 9, 2, 6];
    vec2.sort_unstable();
    println!("   sort_unstable() 后: {:?}", vec2);

    // reverse: 反转
    let mut vec3 = vec![1, 2, 3, 4, 5];
    vec3.reverse();
    println!("   reverse() 后: {:?}", vec3);

    // ========== 自定义比较器排序 ==========
    println!("\n🎯 自定义比较器排序:");

    // sort_by: 使用自定义比较函数
    let mut vec4 = vec![3, 1, 4, 1, 5, 9, 2, 6];
    vec4.sort_by(|a, b| b.cmp(a)); // 降序排序
    println!("   降序排序: {:?}", vec4);

    // sort_by_key: 根据键值排序
    let mut words = vec!["hello", "world", "rust", "programming"];
    println!("   原始字符串: {:?}", words);

    words.sort_by_key(|s| s.len()); // 按长度排序
    println!("   按长度排序: {:?}", words);

    words.sort_by_key(|s| s.chars().count()); // 按字符数排序
    println!("   按字符数排序: {:?}", words);

    // ========== 复杂数据结构排序 ==========
    println!("\n🏗️ 复杂数据结构排序:");

    #[derive(Debug, Clone)]
    struct Student {
        name: String,
        age: u32,
        score: f64,
    }

    let mut students = vec![
        Student {
            name: "Alice".to_string(),
            age: 20,
            score: 85.5,
        },
        Student {
            name: "Bob".to_string(),
            age: 19,
            score: 92.0,
        },
        Student {
            name: "Charlie".to_string(),
            age: 21,
            score: 78.5,
        },
        Student {
            name: "Diana".to_string(),
            age: 20,
            score: 88.0,
        },
    ];

    println!("   原始学生列表:");
    for student in &students {
        println!("     {:?}", student);
    }

    // 按分数排序
    students.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    println!("\n   按分数降序排序:");
    for student in &students {
        println!("     {} - {:.1}", student.name, student.score);
    }

    // 多级排序：先按年龄，再按分数
    students.sort_by(|a, b| {
        a.age
            .cmp(&b.age)
            .then_with(|| b.score.partial_cmp(&a.score).unwrap())
    });
    println!("\n   多级排序(年龄升序，分数降序):");
    for student in &students {
        println!(
            "     {} - 年龄:{}, 分数:{:.1}",
            student.name, student.age, student.score
        );
    }

    demonstrate_sorting_performance();
}

fn demonstrate_sorting_performance() {
    println!("\n⚡ 排序性能对比:");

    let size = 100_000;
    let mut data1: Vec<i32> = (0..size).rev().collect(); // 逆序数据
    let mut data2 = data1.clone();
    let mut data3 = data1.clone();

    // 测试稳定排序
    let start = Instant::now();
    data1.sort();
    let stable_duration = start.elapsed();

    // 测试不稳定排序
    let start = Instant::now();
    data2.sort_unstable();
    let unstable_duration = start.elapsed();

    // 测试自定义比较器
    let start = Instant::now();
    data3.sort_by(|a, b| a.cmp(b));
    let custom_duration = start.elapsed();

    println!("   稳定排序: {:?}", stable_duration);
    println!("   不稳定排序: {:?}", unstable_duration);
    println!("   自定义比较: {:?}", custom_duration);
    println!(
        "   不稳定排序性能提升: {:.2}x",
        stable_duration.as_nanos() as f64 / unstable_duration.as_nanos() as f64
    );

    // 排序算法特性说明
    println!("\n📚 排序算法特性:");
    println!("   • sort(): 稳定排序，保持相等元素的相对顺序");
    println!("   • sort_unstable(): 不稳定排序，性能更好");
    println!("   • sort_by(): 自定义比较逻辑");
    println!("   • sort_by_key(): 根据键值排序，简洁易用");

    // 特殊排序需求
    demonstrate_special_sorting();
}

fn demonstrate_special_sorting() {
    println!("\n🎨 特殊排序需求:");

    // 部分排序
    let mut vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    println!("   原始数据: {:?}", vec);

    // 只排序前5个元素
    vec[..5].sort();
    println!("   部分排序前5个: {:?}", vec);

    // 查找第k小的元素（使用 select_nth_unstable）
    let mut vec2 = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    vec2.select_nth_unstable(3); // 找第4小的元素
    println!("   select_nth_unstable(3): {:?}", vec2);
    println!("   第4小的元素: {}", vec2[3]);

    // 自定义排序：按绝对值排序
    let mut signed_nums = vec![-5, 3, -1, 4, -2, 6];
    signed_nums.sort_by_key(|&x| (x as i32).abs());
    println!("   按绝对值排序: {:?}", signed_nums);

    // 字符串自然排序
    let mut versions = vec!["v1.10", "v1.2", "v1.1", "v2.0", "v1.20"];
    versions.sort(); // 字典序
    println!("   版本号字典序: {:?}", versions);

    // 更复杂的版本号排序需要自定义逻辑
    let mut versions2 = vec!["v1.10", "v1.2", "v1.1", "v2.0", "v1.20"];
    versions2.sort_by(|a, b| {
        // 简化的版本号比较（实际应用中需要更复杂的解析）
        let parse_version = |s: &str| -> Vec<u32> {
            s.trim_start_matches('v')
                .split('.')
                .map(|n| n.parse().unwrap_or(0))
                .collect()
        };

        let va = parse_version(a);
        let vb = parse_version(b);
        va.cmp(&vb)
    });
    println!("   版本号语义排序: {:?}", versions2);
}

// ============================================================================
// 8. Vector 高级特性：drain、retain、dedup等
// ============================================================================

fn vector_advanced_features() {
    println!("\n=== 8. Vector 高级特性：drain、retain、dedup等 ===");

    // ========== drain 操作 ==========
    println!("\n🚰 drain 操作:");

    let mut vec1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("   原始Vector: {:?}", vec1);

    // drain: 移除并返回指定范围的元素
    let drained: Vec<i32> = vec1.drain(2..5).collect();
    println!("   drain(2..5): {:?}", drained);
    println!("   剩余Vector: {:?}", vec1);

    // drain_filter: 按条件移除元素（需要nightly）
    // let mut vec2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let evens: Vec<i32> = vec2.drain_filter(|x| *x % 2 == 0).collect();

    // 使用 retain 替代 drain_filter
    let mut vec2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut removed = Vec::new();
    vec2.retain(|&x| {
        if x % 2 == 0 {
            removed.push(x);
            false
        } else {
            true
        }
    });
    println!("   移除偶数后: {:?}", vec2);
    println!("   被移除的偶数: {:?}", removed);

    // ========== retain 操作 ==========
    println!("\n🔍 retain 操作:");

    let mut vec3 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("   原始Vector: {:?}", vec3);

    // 保留大于5的元素
    vec3.retain(|&x| x > 5);
    println!("   retain(|&x| x > 5): {:?}", vec3);

    // 字符串retain示例
    let mut words = vec!["hello", "world", "rust", "programming", "language"];
    println!("   原始单词: {:?}", words);

    // 保留长度大于4的单词
    words.retain(|word| word.len() > 4);
    println!("   retain(长度>4): {:?}", words);

    // ========== dedup 去重操作 ==========
    println!("\n🔄 dedup 去重操作:");

    let mut vec4 = vec![1, 2, 2, 3, 3, 3, 4, 4, 5];
    println!("   原始Vector: {:?}", vec4);

    // dedup: 移除连续的重复元素
    vec4.dedup();
    println!("   dedup() 后: {:?}", vec4);

    // dedup_by: 使用自定义比较函数去重
    let mut vec5 = vec![1, 2, 3, 2, 4, 3, 5, 4];
    println!("   原始Vector: {:?}", vec5);

    // 先排序再去重以移除所有重复
    vec5.sort();
    vec5.dedup();
    println!("   排序+去重后: {:?}", vec5);

    // dedup_by_key: 根据键值去重
    #[derive(Debug, Clone)]
    struct Item {
        id: u32,
        name: String,
    }

    let mut items = vec![
        Item {
            id: 1,
            name: "Apple".to_string(),
        },
        Item {
            id: 2,
            name: "Banana".to_string(),
        },
        Item {
            id: 1,
            name: "Apple2".to_string(),
        }, // 相同ID
        Item {
            id: 3,
            name: "Cherry".to_string(),
        },
        Item {
            id: 2,
            name: "Banana2".to_string(),
        }, // 相同ID
    ];

    println!("   原始items:");
    for item in &items {
        println!("     {:?}", item);
    }

    // 按ID排序后去重
    items.sort_by_key(|item| item.id);
    items.dedup_by_key(|item| item.id);

    println!("   按ID去重后:");
    for item in &items {
        println!("     {:?}", item);
    }

    demonstrate_advanced_patterns();
}

fn demonstrate_advanced_patterns() {
    println!("\n🎯 高级使用模式:");

    // 模式1: 分区操作
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("   原始数据: {:?}", vec);

    // partition_point: 找到分区点
    let partition_point = vec.partition_point(|&x| x <= 5);
    println!("   分区点(<=5): {}", partition_point);

    let (left, right) = vec.split_at(partition_point);
    println!("   左分区: {:?}", left);
    println!("   右分区: {:?}", right);

    // 模式2: 窗口操作
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 滑动窗口求和
    let window_sums: Vec<i32> = data.windows(3).map(|window| window.iter().sum()).collect();
    println!("   滑动窗口(3)求和: {:?}", window_sums);

    // 分块处理
    let chunk_sums: Vec<i32> = data.chunks(3).map(|chunk| chunk.iter().sum()).collect();
    println!("   分块(3)求和: {:?}", chunk_sums);

    // 模式3: 条件处理
    let mut mixed_data = vec![1, -2, 3, -4, 5, -6, 7, -8, 9, -10];
    println!("   混合数据: {:?}", mixed_data);

    // 将负数转为正数
    let mut mixed_data = mixed_data;
    mixed_data.iter_mut().for_each(|mut x| {
        if *x < 0 {
            *x = -*x;
        }
    });
    println!("   转换负数后: {:?}", mixed_data);

    // 模式4: 复杂的数据变换
    let text_data = vec!["hello", "world", "rust", "programming"];

    // 转换为大写并过滤长度
    let processed: Vec<String> = text_data
        .iter()
        .map(|s| s.to_uppercase())
        .filter(|s| s.len() > 4)
        .collect();
    println!("   处理文本数据: {:?}", processed);

    // 模式5: 错误处理与Vector
    let string_numbers = vec!["1", "2", "invalid", "4", "5"];

    // 收集成功解析的数字
    let parsed_numbers: Vec<i32> = string_numbers
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    println!("   解析数字: {:?}", parsed_numbers);

    // 收集解析结果（包括错误）
    let parse_results: Vec<Result<i32, _>> = string_numbers.iter().map(|s| s.parse()).collect();
    println!("   解析结果:");
    for (i, result) in parse_results.iter().enumerate() {
        match result {
            Ok(num) => println!("     [{}]: 成功解析 -> {}", i, num),
            Err(e) => println!("     [{}]: 解析失败 -> {}", i, e),
        }
    }
}

// ============================================================================
// 9. Vector 性能优化策略和基准测试
// ============================================================================

fn vector_performance_analysis() {
    println!("\n=== 9. Vector 性能优化策略和基准测试 ===");

    // ========== 容量预分配性能测试 ==========
    println!("\n⚡ 容量预分配性能测试:");

    let sizes = [1_000, 10_000, 100_000, 1_000_000];

    for &size in &sizes {
        // 测试不预分配
        let start = Instant::now();
        let mut vec_no_prealloc = Vec::new();
        for i in 0..size {
            vec_no_prealloc.push(i);
        }
        let duration_no_prealloc = start.elapsed();

        // 测试预分配
        let start = Instant::now();
        let mut vec_prealloc = Vec::with_capacity(size);
        for i in 0..size {
            vec_prealloc.push(i);
        }
        let duration_prealloc = start.elapsed();

        let speedup = duration_no_prealloc.as_nanos() as f64 / duration_prealloc.as_nanos() as f64;
        println!(
            "   大小 {}: 不预分配 {:?}, 预分配 {:?}, 提升 {:.2}x",
            size, duration_no_prealloc, duration_prealloc, speedup
        );
    }

    // ========== 不同访问模式性能对比 ==========
    println!("\n🔍 不同访问模式性能对比:");

    let vec: Vec<i32> = (0..1_000_000).collect();

    // 顺序访问
    let start = Instant::now();
    let mut sum1 = 0i64;
    for i in 0..vec.len() {
        sum1 += vec[i] as i64;
    }
    let sequential_duration = start.elapsed();

    // 随机访问
    let start = Instant::now();
    let mut sum2 = 0i64;
    for i in (0..vec.len()).step_by(1000) {
        sum2 += vec[i] as i64;
    }
    let random_duration = start.elapsed();

    // 迭代器访问
    let start = Instant::now();
    let sum3: i64 = vec.iter().map(|&x| x as i64).sum();
    let iterator_duration = start.elapsed();

    println!("   顺序访问: {:?} (sum: {})", sequential_duration, sum1);
    println!("   随机访问: {:?} (sum: {})", random_duration, sum2);
    println!("   迭代器访问: {:?} (sum: {})", iterator_duration, sum3);

    // ========== 内存使用优化 ==========
    println!("\n💾 内存使用优化:");

    // 大对象存储策略对比
    #[derive(Clone)]
    struct LargeObject {
        data: [u8; 1024], // 1KB 数据
        id: u32,
    }

    impl LargeObject {
        fn new(id: u32) -> Self {
            Self {
                data: [0; 1024],
                id,
            }
        }
    }

    // 策略1: 直接存储对象
    let start = Instant::now();
    let mut vec_objects = Vec::with_capacity(1000);
    for i in 0..1000 {
        vec_objects.push(LargeObject::new(i));
    }
    let direct_duration = start.elapsed();
    let direct_memory = vec_objects.capacity() * std::mem::size_of::<LargeObject>();

    // 策略2: 存储 Box 指针
    let start = Instant::now();
    let mut vec_boxed = Vec::with_capacity(1000);
    for i in 0..1000 {
        vec_boxed.push(Box::new(LargeObject::new(i)));
    }
    let boxed_duration = start.elapsed();
    let boxed_memory = vec_boxed.capacity() * std::mem::size_of::<Box<LargeObject>>();

    println!(
        "   直接存储: {:?}, 内存: {} bytes",
        direct_duration, direct_memory
    );
    println!(
        "   Box存储: {:?}, 内存: {} bytes (Vector部分)",
        boxed_duration, boxed_memory
    );

    demonstrate_cache_performance();
}

fn demonstrate_cache_performance() {
    println!("\n🚀 缓存友好性能优化:");

    let size = 1_000_000;
    let vec: Vec<i32> = (0..size).collect();

    // 缓存友好的顺序访问
    let start = Instant::now();
    let mut sum1 = 0i64;
    for &value in &vec {
        sum1 += value as i64;
    }
    let cache_friendly_duration = start.elapsed();

    // 缓存不友好的跳跃访问
    let start = Instant::now();
    let mut sum2 = 0i64;
    let step = 1000;
    for i in (0..size).step_by(step) {
        sum2 += vec[i as usize] as i64;
    }
    let cache_unfriendly_duration = start.elapsed();

    println!(
        "   缓存友好访问: {:?} (sum: {})",
        cache_friendly_duration, sum1
    );
    println!(
        "   缓存不友好访问: {:?} (sum: {})",
        cache_unfriendly_duration, sum2
    );

    // SIMD 优化示例（概念性）
    println!("\n🔧 SIMD 优化概念:");
    println!("   • 使用 SIMD 指令可以并行处理多个元素");
    println!("   • Rust 的迭代器在某些情况下会自动向量化");
    println!("   • 对于数值计算密集的操作，考虑使用专门的库");

    // 分支预测优化
    println!("\n🎯 分支预测优化:");
    let mixed_data: Vec<i32> = (0..100_000)
        .map(|i| if i % 2 == 0 { i } else { -i })
        .collect();

    // 有分支的处理
    let start = Instant::now();
    let mut positive_sum = 0i64;
    let mut negative_sum = 0i64;
    for &value in &mixed_data {
        if value > 0 {
            positive_sum += value as i64;
        } else {
            negative_sum += value as i64;
        }
    }
    let branched_duration = start.elapsed();

    // 无分支的处理（使用迭代器）
    let start = Instant::now();
    let positive_sum2: i64 = mixed_data
        .iter()
        .filter(|&&x| x > 0)
        .map(|&x| x as i64)
        .sum();
    let negative_sum2: i64 = mixed_data
        .iter()
        .filter(|&&x| x <= 0)
        .map(|&x| x as i64)
        .sum();
    let branchless_duration = start.elapsed();

    println!(
        "   有分支处理: {:?} (正数和: {}, 负数和: {})",
        branched_duration, positive_sum, negative_sum
    );
    println!(
        "   迭代器处理: {:?} (正数和: {}, 负数和: {})",
        branchless_duration, positive_sum2, negative_sum2
    );
}

// ============================================================================
// 10. Vector 在实际项目中的应用案例
// ============================================================================

fn vector_real_world_examples() {
    println!("\n=== 10. Vector 在实际项目中的应用案例 ===");

    // ========== 案例1: 数据处理管道 ==========
    println!("\n📊 案例1: 数据处理管道");

    #[derive(Debug, Clone)]
    struct DataPoint {
        timestamp: u64,
        value: f64,
        category: String,
    }

    impl DataPoint {
        fn new(timestamp: u64, value: f64, category: &str) -> Self {
            Self {
                timestamp,
                value,
                category: category.to_string(),
            }
        }
    }

    // 模拟原始数据
    let raw_data = vec![
        DataPoint::new(1000, 10.5, "A"),
        DataPoint::new(1001, 15.2, "B"),
        DataPoint::new(1002, 8.7, "A"),
        DataPoint::new(1003, 22.1, "C"),
        DataPoint::new(1004, 12.3, "B"),
        DataPoint::new(1005, 18.9, "A"),
    ];

    println!("   原始数据: {} 条记录", raw_data.len());

    // 数据处理管道
    let processed_data: Vec<DataPoint> = raw_data
        .into_iter()
        .filter(|dp| dp.value > 10.0) // 过滤小值
        .map(|mut dp| {
            dp.value = dp.value * 1.1; // 应用校准因子
            dp
        })
        .collect();

    println!("   处理后数据: {} 条记录", processed_data.len());

    // 按类别分组
    let mut grouped_data: HashMap<String, Vec<DataPoint>> = HashMap::new();
    for data_point in processed_data {
        grouped_data
            .entry(data_point.category.clone())
            .or_insert_with(Vec::new)
            .push(data_point);
    }

    println!("   分组结果:");
    for (category, points) in &grouped_data {
        let avg_value: f64 = points.iter().map(|p| p.value).sum::<f64>() / points.len() as f64;
        println!(
            "     类别 {}: {} 条记录, 平均值: {:.2}",
            category,
            points.len(),
            avg_value
        );
    }

    // ========== 案例2: 缓存系统 ==========
    println!("\n💾 案例2: LRU 缓存系统");

    struct LRUCache<K, V> {
        capacity: usize,
        items: Vec<(K, V)>,
    }

    impl<K: Clone + PartialEq, V: Clone> LRUCache<K, V> {
        fn new(capacity: usize) -> Self {
            Self {
                capacity,
                items: Vec::with_capacity(capacity),
            }
        }

        fn get(&mut self, key: &K) -> Option<V> {
            if let Some(pos) = self.items.iter().position(|(k, _)| k == key) {
                let (k, v) = self.items.remove(pos);
                self.items.push((k, v.clone()));
                Some(v)
            } else {
                None
            }
        }

        fn put(&mut self, key: K, value: V) {
            if let Some(pos) = self.items.iter().position(|(k, _)| k == &key) {
                self.items.remove(pos);
            } else if self.items.len() >= self.capacity {
                self.items.remove(0);
            }
            self.items.push((key, value));
        }

        fn len(&self) -> usize {
            self.items.len()
        }
    }

    let mut cache = LRUCache::new(3);
    cache.put("key1", "value1");
    cache.put("key2", "value2");
    cache.put("key3", "value3");

    println!("   缓存大小: {}", cache.len());
    println!("   获取 key1: {:?}", cache.get(&"key1"));

    cache.put("key4", "value4"); // 应该移除 key2
    println!("   添加 key4 后，获取 key2: {:?}", cache.get(&"key2"));

    // ========== 案例3: 事件处理系统 ==========
    println!("\n🎯 案例3: 事件处理系统");

    #[derive(Debug, Clone)]
    enum Event {
        Click { x: i32, y: i32 },
        KeyPress { key: char },
        Resize { width: u32, height: u32 },
    }

    struct EventProcessor {
        event_queue: Vec<Event>,
        handlers: Vec<Box<dyn Fn(&Event)>>,
    }

    impl EventProcessor {
        fn new() -> Self {
            Self {
                event_queue: Vec::new(),
                handlers: Vec::new(),
            }
        }

        fn add_event(&mut self, event: Event) {
            self.event_queue.push(event);
        }

        fn add_handler<F>(&mut self, handler: F)
        where
            F: Fn(&Event) + 'static,
        {
            self.handlers.push(Box::new(handler));
        }

        fn process_events(&mut self) {
            let events = std::mem::take(&mut self.event_queue);
            for event in &events {
                for handler in &self.handlers {
                    handler(event);
                }
            }
            println!("   处理了 {} 个事件", events.len());
        }
    }

    let mut processor = EventProcessor::new();

    // 添加事件处理器
    processor.add_handler(|event| match event {
        Event::Click { x, y } => println!("     点击事件: ({}, {})", x, y),
        Event::KeyPress { key } => println!("     按键事件: {}", key),
        Event::Resize { width, height } => println!("     调整大小: {}x{}", width, height),
    });

    // 添加事件
    processor.add_event(Event::Click { x: 100, y: 200 });
    processor.add_event(Event::KeyPress { key: 'a' });
    processor.add_event(Event::Resize {
        width: 800,
        height: 600,
    });

    // 处理事件
    processor.process_events();

    // ========== 案例4: 批处理优化 ==========
    println!("\n⚡ 案例4: 批处理优化");

    struct BatchProcessor<T> {
        batch: Vec<T>,
        batch_size: usize,
        processor: Box<dyn Fn(&[T])>,
    }

    impl<T> BatchProcessor<T> {
        fn new<F>(batch_size: usize, processor: F) -> Self
        where
            F: Fn(&[T]) + 'static,
        {
            Self {
                batch: Vec::with_capacity(batch_size),
                batch_size,
                processor: Box::new(processor),
            }
        }

        fn add(&mut self, item: T) {
            self.batch.push(item);
            if self.batch.len() >= self.batch_size {
                self.flush();
            }
        }

        fn flush(&mut self) {
            if !self.batch.is_empty() {
                (self.processor)(&self.batch);
                self.batch.clear();
            }
        }
    }

    let mut batch_processor = BatchProcessor::new(3, |batch: &[i32]| {
        let sum: i32 = batch.iter().sum();
        println!("     处理批次: {:?}, 总和: {}", batch, sum);
    });

    // 添加数据
    for i in 1..=10 {
        batch_processor.add(i);
    }

    // 处理剩余数据
    batch_processor.flush();

    println!("\n💡 实际应用最佳实践:");
    println!("   1. 数据处理：使用迭代器链进行高效的数据转换");
    println!("   2. 缓存系统：Vector 适合实现简单的 LRU 缓存");
    println!("   3. 事件系统：Vector 可以作为事件队列使用");
    println!("   4. 批处理：预分配容量，批量处理提高性能");
    println!("   5. 内存管理：及时释放不需要的内存");
    println!("   6. 错误处理：结合 Result 类型进行安全的数据处理");
}
