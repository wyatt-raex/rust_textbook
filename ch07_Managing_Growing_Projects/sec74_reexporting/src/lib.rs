mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

/*
When we utilize the `use` keyword, the name available in the new scope is initially private.
If we want code to be able to utilize this `use` statement that is located outside of the
scope it's declared in, we can attach `pub` to the use statement.

This process is known as 're-exporting', because we are bringing an item into scope but also
making that item available for others to bring into their scope.
*/
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

/*
Before using `pub use`, if any other function wanted to call `hosting::add_to_waitlist()` they
would have to utlize the path:
    `restaurant::front_of_house::hosting::add_to_waitlist()`.

However, now that we have declared the `use` statement on line 15 to be `pub use`, outside code
can now just call utilizing the path:
    `restaurant::hosting::add_to_waitlist()`
*/
