/*
The Option enum is defined by the standard library.
The Option type encodes the very common scenario in which a value could be
something or it could be nothing.

Rust does not have the `NULL` feature.

In languages with null, variables can always be in one of two states:
    - NULL
    - not NULL

The problem with NULL values is that if you try to use a NULL value as a not-NULL
value, you'll get an error of some kind. e.g. the following would crash in another language:

`
    String gonna_crash = NULL;
    print(gonna_crash.length()); // ERROR!!! Can't get length of NULL
`

However, the concept/value of NULL is still useful for programming. The problem with it is
not the concept itself, but the implementation in most languages. As such Rust has an enum
defined in the Standard Library: `Option<T>` which can encode the concept of a value being
present or absent.

`
    enum Option<T> {
        None,
        Some(T),
    }
`
*/

fn main() {
    /*
    Because Option<T> enum is so useful, it's included in the prelude for Rust; aka, you don't
    need to include it/bring it into scope directly. This also means you can use Some and None
    without the Option:: prefix.
    */
    let some_number = Some(5);
    let some_char = Some('e');

    /*
    Because we are passing None as the variable definition here the compiler can't figure out
    what type exactly `absent_number` is supposed to be. So we MUST tell it what type it's
    supposed to be: `Option<i32>`.

    We need to tell it it's of enum type `Option` so we can utilize None I pressume.
    */
    let absent_number: Option<i32> = None;

    /*
    Keep in mind that `Option<T>` and `T` (where `T` can be any type) are different types.
    The compiler won't let us use an `Option<T>` value as i it were definitely a valid value.
    For example the following code won't compile because we are trying to add an `i8` to an
    `Option<i8>`.
    */
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; ERROR!!!

    /*
    Remember the `Some` variant in `Option<T>` enum only infers that we have some value
    that is present. And that that value is held inside the `Some` variant.

    So in order to utilize an `Option<T>` value with `T` values, you must first convert
    `Option<T>` to `T`. Doing so will prevent many common errors/issues you would encounter
    with NULL ("That issue being: assuming that something isn't NULL when it actually is").

    So how can you convert an `Option<T>` value to `T`?
    The `Option<T>` enum has many methods that can be useful in many situations to possibly
    handle just that.

    In general, in order to use an `Option<T>` value, you want to have code that will handle
    each variant. For example you can utilize a match expression (known as switch case in
    other languages) to handle code that will run when you have `Some(T)` value and the code
    is allowed to use the inner `T`. And you will want other code to run if you have `None`
    value, and that code doesn't have a `T` value available.
    */
}
