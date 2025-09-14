//! Rust 数组练习题
//!
//! 本模块包含了一系列递进式的数组练习题，帮助巩固数组知识
//! 练习涵盖：基础操作、多维数组、切片、算法实现、实际应用等

/// 练习1：数组基础操作
///
/// 实现一个函数，对数组进行基本操作
pub fn exercise1_basic_operations() {
    println!("=== 练习1：数组基础操作 ===");

    // TODO: 创建一个包含1到10的数组
    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // TODO: 计算数组的和
    let sum: i32 = numbers.iter().sum();
    println!("数组和: {}", sum);

    // TODO: 找到最大值和最小值
    let max = numbers.iter().max().unwrap();
    let min = numbers.iter().min().unwrap();
    println!("最大值: {}, 最小值: {}", max, min);

    // TODO: 计算平均值
    let average = sum as f64 / numbers.len() as f64;
    println!("平均值: {:.2}", average);

    // TODO: 统计偶数个数
    let even_count = numbers.iter().filter(|&&x| x % 2 == 0).count();
    println!("偶数个数: {}", even_count);

    // TODO: 创建一个新数组，包含原数组每个元素的平方
    let squares = numbers.map(|x| x * x);
    println!("平方数组: {:?}", squares);

    println!("✅ 练习1完成\n");
}

/// 练习2：数组搜索和排序
///
/// 实现各种搜索和排序算法
pub fn exercise2_search_and_sort() {
    println!("=== 练习2：数组搜索和排序 ===");

    let arr = [64, 34, 25, 12, 22, 11, 90, 5];
    println!("原数组: {:?}", arr);

    // TODO: 实现线性搜索
    fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
        for (index, &value) in arr.iter().enumerate() {
            if value == target {
                return Some(index);
            }
        }
        None
    }

    let target = 25;
    match linear_search(&arr, target) {
        Some(index) => println!("线性搜索: 找到 {} 在索引 {}", target, index),
        None => println!("线性搜索: 未找到 {}", target),
    }

    // TODO: 实现选择排序
    fn selection_sort(arr: &mut [i32]) {
        let len = arr.len();
        for i in 0..len {
            let mut min_idx = i;
            for j in (i + 1)..len {
                if arr[j] < arr[min_idx] {
                    min_idx = j;
                }
            }
            arr.swap(i, min_idx);
        }
    }

    let mut arr_copy = arr;
    selection_sort(&mut arr_copy);
    println!("选择排序后: {:?}", arr_copy);

    // TODO: 实现插入排序
    fn insertion_sort(arr: &mut [i32]) {
        for i in 1..arr.len() {
            let key = arr[i];
            let mut j = i;
            while j > 0 && arr[j - 1] > key {
                arr[j] = arr[j - 1];
                j -= 1;
            }
            arr[j] = key;
        }
    }

    let mut arr_copy2 = arr;
    insertion_sort(&mut arr_copy2);
    println!("插入排序后: {:?}", arr_copy2);

    // TODO: 实现二分搜索（需要先排序）
    fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = left + (right - left) / 2;
            match arr[mid].cmp(&target) {
                std::cmp::Ordering::Equal => return Some(mid),
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Greater => right = mid,
            }
        }
        None
    }

    arr_copy2.sort(); // 确保数组已排序
    match binary_search(&arr_copy2, target) {
        Some(index) => println!("二分搜索: 找到 {} 在索引 {}", target, index),
        None => println!("二分搜索: 未找到 {}", target),
    }

    println!("✅ 练习2完成\n");
}

