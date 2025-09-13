//! # 实际案例模块
//! 
//! 本模块通过复杂的实际场景展示作用域、生命周期和NLL的应用，
//! 包括数据结构设计、异步编程、错误处理、性能优化等真实世界的问题。

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// 运行所有实际案例
pub fn run_practical_examples() {
    println!("\n🚀 实际案例模块");
    println!("通过复杂的实际场景展示作用域、生命周期和NLL的应用");
    
    data_structure_examples();
    async_programming_examples();
    error_handling_examples();
    performance_optimization_examples();
    concurrent_programming_examples();
    web_development_examples();
    game_development_examples();
    system_programming_examples();
}

/// 1. 数据结构设计案例
fn data_structure_examples() {
    println!("\n📊 1. 数据结构设计案例");
    println!("展示在复杂数据结构中如何处理作用域、生命周期和NLL问题。");
    
    linked_list_example();
    tree_structure_example();
    graph_structure_example();
}

/// 链表实现案例
fn linked_list_example() {
    println!("\n🔗 链表实现案例:");
    
    // 使用生命周期参数的链表节点
    #[derive(Debug)]
    struct Node<'a, T> {
        data: T,
        next: Option<&'a Node<'a, T>>,
    }
    
    impl<'a, T> Node<'a, T> {
        fn new(data: T) -> Self {
            Node { data, next: None }
        }
        
        // 生命周期确保引用的安全性
        fn set_next(&mut self, next: &'a Node<'a, T>) {
            self.next = Some(next);
        }
        
        // NLL允许更灵活的借用
        fn traverse(&self) -> Vec<&T> {
            let mut result = Vec::new();
            let mut current = Some(self);
            
            while let Some(node) = current {
                result.push(&node.data);
                current = node.next;
            }
            
            result
        }
    }
    
    // 演示链表的使用
    {
        let _node1 = Node::new("第一个节点");
        let node2 = Node::new("第二个节点");
        let mut node3 = Node::new("第三个节点");
        
        // 作用域确保所有节点在使用期间都有效
        node3.set_next(&node2);
        
        let values = node3.traverse();
        println!("   链表遍历结果: {:?}", values);
        
        // 所有节点在作用域结束时自动清理
    }
    
    println!("   ✅ 链表案例展示了生命周期如何确保引用安全");
}

/// 树结构案例
fn tree_structure_example() {
    println!("\n🌳 树结构案例:");
    
    // 使用Rc和RefCell实现可共享的树节点
    #[derive(Debug)]
    struct TreeNode {
        value: i32,
        children: RefCell<Vec<Rc<TreeNode>>>,
    }
    
    impl TreeNode {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(TreeNode {
                value,
                children: RefCell::new(Vec::new()),
            })
        }
        
        fn add_child(&self, child: Rc<TreeNode>) {
            // NLL使得借用检查更加智能
            self.children.borrow_mut().push(child);
        }
        
        // 深度优先遍历
        fn dfs_traverse(&self) -> Vec<i32> {
            let mut result = vec![self.value];
            
            // NLL允许在借用后继续使用self
            for child in self.children.borrow().iter() {
                result.extend(child.dfs_traverse());
            }
            
            result
        }
    }
    
    // 演示树的构建和遍历
    {
        let root = TreeNode::new(1);
        let child1 = TreeNode::new(2);
        let child2 = TreeNode::new(3);
        let grandchild = TreeNode::new(4);
        
        // 构建树结构
        child1.add_child(grandchild);
        root.add_child(child1);
        root.add_child(child2);
        
        let traversal = root.dfs_traverse();
        println!("   树的DFS遍历: {:?}", traversal);
        
        // Rc确保节点在需要时保持有效
    }
    
    println!("   ✅ 树结构案例展示了智能指针与生命周期的结合");
}

/// 图结构案例
fn graph_structure_example() {
    println!("\n🕸️ 图结构案例:");
    
    // 使用HashMap和Vec实现图
    struct Graph<'a> {
        nodes: HashMap<&'a str, Vec<&'a str>>,
    }
    
    impl<'a> Graph<'a> {
        fn new() -> Self {
            Graph {
                nodes: HashMap::new(),
            }
        }
        
        fn add_edge(&mut self, from: &'a str, to: &'a str) {
            // NLL使得可变借用更加灵活
            self.nodes.entry(from).or_insert_with(Vec::new).push(to);
        }
        
        // 广度优先搜索
        fn bfs<'b>(&self, start: &'b str) -> Vec<&'b str> 
        where
            'a: 'b,
        {
            use std::collections::VecDeque;
            
            let mut visited = std::collections::HashSet::new();
            let mut queue = VecDeque::new();
            let mut result = Vec::new();
            
            queue.push_back(start);
            visited.insert(start);
            
            while let Some(current) = queue.pop_front() {
                result.push(current);
                
                // NLL允许在借用后继续使用self
                if let Some(neighbors) = self.nodes.get(current) {
                    for &neighbor in neighbors {
                        if !visited.contains(neighbor) {
                            visited.insert(neighbor);
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
            
            result
        }
    }
    
    // 演示图的使用
    {
        let mut graph = Graph::new();
        
        // 添加边
        graph.add_edge("A", "B");
        graph.add_edge("A", "C");
        graph.add_edge("B", "D");
        graph.add_edge("C", "D");
        
        let bfs_result = graph.bfs("A");
        println!("   图的BFS遍历: {:?}", bfs_result);
    }
    
    println!("   ✅ 图结构案例展示了复杂数据结构中的生命周期管理");
}

/// 2. 异步编程案例
fn async_programming_examples() {
    println!("\n⚡ 2. 异步编程案例");
    println!("展示在异步编程中如何处理生命周期和借用检查问题。");
    
    future_lifetime_example();
    async_closure_example();
    stream_processing_example();
}

/// Future生命周期案例
fn future_lifetime_example() {
    println!("\n🔮 Future生命周期案例:");
    
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    
    // 自定义Future实现
    struct DelayedValue<'a> {
        value: &'a str,
        ready: bool,
    }
    
    impl<'a> DelayedValue<'a> {
        fn new(value: &'a str) -> Self {
            DelayedValue { value, ready: false }
        }
    }
    
    impl<'a> Future for DelayedValue<'a> {
        type Output = &'a str;
        
        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.ready {
                Poll::Ready(self.value)
            } else {
                self.ready = true;
                Poll::Pending
            }
        }
    }
    
    // 演示Future的使用
    {
        let data = "异步数据";
        let _future = DelayedValue::new(data);
        
        // 在实际应用中，这里会使用async runtime
        println!("   创建了Future，数据: {}", data);
        println!("   ✅ Future生命周期确保数据在异步操作期间有效");
    }
}

