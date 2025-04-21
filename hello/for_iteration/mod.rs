// 基础for循环遍历
fn basic_for() {
    let numbers = vec![1, 2, 5, 6];

    // for num in numbers {
    //     println!("{}", num);
    // }

    // let numbers = vec![1, 3, 6, 8];
    // for num in &numbers {
    //     println!("{}", num);
    // }

    let mut numbers = vec![1, 3, 5, 6];
    println!("numbers: {:?}", numbers);
    for num in &mut numbers {
        *num *= 2;
    }
    println!("numbers: {:?}", numbers)
}

fn while_let() {
    let mut numbers = vec![1, 2, 6, 8];
    // whie let写法--反序
    // while let Some(num) = numbers.pop() {
    //     println!("{}", num);
    // }

    // loop写法--正序
    loop {
        if numbers.is_empty() {
            break;
        }
        let num = numbers.remove(0);
        println!("{}", num);
    }
}

fn map_filter() {
    let numbers = vec![1, 5, 6, 8];
    let result: Vec<i32> = numbers
        .iter()
        .map(|num| num * 2)
        .filter(|n| n > &5)
        .collect();
    println!("numbers: {:?}", result);
    // numbers: [10, 12, 16]

    for (index, value) in numbers.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }
    // Index: 0, Value: 1
    // Index: 1, Value: 5
    // Index: 2, Value: 6
    // Index: 3, Value: 8
}

fn fold_reduce() {
    let numbers = vec![1, 3, 5, 6];

    // 相当于reduce累加
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("sum: {}", sum);
    // sum: 15

    // reduce
    let product = numbers
        .iter()
        .map(|&x| x)
        .reduce(|acc, x| acc + x)
        .unwrap_or(0);
    println!("pruduct: {}", product);
    // pruduct: 15

    // 自定义累加器
    let stats = numbers
        .iter()
        .fold((0, 0), |(sum, count), &x| (sum + x, count + 1));
    println!("stats: {:?}", stats);
    // stats: (15, 4)
}

struct Counter {
    count: u32,
    max: u32,
}
impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
// 自定义迭代器
fn custom_iterator() {
    let counter = Counter::new(5);
    for num in counter {
        println!("{}", num);
    }
}

pub fn handle_test() {
    // basic_for();
    // while_let();

    // map_filter();
    // fold_reduce();
    custom_iterator();
}
