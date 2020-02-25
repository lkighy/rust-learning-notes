
// 定义
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // 结构体实例
    let user = User{
        username: String::from("daming"),
        email: String::from("1000000@123.com"),
        active: true,
        sign_in_count: 1,
    };

    // 定义可变的结构体实例的值
    let mut user = User{
        username: String::from("daming"),
        email: String::from("1000000@123.com"),
        active: true,
        sign_in_count: 1,
    };

    // 改变结构体实例的值
    user.email = String::from("11111@123.com");

    // 变量与字段同名时的字段初始化简写语法
    let email = String::from("10000@123.com");
    let username = String::from("daming");

    let user = User{
        username,
        email,
        active: true,
        sign_in_count: 1,
    };

    // 使用结构体更新语法从其他实例创建实例
    let user2 = User{
        email: String::from("122@123.com"),
        username: String::from("another"),
        active: user.active,
        sign_in_count: user.sign_in_count,
    };

    // 使用 .. 语法指定了剩余未显式设置值的字段
    // 应有与给定实例对应字段相同的值

    let user2 = User {
        email: String::from("1000@123.com"),
        username: String::from("another"),
        ..user
    };

    // 使用没有命名字段的元组结构体来创建不同的类型
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // 没有任何字段的类单元结构体
    // ...

    // 结构体数据的所有权
    // ...
}