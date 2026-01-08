//! # 第七部分：实际应用案例
//!
//! 所有权系统在实际项目中的应用示例。

use std::cell::RefCell;
use std::rc::Rc;

/// ## 第七部分：实际应用案例
///
/// 所有权系统在实际项目中的应用示例。
pub fn practical_ownership_examples() {
  println!("\n=== 第七部分：实际应用案例 ===");

  data_structure_examples();
  async_programming_ownership();
  error_handling_patterns();
  performance_optimization_cases();
}

/// ### 7.1 数据结构实现
///
/// 使用所有权系统实现常见数据结构。
fn data_structure_examples() {
  println!("\n--- 7.1 数据结构实现 ---");

  println!("\n🔍 链表实现：");

  #[derive(Debug)]
  struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
  }

  #[derive(Debug)]
  struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
  }

  impl<T> LinkedList<T> {
    fn new() -> Self {
      LinkedList { head: None }
    }

    fn push(&mut self, data: T) {
      let new_node = Box::new(Node {
        data,
        next: self.head.take(),
      });
      self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
      self.head.take().map(|node| {
        self.head = node.next;
        node.data
      })
    }
  }

  let mut list = LinkedList::new();
  list.push(1);
  list.push(2);
  list.push(3);

  println!("链表: {:?}", list);
  println!("弹出: {:?}", list.pop());
  println!("弹出后: {:?}", list);

  println!("\n🔍 二叉树实现：");

  #[derive(Debug)]
  struct BinaryTree<T> {
    root: Option<Box<TreeNode<T>>>,
  }

  #[derive(Debug)]
  struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
  }

  impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
      BinaryTree { root: None }
    }

    fn insert(&mut self, value: T) {
      self.root = Self::insert_node(self.root.take(), value);
    }

    fn insert_node(node: Option<Box<TreeNode<T>>>, value: T) -> Option<Box<TreeNode<T>>> {
      match node {
        None => Some(Box::new(TreeNode {
          value,
          left: None,
          right: None,
        })),
        Some(mut node) => {
          if value <= node.value {
            node.left = Self::insert_node(node.left.take(), value);
          } else {
            node.right = Self::insert_node(node.right.take(), value);
          }
          Some(node)
        }
      }
    }
  }

  let mut tree = BinaryTree::new();
  tree.insert(5);
  tree.insert(3);
  tree.insert(7);
  tree.insert(1);
  tree.insert(9);

  println!("二叉树: {:?}", tree);

  println!("\n💡 数据结构要点：");
  println!("• Box 用于递归数据结构");
  println!("• Option 表示可能为空的节点");
  println!("• take() 方法转移所有权");
  println!("• 所有权确保内存安全");
}

/// ### 7.2 异步编程中的所有权
///
/// 异步编程场景下的所有权处理。
fn async_programming_ownership() {
  println!("\n--- 7.2 异步编程中的所有权 ---");

  println!("\n🔍 数据共享模式：");

  // 模拟异步任务的数据共享
  let shared_counter = Rc::new(RefCell::new(0));

  // 创建多个"任务"（这里用闭包模拟）
  let tasks: Vec<Box<dyn Fn()>> = (0..3)
    .map(|i| {
      let counter = Rc::clone(&shared_counter);
      Box::new(move || {
        let mut count = counter.borrow_mut();
        *count += 1;
        println!("任务 {} 执行，计数器: {}", i, *count);
      }) as Box<dyn Fn()>
    })
    .collect();

  // 执行所有任务
  for task in tasks {
    task();
  }

  println!("最终计数器值: {}", shared_counter.borrow());

  println!("\n🔍 消息传递模式：");

  // 模拟消息传递
  #[derive(Debug, Clone)]
  #[allow(dead_code)]
  struct Message {
    id: u32,
    content: String,
  }

  let messages = vec![
    Message {
      id: 1,
      content: "Hello".to_string(),
    },
    Message {
      id: 2,
      content: "World".to_string(),
    },
    Message {
      id: 3,
      content: "Rust".to_string(),
    },
  ];

  // 处理消息的闭包
  let process_message = |msg: Message| {
    println!("处理消息 {}: {}", msg.id, msg.content);
    msg // 返回消息的所有权
  };

  let processed_messages: Vec<Message> = messages.into_iter().map(process_message).collect();

  println!("处理后的消息: {:?}", processed_messages);

  println!("\n💡 异步编程要点：");
  println!("• Rc<RefCell<T>> 用于单线程异步数据共享");
  println!("• Arc<Mutex<T>> 用于多线程异步数据共享");
  println!("• 消息传递避免共享状态");
  println!("• 所有权转移确保数据安全");
}