/// 异步闭包案例
fn async_closure_example() {
    println!("\n🔒 异步闭包案例:");
    
    // 模拟异步操作
    fn simulate_async_operation<F>(callback: F) 
    where
        F: FnOnce() -> String,
    {
        // 在实际应用中，这里会是真正的异步操作
        let result = callback();
        println!("   异步操作结果: {}", result);
    }
    
    // 演示闭包捕获和生命周期
    {
        let data = String::from("重要数据");
        
        // NLL允许更灵活的闭包使用
        simulate_async_operation(|| {
            format!("处理: {}", data)
        });
        
        // NLL知道data在闭包执行后仍然可用
        println!("   原始数据仍然有效: {}", data);
    }
    
    println!("   ✅ 异步闭包案例展示了NLL在异步编程中的优势");
}

/// 流处理案例
fn stream_processing_example() {
    println!("\n🌊 流处理案例:");
    
    // 简化的流处理器
    struct StreamProcessor<'a> {
        data: &'a [i32],
        position: usize,
    }
    
    impl<'a> StreamProcessor<'a> {
        fn new(data: &'a [i32]) -> Self {
            StreamProcessor { data, position: 0 }
        }
        
        // 处理下一个元素
        fn process_next(&mut self) -> Option<i32> {
            if self.position < self.data.len() {
                let value = self.data[self.position];
                self.position += 1;
                Some(value * 2) // 简单的处理逻辑
            } else {
                None
            }
        }
        
        // 批量处理
        fn process_batch(&mut self, count: usize) -> Vec<i32> {
            let mut results = Vec::new();
            
            for _ in 0..count {
                if let Some(result) = self.process_next() {
                    results.push(result);
                } else {
                    break;
                }
            }
            
            results
        }
    }
    
    // 演示流处理
    {
        let stream_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut processor = StreamProcessor::new(&stream_data);
        
        // 批量处理
        let batch1 = processor.process_batch(3);
        println!("   第一批处理结果: {:?}", batch1);
        
        let batch2 = processor.process_batch(3);
        println!("   第二批处理结果: {:?}", batch2);
        
        // 生命周期确保stream_data在处理期间有效
    }
    
    println!("   ✅ 流处理案例展示了生命周期在数据流中的重要性");
}

/// 3. 错误处理案例
fn error_handling_examples() {
    println!("\n❌ 3. 错误处理案例");
    println!("展示在错误处理中如何利用作用域和生命周期确保资源安全。");
    
    resource_management_example();
    error_propagation_example();
    recovery_strategy_example();
}

/// 资源管理案例
fn resource_management_example() {
    println!("\n🛡️ 资源管理案例:");
    
    // 模拟资源类型
    struct Resource {
        name: String,
        is_open: bool,
    }
    
    impl Resource {
        fn new(name: &str) -> Result<Self, &'static str> {
            if name.is_empty() {
                Err("资源名称不能为空")
            } else {
                Ok(Resource {
                    name: name.to_string(),
                    is_open: true,
                })
            }
        }
        
        fn use_resource(&self) -> Result<String, &'static str> {
            if self.is_open {
                Ok(format!("使用资源: {}", self.name))
            } else {
                Err("资源已关闭")
            }
        }
    }
    
    impl Drop for Resource {
        fn drop(&mut self) {
            self.is_open = false;
            println!("   🔒 资源 '{}' 已自动释放", self.name);
        }
    }
    
    // 演示资源管理
    {
        match Resource::new("数据库连接") {
            Ok(resource) => {
                // 作用域确保资源在使用期间有效
                match resource.use_resource() {
                    Ok(result) => println!("   {}", result),
                    Err(e) => println!("   错误: {}", e),
                }
                // 资源在作用域结束时自动释放
            }
            Err(e) => println!("   创建资源失败: {}", e),
        }
    }
    
    println!("   ✅ 资源管理案例展示了作用域在错误处理中的作用");
}

/// 错误传播案例
fn error_propagation_example() {
    println!("\n📡 错误传播案例:");
    
    // 自定义错误类型
    #[derive(Debug)]
    enum ProcessingError {
        InvalidInput(String),
        ProcessingFailed(String),
        ResourceUnavailable,
    }
    
    // 多层错误处理
    fn validate_input(input: &str) -> Result<&str, ProcessingError> {
        if input.is_empty() {
            Err(ProcessingError::InvalidInput("输入不能为空".to_string()))
        } else if input.len() < 3 {
            Err(ProcessingError::InvalidInput("输入太短".to_string()))
        } else {
            Ok(input)
        }
    }
    
    fn process_data(input: &str) -> Result<String, ProcessingError> {
        let validated = validate_input(input)?; // 错误传播
        
        if validated.contains("error") {
            Err(ProcessingError::ProcessingFailed("包含错误关键字".to_string()))
        } else {
            Ok(format!("处理完成: {}", validated))
        }
    }
    
    fn handle_request(input: &str) -> Result<String, ProcessingError> {
        // NLL允许更自然的错误处理
        let result = process_data(input)?;
        Ok(format!("请求处理结果: {}", result))
    }
    
    // 演示错误传播
    let test_cases = vec![
        "valid_input",
        "error_input", 
        "ab",
        "",
    ];
    
    for input in test_cases {
        match handle_request(input) {
            Ok(result) => println!("   ✅ {}", result),
            Err(e) => println!("   ❌ 输入 '{}' 错误: {:?}", input, e),
        }
    }
    
    println!("   ✅ 错误传播案例展示了生命周期在错误处理中的应用");
}

