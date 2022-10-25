fn main() {
    let num = 3;
    
    if num < 5 {
        println!("Condition was true");
    }
    else if num == 3 {
        println!("Number was 3");
    }
    else {
        println!("Condition was false");
    }
    
    // Using if in a let statement
    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is: {number}.");

    // However this is not possible. Both arms (true & false branches) must be the same type!
    // let number = if condition {5} else {"six"};
}
