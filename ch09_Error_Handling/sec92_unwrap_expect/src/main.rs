use std::fs::File;

/*
Using `match` statements works well enough, but it can be verbose and doesn't always communicate intent well.
The `unwrap()` method for `Result<T, E>` is a shortcut method implemented just like the match expression
wrote in the previous section: "sec92_match_different_errors".

If the `Result` value is the `Ok` variant, `unwrap()` will return the value inside the `Ok`.
If the `Reulst` value is the `Err` variant, `unwrap()` will call the `panic!` macro for us.
*/

fn main() {
    let _greeting_file = File::open("hello.txt").unwrap();

    // Similarly, `expect()` lets us also choose the `panic!` message.
    // In production code, most Rustaceans choose `expect` over `unwrap` to give more context in an error.
    let _greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}
