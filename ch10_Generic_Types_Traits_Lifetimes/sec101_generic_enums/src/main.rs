/*
Similar to Structs, enums can hold generic data types in their variants as well.
*/
enum Shape<T, U> {
    Square(T),
    Triangle(U),
    Hexagon,
    None,
}

// Here's `Option<T>` enum for example
/*
`Option<T>` enum is generic over type `T` and has two variants: `Some`, which holds one value of type `T`,
and a `None` variant that doesn't hold any value.
*/
enum Option<T> {
    Some(T),
    None,
}

// Or here's `Result<T, E>`
/*
`Result<T, E>` as an enum is generic over type `T` and `E`. It has two variants: `Ok`, which holds one value
of type `T`, and `Err` which holds one value of type `E`.
*/
enum Result<T, E> {
    Ok(T),
    Err(E),
}

/*
When you recognize situations in your code with multiple struct or enum definitions that differ only in the
types of values they hold, you can avoid duplication by using generic types instead.
*/

fn main() {
    println!("Hello, world!");
}
