fn main() {
    /*
    Instead of using indices to iterate over a vector, we iterate through all of the elements.
    */
    let v = vec![100, 32, 57];
    for element in &v {
        println!("{}", element);
    }

    /*
    Can also iterate over a mutable vector and make changes to the elements.

    Remember that you would be accessing references to elements in a vector, so to modify
    the elements in the vector you need to dereference them with `*`.
    */
    let mut v_mut = vec![100, 32, 57];
    for element_mut in &mut v_mut {
        *element_mut += 50;
    }

    /*
    Iterating over a vector, mutable or immutable, is safe because of the borrow checker's rules.
    If we were to try to insert or remove elements in either vector above, because the for loop
    utilizes a reference to the vector to iterate over, the compiler would give us an error.
    */
}