/// 恢复策略案例
fn recovery_strategy_example() {
    println!("\n🔄 恢复策略案例:");
    
    // 带重试机制的操作
    fn retry_operation<F, T, E>(mut operation: F, max_retries: usize) -> Result<T, E>
    where
        F: FnMut() -> Result<T, E>,
    {
        let mut attempts = 0;
        
        loop {
            match operation() {
                Ok(result) => return Ok(result),
                Err(e) => {
                    attempts += 1;
                    if attempts >= max_retries {
                        return Err(e);
                    }
                    println!("   🔄 重试第 {} 次", attempts);
                }
            }
        }
    }
    
    // 模拟不稳定的操作
    let mut attempt_count = 0;
    let unstable_operation = || {
        attempt_count += 1;
        if attempt_count < 3 {
            Err("操作失败")
        } else {
            Ok("操作成功")
        }
    };
    
    // 演示重试机制
    match retry_operation(unstable_operation, 5) {
        Ok(result) => println!("   ✅ {}", result),
        Err(e) => println!("   ❌ 最终失败: {}", e),
    }
    
    println!("   ✅ 恢复策略案例展示了闭包和生命周期的结合使用");
}

/// 4. 性能优化案例
fn performance_optimization_examples() {
    println!("\n⚡ 4. 性能优化案例");
    println!("展示如何利用作用域、生命周期和NLL进行性能优化。");
    
    zero_copy_optimization();
    memory_pool_example();
    lazy_evaluation_example();
}

/// 零拷贝优化案例
fn zero_copy_optimization() {
    println!("\n📋 零拷贝优化案例:");
    
    // 零拷贝字符串处理
    fn process_string_zero_copy(input: &str) -> Vec<&str> {
        // NLL允许更高效的借用
        input
            .split_whitespace()
            .filter(|word| word.len() > 3)
            .collect()
    }
    
    // 对比：需要拷贝的版本
    fn process_string_with_copy(input: &str) -> Vec<String> {
        input
            .split_whitespace()
            .filter(|word| word.len() > 3)
            .map(|word| word.to_string()) // 额外的拷贝
            .collect()
    }
    
    // 演示性能差异
    let large_text = "这是一个用于演示零拷贝优化的长文本字符串 包含很多单词 用于测试性能差异";
    
    {
        let start = std::time::Instant::now();
        let zero_copy_result = process_string_zero_copy(large_text);
        let zero_copy_time = start.elapsed();
        
        println!("   零拷贝结果: {:?}", zero_copy_result);
        println!("   零拷贝耗时: {:?}", zero_copy_time);
    }
    
    {
        let start = std::time::Instant::now();
        let copy_result = process_string_with_copy(large_text);
        let copy_time = start.elapsed();
        
        println!("   拷贝版本结果: {:?}", copy_result);
        println!("   拷贝版本耗时: {:?}", copy_time);
    }
    
    println!("   ✅ 零拷贝优化展示了生命周期在性能优化中的价值");
}

/// 内存池案例
fn memory_pool_example() {
    println!("\n🏊 内存池案例:");
    
    // 简化的内存池实现
    struct MemoryPool<T> {
        pool: Vec<T>,
        available: Vec<usize>,
    }
    
    impl<T: Default + Clone> MemoryPool<T> {
        fn new(capacity: usize) -> Self {
            let mut pool = Vec::with_capacity(capacity);
            let mut available = Vec::with_capacity(capacity);
            
            for i in 0..capacity {
                pool.push(T::default());
                available.push(i);
            }
            
            MemoryPool { pool, available }
        }
        
        // 从池中获取对象
        fn acquire(&mut self) -> Option<PooledObject<T>> {
            if let Some(index) = self.available.pop() {
                Some(PooledObject {
                    index,
                    pool_ref: self as *mut Self,
                })
            } else {
                None
            }
        }
        
        // 归还对象到池中
        fn release(&mut self, index: usize) {
            if index < self.pool.len() {
                self.available.push(index);
            }
        }
        
        // 获取对象的可变引用
        fn get_mut(&mut self, index: usize) -> Option<&mut T> {
            self.pool.get_mut(index)
        }
    }
    
    // 池化对象包装器
    struct PooledObject<T: Default + Clone> {
        index: usize,
        pool_ref: *mut MemoryPool<T>,
    }
    
    impl<T: Default + Clone> PooledObject<T> {
        fn get_mut(&mut self) -> Option<&mut T> {
            unsafe {
                (*self.pool_ref).get_mut(self.index)
            }
        }
    }
    
    impl<T: Default + Clone> Drop for PooledObject<T> {
        fn drop(&mut self) {
            unsafe {
                (*self.pool_ref).release(self.index);
            }
            println!("   🔄 对象已归还到内存池");
        }
    }
    
    // 演示内存池使用
    {
        let mut pool: MemoryPool<Vec<i32>> = MemoryPool::new(3);
        
        {
            let mut obj1 = pool.acquire().expect("获取对象1");
            if let Some(vec) = obj1.get_mut() {
                vec.extend_from_slice(&[1, 2, 3]);
                println!("   对象1数据: {:?}", vec);
            }
            
            let mut obj2 = pool.acquire().expect("获取对象2");
            if let Some(vec) = obj2.get_mut() {
                vec.extend_from_slice(&[4, 5, 6]);
                println!("   对象2数据: {:?}", vec);
            }
            
            // 对象在作用域结束时自动归还
        }
        
        // 现在可以重新获取对象
        let mut obj3 = pool.acquire().expect("重新获取对象");
        if let Some(vec) = obj3.get_mut() {
            vec.clear();
            vec.extend_from_slice(&[7, 8, 9]);
            println!("   重用对象数据: {:?}", vec);
        }
    }
    
    println!("   ✅ 内存池案例展示了作用域在资源管理中的应用");
}