/// ### 7.3 错误处理模式
///
/// 所有权系统在错误处理中的应用。
fn error_handling_patterns() {
  println!("\n--- 7.3 错误处理模式 ---");

  println!("\n🔍 Result 类型与所有权：");

  // 自定义错误类型
  #[derive(Debug)]
  enum ParseError {
    InvalidFormat,
    OutOfRange,
  }

  // ✅ 优化：使用 &str 切片，避免 String 分配
  fn parse_number_from_str(s: &str) -> Result<i32, ParseError> {
    if s.is_empty() {
      return Err(ParseError::InvalidFormat);
    }
    match s.parse::<i32>() {
      Ok(n) if n >= 0 && n <= 100 => Ok(n),
      Ok(_) => Err(ParseError::OutOfRange),
      Err(_) => Err(ParseError::InvalidFormat),
    }
  }

  let inputs = vec!["42", "150", "abc", ""];

  // ✅ 优化：使用引用迭代，避免克隆
  for input in &inputs {
    match parse_number_from_str(input) {
      Ok(n) => println!("解析成功: {} -> {}", input, n),
      Err(e) => println!("解析失败: {} -> {:?}", input, e),
    }
  }

  println!("\n🔍 错误传播与所有权：");

  // ✅ 优化：使用 &[&str] 切片，避免 String 分配
  fn process_data_optimized(data: &[&str]) -> Result<Vec<i32>, ParseError> {
    data.iter().map(|s| parse_number_from_str(s)).collect()
  }

  // 测试数据
  let test_data = ["1", "2", "3"];
  match process_data_optimized(&test_data) {
    Ok(numbers) => println!("处理成功: {:?}", numbers),
    Err(e) => println!("处理失败: {:?}", e),
  }

  println!("\n💡 错误处理要点：");
  println!("• Result 类型转移错误和成功值的所有权");
  println!("• ? 操作符简化错误传播");
  println!("• 错误类型设计要考虑所有权");
  println!("• 避免不必要的克隆");
}

/// ### 7.4 性能优化案例
///
/// 利用所有权系统进行性能优化。
fn performance_optimization_cases() {
  println!("\n--- 7.4 性能优化案例 ---");

  println!("\n🔍 零拷贝字符串处理：");

  // 避免不必要的字符串分配
  fn process_lines(text: &str) -> Vec<&str> {
    text
      .lines()
      .filter(|line| !line.is_empty())
      .filter(|line| !line.starts_with('#'))
      .collect()
  }

  let input = "line1\n# comment\nline2\n\nline3";
  let processed = process_lines(input);
  println!("处理后的行: {:?}", processed);

  println!("\n🔍 内存池模式：");

  // 简单的对象池
  struct ObjectPool<T> {
    objects: RefCell<Vec<T>>,
    factory: Box<dyn Fn() -> T>,
  }

  impl<T> ObjectPool<T> {
    fn new<F>(factory: F) -> Self
    where
      F: Fn() -> T + 'static,
    {
      ObjectPool {
        objects: RefCell::new(Vec::new()),
        factory: Box::new(factory),
      }
    }

    fn get(&self) -> T {
      self
        .objects
        .borrow_mut()
        .pop()
        .unwrap_or_else(|| (self.factory)())
    }

    fn return_object(&self, obj: T) {
      self.objects.borrow_mut().push(obj);
    }
  }

  let pool = ObjectPool::new(|| Vec::<i32>::with_capacity(100));

  // 使用对象池
  let mut vec1 = pool.get();
  vec1.extend(1..=10);
  println!("使用向量: {:?}", vec1);

  vec1.clear();
  pool.return_object(vec1);

  let vec2 = pool.get(); // 重用之前的向量
  println!("重用向量容量: {}", vec2.capacity());

  println!("\n💡 性能优化要点：");
  println!("• 使用引用避免不必要的拷贝");
  println!("• 对象池减少内存分配");
  println!("• 所有权转移比引用计数更高效");
  println!("• 编译时优化胜过运行时检查");
}
