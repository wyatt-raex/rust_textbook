use std::collections::HashMap;

/*
The type `HashMap<K, V> stores a mapping of keys of type `K`, to values of type `V` using a `hashing
function`, which determines how it places these keys and value pairs into memory. In other programming
languages, this data-structure may be known as: hash, map, object, hash table, dictionary, associative
array, etc...

HashMaps are useful when you want to look up data not via an index, but instead using a `key` that could
be of any type. (e.g. in a game you could look up a team's score via the `key`: team_name, and the
corresponding `value` is that team's score).
*/

fn main() {
    /*
    Just like vectors, HashMaps store their data on the HEAP. Similarly to vectors, all keys must be
    the same type, and all values must be the same type.
    */
    // Create a HashMap of teams and their scores.
    let mut scores = HashMap::new();

    // HashMap<String, i32>
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    /*
    You can get a value out of a HashMap using the `get()` method. The `get()` method returns an
    `Option<&V>`; if there's no value for the provided key in the HashMap, `get()` will return
    `None`.

    Otherwise, this program handles the `Option` by calling `copied()` to get an `Option<i32>` rather
    than an `Option<&i32>`. Then it calls `unwrap_or()` to set `score` to `0` if `scores` doesn't have
    any entry for the provided key.

    In other common programming languages the logic flow looks something like this:
    `
        if (scores[&team_name] != Null) { // If the team exists
            if (scores[&team_name].score != Null) {// If a score for the team exists
                return scores[&team_name].score;
            }
            else { return 0; }
        }
    `
    */
    let team_name = String::from("Blue");
    let _scores = scores.get(&team_name).copied().unwrap_or(0);

    /*
    You can also iterate each key/value pair in a HashMap using a for loop, very similar to how
    you would with Vectors. However, don't expect the iteration to be in a specific order of
    <Key, Value> pairs.
    */
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    /*
    TESTING:
    Considering Vectors and HashMaps are similar in terms of they must have the same type.
    I wonder if I can use Enums/Structs to give them the 'same' type, but have different types
    in the Enum/Struct...
    */
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    enum IpAddr {
        V4(String),
        V6(String),
    }

    let _user_addr = IpAddr::V4(String::from("127.0.0.1"));
    let user = User {
        active: true,
        username: String::from("wyatt-raex"),
        email: String::from("testing@email.com"),
        sign_in_count: 1u64,
    };
    let mut enum_struct_map = HashMap::new();

    // enum_struct_map.insert(_user_addr, user);
    enum_struct_map.insert(String::from("127.0.0.1"), user);

    /*
    So it seems HashMaps are perfectly happy to accept Structs, but not Enums
    */
}
