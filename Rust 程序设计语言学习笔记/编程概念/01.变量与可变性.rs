fn main() {
    // 1. 变量和可变性
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6; // 此处会报错, rust
    // println!("The value of x is: {}", x);

    // 如果需要改变变量, 则需要显示的声明变量的可变性
    // let mut x = 5; // 需要改变值时, 需要使用 mut 关键字来表示
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // 2. 常量
    // const MAX_POINTS: u32 = 100_000;
    // println!("The value of x is: {}", MAX_POINTS);

    // 3. 隐藏(Shadowing)
    // let x = 5;
    // let x = x + 1; // 通过 let x = 隐藏 x, 获取初始值并加 1;
    // let x = x ^ 2;
    // println!("The value of x is: {}", x);

    // 隐藏与 mut 的另一个区别时, 再次使用 let 时,实际上创建了一个新的变量
    // 例如
    // let spaces = "     "; // 这里 ,spaces 是 字符串类型
    // let spaces = spaces.len(); // 在这里变成了数字类型

    // 下面就是会报错
    // let mut spaces = "      ";
    // spaces = spaces.len();
}
