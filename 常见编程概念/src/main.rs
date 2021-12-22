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

    // 数据类型
    // 整数 i表示有符号 u表示无符号
    let _i8_1: i8 = -111;
    let _u8_1: u8 = 111;
    let _i16_1: i16 = -11111;
    let _u16_1: u16 = 11111;
    let _i32_1: i32 = -1111111;
    let _u32_1: u32 = 1111111;
    let _i64_1: i64 = -111111111;
    let _u64_1: u64 = 111111111;
    let _i128_1: i128 = -11111111111;
    let _u128_1: u128 = 11111111111;
    let _isize_1: isize = -111111111;
    let _usize_1: usize = 111111111;

    // 浮点型
    let _f32_1: f32 = -0.111;
    let _f64_1: f64 = -0.1111111111;

    // 布尔值
    let _bool_1: bool = true;
    let _bool_2: bool = false;

    // char and 字符串
    let _char_1 = 'a';
    let _char_2 = '中';
    let _str_1 = "字符串";

    // 元组
    let tuple_1 = ('1', 1, 1.1, true);
    let (a, b, c, d) = tuple_1;
    println!("{} {} {} {}", tuple_1.0, tuple_1.1, tuple_1.2, tuple_1.3);
    println!("{} {} {} {}", a, b, c, d);

    // 数组
    let array_1 = [3, 5];
    let array_2 = [1, 2, 3];
    let array_3: [i32; 2] = [1, 2];
}
