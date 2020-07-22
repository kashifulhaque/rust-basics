#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// We can also separate this code into multiple "impl" blocks
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 90,
        height: 25,
    };
    let rect1 = Rectangle {
        width: 80,
        height: 20,
    };
    let rect2 = Rectangle {
        width: 90,
        height: 30,
    };
    // Usage of associated functions
    let square = Rectangle::square(4);
    // println!("rect is {:#?}", rect);
    println!("Area: {} sq. units", rect.area());
    println!("rect can hold rect1? {}", rect.can_hold(&rect1));
    println!("rect can hold rect2? {}", rect.can_hold(&rect2));
}
