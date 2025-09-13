//! # Rust 内联汇编深度分析
//! 
//! 本项目全面分析 Rust 内联汇编的各个方面，包括基础概念、语法、操作数类型、
//! 寄存器约束、性能优化、安全性考虑等，并提供丰富的实际案例。
//! 
//! 注意：本示例针对 ARM64 (AArch64) 架构进行了优化

use std::arch::asm;

/// 内联汇编基础概念和使用场景
fn inline_assembly_basics() {
    println!("=== Rust 内联汇编基础概念 ===");
    
    // 1. 最简单的内联汇编 - NOP 指令
    unsafe {
        asm!("nop");
    }
    println!("✓ 执行了 NOP (无操作) 指令");
    
    // 2. 基本的数据操作
    let x: u64;
    unsafe {
        asm!("mov {}, #42", out(reg) x);
    }
    println!("✓ 通过内联汇编设置变量值: x = {}", x);
    
    // 3. 输入和输出操作数
    let input = 10u64;
    let output: u64;
    unsafe {
        asm!(
            "mov {0}, {1}",
            "add {0}, {0}, #5",
            out(reg) output,
            in(reg) input,
        );
    }
    println!("✓ 输入输出操作: {} + 5 = {}", input, output);
}

/// 操作数类型详解
fn operand_types_analysis() {
    println!("\n=== 操作数类型详解 ===");
    
    // 1. in 操作数 - 只读输入
    let a = 15u64;
    let b = 25u64;
    let result: u64;
    unsafe {
        asm!(
            "add {result}, {a}, {b}",
            result = out(reg) result,
            a = in(reg) a,
            b = in(reg) b,
        );
    }
    println!("✓ in 操作数: {} + {} = {}", a, b, result);
    
    // 2. out 操作数 - 只写输出
    let output1: u64;
    let output2: u64;
    unsafe {
        asm!(
            "mov {out1}, #100",
            "mov {out2}, #200",
            out1 = out(reg) output1,
            out2 = out(reg) output2,
        );
    }
    println!("✓ out 操作数: output1 = {}, output2 = {}", output1, output2);
    
    // 3. inout 操作数 - 读写
    let mut value = 50u64;
    unsafe {
        asm!(
            "lsl {0}, {0}, #1",  // 左移1位，相当于乘以2
            inout(reg) value,
        );
    }
    println!("✓ inout 操作数: 50 << 1 = {}", value);
    
    // 4. lateout 操作数 - 延迟输出 (ARM64 除法示例)
    let input1 = 8u64;
    let input2 = 3u64;
    let quotient: u64;
    let remainder: u64;
    unsafe {
        asm!(
            "udiv {quotient}, {dividend}, {divisor}",
            "msub {remainder}, {quotient}, {divisor}, {dividend}",
            dividend = in(reg) input1,
            divisor = in(reg) input2,
            quotient = out(reg) quotient,
            remainder = lateout(reg) remainder,
        );
    }
    println!("✓ lateout 操作数: {} ÷ {} = {} 余 {}", input1, input2, quotient, remainder);
}

/// 寄存器约束和分配
fn register_constraints() {
    println!("\n=== 寄存器约束和分配 ===");
    
    // 1. 通用寄存器约束 (reg)
    let a = 10u64;
    let b = 20u64;
    let sum: u64;
    unsafe {
        asm!(
            "add {}, {}, {}",
            out(reg) sum,
            in(reg) a,
            in(reg) b,
        );
    }
    println!("✓ 通用寄存器约束: {} + {} = {}", a, b, sum);
    
    // 2. 显式寄存器约束 (ARM64)
    let value = 0x12345678u64;
    let low_part: u32;
    let high_part: u32;
    unsafe {
        asm!(
            "mov {low:w}, {value:w}",
            "lsr {value}, {value}, #32",
            "mov {high:w}, {value:w}",
            value = inout(reg) value => _,
            low = out(reg) low_part,
            high = out(reg) high_part,
        );
    }
    println!("✓ 显式寄存器: 0x{:x} = 0x{:x} (高32位) + 0x{:x} (低32位)", 
             0x12345678u64, high_part, low_part);
    
    // 3. 寄存器类别约束 (ARM64 NEON)
    #[cfg(target_arch = "aarch64")]
    {
        let input = 42.0f64;
        let output: f64;
        unsafe {
            asm!(
                "fmov {out:d}, {inp:d}",
                "fadd {out:d}, {out:d}, {out:d}",  // 乘以2
                inp = in(vreg) input,
                out = out(vreg) output,
            );
        }
        println!("✓ NEON 寄存器: {} * 2 = {}", input, output);
    }
}

