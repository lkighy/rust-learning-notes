// 使用函数方法
// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//     // 函数方法
//     println!("this area of the recatange is {} square pixels", area(width1, height1));
// }
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// 使用元组重构
// fn main() {
//     let rect1 = (30, 50);
//     println!("The area of rectangle is {} square pixels.", area(rect1));
// }
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// 使用结构体重构
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle { width: 30, height: 50};
    println!("The area of the rectangle is {} square pixels.", area(&rect1));
}

fn area(rectangle: &Rectangle)  -> u32 {
    rectangle.width * rectangle.height
}

// 通过派生 trait 增加实用功能
// #[derive(Debug)]
// struct  Rectangle {
//     width: u32,
//     height: u32,
// }
// fn main() {
//     let rect1 = Rectangle {width: 30, height: 50};
//     println!("rect1 is {}:?", rect1);
// }