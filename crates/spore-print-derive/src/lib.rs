// crates/spore-print-derive/src/lib.rs

//! This crate provides a procedural macro to derive the `SporePrint` trait,
//! which allows for custom string representations of structs and enums.
//!
//! # Examples
//!
//! ```
//! use spore_print::SporePrint;
//! use spore_print_derive::SporePrint;
//!
//! #[derive(SporePrint)]
//! struct MyStruct {
//!     field1: i32,
//!     field2: String,
//! }
//!
//! let instance = MyStruct {
//!     field1: 42,
//!     field2: "hello".to_string(),
//! };
//! assert_eq!(instance.spore_print(), "MyStruct { field1: 42, field2: hello }");
//! ```
//!
//! ```
//! use spore_print::SporePrint;
//! use spore_print_derive::SporePrint;
//!
//! #[derive(SporePrint)]
//! enum MyEnum {
//!     Variant1,
//!     Variant2(i32),
//!     Variant3 { field: String },
//! }
//!
//! let instance1 = MyEnum::Variant1;
//! assert_eq!(instance1.spore_print(), "MyEnum::Variant1");
//!
//! let instance2 = MyEnum::Variant2(42);
//! assert_eq!(instance2.spore_print(), "MyEnum::Variant2(42)");
//!
//! let instance3 = MyEnum::Variant3 { field: "hello".to_string() };
//! assert_eq!(instance3.spore_print(), "MyEnum::Variant3 { field: hello }"); // Regular comment
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, WhereClause};

/// Derives the `SporePrint` trait for a struct or enum.
#[proc_macro_derive(SporePrint)]
pub fn spore_print_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Generate the implementation by delegating to the helper function
    let expanded = impl_spore_print(&input);

    // Return the generated code as a TokenStream
    TokenStream::from(expanded)
}

/// Generates the implementation of the `SporePrint` trait for the given input.
fn impl_spore_print(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    match &input.data {
        Data::Struct(data_struct) => impl_spore_print_for_struct(
            name,
            &impl_generics,
            &ty_generics,
            where_clause,
            data_struct,
        ),
        Data::Enum(data_enum) => {
            impl_spore_print_for_enum(name, &impl_generics, &ty_generics, where_clause, data_enum)
        }
        _ => unimplemented!("SporePrint can only be derived for structs and enums"),
    }
}

/// Generates the `SporePrint` implementation for a struct.
fn impl_spore_print_for_struct(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&WhereClause>,
    data_struct: &syn::DataStruct,
) -> proc_macro2::TokenStream {
    let fields_fmt = match &data_struct.fields {
        Fields::Named(fields_named) => {
            let field_names: Vec<syn::Ident> = fields_named
                .named
                .iter()
                .map(|f| f.ident.clone().unwrap())
                .collect();
            let field_accessors = field_names
                .iter()
                .map(|ident| quote! { self.#ident.spore_print() });
            let field_strings = field_names.iter().map(|ident| ident.to_string());

            quote! {
                vec![
                    #(
                        format!("{}: {}", #field_strings, #field_accessors)
                    ),*
                ].join(", ")
            }
        }
        Fields::Unnamed(fields_unnamed) => {
            let field_indices: Vec<syn::Index> = (0..fields_unnamed.unnamed.len())
                .map(syn::Index::from)
                .collect();
            let field_accessors = field_indices
                .iter()
                .map(|index| quote! { self.#index.spore_print() });

            quote! {
                vec![
                    #(
                        #field_accessors
                    ),*
                ].join(", ")
            }
        }
        Fields::Unit => quote! { String::new() },
    };

    // For unnamed (tuple) structs, format without named fields.
    let format_string = match &data_struct.fields {
        Fields::Named(_) => quote! { format!("{} {{ {} }}", stringify!(#name), fields) },
        Fields::Unnamed(_) => quote! { format!("{}({})", stringify!(#name), fields) },
        Fields::Unit => quote! { stringify!(#name).to_string() },
    };

    quote! {
        impl #impl_generics spore_print::SporePrint for #name #ty_generics #where_clause {
            fn spore_print(&self) -> String {
                let fields = #fields_fmt;
                #format_string
            }
        }
    }
}

/// Generates the `SporePrint` implementation for an enum.
fn impl_spore_print_for_enum(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&WhereClause>,
    data_enum: &syn::DataEnum,
) -> proc_macro2::TokenStream {
    if data_enum.variants.is_empty() {
        return quote! {
            impl #impl_generics spore_print::SporePrint for #name #ty_generics #where_clause {
                fn spore_print(&self) -> String {
                    panic!("Cannot print an instance of an empty enum {}", stringify!(#name))
                }
            }
        };
    }

    let variant_matches = data_enum.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let variant_name = variant_ident.to_string();

        match &variant.fields {
            Fields::Named(fields_named) => {
                let field_idents: Vec<_> = fields_named
                    .named
                    .iter()
                    .map(|f| f.ident.clone().unwrap())
                    .collect();
                let field_patterns = quote! { { #(#field_idents),* } };
                let field_accessors = field_idents
                    .iter()
                    .map(|ident| quote! { #ident.spore_print() });
                let field_strings = field_idents.iter().map(|ident| ident.to_string());

                let fields_fmt = quote! {
                    vec![
                        #(
                            format!("{}: {}", #field_strings, #field_accessors)
                        ),*
                    ].join(", ")
                };

                quote! {
                    #name::#variant_ident #field_patterns => {
                        let fields = #fields_fmt;
                        format!("{}::{} {{ {} }}", stringify!(#name), #variant_name, fields)
                    }
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                let field_count = fields_unnamed.unnamed.len();
                let field_idents: Vec<_> = (0..field_count)
                    .map(|i| {
                        syn::Ident::new(&format!("field{}", i), proc_macro2::Span::call_site())
                    })
                    .collect();
                let field_patterns = quote! { ( #(#field_idents),* ) };
                let field_accessors = field_idents
                    .iter()
                    .map(|ident| quote! { #ident.spore_print() });

                let fields_fmt = quote! {
                    vec![
                        #(
                            #field_accessors
                        ),*
                    ].join(", ")
                };

                quote! {
                    #name::#variant_ident #field_patterns => {
                        let fields = #fields_fmt;
                        format!("{}::{}({})", stringify!(#name), #variant_name, fields)
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    #name::#variant_ident => {
                        format!("{}::{}", stringify!(#name), #variant_name)
                    }
                }
            }
        }
    });

    quote! {
        impl #impl_generics spore_print::SporePrint for #name #ty_generics #where_clause {
            fn spore_print(&self) -> String {
                match self {
                    #(#variant_matches),*
                }
            }
        }
    }
}
