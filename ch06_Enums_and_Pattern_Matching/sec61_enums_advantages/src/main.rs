/*
Create enum containing the kinds of IP Addresses: IPV4 or IPV6
As well as the IP associated.

This effectively has the same effect as in code located in:
    sec61_enums_in_structs
however the code here is more concise, though could be more confusing
at first (especially after just looking at/coding the code located in
"enums_in_structs").
*/
enum IpAddr {
    V4(String),
    V6(String),
}

/*
Through attaching data directly to each variant of the enum, there's no need for an extra struct.
Another detail of how enums work is that each variant we define also becomes a function that
CONSTRUCTS an instance of the enum.

In the case above: IpAddr::V4() is a function that takes a String argument and returns an instance
of the IpAddr type. We automatically get this constructor function as a result of defining the enum.

One other advantage to using an enum rather than a struct:
    Each variant can have different types and amounts of associated data.

As such the following can't be done within a single struct:
*/
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

/*
Enums are flexible with what data can be put in them. Even structs can be used inside them.
*/

struct Rectangle {
    width: u32,
    hight: u32,
}

struct Triangle {
    side_length: u32,
}

enum Shapes {
    Rect(Rectangle),
    Tri(Triangle),
}

/*
Another example of enums handling flexible data
    Quit: has no data associated with it at all
    Move: has named fields like a struct does
    Write: includes a single String
    ChangeColor: includes three i32 values
*/
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/*
If we used different structs with each having their own type, we couldn't easily define a
function to take any of these kinds of messages as we could with the Message enum above.

One further similarity between enums and structs: you can also define methods on structs
using impl keyword.
*/
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

/*
Defining an enum with variants such as the one above is similar to defining different kinds of
struct definitions, except the enum doesn't use the struct keyword and all the variants are
grouped toegether under the Message type.

The following code would be the equivalent of the enum above
*/
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32); // tuple struct

fn main() {
    let _home = IpAddr2::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    // Showing we can use functions defined for enums
    let m = Message::Write(String::from("hello"));
    m.call();
}
