/*
How do you decide when you should call `panic!` and when you should return `Result`?
    - When code panics, there's no way to recover
    - When you choose to return a `Result` value, you give the calling code options to handle it

Returning `Result` is a good default choice for when you're defining a function that might fail.

`unwrap()` and `expect()` are understood as methods that are used as markers to designate where code can
fail, and are used before you've decided how to deal with the errors. As such they are handy in examples,
where verbose error-handling code would take away from learning the concept being shown, as well as in
program prototyping, where you're looking to just build functionality of your program.

If a method fails in a test, you'd want the whole test to fail, even if that method isn't the
functionality under test. Because `panic!` is how a test is marked as a failure, calling `unwrap()` or
`expect()` is exactly what should happen.
*/

use std::net::IpAddr;

fn main() {
    /*
    It's also appropriate to call `unwrap()` or `expect()` when you have some other logic that ensures the
    `Result` will have an `Ok` value, BUT the logic isn't something the COMPILER understands.

    To put in other words, you have a `Result` value you know will be `Ok` no matter what, but the compiler
    isn't smart enought to know that. Therefor, you still need to handle the `Result` value (example below):
    */
    let _home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    // As stated a hardcoded ip addr should never be wrong, but we still have to handle the `Result` from
    // `parse()`.
}
