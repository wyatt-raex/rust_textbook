This chapter focuses on reviewing previously learned concepts:
- Organizing code (using what you learned about modules in Ch 07)
- Using vectors & strings (collections, Ch 08)
- Handling errors (Ch 09)
- Using traits and lifetimes where appropriate (Ch 10)
- Writing tests (Ch 11)

# 12.1 Accepting Command Line Arguments
To enable our minigrep program to read values of command line arguments we need the 
`std::env::args` function provided in Rust's standard library.

In the `Vec<String>` of arguments the first element is the program name `minigrep` followed by our
arguments. So running `cargo run -- needle haystack` in the terminal will end up with the following
arguments:
    - [0]: "minigrep"
    - [1]: "needle"
    - [2]: "haystack"

# 12.2 Reading a File
We'll `use std::fs` to be able to read data from a file.