/// 惰性求值案例
fn lazy_evaluation_example() {
    println!("\n😴 惰性求值案例:");
    
    // 惰性求值结构
    struct LazyValue<F, T>
    where
        F: FnOnce() -> T,
    {
        computation: Option<F>,
        cached_value: Option<T>,
    }
    
    impl<F, T> LazyValue<F, T>
    where
        F: FnOnce() -> T,
        T: Clone,
    {
        fn new(computation: F) -> Self {
            LazyValue {
                computation: Some(computation),
                cached_value: None,
            }
        }
        
        // 惰性求值
        fn get(&mut self) -> &T {
            if self.cached_value.is_none() {
                if let Some(computation) = self.computation.take() {
                    let value = computation();
                    self.cached_value = Some(value);
                    println!("   💡 执行了惰性计算");
                }
            }
            
            self.cached_value.as_ref().unwrap()
        }
    }
    
    // 演示惰性求值
    {
        let mut expensive_computation = LazyValue::new(|| {
            println!("   🔄 执行昂贵的计算...");
            thread::sleep(Duration::from_millis(10)); // 模拟耗时操作
            42
        });
        
        println!("   创建了惰性值，但还未计算");
        
        // 第一次访问时才计算
        let result1 = expensive_computation.get();
        println!("   第一次获取结果: {}", result1);
        
        // 第二次访问使用缓存
        let result2 = expensive_computation.get();
        println!("   第二次获取结果: {} (使用缓存)", result2);
    }
    
    println!("   ✅ 惰性求值案例展示了生命周期在性能优化中的应用");
}

/// 5. 并发编程案例
fn concurrent_programming_examples() {
    println!("\n🔀 5. 并发编程案例");
    println!("展示在并发编程中如何处理生命周期和所有权问题。");
    
    thread_safety_example();
    channel_communication_example();
    shared_state_example();
}

/// 线程安全案例
fn thread_safety_example() {
    println!("\n🛡️ 线程安全案例:");
    
    // 线程安全的计数器
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    // 创建多个线程
    for i in 0..3 {
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            for j in 0..3 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
                println!("   线程 {} 第 {} 次增加，当前值: {}", i, j + 1, *num);
                
                // NLL允许在这里释放锁
                drop(num);
                
                // 模拟一些工作
                thread::sleep(Duration::from_millis(1));
            }
        });
        
        handles.push(handle);
    }
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_value = *counter.lock().unwrap();
    println!("   最终计数器值: {}", final_value);
    
    println!("   ✅ 线程安全案例展示了Arc和Mutex在并发中的使用");
}

/// 通道通信案例
fn channel_communication_example() {
    println!("\n📡 通道通信案例:");
    
    use std::sync::mpsc;
    
    // 创建通道
    let (tx, rx) = mpsc::channel();
    
    // 生产者线程
    let producer = thread::spawn(move || {
        let messages = vec!["消息1", "消息2", "消息3", "消息4", "消息5"];
        
        for (i, message) in messages.into_iter().enumerate() {
            tx.send(format!("第{}条: {}", i + 1, message)).unwrap();
            println!("   📤 发送: 第{}条消息", i + 1);
            thread::sleep(Duration::from_millis(10));
        }
        
        println!("   ✅ 生产者完成发送");
    });
    
    // 消费者线程
    let consumer = thread::spawn(move || {
        while let Ok(message) = rx.recv() {
            println!("   📥 接收: {}", message);
            thread::sleep(Duration::from_millis(5));
        }
        
        println!("   ✅ 消费者完成接收");
    });
    
    // 等待线程完成
    producer.join().unwrap();
    consumer.join().unwrap();
    
    println!("   ✅ 通道通信案例展示了所有权在线程间的转移");
}

/// 共享状态案例
fn shared_state_example() {
    println!("\n🤝 共享状态案例:");
    
    // 共享的工作队列
    let work_queue = Arc::new(Mutex::new(Vec::new()));
    let completed_work = Arc::new(Mutex::new(Vec::new()));
    
    // 添加工作项
    {
        let mut queue = work_queue.lock().unwrap();
        for i in 1..=10 {
            queue.push(format!("任务{}", i));
        }
    }
    
    let mut workers = vec![];
    
    // 创建工作线程
    for worker_id in 0..3 {
        let queue_clone = Arc::clone(&work_queue);
        let completed_clone = Arc::clone(&completed_work);
        
        let worker = thread::spawn(move || {
            loop {
                // 获取工作项
                let work_item = {
                    let mut queue = queue_clone.lock().unwrap();
                    queue.pop()
                };
                
                match work_item {
                    Some(item) => {
                        println!("   🔧 工作者 {} 处理: {}", worker_id, item);
                        
                        // 模拟工作
                        thread::sleep(Duration::from_millis(20));
                        
                        // 记录完成的工作
                        {
                            let mut completed = completed_clone.lock().unwrap();
                            completed.push(format!("工作者{}完成{}", worker_id, item));
                        }
                    }
                    None => {
                        println!("   ✅ 工作者 {} 没有更多工作，退出", worker_id);
                        break;
                    }
                }
            }
        });
        
        workers.push(worker);
    }
    
    // 等待所有工作者完成
    for worker in workers {
        worker.join().unwrap();
    }
    
    // 显示完成的工作
    let completed = completed_work.lock().unwrap();
    println!("   📋 完成的工作: {:?}", *completed);
    
    println!("   ✅ 共享状态案例展示了多线程环境下的状态管理");
}

/// 6. Web开发案例
fn web_development_examples() {
    println!("\n🌐 6. Web开发案例");
    println!("展示在Web开发中如何处理请求生命周期和资源管理。");
    
    request_handler_example();
    middleware_example();
    session_management_example();
}