/// 内存操作和指针处理
fn memory_operations() {
    println!("\n=== 内存操作和指针处理 ===");
    
    // 1. 内存读写 (ARM64)
    let buffer = [1u32, 2, 3, 4, 5];
    let sum: u32;
    unsafe {
        asm!(
            "mov {sum:w}, #0",
            "mov {counter:w}, #5",
            "2:",
            "ldr {tmp:w}, [{ptr}], #4",
            "add {sum:w}, {sum:w}, {tmp:w}",
            "subs {counter:w}, {counter:w}, #1",
            "b.ne 2b",
            ptr = inout(reg) buffer.as_ptr() => _,
            sum = out(reg) sum,
            counter = out(reg) _,
            tmp = out(reg) _,
        );
    }
    println!("✓ 内存操作: 数组求和 = {}", sum);
    
    // 2. 字符串操作 (ARM64)
    let src = b"Hello, Rust!";
    let mut dst = [0u8; 20];
    let len = src.len();
    unsafe {
        asm!(
            "1:",
            "ldrb {tmp:w}, [{src}], #1",
            "strb {tmp:w}, [{dst}], #1",
            "subs {len:w}, {len:w}, #1",
            "b.ne 1b",
            src = inout(reg) src.as_ptr() => _,
            dst = inout(reg) dst.as_mut_ptr() => _,
            len = inout(reg) len => _,
            tmp = out(reg) _,
        );
    }
    let copied_str = std::str::from_utf8(&dst[..src.len()]).unwrap();
    println!("✓ 字符串复制: '{}'", copied_str);
}

/// 位操作和算术运算
fn bit_operations() {
    println!("\n=== 位操作和算术运算 ===");
    
    // 1. 位计数操作 (ARM64)
    let value = 0b11010110u64;
    let leading_zeros: u64;
    let trailing_zeros: u64;
    unsafe {
        asm!(
            "clz {lz}, {val}",
            "rbit {tmp}, {val}",
            "clz {tz}, {tmp}",
            val = in(reg) value,
            lz = out(reg) leading_zeros,
            tz = out(reg) trailing_zeros,
            tmp = out(reg) _,
        );
    }
    // 手动计算 popcount
    let popcount = value.count_ones() as u64;
    println!("✓ 位操作: 0b{:08b} -> popcount={}, lzcnt={}, tzcnt={}", 
             value, popcount, leading_zeros, trailing_zeros);
    
    // 2. 快速乘法和除法 (ARM64)
    let dividend = 100u64;
    let quotient: u64;
    let remainder: u64;
    unsafe {
        asm!(
            "lsr {quotient}, {div}, #2",  // 除以4的快速方法
            "and {remainder}, {div}, #3",  // 取模4的快速方法
            div = in(reg) dividend,
            quotient = out(reg) quotient,
            remainder = out(reg) remainder,
        );
    }
    println!("✓ 快速运算: {} ÷ 4 = {} 余 {}", dividend, quotient, remainder);
}

/// 控制流和条件操作
fn control_flow_operations() {
    println!("\n=== 控制流和条件操作 ===");
    
    // 1. 条件移动 (ARM64)
    let a = 15i32;
    let b = 25i32;
    let max_value: i32;
    unsafe {
        asm!(
            "cmp {a:w}, {b:w}",
            "csel {result:w}, {b:w}, {a:w}, lt",  // 如果 a < b，选择 b，否则选择 a
            a = in(reg) a,
            b = in(reg) b,
            result = out(reg) max_value,
        );
    }
    println!("✓ 条件移动: max({}, {}) = {}", a, b, max_value);
    
    // 2. 简单循环计数
    let mut counter = 0u32;
    unsafe {
        asm!(
            "mov {counter:w}, #0",
            "1:",
            "add {counter:w}, {counter:w}, #1",
            "cmp {counter:w}, #10",
            "b.lt 1b",
            counter = inout(reg) counter,
        );
    }
    println!("✓ 循环计数: 计数到 {}", counter);
}

