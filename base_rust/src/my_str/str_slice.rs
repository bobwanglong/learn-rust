/// ## 字符串和切片 (这是文档注释)
/// rust中的字符串的概念和其他语言中的略有不同，在学习之前先了解一下切片
/// 对于字符串而言，切片就是对string类型中某一部分的引用，

/** ## 测试文档块注释
```
fn run(){
    let s = String::from("hello,world");
    let hello = &s[..5];
    let world = &s[6..];
}
```

 */
pub fn run(){
    let s = String::from("hello,string");
    // let hello = &s[..5];
    // let world = &s[6..];
    // println!("h:{},w:{}",hello,world);
    // str_push()
    // str_delete()
    // str_concatenate()
    str_utf_8()
}

/// 字符串字面量
/// ``` let s= "hello,world!";```
///  s的类型是 &str，因此 也可以这样声明 
/// ```let s:&str = "hello,world!";```
/// 该切片指向了程序可执行文件中的某个点，这也是为什么说字符串字面量是不可变的，因为&str是一个不可变引用
fn str_literal(){
    let s= "hello,world!";
}

/** ### string和 &str的转换
  #### &str to String
  1. String::from("hello,world")
  2. "hello,world".to_string()
  3. 测试
 * #### String to &str
 答案很简单取引用即可
  ```
  fn main() {
    let s = String::from("hello,world!");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());
}
fn say_hello(s: &str) {
    println!("{}",s);
}
 ```
 */
fn string_conv_str(){}

/**
 ### 字符串索引
 **注意：rust中的字符串类型不能索引**，因为会造成返回值模糊，因为UTF-8编码，导致字符串中的
 不同字符所占的字节是不一定的，由此访问某个索引位置会没有意义。
 */
fn str_index(){}

/**
 ### 追加字符串
 注意点：
 1. 字符串变量必须是mut可变
 2. 在原字符串上追加，并不会返回新的字符串
 3. push()追加字符， push_str()追加字符串
 */
fn str_push(){
    let mut s=String::from("hello ");
    s.push_str("rust");
    println!("追加字符串 push_str() -> {}",s);
    s.push('!'); // 追加字符
    println!("追加字符push() -> {}",s);

}
/**### 插入
 可以使用 insert() 方法插入单个字符 char，也可以使用 insert_str() 方法插入字符串字面量，与 push() 方法不同，
 这俩方法需要传入两个参数，第一个参数是字符（串）插入位置的索引，第二个参数是要插入的字符（串），索引从 0 开始计数，
 如果越界则会发生错误。由于字符串插入操作要修改原来的字符串，则该字符串必须是可变的，即字符串变量必须由 mut 关键字修饰。
 */
fn str_insert(){
    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);
}
/** ### 替换
 * 相关的方法有三个，
 * replace
 该方法可适用于 String 和 &str 类型。
 replace() 方法接收两个参数，第一个参数是要被替换的字符串，第二个参数是新的字符串。
 该方法会替换所有匹配到的字符串。该方法是返回一个新的字符串，而不是操作原来的字符串。
 */
fn str_replace(){
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);
}
/**
 *  replacen
该方法可适用于 String 和 &str 类型。
replacen() 方法接收三个参数，前两个参数与 replace() 方法一样，第三个参数则表示替换的个数。
该方法是返回一个新的字符串，而不是操作原来的字符串
 */
fn str_replacen(){
    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);
}
/**
 * replace_range
 该方法仅适用于 String 类型。replace_range 接收两个参数，第一个参数是要替换字符串的范围（Range），第二个参数是新的字符串。
 该方法是直接操作原来的字符串，不会返回新的字符串。该方法需要使用 mut 关键字修饰。
 */
fn str_repalce_range(){
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);
}
/**### 删除 (Delete)
与字符串删除相关的方法有 4 个，他们分别是 pop()，remove()，truncate()，clear()。
这四个方法仅适用于 String 类型。
* 1.pop —— 删除并返回字符串的最后一个字符
该方法是直接操作原来的字符串。但是存在返回值，其返回值是一个 Option 类型，如果字符串为空，则返回 None。
* 2.remove —— 删除并返回字符串中指定位置的字符
该方法是直接操作原来的字符串。但是存在返回值，其返回值是删除位置的字符串，只接收一个参数，表示该字符起始索引位置。
remove() 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。
* 3.truncate —— 删除字符串中从指定位置开始到结尾的全部字符
该方法是直接操作原来的字符串。无返回值。该方法 truncate() 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误
* 4.clear —— 清空字符串
该方法是直接操作原来的字符串。调用后，删除字符串中的所有字符，相当于 truncate() 方法参数为 0 的时候。
 */
fn str_delete(){
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);

    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    string_remove.remove(0); //删除第一个汉字：测
    // string_remove.remove(1); //  panicked at 'byte index 1 is not a char boundary; it is inside '测' (bytes 0..3) of `测试remove方法`'
    //string_remove.remove(3) // 删掉第二个汉字：试
    // string_remove.remove(6);// 删掉第一个字母 ：r
    dbg!(string_remove);

    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);// 删除测之后的所有字符。测在字符串中的索引是0:3
    dbg!(string_truncate); 

    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);
}

/**### 连接 (Concatenate)
 * 1.使用+或者+= 连接字符
 使用 + 或者 += 连接字符串，要求右边的参数必须为字符串的切片引用（Slice）类型。其实当调用 + 的操作符时，相当于调用了 std::string 标准库中的 add() 方法，这里 add() 方法的第二个参数是一个引用的类型。因此我们在使用 +， 必须传递切片引用类型。不能直接传递 String 类型。+ 是返回一个新的字符串，所以变量声明可以不需要 mut 关键字修饰。
 * 2.使用 format! 连接字符串
 format! 这种方法适用于String和&str，format!的用法与print!的用法类似。
 ···
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);
 ···
 */
fn str_concatenate(){
    let string_append = String::from("hello ");
    let string_rust= String::from("rust");
    //String + &str返回一个 String，然后再继续跟一个 &str 进行 + 操作，返回一个 String 类型，不断循环，最终生成一个 s，也是 String 类型
    let result = string_append+&string_rust; //string_append 所有权被转移 
    let mut result = result+"!";
    result+="!!!";
    println!("连接字符串 + -> {}",result);
    println!("{}",string_rust);// string_append 所有权被转移

    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{},{}!", s1, s2);
    println!("{}", s);
}

/** ### 字符串转义
我们可以通过转义的方式 \ 输出 ASCII 和 Unicode 字符。
```
fn main() {
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
}
```
当然，在某些情况下，可能你会希望保持字符串的原样，不要转义：
```
fn main() {
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}
```
 */
fn str_escape(){}

/// 操作UTF-8字符串
fn str_utf_8(){
    /// 字符
    let s="中国人";
    for c in s.chars(){
        println!("{}",c);
    }
    /// 字节
    for b in "美国人".bytes(){
        println!("{}",b);
    }
}