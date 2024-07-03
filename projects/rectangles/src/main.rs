struct Rectangle(u32, u32);

fn main() {
    let rectangle = Rectangle(30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rectangle)
    );
}

fn area(dimensions: Rectangle) -> u32 {
    dimensions.0 * dimensions.1
}
