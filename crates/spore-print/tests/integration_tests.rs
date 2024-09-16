use spore_print::{sprint, SporePrint};
use spore_print_derive::SporePrint;
use std::collections::{HashMap, HashSet};
use std::ops::Range;

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

/// A struct with primitive types.
#[derive(SporePrint)]
struct PrimitiveStruct {
    int_field: i32,
    float_field: f64,
    bool_field: bool,
}

/// Tests `PrimitiveStruct` to ensure the `spore_print` method correctly represents the struct.
#[test]
fn test_primitive_struct() {
    let test_struct = PrimitiveStruct {
        int_field: 42,
        float_field: std::f64::consts::PI,
        bool_field: true,
    };
    assert_eq!(
        test_struct.spore_print(),
        "PrimitiveStruct { int_field: 42, float_field: 3.141592653589793, bool_field: true }"
    );
}

/// A struct with reference and complex types.
#[derive(SporePrint)]
struct ComplexStruct<'a> {
    ref_field: &'a str,
    vec_field: Vec<i32>,
    option_field: Option<String>,
}

/// Tests `ComplexStruct` to ensure the `spore_print` method correctly represents the struct.
#[test]
fn test_complex_struct() {
    let test_struct: ComplexStruct = ComplexStruct {
        ref_field: "Hello",
        vec_field: vec![1i32, 2, 3],
        option_field: Some("World".to_string()),
    };
    assert_eq!(
        test_struct.spore_print(),
        "ComplexStruct { ref_field: Hello, vec_field: [1, 2, 3], option_field: Some(World) }"
    );
}

/// An enum with unit variants.
#[derive(SporePrint)]
enum UnitEnum {
    VariantA,
    VariantB,
}

/// Tests `UnitEnum` to ensure the `spore_print` method correctly represents the enum.
#[test]
fn test_unit_enum() {
    let variant = UnitEnum::VariantA;
    assert_eq!(variant.spore_print(), "UnitEnum::VariantA");
}

/// An enum with named fields.
#[derive(SporePrint)]
enum NamedEnum {
    VariantX { id: u32, name: String },
    VariantY { value: f64 },
}

/// Tests `NamedEnum` to ensure the `spore_print` method correctly represents the enum.
#[test]
fn test_named_enum() {
    let variant = NamedEnum::VariantX {
        id: 1,
        name: "Alice".to_string(),
    };
    assert_eq!(
        variant.spore_print(),
        "NamedEnum::VariantX { id: 1, name: Alice }"
    );
}

/// An enum with unnamed fields.
#[derive(SporePrint)]
enum UnnamedEnum {
    Variant1(i32, String),
    Variant2(f64, bool),
}

/// Tests `UnnamedEnum` to ensure the `spore_print` method correctly represents the enum.
#[test]
fn test_unnamed_enum() {
    let variant = UnnamedEnum::Variant1(42, "Answer".to_string());
    assert_eq!(variant.spore_print(), "UnnamedEnum::Variant1(42, Answer)");
}

/// An edge case: Empty struct.
#[derive(SporePrint)]
struct EmptyStruct;

/// Tests `EmptyStruct` to ensure the `spore_print` method correctly represents the struct.
#[test]
fn test_empty_struct() {
    let instance = EmptyStruct;
    assert_eq!(instance.spore_print(), "EmptyStruct");
}

/// Tests various enum constructions to ensure they compile and are correctly represented.
#[test]
fn test_enum_constructions() {
    // UnitEnum usage
    let _ = UnitEnum::VariantA;
    let _ = UnitEnum::VariantB;

    // NamedEnum usage
    let _ = NamedEnum::VariantX {
        id: 1,
        name: "Test".to_string(),
    };
    let _ = NamedEnum::VariantY {
        value: std::f64::consts::PI,
    };

    // UnnamedEnum usage
    let _ = UnnamedEnum::Variant1(42, "Hello".to_string());
    let _ = UnnamedEnum::Variant2(2.71, true);
}

/// A struct with slices.
#[derive(SporePrint)]
struct SliceStruct<'a> {
    slice_field: &'a [i32],
}

/// Tests `SliceStruct` to ensure the `spore_print` method correctly represents the struct.
#[test]
fn test_slice_struct() {
    let test_struct = SliceStruct {
        slice_field: &[1, 2, 3],
    };
    assert_eq!(
        test_struct.spore_print(),
        "SliceStruct { slice_field: [1, 2, 3] }"
    );
}

/// A struct with arrays.
#[derive(SporePrint)]
struct ArrayStruct {
    array_field: [i32; 3],
}

/// Tests `ArrayStruct` to ensure the `spore_print` method correctly represents the struct.
#[test]
fn test_array_struct() {
    let test_struct = ArrayStruct {
        array_field: [1, 2, 3],
    };
    assert_eq!(
        test_struct.spore_print(),
        "ArrayStruct { array_field: [1, 2, 3] }"
    );
}

/// A struct with references.
#[derive(SporePrint)]
struct ReferenceStruct<'a> {
    ref_field: &'a i32,
}

/// Tests `ReferenceStruct` to ensure the `spore_print` method correctly represents the struct.
#[test]
fn test_reference_struct() {
    let value = 42;
    let test_struct = ReferenceStruct { ref_field: &value };
    assert_eq!(
        test_struct.spore_print(),
        "ReferenceStruct { ref_field: 42 }"
    );
}

/// A struct with `Result<T, E>`.
#[derive(SporePrint)]
struct ResultStruct {
    result_field: Result<i32, &'static str>,
}

