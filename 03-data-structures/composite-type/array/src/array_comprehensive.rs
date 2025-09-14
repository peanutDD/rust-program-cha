//! Rust 数组全面教程
//!
//! 本模块提供了 Rust 数组的全面讲解，包括：
//! - 数组的基本概念和特性
//! - 数组的声明和初始化
//! - 数组的访问和操作
//! - 数组的遍历方法
//! - 多维数组
//! - 数组切片
//! - 数组的常用方法
//! - 数组与其他类型的比较
//! - 实际应用案例

use std::mem;

/// 1. 数组基础概念
///
/// 数组是一组拥有相同类型 T 的对象的集合，在内存中连续存储
/// - 固定长度：编译时确定大小
/// - 同类型元素：所有元素必须是相同类型
/// - 栈上分配：数组存储在栈上，性能高
/// - 类型标记：[T; N]，T 是元素类型，N 是数组长度
pub fn array_basics() {
    println!("=== 数组基础概念 ===");

    // 基本数组声明
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 5]; // 类型推断

    println!("arr1: {:?}", arr1);
    println!("arr2: {:?}", arr2);

    // 数组类型信息
    println!("数组类型: {}", std::any::type_name::<[i32; 5]>());
    println!("数组大小: {} 字节", mem::size_of_val(&arr1));
    println!("元素大小: {} 字节", mem::size_of::<i32>());
    println!("数组长度: {} 个元素", arr1.len());

    // 数组在栈上分配的证明
    println!("数组地址: {:p}", &arr1);
    println!("第一个元素地址: {:p}", &arr1[0]);
    println!("第二个元素地址: {:p}", &arr1[1]);
    println!(
        "地址差: {} 字节",
        &arr1[1] as *const i32 as usize - &arr1[0] as *const i32 as usize
    );
}

/// 2. 数组的声明和初始化
///
/// Rust 提供了多种数组初始化方式
pub fn array_initialization() {
    println!("\n=== 数组的声明和初始化 ===");

    // 1. 直接初始化
    let arr1 = [1, 2, 3, 4, 5];
    println!("直接初始化: {:?}", arr1);

    // 2. 显式类型声明
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    println!("显式类型: {:?}", arr2);

    // 3. 重复值初始化
    let arr3 = [0; 10]; // 10个0
    let arr4 = [42; 5]; // 5个42
    println!("重复值初始化 (10个0): {:?}", arr3);
    println!("重复值初始化 (5个42): {:?}", arr4);

    // 4. 不同类型的数组
    let float_arr: [f64; 3] = [1.1, 2.2, 3.3];
    let char_arr: [char; 4] = ['a', 'b', 'c', 'd'];
    let bool_arr: [bool; 3] = [true, false, true];
    let str_arr: [&str; 3] = ["hello", "world", "rust"];

    println!("浮点数组: {:?}", float_arr);
    println!("字符数组: {:?}", char_arr);
    println!("布尔数组: {:?}", bool_arr);
    println!("字符串数组: {:?}", str_arr);

    // 5. 可变数组
    let mut mut_arr = [1, 2, 3, 4, 5];
    println!("修改前: {:?}", mut_arr);
    mut_arr[0] = 10;
    mut_arr[4] = 50;
    println!("修改后: {:?}", mut_arr);

    // 6. 字节数组
    let byte_arr: [u8; 5] = [65, 66, 67, 68, 69]; // ASCII 码
    println!("字节数组: {:?}", byte_arr);
    println!("转为字符: {:?}", byte_arr.map(|b| b as char));

    // 7. 字节字符串字面量
    let byte_string: &[u8; 11] = b"Hello World";
    println!("字节字符串: {:?}", byte_string);
    for &byte in byte_string {
        print!("{} ", byte as char);
    }
    println!();
}

