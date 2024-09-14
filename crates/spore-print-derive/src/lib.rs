// crates/spore-print-derive/src/lib.rs
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(SporePrint)]
pub fn spore_print_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Generate the implementation by delegating to the helper function
    let expanded = impl_spore_print(&input);

    // Return the generated code as a TokenStream
    TokenStream::from(expanded)
}

fn impl_spore_print(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

    match &input.data {
        Data::Struct(data_struct) => impl_spore_print_for_struct(name, data_struct),
        Data::Enum(data_enum) => impl_spore_print_for_enum(name, data_enum),
        _ => unimplemented!("SporePrint can only be derived for structs and enums"),
    }
}

fn impl_spore_print_for_struct(
    name: &syn::Ident,
    data_struct: &syn::DataStruct,
) -> proc_macro2::TokenStream {
    let field_names: Vec<syn::Ident> = match &data_struct.fields {
        Fields::Named(fields_named) => fields_named
            .named
            .iter()
            .map(|f| f.ident.clone().unwrap())
            .collect(),
        Fields::Unnamed(fields_unnamed) => (0..fields_unnamed.unnamed.len())
            .map(|i| syn::Ident::new(&format!("_{}", i), proc_macro2::Span::call_site()))
            .collect(),
        Fields::Unit => Vec::new(),
    };

    let field_accessors = field_names
        .iter()
        .map(|ident| quote! { self.#ident.spore_print() });
    let field_strings = field_names.iter().map(|ident| ident.to_string());

    let fields_fmt = quote! {
        vec![
            #(
                format!("{}: {}", #field_strings, #field_accessors)
            ),*
        ].join(", ")
    };

    quote! {
        impl spore_print::SporePrint for #name {
            fn spore_print(&self) -> String {
                let fields = #fields_fmt;
                format!("{} {{ {} }}", stringify!(#name), fields)
            }
        }
    }
}

fn impl_spore_print_for_enum(
    name: &syn::Ident,
    data_enum: &syn::DataEnum,
) -> proc_macro2::TokenStream {
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
        impl spore_print::SporePrint for #name {
            fn spore_print(&self) -> String {
                match self {
                    #(#variant_matches),*
                }
            }
        }
    }
}