/// 性能优化技巧
fn performance_optimizations() {
    println!("\n=== 性能优化技巧 ===");
    
    // 1. 寄存器重用优化
    let x = 10u64;
    let result: u64;
    unsafe {
        asm!(
            "lsl {tmp}, {x}, #1",      // x * 2
            "lsl {x}, {x}, #2",        // x * 4
            "add {x}, {x}, {tmp}",     // x * 4 + x * 2 = x * 6
            x = inout(reg) x => result,
            tmp = out(reg) _,
        );
    }
    println!("✓ 寄存器重用: {} * 6 = {} (使用移位和加法)", 10, result);
    
    // 2. 避免分支的技巧
    let condition = true;
    let a = 100u64;
    let b = 200u64;
    let result: u64;
    unsafe {
        asm!(
            "cmp {cond}, #0",
            "csel {result}, {a}, {b}, ne",  // 如果条件非0，选择a，否则选择b
            cond = in(reg) condition as u64,
            a = in(reg) a,
            b = in(reg) b,
            result = out(reg) result,
        );
    }
    println!("✓ 无分支选择: condition={} -> {}", condition, result);
}

/// 系统调用和底层接口
fn system_calls() {
    println!("\n=== 系统调用和底层接口 ===");
    
    // 1. CPU特性检测 (通用)
    println!("✓ CPU架构: {}", std::env::consts::ARCH);
    println!("✓ 操作系统: {}", std::env::consts::OS);
    
    // 2. 时间戳读取 (ARM64)
    #[cfg(target_arch = "aarch64")]
    {
        let timestamp: u64;
        unsafe {
            asm!(
                "mrs {}, cntvct_el0",  // 读取虚拟计数器
                out(reg) timestamp,
            );
        }
        println!("✓ 时间戳计数器: {}", timestamp);
    }
}

/// 安全性考虑和最佳实践
fn safety_considerations() {
    println!("\n=== 安全性考虑和最佳实践 ===");
    
    // 1. 正确的内存对齐
    #[repr(align(16))]
    struct AlignedData {
        data: [u8; 16],
    }
    
    let aligned = AlignedData { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16] };
    let sum: u64;
    unsafe {
        asm!(
            "mov {sum}, #0",
            "ldr {tmp}, [{ptr}]",      // 加载8字节
            "add {sum}, {sum}, {tmp}",
            "ldr {tmp}, [{ptr}, #8]",  // 加载下8字节
            "add {sum}, {sum}, {tmp}",
            ptr = in(reg) aligned.data.as_ptr(),
            sum = out(reg) sum,
            tmp = out(reg) _,
        );
    }
    println!("✓ 内存对齐: 正确处理16字节对齐的数据，和 = {}", sum);
    
    // 2. 栈平衡检查
    let stack_value = 123u64;
    let retrieved: u64;
    unsafe {
        asm!(
            "str {val}, [sp, #-16]!",  // 压栈
            "ldr {result}, [sp], #16", // 弹栈，确保栈平衡
            val = in(reg) stack_value,
            result = out(reg) retrieved,
        );
    }
    println!("✓ 栈平衡: 压栈并弹出值 = {}", retrieved);
}

