// ========
// if let
// ========

#[derive(PartialEq)]
enum Foo {
    Bar,
    Baz,
    Qux(u32),
}
fn if_let() {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }

    // 需要在enum上使用 #[derive(PartialEq)]
    if Foo::Bar == a {
        println!("a is foobar");
    }
}

pub fn handle_test() {
    if_let();
}
