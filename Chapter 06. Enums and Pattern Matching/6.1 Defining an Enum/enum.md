# Enumerations(Enums)
Enums allow you to **define a type** by enumerating its possible variants.  
Enums give you a way of saying a value is one of a possible set of values


**Example-**  
 Any IP address can be either a version four or a version six address, but not both at the same time. That property of IP addresses makes the enum data structure appropriate because an enum value can only be one of its variants.  Both version four and version six addresses are still fundamentally IP addresses, so they should be treated as the same type when the code is handling situations that apply to any kind of IP address.
```rs
enum IpAddrKind { //custom type IpAddrKind 
    V4, //of V4
    V6, //and V6 variants
}
```
<br>
Creating Instance-

```rs
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
```
<br>
Function Parameter-

```rs
fn route(ip_kind: IpAddrKind) {}

//calling with either variant
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

## Storing the data in enums
We have two ways-

1. using the struct and enums **(NOT recommended)**-
    ```rs
    fn main() {
        enum IpAddrKind {
            V4,
            V6,
        }

        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }

        //home and loopback is of type `struct` (not so intuitive way)
        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
    }
    ```
    OR
     ```rs
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct
    ```
    **Diadvantage**:
    - we would need to define the functions(methods) explicitly for each structs
    - not so good structure

2. Storing the data directly inside the enum **(Recommended)**
    ```rs
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    ```
    

    OR
    ```rs
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    ```
    **Advantages:**  
    - We can have any kind of data inside an enum variant: `strings, numeric types,or structs`, even another `enum`!
    - Easy to structure
    - Defining a function(method) on one enum is easier than defining it for each structs(which is duplicating code)``
    
    
## Methods on enums
```rs
impl Message {
    //&self will be ref to instance `m` i.e, `Message::Write(String::from("hello"))`
    fn call(&self) { 
        
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```
Here, it’s also easier to see another detail of how enums work: the name of each enum variant that we define also becomes a function that constructs an instance of the enum.   
i.e, `Message::Write()` is a function call that takes a String argument and returns an instance of the `Message` type. *We automatically get this constructor function defined as a result of defining the enum*.

# Null values in Rust
Rust doesn’t have null values, like in other programming langauges like in C/C++, which is the biggest cause of bugs. 

In his 2009 presentation “Null References: The Billion Dollar Mistake,” Tony Hoare, the inventor of null, has this to say:
> *I call it my billion-dollar mistake. At that time, I was designing the first comprehensive type system for references in an object-oriented language. My goal was to ensure that all use of references should be absolutely safe, with checking performed automatically by the compiler. But I couldn’t resist the temptation to put in a null reference, simply because it was so easy to implement. This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years.*

The problem isn’t really with the concept but with the particular implementation. Rust solves this problem with `Option` enum, a simply and beautiful concept.


`Option` enum from the Standard Library, even included in the prelude, meaning we don’t need to bring it into scope explicitly, we can use `Some` and `None` directly without the `Option::` prefix.
```rs
enum Option<T> {
    None,
    Some(T),
}
```

```rs
//infering types from the argument
let some_number = Some(5); 
let some_char = Some('e');

//Cannot infer types in this case, hence, have to specify it explicitly
let absent_number: Option<i32> = None; 
```

- with `Some`, it is clear that data is **NOT NULL**
- with `None`, it is clear that data in some sense is **NULL(Absent)**

### Advantage of `Option<T>` over other langauge's `null`- 
```rs
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
```
compiler takes `Option<T>` and `i8` as different types, hence the error.

- When we have type other than `Option<T>`, we can proceed confidently without having to check for null values
- Only, when we have type `Option<T>` we would have to worry about `null` value

And the compiler will make sure we handle that case before using the value, meaning, we have to convert an `Option<T> `to a `T` before we can perform `T` operations with it.  
 This helps catch one of the most common issues with `null`: assuming that something isn’t null when it actually is.  
 >This was a deliberate design decision for Rust to limit null’s pervasiveness and increase the safety of Rust code.

### Getting the `T` value out of `Option<T>` - 
Check its methods in the [documentation](https://doc.rust-lang.org/std/option/enum.Option.html)

We must handle both case when handling `Option<T>` - 

Using the `match` expression-  
**Case 1-** When we have `Some(T)` value, the code is allowed to use the inner `T`.  
**Case 2-** When we have `None` value, that will not have `T` value available.