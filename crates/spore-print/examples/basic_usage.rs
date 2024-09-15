use spore_print::SporePrint;
use spore_print_derive::SporePrint;
use std::ops::Range;

/// A struct representing a mushroom with species and cap diameter range.
#[derive(SporePrint)]
struct Mushroom {
    species: String,
    cap_diameter: Range<usize>,
}

fn main() {
    let mushroom = Mushroom {
        species: "Coprinus Comatus".to_string(),
        cap_diameter: 3..10,
    };
    println!("{}", mushroom.spore_print());
}