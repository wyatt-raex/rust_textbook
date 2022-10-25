/*
You only need to load a file using a `mod` declaration ONCE in your module tree.
The compiler from that point onwards will know the file is part of the project as
well as where the imported file is located from.

Other files in your project should refer to the loaded file's code using a path
to where it was declared.

In other words `mod` is NOT an `include` operation.
*/
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
