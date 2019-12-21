// 控制流

fn main() {
    // if 表达式
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if 表达式中的条件必须是 bool 值,吐过条件不是 bool 值, 我们将得到一个错误
    // let num = 3;
    //  if number {
    //  println!("number was three");
    //  }

    // if number != 0 {
    //      println!("number was three");
    // }

    //  else if 多重条件
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // let 语句中使用 if
    let condition = true;
    let number = if condition { 5 } else { 8 };

    //注意!!
    // let 语句中使用 if 条件,每个分支返回的值都必须是相同的类型
    // let num = if condition {
    //     5
    // } else {
    //     "six"
    // };
    println!("The value of number is: {}", number);

    // 使用循环重复执行
    // loop {
    //     println!("again!");
    // }

    // 从循环返回
    // break 退出循环可以附带值
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is {}", result);

    // while 条件循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");

    // 使用 for 遍历集合
    let x = [10, 20, 30, 40, 120];

    for element in x.iter() {
        // iter 是 数组的遍历方法
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        // rev 方法, 用来反转range
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    let x = [1,2,3,4,5];
    println!("最终: {:?}", x.iter());
}
