enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("'{}'.", c),
        WebEvent::Paste(s) => println!("paste \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked as x={}, y={}", x, y);
        }
    }
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

pub fn handle_test() {
    let pressed = WebEvent::KeyPress('x');
    inspect(pressed); // 'x'.

    let pasted = WebEvent::Paste("my text".to_owned());
    inspect(pasted); // paste "my text".

    let click = WebEvent::Click { x: 20, y: 80 };
    inspect(click); // clicked as x=20, y=80

    // 解构enum用法
    let load = WebEvent::PageLoad;
    inspect(load); // page loaded

    let unload = WebEvent::PageUnload;
    inspect(unload); // page unloaded

    // C-like用法，需要使用as，同ts里的enum
    // roses are #ff0000
    let red = format!("{:06x}", Color::Red as i8);
    println!("roses are #{}", red);
    // trees are #000000
    println!("trees are #{:06x}", Color::Green as i8);
    // violets are #0000ff
    println!("violets are #{:06x}", Color::Blue as i8);
}
