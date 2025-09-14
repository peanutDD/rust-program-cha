//! 切片 vs 切片引用：实际代码对比示例
//! 
//! 本模块通过具体的代码示例展示切片 [T] 和切片引用 &[T] 的区别

use std::mem;
use std::time::Instant;

/// 1. 基本类型对比演示
pub fn basic_type_comparison() {
    println!("=== 1. 基本类型对比 ===");
    
    let array = [1, 2, 3, 4, 5];
    
    // ❌ 不能直接声明切片类型
    // let slice: [i32] = array[1..4]; // 编译错误！
    
    // ✅ 只能通过引用使用切片
    let slice_ref: &[i32] = &array[1..4];
    
    println!("原数组: {:?}", array);
    println!("切片引用: {:?}", slice_ref);
    
    // 类型信息对比
    println!("\n类型信息:");
    println!("  array 类型: {}", std::any::type_name::<[i32; 5]>());
    println!("  slice_ref 类型: {}", std::any::type_name::<&[i32]>());
    
    // 大小信息对比
    println!("\n大小信息:");
    println!("  array 占用: {} 字节", mem::size_of_val(&array));
    println!("  slice_ref 占用: {} 字节 (胖指针)", mem::size_of_val(&slice_ref));
    println!("  slice_ref 指向的数据: {} 字节", mem::size_of_val(slice_ref));
    
    println!();
}

/// 2. 内存布局对比演示
pub fn memory_layout_comparison() {
    println!("=== 2. 内存布局对比 ===");
    
    let data = vec![10, 20, 30, 40, 50];
    let slice_ref = &data[1..4]; // [20, 30, 40]
    
    println!("Vec 数据: {:?}", data);
    println!("切片引用: {:?}", slice_ref);
    
    // 内存地址信息
    println!("\n内存布局:");
    println!("  Vec 地址: {:p}", data.as_ptr());
    println!("  切片引用指向: {:p}", slice_ref.as_ptr());
    println!("  切片引用本身地址: {:p}", &slice_ref);
    
    // 胖指针结构
    let fat_pointer = slice_ref as *const [i32];
    let (ptr, len) = (fat_pointer as *const i32, slice_ref.len());
    
    println!("\n胖指针结构:");
    println!("  数据指针: {:p}", ptr);
    println!("  长度: {}", len);
    println!("  胖指针大小: {} 字节", mem::size_of::<&[i32]>());
    
    // 验证指针偏移
    unsafe {
        println!("\n指针验证:");
        for i in 0..len {
            let value = *ptr.add(i);
            println!("  偏移 {}: 地址 {:p}, 值 {}", i, ptr.add(i), value);
        }
    }
    
    println!();
}

/// 3. 函数参数对比演示
pub fn function_parameter_comparison() {
    println!("=== 3. 函数参数对比 ===");
    
    // ❌ 不能直接接受切片类型
    // fn process_slice(slice: [i32]) { } // 编译错误
    
    fn process_slice_ref(slice: &[i32]) -> i32 {
        slice.iter().sum()
    }
    
    fn process_mut_slice_ref(slice: &mut [i32]) {
        for item in slice.iter_mut() {
            *item *= 2;
        }
    }
    
    let mut array = [1, 2, 3, 4, 5];
    let mut vec = vec![10, 20, 30];
    
    println!("原始数据:");
    println!("  array: {:?}", array);
    println!("  vec: {:?}", vec);
    
    // 处理不同来源的切片引用
    let sum1 = process_slice_ref(&array[1..4]);
    let sum2 = process_slice_ref(&vec);
    
    println!("\n不可变处理结果:");
    println!("  array[1..4] 求和: {}", sum1);
    println!("  vec 求和: {}", sum2);
    
    // 可变处理
    process_mut_slice_ref(&mut array[1..4]);
    process_mut_slice_ref(&mut vec);
    
    println!("\n可变处理后:");
    println!("  array: {:?}", array);
    println!("  vec: {:?}", vec);
    
    println!();
}

/// 4. 所有权和借用对比演示
pub fn ownership_borrowing_comparison() {
    println!("=== 4. 所有权和借用对比 ===");
    
    let vec = vec![1, 2, 3, 4, 5];
    
    // 多个不可变切片引用可以同时存在
    let slice1: &[i32] = &vec[0..3];
    let slice2: &[i32] = &vec[2..5];
    let slice3: &[i32] = &vec;
    
    println!("同时存在的不可变切片引用:");
    println!("  slice1 (0..3): {:?}", slice1);
    println!("  slice2 (2..5): {:?}", slice2);
    println!("  slice3 (全部): {:?}", slice3);
    println!("  原始 vec: {:?}", vec);
    
    // 演示可变借用的排他性
    {
        let mut vec2 = vec![10, 20, 30, 40, 50];
        println!("\n可变借用演示:");
        println!("  原始 vec2: {:?}", vec2);
        
        {
            let mut_slice: &mut [i32] = &mut vec2[1..4];
            mut_slice[0] = 100;
            println!("  修改 mut_slice[0] = 100");
            
            // 在可变借用期间，不能有其他借用
            // let another_ref = &vec2; // ❌ 编译错误
            println!("  mut_slice: {:?}", mut_slice);
        } // 可变借用结束
        
        // 现在可以再次访问
        println!("  修改后 vec2: {:?}", vec2);
    }
    
    println!();
}

