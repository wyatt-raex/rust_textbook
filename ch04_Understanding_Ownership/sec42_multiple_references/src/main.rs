fn main() {
    // You can't have a mutable reference while you have an immutable reference to the same value!
    let mut s = String::from("hello");

    let _r1 = &s; // no problem
    let _r2 = &s; // no problem
                  //let r3 = &mut s; // BIG PROBLEM

    //println!("{}, {}, and {}", r1, r2, r3);

    // A reference's scope lasts from where it's introduced and continues to where it is last used.
    let mut s1 = String::from("goodbye");

    let r4 = &s1; // no problem
    let r5 = &s1; // no problem
    println!("{} and {}", r4, r5);
    // variables r4 and r5 will not be used after this point

    let r6 = &mut s1; // no problem
    println!("{}", r6);
}
