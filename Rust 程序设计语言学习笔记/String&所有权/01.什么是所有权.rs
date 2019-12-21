fn main() {
    // 所有权
     
    // 变量作用域

    {                   // s 在这里无效, 它尚未声明
        let s = "hello";// 从此处起, s 是有效的

        // 使用 s
        println!("the s value is: {}", s);
    }                   // 此作用域已结束, s 不再有效

    // String 类型
    let mut s = String::from("hello"); // 声明可修改的字符串
    s.push_str(", world!"); //在字符串后最佳字面值
    println!("{}", s);

    // 内存与分配
    // 变量与数据交互的方式(一): 移动
    // 
    /*
    rust 在字符串方面使用 等号给值赋值与其他语言中有着不同的处理方式
    声明的 String 值, 在将这个存储的 String 值的变量赋值给一个新的变量
    原本声明 String 值的变量将指向无效的引用.
    简单解释就是, rust 不允许对 String 类型进行直接的复制栈上的数据,直接赋值会将栈上的数据转交给需要赋值的变量
    就好像接力棒,接力棒交接之后,之前的人就没有接力棒了
    例如:
    */ 
    let s1 = String::from("hello"); // 在堆中开辟了空间存储了 "hello" 这个值, s1 存储指向这个值的内存地址
    let s2 = s1; // 这里, s1 将指向 "hello" 值的内存地址转交给 s2, 而 s1 将变成无效的引用

    println!("{}, world!", s1); // 此处会报错

    // 变量与数据交互的方式(二): 克隆
    // 如果确实需要深度复制 String 中堆的数据,而不仅仅是在栈上的数据
    // 可以使用通用函数 clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    // 只有栈上的数据: 拷贝
    /*
        如果一个类型拥有 Copy trait，一个旧的变量在将其赋值给其他变量后仍然可用
        。Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait。
        如果我们对其值离开作用域时需要特殊处理的类型使用 Copy 注解，将会出现一个编译时错误。
        那么什么类型是 Copy 的呢？可以查看给定类型的文档来确认，不过作为一个通用的规则，任何简单标量值的组合可以是 Copy 的，不需要分配内存或某种形式资源的类型是 Copy 的。如下是一些 Copy 的类型：

        所有整数类型，比如 u32。
        布尔类型，bool，它的值是 true 和 false。
        所有浮点数类型，比如 f64。
        字符类型，char。
        元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。
    */

    // 所有权与函数
    // 将值传递给函数在语义上与给变量赋值相似.向函数传递值可能会移动火复制就像赋值语句一样
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里
                        // 所以 s 到这里不再有效
    let x = 5;          // x 进入作用域

    makes_copy(x);      // x 应该移动到函数里了
                        // 但 i32 是 Copy 的, 所以再后面可以继续使用 x

    // fn takes_ownership(some_string: String) { // some_string 进入作用域
    //     println!("{}", some_string);
    // } // 这里, some_string 移出了作用域并调用 'drop' 方法.占用的内存被释放

    // fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    //     println!("{}", some_integer)
    // } // 这里, some_integer 移出作用域, 不会有特殊操作

    // 返回值
    // 返回值也可以转移所有权
    
    let s1 = gives_ownership();         // gives_ownership 将返回值


    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到 takes_and_gives_back 中
                                        // 它也将返回值交给 s3
}   // 这里, s3 移出作用域并被丢弃, s2 也移出作用域,但已被移走. 所以什么也不会发生, s1 移出作用域并被丢弃

fn gives_ownership() -> String {            // gives_ownership 将返回值移动给调用它的函数

    let some_string = String::from("hello");// some_string 进入作用域

    some_string                             // 返回 some_string 并移出给调用的函数
}

fn takes_and_gives_back(a_string: String) -> String {   // a_string 进入作用域

    a_string                                            // 返回 a_string 并移出给调用的函数
}


