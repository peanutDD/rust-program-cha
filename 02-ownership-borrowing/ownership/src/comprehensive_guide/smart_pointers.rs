//! # 第六部分：智能指针与所有权
//!
//! 智能指针提供了额外的功能和保证，扩展了所有权系统。

use std::cell::RefCell;
use std::rc::Rc;

/// ## 第六部分：智能指针与所有权
///
/// 智能指针提供了额外的功能和保证，扩展了所有权系统。
pub fn smart_pointers_and_ownership() {
  println!("\n=== 第六部分：智能指针与所有权 ===");

  box_pointer_explanation();
  rc_pointer_explanation();
  refcell_interior_mutability();
  smart_pointer_combinations();
}

/// ### 6.1 Box<T> 智能指针
///
/// Box 提供堆上数据的所有权。
fn box_pointer_explanation() {
  println!("\n--- 6.1 Box<T> 智能指针 ---");

  println!("\n🔍 Box 的基本用法：");

  // 在堆上存储数据
  let b = Box::new(5);
  println!("Box 中的值: {}", b);

  // Box 实现了 Deref，可以像引用一样使用
  let x = 5;
  let y = Box::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y); // 解引用 Box
  println!("Box 解引用: x={}, *y={}", x, *y);

  println!("\n🔍 递归类型与 Box：");

  // 使用 Box 创建递归类型
  #[derive(Debug)]
  #[allow(dead_code)]
  enum List {
    Cons(i32, Box<List>),
    Nil,
  }

  use List::{Cons, Nil};

  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  println!("递归列表: {:?}", list);

  println!("\n🔍 Box 的所有权转移：");

  let boxed_value = Box::new(String::from("boxed string"));
  let moved_box = boxed_value; // Box 的所有权转移
  // println!("{}", boxed_value);  // 编译错误！
  println!("移动后的 Box: {}", moved_box);

  println!("\n💡 Box 特点：");
  println!("• 在堆上分配数据");
  println!("• 编译时已知大小");
  println!("• 实现 Deref 和 Drop trait");
  println!("• 用于递归类型和大型数据");
}

/// ### 6.2 Rc<T> 引用计数指针
///
/// Rc 允许多个所有者共享数据。
fn rc_pointer_explanation() {
  println!("\n--- 6.2 Rc<T> 引用计数指针 ---");

  println!("\n🔍 Rc 的基本用法：");

  let data = Rc::new(String::from("shared data"));
  println!("创建 Rc，引用计数: {}", Rc::strong_count(&data));

  let data2 = Rc::clone(&data); // 增加引用计数
  println!("克隆后，引用计数: {}", Rc::strong_count(&data));

  {
    let data3 = Rc::clone(&data); // 再次增加引用计数
    println!("内层作用域，引用计数: {}", Rc::strong_count(&data));
    println!("所有引用的值: {}, {}, {}", data, data2, data3);
  } // data3 离开作用域，引用计数减少

  println!("离开内层作用域，引用计数: {}", Rc::strong_count(&data));

  println!("\n🔍 Rc 与链表：");

  #[derive(Debug)]
  #[allow(dead_code)]
  enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
  }

  use RcList::{Cons as RcCons, Nil as RcNil};

  let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
  println!("a 的引用计数: {}", Rc::strong_count(&a));

  let b = RcCons(3, Rc::clone(&a));
  println!("创建 b 后，a 的引用计数: {}", Rc::strong_count(&a));

  let c = RcCons(4, Rc::clone(&a));
  println!("创建 c 后，a 的引用计数: {}", Rc::strong_count(&a));

  println!("列表 a: {:?}", a);
  println!("列表 b: {:?}", b);
  println!("列表 c: {:?}", c);

  println!("\n💡 Rc 特点：");
  println!("• 允许多个所有者");
  println!("• 只能用于单线程");
  println!("• 数据是不可变的");
  println!("• 运行时引用计数");
}

