use spore_print::SporePrint;
use spore_print_derive::SporePrint;

/// A struct representing a book with a title and number of pages.
#[derive(SporePrint)]
struct Book {
    title: String,
    pages: usize,
}

fn main() {
    let book = Book {
        title: "Mycelium Running: How Mushrooms Can Help Save the World".to_string(),
        pages: 356,
    };
    println!("{}", book.spore_print());
}
