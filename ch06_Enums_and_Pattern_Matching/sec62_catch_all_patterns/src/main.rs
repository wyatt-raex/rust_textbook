fn main() {
    let dice_roll: u8 = 9;

    /*
    This match statement fufills the requirements of match statements being exhaustive
    because the final arm `other` takes care of all other possibilities.
    */
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    /*
    This match statement also fulfills the same requirements of match statements being exhaustive.
    The only difference compared to the above example is now, we don't care to use the value of
    dice_roll if it's any value besides 3 or 7.
    */
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    /*
    Same as above this match statement fufills requirements to be valid. Now not only are we
    not caring about the value of dice_roll in the `_` arm, we're also not doing anything with
    it by giving that case an empty tuple `()` to execute.
    */
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