/// 练习3：多维数组操作
///
/// 实现矩阵运算和操作
pub fn exercise3_multidimensional_arrays() {
    println!("=== 练习3：多维数组操作 ===");

    // TODO: 创建两个3x3矩阵
    let matrix_a = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    let matrix_b = [[9, 8, 7], [6, 5, 4], [3, 2, 1]];

    println!("矩阵A:");
    print_matrix(&matrix_a);
    println!("矩阵B:");
    print_matrix(&matrix_b);

    // TODO: 实现矩阵加法
    fn matrix_add(a: &[[i32; 3]; 3], b: &[[i32; 3]; 3]) -> [[i32; 3]; 3] {
        let mut result = [[0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                result[i][j] = a[i][j] + b[i][j];
            }
        }
        result
    }

    let sum_matrix = matrix_add(&matrix_a, &matrix_b);
    println!("矩阵A + B:");
    print_matrix(&sum_matrix);

    // TODO: 实现矩阵转置
    fn matrix_transpose(matrix: &[[i32; 3]; 3]) -> [[i32; 3]; 3] {
        let mut result = [[0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                result[j][i] = matrix[i][j];
            }
        }
        result
    }

    let transposed = matrix_transpose(&matrix_a);
    println!("矩阵A的转置:");
    print_matrix(&transposed);

    // TODO: 实现矩阵乘法
    fn matrix_multiply(a: &[[i32; 3]; 3], b: &[[i32; 3]; 3]) -> [[i32; 3]; 3] {
        let mut result = [[0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    result[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        result
    }

    let product_matrix = matrix_multiply(&matrix_a, &matrix_b);
    println!("矩阵A × B:");
    print_matrix(&product_matrix);

    // TODO: 计算矩阵的迹（对角线元素之和）
    fn matrix_trace(matrix: &[[i32; 3]; 3]) -> i32 {
        let mut trace = 0;
        for i in 0..3 {
            trace += matrix[i][i];
        }
        trace
    }

    let trace_a = matrix_trace(&matrix_a);
    println!("矩阵A的迹: {}", trace_a);

    println!("✅ 练习3完成\n");
}

// 辅助函数：打印3x3矩阵
fn print_matrix(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        println!("  {:?}", row);
    }
}

/// 练习4：数组切片高级操作
///
/// 实现各种切片操作和算法
pub fn exercise4_advanced_slicing() {
    println!("=== 练习4：数组切片高级操作 ===");

    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    println!("原数据: {:?}", data);

    // TODO: 实现滑动窗口最大值
    fn sliding_window_max(arr: &[i32], window_size: usize) -> Vec<i32> {
        let mut result = Vec::new();
        for window in arr.windows(window_size) {
            if let Some(&max) = window.iter().max() {
                result.push(max);
            }
        }
        result
    }

    let window_max = sliding_window_max(&data, 3);
    println!("滑动窗口(大小3)最大值: {:?}", window_max);

    // TODO: 实现数组分块处理
    fn process_chunks(arr: &[i32], chunk_size: usize) -> Vec<i32> {
        arr.chunks(chunk_size)
            .map(|chunk| chunk.iter().sum())
            .collect()
    }

    let chunk_sums = process_chunks(&data, 4);
    println!("每4个元素的和: {:?}", chunk_sums);

    // TODO: 实现数组旋转
    fn rotate_array(arr: &mut [i32], k: usize) {
        let len = arr.len();
        let k = k % len; // 处理k大于数组长度的情况
        arr.reverse();
        arr[..k].reverse();
        arr[k..].reverse();
    }

    let mut rotated = data;
    rotate_array(&mut rotated, 3);
    println!("右旋转3位: {:?}", rotated);

    // TODO: 实现子数组最大和（Kadane算法）
    fn max_subarray_sum(arr: &[i32]) -> i32 {
        let mut max_sum = arr[0];
        let mut current_sum = arr[0];

        for &num in &arr[1..] {
            current_sum = num.max(current_sum + num);
            max_sum = max_sum.max(current_sum);
        }

        max_sum
    }

    let test_array = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = max_subarray_sum(&test_array);
    println!("数组 {:?} 的最大子数组和: {}", test_array, max_sum);

    // TODO: 实现数组去重（保持顺序）
    fn remove_duplicates(arr: &[i32]) -> Vec<i32> {
        let mut result = Vec::new();
        for &item in arr {
            if !result.contains(&item) {
                result.push(item);
            }
        }
        result
    }

    let with_duplicates = [1, 2, 2, 3, 4, 4, 5, 1, 6];
    let unique = remove_duplicates(&with_duplicates);
    println!("去重前: {:?}", with_duplicates);
    println!("去重后: {:?}", unique);

    println!("✅ 练习4完成\n");
}

/// 练习5：数组算法实现
///
/// 实现经典算法
pub fn exercise5_algorithms() {
    println!("=== 练习5：数组算法实现 ===");

    // TODO: 实现快速排序
    fn quick_sort(arr: &mut [i32]) {
        if arr.len() <= 1 {
            return;
        }

        let pivot_index = partition(arr);
        let (left, right) = arr.split_at_mut(pivot_index);
        quick_sort(left);
        quick_sort(&mut right[1..]);
    }

    fn partition(arr: &mut [i32]) -> usize {
        let len = arr.len();
        let pivot_index = len - 1;
        let mut i = 0;

        for j in 0..pivot_index {
            if arr[j] <= arr[pivot_index] {
                arr.swap(i, j);
                i += 1;
            }
        }

        arr.swap(i, pivot_index);
        i
    }

    let mut quick_sort_data = [64, 34, 25, 12, 22, 11, 90];
    println!("快速排序前: {:?}", quick_sort_data);
    quick_sort(&mut quick_sort_data);
    println!("快速排序后: {:?}", quick_sort_data);

    // TODO: 实现归并排序
    fn merge_sort(arr: &mut [i32]) {
        if arr.len() <= 1 {
            return;
        }

        let mid = arr.len() / 2;
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);

        let mut temp = arr.to_vec();
        merge(&arr[..mid], &arr[mid..], &mut temp);
        arr.copy_from_slice(&temp);
    }

    fn merge(left: &[i32], right: &[i32], result: &mut [i32]) {
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;

        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                result[k] = left[i];
                i += 1;
            } else {
                result[k] = right[j];
                j += 1;
            }
            k += 1;
        }

        while i < left.len() {
            result[k] = left[i];
            i += 1;
            k += 1;
        }

        while j < right.len() {
            result[k] = right[j];
            j += 1;
            k += 1;
        }
    }

    let mut merge_sort_data = [64, 34, 25, 12, 22, 11, 90];
    println!("归并排序前: {:?}", merge_sort_data);
    merge_sort(&mut merge_sort_data);
    println!("归并排序后: {:?}", merge_sort_data);

    // TODO: 实现堆排序
    fn heap_sort(arr: &mut [i32]) {
        let len = arr.len();

        // 构建最大堆
        for i in (0..len / 2).rev() {
            heapify(arr, len, i);
        }

        // 逐个提取元素
        for i in (1..len).rev() {
            arr.swap(0, i);
            heapify(arr, i, 0);
        }
    }

    fn heapify(arr: &mut [i32], heap_size: usize, root: usize) {
        let mut largest = root;
        let left = 2 * root + 1;
        let right = 2 * root + 2;

        if left < heap_size && arr[left] > arr[largest] {
            largest = left;
        }

        if right < heap_size && arr[right] > arr[largest] {
            largest = right;
        }

        if largest != root {
            arr.swap(root, largest);
            heapify(arr, heap_size, largest);
        }
    }

    let mut heap_sort_data = [64, 34, 25, 12, 22, 11, 90];
    println!("堆排序前: {:?}", heap_sort_data);
    heap_sort(&mut heap_sort_data);
    println!("堆排序后: {:?}", heap_sort_data);

    // TODO: 实现计数排序（适用于小范围整数）
    fn counting_sort(arr: &mut [usize], max_val: usize) {
        let mut count = vec![0; max_val + 1];

        // 计数
        for &num in arr.iter() {
            count[num] += 1;
        }

        // 重建数组
        let mut index = 0;
        for (num, &cnt) in count.iter().enumerate() {
            for _ in 0..cnt {
                arr[index] = num;
                index += 1;
            }
        }
    }

    let mut counting_sort_data = [4, 2, 2, 8, 3, 3, 1];
    println!("计数排序前: {:?}", counting_sort_data);
    counting_sort(&mut counting_sort_data, 8);
    println!("计数排序后: {:?}", counting_sort_data);

    println!("✅ 练习5完成\n");
}

