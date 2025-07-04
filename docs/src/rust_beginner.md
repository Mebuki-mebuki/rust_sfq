# For Rust Beginners

This page is intended for readers who have **never used Rust before**.  
It explains the **minimum Rust knowledge required to use RustSFQ**, including basic syntax, data types, ownership, and how to run your first Rust program.

---

# Hello World

## Creating a Project

To create a new project using Cargo (Rust’s build system and package manager), use the following command:

```bash
cargo new hello_rust
```

This creates a new folder `hello_rust` with the basic project structure.

## The `main` Function

In Rust, the execution of any program begins at the `main` function. This is the **entry point** of the program.

Here is the simplest possible Rust program:

```rust
fn main() {
    println!("Hello, World!");
}
```

## Running a Project

Navigate into the project directory and run it:

```bash
cd hello_rust
cargo run
```

This will compile the code and execute the `main` function, producing the output:

```text
Hello, World!
```

## The `print!` Macro

Rust uses **macros** for certain language features, and printing to the console is done using the `print!` macro (note the exclamation mark `!`).

`println!` macro prints text with a newline.

```rust
print!("Hello");
println!(", World!");
```

output:

```text
Hello, World!
```

You can also print variables by using `{}` inside the string:

```rust
let name = "Alice";
println!("Hello, {}!", name); // Hello, Alice!
let x = 1;
let y = 2;
println!("{} + {} = {}", x, y, x + y);  // 1 + 2 = 3
```

---

# Variables and Types in Rust

In Rust, variables are declared using the `let` keyword. By default, variables are **immutable**, but you can make them mutable by adding `mut`.

```rust
let x = 5;
println!("x = {}", x);  // x = 5
// x = 6; // Error: `x` is immutable

let mut y = 10;
y = 20; // Allowed: `y` is mutable
println!("y = {}", y);  // y = 20
```

## Shadowing (Re-declaring a Variable)

Rust allows you to re-declare a variable using `let` again. This is called **shadowing** and can be used to change the value.

```rust
let z = 100;
let z = z + 1;
println!("z = {}", z);  // z = 101
```

## Integers

Rust has both signed and unsigned integer types with an explicit size, such as:

- `i8`, `i16`, `i32`, `i64`, `i128`: signed
- `u8`, `u16`, `u32`, `u64`, `u128`: unsigned

These types depend on the system architecture (32-bit or 64-bit):

- `isize` = signed integer the size of a pointer
- `usize` = unsigned integer the size of a pointer

By default:

```rust
let a = 10;      // inferred as i32
let b = 20u64;   // explicitly u64
let c: i16 = -5; // type annotation
```

## Type Annotations

While Rust often infers types automatically, you can specify them explicitly when needed:

```rust
let x: u32 = 123;
let y: f64 = 3.1415;
let flag: bool = true;
```

## String Types

Rust has two kinds of strings:

1. String slices (`&str`): used for fixed string data
2. `String`: a growable heap-allocated string

```rust
let name: &str = "Alice";
println!("{}", name);   // Alice

let mut greeting = String::from("Hello");
greeting.push_str(", World!");
println!("{}", greeting);   // Hello, World!
```

### Converting `String` to `&str`

1. Using `.as_str()`
2. Using `&` for implicit conversion

```rust
fn print_str(s: &str) {
    println!("{}", s);
}

fn main() {
    let s = String::from("Hello");

    print_str(s.as_str());  // OK. explicit conversion
    print_str(&s);          // OK. implicit conversion from &String to &str
    print_str(s);           // NG. Cannot implicitly convert from String to &str
}
```

---

# Functions

In Rust, functions are defined using the `fn` keyword. All parameters and return types must be explicitly typed.  
Unlike Python or JavaScript, Rust does not support default parameter values.  
You must explicitly pass all arguments when calling a function.

## Basic Syntax

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn square(x: i32) -> i32 {
    return x * x;
}