/// 5. 性能特性对比演示
pub fn performance_comparison() {
    println!("=== 5. 性能特性对比 ===");
    
    let data: Vec<i32> = (0..10_000).collect();
    let slice_ref: &[i32] = &data;
    
    println!("测试数据大小: {} 个元素", data.len());
    
    // 测试1: 迭代器访问
    let start = Instant::now();
    let sum1: i32 = slice_ref.iter().sum();
    let time1 = start.elapsed();
    
    // 测试2: 索引访问
    let start = Instant::now();
    let mut sum2 = 0;
    for i in 0..slice_ref.len() {
        sum2 += slice_ref[i];
    }
    let time2 = start.elapsed();
    
    // 测试3: 不安全访问
    let start = Instant::now();
    let mut sum3 = 0;
    unsafe {
        let ptr = slice_ref.as_ptr();
        for i in 0..slice_ref.len() {
            sum3 += *ptr.add(i);
        }
    }
    let time3 = start.elapsed();
    
    println!("\n性能测试结果:");
    println!("  迭代器访问: {} (耗时: {:?})", sum1, time1);
    println!("  索引访问: {} (耗时: {:?})", sum2, time2);
    println!("  不安全访问: {} (耗时: {:?})", sum3, time3);
    
    assert_eq!(sum1, sum2);
    assert_eq!(sum2, sum3);
    println!("  ✓ 所有结果一致");
    
    // 传递成本测试
    fn process_by_value(vec: Vec<i32>) -> i32 {
        vec.iter().sum()
    }
    
    fn process_by_slice_ref(slice: &[i32]) -> i32 {
        slice.iter().sum()
    }
    
    let test_data = vec![1, 2, 3, 4, 5];
    
    println!("\n传递成本对比:");
    println!("  Vec 传递: 需要移动所有权或克隆");
    println!("  切片引用传递: 仅传递 16 字节胖指针");
    
    let result1 = process_by_slice_ref(&test_data); // 零成本
    let result2 = process_by_value(test_data.clone()); // 需要克隆
    
    println!("  切片引用结果: {}", result1);
    println!("  Vec 值结果: {}", result2);
    
    println!();
}

/// 6. 类型转换对比演示
pub fn type_conversion_comparison() {
    println!("=== 6. 类型转换对比 ===");
    
    // 从不同类型创建切片引用
    let array = [1, 2, 3, 4, 5];
    let vec = vec![10, 20, 30, 40, 50];
    let string = String::from("hello");
    
    println!("原始数据:");
    println!("  array: {:?}", array);
    println!("  vec: {:?}", vec);
    println!("  string: {:?}", string);
    
    // 创建切片引用的不同方式
    let from_array: &[i32] = &array;
    let from_array_range: &[i32] = &array[1..4];
    let from_vec: &[i32] = &vec;
    let from_vec_as_slice: &[i32] = vec.as_slice();
    let from_string: &[u8] = string.as_bytes();
    let from_str: &[u8] = "world".as_bytes();
    
    println!("\n切片引用创建:");
    println!("  从数组: {:?}", from_array);
    println!("  从数组范围: {:?}", from_array_range);
    println!("  从 Vec: {:?}", from_vec);
    println!("  Vec.as_slice(): {:?}", from_vec_as_slice);
    println!("  从 String: {:?}", from_string);
    println!("  从 &str: {:?}", from_str);
    
    // 切片引用之间的转换
    let slice_ref = &vec[1..4];
    let to_vec: Vec<i32> = slice_ref.to_vec(); // 克隆数据
    let to_array: [i32; 3] = slice_ref.try_into().unwrap(); // 固定大小
    
    println!("\n切片引用转换:");
    println!("  原切片引用: {:?}", slice_ref);
    println!("  转为 Vec: {:?}", to_vec);
    println!("  转为数组: {:?}", to_array);
    
    println!();
}

