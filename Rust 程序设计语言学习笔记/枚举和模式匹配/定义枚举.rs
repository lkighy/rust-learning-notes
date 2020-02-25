//  定义枚举
// 关键字 enum
// 例如处理ip, IP
enum IpAddrKind {
    V4,
    v6,
}

// 实例化
let four = IpAddrKind::v4;
let six = IpAddrKind::v6;

// 这样可以定义一个函数来获取任何的 IpAddrKind

fn route(ip_type: IpAddrKind) {}

// 调用函数
route(four);
route(six);

// 存储值: 使用结构体

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::v4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::v6,
    address: String::from("::1")
};

// 存储值

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// let home = IpAddr::V4(String::from("127.0.0.1"));

// let loopback = IpAddr::v6(String::from("::1"));

// 存储值: 与元组使用

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }


// let home = IpAddr::V4(127, 0, 0, 1);

// let loopback = IpAddr::v6(String::from("::1"));

// 存储值: 存储 struct


struct Ipv4Addr {}

struct Ipv6Addr {}

enum IpAddr {
    v4(Ipv4Addr),
    v6(Ipv6Addr),
}

// 内嵌多种类型, 例如下面的例子

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write!(String),
    ChangeColor(i32, i32, i32),
}

// Quit 没有关联任何数据。
// Move 包含一个匿名结构体。
// Write 包含单独一个 String。
// ChangeColor 包含三个 i32。

// 结构体和枚举的另一个相似点: 可以使用 impl 来为结构定义方法那样在枚举上定义方法.

impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}