## Takeaways from this section, which was different from the usual way of C/C++

```rust
fn main() {
    let number = 3;

    //if else expressions only evaluates for a `bool` value
    if number {
        println!("number was three");
    }
}
```
> Error- "expected `bool`, found integer"
<hr>

> Intead of using too many `else if` expressions we can use `match`, discussed in Chapter 6.

##  Using if in a let Statement
Because `if` is an expression, we can use it on the right side of a `let statement` to assign the outcome to a variable
```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 }; 
    
    //error, return value from if and else code blocks must be of same type
    let number2 = if condition {5} else {"six"}; 

    println!("The value of number is: {number}");
}
```

> **SIDE NOTE:**
> The expression in the if block evaluates to an integer, and the expression in the else block evaluates to a string. This won’t work because variables must have a single type, and Rust needs to know at compile time what type the number variable is, definitively. Knowing the type of number lets the compiler verify the type is valid everywhere we use number. Rust wouldn’t be able to do that if the type of number was only determined at runtime; the compiler would be more complex and would make fewer guarantees about the code if it had to keep track of multiple hypothetical types for any variable.


## Returing values from loop
 Rust has three kinds of loops: `loop`, `while`, and `for`
```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; //break will stop the loop, anything after the `break` will be returned out of the loop 
        }
    };

    println!("The result is {result}");
}
```
<br>

Loops distinguished with labels
```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```
<br>

**<u>While loop-</u>**  
While iterating over some iterable while loops can be `Error prone`, code would panic if suppose `while index < 6`. t’s also slow, because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration through the loop.  
**Solution:**  use `for` loop
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```
**<u>For Loop- </u>**  
The safety and conciseness of for loops make them the most commonly used loop construct in Rust.  
Even in situations in which you want to run some code a certain number of times, for loop is a go to solution

- Range function
    ```rust
    fn main() {
        for number in (1..4).rev() { //(1..4) will range from [1,4) and rev() is a  method to reverse the range
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }
    ```