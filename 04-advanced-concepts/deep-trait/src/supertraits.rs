//! 超特征模块
//! 
//! 本模块深入展示了 Rust 中超特征 (Supertraits) 的概念和应用，
//! 包括基础超特征、多重超特征、泛型超特征约束以及实际应用场景，
//! 同时提供了更全面的错误处理和性能优化示例。

use std::fmt::{Debug, Display};
use std::error::Error;

/// 基础超特征概念演示
/// 
/// 展示了超特征的基本关系和方法调用链
pub fn basic_supertraits_demo() {
    println!("\n--- 4.1 基础超特征概念 ---");

    // 定义超特征关系
    trait Animal {
        fn name(&self) -> &str;
        fn make_sound(&self);
        // 添加默认实现的通用描述方法
        fn describe(&self) {
            println!("Animal: {}", self.name());
        }
    }

    // Dog 特征依赖于 Animal 特征（Animal 是 Dog 的超特征）
    trait Dog: Animal {
        fn breed(&self) -> &str;
        fn fetch(&self) {
            println!("{} is fetching!", self.name());
        }
        // 扩展默认行为
        fn full_description(&self) {
            self.describe();
            println!("Breed: {}", self.breed());
        }
    }

    #[derive(Debug)]
    pub struct GoldenRetriever {
        name: String,
    }

    impl Animal for GoldenRetriever {
        fn name(&self) -> &str {
            &self.name
        }

        fn make_sound(&self) {
            println!("{} says: Woof! Woof!", self.name());
        }
    }

    impl Dog for GoldenRetriever {
        fn breed(&self) -> &str {
            "Golden Retriever"
        }
    }

    let dog = GoldenRetriever {
        name: "Buddy".to_string(),
    };

    // 展示方法调用链
    dog.full_description();
    dog.make_sound();
    dog.fetch();

    // 超特征约束的函数
    fn train_dog<T: Dog>(dog: &T) {
        println!("\nTraining {} ({})", dog.name(), dog.breed());
        dog.fetch();
        dog.make_sound();
    }

    train_dog(&dog);

    // 展示特征对象的使用
    let animal_ref: &dyn Animal = &dog;
    println!("\nUsing as Animal trait object:");
    animal_ref.describe();
    animal_ref.make_sound();
}

