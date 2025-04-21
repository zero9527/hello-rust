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
