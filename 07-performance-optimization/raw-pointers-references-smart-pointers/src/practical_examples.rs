//! # 实际应用案例和最佳实践
//!
//! 本模块提供了裸指针、引用和智能指针在实际项目中的应用案例，
//! 包括数据结构实现、系统编程、网络编程、GUI 应用等场景。

use std::ptr;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex, RwLock};
use std::cell::{RefCell, Cell};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// 运行所有实际应用案例
pub fn run_all_examples() {
    println!("\n🏗️ 实际应用案例和最佳实践");
    println!("{}\n", "=".repeat(60));
    
    // 数据结构实现案例
    data_structure_examples();
    
    // 系统编程案例
    system_programming_examples();
    
    // 多线程编程案例
    multithreading_examples();
    
    // 网络编程案例
    network_programming_examples();
    
    // GUI 应用案例
    gui_application_examples();
    
    // 游戏开发案例
    game_development_examples();
    
    // FFI 交互案例
    ffi_interaction_examples();
    
    // 性能优化案例
    performance_optimization_examples();
    
    // 内存池管理案例
    memory_pool_examples();
    
    // 最佳实践总结
    best_practices_summary();
}

/// 数据结构实现案例
fn data_structure_examples() {
    println!("📊 1. 数据结构实现案例");
    println!("{}", "-".repeat(40));
    
    // 链表实现 - 使用 Box
    println!("\n🔗 链表实现 (使用 Box):");
    linked_list_with_box();
    
    // 二叉树实现 - 使用 Rc
    println!("\n🌳 二叉树实现 (使用 Rc):");
    binary_tree_with_rc();
    
    // 图结构实现 - 使用 Weak 引用
    println!("\n🕸️ 图结构实现 (使用 Weak 引用):");
    graph_with_weak_references();
    
    // 缓存实现 - 使用 RefCell
    println!("\n💾 LRU 缓存实现 (使用 RefCell):");
    lru_cache_with_refcell();
}

/// 链表实现
fn linked_list_with_box() {
    #[derive(Debug)]
    struct Node<T> {
        data: T,
        next: Option<Box<Node<T>>>,
    }
    
    impl<T: std::fmt::Debug> Node<T> {
        fn new(data: T) -> Self {
            Node { data, next: None }
        }
        
        fn append(&mut self, data: T) {
            match &mut self.next {
                None => self.next = Some(Box::new(Node::new(data))),
                Some(next_node) => next_node.append(data),
            }
        }
        
        fn print_list(&self) {
            print!("{:?}", self.data);
            if let Some(next) = &self.next {
                print!(" -> ");
                next.print_list();
            } else {
                println!(" -> None");
            }
        }
    }
    
    let mut head = Node::new(1);
    head.append(2);
    head.append(3);
    head.append(4);
    
    print!("  链表: ");
    head.print_list();
    
    println!("  ✅ Box 适合递归数据结构，提供独占所有权");
}

/// 二叉树实现
fn binary_tree_with_rc() {
    #[derive(Debug)]
    struct TreeNode<T> {
        value: T,
        left: Option<Rc<RefCell<TreeNode<T>>>>,
        right: Option<Rc<RefCell<TreeNode<T>>>>,
    }
    
    impl<T: std::fmt::Display + Clone> TreeNode<T> {
        fn new(value: T) -> Rc<RefCell<Self>> {
            Rc::new(RefCell::new(TreeNode {
                value,
                left: None,
                right: None,
            }))
        }
        
        fn insert(node: &Rc<RefCell<Self>>, value: T) 
        where T: PartialOrd {
            let mut node_ref = node.borrow_mut();
            if value < node_ref.value {
                match &node_ref.left {
                    None => node_ref.left = Some(Self::new(value)),
                    Some(left) => Self::insert(left, value),
                }
            } else {
                match &node_ref.right {
                    None => node_ref.right = Some(Self::new(value)),
                    Some(right) => Self::insert(right, value),
                }
            }
        }
        
        fn inorder_traversal(node: &Option<Rc<RefCell<Self>>>) {
            if let Some(n) = node {
                let node_ref = n.borrow();
                Self::inorder_traversal(&node_ref.left);
                print!("{} ", node_ref.value);
                Self::inorder_traversal(&node_ref.right);
            }
        }
    }
    
    let root = TreeNode::new(5);
    TreeNode::insert(&root, 3);
    TreeNode::insert(&root, 7);
    TreeNode::insert(&root, 1);
    TreeNode::insert(&root, 9);
    
    print!("  中序遍历: ");
    TreeNode::inorder_traversal(&Some(root));
    println!();
    
    println!("  ✅ Rc<RefCell<T>> 适合共享可变的树结构");
}

/// 图结构实现
fn graph_with_weak_references() {
    #[derive(Debug)]
    struct GraphNode {
        id: usize,
        neighbors: RefCell<Vec<Weak<GraphNode>>>,
    }
    
    impl GraphNode {
        fn new(id: usize) -> Rc<Self> {
            Rc::new(GraphNode {
                id,
                neighbors: RefCell::new(Vec::new()),
            })
        }
        
        fn add_neighbor(self: &Rc<Self>, neighbor: &Rc<Self>) {
            self.neighbors.borrow_mut().push(Rc::downgrade(neighbor));
            neighbor.neighbors.borrow_mut().push(Rc::downgrade(self));
        }
        
        fn print_neighbors(&self) {
            print!("  节点 {} 的邻居: ", self.id);
            for weak_neighbor in self.neighbors.borrow().iter() {
                if let Some(neighbor) = weak_neighbor.upgrade() {
                    print!("{} ", neighbor.id);
                }
            }
            println!();
        }
    }
    
    let node1 = GraphNode::new(1);
    let node2 = GraphNode::new(2);
    let node3 = GraphNode::new(3);
    
    node1.add_neighbor(&node2);
    node2.add_neighbor(&node3);
    node1.add_neighbor(&node3);
    
    node1.print_neighbors();
    node2.print_neighbors();
    node3.print_neighbors();
    
    println!("  ✅ Weak 引用防止循环引用，适合图结构");
}

/// LRU 缓存实现
fn lru_cache_with_refcell() {
    use std::collections::LinkedList;
    
    struct LRUCache<K, V> {
        capacity: usize,
        map: RefCell<HashMap<K, V>>,
        order: RefCell<LinkedList<K>>,
    }
    
    impl<K: Clone + Eq + std::hash::Hash, V: Clone> LRUCache<K, V> {
        fn new(capacity: usize) -> Self {
            LRUCache {
                capacity,
                map: RefCell::new(HashMap::new()),
                order: RefCell::new(LinkedList::new()),
            }
        }
        
        fn get(&self, key: &K) -> Option<V> {
            let mut map = self.map.borrow_mut();
            if let Some(value) = map.get(key) {
                let value = value.clone();
                drop(map);
                
                // 更新访问顺序
                let mut order = self.order.borrow_mut();
                if let Some(pos) = order.iter().position(|k| k == key) {
                    let mut split_list = order.split_off(pos);
                    let key = split_list.pop_front().unwrap();
                    order.append(&mut split_list);
                    order.push_back(key);
                }
                
                Some(value)
            } else {
                None
            }
        }
        
        fn put(&self, key: K, value: V) {
            let mut map = self.map.borrow_mut();
            let mut order = self.order.borrow_mut();
            
            if map.contains_key(&key) {
                // 更新现有键
                map.insert(key.clone(), value);
                if let Some(pos) = order.iter().position(|k| k == &key) {
                    let mut split_list = order.split_off(pos);
                    let key = split_list.pop_front().unwrap();
                    order.append(&mut split_list);
                    order.push_back(key);
                }
            } else {
                // 插入新键
                if map.len() >= self.capacity {
                    // 移除最久未使用的项
                    if let Some(old_key) = order.pop_front() {
                        map.remove(&old_key);
                    }
                }
                map.insert(key.clone(), value);
                order.push_back(key);
            }
        }
        
        fn len(&self) -> usize {
            self.map.borrow().len()
        }
    }
    
    let cache = LRUCache::new(3);
    
    cache.put("a", 1);
    cache.put("b", 2);
    cache.put("c", 3);
    
    println!("  缓存大小: {}", cache.len());
    println!("  获取 'a': {:?}", cache.get(&"a"));
    
    cache.put("d", 4); // 这会移除 'b'
    println!("  插入 'd' 后获取 'b': {:?}", cache.get(&"b"));
    println!("  获取 'c': {:?}", cache.get(&"c"));
    
    println!("  ✅ RefCell 提供内部可变性，适合复杂的缓存逻辑");
}

