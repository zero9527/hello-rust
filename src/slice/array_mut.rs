fn modify_slice(slice: &mut [i32]) {
    for item in slice.iter_mut() {
        *item *= 2;
    }
}

pub fn handle_test() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    // 方法1：处理完切片再打印
    modify_slice(&mut numbers[1..4]);
    // After modification: [1, 4, 6, 8, 5]
    println!("After modification: {:?}", numbers);
}
