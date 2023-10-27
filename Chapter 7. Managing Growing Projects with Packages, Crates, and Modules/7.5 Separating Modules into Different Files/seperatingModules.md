## Modules in single file
```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
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

## Seperated Modules
<br>
Filename: src/lib.rs

```rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
<br>
Filename: src/front_of_house.rs

```rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```
Further Seperating the `hosting` module-

<br>
Filename: src/front_of_house.rs

```rs
pub mod hosting;
```
<br>
Filename: src/front_of_house/hosting.rs

```rs
pub fn add_to_waitlist() {}
```

## Older Style of Seperating Module
- `src/front_of_house.rs` (what we covered)
- `src/front_of_house/mod.rs` (older style, still supported path)

If you use both styles for the same module, you’ll get a compiler error.  
 Using a mix of both styles for different modules in the same project is allowed, but might be confusing for people navigating your project.