/// 3. 数组的访问和边界检查
///
/// Rust 提供安全的数组访问，包括编译时和运行时边界检查
pub fn array_access() {
    println!("\n=== 数组的访问和边界检查 ===");

    let arr = [10, 20, 30, 40, 50];

    // 1. 基本索引访问
    println!("第一个元素: {}", arr[0]);
    println!("最后一个元素: {}", arr[arr.len() - 1]);

    // 2. 安全访问方法
    match arr.get(2) {
        Some(value) => println!("索引2的值: {}", value),
        None => println!("索引2越界"),
    }

    match arr.get(10) {
        Some(value) => println!("索引10的值: {}", value),
        None => println!("索引10越界"),
    }

    // 3. 使用 get_unchecked (unsafe)
    unsafe {
        println!("不安全访问索引1: {}", arr.get_unchecked(1));
    }

    // 4. 数组长度和容量
    println!("数组长度: {}", arr.len());
    println!("数组是否为空: {}", arr.is_empty());

    // 5. 首尾元素访问
    println!("首元素: {:?}", arr.first());
    println!("尾元素: {:?}", arr.last());

    // 6. 可变访问
    let mut mut_arr = [1, 2, 3, 4, 5];
    if let Some(first) = mut_arr.first_mut() {
        *first = 100;
    }
    if let Some(last) = mut_arr.last_mut() {
        *last = 500;
    }
    println!("修改后的数组: {:?}", mut_arr);

    // 注意：以下代码会导致 panic（运行时错误）
    // println!("{}", arr[10]); // 索引越界
}

/// 4. 数组的遍历方法
///
/// Rust 提供了多种遍历数组的方式
pub fn array_iteration() {
    println!("\n=== 数组的遍历方法 ===");

    let arr = [1, 2, 3, 4, 5];

    // 1. 基本 for 循环（索引遍历）
    println!("1. 索引遍历:");
    for i in 0..arr.len() {
        println!("  arr[{}] = {}", i, arr[i]);
    }

    // 2. for-in 直接遍历（值遍历）
    println!("2. 值遍历:");
    for value in arr {
        println!("  值: {}", value);
    }

    // 3. 引用遍历
    println!("3. 引用遍历:");
    for value in &arr {
        println!("  引用值: {}", value);
    }

    // 4. 使用 iter() 迭代器
    println!("4. iter() 迭代器:");
    for value in arr.iter() {
        println!("  迭代器值: {}", value);
    }

    // 5. 使用 enumerate() 获取索引和值
    println!("5. enumerate() 索引和值:");
    for (index, value) in arr.iter().enumerate() {
        println!("  arr[{}] = {}", index, value);
    }

    // 6. 可变遍历
    let mut mut_arr = [1, 2, 3, 4, 5];
    println!("6. 可变遍历（修改前）: {:?}", mut_arr);
    for value in mut_arr.iter_mut() {
        *value *= 2;
    }
    println!("   可变遍历（修改后）: {:?}", mut_arr);

    // 7. 使用 into_iter() 获取所有权
    let arr2 = [10, 20, 30];
    println!("7. into_iter() 获取所有权:");
    for value in arr2.into_iter() {
        println!("  拥有的值: {}", value);
    }

    // 8. 反向遍历
    println!("8. 反向遍历:");
    for value in arr.iter().rev() {
        println!("  反向值: {}", value);
    }

    // 9. 跳过和取前几个
    println!("9. 跳过前2个，取接下来的2个:");
    for value in arr.iter().skip(2).take(2) {
        println!("  跳过取值: {}", value);
    }
}

