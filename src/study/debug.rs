#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

pub fn handle_test() {
    println!("Hello, world!");

    // println!("{:?}", DebugPrintable(666));

    // println!("{:?}", Deep(Structure(2333)));

    let name = "厉飞雨";
    let age = 200;
    let lfy = Person { name, age };
    println!("{:?}", lfy); // Person { name: "厉飞雨", age: 200 }
    println!("{0}今年{1}岁", lfy.name, lfy.age); // 厉飞雨今年200岁
}
