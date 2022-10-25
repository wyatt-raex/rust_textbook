fn statements_expressions() {
    // The following statement works
    let y = {        // Begin over-arching statement
        let x = 3;   // Another statement
        x + 1        // Expression, note no semi-colon
    };               // End over-arching statement   
    println!("The value of y is: {}", y);
    
    /*
    Different types of expressions include:
    - Calling a function
    - Calling a macro
    - Creating a new scope with curly brackets
    - A math operation that evaluates but doesn't assign to anything
    */ 
}

fn main() {
    println!("Hello, world!");
    
    another_function(5, 'h');
    
    let x = return_me_five();
    println!("Here's the value of new x: {x}");
}

/* 
Rust uses snake case style for funcitons.

Rust doesn't care where you define your functions. another_function() could be declared before main,
after main, in another file, etc. As long as the function being called is declared somewhere in a
scope that can be seen from the caller.

Parameter syntax is the same as Typescript.
*/
fn another_function(x: i32, c: char) {
    println!("Another function.");
    println!("The value of x is: {x}.\nThe choosen letter is {c}");
    
    statements_expressions();
}

// Return type is noted after ->
// Note 5 is just an expression. If we put a semi-colon we'll get an error.
fn return_me_five() -> i32 {
    5
}