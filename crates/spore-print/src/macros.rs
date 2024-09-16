/// Returns the formatted string representation of the given arguments using the `SporePrint` trait.
///
/// This macro uses the `SporePrint` trait to generate a consistent and immutable string representation
/// of the provided arguments and returns it as a `String`.
///
/// # Examples
///
/// ```
/// use spore_print::sprint;
///
/// let value = 42;
/// let result = sprint!(value);  // result: "42"
/// ```
#[macro_export]
macro_rules! sprint {
    ($($arg:tt)*) => {
        spore_print::SporePrint::spore_print(&$($arg)*)
    };
}

/// Prints the formatted string representation of the given arguments using the `SporePrint` trait,
/// followed by a newline.
///
/// This macro is a wrapper around `sprint!` that adds a newline after printing.
///
/// # Examples
///
/// ```
/// use spore_print::{sprint, sprintln};
///
/// let value = 42;
/// sprintln!(value);  // Output: 42 (with newline)
/// ```
#[macro_export]
macro_rules! sprintln {
    ($($arg:tt)*) => {
        sprint!($($arg)*);
        println!();
    };
}
