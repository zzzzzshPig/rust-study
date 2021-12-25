use std::io;

fn main() {
    // 变量与可变性

    // let let1 = "let不可以修改";
    // print!("{}", let1);
    // let let1 = "但是可以隐藏";
    // println!("{}", let1);
    // let mut let_mut_1 = "let mut 可以修改";
    // print!("{}", let_mut_1);
    // let mut let_mut_1 = "也可以隐藏";
    // println!("{}", let_mut_1);
    // const CONST: &str = "const1 不可以修改，也不可以隐藏";
    // println!("{}", CONST);

    // --- 数据类型 ---
    // 整数 i表示有符号 u表示无符号
    // let _i8_1: i8 = -111;
    // let _u8_1: u8 = 111;
    // let _i16_1: i16 = -11111;
    // let _u16_1: u16 = 11111;
    // let _i32_1: i32 = -1111111;
    // let _u32_1: u32 = 1111111;
    // let _i64_1: i64 = -111111111;
    // let _u64_1: u64 = 111111111;
    // let _i128_1: i128 = -11111111111;
    // let _u128_1: u128 = 11111111111;
    // let _isize_1: isize = -111111111;
    // let _usize_1: usize = 111111111;

    // // 浮点型
    // let _f32_1: f32 = -0.111;
    // let _f64_1: f64 = -0.1111111111;

    // // 布尔值
    // let _bool_1: bool = true;
    // let _bool_2: bool = false;

    // // char and 字符串
    // let _char_1 = 'a';
    // let _char_2 = '中';
    // let _str_1 = "字符串";

    // // 元组
    // let tuple_1 = ('1', 1, 1.1, true);
    // let (a, b, c, d) = tuple_1;
    // println!("{} {} {} {}", tuple_1.0, tuple_1.1, tuple_1.2, tuple_1.3);
    // println!("{} {} {} {}", a, b, c, d);

    // // 数组
    // let array_1 = [3, 5];
    // let array_2 = [1, 2, 3];
    // let array_3: [i32; 2] = [1, 2];

    // --- 函数 ---
    // fn plus_one(x: f32) -> f32 {
    //     x + 1.0
    // }
    // println!("plus_one result is {}", plus_one(1.0))

    // --- if ---
    // println!("if condition result is {}", if true { 5 } else { 6 })

    // --- loop ---
    // println!(
    //     "loop result is {} and {}",
    //     loop {
    //         break 5;
    //     },
    //     'loop2: loop {
    //         loop {
    //             break 'loop2 6;
    //         }
    //     }
    // )

    // --- while ---
    // let mut while_num = 0;
    // while while_num < 5 {
    //     while_num += 1;
    //     println!("while result is {}", while_num)
    // }

    // --- for ---
    // for n in 0..5 {
    //     println!("for result is {}", n)
    // }

    // 相互转换摄氏与华氏温度
    // println!("请输入摄氏度");
    // let mut ipt_c = String::new();
    // io::stdin().read_line(&mut ipt_c).expect("error");
    // let ipt_c = ipt_c.trim();
    // let ipt_c_num: f32 = match ipt_c.parse() {
    //     Ok(n) => n,
    //     Err(_) => {
    //         panic!("输入非数字");
    //     }
    // };
    // println!("{}℃ 转化为华氏度为{} ℉", ipt_c, 32.0 + 1.8 * ipt_c_num);

    // println!("请输入华氏度");
    // let mut ipt_f = String::new();
    // io::stdin().read_line(&mut ipt_f).expect("error");
    // let ipt_f = ipt_f.trim();
    // let ipt_f_num: f32 = match ipt_f.parse() {
    //     Ok(n) => n,
    //     Err(_) => {
    //         panic!("输入非数字");
    //     }
    // };
    // println!("{}℉ 转化为摄氏度为{} ℃", ipt_f, (ipt_f_num - 32.0) / 1.8);

    // --- 斐波那契 ---
    // fn fb(n: u32) -> u32 {
    //     let mut a = 1;
    //     let mut b = 1;
    //     let mut n = n;

    //     while n > 2 {
    //         n -= 1;
    //         b += a;
    //         a = b - a;
    //     }

    //     b
    // }
    // println!("{}", fb(10));
}
