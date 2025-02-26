// 大型数据结构
struct BigStruct {
    data: [u8; 1000000],
}

fn process_data(data: Box<BigStruct>) {
    // first byte: 6
    println!("data len: {}", data.data.len());
    println!("first byte: {}", data.data[0]);
}

pub fn handle_test() {
    let big_data = Box::new(BigStruct { data: [6; 1000000] });
    process_data(big_data);
}
