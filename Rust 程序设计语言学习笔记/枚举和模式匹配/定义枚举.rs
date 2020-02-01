// enum IpAddKind {
//     v4,
//     v6,
// }

// struct IpAddr {
//     Kind: IpAddKind,
//     address: String,
// }
// fn main() {
//     let four = IpAddKind::v4;
//     let six = IpAddKind::v6;

//     route(IpAddKind::v4);
//     route(IpAddKind::v6);

//     let home = IpAddr {
//         Kind: IpAddKind::v4,
//         address: String::from("127.0.0.1"),
//     }
//     let loopback = IpAddr = {
//         Kind: IpAddKind::v6,
//         address: String::from("::1"),
//     }
// }

// fn route(ip_type: IpAddKind) {}

enum IpAddr {
    V4(String),
    V6(String),
}