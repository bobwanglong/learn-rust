mod my_str; 
mod my_cat;// 挂载在根crate
mod my_ownership;
// use my_cat::cat::*; // 使用crate的相对路径
// use crate::my_str::my_str::new_str; // 使用crate的绝对路径
mod my_mount_bc;
mod my_mount;
// use crate::my_mount_bc::mount_bc;

fn main() {
    // let s =  new_str();
    // println!("{}",s);
    // cat();
    // 所有权demo1，可变引用
    // my_ownership::ownership_borrowing::demo1();
    // 测试挂载其他crate
    // mount_bc::test2();
    //  字符串操作
    my_str::str_slice::run()
    //  
    
}