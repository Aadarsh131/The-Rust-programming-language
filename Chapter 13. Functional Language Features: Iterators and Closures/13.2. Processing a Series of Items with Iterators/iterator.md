# Iterators
In languages that don’t have iterators provided by their standard libraries, you would likely write this same functionality by starting a variable at index 0, using that variable to index into the vector to get a value, and incrementing the variable value in a loop until it reached the total number of items in the vector.
```rs
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}
```

# `Iterator` trait
All iterators implement a trait named `Iterator` that is defined in the standard library.
```rs
pub trait Iterator {
    type Item; //`associated type`, discussed in chapter 19

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```
Each `next()` on iterator will move on to the next element
```rs
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

>**NOTE:**   
Values we get from the calls to `next` are immutable references, as we have used `iter()` method.  
>`iter_mut()` will return the mutable ref.  
>`into_iter()` will return the owned values

## (Consuming adapters) methods that consume the Iterator
In `Iterator` trait. It is required to implement the `next()` method because some methods call the `next()` method in their definition.  
Methods that call `next()` are called **consuming adaptors**, because calling them uses up the iterator.   One example is the `sum()` method, which takes ownership of the iterator and iterates through the items by repeatedly calling next, thus consuming the iterator. As it iterates through, it adds each item to a running total and returns the total when iteration is complete. 
```rs
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum(); // `sum()` takes the ownership of `v1_iter`

    //`v1_iter` is invalid here
    assert_eq!(total, 6);
}
```
>**NOTE:** We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on.

## (Iterator Adaptors) Methods that Produce other Iterators
**Iterator adaptors** are methods defined on the `Iterator` trait that don’t consume the iterator.       Instead, they produce different iterators by changing some aspect of the original iterator.

Example- 
```rs
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1); //map() is an Iterator adaptor, it will create a new iterator
```

BUT, ALL `Iterators` are lazy.   
Hence, we would need to consume it in some way.

```rs
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); //consuming with collect() and storing values in `v2`.

    assert_eq!(v2, vec![2, 3, 4]);
```
We can chain multiple calls to iterator adaptors to perform complex actions in a readable way. But because all iterators are lazy, we have to call one of the consuming adaptor methods to get results from calls to iterator adaptors.

```rs
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    //shoe_size is captured from its environment
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    //into_iter() will take the ownership of `shoes`
    //filter() is a `Iterator adaptor`
}

fn main() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let in_my_size = shoes_in_size(shoes, 10);

    dbg!(in_my_size);
}
```