/// Tests `ResultStruct` to ensure the `spore_print` method correctly represents the struct.
#[test]
fn test_result_struct() {
    let test_struct_ok = ResultStruct {
        result_field: Ok(42),
    };
    assert_eq!(
        test_struct_ok.spore_print(),
        "ResultStruct { result_field: Ok(42) }"
    );

    let test_struct_err = ResultStruct {
        result_field: Err("error"),
    };
    assert_eq!(
        test_struct_err.spore_print(),
        "ResultStruct { result_field: Err(error) }"
    );
}

/// A struct with `Range<usize>`.
#[derive(SporePrint)]
struct RangeStruct {
    range_field: Range<usize>,
}

/// Tests `RangeStruct` to ensure the `spore_print` method correctly represents the struct.
#[test]
fn test_range_struct() {
    let test_struct = RangeStruct { range_field: 3..10 };
    assert_eq!(
        test_struct.spore_print(),
        "RangeStruct { range_field: 3..10 }"
    );
}

/// A struct with generic `Range<T>`.
#[derive(SporePrint)]
struct GenericRangeStruct<T: SporePrint> {
    range_field: Range<T>,
}

/// Tests `GenericRangeStruct` with `usize` to ensure the `spore_print` method correctly represents the struct.
#[test]
fn test_generic_range_struct_usize() {
    let test_struct = GenericRangeStruct { range_field: 3..10 };
    assert_eq!(
        test_struct.spore_print(),
        "GenericRangeStruct { range_field: 3..10 }"
    );
}

/// Tests `GenericRangeStruct` with `f32` to ensure the `spore_print` method correctly represents the struct.
#[test]
fn test_generic_range_struct_f32() {
    let test_struct = GenericRangeStruct {
        range_field: 1.5..4.5,
    };
    assert_eq!(
        test_struct.spore_print(),
        "GenericRangeStruct { range_field: 1.5..4.5 }"
    );
}

/// Tests `GenericRangeStruct` with `String` to ensure the `spore_print` method correctly represents the struct.
#[test]
fn test_generic_range_struct_string() {
    let test_struct = GenericRangeStruct {
        range_field: "a".to_string().."z".to_string(),
    };
    assert_eq!(
        test_struct.spore_print(),
        "GenericRangeStruct { range_field: a..z }"
    );
}

/// A struct with a tuple field.
#[derive(SporePrint)]
struct TupleStruct {
    tuple_field: (i32, &'static str, Option<f64>),
}

/// Tests `TupleStruct` to ensure the `spore_print` method correctly represents the struct.
#[test]
fn test_tuple_struct() {
    let test_struct = TupleStruct {
        tuple_field: (42, "hello", Some(std::f64::consts::PI)),
    };
    assert_eq!(
        test_struct.spore_print(),
        "TupleStruct { tuple_field: (42, hello, Some(3.141592653589793)) }"
    );
}

/// A struct with a vector field.
#[derive(SporePrint)]
struct VecStruct {
    vec_field: Vec<String>,
}

/// Tests `VecStruct` to ensure the `spore_print` method correctly represents the struct.
#[test]
fn test_vec_struct() {
    let test_struct = VecStruct {
        vec_field: vec!["one".to_string(), "two".to_string(), "three".to_string()],
    };
    assert_eq!(
        test_struct.spore_print(),
        "VecStruct { vec_field: [one, two, three] }"
    );
}

/// A struct with a `HashSet` field.
#[derive(SporePrint)]
struct HashSetStruct {
    set_field: HashSet<i32>,
}

/// Tests `HashSetStruct` to ensure the `spore_print` method correctly represents the struct.
#[test]
fn test_hashset_struct() {
    let test_struct = HashSetStruct {
        set_field: HashSet::from([1, 2]),
    };
    let expected: HashSet<String> = HashSet::from_iter(vec!["1".to_string(), "2".to_string()]);
    let actual: HashSet<String> = HashSet::from_iter(
        test_struct
            .spore_print()
            .trim_start_matches("HashSetStruct { set_field: [")
            .trim_end_matches("] }")
            .split(", ")
            .map(|s| s.trim().to_string()),
    );
    assert_eq!(actual, expected);
}

#[derive(SporePrint)]
struct TestStruct {
    field1: i32,
    field2: String,
}

/// Tests `TestStruct` to ensure the `spore_print` method correctly represents the struct.
#[test]
fn test_struct_spore_print() {
    let instance = TestStruct {
        field1: 42,
        field2: "hello".to_string(),
    };
    assert_eq!(
        instance.spore_print(),
        "TestStruct { field1: 42, field2: hello }"
    );
}
/// An enum with unit variants.
#[derive(SporePrint)]
enum TestEnum {
    Variant1,
    Variant2(i32),
    Variant3 { field: String },
}

/// Tests `TestEnum` to ensure the `spore_print` method correctly represents the enum.
#[test]
fn test_enum_spore_print() {
    let instance1 = TestEnum::Variant1;
    let sprint_value1 = sprint!(instance1);
    assert_eq!(sprint_value1, "TestEnum::Variant1");

    let instance2 = TestEnum::Variant2(42);
    let sprint_value2 = sprint!(instance2);
    assert_eq!(sprint_value2, "TestEnum::Variant2(42)");

    let instance3 = TestEnum::Variant3 {
        field: "hello".to_string(),
    };
    let sprint_value3 = sprint!(instance3);
    assert_eq!(sprint_value3, "TestEnum::Variant3 { field: hello }");
}
