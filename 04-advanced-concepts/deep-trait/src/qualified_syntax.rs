//! 完全限定语法模块
//! 
//! 本模块展示了 Rust 中完全限定语法 (Fully Qualified Syntax) 的使用，
//! 包括基础用法、消歧义调用和关联函数调用。

/// 基础完全限定语法演示
pub fn basic_fully_qualified_syntax_demo() {
    println!("\n--- 3.1 基础完全限定语法 ---");

    trait Pilot {
        fn fly(&self);
        fn name() -> String;
    }

    trait Wizard {
        fn fly(&self);
        fn name() -> String;
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }

        fn name() -> String {
            String::from("Captain")
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }

        fn name() -> String {
            String::from("Gandalf")
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }

        fn name() -> String {
            String::from("Human")
        }
    }

    let person = Human;

    // 默认调用 Human 的方法
    person.fly();

    // 使用完全限定语法调用特定 trait 的方法
    Pilot::fly(&person);
    Wizard::fly(&person);

    // 对于关联函数，必须使用完全限定语法
    println!("Human name: {}", Human::name());
    println!("Pilot name: {}", <Human as Pilot>::name());
    println!("Wizard name: {}", <Human as Wizard>::name());
}

/// 消歧义调用演示
pub fn disambiguation_demo() {
    println!("\n--- 3.2 消歧义调用 ---");

    trait Animal {
        fn baby_name() -> String;
        fn make_sound(&self);
    }

    trait Dog {
        fn baby_name() -> String;
        fn make_sound(&self);
    }

    struct Puppy;

    impl Animal for Puppy {
        fn baby_name() -> String {
            String::from("puppy")
        }

        fn make_sound(&self) {
            println!("Animal sound: generic animal noise");
        }
    }

    impl Dog for Puppy {
        fn baby_name() -> String {
            String::from("Spot")
        }

        fn make_sound(&self) {
            println!("Dog sound: Woof!");
        }
    }

    impl Puppy {
        fn baby_name() -> String {
            String::from("puppy instance")
        }

        fn make_sound(&self) {
            println!("Puppy sound: Yip!");
        }
    }

    let puppy = Puppy;

    // 实例方法调用
    puppy.make_sound(); // 默认调用 Puppy 的方法
    Animal::make_sound(&puppy); // 调用 Animal trait 的方法
    Dog::make_sound(&puppy); // 调用 Dog trait 的方法

    // 关联函数调用需要完全限定语法
    println!("Puppy baby name: {}", Puppy::baby_name());
    println!("Animal baby name: {}", <Puppy as Animal>::baby_name());
    println!("Dog baby name: {}", <Puppy as Dog>::baby_name());

    // 复杂的消歧义场景
    trait Display {
        fn fmt(&self) -> String;
    }

    trait Debug {
        fn fmt(&self) -> String;
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl Display for Point {
        fn fmt(&self) -> String {
            format!("({}, {})", self.x, self.y)
        }
    }

    impl Debug for Point {
        fn fmt(&self) -> String {
            format!("Point {{ x: {}, y: {} }}", self.x, self.y)
        }
    }

    let point = Point { x: 1, y: 2 };

    println!("Display format: {}", <Point as Display>::fmt(&point));
    println!("Debug format: {}", <Point as Debug>::fmt(&point));
}

/// 关联函数的完全限定调用
pub fn associated_functions_demo() {
    println!("\n--- 3.3 关联函数的完全限定调用 ---");

    trait Factory<T> {
        fn create() -> T;
        fn create_with_value(value: i32) -> T;
    }

    #[derive(Debug)]
    pub struct Product {
        id: i32,
        name: String,
    }

    impl Factory<Product> for Product {
        fn create() -> Product {
            Product {
                id: 0,
                name: "Default Product".to_string(),
            }
        }

        fn create_with_value(value: i32) -> Product {
            Product {
                id: value,
                name: format!("Product {}", value),
            }
        }
    }

    // 使用完全限定语法调用关联函数
    let default_product = <Product as Factory<Product>>::create();
    let custom_product = <Product as Factory<Product>>::create_with_value(42);

    println!("Default product: {:?}", default_product);
    println!("Custom product: {:?}", custom_product);

    // 泛型关联函数的完全限定调用
    trait Parser<T> {
        type Error;
        fn parse(input: &str) -> Result<T, Self::Error>;
    }

    pub struct IntParser;

    impl Parser<i32> for IntParser {
        type Error = std::num::ParseIntError;

        fn parse(input: &str) -> Result<i32, Self::Error> {
            input.parse()
        }
    }

    pub struct FloatParser;

    impl Parser<f64> for FloatParser {
        type Error = std::num::ParseFloatError;

        fn parse(input: &str) -> Result<f64, Self::Error> {
            input.parse()
        }
    }

    // 完全限定语法调用泛型关联函数
    match <IntParser as Parser<i32>>::parse("42") {
        Ok(value) => println!("Parsed integer: {}", value),
        Err(e) => println!("Failed to parse integer: {}", e),
    }

    match <FloatParser as Parser<f64>>::parse("3.14") {
        Ok(value) => println!("Parsed float: {}", value),
        Err(e) => println!("Failed to parse float: {}", e),
    }

    match <IntParser as Parser<i32>>::parse("invalid") {
        Ok(value) => println!("Parsed integer: {}", value),
        Err(e) => println!("Failed to parse integer: {}", e),
    }
}

/// 完全限定语法主入口函数
pub fn fully_qualified_syntax_analysis() {
    println!("\n=== 3. 完全限定语法分析 ===");
    
    basic_fully_qualified_syntax_demo();
    disambiguation_demo();
    associated_functions_demo();
}