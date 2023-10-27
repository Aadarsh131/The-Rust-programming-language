## Module System
Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the *module system*, include:

- **Packages:** A Cargo feature that lets you build, test, and share crates
- **Crates:** A tree of modules that produces a library or executable
- **Modules** and **use:** Let you control the organization, scope, and privacy of paths
- **Paths:** A way of naming an item, such as a struct, function, or module

# Packages and Crates
### <u>Crates</u>
A crate is the smallest amount of code that the Rust compiler considers at a time. Even if we run `rustc` rather than `cargo` and pass a single source code file, the compiler considers that file to be a crate.

#### Two Forms of Crates-
 1. **Binary Crate**- which produces an executable and it must have `main` function.
 2. **Library Crate**- doesnot produce an executable and it doesn't have `main` function. Instead, they define functionality intended to be shared with multiple projects.   
 Rustaceans often say `crate` interchangeably with `library`.

 The **crate root** is a source file that the Rust compiler starts from and makes up the root module of your crate

 ### <u> Packages </u>
 A package is a bundle of one or more crates that provides a set of functionality. A package contains a *Cargo.toml* file that describes how to build those crates.  
 
  A package can contain as many binary crates as you like, but at most only one library crate.  
   A package must contain at least one crate, whether that’s a library or binary crate.
 
 - `Cargo` is actually a package that contains the **binary crate** for the command-line tool you’ve been using to build your code
 - The `Cargo` package also contains a **library crate** that the binary crate depends on. 
 - Other projects can depend on the `Cargo library crate` to use the same logic the Cargo command-line tool uses

## Basic Project Structure
In the project directory, after we create a new project with `cargo new`, there's a *Cargo.toml* file, giving us a package.  
Cargo follows a convention that *src/main.rs* is the crate root of a binary crate with the same name as the package.  
and *src/lib.rs*, is the crate root of library crate with the same name as package.  

Cargo passes the crate root files to `rustc` to build the *library or binary*.

If a package contains *src/main.rs* and *src/lib.rs*, it has two crates: a binary and a library, both with the same name as the package. A package can have multiple binary crates by placing files in the *src/bin* directory: each file will be a separate binary crate.
