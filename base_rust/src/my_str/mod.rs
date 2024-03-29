pub mod my_str;
/**
 * # 什么是字符串？
 * rust字符串是 Unicode类型，因为每个字符占据4个字节的内存空间，但是在字符串中不一样，是UTF-8编码，
 * 也就是说，字符串中的每个字符所占的字节数是变化的(1-4)，这样有助于大幅度降低字符串所占的内存空间
 * ## 常用类型：String ||  &str
 * rust在语言级别，只有一种字符串类型： str，通常以 引用的类型出现 &str。但是在标准库中还有多种不同用途的类型
 * 其中String应用最广
 * ## String &str区别
 * String是一个可增长，可改变且具有所有权的UTF-8编码字符串，而str类型是硬编码进可执行文件，无法被修改。
 * 当rust中提到字符串时，往往指的就是 String类型和 &str字符串切片类型，二者都是UTF-8编码
 * ## 字符串深度剖析
 为什么String是可变的，str却不可以？
 就字符串字面值来说，在编译的时候就知道其内容，最终字面值文本被硬编码进可执行文件中，这使得字符串字面值快速且高效
 这主要得益于**字符串字面值的不可变性**，不幸的是，我们不能为了获得这种性能，而把每一个在编译时大小未知的文本都放进内存中（你也做不到！），因为有的字符串是在程序运行得过程中动态生成的。
 **对于String类型**，为了支持一个可变的，可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容，这些都是在运行时完成的
 * 首先向操作系统请求内存来存放String对象
 * 在使用完成后，将内存释放，归还给操作系统
 其中第一部分由 String::from 完成，它创建了一个全新的 String。
 重点来了，到了第二部分，就是百家齐放的环节，在有垃圾回收 GC 的语言中，GC 来负责标记并清除这些不再使用的内存对象，这个过程都是自动完成，无需开发者关心，非常简单好用；但是在无 GC 的语言中，需要开发者手动去释放这些内存对象，就像创建对象需要通过编写代码来完成一样，未能正确释放对象造成的后果简直不可估量。

对于 Rust 而言，安全和性能是写到骨子里的核心特性，如果使用 GC，那么会牺牲性能；如果使用手动管理内存，那么会牺牲安全，这该怎么办？为此，Rust 的开发者想出了一个无比惊艳的办法：
**变量在离开作用域后，就自动释放其占用的内存：**
```
{
    let s = String::from("hello"); // 从此处起，s 是有效的

    // 使用 s
}                                  // 此作用域已结束，
                                   // s 不再有效，内存被释放
```
与其它系统编程语言的 free 函数相同，Rust 也提供了一个释放内存的函数： drop，但是不同的是，其它语言要手动调用 free 来释放每一个变量占用的内存，而 Rust 则在变量离开作用域时，自动调用 drop 函数: 上面代码中，Rust 在结尾的 } 处自动调用 drop。
 */
pub mod str_slice;