/// 系统编程案例
fn system_programming_examples() {
    println!("\n🔧 2. 系统编程案例");
    println!("{}", "-".repeat(40));
    
    // 内存映射文件
    println!("\n📁 内存映射文件模拟:");
    memory_mapped_file_simulation();
    
    // 自定义分配器
    println!("\n🏭 自定义分配器:");
    custom_allocator_example();
    
    // 系统调用包装
    println!("\n⚙️ 系统调用包装:");
    system_call_wrapper();
}

/// 内存映射文件模拟
fn memory_mapped_file_simulation() {
    struct MemoryMappedFile {
        data: *mut u8,
        size: usize,
    }
    
    impl MemoryMappedFile {
        fn new(size: usize) -> Self {
            unsafe {
                let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
                let data = std::alloc::alloc(layout);
                if data.is_null() {
                    panic!("内存分配失败");
                }
                
                // 初始化内存
                ptr::write_bytes(data, 0, size);
                
                MemoryMappedFile { data, size }
            }
        }
        
        fn write_at(&mut self, offset: usize, data: &[u8]) -> Result<(), &'static str> {
            if offset + data.len() > self.size {
                return Err("写入超出文件边界");
            }
            
            unsafe {
                ptr::copy_nonoverlapping(data.as_ptr(), self.data.add(offset), data.len());
            }
            Ok(())
        }
        
        fn read_at(&self, offset: usize, len: usize) -> Result<Vec<u8>, &'static str> {
            if offset + len > self.size {
                return Err("读取超出文件边界");
            }
            
            let mut buffer = vec![0u8; len];
            unsafe {
                ptr::copy_nonoverlapping(self.data.add(offset), buffer.as_mut_ptr(), len);
            }
            Ok(buffer)
        }
    }
    
    impl Drop for MemoryMappedFile {
        fn drop(&mut self) {
            unsafe {
                let layout = std::alloc::Layout::from_size_align(self.size, 1).unwrap();
                std::alloc::dealloc(self.data, layout);
            }
        }
    }
    
    let mut file = MemoryMappedFile::new(1024);
    
    let data = b"Hello, Memory Mapped File!";
    file.write_at(0, data).unwrap();
    
    let read_data = file.read_at(0, data.len()).unwrap();
    println!("  写入数据: {:?}", std::str::from_utf8(data).unwrap());
    println!("  读取数据: {:?}", std::str::from_utf8(&read_data).unwrap());
    
    println!("  ✅ 裸指针适合底层内存操作和系统编程");
}

/// 自定义分配器示例
fn custom_allocator_example() {
    struct StackAllocator {
        buffer: Vec<u8>,
        offset: Cell<usize>,
    }
    
    impl StackAllocator {
        fn new(size: usize) -> Self {
            StackAllocator {
                buffer: vec![0; size],
                offset: Cell::new(0),
            }
        }
        
        fn allocate(&self, size: usize, align: usize) -> Option<*mut u8> {
            let current_offset = self.offset.get();
            let aligned_offset = (current_offset + align - 1) & !(align - 1);
            
            if aligned_offset + size <= self.buffer.len() {
                self.offset.set(aligned_offset + size);
                Some(unsafe { self.buffer.as_ptr().add(aligned_offset) as *mut u8 })
            } else {
                None
            }
        }
        
        fn reset(&self) {
            self.offset.set(0);
        }
        
        fn used(&self) -> usize {
            self.offset.get()
        }
    }
    
    let allocator = StackAllocator::new(1024);
    
    // 分配一些内存
    let ptr1 = allocator.allocate(64, 8).unwrap();
    let ptr2 = allocator.allocate(128, 4).unwrap();
    
    println!("  分配器使用: {} 字节", allocator.used());
    println!("  指针1: {:p}", ptr1);
    println!("  指针2: {:p}", ptr2);
    
    // 重置分配器
    allocator.reset();
    println!("  重置后使用: {} 字节", allocator.used());
    
    println!("  ✅ 自定义分配器提供特殊的内存管理策略");
}

/// 系统调用包装
fn system_call_wrapper() {
    // 模拟系统调用接口
    struct SystemBuffer {
        data: *mut u8,
        capacity: usize,
        len: usize,
    }
    
    impl SystemBuffer {
        fn new(capacity: usize) -> Self {
            unsafe {
                let layout = std::alloc::Layout::array::<u8>(capacity).unwrap();
                let data = std::alloc::alloc(layout);
                if data.is_null() {
                    panic!("系统缓冲区分配失败");
                }
                
                SystemBuffer {
                    data,
                    capacity,
                    len: 0,
                }
            }
        }
        
        fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
            if self.len + data.len() > self.capacity {
                return Err("缓冲区空间不足");
            }
            
            unsafe {
                ptr::copy_nonoverlapping(
                    data.as_ptr(),
                    self.data.add(self.len),
                    data.len()
                );
            }
            
            self.len += data.len();
            Ok(data.len())
        }
        
        fn read(&self) -> &[u8] {
            unsafe {
                std::slice::from_raw_parts(self.data, self.len)
            }
        }
        
        fn as_ptr(&self) -> *const u8 {
            self.data
        }
    }
    
    impl Drop for SystemBuffer {
        fn drop(&mut self) {
            unsafe {
                let layout = std::alloc::Layout::array::<u8>(self.capacity).unwrap();
                std::alloc::dealloc(self.data, layout);
            }
        }
    }
    
    let mut buffer = SystemBuffer::new(256);
    
    let message = b"System call data";
    let written = buffer.write(message).unwrap();
    
    println!("  写入 {} 字节到系统缓冲区", written);
    println!("  缓冲区内容: {:?}", std::str::from_utf8(buffer.read()).unwrap());
    println!("  缓冲区指针: {:p}", buffer.as_ptr());
    
    println!("  ✅ 裸指针适合与系统 API 交互");
}

/// 多线程编程案例
fn multithreading_examples() {
    println!("\n🧵 3. 多线程编程案例");
    println!("{}", "-".repeat(40));
    
    // 生产者-消费者模式
    println!("\n🏭 生产者-消费者模式:");
    producer_consumer_pattern();
    
    // 工作窃取队列
    println!("\n⚡ 工作窃取队列:");
    work_stealing_queue();
    
    // 读写锁应用
    println!("\n📚 读写锁应用:");
    reader_writer_lock_example();
}

