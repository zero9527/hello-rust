trait Animal {
    fn make_sound(&self);
}

// #[derive(Debug)]
struct Dog;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

// #[derive(Debug)]
struct Cat;

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}

fn animal_sound(animals: Vec<Box<dyn Animal>>) {
    for animal in animals {
        animal.make_sound();
    }
}

// 使用 Box 创建特征对象。通过 Box<dyn Animal>，
// 可以在一个集合中存储不同类型的动物，实现运行时多态。

pub fn handle_test() {
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    animal_sound(animals);
}
