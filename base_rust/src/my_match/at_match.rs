pub fn run() {
    at()
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
/**
* @前绑定后解构(Rust 1.56 新增)
 使用 @ 还可以在绑定新变量的同时，对目标进行解构：
*/
fn at() {
    let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
    println!("x:{},y:{}", px, py);
    println!("{:?}", p);
    match 1 {
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => {}
    }
}
