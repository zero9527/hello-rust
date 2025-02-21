#[macro_export]
macro_rules! vector {
    // 空向量
    () => {
        Vec::new()
    };

    // 单个元素
    ($element: expr) => {{
        let mut v = Vec::new();
        v.push($element);
        v
    }};

    // 多个元素
    ($($element: expr),+ $(,)?) => {{
        let mut v = Vec::new();
        $(
            v.push($element)
        )+
        v
    }}
}
