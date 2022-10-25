// Explicitly opt-in to Debug attribute for Rectangle struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // {:?} says we want to print rect1 using the Debug trait format. To use this we need the
    // #[derive(Debug)] attribute opted in on line 2 for the Rectangle struct.
    println!("rect1 is {:?}", rect1);

    // Another option is {:#?} to style the output
    println!("Styled debug output for rect 1: {:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
