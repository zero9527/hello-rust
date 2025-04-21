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
