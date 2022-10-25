// Constants are similar to immutable variables, except they may NEVER be changed to be mutable and
// they MUST have their type declared. Also their value will NEVER change. Even due to shadowing.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    
    // This code won't compile, remember by default variables are immutable in Rust.
    // To fix this, add mut to the variable x decleration
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}