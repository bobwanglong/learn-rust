/**
这个例子会打印出 less than five: 4
当 num 与模式中第一个分支匹配时，Some(4) 可以与 Some(x) 匹配，接着匹配守卫检查 x 值是否小于 5，因为 4 小于 5，所以第一个分支被选择。
相反如果 num 为 Some(10)，因为 10 不小于 5 ，所以第一个分支的匹配守卫为假。接着 Rust 会前往第二个分支，因为这里没有匹配守卫所以会匹配任何 Some 成员。
*/
pub fn guard() {
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five:{}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}
/**
 * 使用匹配守卫解决变量覆盖问题
 ```
 let x = Some(5);
 let y = 10;

 match x {
     Some(50) => println!("Got 50"),
     Some(n) if n == y => println!("Matched, n = {}", n),
     _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
```
    现在这会打印出 Default case, x = Some(5)。现在第二个匹配分支中的模式不会引入一个覆盖外部 y 的新变量 y，这意味着可以在匹配守卫中使用外部的 y。相比指定会覆盖外部 y 的模式 Some(y)，这里指定为 Some(n)。此新建的变量 n 并没有覆盖任何值，因为 match 外部没有变量 n。
匹配守卫 if n == y 并不是一个模式所以没有引入新变量。这个 y 正是 外部的 y 而不是新的覆盖变量 y，这样就可以通过比较 n 和 y 来表达寻找一个与外部 y 相同的值的概念了。

 */
pub fn guard1() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) if y == y => println!("Matched, n = {},y={}", y, y), // y 会覆盖外部的y
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}
