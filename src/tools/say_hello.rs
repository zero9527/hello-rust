#[macro_export]
macro_rules! say_hello {
    () => {
        println!("hello")
    };

    ($name: expr) => {
        println!("hello {}", $name)
    };
}
