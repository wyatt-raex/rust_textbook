/*
Vectors can only store values that are the same type. But there's a trick we can use with
enums in order to store a list of items that are different types. The way you do this is
through variants of enums.
*/

fn main() {
    // This enum has variants that are different types.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // This vector ultimately only holds `SpreadsheetCell` enums. But that enum has variants ;)
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    /*
    There is one drawback of this method. Rust NEEDS to know what types will be in the vector
    at COMPILE time, so it knows exactly how much memory ON THE HEAP will be needed to store
    each element.

    If you don't know the exhaustive set of types a program will get at runtime to store in a
    vector, the enum technique won't work. Instead you can use a trait object (Covered in CH17).
    */
}
