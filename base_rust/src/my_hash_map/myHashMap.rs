use std::collections::HashMap;
pub fn run() {
    createHashMap1();
    createHashMapCollect();
}
fn createHashMap1() {
    let mut my_gems = HashMap::new();
    my_gems.insert("红宝石", 1);
    my_gems.insert("绿宝石", 2);

    for (key, val) in &my_gems {
        println!("{},{}", key, val);
    }
}
fn createHashMapCollect() {
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    // 笨办法循环vec
    // let mut teams_map:HashMap<&str,i32> = HashMap::new();
    // for team in &teams_list{
    // teams_map.insert(&team.0, team.1);
    // }

    // 使用迭代器和collect方法创建
    let teams_map: HashMap<_, _> = teams_list.into_iter().collect();
    println!("{:?}", teams_map);
}
