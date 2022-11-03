fn main() {
    let v = vec![1, 2, 3];

    /*
    Rust will panic here to prvent a buffer overread (reading memory outside of what is allocated
    for a given data_structure, variable, etc.).

    We can `backtrace` this by setting `RUST_BACKTRACE` to anything but `0` when executing the command
    `cargo run`. So in total it'll look like this: `RUST_BACKTRACE=1 cargo run`

    A `backtrace` is a list of all the functions that have been called to get to the point of the panic.
    Backtraces in Rust work as they do in other languages: the key to reading the backtrace is to start
    from the top and read until you see files you wrote. That's the spot where the problem orginated.
    */
    v[99];
}
