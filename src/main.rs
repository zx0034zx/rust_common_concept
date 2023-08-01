use std::any::type_name;
use std::io;
fn main() {
    println!("Hello, world!");
    // variable();
    // data_type();
    // function();
}

fn variable() {
    // 可变变量和不可变变量
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // 常量
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let y = THREE_HOURS_IN_SECONDS;
    println!("The value of y is: {y}");

    // 隐藏
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    let mut spaces = "   ";
    let mut spaces = spaces.len();
    print!("The value of spaces is: {spaces}");
}

fn data_type() {
    let guess:u32 = "42".parse().expect("Not a number!");
    // 标量 scalar
    // isize usize 依赖运行程序的计算机架构
    // 整形溢出
    let x: u8 = 255;
    println!("The value of x is: {x}");
    // 浮点型
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
    // 数值运算
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;


    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 结果为 -1

    // remainder
    let remainder = 43 % 5;

    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup {} {} {}",tup.0,tup.1,tup.2);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // 数组
    let a = [1, 2, 3, 4, 5];
    println!("a[0] type {:?}",a[0]);


    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn function(){

}