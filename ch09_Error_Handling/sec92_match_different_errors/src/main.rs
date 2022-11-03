use std::fs::File;
use std::io::ErrorKind;

/*
If you wanted to match based on multiple different possible errors, nested match statements is a possible
solution for that. Granted nested code is a bit messy to my eyes so I wonder if there is a cleaner more
readable way to write this.
*/
fn main() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,

        /*
        The type that `File::open` returns inside the `Err` variant is `io::Error`, which is a struct
        provided by the standard library.

        This struct has a method `kind()` that we can call to get an `io::ErrorKind` value. The enum
        `io::ErrorKind` is provided by the standard library and has variants representing the different
        kinds of errors that might result from an `io` operation.
        */
        Err(error) => match error.kind() {
            // If the error is of type `NotFound`, then let's match based on result of `File::create`
            ErrorKind::NotFound => match File::create("hello.txt") {
                // If ok, return the created file
                Ok(fc) => fc,
                // Else panic, we couldn't create the file
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },

            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    /*
    As stated previously, that's a lot of match statements being used and not very easy to read. A way
    to clean this up would be through closures which has not been covered yet in the textbook.

    Closures end up being used with many of the methods defined on `Result<T, E>`. This results in the
    code being more concise rather than using `match` statements.

    Here's the same code as above reformatted using `closures` and `unwrap_or_else()`.
    */

    let _alternate_file = File::open("alternate.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("alternate.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
