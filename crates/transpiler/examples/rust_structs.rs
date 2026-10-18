//! Practice structs, enums, and pattern matching with player stats and a respawn timer.

enum PlayerStatus {
    Alive,
    Dead,
    Respawning(u32),
}

struct Player {
    level: i32,
    health: i32,
    experience: i32,
}

fn main() {
    // Instance structure from Player
    let mut faker = Player {
        level: 1,
        health: 100,
        experience: 0,
    };

    println!("Faker initial health: {}", faker.health);

    faker.health = 80;
    faker.experience = 50;

    println!("Faker current health: {}", faker.health);
    println!("Faker experiences: {}", faker.experience);

    // Pattern matching
    let player_status = PlayerStatus::Respawning(5);

    match player_status {
        PlayerStatus::Alive => println!("Faker is alive!"),
        PlayerStatus::Dead => println!("Faker has been defeated"),
        PlayerStatus::Respawning(seconds) => println!("Faker is respawning in {seconds} seconds..."),
    }
}
