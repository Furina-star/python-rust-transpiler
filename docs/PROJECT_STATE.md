# Project State

## Current Position
- **Foundation:** Cargo workspace initialized and checked successfully in the initial setup.
- **Learning:** Week 1, Day 1 — completed.
- **Compiler implementation:** Not started. Phase 1 (Lexical Analysis) is next after the Rust fundamentals warm-up.

## Completed
- Configured Cargo workspace with `crates/transpiler` as the active member.
- Created initial repository documentation.
- Learned immutable bindings, mutable reassignment, shadowing, and basic types.
- Practiced Rust functions with typed parameters and return values.
- Completed the Day 1 health calculation exercise using `take_damage` and `restore_health`.
- Reviewed expression-style return values and the difference between `print!` and `println!`.

## Next Task
**Day 2: Structs, enums, and pattern matching.**

Start with small Rust-only examples, then explain how enums and `match` can represent lexical tokens. Do not begin the actual lexer implementation until the fundamentals are clear.

## Follow-ups
- Run `cargo fmt --all`, `cargo test --workspace`, and the latest example locally after making any edits.
- Rename local result bindings in the example for readability (optional).
- Use `println!` instead of `print!` if a trailing newline is preferred (optional).
- Check the folders `cli` and `python_macro`: they should eventually be siblings of `transpiler` under `crates/`, not nested inside it.

## Documentation Convention
At the end of each study day:
1. Review the code and explain the concepts learned.
2. Run relevant tests or examples.
3. Add/update the corresponding note in `docs/learning/`.
4. Update this file and mark completed roadmap items.
5. Update `DECISIONS.md` only for meaningful architectural or scope decisions.
6. Commit verified code and documentation together.

## Last Review
2026-10-17: Day 1 source reviewed. The output is expected to be `Remaining health: 85`; execution of the latest submitted revision was not independently verified.
