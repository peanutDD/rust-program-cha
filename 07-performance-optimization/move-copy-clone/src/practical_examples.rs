//! # 实际应用案例和最佳实践
//!
//! 本模块提供 Move、Copy 和 Clone 在实际项目中的应用案例，
//! 展示最佳实践和常见设计模式。

use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

/// 实际应用案例演示
pub fn practical_examples() {
    println!("=== 实际应用案例和最佳实践 ===");
    
    // 1. 游戏开发案例
    println!("\n🎮 游戏开发案例:");
    game_development_examples();
    
    // 2. Web 服务器案例
    println!("\n🌐 Web 服务器案例:");
    web_server_examples();
    
    // 3. 数据处理管道案例
    println!("\n📊 数据处理管道案例:");
    data_pipeline_examples();
    
    // 4. GUI 应用案例
    println!("\n🖥️ GUI 应用案例:");
    gui_application_examples();
    
    // 5. 并发编程案例
    println!("\n🔄 并发编程案例:");
    concurrent_programming_examples();
    
    // 6. 设计模式案例
    println!("\n🏗️ 设计模式案例:");
    design_pattern_examples();
}

/// 游戏开发案例
fn game_development_examples() {
    println!("\n1️⃣ 游戏实体系统 (ECS):");
    
    // 组件使用 Copy（小型数据）
    #[derive(Debug, Copy, Clone)]
    struct Position {
        x: f32,
        y: f32,
        z: f32,
    }
    
    #[derive(Debug, Copy, Clone)]
    struct Velocity {
        dx: f32,
        dy: f32,
        dz: f32,
    }
    
    #[derive(Debug, Copy, Clone)]
    struct Health {
        current: u32,
        max: u32,
    }
    
    // 复杂组件使用 Clone（大型数据）
    #[derive(Debug, Clone)]
    struct Inventory {
        items: Vec<String>,
        capacity: usize,
        gold: u32,
    }
    
    #[derive(Debug, Clone)]
    struct AIBehavior {
        state_machine: HashMap<String, String>,
        decision_tree: Vec<String>,
        memory: VecDeque<String>,
    }
    
    // 实体使用 Move（资源管理）
    #[derive(Debug)]
    struct GameEntity {
        id: u64,
        position: Position,
        velocity: Option<Velocity>,
        health: Option<Health>,
        inventory: Option<Inventory>,
        ai: Option<AIBehavior>,
    }
    
    impl GameEntity {
        fn new(id: u64, pos: Position) -> Self {
            GameEntity {
                id,
                position: pos,
                velocity: None,
                health: None,
                inventory: None,
                ai: None,
            }
        }
        
        // 使用 Move 添加组件
        fn with_velocity(mut self, vel: Velocity) -> Self {
            self.velocity = Some(vel);
            self // Move self
        }
        
        fn with_health(mut self, health: Health) -> Self {
            self.health = Some(health);
            self // Move self
        }
        
        // Clone 用于复制复杂组件
        fn with_inventory(mut self, inventory: Inventory) -> Self {
            self.inventory = Some(inventory);
            self // Move self
        }
    }
    
    // 游戏系统
    struct MovementSystem;
    
    impl MovementSystem {
        // Copy 组件可以直接传递
        fn update(entities: &mut [GameEntity], delta_time: f32) {
            for entity in entities {
                if let Some(vel) = &entity.velocity {
                    // Copy 类型可以直接使用
                    entity.position.x += vel.dx * delta_time;
                    entity.position.y += vel.dy * delta_time;
                    entity.position.z += vel.dz * delta_time;
                }
            }
        }
    }
    
    // 创建游戏实体
    let player = GameEntity::new(
        1,
        Position { x: 0.0, y: 0.0, z: 0.0 }
    )
    .with_velocity(Velocity { dx: 1.0, dy: 0.0, dz: 0.0 })
    .with_health(Health { current: 100, max: 100 })
    .with_inventory(Inventory {
        items: vec!["sword".to_string(), "potion".to_string()],
        capacity: 20,
        gold: 100,
    });
    
    println!("   ✅ 玩家实体创建: ID {}", player.id);
    println!("   📍 位置: {:?}", player.position);
    println!("   💰 物品数量: {}", player.inventory.as_ref().unwrap().items.len());
    
    // 创建 NPC（通过克隆模板）
    let npc_template = GameEntity::new(
        0,
        Position { x: 10.0, y: 0.0, z: 0.0 }
    )
    .with_health(Health { current: 50, max: 50 })
    .with_inventory(Inventory {
        items: vec!["basic_item".to_string()],
        capacity: 10,
        gold: 10,
    });
    
    // 使用 Clone 创建多个 NPC
    let mut npcs = Vec::new();
    for i in 2..5 {
        let mut npc = GameEntity {
            id: i,
            ..GameEntity::new(i, Position { x: i as f32 * 5.0, y: 0.0, z: 0.0 })
        };
        if let Some(template_inventory) = &npc_template.inventory {
            npc.inventory = Some(template_inventory.clone()); // Clone 模板
        }
        if let Some(template_health) = npc_template.health {
            npc.health = Some(template_health); // Copy 组件
        }
        npcs.push(npc);
    }
    
    println!("   🤖 创建了 {} 个 NPC", npcs.len());
    
    println!("\n   📝 设计要点:");
    println!("   • 小型组件(Position, Velocity)使用 Copy - 高效传递");
    println!("   • 复杂组件(Inventory, AI)使用 Clone - 灵活复制");
    println!("   • 实体使用 Move - 确保唯一所有权和资源管理");
}

