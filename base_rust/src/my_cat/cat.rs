use crate::my_str::my_str::new_str; // 导出 new_str方法
pub fn cat(){
    println!("{}","my cat");
    // 使用其my_str包的函数
    let s = new_str();
    println!("{:?}",s)
}