/// 练习6：实际应用场景
///
/// 模拟真实世界的数组应用
pub fn exercise6_real_world_applications() {
    println!("=== 练习6：实际应用场景 ===");

    // TODO: 学生成绩管理系统
    println!("1. 学生成绩管理系统:");

    #[derive(Debug)]
    struct Student {
        name: String,
        scores: [f64; 5], // 5门课程成绩
    }

    impl Student {
        fn new(name: String, scores: [f64; 5]) -> Self {
            Student { name, scores }
        }

        fn average(&self) -> f64 {
            self.scores.iter().sum::<f64>() / self.scores.len() as f64
        }

        fn highest_score(&self) -> f64 {
            *self
                .scores
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
        }

        fn lowest_score(&self) -> f64 {
            *self
                .scores
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
        }
    }

    let students = [
        Student::new("张三".to_string(), [85.5, 92.0, 78.5, 88.0, 91.5]),
        Student::new("李四".to_string(), [76.0, 84.5, 89.0, 82.5, 87.0]),
        Student::new("王五".to_string(), [94.0, 88.5, 92.5, 90.0, 89.5]),
    ];

    for student in &students {
        println!(
            "  {}: 平均分 {:.2}, 最高分 {:.1}, 最低分 {:.1}",
            student.name,
            student.average(),
            student.highest_score(),
            student.lowest_score()
        );
    }

    // 计算班级平均分
    let class_average: f64 =
        students.iter().map(|s| s.average()).sum::<f64>() / students.len() as f64;
    println!("  班级平均分: {:.2}", class_average);

    // TODO: 图像处理 - 简单的图像滤镜
    println!("\n2. 图像处理 - 简单滤镜:");

    type Image = [[u8; 5]; 5]; // 5x5 灰度图像

    let mut image: Image = [
        [100, 120, 110, 130, 140],
        [110, 125, 115, 135, 145],
        [105, 115, 120, 125, 135],
        [115, 130, 125, 140, 150],
        [120, 135, 130, 145, 155],
    ];

    println!("  原始图像:");
    print_image(&image);

    // 应用均值滤镜（3x3）
    fn apply_mean_filter(image: &mut Image) {
        let original = *image;
        for i in 1..4 {
            for j in 1..4 {
                let mut sum = 0u32;
                for di in -1i32..=1 {
                    for dj in -1i32..=1 {
                        let ni = (i as i32 + di) as usize;
                        let nj = (j as i32 + dj) as usize;
                        sum += original[ni][nj] as u32;
                    }
                }
                image[i][j] = (sum / 9) as u8;
            }
        }
    }

    apply_mean_filter(&mut image);
    println!("  均值滤镜后:");
    print_image(&image);

    // TODO: 时间序列数据分析
    println!("\n3. 时间序列数据分析:");

    let temperature_data = [
        22.5, 23.1, 24.0, 25.2, 26.8, 28.1, 29.5, 30.2, 29.8, 28.5, 27.2, 25.8, 24.1, 23.5, 22.9,
        22.3,
    ]; // 16小时的温度数据

    // 计算移动平均
    fn moving_average(data: &[f64], window: usize) -> Vec<f64> {
        data.windows(window)
            .map(|w| w.iter().sum::<f64>() / window as f64)
            .collect()
    }

    let ma_3 = moving_average(&temperature_data, 3);
    println!("  原始温度数据: {:?}", temperature_data);
    println!("  3小时移动平均: {:?}", ma_3);

    // 检测温度异常（偏离平均值超过2个标准差）
    let mean: f64 = temperature_data.iter().sum::<f64>() / temperature_data.len() as f64;
    let variance: f64 = temperature_data
        .iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f64>()
        / temperature_data.len() as f64;
    let std_dev = variance.sqrt();

    println!("  平均温度: {:.2}°C, 标准差: {:.2}°C", mean, std_dev);

    for (i, &temp) in temperature_data.iter().enumerate() {
        if (temp - mean).abs() > 2.0 * std_dev {
            println!("  异常温度检测: 第{}小时温度{:.1}°C异常", i, temp);
        }
    }

    // TODO: 库存管理系统
    println!("\n4. 库存管理系统:");

    #[derive(Debug, Clone)]
    struct Product {
        id: u32,
        name: String,
        stock: u32,
        min_stock: u32,
    }

    let mut inventory = [
        Product {
            id: 1,
            name: "笔记本电脑".to_string(),
            stock: 15,
            min_stock: 5,
        },
        Product {
            id: 2,
            name: "鼠标".to_string(),
            stock: 3,
            min_stock: 10,
        },
        Product {
            id: 3,
            name: "键盘".to_string(),
            stock: 25,
            min_stock: 8,
        },
        Product {
            id: 4,
            name: "显示器".to_string(),
            stock: 7,
            min_stock: 3,
        },
        Product {
            id: 5,
            name: "耳机".to_string(),
            stock: 2,
            min_stock: 5,
        },
    ];

    // 检查低库存商品
    println!("  库存警告:");
    for product in &inventory {
        if product.stock < product.min_stock {
            println!(
                "    ⚠️  {} (ID: {}) 库存不足: {} < {}",
                product.name, product.id, product.stock, product.min_stock
            );
        }
    }

    // 模拟销售
    fn sell_product(inventory: &mut [Product], product_id: u32, quantity: u32) -> bool {
        for product in inventory.iter_mut() {
            if product.id == product_id {
                if product.stock >= quantity {
                    product.stock -= quantity;
                    println!(
                        "    ✅ 销售成功: {} x{}, 剩余库存: {}",
                        product.name, quantity, product.stock
                    );
                    return true;
                } else {
                    println!(
                        "    ❌ 库存不足: {} 需要{}, 仅有{}",
                        product.name, quantity, product.stock
                    );
                    return false;
                }
            }
        }
        println!("    ❌ 商品不存在: ID {}", product_id);
        false
    }

    println!("  销售记录:");
    sell_product(&mut inventory, 1, 2); // 销售2台笔记本
    sell_product(&mut inventory, 2, 5); // 尝试销售5个鼠标（库存不足）
    sell_product(&mut inventory, 3, 3); // 销售3个键盘

    println!("✅ 练习6完成\n");
}

