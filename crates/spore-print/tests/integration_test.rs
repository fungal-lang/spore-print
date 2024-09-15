use spore_print::SporePrint;
use std::collections::{HashMap, HashSet};

/// Tests nested vectors to ensure the `spore_print` method preserves the original order.
#[test]
fn test_nested_vecs() {
    let nested_vec = vec![vec![1, 2], vec![3, 4]];
    assert_eq!(nested_vec.spore_print(), "[[1, 2], [3, 4]]");
}

/// Tests complex options to ensure the `spore_print` method matches the deeply nested structure.
#[test]
fn test_complex_option() {
    let complex_option: Option<Vec<Option<i32>>> = Some(vec![Some(1), None, Some(3)]);
    assert_eq!(
        complex_option.spore_print(),
        "Some([Some(1), None, Some(3)])"
    );
}

/// Tests deeply nested vectors of options to ensure the `spore_print` method matches the structure.
#[test]
fn test_deeply_nested_vec_option() {
    let nested_vec = vec![
        Some(vec![Some(1), None, Some(3)]),
        None,
        Some(vec![None, Some(4)]),
    ];
    assert_eq!(
        nested_vec.spore_print(),
        "[Some([Some(1), None, Some(3)]), None, Some([None, Some(4)])]"
    );
}

/// Tests `HashMap` with `SporePrint` to ensure the `spore_print` method correctly represents the map.
#[test]
fn test_hashmap_membership() {
    let map: HashMap<&str, i32> = [("key1", 1), ("key2", 2)].iter().cloned().collect();

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

/// Tests `Range<usize>` with `SporePrint` to ensure the `spore_print` method correctly represents the range.
#[test]
fn test_range_usize() {
    let range = 3..10;
    assert_eq!(range.spore_print(), "3..10");
}

/// Tests `Range<f32>` with `SporePrint` to ensure the `spore_print` method correctly represents the range.
#[test]
fn test_range_f32() {
    let range = 1.5..4.5;
    assert_eq!(range.spore_print(), "1.5..4.5");
}

/// Tests `Range<String>` with `SporePrint` to ensure the `spore_print` method correctly represents the range.
#[test]
fn test_range_string() {
    let range = "a".to_string().."z".to_string();
    assert_eq!(range.spore_print(), "a..z");
}

/// Tests tuple with `SporePrint` to ensure the `spore_print` method correctly represents the tuple.
#[test]
fn test_tuple() {
    let tuple = (42, "hello", Some(std::f64::consts::PI));
    assert_eq!(tuple.spore_print(), "(42, hello, Some(3.141592653589793))");
}

/// Tests vector of strings with `SporePrint` to ensure the `spore_print` method correctly represents the vector.
#[test]
fn test_vec_of_strings() {
    let vec = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    assert_eq!(vec.spore_print(), "[one, two, three]");
}

/// Tests `HashSet` with `SporePrint` to ensure the `spore_print` method correctly represents the set.
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
