/*
The `if let` syntax lets you combine `if` and `let` into a less verbose way to handle values
that match one pattern while ignoring the rest. Consider the program below that matches on
an `Option<u8>` value in the `config_max` variable, but only wants to execute code if the value
is the `Some` variant.
*/

fn main() {
    // Remember you can right the types of number literals into the number literal itself
    // So 3u8 is an `unsigned 8 bit integer` with the value `3`
    let config_max = Some(3u8);
    match config_max {
        // If the value is `Some` we print out the value in the `Some` variant by binding
        // the value to the variable `max` in the pattern.
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    /*
    Above, in order to satisfy the exhaustive requirement of match statements we need to add
    in the `_` arm. However, there is a shorter option to do this exact thing utilizing
    `if let`.

    However, through using `if let` we do lose the power of exhaustive checking. So bear that
    in mind when you choose whether to use `match statements` or `if let` statements.
    */
    let other_max = Some(10u8);
    if let Some(max) = other_max {
        println!("The maximum is configured to be {}", max);
    }
    // `Else` statements can still be used in `if let` statements.
    else {
        println!("The maximum is not configured.");
    }
}
