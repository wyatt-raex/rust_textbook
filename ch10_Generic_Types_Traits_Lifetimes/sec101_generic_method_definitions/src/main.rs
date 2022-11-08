struct PointBase<T> {
    x: T,
    y: T,
}

/*
Again in reference to generic functions and structs, we can use generic data types on struct implementation
methods. Note here: We have to declare `T` just after `impl` so we can use `T` to specify that we're
implementing methods on the type `Point<T>`.

Rust can identify that the type in the angle brackets in `Point` is a generic type rather than a concrete
type. We could have chosen a different name for this generic parameter than the generic parameter we declared
in the struct definition, but usin ght esame name is conventional.
*/
impl<T> PointBase<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

/*
We can also specify constraints on generic types when defining methods on the type. We could for example,
implement methods only on `Point<f32>` instances rather than on `Point<T>` instance with any generic type.
In the Example below, we use the concrete type `f32`, meaning we don't ddeclare any types after `impl`.
*/
impl PointBase<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

/*
Generic type parameters in a struct definition aren't always the same as those you use in that same struct's
method signatures. The example below uses the generic types `X1` and `Y1` for the `Point` struct and `X2`
and `Y2` for the `mixup()` method signature to make the example clearer. The method creates a new `Point`
instance with the `x` value from the `self` `Point`(of type `X1`) and the `y` value from the passed-in
`Point`(of type `Y2`).
*/
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p = PointBase { x: 5, y: 10 };

    println!("p.x = {}, p.y = {}", p.x(), p.y());

    // Can't do: Because `p` as a `Point` is `Point<T>` not `Point<f32>`
    //println!("distance from origin: {}", p.distance_from_origin());

    let pf32 = PointBase { x: 5f32, y: 10f32 };
    // Now we can do it with `pf32`
    println!("distance from origin: {}", pf32.distance_from_origin());

    // Third example with mixup
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    // will result in : p3.x = 5, p3.y = 'c'
}
