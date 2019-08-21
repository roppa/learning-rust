# Learning Rust

One of the first things I like to do after gaining some familiarity with a language is a couple of katas, maybe Blackjack, an API of some sort. All test driven of course. Thats what we'll do here.

## Definitions

Rust is a [system language](https://en.wikipedia.org/wiki/System_programming_language), meaning it can be used for low level stuff such as Operating Systems, hardware and the like. That doesn't mean it is limited to those however.

Why Rust? Type safety, concurrency, blazingly fast. Rust has a steep learning curve however.

## Style

Rust has an opinionated coding style:

- indentations are 4 spaces
- variable names are snake case

But, before fundamentals, lets talk about the build runner, documentation generator, test runner and package manager - Cargo.

## Cargo

Assuming you have installed Rust, go ahead and run this:

```bash
cargo new hello
```

This creates a folder:

```bash
└── hello
    ├── Cargo.toml
    └── src
        └── main.rs
```

Cargo.toml:

```toml
[package]
name = "hello"
version = "0.1.0"
authors = ["Your Name <you@your-email.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

And the rust file:

```rust
fn main() {
    println!("Hello, world!");
}
```

First things first, the Cargo.toml file. This is the configuration file for your software. This has all the information about your project, the name, authors, version (semver) etc. It is also where you add your dependencies.

The rust file that is generated is a 'hello world' by default.

Now you can create an executable by running `cargo build`. Then execute by running `cargo run` (`cargo run` by itself builds and then runs).

After you run `build` you will see a 'target' folder is created. in the target folder is 'debug', so you probably want to add this to your `.gitignore` file.

## Main

The genesis of your project is the `main` function. This is what is executed when you run.

```rust
fn main() {}
```

Functions are declared with a `fn` then name, parenthesis, then the body is in braces.

## Types

Integers are signed or unsigned: 8, 16, 32, 64, 128 (usize and isize). i32 or u32 etc.

You can use binary: 0b, octal: 0o, hex: 0x.

Floats are f32 and f64. This is valid: 0.1, this is not: .1.

Bool is `bool`, which are `true` and `false`.

A char is 4 bytes, a scalar, differing from a character within a string, which is utf-8 2 bytes.

Tuple stores multiple values, accessed by dot syntax (maximum of 12):

```rust
let a = (1, 2, 3);
println!("{}, {}, {}", a.0, a.1, a.2);
```

Arrays are limited to 32 length. Vectors are preferred over Arrays.

## Variables

Variables are immutable in Rust, however, you can mutate them by declaring using `let mut`. Rust is strongly typed, but if it can deduce what the variable is, it will automatically create that type for you.

```rust
let [mut] [variable-name][: type] = [value];
```

Just so we can see something, Rust uses the format `println!` to write to the console, more on this later, but notice the exclamation mark before the parenthesis. Also, when you see a `{}` that means it will be replaced by the params you pass after the string.

```rust
let a = 34;
println!("Rust can deduce what type you want: {}", a);

let a:u32 = 34;
println!("Or, you could add the unsigned integer 32 bit `:u32` type {}", a);

let mut change = 34;
change += 1;
println!("This value is mutable, therefore changed {}", change);

let string = "Hello world!";
println!("{}", string);
```

We'll come back to strings.

`const` always uses SCREAMING snake case, and you MUST specify type. Consts are really fast to access. You can use them 'globally' i.e. outside functions.

```rust
const UNCHANGING:u32 = 123;
println!("Const value is {}", UNCHANGING);
```

You can destructure values and assign like this:

```rust
let (var1, var2) = (1, 2);
```

## Help from the Compiler

When you get compile errors, Rust gives you a handy explain command: `rustc --explain E0384`. Run that to get more information on the error. Pretty useful stuff.

## Blocks and Scope

Scope in Rust is block level. It does have access to parent scope (nested blocks). A block can be created with braces `{}`, as in:

```rust
fn main() {
    {
        let x = 1;
        println!("{}", x);
    }
}
```

Variables can also be 'shadowed', so you can declare the same variable name multiple times.

## Functions

```
fn [snake_case function name]([param :type]) -> [return type]
```

For example:

```rust
fn volume(x: i32, y: i32, z: i32) -> i32 {
    return x * y * z;
}

```

With the returned value, if you leave off the trailing semicolon of an expression, that value gets returned. (remember, an expression is anything that results in a value).

```rust
fn volume(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}
```

Note: you know the `println!`, see the exclamation mark? That is a 'macro'. A macro is a type of function (more later). You can use macros when you do not have a fixed arity.

## Loops

- [Rust loops](https://doc.rust-lang.org/1.6.0/book/loops.html)

## Libraries

## Tests

Like all good languages, testing is built in to Rust.

## Crates

https://crates.io/

## Ownership and References

## Resources

- [Actix api framework](https://actix.rs)

## References

- [CleanCut Rust corse](https://github.com/CleanCut/rust_programming/)
