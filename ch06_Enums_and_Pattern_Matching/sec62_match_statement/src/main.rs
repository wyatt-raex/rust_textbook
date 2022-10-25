enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // so we can inspect the state later
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    California,
    Colorado,
    Florida,
    Georgia,
    Hawaii,
    Illinois,
    Nevada,
    NorthCarolina,
    Washington,
    Wyoming,
}

fn main() {
    let my_coin = Coin::Penny;
    println!("What is the value of my_coin? {}", value_in_cents(my_coin));

    let state_coin = Coin::Quarter(UsState::NorthCarolina);
    println!(
        "What is the value of state_coin? {}",
        value_in_cents(state_coin)
    );
}

fn value_in_cents(coin: Coin) -> u8 {
    // Just a switch-case statement with different syntax
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,

        /*
        Match arms can bind to the parts of the values that match the pattern given.
        This is how we extract values out of enum variants.

        So in a run-time example we have a coin:
        `let state_coin = Coin::Quarter(UsState::NorthCarolina);`

        When we call `value_in_cents()` on `state_coin` and come through this match
        statement, `Coin::Quarter(state)` will match the given pattern by the passed
        in parameter `coin`.

        With that, as shown below, we can extract out the 'binded' value inside the
        Coin enum and print out which state the quarter is from.
        */
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
