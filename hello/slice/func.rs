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
