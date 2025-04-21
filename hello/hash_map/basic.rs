use std::collections::HashMap;

pub fn handle_test() {
    let mut map = HashMap::new();
    // 插入
    map.insert("key1", "value1");
    println!("map: {:?}", map); // map: {"key1": "value1"}

    // 获取
    if let Some(v) = map.get("key1") {
        println!("v: {}", v); // v: value1
    }

    // 更新;不存在则插入
    map.entry("key1")
        .and_modify(|v| *v = "new_value1")
        .or_insert("new_value1");
    // 不存在则插入，否则忽略
    map.entry("key2").or_insert("new_value2");
    println!("map: {:?}", map); // map: {"key1": "new_value1", "key2": "new_value2"}

    map.remove("key1");
    println!("map: {:?}", map); // map: {"key2": "new_value2"}
}
