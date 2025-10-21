# 再也不用怕 Box ！这篇文章让你一看就懂

[mp.weixin.qq.com](https://mp.weixin.qq.com/s/FaU1ZJCQOouJ4-siepRypQ) 七成 大鱼七成饱


## 前言

这是rust九九八十一难第二篇文章，整理下Box是啥，怎么用。原因是写代码时遇到Box，先查了一些资料，依然满脑子问号。最后焚香沐浴，打开官方文档，咬紧牙关读完，通篇每个字都认识，写代码却不知怎么下手。仔细分析下，就是没研究明白，蚊子还是要用大炮打，下面整体梳理下Box指针。
![](https://cubox.pro/c/filters:no_upscale()?imageUrl=https%3A%2F%2Fmmbiz.qpic.cn%2Fsz_mmbiz_jpg%2FnDwjlkXyqXsCjBQFia7oPkBeibqnI7bQHZabY0JH6hBMrdxYgEtULBYbibT8BomlPvHy6VHGvtib5IrNB9bYUSNkag%2F640%3Fwx_fmt%3Djpeg%26from%3Dappmsg%26watermark%3D1%23imgIndex%3D0) who knows

### 一、什么是智能指针

在 C/C++ 里，指针只是单纯的内存地址，开发者需要手动管理内存（malloc/free 或 new/delete），很容易出 bug（悬垂指针、内存泄漏）。说个额外的，之前也做过一段iOS开发，古早版本oc的也是自己管理对象（alloc/release）。坏处是类似代码写的发抖，不然很容易有内存bug，好处是用户体验上一直无比丝滑。

Rust 中的智能指针是一类结构体，行为像指针，但 **内部实现了额外的逻辑** （比如自动释放、引用计数、共享所有权等）。Rust 中常见的智能指针有这些：

*
  Box<T>：堆分配，唯一所有权
*
  Rc<T> / Arc<T>：引用计数，允许多个所有者
*
  RefCell<T> / Mutex<T>：运行时借用检查，内部可变性
*
  String, Vec<T> 也算广义的智能指针（它们内部包含堆分配的数据）

内容有点多，今天只整理Box指针的使用。

### 二、Box智能指针使用举例

为了方便理解，这里举几个例子，演示下Box怎么使用。

#### 1. 大对象传递

有时候类型很大，不想在栈上复制它，而是只操作一个指针。 Box 在栈上只占 1 个指针大小，移动/传递开销很小。

    struct BigData {
        buf: [u8; 1024 * 1024], // 1MB
    }

    fn process(data: Box<BigData>) {
        println!("大小: {}", data.buf.len());
    }

    fn main() {
        let data = Box::new(BigData { buf: [0; 1024 * 1024] });
        process(data); // 传递 Box 指针，而不是整个 1MB 数据
    }

👉 如果直接传 BigData，会发生大对象拷贝。

👉 用 Box<BigData> 只拷贝一个指针（8字节），效率高。

#### 2. **把值放到堆上** （解决大小不确定/递归类型的问题）

Rust 默认所有值都分配在 **栈上** ，但有些情况需要 **堆分配** ：

*
  类型大小在编译期无法确定（比如递归数据结构）。
*
  想要在栈上只放一个指针，而不是整个大对象。

    struct Node {
        val: i32,
        next: Option<Box<Node>>,  // 递归类型
    }

    fn main() {
        let list = Node {
            val: 1,
            next: Some(Box::new(Node {
                val: 2,
                next: Some(Box::new(Node { val: 3, next: None })),
            })),
        };

        // 遍历
        let mut current = &list;
        whilelet Some(ref next) = current.next {
            println!("{}", current.val);
            current = next;
        }
        println!("{}", current.val);
    }

也有用Cons(i32, Box)定义的，不过前期感觉上面的例子更纯粹，容易理解。这里如果不用 Box，next 会无限递归，编译器没法确定大小。用 Box 后，只在栈上存一个指针，堆上放后续内容，大小固定。

为什么用 Option<Box<Node>> ？ 主要问题是最后一个节点没法处理，除非链表必须无限延伸，没有终点。用None可以优雅表示链表结束。

#### 3. **Trait 对象** （实现动态分派，多态）

Rust 的 dyn Trait 不能直接存放在栈上（大小不确定），必须通过指针包装，比如 Box<dyn Trait>。

例子：存放异构对象

    trait Drawable {
        fn draw(&self);
    }

    struct Circle;
    struct Rectangle;

    impl Drawable for Circle { fn draw(&self) { println!("circle"); } }
    impl Drawable for Rectangle { fn draw(&self) { println!("rectangle"); } }

    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle),
        Box::new(Rectangle),
    ];

备注：**异构对象** 指的是 **类型不同，但都实现了同一个 trait 的对象** 。

#### 4. 可变和不可变举例

    fn main() {
        let mut b = Box::new(5);

        let r1: &i32 = &b;      // 不可变引用
        let r2: &mut Box<i32> = &mut b; // 可变引用到 Box 本身

        println!("r1 = {}", r1);
        *r2 = Box::new(100); // 修改 b 指向的对象
    }

