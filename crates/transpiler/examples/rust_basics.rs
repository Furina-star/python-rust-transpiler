fn take_damage(health: i32, amount: i32) -> i32 {
    health - amount
}

fn restore_health(health: i32, amount: i32) -> i32 {
    health + amount
}

fn main() {
    let health = 100;
    let damage = 25;
    let healing = 10;

    let take_damage = take_damage(health, damage);
    let restore_health = restore_health(take_damage, healing);

    print!("Remaining health: {}", restore_health);
}