struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn plus_one_to_width(&mut self) {
        self.width += 1;
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    rect1.plus_one_to_width();

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
