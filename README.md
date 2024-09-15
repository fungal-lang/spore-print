<div style="display: flex; align-items: center;">
  <img src="assets/sporeprint-logo.svg" alt="logo" width="64" height="64" style="margin-right: 12px;margin-top: 32px;">
  <h1>SporePrint: A Clear, Immutable Representation for Rust Types</h1>
</div>

## What is SporePrint?

SporePrint is a Rust library that provides a trait and a procedural macro to derive a consistent, immutable string
representation for custom types. It’s like the `Display` trait but with a stronger focus on immutability, designed to be
simple and easy to use.

## Why SporePrint?

Representing data as strings is essential for debugging, logging, and more. While `Display` offers flexibility,
SporePrint
takes it a step further by guaranteeing immutability—because who doesn’t love a string that stays perfectly unchanged?
It was created to support the Hypha compiler, a project that embraces immutability in its design. With `SporePrint`,
your types have a clear and stable format, making them easier to understand and work with.

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

This project is licensed under the GPL-3 license.

## Contribution

Contributions are welcome! Please see the [CONTRIBUTING.md](CONTRIBUTING.md) for details.

