mod format;
mod macros;

use crate::format::{format_collection, format_enum, format_map, format_set, format_tuple};
use im::{vector, HashMap, HashSet, Vector};
use std::ops::{Range, RangeInclusive};
use std::rc::Rc;
use std::sync::Arc;

/// The `SporePrint` trait provides a method to get a consistent and immutable string representation of a type.
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

// Implement `SporePrint` for `Struct<T>`
impl<T> SporePrint for std::marker::PhantomData<T> {
    fn spore_print(&self) -> String {
        "PhantomData".to_string()
    }
}

// macro_rules! impl_spore_print_for_struct {
//     ($name:ident, $fields:ident) => {
//         impl SporePrint for $name {
//             fn spore_print(&self) -> String {
//                 format_struct(stringify!($name), vector![$(self.$fields.spore_print()),*])
//             }
//         }
//     };
// }

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

// Helper function for spore_print implementation for collections
fn spore_print_collection<I>(items: I) -> String
where
    I: IntoIterator,
    I::Item: SporePrint,
{
    let items = items
        .into_iter()
        .map(|item| item.spore_print())
        .collect::<Vector<_>>();
    format_collection(items.into_iter())
}

impl<T> SporePrint for HashSet<T>
where
    T: SporePrint + Clone, // Ensure T is Clone
{
    fn spore_print(&self) -> String {
        format_set(self.iter().cloned())
    }
}

impl<K, V> SporePrint for HashMap<K, V>
where
    K: SporePrint,
    V: SporePrint,
{
    fn spore_print(&self) -> String {
        if self.is_empty() {
            "{}".to_string()
        } else {
            let entries = self
                .iter()
                .map(|(key, value)| (key.spore_print(), value.spore_print()));
            format_map(entries)
        }
    }
}

// Implement `SporePrint` for `phf::Map<K, V>`
impl<K, V> SporePrint for phf::Map<K, V>
where
    K: SporePrint,
    V: SporePrint,
{
    fn spore_print(&self) -> String {
        format_map(self.entries())
    }
}