/// Web 服务器案例
fn web_server_examples() {
    println!("\n1️⃣ HTTP 请求处理:");
    
    // 配置使用 Copy（频繁传递）
    #[derive(Debug, Copy, Clone)]
    struct ServerConfig {
        port: u16,
        max_connections: usize,
        timeout_ms: u64,
        keep_alive: bool,
    }
    
    // 请求数据使用 Clone（需要独立处理）
    #[derive(Debug, Clone)]
    struct HttpRequest {
        method: String,
        path: String,
        headers: HashMap<String, String>,
        body: Vec<u8>,
        timestamp: Instant,
    }
    
    // 响应构建器使用 Move（链式调用）
    #[derive(Debug)]
    struct HttpResponse {
        status_code: u16,
        headers: HashMap<String, String>,
        body: Vec<u8>,
    }
    
    impl HttpResponse {
        fn new() -> Self {
            HttpResponse {
                status_code: 200,
                headers: HashMap::new(),
                body: Vec::new(),
            }
        }
        
        fn status(mut self, code: u16) -> Self {
            self.status_code = code;
            self // Move
        }
        
        fn header(mut self, key: String, value: String) -> Self {
            self.headers.insert(key, value);
            self // Move
        }
        
        fn body(mut self, content: Vec<u8>) -> Self {
            self.body = content;
            self // Move
        }
    }
    
    // 请求处理器
    struct RequestHandler {
        config: ServerConfig, // Copy 类型，可以直接存储
    }
    
    impl RequestHandler {
        fn new(config: ServerConfig) -> Self {
            RequestHandler { config }
        }
        
        // 处理请求（Clone 用于并发处理）
        fn handle_request(&self, request: HttpRequest) -> HttpResponse {
            println!("   处理请求: {} {}", request.method, request.path);
            
            // 根据路径路由
            match request.path.as_str() {
                "/api/users" => self.handle_users_api(request),
                "/api/data" => self.handle_data_api(request),
                _ => HttpResponse::new()
                    .status(404)
                    .header("Content-Type".to_string(), "text/plain".to_string())
                    .body(b"Not Found".to_vec()),
            }
        }
        
        fn handle_users_api(&self, _request: HttpRequest) -> HttpResponse {
            let users_json = r#"[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]"#;
            
            HttpResponse::new()
                .status(200)
                .header("Content-Type".to_string(), "application/json".to_string())
                .header("Cache-Control".to_string(), "max-age=3600".to_string())
                .body(users_json.as_bytes().to_vec())
        }
        
        fn handle_data_api(&self, request: HttpRequest) -> HttpResponse {
            // Clone 请求用于日志记录
            let log_request = request.clone();
            println!("   📝 记录请求: {} 字节", log_request.body.len());
            
            // 处理原始请求
            let response_data = format!("Processed {} bytes", request.body.len());
            
            HttpResponse::new()
                .status(200)
                .header("Content-Type".to_string(), "text/plain".to_string())
                .body(response_data.as_bytes().to_vec())
        }
    }
    
    // 使用示例
    let config = ServerConfig {
        port: 8080,
        max_connections: 1000,
        timeout_ms: 30000,
        keep_alive: true,
    };
    
    let handler = RequestHandler::new(config); // Copy 配置
    
    // 模拟请求
    let request1 = HttpRequest {
        method: "GET".to_string(),
        path: "/api/users".to_string(),
        headers: {
            let mut headers = HashMap::new();
            headers.insert("Accept".to_string(), "application/json".to_string());
            headers
        },
        body: Vec::new(),
        timestamp: Instant::now(),
    };
    
    let request2 = HttpRequest {
        method: "POST".to_string(),
        path: "/api/data".to_string(),
        headers: HashMap::new(),
        body: b"Hello, World!".to_vec(),
        timestamp: Instant::now(),
    };
    
    // 处理请求（Move 到处理函数）
    let response1 = handler.handle_request(request1);
    let response2 = handler.handle_request(request2);
    
    println!("   ✅ 响应1状态: {}", response1.status_code);
    println!("   ✅ 响应2状态: {}", response2.status_code);
    
    println!("\n   📝 设计要点:");
    println!("   • 配置使用 Copy - 轻量级，频繁传递");
    println!("   • 请求/响应使用 Move - 避免不必要复制");
    println!("   • Clone 用于日志和并发处理");
}

