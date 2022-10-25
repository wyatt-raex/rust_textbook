#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/*
All functions defined within an impl block are called associated functions.
This is because they're associaetd with the type named after the impl.

We can define associated functions that don't have self as their first parameter (and thus
are not methods) bcause they don't need an instance of the type to work with.

Associated funtions that aren't functions are often used for constructors that will return a
new instance of the struct. For example in this case the square associated function, which
will allow us to create a Rectangle struct that only has 1 dimension parameter instead of
having to specify both the width and height.

*/
impl Rectangle {
    /*
    The Self keyword used as the return type here will refer to the type that appears after
    the impl keyword. In this case Self refers to Rectangle. So fn square WILL return a
    Rectangle.
    */
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    /*
    Because we specifically want to call the Square associated function within the
    Rectangle struct as our "construtor", we use the :: operator.
    */

    let _square = Rectangle::square(3);
}
