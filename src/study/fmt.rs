#[allow(dead_code)]
pub fn run() {
    // 打印输出;宏指令;
    println!();
    println!(
        " number : {:.2}\n string : {}\n tuple  : {:?}\n trait  : {:?}\n array  : {:#?}\n",
        123.456,
        "Hello",
        (1, 2, 3),
        (),
        ["name", "age", "sex"],
    );

    // 数组
    // https://doc.rust-lang.org/std/primitive.array.html
    let mut a: [u32; 3] = [1, 2, 3]; // 无符号整数的数组;固定长度3
    a[0] = 10;
    // a[1] = 2;
    // a[2] = 3;
    // 断言
    assert_eq!(&a[1..], [2, 3]);
    // 调试
    if cfg!(debug_assertions) {
        print!("debug : array =>");
        a.into_iter().for_each(|i| print!(" {}", i));
    }

    // 

    // 退出
    println!("\n");
}
