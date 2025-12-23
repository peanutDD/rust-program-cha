//! # 实战复杂案例
//!
//! 展示闭包在实际项目中的复杂应用

use std::collections::HashMap;

/// 演示实战案例
pub fn demo_real_world_cases() {
    println!("\n=== 实战复杂案例 ===");

    case_1_event_system();
    case_2_middleware_pattern();
    case_3_lazy_evaluation();
    case_4_builder_with_closures();
}

/// 案例1：事件系统
fn case_1_event_system() {
    println!("\n--- 案例1：事件系统 ---");

    type EventHandler = Box<dyn Fn(&str) + Send + Sync>;

    struct EventBus {
        handlers: HashMap<String, Vec<EventHandler>>,
    }

    impl EventBus {
        fn new() -> Self {
            EventBus {
                handlers: HashMap::new(),
            }
        }

        fn on(&mut self, event: &str, handler: EventHandler) {
            self.handlers
                .entry(event.to_string())
                .or_insert_with(Vec::new)
                .push(handler);
        }

        fn emit(&self, event: &str, data: &str) {
            if let Some(handlers) = self.handlers.get(event) {
                for handler in handlers {
                    handler(data);
                }
            }
        }
    }

    let mut bus = EventBus::new();
    
    bus.on("login", Box::new(|user| {
        println!("用户登录: {}", user);
    }));

    bus.on("login", Box::new(|user| {
        println!("记录日志: {} 登录", user);
    }));

    bus.emit("login", "Alice");
}

/// 案例2：中间件模式
fn case_2_middleware_pattern() {
    println!("\n--- 案例2：中间件模式 ---");

    type Request = String;
    type Response = String;
    type Middleware = Box<dyn Fn(Request, &dyn Fn(Request) -> Response) -> Response>;

    struct Server {
        middlewares: Vec<Middleware>,
    }

    impl Server {
        fn new() -> Self {
            Server {
                middlewares: Vec::new(),
            }
        }

        fn use_middleware(&mut self, middleware: Middleware) {
            self.middlewares.push(middleware);
        }

        fn handle(&self, request: Request) -> Response {
            let handler = |req: Request| -> Response {
                format!("处理: {}", req)
            };

            self.execute_middlewares(request, &handler, 0)
        }

        fn execute_middlewares(
            &self,
            request: Request,
            final_handler: &dyn Fn(Request) -> Response,
            index: usize,
        ) -> Response {
            if index >= self.middlewares.len() {
                return final_handler(request);
            }

            let middleware = &self.middlewares[index];
            let next = |req: Request| self.execute_middlewares(req, final_handler, index + 1);
            
            middleware(request, &next)
        }
    }

    let mut server = Server::new();

    server.use_middleware(Box::new(|req, next| {
        println!("中间件1: 请求前");
        let res = next(format!("[Auth]{}", req));
        println!("中间件1: 请求后");
        res
    }));

    server.use_middleware(Box::new(|req, next| {
        println!("中间件2: 日志");
        next(req)
    }));

    let response = server.handle("GET /api/users".to_string());
    println!("响应: {}", response);
}

/// 案例3：惰性求值
fn case_3_lazy_evaluation() {
    println!("\n--- 案例3：惰性求值 ---");

    struct Lazy<T, F>
    where
        F: FnOnce() -> T,
    {
        init: Option<F>,
        value: Option<T>,
    }

    impl<T, F> Lazy<T, F>
    where
        F: FnOnce() -> T,
    {
        fn new(init: F) -> Self {
            Lazy {
                init: Some(init),
                value: None,
            }
        }

        fn force(&mut self) -> &T {
            if self.value.is_none() {
                let init = self.init.take().unwrap();
                self.value = Some(init());
            }
            self.value.as_ref().unwrap()
        }
    }

    let mut expensive = Lazy::new(|| {
        println!("执行昂贵计算...");
        42
    });

    println!("值还未计算");
    println!("第一次获取: {}", expensive.force());
    println!("第二次获取(无需重新计算): {}", expensive.force());
}

/// 案例4：构建器模式with闭包
fn case_4_builder_with_closures() {
    println!("\n--- 案例4：构建器模式 ---");

    struct Query {
        sql: String,
    }

    impl Query {
        fn new() -> Self {
            Query {
                sql: String::new(),
            }
        }

        fn select<F>(mut self, f: F) -> Self
        where
            F: Fn() -> String,
        {
            self.sql = format!("SELECT {}", f());
            self
        }

        fn from<F>(mut self, f: F) -> Self
        where
            F: Fn() -> String,
        {
            self.sql = format!("{} FROM {}", self.sql, f());
            self
        }

        fn where_clause<F>(mut self, f: F) -> Self
        where
            F: Fn() -> String,
        {
            self.sql = format!("{} WHERE {}", self.sql, f());
            self
        }

        fn build(self) -> String {
            self.sql
        }
    }

    let query = Query::new()
        .select(|| "id, name, email".to_string())
        .from(|| "users".to_string())
        .where_clause(|| "age > 18".to_string())
        .build();

    println!("生成的SQL: {}", query);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy_evaluation() {
        struct Lazy<T, F: FnOnce() -> T> {
            init: Option<F>,
            value: Option<T>,
        }

        impl<T, F: FnOnce() -> T> Lazy<T, F> {
            fn new(init: F) -> Self {
                Lazy {
                    init: Some(init),
                    value: None,
                }
            }

            fn force(&mut self) -> &T {
                if self.value.is_none() {
                    let init = self.init.take().unwrap();
                    self.value = Some(init());
                }
                self.value.as_ref().unwrap()
            }
        }

        let mut lazy = Lazy::new(|| 42);
        assert_eq!(*lazy.force(), 42);
    }
}

