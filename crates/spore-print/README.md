# spore-print

`spore-print` is a Rust library for generating consistent and immutable string representations of various types.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spore-print = "0.1.0"
```

## Example

```rust 
use spore_print::SporePrint;
use std::ops::Range;

/// A struct representing a range of numbers.
struct NumberRange {
    range: Range<usize>,
}

impl SporePrint for NumberRange {
    fn spore_print(&self) -> String {
        self.range.spore_print()
    }
}

fn main() {
    let number_range = NumberRange { range: 3..10 };
    println!("{}", number_range.spore_print());
}   println!("{}", mushroom.spore_print());
}
```

## Features

Generate consistent and immutable string representations for various types.
Supports collections, options, results, and more.

## License

This project is licensed under the GPL-3 license.