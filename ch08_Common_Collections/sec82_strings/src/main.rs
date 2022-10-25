/*
String slices literally mean char* arrays, can also mean substring

Rust has 2 "Strings". 1 is the `&str` string slice, implemented in the Rust core.
2 is the Standard library `String`, which is growable, mutable, and owned.

Both string types utilize UTF-8 encoding.
*/

fn main() {
    /*
    Many of the same operations available with `Vec<T>` are available with `String`. That's
    because `String` is actually implemented as a wrapper around a vector of bytes, with
    some extra guarantees, restrictions, and capabilities.
    */

    // Create a new string
    let mut _s = String::new();

    /*
    If you have some "data" initially that you want to load into a `String`, you can do so
    utilizing the `to_string()` method.
    */
    let data = "initial contents";
    let _s = data.to_string();

    // the method also works on a literal directly:
    let _s = "initial contents".to_string();

    // You can also utilize String::from() to the same effect. That is create a string from a
    // String literal
    let _s = String::from("initial contents");

    /*
    Because strings are used for many things, we can use many different generic APIs for strings.
    It may seem redundant, but they all have their place. In THIS case, `String::from()` and
    `to_string()` do the same thing, so whichever you choose is matter of personal preference,
    style, and readability.
    */

    // A `String` can grow in size and its contents can change, just like the contents of a `Vec<T>`.

    // Appending a string slice to a `String`.
    let mut _s = String::from("foo");
    _s.push_str("bar");

    // `push_str()` takes in string slices/string literals, so ownership is respected.
    let mut _s = String::from("foo");
    let s2 = "bar";
    _s.push_str(s2); // s2 retains ownership of s2. It doesn't get passed to _s
    println!("s2 is {}", s2); // We can still print out s2 :D

    // If we want to add a single character to a string, use `push()`.
    let mut _s = String::from("lo");
    _s.push('l');

    // Concatenation works as expected
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2; // note: s1's ownership has been taken by _s3, so s1 can no longer be used.

    /*
    The + operator when doing concatenation, specifically calls the `add()` method. `add()`'s signature
    looks something like this:
    `
        fn add(self, s: &str) -> String {...}
    `
    */

    let str1 = String::from("tic");
    let str2 = String::from("tac");
    let str3 = String::from("toe");

    // You can also concatonate multiple strings, but there is a better way visually to do it.
    let _s = str1 + "-" + &str2 + "-" + &str3;

    // For more complicated string combining, we can instead use the `format!` macro
    let cstr1 = String::from("tic");
    let cstr2 = String::from("tac");
    let cstr3 = String::from("toe");

    let _s = format!("{}-{}-{}", cstr1, cstr2, cstr3);

    /* ========== Indexing into Strings ========== */
    /*
    Unlike other programming languages, you can't index into Strings using array syntax

    ERROR
    `
        let s1 = String::from("hello");
        let h = s1[0];
    `

    But why not? Why doesn't Rust support String indexing?
    That's due to how Rust stores strings in memory.
    */

    // Strings are a wrapper over a `Vec<u8>`
    let _hello = String::from("Hola");

    /*
    In the above string, each character is 1 byte long and encoded in UTF-8. However the following
    string actually takes 2 bytes per character.
    */
    let _hello = String::from("Здравствуйте");
    /*
    If we were to do:
    `
        let answer = &_hello[0];
    `

    `answer` would not hold З, the first letter. For Russian, and other languages, sometimes need
    multiple bytes to store a character in UTF-8. So for 3 it would be two bytes: 208 & 151.
    Returning 208 to `answer` is not what a user would want/would have asked for.

    So to avoid returning unexpected values and causing bugs, Rust refuses to compile code
    containing string indexing.
    */

    // Slicing Strings

    /*
    Indexing into a string is often a bad idea because it's not clear what the return type of the
    string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string
    slice. If you really need to use indices to create string slices, Rust asks you to be more specific.

    Rather than indexing using `[]` with a single number, you can use `[]` with a range to create a
    string slice containing particular bytes:
    */
    let _slice = &hello[0..4];

    /*
    So in this case `_s` will be a `&str` that contains the first 4 bytes of the string `hello`: Зд

    If you instead tried something like: `&helo[0..1]`, Rust will panic at runtime the same way as
    if an invalid index were accessed in a vector.

    You should use ranges to create string slices with caution, because doing so can crash your program.
    */

    /* ========== Methods for Iterating Over Strings ========== */

    /*
    The best way to operate on pieces of strings is to be explicit about whether you wan characters
    or bytes.
    */
    for c in "Зд".chars() {
        println!("{}", c);
    }
    /*
    Output:
    З
    д
    */

    for b in "Зд".bytes() {
        println!("{}", b);
    }
    /*
    Output:
    208
    151
    208
    180
    */
}
