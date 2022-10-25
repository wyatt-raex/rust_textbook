mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
/*
`Use` creates a valid shortcut only for the particular scope in which the `use` occurs.
Remember `crate` specifies the `root` of the project/crate/library. In this case src/lib.rs
*/
use crate::front_of_house::hosting;

// For example this will fail
mod customer {
    pub fn go_eat() {
        // hosting::add_to_waitlist(); // ERROR

        // To make the above error work, remember we can use super to grab the prior scope
        super::hosting::add_to_waitlist();
    }
}

// But this succeeds because it's in the proper scope.
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
