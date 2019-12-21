fn main() {
    // 标量类型

    // 整形
    let _x: i8 = 127;
    let _x: u8 = 255;
    let _x: i16 = 1024;
    let _x: u16 = 1024;
    let _x: i32 = 1024;
    let _x: u32 = 1024;
    let _x: i64 = 1024;
    let _x: u64 = 1024;
    let _x: i128 = 1024;
    let _x: u128 = 1024;
    let _x: isize = 1024;
    let _x: usize = 1024;

    // 浮点
    let _x: f32 = 3.0;
    let _y: f64 = 3.0;

    // 布尔
    let _x: bool = false;

    // 字符类型
    let x: char = 'z';

    println!("Hello, world! {}", x);

    // 复合类型

    // 元组类型
    let tup: (i32, f64, char) = (500, 3.2, 'z');
    println!("tup: {:?}", tup);

    // 解构
    let (_x, _y, z) = tup;
    println!("the value of z is: {}", z);

    // 直接访问
    let tup = (2, 3.0, 5);
    let x = tup.0;
    let _y = tup.1;
    let _z = tup.2;
    println!("x: {}",x);

    // 数组类型
    let x: [i32; 5] = [1,3,4,5,6]; // 只允许相同类型的数组,并且数量固定
    // 访问数组
    let _x = x[0];
    let _x = x[1];

    // 快速创建相同值的数组
    let x = [4; 5];
    println!("x: {:?}", x);
}
