Both `String` and `string slices` are `UTF-8` encoded

## String
`String` is actually implemented as a wrapper around a `vector of bytes` with some extra guarantees, restrictions, and capabilities.  
Hence, many of the same operations available with `Vec<T>` are available with `String` as well.  
Example-  
```rs    
let mut s = String::new(); //same as Vec::new();
```

> `to_string` method, is available on any type that implements the `Display` trait.

```rs
let data = "initial contents";
let s = data.to_string(); //Or, let s = String::from("initial contents");

//We can use either way to convert to `String`
```

# Updating `String`

using the `+` operator or the `format!` macro

- #### Appending `str` to String using `push_str`
    ```rs
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");//valid as we s2 is just a ref (&s2)
    ```
- #### Appending `char` to String using `push`
    ```rs
    let mut s = String::from("lo");
    s.push('l');
    ```
1. ### `+` operator
    Method for `+` operator
    ```rs 
    fn add(self, s: &str) -> String {
    ```
    The internal implementation of `add` is such that we can only pass-
    - 1st param as variable without ref (*hence, ownership is moved*)
    - 2nd param as a ref to `string slice` or `String`
    
    ```rs
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    ```
    ## *Deref coercion*
    The reason we’re able to use `&s2` in the call to `add` is that the compiler can *coerce* the `&String` argument into a `&str`. Which here turns `&s2` into `&s2[..]`
2. ### `fomat!` macro
    ```rs
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); //doesn't take ownership of any of the argument, so value of s1 is retained using format! 
    ```
# Indexing into Strings
```rs
let s1 = String::from("hello");
let h = s1[0]; //Error, String cannot be indexed
```

>/////////////////////////
>
> SKIPPED THE REASONING IT IS A BIT CONFUSING
>
> COVER THIS LATER !!
>
>/////////////////////////