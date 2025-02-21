// ========
// while let替代match的场景
// ========

fn match_loop() {
    let mut optional = Some(0);
    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("i > 9");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`", i);
                    optional = Some(i + 1);
                }
            }
            _ => {
                break;
            }
        }
    }
}

fn while_let() {
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("i > 9");
            optional = None;
        } else {
            println!("`i` is `{}`", i);
            optional = Some(i + 1);
        }
    }
}

pub fn handle_test() {
    match_loop();
    while_let();
}
