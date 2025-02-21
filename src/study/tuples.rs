use std::fmt::Display;

#[derive(Debug)]
struct Matric(f32, f32, f32, f32);

impl Display for Matric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matric: Matric) -> Matric {
    Matric(matric.0, matric.2, matric.1, matric.3)
}

pub fn handle_test() {
    let matric = Matric(1.1, 1.2, 2.1, 2.2);
    println!("{}", matric);
    println!("{}", transpose(matric));
}
