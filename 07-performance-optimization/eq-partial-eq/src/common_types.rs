//! # 常见类型模块
//!
//! 本模块演示常见 Rust 类型的 PartialEq 和 Eq 实现行为

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Arc;

/// 演示常见类型的行为
pub fn demonstrate_common_types() {
    println!("\n🔸 常见类型的 PartialEq/Eq 行为:");
    
    primitive_types();
    string_types();
    collection_types();
    smart_pointer_types();
    option_result_types();
    tuple_array_types();
}

/// 基本类型
fn primitive_types() {
    println!("\n  📌 基本类型:");
    
    // 整数类型 - 实现 Eq
    println!("\n    🔹 整数类型 (实现 Eq):");
    let a: i32 = 42;
    let b: i32 = 42;
    let _c: i64 = 42;
    
    println!("      {} == {} : {}", a, b, a == b);
    println!("      自反性: {} == {} : {}", a, a, a == a);
    // println!("      不同类型: {} == {} : {}", a, c, a == c); // 编译错误
    println!("      不同整数类型需要显式转换");
    
    // 浮点类型 - 只实现 PartialEq
    println!("\n    🔹 浮点类型 (只实现 PartialEq):");
    let f1 = 1.0f64;
    let f2 = 1.0f64;
    let nan = f64::NAN;
    
    println!("      {} == {} : {}", f1, f2, f1 == f2);
    println!("      NaN == NaN : {} (违反自反性)", nan == nan);
    println!("      {} == NaN : {}", f1, f1 == nan);
    
    // 特殊浮点值
    let inf = f64::INFINITY;
    let neg_inf = f64::NEG_INFINITY;
    println!("      INFINITY == INFINITY : {}", inf == inf);
    println!("      INFINITY == NEG_INFINITY : {}", inf == neg_inf);
    
    // 布尔类型 - 实现 Eq
    println!("\n    🔹 布尔类型 (实现 Eq):");
    let t = true;
    let f = false;
    println!("      {} == {} : {}", t, t, t == t);
    println!("      {} == {} : {}", t, f, t == f);
    
    // 字符类型 - 实现 Eq
    println!("\n    🔹 字符类型 (实现 Eq):");
    let ch1 = 'a';
    let ch2 = 'a';
    let ch3 = 'A';
    println!("      '{}' == '{}' : {}", ch1, ch2, ch1 == ch2);
    println!("      '{}' == '{}' : {}", ch1, ch3, ch1 == ch3);
}

/// 字符串类型
fn string_types() {
    println!("\n  📌 字符串类型:");
    
    // String 和 &str
    println!("\n    🔹 String 和 &str (实现 Eq):");
    let string = String::from("hello");
    let str_slice = "hello";
    let another_string = String::from("hello");
    
    println!("      String == String : {} == {} : {}", 
             string, another_string, string == another_string);
    println!("      String == &str : \"{}\" == \"{}\" : {}", 
             string, str_slice, string == str_slice);
    println!("      &str == String : \"{}\" == \"{}\" : {}", 
             str_slice, string, str_slice == string);
    
    // 字符串比较是按字节进行的
    let s1 = "café";
    let s2 = "cafe\u{0301}";  // 使用组合字符
    println!("\n    🔹 Unicode 字符串比较:");
    println!("      \"{}\" == \"{}\" : {} (不同的 Unicode 表示)", 
             s1, s2, s1 == s2);
    println!("      字节长度: {} vs {}", s1.len(), s2.len());
    
    // 空字符串
    let empty1 = String::new();
    let empty2 = "";
    println!("\n    🔹 空字符串:");
    println!("      String::new() == \"\" : {}", empty1 == empty2);
}

