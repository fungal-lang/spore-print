use spore_print::SporePrint;
use std::collections::{HashMap, HashSet};

#[test]
fn test_nested_vecs() {
    let nested_vec = vec![vec![1, 2], vec![3, 4]];
    assert_eq!(nested_vec.spore_print(), "[[1, 2], [3, 4]]"); // Preserves the original order
}

#[test]
fn test_complex_option() {
    let complex_option: Option<Vec<Option<i32>>> = Some(vec![Some(1), None, Some(3)]);
    assert_eq!(
        complex_option.spore_print(),
        "Some([Some(1), None, Some(3)])"
    );
}

// Test deeply nested Vec<Option<T>>
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
    ); // Matches the deeply nested structure
}

// Test HashMap with SporePrint
#[test]
fn test_hashmap_membership() {
    let mut map = HashMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);

    // Convert the output of spore_print to a set for membership comparison
    let expected: HashSet<String> =
        HashSet::from_iter(vec!["key1: 1".to_string(), "key2: 2".to_string()]);
    let actual: HashSet<String> = HashSet::from_iter(
        map.spore_print()
            .trim_matches(|c| c == '{' || c == '}')
            .split(", ")
            .map(|s| s.to_string()),
    );

    assert_eq!(actual, expected); // Corrected to use membership comparison
}
