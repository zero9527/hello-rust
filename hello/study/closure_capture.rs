// ========
// cloure闭包, capture
// ========

use std::mem;

pub fn handle_test() {
    let color = String::from("green");
    // 可以捕获外部环境的参数color
    let print = || println!("`color`: {}", color);
    print(); // `color`: green

    let _reborrow = &color;
    print(); // `color`: green

    let _color_moved = color;
    // 这里调用会报错，因为color已经被move到_color_moved，闭包内引用不到了
    // print();

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc(); // `count`: 1
    inc(); // `count`: 2

    let _count_reborrowed = &mut count;
    println!("`_count_reborrowed`: {}", _count_reborrowed); // `_count_reborrowed`: 2
    println!("`count`: {}", count); // `count`: 2

    // 不可复制的类型
    let movable = Box::new(3);
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
    consume(); // `movable`: 3
               // 以下两个都不能调用了，因为movable已经被mem::drop掉了
               // println!("`movable`: {:?}", movable);
               // consume();

    let haystack = vec![1, 2, 3];
    let contains = move |neddle| haystack.contains(neddle);
    println!("{}", contains(&1)); // true
    println!("{}", contains(&5)); // false
}
