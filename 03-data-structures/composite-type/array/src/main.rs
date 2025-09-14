//! Rust 数组学习项目主程序
//!
//! 本项目基于 Rust 语言圣经的数组章节，提供全面的数组学习资源
//! 包括详细的教程、实践练习和实际应用案例

mod array_comprehensive;
mod exercises;

use array_comprehensive::*;
use exercises::*;

fn main() {
    print_welcome();

    println!("\n🎯 选择学习模式:");
    println!("1. 📚 完整教程模式 - 系统学习所有数组知识点");
    println!("2. 💪 练习模式 - 通过实践巩固数组技能");
    println!("3. 🚀 综合模式 - 教程 + 练习完整体验\n");

    // 为了演示，我们运行综合模式
    run_comprehensive_mode();

    print_summary();
}

/// 打印欢迎信息
fn print_welcome() {
    println!("{}", "=".repeat(60));
    println!("🦀 Rust 数组 (Array) 全面学习项目");
    println!("{}", "=".repeat(60));
    println!("📖 基于 Rust 语言圣经: https://course.rs/basic/compound-type/array.html");
    println!("🎯 目标: 全面掌握 Rust 数组的核心概念和实际应用");
    println!("{}", "=".repeat(60));
}

/// 运行综合学习模式
fn run_comprehensive_mode() {
    println!("🚀 启动综合学习模式\n");

    // 第一部分：系统教程
    println!("📚 第一部分：系统教程");
    println!("{}", "-".repeat(40));
    run_all_examples();

    println!("\n{}", "=".repeat(60));

    // 第二部分：实践练习
    println!("💪 第二部分：实践练习");
    println!("{}", "-".repeat(40));
    run_all_exercises();
}

