# Project State

## Current Position

- **Foundation:** Phase 0 (Cargo workspace and repository setup) completed.
- **Learning:** Week 1, Day 2 — structs, enums, and pattern matching exercise implemented and run successfully.
- **Compiler implementation:** Not started; Phase 1 (Lexical Analysis) follows Rust fundamentals. The library entry point and compiler source files are placeholders; no transpilation API or CLI is available.

## Completed

- Configured the Cargo workspace with `crates/transpiler` as its only member.
- Day 1: Variables, mutability, types, functions, and return expressions. The current [basics example](../crates/transpiler/examples/rust_basics.rs) calculates damage and healing using a `Champion` struct.
- Day 2: Defined and instantiated a struct, updated fields, declared an enum, and implemented exhaustive pattern matching in [the structs example](../crates/transpiler/examples/rust_structs.rs).
- Implemented associated data with `PlayerStatus::Respawning(u32)` and bound the countdown in `PlayerStatus::Respawning(seconds)`.
- Built and ran both practice examples successfully.

## Current Task

Begin Day 3: `Vec`, `String`, loops, and iterators. Keep practice examples separate from the compiler source files.

## Known Issues / Follow-ups

- `cargo fmt --all -- --check` currently fails because `rust_structs.rs` needs formatting around the `Respawning` match arm and at the end of the file. Run `cargo fmt --all` to apply formatting, then repeat the check.
- The examples compile with `dead_code` warnings: `Champion.ability_power`, `Champion.defense`, and `Player.level` are unread, and `PlayerStatus::Alive` and `PlayerStatus::Dead` are never constructed.
- There are no automated tests yet. Passing the current test harness does not establish compiler correctness.
- Per-day learning notes are not currently present in the working tree. Add links only after the corresponding files exist.

## Verification Commands

Run these commands from the repository root:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo run -p python-rust-transpiler --example rust_basics
cargo run -p python-rust-transpiler --example rust_structs
```

## Verification Results

Reviewed on 2026-10-18 using `rustc 1.98.1` and `cargo 1.98.1`:

| Check | Result |
| --- | --- |
| Workspace check, including all targets | Passed with the unused-code warnings listed above |
| Workspace tests | Passed; 0 unit tests and 0 documentation tests |
| Formatting check | Failed; formatting changes needed in `rust_structs.rs` |
| `rust_basics` example | Passed; printed `Ahri's remaining health: 1000` |
| `rust_structs` example | Passed; output shown below |

```text
Faker initial health: 100
Faker current health: 80
Faker experiences: 50
Faker is respawning in 5 seconds...
```

The examples were run as they currently exist; this documentation review did not change Rust source files.

## Documentation Convention

At the end of each study day:

1. Review the implementation and concepts learned.
2. Run relevant checks and examples; record failures and warnings as well as successes.
3. Update this file and the [roadmap](ROADMAP.md) to reflect verified progress.
4. If keeping per-day notes, create `docs/learning/` and the relevant `DAY_XX.md` file before linking to it.
5. Update `DECISIONS.md` only when an actual architecture or scope decision changes.
6. Commit verified code and documentation together when ready.

## Last Review

2026-10-18: Checked documentation against the current manifests and source files, verified Day 2's associated-data implementation, and ran the checks and examples listed above. Day 3 is next; formatting remains a follow-up.