/// 多重超特征演示
/// 
/// 展示了多个超特征组合和复杂约束条件，增强了错误处理和状态管理
pub fn multiple_supertraits_demo() {
    println!("\n--- 4.2 多重超特征 ---");

    // 多重超特征约束
    trait Drawable {
        fn draw(&self);
        // 添加绘制状态检查
        fn can_draw(&self) -> bool {
            true
        }
    }

    trait Clickable {
        fn click(&self);
        // 添加点击有效性检查
        fn is_clickable(&self) -> bool {
            true
        }
    }

    // Button 需要同时实现 Drawable 和 Clickable
    trait Button: Drawable + Clickable {
        fn label(&self) -> &str;
        fn id(&self) -> &str;

        fn render(&self) {
            if self.can_draw() {
                println!("Rendering button [{}]: {}", self.id(), self.label());
                self.draw();
            } else {
                println!("Cannot render button [{}]", self.id());
            }
        }

        fn handle_click(&self) {
            if self.is_clickable() {
                println!("Processing click for button [{}]", self.id());
                self.click();
            } else {
                println!("Button [{}] is not clickable", self.id());
            }
        }
    }

    #[derive(Debug)]
    pub struct SubmitButton {
        id: String,
        text: String,
        enabled: bool,
    }

    impl Drawable for SubmitButton {
        fn draw(&self) {
            println!("Drawing submit button with text: {}", self.text);
            println!("  Button ID: {}", self.id);
            println!("  Enabled: {}", self.enabled);
        }

        fn can_draw(&self) -> bool {
            true // 按钮始终可绘制，无论是否启用
        }
    }

    impl Clickable for SubmitButton {
        fn click(&self) {
            if self.enabled {
                println!("Submit button clicked! Submitting form...");
            } else {
                println!("Cannot submit: button is disabled");
            }
        }

        fn is_clickable(&self) -> bool {
            self.enabled // 只有启用的按钮才可点击
        }
    }

    impl Button for SubmitButton {
        fn label(&self) -> &str {
            &self.text
        }

        fn id(&self) -> &str {
            &self.id
        }
    }

    // 测试启用的按钮
    let enabled_btn = SubmitButton {
        id: "submit-1".to_string(),
        text: "Submit".to_string(),
        enabled: true,
    };

    println!("\n--- Enabled Button ---");
    enabled_btn.render();
    enabled_btn.handle_click();

    // 测试禁用的按钮
    let disabled_btn = SubmitButton {
        id: "submit-2".to_string(),
        text: "Disabled Submit".to_string(),
        enabled: false,
    };

    println!("\n--- Disabled Button ---");
    disabled_btn.render();
    disabled_btn.handle_click();

    // 复杂的多重超特征约束与错误处理
    trait Serializable {
        fn serialize(&self) -> String;
    }

    trait Deserializable: Sized {
        fn deserialize(data: &str) -> Result<Self, Box<dyn Error>>;
    }

    // 增强的持久化特征，添加错误处理
    trait Persistable: Serializable + Deserializable + Clone + Debug + Display {
        fn save(&self) -> Result<(), Box<dyn Error>> {
            let data = self.serialize();
            println!("Saving data: {}", data);
            // 模拟保存操作
            Ok(())
        }

        fn load(data: &str) -> Result<Self, Box<dyn Error>> {
            Self::deserialize(data)
        }

        fn update_and_save(&mut self, update_fn: impl FnOnce(&mut Self)) -> Result<(), Box<dyn Error>> {
            update_fn(self);
            self.save()
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct User {
        id: u32,
        name: String,
        email: String,
        active: bool,
    }

    impl Display for User {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "User(id={}, name='{}', email='{}', active={})", 
                   self.id, self.name, self.email, self.active)
        }
    }

    impl Serializable for User {
        fn serialize(&self) -> String {
            format!("{}|{}|{}|{}", self.id, self.name, self.email, self.active)
        }
    }

    impl Deserializable for User {
        fn deserialize(data: &str) -> Result<Self, Box<dyn Error>> {
            let parts: Vec<&str> = data.split('|').collect();
            if parts.len() != 4 {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("expected 4 fields, got {}", parts.len())
                )));
            }

            // 改进的类型安全和错误处理，提供更具体的错误信息
            let id = parts[0].parse::<u32>().map_err(|e| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("invalid id format: {}", e)
                )) as Box<dyn Error>
            })?;
            
            let name = parts[1].to_string();
            let email = parts[2].to_string();
            
            let active = parts[3].parse::<bool>().map_err(|e| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("invalid active status format: {}", e)
                )) as Box<dyn Error>
            })?;

            Ok(User {
                id,
                name,
                email,
                active,
            })
        }
    }

    impl Persistable for User {}

    let mut user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        active: true,
    };

    println!("\n--- User Persistence Demo ---");
    println!("Original user: {}", user);

    match user.save() {
        Ok(()) => println!("User saved successfully"),
        Err(e) => println!("Failed to save user: {}", e),
    }

    // 测试成功加载
    let valid_data = "2|Bob|bob@example.com|true";
    println!("\nLoading valid data: {}", valid_data);
    match User::load(valid_data) {
        Ok(loaded_user) => println!("Loaded user: {}", loaded_user),
        Err(e) => println!("Failed to load user: {}", e),
    }

    // 测试错误加载
    let invalid_data = "3|Charlie|invalid-email";
    println!("\nLoading invalid data: {}", invalid_data);
    match User::load(invalid_data) {
        Ok(loaded_user) => println!("Loaded user: {}", loaded_user),
        Err(e) => println!("Failed to load user: {}", e),
    }

    // 测试更新并保存
    println!("\nUpdating user...");
    match user.update_and_save(|u| {
        u.name = "Alice Smith".to_string();
        u.email = "alice.smith@example.com".to_string();
    }) {
        Ok(()) => println!("User updated and saved successfully: {}", user),
        Err(e) => println!("Failed to update and save user: {}", e),
    }
}

