use spore_print::SporePrint;
use spore_print_derive::SporePrint;
use std::collections::HashSet;
use std::ops::Range;

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
        float_field: 3.14,
        bool_field: true,
    };
    assert_eq!(
        test_struct.spore_print(),
        "PrimitiveStruct { int_field: 42, float_field: 3.14, bool_field: true }"
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
    let _ = NamedEnum::VariantY { value: 3.14 };

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
        tuple_field: (42, "hello", Some(3.14)),
    };
    assert_eq!(
        test_struct.spore_print(),
        "TupleStruct { tuple_field: (42, hello, Some(3.14)) }"
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