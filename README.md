# SporePrint

**SporePrint** is a Rust library providing a `SporePrint` trait and a procedural macro to derive it for custom types.

## Installation

To use `SporePrint` in your project, add both `spore-print` and `spore-print-derive` to your `Cargo.toml`:

```toml
[dependencies]
spore-print = "0.1.0"
spore-print-derive = "0.1.0"

```
Usage

Add both spore-print and spore-print-derive to your dependencie
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

This project is licensed under the MIT or Apache-2.0 license.
Contribution

Contributions are welcome! Please see the CONTRIBUTING.md for details.

