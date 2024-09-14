use spore_print::SporePrint; // Import the SporePrint trait
use spore_print_derive::SporePrint; // Import the derive macro

#[derive(SporePrint)] // Derive the SporePrint trait
struct Mushroom {
    species: String,
    cap_diameter: usize,
}

fn main() {
    let mushroom = Mushroom {
        species: "Amanita muscaria".to_string(),
        cap_diameter: 10,
    };
    println!("{}", mushroom.spore_print());
}