/// 超特征的实际应用
/// 
/// 展示了在实际图形系统中的超特征层次结构、性能优化和高级功能
pub fn supertraits_practical_demo() {
    println!("\n--- 4.3 超特征的实际应用 ---");

    // 构建一个图形系统的特征层次结构
    trait Shape {
        fn area(&self) -> f64;
        fn perimeter(&self) -> f64;
        // 添加边界框计算方法
        fn bounding_box(&self) -> (f64, f64, f64, f64); // (min_x, min_y, max_x, max_y)
    }

    trait Drawable: Shape {
        fn draw(&self) {
            let (min_x, min_y, max_x, max_y) = self.bounding_box();
            println!(
                "Drawing shape with area: {:.2}, perimeter: {:.2}",
                self.area(),
                self.perimeter()
            );
            println!(
                "  Bounding box: [{:.1}, {:.1}] to [{:.1}, {:.1}]",
                min_x, min_y, max_x, max_y
            );
        }

        // 高级绘制方法，支持细节级别
        fn draw_detailed(&self, detail_level: u8) {
            match detail_level {
                0 => println!("Drawing shape outline only"),
                1 => {
                    self.draw();
                    println!("  Drawing basic details");
                },
                _ => {
                    self.draw();
                    println!("  Drawing high detailed version");
                },
            }
        }
    }

    trait Transformable: Shape {
        fn translate(&mut self, dx: f64, dy: f64);
        fn scale(&mut self, factor: f64);
        // 添加旋转方法
        fn rotate(&mut self, angle_degrees: f64);

        // 组合变换方法，优化多次变换的性能
        fn transform(&mut self, dx: f64, dy: f64, scale_factor: f64, angle: f64) {
            self.translate(dx, dy);
            self.scale(scale_factor);
            self.rotate(angle);
        }
    }

    // 高级图形特征需要同时支持绘制和变换
    trait AdvancedShape: Drawable + Transformable + Debug + PartialEq {
        fn render_with_transform(&mut self, dx: f64, dy: f64, scale: f64) {
            println!("\nApplying transformations: translate({:.1}, {:.1}), scale({:.1})", dx, dy, scale);
            self.translate(dx, dy);
            self.scale(scale);
            self.draw();
        }

        fn render_with_full_transform(&mut self, dx: f64, dy: f64, scale: f64, angle: f64) {
            println!("\nApplying full transformation: translate({:.1}, {:.1}), scale({:.1}), rotate({:.1}°)", 
                     dx, dy, scale, angle);
            self.transform(dx, dy, scale, angle);
            self.draw_detailed(2);
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Circle {
        x: f64,
        y: f64,
        radius: f64,
        rotation: f64, // 增加旋转状态
    }

    impl Circle {
        pub fn new(x: f64, y: f64, radius: f64) -> Self {
            Circle {
                x, y, radius,
                rotation: 0.0,
            }
        }

        pub fn distance_to(&self, other: &Self) -> f64 {
            ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
        }

        pub fn is_overlapping(&self, other: &Self) -> bool {
            self.distance_to(other) <= (self.radius + other.radius)
        }
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius.powi(2)
        }

        fn perimeter(&self) -> f64 {
            2.0 * std::f64::consts::PI * self.radius
        }

        fn bounding_box(&self) -> (f64, f64, f64, f64) {
            // 考虑旋转后的边界框
            let radius = self.radius;
            (self.x - radius, self.y - radius, self.x + radius, self.y + radius)
        }
    }

    impl Drawable for Circle {
        fn draw(&self) {
            println!(
                "Drawing circle at ({:.1}, {:.1}) with radius {:.1}",
                self.x, self.y, self.radius
            );
            if self.rotation != 0.0 {
                println!("  Rotation: {:.1} degrees", self.rotation);
            }
            println!(
                "  Area: {:.2}, Perimeter: {:.2}",
                self.area(),
                self.perimeter()
            );
        }
    }

    impl Transformable for Circle {
        fn translate(&mut self, dx: f64, dy: f64) {
            self.x += dx;
            self.y += dy;
            println!("Translated circle to ({:.1}, {:.1})", self.x, self.y);
        }

        fn scale(&mut self, factor: f64) {
            self.radius *= factor;
            println!("Scaled circle radius to {:.1}", self.radius);
        }

        fn rotate(&mut self, angle_degrees: f64) {
            self.rotation = (self.rotation + angle_degrees) % 360.0;
            println!("Rotated circle by {:.1} degrees, total rotation: {:.1} degrees", 
                     angle_degrees, self.rotation);
        }

        // 优化的组合变换实现
        fn transform(&mut self, dx: f64, dy: f64, scale_factor: f64, angle: f64) {
            // 优化版本，减少重复计算和日志输出
            self.x += dx;
            self.y += dy;
            self.radius *= scale_factor;
            self.rotation = (self.rotation + angle) % 360.0;
            println!("Applied combined transformation: position=({:.1},{:.1}), radius={:.1}, rotation={:.1}°",
                     self.x, self.y, self.radius, self.rotation);
        }
    }

    impl AdvancedShape for Circle {}

    // 创建并测试圆
    let mut circle = Circle::new(0.0, 0.0, 5.0);

    println!("Initial circle:");
    circle.draw_detailed(1);

    // 测试基础变换
    println!("\nApplying basic transformations:");
    circle.render_with_transform(10.0, 20.0, 1.5);

    // 测试旋转
    println!("\nApplying rotation:");
    circle.rotate(45.0);
    circle.draw();

    // 测试完整变换
    println!("\nApplying full transformation:");
    circle.render_with_full_transform(5.0, 5.0, 0.8, 30.0);

    // 测试圆与圆的交互
    println!("\nTesting circle interactions:");
    let another_circle = Circle::new(25.0, 30.0, 4.0);
    println!("Distance between circles: {:.2}", circle.distance_to(&another_circle));
    println!("Circles are overlapping: {}", circle.is_overlapping(&another_circle));

    // 使用超特征约束的泛型函数
    fn process_advanced_shape<T: AdvancedShape>(shape: &mut T) {
        println!("\nProcessing advanced shape:");
        shape.draw();
        shape.translate(5.0, 5.0);
        shape.scale(0.8);
        shape.rotate(15.0);
        shape.draw_detailed(2);
    }

    process_advanced_shape(&mut circle);

    // 测试克隆和比较功能
    println!("\nTesting clone and comparison:");
    let circle_clone = circle.clone();
    println!("Original circle: {:.1?}", circle);
    println!("Cloned circle: {:.1?}", circle_clone);
    println!("Circles are equal: {}", circle == circle_clone);

    // 展示特征对象的局限性（不能直接使用AdvancedShape作为特征对象，因为它需要Sized）
    // 但我们可以创建一个函数来接受任何AdvancedShape实现者
    fn render_any_shape<T: AdvancedShape>(shape: &T) {
        println!("\nRendering any shape:");
        shape.draw_detailed(1);
        println!("Shape area: {:.2}", shape.area());
    }

    render_any_shape(&circle);
}