/// 集合类型
fn collection_types() {
    println!("\n  📌 集合类型:");
    
    // Vec - 实现 Eq (如果元素实现 Eq)
    println!("\n    🔹 Vec<T> (T: Eq 时实现 Eq):");
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![1, 2, 3];
    let vec3 = vec![3, 2, 1];
    
    println!("      {:?} == {:?} : {}", vec1, vec2, vec1 == vec2);
    println!("      {:?} == {:?} : {} (顺序重要)", vec1, vec3, vec1 == vec3);
    
    // 不同长度
    let vec4 = vec![1, 2];
    println!("      {:?} == {:?} : {} (长度不同)", vec1, vec4, vec1 == vec4);
    
    // HashMap - 实现 PartialEq (但不实现 Eq，因为迭代顺序不确定)
    println!("\n    🔹 HashMap<K, V> (实现 PartialEq，不实现 Eq):");
    let mut map1: HashMap<i32, &str> = HashMap::new();
    map1.insert(1, "one");
    map1.insert(2, "two");
    
    let mut map2: HashMap<i32, &str> = HashMap::new();
    map2.insert(2, "two");
    map2.insert(1, "one");  // 不同的插入顺序
    
    println!("      {:?} == {:?} : {} (忽略顺序)", map1, map2, map1 == map2);
    
    // HashSet - 类似 HashMap
    println!("\n    🔹 HashSet<T> (实现 PartialEq，不实现 Eq):");
    let mut set1: HashSet<i32> = HashSet::new();
    set1.insert(1);
    set1.insert(2);
    
    let mut set2: HashSet<i32> = HashSet::new();
    set2.insert(2);
    set2.insert(1);
    
    println!("      {:?} == {:?} : {} (忽略顺序)", set1, set2, set1 == set2);
    
    // 数组 - 实现 Eq (如果元素实现 Eq)
    println!("\n    🔹 [T; N] (T: Eq 时实现 Eq):");
    let arr1 = [1, 2, 3];
    let arr2 = [1, 2, 3];
    let arr3 = [3, 2, 1];
    
    println!("      {:?} == {:?} : {}", arr1, arr2, arr1 == arr2);
    println!("      {:?} == {:?} : {}", arr1, arr3, arr1 == arr3);
}

/// 智能指针类型
fn smart_pointer_types() {
    println!("\n  📌 智能指针类型:");
    
    // Box<T> - 实现 Eq (如果 T 实现 Eq)
    println!("\n    🔹 Box<T> (T: Eq 时实现 Eq):");
    let box1 = Box::new(42);
    let box2 = Box::new(42);
    let box3 = Box::new(24);
    
    println!("      {:?} == {:?} : {}", box1, box2, box1 == box2);
    println!("      {:?} == {:?} : {}", box1, box3, box1 == box3);
    println!("      比较的是内容，不是指针地址");
    
    // Rc<T> - 实现 Eq (如果 T 实现 Eq)
    println!("\n    🔹 Rc<T> (T: Eq 时实现 Eq):");
    let rc1 = Rc::new(String::from("hello"));
    let rc2 = Rc::clone(&rc1);  // 相同的数据
    let rc3 = Rc::new(String::from("hello"));  // 不同的 Rc，相同的内容
    
    println!("      {:?} == {:?} : {} (相同 Rc)", rc1, rc2, rc1 == rc2);
    println!("      {:?} == {:?} : {} (不同 Rc，相同内容)", rc1, rc3, rc1 == rc3);
    
    // Arc<T> - 类似 Rc<T>
    println!("\n    🔹 Arc<T> (T: Eq 时实现 Eq):");
    let arc1 = Arc::new(42);
    let arc2 = Arc::clone(&arc1);
    let arc3 = Arc::new(42);
    
    println!("      {:?} == {:?} : {} (相同 Arc)", arc1, arc2, arc1 == arc2);
    println!("      {:?} == {:?} : {} (不同 Arc，相同内容)", arc1, arc3, arc1 == arc3);
}