/// 数据处理管道案例
fn data_pipeline_examples() {
    println!("\n1️⃣ 数据转换管道:");
    
    // 处理配置使用 Copy
    #[derive(Debug, Copy, Clone)]
    struct ProcessingConfig {
        batch_size: usize,
        timeout_ms: u64,
        retry_count: u32,
        parallel: bool,
    }
    
    // 数据记录使用 Clone（需要在管道中传递）
    #[derive(Debug, Clone)]
    struct DataRecord {
        id: u64,
        timestamp: u64,
        data: HashMap<String, String>,
        metadata: Vec<String>,
    }
    
    // 处理结果使用 Move（一次性传递）
    #[derive(Debug)]
    struct ProcessingResult {
        processed_count: usize,
        errors: Vec<String>,
        duration: Duration,
        output_data: Vec<DataRecord>,
    }
    
    // 数据处理管道
    struct DataPipeline {
        config: ProcessingConfig,
        processors: Vec<Box<dyn Fn(&DataRecord) -> Result<DataRecord, String>>>,
    }
    
    impl DataPipeline {
        fn new(config: ProcessingConfig) -> Self {
            DataPipeline {
                config,
                processors: Vec::new(),
            }
        }
        
        // 添加处理步骤（Move 闭包）
        fn add_processor<F>(mut self, processor: F) -> Self
        where
            F: Fn(&DataRecord) -> Result<DataRecord, String> + 'static,
        {
            self.processors.push(Box::new(processor));
            self // Move self
        }
        
        // 处理数据批次
        fn process_batch(&self, mut records: Vec<DataRecord>) -> ProcessingResult {
            let start_time = Instant::now();
            let mut errors = Vec::new();
            let mut processed_records = Vec::new();
            
            for record in records.drain(..) { // Move 每个记录
                let mut current_record = record;
                let mut processing_failed = false;
                
                // 通过管道处理
                for processor in &self.processors {
                    match processor(&current_record) {
                        Ok(processed) => {
                            current_record = processed; // Move 处理结果
                        }
                        Err(error) => {
                            errors.push(format!("Record {}: {}", current_record.id, error));
                            processing_failed = true;
                            break;
                        }
                    }
                }
                
                if !processing_failed {
                    processed_records.push(current_record); // Move 到结果
                }
            }
            
            ProcessingResult {
                processed_count: processed_records.len(),
                errors,
                duration: start_time.elapsed(),
                output_data: processed_records,
            }
        }
    }
    
    // 创建处理管道
    let config = ProcessingConfig {
        batch_size: 100,
        timeout_ms: 5000,
        retry_count: 3,
        parallel: false,
    };
    
    let pipeline = DataPipeline::new(config)
        .add_processor(|record| {
            // 数据清洗
            let mut cleaned = record.clone();
            cleaned.data.retain(|_, v| !v.is_empty());
            Ok(cleaned)
        })
        .add_processor(|record| {
            // 数据验证
            if record.data.is_empty() {
                Err("Empty data after cleaning".to_string())
            } else {
                Ok(record.clone())
            }
        })
        .add_processor(|record| {
            // 数据转换
            let mut transformed = record.clone();
            for (key, value) in &mut transformed.data {
                *value = value.to_uppercase();
            }
            transformed.metadata.push("transformed".to_string());
            Ok(transformed)
        });
    
    // 创建测试数据
    let test_records = vec![
        DataRecord {
            id: 1,
            timestamp: 1640995200,
            data: {
                let mut data = HashMap::new();
                data.insert("name".to_string(), "alice".to_string());
                data.insert("email".to_string(), "alice@example.com".to_string());
                data
            },
            metadata: vec!["raw".to_string()],
        },
        DataRecord {
            id: 2,
            timestamp: 1640995260,
            data: {
                let mut data = HashMap::new();
                data.insert("name".to_string(), "bob".to_string());
                data.insert("email".to_string(), "".to_string()); // 空值，会被清洗
                data
            },
            metadata: vec!["raw".to_string()],
        },
        DataRecord {
            id: 3,
            timestamp: 1640995320,
            data: HashMap::new(), // 空数据，会导致验证失败
            metadata: vec!["raw".to_string()],
        },
    ];
    
    // 处理数据
    let result = pipeline.process_batch(test_records); // Move 数据到管道
    
    println!("   ✅ 处理完成:");
    println!("   📊 处理成功: {} 条记录", result.processed_count);
    println!("   ❌ 错误数量: {} 个", result.errors.len());
    println!("   ⏱️ 处理时间: {:?}", result.duration);
    
    for error in &result.errors {
        println!("   🚨 错误: {}", error);
    }
    
    for record in &result.output_data {
        println!("   📝 处理后记录 {}: {:?}", record.id, record.data);
    }
    
    println!("\n   📝 设计要点:");
    println!("   • 配置使用 Copy - 轻量级配置参数");
    println!("   • 记录使用 Clone - 在管道中需要多次处理");
    println!("   • 结果使用 Move - 一次性传递最终结果");
    println!("   • 管道构建使用 Move - 链式调用模式");
}

