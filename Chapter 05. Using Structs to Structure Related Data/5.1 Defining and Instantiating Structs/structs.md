## Structs
1. ### Defining Struct
    Struct name specifies what data we are dealing with, it should be meaningful to the stored data(fields)
    ```rs
    struct User {
        //fields
        active: bool, 
        username: String,
        email: String,
        sign_in_count: u64,
    }
    ```

2. ### Instantiating Struct
    ```rs
    fn main() {
        let user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };
    }
    ```
    *`user1` is an new instance of `User` struct type*

## Accessing and mutating Structs
using the dot notation
```rs
user1.email = String::from("anotheremail@example.com"); //only possible if `user1` is mut type
```

<br>
Notice the function name is also meaningful w.r.t the return data, i.e, it builds the user

```rs
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,//'Field init Shorthand' for `username:username`
        email, //'Field init Shorthand' for `email:email`
        sign_in_count: 1,
    }
}
```

## Creating Instances from Other Instances with Struct Update Syntax
<br>
Not so convenient way-

```rs
fn main() {
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

<br>
Using Update Syntax (convenient way)-

```rs
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```
The `..user1` must come last to specify that any remaining fields should get their values from the corresponding fields in `user1`

## Ownership of Struct Data
>Note that the struct update syntax uses `=` like an assignment; this is because it moves the data. So, in the above example, user1's `username` value which is a `String` type, is moved to `user2` rather than copied, hence the `user1` as a whole is no longer valid.  
>If suppose we had given `user2` new String values for both `email` and `username`, and thus only used the `active` and `sign_in_count` values from `user1`, then `user1` would still be valid after creating `user2`.  
> Both `active` and `sign_in_count` are types that implement the `Copy` trait.

We used the owned `String` type rather than the `&str` string slice type. This is a deliberate choice because we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.

To store references to data owned by something else, we would need the use of *lifetime*.
Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.

```rs
struct User {
    active: bool,
    username: &str, //ERROR: expected named lifetime parameter
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
```
Without the use of *lifetimes*, we would get compile time error

## Tuple Structs
It doesn't have names associated with their fields, rather just the types of the fields.   
Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples.
```rs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```
We can destructure them into their individual pieces, and use a '`.`' followed by the index to access an individual value.  
NOTE: `black` and `origin` are completely different types, they are not related to their made up types of `i32` in any way.

## Unit-Like Structs Without Any Fields
They behave similarly to (), the unit type.  
Can be useful when we need to implement a *trait* on some type but don’t want any data to be stored.
```rs
struct AlwaysEqual; //(), unit type struct

fn main() {
    let subject = AlwaysEqual;
}
```
 Imagine that later we’ll implement behavior for this type such that every instance of `AlwaysEqual` is always equal to every instance of any other type, perhaps to have a known result for testing purposes. We wouldn’t need any data to implement that behavior! 