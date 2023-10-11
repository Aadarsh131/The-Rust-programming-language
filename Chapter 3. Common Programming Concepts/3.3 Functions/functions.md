- **Statements** are instructions that perform some action and do not return a value.

- **Expressions** evaluate to a resultant value. 
    >eg.
    > Calling a function is an expression
    >
    > Calling a macro is an expression
    >
    >A new scope block created with curly brackets is an expression
    >
    > 5+6 is an expression

```rs
fn main() {
   let x = (let y = 6); //(ERROR) let y = 6; is a statement (which by nature do not return a value), hence cannot be stored in a variable

   let z = {
   let a = 3;
    a + 1//is an expression, notice it doesnot end with ;
   }
   println!("{z}");
}
```
In C/C++ we can write `x = y = 6` and have both x and y have the `same value 6`; that is not the case in Rust.