// 辅助函数：打印5x5图像
fn print_image(image: &[[u8; 5]; 5]) {
    for row in image {
        print!("    ");
        for &pixel in row {
            print!("{:3} ", pixel);
        }
        println!();
    }
}

/// 练习7：性能优化和内存管理
///
/// 探索数组的性能特性
pub fn exercise7_performance_optimization() {
    println!("=== 练习7：性能优化和内存管理 ===");

    // TODO: 缓存友好的数组访问
    println!("1. 缓存友好的数组访问:");

    const SIZE: usize = 1000;
    let mut matrix: [[i32; SIZE]; SIZE] = [[0; SIZE]; SIZE];

    // 初始化矩阵
    for i in 0..SIZE {
        for j in 0..SIZE {
            matrix[i][j] = (i * SIZE + j) as i32;
        }
    }

    // 行优先访问（缓存友好）
    let start = std::time::Instant::now();
    let mut sum_row = 0i64;
    for i in 0..SIZE {
        for j in 0..SIZE {
            sum_row += matrix[i][j] as i64;
        }
    }
    let row_time = start.elapsed();

    // 列优先访问（缓存不友好）
    let start = std::time::Instant::now();
    let mut sum_col = 0i64;
    for j in 0..SIZE {
        for i in 0..SIZE {
            sum_col += matrix[i][j] as i64;
        }
    }
    let col_time = start.elapsed();

    println!("  行优先访问: 和={}, 时间={:?}", sum_row, row_time);
    println!("  列优先访问: 和={}, 时间={:?}", sum_col, col_time);
    println!(
        "  性能差异: {:.2}x",
        col_time.as_nanos() as f64 / row_time.as_nanos() as f64
    );

    // TODO: 内存对齐和大小
    println!("\n2. 内存对齐和大小:");

    #[repr(C)]
    #[derive(Copy, Clone)]
    struct AlignedStruct {
        a: u8,
        b: u32,
        c: u16,
    }

    let arr_u8: [u8; 100] = [0; 100];
    let arr_u32: [u32; 100] = [0; 100];
    let arr_struct: [AlignedStruct; 10] = [AlignedStruct { a: 0, b: 0, c: 0 }; 10];

    println!("  u8数组[100]: {} 字节", std::mem::size_of_val(&arr_u8));
    println!("  u32数组[100]: {} 字节", std::mem::size_of_val(&arr_u32));
    println!(
        "  结构体数组[10]: {} 字节",
        std::mem::size_of_val(&arr_struct)
    );
    println!(
        "  单个结构体: {} 字节",
        std::mem::size_of::<AlignedStruct>()
    );

    // TODO: SIMD优化示例（概念性）
    println!("\n3. 向量化操作:");

    let arr1 = [1.0f32; 1000];
    let arr2 = [2.0f32; 1000];
    let mut result = [0.0f32; 1000];

    // 标量操作
    let start = std::time::Instant::now();
    for i in 0..1000 {
        result[i] = arr1[i] + arr2[i];
    }
    let scalar_time = start.elapsed();

    // 使用迭代器（编译器可能优化为SIMD）
    let start = std::time::Instant::now();
    for ((a, b), r) in arr1.iter().zip(arr2.iter()).zip(result.iter_mut()) {
        *r = a + b;
    }
    let iterator_time = start.elapsed();

    println!("  标量操作时间: {:?}", scalar_time);
    println!("  迭代器操作时间: {:?}", iterator_time);

    println!("✅ 练习7完成\n");
}

