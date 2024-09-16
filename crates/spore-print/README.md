<div style="display: flex; align-items: center; gap: 16px; margin-top: 24px; margin-bottom: 24px;">
  <img src="../../assets/sporeprint-logo.svg" alt="SporePrint Logo" width="64" height="64" style="margin-top: 18px">
  <h1 style="margin: 0; line-height: 1.3;">spore-print</h1>
</div>

#### `spore-print` is a Rust library for generating consistent and immutable string representations of various types.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spore-print = "0.1.1"
```

## Example

```rust 
use spore_print::{SporePrint, sprint, sprintln};
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
    sprint!(number_range); // Prints without a newline
    sprintln!(number_range); // Prints with a newline
}
```

## Features

Generate consistent and immutable string representations for various types.
Supports collections, options, results, and more.

## License

This project is licensed under the GPL-3 license.