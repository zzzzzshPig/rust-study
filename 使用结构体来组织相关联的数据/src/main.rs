fn main() {
    // #[derive(Debug)]
    // struct Student {
    //     name: String,
    //     age: i16,
    //     sex: String,
    //     tel: String,
    // }

    // let mut zhang_san = Student {
    //     name: String::from("张三"),
    //     age: 18,
    //     sex: String::from("男"),
    //     tel: String::from("12345678910"),
    // };

    // zhang_san.name = String::from("123");

    // println!("zhangSan is {:#?}", zhangSan);
    // dbg!(&zhangSan);

    struct Rectangle {
        width: u32,
        height: u32,
    }

    // constructor
    impl Rectangle {
        fn new(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    // fn
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, rect: &Rectangle) -> bool {
            self.width > rect.width && self.height > rect.height
        }
    }

    let rect1 = Rectangle::new(3);
    let rect2 = Rectangle::new(2);
    let rect3 = Rectangle::new(4);

    println!("矩形面积为：{}", rect1.area());
    println!(
        "rect1{}包含rect2",
        if rect1.can_hold(&rect2) { "" } else { "不" }
    );
    println!(
        "rect1{}包含rect3",
        if rect1.can_hold(&rect3) { "" } else { "不" }
    );
}
