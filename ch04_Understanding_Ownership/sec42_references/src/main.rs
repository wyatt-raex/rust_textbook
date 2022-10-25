fn main() {
    let s1 = String::from("hello");
    
    // Passing &s1 to calculate_length() allows us to refer to the value of s1 within calculate_length()
    // but, it doesn't give ownership of s1 to the function.
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");
    
    // One BIG limitation of mutable references. If you have a mutable reference you can  NOT have any
    // other reference to that value. This is to prevent data races:
    //     - When two or more pointers access the same data at the same time.
    //     - When at least one of the pointers ie being used to write to the data.
    //     - When there's no mechanism being used to synchronize access to the data.
    let mut mutable_ref = String::from("This is ");
    change(&mut mutable_ref);
    println!("{}", mutable_ref);
    
    // However, you can allow for multiple mutable references in the case they are NOT simultaneous
    let mut not_simultaneous = String::from("This mutable reference is not simultaneous!");
    {
        let _r1 = &mut not_simultaneous;
    } // r1 goes out of scope here, so we can make a new reference with no problems
    
    let _r2 = &mut not_simultaneous;
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String. calculate_length() borrows s
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not
  // dropped.

fn change(mutable_ref: &mut String) {
    mutable_ref.push_str("a mutable reference.");
}
