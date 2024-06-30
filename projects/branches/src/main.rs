fn main() {
    let s = String::from("hello world");

    let size = first_word_len(&s);

    println!("size: {size}");
}

fn first_word_len(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
