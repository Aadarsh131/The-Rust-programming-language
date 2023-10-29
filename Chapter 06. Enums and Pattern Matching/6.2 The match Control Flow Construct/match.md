# Patterns That Bind to Values
```rs
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alabama)); //State quarter from Alabama!
    println!("{}",value_in_cents(Coin::Dime)); //10
}
```
When `Coin::Quarter(UsState::Alabama))` is passed as the argument, `match` will try to match the pattern `Coin::Quarter`, when it matches, the `state` variable will get bind to `UsState::Alabama`, i.e, `state = UsState::Alabama`
# Matching with `Option<T>`
```rs
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, //Option::None => None
        Some(i) => Some(i + 1),
    }
}
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

dbg!(six);
dbg!(none);
```

## Matches are Exhaustive
Meaning, every possibility must be checked.
```rs
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), //all possibilities are checked 
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
```
Ignoring the variable name with **_** placeholder:
```rs
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), 
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
```
Returning nothing:
```rs
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // unit type
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
```
