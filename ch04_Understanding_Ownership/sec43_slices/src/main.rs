fn main() {
    let s = String::from("hello world");
    
    // Create slices using range within brackets: [starting_index..ending_index]
    // .. is the range syntax
    let hello = &s[0..5];
    let world = &s[6..11];
    
    // These are equivalent
    let slice = &s[0..2];
    let slice = &s[..2];
    
    // These are also equivalent
    let another_slice = &s[3..len];
    let another_slice = &s[3..];
    
    // As well as these are equivalent
    let yet_another_slice = &s[0..len];
    let yet_another_slice = &s[..];
}

fn first_word(s: &String) -> usize {
    // Convert the string to bytes so we can check element by element if there's a space
    let bytes = s.as_bytes();
    
    // Create iterator to iterate over the string. iter() returns each element in a collection.
    // enumerate() wraps the result of iter() and returns each element as part of a tuple instead.
    // i is index, &item is the single byte in the current tuple
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    
    s.len()
}

// &str signifies returning a string slice
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        
        // If we detect a space/whitespace, return the string slice up till that point
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    // Else return length of string
    &s[..]
}
