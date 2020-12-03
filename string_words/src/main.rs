fn main() {
    let s = String::from("Hello Rust");
    println!("{}", s);
    println!("{}", first_word(&s));
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    // destrukturyzacja dwójki
    //   \/ \/ \/
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}
