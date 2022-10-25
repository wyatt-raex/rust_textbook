fn main() {
    // Integer Types //
    let int_i8: i8 = 127; // Signed 8 bit integer
    let int_u8: u8 = 255; // Unsigned 8 bit integer
    
    /*
    Similarly there is:
    i16, u16,
    i32, u32,
    i64, u64,
    i128, u128
    */
    
    // Integer who's size will change based on architecture of machine it's run on.
    let arch_i: isize = 30; // Signed arch integer (on my machine 64 bit)
    let arch_u: usize = 30; // Unsigned arch integer (on my machine 64 bit)

    // The primary situation in which you'd use isize or usize is when indexing some sort of collection
        
    // Overflow
    let overflow_0: u8 = 256; // overflow value is 0
    let overflow_1: u8 = 257; // overflow value is 1
    let overflow_2: u8 = 258; // overflow value is 2
    
    /*
    Handling Overflow
    
    Families of methods are provided in the standard library to handle overflow of primitive numeric types.
    
    - Wrap in all modes with wrapping_* methods, such as wrapping_add
    - Return the None value if there is overflow with the checked_* methods
    - Return the value and a boolean indicating whether there was overflow with overflowing_* methods
    - Saturate at the value's minimum/maximum values with saturating_* methods
    */   
     
    // Ways to write Number Literals
    let decimal: u32 = 98_222; // 98,222. Underscores can be used as visual seperators to read numbers easier
    let hex: u32 = 0xff;
    let octal: u32 = 0o77;
    let binary: u32 = 0b111_000;
    let byte: u8 = b'A'; // Byte format can ONLY use u8 data-type.
    
    
    // Floating-Point types //
    let float_64 = 2.0; // f64; Floating-point double-precision number. Follows IEEE-754 standard.
    let float_32: f32 = 3.0; // Floating-point single-precision number. Follows IEEE-754 standard.

    //Floating point numbers default to f64.
    
    // Boolean type //
    let t = true;
    let f: bool = false;
    
    // Character type //
    let c = 'z';
    let z: char = 'Z';
    
    // In Rust, char uses Unicode Scalar Value instead of ASCII. So you can use Japanese, Chinese, and
    // Korean characters if you so desired.
    
    // Chars take up 4 bytes in Rust.
    
    
    /* ----- Compound Types ----- */
    // Compound types group multiple values into one type. Rust has two primitive compound types.
    
    // Tuple Type //
    let tup: (i32, f64, u8) = (500, 6.4, 1);
        
    /*
    A tuple is a compound type that can group different values with different types together.
    Tuples have a fixed length: Once declared, they cannot grow or shrink in size.
    
    We can destructure a tuple, in doing so become able to access individual values, like so:
    */

    let tup2 = (500, 6.4, 1);
    let (x, y, z) = tup2;
    println!("The value of y is: {y}");
    
    /* 
    You can also access tuple elements directly by using a period (.) followed by the index of the
    value you want to access.
    */
    let tup3: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tup3.0;
    let six_point_four = tup3.1;
    let one = tup3.2;
    
    // Arrays //
    // Allow only 1 type. Have a fixed length;
    let numbers = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
                "August", "September", "October", "November", "December"];
    
    let explicit_numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Array of 5 elements typed i32
    let initial_value = [3; 5]; // Array of 5 elements each assigned to default value 3
    
    println!("{}", explicit_numbers[2]); // Access array values as usual.
    
}