/// 练习8：综合应用 - 数据分析工具
///
/// 实现一个简单的数据分析工具
pub fn exercise8_data_analysis_tool() {
    println!("=== 练习8：综合应用 - 数据分析工具 ===");

    // 模拟一年的销售数据（12个月）
    let monthly_sales = [
        120.5, 135.2, 98.7, 156.3, 189.1, 201.8, 178.9, 165.4, 143.2, 167.8, 198.5, 210.3,
    ];

    let months = [
        "1月", "2月", "3月", "4月", "5月", "6月", "7月", "8月", "9月", "10月", "11月", "12月",
    ];

    println!("📊 年度销售数据分析报告");
    println!("{}", "-".repeat(40));

    // 1. 基础统计
    let total_sales: f64 = monthly_sales.iter().sum();
    let average_sales = total_sales / monthly_sales.len() as f64;
    let max_sales = monthly_sales
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let min_sales = monthly_sales
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    println!("📈 基础统计:");
    println!("  总销售额: {:.1}万元", total_sales);
    println!("  平均月销售额: {:.1}万元", average_sales);
    println!("  最高月销售额: {:.1}万元", max_sales);
    println!("  最低月销售额: {:.1}万元", min_sales);

    // 2. 找出最佳和最差月份
    let best_month_idx = monthly_sales.iter().position(|&x| x == *max_sales).unwrap();
    let worst_month_idx = monthly_sales.iter().position(|&x| x == *min_sales).unwrap();

    println!("\n🏆 表现分析:");
    println!(
        "  最佳月份: {} ({:.1}万元)",
        months[best_month_idx], max_sales
    );
    println!(
        "  最差月份: {} ({:.1}万元)",
        months[worst_month_idx], min_sales
    );

    // 3. 季度分析
    let quarters = [
        monthly_sales[0..3].iter().sum::<f64>(),  // Q1
        monthly_sales[3..6].iter().sum::<f64>(),  // Q2
        monthly_sales[6..9].iter().sum::<f64>(),  // Q3
        monthly_sales[9..12].iter().sum::<f64>(), // Q4
    ];

    println!("\n📅 季度分析:");
    for (i, &quarter_sales) in quarters.iter().enumerate() {
        println!(
            "  Q{}: {:.1}万元 (占比 {:.1}%)",
            i + 1,
            quarter_sales,
            quarter_sales / total_sales * 100.0
        );
    }

    // 4. 增长率分析
    println!("\n📊 月度增长率:");
    for i in 1..monthly_sales.len() {
        let growth_rate = (monthly_sales[i] - monthly_sales[i - 1]) / monthly_sales[i - 1] * 100.0;
        let trend = if growth_rate > 0.0 { "📈" } else { "📉" };
        println!(
            "  {} vs {}: {:.1}% {}",
            months[i],
            months[i - 1],
            growth_rate,
            trend
        );
    }

    // 5. 移动平均
    println!("\n📈 3个月移动平均:");
    for i in 2..monthly_sales.len() {
        let ma3 = (monthly_sales[i - 2] + monthly_sales[i - 1] + monthly_sales[i]) / 3.0;
        println!("  {}: {:.1}万元", months[i], ma3);
    }

    // 6. 异常检测
    let variance: f64 = monthly_sales
        .iter()
        .map(|&x| (x - average_sales).powi(2))
        .sum::<f64>()
        / monthly_sales.len() as f64;
    let std_dev = variance.sqrt();

    println!("\n⚠️  异常月份检测 (偏离平均值超过1个标准差):");
    for (i, &sales) in monthly_sales.iter().enumerate() {
        let deviation = (sales - average_sales).abs();
        if deviation > std_dev {
            let status = if sales > average_sales {
                "异常高"
            } else {
                "异常低"
            };
            println!("  {}: {:.1}万元 ({})", months[i], sales, status);
        }
    }

    // 7. 简单预测（线性趋势）
    let n = monthly_sales.len() as f64;
    let sum_x: f64 = (0..monthly_sales.len()).map(|i| i as f64).sum();
    let sum_y: f64 = monthly_sales.iter().sum();
    let sum_xy: f64 = monthly_sales
        .iter()
        .enumerate()
        .map(|(i, &y)| i as f64 * y)
        .sum();
    let sum_x2: f64 = (0..monthly_sales.len()).map(|i| (i as f64).powi(2)).sum();

    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x.powi(2));
    let intercept = (sum_y - slope * sum_x) / n;

    let next_month_prediction = slope * monthly_sales.len() as f64 + intercept;

    println!("\n🔮 下月销售预测:");
    println!("  预测销售额: {:.1}万元", next_month_prediction);
    println!(
        "  趋势斜率: {:.2} ({})",
        slope,
        if slope > 0.0 {
            "上升趋势"
        } else {
            "下降趋势"
        }
    );

    // 8. 可视化（简单的ASCII图表）
    println!("\n📊 销售趋势图 (ASCII):");
    let max_val = *max_sales;
    let scale = 50.0 / max_val; // 缩放到50个字符宽度

    for (i, &sales) in monthly_sales.iter().enumerate() {
        let bar_length = (sales * scale) as usize;
        let bar = "█".repeat(bar_length);
        println!("  {:>3}: {:>5.1} {}", months[i], sales, bar);
    }

    println!("\n✅ 数据分析完成！");
    println!("✅ 练习8完成\n");
}

