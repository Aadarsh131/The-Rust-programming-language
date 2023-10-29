For all the collections list- check [Documentation](https://doc.rust-lang.org/std/collections/index.html)
## creating vector
```rs
    let v: Vec<i32> = Vec::new(); //cannot infer the types automatically in this case, we need to annotate the type explicitly
```
- creation using the `vec!` macro
```rs
    let v = vec![1, 2, 3]; //automatic type infer to <i32>
    v.push(4); //updating with `push`
```

## Accessing the vector element
- using indexing
- using `get()` method

```rs
let v = vec![1, 2, 3, 4, 5];
let third: &i32 = &v[2]; //indexing panics for out of range elements

println!("The third element is {third}");

let third: Option<&i32> = v.get(2); //get() returns a `None` for out of range elements

match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
```

# Borrow Checker 
When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules to ensure this reference and any other references to the contents of the vector remain valid.
```rs
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {first}");
```
Even the item is pushed at the last index, Rust still ensure the borrow checker helps us to throw an error (*It is basically for safety reasons*)  

This error is due to the way vectors work: because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.

## Iterating vectors
```rs
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
```
`NOTE:` inside the for loop scope, other mutation of original vector is not allowed and would result in error (*common borrowing rules*). The reference to the vector that the for loop holds prevents simultaneous modification of the whole vector.

## Using an Enum to Store Multiple Types
Vectors can only store values that are the same type.  
Using `Enum` we can tackle this limitation.
```rs
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```
vector `row` is of same type `SpreadsheetCell`. But, we can store different values in it using their variants.

## Limitation of storing types in Vector
Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element. If Rust allowed a vector to hold any type, there would be a chance that one or more of the types would cause errors with the operations performed on the elements of the vector.  
Example(*generated from ChatGpt*)- 
```rs
enum BadDataType {
    SelfReferential(i32, Box<BadDataType>),
    Integer(i32),
}
```
In this example, the `BadDataType` enum has a variant `SelfReferential` that includes an `i32` and a `box` pointing to another `BadDataType`. This creates a situation where the data refers to itself, forming a recursive structure. This self-referential data cannot be directly stored in a Rust vector because vectors require that their elements have a fixed size, and the size of a `SelfReferential` value can vary based on the depth of recursion. This would violate Rust's memory safety guarantees.

Using an `enum` plus a `match` expression means that Rust will ensure at compile time that every possible case is handled. If you don’t know the exhaustive set of types a program will get at runtime to store in a vector, the enum technique won’t work. Instead, you can use a `trait` object.

## Vector methods
```rs
let mut vec = Vec::new();
vec.push(1);
vec.push(2);

assert_eq!(vec.len(), 2);
assert_eq!(vec[0], 1);

assert_eq!(vec.pop(), Some(2));
assert_eq!(vec.len(), 1);

vec[0] = 7;
assert_eq!(vec[0], 7);

vec.extend([1, 2, 3]);

for x in &vec {
    println!("{x}");
}
assert_eq!(vec, [7, 1, 2, 3]);
```
More on [Api documentation](https://doc.rust-lang.org/std/vec/struct.Vec.html) for vector methods.

## Dropping a Vector Drops Its Elements
```rs
{
    let v = vec![1, 2, 3, 4];
    // do stuff with v
} // <- v goes out of scope and is freed here
```
When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up.