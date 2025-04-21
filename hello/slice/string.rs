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
