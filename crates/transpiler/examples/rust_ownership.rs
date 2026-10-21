fn show_character(name: &str) {
    println!("Character: {name}");
}

fn update_character(name: &mut String) {
    name.push_str(" the Iudex");
}

fn make_label(name: &str) -> String {
    let mut label = String::from("Character: ");
    label.push_str(name);
    label
}

fn view_label(label: &str) -> &str {
    label
}

fn main() {
    // Moving ownership
    let character1 = String::from("Furina");
    let character2 = character1;

    // println!("{character1}");
    println!("{character2}");

    // Copy trait specifically for i32
    let level11 = 10;
    let level12 = level11;

    println!("\nLevel 11: {} {} {}", level11, "\nLevel 12:", level12);
    {
        let name = String::from("Focalor");
        println!("\nInside Scope Name: {name}");
    }

    // println!("{name}");

    // Clone trait implementation
    let original = String::from("Furina");
    let mut duplicate = original.clone();
    duplicate.push_str(" de Fontaine");

    println!("\nOriginal: {original}");
    println!("Duplicate: {duplicate} \n");

    // Borrowing and References
    let mut character = String::from("Neuvillette");

    show_character(&character);
    update_character(&mut character);
    show_character(&character);

    println!("Original owner still has: {character}");

    // Ownership in Functions & Lifetimes
    let favorite_character = String::from("Furina");
    let label = make_label(&favorite_character);
    let view = view_label(&label);

    println!("\n{view}");
    println!("Original Name: {favorite_character}");
}
