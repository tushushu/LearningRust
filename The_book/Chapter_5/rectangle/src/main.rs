fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rec1.area()
    );

    let rec2 = Rectangle::square(20);
    println!(
        "{:?} can hold {:?} is {}!",
        rec1,
        rec2,
        rec1.can_hold(&rec2)
    );

    println!();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