/// GUI 应用案例
fn gui_application_examples() {
    println!("\n1️⃣ GUI 组件系统:");
    
    // 样式属性使用 Copy（频繁使用）
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Color {
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    }
    
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    #[derive(Debug, Copy, Clone)]
    struct Size {
        width: u32,
        height: u32,
    }
    
    #[derive(Debug, Copy, Clone)]
    struct Rect {
        origin: Point,
        size: Size,
    }
    
    // 样式使用 Clone（可能包含复杂数据）
    #[derive(Debug, Clone)]
    struct Style {
        background_color: Color,
        border_color: Color,
        font_family: String,
        font_size: u16,
        padding: (u16, u16, u16, u16), // top, right, bottom, left
    }
    
    impl Style {
        fn default() -> Self {
            Style {
                background_color: Color { r: 255, g: 255, b: 255, a: 255 },
                border_color: Color { r: 0, g: 0, b: 0, a: 255 },
                font_family: "Arial".to_string(),
                font_size: 14,
                padding: (8, 8, 8, 8),
            }
        }
    }
    
    // 事件使用 Clone（需要传播）
    #[derive(Debug, Clone)]
    enum Event {
        Click { position: Point, button: u8 },
        KeyPress { key: String, modifiers: Vec<String> },
        Resize { new_size: Size },
        Custom { name: String, data: HashMap<String, String> },
    }
    
    // 组件使用 Move（唯一所有权）
    struct Widget {
        id: String,
        rect: Rect,
        style: Style,
        children: Vec<Widget>,
        event_handlers: HashMap<String, Box<dyn Fn(&Event) -> bool>>,
    }
    
    impl std::fmt::Debug for Widget {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Widget")
                .field("id", &self.id)
                .field("rect", &self.rect)
                .field("style", &self.style)
                .field("children", &self.children)
                .field("event_handlers", &format!("<{} handlers>", self.event_handlers.len()))
                .finish()
        }
    }
    
    impl Widget {
        fn new(id: String, rect: Rect) -> Self {
            Widget {
                id,
                rect,
                style: Style::default(),
                children: Vec::new(),
                event_handlers: HashMap::new(),
            }
        }
        
        // 链式调用使用 Move
        fn with_style(mut self, style: Style) -> Self {
            self.style = style;
            self
        }
        
        fn add_child(mut self, child: Widget) -> Self {
            self.children.push(child);
            self
        }
        
        fn on_click<F>(mut self, handler: F) -> Self
        where
            F: Fn(&Event) -> bool + 'static,
        {
            self.event_handlers.insert("click".to_string(), Box::new(handler));
            self
        }
        
        // 事件处理（Clone 事件进行传播）
        fn handle_event(&self, event: &Event) -> bool {
            // 处理自己的事件
            if let Some(handler) = self.event_handlers.get("click") {
                if matches!(event, Event::Click { .. }) {
                    if handler(event) {
                        return true; // 事件被消费
                    }
                }
            }
            
            // 传播给子组件
            for child in &self.children {
                if child.handle_event(event) {
                    return true; // 子组件消费了事件
                }
            }
            
            false // 事件未被处理
        }
        
        // 渲染（Copy 样式属性）
        fn render(&self, context: &mut RenderContext) {
            // 渲染背景
            context.fill_rect(self.rect, self.style.background_color);
            
            // 渲染边框
            context.stroke_rect(self.rect, self.style.border_color);
            
            // 渲染子组件
            for child in &self.children {
                child.render(context);
            }
        }
    }
    
    // 渲染上下文（模拟）
    struct RenderContext {
        operations: Vec<String>,
    }
    
    impl RenderContext {
        fn new() -> Self {
            RenderContext {
                operations: Vec::new(),
            }
        }
        
        fn fill_rect(&mut self, rect: Rect, color: Color) {
            self.operations.push(format!(
                "fill_rect({:?}, rgba({}, {}, {}, {}))",
                rect, color.r, color.g, color.b, color.a
            ));
        }
        
        fn stroke_rect(&mut self, rect: Rect, color: Color) {
            self.operations.push(format!(
                "stroke_rect({:?}, rgba({}, {}, {}, {}))",
                rect, color.r, color.g, color.b, color.a
            ));
        }
    }
    
    // 创建 GUI 应用
    let button_style = Style {
        background_color: Color { r: 70, g: 130, b: 180, a: 255 }, // 钢蓝色
        border_color: Color { r: 0, g: 0, b: 0, a: 255 },
        font_family: "Arial".to_string(),
        font_size: 16,
        padding: (10, 20, 10, 20),
    };
    
    let main_window = Widget::new(
        "main_window".to_string(),
        Rect {
            origin: Point { x: 0, y: 0 },
            size: Size { width: 800, height: 600 },
        },
    )
    .with_style(Style::default())
    .add_child(
        Widget::new(
            "button1".to_string(),
            Rect {
                origin: Point { x: 100, y: 100 },
                size: Size { width: 120, height: 40 },
            },
        )
        .with_style(button_style.clone()) // Clone 样式
        .on_click(|event| {
            if let Event::Click { position, .. } = event {
                println!("   🖱️ 按钮1被点击，位置: {:?}", position);
                true
            } else {
                false
            }
        })
    )
    .add_child(
        Widget::new(
            "button2".to_string(),
            Rect {
                origin: Point { x: 250, y: 100 },
                size: Size { width: 120, height: 40 },
            },
        )
        .with_style(button_style) // Move 样式
        .on_click(|event| {
            if let Event::Click { position, .. } = event {
                println!("   🖱️ 按钮2被点击，位置: {:?}", position);
                true
            } else {
                false
            }
        })
    );
    
    // 模拟事件处理
    let click_event = Event::Click {
        position: Point { x: 160, y: 120 },
        button: 1,
    };
    
    println!("   🎯 处理点击事件:");
    let handled = main_window.handle_event(&click_event);
    println!("   ✅ 事件被处理: {}", handled);
    
    // 模拟渲染
    let mut render_context = RenderContext::new();
    main_window.render(&mut render_context);
    
    println!("   🎨 渲染操作:");
    for (i, op) in render_context.operations.iter().enumerate() {
        println!("   {}. {}", i + 1, op);
    }
    
    println!("\n   📝 设计要点:");
    println!("   • 基础类型(Point, Size, Color)使用 Copy - 频繁传递");
    println!("   • 样式使用 Clone - 可以共享和定制");
    println!("   • 事件使用 Clone - 需要在组件树中传播");
    println!("   • 组件使用 Move - 确保唯一所有权和层次结构");
}

