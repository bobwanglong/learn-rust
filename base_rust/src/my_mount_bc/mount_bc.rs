use crate::my_mount::test; // 需要找到在 module tree下该crate的路径

pub fn test2() {
    test();
    println!("这是mount_bc的test2");
}
