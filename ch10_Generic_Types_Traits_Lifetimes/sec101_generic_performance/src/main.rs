/*
You might wonder whether there is a runtime cost when using generic type parameters.
Using generic types won't makeyour program run any slower thanit would with concrete types.
Rust accomplishes this by performing monomophization of the code using generics at compile time.

Monomorphization: The process of turning generic code into specific code by filling in the concrete types
that are used when compiled. In this process, the compiler does the opposite of the steps we used to create
the generic function: "In Listing 10-5, the compiler looks at all the places where generic code is called
and generates code for the concrete types the generic code is called with".
*/

/*
To understand this let's look at the standard library's generic `Option<T>` enum:

```
    let integer = Some(5);
    let float = Some(5.0);
```

When rust compiles the code above, it performs monomorphization. During that process, the compiler reads
the values that have been used in `Option<T>` instances and identifies two kinds of `Option<T>`. One is
`i32` and the other is `f64`. As such it expands the generic definition of `Option<T>` into two
definitions specialized to `i32` and `f64`, thereby replacing the generic definition with specific ones.

The result of the code going through monomorphization looks similar to the code below:
*/

enum OptionI32 {
    Some(i32),
    None,
}

enum OptionF64 {
    Some(f64),
    None,
}

// Remember, because Rust compiles generic code into code that specifies the type in each instance, we pay
// no runtime cost for using generics. When the code runs, it performs just as it would if we had duplicated
// each definition by hand.
fn main() {
    let integer = OptionI32::Some(5);
    let float = OptionF64::Some(5.0);
}
