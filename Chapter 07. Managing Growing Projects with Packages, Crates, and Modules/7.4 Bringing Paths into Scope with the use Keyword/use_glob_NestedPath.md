We bring names to the scope using `use` keyoword.  
Adding `use` and a path in a scope is similar to creating a **symbolic link** in the filesystem.
```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; //brought the name into scope of crate root/restraunt module

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
**Note**: that `use` only creates the shortcut for the particular scope in which the `use` occurs.  
For example- the below code is an error, because, `use` scope is out of the `mod customer` scope
```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
//to solve the problem, either move the `use` within the `customer` OR do super::hosting::add_to_waitlist();
```

## An Idiomatic way 
1. Bringing the parent scope-
    ```rs
    use std::fmt;
    use std::io;

    fn function1() -> fmt::Result {
        // --snip--
    }

    fn function2() -> io::Result<()> {
        // --snip--
    }
    ```

    If we try `use std::fmt::Result` and `use std::io::Result`, we’d have two `Result` types in the same scope and Rust wouldn’t know which one we meant when we used `Result`. There's a solution for it though, we can make *alias* names using the `as` keyword.

2. Making *alias* names-
    ### `as` Keyword

    ```rs
    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function1() -> Result {
        // --snip--
    }

    fn function2() -> IoResult<()> {
        // --snip--
    }
    ```
Both ways are considered **Idiomatic way**, choice is upto us.

## Re-exporting Names with `pub use`

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
`pub use` has re-exported `hosting` from `restraunt`/`crate root` module, i.e, from where the `pub use` is called.  
So, instead of the path `restaurant::front_of_house::hosting::add_to_waitlist()`, we can now simply have `restaurant::hosting::add_to_waitlist()`

Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain.

## Using External Packages
We need to add an external pacakge name as a dependency in *Cargo.toml*, it tell the Cargo to the download that package and any related dependency from [crates.io](https://crates.io/) and make that pacakge available to our project.

Eg. `rand` is an external pacakge

```rs
use rand::Rng;

fn main() {
let secret_number = rand::thread_rng().gen_range(1..=100);
}
```
>**Note** The standard `std` library is also a crate that’s external to our package. Because the standard library is shipped with the Rust language, we don’t need to change *Cargo.toml* to include `std`.

## Nested Paths
Combnined two `use` statement, that was sharing the subpath
```rs
use std::cmp::Ordering;
use std::io;
//OR
use std::{cmp::Ordering, io};
```
```rs
use std::io;
use std::io::Write;
//OR
use std::io::{self, Write}; //notice self
```

## Glob operator
`*` will bring all the items defined in the path
```rs
use std::collections::*;
```

