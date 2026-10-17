# Day 1 — Rust Fundamentals

**Date:** 2026-10-17  
**Status:** Completed (source reviewed; rerun latest version locally)

## Objectives
- Understand immutable bindings and mutable reassignment.
- Distinguish shadowing from reassignment.
- Learn basic types and type inference.
- Declare functions with parameters and return types.
- Understand return expressions and semicolons.
- Run a Rust example through Cargo.

## Vocabulary

| Term | Meaning |
|---|---|
| Binding | A name associated with a value, introduced with `let`. |
| Immutable | A binding that cannot be reassigned through that binding. |
| Mutable | A binding declared with `let mut` that permits reassignment. |
| Shadowing | Introducing a new binding using a name that is already in scope. |
| Type inference | The compiler determining a type from context without an explicit annotation. |
| Parameter | A named input declared in a function signature. |
| Argument | A value supplied in a function call. |
| Expression | Code that evaluates to a value. |
| Statement | A program action such as a `let` declaration. |
| Crate | A Rust compilation unit, either a library or an executable. |

## Implemented Exercise

**File:** `crates/transpiler/examples/rust_basics.rs`

The example defines two functions:

```rust
fn take_damage(health: i32, amount: i32) -> i32 {
    health - amount
}

fn restore_health(health: i32, amount: i32) -> i32 {
    health + amount
}
```

`main` starts with 100 health, applies 25 damage, restores 10 health, and prints the result.

**Expected:** `Remaining health: 85`

Run with:

```bash
cargo run -p python-rust-transpiler --example rust_basics
```

## Key Takeaways
1. Creating a new binding with `let` does not reassign the previous binding.
2. `let mut` is required when we want to reassign the same binding.
3. `let` can shadow an existing variable name; shadowing is not reassignment.
4. Rust can infer types such as `i32` from a simple integer initializer.
5. The last expression of a function body may be its return value when there is no trailing semicolon.
6. `print!` does not add a newline; `println!` does.
7. The Day 1 exercise calculates and returns values rather than mutating shared character state.

## Code Review Notes
- Both functions are correct for the exercise's intended arithmetic.
- Their parameters and return types are explicit `i32` values.
- The original local names `take_damage` and `restore_health` are legal, but names such as `remaining_health` and `final_health` better distinguish values from functions.
- No minimum- or maximum-health rules are required for this initial exercise.
- Automated test cases for the functions have not yet been added.

## Next Session
**Day 2: Structs, enums, and `match`.** Focus on the ideas and small exercises before implementing the compiler's `Token` enum.
