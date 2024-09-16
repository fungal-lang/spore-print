<div style="display: flex; align-items: center;">
  <style>
    .light-mode-icon {
      display: none;
    }
    @media (prefers-color-scheme: dark) {
      .dark-mode-icon {
        display: block;
      }
      .light-mode-icon {
        display: none;
      }
    }
    @media (prefers-color-scheme: light) {
      .dark-mode-icon {
        display: none;
      }
      .light-mode-icon {
        display: block;
      }
    }
  </style>

  <img src="../../assets/sporeprint-logo-dark.svg" alt="logo" width="64" height="64" style="margin-right: 12px; margin-top: 32px;" class="dark-mode-icon">
  <img src="../../assets/sporeprint-logo-light.svg" alt="logo" width="64" height="64" style="margin-right: 12px; margin-top: 32px;" class="light-mode-icon">

  <h1>spore-print</h1>
</div>

`spore-print` is a Rust library for generating consistent and immutable string representations of various types.

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