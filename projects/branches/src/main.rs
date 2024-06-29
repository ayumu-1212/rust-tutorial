fn main() {
    let mut count = 0;
    loop {
        if count > 10 {
            break;
        };
        println!("again!");
        count = count + 1;
    }
}