/// 请求处理案例
fn request_handler_example() {
    println!("\n🔄 请求处理案例:");
    
    // 模拟HTTP请求
    #[derive(Debug)]
    struct HttpRequest<'a> {
        method: &'a str,
        path: &'a str,
        headers: HashMap<&'a str, &'a str>,
        body: Option<&'a str>,
    }
    
    impl<'a> HttpRequest<'a> {
        fn new(method: &'a str, path: &'a str) -> Self {
            HttpRequest {
                method,
                path,
                headers: HashMap::new(),
                body: None,
            }
        }
        
        fn add_header(&mut self, key: &'a str, value: &'a str) {
            self.headers.insert(key, value);
        }
        
        fn set_body(&mut self, body: &'a str) {
            self.body = Some(body);
        }
    }
    
    // 请求处理器
    fn handle_request(request: &HttpRequest) -> String {
        match (request.method, request.path) {
            ("GET", "/") => "欢迎页面".to_string(),
            ("GET", "/api/users") => {
                // NLL允许更灵活的借用
                if let Some(auth) = request.headers.get("Authorization") {
                    format!("用户列表 (认证: {})", auth)
                } else {
                    "未授权访问".to_string()
                }
            }
            ("POST", "/api/users") => {
                if let Some(body) = request.body {
                    format!("创建用户: {}", body)
                } else {
                    "缺少请求体".to_string()
                }
            }
            _ => "404 Not Found".to_string(),
        }
    }
    
    // 演示请求处理
    {
        let mut request = HttpRequest::new("GET", "/api/users");
        request.add_header("Authorization", "Bearer token123");
        
        let response = handle_request(&request);
        println!("   📥 请求: {:?}", request);
        println!("   📤 响应: {}", response);
    }
    
    {
        let mut request = HttpRequest::new("POST", "/api/users");
        request.set_body("{\"name\": \"张三\", \"email\": \"zhang@example.com\"}");
        
        let response = handle_request(&request);
        println!("   📥 请求: {:?}", request);
        println!("   📤 响应: {}", response);
    }
    
    println!("   ✅ 请求处理案例展示了生命周期在Web开发中的应用");
}

/// 中间件案例
fn middleware_example() {
    println!("\n🔗 中间件案例:");
    
    // 中间件trait
    trait Middleware {
        fn process<'a>(&self, request: &'a str, next: Box<dyn Fn(&'a str) -> String + 'a>) -> String;
    }
    
    // 日志中间件
    struct LoggingMiddleware;
    
    impl Middleware for LoggingMiddleware {
        fn process<'a>(&self, request: &'a str, next: Box<dyn Fn(&'a str) -> String + 'a>) -> String {
            println!("   📝 [LOG] 处理请求: {}", request);
            let response = next(request);
            println!("   📝 [LOG] 响应: {}", response);
            response
        }
    }
    
    // 认证中间件
    struct AuthMiddleware;
    
    impl Middleware for AuthMiddleware {
        fn process<'a>(&self, request: &'a str, next: Box<dyn Fn(&'a str) -> String + 'a>) -> String {
            if request.contains("auth=true") {
                println!("   🔐 [AUTH] 认证通过");
                next(request)
            } else {
                println!("   🔐 [AUTH] 认证失败");
                "未授权".to_string()
            }
        }
    }
    
    // 最终处理器
    fn final_handler(request: &str) -> String {
        format!("处理请求: {}", request)
    }
    
    // 中间件链
    fn process_with_middleware(request: &str) -> String {
        let logging = LoggingMiddleware;
        let auth = AuthMiddleware;
        
        // 构建中间件链
        logging.process(request, Box::new(|req| {
            auth.process(req, Box::new(|req| final_handler(req)))
        }))
    }
    
    // 演示中间件
    let requests = vec![
        "GET /api/data?auth=true",
        "GET /api/data?auth=false",
    ];
    
    for request in requests {
        println!("\n   处理请求: {}", request);
        let response = process_with_middleware(request);
        println!("   最终响应: {}", response);
    }
    
    println!("   ✅ 中间件案例展示了高阶函数和生命周期的结合");
}

