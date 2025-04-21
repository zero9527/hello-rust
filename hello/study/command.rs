use std::process::Command;

pub fn handle_test() {
    let output = Command::new("echo")
        .arg("Hello World")
        .output()
        .expect("Failed to execute command");
    assert_eq!(b"Hello World\n", output.stdout.as_slice());

    // "Hello World\n"
    // 使用std::str::from_utf8将Vec<u8>转成字符串
    println!("{:?}", std::str::from_utf8(&output.stdout).unwrap());
}
