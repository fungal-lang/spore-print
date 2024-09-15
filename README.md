<div style="display: flex; align-items: center;">
  <img src="assets/sporeprint-logo.svg" alt="logo" width="64" height="64" style="margin-right: 12px;margin-top: 32px;">
  <h1>SporePrint</h1>
</div>


**SporePrint** is a Rust library providing a `SporePrint` trait and a procedural macro to derive it for custom types.

## What is SporePrint?

SporePrint is designed to offer a consistent and immutable string representation of Rust data structures. This can be
particularly useful for debugging, logging, and serialization purposes. By deriving the `SporePrint` trait, you can
easily implement a standardized way to print your custom types.

## Why SporePrint?

In many Rust projects, there is a need for a reliable and uniform way to represent data structures as strings. While the
standard `Display` trait provides great functionality, it may not always meet the specific requirements for
immutability and consistency. The original purpose of SporePrint was to have an immutable alternative to `Display` for
building the compiler for Hypha, a functional language with a focus on immutability. SporePrint addresses this gap by
offering a dedicated trait and a procedural macro to automate its implementation, ensuring that your data structures can
be printed in a predictable and immutable manner.

## Installation

To use `SporePrint` in your project, add both `spore-print` and `spore-print-derive` to your `Cargo.toml`:

```toml
[dependencies]
spore-print = "0.1.0"
spore-print-derive = "0.1.0"

```

Usage
Add both spore-print and spore-print-derive to your dependencies:

```rust
use spore_print::SporePrint;  // Import the SporePrint trait
use spore_print_derive::SporePrint;  // Import the derive macro

#[derive(SporePrint)]
struct MyStruct {
    field1: String,
    field2: usize,
}

fn main() {
    let instance = MyStruct { field1: "Hello".to_string(), field2: 42 };
    println!("{}", instance.spore_print());
}

```

Documentation
Full documentation is available on docs.rs.
License

This project is licensed under the GPL-3 license.

## Contribution

Contributions are welcome! Please see the [CONTRIBUTING.md](CONTRIBUTING.md) for details.

