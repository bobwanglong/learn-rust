use std::{
    error::Error,
    fs::File,
    io::{self, ErrorKind, Read},
    num::ParseIntError,
};

pub fn err_run() {
    // let arr = [1,2];
    // let b = get_array_option(&arr);
    // match b {
    //     Some(x) => println!("x:{}",x),
    //     None => println!("没有获取到值"),
    // }
    // let a = get_array_result_demo(&arr);
    // match  a{
    //     Ok(n) => println!("{}",n),
    //     Err(e) => println!("err:{}",e),
    // }

    //    open_file("file_path.txt")
    // let res = spread_err("file_path.txt");
    // match res {
    // Ok(s) => println!("res:{}",s),
    // Err(e) => println!("err:{}",e),
    // }
    let res = multiply("10", "2");
    assert_eq!(res, Ok(20));
    let res = multiply("4", "2");
    assert_eq!(res.unwrap(), 8);
    assert_eq!(multiply1("3", "4").unwrap(), 12);

    // assert_eq!(add_two("4").unwrap(), 6);
    let twenty = multiply3("10", "2");
    print(twenty);
    // 下面的调用会提供更有帮助的错误信息
    let tt = multiply3("t", "2");
    print(tt);
    println!("正常结束");
}

fn get_array_option(arr: &[i32]) -> Option<&i32> {
    arr.get(0)
}
fn get_array_result(arr: &[i32]) -> Result<i32, ParseIntError> {
    let n = arr[2];
    Ok(n)
}

fn get_array_result_demo(arr: &[i32]) -> Result<i32, Box<dyn Error>> {
    let n = arr[2];
    Ok(n)
}

fn open_file(file_path: &str) {
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
fn spread_err(file_path: &str) -> Result<String, io::Error> {
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

pub fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}
pub fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, Box<dyn Error>> {
    let n1 = n1_str.parse::<i32>()?;
    let n2 = n2_str.parse::<i32>()?;
    Ok(n1 * n2)
}

// 使用两种方式填空: map, and then
fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
    n_str.parse::<i32>().map(|num| num + 2)
}

fn multiply2(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    // 实现...
    // let r= n1_str.parse::<i32>()?*n2_str.parse::<i32>()?;
    // Ok(r)
    //   n1_str.parse::<i32>().and_then(|n1| {
    //     n2_str.parse::<i32>().map(|n2| n1 * n2)
    // })

    // 原复杂写法
    let n1 = n1_str.parse::<i32>();
    match n1 {
        Ok(n1) => match n2_str.parse::<i32>() {
            Ok(n2) => Ok(n1 * n2),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
// 类型别名
//如果我们要在代码中到处使用 std::result::Result<T, ParseIntError> ，那毫无疑问，代码将变得特别冗长和啰嗦，对于这种情况，可以使用类型别名来解决。
type Res<T> = Result<T, ParseIntError>;
fn multiply3(first_number_str: &str, second_number_str: &str) -> Res<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        if value >= 10 {
            Ok(PositiveNonzeroInteger(value as u64))
        } else if value == -10 {
            Err(CreationError::Negative)
        } else {
            Err(CreationError::Zero)
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
