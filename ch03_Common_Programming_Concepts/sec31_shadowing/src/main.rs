/*
Ok, so I get that shadowing allows us to change the value and even data type of a variable without
making it mutable. But the textbook/documentation states that each call of "let" is creating a new
variable.

So how many variables are in this program? 1, 2, or 3? 3 for each call of let? 2 for each layer of
scope? Or just 1 (least likely in my mind)?
*/

fn main() {
    let x = 5;
    
    let x = x + 1;
    
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    
    println!("The value of x is: {x}");
    
    // Shadowing effectively creates a new variable, that way it is possible to "change" it's type.
    // I assume behind the scenes it is creating a new variable and then getting rid of the old one.
    // For example the following works:
    let spaces = "    "; // Old variable. Can't access this value after next line.
    let spaces = spaces.len(); // New variable that gets assigned to same variable name.
    
    // However the following does not compile.
    // You can not change a mut variable's type.
    let mut spaces_2 = "    ";
    spaces_2 = spaces_2.len();
}