/// 会话管理案例
fn session_management_example() {
    println!("\n🎫 会话管理案例:");
    
    use std::collections::HashMap;
    
    // 会话数据
    #[derive(Debug, Clone)]
    struct SessionData {
        user_id: String,
        created_at: std::time::SystemTime,
        last_accessed: std::time::SystemTime,
    }
    
    // 会话管理器
    struct SessionManager {
        sessions: HashMap<String, SessionData>,
    }
    
    impl SessionManager {
        fn new() -> Self {
            SessionManager {
                sessions: HashMap::new(),
            }
        }
        
        // 创建会话
        fn create_session(&mut self, session_id: String, user_id: String) {
            let now = std::time::SystemTime::now();
            let session_data = SessionData {
                user_id,
                created_at: now,
                last_accessed: now,
            };
            
            self.sessions.insert(session_id, session_data);
        }
        
        // 获取会话
        fn get_session(&mut self, session_id: &str) -> Option<&SessionData> {
            // NLL允许在借用后修改
            if let Some(session) = self.sessions.get_mut(session_id) {
                session.last_accessed = std::time::SystemTime::now();
            }
            
            self.sessions.get(session_id)
        }
        
        // 清理过期会话
        fn cleanup_expired_sessions(&mut self, max_age: Duration) {
            let now = std::time::SystemTime::now();
            
            self.sessions.retain(|session_id, session_data| {
                if let Ok(age) = now.duration_since(session_data.last_accessed) {
                    if age > max_age {
                        println!("   🗑️ 清理过期会话: {}", session_id);
                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            });
        }
    }
    
    // 演示会话管理
    {
        let mut session_manager = SessionManager::new();
        
        // 创建会话
        session_manager.create_session("session1".to_string(), "user123".to_string());
        session_manager.create_session("session2".to_string(), "user456".to_string());
        
        // 访问会话
        if let Some(session) = session_manager.get_session("session1") {
            println!("   🎫 会话1数据: {:?}", session);
        }
        
        // 模拟时间流逝
        thread::sleep(Duration::from_millis(50));
        
        // 清理过期会话
        session_manager.cleanup_expired_sessions(Duration::from_millis(30));
        
        println!("   📊 剩余会话数: {}", session_manager.sessions.len());
    }
    
    println!("   ✅ 会话管理案例展示了状态管理和生命周期控制");
}

/// 7. 游戏开发案例
fn game_development_examples() {
    println!("\n🎮 7. 游戏开发案例");
    println!("展示在游戏开发中如何处理实体生命周期和资源管理。");
    
    entity_component_system();
    resource_loading_example();
    game_state_management();
}

/// 实体组件系统案例
fn entity_component_system() {
    println!("\n🎯 实体组件系统案例:");
    
    // 组件trait
    trait Component: std::fmt::Debug {}
    
    // 位置组件
    #[derive(Debug, Clone)]
    struct Position {
        x: f32,
        y: f32,
    }
    
    impl Component for Position {}
    
    // 速度组件
    #[derive(Debug, Clone)]
    struct Velocity {
        dx: f32,
        dy: f32,
    }
    
    impl Component for Velocity {}
    
    // 生命值组件
    #[derive(Debug, Clone)]
    struct Health {
        current: i32,
        max: i32,
    }
    
    impl Component for Health {}
    
    // 实体ID
    type EntityId = usize;
    
    // 简化的ECS系统
    struct ECS {
        next_entity_id: EntityId,
        positions: HashMap<EntityId, Position>,
        velocities: HashMap<EntityId, Velocity>,
        healths: HashMap<EntityId, Health>,
    }
    
    impl ECS {
        fn new() -> Self {
            ECS {
                next_entity_id: 0,
                positions: HashMap::new(),
                velocities: HashMap::new(),
                healths: HashMap::new(),
            }
        }
        
        // 创建实体
        fn create_entity(&mut self) -> EntityId {
            let id = self.next_entity_id;
            self.next_entity_id += 1;
            id
        }
        
        // 添加组件
        fn add_position(&mut self, entity: EntityId, position: Position) {
            self.positions.insert(entity, position);
        }
        
        fn add_velocity(&mut self, entity: EntityId, velocity: Velocity) {
            self.velocities.insert(entity, velocity);
        }
        
        fn add_health(&mut self, entity: EntityId, health: Health) {
            self.healths.insert(entity, health);
        }
        
        // 移动系统
        fn movement_system(&mut self, delta_time: f32) {
            // NLL允许同时借用多个HashMap
            for (entity_id, velocity) in &self.velocities {
                if let Some(position) = self.positions.get_mut(entity_id) {
                    position.x += velocity.dx * delta_time;
                    position.y += velocity.dy * delta_time;
                }
            }
        }
        
        // 获取实体信息
        fn get_entity_info(&self, entity: EntityId) -> String {
            let mut info = format!("实体 {}: ", entity);
            
            if let Some(pos) = self.positions.get(&entity) {
                info.push_str(&format!("位置({:.1}, {:.1}) ", pos.x, pos.y));
            }
            
            if let Some(vel) = self.velocities.get(&entity) {
                info.push_str(&format!("速度({:.1}, {:.1}) ", vel.dx, vel.dy));
            }
            
            if let Some(health) = self.healths.get(&entity) {
                info.push_str(&format!("生命值({}/{}) ", health.current, health.max));
            }
            
            info
        }
    }
    
    // 演示ECS系统
    {
        let mut ecs = ECS::new();
        
        // 创建玩家实体
        let player = ecs.create_entity();
        ecs.add_position(player, Position { x: 0.0, y: 0.0 });
        ecs.add_velocity(player, Velocity { dx: 10.0, dy: 5.0 });
        ecs.add_health(player, Health { current: 100, max: 100 });
        
        // 创建敌人实体
        let enemy = ecs.create_entity();
        ecs.add_position(enemy, Position { x: 50.0, y: 30.0 });
        ecs.add_velocity(enemy, Velocity { dx: -5.0, dy: -2.0 });
        ecs.add_health(enemy, Health { current: 50, max: 50 });
        
        println!("   初始状态:");
        println!("   {}", ecs.get_entity_info(player));
        println!("   {}", ecs.get_entity_info(enemy));
        
        // 模拟游戏循环
        for frame in 1..=3 {
            ecs.movement_system(0.1); // 0.1秒的时间步长
            
            println!("   第{}帧后:", frame);
            println!("   {}", ecs.get_entity_info(player));
            println!("   {}", ecs.get_entity_info(enemy));
        }
    }
    
    println!("   ✅ ECS案例展示了组件系统中的生命周期管理");
}

/// 资源加载案例
fn resource_loading_example() {
    println!("\n📦 资源加载案例:");
    
    // 资源类型
    #[derive(Debug, Clone)]
    enum Resource {
        Texture { width: u32, height: u32, data: Vec<u8> },
        Audio { sample_rate: u32, channels: u8, data: Vec<i16> },
        Model { vertices: Vec<f32>, indices: Vec<u32> },
    }
    
    // 资源管理器
    struct ResourceManager {
        resources: HashMap<String, Resource>,
        loading_queue: Vec<String>,
    }
    
    impl ResourceManager {
        fn new() -> Self {
            ResourceManager {
                resources: HashMap::new(),
                loading_queue: Vec::new(),
            }
        }
        
        // 异步加载资源（模拟）
        fn load_resource(&mut self, name: String, resource_type: &str) {
            println!("   📥 开始加载资源: {} (类型: {})", name, resource_type);
            
            let resource = match resource_type {
                "texture" => Resource::Texture {
                    width: 256,
                    height: 256,
                    data: vec![0; 256 * 256 * 4], // RGBA
                },
                "audio" => Resource::Audio {
                    sample_rate: 44100,
                    channels: 2,
                    data: vec![0; 44100 * 2], // 1秒的音频
                },
                "model" => Resource::Model {
                    vertices: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.5, 1.0, 0.0],
                    indices: vec![0, 1, 2],
                },
                _ => {
                    println!("   ❌ 未知资源类型: {}", resource_type);
                    return;
                }
            };
            
            self.resources.insert(name.clone(), resource);
            println!("   ✅ 资源加载完成: {}", name);
        }
        
        // 获取资源
        fn get_resource(&self, name: &str) -> Option<&Resource> {
            self.resources.get(name)
        }
        
        // 预加载资源
        fn preload_resources(&mut self, resource_list: Vec<(&str, &str)>) {
            for (name, resource_type) in resource_list {
                self.load_resource(name.to_string(), resource_type);
            }
        }
        
        // 清理未使用的资源
        fn cleanup_unused_resources(&mut self, used_resources: &[&str]) {
            let used_set: std::collections::HashSet<_> = used_resources.iter().collect();
            
            self.resources.retain(|name, _| {
                if used_set.contains(&name.as_str()) {
                    true
                } else {
                    println!("   🗑️ 清理未使用资源: {}", name);
                    false
                }
            });
        }
    }
    
    // 演示资源管理
    {
        let mut resource_manager = ResourceManager::new();
        
        // 预加载资源
        let resources_to_load = vec![
            ("player_texture", "texture"),
            ("background_music", "audio"),
            ("enemy_model", "model"),
            ("ui_texture", "texture"),
        ];
        
        resource_manager.preload_resources(resources_to_load);
        
        // 使用资源
        if let Some(texture) = resource_manager.get_resource("player_texture") {
            match texture {
                Resource::Texture { width, height, .. } => {
                    println!("   🎨 使用玩家纹理: {}x{}", width, height);
                }
                _ => {}
            }
        }
        
        // 清理未使用的资源
        let used_resources = vec!["player_texture", "background_music"];
        resource_manager.cleanup_unused_resources(&used_resources);
        
        println!("   📊 剩余资源数: {}", resource_manager.resources.len());
    }
    
    println!("   ✅ 资源加载案例展示了资源生命周期管理");
}

/// 游戏状态管理案例
fn game_state_management() {
    println!("\n🎲 游戏状态管理案例:");
    
    // 游戏状态枚举
    #[derive(Debug, Clone, PartialEq)]
    enum GameState {
        MainMenu,
        Playing,
        Paused,
        GameOver,
    }
    
    // 游戏状态管理器
    struct GameStateManager {
        current_state: GameState,
        previous_state: Option<GameState>,
        state_stack: Vec<GameState>,
    }
    
    impl GameStateManager {
        fn new() -> Self {
            GameStateManager {
                current_state: GameState::MainMenu,
                previous_state: None,
                state_stack: vec![GameState::MainMenu],
            }
        }
        
        // 切换状态
        fn transition_to(&mut self, new_state: GameState) {
            println!("   🔄 状态转换: {:?} -> {:?}", self.current_state, new_state);
            
            self.previous_state = Some(self.current_state.clone());
            self.current_state = new_state.clone();
            self.state_stack.push(new_state);
        }
        
        // 推入状态（用于暂停等）
        fn push_state(&mut self, new_state: GameState) {
            println!("   ⬆️ 推入状态: {:?}", new_state);
            
            self.previous_state = Some(self.current_state.clone());
            self.current_state = new_state.clone();
            self.state_stack.push(new_state);
        }
        
        // 弹出状态
        fn pop_state(&mut self) -> Option<GameState> {
            if self.state_stack.len() > 1 {
                let popped = self.state_stack.pop();
                
                if let Some(previous) = self.state_stack.last() {
                    println!("   ⬇️ 弹出状态: {:?} -> {:?}", self.current_state, previous);
                    self.current_state = previous.clone();
                }
                
                popped
            } else {
                None
            }
        }
        
        // 处理状态更新
        fn update(&self) {
            match self.current_state {
                GameState::MainMenu => {
                    println!("   🏠 更新主菜单状态");
                }
                GameState::Playing => {
                    println!("   🎮 更新游戏状态");
                }
                GameState::Paused => {
                    println!("   ⏸️ 更新暂停状态");
                }
                GameState::GameOver => {
                    println!("   💀 更新游戏结束状态");
                }
            }
        }
        
        // 获取当前状态
        fn get_current_state(&self) -> &GameState {
            &self.current_state
        }
    }
    
    // 演示游戏状态管理
    {
        let mut state_manager = GameStateManager::new();
        
        // 模拟游戏流程
        println!("   🎮 游戏开始");
        state_manager.update();
        
        // 开始游戏
        state_manager.transition_to(GameState::Playing);
        state_manager.update();
        
        // 暂停游戏
        state_manager.push_state(GameState::Paused);
        state_manager.update();
        
        // 恢复游戏
        state_manager.pop_state();
        state_manager.update();
        
        // 游戏结束
        state_manager.transition_to(GameState::GameOver);
        state_manager.update();
        
        println!("   📊 状态历史: {:?}", state_manager.state_stack);
    }
    
    println!("   ✅ 游戏状态管理案例展示了状态机的生命周期控制");
}

/// 8. 系统编程案例
fn system_programming_examples() {
    println!("\n⚙️ 8. 系统编程案例");
    println!("展示在系统编程中如何处理底层资源和内存管理。");
    
    file_system_example();
    memory_mapping_example();
    process_management_example();
}

/// 文件系统案例
fn file_system_example() {
    println!("\n📁 文件系统案例:");
    
    use std::fs;
    use std::io::{self, Write, Read};
    use std::path::Path;
    
    // 文件操作包装器
    struct FileManager {
        temp_dir: String,
    }
    
    impl FileManager {
        fn new() -> io::Result<Self> {
            let temp_dir = "/tmp/rust_scope_lifetime_nll_demo".to_string();
            
            // 创建临时目录
            if !Path::new(&temp_dir).exists() {
                fs::create_dir_all(&temp_dir)?;
            }
            
            Ok(FileManager { temp_dir })
        }
        
        // 写入文件
        fn write_file(&self, filename: &str, content: &str) -> io::Result<()> {
            let file_path = format!("{}/{}", self.temp_dir, filename);
            let mut file = fs::File::create(&file_path)?;
            file.write_all(content.as_bytes())?;
            println!("   📝 写入文件: {}", file_path);
            Ok(())
        }
        
        // 读取文件
        fn read_file(&self, filename: &str) -> io::Result<String> {
            let file_path = format!("{}/{}", self.temp_dir, filename);
            let mut file = fs::File::open(&file_path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            println!("   📖 读取文件: {}", file_path);
            Ok(content)
        }
        
        // 列出文件
        fn list_files(&self) -> io::Result<Vec<String>> {
            let mut files = Vec::new();
            
            for entry in fs::read_dir(&self.temp_dir)? {
                let entry = entry?;
                if let Some(filename) = entry.file_name().to_str() {
                    files.push(filename.to_string());
                }
            }
            
            Ok(files)
        }
    }
    
    impl Drop for FileManager {
        fn drop(&mut self) {
            // 清理临时文件
            if let Err(e) = fs::remove_dir_all(&self.temp_dir) {
                println!("   ⚠️ 清理临时目录失败: {}", e);
            } else {
                println!("   🗑️ 清理临时目录: {}", self.temp_dir);
            }
        }
    }
    
    // 演示文件系统操作
    {
        match FileManager::new() {
            Ok(file_manager) => {
                // 写入文件
                let _ = file_manager.write_file("test1.txt", "这是测试文件1的内容");
                let _ = file_manager.write_file("test2.txt", "这是测试文件2的内容");
                
                // 读取文件
                if let Ok(content) = file_manager.read_file("test1.txt") {
                    println!("   📄 文件内容: {}", content);
                }
                
                // 列出文件
                if let Ok(files) = file_manager.list_files() {
                    println!("   📋 文件列表: {:?}", files);
                }
                
                // 文件管理器在作用域结束时自动清理
            }
            Err(e) => println!("   ❌ 创建文件管理器失败: {}", e),
        }
    }
    
    println!("   ✅ 文件系统案例展示了RAII在系统资源管理中的应用");
}

/// 内存映射案例
fn memory_mapping_example() {
    println!("\n🗺️ 内存映射案例:");
    
    // 模拟内存映射结构
    struct MemoryMap {
        data: Vec<u8>,
        size: usize,
        is_mapped: bool,
    }
    
    impl MemoryMap {
        fn new(size: usize) -> Self {
            println!("   📍 创建内存映射，大小: {} 字节", size);
            MemoryMap {
                data: vec![0; size],
                size,
                is_mapped: true,
            }
        }
        
        // 写入数据
        fn write_at(&mut self, offset: usize, data: &[u8]) -> Result<(), &'static str> {
            if !self.is_mapped {
                return Err("内存映射已取消");
            }
            
            if offset + data.len() > self.size {
                return Err("写入超出边界");
            }
            
            self.data[offset..offset + data.len()].copy_from_slice(data);
            println!("   ✍️ 在偏移 {} 处写入 {} 字节", offset, data.len());
            Ok(())
        }
        
        // 读取数据
        fn read_at(&self, offset: usize, len: usize) -> Result<&[u8], &'static str> {
            if !self.is_mapped {
                return Err("内存映射已取消");
            }
            
            if offset + len > self.size {
                return Err("读取超出边界");
            }
            
            println!("   📖 从偏移 {} 处读取 {} 字节", offset, len);
            Ok(&self.data[offset..offset + len])
        }
        
        // 同步到磁盘（模拟）
        fn sync(&self) -> Result<(), &'static str> {
            if !self.is_mapped {
                return Err("内存映射已取消");
            }
            
            println!("   💾 同步内存映射到磁盘");
            Ok(())
        }
    }
    
    impl Drop for MemoryMap {
        fn drop(&mut self) {
            if self.is_mapped {
                self.is_mapped = false;
                println!("   🔓 取消内存映射");
            }
        }
    }
    
    // 演示内存映射
    {
        let mut memory_map = MemoryMap::new(1024);
        
        // 写入数据
        let data1 = b"Hello, Memory Map!";
        let _ = memory_map.write_at(0, data1);
        
        let data2 = b"Rust Lifetime Demo";
        let _ = memory_map.write_at(100, data2);
        
        // 读取数据
        if let Ok(read_data) = memory_map.read_at(0, data1.len()) {
            let content = String::from_utf8_lossy(read_data);
            println!("   📄 读取内容: {}", content);
        }
        
        // 同步数据
        let _ = memory_map.sync();
        
        // 内存映射在作用域结束时自动取消
    }
    
    println!("   ✅ 内存映射案例展示了系统资源的生命周期管理");
}

