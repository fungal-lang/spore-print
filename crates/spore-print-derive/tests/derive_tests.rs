use spore_print::SporePrint;
use spore_print_derive::SporePrint;

// Struct with primitive types
#[derive(SporePrint)]
struct PrimitiveStruct {
    int_field: i32,
    float_field: f64,
    bool_field: bool,
}

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

// Struct with reference and complex types
#[derive(SporePrint)]
struct ComplexStruct<'a> {
    ref_field: &'a str,
    vec_field: Vec<i32>,
    option_field: Option<String>,
}

#[test]
fn test_complex_struct() {
    let test_struct: ComplexStruct = ComplexStruct {
        ref_field: "Hello",
        vec_field: vec![1i32, 2, 3], // Explicit type annotation for the vector elements
        option_field: Some("World".to_string()),
    };
    assert_eq!(
        test_struct.spore_print(),
        "ComplexStruct { ref_field: Hello, vec_field: [1, 2, 3], option_field: Some(World) }"
    );
}

// Enum with unit variants
#[derive(SporePrint)]
enum UnitEnum {
    VariantA,
    VariantB,
}

#[test]
fn test_unit_enum() {
    let variant = UnitEnum::VariantA;
    assert_eq!(variant.spore_print(), "UnitEnum::VariantA");
}

// Enum with named fields
#[derive(SporePrint)]
enum NamedEnum {
    VariantX { id: u32, name: String },
    VariantY { value: f64 },
}

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

// Enum with unnamed fields
#[derive(SporePrint)]
enum UnnamedEnum {
    Variant1(i32, String),
    Variant2(f64, bool),
}

#[test]
fn test_unnamed_enum() {
    let variant = UnnamedEnum::Variant1(42, "Answer".to_string());
    assert_eq!(variant.spore_print(), "UnnamedEnum::Variant1(42, Answer)");
}

// Edge case: Empty struct
#[derive(SporePrint)]
struct EmptyStruct;

#[test]
fn test_empty_struct() {
    let instance = EmptyStruct;
    assert_eq!(instance.spore_print(), "EmptyStruct");
}

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

// Struct with slices
#[derive(SporePrint)]
struct SliceStruct<'a> {
    slice_field: &'a [i32],
}

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

// Struct with arrays
#[derive(SporePrint)]
struct ArrayStruct {
    array_field: [i32; 3],
}

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

// Struct with references
#[derive(SporePrint)]
struct ReferenceStruct<'a> {
    ref_field: &'a i32,
}

#[test]
fn test_reference_struct() {
    let value = 42;
    let test_struct = ReferenceStruct { ref_field: &value };
    assert_eq!(
        test_struct.spore_print(),
        "ReferenceStruct { ref_field: 42 }"
    );
}

// Struct with Result<T, E>
#[derive(SporePrint)]
struct ResultStruct {
    result_field: Result<i32, &'static str>,
}

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
