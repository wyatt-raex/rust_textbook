/*
Note: Traits are similar to a feature often called 'interfaces' in other languages, although with some
differences.

In this example we're going to have multiple structs that hold various kinds and amounts of text.
- A `NewsArticle` struct that holds a news story filled in a particular location.
- A `Tweet` that can have at most 280 characters along with metadata that indicates whether it was a new
tweet, a retweet, or a reply to another tweet.

Let's then make a `Trait` called `Summary` that acts as an `interface` to each of these structs. Where we
will be able to implment a method `summarize()` on each strut that implements this trait `Summary`.
*/

/*
A type's behavior consists of methods we can call on that type. Different types share the same behavior
if we can call the same methods on all of those types. Trait definitions are a way to group method
signatures together to define a set of behaviors necessary to accomplish some purpose.

To declaer a `trait` we use the `trait` keyword followed by the trait's name. We've also declared the
trait as `pub` here so that crates depending on this crate can make of use of it as well.

Inside the curly braces, we declare the method signatures that describe the behaviors that implement this
trait. After the signature, instead of providing an implementation in curly braces, we use a semi-colon.
Every type that implements this type `Summary` must implement it's own custom behavior for each of the
method signatures listed below. And the compiler will enforce that each type will have the method
`summarize` defined with the below signature exactly.
*/

// Remember traits are similar to interfaces
pub trait Summary {
    //fn summarize(&self) -> String;

    /*
    We can also implment default behavior for some or all methods in a trait.
    That behavior can then be overrode in each type that uses this trait.
    */
    /*fn summarize(&self) -> String {
        // Define default behavior within these curly braces
        String::from("(Read more..)")
    }*/

    /*
    Default implementations can call other methods in the same trait, even if those
    other methods don't have a default implmentation. For example, we could define the
    Summary trait to have a `summarize_author()` method whose implementation is required,
    and then a `summarize()` method that has a defualt implementation that calls `summarize_author()`.
    */
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Let's give NewsArticle the Summary trait with it's default implementations
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // Override the default implmentation
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
