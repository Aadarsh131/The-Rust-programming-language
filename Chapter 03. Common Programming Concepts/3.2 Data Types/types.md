# Scalar Types
scalar types represents one value. Rust has 4 primary scalar types:

1. Integers
2. Floating-point numbers
3. Booleans
4. Characters

### Integers
| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

- Signed numbers are stored using two’s complement representationsigned
- Each `signed` variant can store numbers from `-(2 ^(n - 1)) to 2^(n - 1) - 1` inclusive, where n is the number of bits that variant uses. So an i8 can store numbers from -(2^7) to 2^7 - 1, which equals -128 to 127. `Unsigned` variants can store numbers from `0 to 2^n - 1`, so a u8 can store numbers from 0 to 2^8 - 1, which equals 0 to 255.

### Floating-Point types
Rust’s floating-point types are `f32 and f64`.
The `default type is f64` because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision.

- All floating-point types are signed
- The f32 type is a single-precision float, and f64 has double precision

### Characters
Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII

Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust. Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.

# Compound Types
Compound types can group multiple values into one type. Rust has two primitive compound types: 

1. tuples 
2. arrays
   
### Tuples
A tuple is a general way of grouping together a number of values with a variety of types into one compound type
- Tuples have a fixed length: once declared, they cannot grow or shrink in size.
```rs
fn main() {
let tup: (i32, f64, u8) = (500, 6.4, 1); //type annotations are optional
let (x,y,z) = tup; //destructuring
println!("The value of x,y,z are: {x}, {y}, {z}");

//accessing tuple values with . notation
let a = tup.0;
let b = tup.1;
let c = tup.2;
println!("The value of x,y,z (accessing using . notation) are: {a}, {b}, {c}");
}
```

### Arrays
Every element of an array must have the same type and are of fixed length.

- Arrays are useful when you want your data allocated on the stack rather than the heap
```rs
let a: [i32; 5] = [1, 2, 3, 4, 5];
let b = [3;5]; // let a = [3, 3, 3, 3, 3];

let first = a[0];
let second = a[1];
```