/// 5. 多维数组
///
/// Rust 支持多维数组，本质上是数组的数组
pub fn multidimensional_arrays() {
    println!("\n=== 多维数组 ===");

    // 1. 二维数组
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    println!("二维数组: {:?}", matrix);

    // 2. 访问二维数组元素
    println!("matrix[0][1] = {}", matrix[0][1]);
    println!("matrix[1][2] = {}", matrix[1][2]);

    // 3. 遍历二维数组
    println!("遍历二维数组:");
    for (i, row) in matrix.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            println!("  matrix[{}][{}] = {}", i, j, value);
        }
    }

    // 4. 三维数组
    let cube: [[[i32; 2]; 2]; 2] = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]];
    println!("三维数组: {:?}", cube);
    println!("cube[1][0][1] = {}", cube[1][0][1]);

    // 5. 初始化多维数组
    let zeros_2d: [[i32; 3]; 4] = [[0; 3]; 4]; // 4x3 的零矩阵
    println!("零矩阵: {:?}", zeros_2d);

    // 6. 可变多维数组
    let mut mut_matrix = [[1, 2], [3, 4], [5, 6]];
    println!("修改前: {:?}", mut_matrix);
    mut_matrix[1][0] = 30;
    mut_matrix[2][1] = 60;
    println!("修改后: {:?}", mut_matrix);

    // 7. 多维数组的大小
    println!("二维数组大小: {} 字节", mem::size_of_val(&matrix));
    println!("三维数组大小: {} 字节", mem::size_of_val(&cube));

    // 8. 方向数组（常用于算法中）
    const DIRECTIONS: [[i32; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];
    println!("方向数组: {:?}", DIRECTIONS);
    for (i, &[dx, dy]) in DIRECTIONS.iter().enumerate() {
        println!("  方向{}: dx={}, dy={}", i, dx, dy);
    }
}

/// 6. 数组切片
///
/// 切片是对数组部分的引用，提供了灵活的数组操作方式
pub fn array_slices() {
    println!("\n=== 数组切片 ===");

    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 1. 基本切片操作
    let full_slice: &[i32] = &arr[..]; // 完整切片
    let partial_slice = &arr[2..5]; // 部分切片 [3, 4, 5]
    let from_start = &arr[..3]; // 从开始到索引3 [1, 2, 3]
    let to_end = &arr[7..]; // 从索引7到结束 [8, 9, 10]
    let inclusive_slice = &arr[1..=3]; // 包含结束索引 [2, 3, 4]

    println!("原数组: {:?}", arr);
    println!("完整切片: {:?}", full_slice);
    println!("部分切片 [2..5]: {:?}", partial_slice);
    println!("从开始 [..3]: {:?}", from_start);
    println!("到结束 [7..]: {:?}", to_end);
    println!("包含结束 [1..=3]: {:?}", inclusive_slice);

    // 2. 切片的类型和大小
    println!("\n切片类型信息:");
    println!("数组类型: {}", std::any::type_name::<[i32; 10]>());
    println!("切片类型: {}", std::any::type_name::<&[i32]>());
    println!("数组大小: {} 字节", mem::size_of_val(&arr));
    println!("切片大小: {} 字节", mem::size_of_val(&partial_slice));

    // 3. 可变切片
    let mut mut_arr = [1, 2, 3, 4, 5];
    println!("\n可变切片操作:");
    println!("修改前: {:?}", mut_arr);
    {
        let mut_slice = &mut mut_arr[1..4];
        for value in mut_slice {
            *value *= 10;
        }
    }
    println!("修改后: {:?}", mut_arr);

    // 4. 切片的方法
    println!("\n切片方法:");
    let slice = &arr[2..8];
    println!("切片: {:?}", slice);
    println!("长度: {}", slice.len());
    println!("是否为空: {}", slice.is_empty());
    println!("第一个元素: {:?}", slice.first());
    println!("最后一个元素: {:?}", slice.last());
    println!("获取索引2: {:?}", slice.get(2));

    // 5. 切片分割
    println!("\n切片分割:");
    let (left, right) = slice.split_at(3);
    println!("原切片: {:?}", slice);
    println!("左半部分: {:?}", left);
    println!("右半部分: {:?}", right);

    // 6. 窗口和块
    println!("\n窗口和块:");
    println!("滑动窗口 (大小2):");
    for window in arr.windows(3) {
        println!("  {:?}", window);
    }

    println!("固定块 (大小3):");
    for chunk in arr.chunks(3) {
        println!("  {:?}", chunk);
    }

    // 7. 切片比较
    let slice1 = &arr[0..3];
    let slice2 = &arr[0..3];
    let slice3 = &arr[1..4];
    println!("\n切片比较:");
    println!("slice1: {:?}", slice1);
    println!("slice2: {:?}", slice2);
    println!("slice3: {:?}", slice3);
    println!("slice1 == slice2: {}", slice1 == slice2);
    println!("slice1 == slice3: {}", slice1 == slice3);
}

