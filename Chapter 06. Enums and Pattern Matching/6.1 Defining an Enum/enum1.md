The Standard library has `IpAddr` and its implementation as the following-

```rs
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```
>**NOTE:**  
>Even though the standard library contains a definition for `IpAddr`, we can still create and use our own definition exactly the same, without conflict because we haven’t brought the standard library’s definition into our scope