/// Option 和 Result 类型
fn option_result_types() {
    println!("\n  📌 Option 和 Result 类型:");
    
    // Option<T> - 实现 Eq (如果 T 实现 Eq)
    println!("\n    🔹 Option<T> (T: Eq 时实现 Eq):");
    let some1 = Some(42);
    let some2 = Some(42);
    let some3 = Some(24);
    let none1: Option<i32> = None;
    let none2: Option<i32> = None;
    
    println!("      {:?} == {:?} : {}", some1, some2, some1 == some2);
    println!("      {:?} == {:?} : {}", some1, some3, some1 == some3);
    println!("      {:?} == {:?} : {}", some1, none1, some1 == none1);
    println!("      {:?} == {:?} : {}", none1, none2, none1 == none2);
    
    // Result<T, E> - 实现 Eq (如果 T 和 E 都实现 Eq)
    println!("\n    🔹 Result<T, E> (T: Eq, E: Eq 时实现 Eq):");
    let ok1: Result<i32, &str> = Ok(42);
    let ok2: Result<i32, &str> = Ok(42);
    let err1: Result<i32, &str> = Err("error");
    let err2: Result<i32, &str> = Err("error");
    let err3: Result<i32, &str> = Err("different error");
    
    println!("      {:?} == {:?} : {}", ok1, ok2, ok1 == ok2);
    println!("      {:?} == {:?} : {}", err1, err2, err1 == err2);
    println!("      {:?} == {:?} : {}", err1, err3, err1 == err3);
    println!("      {:?} == {:?} : {}", ok1, err1, ok1 == err1);
}

/// 元组和数组类型
fn tuple_array_types() {
    println!("\n  📌 元组和数组类型:");
    
    // 元组 - 实现 Eq (如果所有元素都实现 Eq)
    println!("\n    🔹 元组 (所有元素 Eq 时实现 Eq):");
    let tuple1 = (1, "hello", true);
    let tuple2 = (1, "hello", true);
    let tuple3 = (1, "world", true);
    
    println!("      {:?} == {:?} : {}", tuple1, tuple2, tuple1 == tuple2);
    println!("      {:?} == {:?} : {}", tuple1, tuple3, tuple1 == tuple3);
    
    // 嵌套元组
    let nested1 = ((1, 2), (3, 4));
    let nested2 = ((1, 2), (3, 4));
    let nested3 = ((1, 2), (4, 3));
    
    println!("\n    🔹 嵌套元组:");
    println!("      {:?} == {:?} : {}", nested1, nested2, nested1 == nested2);
    println!("      {:?} == {:?} : {}", nested1, nested3, nested1 == nested3);
    
    // 单元类型
    println!("\n    🔹 单元类型 () (实现 Eq):");
    let unit1 = ();
    let unit2 = ();
    println!("      {:?} == {:?} : {}", unit1, unit2, unit1 == unit2);
    
    // 切片比较
    println!("\n    🔹 切片 &[T] (T: Eq 时实现 Eq):");
    let slice1 = &[1, 2, 3][..];
    let slice2 = &[1, 2, 3][..];
    let slice3 = &[1, 2, 3, 4][..3];  // 前三个元素
    
    println!("      {:?} == {:?} : {}", slice1, slice2, slice1 == slice2);
    println!("      {:?} == {:?} : {}", slice1, slice3, slice1 == slice3);
}

/// 演示类型约束的影响
pub fn demonstrate_type_constraints() {
    println!("\n🔸 类型约束的影响:");
    
    println!("\n  📌 泛型约束:");
    
    // 函数需要 PartialEq
    fn compare_values<T: PartialEq>(a: &T, b: &T) -> bool {
        a == b
    }
    
    println!("    🔹 PartialEq 约束的函数:");
    println!("      compare_values(&42, &42) : {}", compare_values(&42, &42));
    println!("      compare_values(&1.0, &1.0) : {}", compare_values(&1.0, &1.0));
    println!("      compare_values(&\"hello\", &\"hello\") : {}", 
             compare_values(&"hello", &"hello"));
    
    // 函数需要 Eq (用于 HashMap)
    fn create_lookup<T: Eq + std::hash::Hash + Clone>(items: Vec<T>) -> HashMap<T, usize> {
        let mut map = HashMap::new();
        for (index, item) in items.into_iter().enumerate() {
            map.insert(item, index);
        }
        map
    }
    
    println!("\n    🔹 Eq + Hash 约束的函数:");
    let int_lookup = create_lookup(vec![1, 2, 3, 2]);
    println!("      整数查找表: {:?}", int_lookup);
    
    let string_lookup = create_lookup(vec!["a".to_string(), "b".to_string(), "a".to_string()]);
    println!("      字符串查找表: {:?}", string_lookup);
    
    // 浮点数不能用于此函数
    // let float_lookup = create_lookup(vec![1.0, 2.0, 3.0]); // 编译错误!
    println!("      浮点数不能用于此函数 (不实现 Eq)");
}