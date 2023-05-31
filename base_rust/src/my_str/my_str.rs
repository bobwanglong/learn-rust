fn my_print() {
    println!("hello string")
}

pub fn new_str() -> String {
    my_print();
    let s = String::from("hello");
    s
}
