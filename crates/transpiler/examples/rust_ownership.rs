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
    println!("Duplicate: {duplicate}");
}