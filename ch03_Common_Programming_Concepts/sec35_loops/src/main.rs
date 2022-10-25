fn main() {
    let mut count: u8 = 0;

    // loop keyword will execute block of code until you tell it to stop.
    loop {
        count += 1;
        println!("And the yo-yo master keeps yo-yo-ing!");
        if count >= 10 { break; }
        
        continue;
        /*println!("Notice with the continue keyword this will never get printed!");
        println!("That's because the continue keyword tells rust to skip all the");
        println!("rest of the code in the loop and proceed to the next loop iteration.");*/
    };
    
    // Returning Values from Loops
    count = 0;
    let result = loop {
        count += 1;
        
        if count == 10 {
            break count * 2;
        }
    }; // You need a semicolon here to end the assignment to result variable
    
    println!("\nAnd the result is {result}.\n");
    
    // Loop Labels
    count = 0;
    
    // Notice the label proceeds with an '
    'counting_up: loop {
        println!("count = {count}.");
        let mut remaining = 10;
        
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    
    println!("End count = {count}.\n");
    
    // While loops
    count = 3;
    
    while count != 0 {
        println!("{count}");
        count -= 1;
    }
    println!("LIFTOFF!!!\n");
    
    // For loops
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("The value is: {element}.");
    }
    
    println!("\nAnother countdown.\n");
    
    // Use rev() to reverse the range (1..4)
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!");
}
