pub struct NewsArticle {
    author: String,
    headline: String,
    content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{} by {}", self.headline, self.author) // The format! macro creates a new String by copying the contents of self.headline and self.author into the new String
    //                                                     //hence, ownership is not moved
    // }
    fn summarize_content(&self) -> String {
        format!("{}", self.content)
    }
}

pub struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
impl Summary for Tweet {
    //overriding the default implementation
    fn summarize(&self) -> String {
        format!("{} by {}", self.content, self.username)
    }

    fn summarize_content(&self) -> String {
        format!("{}", self.content)
    }
}

pub trait Summary {
    fn summarize_content(&self) -> String;

    fn summarize(&self) -> String {
        //default implementation, so if actual implementation is not found with this method name, then this value will be used by default
        format!("from default implementation: {}", self.summarize_content())
    }
}

//notify() will take any type that implements Summary trait
fn notify(item: &impl Summary) {
    println!("FRom fn notify: {} -----", item.summarize());
}


/*
//Using trait bounds on conditionally implemented traits
// explanation- `cmp_display()` will be available to type that only implements `Display and PartialOrd`
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

//implementing trait for another trait
//standart library example
impl<T: Display> ToString for T {
    // implements ToString on any trait that implements Display trait
    // --snip--
}
//the standard `to_string()` method defined on `ToString` trait is avaibale to all the types that implements `Display` trait
//example- let s = 3.to_string();
*/
fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {} -----", tweet.summarize());
    println!("New article available! {} -----", article.summarize());

    notify(&tweet);
    println!(
        "From returns_summarizable(): {} -----",
        returns_summarizable().summarize()
    );
}
