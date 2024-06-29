use std::io;

fn main() {
    println! {"Please input number"};
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let number: i32 = input.trim().parse().expect("Failed to parse.");

    if number < 5 {
        println!("low number: {number}");
    } else if number < 15 {
        println!("mideum number: {number}");
    } else {
        println!("condition was false");
    }
}
