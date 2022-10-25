/*
Tuple structs are basically structs without having names for each field. Simply just their type.
Good example is like below for Color or Point. It can already be assumed you mean rgb with
color or xyz with point.
*/
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // Note this is not possible. Even though Color and Point are essentially the same minus the name.
    // They are still different structs.
    // example_function(&origin); // CANT PASS A POINT WHEN WE'RE EXPECTING A COLOR!!!
    example_function(&black);
}

fn example_function(_param: &Color) {
    println!("I'm not really gonna do anything. This is an example.");
}
