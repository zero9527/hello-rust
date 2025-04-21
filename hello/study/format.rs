use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(
            f,
            "{}: {:.3}`{} {:.3}`{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // :0>2 表示不足两位左边补零, ^则中间, <则右边
        write!(
            f,
            "RGB ({red}, {green}, {blue}) 0x{x_red:0>2}{x_green:0>2}{x_blue:0>2}",
            red = self.red,
            green = self.green,
            blue = self.blue,
            x_red = format!("{:X}", self.red),
            x_green = format!("{:X}", self.green),
            x_blue = format!("{:X}", self.blue)
        )
    }
}

pub fn handle_test() {
    for city in [
        City {
            name: "深圳",
            lat: 11.11,
            lon: 22.22,
        },
        City {
            name: "广州",
            lat: 33.33,
            lon: 44.44,
        },
    ] {
        // 深圳: 11.110`N 22.220`E
        // 广州: 33.330`N 44.440`E
        println!("{}", city);
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        // RGB (128, 255, 90) 0x80FF5A
        // RGB (0, 3, 254) 0x0003FE
        // RGB (0, 0, 0) 0x000000
        println!("{}", *color);
    }
}
