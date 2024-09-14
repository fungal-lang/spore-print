use spore_print::SporePrint;
// Import the trait
use spore_print_derive::SporePrint;
// Import the derive macro

#[derive(SporePrint)]
struct Book {
    title: String,
    pages: usize,
}

fn main() {
    let book = Book {
        title: "The Rust Programming Language".to_string(),
        pages: 552,
    };
    println!("{}", book.spore_print());
}
