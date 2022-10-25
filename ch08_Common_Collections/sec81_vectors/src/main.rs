/*
Vectors allow you to store more than one value in  single data structure that puts
all the values next to each other in memory. However, Vectors can only store values
of the same type.

They are useful when you have a list of items, such as the lines of text in a file
or the pices of items in a shopping cart.
*/

fn main() {
    let _v: Vec<i32> = Vec::new();

    /*
    Rust conveniently provides `vec!` macro, which will create a new vector that holds
    the values you give it.
    */
    let _v = vec![1, 2, 3];

    // Once again, even vectors need to be declared mutable in order to be changed.
    let mut v = Vec::new();

    // Use push method to add elements to a vector.
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);

    // Reading Elements of Vectors

    // Two ways to reference a value stored in a vector: via indexing or
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    /*
    via using the get method.

    When you use `get()` method it returns an `Option<&T>` so we should handle it afterwords
    with a match or something similar.
    */
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    /*
    There's different behavior for trying to access a element that is out of bounds depending
    on which method you use:

    `
        let v = vec![1, 2, 3, 4, 5];

        let does_not_exist = &v[100]; // This will cause the program to panic and crash
        let does_not_exist = v.get(100); // This will return a `None` and doesn't panic/crash.
    `
    */

    /*
    When the program has a valid reference, the borrow checker enforces the ownership and borrowing
    rules to ensure this reference and any other references to the contents of the vector remain
    valid.

    Recall, you can't have mutable and immutable references in the same scope. This still applies
    in the example below, where we hold an immutable reference to the first element in a vector
    and then try to add an element in the end.

    `
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];

        v.push(6);
        println!("The first element is: {}", fist);
    `

    Even if we try to refer to `first` later in the function, it would still be an error.

    The error the compiler is helping you avoid here deals with pointing to now de-allocated memory.
    When you push values to a vector you may need more allocated memory, so it could end up copying
    the vector from it's current memory location to a new location and de-allocating the old memory.

    If you have a pointer pointing to that previous memory location (which is now de-allocated
    memory), you now have an error.
    */

    {
        /*
        Like any other `struct`, a vector is freed when it goes out of scope. When a vector gets
        dropped, all of its contents are also dropped. The borrow checker ensures that any
        references to contents of a vector are only usedwhile the vector itself is valid.
        */
        let _drop = vec![1, 2, 3, 4];
        // do stuff with `drop`
    }
    // <- `drop` now goes out of scope and is freed here
}
