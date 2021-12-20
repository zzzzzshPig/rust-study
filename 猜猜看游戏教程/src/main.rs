use std::io;
use rand:: Rng;
use std::cmp::Ordering;

fn main() {
    println!("欢迎来到猜猜看小游戏！");
    println!("请输入您认为正确的数字，如果输入错了，系统会提示您是大了还是小了");

    // 生产随机数
    let seed = rand::thread_rng().gen_range(1..101);

    loop {
        // 获取用户输入
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("啊哦，出错啦！");
        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入数字");
                continue
            }
        };

        // 比较数字
        match user_input.cmp(&seed) {
            Ordering::Greater => println!("大了"),
            Ordering::Less => println!("小了"),
            Ordering::Equal => {
                println!("正确");
                break;
            }
        }
    }
}
