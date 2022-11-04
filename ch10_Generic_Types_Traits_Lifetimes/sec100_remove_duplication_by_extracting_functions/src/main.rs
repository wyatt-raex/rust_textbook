/*
Generics allow us to replace specific types with a placeholder that represents multiple types to remove
code duplication.

Before doing generics however, lets look at a similar concept of removing duplicate code by extracting
functions from said duplicated code.
*/

fn find_largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest // return statement
}

fn main() {
    // Find the largest number in a given vector of i32
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    // Say we were asked to find the largest number in a different list, we could just duplicate the code:
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    /*
    Although this works (to duplicate the code), it is tedious and error prone. We also have to remember
    to update the code in multiple places if we need or want to make changes to it.

    Instead we can abstract this code into it's own function, removing all instances of it's duplication
    making it easier to manage and change later.
    */

    let number_list = vec![34, 50, 25, 100, 65];

    let result = find_largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = find_largest(&number_list);
    println!("The largest number is {}", result);
}
