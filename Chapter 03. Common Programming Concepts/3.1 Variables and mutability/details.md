***Mutability-*** By default, all the variables are `immutable`

```rs
let x = 5; //immutable
let mut y = 5; //mutable
```
### Difference between `const and immutable variable`
```rs
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //Rust’s naming convention for constants is to use all uppercase with underscores between words
```
- the `const` variable cannot be made `mut`
- constants may be set only to a constant expression, not the result of a value that could only be computed at runtime

## Shadowing
We can declare a new variable with the same name as a previous variable. Rustaceans say that the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable.

Shadowing using `let` (redeclaration of same variable NAME)
```rs
    let spaces = "   ";
    let spaces = spaces.len();
```
- notice the type has also changed (from string to int), it is obvious as we are creating a new variable type using `let`

```rs
    let mut spaces = "   ";
    spaces = spaces.len(); //error, cannot change the type 
```

