fn main() {
    let mut count = 0;

    let result = loop {
        if count > 10 {
            break count * 2;
        }
        println!("again!");
        count = count + 1;
    };

    println!("The result is {result}");
}
