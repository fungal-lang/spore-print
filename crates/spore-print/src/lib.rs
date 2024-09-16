mod macros;
use std::collections::{HashMap, HashSet};
use std::ops::{Range, RangeInclusive};

/// The `SporePrint` trait provides a method to get a consistent and immutable string representation of a type.
///
/// # Examples
///
/// ```
/// use spore_print::SporePrint;
///
/// let value = 42;
/// assert_eq!(value.spore_print(), "42");
/// ```
pub trait SporePrint {
    fn spore_print(&self) -> String;
}

// Implement `SporePrint` for types that implement `Display`
macro_rules! impl_spore_print_for_display {
    ($($t:ty),*) => {
        $(impl SporePrint for $t {
            fn spore_print(&self) -> String {
                self.to_string()
            }
        })*
    };
}

impl_spore_print_for_display!(
    u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64, String, &str, char, bool
);

// Implement `SporePrint` for `Option<T>`
impl<T> SporePrint for Option<T>
where
    T: SporePrint,
{
    fn spore_print(&self) -> String {
        match self {
            Some(value) => format!("Some({})", value.spore_print()),
            None => "None".to_string(),
        }
    }
}

// Implement `SporePrint` for unit type `()`
impl SporePrint for () {
    fn spore_print(&self) -> String {
        "()".to_string()
    }
}

// Implement `SporePrint` for collections
macro_rules! impl_spore_print_for_collections {
    ($($t:ty),*) => {
        $(
            impl<T> SporePrint for $t
            where
                T: SporePrint,
            {
                fn spore_print(&self) -> String {
                    let items = self
                        .iter()
                        .map(|item| item.spore_print())
                        .collect::<Vec<_>>();

                    format!("[{}]", items.join(", "))
                }
            }
        )*
    };
}

impl_spore_print_for_collections!(Vec<T>, HashSet<T>);

// Implement `SporePrint` for `HashMap<K, V>`
impl<K, V> SporePrint for HashMap<K, V>
where
    K: SporePrint,
    V: SporePrint,
{
    fn spore_print(&self) -> String {
        let items = self
            .iter()
            .map(|(key, value)| format!("{}: {}", key.spore_print(), value.spore_print()))
            .collect::<Vec<_>>();

        format!("{{{}}}", items.join(", "))
    }
}

/// Macro to implement `SporePrint` for tuples of varying lengths. We provide implementations up to tuples of size 12,
/// similar to Rust's standard library support for traits like `Debug` and `Display`.
macro_rules! impl_spore_print_for_tuples {
    // Special case for single-element tuples
    ($T1:ident) => {
        #[allow(non_snake_case)]
        impl<$T1: SporePrint> SporePrint for ($T1,) {
            fn spore_print(&self) -> String {
                let ($T1,) = self;
                format!("({},)", $T1.spore_print())
            }
        }
    };

    // General case for tuples with more than one element
    ($($T:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($T: SporePrint),+> SporePrint for ($($T,)+) {
            fn spore_print(&self) -> String {
                #[allow(non_snake_case)]
                let ($($T,)+) = self;
                let values = vec![$($T.spore_print()),+];
                format!("({})", values.join(", "))
            }
        }
    };
}

impl_spore_print_for_tuples!(T1);
impl_spore_print_for_tuples!(T1, T2);
impl_spore_print_for_tuples!(T1, T2, T3);
impl_spore_print_for_tuples!(T1, T2, T3, T4);
impl_spore_print_for_tuples!(T1, T2, T3, T4, T5);
impl_spore_print_for_tuples!(T1, T2, T3, T4, T5, T6);
impl_spore_print_for_tuples!(T1, T2, T3, T4, T5, T6, T7);
impl_spore_print_for_tuples!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_spore_print_for_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_spore_print_for_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_spore_print_for_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_spore_print_for_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);

// Helper function to format items
fn format_items<T: SporePrint>(items: &[T]) -> String {
    let formatted_items = items
        .iter()
        .map(|item| item.spore_print())
        .collect::<Vec<_>>();
    format!("[{}]", formatted_items.join(", "))
}

// Implement `SporePrint` for slices
impl<T> SporePrint for &[T]
where
    T: SporePrint,
{
    fn spore_print(&self) -> String {
        format_items(self)
    }
}

// Implement `SporePrint` for arrays
impl<T, const N: usize> SporePrint for [T; N]
where
    T: SporePrint,
{
    fn spore_print(&self) -> String {
        format_items(self)
    }
}

// Implement `SporePrint` for references
impl<T> SporePrint for &T
where
    T: SporePrint,
{
    fn spore_print(&self) -> String {
        (*self).spore_print()
    }
}

