/*
3 Rules of Ownership in Rust:

1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.
*/

fn main() {
    // Quick thing on scope

    // s is not valid here, not yet declared
    {
        let _s = "hello"; // s is now valid
    } // scope now over. s had been dropped.

    // :: operator allows us to namespace from() under the String type. Instead of using string_from().
    // This type of string can be mutable.
    let mut s = String::from("Hello");

    s.push_str(", world!");
    println!("{}", s);

    /*
    Why can String types be mutable, but not string literals?

    String literal content is known at compile time. So it's hardcoded into final executable.
        - This makes them REALLY fast & efficient to use.

    String literals are actually a str slice type. Meaning they are an immutable reference.
    */

    // Way Variables and Data Interact: Move
    let x = 5;
    let _y = x;

    /*
    5 get's bound to x, the value stored from x gets bound to y.
    Both x's value and y's value get's pushed to stack because the size is fixed and known.
    */

    let s1 = String::from("hello");
    let _s2 = s1;

    /*
    In the case of s1 & s2, s1 allocates space on the heap for the String. Then has a pointer
    point to it. Then s2 gets assigned to the value of s1, so it also points a pointer to the
    location in memory containing the string "hello".
    */

    // If you try this it won't work
    // println!("{}, world!", s1);

    /*
    Why? Rust automatically prevents a double-free error by "moving" ownership of the string
    "hello" from s1 to s2 during line 41.
        - If Rust didn't do that, when s1 & s2 go out of scope and get dropped, they would both
          be trying to free the same block of memory: the string "hello". Giving a double-free error.
    */

    // Ways Variables and Data Interact: Clone

    // What if we do want both variables to have a deep copy of the string?
    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);

    /*
    That's where .clone() comes in. It will make a copy of the data stored as well as the value
    allocated to the heap for each variable. So now we have two strings located in the heap, with
    2 variables in the stack.
    */
}
