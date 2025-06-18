# Installation

RustSFQ is provided as a Rust library crate. To use it, you must first install Rust and its package manager, Cargo.

## 1. Install Rust

Please install Rust by following the instructions on the official website:

<https://www.rust-lang.org/tools/install>

Installing Rust will also install Cargo, the tool used to manage Rust projects and packages.

## 2. Create a New Rust Project

Once Cargo is installed, you can create a new Rust project using the following command:

```sh
cargo new my_rustsfq_project
```

## 3. Add RustSFQ to Dependencies

To use RustSFQ in your project, add it to your `Cargo.toml` under the `[dependencies]` section:

```toml
[dependencies]
rust_sfq = "0.1"  # Replace with the actual version
```

## 4. Optional: Using VSCode and rust-analyzer

You can use any text editor to develop with RustSFQ, but we recommend using Visual Studio Code along with the **rust-analyzer** extension.

Once installed, rust-analyzer provides powerful IDE features such as:

- Real-time type inference
- Inline compilation errors and warnings
- Code navigation and autocompletion

These features significantly improve development productivity and code quality within the editor.