/// 打印学习总结
fn print_summary() {
    println!("\n{}", "=".repeat(60));
    println!("📋 Rust 数组学习总结");
    println!("{}", "=".repeat(60));

    println!("\n🎯 核心知识点:");
    println!("  ✅ 数组基础概念 - 固定长度、同类型、栈分配");
    println!("  ✅ 数组声明和初始化 - 多种初始化方式");
    println!("  ✅ 数组访问和边界检查 - 安全访问机制");
    println!("  ✅ 数组遍历 - 多种遍历模式");
    println!("  ✅ 多维数组 - 矩阵和高维数据结构");
    println!("  ✅ 数组切片 - 灵活的数据视图");
    println!("  ✅ 数组方法 - 丰富的内置操作");
    println!("  ✅ 性能特性 - 零成本抽象和内存效率");

    println!("\n💪 完成的练习:");
    println!("  🏆 练习1: 数组基础操作 - 统计、查找、变换");
    println!("  🏆 练习2: 搜索和排序 - 经典算法实现");
    println!("  🏆 练习3: 多维数组操作 - 矩阵运算");
    println!("  🏆 练习4: 高级切片操作 - 窗口、分块、旋转");
    println!("  🏆 练习5: 算法实现 - 排序算法大全");
    println!("  🏆 练习6: 实际应用 - 成绩管理、图像处理、数据分析");
    println!("  🏆 练习7: 性能优化 - 缓存友好、内存对齐");
    println!("  🏆 练习8: 综合应用 - 数据分析工具");

    println!("\n🔍 实际应用场景:");
    println!("  🎯 矩阵运算和科学计算");
    println!("  🎯 图像和信号处理");
    println!("  🎯 游戏开发中的网格和地图");
    println!("  🎯 嵌入式系统的缓冲区管理");
    println!("  🎯 算法竞赛和数据结构");
    println!("  🎯 系统编程中的固定大小数据");

    println!("\n📚 学习建议:");
    println!("  💡 数组 vs 向量: 理解固定大小 vs 动态大小的权衡");
    println!("  💡 切片的威力: 掌握零成本的数据视图");
    println!("  💡 性能考虑: 利用数组的缓存友好特性");
    println!("  💡 安全第一: Rust 的边界检查保护");
    println!("  💡 实践应用: 在实际项目中应用数组知识");

    println!("\n🔗 相关资源:");
    println!("  📖 Rust 语言圣经 - 数组章节");
    println!("  📖 Rust By Example - 数组和切片");
    println!("  📖 Rust 标准库文档 - std::array");
    println!("  📖 The Rust Programming Language - Arrays");

    println!("\n{}", "=".repeat(60));
    println!("🎉 恭喜！你已经全面掌握了 Rust 数组的核心知识！");
    println!("🚀 继续探索 Rust 的其他复合类型：向量、哈希表、结构体等");
    println!("{}", "=".repeat(60));
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_array_basic_functionality() {
        // 测试基本数组功能
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(arr.len(), 5);
        assert_eq!(arr[0], 1);
        assert_eq!(arr[4], 5);

        // 测试数组方法
        assert!(arr.contains(&3));
        assert!(!arr.contains(&10));

        let sum: i32 = arr.iter().sum();
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_multidimensional_arrays() {
        // 测试多维数组
        let matrix: [[i32; 2]; 3] = [[1, 2], [3, 4], [5, 6]];
        assert_eq!(matrix[0][1], 2);
        assert_eq!(matrix[2][0], 5);
        assert_eq!(matrix.len(), 3);
        assert_eq!(matrix[0].len(), 2);
    }

    #[test]
    fn test_array_slicing() {
        // 测试数组切片
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let slice = &arr[2..5];
        assert_eq!(slice, &[3, 4, 5]);
        assert_eq!(slice.len(), 3);

        let full_slice = &arr[..];
        assert_eq!(full_slice.len(), 10);

        let from_start = &arr[..3];
        assert_eq!(from_start, &[1, 2, 3]);

        let to_end = &arr[7..];
        assert_eq!(to_end, &[8, 9, 10]);
    }

    #[test]
    fn test_array_iteration() {
        // 测试数组遍历
        let arr = [1, 2, 3, 4, 5];

        // 测试值遍历
        let mut sum = 0;
        for value in arr {
            sum += value;
        }
        assert_eq!(sum, 15);

        // 测试引用遍历
        let mut sum2 = 0;
        for value in &arr {
            sum2 += value;
        }
        assert_eq!(sum2, 15);

        // 测试迭代器
        let sum3: i32 = arr.iter().sum();
        assert_eq!(sum3, 15);

        // 测试 enumerate
        for (i, &value) in arr.iter().enumerate() {
            assert_eq!(value, i as i32 + 1);
        }
    }

    #[test]
    fn test_array_methods() {
        // 测试数组方法
        let mut arr = [3, 1, 4, 1, 5, 9, 2, 6];

        // 测试查找
        assert!(arr.contains(&5));
        assert!(!arr.contains(&10));

        // 测试排序
        arr.sort();
        assert_eq!(arr, [1, 1, 2, 3, 4, 5, 6, 9]);

        // 测试反转
        arr.reverse();
        assert_eq!(arr, [9, 6, 5, 4, 3, 2, 1, 1]);

        // 测试首尾元素
        assert_eq!(arr.first(), Some(&9));
        assert_eq!(arr.last(), Some(&1));
    }

    #[test]
    fn test_array_windows_and_chunks() {
        // 测试窗口和分块
        let arr = [1, 2, 3, 4, 5, 6];

        // 测试窗口
        let windows: Vec<&[i32]> = arr.windows(3).collect();
        assert_eq!(windows.len(), 4);
        assert_eq!(windows[0], &[1, 2, 3]);
        assert_eq!(windows[3], &[4, 5, 6]);

        // 测试分块
        let chunks: Vec<&[i32]> = arr.chunks(2).collect();
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], &[1, 2]);
        assert_eq!(chunks[2], &[5, 6]);
    }

    #[test]
    fn test_array_safety() {
        // 测试数组安全性
        let arr = [1, 2, 3, 4, 5];

        // 安全访问
        assert_eq!(arr.get(2), Some(&3));
        assert_eq!(arr.get(10), None);

        // 边界检查
        assert!(arr.get(0).is_some());
        assert!(arr.get(arr.len() - 1).is_some());
        assert!(arr.get(arr.len()).is_none());
    }

    #[test]
    fn test_array_memory_layout() {
        // 测试数组内存布局
        let arr = [1i32, 2, 3, 4, 5];

        // 验证连续性
        let ptr0 = &arr[0] as *const i32;
        let ptr1 = &arr[1] as *const i32;

        unsafe {
            let diff = ptr1.offset_from(ptr0);
            assert_eq!(diff, 1); // 相邻元素相差1个i32大小
        }

        // 验证大小
        assert_eq!(std::mem::size_of_val(&arr), 5 * std::mem::size_of::<i32>());
    }

    #[test]
    fn test_array_type_conversions() {
        // 测试数组类型转换
        let arr = [1, 2, 3, 4, 5];

        // 数组到切片
        let slice: &[i32] = &arr;
        assert_eq!(slice.len(), 5);

        // 数组到向量
        let vec: Vec<i32> = arr.to_vec();
        assert_eq!(vec.len(), 5);
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);

        // 切片到向量
        let vec2: Vec<i32> = slice.to_vec();
        assert_eq!(vec2, vec);
    }
}
