# Methods
Unlike functions, methods are defined within the context of a struct (or an enum or a trait object), and their first param is always `self`, which represents the instance of the struct the method is being called on.

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    //&self is short for `self: &Self`
    fn area(&self) -> u32 { //Methods can take ownership of self, borrow self immutably, as we’ve done here, or borrow self mutably, using- self, &self and &mut self resp.
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```
**NOTE**: Having a method that takes ownership of the instance by using just `self` as the first parameter is rare; this technique is usually used when the method transforms `self` into something else and you want to prevent the caller from using the original instance after the transformation.


```rs
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```
Calling `rect1.width` and `rect1.width()` will not confuse the compiler, it will know that one is 'field' and other is a 'method'.  

Often, but not always, when we give a method the same name as a field we want it to only return the value in the field and do nothing else. Methods like this are called **getters**, and Rust does not implement them automatically for struct fields as some other languages do. **Getters** are useful because you can make the field *private* but the method *public*, and thus enable read-only access to that field as part of the type’s public API. 

## Better alternative to `->` operator of C++
 Rust has a feature called **automatic referencing and dereferencing**
 
 eg. when calling a method, we are using this feature internally.

 When `object.something()` is called, Rust automatically adds in `&, &mut, or *` so object matches the signature of the method.
 ```rs
 //Both meaning the same
 p1.distance(&p2);
(&p1).distance(&p2);
 ```
 >This *automatic referencing* behavior works because methods have a clear receiver—the type of `self`
 >
 >The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.

 # Associated Functions
 Functions inside the `impl` block are *associated functions*.  
 Functions that doesn't have `self` as the first parameter is NOT a `method` and is used as a `constructor` that will return a new instance of the struct.  
 These can be compared to `new` of OOPS language.

 ```rs
 impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
 ```
```rs
 let sq = Rectangle::square(3);
```
Call using `::` syntax, this function is namespaced by struct.  
 The `::` syntax is used for both associated functions and namespaces created by modules.  
`String::from` is also like a construtor.

 ## Multiple impl blocks
Same Struct can have multiple impl blocks, but it is only useful when we create **Generic types and traits**.