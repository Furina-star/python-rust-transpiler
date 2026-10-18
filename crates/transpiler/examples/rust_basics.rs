//! Practice variables, structs, and functions by calculating damage and healing.
//!
//! Create `Champion` values with `i32` fields and access their health and attack damage.
//! Bind intermediate results with `let`, then pass them to `take_damage()` and
//! `restore_health()`, which return values through their final expressions.
//! Print Ahri's remaining health after applying Zed's attack and a heal.

struct Champion {
    health: i32,
    attack_damage: i32,
    ability_power: i32,
    defense: i32,
}

fn take_damage(health: i32, amount: i32) -> i32 {
    health - amount
}

fn restore_health(health: i32, amount: i32) -> i32 {
    health + amount
}

fn main() {
    let ahri = Champion {
        health: 1000,
        attack_damage: 0,
        ability_power: 200,
        defense: 100,
    };

    let zed = Champion {
        health: 1000,
        attack_damage: 100,
        ability_power: 0,
        defense: 100,
    };

    let heal = 100;

    let ahri_took_damage = take_damage(ahri.health, zed.attack_damage);
    let ahri_final_health = restore_health(ahri_took_damage, heal);

    println!("Ahri's remaining health: {}", ahri_final_health);
}
