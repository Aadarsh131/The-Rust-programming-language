# Generic Data Types
### In function Definitions
```rust
//We read this definition as: the function largest is generic over some type T

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest { //Error: binary operation `>` cannot be applied to type `&T`
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```
We can implement a trait on type `T` that implements both `i32` and `char`, or we can use a standard library that implements it, the suggestion says to use `std::cmp::PartialOrd`
```rust
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T { //works fine, because the standard libray implements `PartialOrd` on both `i32` and `char`
//we have now restricted the valid types for `T` to only those that implements `PartialOrd`
```

### In Struct Definitions
```rust
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Dot<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    //Now, all the instances are allowed
    let a = Dot { x: 4, y: 5 };
    let b = Dot { x: 4.0, y: 5.6 };
    let c = Dot { x: 3.7, y: 5 };
    let d = Dot { x: 3, y: 5.6 };

    dbg!(a, b, c, d);
}
```
### In Enum Definitions
```rust
enum Option<T> {
    Some(T),
    None,
}
```
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
### In Method Definitions
```rust
struct Point<T> {
    x: T,
    y: T,
}

//Note: we have to declare T just after impl so we can use T to specify that we’re implementing methods on the type Point<T>. By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```
Concrete type in methods: 
```rust
impl Point<i32> { //no need to declare the concrete type after `impl`
    fn x(&self) -> &i32 {
        &self.x
    }
}
```
Mixedup types:
```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

//we can use any type name, even though original generic type name of Point was <X1, Y1>
impl<A1, A2> Point<A1, A2> {
    //method mixup has seperate typename
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<A1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

### Code performance with Generics
There is no runtime cost of using generics compared to concrete types, because of `Monomorphization` at the compile time.  
`Monomorphization` is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.