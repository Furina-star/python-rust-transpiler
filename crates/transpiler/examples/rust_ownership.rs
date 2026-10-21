fn main() {
    let character1 = String::from("Furina");
    let character2 = character1;

    // println!("{character1}");
    println!("{character2}");

    let level11 = 10;
    let level12 = level11;

    println!("\nLevel 11: {} {} {}", level11, "\nLevel 12:", level12);
    {
        let name = String::from("Focalor");
    }

    // println!("{name}");
}