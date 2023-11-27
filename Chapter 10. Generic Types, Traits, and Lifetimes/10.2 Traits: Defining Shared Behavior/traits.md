# Traits
A trait defines functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way.  

>NOTE: Traits are similar to a feature often called interfaces in other languages, although with some differences.

```rust
pub trait Summary { // Inside the curly brackets, we declare the method signatures that describe the behaviors of the types that implement this trait
    fn summarize(&self) -> String;
}
//The compiler will enforce that any type that has the Summary trait will have the method summarize defined with this signature exactly.
```
```rust
//need to also bring into scope the trait `Summary` 
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
```

## Trait Bounds
```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {

//multiple trait bounds
pub fn notify(item: &(impl Summary + Display)) {
```
can be reduced to (using trait bounds)-
```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {

//multple trait bounds
pub fn notify<T: Summary + Display>(item: &T) {
```
- using `where` clause
    ```rust
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    ```
    ```rust
    fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
    ```
```rust
//trait as return type
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("Aadarsh"),
        content: String::from("Some great content"),
        reply: false,
        retweet: true,
    }
//NOTE: we cannot return types here in if or else condition, not allowd by compiler directly, but there's a way to tackle, google for it (suggestion from compiler: using Box)
    //example below conditional returns are not allowed-
    /*
    fn returns_summarizable(switch: bool) -> impl Summary {
        if switch {
            NewsArticle {
                headline: String::from("Penguins win the Stanley Cup Championship!"),
                author: String::from("Iceburgh"),
                content: String::from(
                    "The Pittsburgh Penguins once again are the best \
                     hockey team in the NHL.",
                ),
            }
        } else {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            }
        }
    }
    */
}
```
- `blanket implementation`  
    Implementing a trait on another trait.  
    Implementations of a trait on any type that satisfies the trait bounds are called *blanket implementations* and are extensively used in the Rust standard library
    ```rust
    //Standard library implements `ToString` trait on any type that implements `Display` trait
    impl<T: Display> ToString for T {
    // --snip--
    }
    ```
    Because the standard library has this blanket implementation, we can call the to_string method defined by the ToString trait on any type that implements the Display trait. For example, we can turn integers into their corresponding String values like this because integers implement Display:
    ```rust
    let s = 3.to_string();
    ```