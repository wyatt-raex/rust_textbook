use rand::Rng;
use std::cmp::Ordering;
use std::io;

/*
Remember the guessing game from Chapter 2? We ask the user to guess a number from 1 to 100 but we never
actually checked to ensure that the user's input was a `u32` (Check that they never inputted a negative
number). One way to do this, as shown below the boilerplate code, is to parse the guess as an `i32`
instead of only a `u32` to allow potentially negative numbers, and then add a check for the number being
in range.
*/
fn main() {
    loop {
        /* Boilerplate, Ignore this START */
        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1..=100);

        loop {
            println!("Please input your guess.");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            /* Boilerplate, END */

            let guess: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            if guess < 1 || guess > 100 {
                println!("The secret number will be between 1 and 100.");
                continue;
            }

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    }
}

/*
However, this is not an idea solution: if it was absolutely critical that the program only operated on
values between 1 and 100, and it had many functions with this requirement, having a check like the one
above in every function would be tedious (and might impact performance).

Instead, it would be better to make a new type and put the validations in a function to create an
instance of the type, rather than repeating the validations everywhere. That way, it's safe for functions
to use the new type in their signatures and confidently use the values they receive.
*/

pub struct Guess {
    value: i32,
}

impl Guess {
    // "Constructor"
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    // "Getter"
    pub fn value(&self) -> i32 {
        self.value
    }
}