/// 并发编程案例
fn concurrent_programming_examples() {
    println!("\n1️⃣ 多线程数据处理:");
    
    // 共享配置使用 Copy（线程安全）
    #[derive(Debug, Copy, Clone)]
    struct WorkerConfig {
        worker_id: usize,
        batch_size: usize,
        sleep_ms: u64,
    }
    
    // 任务数据使用 Clone（需要分发到多个线程）
    #[derive(Debug, Clone)]
    struct Task {
        id: u64,
        data: Vec<u8>,
        priority: u8,
        metadata: HashMap<String, String>,
    }
    
    // 结果使用 Move（从线程返回）
    #[derive(Debug)]
    struct TaskResult {
        task_id: u64,
        worker_id: usize,
        result: Vec<u8>,
        processing_time: Duration,
    }
    
    // 工作线程函数
    fn worker_thread(config: WorkerConfig, tasks: Vec<Task>) -> Vec<TaskResult> {
        let mut results = Vec::new();
        
        for task in tasks { // Move 每个任务
            let start_time = Instant::now();
            
            // 模拟处理
            thread::sleep(Duration::from_millis(config.sleep_ms));
            
            // 处理数据（简单的数据转换）
            let processed_data: Vec<u8> = task.data
                .iter()
                .map(|&b| b.wrapping_add(1))
                .collect();
            
            let result = TaskResult {
                task_id: task.id,
                worker_id: config.worker_id,
                result: processed_data,
                processing_time: start_time.elapsed(),
            };
            
            results.push(result); // Move 结果
        }
        
        results // Move 所有结果
    }
    
    // 创建任务
    let tasks: Vec<Task> = (1..=12)
        .map(|i| Task {
            id: i,
            data: vec![i as u8; 10], // 简单的测试数据
            priority: (i % 3) as u8,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("source".to_string(), "generator".to_string());
                meta.insert("batch".to_string(), ((i - 1) / 4 + 1).to_string());
                meta
            },
        })
        .collect();
    
    println!("   📋 创建了 {} 个任务", tasks.len());
    
    // 分配任务到多个线程
    let num_workers = 3;
    let tasks_per_worker = tasks.len() / num_workers;
    
    let mut handles = Vec::new();
    
    for worker_id in 0..num_workers {
        let start_idx = worker_id * tasks_per_worker;
        let end_idx = if worker_id == num_workers - 1 {
            tasks.len()
        } else {
            (worker_id + 1) * tasks_per_worker
        };
        
        // Clone 任务分配给每个线程
        let worker_tasks: Vec<Task> = tasks[start_idx..end_idx]
            .iter()
            .cloned() // Clone 每个任务
            .collect();
        
        let config = WorkerConfig {
            worker_id,
            batch_size: worker_tasks.len(),
            sleep_ms: 10,
        };
        
        // 启动工作线程
        let handle = thread::spawn(move || {
            println!("   🔄 工作线程 {} 开始处理 {} 个任务", config.worker_id, config.batch_size);
            worker_thread(config, worker_tasks) // Move 配置和任务
        });
        
        handles.push(handle);
    }
    
    // 收集结果
    let mut all_results = Vec::new();
    for handle in handles {
        match handle.join() {
            Ok(mut results) => {
                println!("   ✅ 工作线程完成，返回 {} 个结果", results.len());
                all_results.append(&mut results); // Move 结果
            }
            Err(e) => {
                println!("   ❌ 工作线程失败: {:?}", e);
            }
        }
    }
    
    // 统计结果
    println!("   📊 处理统计:");
    println!("   总任务数: {}", tasks.len());
    println!("   完成任务数: {}", all_results.len());
    
    let total_time: Duration = all_results
        .iter()
        .map(|r| r.processing_time)
        .sum();
    
    let avg_time = total_time / all_results.len() as u32;
    println!("   平均处理时间: {:?}", avg_time);
    
    // 按工作线程分组统计
    let mut worker_stats: HashMap<usize, usize> = HashMap::new();
    for result in &all_results {
        *worker_stats.entry(result.worker_id).or_insert(0) += 1;
    }
    
    for (worker_id, count) in worker_stats {
        println!("   工作线程 {}: {} 个任务", worker_id, count);
    }
    
    println!("\n   📝 设计要点:");
    println!("   • 配置使用 Copy - 可以安全地在线程间传递");
    println!("   • 任务使用 Clone - 需要分发到多个线程");
    println!("   • 结果使用 Move - 从线程返回，避免复制");
    println!("   • 线程间通信通过所有权转移保证安全");
}

/// 设计模式案例
fn design_pattern_examples() {
    println!("\n1️⃣ 建造者模式 (Builder Pattern):");
    demonstrate_builder_pattern();
    
    println!("\n2️⃣ 原型模式 (Prototype Pattern):");
    demonstrate_prototype_pattern();
    
    println!("\n3️⃣ 享元模式 (Flyweight Pattern):");
    demonstrate_flyweight_pattern();
}

