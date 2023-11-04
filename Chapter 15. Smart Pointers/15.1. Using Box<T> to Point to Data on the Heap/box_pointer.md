# Smart pointers
- **Pointers:**
The most common kind of pointer in Rust is a reference, which is not *smart*. They don’t have any special capabilities other than referring to data, and have no overhead.

Rust, with its concept of ownership and borrowing, has an additional difference between references and smart pointers: while references only borrow data, in many cases, smart pointers own the data they point to.

`String` and `Vec<T>`- Both these types count as smart pointers because they own some memory and allow you to manipulate it. They also have metadata and extra capabilities or guarantees. `String`, for example, stores its capacity as metadata and has the extra ability to ensure its data will always be valid UTF-8.

Smart pointers implement the `Deref` and `Drop` traits. The `Deref` trait allows an instance of the smart pointer struct to behave like a reference so you can write your code to work with either references or smart pointers. The `Drop` trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope. 
# When to use Box<T>
The most straightforward smart pointer is a box.Also, they don’t have many extra capabilities either.
- When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
- When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
- When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type. This case is known as *trait object*, discussed in Chapter 17

## Cons List
A cons list is a data structure that comes from the Lisp programming language and its dialects and is made up of nested pairs, and is the Lisp version of a linked list.

The canonical name to denote the base case of the recursion is `Nil`. Note that this is not the same as the “null” or “nil” which is an invalid or absent value.

```rs
enum List {
    Cons(i32, List), //ERROR: recursive without indirection (infinite stack size allocation)
    //indirection- storing the value indirectly using pointers, unlike direct value storing as we are doing here
    Nil,
}
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```
Instead of storing the value, just store the reference/pointer to the value.
```rs
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```
The `Box<T>` type is a smart pointer because it implements the `Deref` trait, which allows `Box<T>` values to be treated like references. When a `Box<T>` value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the `Drop` trait implementation.