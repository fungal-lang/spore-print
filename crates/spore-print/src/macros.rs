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
/// This macro prints the SporePrint representation to stdout with a trailing newline.
///
/// # Examples
///
/// ```
/// use spore_print::sprintln;
///
/// let value = 42;
/// sprintln!(value);  // Output: 42 (with newline)
/// ```
#[macro_export]
macro_rules! sprintln {
    ($($arg:tt)*) => {
        println!("{}", spore_print::SporePrint::spore_print(&$($arg)*))
    };
}
