use crate::List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

/**
 * 基于enum的链表实现
 */

pub fn handle_test() {
    let mut list = List::new();

    list = list.prepend(1);
    println!("{}", list.len()); // 1

    list = list.prepend(2);
    println!("{}", list.len()); // 2

    println!("{}", list.stringify()); // 2, 1, Nil
}
