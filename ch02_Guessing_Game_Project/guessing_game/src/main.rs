// Includes io library which is located in standard library (std)
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    // Generate a random number between 1 and 100; including upper and lower bounds
    let secret_num = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is: {}", secret_num);
       
    // Loop keyword with no conditions creates infinite loop
    loop { 
        println!("Please input your guess.");
    
        /* 
            "let" is used to create a variable   
            by default, variables in Rust are immutable:
                - meaning that once we give the variable a value. The value won't change.
                - more than likely similar to a const variable
    
            in order to make a variable mutable or "non-constant" we add the keyword mut
            when declaring a variable
        */ 
        let mut guess = String::new();
    
        /* 
           read_line() reads input it's given and appends it to the string passed as parameter
           &: indicates something is a reference. References are imutable by default,
           so instead of passing read_line(&guess) we pass read_line(&mut guess) so we can change guess.
     
           expect(...) is one way to handle a Result value from read_line(). Without this the compiler will 
           give a warning. The Result in this case is an enum which can return an OK state or an ERR (error) 
           state.
        */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        /*
            Rust allows you to shadow a previous value of a variable.
    
            Not much detail is given now, but basically one of it's uses is:
            In the context where you want to convert the value of a given variable to another type, ususally
            you would have to create two different variables for it.
    
            Example:
            guess -> which is a number
            guess_str -> which converts to a string
    
            Rust just allows us to use one variable when converting types via Shadowing.
    
            Moving on, trim() removes whitespace from a string. parse() converts one type to another.
    
            u32 is a numeric type: unsigned integer 32 bit. Similarly there is i32 (integer 32 bit),
            u64, and i64 respectively. Unless otherwise specified, when declaring a numeric variable,
            Rust defaults to i32.
    
            let guess: u32 = ...
            The ":" in the statement above tells Rust we are explicitly using u32 data type to convert.
            I assume this syntax can also be used in default variable creation.
        
            Use match expression to handle Result type if we get a non-number input. For now the program
            will simply ignore if it's been given a non-number input and continue running.
            
            Within Err(_) => ... the "_" is a "catchall" value; we want to match all Err values, no matter
            what information is inside them.
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        /* 
           {} in Rust act as placeholders when printing strings.
           Could also write this as:
           println!("You guessed: {}", guess);
        */
        println!("You guessed: {guess}");   
    
        // Match is similar to switch case statement
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Exit the loop
            }
        }
    } 
}