// Implement `SporePrint` for `Result<T, E>`
impl<T, E> SporePrint for Result<T, E>
where
    T: SporePrint,
    E: SporePrint,
{
    fn spore_print(&self) -> String {
        match self {
            Ok(value) => format!("Ok({})", value.spore_print()),
            Err(err) => format!("Err({})", err.spore_print()),
        }
    }
}

// Implement `SporePrint` for `Range<T>`
impl<T> SporePrint for Range<T>
where
    T: SporePrint,
{
    fn spore_print(&self) -> String {
        format!("{}..{}", self.start.spore_print(), self.end.spore_print())
    }
}

// Implement `SporePrint` for `RangeInclusive<T>`
impl<T> SporePrint for RangeInclusive<T>
where
    T: SporePrint,
{
    fn spore_print(&self) -> String {
        format!(
            "{}..={}",
            self.start().spore_print(),
            self.end().spore_print()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};
    use std::iter::FromIterator;

    /// Tests `SporePrint` implementation for `String`
    #[test]
    fn test_string() {
        let value = String::from("test");
        assert_eq!(value.spore_print(), "test");
    }

    /// Tests `SporePrint` implementation for `&str`
    #[test]
    fn test_str() {
        let value: &str = "test";
        assert_eq!(value.spore_print(), "test");
    }

    /// Tests `SporePrint` implementation for integer types
    #[test]
    fn test_integer() {
        let value = 42;
        assert_eq!(value.spore_print(), "42");
    }

    /// Tests `SporePrint` implementation for `Option` with `Some` value
    #[test]
    fn test_option_some() {
        let value = Some(42);
        assert_eq!(value.spore_print(), "Some(42)");
    }

    /// Tests `SporePrint` implementation for `Option` with `None` value
    #[test]
    fn test_option_none() {
        let value: Option<i32> = None;
        assert_eq!(value.spore_print(), "None");
    }

    /// Tests `SporePrint` implementation for nested `Option`
    #[test]
    fn test_nested_option() {
        let nested_option: Option<Option<i32>> = Some(Some(42));
        assert_eq!(nested_option.spore_print(), "Some(Some(42))");
    }

    /// Tests `SporePrint` implementation for `Result` with `Ok` value
    #[test]
    fn test_result_ok() {
        let result: Result<i32, &str> = Ok(42);
        assert_eq!(result.spore_print(), "Ok(42)");
    }

    /// Tests `SporePrint` implementation for `Result` with `Err` value
    #[test]
    fn test_result_err() {
        let result: Result<i32, &str> = Err("error");
        assert_eq!(result.spore_print(), "Err(error)");
    }

    /// Tests `SporePrint` implementation for `Range<usize>`
    #[test]
    fn test_range_usize() {
        let range = 3..10;
        assert_eq!(range.spore_print(), "3..10");
    }

    /// Tests `SporePrint` implementation for `Range<f32>`
    #[test]
    fn test_range_f32() {
        let range = 1.5..4.5;
        assert_eq!(range.spore_print(), "1.5..4.5");
    }

    /// Tests `SporePrint` implementation for `Range<String>`
    #[test]
    fn test_range_string() {
        let range = "a".to_string().."z".to_string();
        assert_eq!(range.spore_print(), "a..z");
    }

    /// Tests `SporePrint` implementation for `Range<i32>`
    #[test]
    fn test_range_i32() {
        let range = -5..5;
        assert_eq!(range.spore_print(), "-5..5");
    }

    /// Tests `SporePrint` implementation for `RangeInclusive<i32>`
    #[test]
    fn test_range_char_inclusive() {
        let range = 'a'..='z';
        assert_eq!(range.spore_print(), "a..=z");
    }

    /// Tests `SporePrint` implementation for vector of strings
    #[test]
    fn test_vec_of_strings() {
        let vec = vec!["one".to_string(), "two".to_string(), "three".to_string()];
        assert_eq!(vec.spore_print(), "[one, two, three]");
    }

    /// Tests `SporePrint` implementation for `HashSet<i32>`
    #[test]
    fn test_hashset() {
        let set: HashSet<i32> = HashSet::from([1, 2]);
        let expected: HashSet<String> = HashSet::from_iter(vec!["1".to_string(), "2".to_string()]);
        let actual: HashSet<String> = HashSet::from_iter(
            set.spore_print()
                .trim_matches(|c| c == '[' || c == ']')
                .split(", ")
                .map(|s| s.to_string()),
        );
        assert_eq!(actual, expected);
    }

    /// Tests `SporePrint` implementation for `HashMap<&str, i32>`
    #[test]
    fn test_hashmap() {
        let map: HashMap<&str, i32> = HashMap::from([("key1", 1), ("key2", 2)]);
        let expected: HashSet<String> =
            HashSet::from_iter(vec!["key1: 1".to_string(), "key2: 2".to_string()]);
        let actual: HashSet<String> = HashSet::from_iter(
            map.spore_print()
                .trim_matches(|c| c == '{' || c == '}')
                .split(", ")
                .map(|s| s.to_string()),
        );
        assert_eq!(actual, expected);
    }

    /// Tests `SporePrint` implementation for tuples of varying lengths
    #[test]
    fn test_tuples() {
        let tuple_2 = (42, "hello");
        assert_eq!(tuple_2.spore_print(), "(42, hello)");

        let tuple_3 = (42, "hello", Some(std::f64::consts::PI));
        assert_eq!(
            tuple_3.spore_print(),
            "(42, hello, Some(3.141592653589793))"
        );

        let tuple_4 = (42, "hello", std::f64::consts::PI, true);
        assert_eq!(
            tuple_4.spore_print(),
            "(42, hello, 3.141592653589793, true)"
        );

        let tuple_nested = ((1, 2), ("a", "b"));
        assert_eq!(tuple_nested.spore_print(), "((1, 2), (a, b))");

        let tuple_complex = (42, "hello", vec![1, 2, 3]);
        assert_eq!(tuple_complex.spore_print(), "(42, hello, [1, 2, 3])");
    }

    /// Tests `SporePrint` implementation for an empty tuple `()`
    #[test]
    fn test_empty_tuple() {
        let tuple: () = ();
        assert_eq!(tuple.spore_print(), "()");
    }

    /// Tests `SporePrint` implementation for a single-element tuple `(T1,)`
    #[test]
    fn test_single_element_tuple() {
        let tuple = (42,);
        assert_eq!(tuple.spore_print(), "(42,)");
    }

    /// Tests `SporePrint` implementation for tuples containing references
    #[test]
    fn test_tuple_with_references() {
        let value = 42;
        let reference_tuple = (&value, &"hello");
        assert_eq!(reference_tuple.spore_print(), "(42, hello)");
    }

    /// Tests `SporePrint` implementation for deeply nested tuples
    #[test]
    fn test_nested_tuples() {
        let tuple = ((1, 2), (3, (4, (5, "deep"))));
        assert_eq!(tuple.spore_print(), "((1, 2), (3, (4, (5, deep))))");
    }

    /// Tests `SporePrint` implementation for a tuple with `Option` values, all `None`
    #[test]
    fn test_tuple_with_none_options() {
        let tuple = (None::<i32>, None::<&str>, None::<f64>);
        assert_eq!(tuple.spore_print(), "(None, None, None)");
    }

    /// Tests `SporePrint` implementation for a tuple with mixed types
    #[test]
    fn test_tuple_with_mixed_types() {
        let tuple = (
            "string",
            vec![1, 2, 3],
            Option::<()>::None,
            Result::<f64, &str>::Err("error"),
        );
        assert_eq!(tuple.spore_print(), "(string, [1, 2, 3], None, Err(error))");
    }

    /// Tests `SporePrint` implementation for slices
    #[test]
    fn test_slice() {
        let slice: &[i32] = &[1, 2, 3];
        assert_eq!(slice.spore_print(), "[1, 2, 3]");
    }

    /// Tests `SporePrint` implementation for arrays
    #[test]
    fn test_array() {
        let array: [i32; 3] = [1, 2, 3];
        assert_eq!(array.spore_print(), "[1, 2, 3]");
    }

    /// Tests `SporePrint` implementation for references to various types
    #[test]
    fn test_references() {
        let value = 42;
        let reference: &i32 = &value;
        assert_eq!(reference.spore_print(), "42");

        let value = String::from("hello");
        let reference: &String = &value;
        assert_eq!(reference.spore_print(), "hello");

        let value = vec![1, 2, 3];
        let reference: &Vec<i32> = &value;
        assert_eq!(reference.spore_print(), "[1, 2, 3]");

        let value = Some(42);
        let reference: &Option<i32> = &value;
        assert_eq!(reference.spore_print(), "Some(42)");

        let value = (42, "hello");
        let reference: &(i32, &str) = &value;
        assert_eq!(reference.spore_print(), "(42, hello)");
    }

    /// Tests `SporePrint` implementation for custom types
    #[test]
    fn test_custom_types() {
        struct Custom(i32);
        impl SporePrint for Custom {
            fn spore_print(&self) -> String {
                format!("Custom({})", self.0)
            }
        }

        let slice: &[Custom] = &[Custom(1), Custom(2)];
        assert_eq!(slice.spore_print(), "[Custom(1), Custom(2)]");

        let value = Custom(1);
        let reference: &Custom = &value;
        assert_eq!(reference.spore_print(), "Custom(1)");
    }
}
