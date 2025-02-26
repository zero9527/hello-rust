# hello-rust

Rust学习

+ `pub`关键字导出
+ `use`关键字导入
+ `mod`关键字引入模块，有两种引入方式：注意`文件/文件夹`需要以**下划线命名**
    - `文件名.rs`引入
    - `文件夹/mod.rs`引入

> `main.rs`是项目入口，代码是从过这里开始运行的
>
> `mod.rs`一般都有调用对应`mod`模块导出的`handle_test`方法
>

源码[github地址](https://github.com/zero9527/hello-rust)

## src目录结构
```rust
.
├── box1
│   ├── big_struct.rs
│   ├── box_trait.rs
│   └── mod.rs
├── for_iteration
│   └── mod.rs
├── hash_map
│   ├── basic.rs
│   ├── cache_RwLock.rs
│   ├── custom_key.rs
│   ├── lru_cache.rs
│   └── mod.rs
├── main.rs
├── slice
│   ├── array.rs
│   ├── array_mut.rs
│   ├── func.rs
│   ├── mod.rs
│   └── string.rs
├── struct1
│   ├── mod.rs
│   ├── rect_impl.rs
│   └── rect_point_fn.rs
├── study
│   ├── closure_capture.rs
│   ├── command.rs
│   ├── debug.rs
│   ├── display.rs
│   ├── enum1.rs
│   ├── file.rs
│   ├── format.rs
│   ├── function_methods.rs
│   ├── if_let.rs
│   ├── linked_list_enum.rs
│   ├── list.rs
│   ├── match1.rs
│   ├── match2.rs
│   ├── mod.rs
│   ├── tuples.rs
│   └── while_let.rs
└── tools
    ├── calc.rs
    ├── mod.rs
    ├── say_hello.rs
    └── vector.rs
```

## 项目入口 main.rs
```rust
mod for_iteration;
mod hash_map;
mod slice;
mod struct1;
mod study;
mod tools;

fn main() {
    // study::handle_test();
    // tools::handle_test();
    // for_iteration::handle_test();

    // struct1::handle_test();
    // hash_map::handle_test();
    slice::handle_test();
}
```

## struct1目录
结构体

### mod.rs
```rust
mod rect_impl;
mod rect_point_fn;

pub fn handle_test() {
    // rect_point_fn::handle_test();
    rect_impl::handle_test();
}
```

### rect_impl.rs
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 构造方法
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    // 不可变方法
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 可变方法
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
    // 关联函数（静态方法）
    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn handle_test() {
    let mut rectangle = Rectangle::new(10, 8);
    println!("rectangle: {:?}", rectangle);
    println!("area: {}", rectangle.area());

    rectangle.resize(7, 5);
    println!("resize: {:?}", rectangle);

    let sq = Rectangle::square(10);
    println!("sq: {:?}", sq);
}

```

### rect_point_fn.rs
```rust
#![allow(dead_code)]

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

// 求面积
fn rect_area(rect: Rectangle) -> f32 {
    println!("{:?}", rect);
    let Point { x: x1, y: y1 } = rect.top_left;
    let Point { x: x2, y: y2 } = rect.bottom_right;

    println!("{}, {}, {}, {}", x1, x2, y1, y2);
    (x2 - x1) * (y2 - y1)
}

fn square(left_corner: Point, len: f32) -> Rectangle {
    let Point { x, y } = left_corner;
    // 赋值需要显式的写struct
    let bottom_right = Point {
        x: x + len,
        y: y + len,
    };
    Rectangle {
        top_left: left_corner,
        bottom_right,
    }
}

pub fn handle_test() {
    let point = Point { x: 10.3, y: 0.4 };
    // x: 10.3, y: 0.4
    println!("x: {}, y: {}", point.x, point.y);

    let bottom_right = Point { x: 5.2, y: 0.6 };
    // x: 5.2, y: 0.6
    println!("x: {}, y: {}", bottom_right.x, bottom_right.y);

    // 解构并重命名写法，需要带struct
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;
    let _reactangle = Rectangle {
        // 参数赋值带struct
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };
    // Rectangle { top_left: Point { x: 10.3, y: 0.4 }, bottom_right: Point { x: 5.2, y: 0.6 } }
    // 10.3, 5.2, 0.4, 0.6
    // -1.0200002
    println!("{}", rect_area(_reactangle));

    let _square = square(Point { x: 1.1, y: 2.2 }, 6.6);
    // Rectangle { top_left: Point { x: 1.1, y: 2.2 }, bottom_right: Point { x: 7.7, y: 8.8 } }
    println!("{:?}", _square);
}

```

## tools目录
宏`macro_rules!`

+ 声明处使用`#[macro_export]`导出，使用时引入`mod`即可，方法可以直接使用
+ 引入处使用`#[macro_use]`，在当前引入的文件范围使用

### mod.rs
```rust
#[macro_use]
pub mod say_hello;
#[macro_use]
pub mod vector;
#[macro_use]
pub mod calc;

pub fn handle_test() {
    say_hello!("haha");

    let v1 = vector![1];
    let v2 = vector!('a');
    println!("v1: {:?}, v2: {:?}", v1, v2);

    let result = calc!(10, add 5, sub 3, add 2);
    println!("Result: {}", result);
}

```

### say_hello.rs
```rust
#[macro_export]
macro_rules! say_hello {
    () => {
        println!("hello")
    };

    ($name: expr) => {
        println!("hello {}", $name)
    };
}

```

### calc.rs
递归宏

```rust
#[macro_export]
macro_rules! calc {
    // 终止条件
    ($num: expr) => {
        $num
    };
    // 加法
    ($num: expr, add $($rest: tt)*) => {
        calc!($num + $($rest)*)
    };
    // 减法
    ($num: expr, sub $($rest: tt)*) => {
        calc!($num - $($rest)*)
    };
}

```

## for_iteration目录
遍历`Vec`集合的一些方法

### mod.rs
```rust
// 基础for循环遍历
fn basic_for() {
    let numbers = vec![1, 2, 5, 6];

    // for num in numbers {
    //     println!("{}", num);
    // }

    // let numbers = vec![1, 3, 6, 8];
    // for num in &numbers {
    //     println!("{}", num);
    // }

    let mut numbers = vec![1, 3, 5, 6];
    println!("numbers: {:?}", numbers);
    for num in &mut numbers {
        *num *= 2;
    }
    println!("numbers: {:?}", numbers)
}

fn while_let() {
    let mut numbers = vec![1, 2, 6, 8];
    // whie let写法--反序
    // while let Some(num) = numbers.pop() {
    //     println!("{}", num);
    // }

    // loop写法--正序
    loop {
        if numbers.is_empty() {
            break;
        }
        let num = numbers.remove(0);
        println!("{}", num);
    }
}

fn map_filter() {
    let numbers = vec![1, 5, 6, 8];
    let result: Vec<i32> = numbers
        .iter()
        .map(|num| num * 2)
        .filter(|n| n > &5)
        .collect();
    println!("numbers: {:?}", result);
    // numbers: [10, 12, 16]

    for (index, value) in numbers.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }
    // Index: 0, Value: 1
    // Index: 1, Value: 5
    // Index: 2, Value: 6
    // Index: 3, Value: 8
}

fn fold_reduce() {
    let numbers = vec![1, 3, 5, 6];

    // 相当于reduce累加
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("sum: {}", sum);
    // sum: 15

    // reduce
    let product = numbers
        .iter()
        .map(|&x| x)
        .reduce(|acc, x| acc + x)
        .unwrap_or(0);
    println!("pruduct: {}", product);
    // pruduct: 15

    // 自定义累加器
    let stats = numbers
        .iter()
        .fold((0, 0), |(sum, count), &x| (sum + x, count + 1));
    println!("stats: {:?}", stats);
    // stats: (15, 4)
}

struct Counter {
    count: u32,
    max: u32,
}
impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
// 自定义迭代器
fn custom_iterator() {
    let counter = Counter::new(5);
    for num in counter {
        println!("{}", num);
    }
}

pub fn handle_test() {
    // basic_for();
    // while_let();

    // map_filter();
    // fold_reduce();
    custom_iterator();
}

```

### 自定义累加器`fold`
利用元组`()`，可以累加返回多个参数

```rust
// 自定义累加器
let stats = numbers
    .iter()
    .fold((0, 0), |(sum, count), &x| (sum + x, count + 1));
println!("stats: {:?}", stats);
// stats: (15, 4)
```

### 自定义迭代器next
`impl`实现`Iterator`，实现`next`方法

```rust
struct Counter {
    count: u32,
    max: u32,
}
impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
// 自定义迭代器
fn custom_iterator() {
    let counter = Counter::new(5);
    for num in counter {
        println!("{}", num);
    }
}
```



## hash_map目录
`HashMap`一些使用场景

### mod.rs
```rust
mod basic;
mod cache_RwLock;
mod custom_key;
mod lru_cache;

pub fn handle_test() {
    // basic::handle_test();
    // lru_cache::handle_test();
    // custom_key::handle_test();
    cache_RwLock::handle_test();
}
```

### basic.rs
基本使用

```rust
use std::collections::HashMap;

pub fn handle_test() {
    let mut map = HashMap::new();
    // 插入
    map.insert("key1", "value1");
    println!("map: {:?}", map); // map: {"key1": "value1"}

    // 获取
    if let Some(v) = map.get("key1") {
        println!("v: {}", v); // v: value1
    }

    // 更新;不存在则插入
    map.entry("key1")
        .and_modify(|v| *v = "new_value1")
        .or_insert("new_value1");
    // 不存在则插入，否则忽略
    map.entry("key2").or_insert("new_value2");
    println!("map: {:?}", map); // map: {"key1": "new_value1", "key2": "new_value2"}

    map.remove("key1");
    println!("map: {:?}", map); // map: {"key2": "new_value2"}
}

```

### lrc_cache.rs
LRU缓存实现

```rust
use std::collections::HashMap;
use std::collections::VecDeque;

pub struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    keys: VecDeque<K>,
}

impl<K: Clone + Eq + std::hash::Hash, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: HashMap::with_capacity(capacity),
            keys: VecDeque::with_capacity(capacity),
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.map.len() >= self.capacity {
            if let Some(old_key) = self.keys.pop_back() {
                self.map.remove(&old_key);
            }
        }
        self.keys.push_front(key.clone());
        self.map.insert(key, value);
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(index) = self.keys.iter().position(|k| k == key) {
            let k = self.keys.remove(index).unwrap();
            self.keys.push_front(k);
        }
        self.map.get(key)
    }
}

pub fn handle_test() {
    let mut cache = LRUCache::new(10);
    cache.put(String::from("hello"), String::from("world"));

    if let Some(v) = cache.get(&String::from("hello")) {
        println!("hello: {:?}", v); // hello: "world"
    }

    cache.put(String::from("hello"), String::from("rust"));
    if let Some(v) = cache.get(&String::from("hello")) {
        println!("hello: {:?}", v); // hello: "rust"
    }
}
```

### custom_key.rs
`HashMap`的自定义`key`

```rust
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == self.y
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

pub fn handle_test() {
    let mut map = HashMap::new();
    map.insert(Point { x: 1, y: 2 }, "point1");
    map.insert(Point { x: 3, y: 4 }, "point2");
    map.insert(Point { x: 1, y: 2 }, "point3");

    // map: {Point { x: 1, y: 2 }: "point3", Point { x: 3, y: 4 }: "point2"}
    println!("map: {:?}", map);
}
```

### cache_RwLock.rs
基于`RwLock+HashMap`缓存系统实现

```rust
use std::{
    collections::HashMap,
    sync::RwLock,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Cache {
    data: RwLock<HashMap<String, (String, Instant)>>,
    ttl: Duration,
}

impl Cache {
    fn new(ttl_secs: u64) -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
            ttl: Duration::from_secs(ttl_secs),
        }
    }

    fn set(&self, key: String, value: String) {
        let mut data = self.data.write().unwrap();
        data.insert(key, (value, Instant::now()));
    }

    fn get(&self, key: &str) -> Option<String> {
        let data = self.data.read().unwrap();
        if let Some((value, timestamp)) = data.get(key) {
            if timestamp.elapsed() < self.ttl {
                return Some(value.clone());
            }
        }
        None
    }

    fn cleanup(&self) {
        let mut data = self.data.write().unwrap();
        data.retain(|_, (_, timestamp)| timestamp.elapsed() < self.ttl);
    }
}

pub fn handle_test() {
    let data = Cache::new(60);
    data.set("hello".to_string(), "world".to_string());

    if let Some(v) = data.get("hello") {
        println!("hello: {:?}", v); // hello: "world"
    }

    data.cleanup();
    println!("data: {:?}", data);
    // data: Cache { data: RwLock { data: {"hello": ("world", Instant { tv_sec: 283563, tv_nsec: 317218864 })}, poisoned: false, .. }, ttl: 60s }
}

```



## slice目录
切片

+ 切片是引用，没有所有权
+ 切片保证了内存安全
+ 可以引用数组或Vec的一部分
+ 字符串切片是最常见的切片形式
+ 切片存储了指针和长度信息

### mod.rs
```rust
mod array;
mod array_mut;
mod func;
mod string;

pub fn handle_test() {
    // array::handle_test();
    array_mut::handle_test();

    // string::handle_test();

    func::handle_test();
}
```

### array.rs
数组切片普通操作

```rust
use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("{:?}", slice);
    println!("len: {}", slice.len())
}

pub fn handle_test() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // [1, 2, 3, 4, 5], len: 5
    println!("{:?}, len: {}", xs, xs.len());

    // a;b表示b个一样的a
    let ys: [i32; 10] = [0; 10];
    // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], len: 10
    println!("{:?}, len: {}", ys, ys.len());
    // 20字节
    println!("{}", mem::size_of_val(&xs));

    // analyze_slice(&xs);
    // [1, 2]
    // len: 2
    analyze_slice(&xs[0..2]); // 取包含0，不包含2之间的区间

    // println!("{}", xs[5]);

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice1 = &numbers[1..4];
    let slice2 = &numbers[..3];
    let slice3 = &numbers[7..];
    println!("slice1: {:?}", slice1); // slice1: [2, 3, 4]
    println!("slice2: {:?}", slice2); // slice2: [1, 2, 3]
    println!("slice3: {:?}", slice3); // slice3: [8, 9, 10]

    for num in slice1.iter() {
        println!("Number: {num}");
    }
}
```

### array_mut.rs
可变切片

```rust
fn modify_slice(slice: &mut [i32]) {
    for item in slice.iter_mut() {
        *item *= 2;
    }
}

pub fn handle_test() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    // 方法1：处理完切片再打印
    modify_slice(&mut numbers[1..4]);
    // After modification: [1, 4, 6, 8, 5]
    println!("After modification: {:?}", numbers);
}
```

### string.rs
字符串切片

```rust
pub fn handle_test() {
    let message = String::from("Hello Rust");
    let hello = &message[0..5];
    let world = &message[6..10];
    // let world = &message[6..message.len()];

    // first: Hello, end: Rust
    println!("first: {}, end: {}", hello, world);

    let start = &message[..5];
    let end = &message[6..];

    assert_eq!(hello, start);
    assert_eq!("Rust", end);
}
```

### func.rs
在函数参数中使用切片，使函数更加灵活，既可以接受`String`也可以接受`&str`。

```rust
// 打印第一个单词
fn print_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn handle_test() {
    let message = String::from("Hello World");
    let word = print_first_word(&message);
    // First word: Hello
    println!("First word: {}", word);

    // 直接使用字符串字面量
    let word = print_first_word("Rust Programming");
    // First word: Rust
    println!("First word: {}", word);
}
```

## box1目录
参考[链接](https://mp.weixin.qq.com/s/xVMKbzYaLbY3kE41vxuUzw)

### 什么是 Box？
Box 是 Rust 中最简单的智能指针类型。它允许你将值分配到堆上，而不是栈上。简单来说，Box 就像是一个指向堆内存的指针，但它还负责管理这块内存的生命周期。

+ **<font style="color:rgb(255, 104, 39);">Box 的作用和功能</font>**
    1. **堆内存分配**：将数据存储在堆上而不是栈上。
    2. **所有权转移**：可以轻松转移大型数据结构的所有权。
    3. **动态大小类型**：允许使用编译时大小未知的类型。
    4. **特征对象**：用于创建特征对象，实现运行时多态。
    5. **递归类型**：使得定义递归数据结构成为可能。
    6. **内存管理**：自动管理分配的堆内存，无需手动释放。
+ **<font style="color:rgb(255, 104, 39);">Box 解决的问题</font>**
    1. **栈溢出**：通过将大型数据结构放在堆上，避免栈溢出。
    2. **动态大小**：处理编译时大小未知的类型。
    3. **多态**：实现特征对象，支持运行时多态。
    4. **递归定义**：允许创建递归数据结构。
    5. **跨线程共享**：可以安全地在线程间传递堆上的数据。
+ **<font style="color:rgb(255, 104, 39);">Box 在 Rust 中存在的意义</font>**

Box 体现了 Rust 的核心理念：安全、高效、灵活。它提供了一种简单而强大的方式来管理堆内存，同时保持了 Rust 的内存安全保证。Box 是连接 Rust 高级抽象和底层内存管理的桥梁。

### mod.rs
```rust
mod big_struct;
mod box_trait;

pub fn handle_test() {
    big_struct::handle_test();
    box_trait::handle_test();
}
```

### big_struct.rs
```rust
// 大型数据结构
struct BigStruct {
    data: [u8; 1000000],
}

fn process_data(data: Box<BigStruct>) {
    // first byte: 6
    println!("data len: {}", data.data.len());
    println!("first byte: {}", data.data[0]);
}

pub fn handle_test() {
    let big_data = Box::new(BigStruct { data: [6; 1000000] });
    process_data(big_data);
}
```

### box_trait.rs
```rust
trait Animal {
    fn make_sound(&self);
}

// #[derive(Debug)]
struct Dog;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

// #[derive(Debug)]
struct Cat;

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}

fn animal_sound(animals: Vec<Box<dyn Animal>>) {
    for animal in animals {
        animal.make_sound();
    }
}

// 使用 Box 创建特征对象。通过 Box<dyn Animal>，
// 可以在一个集合中存储不同类型的动物，实现运行时多态。

pub fn handle_test() {
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    animal_sound(animals);
}

```

## study目录
### mod.rs
```rust
pub mod array;
pub mod closure_capture;
pub mod command;
pub mod debug;
pub mod display;
pub mod enum1;
pub mod file;
pub mod format;
pub mod function_methods;
pub mod if_let;
// pub mod linked_list_enum;
pub mod list;
pub mod match1;
pub mod match2;
pub mod tuples;
pub mod while_let;

pub fn handle_test() {
    array::handle_test();

    closure_capture::handle_test();
    command::handle_test();
    enum1::handle_test();

    debug::handle_test();
    display::handle_test();
    format::handle_test();

    file::handle_test();

    function_methods::handle_test();
    if_let::handle_test();
    while_let::handle_test();

    // TODO
    // linked_list_enum::handle_test();

    list::handle_test();
    tuples::handle_test();

    match2::handle_test();
}

```

### 枚举 enum1.rs
```rust
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("'{}'.", c),
        WebEvent::Paste(s) => println!("paste \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked as x={}, y={}", x, y);
        }
    }
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

pub fn handle_test() {
    let pressed = WebEvent::KeyPress('x');
    inspect(pressed); // 'x'.

    let pasted = WebEvent::Paste("my text".to_owned());
    inspect(pasted); // paste "my text".

    let click = WebEvent::Click { x: 20, y: 80 };
    inspect(click); // clicked as x=20, y=80

    // 解构enum用法
    let load = WebEvent::PageLoad;
    inspect(load); // page loaded

    let unload = WebEvent::PageUnload;
    inspect(unload); // page unloaded

    // C-like用法，需要使用as，同ts里的enum
    // roses are #ff0000
    let red = format!("{:06x}", Color::Red as i8);
    println!("roses are #{}", red);
    // trees are #000000
    println!("trees are #{:06x}", Color::Green as i8);
    // violets are #0000ff
    println!("violets are #{:06x}", Color::Blue as i8);
}

```

### 匹配模式 match.rs
```rust
#[derive(Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
}

//解构匹配
fn analyze_point(point: Point) {
    match point {
        Point { x: 0, y: 0 } => println!("原点"),
        Point { x: 0, y } => println!("y轴上的点: {}", y),
        Point { x, y: 0 } => println!("x轴上的点: {}", x),
        Point { x, y } => println!("普通点: ({}, {})", x, y),
    }
}

#[derive(Debug)]
struct Color(i32, i32, i32);

// 枚举匹配
fn analyze_color(color: Color) {
    match color {
        Color(r, g, b) if r == g && g == b => println!("灰度颜色"),
        Color(255, 0, 0) => println!("纯红色"),
        Color(r, g, b) => println!("RGB: ({}, {}, {})", r, g, b),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
// 枚举匹配
fn handle_message(msg: Message) {
    match msg {
        Message::Quit => println!("退出Exit"),
        Message::Move { x: 0, y } => println!("垂直移动: {}", y),
        Message::Move { x, y: 0 } => println!("水平移动: {}", x),
        Message::Move { x, y } => println!("移动到: ({}, {})", x, y),
        Message::Write(text) => println!("文本消息: {}", text),
        Message::ChangeColor(Color(r, g, b)) => println!("修改颜色: rgb({}, {}, {})", r, g, b),
    }
}

// 复杂模式组合
fn complex_match(value: Option<Result<Vec<i32>, String>>) {
    match value {
        Some(Ok(vec)) if vec.is_empty() => println!("空向量"),
        Some(Ok(vec)) => println!("向量: {:?}", vec),
        Some(Err(e)) => println!("错误: {}", e),
        None => println!("没有值"),
    }
}

// 范围匹配
fn match_range(value: i32) {
    match value {
        i if i < 0 => println!("负数"),
        0 => println!("0"),
        1..=9 => println!("个位数"),
        10..=99 => println!("两位数"),
        _ => println!("大数"),
    }
}

//引用匹配
fn match_references(value: &Option<String>) {
    match value {
        &Some(ref s) => println!("字符串: {}", s),
        None => println!("空值"),
    }
}

use std::{fs::File, io::Read, rc::Rc, sync::Arc};
// 智能指针匹配
fn match_smart_pointers(value: Rc<Option<Arc<String>>>) {
    match value.as_ref() {
        Some(arc_str) => println!("值: {}", arc_str),
        None => println!("空值"),
    }
}

enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}
// 守卫条件
fn analyze_temperature(temp: Temperature) {
    match temp {
        Temperature::Celsius(c) if c >= 100.0 => println!("水在沸腾^^^"),
        Temperature::Celsius(c) if c < 0.0 => println!("水以结冰__"),
        Temperature::Celsius(c) => println!("水温: {}°", c),
        Temperature::Fahrenheit(f) if f > 212.0 => println!("水在沸腾"),
        Temperature::Fahrenheit(f) if f < 32.0 => println!("水以结冰"),
        Temperature::Fahrenheit(f) => println!("水温: {}°F", f),
    }
}

#[derive(Debug)]
enum Command {
    Start { program: String, args: Vec<String> },
    Stop { pid: i32 },
    Restart { pid: i32, program: String },
}
// 绑定匹配
fn handle_command(cmd: Command) {
    match cmd {
        Command::Start { program, args } => {
            println!("启动程序: {} 参数: {:?}", program, args);
            if program == "hello world" {
                println!("====hello 启动了====")
            }
        }
        cmd @ Command::Stop { .. } => {
            println!("停止命令: {:?}", cmd);
        }
        Command::Restart {
            pid: id @ 1..=999,
            program,
        } => {
            println!("重启PID {} 的程序: {}", id, program);
        }
        Command::Restart { pid, program } => {
            println!("无效的重启命令: PID={}, 程序={}", pid, program);
        }
    }
}

// if let 简化 match
fn quick_check(value: Option<i32>) {
    if let Some(x) = value {
        println!("有值: {}", x)
    }
}

// Option链式处理
fn process_optional_data(data: Option<Vec<i32>>) -> Option<f64> {
    data.filter(|v| !v.is_empty())
        .map(|v| v.iter().sum::<i32>())
        .filter(|&sum| sum > 0)
        .map(|sum| sum as f64 / 2.0)
}

use std::error::Error;
// use std::fs::File;
// use std::io::Read;

// 错误处理模式--match匹配
fn read_file_content(path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(Box::new(e)),
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(Box::new(e)),
    }
}
// 错误处理模式--?捕获
fn read_file_simple(path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// 测试函数
pub fn handle_test() {
    // analyze_point(Point { x: 0, y: 0 });
    // analyze_point(Point { x: 0, y: 10 });
    // analyze_point(Point { x: 100, y: 0 });
    // analyze_point(Point { x: 100, y: 100 });

    // analyze_color(Color(255, 0, 0));
    // analyze_color(Color(100, 100, 100));
    // analyze_color(Color(12, 66, 233));

    // handle_message(Message::Move { x: 0, y: 10 });
    // handle_message(Message::Move { x: 10, y: 0 });
    // handle_message(Message::ChangeColor(Color(66, 88, 233)));

    // complex_match(None);
    // complex_match(Some(Ok(vec![])));
    // complex_match(Some(Ok(vec![1, 2, 3])));
    // complex_match(Some(Err("good bye".to_string())));

    // match_range(-8);
    // match_range(0);
    // match_range(666);
    // match_range(88);

    // match_references(&Some("hello".to_string()));
    // match_smart_pointers(Some(Arc::new("dance".to_string())).into());

    // analyze_temperature(Temperature::Celsius(60.0));
    // analyze_temperature(Temperature::Celsius(100.0));
    // analyze_temperature(Temperature::Celsius(-10.0));
    // analyze_temperature(Temperature::Fahrenheit(160.0));

    // handle_command(Command::Start {
    //     program: "hello world".to_string(),
    //     args: vec!["easy".to_string()],
    // });
    // handle_command(Command::Stop { pid: 66 });
    // handle_command(Command::Restart {
    //     pid: 6666,
    //     program: "hello world".to_string(),
    // });
    // handle_command(Command::Restart {
    //     pid: 66,
    //     program: "hello world".to_string(),
    // });

    // quick_check(Some(66));

    // let process_val = process_optional_data(Some(vec![66, 88]));
    // println!("new value: {:?}", process_val);

    let path = "./src/main.rs";
    if let Ok(content) = read_file_content(path) {
        println!("path: {}, content: {:?}", path, content);
    }
}

```



+ 错误处理模式，读取文件的两种方式
    - `match`匹配写法
    - `?`错误捕获简写

```rust
use std::error::Error;
use std::fs::File;
use std::io::Read;

// 方式一: match
fn read_file_content(path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(Box::new(e)),
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(Box::new(e)),
    }
}

// 方式二: ?简写
fn read_file_simple(path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}


// 使用
pub fn handle_test() {
    let path = "./src/main.rs";
    if let Ok(content) = read_file_content(path) {
        println!("path: {}, content: {:?}", path, content);
    }
}
```