解释：

*
  r1 看的是 Box<i32> 的内容（解引用后是 i32）。
*
  r2 是对整个 Box 的可变引用，可以让它重新指向别的堆对象。

    struct Person {
        name: String,
        age: u32,
    }

    fn main() {
        let mut p = Box::new(Person {
            name: String::from("Bob"),
            age: 25,
        });

        // 修改堆中对象的字段
        p.age = 26;
        println!("name = {}, age = {}", p.name, p.age);

        // 或者整体替换
        *p = Person {
            name: String::from("Charlie"),
            age: 40,
        };
        println!("name = {}, age = {}", p.name, p.age);
    }

这里：

*
  let mut p 使得 Box 可变。
*
  p.age = 26; 修改堆里 Person 的字段。
*
  *p = Person {...} 可以替换整个堆对象。

### 三、原理分析

#### 1. 例子1为什么对分配要用智能指针

    let x = Box::new(42);

*
  普通变量分配在栈上，大小必须编译时已知。
*
  Box<T> 把数据放到堆上，返回一个指针（固定大小）。 解决了 **动态大小类型** （DST）或大数据的存储问题。 比如：Box<dyn Trait> 就能把不确定大小的 trait 对象存起来。

#### 2. 堆上的Box内存布局长啥样

##### 1）值的内存举例：

    let x = Box::new(10);

内存大概是这样：

    栈内存:        |   x   | → (0x1000) 指向堆地址
    堆内存(0x1000):|  10   |

##### 2) 上述 dyn Trait 例子里长这样

    Vec
     ├── [ ptr1, ptr2 ]      ← 存放的是 Box 指针（指向堆内存）

    堆内存：
     ptr1 ──> +----------------+   +-------------------+
              | data: Circle   |   | vtable: Drawable  | ← vtable（虚表）
              +----------------+   +-------------------+

     ptr2 ──> +----------------+   +-------------------+
              | data: Rectangle|   | vtable: Drawable  |
              +----------------+   +-------------------+
    Box<dyn Drawable> 存的是 胖指针：

    一个指针指向对象数据（Circle/Rectangle）。

    一个指针指向 vtable（虚函数表）。

    调用 draw() 时：

    先通过胖指针找到 vtable，

    再 动态分派 调用对应的方法。


##### 3）Box与普通指针区别

*
  **裸指针** （C 风格 *mut T / *const T）：只是一个地址，不负责内存安全。

*
  **引用 &T / &mut T** ：指向栈或堆上的数据，但不拥有数据，生命周期受限。

*
  **Box** ：指针 + 所有权，拥有堆上数据，负责释放。

##### 4) Box取数分析

因为box本身是指针，可以通过 * 解引用来访问 Box 里的值：

    let x = Box::new(42);
    println!("{}", *x);   // 解引用，输出 42

也可以像普通变量一样使用，因为 Box<T> 实现了 Deref：

    let s = Box::new(String::from("hello"));
    println!("{}", s.len()); // 自动解引用，不需要写 (*s).len()

    源码长这样：
    impl<T: ?Sized> Deref for Box<T> {
        type Target = T;
        fn deref(&self) -> &T
    }

    impl<T: ?Sized> DerefMut for Box<T> {
        fn deref_mut(&mut self) -> &mut T
    }

也就是说：

*
  Box<ListNode> 在使用 . 运算符时，会 **自动解引用** 成 ListNode
*
  所以假设node是例子里的节点， node.next 相当于 (*node).next

#### 3. 不同类型转成同一个类型，都用 Box 这种方式吗

不一定，下面分两点说

##### 1） 什么时候需要 Box<dyn Trait>

*
  当你有多个**不同的具体类型** ，但它们都实现了同一个 **trait** ，并且你希望在**运行时统一对待** 它们。
*
  这种情况就必须用 **trait 对象** 。而 trait 对象大小不确定，必须通过 **指针（智能指针）** 来管理。

常见选择：

*
  Box<dyn Trait> → 存放在堆上（最常见，灵活）。
*
  &dyn Trait → 只借用，不拥有（轻量级，不负责释放）。
*
  Rc<dyn Trait> / Arc<dyn Trait> → 多所有权或跨线程共享。

👉 上述例子就是这种情况：Rectangle 和 Circle 是不同的类型，但都实现了 Drawable，所以用 Box<dyn Drawable> 来统一。

##### 2） 不一定要用 Box

很多时候可以不用 Box，要看场景：

###### ✅ 借用引用

    let draw = Circle{...};
    let reader: &dyn Drawable = &draw;

*
  如果只是**临时用一下，不需要长期存放** ，可以直接用 &dyn Drawable。
*
  不会分配堆内存，更轻量。

###### ✅ 泛型 + Trait Bound

如果能在编译期就确定类型，可以用泛型而不是 trait 对象：

    fn process<R: Drawable>(mut drawable: R) -> std::io::Result<()> {
        drawable.draw();
        Ok(())
    }

*
  好处：**零开销抽象** （编译器会 monomorphization，直接展开成具体类型）。
