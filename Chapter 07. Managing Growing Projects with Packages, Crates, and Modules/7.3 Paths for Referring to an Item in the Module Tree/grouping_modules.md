# Grouping Related Code in Modules
Modules also allow us to control the privacy of items, because code within a module is **private by default**.

Create a new library named `restaurant` by running `cargo new restaurant --lib;`

<br>
Filename: src/lib.rs

```rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```
Module tree- 
 - implicitly created `crate` module is the content of *src/lib.rs* or *src/main.rs*
<pre>
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
</pre>

```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

//eat_at_restaurant function is part of our library crate’s public API, so we mark it with the pub keyword
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path (A relative path starts from the current module and uses 'self', 'super', or an identifier in the current module)
    //Starting with a module name means that the path is relative.
    front_of_house::hosting::add_to_waitlist();
}
```
We can access `pub mod hosting` because the parent module of `hosting` i.e, `front_of_house` is directly accessible to  `eat_at_restaurant` (as, `eat_at_restaurant` and `front_of_house` are siblings).  

We can access `pub fn add_to_waitlist()` because, its parent module `hosting` is public.

There are many considerations around managing changes to your public API to make it easier for people to depend on your crate. Checkout [The Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) for more.

> ## Best Practices for Packages with a Binary and a Library
>We mentioned a package can contain both a src/main.rs binary crate root as well as a src/lib.rs library crate root, and both crates will have the package name by default. Typically, packages with this pattern of containing both a library and a binary crate will have just enough code in the binary crate to start an executable that calls code with the library crate. This lets other projects benefit from the most functionality that the package provides, because the library crate’s code can be shared.
>
>The module tree should be defined in src/lib.rs. Then, any public items can be used in the binary crate by starting paths with the name of the package. The binary crate becomes a user of the library crate just like a completely external crate would use the library crate: it can only use the public API. This helps you design a good API; not only are you the author, you’re also a client!

## `Super` in Relative Paths
accessing the immediate parent module
```rs
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); //checks for its parent module
    }

    fn cook_order() {}
}
```

# Public Structs
Making the *structs* `pub` doesn't make its fields public, we need to explictly make its fields `pub`
```rs
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

# Public Enums
Making the `enum` as `pub` will make its variants also as `pub`, because at the end, they are just variants of the same type.  
Enums aren’t very useful unless their variants are public
```rs
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```