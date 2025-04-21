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
