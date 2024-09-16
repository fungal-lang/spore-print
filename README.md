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

  <img src="assets/sporeprint-logo-dark.svg" alt="logo" width="64" height="64" style="margin-right: 12px; margin-top: 32px;" class="dark-mode-icon">
  <img src="assets/sporeprint-logo-light.svg" alt="logo" width="64" height="64" style="margin-right: 12px; margin-top: 32px;" class="light-mode-icon">

  <h1>SporePrint: A Clear, Immutable Representation for Rust Types</h1>
</div>


The SporePrint project consists of two crates:

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

