use std::{cmp::Ordering, io};

#[allow(dead_code)]
pub fn run() {
    println!("请输入数字：");

    // 读取标准输入
    let mut input_result = String::new();
    io::stdin()
        .read_line(&mut input_result)
        .expect("\n无法读取输入内容！");

    // 如果解析出错，退出应用程序
    let input_num = input_result.trim_end().parse::<u8>();
    if input_num.is_err() {
        println!("\n无法解析输入的数字：{:?}", input_num.err());
        return;
    }

    // 解析成功后，取出相应的值
    let input_num = input_num.ok().unwrap();

    // 检查条件
    assert!(input_num > 0, "\n输入的数字必须大于0");
    // assert!(i < 255, "输入的数字必须小于255");
    println!("\n获取到输入数字：{}", input_num);
    // 产生随机数
    let rand_num = rand::random::<u8>();
    println!("获取一个随机数：{}\n", rand_num);
    print!("进行比较：");
    let mut opt_num = String::new();
    match input_num.cmp(&rand_num) {
        Ordering::Less => opt_num.push_str("小于"),
        Ordering::Equal => opt_num.push_str("等于"),
        Ordering::Greater => opt_num.push_str("大于"),
    }
    println!("输入数字 {} {} 随机数 {}\n", input_num, opt_num, rand_num);
}
