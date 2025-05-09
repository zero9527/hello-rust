use std::{
    env::{self, current_dir},
    fs::File,
    io::Read,
};

pub fn handle_test() {
    println!("current_dir: {:?}\n", env::current_dir().unwrap());

    let file_path = current_dir().unwrap().join("hello/study/file.rs");

    let mut s = String::new();
    let mut file = File::open(file_path).unwrap();
    let data = file.read_to_string(&mut s);
    match data {
        Ok(_) => println!("=====Ok:\n{:#?}", s),
        Err(e) => println!("Err: {:?}", e),
    }
}
