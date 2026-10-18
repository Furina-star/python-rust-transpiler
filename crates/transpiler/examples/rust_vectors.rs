//! Practice vectors, strings, loops, and iterators with player info and damage values.
//!
//! Grow and index a `Vec`, append to a `String`, and use a borrowed `&str`.
//! Traverse damage values with `while` and `for` loops, then compare iterator modes:
//! `iter()` borrows elements, `iter_mut()` borrows them mutably to add damage,
//! and `into_iter()` consumes the vector to yield owned elements.

fn main() {
    // Vectors
    let mut numbers = vec![100, 200, 300];
    numbers.push(400);

    // String and &str
    let mut name = String::from("Faker");
    name.push_str(" T1");
    let champion: &str = "Ahri";

    println!("\nPlayer: {}", name);
    println!("Champion: {}", champion);
    println!("Player name length: {}", name.len());

    println!("First element index: {}", numbers[0]);
    println!("Total numbers of element: {}", numbers.len());
    println!("Fourth element index: {}", numbers[3]);

    // Looping
    println!("\nLOOPING: ");
    let mut count = 0;
    while count < numbers.len() {
        println!("Damage: {}", numbers[count]);
        count += 1;
    }

    let mut sum1 = 0;
    for num in &numbers {
        sum1 += num;
    }
    println!("\nTotal damage: {}", sum1);

    // Iterators
    println!("\nITERATORS: ");
    for num in numbers.iter() {
        println!("{num}");
    }

    for num in numbers.iter_mut() {
        *num += 50;
    }
    println!("\n{:?}", numbers);

    let mut sum2 = 0;
    for num in &numbers {
        sum2 += num;
    }
    println!("\nTotal damage: {}", sum2);

    let new_numbers = vec![1, 2, 3];
    for num in new_numbers.into_iter() {
        println!("{num}");
    }
}