/// 进程管理案例
fn process_management_example() {
    println!("\n🔄 进程管理案例:");
    
    use std::process::{Command, Stdio};
    
    // 进程管理器
    struct ProcessManager {
        processes: Vec<String>,
    }
    
    impl ProcessManager {
        fn new() -> Self {
            ProcessManager {
                processes: Vec::new(),
            }
        }
        
        // 执行命令
        fn execute_command(&mut self, command: &str, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
            println!("   🚀 执行命令: {} {:?}", command, args);
            
            let output = Command::new(command)
                .args(args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()?;
            
            let command_str = format!("{} {}", command, args.join(" "));
            self.processes.push(command_str);
            
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                Ok(stdout.to_string())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err(format!("命令执行失败: {}", stderr).into())
            }
        }
        
        // 获取系统信息
        fn get_system_info(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            // 获取当前日期
            match self.execute_command("date", &[]) {
                Ok(output) => println!("   📅 系统时间: {}", output.trim()),
                Err(e) => println!("   ❌ 获取时间失败: {}", e),
            }
            
            // 获取用户信息
            match self.execute_command("whoami", &[]) {
                Ok(output) => println!("   👤 当前用户: {}", output.trim()),
                Err(e) => println!("   ❌ 获取用户失败: {}", e),
            }
            
            Ok(())
        }
        
        // 获取执行历史
        fn get_execution_history(&self) -> &[String] {
            &self.processes
        }
    }
    
    // 演示进程管理
    {
        let mut process_manager = ProcessManager::new();
        
        // 获取系统信息
        let _ = process_manager.get_system_info();
        
        // 执行一些基本命令
        match process_manager.execute_command("echo", &["Hello from Rust!"]) {
            Ok(output) => println!("   📢 Echo输出: {}", output.trim()),
            Err(e) => println!("   ❌ Echo失败: {}", e),
        }
        
        // 显示执行历史
        let history = process_manager.get_execution_history();
        println!("   📋 执行历史: {:?}", history);
    }
    
    println!("   ✅ 进程管理案例展示了系统调用的生命周期控制");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_practical_examples() {
        // 测试所有实际案例是否能正常运行
        run_practical_examples();
    }

    #[test]
    fn test_data_structures() {
        // 测试数据结构案例
        data_structure_examples();
    }

    #[test]
    fn test_error_handling() {
        // 测试错误处理案例
        error_handling_examples();
    }

    #[test]
    fn test_performance_optimization() {
        // 测试性能优化案例
        performance_optimization_examples();
    }

    #[test]
    fn test_concurrent_programming() {
        // 测试并发编程案例
        concurrent_programming_examples();
    }
}