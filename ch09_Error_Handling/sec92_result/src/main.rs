/*
Recall that Result is an enum defined having two variants:
`
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
`

The `T` and `E` are generic type parameters. `T` represents the type of value that will be returned in a
success case with the `Ok` variant. `E` represents the type of error that will be returned in a failure
case within the `Err` varaint.
*/

use std::fs::File;

fn main() {
    /*
    Call a function that returns a `Result` value because the funciton could fail.
    */

    let greeting_file_result = File::open("hello.txt");

    /*
    The return type of `File::open()` is a `Result<T, E>`. `T`
    */

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file {:?}", error),
    };
}