/// 高级特性和选项
fn advanced_features() {
    println!("\n=== 高级特性和选项 ===");
    
    // 1. 纯函数标记 (pure)
    let input = 25u64;
    let sqrt_result: u64;
    unsafe {
        asm!(
            "lsr {result}, {input}, #1",     // 简单的平方根近似
            input = in(reg) input,
            result = out(reg) sqrt_result,
            options(pure, nomem, nostack),  // 标记为纯函数
        );
    }
    println!("✓ 纯函数: sqrt({}) ≈ {}", input, sqrt_result);
    
    // 2. 只读内存访问
    let data = [1u32, 2, 3, 4, 5];
    let checksum: u32;
    unsafe {
        asm!(
            "mov {sum:w}, #0",
            "mov {i:w}, #0",
            "2:",
            "cmp {i:w}, #5",
            "b.ge 3f",
            "ldr {tmp:w}, [{ptr}, {i}, lsl #2]",
            "add {sum:w}, {sum:w}, {tmp:w}",
            "add {i:w}, {i:w}, #1",
            "b 2b",
            "3:",
            ptr = in(reg) data.as_ptr(),
            sum = out(reg) checksum,
            i = out(reg) _,
            tmp = out(reg) _,
            options(readonly),  // 只读内存访问
        );
    }
    println!("✓ 只读访问: 数组校验和 = {}", checksum);
}

/// 测试用例
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_operations() {
        let x: u64;
        unsafe {
            asm!("mov {}, #42", out(reg) x);
        }
        assert_eq!(x, 42);
    }
    
    #[test]
    fn test_arithmetic() {
        let a = 10u64;
        let b = 5u64;
        let sum: u64;
        unsafe {
            asm!(
                "add {}, {}, {}",
                out(reg) sum,
                in(reg) a,
                in(reg) b,
            );
        }
        assert_eq!(sum, 15);
    }
    
    #[test]
    fn test_bit_operations() {
        let value = 0b1010u64;
        let leading_zeros: u64;
        unsafe {
            asm!(
                "clz {}, {}",
                out(reg) leading_zeros,
                in(reg) value,
            );
        }
        assert_eq!(leading_zeros, 60); // 64 - 4 = 60
    }
    
    #[test]
    fn test_conditional_operations() {
        let a = 15i32;
        let b = 25i32;
        let max: i32;
        unsafe {
            asm!(
                "cmp {a:w}, {b:w}",
                "csel {result:w}, {b:w}, {a:w}, lt",
                a = in(reg) a,
                b = in(reg) b,
                result = out(reg) max,
            );
        }
        assert_eq!(max, 25);
    }
}

fn main() {
    println!("🚀 Rust 内联汇编深度分析 (ARM64/AArch64)");
    println!("===========================================\n");
    
    inline_assembly_basics();
    operand_types_analysis();
    register_constraints();
    memory_operations();
    bit_operations();
    control_flow_operations();
    performance_optimizations();
    system_calls();
    safety_considerations();
    advanced_features();
    
    println!("\n=== 总结 ===");
    println!("✅ 内联汇编基础概念和语法");
    println!("✅ 操作数类型 (in, out, inout, lateout)");
    println!("✅ 寄存器约束和分配策略");
    println!("✅ 内存操作和指针处理");
    println!("✅ 位操作和算术运算优化");
    println!("✅ 控制流和条件操作");
    println!("✅ 性能优化技巧");
    println!("✅ 系统调用和底层接口");
    println!("✅ 安全性考虑和最佳实践");
    println!("✅ 高级特性和编译选项");
    
    println!("\n🎯 关键要点 (ARM64):");
    println!("• 内联汇编必须在 unsafe 块中使用");
    println!("• ARM64 使用不同的指令集和寄存器约定");
    println!("• 寄存器模板修饰符: :w (32位), :x (64位), :d (双精度浮点)");
    println!("• 立即数需要 # 前缀");
    println!("• 条件分支使用 b.condition 格式");
    println!("• NEON 寄存器用于 SIMD 和浮点运算");
    
    println!("\n📚 学习建议:");
    println!("• 了解目标架构的指令集特性");
    println!("• 掌握 ARM64 的寻址模式和指令格式");
    println!("• 理解不同寄存器类别的用途");
    println!("• 重视内存对齐和栈管理");
    println!("• 使用适当的编译选项优化代码");
    
    println!("\n🔧 实践应用场景:");
    println!("• 移动设备和嵌入式系统");
    println!("• 服务器和云计算 (ARM 服务器)");
    println!("• 高性能计算和机器学习");
    println!("• 操作系统内核开发");
    println!("• 密码学和安全算法");
    println!("• 实时系统和驱动程序");
}