// ========
// match - 解构
// ========
#[allow(dead_code)]

/** 元组 tuples */
fn d_tuples() {
    let triple = (0, -2, 3);
    println!("{:?}", triple); // (0, -2, 3)

    match triple {
        // 第一个元素0的时候匹配
        (0, y, z) => println!("y: {}, z: {}", y, z),
        // 第一个元素1的时候匹配
        (1, ..) => println!("1"),
        // 无匹配结果的情况
        _ => {}
    }
}

/** 数组 array/slice */
fn d_array() {
    let array = [1, -2, 6];
    // let array1 = [0, -2, 6];
    // let array2 = [-1, -2, 6];
    match array {
        [0, second, third] => println!("array[1]={}, array[2]={}", second, third),
        [1, _, third] => println!("array[0]=1, array[1]忽略, array[2]={}", third),
        [-1, second, ..] => println!("array[0]=-1, array[2]={}, 其余忽略", second),
        [3, second, tail @ ..] => println!("array[0]=3, array[1]={}, 其他的: {:?}", second, tail),
        [first, middle @ .., last] => {
            println!("array[0]={}, middle={:?}, array[2]={}", first, middle, last)
        }
    }
}

#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    // CMY(u32, u32, u32),
    // CMYK(u32, u32, u32, u32),
}

/** 枚举enum */
fn d_enum() {
    let color = Color::RGB(122, 17, 40);
    match color {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
        Color::RGB(r, g, b) => println!("RGB; r: {}, g: {}, b: {}", r, g, b),
        Color::HSV(h, s, v) => println!("HSV; h: {}, s: {}, v: {}", h, s, v),
        Color::HSL(h, s, l) => println!("HSL; h: {}, s: {}, l: {}", h, s, l),
    }
}

struct Foo {
    x: (u32, u32),
    y: u32,
}
/** 结构体 struct */
fn d_struct() {
    let foo = Foo { x: (1, 2), y: 3 };
    let (x0, x1) = foo.x;
    println!("x0={}, x1={}", x0, x1);
    assert_eq!(x0, foo.x.0);

    match foo {
        Foo { x: (1, b), y } => println!("x.0=1, x.1={}, y={}", b, y),
        Foo { y: 2, x: i } => println!("y is 2, i={:?}", i),
        Foo { y, .. } => println!("y={}, 其他忽略", y),
    }
}

pub fn handle_test() {
    // d_tuples();
    // d_array();
    // d_enum();
    d_struct();
}