/// 生产者-消费者模式
fn producer_consumer_pattern() {
    let (tx, rx) = mpsc::channel();
    let buffer = Arc::new(Mutex::new(Vec::new()));
    let buffer_clone = Arc::clone(&buffer);
    
    // 生产者线程
    let producer = thread::spawn(move || {
        for i in 0..5 {
            let item = format!("Item {}", i);
            {
                let mut buf = buffer.lock().unwrap();
                buf.push(item.clone());
                println!("  生产者生产: {}", item);
            }
            tx.send(()).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // 消费者线程
    let consumer = thread::spawn(move || {
        for _ in 0..5 {
            rx.recv().unwrap();
            let mut buf = buffer_clone.lock().unwrap();
            if let Some(item) = buf.pop() {
                println!("  消费者消费: {}", item);
            }
        }
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
    
    println!("  ✅ Arc<Mutex<T>> 适合多线程共享可变数据");
}

/// 工作窃取队列
fn work_stealing_queue() {
    struct WorkStealingQueue<T> {
        queue: Arc<Mutex<Vec<T>>>,
    }
    
    impl<T> WorkStealingQueue<T> {
        fn new() -> Self {
            WorkStealingQueue {
                queue: Arc::new(Mutex::new(Vec::new())),
            }
        }
        
        fn push(&self, item: T) {
            self.queue.lock().unwrap().push(item);
        }
        
        fn pop(&self) -> Option<T> {
            self.queue.lock().unwrap().pop()
        }
        
        fn steal(&self) -> Option<T> {
            let mut queue = self.queue.lock().unwrap();
            if !queue.is_empty() {
                Some(queue.remove(0))
            } else {
                None
            }
        }
        
        fn clone_queue(&self) -> Self {
            WorkStealingQueue {
                queue: Arc::clone(&self.queue),
            }
        }
    }
    
    let main_queue = WorkStealingQueue::new();
    
    // 添加一些工作项
    for i in 0..10 {
        main_queue.push(format!("Task {}", i));
    }
    
    let mut handles = vec![];
    
    // 创建工作线程
    for worker_id in 0..3 {
        let queue = main_queue.clone_queue();
        let handle = thread::spawn(move || {
            while let Some(task) = queue.pop().or_else(|| queue.steal()) {
                println!("  工作线程 {} 执行: {}", worker_id, task);
                thread::sleep(Duration::from_millis(50));
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("  ✅ Arc 允许多个线程共享同一个队列");
}

/// 读写锁应用
fn reader_writer_lock_example() {
    let data = Arc::new(RwLock::new(HashMap::new()));
    let mut handles = vec![];
    
    // 写入线程
    for i in 0..2 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut map = data.write().unwrap();
            map.insert(format!("key{}", i), format!("value{}", i));
            println!("  写入线程 {} 插入数据", i);
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }
    
    // 读取线程
    for i in 0..4 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            let map = data.read().unwrap();
            println!("  读取线程 {} 看到 {} 个条目", i, map.len());
            for (k, v) in map.iter() {
                println!("    {} -> {}", k, v);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("  ✅ RwLock 允许多个读者或一个写者");
}

/// 网络编程案例
fn network_programming_examples() {
    println!("\n🌐 4. 网络编程案例");
    println!("{}", "-".repeat(40));
    
    // 连接池
    println!("\n🏊 连接池管理:");
    connection_pool_example();
    
    // 请求路由
    println!("\n🛣️ 请求路由:");
    request_router_example();
    
    // 缓存层
    println!("\n💾 分布式缓存:");
    distributed_cache_example();
}

/// 连接池示例
fn connection_pool_example() {
    #[derive(Debug, Clone)]
    struct Connection {
        id: usize,
        active: bool,
    }
    
    impl Connection {
        fn new(id: usize) -> Self {
            Connection { id, active: true }
        }
        
        fn execute_query(&self, query: &str) -> String {
            format!("连接 {} 执行查询: {}", self.id, query)
        }
    }
    
    struct ConnectionPool {
        connections: Arc<Mutex<Vec<Connection>>>,
        max_size: usize,
    }
    
    impl ConnectionPool {
        fn new(max_size: usize) -> Self {
            let mut connections = Vec::new();
            for i in 0..max_size {
                connections.push(Connection::new(i));
            }
            
            ConnectionPool {
                connections: Arc::new(Mutex::new(connections)),
                max_size,
            }
        }
        
        fn get_connection(&self) -> Option<Connection> {
            let mut pool = self.connections.lock().unwrap();
            pool.pop()
        }
        
        fn return_connection(&self, conn: Connection) {
            let mut pool = self.connections.lock().unwrap();
            if pool.len() < self.max_size {
                pool.push(conn);
            }
        }
        
        fn available_connections(&self) -> usize {
            self.connections.lock().unwrap().len()
        }
    }
    
    let pool = Arc::new(ConnectionPool::new(3));
    let mut handles = vec![];
    
    for i in 0..5 {
        let pool = Arc::clone(&pool);
        let handle = thread::spawn(move || {
            if let Some(conn) = pool.get_connection() {
                let result = conn.execute_query(&format!("SELECT * FROM table{}", i));
                println!("  {}", result);
                thread::sleep(Duration::from_millis(100));
                pool.return_connection(conn);
            } else {
                println!("  线程 {} 无法获取连接", i);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("  最终可用连接: {}", pool.available_connections());
    println!("  ✅ Arc<Mutex<T>> 适合管理共享资源池");
}

/// 请求路由示例
fn request_router_example() {
    type Handler = Box<dyn Fn(&str) -> String + Send + Sync>;
    
    struct Router {
        routes: Arc<RwLock<HashMap<String, Handler>>>,
    }
    
    impl Router {
        fn new() -> Self {
            Router {
                routes: Arc::new(RwLock::new(HashMap::new())),
            }
        }
        
        fn add_route<F>(&self, path: String, handler: F)
        where
            F: Fn(&str) -> String + Send + Sync + 'static,
        {
            let mut routes = self.routes.write().unwrap();
            routes.insert(path, Box::new(handler));
        }
        
        fn handle_request(&self, path: &str, body: &str) -> String {
            let routes = self.routes.read().unwrap();
            if let Some(handler) = routes.get(path) {
                handler(body)
            } else {
                "404 Not Found".to_string()
            }
        }
        
        fn clone_router(&self) -> Self {
            Router {
                routes: Arc::clone(&self.routes),
            }
        }
    }
    
    let router = Router::new();
    
    // 添加路由
    router.add_route("/api/users".to_string(), |body| {
        format!("用户 API 处理: {}", body)
    });
    
    router.add_route("/api/orders".to_string(), |body| {
        format!("订单 API 处理: {}", body)
    });
    
    // 模拟多个请求处理线程
    let mut handles = vec![];
    let requests = vec![
        ("/api/users", "获取用户列表"),
        ("/api/orders", "创建新订单"),
        ("/api/products", "获取产品信息"),
    ];
    
    for (i, (path, body)) in requests.into_iter().enumerate() {
        let router = router.clone_router();
        let handle = thread::spawn(move || {
            let response = router.handle_request(path, body);
            println!("  请求 {}: {} -> {}", i + 1, path, response);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("  ✅ Arc<RwLock<T>> 适合读多写少的路由表");
}

/// 分布式缓存示例
fn distributed_cache_example() {
    struct CacheNode {
        id: usize,
        data: Arc<RwLock<HashMap<String, String>>>,
    }
    
    impl CacheNode {
        fn new(id: usize) -> Self {
            CacheNode {
                id,
                data: Arc::new(RwLock::new(HashMap::new())),
            }
        }
        
        fn set(&self, key: String, value: String) {
            let mut data = self.data.write().unwrap();
            data.insert(key.clone(), value.clone());
            println!("  节点 {} 设置: {} = {}", self.id, key, value);
        }
        
        fn get(&self, key: &str) -> Option<String> {
            let data = self.data.read().unwrap();
            data.get(key).cloned()
        }
        
        fn clone_data(&self) -> Arc<RwLock<HashMap<String, String>>> {
            Arc::clone(&self.data)
        }
    }
    
    struct DistributedCache {
        nodes: Vec<CacheNode>,
    }
    
    impl DistributedCache {
        fn new(node_count: usize) -> Self {
            let mut nodes = Vec::new();
            for i in 0..node_count {
                nodes.push(CacheNode::new(i));
            }
            DistributedCache { nodes }
        }
        
        fn hash_key(&self, key: &str) -> usize {
            // 简单的哈希函数
            key.len() % self.nodes.len()
        }
        
        fn set(&self, key: String, value: String) {
            let node_index = self.hash_key(&key);
            self.nodes[node_index].set(key, value);
        }
        
        fn get(&self, key: &str) -> Option<String> {
            let node_index = self.hash_key(key);
            self.nodes[node_index].get(key)
        }
    }
    
    let cache = DistributedCache::new(3);
    
    // 设置一些数据
    cache.set("user:1".to_string(), "Alice".to_string());
    cache.set("user:2".to_string(), "Bob".to_string());
    cache.set("user:3".to_string(), "Charlie".to_string());
    cache.set("order:1".to_string(), "Order #1".to_string());
    
    // 获取数据
    println!("  获取 user:1: {:?}", cache.get("user:1"));
    println!("  获取 user:2: {:?}", cache.get("user:2"));
    println!("  获取 order:1: {:?}", cache.get("order:1"));
    println!("  获取不存在的键: {:?}", cache.get("user:999"));
    
    println!("  ✅ 智能指针组合适合构建分布式系统");
}

/// GUI 应用案例
fn gui_application_examples() {
    println!("\n🖥️ 5. GUI 应用案例");
    println!("{}", "-".repeat(40));
    
    // 组件树
    println!("\n🌳 组件树管理:");
    component_tree_example();
    
    // 事件系统
    println!("\n📡 事件系统:");
    event_system_example();
    
    // 状态管理
    println!("\n📊 状态管理:");
    state_management_example();
}

/// 组件树示例
fn component_tree_example() {
    #[derive(Debug)]
    struct Component {
        id: String,
        children: RefCell<Vec<Rc<Component>>>,
        parent: RefCell<Option<Weak<Component>>>,
    }
    
    impl Component {
        fn new(id: String) -> Rc<Self> {
            Rc::new(Component {
                id,
                children: RefCell::new(Vec::new()),
                parent: RefCell::new(None),
            })
        }
        
        fn add_child(self: &Rc<Self>, child: &Rc<Self>) {
            self.children.borrow_mut().push(Rc::clone(child));
            *child.parent.borrow_mut() = Some(Rc::downgrade(self));
        }
        
        fn render(&self, depth: usize) {
            let indent = "  ".repeat(depth);
            println!("{}📦 组件: {}", indent, self.id);
            
            for child in self.children.borrow().iter() {
                child.render(depth + 1);
            }
        }
        
        fn find_parent(&self) -> Option<Rc<Component>> {
            self.parent.borrow().as_ref()?.upgrade()
        }
    }
    
    let root = Component::new("App".to_string());
    let header = Component::new("Header".to_string());
    let main = Component::new("Main".to_string());
    let sidebar = Component::new("Sidebar".to_string());
    let content = Component::new("Content".to_string());
    
    root.add_child(&header);
    root.add_child(&main);
    main.add_child(&sidebar);
    main.add_child(&content);
    
    println!("  组件树结构:");
    root.render(1);
    
    if let Some(parent) = content.find_parent() {
        println!("  Content 的父组件: {}", parent.id);
    }
    
    println!("  ✅ Rc + Weak 适合构建组件树，避免循环引用");
}

/// 事件系统示例
fn event_system_example() {
    type EventHandler = Box<dyn Fn(&str) + Send + Sync>;
    
    struct EventBus {
        handlers: Arc<RwLock<HashMap<String, Vec<EventHandler>>>>,
    }
    
    impl EventBus {
        fn new() -> Self {
            EventBus {
                handlers: Arc::new(RwLock::new(HashMap::new())),
            }
        }
        
        fn subscribe<F>(&self, event_type: String, handler: F)
        where
            F: Fn(&str) + Send + Sync + 'static,
        {
            let mut handlers = self.handlers.write().unwrap();
            handlers
                .entry(event_type)
                .or_insert_with(Vec::new)
                .push(Box::new(handler));
        }
        
        fn emit(&self, event_type: &str, data: &str) {
            let handlers = self.handlers.read().unwrap();
            if let Some(event_handlers) = handlers.get(event_type) {
                for handler in event_handlers {
                    handler(data);
                }
            }
        }
        
        fn clone_bus(&self) -> Self {
            EventBus {
                handlers: Arc::clone(&self.handlers),
            }
        }
    }
    
    let event_bus = EventBus::new();
    
    // 订阅事件
    event_bus.subscribe("button_click".to_string(), |data| {
        println!("  按钮点击处理器1: {}", data);
    });
    
    event_bus.subscribe("button_click".to_string(), |data| {
        println!("  按钮点击处理器2: {}", data);
    });
    
    event_bus.subscribe("window_resize".to_string(), |data| {
        println!("  窗口大小改变: {}", data);
    });
    
    // 发送事件
    event_bus.emit("button_click", "提交按钮");
    event_bus.emit("window_resize", "800x600");
    event_bus.emit("unknown_event", "不会有处理器");
    
    println!("  ✅ Arc<RwLock<T>> 适合实现事件总线");
}

/// 状态管理示例
fn state_management_example() {
    #[derive(Debug, Clone)]
    struct AppState {
        user_name: String,
        theme: String,
        notifications: Vec<String>,
    }
    
    impl AppState {
        fn new() -> Self {
            AppState {
                user_name: "Guest".to_string(),
                theme: "light".to_string(),
                notifications: Vec::new(),
            }
        }
    }
    
    struct StateManager {
        state: Arc<RwLock<AppState>>,
        subscribers: Arc<RwLock<Vec<Box<dyn Fn(&AppState) + Send + Sync>>>>,
    }
    
    impl StateManager {
        fn new(initial_state: AppState) -> Self {
            StateManager {
                state: Arc::new(RwLock::new(initial_state)),
                subscribers: Arc::new(RwLock::new(Vec::new())),
            }
        }
        
        fn subscribe<F>(&self, callback: F)
        where
            F: Fn(&AppState) + Send + Sync + 'static,
        {
            self.subscribers.write().unwrap().push(Box::new(callback));
        }
        
        fn update_state<F>(&self, updater: F)
        where
            F: FnOnce(&mut AppState),
        {
            {
                let mut state = self.state.write().unwrap();
                updater(&mut state);
            }
            
            // 通知订阅者
            let state = self.state.read().unwrap();
            let subscribers = self.subscribers.read().unwrap();
            for callback in subscribers.iter() {
                callback(&state);
            }
        }
        
        fn get_state(&self) -> AppState {
            self.state.read().unwrap().clone()
        }
    }
    
    let state_manager = StateManager::new(AppState::new());
    
    // 订阅状态变化
    state_manager.subscribe(|state| {
        println!("  UI 组件更新: 用户 = {}, 主题 = {}", state.user_name, state.theme);
    });
    
    state_manager.subscribe(|state| {
        println!("  通知组件: {} 条通知", state.notifications.len());
    });
    
    // 更新状态
    state_manager.update_state(|state| {
        state.user_name = "Alice".to_string();
    });
    
    state_manager.update_state(|state| {
        state.theme = "dark".to_string();
        state.notifications.push("欢迎使用暗色主题".to_string());
    });
    
    let final_state = state_manager.get_state();
    println!("  最终状态: {:?}", final_state);
    
    println!("  ✅ Arc<RwLock<T>> 适合全局状态管理");
}

/// 游戏开发案例
fn game_development_examples() {
    println!("\n🎮 6. 游戏开发案例");
    println!("{}", "-".repeat(40));
    
    // 实体组件系统
    println!("\n🎯 实体组件系统 (ECS):");
    entity_component_system();
    
    // 资源管理
    println!("\n📦 游戏资源管理:");
    game_resource_management();
}

/// 实体组件系统
fn entity_component_system() {
    type EntityId = usize;
    
    trait Component: std::fmt::Debug {}
    
    #[derive(Debug)]
    struct Position { x: f32, y: f32 }
    impl Component for Position {}
    
    #[derive(Debug)]
    struct Velocity { dx: f32, dy: f32 }
    impl Component for Velocity {}
    
    #[derive(Debug)]
    struct Health { current: i32, max: i32 }
    impl Component for Health {}
    
    struct World {
        entities: Vec<EntityId>,
        positions: HashMap<EntityId, Position>,
        velocities: HashMap<EntityId, Velocity>,
        healths: HashMap<EntityId, Health>,
        next_entity_id: EntityId,
    }
    
    impl World {
        fn new() -> Self {
            World {
                entities: Vec::new(),
                positions: HashMap::new(),
                velocities: HashMap::new(),
                healths: HashMap::new(),
                next_entity_id: 0,
            }
        }
        
        fn create_entity(&mut self) -> EntityId {
            let id = self.next_entity_id;
            self.entities.push(id);
            self.next_entity_id += 1;
            id
        }
        
        fn add_position(&mut self, entity: EntityId, pos: Position) {
            self.positions.insert(entity, pos);
        }
        
        fn add_velocity(&mut self, entity: EntityId, vel: Velocity) {
            self.velocities.insert(entity, vel);
        }
        
        fn add_health(&mut self, entity: EntityId, health: Health) {
            self.healths.insert(entity, health);
        }
        
        fn movement_system(&mut self) {
            for entity in &self.entities {
                if let (Some(pos), Some(vel)) = (
                    self.positions.get_mut(entity),
                    self.velocities.get(entity)
                ) {
                    pos.x += vel.dx;
                    pos.y += vel.dy;
                }
            }
        }
        
        fn print_entities(&self) {
            for entity in &self.entities {
                println!("  实体 {}:", entity);
                if let Some(pos) = self.positions.get(entity) {
                    println!("    位置: {:?}", pos);
                }
                if let Some(vel) = self.velocities.get(entity) {
                    println!("    速度: {:?}", vel);
                }
                if let Some(health) = self.healths.get(entity) {
                    println!("    生命值: {:?}", health);
                }
            }
        }
    }
    
    let mut world = World::new();
    
    // 创建玩家实体
    let player = world.create_entity();
    world.add_position(player, Position { x: 0.0, y: 0.0 });
    world.add_velocity(player, Velocity { dx: 1.0, dy: 0.5 });
    world.add_health(player, Health { current: 100, max: 100 });
    
    // 创建敌人实体
    let enemy = world.create_entity();
    world.add_position(enemy, Position { x: 10.0, y: 10.0 });
    world.add_velocity(enemy, Velocity { dx: -0.5, dy: -0.3 });
    world.add_health(enemy, Health { current: 50, max: 50 });
    
    println!("  初始状态:");
    world.print_entities();
    
    // 运行移动系统
    world.movement_system();
    
    println!("\n  移动后状态:");
    world.print_entities();
    
    println!("  ✅ HashMap 适合实现 ECS 的组件存储");
}

/// 游戏资源管理
fn game_resource_management() {
    #[derive(Debug, Clone)]
    struct Texture {
        id: String,
        width: u32,
        height: u32,
        data: Vec<u8>,
    }
    
    impl Texture {
        fn new(id: String, width: u32, height: u32) -> Self {
            Texture {
                id,
                width,
                height,
                data: vec![0; (width * height * 4) as usize], // RGBA
            }
        }
    }
    
    #[derive(Debug, Clone)]
    struct Sound {
        id: String,
        duration: f32,
        data: Vec<f32>,
    }
    
    impl Sound {
        fn new(id: String, duration: f32) -> Self {
            Sound {
                id,
                duration,
                data: vec![0.0; (duration * 44100.0) as usize], // 44.1kHz
            }
        }
    }
    
    struct ResourceManager {
        textures: Arc<RwLock<HashMap<String, Arc<Texture>>>>,
        sounds: Arc<RwLock<HashMap<String, Arc<Sound>>>>,
    }
    
    impl ResourceManager {
        fn new() -> Self {
            ResourceManager {
                textures: Arc::new(RwLock::new(HashMap::new())),
                sounds: Arc::new(RwLock::new(HashMap::new())),
            }
        }
        
        fn load_texture(&self, id: String, width: u32, height: u32) {
            let texture = Arc::new(Texture::new(id.clone(), width, height));
            self.textures.write().unwrap().insert(id.clone(), texture);
            println!("  加载纹理: {} ({}x{})", id, width, height);
        }
        
        fn load_sound(&self, id: String, duration: f32) {
            let sound = Arc::new(Sound::new(id.clone(), duration));
            self.sounds.write().unwrap().insert(id.clone(), sound);
            println!("  加载音效: {} ({:.1}s)", id, duration);
        }
        
        fn get_texture(&self, id: &str) -> Option<Arc<Texture>> {
            self.textures.read().unwrap().get(id).cloned()
        }
        
        fn get_sound(&self, id: &str) -> Option<Arc<Sound>> {
            self.sounds.read().unwrap().get(id).cloned()
        }
        
        fn resource_count(&self) -> (usize, usize) {
            let textures = self.textures.read().unwrap().len();
            let sounds = self.sounds.read().unwrap().len();
            (textures, sounds)
        }
        
        fn clone_manager(&self) -> Self {
            ResourceManager {
                textures: Arc::clone(&self.textures),
                sounds: Arc::clone(&self.sounds),
            }
        }
    }
    
    let resource_manager = ResourceManager::new();
    
    // 加载资源
    resource_manager.load_texture("player_sprite".to_string(), 64, 64);
    resource_manager.load_texture("enemy_sprite".to_string(), 32, 32);
    resource_manager.load_texture("background".to_string(), 1920, 1080);
    
    resource_manager.load_sound("jump_sound".to_string(), 0.5);
    resource_manager.load_sound("background_music".to_string(), 120.0);
    
    // 在不同线程中使用资源
    let mut handles = vec![];
    
    for i in 0..3 {
        let manager = resource_manager.clone_manager();
        let handle = thread::spawn(move || {
            if let Some(texture) = manager.get_texture("player_sprite") {
                println!("  线程 {} 使用纹理: {} ({}x{})", 
                         i, texture.id, texture.width, texture.height);
            }
            
            if let Some(sound) = manager.get_sound("jump_sound") {
                println!("  线程 {} 播放音效: {} ({:.1}s)", 
                         i, sound.id, sound.duration);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let (texture_count, sound_count) = resource_manager.resource_count();
    println!("  资源统计: {} 个纹理, {} 个音效", texture_count, sound_count);
    
    println!("  ✅ Arc<RwLock<HashMap<T>>> 适合多线程资源管理");
}

/// FFI 交互案例
fn ffi_interaction_examples() {
    println!("\n🔗 7. FFI 交互案例");
    println!("{}", "-".repeat(40));
    
    // C 字符串处理
    println!("\n📝 C 字符串处理:");
    c_string_handling();
    
    // 回调函数
    println!("\n📞 回调函数:");
    callback_functions();
    
    // 结构体传递
    println!("\n📦 结构体传递:");
    struct_passing();
}

/// C 字符串处理
fn c_string_handling() {
    // 模拟 C API
    unsafe fn c_strlen(s: *const c_char) -> usize {
        let mut len = 0;
        let mut ptr = s;
        while *ptr != 0 {
            len += 1;
            ptr = ptr.offset(1);
        }
        len
    }
    
    unsafe fn c_strcpy(dest: *mut c_char, src: *const c_char) {
        let mut i = 0;
        loop {
            let ch = *src.offset(i);
            *dest.offset(i) = ch;
            if ch == 0 {
                break;
            }
            i += 1;
        }
    }
    
    // Rust 字符串转 C 字符串
    let rust_string = "Hello, FFI!";
    let c_string = CString::new(rust_string).unwrap();
    let c_ptr = c_string.as_ptr();
    
    unsafe {
        let len = c_strlen(c_ptr);
        println!("  C 字符串长度: {}", len);
        
        // 分配缓冲区并复制
        let buffer_size = len + 1;
        let layout = std::alloc::Layout::array::<c_char>(buffer_size).unwrap();
        let buffer = std::alloc::alloc(layout) as *mut c_char;
        
        if !buffer.is_null() {
            c_strcpy(buffer, c_ptr);
            
            // 转换回 Rust 字符串
            let c_str = CStr::from_ptr(buffer);
            if let Ok(rust_str) = c_str.to_str() {
                println!("  复制的字符串: {}", rust_str);
            }
            
            std::alloc::dealloc(buffer as *mut u8, layout);
        }
    }
    
    println!("  ✅ 裸指针适合与 C API 交互");
}

/// 回调函数
fn callback_functions() {
    // 定义回调函数类型
    type Callback = unsafe extern "C" fn(i32) -> i32;
    
    // 模拟 C API，接受回调函数
    unsafe fn c_process_array(arr: *const i32, len: usize, callback: Callback) -> i32 {
        let mut sum = 0;
        for i in 0..len {
            let value = *arr.offset(i as isize);
            sum += callback(value);
        }
        sum
    }
    
    // Rust 回调函数
    unsafe extern "C" fn square_callback(x: i32) -> i32 {
        x * x
    }
    
    unsafe extern "C" fn double_callback(x: i32) -> i32 {
        x * 2
    }
    
    let numbers = [1, 2, 3, 4, 5];
    
    unsafe {
        let sum_of_squares = c_process_array(
            numbers.as_ptr(),
            numbers.len(),
            square_callback
        );
        
        let sum_of_doubles = c_process_array(
            numbers.as_ptr(),
            numbers.len(),
            double_callback
        );
        
        println!("  数组: {:?}", numbers);
        println!("  平方和: {}", sum_of_squares);
        println!("  双倍和: {}", sum_of_doubles);
    }
    
    println!("  ✅ 函数指针适合实现 C 回调机制");
}

/// 结构体传递
fn struct_passing() {
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: f64,
        y: f64,
    }
    
    #[repr(C)]
    #[derive(Debug)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }
    
    // 模拟 C API
    unsafe fn c_calculate_area(rect: *const Rectangle) -> f64 {
        let rect_ref = &*rect;
        let width = rect_ref.bottom_right.x - rect_ref.top_left.x;
        let height = rect_ref.bottom_right.y - rect_ref.top_left.y;
        width * height
    }
    
    unsafe fn c_move_point(point: *mut Point, dx: f64, dy: f64) {
        (*point).x += dx;
        (*point).y += dy;
    }
    
    let mut rect = Rectangle {
        top_left: Point { x: 0.0, y: 0.0 },
        bottom_right: Point { x: 10.0, y: 5.0 },
    };
    
    println!("  原始矩形: {:?}", rect);
    
    unsafe {
        let area = c_calculate_area(&rect);
        println!("  矩形面积: {}", area);
        
        // 移动左上角点
        c_move_point(&mut rect.top_left, 2.0, 1.0);
        println!("  移动后矩形: {:?}", rect);
        
        let new_area = c_calculate_area(&rect);
        println!("  新面积: {}", new_area);
    }
    
    println!("  ✅ #[repr(C)] 确保结构体布局与 C 兼容");
}

/// 性能优化案例
fn performance_optimization_examples() {
    println!("\n⚡ 8. 性能优化案例");
    println!("{}", "-".repeat(40));
    
    // 零拷贝字符串处理
    println!("\n📄 零拷贝字符串处理:");
    zero_copy_string_processing();
    
    // 内存池优化
    println!("\n🏊 对象池优化:");
    object_pool_optimization();
    
    // 缓存友好的数据结构
    println!("\n💾 缓存友好的数据结构:");
    cache_friendly_structures();
}

/// 零拷贝字符串处理
fn zero_copy_string_processing() {
    fn process_data(data: Cow<str>) -> Cow<str> {
        if data.contains("ERROR") {
            // 需要修改，创建新的 String
            Cow::Owned(data.replace("ERROR", "WARNING"))
        } else {
            // 不需要修改，直接返回借用
            data
        }
    }
    
    // 测试不同的输入
    let static_str = "This is a normal message";
    let error_string = String::from("This is an ERROR message");
    let another_error = "Another ERROR occurred";
    
    println!("  处理静态字符串:");
    let result1 = process_data(Cow::Borrowed(static_str));
    println!("    输入: {}", static_str);
    println!("    输出: {} (类型: {})", result1, 
             if matches!(result1, Cow::Borrowed(_)) { "借用" } else { "拥有" });
    
    println!("  处理包含错误的 String:");
    let result2 = process_data(Cow::Owned(error_string.clone()));
    println!("    输入: {}", error_string);
    println!("    输出: {} (类型: {})", result2,
             if matches!(result2, Cow::Borrowed(_)) { "借用" } else { "拥有" });
    
    println!("  处理包含错误的静态字符串:");
    let result3 = process_data(Cow::Borrowed(another_error));
    println!("    输入: {}", another_error);
    println!("    输出: {} (类型: {})", result3,
             if matches!(result3, Cow::Borrowed(_)) { "借用" } else { "拥有" });
    
    println!("  ✅ Cow 实现零拷贝优化，按需分配");
}

/// 对象池优化
fn object_pool_optimization() {
    #[derive(Debug)]
    struct ExpensiveObject {
        id: usize,
        data: Vec<u8>,
        processed: bool,
    }
    
    impl ExpensiveObject {
        fn new(id: usize) -> Self {
            ExpensiveObject {
                id,
                data: vec![0; 1024], // 模拟大对象
                processed: false,
            }
        }
        
        fn reset(&mut self, new_id: usize) {
            self.id = new_id;
            self.data.fill(0);
            self.processed = false;
        }
        
        fn process(&mut self) {
            // 模拟处理
            for i in 0..self.data.len() {
                self.data[i] = (i % 256) as u8;
            }
            self.processed = true;
        }
    }
    
    struct ObjectPool {
        available: RefCell<Vec<ExpensiveObject>>,
        next_id: Cell<usize>,
    }
    
    impl ObjectPool {
        fn new(initial_size: usize) -> Self {
            let mut available = Vec::new();
            for i in 0..initial_size {
                available.push(ExpensiveObject::new(i));
            }
            
            ObjectPool {
                available: RefCell::new(available),
                next_id: Cell::new(initial_size),
            }
        }
        
        fn acquire(&self) -> ExpensiveObject {
            let mut available = self.available.borrow_mut();
            if let Some(mut obj) = available.pop() {
                let id = self.next_id.get();
                obj.reset(id);
                self.next_id.set(id + 1);
                obj
            } else {
                // 池中没有可用对象，创建新的
                let id = self.next_id.get();
                self.next_id.set(id + 1);
                ExpensiveObject::new(id)
            }
        }
        
        fn release(&self, obj: ExpensiveObject) {
            self.available.borrow_mut().push(obj);
        }
        
        fn available_count(&self) -> usize {
            self.available.borrow().len()
        }
    }
    
    let pool = ObjectPool::new(3);
    println!("  初始池大小: {}", pool.available_count());
    
    // 获取和使用对象
    let mut obj1 = pool.acquire();
    let mut obj2 = pool.acquire();
    println!("  获取 2 个对象后池大小: {}", pool.available_count());
    
    obj1.process();
    obj2.process();
    println!("  对象1 处理状态: {}, ID: {}", obj1.processed, obj1.id);
    println!("  对象2 处理状态: {}, ID: {}", obj2.processed, obj2.id);
    
    // 归还对象
    pool.release(obj1);
    pool.release(obj2);
    println!("  归还对象后池大小: {}", pool.available_count());
    
    // 再次获取（应该重用对象）
    let mut obj3 = pool.acquire();
    println!("  重用对象 ID: {}, 处理状态: {}", obj3.id, obj3.processed);
    
    println!("  ✅ 对象池减少内存分配，提高性能");
}

/// 缓存友好的数据结构
fn cache_friendly_structures() {
    // 结构体数组 (SoA - Structure of Arrays)
    #[derive(Debug)]
    struct ParticleSystemSoA {
        positions_x: Vec<f32>,
        positions_y: Vec<f32>,
        velocities_x: Vec<f32>,
        velocities_y: Vec<f32>,
        masses: Vec<f32>,
    }
    
    impl ParticleSystemSoA {
        fn new(capacity: usize) -> Self {
            ParticleSystemSoA {
                positions_x: Vec::with_capacity(capacity),
                positions_y: Vec::with_capacity(capacity),
                velocities_x: Vec::with_capacity(capacity),
                velocities_y: Vec::with_capacity(capacity),
                masses: Vec::with_capacity(capacity),
            }
        }
        
        fn add_particle(&mut self, x: f32, y: f32, vx: f32, vy: f32, mass: f32) {
            self.positions_x.push(x);
            self.positions_y.push(y);
            self.velocities_x.push(vx);
            self.velocities_y.push(vy);
            self.masses.push(mass);
        }
        
        fn update_positions(&mut self) {
            // 缓存友好：连续访问同类型数据
            for i in 0..self.positions_x.len() {
                self.positions_x[i] += self.velocities_x[i];
                self.positions_y[i] += self.velocities_y[i];
            }
        }
        
        fn count(&self) -> usize {
            self.positions_x.len()
        }
    }
    
    // 数组结构体 (AoS - Array of Structures)
    #[derive(Debug, Clone, Copy)]
    struct Particle {
        position_x: f32,
        position_y: f32,
        velocity_x: f32,
        velocity_y: f32,
        mass: f32,
    }
    
    struct ParticleSystemAoS {
        particles: Vec<Particle>,
    }
    
    impl ParticleSystemAoS {
        fn new() -> Self {
            ParticleSystemAoS {
                particles: Vec::new(),
            }
        }
        
        fn add_particle(&mut self, x: f32, y: f32, vx: f32, vy: f32, mass: f32) {
            self.particles.push(Particle {
                position_x: x,
                position_y: y,
                velocity_x: vx,
                velocity_y: vy,
                mass,
            });
        }
        
        fn update_positions(&mut self) {
            // 可能不够缓存友好：跳跃访问不同字段
            for particle in &mut self.particles {
                particle.position_x += particle.velocity_x;
                particle.position_y += particle.velocity_y;
            }
        }
        
        fn count(&self) -> usize {
            self.particles.len()
        }
    }
    
    // 比较两种数据布局
    let mut soa_system = ParticleSystemSoA::new(1000);
    let mut aos_system = ParticleSystemAoS::new();
    
    // 添加相同的粒子数据
    for i in 0..1000 {
        let x = i as f32;
        let y = (i * 2) as f32;
        let vx = 1.0;
        let vy = 0.5;
        let mass = 1.0;
        
        soa_system.add_particle(x, y, vx, vy, mass);
        aos_system.add_particle(x, y, vx, vy, mass);
    }
    
    println!("  SoA 系统粒子数: {}", soa_system.count());
    println!("  AoS 系统粒子数: {}", aos_system.count());
    
    // 更新位置（在实际应用中，SoA 通常更快）
    soa_system.update_positions();
    aos_system.update_positions();
    
    println!("  SoA 前几个粒子位置: ({:.1}, {:.1}), ({:.1}, {:.1})",
             soa_system.positions_x[0], soa_system.positions_y[0],
             soa_system.positions_x[1], soa_system.positions_y[1]);
    
    println!("  AoS 前几个粒子位置: ({:.1}, {:.1}), ({:.1}, {:.1})",
             aos_system.particles[0].position_x, aos_system.particles[0].position_y,
             aos_system.particles[1].position_x, aos_system.particles[1].position_y);
    
    println!("  ✅ SoA 布局通常更缓存友好，适合 SIMD 优化");
}

/// 内存池管理案例
fn memory_pool_examples() {
    println!("\n🏊 9. 内存池管理案例");
    println!("{}", "-".repeat(40));
    
    // 固定大小内存池
    println!("\n🔲 固定大小内存池:");
    fixed_size_memory_pool();
    
    // 分层内存池
    println!("\n📚 分层内存池:");
    tiered_memory_pool();
}

/// 固定大小内存池
fn fixed_size_memory_pool() {
    struct FixedSizePool {
        memory: Vec<u8>,
        block_size: usize,
        free_blocks: RefCell<Vec<*mut u8>>,
    }
    
    impl FixedSizePool {
        fn new(block_size: usize, block_count: usize) -> Self {
            let total_size = block_size * block_count;
            let mut memory = vec![0u8; total_size];
            let mut free_blocks = Vec::new();
            
            // 初始化空闲块链表
            for i in 0..block_count {
                let block_ptr = unsafe { memory.as_mut_ptr().add(i * block_size) };
                free_blocks.push(block_ptr);
            }
            
            FixedSizePool {
                memory,
                block_size,
                free_blocks: RefCell::new(free_blocks),
            }
        }
        
        fn allocate(&self) -> Option<*mut u8> {
            self.free_blocks.borrow_mut().pop()
        }
        
        fn deallocate(&self, ptr: *mut u8) {
            // 简单验证指针是否在池范围内
            let pool_start = self.memory.as_ptr() as usize;
            let pool_end = pool_start + self.memory.len();
            let ptr_addr = ptr as usize;
            
            if ptr_addr >= pool_start && ptr_addr < pool_end {
                self.free_blocks.borrow_mut().push(ptr);
            }
        }
        
        fn available_blocks(&self) -> usize {
            self.free_blocks.borrow().len()
        }
        
        fn total_blocks(&self) -> usize {
            self.memory.len() / self.block_size
        }
    }
    
    let pool = FixedSizePool::new(64, 10);
    println!("  创建内存池: {} 个 {} 字节的块", pool.total_blocks(), 64);
    println!("  可用块数: {}", pool.available_blocks());
    
    // 分配一些块
    let mut allocated_blocks = Vec::new();
    for i in 0..5 {
        if let Some(ptr) = pool.allocate() {
            allocated_blocks.push(ptr);
            println!("  分配块 {}: {:p}", i + 1, ptr);
        }
    }
    
    println!("  分配后可用块数: {}", pool.available_blocks());
    
    // 释放一些块
    for (i, ptr) in allocated_blocks.iter().take(3).enumerate() {
        pool.deallocate(*ptr);
        println!("  释放块 {}: {:p}", i + 1, ptr);
    }
    
    println!("  释放后可用块数: {}", pool.available_blocks());
    
    println!("  ✅ 固定大小内存池提供 O(1) 分配和释放");
}

/// 分层内存池
fn tiered_memory_pool() {
    struct TieredPool {
        small_pool: FixedSizePool,   // 32 字节
        medium_pool: FixedSizePool,  // 128 字节
        large_pool: FixedSizePool,   // 512 字节
    }
    
    struct FixedSizePool {
        memory: Vec<u8>,
        block_size: usize,
        free_blocks: RefCell<Vec<*mut u8>>,
    }
    
    impl FixedSizePool {
        fn new(block_size: usize, block_count: usize) -> Self {
            let total_size = block_size * block_count;
            let mut memory = vec![0u8; total_size];
            let mut free_blocks = Vec::new();
            
            for i in 0..block_count {
                let block_ptr = unsafe { memory.as_mut_ptr().add(i * block_size) };
                free_blocks.push(block_ptr);
            }
            
            FixedSizePool {
                memory,
                block_size,
                free_blocks: RefCell::new(free_blocks),
            }
        }
        
        fn allocate(&self) -> Option<*mut u8> {
            self.free_blocks.borrow_mut().pop()
        }
        
        fn deallocate(&self, ptr: *mut u8) {
            self.free_blocks.borrow_mut().push(ptr);
        }
        
        fn available_blocks(&self) -> usize {
            self.free_blocks.borrow().len()
        }
    }
    
    impl TieredPool {
        fn new() -> Self {
            TieredPool {
                small_pool: FixedSizePool::new(32, 100),
                medium_pool: FixedSizePool::new(128, 50),
                large_pool: FixedSizePool::new(512, 20),
            }
        }
        
        fn allocate(&self, size: usize) -> Option<(*mut u8, usize)> {
            if size <= 32 {
                self.small_pool.allocate().map(|ptr| (ptr, 32))
            } else if size <= 128 {
                self.medium_pool.allocate().map(|ptr| (ptr, 128))
            } else if size <= 512 {
                self.large_pool.allocate().map(|ptr| (ptr, 512))
            } else {
                None // 超出最大块大小
            }
        }
        
        fn deallocate(&self, ptr: *mut u8, size: usize) {
            if size <= 32 {
                self.small_pool.deallocate(ptr);
            } else if size <= 128 {
                self.medium_pool.deallocate(ptr);
            } else if size <= 512 {
                self.large_pool.deallocate(ptr);
            }
        }
        
        fn stats(&self) -> (usize, usize, usize) {
            (
                self.small_pool.available_blocks(),
                self.medium_pool.available_blocks(),
                self.large_pool.available_blocks(),
            )
        }
    }
    
    let pool = TieredPool::new();
    let (small, medium, large) = pool.stats();
    println!("  分层池初始状态: 小块={}, 中块={}, 大块={}", small, medium, large);
    
    // 分配不同大小的内存
    let allocations = vec![
        ("小对象", 20),
        ("中对象", 100),
        ("大对象", 400),
        ("另一个小对象", 16),
    ];
    
    let mut allocated = Vec::new();
    
    for (name, size) in allocations {
        if let Some((ptr, actual_size)) = pool.allocate(size) {
            allocated.push((ptr, actual_size));
            println!("  分配 {}: 请求 {} 字节, 实际 {} 字节, 地址 {:p}", 
                     name, size, actual_size, ptr);
        } else {
            println!("  分配 {} 失败: 请求 {} 字节", name, size);
        }
    }
    
    let (small, medium, large) = pool.stats();
    println!("  分配后状态: 小块={}, 中块={}, 大块={}", small, medium, large);
    
    // 释放内存
    for (ptr, size) in allocated {
        pool.deallocate(ptr, size);
    }
    
    let (small, medium, large) = pool.stats();
    println!("  释放后状态: 小块={}, 中块={}, 大块={}", small, medium, large);
    
    println!("  ✅ 分层内存池根据大小选择合适的池");
}

/// 最佳实践总结
fn best_practices_summary() {
    println!("\n📋 10. 最佳实践总结");
    println!("{}", "=".repeat(60));
    
    println!("\n🎯 选择指导原则:");
    println!("  1. 默认使用引用 (&T, &mut T)");
    println!("     - 安全、零成本、编译时检查");
    println!("     - 适合大多数日常编程场景");
    
    println!("\n  2. 需要所有权时使用智能指针:");
    println!("     - Box<T>: 堆分配、递归结构");
    println!("     - Rc<T>: 单线程共享所有权");
    println!("     - Arc<T>: 多线程共享所有权");
    println!("     - RefCell<T>: 内部可变性");
    println!("     - Mutex<T>/RwLock<T>: 线程安全的内部可变性");
    
    println!("\n  3. 系统编程时谨慎使用裸指针:");
    println!("     - FFI 交互");
    println!("     - 自定义内存管理");
    println!("     - 性能关键的底层操作");
    println!("     - 必须在 unsafe 块中使用");
    
    println!("\n🔒 安全性考虑:");
    println!("  • 引用: 编译时保证安全");
    println!("  • 智能指针: 运行时检查 + RAII");
    println!("  • 裸指针: 程序员负责安全性");
    
    println!("\n⚡ 性能特征:");
    println!("  • 引用: 零成本抽象");
    println!("  • Box<T>: 一次间接访问");
    println!("  • Rc<T>: 引用计数开销");
    println!("  • Arc<T>: 原子引用计数开销");
    println!("  • RefCell<T>: 运行时借用检查");
    println!("  • 裸指针: 直接内存访问");
    
    println!("\n🧵 并发编程:");
    println!("  • 引用: 受借用检查器限制");
    println!("  • Arc<T>: 多线程共享");
    println!("  • Mutex<T>/RwLock<T>: 线程安全");
    println!("  • 裸指针: 需要手动同步");
    
    println!("\n💡 实用建议:");
    println!("  1. 从引用开始，根据需要升级到智能指针");
    println!("  2. 优先使用 Rust 的安全抽象");
    println!("  3. 只在必要时使用 unsafe 和裸指针");
    println!("  4. 使用类型系统表达设计意图");
    println!("  5. 利用编译器的帮助发现问题");
    
    println!("\n🎉 恭喜！你已经掌握了 Rust 中指针和引用的精髓！");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_linked_list() {
        // 测试链表基本功能
        #[derive(Debug)]
        struct Node<T> {
            data: T,
            next: Option<Box<Node<T>>>,
        }
        
        impl<T> Node<T> {
            fn new(data: T) -> Self {
                Node { data, next: None }
            }
            
            fn append(&mut self, data: T) {
                match &mut self.next {
                    None => self.next = Some(Box::new(Node::new(data))),
                    Some(next_node) => next_node.append(data),
                }
            }
            
            fn len(&self) -> usize {
                1 + self.next.as_ref().map_or(0, |node| node.len())
            }
        }
        
        let mut head = Node::new(1);
        head.append(2);
        head.append(3);
        
        assert_eq!(head.len(), 3);
        assert_eq!(head.data, 1);
    }
    
    #[test]
    fn test_object_pool() {
        struct SimplePool<T> {
            items: RefCell<Vec<T>>,
        }
        
        impl<T> SimplePool<T> {
            fn new() -> Self {
                SimplePool {
                    items: RefCell::new(Vec::new()),
                }
            }
            
            fn acquire(&self) -> Option<T> {
                self.items.borrow_mut().pop()
            }
            
            fn release(&self, item: T) {
                self.items.borrow_mut().push(item);
            }
            
            fn len(&self) -> usize {
                self.items.borrow().len()
            }
        }
        
        let pool = SimplePool::new();
        assert_eq!(pool.len(), 0);
        
        pool.release(String::from("test"));
        assert_eq!(pool.len(), 1);
        
        let item = pool.acquire().unwrap();
        assert_eq!(item, "test");
        assert_eq!(pool.len(), 0);
    }
    
    #[test]
    fn test_cow_optimization() {
        fn maybe_modify(s: Cow<str>) -> Cow<str> {
            if s.len() > 10 {
                Cow::Owned(format!("Long: {}", s))
            } else {
                s
            }
        }
        
        let short = "short";
        let result1 = maybe_modify(Cow::Borrowed(short));
        assert!(matches!(result1, Cow::Borrowed(_)));
        
        let long = "this is a very long string";
        let result2 = maybe_modify(Cow::Borrowed(long));
        assert!(matches!(result2, Cow::Owned(_)));
    }
}