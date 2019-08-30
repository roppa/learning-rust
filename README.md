# Learning Rust

One of the first things I like to do after gaining some familiarity with a language is a couple of katas, maybe Blackjack, an API of some sort. All test driven of course. Thats what we'll do here.

What I find sorely missing in education is hierarchy of importances. Most resources give you every single bit of data related to a subject with equal stress. For example, with a written language a hierarchy of inportances would be:

- alphabet
- words
- grammar
- structure

So, for a new language I first learn the alphabet (operators), words (reserved words), grammar (syntax), structure (files, modules, tests).

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

First things first, the Cargo.toml file. This is the configuration file for your software. This has all the information about your project, the name, authors, version (semver), project name (used when imporiting files) etc. It is also where you add your dependencies.

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

To inspect an array use `println!("{:?}", arr);`.

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

## Structs

Structs are types that you define. You can access the attributes of the data type using dot notation.

```rust
struct User {
    name: String,
    email: String,
    age: i32
}

fn main() {
    let bob = User{
        name: "Bob".to_string(),
        email: "bob@bobbity.com".to_string(),
        age: 32
    };

    println!("Bob is: {}, {}, {}", bob.name, bob.email, bob.age);
}
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

## Operators and Control Flow

Operators are just like C like languges, such as `+`, `-`, `!`, `&&`, `||`, `==`, `%`.

Rust does not have a ternery operator because of the way if/else logic works. Notice there is no semi-colon, meaning the expression is returned. For example:

```rust
let toggle = if toggle == 0 { 1 } else { 0 };
```

Remember in TDD you start off with an if/else for one condition, then change it into a while/for etc. Look Mom, no brackets.

```rust
let n = 0;
if n == 0 {
    println!("Oh");
} else {
    println!("Uh oh");
}
```

## Loops

Infinite loop is:

```rust
print!("N");
loop {
    print!("o");
}
```

To escape, use `break`.

While:

```rust
let mut n = 0;
while n < 10 {
    n += 1;
}
```

For:

```rust
for x in 1..101 {
    if x % 3 == 0 && x % 5 == 0 {
        result = format!("{}{}\n", result, "FizzBuzz");
    } else if x % 5 == 0 {
        result = format!("{}{}\n", result, "Buzz");
    } else if x % 3 == 0 {
        result = format!("{}{}\n", result, "Fizz");
    } else {
        result = format!("{}{}\n", result, x.to_string());
    }
}
```

There are iterators too which we'll come to later.

## Important files

The important files are: `main.rs`, `lib.rs` and `mod.rs`.

`main.rs` and `lib.rs` 'crate roots' - contents of either form a module named 'crate' at the root of the crate’s module structure (module tree). Basically a module can be a single file, or a file in a folder. If it is in a folder, you need

You can create module in two ways either a single file or a file within folder.

## Code Structure

Terms are: `crate`, `module`, `path`, `use`, `pub`, `as`.

## Tests

Like all good languages, testing is built in to Rust. Tests go in the file they are testing. The convention is to use a `tests` module in each file, annotating with `cfg(test)`:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn addition_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

## Ownership and References

Rust does not have a garbage collector.

## Command line

To access args you can use `std::env::args()`. This gets them once the program is called:

```rust
let args: Vec<String> = std::env::args().skip(1).collect();
println!("{:?}", args);
```

This loops over input:

```rust
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input = line.unwrap();
        println!("you entered {}", input);
    }
}
```

## System Libraries

```rust
use std::{thread, time};

fn main() {
    let one_second = time::Duration::from_millis(1000);
    print!("Hello, world!");
    thread::sleep(one_second);
    print!("Hello, world!");
    thread::sleep(one_second);
    print!("Hello, world!");
}
```

## Resources

- [Actix api framework](https://actix.rs)
- [Crates](https://crates.io/)

## References

- [CleanCut Rust corse](https://github.com/CleanCut/rust_programming/)
- [Rust loops](https://doc.rust-lang.org/1.6.0/book/loops.html)
- [Modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)
- [Rust unit test](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
- [Rust modules](https://medium.com/@tak2siva/rust-modules-explained-96809931bbbf)
