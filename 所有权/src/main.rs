fn main() {
    let s = String::from("hello world");
    let word = frist_word(&s);
    println!("{}", word)
}

fn frist_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &v) in bytes.iter().enumerate() {
        if v == b' ' {
            return &s[..i];
        }
    }

    &s
}
