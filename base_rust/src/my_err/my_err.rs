use std::{num::ParseIntError, fs::File, io::{ErrorKind, self, Read,}, error::Error};

pub fn err_run(){
    let arr = [1,2];
    // let b = get_array_option(&arr);
    // match b {
    //     Some(x) => println!("x:{}",x),
    //     None => println!("没有获取到值"),
    // }
    let a = get_array_result_demo(&arr);
    match  a{
        Ok(n) => println!("{}",n),
        Err(e) => println!("err:{}",e),
    } 
   
//    open_file("file_path.txt")
let res = spread_err("file_path.txt");
match res {
    Ok(s) => println!("res:{}",s),
    Err(e) => println!("err:{}",e),
}
}

fn get_array_option(arr:&[i32])->Option<&i32> {
    arr.get(0)
}
fn get_array_result(arr:&[i32])->Result<i32,ParseIntError> {
    let n = arr[2];
    Ok(n)
}

fn get_array_result_demo(arr:&[i32])->Result<i32,Box<dyn Error>> {
    let n = arr[2];
    Ok(n)
}

fn open_file(file_path:&str) {
    // let f= File::open(file_path);
    // let f = match f {
    //     Ok(file) => file,
    //     Err(err) => panic!("error open file:{:?}",err),
    // };

    // 等效1
        // let f = File::open(file_path).unwrap_or_else(|error|{
        //     if error.kind()==ErrorKind::NotFound{
        //         File::create(file_path).unwrap_or_else(|error|{
        //             panic!("create file err:{:?}",error)
        //         })
        //     }else {
        //         panic!("error opening file:{:?}",error)
        //     }
        // });
    // 等效match
    let _ = File::open(file_path).expect("无法打开文件"); // expect自定义错误信息信息
    
    // 
}
// 传播错误
fn spread_err(file_path:&str) ->Result<String,io::Error>{
    // let  f = File::open(file_path);
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s  = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
    // 等效替换
    // let mut f= File::open(file_path)?;
    // let mut s =  String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)

    // 最简
    let mut s = String::new();
    File::open(file_path)?.read_to_string(&mut s)?;
    Ok(s)
}