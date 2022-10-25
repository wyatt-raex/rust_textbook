fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    /*
    If there's no value inside the Option<i32> do nothing.
    If there's a value inside the Option<i32> add 1 to said value.
    */
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }

    /*
    Matches are exhaustive. In other words, the arms MUST cover ALL possibilities.
    For example the following would not compile:
    `
        match x {
            Some(i) => Some(i + 1),
        }
    `
    Why? Because we're using Option<i32> in the match statement we must have arms
    that take care of every possibility of what we're matching. In this case we're
    missing an arm to handle what to do if x is None.
    */
}