fn main() {
    let sum = add(3, 5);
    let sq = square(sum);
}
```

- `fn` — starts a function definition
- `(a: i32, b: i32)` — parameters with required type annotations
- `-> i32` — return type (also required)

- The final expression (without a semicolon) is returned
  - You can also use `return` (with a semicolon)

---

# Ownership and References

Rust has a unique **ownership system** that ensures memory safety without a garbage collector.

## Copy vs Move

Some simple types, like integers, implement the `Copy` trait.  
This means assigning them creates a copy, and both variables can be used.

```rust
let a = 5;
let b = a; // a is copied into b

println!("a = {}, b = {}", a, b); // OK. both are usable
```

However, **complex types like `String` do not implement `Copy`**.  
Instead, assigning them transfers ownership — this is called a **move**.

```rust
let s1 = String::from("hello");
let s2 = s1; // ownership moves from s1 to s2

// println!("{}", s1); // Error: s1 was moved
println!("{}", s2);   // OK
```

Once ownership has moved, the original variable (`s1`) can no longer be used.

## References: Borrowing Instead of Moving

To use a value **without taking ownership**, you can **borrow** it by using a reference (`&`).

```rust
fn print_message(msg: &String) {
    println!("Message: {}", msg);
}

fn main() {
    let s = String::from("Rust");
    print_message(&s); // pass by reference
    println!("{}", s); // OK. still usable
}
```

- `&s` is a immutable reference to `s`
- Ownership is not transferred
- The original variable remains usable

## Mutable References

If you want to **modify** a value through a reference, use a **mutable reference** (`&mut`).

```rust
fn change(s: &mut String) {
    s.push_str(", Rust!");
}

fn main() {
    let mut s = String::from("Hi");
    change(&mut s); // pass by mutable reference
    println!("{}", s); // "Hi, Rust!"
}
```

## Reference Safety Rules

- **Only one** mutable reference (`&mut`) is allowed at a time
- A mutable reference (`&mut`) and immutable reference (`&`) **cannot coexist**
- **Multiple** immutable references (`&`) are allowed simultaneously

These rules are enforced at compile time, ensuring memory safety without the need for runtime checks.

---

# Array, Vector, and Tuple

Rust provides multiple ways to store collections of values.  
Here are the most common types: **array**, **vector**, and **tuple**.

## Array `[T; N]`

- Fixed-size, stack-allocated collection of elements of the same type.
- Size `N` must be known at compile time.
- You cannot resize an array after creation.
- Arrays can also be unpacked (destructured) just like tuples.

```rust
let arr: [i32; 3] = [10, 20, 30];
println!("{}", arr[0]); // 10

let [x, y, z] = arr;
println!("x = {}, y = {}, z = {}", x, y, z); // x = 10, y = 20, z = 30
```

## Vector `Vec<T>`

- Growable, heap-allocated list of elements.
- All elements must be the same type.
- You cannot destructure a vector in the same way as an array.

```rust
let mut vec: Vec<i32> = Vec::new();
vec.push(1);
vec.push(2);
println!("{:?}", vec); // [1, 2]
```

Shortcut initialization with macro:

```rust
let v = vec![1, 2, 3];
```

## Tuple `(T1, T2, ...)`

- Group values of **different types** together.
- Fixed size and order matters.

```rust
let tup: (i32, bool, &str) = (42, true, "hello");
let (a, b, c) = tup; // destructuring

println!("First: {}", tup.0); // 42
```

Tuples are especially useful for returning multiple values from a function.

---

# Using Modules

Rust uses a modular system to organize code. When working with external crates (libraries) or internal modules, you use the `use` keyword to bring items into scope.

## External Libraries

To use an external library (crate), you must:

1. Add it to your `Cargo.toml` dependencies:

    ```toml
    [dependencies]
    rust_sfq = "0.1"
    ```

2. Then import items from the crate in your code:

    ```rust
    use rust_sfq::*;
    ```

Rust will download the crate from [crates.io](https://crates.io) the first time you build the project.

This brings all public items from the crate into scope, including:

- `Circuit`
- `Wire`, `CounterWire`
- Backend modules like `RsfqlibSpice` or `RsfqlibVerilog`

If you prefer more explicit imports:

```rust
use rust_sfq::{Circuit, RsfqlibSpice};
```
