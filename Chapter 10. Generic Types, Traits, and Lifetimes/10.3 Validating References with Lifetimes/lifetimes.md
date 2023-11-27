# Lifetime
Every reference in Rust has a lifetime (owned types dont have lifetimes), which is the scope for which that reference is valid.  
Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred.

- Lifetimes prevent dangling references

## The Borrow Checker
The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+

// The program is rejected because 'b is shorter than 'a: the subject of the reference doesn’t live as long as the reference
```
```rust
//This works fine
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```

Lifetime annotations don’t change how long any of the references live. Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

```rust
fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    //meaning: lifetime returned is the smaller of the lifetimes of both the params passed in
    //NOTE: the lifetimes relationship is only designed at the start of the function, not within the body, it speeds up the execution
    if first.len() > second.len() {
        return &first;
    }
    &second
}

fn main() {
    let a = String::from("Aadarsh");
    let result;
    {
        let b = String::from("Atul");
        result = longest(&a, &b); //the smaller lifetime is of `b`, so the `result` will have the lifetime of `b`
    } 
    //b is dropped here

    print!("{}", result); //`result` (which has the lifetime of `b`), is not valid upto here, even though the `result` will get the value of `&a` and has nothing to do with `&b`(because, the compiler wont check for the function implementation only the declaration), hence, borrow check will reject this program
}
```
If we dont want to return the value of `y`, then we dont need to specify the lifetime on that
```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```
When the lifetime of return type does not refer to any of the parameter's lifetime in a function, then it must refer to a value created within this function, like in this example
```rust
fn longest<'a>(x: &str, y: &str) -> &'a str { // 'a refers to &result, which is a dangling ref btw
    let result = String::from("really long string");
    result.as_str()
}
//result is dropped here
```
**Q**. Why is `s.clear()` an error in the below example
```rust
fn main() {
let mut s = String::from("helloworld");

let word = first_word(&s);
s.clear(); // why is this error
    println!("the first word is: {}", word);
}
fn first_word(_s: &String) -> &str {
    // not even using the ref of s
    let a = "Aadarsh";
    &a[..] 
}
```
**Reason**- the *first_word()* function definition would get replaced by  
```rust
fn first_word<'a>(_s:&'a String) -> &'a str{ //because of Lifetime Elision's 1st and 2nd rule together 
```
**Step 1-** the compiler would apply the 1st lifetime elision rule, which specifies that each parameter gets its own lifetime
```rust
fn first_word<'a>(_s: &'a String) -> &str {
```
**Step 2-** The second rule applies because there is exactly one input lifetime. The second rule specifies that the lifetime of the one input parameter gets assigned to the output lifetime, so the signature is now this:  
```rust
fn first_word<'a>(_s: &'a String) -> &'a str {
```
The reference of String `s` would be valid/live when  the `word` gets its data from the `first_word()`'s return, because the lifetime is still active. And on the very next line we are creating a `&mut`(as `clear()` is taking `&mut self`) which is not allowed at the same time with another reference still active.  

**NOTE**: in the process the actual value that `word` is storing is `&a`(which was created within the function), lifetime elision has just increased the lifetime of `s` also out of the function (which needs to be handled carefully now)

## Lifetime Annotations in Struct Definitions
```rust
struct ImportantExcerpt<'a> {
    part: &'a str, //This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    //novel doesn’t go out of scope until after the ImportantExcerpt goes out of scope, so the reference in the ImportantExcerpt instance is valid.
}
```

## Lifetime Elision
In early versions (pre-1.0) of Rust, every reference needed an explicit liftime annotation.  

The patterns programmed into Rust’s analysis of references are called the ***lifetime elision rules***. These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler will consider

- lifetimes on function or method parameters are called **input lifetimes**
- lifetimes on return values are called **output lifetimes**

> ### 3 rules of lifetime elision (implicit lifetime inference)
> The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes. If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error.  
> These rules apply to `fn` definitions as well as `impl` blocks.  
> - **First rule**- The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter:  
>   ```rust
>   fn foo<'a>(x: &'a i32);
>   ```
>   a function with two parameters gets two separate lifetime parameters:
>   ```rust
>   fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
>   ```
>   and so on.
>
> - **Second rule**- The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: 
>   ```rust
>   fn foo<'a>(x: &'a i32) -> &'a i32.
>   ```
> - **Third rule**- The third rule is that, if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

**Q.** Consider yourself a compiler and apply the lifetime rules for the below fn signature
```rust
fn longest(x: &str, y: &str) -> &str {
```
**Solution**-   
 Because of 1st lifetime elision rule
```rust
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```
But we cannot apply 2nd or 3rd rule either to the know the output lifetime, 2nd rule doesn't apply becuase we have more than 1 input lifetime param

Hence, we will get a **compiler error**

## Lifetime Annotations in Method Definitions





