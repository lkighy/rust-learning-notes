// 定义方法
struct Rectangle {
    width: u32,
    height: u32,
}
// impl 是 implementation 的缩写
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50};
//     println!("The area of the rectangle is {} square pixels.", rect1.area());
// }

// 带更多参数的方法
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
// fn main() {
//     let rect1 = Rectangle {width: 30, height: 50};
//     let rect2 = Rectangle {width: 10, height: 59};
//     let rect3 = Rectangle {width: 40, height: 57};

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
// }

// 关联函数
// impl 块的另一个有用的功能是: 允许在 impl 块中定义 不 以 self 作为参数的函数
// 这被称为 关联函数, 因为它们与结构体相关.它们仍是函数而不是方法,
// 因为它们并不作用于一个结构体的实例.例如 已经使用过的 String::from 关联函数

// 关联函数经常被用作返回一个结构体新实例的构造函数.
// impl Rectangle {
//     fn square(size: u32) -> Rectangle {
//         Rectangle { width: size, height: size}
//     }
// }

// 多个 impl 块
// 每个结构体都允许拥有多个 impl 块。
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