// Implement `SporePrint` for tuples
macro_rules! impl_spore_print_for_tuples {
    ($($T:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($T: SporePrint),+> SporePrint for ($($T,)+) {
            fn spore_print(&self) -> String {
                #[allow(non_snake_case)]
                let ($($T,)+) = self;
                format_tuple(Vector::from(vector![$($T.spore_print()),+]))
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

// Implement `SporePrint` for slices
impl<T> SporePrint for &[T]
where
    T: SporePrint,
{
    fn spore_print(&self) -> String {
        spore_print_collection(*self)
    }
}

// Implement `SporePrint` for arrays
impl<T, const N: usize> SporePrint for [T; N]
where
    T: SporePrint,
{
    fn spore_print(&self) -> String {
        spore_print_collection(self)
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
            Ok(value) => format_enum("", "Ok", vector![value.spore_print()]),
            Err(err) => format_enum("", "Err", vector![err.spore_print()]),
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

// Implement `SporePrint` for `Vector<T>`
impl<T> SporePrint for Vector<T>
where
    T: SporePrint + Clone,
{
    fn spore_print(&self) -> String {
        spore_print_collection(self)
    }
}

// Implement `SporePrint` for `Box<T>`, `Rc<T>`, and `Arc<T>`
impl<T> SporePrint for Box<T>
where
    T: SporePrint,
{
    fn spore_print(&self) -> String {
        self.as_ref().spore_print()
    }
}

impl<T> SporePrint for Rc<T>
where
    T: SporePrint,
{
    fn spore_print(&self) -> String {
        self.as_ref().spore_print()
    }
}

impl<T> SporePrint for Arc<T>
where
    T: SporePrint,
{
    fn spore_print(&self) -> String {
        self.as_ref().spore_print()
    }
}

#[cfg(test)]
mod core_tests {
    use super::*;
    use im::HashSet;
    use phf::phf_map;
    use std::fmt::Debug;
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
    fn test_vector_of_strings() {
        let vector = vector!["one".to_string(), "two".to_string(), "three".to_string()];
        assert_eq!(vector.spore_print(), "[one, two, three]");
    }

    /// Tests `SporePrint` implementation for `HashSet<i32>`
    #[test]
    fn test_im_hashset() {
        let set: HashSet<i32> = HashSet::from_iter(vector![1, 2, 3]);
        let actual: HashSet<String> = set
            .spore_print()
            .trim_start_matches("{")
            .trim_end_matches("}")
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        let expected: HashSet<String> =
            HashSet::from_iter(vector!["1".to_string(), "2".to_string(), "3".to_string()]);
        assert_eq!(
            actual, expected,
            "Serialized HashSet does not match expected values."
        );
    }

    #[test]
    fn test_nested_hashset() {
        use im::HashSet;

        // Create inner HashSets
        let inner1: HashSet<String> =
            HashSet::from_iter(vector!["1".to_string(), "2".to_string(), "3".to_string()]);
        let inner2: HashSet<String> =
            HashSet::from_iter(vector!["4".to_string(), "5".to_string(), "6".to_string()]);

        // Create the outer HashSet containing the inner HashSets
        let nested_set: HashSet<HashSet<String>> =
            HashSet::from_iter(vector![inner1.clone(), inner2.clone()]);

        // Expected serialized output
        let actual: HashSet<String> = nested_set
            .spore_print()
            .trim_start_matches("{")
            .trim_end_matches("}")
            .split("}, {")
            .map(|s| format!("{{{}}}", s))
            .collect();
        let expected: HashSet<String> =
            HashSet::from_iter(vector![inner1.spore_print(), inner2.spore_print()]);

        // Assert that the serialization matches the expected output
        assert_eq!(
            actual, expected,
            "Serialized nested HashSet does not match expected values."
        );
    }

    /// Tests `SporePrint` implementation for `phf::Map<&str, i32>`
    #[test]
    fn test_phf_map() {
        // Define a `phf::Map`
        static TEST_MAP: phf::Map<&'static str, i32> = phf_map! {
            "one" => 1,
            "two" => 2,
            "three" => 3,
        };

        // Expected output as a string
        let expected_output = "{one: 1, two: 2, three: 3}";

        // Assert the `spore_print` output matches the expected string
        assert_eq!(TEST_MAP.spore_print(), expected_output);
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

        let tuple_complex = (42, "hello", vector![1, 2, 3]);
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
        assert_eq!(tuple.spore_print(), "(42)");
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
    fn test_tuple_complex() {
        let vector = vector![1, 2, 3];
        let tuple_complex = (42, "hello", vector);
        assert_eq!(tuple_complex.spore_print(), "(42, hello, [1, 2, 3])");
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

        let value = vector![1, 2, 3];
        let reference: &Vector<i32> = &value;
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

    #[test]
    fn test_box() {
        let boxed = Box::new(42);
        assert_eq!(boxed.spore_print(), "42");

        let nested_boxed = Box::new(Box::new("hello"));
        assert_eq!(nested_boxed.spore_print(), "hello");
    }

    #[test]
    fn test_rc() {
        let rc = Rc::new(42);
        assert_eq!(rc.spore_print(), "42");

        let nested_rc = Rc::new(Rc::new("hello"));
        assert_eq!(nested_rc.spore_print(), "hello");
    }

    #[test]
    fn test_arc() {
        let arc = Arc::new(42);
        assert_eq!(arc.spore_print(), "42");

        let nested_arc = Arc::new(Arc::new("hello"));
        assert_eq!(nested_arc.spore_print(), "hello");
    }
    #[test]
    fn test_format_tuple() {
        use crate::format::format_tuple;
        use im::Vector;

        let items = vector!["42".to_string(), "hello".to_string(), "true".to_string(),];
        assert_eq!(format_tuple(items), "(42, hello, true)");

        let single_item = vector!["42".to_string()];
        assert_eq!(format_tuple(single_item), "(42)");

        let empty = Vector::<String>::new();
        assert_eq!(format_tuple(empty), "()");
    }

    #[test]
    fn test_format_struct() {
        use crate::format::format_struct;
        use im::Vector;

        let fields = vector!["42".to_string(), "hello".to_string()];
        assert_eq!(format_struct("MyStruct", fields), "MyStruct { 42, hello }");

        let no_fields = Vector::<String>::new();
        assert_eq!(format_struct("EmptyStruct", no_fields), "EmptyStruct { }");
    }

    #[test]
    fn test_format_enum() {
        use crate::format::format_enum;
        use im::Vector;

        let fields = vector!["42".to_string(), "hello".to_string()];
        assert_eq!(
            format_enum("MyEnum", "MyVariant", fields),
            "MyEnum::MyVariant(42, hello)"
        );

        let no_fields = Vector::<String>::new();
        assert_eq!(
            format_enum("MyEnum", "EmptyVariant", no_fields),
            "MyEnum::EmptyVariant"
        );
    }

    #[test]
    fn test_format_collection() {
        use crate::format::format_collection;

        let items = ["42".to_string(), "hello".to_string(), "true".to_string()];
        assert_eq!(format_collection(items.iter()), "[42, hello, true]");

        let empty: Vector<String> = Vector::new();
        assert_eq!(format_collection(empty.iter()), "[]");
    }

    use im::Vector;

    #[test]
    fn test_empty_vector() {
        let empty_vector: Vector<i32> = Vector::new();
        assert_eq!(empty_vector.spore_print(), "[]");
    }

    #[test]
    fn test_empty_hashmap() {
        let empty_map: HashMap<i32, i32> = HashMap::new();
        assert_eq!(empty_map.spore_print(), "{}");
    }
    #[test]
    fn test_empty_hashset() {
        let empty_set: HashSet<i32> = HashSet::new();
        assert_eq!(empty_set.spore_print(), "{}");
    }

    #[test]
    fn test_numeric_boundaries() {
        assert_eq!(i32::MIN.spore_print(), "-2147483648");
        assert_eq!(i32::MAX.spore_print(), "2147483647");
        assert_eq!(f32::INFINITY.spore_print(), "inf");
        assert_eq!(f32::NEG_INFINITY.spore_print(), "-inf");
        assert!(f32::NAN.spore_print().contains("NaN"));
    }

    pub fn assert_hashset_eq<T>(left: &HashSet<T>, right: &HashSet<T>)
    where
        T: Eq + std::hash::Hash + Debug,
    {
        assert_eq!(
            left, right,
            "HashSets are not equal: left = {:?}, right = {:?}",
            left, right
        );
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_assert_hashset_eq() {
            let set1: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
            let set2: HashSet<i32> = [3, 2, 1].iter().cloned().collect();
            assert_hashset_eq(&set1, &set2);
        }

        #[test]
        #[should_panic]
        fn test_assert_hashset_eq_fail() {
            let set1: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
            let set2: HashSet<i32> = [4, 5, 6].iter().cloned().collect();
            assert_hashset_eq(&set1, &set2);
        }
    }
    #[test]
    fn test_non_empty_hashmap() {
        let map: HashMap<i32, String> = HashMap::from_iter(vector![
            (1, "one".to_string()),
            (2, "two".to_string()),
            (3, "three".to_string()),
        ]);

        let binding = map.spore_print();
        let result: Vector<_> = binding
            .trim_matches(|c| c == '{' || c == '}')
            .split(", ")
            .collect();
        let expected = vector!["1: one", "2: two", "3: three"];

        assert!(result.len() == expected.len() && result.iter().all(|e| expected.contains(e)));
    }

    #[test]
    fn test_nested_hashmap() {
        let inner_map: HashMap<i32, String> =
            HashMap::from_iter(vector![(1, "one".to_string()), (2, "two".to_string())]);

        let outer_map: HashMap<i32, HashMap<i32, String>> =
            HashMap::from_iter(vector![(1, inner_map.clone()), (2, inner_map.clone())]);

        let result: Vector<_> = outer_map
            .spore_print()
            .trim_matches(|c| c == '{' || c == '}')
            .split("}, ")
            .map(|s| {
                if s.ends_with('}') {
                    s.to_string()
                } else {
                    format!("{}}}", s)
                }
            })
            .collect();

        let expected_variants = vector![
            vector![
                "1: {1: one, 2: two}".to_string(),
                "2: {1: one, 2: two}".to_string()
            ],
            vector![
                "1: {1: one, 2: two}".to_string(),
                "2: {2: two, 1: one}".to_string()
            ],
            vector![
                "1: {2: two, 1: one}".to_string(),
                "2: {1: one, 2: two}".to_string()
            ],
            vector![
                "1: {2: two, 1: one}".to_string(),
                "2: {2: two, 1: one}".to_string()
            ],
            vector![
                "2: {1: one, 2: two}".to_string(),
                "1: {1: one, 2: two}".to_string()
            ],
            vector![
                "2: {1: one, 2: two}".to_string(),
                "1: {2: two, 1: one}".to_string()
            ],
            vector![
                "2: {2: two, 1: one}".to_string(),
                "1: {1: one, 2: two}".to_string()
            ],
            vector![
                "2: {2: two, 1: one}".to_string(),
                "1: {2: two, 1: one}".to_string()
            ],
        ];

        assert!(expected_variants.iter().any(|expected| result == *expected));
    }

    #[test]
    fn test_hashmap_with_different_types() {
        let map: HashMap<&str, i32> =
            HashMap::from_iter(vector![("one", 1), ("two", 2), ("three", 3)]);

        let binding = map.spore_print();
        let result: Vector<_> = binding
            .trim_matches(|c| c == '{' || c == '}')
            .split(", ")
            .collect();
        let expected = vector!["one: 1", "two: 2", "three: 3"];

        assert!(result.len() == expected.len() && result.iter().all(|e| expected.contains(e)));
    }
    #[test]
    fn test_unicode_strings() {
        let unicode_str = "こんにちは";
        assert_eq!(unicode_str.spore_print(), "こんにちは");

        let escape_str = "Hello\nWorld";
        assert_eq!(escape_str.spore_print(), "Hello\nWorld");

        let emoji_str = "😊";
        assert_eq!(emoji_str.spore_print(), "😊");
    }
}
