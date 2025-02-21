#[derive(Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
}

//解构匹配
fn analyze_point(point: Point) {
    match point {
        Point { x: 0, y: 0 } => println!("原点"),
        Point { x: 0, y } => println!("y轴上的点: {}", y),
        Point { x, y: 0 } => println!("x轴上的点: {}", x),
        Point { x, y } => println!("普通点: ({}, {})", x, y),
    }
}

#[derive(Debug)]
struct Color(i32, i32, i32);

// 枚举匹配
fn analyze_color(color: Color) {
    match color {
        Color(r, g, b) if r == g && g == b => println!("灰度颜色"),
        Color(255, 0, 0) => println!("纯红色"),
        Color(r, g, b) => println!("RGB: ({}, {}, {})", r, g, b),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
// 枚举匹配
fn handle_message(msg: Message) {
    match msg {
        Message::Quit => println!("退出Exit"),
        Message::Move { x: 0, y } => println!("垂直移动: {}", y),
        Message::Move { x, y: 0 } => println!("水平移动: {}", x),
        Message::Move { x, y } => println!("移动到: ({}, {})", x, y),
        Message::Write(text) => println!("文本消息: {}", text),
        Message::ChangeColor(Color(r, g, b)) => println!("修改颜色: rgb({}, {}, {})", r, g, b),
    }
}

// 复杂模式组合
fn complex_match(value: Option<Result<Vec<i32>, String>>) {
    match value {
        Some(Ok(vec)) if vec.is_empty() => println!("空向量"),
        Some(Ok(vec)) => println!("向量: {:?}", vec),
        Some(Err(e)) => println!("错误: {}", e),
        None => println!("没有值"),
    }
}

// 范围匹配
fn match_range(value: i32) {
    match value {
        i if i < 0 => println!("负数"),
        0 => println!("0"),
        1..=9 => println!("个位数"),
        10..=99 => println!("两位数"),
        _ => println!("大数"),
    }
}

//引用匹配
fn match_references(value: &Option<String>) {
    match value {
        &Some(ref s) => println!("字符串: {}", s),
        None => println!("空值"),
    }
}

use std::{fs::File, io::Read, rc::Rc, sync::Arc};
// 智能指针匹配
fn match_smart_pointers(value: Rc<Option<Arc<String>>>) {
    match value.as_ref() {
        Some(arc_str) => println!("值: {}", arc_str),
        None => println!("空值"),
    }
}

enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}
// 守卫条件
fn analyze_temperature(temp: Temperature) {
    match temp {
        Temperature::Celsius(c) if c >= 100.0 => println!("水在沸腾^^^"),
        Temperature::Celsius(c) if c < 0.0 => println!("水以结冰__"),
        Temperature::Celsius(c) => println!("水温: {}°", c),
        Temperature::Fahrenheit(f) if f > 212.0 => println!("水在沸腾"),
        Temperature::Fahrenheit(f) if f < 32.0 => println!("水以结冰"),
        Temperature::Fahrenheit(f) => println!("水温: {}°F", f),
    }
}

#[derive(Debug)]
enum Command {
    Start { program: String, args: Vec<String> },
    Stop { pid: i32 },
    Restart { pid: i32, program: String },
}
// 绑定匹配
fn handle_command(cmd: Command) {
    match cmd {
        Command::Start { program, args } => {
            println!("启动程序: {} 参数: {:?}", program, args);
            if program == "hello world" {
                println!("====hello 启动了====")
            }
        }
        cmd @ Command::Stop { .. } => {
            println!("停止命令: {:?}", cmd);
        }
        Command::Restart {
            pid: id @ 1..=999,
            program,
        } => {
            println!("重启PID {} 的程序: {}", id, program);
        }
        Command::Restart { pid, program } => {
            println!("无效的重启命令: PID={}, 程序={}", pid, program);
        }
    }
}

// if let 简化 match
fn quick_check(value: Option<i32>) {
    if let Some(x) = value {
        println!("有值: {}", x)
    }
}

// Option链式处理
fn process_optional_data(data: Option<Vec<i32>>) -> Option<f64> {
    data.filter(|v| !v.is_empty())
        .map(|v| v.iter().sum::<i32>())
        .filter(|&sum| sum > 0)
        .map(|sum| sum as f64 / 2.0)
}

use std::error::Error;
// use std::fs::File;
// use std::io::Read;

// 错误处理模式--match匹配
fn read_file_content(path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(Box::new(e)),
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(Box::new(e)),
    }
}
// 错误处理模式--?捕获
fn read_file_simple(path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// 测试函数
pub fn handle_test() {
    // analyze_point(Point { x: 0, y: 0 });
    // analyze_point(Point { x: 0, y: 10 });
    // analyze_point(Point { x: 100, y: 0 });
    // analyze_point(Point { x: 100, y: 100 });

    // analyze_color(Color(255, 0, 0));
    // analyze_color(Color(100, 100, 100));
    // analyze_color(Color(12, 66, 233));

    // handle_message(Message::Move { x: 0, y: 10 });
    // handle_message(Message::Move { x: 10, y: 0 });
    // handle_message(Message::ChangeColor(Color(66, 88, 233)));

    // complex_match(None);
    // complex_match(Some(Ok(vec![])));
    // complex_match(Some(Ok(vec![1, 2, 3])));
    // complex_match(Some(Err("good bye".to_string())));

    // match_range(-8);
    // match_range(0);
    // match_range(666);
    // match_range(88);

    // match_references(&Some("hello".to_string()));
    // match_smart_pointers(Some(Arc::new("dance".to_string())).into());

    // analyze_temperature(Temperature::Celsius(60.0));
    // analyze_temperature(Temperature::Celsius(100.0));
    // analyze_temperature(Temperature::Celsius(-10.0));
    // analyze_temperature(Temperature::Fahrenheit(160.0));

    // handle_command(Command::Start {
    //     program: "hello world".to_string(),
    //     args: vec!["easy".to_string()],
    // });
    // handle_command(Command::Stop { pid: 66 });
    // handle_command(Command::Restart {
    //     pid: 6666,
    //     program: "hello world".to_string(),
    // });
    // handle_command(Command::Restart {
    //     pid: 66,
    //     program: "hello world".to_string(),
    // });

    // quick_check(Some(66));

    // let process_val = process_optional_data(Some(vec![66, 88]));
    // println!("new value: {:?}", process_val);

    let path = "./src/main.rs";
    if let Ok(content) = read_file_content(path) {
        println!("path: {}, content: {:?}", path, content);
    }
}
