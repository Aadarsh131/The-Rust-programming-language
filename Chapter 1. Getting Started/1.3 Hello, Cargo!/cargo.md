# Cargo

>**Rust's Build system + Package manager**

No need to install seperate build system like in C/C++ (cmake, makefile), Cargo comes preintalled with `rustc` compiler, also it is `cross platform`, the commands are same for all OS

>$ cargo new hello_cargo  
>$ cd hello_cargo

- It has also initialized a new Git repository along with a .gitignore file. Git files won’t be generated if you run cargo new within an existing Git repository; you can override this behavior by using `cargo new --vcs=git`

- In Rust, packages of code are referred to as crates
- If you started a project that doesn’t use Cargo, as we did with the “Hello, world!” project, you can convert it to a project that does use Cargo. Move the project code into the src directory and create an appropriate Cargo.toml file.
  
> $ cargo build

- default build is `Debug build`
- if the files havn't changed since the last compilation/build, `cargo build` / `cargo run` would not compile the code again
    > $ cargo build --release

    - release build with optimizations
    -  The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile
> $ cargo run
- build and executes the executable


> $ cargo check  
- produces a binary to check for errors, it makes sure the code compiles but doesn’t produce an executable

- why would you not want an executable? Often, `cargo check is much faster than cargo build` because it skips the step of producing an executable. If you’re continually checking your work while writing the code, using cargo check will speed up the process of letting you know if your project is still compiling! As such, `many Rustaceans run cargo check periodically as they write their program to make sure it compiles`. Then they run cargo build when they’re ready to use the executable.
