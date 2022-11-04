/*
We can also define structs to use a generic type parameter in one or more fields using the `<>` syntax.
The syntax for using generics in struct definitions is similar to that used in function definitions.

Note that because we only declared 1 generic type `T`, both fields `x` and `y` must be the same type of
`T`.
*/
struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    //let wont_work = Point { x: 5, y: 4.0 }; // `x` and `y` must be the same type of `T`. Not i32, f32.

    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
}