/// 7. 数组的常用方法
///
/// Rust 数组提供了丰富的方法用于各种操作
pub fn array_methods() {
    println!("\n=== 数组的常用方法 ===");

    // 1. 查找方法
    let arr = [3, 1, 4, 1, 5, 9, 2, 6, 5];
    println!("原数组: {:?}", arr);

    println!("\n查找方法:");
    println!("包含元素5: {}", arr.contains(&5));
    println!("包含元素10: {}", arr.contains(&10));

    if let Some(pos) = arr.iter().position(|&x| x == 5) {
        println!("元素5的位置: {}", pos);
    }

    if let Some(pos) = arr.iter().rposition(|&x| x == 5) {
        println!("元素5的最后位置: {}", pos);
    }

    // 2. 排序方法
    let mut sort_arr = arr.clone();
    println!("\n排序方法:");
    println!("排序前: {:?}", sort_arr);
    sort_arr.sort();
    println!("升序排序: {:?}", sort_arr);

    let mut sort_arr2 = arr.clone();
    sort_arr2.sort_by(|a, b| b.cmp(a)); // 降序
    println!("降序排序: {:?}", sort_arr2);

    let mut sort_arr3 = arr.clone();
    sort_arr3.sort_unstable(); // 不稳定排序，性能更好
    println!("不稳定排序: {:?}", sort_arr3);

    // 3. 反转方法
    let mut rev_arr = arr.clone();
    println!("\n反转方法:");
    println!("反转前: {:?}", rev_arr);
    rev_arr.reverse();
    println!("反转后: {:?}", rev_arr);

    // 4. 填充方法
    let mut fill_arr = [0; 5];
    println!("\n填充方法:");
    println!("填充前: {:?}", fill_arr);
    fill_arr.fill(42);
    println!("填充42: {:?}", fill_arr);

    fill_arr.fill_with(|| rand::random::<i32>() % 100);
    println!("随机填充: {:?}", fill_arr);

    // 5. 交换方法
    let mut swap_arr = [1, 2, 3, 4, 5];
    println!("\n交换方法:");
    println!("交换前: {:?}", swap_arr);
    swap_arr.swap(0, 4);
    println!("交换索引0和4: {:?}", swap_arr);

    // 6. 旋转方法
    let mut rotate_arr = [1, 2, 3, 4, 5];
    println!("\n旋转方法:");
    println!("旋转前: {:?}", rotate_arr);
    rotate_arr.rotate_left(2);
    println!("左旋转2位: {:?}", rotate_arr);

    let mut rotate_arr2 = [1, 2, 3, 4, 5];
    rotate_arr2.rotate_right(2);
    println!("右旋转2位: {:?}", rotate_arr2);

    // 7. 映射方法
    println!("\n映射方法:");
    let mapped = arr.map(|x| x * 2);
    println!("原数组: {:?}", arr);
    println!("每个元素乘2: {:?}", mapped);

    // 8. 聚合方法
    println!("\n聚合方法:");
    let sum: i32 = arr.iter().sum();
    let product: i32 = arr.iter().product();
    let max = arr.iter().max();
    let min = arr.iter().min();

    println!("数组和: {}", sum);
    println!("数组积: {}", product);
    println!("最大值: {:?}", max);
    println!("最小值: {:?}", min);
}

