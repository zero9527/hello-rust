use std::{env, fs::File, io::Read, path::Path};

pub fn handle_test() {
    println!("current_dir: {:?}\n", env::current_dir().unwrap());

    let file_path = Path::new("./src/study/file.rs");

    let mut s = String::new();
    let mut file = File::open(file_path).unwrap();
    let data = file.read_to_string(&mut s);
    match data {
        Ok(_) => println!("=====Ok:\n{:#?}", s),
        Err(e) => println!("Err: {:?}", e),
    }
}
