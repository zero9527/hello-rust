#[macro_export]
macro_rules! calc {
    // 终止条件
    ($num: expr) => {
        $num
    };
    // 加法
    ($num: expr, add $($rest: tt)*) => {
        calc!($num + $($rest)*)
    };
    // 减法
    ($num: expr, sub $($rest: tt)*) => {
        calc!($num - $($rest)*)
    };
}