/// 8. 数组与其他类型的比较
///
/// 比较数组与向量、切片等类型的异同
pub fn array_vs_others() {
    println!("\n=== 数组与其他类型的比较 ===");

    // 1. 数组 vs 向量 (Vec)
    println!("1. 数组 vs 向量:");
    let array = [1, 2, 3, 4, 5];
    let vector = vec![1, 2, 3, 4, 5];

    println!(
        "数组: {:?}, 大小: {} 字节, 存储: 栈",
        array,
        mem::size_of_val(&array)
    );
    println!(
        "向量: {:?}, 大小: {} 字节, 存储: 堆",
        vector,
        mem::size_of_val(&vector)
    );

    // 2. 数组 vs 切片
    println!("\n2. 数组 vs 切片:");
    let slice: &[i32] = &array;
    println!("数组类型: {}", std::any::type_name::<[i32; 5]>());
    println!("切片类型: {}", std::any::type_name::<&[i32]>());
    println!("数组大小: {} 字节", mem::size_of_val(&array));
    println!("切片大小: {} 字节", mem::size_of_val(&slice));

    // 3. 性能比较
    println!("\n3. 性能特点:");
    println!("数组:");
    println!("  - 编译时已知大小");
    println!("  - 栈上分配，访问速度快");
    println!("  - 大小固定，不能动态增长");
    println!("  - 零成本抽象");

    println!("向量:");
    println!("  - 运行时动态大小");
    println!("  - 堆上分配，有额外开销");
    println!("  - 可以动态增长和缩减");
    println!("  - 有容量管理开销");

    println!("切片:");
    println!("  - 对数组或向量的引用");
    println!("  - 运行时已知大小");
    println!("  - 不拥有数据，只是视图");
    println!("  - 可以指向数组或向量的任意部分");

    // 4. 转换示例
    println!("\n4. 类型转换:");
    let arr = [1, 2, 3, 4, 5];

    // 数组到切片
    let slice_from_array: &[i32] = &arr;
    println!("数组到切片: {:?}", slice_from_array);

    // 数组到向量
    let vec_from_array: Vec<i32> = arr.to_vec();
    println!("数组到向量: {:?}", vec_from_array);

    // 向量到切片
    let slice_from_vec: &[i32] = &vec_from_array;
    println!("向量到切片: {:?}", slice_from_vec);

    // 切片到向量
    let vec_from_slice: Vec<i32> = slice_from_array.to_vec();
    println!("切片到向量: {:?}", vec_from_slice);
}