/// 演示建造者模式
fn demonstrate_builder_pattern() {
    // 配置参数使用 Copy（简单值）
    #[derive(Debug, Copy, Clone)]
    struct DatabaseConfig {
        max_connections: u32,
        timeout_seconds: u64,
        retry_attempts: u32,
    }
    
    impl Default for DatabaseConfig {
        fn default() -> Self {
            DatabaseConfig {
                max_connections: 10,
                timeout_seconds: 30,
                retry_attempts: 3,
            }
        }
    }
    
    // 复杂配置使用 Clone
    #[derive(Debug, Clone)]
    struct ServerConfig {
        host: String,
        port: u16,
        ssl_cert_path: Option<String>,
        allowed_origins: Vec<String>,
        middleware: Vec<String>,
    }
    
    // 最终产品使用 Move
    #[derive(Debug)]
    struct WebServer {
        server_config: ServerConfig,
        database_config: DatabaseConfig,
        routes: HashMap<String, String>,
        is_running: bool,
    }
    
    // 建造者使用 Move（链式调用）
    struct WebServerBuilder {
        server_config: ServerConfig,
        database_config: DatabaseConfig,
        routes: HashMap<String, String>,
    }
    
    impl WebServerBuilder {
        fn new() -> Self {
            WebServerBuilder {
                server_config: ServerConfig {
                    host: "localhost".to_string(),
                    port: 8080,
                    ssl_cert_path: None,
                    allowed_origins: vec!["*".to_string()],
                    middleware: Vec::new(),
                },
                database_config: DatabaseConfig::default(),
                routes: HashMap::new(),
            }
        }
        
        // 链式调用使用 Move
        fn host(mut self, host: String) -> Self {
            self.server_config.host = host;
            self
        }
        
        fn port(mut self, port: u16) -> Self {
            self.server_config.port = port;
            self
        }
        
        fn ssl_cert(mut self, cert_path: String) -> Self {
            self.server_config.ssl_cert_path = Some(cert_path);
            self
        }
        
        fn add_origin(mut self, origin: String) -> Self {
            if self.server_config.allowed_origins == vec!["*"] {
                self.server_config.allowed_origins.clear();
            }
            self.server_config.allowed_origins.push(origin);
            self
        }
        
        fn add_middleware(mut self, middleware: String) -> Self {
            self.server_config.middleware.push(middleware);
            self
        }
        
        fn database_config(mut self, config: DatabaseConfig) -> Self {
            self.database_config = config; // Copy
            self
        }
        
        fn add_route(mut self, path: String, handler: String) -> Self {
            self.routes.insert(path, handler);
            self
        }
        
        // 构建最终产品（Move 所有数据）
        fn build(self) -> WebServer {
            WebServer {
                server_config: self.server_config,
                database_config: self.database_config,
                routes: self.routes,
                is_running: false,
            }
        }
    }
    
    // 使用建造者模式
    let db_config = DatabaseConfig {
        max_connections: 50,
        timeout_seconds: 60,
        retry_attempts: 5,
    };
    
    let server = WebServerBuilder::new()
        .host("0.0.0.0".to_string())
        .port(3000)
        .ssl_cert("/path/to/cert.pem".to_string())
        .add_origin("https://example.com".to_string())
        .add_origin("https://api.example.com".to_string())
        .add_middleware("cors".to_string())
        .add_middleware("auth".to_string())
        .database_config(db_config) // Copy 配置
        .add_route("/api/users".to_string(), "users_handler".to_string())
        .add_route("/api/posts".to_string(), "posts_handler".to_string())
        .build(); // Move 构建
    
    println!("   ✅ Web服务器构建完成:");
    println!("   🌐 地址: {}:{}", server.server_config.host, server.server_config.port);
    println!("   🔒 SSL: {:?}", server.server_config.ssl_cert_path.is_some());
    println!("   🛡️ 中间件: {:?}", server.server_config.middleware);
    println!("   🗄️ 数据库连接数: {}", server.database_config.max_connections);
    println!("   🛣️ 路由数量: {}", server.routes.len());
    
    println!("\n   📝 建造者模式要点:");
    println!("   • 建造者使用 Move 支持链式调用");
    println!("   • 简单配置使用 Copy 便于传递");
    println!("   • 复杂配置使用 Clone 支持灵活组合");
    println!("   • 最终产品使用 Move 确保唯一所有权");
}

