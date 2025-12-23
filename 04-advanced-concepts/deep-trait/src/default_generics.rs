//! 默认泛型参数模块
//! 
//! 本模块展示了 Rust 中默认泛型参数的定义和使用，包括基础用法和高级应用。

use std::fmt::Display;
use std::ops::Add;
use std::fmt::Debug;

/// 基础默认泛型参数演示
pub fn basic_default_generics_demo() {
    println!("\n--- 2.1 基础默认泛型参数 ---");

    // 定义带有默认泛型参数的结构体
    struct Point<T = f64> {
        // 默认类型为 f64
        x: T,
        y: T,
    }

    impl<T> Point<T>
    where
        T: Add<Output = T> + Copy + Display,
    {
        fn new(x: T, y: T) -> Self {
            Point { x, y }
        }

        fn add(&self, other: &Point<T>) -> Point<T> {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }

        fn display(&self) {
            println!("Point({}, {})", self.x, self.y);
        }
    }

    // 使用默认类型参数
    let point1 = Point::new(1.0, 2.0); // 自动推断为 Point<f64>
    let point2 = Point::new(3.0, 4.0);
    let sum = point1.add(&point2);

    println!("Default f64 points:");
    point1.display();
    point2.display();
    sum.display();

    // 显式指定类型参数
    let int_point1 = Point::<i32>::new(1, 2);
    let int_point2 = Point::<i32>::new(3, 4);
    let int_sum = int_point1.add(&int_point2);

    println!("\nExplicit i32 points:");
    int_point1.display();
    int_point2.display();
    int_sum.display();
}

/// 运算符重载中的默认泛型参数
pub fn operator_overloading_defaults_demo() {
    println!("\n--- 2.2 运算符重载中的默认泛型参数 ---");

    #[derive(Debug, Clone, Copy)]
    pub struct Millimeters(u32);

    #[derive(Debug, Clone, Copy)]
    pub struct Meters(u32);

    // 为 Millimeters 实现 Add，默认与自身相加
    impl Add for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Millimeters) -> Millimeters {
            Millimeters(self.0 + other.0)
        }
    }

    // 为 Millimeters 实现与 Meters 相加
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    // 使用示例
    let mm1 = Millimeters(1000);
    let mm2 = Millimeters(500);
    let m1 = Meters(2);

    let result1 = mm1 + mm2; // 使用默认的 Rhs = Self
    let result2 = mm1 + m1; // 使用显式的 Rhs = Meters

    println!("Millimeters + Millimeters: {:?}", result1);
    println!("Millimeters + Meters: {:?}", result2);

    // 自定义数值类型的运算符重载
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Complex {
        real: f64,
        imag: f64,
    }

    impl Complex {
        pub fn new(real: f64, imag: f64) -> Self {
            Complex { real, imag }
        }
    }

    impl Add for Complex {
        type Output = Complex;

        fn add(self, other: Complex) -> Complex {
            Complex {
                real: self.real + other.real,
                imag: self.imag + other.imag,
            }
        }
    }

    // 复数与实数相加
    impl Add<f64> for Complex {
        type Output = Complex;

        fn add(self, other: f64) -> Complex {
            Complex {
                real: self.real + other,
                imag: self.imag,
            }
        }
    }

    let c1 = Complex::new(1.0, 2.0);
    let c2 = Complex::new(3.0, 4.0);
    let real_num = 5.0;

    let complex_sum = c1 + c2;
    let mixed_sum = c1 + real_num;

    println!("Complex + Complex: {:?}", complex_sum);
    println!("Complex + f64: {:?}", mixed_sum);
}

/// 复杂默认泛型参数应用
pub fn complex_default_generics_demo() {
    println!("\n--- 2.3 复杂默认泛型参数应用 ---");

    // 带有多个默认泛型参数的容器
    struct Container<T, E = String, const N: usize = 10> {
        items: Vec<Option<T>>,
        error_handler: fn(E),
    }

    impl<T, E, const N: usize> Container<T, E, N>
    where
        T: Debug,
        E: Display,
    {
        fn new(error_handler: fn(E)) -> Self
        where
            T: Clone,
        {
            Container {
                items: vec![None; N],
                error_handler,
            }
        }

        fn add(&mut self, item: T) -> Result<(), E>
        where
            E: From<&'static str>,
        {
            // 使用iter_mut和位置查找，更高效
            if let Some(slot) = self.items.iter_mut().find(|slot| slot.is_none()) {
                *slot = Some(item);
                Ok(())
            } else {
                Err(E::from("Container is full"))
            }
        }

        fn get(&self, index: usize) -> Option<&T> {
            // 更简洁的实现
            self.items.get(index)?.as_ref()
        }

        fn len(&self) -> usize {
            self.items.iter().filter_map(|item| item.as_ref()).count()
        }
    }

    // 使用默认参数
    let mut default_container = Container::<i32>::new(|e: String| {
        println!("Error: {}", e);
    });

    // 添加元素
    for i in 0..12 {
        match default_container.add(i) {
            Ok(()) => println!("Added {}", i),
            Err(e) => {
                (default_container.error_handler)(e);
                break;
            }
        }
    }

    println!("Container length: {}", default_container.len());

    // 使用自定义参数
    #[derive(Debug)]
    pub struct CustomError {
        code: u32,
        message: String,
    }

    impl Display for CustomError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error {}: {}", self.code, self.message)
        }
    }

    impl From<&'static str> for CustomError {
        fn from(msg: &'static str) -> Self {
            CustomError {
                code: 500,
                message: msg.to_string(),
            }
        }
    }

    let mut custom_container = Container::<String, CustomError, 5>::new(|e: CustomError| {
        println!("Custom error handler: {}", e);
    });

    for i in 0..7 {
        let item = format!("Item {}", i);
        match custom_container.add(item.clone()) {
            Ok(()) => println!("Added {}", item),
            Err(e) => {
                (custom_container.error_handler)(e);
                break;
            }
        }
    }

    println!("Custom container length: {}", custom_container.len());
}

/// 默认泛型类型参数主入口函数
pub fn default_generic_params_analysis() {
    println!("\n=== 2. 默认泛型类型参数分析 ===");
    
    basic_default_generics_demo();
    operator_overloading_defaults_demo();
    complex_default_generics_demo();
}