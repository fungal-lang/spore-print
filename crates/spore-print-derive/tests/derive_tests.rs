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
