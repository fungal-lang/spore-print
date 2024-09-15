use spore_print::SporePrint;
// Import trait from the `spore-print` crate.
use spore_print_derive::SporePrint;
// Import the procedural macro from `spore-print-derive` crate.

/// Tests `SporePrint` derivation for unit structs.
#[test]
fn test_unit_struct() {
    #[derive(SporePrint)]
    struct UnitStruct;

    let instance = UnitStruct;
    assert_eq!(instance.spore_print(), "UnitStruct");
}

/// Tests `SporePrint` derivation for a struct with named fields.
#[test]
fn test_named_fields_struct() {
    #[derive(SporePrint)]
    struct NamedFieldsStruct {
        field1: i32,
        field2: String,
    }

    let instance = NamedFieldsStruct {
        field1: 42,
        field2: "hello".to_string(),
    };
    assert_eq!(
        instance.spore_print(),
        "NamedFieldsStruct { field1: 42, field2: hello }"
    );
}

/// Tests `SporePrint` derivation for a struct with unnamed (tuple) fields.
#[test]
fn test_tuple_struct() {
    #[derive(SporePrint)]
    struct TupleStruct(i32, String);

    let instance = TupleStruct(42, "hello".to_string());
    assert_eq!(instance.spore_print(), "TupleStruct(42, hello)");
}

/// Tests `SporePrint` derivation for a struct with generic fields.
#[test]
fn test_generic_struct() {
    #[derive(SporePrint)]
    struct GenericStruct<T: SporePrint> {
        // Added trait bound for `SporePrint`
        field: T,
    }

    let instance = GenericStruct { field: 42 };
    assert_eq!(instance.spore_print(), "GenericStruct { field: 42 }");
}

/// Tests `SporePrint` derivation for unit enums.
#[test]
fn test_unit_enum() {
    #[derive(SporePrint)]
    enum UnitEnum {
        Variant1,
        Variant2,
    }

    let instance = UnitEnum::Variant1;
    assert_eq!(instance.spore_print(), "UnitEnum::Variant1");

    let instance = UnitEnum::Variant2;
    assert_eq!(instance.spore_print(), "UnitEnum::Variant2");
}

/// Tests `SporePrint` derivation for enums with named fields.
#[test]
fn test_enum_with_named_fields() {
    #[derive(SporePrint)]
    enum NamedEnum {
        Variant1 { field1: i32 },
        Variant2 { field2: String },
    }

    let instance = NamedEnum::Variant1 { field1: 42 };
    assert_eq!(instance.spore_print(), "NamedEnum::Variant1 { field1: 42 }");

    let instance = NamedEnum::Variant2 {
        field2: "hello".to_string(),
    };
    assert_eq!(
        instance.spore_print(),
        "NamedEnum::Variant2 { field2: hello }"
    );
}

/// Tests `SporePrint` derivation for enums with unnamed (tuple) fields.
#[test]
fn test_enum_with_unnamed_fields() {
    #[derive(SporePrint)]
    enum UnnamedEnum {
        Variant1(i32),
        Variant2(String),
    }

    let instance = UnnamedEnum::Variant1(42);
    assert_eq!(instance.spore_print(), "UnnamedEnum::Variant1(42)");

    let instance = UnnamedEnum::Variant2("hello".to_string());
    assert_eq!(instance.spore_print(), "UnnamedEnum::Variant2(hello)");
}

/// Tests `SporePrint` derivation for an enum with a mix of unit, named, and unnamed variants.
#[test]
fn test_mixed_enum_variants() {
    #[derive(SporePrint)]
    enum MixedEnum {
        UnitVariant,
        NamedVariant { field1: i32 },
        UnnamedVariant(String),
    }

    let instance = MixedEnum::UnitVariant;
    assert_eq!(instance.spore_print(), "MixedEnum::UnitVariant");

    let instance = MixedEnum::NamedVariant { field1: 42 };
    assert_eq!(
        instance.spore_print(),
        "MixedEnum::NamedVariant { field1: 42 }"
    );

    let instance = MixedEnum::UnnamedVariant("hello".to_string());
    assert_eq!(instance.spore_print(), "MixedEnum::UnnamedVariant(hello)");
}
