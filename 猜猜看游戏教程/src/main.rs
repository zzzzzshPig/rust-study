use std::io;
use rand::Rng;

fn main() {
    println!("欢迎来到石头剪刀布小游戏！");

    loop {
        println!("请输入您的决定");

        // 用户出
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("啊哦，出错啦！");
        user_input = user_input.trim().to_string();
        let input_to_num;

        if user_input == "石头" {
            input_to_num = 0
        } else if user_input == "剪刀" {
            input_to_num = 1
        } else if user_input == "布" {
            input_to_num = 2
        } else {
            println!("请输入有效值");
            continue
        }

        // 电脑出
        let seed = rand::thread_rng().gen_range(0..3);
        let seed_str;

        if seed == 0 {
            seed_str = "石头"
        } else if seed == 1 {
            seed_str = "剪刀"
        } else {
            seed_str = "布"
        }

        println!("电脑出 {}", seed_str);

        let win_or_loss = input_to_num - seed;

        if input_to_num == seed {
            println!("平局")
        } else if win_or_loss == -1 || win_or_loss == 2 {
            println!("您赢了");
            break
        } else {
            println!("您输了")
        }
    }
}
