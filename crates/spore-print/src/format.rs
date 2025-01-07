use crate::SporePrint;
use im::Vector;

/// Helper function to join items with a separator in a functional style.
pub(crate) fn join_items(items: impl Iterator<Item = String>, separator: &str) -> String {
    items.fold(String::new(), |acc, item| {
        if acc.is_empty() {
            item
        } else {
            format!("{}{}{}", acc, separator, item)
        }
    })
}

/// Formats a collection as `[item1, item2, ...]`.
pub fn format_collection<T: SporePrint>(items: impl Iterator<Item = T>) -> String {
    items
        .map(|item| item.spore_print())
        .collect::<Vector<_>>()
        .pipe(|v| format!("[{}]", join_items(v.into_iter(), ", ")))
}

/// Formats a set as `{item1, item2, ...}`.
pub fn format_set<T: SporePrint>(items: impl Iterator<Item = T>) -> String {
    let serialized_items: Vector<String> = items.map(|item| item.spore_print()).collect();
    format!("{{{}}}", join_items(serialized_items.into_iter(), ", "))
}

/// Formats a map as `{key1: value1, key2: value2, ...}`.
pub fn format_map<K: SporePrint, V: SporePrint>(entries: impl Iterator<Item = (K, V)>) -> String {
    entries
        .map(|(key, value)| format!("{}: {}", key.spore_print(), value.spore_print()))
        .collect::<Vector<_>>()
        .pipe(|v| format!("{{{}}}", join_items(v.into_iter(), ", ")))
}

/// Formats a tuple as `(item1, item2, ...)`.
pub fn format_tuple(items: Vector<String>) -> String {
    format!("({})", join_items(items.into_iter(), ", "))
}

/// Formats a struct as `StructName { field1: value1, field2: value2, ... }`.
#[allow(dead_code)]
pub fn format_struct(name: &str, fields: Vector<String>) -> String {
    join_items(fields.into_iter(), ", ").pipe(|formatted| {
        if formatted.is_empty() {
            format!("{} {{ }}", name)
        } else {
            format!("{} {{ {} }}", name, formatted)
        }
    })
}

/// Formats an enum variant as `EnumName::VariantName(field1, field2, ...)`.
pub fn format_enum(name: &str, variant: &str, fields: Vector<String>) -> String {
    let formatted = join_items(fields.into_iter(), ", ");
    match (name, formatted.is_empty()) {
        ("", true) => variant.to_string(),
        ("", false) => format!("{}({})", variant, formatted),
        (_, true) => format!("{}::{}", name, variant),
        (_, false) => format!("{}::{}({})", name, variant, formatted),
    }
}

trait Pipe: Sized {
    fn pipe<U>(self, f: impl FnOnce(Self) -> U) -> U;
}

impl<T> Pipe for T {
    fn pipe<U>(self, f: impl FnOnce(Self) -> U) -> U {
        f(self)
    }
}
