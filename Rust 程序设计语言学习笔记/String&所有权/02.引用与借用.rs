fn main() {
    // 引用与借用
    // & 符号就是引用,它们允许你使用值但不获取其所有权
    //     let s1 = String::from("hello");
        
    //     let len = calculate_length(&s1);    // 将引用的值传递到 calculate_length
    //     // &s1 语法让我们创建一个 指向 值 s1 的引用,但是并不拥有它.因为并不拥有这个值
    //     // 当引用离开作用域时其指向的值也不会被丢弃.
    // fn calculate_length(s: &String) -> usize {  // s 是对 String 的引用
    //     s.len()
    // }   // 这里, s 离开了作用域.但因为它并不拥有引用值得所有权, 所以什么也不会发生
    // println!("The length of '{}' is {}.", s1, len);

    // 注意 与使用 & 引用相反的操作是 解引用, 它使用解引用运算符 * .
    // 引用得值不能被修改,相当于在 linux 系统中只有只读的权限,而没有修改的权限

    // 获取应用作为函数参数称为 借用(borrowing). 
    // 正如现实生活中, 如果一个人拥有某样东西,你可以从他那里借来.当你使用完毕必须还回去

    // 可变引用
    let mut s = String::from("hello");

    change(&mut s);     // change 函数修改了 s
    
    println!("{}", s); // 这里输出 "hello, world"

    // 下面会报错
    // let r1 = &mut s;
    // let r2 = &mut s;

    // 在特定作用域只能有一个可变引用
    // 这个限制的好处是 在编译时就避免了数据竞争

    /*
        数据竞争:
            1. 两个火更多指针同时访问同一数据.
            2. 至少有一个指针被用来写入数据.
            3. 没有同步数据访问的机制.
    */

    // 测试 在同一作用域下, 如果上一个可变引用被隐藏了,那么能不能新建一个可变引用
    {
        let mut s = String::from("hello");

        let s1 = &mut s;
        s1.push_str(", world");
        let s1 = String::from("!");
        println!("{}{}", s, s1);
        let s2 = &mut s;
        s2.push_str("!! zero");
        println!("{}", s2);
    }
    
    // 还有一个注意点
    // 可有多个不可变引用或一个可变引用, 但不能同时拥有一个或多个不可变引用与一个可变引用
    // 也就是说 不可变引用与可变引用只能二选一
    // 如下 面代码块
    {
        // let mut s = String::from("hello");

        // let r1 = &s; // 没问题
        // let r2 = &s; // 没问题
        // let r3 = &mut s; // 大问题

        // println!("{}, {}, and {}", r1, r2, r3);
    }

    // 另一种情况, 比如下面的代码就可以编译
    {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        println!("{} and {}", r1, r2);
        // 此位置之后 r1 和 r2 不再使用

        let r3 = &mut s;
        println!("{}", r3);
    }

    // 垂悬引用(Dangling References)
    /*谓悬垂指针是其指向的内存可能已经被分配给其它持有者*/
    // Rust 确保引用永远不会变成垂悬状态

    // 创建一个垂悬引用(误)
    {
        // let references_to_nothing = if true {
        //     let s = String::from("hello"); // 这里创建了一个 String 类型
        //     &s // 将值返回
        // } else {
        //     let s = String::from("world!"); // 创建了一个 String 类型
        //     &s  // 将值返回
        // }; // 这里 将执行 drop 函数 ,将 s 清除, 这意味着这个引用将会指向一个无效的 String ,而 Rust 不会允许这么做
        // println!("{}",references_to_nothing);
    }

    /*
        引用的规则
            1. 在任意给定时间,要么只能有一个可变引用,要么只能有多个不可变引用
            2. 引用必须总是有效的.
    */

}

fn change(some_string: &mut String) {
    some_string.push_str(", world"); // 因为是可变引用,所以这里是可以修改的
}