/// ### 6.3 RefCell<T> 内部可变性
///
/// RefCell 提供运行时借用检查的内部可变性。
fn refcell_interior_mutability() {
  println!("\n--- 6.3 RefCell<T> 内部可变性 ---");

  println!("\n🔍 RefCell 基本用法：");

  let data = RefCell::new(5);

  // 不可变借用
  {
    let borrowed = data.borrow();
    println!("不可变借用: {}", *borrowed);
  } // 借用在这里结束

  // 可变借用
  {
    let mut borrowed_mut = data.borrow_mut();
    *borrowed_mut += 10;
    println!("可变借用修改后: {}", *borrowed_mut);
  } // 可变借用在这里结束

  println!("最终值: {}", data.borrow());

  println!("\n🔍 RefCell 与 Rc 结合：");

  let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));
  let data1 = Rc::clone(&shared_data);
  let data2 = Rc::clone(&shared_data);

  // 通过第一个引用修改数据
  data1.borrow_mut().push(4);
  println!("通过 data1 修改后: {:?}", data1.borrow());

  // 通过第二个引用修改数据
  data2.borrow_mut().push(5);
  println!("通过 data2 修改后: {:?}", data2.borrow());

  // 原始引用也能看到变化
  println!("原始 shared_data: {:?}", shared_data.borrow());

  println!("\n🔍 运行时借用检查：");

  let cell = RefCell::new(String::from("hello"));

  // 正确的借用模式
  {
    let borrow1 = cell.borrow();
    let borrow2 = cell.borrow(); // 多个不可变借用是允许的
    println!("多个不可变借用: {}, {}", borrow1, borrow2);
  }

  {
    let mut borrow_mut = cell.borrow_mut();
    borrow_mut.push_str(", world");
    println!("可变借用: {}", borrow_mut);
    // let borrow = cell.borrow();  // 运行时 panic！不能同时有可变和不可变借用
  }

  println!("\n💡 RefCell 特点：");
  println!("• 运行时借用检查");
  println!("• 内部可变性模式");
  println!("• 违反借用规则会导致 panic");
  println!("• 常与 Rc 结合使用");
}

/// ### 6.4 智能指针组合
///
/// 不同智能指针的组合使用模式。
fn smart_pointer_combinations() {
  println!("\n--- 6.4 智能指针组合 ---");

  println!("\n🔍 Rc<RefCell<T>> 模式：");

  #[derive(Debug)]
  #[allow(dead_code)]
  struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
  }

  let leaf = Rc::new(Node {
    value: 3,
    children: RefCell::new(vec![]),
  });

  let branch = Rc::new(Node {
    value: 5,
    children: RefCell::new(vec![Rc::clone(&leaf)]),
  });

  println!("叶子节点: {:?}", leaf);
  println!("分支节点: {:?}", branch);

  // 修改子节点列表
  leaf.children.borrow_mut().push(Rc::clone(&branch));
  println!("修改后的叶子节点引用计数: {}", Rc::strong_count(&leaf));

  println!("\n🔍 Box<dyn Trait> 模式：");

  trait Draw {
    fn draw(&self);
  }

  struct Circle {
    radius: f64,
  }

  struct Rectangle {
    width: f64,
    height: f64,
  }

  impl Draw for Circle {
    fn draw(&self) {
      println!("绘制圆形，半径: {}", self.radius);
    }
  }

  impl Draw for Rectangle {
    fn draw(&self) {
      println!("绘制矩形，宽: {}, 高: {}", self.width, self.height);
    }
  }

  let shapes: Vec<Box<dyn Draw>> = vec![
    Box::new(Circle { radius: 5.0 }),
    Box::new(Rectangle {
      width: 10.0,
      height: 20.0,
    }),
  ];

  for shape in shapes {
    shape.draw();
  }

  println!("\n💡 组合模式总结：");
  println!("• Rc<RefCell<T>>: 多所有者 + 内部可变性");
  println!("• Box<dyn Trait>: 堆分配 + trait 对象");
  println!("• 选择合适的智能指针组合");
  println!("• 注意循环引用问题");
}
