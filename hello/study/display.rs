use std::fmt;

#[derive(Debug)]
struct MinMax(i32, i32);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 以下末尾无分号 ";" 相当于 return write!(f, "{}", self.0);
        write!(f, "{}, {}", self.0, self.1)
    }
}

pub fn handle_test() {
    let minmax = MinMax(0, 14);
    println!("Compare structures: ");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);
}