/// 运行所有练习
pub fn run_all_exercises() {
    println!("🦀 Rust 数组练习题集");
    println!("{}", "=".repeat(50));

    exercise1_basic_operations();
    exercise2_search_and_sort();
    exercise3_multidimensional_arrays();
    exercise4_advanced_slicing();
    exercise5_algorithms();
    exercise6_real_world_applications();
    exercise7_performance_optimization();
    exercise8_data_analysis_tool();

    println!("{}", "=".repeat(50));
    println!("🎉 所有练习完成！恭喜你掌握了 Rust 数组的核心知识！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
            for (index, &value) in arr.iter().enumerate() {
                if value == target {
                    return Some(index);
                }
            }
            None
        }

        let arr = [1, 3, 5, 7, 9];
        assert_eq!(linear_search(&arr, 5), Some(2));
        assert_eq!(linear_search(&arr, 6), None);
    }

    #[test]
    fn test_matrix_operations() {
        let matrix_a = [[1, 2], [3, 4]];
        let matrix_b = [[5, 6], [7, 8]];

        // 矩阵加法
        let mut result = [[0; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                result[i][j] = matrix_a[i][j] + matrix_b[i][j];
            }
        }

        assert_eq!(result, [[6, 8], [10, 12]]);
    }

    #[test]
    fn test_sliding_window_max() {
        fn sliding_window_max(arr: &[i32], window_size: usize) -> Vec<i32> {
            let mut result = Vec::new();
            for window in arr.windows(window_size) {
                if let Some(&max) = window.iter().max() {
                    result.push(max);
                }
            }
            result
        }

        let data = [1, 3, 2, 5, 4];
        let result = sliding_window_max(&data, 3);
        assert_eq!(result, vec![3, 5, 5]);
    }

    #[test]
    fn test_quick_sort() {
        fn quick_sort(arr: &mut [i32]) {
            if arr.len() <= 1 {
                return;
            }

            let pivot_index = partition(arr);
            let (left, right) = arr.split_at_mut(pivot_index);
            quick_sort(left);
            quick_sort(&mut right[1..]);
        }

        fn partition(arr: &mut [i32]) -> usize {
            let len = arr.len();
            let pivot_index = len - 1;
            let mut i = 0;

            for j in 0..pivot_index {
                if arr[j] <= arr[pivot_index] {
                    arr.swap(i, j);
                    i += 1;
                }
            }

            arr.swap(i, pivot_index);
            i
        }

        let mut data = [64, 34, 25, 12, 22, 11, 90];
        quick_sort(&mut data);
        assert_eq!(data, [11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_max_subarray_sum() {
        fn max_subarray_sum(arr: &[i32]) -> i32 {
            let mut max_sum = arr[0];
            let mut current_sum = arr[0];

            for &num in &arr[1..] {
                current_sum = num.max(current_sum + num);
                max_sum = max_sum.max(current_sum);
            }

            max_sum
        }

        let test_array = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(max_subarray_sum(&test_array), 6); // [4, -1, 2, 1]
    }

    #[test]
    fn test_remove_duplicates() {
        fn remove_duplicates(arr: &[i32]) -> Vec<i32> {
            let mut result = Vec::new();
            for &item in arr {
                if !result.contains(&item) {
                    result.push(item);
                }
            }
            result
        }

        let with_duplicates = [1, 2, 2, 3, 4, 4, 5, 1];
        let unique = remove_duplicates(&with_duplicates);
        assert_eq!(unique, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_moving_average() {
        fn moving_average(data: &[f64], window: usize) -> Vec<f64> {
            data.windows(window)
                .map(|w| w.iter().sum::<f64>() / window as f64)
                .collect()
        }

        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        let ma = moving_average(&data, 3);
        assert_eq!(ma, vec![2.0, 3.0, 4.0]);
    }
}