/// 7. 错误示例对比
pub fn error_examples_comparison() {
    println!("=== 7. 常见错误示例 ===");
    
    // 错误1: 尝试直接使用切片类型
    println!("错误1: 不能直接声明切片类型");
    println!("  ❌ let slice: [i32] = [1, 2, 3]; // 编译错误");
    println!("  ✅ let slice_ref: &[i32] = &[1, 2, 3];");
    
    // 错误2: 混淆大小概念
    let array = [1, 2, 3, 4, 5];
    let slice_ref = &array[1..4];
    
    println!("\n错误2: 混淆大小概念");
    println!("  切片引用本身大小: {} 字节 (胖指针)", mem::size_of_val(&slice_ref));
    println!("  切片引用指向数据大小: {} 字节 (实际数据)", mem::size_of_val(slice_ref));
    println!("  ❌ 错误理解: 认为两者相等");
    println!("  ✅ 正确理解: 胖指针 vs 数据大小");
    
    // 错误3: 生命周期问题
    println!("\n错误3: 生命周期问题");
    println!("  ❌ 错误代码:");
    println!("      let slice_ref;");
    println!("      {{");
    println!("          let vec = vec![1, 2, 3];");
    println!("          slice_ref = &vec[..]; // vec 在此作用域结束后被销毁");
    println!("      }}");
    println!("      println!(\"{{:?}}\", slice_ref); // 悬垂引用！");
    
    println!("  ✅ 正确做法: 确保数据生命周期足够长");
    
    // 错误4: 可变借用冲突
    println!("\n错误4: 可变借用冲突");
    let mut vec = vec![1, 2, 3, 4, 5];
    println!("  原始数据: {:?}", vec);
    
    {
        let _mut_slice = &mut vec[1..4];
        // let _immut_ref = &vec; // ❌ 编译错误：可变借用期间不能有不可变借用
        println!("  在可变借用期间，不能创建其他借用");
    }
    
    let _immut_ref = &vec; // ✅ 现在可以了
    println!("  可变借用结束后，可以创建不可变借用");
    
    println!();
}

/// 8. 高级用法对比
pub fn advanced_usage_comparison() {
    println!("=== 8. 高级用法对比 ===");
    
    // 泛型函数中的切片
    fn generic_slice_processor<T: std::fmt::Debug + Clone>(slice: &[T]) -> Vec<T> {
        slice.to_vec()
    }
    
    let int_slice: &[i32] = &[1, 2, 3];
    let str_slice: &[&str] = &["a", "b", "c"];
    
    println!("泛型处理:");
    println!("  整数切片: {:?} -> {:?}", int_slice, generic_slice_processor(int_slice));
    println!("  字符串切片: {:?} -> {:?}", str_slice, generic_slice_processor(str_slice));
    
    // 切片的切片
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice1 = &data[2..8]; // [3, 4, 5, 6, 7, 8]
    let slice2 = &slice1[1..4]; // [4, 5, 6]
    
    println!("\n嵌套切片:");
    println!("  原数据: {:?}", data);
    println!("  第一层切片 [2..8]: {:?}", slice1);
    println!("  第二层切片 [1..4]: {:?}", slice2);
    
    // 动态切片创建
    fn create_dynamic_slice(data: &[i32], start: usize, len: usize) -> Option<&[i32]> {
        if start + len <= data.len() {
            Some(&data[start..start + len])
        } else {
            None
        }
    }
    
    println!("\n动态切片创建:");
    if let Some(dynamic_slice) = create_dynamic_slice(&data, 3, 4) {
        println!("  动态切片 [3, 4): {:?}", dynamic_slice);
    }
    
    if let Some(invalid_slice) = create_dynamic_slice(&data, 8, 5) {
        println!("  无效切片: {:?}", invalid_slice);
    } else {
        println!("  无效切片请求被安全拒绝");
    }
    
    println!();
}

/// 运行所有对比示例
pub fn run_all_comparisons() {
    println!("🔍 Rust 切片 vs 切片引用：详细对比演示\n");
    
    basic_type_comparison();
    memory_layout_comparison();
    function_parameter_comparison();
    ownership_borrowing_comparison();
    performance_comparison();
    type_conversion_comparison();
    error_examples_comparison();
    advanced_usage_comparison();
    
    println!("📚 总结:");
    println!("  • 切片 [T] 是动态大小类型，不能直接使用");
    println!("  • 切片引用 &[T] 是胖指针，包含指针和长度");
    println!("  • 切片引用提供零成本抽象和类型安全");
    println!("  • 理解两者区别是掌握 Rust 内存模型的关键");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_slice_reference_size() {
        let array = [1, 2, 3, 4, 5];
        let slice_ref = &array[1..4];
        
        // 在64位系统上，切片引用应该是16字节
        assert_eq!(mem::size_of_val(&slice_ref), 16);
        // 指向的数据应该是12字节 (3个i32)
        assert_eq!(mem::size_of_val(slice_ref), 12);
    }
    
    #[test]
    fn test_slice_reference_content() {
        let vec = vec![10, 20, 30, 40, 50];
        let slice_ref = &vec[1..4];
        
        assert_eq!(slice_ref, &[20, 30, 40]);
        assert_eq!(slice_ref.len(), 3);
    }
    
    #[test]
    fn test_multiple_slice_references() {
        let data = vec![1, 2, 3, 4, 5];
        let slice1 = &data[0..3];
        let slice2 = &data[2..5];
        
        assert_eq!(slice1, &[1, 2, 3]);
        assert_eq!(slice2, &[3, 4, 5]);
        // 两个切片引用可以同时存在
        assert_eq!(slice1[2], slice2[0]); // 都是3
    }
}