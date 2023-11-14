- **Statements** are instructions that perform some action and do not return a value.

- **Expressions** evaluate to a resultant value. 
    >eg.  
    > Calling a function is an expression
    >
    > Calling a macro is an expression
    >
    >A new scope block created with curly brackets is an expression
    >
    > `5+6` is an expression
    >
    > `if else` expression

```rs
fn main() {
   let x = (let y = 6); //(ERROR) let y = 6; is a statement (which by nature do not return a value), hence cannot be stored in a variable

   let z = {
   let a = 3;
    a + 1//is an expression(since it is within the {}), notice it doesnot end with ;
   }
   //Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value
   println!("{z}");
}
```
In C/C++ we can write `x = y = 6` and have both x and y have the `same value 6`; that is not the case in Rust.

## Function with return values
You can return early from a function by using the `return` keyword and specifying a value, but most functions return the last expression implicitly

- example 1-
    ```rust 
    fn five() -> i32 {
        5 //without semicolon, beacause thats how the values are returned in rust
        //a semicolon would give an error, because then we will not returning any value from this function
        //The error would say, "expected `i32` , found `()`". NOTE: `()` is a Unit Type, which means the return doenot evaluates to any value
    }

    fn main() {
        let x = five();

        println!("The value of x is: {x}");
    }
    ```
    There are no function calls, macros, or even let statements in the five function—just the number 5 by itself. That’s a perfectly valid function in Rust