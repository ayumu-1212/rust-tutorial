fn main() {
    let mut s = String::from("hello world");

    let word = first_word_len(&s);

    s.clear();

    println!("s: {s}");
}

fn first_word_len(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