/// 演示原型模式
fn demonstrate_prototype_pattern() {
    // 原型对象使用 Clone
    #[derive(Debug, Clone)]
    struct DocumentTemplate {
        title: String,
        author: String,
        content_sections: Vec<String>,
        metadata: HashMap<String, String>,
        formatting: DocumentFormatting,
    }
    
    #[derive(Debug, Clone)]
    struct DocumentFormatting {
        font_family: String,
        font_size: u16,
        line_spacing: f32,
        margins: (u16, u16, u16, u16), // top, right, bottom, left
    }
    
    impl DocumentTemplate {
        fn new(title: String, author: String) -> Self {
            DocumentTemplate {
                title,
                author,
                content_sections: Vec::new(),
                metadata: HashMap::new(),
                formatting: DocumentFormatting {
                    font_family: "Times New Roman".to_string(),
                    font_size: 12,
                    line_spacing: 1.5,
                    margins: (72, 72, 72, 72), // 1 inch margins
                },
            }
        }
        
        fn add_section(&mut self, section: String) {
            self.content_sections.push(section);
        }
        
        fn set_metadata(&mut self, key: String, value: String) {
            self.metadata.insert(key, value);
        }
        
        // 原型克隆方法
        fn create_variant(&self, new_title: String) -> Self {
            let mut variant = self.clone(); // Clone 整个模板
            variant.title = new_title;
            variant.metadata.insert("created_from".to_string(), self.title.clone());
            variant
        }
        
        // 创建特定类型的文档
        fn create_report(&self, report_title: String) -> Self {
            let mut report = self.clone();
            report.title = report_title;
            report.formatting.font_family = "Arial".to_string();
            report.formatting.font_size = 11;
            report.add_section("Executive Summary".to_string());
            report.add_section("Methodology".to_string());
            report.add_section("Results".to_string());
            report.add_section("Conclusions".to_string());
            report.set_metadata("document_type".to_string(), "report".to_string());
            report
        }
        
        fn create_letter(&self, recipient: String) -> Self {
            let mut letter = self.clone();
            letter.title = format!("Letter to {}", recipient);
            letter.formatting.line_spacing = 1.0;
            letter.add_section("Greeting".to_string());
            letter.add_section("Body".to_string());
            letter.add_section("Closing".to_string());
            letter.set_metadata("document_type".to_string(), "letter".to_string());
            letter.set_metadata("recipient".to_string(), recipient);
            letter
        }
    }
    
    // 创建基础模板
    let mut base_template = DocumentTemplate::new(
        "Base Template".to_string(),
        "Template System".to_string(),
    );
    
    base_template.add_section("Introduction".to_string());
    base_template.add_section("Main Content".to_string());
    base_template.set_metadata("version".to_string(), "1.0".to_string());
    base_template.set_metadata("language".to_string(), "en".to_string());
    
    println!("   📄 基础模板创建完成: {}", base_template.title);
    
    // 使用原型模式创建不同类型的文档
    let quarterly_report = base_template.create_report(
        "Q4 2023 Financial Report".to_string()
    );
    
    let business_letter = base_template.create_letter(
        "John Smith".to_string()
    );
    
    let custom_variant = base_template.create_variant(
        "Custom Document".to_string()
    );
    
    println!("   📊 报告文档: {}", quarterly_report.title);
    println!("   📝 信件文档: {}", business_letter.title);
    println!("   📋 自定义文档: {}", custom_variant.title);
    
    // 显示文档详情
    println!("\n   📊 季度报告详情:");
    println!("   作者: {}", quarterly_report.author);
    println!("   字体: {} {}pt", quarterly_report.formatting.font_family, quarterly_report.formatting.font_size);
    println!("   章节: {:?}", quarterly_report.content_sections);
    println!("   元数据: {:?}", quarterly_report.metadata);
    
    println!("\n   📝 商务信件详情:");
    println!("   收件人: {:?}", business_letter.metadata.get("recipient"));
    println!("   行距: {}", business_letter.formatting.line_spacing);
    println!("   章节: {:?}", business_letter.content_sections);
    
    println!("\n   📝 原型模式要点:");
    println!("   • 使用 Clone 创建对象副本");
    println!("   • 避免复杂对象的重复初始化");
    println!("   • 支持运行时动态创建对象变体");
    println!("   • 保持原型对象不变，创建独立副本");
}

/// 演示享元模式
fn demonstrate_flyweight_pattern() {
    use std::collections::HashMap;
    use std::rc::Rc;
    
    // 享元对象使用 Rc（共享不可变数据）
    #[derive(Debug, Clone)]
    struct CharacterStyle {
        font_family: String,
        font_size: u16,
        color: (u8, u8, u8), // RGB
        bold: bool,
        italic: bool,
    }
    
    // 享元工厂
    struct StyleFactory {
        styles: HashMap<String, Rc<CharacterStyle>>,
    }
    
    impl StyleFactory {
        fn new() -> Self {
            StyleFactory {
                styles: HashMap::new(),
            }
        }
        
        fn get_style(
            &mut self,
            font_family: String,
            font_size: u16,
            color: (u8, u8, u8),
            bold: bool,
            italic: bool,
        ) -> Rc<CharacterStyle> {
            let key = format!("{}-{}-{:?}-{}-{}", font_family, font_size, color, bold, italic);
            
            // 如果样式已存在，返回共享引用
            if let Some(style) = self.styles.get(&key) {
                println!("   ♻️ 重用样式: {}", key);
                style.clone() // Clone Rc（只复制引用计数）
            } else {
                // 创建新样式
                let style = Rc::new(CharacterStyle {
                    font_family,
                    font_size,
                    color,
                    bold,
                    italic,
                });
                
                println!("   🆕 创建新样式: {}", key);
                self.styles.insert(key, style.clone());
                style
            }
        }
        
        fn get_style_count(&self) -> usize {
            self.styles.len()
        }
    }
    
    // 上下文对象（外部状态）
    #[derive(Debug)]
    struct Character {
        char: char,
        position: (u32, u32), // x, y 坐标
        style: Rc<CharacterStyle>, // 共享的享元对象
    }
    
    impl Character {
        fn new(char: char, position: (u32, u32), style: Rc<CharacterStyle>) -> Self {
            Character {
                char,
                position,
                style,
            }
        }
        
        fn render(&self) {
            println!(
                "   渲染字符 '{}' 在位置 {:?}，样式: {}pt {} {:?}",
                self.char,
                self.position,
                self.style.font_size,
                self.style.font_family,
                self.style.color
            );
        }
    }
    
    // 文档类
    struct Document {
        characters: Vec<Character>,
        style_factory: StyleFactory,
    }
    
    impl Document {
        fn new() -> Self {
            Document {
                characters: Vec::new(),
                style_factory: StyleFactory::new(),
            }
        }
        
        fn add_text(
            &mut self,
            text: &str,
            start_position: (u32, u32),
            font_family: String,
            font_size: u16,
            color: (u8, u8, u8),
            bold: bool,
            italic: bool,
        ) {
            let style = self.style_factory.get_style(
                font_family,
                font_size,
                color,
                bold,
                italic,
            );
            
            for (i, ch) in text.chars().enumerate() {
                let position = (start_position.0 + i as u32 * 8, start_position.1); // 简单的字符间距
                let character = Character::new(ch, position, style.clone()); // Clone Rc
                self.characters.push(character);
            }
        }
        
        fn render(&self) {
            println!("   📄 渲染文档 ({} 个字符):", self.characters.len());
            for character in &self.characters {
                character.render();
            }
        }
        
        fn get_stats(&self) -> (usize, usize) {
            (self.characters.len(), self.style_factory.get_style_count())
        }
    }
    
    // 创建文档并添加文本
    let mut document = Document::new();
    
    // 添加标题（大字体，粗体）
    document.add_text(
        "Document Title",
        (10, 10),
        "Arial".to_string(),
        18,
        (0, 0, 0),
        true,
        false,
    );
    
    // 添加正文（普通字体）
    document.add_text(
        "This is the main content of the document. ",
        (10, 40),
        "Times New Roman".to_string(),
        12,
        (0, 0, 0),
        false,
        false,
    );
    
    // 添加更多正文（重用样式）
    document.add_text(
        "This text uses the same style as above.",
        (10, 60),
        "Times New Roman".to_string(),
        12,
        (0, 0, 0),
        false,
        false,
    );
    
    // 添加强调文本（斜体）
    document.add_text(
        "Important note",
        (10, 80),
        "Times New Roman".to_string(),
        12,
        (255, 0, 0), // 红色
        false,
        true,
    );
    
    // 添加更多标题（重用标题样式）
    document.add_text(
        "Section 2",
        (10, 110),
        "Arial".to_string(),
        18,
        (0, 0, 0),
        true,
        false,
    );
    
    let (char_count, style_count) = document.get_stats();
    println!("   📊 文档统计:");
    println!("   字符总数: {}", char_count);
    println!("   样式对象数: {}", style_count);
    println!("   内存效率: {:.1}% (如果每个字符都有独立样式)", 
             (style_count as f64 / char_count as f64) * 100.0);
    
    // 渲染部分文档
    println!("\n   🎨 渲染前5个字符:");
    for character in document.characters.iter().take(5) {
        character.render();
    }
    
    println!("\n   📝 享元模式要点:");
    println!("   • 使用 Rc 共享不可变的内部状态");
    println!("   • Clone Rc 只复制引用，不复制数据");
    println!("   • 外部状态（位置）存储在上下文对象中");
    println!("   • 显著减少内存使用，特别是大量相似对象时");
}

