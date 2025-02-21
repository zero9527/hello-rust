use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("{:?}", slice);
    println!("len: {}", slice.len())
}

pub fn handle_test() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    // [1, 2, 3, 4, 5], len: 5
    println!("{:?}, len: {}", xs, xs.len());

    // a;b表示b个一样的a
    let ys: [i32; 10] = [0; 10];
    // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], len: 10
    println!("{:?}, len: {}", ys, ys.len());
    // 20字节
    println!("{}", mem::size_of_val(&xs));

    // analyze_slice(&xs);
    // [1, 2]
    // len: 2
    analyze_slice(&xs[0..2]); // 取包含0，不包含2之间的区间

    // println!("{}", xs[5]);
}
