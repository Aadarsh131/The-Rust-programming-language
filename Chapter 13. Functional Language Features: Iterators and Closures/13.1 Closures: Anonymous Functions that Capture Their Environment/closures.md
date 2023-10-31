# Closures
- Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.
- You can create the closure in one place (as a variable) and then call the closure elsewhere to evaluate it in a different context.
- Unlike functions, closures can capture values their environment.

```rs
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked()) //FnOnce() trait closure
        //The closure captures an immutable reference to the self Inventory instance. Functions, on the other hand, are not able to capture their environment in this way.
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
```

## Closure type inference
Functions require type annotations because types are part of an explicit interface exposed to your users. Defining this interface rigidly is important for ensuring that everyone agrees on what types of values a function uses and returns.

Closures aren’t used in an exposed interface like this, they are stored in a variable.  

Closures are short and relevant only within a narrow context rather than in any arbitrary scenario.The compiler can infer the types of the parameters and the return type, similar to how it’s able to infer the types of most variables (there are rare cases where the compiler needs closure type annotations too).

```rs
let expensive_closure = |num: u32| -> u32 { //it is possible to explicitly annotate the type though, but its more verbose and unnecessary
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

#### Syntax
```rs
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

## How compiler implicit do type inference for closure
```rs
let example_closure = |x| x;
let s = example_closure(String::from("hello"));//this line will infer the type and store the type as the closure's concrete type, and so later the types cannot be changed
let n = example_closure(5); // ERROR: because the concrete type has been set to "String" already
```
## Capturing reference VS Ownership
Closure captures values from environment as same as three ways a function can take a parameter: `borrowing immutably, borrowing mutably, and taking ownership`.  
- Immutable reference-
    ```rs
    fn main() {
        let list = vec![1, 2, 3];
        let only_borrows = || println!("From closure: {:?}", list);
        only_borrows();
    }
    ```
- Mutable reference- 
    ```rs
    fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
    }
    ```
- Passing Ownership-
    ```rs
    use std::thread;

    fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    }
    ```
    using `move` keyword we can pass ownership. Mostly used in threads.

# `Fn` traits
1. `FnOnce` applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
2. `FnMut` applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
3. `Fn` applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.

```rs
//Standard library definition-
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

The trait bound specified on the generic type `F` is `FnOnce() -> T`, which means `F` must be able to be called once, take no arguments, and return a `T`. Using `FnOnce` in the trait bound expresses the constraint that `unwrap_or_else` is only going to call `f` at most one time.

>Note: Functions can implement all three of the `Fn` traits too. If what we want to do doesn’t require capturing a value from the environment, we can use the name of a function rather than a closure where we need something that implements one of the `Fn` traits. For example, on an `Option<Vec<T>>` value, we could call `unwrap_or_else(Vec::new)` to get a new, empty vector if the value is None.

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        sort_operations.push(value); //this statement is not allowed as we are moving the ownership of `value`, so in the second call when compiler would try to search for `value` in its environment, it would fail as the ownership was moved. 
        //`&value` as argument would work though

        num_sort_operations += 1; //can be called more than once, there is no onwership moved like previous

        r.width
    });
    println!("{:#?}", list);
}
```
`sort_by_key` implements `FnMut` isntead of `FnOnce`. The reason sort_by_key is defined to take an FnMut closure is that it calls the closure multiple times: *once for each item in the slice*.