/// 最佳实践总结
pub fn best_practices_summary() {
    println!("\n=== 最佳实践总结 ===");
    
    println!("\n🎯 选择指导原则:");
    
    println!("\n1️⃣ Move 优先原则:");
    println!("   • 默认使用 Move，这是 Rust 的核心设计");
    println!("   • 提供最强的内存安全保证");
    println!("   • 零运行时成本");
    println!("   • 适用于资源管理和唯一所有权场景");
    
    println!("\n2️⃣ Copy 适用场景:");
    println!("   • 小型值类型（≤ 64 bytes）");
    println!("   • 频繁传递的数据");
    println!("   • 无需资源管理的类型");
    println!("   • 数值计算和配置参数");
    
    println!("\n3️⃣ Clone 使用时机:");
    println!("   • 需要独立副本时");
    println!("   • 复杂数据结构");
    println!("   • 原型模式和模板");
    println!("   • 并发编程中的数据分发");
    
    println!("\n🏗️ 设计模式应用:");
    
    println!("\n• 建造者模式: Move 支持链式调用");
    println!("• 原型模式: Clone 创建对象副本");
    println!("• 享元模式: Rc + Clone 共享不可变数据");
    println!("• 工厂模式: Move 返回新创建的对象");
    
    println!("\n⚡ 性能优化建议:");
    
    println!("\n• 避免不必要的 Clone");
    println!("• 大型数据优先使用 Move");
    println!("• 考虑使用智能指针（Rc/Arc）共享数据");
    println!("• 小型频繁使用的数据实现 Copy");
    println!("• 在性能关键路径上避免堆分配");
    
    println!("\n🔒 内存安全要点:");
    
    println!("\n• Move 提供编译时所有权检查");
    println!("• Copy 类型不能实现 Drop trait");
    println!("• Clone 需要正确处理资源释放");
    println!("• 使用智能指针管理共享资源");
    
    println!("\n🧵 并发编程建议:");
    
    println!("\n• 使用 Arc 在线程间共享数据");
    println!("• Clone Arc 只复制引用计数");
    println!("• Move 确保线程安全的所有权转移");
    println!("• 避免在多线程环境中使用 Rc");
    
    println!("\n📚 学习建议:");
    
    println!("\n• 理解所有权是 Rust 的核心概念");
    println!("• 从 Move 开始，逐步学习 Copy 和 Clone");
    println!("• 通过实际项目练习不同场景的应用");
    println!("• 关注编译器错误信息，它们是最好的老师");
    
    println!("\n✨ 总结:");
    println!("   Move、Copy 和 Clone 是 Rust 内存管理的三大支柱，");
    println!("   正确理解和使用它们是编写高效、安全 Rust 代码的关键。");
    println!("   在实际开发中，要根据具体场景选择最合适的机制，");
    println!("   平衡性能、安全性和代码可维护性。");
}

/// 运行所有实际应用案例
pub fn run_all_examples() {
    practical_examples();
    best_practices_summary();
}