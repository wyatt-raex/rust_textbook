struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // For structs in Rust, either the entire struct is mutable, or not at all.
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // To access values from struct, use dot notation
    user1.email = String::from("anotheremail@example.com");

    let _user2 = build_user(
        String::from("myemail@email.com"),
        String::from("myusername"),
    );

    // Struct Update Syntax

    // MOVE some of user1's data into user3
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("yetanother@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Another way to do the same thing (MOVE user3's data into user4)
    let _user4 = User {
        email: String::from("yetanother@example.com"),
        ..user3 // Make the rest of the fields be the same as set in user3 (which was set the same as user1)
                // the ..user3 must come last
    };
}

// Using Field Init Shorthand

// We don't pass in email and username as references nor &str string slices because we want to
// transfer ownership of the email and username to the struct.
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
