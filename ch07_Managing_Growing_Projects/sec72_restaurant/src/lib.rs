mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    /*
    The pub keyword can be utilized to make modules and functions public, but also
    it can be used to make Structs and Enums public as well. However, there is a quirk
    to structs. You can make the Struct itself public, but its fields will still be private.
    */

    pub struct Breakfast {
        pub toast: String,      // public
        seasonal_fruit: String, // private
    }

    /*
    Take note that because the struct `Breakfast` implements some private fields we need a
    public constructor for `Breakfast` to handle the defining of the private fields. Otherwise
    we'd either not be able to construct the struct because not all fields were filled, or we
    would be forced to change `Breakfast.seasonal_fruit` to be a public field.
    */
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    /*
    By contrast, if an enum is set to public then it and all of it's variants will be public
    as well.

    According to the textbook, enums aren't very useful unless their variants are public; it
    would be annoying to have to annotate all enum variants with pub in every case. Hence the
    default for enum variants are set to being pub.
    */
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();

        /*
        In using the `super` keyword we go backwords a "directory" relative to the
        location where `super` is called. (This is similar to cd ../)

        So in this case we are within the `back_of_house` module, and we cd ../ relative to
        it into `crate` or the `root`. From there we can access `deliver_order()`.
        */
        super::deliver_order();
    }

    fn cook_order() {}
}

fn deliver_order() {}

pub fn eat_at_restuarant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed to see or modify
    // the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
