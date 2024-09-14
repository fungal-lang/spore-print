use std::collections::{HashMap, HashSet};

// SporePrint trait definition
pub trait SporePrint {
    fn spore_print(&self) -> String;
}

// Macro to implement SporePrint for types that implement Display
macro_rules! impl_spore_print_for_display {
    ($($t:ty),*) => {
        $(impl SporePrint for $t {
            fn spore_print(&self) -> String {
                self.to_string()
            }
        })*
    };
}

// Implement SporePrint for all types that implement Display using the macro
impl_spore_print_for_display!(
    u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64, String, &str, char, bool
);

// Implement SporePrint for Option<T> using recursion to unwrap nested options
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

// Macro to implement SporePrint for collections that implement IntoIterator
macro_rules! impl_spore_print_for_collections {
    ($($t:ty),*) => {
        $(
            impl<T> SporePrint for $t
            where
                T: SporePrint,
            {
                fn spore_print(&self) -> String {
                    let items = self
                        .into_iter()
                        .map(|item| item.spore_print())
                        .collect::<Vec<_>>(); // Collect as Vec

                    format!("[{}]", items.join(", "))
                }
            }
        )*
    };
}

// Implement SporePrint for Vec, HashSet, and other collections
impl_spore_print_for_collections!(Vec<T>, HashSet<T>);

// Implement SporePrint for HashMap without assuming a specific order
impl<K, V> SporePrint for HashMap<K, V>
where
    K: SporePrint,
    V: SporePrint,
{
    fn spore_print(&self) -> String {
        let items = self
            .iter()
            .map(|(key, value)| format!("{}: {}", key.spore_print(), value.spore_print()))
            .collect::<Vec<_>>(); // Collect as Vec

        format!("{{{}}}", items.join(", "))
    }
}

// Implement SporePrint for tuples of various arities (up to 3 for brevity; can be extended)
impl<T1, T2> SporePrint for (T1, T2)
where
    T1: SporePrint,
    T2: SporePrint,
{
    fn spore_print(&self) -> String {
        format!("({}, {})", self.0.spore_print(), self.1.spore_print())
    }
}

impl<T1, T2, T3> SporePrint for (T1, T2, T3)
where
    T1: SporePrint,
    T2: SporePrint,
    T3: SporePrint,
{
    fn spore_print(&self) -> String {
        format!(
            "({}, {}, {})",
            self.0.spore_print(),
            self.1.spore_print(),
            self.2.spore_print()
        )
    }
}

// Additional implementations for more tuple sizes can be added here as needed

// Helper function to format items
fn format_items<T: SporePrint>(items: &[T]) -> String {
    let formatted_items = items
        .iter()
        .map(|item| item.spore_print())
        .collect::<Vec<_>>();
    format!("[{}]", formatted_items.join(", "))
}

// Implement SporePrint for slices
impl<T> SporePrint for &[T]
where
    T: SporePrint,
{
    fn spore_print(&self) -> String {
        format_items(self)
    }
}

// Implement SporePrint for arrays
impl<T, const N: usize> SporePrint for [T; N]
where
    T: SporePrint,
{
    fn spore_print(&self) -> String {
        format_items(self)
    }
}

// Implement SporePrint for references
impl<T> SporePrint for &T
where
    T: SporePrint,
{
    fn spore_print(&self) -> String {
        (*self).spore_print()
    }
}

// Implement SporePrint for Result<T, E>
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};
    use std::iter::FromIterator;

    #[test]
    fn test_string() {
        let value = String::from("test");
        assert_eq!(value.spore_print(), "test");
    }

    #[test]
    fn test_str() {
        let value: &str = "test";
        assert_eq!(value.spore_print(), "test");
    }

    #[test]
    fn test_integer() {
        let value = 42;
        assert_eq!(value.spore_print(), "42");
    }

    #[test]
    fn test_vec_of_strings() {
        let vec = vec!["one".to_string(), "two".to_string(), "three".to_string()];
        assert_eq!(vec.spore_print(), "[one, two, three]"); // Corrected to match the original order
    }

    #[test]
    fn test_option_some() {
        let value = Some(42);
        assert_eq!(value.spore_print(), "Some(42)");
    }

    #[test]
    fn test_option_none() {
        let value: Option<i32> = None;
        assert_eq!(value.spore_print(), "None");
    }

    #[test]
    fn test_nested_option() {
        let nested_option: Option<Option<i32>> = Some(Some(42));
        assert_eq!(nested_option.spore_print(), "Some(Some(42))");
    }

    #[test]
    fn test_complex_option() {
        let complex_option: Option<Vec<Option<i32>>> = Some(vec![Some(1), None, Some(3)]);
        assert_eq!(
            complex_option.spore_print(),
            "Some([Some(1), None, Some(3)])"
        );
    }

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

    #[test]
    fn test_tuple() {
        let tuple = (42, "hello", Some(3.14));
        assert_eq!(tuple.spore_print(), "(42, hello, Some(3.14))");
    }

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

    #[test]
    fn test_slice() {
        let slice: &[i32] = &[1, 2, 3];
        assert_eq!(slice.spore_print(), "[1, 2, 3]");
    }

    #[test]
    fn test_array() {
        let array: [i32; 3] = [1, 2, 3];
        assert_eq!(array.spore_print(), "[1, 2, 3]");
    }

    #[test]
    fn test_reference() {
        let value = 42;
        let reference: &i32 = &value;
        assert_eq!(reference.spore_print(), "42");
    }

    #[test]
    fn test_result_ok() {
        let result: Result<i32, &str> = Ok(42);
        assert_eq!(result.spore_print(), "Ok(42)");
    }

    #[test]
    fn test_result_err() {
        let result: Result<i32, &str> = Err("error");
        assert_eq!(result.spore_print(), "Err(error)");
    }
}
