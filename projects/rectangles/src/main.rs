struct Restangle(u32, u32);

fn main() {
    let rectangle = Restangle(30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rectangle.0, rectangle.1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
