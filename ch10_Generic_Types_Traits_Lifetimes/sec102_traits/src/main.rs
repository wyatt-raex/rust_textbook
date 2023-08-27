mod lib;
use lib::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    // This code prints out:
    // "1 new tweet: (Read more from @horse_ebooks)"
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };

    // Due to the default implementation defined in lib.rs, this will read out:
    // "New article available! (Read more from Iceburgh...)"
    println!("New article available! {}", article.summarize());
}
