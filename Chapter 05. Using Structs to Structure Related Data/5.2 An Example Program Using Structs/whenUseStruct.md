<u>Approach 1</u>-

```rs
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```
it’s not clear anywhere in our program that the parameters are related. It would be more readable and more manageable to group `width` and `height` together. 

<u>Approach 2 *(using tuples)*</u>- 

```rs
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

OR, <u>*using tuple struct*</u>- 
```rs
struct Rect(u32, u32);
fn main() {
    let rect1 = Rect(30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: Rect) -> u32 {
    dimensions.0 * dimensions.1
}
```
We would have to keep in mind that `width` is the tuple index `0` and `height` is the tuple index `1`. This would be even harder for someone else to figure out and keep in mind if they were to use our code.

<u>Approach 3 (using Structs)</u>-

```rs
//Struct by default move ownership.
//As most structs in Rust don't have a fixed size and can have complex internal data structures like String(which is stored in Heap), which makes a simple, bitwise copy unsafe and inefficient.  
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1) //we want the rect1's ownership to be retained
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```
This is more meaningful.  

NOTE: accessing fields of a borrowed struct instance does not move the field values.

# Adding Useful Functionality with Derived Traits

```rs
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}
```
ERROR- ``` error[E0277]: `Rectangle` doesn't implement `std::fmt::Display` ```  

The `println!` macro can do many kinds of formatting, and by default, the curly brackets tell `println!` to use formatting known as `Display`: output intended for direct end user consumption.

The primitive types we’ve seen so far implement `Display` by default because there’s only one way you’d want to show a `1` or any other primitive type to a user. But with structs, the way `println!` should format the output is less clear because there are more display possibilities: Do you want commas or not? Do you want to print the curly brackets? Should all the fields be shown? Due to this ambiguity, Rust doesn’t try to guess what we want, and structs don’t have a provided implementation of `Display` to use with `println!` and the `{}` placeholder.

<br>
The Rust error message provides some suggestion, lets try one-

```rs
println!("rect1 is {:?}", rect1);  //still an error, we will find out why shortly 
```
ERROR- ```error[E0277]: `Rectangle` doesn't implement `Debug` ```

Putting the specifier `:?` inside the curly brackets tells `println!` we want to use an output format called `Debug`. The `Debug` trait enables us to print our struct in a way that is useful for developers so we can see its value while we’re debugging our code.

Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. To do that, we add the outer attribute `#[derive(Debug)]` just before the struct definition- 
```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```
>When we have larger structs, it’s useful to have output that’s a bit easier to read; in those cases, we can use `{:#?}` instead of `{:?}` in the `println!` string.
>
> So, the output will be this-
>```rs
>rect1 is Rectangle {
>    width: 30,
>    height: 50,
>}
>```
> Instead of this-
>
>rect1 is Rectangle {
    width: 30,
    height: 50,
}

Another way to print out a value using the `Debug` format is to use the `dbg! macro`, which- 
- takes ownership of an expression **(as opposed to `println!`, which takes a reference)**
- prints the file and line number of where that `dbg! macro `all occurs in your code
- along with the resultant value of that expression, and returns ownership of the value.

>NOTE:  
> Calling the `dbg! macro` prints to the *standard error console stream (`stderr`)*
>
>Whereas, `println!`, prints to the *standard output console stream (`stdout`)*

```rs
#[derive(Debug)] //explicitly specifying the Debug trait
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), //dbg!() will return the resultant value of expression along with the ownership
        height: 50,
    };

    dbg!(&rect1); //we dont want dbg! to take the ownership, hence using the ref of rect1
}
```
