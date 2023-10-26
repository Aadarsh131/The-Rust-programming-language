## `if let` and `else`, instead of `match`
```rs
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (), //one boilerplate code can be removed with `if let`
}
```
<br>
less verbose, trade-off for conciseness over exhaustive checking-

```rs
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```

<br>
if let and else expression-

```rs
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
} else{
    println!("None(null value of Rust)");
}
```


