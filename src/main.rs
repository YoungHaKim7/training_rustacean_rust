// Implementing the "Summary" trait on the "NewsArticle" and "Tweet" types
// how a binary crate could use our "aggregator" library crate

use training_rustacean_rust::Tweet;
use training_rustacean_rust::Summary;

fn main() {
    let tweet = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet : {}", tweet.summarize());
}
