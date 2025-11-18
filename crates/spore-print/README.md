<div style="display: flex; align-items: center; gap: 16px; margin-top: 24px; margin-bottom: 24px;">
  <img src="../../assets/sporeprint-logo.svg" alt="SporePrint Logo" width="64" height="64" style="margin-top: 18px">
  <h1 style="margin: 0; line-height: 1.3;">spore-print</h1>
</div>

## What is SporePrint?

`spore-print` provides **canonical, deterministic string representations** for Rust types.

Unlike `Debug` (which is for developers) or `Display` (which is for end users), `SporePrint` is designed for:

- **Compiler IR pretty-printing** - Stable output for diffs and debugging
- **Serialization to canonical format** - Same value always produces same string
- **Error messages** - Consistent formatting across versions
- **Testing** - Deterministic output for snapshot tests
- **Hashing and comparison** - When you need stable string representations

### Guarantees

✅ **Pure**: `spore_print()` has no side effects
✅ **Deterministic**: Same input always produces same output
✅ **Canonical**: One value, one representation (no aliases)
✅ **Compositional**: Nested types print consistently
✅ **Stack-safe**: Depth limiting prevents overflow on deeply nested types

### When to use

| Trait        | Purpose                      | Example                 |
|--------------|------------------------------|-------------------------|
| `Debug`      | Developer debugging          | `{:?}` with type hints  |
| `Display`    | User-facing output           | `{}` human-friendly     |
| `SporePrint` | Canonical representation     | Compiler IR, hashing    |

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spore-print = "0.2.2"
```

## Example

```rust
use spore_print::{SporePrint, sprint, sprintln};
use std::ops::Range;

// Built-in support for common types
let vec = vec![1, 2, 3];
assert_eq!(vec.spore_print(), "[1, 2, 3]");

let opt = Some(42);
assert_eq!(opt.spore_print(), "Some(42)");

let result: Result<i32, &str> = Ok(100);
assert_eq!(result.spore_print(), "Ok(100)");

// Custom types
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
    sprintln!(number_range); // Prints: 3..10
}
```

## Features

- ✅ **Built-in support** for primitives, strings, Options, Results, tuples, ranges
- ✅ **Collection support** for `Vec`, `HashMap`, `HashSet`, `VecDeque`, `LinkedList`, and `im` crate collections
- ✅ **Smart pointer support** for `Box`, `Rc`, `Arc`
- ✅ **Path support** for `Path` and `PathBuf` (useful for compilers)
- ✅ **Derive macro** for custom structs and enums via `spore-print-derive`
- ✅ **Depth limiting** to prevent stack overflow on pathological inputs
- ✅ **Zero dependencies** for core functionality (only `im` and `phf` for additional collections)

## Advanced Features

### Depth Limiting

Prevent stack overflow on deeply nested structures:

```rust
use spore_print::SporePrint;

// Deeply nested Option (100+ levels)
let mut deeply_nested = Some(42);
for _ in 0..150 {
    deeply_nested = Some(deeply_nested);
}

// Safe - automatically limits depth
println!("{}", deeply_nested.spore_print());

// Custom depth limit
println!("{}", deeply_nested.spore_print_depth(10));
```

### Derive Macro

```rust
use spore_print::SporePrint;
use spore_print_derive::SporePrint;

#[derive(SporePrint)]
struct Person {
    name: String,
    age: u32,
}

let person = Person {
    name: "Alice".to_string(),
    age: 30,
};

assert_eq!(person.spore_print(), "Person { name: Alice, age: 30 }");
```

## Performance

- **O(n) string building** - Uses efficient `Vec::join` instead of naive concatenation
- **Minimal allocations** - Pre-calculates capacity where possible
- **Stack-safe** - Depth limiting prevents overflow

## License

This project is licensed under the MIT license.