/// 9. 实际应用案例
///
/// 展示数组在实际编程中的应用场景
pub fn array_applications() {
    println!("\n=== 实际应用案例 ===");

    // 1. 矩阵运算
    println!("1. 矩阵运算:");
    let matrix_a = [[1, 2], [3, 4]];
    let matrix_b = [[5, 6], [7, 8]];

    // 矩阵加法
    let mut result = [[0; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            result[i][j] = matrix_a[i][j] + matrix_b[i][j];
        }
    }
    println!("矩阵A: {:?}", matrix_a);
    println!("矩阵B: {:?}", matrix_b);
    println!("A + B: {:?}", result);

    // 2. 查找表
    println!("\n2. 查找表 - 月份天数:");
    const DAYS_IN_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const MONTH_NAMES: [&str; 12] = [
        "一月",
        "二月",
        "三月",
        "四月",
        "五月",
        "六月",
        "七月",
        "八月",
        "九月",
        "十月",
        "十一月",
        "十二月",
    ];

    for (i, (&days, &name)) in DAYS_IN_MONTH.iter().zip(MONTH_NAMES.iter()).enumerate() {
        println!("  {}({})有{}天", name, i + 1, days);
    }

    // 3. 缓冲区
    println!("\n3. 固定大小缓冲区:");
    let mut buffer: [u8; 1024] = [0; 1024];
    let data = b"Hello, Rust Array!";

    // 将数据复制到缓冲区
    let len = data.len().min(buffer.len());
    buffer[..len].copy_from_slice(&data[..len]);

    println!("缓冲区前{}字节: {:?}", len, &buffer[..len]);
    println!("转为字符串: {}", String::from_utf8_lossy(&buffer[..len]));

    // 4. 状态机
    println!("\n4. 状态机 - 简单的有限状态自动机:");
    #[derive(Debug, Clone, Copy)]
    enum State {
        S0,
        S1,
        S2,
    }

    // 状态转移表: [当前状态][输入] -> 下一状态
    const TRANSITION_TABLE: [[State; 2]; 3] = [
        [State::S1, State::S0], // S0状态下，输入0->S1, 输入1->S0
        [State::S2, State::S1], // S1状态下，输入0->S2, 输入1->S1
        [State::S0, State::S2], // S2状态下，输入0->S0, 输入1->S2
    ];

    let mut current_state = State::S0;
    let inputs = [0, 1, 0, 1, 1, 0];

    println!("初始状态: {:?}", current_state);
    for &input in &inputs {
        let state_index = match current_state {
            State::S0 => 0,
            State::S1 => 1,
            State::S2 => 2,
        };
        current_state = TRANSITION_TABLE[state_index][input];
        println!("输入: {}, 新状态: {:?}", input, current_state);
    }

    // 5. 图像处理 - RGB像素
    println!("\n5. 图像处理 - RGB像素数组:");
    type Pixel = [u8; 3]; // RGB
    let mut image: [Pixel; 4] = [
        [255, 0, 0],   // 红色
        [0, 255, 0],   // 绿色
        [0, 0, 255],   // 蓝色
        [255, 255, 0], // 黄色
    ];

    println!("原始像素:");
    for (i, pixel) in image.iter().enumerate() {
        println!("  像素{}: RGB({}, {}, {})", i, pixel[0], pixel[1], pixel[2]);
    }

    // 应用灰度滤镜
    for pixel in &mut image {
        let gray = (pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16) / 3;
        pixel[0] = gray as u8;
        pixel[1] = gray as u8;
        pixel[2] = gray as u8;
    }

    println!("灰度处理后:");
    for (i, pixel) in image.iter().enumerate() {
        println!("  像素{}: RGB({}, {}, {})", i, pixel[0], pixel[1], pixel[2]);
    }

    // 6. 算法 - 冒泡排序
    println!("\n6. 算法实现 - 冒泡排序:");
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    println!("排序前: {:?}", arr);

    let n = arr.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }

    println!("排序后: {:?}", arr);
}

/// 10. 数组的内存布局和安全性
///
/// 深入了解数组的内存特性和安全保证
pub fn array_memory_safety() {
    println!("\n=== 数组的内存布局和安全性 ===");

    // 1. 内存布局
    println!("1. 内存布局:");
    let arr = [1i32, 2, 3, 4, 5];

    println!("数组地址: {:p}", &arr);
    for (i, element) in arr.iter().enumerate() {
        println!("  元素{}地址: {:p}, 值: {}", i, element, element);
    }

    // 验证连续性
    let ptr0 = &arr[0] as *const i32;
    let ptr1 = &arr[1] as *const i32;
    let diff = unsafe { ptr1.offset_from(ptr0) };
    println!("相邻元素地址差: {} (应该是1个i32大小)", diff);

    // 2. 边界检查
    println!("\n2. 边界检查:");
    println!("Rust 在运行时进行边界检查，防止缓冲区溢出");

    // 安全的访问
    match arr.get(10) {
        Some(value) => println!("索引10的值: {}", value),
        None => println!("索引10越界，安全返回None"),
    }

    // 3. 所有权和借用
    println!("\n3. 所有权和借用:");
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = arr1; // 数组实现了Copy trait，这是复制而不是移动

    println!("arr1: {:?}", arr1); // 仍然可以使用
    println!("arr2: {:?}", arr2);

    // 借用
    let slice = &arr1[1..4];
    println!("借用的切片: {:?}", slice);
    println!("原数组仍可用: {:?}", arr1);

    // 4. 生命周期
    println!("\n4. 生命周期:");
    {
        let temp_arr = [10, 20, 30];
        let temp_slice = &temp_arr[..];
        println!("临时数组和切片: {:?}", temp_slice);
    } // temp_arr 和 temp_slice 在这里被销毁

    // 5. 线程安全
    println!("\n5. 线程安全:");
    println!("数组本身是线程安全的，因为它们是值类型");
    println!("多个线程可以安全地拥有数组的副本");

    // 6. 零成本抽象
    println!("\n6. 零成本抽象:");
    println!("数组操作在编译时优化，运行时性能接近C语言");

    // 性能测试示例
    let large_arr = [42; 10000];
    let start = std::time::Instant::now();
    let sum: i32 = large_arr.iter().sum();
    let duration = start.elapsed();

    println!("10000个元素求和: {}, 耗时: {:?}", sum, duration);
}

