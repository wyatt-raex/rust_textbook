use std::fs;
use std::fs::File;
use std::io::{self, Read};

/*
When a function's implementation calls something that might fail, instead of handling the error within
the function itself, you can return the error to the calling code so that it can decide what to do.
This is known as `propagating` the error and gives more control to the calling code, where there might
be more information or logic that dictates how the error should be handled than what you have available
in the context of the funtion's code.
*/

// Both `File::open` and `read_to_string()` return an `io::Error` as their `Err` variant
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // If the `Err` gets matched, return the `Err` variant to calling code
    };

    let mut username = String::new();

    // Lack of semi-colon at the end of this match statement indicates this is what we're returning
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

/*
This function has the same functionality as `read_username_from_file()` above but instead uses the `?`
propagation operator to be more concise.

If the `?` operator returns an `Err` variant in a propagation function, it converts the `Err` variant
type to that of what the function would return, and then early returns the propagation function.
*/
fn read_username_propagation_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

/*
`?` operator eliminates a lot of boilerplate and makes this function's implementation simpler. This
function's code can be even shorter by CHAINING methods calls immediately after the `?`, as shown
below:
*/
fn shorter_read_username() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// We can again make this code even shorter utilizing `fs::read_to_string()`
fn shortest_read_username() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

/*
The `?` operator can only be used in functions whose return type is compatible with the value the `?` is
used on. This is because the `?` operator is defined to perform an early return of a value out of the
function.

`?` operator can also be used on `Option<T>`. If a function returns an `Option<T>` and gets `None` value
early, `?` operator will return `None` similarly how it will return an `Err` variant if returning a
`Result<T, E>`
*/
fn main() {
    let _error_propagation = read_username_from_file();
    let _propagation_shortcut = read_username_propagation_shortcut();
    let _shorter_propagation = shorter_read_username();
    let _shortest_read = shortest_read_username();
}
