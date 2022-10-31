use std::collections::HashMap;

fn main() {
    /*
    For types that implement the `Copy` trait, like `i32`, the values are COPIED into the HashMap.
    For owned values like `String`, the values will be moved and the HashMap will be the owner of
    those values.
    */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point... After using `insert()` ownership has
    // been moved.

    /*
    However if we instead used references to move `field_name` and `field_value` into the HashMap,
    ownership wouldn't move to the HashMap it would stay with `field_name` and `field_value`.

    However! The references must have the SAME lifetime as the HashMap. In other words, the references
    must point to valid data at least as long as the HashMap is valid.
    `
        map.insert(&field_name, &field_value);
    `
    */

    /* =========== Updating a Hash Map ========== */

    /*
    In a HashMap each unique key can only have 1 value associated with it at a time.

    When you want to change data in a HashMap you can do it one of a few ways:
        1. Replace the old value with the new value
        2. Keep the old value and ignore the new value (only adding the value if no value existed previously)
        3. Combine the old value and the new value
    */
    let mut scores = HashMap::new();

    // 1. Overwrite a Value

    /*
    If you insert a value with a unique key into a HashMap, and then insert a new value to that same
    key again, the old value will be replaced.
    */
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // Here the print statement will print out `{"Blue": 25}`
    println!("{:?}", scores);

    // 2. Add a Key and Value ONLY if a Key doesn't exist

    scores.insert(String::from("Blue"), 10);

    /*
    HashMaps have a special API `entry()` that takes the key you want to check as a parameter.
    The return value of `entry()` is an enum `Entry` that represents a value that MIGHT or MIGHT NOT
    exist.

    The `or_insert()` is defined to return a mutable reference to the value for the corresponding
    `Entry` key IF that key exists. If now, it inserts the parameter as the new value for this key
    and returns a mutable reference to the new value. (This technique is much cleaner than writing
    the logic ourselves, and plays more nicely with the borrow checker.)
    */
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    // Because the second call to `entry()` on the `Blue` key already has a value. It's value won't change.
    // Prints out: {"Yellow": 50, "Blue": 10}
    println!("{:?}", scores);

    // 3. Updating a Value based on the Old Value

    /*
    Another common use case of HashMaps is to look up a key's value nd then update it based on the old
    value.

    The following counts how many times each word appears in some text.
    */
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // Remembering when iterating over a HashMap the keys are never in a set order
    for word in text.split_whitespace() {
        // `or_insert()` returns a mutable reference `&mut V`. So we need to dereference it with `*`.
        let count = map.entry(word).or_insert(0);
        *count += 1;
    } // The var `count`, aka a mutable reference goes out of scope here. So all changes are safe
      // according to borrowing rules.

    // Print out: {"world: 2, "hello": 1, "wonderful": 1}
    println!("{:?}", map);

    // 4. Hashing Functions

    /*
    By default, HashMap uses a hashing function called `SipHash` that can provide resistance to Denial
    of Service (DoS) attacks involving hash tables. IT's not the fastest hashing algorithm available
    but the trade-of for better security versus the drop in performance is worth it.

    If the default hashing function is too slow for your program you can switch to another function
    by specifiying a different `hasher`. A `hasher` is a typ that implements the `BuildHasher` trait.
    `crates.io` has libraries that provide hashers implementing many common hashing algorithms.
    */
}
