#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Defines a function within the context of the Rectangle Struct. Similar to a class in a way.
// impl -> means implementation
// so... implement in the Struct Rectangle, the function area();
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Can also make a struct method that has the same name as one of its parameters.
    fn width(&self) -> bool {
        self.width > 0
    }

    // Function to demonstrate multiple parameters in struct methods
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // Multiple prameter struct method testing
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    /*
    Where's the -> Operator?

    In C/C++ there are 2 different operators for calling methods:
        .    calling a method on an object
        ->   calling a method on a pointer to the object

    Rust doesn't have an equivalent to the -> operator. Rust has a feature for automatic
    referencing and dereferencing. Calling methods is one of the few places Rust utilizes
    this feature.

    How it works:
    When you call a method with `object.something()` Rust auto adds in &, &mut, or * so object
    matches the signature of the method. In otherwords the following are the same:
    `
        p1.distance(&p2);
        (&p1).distance(&p2);
    `
    */
}
