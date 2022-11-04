fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest // return statement
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest // return statement
}

/*
We use generics to create definitions for items like function signatures or structs, which we can then use
with many different concrete data types.

Let's first look at generics in function definitions.

When we use a parameter in the body of the function, we have to declare the parameter name in the
signature so the compiler knows what the name means.

Similarly, when we use a type parameter name in the function signature, we have to declare the type
paramaeter before we use it. Hence the `<T>` in `fn largest<T>() {}`.

We read this definition as: "the function `largest` is generic over some type `T`. This function has
one parameter named `list`, which is a slice of values of type `T`. The `largest` function will
return a reference to a value of the same type `T`".
*/
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        // NOTE: this code won't compile YET because we havn't specified a trait of the generic type `T`
        // saying `T` is comparable. (Or at least that is my assumption before reading about traits)
        /*if item > largest {
            largest = item;
        }*/
    }

    largest
}

// Generics in Struct Definitions

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
