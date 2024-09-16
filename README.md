<div style="display: flex; align-items: center; gap: 16px; margin-top: 24px; margin-bottom: 24px;">
  <img src="assets/sporeprint-logo.svg" alt="SporePrint Logo" width="72" height="72" style="margin-top: 16px">
  <h1 style="margin: 0; line-height: 1;">SporePrint: A Clear, Immutable Representation for Rust Types</h1>
</div>

#### The SporePrint project consists of two crates:

1. **spore-print**: A Rust library for generating consistent and immutable string representations of various types.
2. **spore-print-derive**: A custom derive macro for the `SporePrint` trait from the `spore-print` crate.

## Crates

### spore-print

`spore-print` is a Rust library for generating consistent and immutable string representations of various types.

For more information, see the [spore-print README](crates/spore-print/README.md).

### spore-print-derive

`spore-print-derive` provides a custom derive macro for the `SporePrint` trait from the `spore-print` crate.

For more information, see the [spore-print-derive README](crates/spore-print-derive/README.md).

## Installation

To use both crates in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
spore-print = "0.1.1"
spore-print-derive = "0.1.1"
```

## Usage

For detailed usage instructions, see the individual crate-level README.md files:

* [spore-print Usage](crates/spore-print/README.md)
* [spore-print-derive Usage](crates/spore-print-derive/README.md)

## License

This project is licensed under the GPL-3 license.

## Contribution

Contributions are welcome! Please see the [CONTRIBUTING.md](CONTRIBUTING.md) for details.

