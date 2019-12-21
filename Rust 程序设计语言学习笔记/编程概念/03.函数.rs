fn main() {
    println!("Hello, world!");

    // 普通函数
    // another_function();
    // 函数传参
    // another_function(3);
    another_function(3, 4);

    // 包含语句和表达式的函数体
    let _x = 5;
    let y = {
        let x = 3;
        x + 1 // 一个代码块内,最后的语句没有分号则表示这是将要返回的值类似于其语言中的 return value
    };
    println!("the value of y is: {}", y);

    // 具有返回值的函数
    let x = five();
    println!("the value of x is: {}", x);

    let x = plus_one(x);
    println!("the value of x is: {}", x);
}

// 函数
// fn another_function() {
//     println!("Another function.");
// }

// 传入参数
// fn another_function(x: i32) {
//     println!("The  value of x is: {}", x);
// }

// 传入多个参数
fn another_function(x: i32, y: i32) {
    println!("The  value of x is: {}", x);
    println!("The  value of x is: {}", y);
}

// 具有返回值的函数
fn five() -> i32 {
    46
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
