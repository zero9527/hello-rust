use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == self.y
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

pub fn handle_test() {
    let mut map = HashMap::new();
    map.insert(Point { x: 1, y: 2 }, "point1");
    map.insert(Point { x: 3, y: 4 }, "point2");
    map.insert(Point { x: 1, y: 2 }, "point3");

    // map: {Point { x: 1, y: 2 }: "point3", Point { x: 3, y: 4 }: "point2"}
    println!("map: {:?}", map);
}
