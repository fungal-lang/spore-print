# spore-print-derive

`spore-print-derive` provides a custom derive macro for the `SporePrint` trait from the `spore-print` crate.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spore-print = "0.1.0"
spore-print-derive = "0.1.0"
```

Example

```rust
use spore_print::SporePrint;
use spore_print_derive::SporePrint;
use std::ops::Range;

#[derive(SporePrint)]
struct Mushroom {
    species: String,
    cap_diameter: Range<usize>,
}

fn main() {
    let mushroom = Mushroom {
        species: "Coprinus Comatus".to_string(),
        cap_diameter: 3..10,
    };
    println!("{}", mushroom.spore_print());
}
```

## Features

Derive macro for the SporePrint trait.

## License

This project is licensed under the GPL-3 license.

This example demonstrates how to use the derive macro from the `spore_print_derive` crate to automatically implement the
`SporePrint` trait for a custom type.