*
  坏处：**只能在编译时确定类型** ，参数是"具体类型"，不能在运行时动态选择。

##### 3）为什么不能直接写：let shape: dyn Drawable

dyn Trait 的大小不确定，在 Rust 里，**所有变量的大小必须在编译期确定** 。

*
  对于 Circle，大小已知
*
  对于 Rectangle，大小也已知。
*
  但是 dyn Drawable 本身只是一个"能力描述"，编译器根本不知道要占多少字节。（为什么需要提前知道：因为rust栈分配需要固定空间，内存布局和对齐规则必须明确，函数调用/返回值的 ABI 必须固定，比如返回值大小不确定，编译器就没法生成正确的调用约定）

所以这句代码：

    let shapes: dyn Drawable

编译器会报错：

    the size for values of type `dyn Drawable` cannot be known at compilation time

dyn Trait 在底层是一个 **胖指针 (fat pointer)** ，包含两部分：

1.
   **数据指针** ：指向真正的值（比如 Circle 或 Rectangle）
2.
   **虚表指针 (vtable)** ：指向一张函数表，记录如何调用 read() 等方法

也就是说，dyn Read 本身不是一个值，而是需要用 **指针** 才能存在。

那么trait的解决方法只能是用指针包裹，必须用某种"指针"来存放 dyn Trait，告诉编译器"这里就是一个胖指针"(胖指针 = 数据指针 + vtable 指针)：

*
  **引用** ：

      let reader: &dyn Drawable = &std::io::stdin();


*
  **Box** （最常见）：

      let reader: Box<dyn Drawable> = Box::new(Circle{..});


*
  **Rc / Arc** ：

      let reader: Rc<dyn Read> = Rc::new(std::io::stdin());
      let reader: Arc<dyn Read> = Arc::new(std::io::stdin());


### 四、Box指针在智能指针中是处于什么位置，怎么区分各自场景

前面说了rust智能指针还有很多别的，这里简单整理一张图，从上帝视角看下。其他指针后面有机会再整理。

                             ┌───────────────┐
                             │  智能指针家族  │
                             └───────┬───────┘
                                     │
            ┌────────────────────────┼────────────────────────┐
            │                        │                        │
       堆分配 (Box)             共享所有权 (Rc/Arc)        内部可变性 (RefCell/Mutex)
            │                        │                        │
     ┌──────┴──────┐          ┌──────┴──────┐          ┌──────┴──────┐
     │ Box<T>      │          │ Rc<T>       │          │ RefCell<T>  │
     │ ─────────── │          │ ─────────── │          │ ─────────── │
     │ - 堆分配     │          │ - 单线程共享 │          │ - 单线程修改 │
     │ - 唯一所有权 │          │ - 引用计数   │          │ - 运行时检查 │
     │ - 无共享     │          │ - 不可跨线程 │          │ - 可多 owner │
     └─────────────┘          └─────────────┘          └─────────────┘
                                                              │
                                                              ▼
                                                         ┌─────────┐
                                                         │ Mutex<T>│
                                                         │─────────│
                                                         │ - 跨线程 │
                                                         │ - 加锁   │
                                                         └─────────┘
                                                              │
                                                              ▼
                                                         ┌─────────┐
                                                         │ Arc<T>  │
                                                         │─────────│
                                                         │ - 跨线程 │
                                                         │ - 原子计数│
                                                         └─────────┘

**简单的总结区分如下：**

#### 1. Box<T> → **堆分配 \& 唯一所有权**

*
  作用：把数据放在堆上，常用于存放 **大小未知的类型** （如 dyn Trait）。
*
  特点：只有一个所有者，不支持共享。
*
  用途：树结构、递归类型、trait 对象封装。

#### 2. Rc<T> → **单线程共享所有权**

*
  作用：允许多个变量共享同一份数据（引用计数）。
*
  特点：只适合单线程，**不是线程安全的** 。
*
  用途：图结构、多处共享配置。

#### 3. Arc<T> → **多线程共享所有权**

*
  作用：和 Rc<T> 类似，但支持跨线程（原子引用计数）。
*
  特点：线程安全，但原子操作有性能开销。
*
  用途：多线程程序里共享不可变数据。

#### 4. RefCell<T> → **单线程内部可变性**

*
  作用：即使外层是不可变引用，也能修改内部数据。
*
  特点：借用规则在运行时检查（可能 panic）。
*
  用途：需要在函数之间传递不可变引用，但仍要修改内容。

#### 5. Mutex<T> → **多线程内部可变性**

*
  作用：多个线程共享数据时，保证修改是互斥的。
*
  特点：运行时加锁，防止数据竞争。
*
  用途：多线程环境下需要可变共享。

### 五、总结

涉及到引用场景,优先考虑引用& 或 &mut（\&符号上篇已有介绍），零开销。**大小不确定/递归结构** 考虑Box<T>。Box可以直接按.语法取数，方便快捷。多个所有者或者线程问题在考虑其他智能指针。

[Read in Cubox](https://cubox.pro/my/card?id=7380296222243817798)
