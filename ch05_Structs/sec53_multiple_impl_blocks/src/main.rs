/*
Simple code example showing that it is possible to have multiple impl blocks
when using structs.
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Rectangle Constructors
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Rectangle Methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let square = Rectangle::square(5);
    println!("Square area: {}", square.area());
}