/// 运行所有数组示例
pub fn run_all_examples() {
    println!("🦀 Rust 数组全面教程");
    println!("{}", "=".repeat(50));

    array_basics();
    array_initialization();
    array_access();
    array_iteration();
    multidimensional_arrays();
    array_slices();
    array_methods();
    array_vs_others();
    array_applications();
    array_memory_safety();

    println!("{}", "=".repeat(50));
    println!("✅ 所有数组示例运行完成！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_basics() {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(arr.len(), 5);
        assert_eq!(arr[0], 1);
        assert_eq!(arr[4], 5);
    }

    #[test]
    fn test_array_initialization() {
        let arr1 = [0; 10];
        assert_eq!(arr1.len(), 10);
        assert!(arr1.iter().all(|&x| x == 0));

        let arr2: [i32; 3] = [1, 2, 3];
        assert_eq!(arr2, [1, 2, 3]);
    }

    #[test]
    fn test_array_access() {
        let arr = [10, 20, 30, 40, 50];
        assert_eq!(arr.get(2), Some(&30));
        assert_eq!(arr.get(10), None);
        assert_eq!(arr.first(), Some(&10));
        assert_eq!(arr.last(), Some(&50));
    }

    #[test]
    fn test_multidimensional_arrays() {
        let matrix: [[i32; 2]; 3] = [[1, 2], [3, 4], [5, 6]];
        assert_eq!(matrix[0][1], 2);
        assert_eq!(matrix[2][0], 5);
        assert_eq!(matrix.len(), 3);
        assert_eq!(matrix[0].len(), 2);
    }

    #[test]
    fn test_array_slices() {
        let arr = [1, 2, 3, 4, 5];
        let slice = &arr[1..4];
        assert_eq!(slice, &[2, 3, 4]);
        assert_eq!(slice.len(), 3);

        let (left, right) = slice.split_at(2);
        assert_eq!(left, &[2, 3]);
        assert_eq!(right, &[4]);
    }

    #[test]
    fn test_array_methods() {
        let mut arr = [3, 1, 4, 1, 5];
        assert!(arr.contains(&4));
        assert!(!arr.contains(&10));

        arr.sort();
        assert_eq!(arr, [1, 1, 3, 4, 5]);

        arr.reverse();
        assert_eq!(arr, [5, 4, 3, 1, 1]);
    }

    #[test]
    fn test_array_iteration() {
        let arr = [1, 2, 3, 4, 5];
        let sum: i32 = arr.iter().sum();
        assert_eq!(sum, 15);

        let doubled: Vec<i32> = arr.iter().map(|&x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_array_windows_chunks() {
        let arr = [1, 2, 3, 4, 5];

        let windows: Vec<&[i32]> = arr.windows(3).collect();
        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], &[1, 2, 3]);
        assert_eq!(windows[2], &[3, 4, 5]);

        let chunks: Vec<&[i32]> = arr.chunks(2).collect();
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], &[1, 2]);
        assert_eq!(chunks[2], &[5]);
    }
}
